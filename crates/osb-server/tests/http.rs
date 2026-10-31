//! HTTP integration tests for the HTTP surface (auth pipeline, rate limiting, the
//! job queue and backpressure, job/artifact endpoints).

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use axum::body::Body;
use axum::http::{header, HeaderMap, Request, StatusCode};
use axum::Router;
use osb_server::{
    router, Artifact, ArtifactKind, AuthConfig, JobRunner, JobSpec, QueueConfig, RateLimitConfig,
    ServerConfig, ServerState, Service, ServiceConfig,
};
use tower::ServiceExt;

fn service() -> Arc<Service> {
    Arc::new(Service::with_config(ServiceConfig::in_process()))
}

#[derive(Default)]
struct StubRunner {
    path: PathBuf,
    gate: Option<Arc<(Mutex<bool>, Condvar)>>,
}

impl JobRunner for StubRunner {
    fn run(&self, _spec: JobSpec, _cancel: &AtomicBool) -> Result<Artifact, String> {
        if let Some(gate) = &self.gate {
            let (lock, condvar) = &**gate;
            let mut released = lock.lock().unwrap();
            while !*released {
                released = condvar.wait(released).unwrap();
            }
        }
        Ok(Artifact {
            id: String::new(),
            name: "Thing".into(),
            target: "after-effects".into(),
            kind: ArtifactKind::File,
            path: self.path.to_string_lossy().into_owned(),
            size: 5,
        })
    }
}

fn artifact_path() -> PathBuf {
    let path =
        osb_core::io::temp_dir().join(format!("osbtk-http-{}-{}.aex", std::process::id(), rand()));
    osb_core::io::write(&path, b"hello").unwrap();
    path
}

fn rand() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

/// A config with the auth pipeline enabled and a known bootstrap token.
fn authed_config() -> ServerConfig {
    ServerConfig {
        auth: Some(AuthConfig {
            admin_token: Some("bootstrap".into()),
            ..AuthConfig::default()
        }),
        ..ServerConfig::default()
    }
}

fn app(runner: Arc<dyn JobRunner>, config: ServerConfig) -> Router {
    router(ServerState::with_runner(service(), config, runner).expect("server state"))
}

fn request(method: &str, uri: &str, body: Option<&str>, token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/json");
    }
    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    builder
        .body(
            body.map(|body| Body::from(body.to_owned()))
                .unwrap_or_else(Body::empty),
        )
        .unwrap()
}

fn post(uri: &str, body: &str, token: Option<&str>) -> Request<Body> {
    request("POST", uri, Some(body), token)
}

fn get(uri: &str, token: Option<&str>) -> Request<Body> {
    request("GET", uri, None, token)
}

async fn send(app: &Router, request: Request<Body>) -> (StatusCode, HeaderMap, Vec<u8>) {
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, headers, bytes.to_vec())
}

fn json(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).unwrap()
}

const BUILD_BODY: &str = r#"{"crateDir":".","target":"after-effects"}"#;
const ADMIN_LOGIN: &str = r#"{"adminToken":"bootstrap"}"#;

#[tokio::test]
async fn health_is_open() {
    let app = app(Arc::new(StubRunner::default()), ServerConfig::default());
    let (status, _, body) = send(&app, get("/v1/health", None)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json(&body)["status"], "ok");
}

#[tokio::test]
async fn login_is_404_when_auth_is_disabled() {
    let app = app(Arc::new(StubRunner::default()), ServerConfig::default());
    let (status, _, _) = send(&app, post("/v1/auth/login", ADMIN_LOGIN, None)).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn build_requires_a_credential() {
    let app = app(Arc::new(StubRunner::default()), authed_config());

    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, Some("wrong"))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::ACCEPTED);
}

#[tokio::test]
async fn login_refresh_me_and_revoke_flow() {
    let app = app(Arc::new(StubRunner::default()), authed_config());

    let (status, _, body) = send(&app, post("/v1/auth/login", ADMIN_LOGIN, None)).await;
    assert_eq!(status, StatusCode::OK);
    let tokens = json(&body);
    let access = tokens["accessToken"].as_str().unwrap().to_owned();
    let refresh = tokens["refreshToken"].as_str().unwrap().to_owned();

    let (status, _, body) = send(&app, get("/v1/auth/me", Some(&access))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json(&body)["role"], "admin");

    let (status, _, body) = send(
        &app,
        post(
            "/v1/auth/refresh",
            &format!(r#"{{"refreshToken":"{refresh}"}}"#),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let refreshed = json(&body)["accessToken"].as_str().unwrap().to_owned();
    assert_eq!(
        send(&app, post("/v1/build", BUILD_BODY, Some(&refreshed)))
            .await
            .0,
        StatusCode::ACCEPTED
    );

    let (status, _, _) = send(&app, post("/v1/auth/revoke", "{}", Some(&access))).await;
    assert_eq!(status, StatusCode::OK);
    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, Some(&access))).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "revoked token is rejected"
    );
}

