//! The headless HTTP API (): `GET /v1/health`, the auth pipeline
//! (`/v1/auth/*`), the converter/job endpoints (`POST /v1/convert`,
//! `POST /v1/build`, `GET /v1/jobs/{id}`, `GET /v1/artifacts/{id}`), behind the
//! auth pipeline, per-key **rate limiting** and a bounded **job queue**.
//!
//! The HTTP server is for CI/devops and headless operation; the desktop app and
//! the CLI call the service layer directly.
//!
//! [`spawn`] starts it in-process for a programmatic embed.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{ConnectInfo, Path, Request, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;

use crate::auth::{Auth, AuthConfig, AuthError, Claims, LoginRequest};
use crate::jobs::{JobQueue, JobRunner, JobSpec, QueueConfig, QueueError};
use crate::package::InstallerConfig;
use crate::ratelimit::{RateLimitConfig, RateLimiter};
use crate::service::{ArtifactKind, BuildRequest, ConvertRequest, Service};

/// How the HTTP API is bound and protected.
#[derive(Clone, Debug)]
pub struct ServerConfig {
    /// Address to bind; use port `0` for an ephemeral port.
    pub bind: SocketAddr,
    /// The auth pipeline; `None` leaves the API open (embedding, tests).
    pub auth: Option<AuthConfig>,
    /// Per-key token bucket; `None` disables rate limiting.
    pub rate_limit: Option<RateLimitConfig>,
    /// Job queue sizing and retention.
    pub queue: QueueConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            bind: SocketAddr::from(([127, 0, 0, 1], 8787)),
            auth: None,
            rate_limit: None,
            queue: QueueConfig::default(),
        }
    }
}

/// Shared HTTP state: the service, auth, limiter and job queue.
#[derive(Clone)]
pub struct ServerState {
    service: Arc<Service>,
    auth: Option<Arc<Auth>>,
    limiter: Option<Arc<RateLimiter>>,
    queue: JobQueue,
}

impl ServerState {
    /// Builds state whose queue runs jobs on `service`.
    pub fn new(service: Arc<Service>, config: ServerConfig) -> Result<Self, AuthError> {
        let runner: Arc<dyn JobRunner> = service.clone();
        ServerState::with_runner(service, config, runner)
    }

    /// Builds state whose queue runs jobs on a supplied runner (used in tests).
    pub fn with_runner(
        service: Arc<Service>,
        config: ServerConfig,
        runner: Arc<dyn JobRunner>,
    ) -> Result<Self, AuthError> {
        let auth = match &config.auth {
            Some(cfg) => {
                let (auth, generated) = Auth::new(cfg.clone())?;
                if let Some(token) = generated {
                    eprintln!("[osbtk] generated admin token (store it now): {token}");
                }
                Some(Arc::new(auth))
            }
            None => None,
        };
        let queue = JobQueue::start(config.queue, runner);
        let limiter = config
            .rate_limit
            .map(|limit| Arc::new(RateLimiter::new(limit)));
        Ok(ServerState {
            service,
            auth,
            limiter,
            queue,
        })
    }
}

/// Builds the router.
pub fn router(state: ServerState) -> Router {
    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/refresh", post(refresh))
        .route("/v1/auth/revoke", post(revoke))
        .route("/v1/auth/me", get(me))
        .route("/v1/auth/users", get(list_users).post(create_user))
        .route("/v1/auth/users/{id}", delete(delete_user))
        .route("/v1/convert", post(convert))
        .route("/v1/build", post(build))
        .route("/v1/package", post(package))
        .route("/v1/jobs/{id}", get(job_status).delete(cancel_job))
        .route("/v1/jobs/{id}/cancel", post(cancel_job))
        .route("/v1/artifacts/{id}", get(artifact))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit))
        .with_state(state)
}