#[tokio::test]
async fn user_management_enforces_admin_role() {
    let app = app(Arc::new(StubRunner::default()), authed_config());

    let (status, _, body) = send(
        &app,
        post(
            "/v1/auth/users",
            r#"{"username":"alice","password":"hunter2"}"#,
            Some("bootstrap"),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json(&body)["role"], "user");

    let (status, _, body) = send(
        &app,
        post(
            "/v1/auth/login",
            r#"{"username":"alice","password":"hunter2"}"#,
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let alice = json(&body)["accessToken"].as_str().unwrap().to_owned();

    assert_eq!(
        send(&app, post("/v1/build", BUILD_BODY, Some(&alice)))
            .await
            .0,
        StatusCode::ACCEPTED,
        "any authenticated user may build"
    );
    assert_eq!(
        send(
            &app,
            post(
                "/v1/auth/users",
                r#"{"username":"bob","password":"pw"}"#,
                Some(&alice)
            ),
        )
        .await
        .0,
        StatusCode::FORBIDDEN,
        "a non-admin cannot create users"
    );

    let (status, _, body) = send(&app, get("/v1/auth/users", Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json(&body)["users"].as_array().unwrap().len(), 1);

    let (status, _, _) = send(
        &app,
        post(
            "/v1/auth/login",
            r#"{"username":"alice","password":"wrong"}"#,
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn a_job_runs_and_its_artifact_downloads() {
    let path = artifact_path();
    let app = app(
        Arc::new(StubRunner {
            path: path.clone(),
            gate: None,
        }),
        authed_config(),
    );

    let (_, _, body) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    let id = json(&body)["jobId"].as_str().unwrap().to_owned();

    let mut record = serde_json::Value::Null;
    for _ in 0..200 {
        let (status, _, body) = send(&app, get(&format!("/v1/jobs/{id}"), Some("bootstrap"))).await;
        assert_eq!(status, StatusCode::OK);
        record = json(&body);
        if record["status"]["status"] == "succeeded" {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(record["status"]["status"], "succeeded");

    let (status, headers, bytes) =
        send(&app, get(&format!("/v1/artifacts/{id}"), Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(bytes, b"hello");
    assert!(headers[header::CONTENT_DISPOSITION]
        .to_str()
        .unwrap()
        .contains(".aex"));
}

#[tokio::test]
async fn unknown_jobs_and_artifacts_are_404() {
    let app = app(Arc::new(StubRunner::default()), authed_config());
    assert_eq!(
        send(&app, get("/v1/jobs/nope", Some("bootstrap"))).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        send(&app, get("/v1/artifacts/nope", Some("bootstrap")))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        send(&app, post("/v1/jobs/nope/cancel", "", Some("bootstrap")))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn rate_limiting_returns_429_with_retry_after() {
    let config = ServerConfig {
        rate_limit: Some(RateLimitConfig::new(0.0, 1)),
        ..authed_config()
    };
    let app = app(Arc::new(StubRunner::default()), config);

    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::ACCEPTED);

    let (status, headers, _) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert!(headers.contains_key(header::RETRY_AFTER));

    let (status, _, _) = send(&app, get("/v1/health", None)).await;
    assert_eq!(status, StatusCode::OK, "health is never rate limited");
}

#[tokio::test]
async fn a_full_queue_returns_503_with_retry_after() {
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let runner = Arc::new(StubRunner {
        path: artifact_path(),
        gate: Some(Arc::clone(&gate)),
    });
    let config = ServerConfig {
        queue: QueueConfig {
            workers: 1,
            capacity: 1,
            ..QueueConfig::default()
        },
        ..authed_config()
    };
    let app = app(runner, config);

    let (_, _, body) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    let first = json(&body)["jobId"].as_str().unwrap().to_owned();
    for _ in 0..200 {
        let (_, _, body) = send(&app, get(&format!("/v1/jobs/{first}"), Some("bootstrap"))).await;
        if json(&body)["status"]["status"] == "running" {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    let (status, _, _) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let (status, headers, _) = send(&app, post("/v1/build", BUILD_BODY, Some("bootstrap"))).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(headers.contains_key(header::RETRY_AFTER));

    let (lock, condvar) = &*gate;
    *lock.lock().unwrap() = true;
    condvar.notify_all();
}