/// Serves the HTTP API until the process ends.
pub async fn serve(service: Service, config: ServerConfig) -> std::io::Result<()> {
    let bind = config.bind;
    let listener = tokio::net::TcpListener::bind(bind).await?;
    let state = ServerState::new(Arc::new(service), config).map_err(io_error)?;
    axum::serve(
        listener,
        router(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

/// A running embedded server.
#[derive(Clone, Debug)]
pub struct ServerHandle {
    pub addr: SocketAddr,
    pub token: Option<String>,
}

impl ServerHandle {
    /// The base URL of the running server.
    pub fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }
}

/// Starts the server on a background thread and returns once it is bound.
pub fn spawn(service: Arc<Service>, config: ServerConfig) -> std::io::Result<ServerHandle> {
    let token = config
        .auth
        .as_ref()
        .and_then(|auth| auth.admin_token.clone());
    let bind = config.bind;
    let (ready, ready_rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("osbtk-http".into())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    let _ = ready.send(Err(error.to_string()));
                    return;
                }
            };
            runtime.block_on(async move {
                let listener = match tokio::net::TcpListener::bind(bind).await {
                    Ok(listener) => listener,
                    Err(error) => {
                        let _ = ready.send(Err(error.to_string()));
                        return;
                    }
                };
                let addr = match listener.local_addr() {
                    Ok(addr) => addr,
                    Err(error) => {
                        let _ = ready.send(Err(error.to_string()));
                        return;
                    }
                };
                let state = match ServerState::new(service, config) {
                    Ok(state) => state,
                    Err(error) => {
                        let _ = ready.send(Err(error.to_string()));
                        return;
                    }
                };
                let _ = ready.send(Ok(addr));
                let _ = axum::serve(
                    listener,
                    router(state).into_make_service_with_connect_info::<SocketAddr>(),
                )
                .await;
            });
        })?;

    match ready_rx.recv() {
        Ok(Ok(addr)) => Ok(ServerHandle { addr, token }),
        Ok(Err(error)) => Err(std::io::Error::other(error)),
        Err(_) => Err(std::io::Error::other("server thread exited before binding")),
    }
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RevokeRequest {
    #[serde(default)]
    refresh_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserRequest {
    username: String,
    password: String,
    #[serde(default)]
    role: Option<String>,
}

async fn login(State(state): State<ServerState>, Json(request): Json<LoginRequest>) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    match auth.login(&request) {
        Ok(pair) => (StatusCode::OK, Json(pair)).into_response(),
        Err(error) => auth_error_response(error),
    }
}

async fn refresh(
    State(state): State<ServerState>,
    Json(request): Json<RefreshRequest>,
) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    match auth.refresh(&request.refresh_token) {
        Ok(pair) => (StatusCode::OK, Json(pair)).into_response(),
        Err(error) => auth_error_response(error),
    }
}

async fn revoke(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(request): Json<RevokeRequest>,
) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    let claims = match require_auth(&state, &headers) {
        Ok(claims) => claims,
        Err(response) => return *response,
    };
    if let Err(error) = auth.revoke(&claims) {
        return auth_error_response(error);
    }
    let mut revoked = 1;
    if let Some(refresh_token) = request.refresh_token.as_deref() {
        match auth.revoke_str(refresh_token) {
            Ok(()) => revoked += 1,
            Err(error) => return auth_error_response(error),
        }
    }
    (
        StatusCode::OK,
        Json(serde_json::json!({ "revoked": revoked })),
    )
        .into_response()
}

async fn me(State(state): State<ServerState>, headers: HeaderMap) -> Response {
    match require_auth(&state, &headers) {
        Ok(claims) => (StatusCode::OK, Json(claims)).into_response(),
        Err(response) => *response,
    }
}

async fn create_user(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(request): Json<CreateUserRequest>,
) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    if let Err(response) = require_admin(&state, &headers) {
        return *response;
    }
    let role = request.role.as_deref().unwrap_or("user");
    match auth.create_user(&request.username, &request.password, role) {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(error) => auth_error_response(error),
    }
}

async fn list_users(State(state): State<ServerState>, headers: HeaderMap) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    if let Err(response) = require_admin(&state, &headers) {
        return *response;
    }
    Json(serde_json::json!({ "users": auth.users() })).into_response()
}

async fn delete_user(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let Some(auth) = &state.auth else {
        return auth_disabled();
    };
    if let Err(response) = require_admin(&state, &headers) {
        return *response;
    }
    if auth.remove_user(&id) {
        (StatusCode::OK, Json(serde_json::json!({ "deleted": true }))).into_response()
    } else {
        not_found("user")
    }
}

async fn convert(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(request): Json<ConvertRequest>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }

    let service = Arc::clone(&state.service);
    match tokio::task::spawn_blocking(move || service.convert(&request)).await {
        Ok(Ok(result)) => (StatusCode::OK, Json(result)).into_response(),
        Ok(Err(error)) => error_response(StatusCode::BAD_REQUEST, &error.to_string()),
        Err(error) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &error.to_string()),
    }
}

async fn build(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(request): Json<BuildRequest>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }

    match state.queue.submit(JobSpec::Build(request)) {
        Ok(id) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "jobId": id, "status": "queued" })),
        )
            .into_response(),
        Err(QueueError::Full) => {
            let mut response = error_response(StatusCode::SERVICE_UNAVAILABLE, "job queue is full");
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(1u64));
            response
        }
    }
}

async fn package(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(request): Json<InstallerConfig>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }

    match state.queue.submit(JobSpec::Package(Box::new(request))) {
        Ok(id) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "jobId": id, "status": "queued" })),
        )
            .into_response(),
        Err(QueueError::Full) => {
            let mut response = error_response(StatusCode::SERVICE_UNAVAILABLE, "job queue is full");
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(1u64));
            response
        }
    }
}

async fn job_status(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }
    match state.queue.status(&id) {
        Some(record) => (StatusCode::OK, Json(record)).into_response(),
        None => not_found("job"),
    }
}

async fn cancel_job(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }
    match state.queue.cancel(&id) {
        Some(cancelled) => (
            StatusCode::OK,
            Json(serde_json::json!({ "cancelled": cancelled })),
        )
            .into_response(),
        None => not_found("job"),
    }
}

async fn artifact(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    if let Err(response) = require_auth(&state, &headers) {
        return *response;
    }
    let Some(artifact) = state.queue.artifact(&id) else {
        return not_found("artifact");
    };

    let path = PathBuf::from(&artifact.path);
    let (file_name, bytes) = match artifact.kind {
        ArtifactKind::File => {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| artifact.name.clone());
            match osb_core::io::read(&path) {
                Ok(bytes) => (name, bytes),
                Err(error) => {
                    return error_response(StatusCode::INTERNAL_SERVER_ERROR, &error.to_string())
                }
            }
        }
        ArtifactKind::Bundle => {
            let prefix = format!("{}.ofx.bundle", artifact.name);
            match crate::zip::zip_directory(&path, &prefix) {
                Ok(bytes) => (format!("{}.zip", artifact.name), bytes),
                Err(error) => {
                    return error_response(StatusCode::INTERNAL_SERVER_ERROR, &error.to_string())
                }
            }
        }
    };

    let mut response = Response::new(Body::from(bytes));
    let response_headers = response.headers_mut();
    response_headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    if let Ok(value) = HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\"")) {
        response_headers.insert(header::CONTENT_DISPOSITION, value);
    }
    response
}

/// Per-key token-bucket limiting, applied to every endpoint except health.
async fn rate_limit(State(state): State<ServerState>, request: Request, next: Next) -> Response {
    let Some(limiter) = &state.limiter else {
        return next.run(request).await;
    };
    if request.uri().path() == "/v1/health" {
        return next.run(request).await;
    }

    let key = match bearer(request.headers()) {
        Some(token) if state.auth.is_some() => format!("token:{token}"),
        _ => {
            let ip = request
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|info| info.0.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            format!("ip:{ip}")
        }
    };

    match limiter.check(&key) {
        Ok(()) => next.run(request).await,
        Err(retry_after) => {
            let mut response = error_response(StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded");
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(retry_after));
            response
        }
    }
}

/// Requires a valid credential when auth is enabled; open otherwise.
fn require_auth(state: &ServerState, headers: &HeaderMap) -> Result<Claims, Box<Response>> {
    match &state.auth {
        None => Ok(anonymous()),
        Some(auth) => match bearer(headers) {
            Some(token) => auth
                .authenticate(token)
                .map_err(|error| Box::new(auth_error_response(error))),
            None => Err(Box::new(unauthorized())),
        },
    }
}

/// Requires an authenticated `admin` caller.
fn require_admin(state: &ServerState, headers: &HeaderMap) -> Result<Claims, Box<Response>> {
    let claims = require_auth(state, headers)?;
    if claims.role == "admin" {
        Ok(claims)
    } else {
        Err(Box::new(forbidden()))
    }
}

/// The credential used when auth is disabled, treated as a local admin.
fn anonymous() -> Claims {
    Claims {
        sub: "anonymous".into(),
        role: "admin".into(),
        jti: String::new(),
        typ: "access".into(),
        iat: 0,
        exp: u64::MAX,
    }
}

/// The bearer token carried by the request, if any.
fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

fn auth_error_response(error: AuthError) -> Response {
    let status = match error {
        AuthError::InvalidCredentials
        | AuthError::InvalidToken
        | AuthError::Expired
        | AuthError::Revoked => StatusCode::UNAUTHORIZED,
        AuthError::Forbidden => StatusCode::FORBIDDEN,
        AuthError::Conflict(_) => StatusCode::CONFLICT,
        AuthError::IO(_) | AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    error_response(status, &error.to_string())
}

fn auth_disabled() -> Response {
    error_response(StatusCode::NOT_FOUND, "auth is not enabled")
}

fn unauthorized() -> Response {
    error_response(StatusCode::UNAUTHORIZED, "unauthorized")
}

fn forbidden() -> Response {
    error_response(StatusCode::FORBIDDEN, "forbidden")
}

fn not_found(what: &str) -> Response {
    error_response(StatusCode::NOT_FOUND, &format!("{what} not found"))
}

fn error_response(status: StatusCode, message: &str) -> Response {
    (status, Json(serde_json::json!({ "error": message }))).into_response()
}

fn io_error(error: AuthError) -> std::io::Error {
    std::io::Error::other(error.to_string())
}
