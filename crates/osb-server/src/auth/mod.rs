//! The server auth pipeline (section 5.6, M15): salted Argon2id credential
//! hashing, a bootstrap **admin token**, and **JWT** access/refresh tokens with
//! revocation.
//!
//! The store persists to JSON when a path is configured (the headless server)
//! and stays in memory otherwise (tests, embedding). On first run with no
//! supplied token a random admin token is generated and returned once so the
//! operator can capture it.

mod credentials;
mod jwt;
mod store;

use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use store::AuthStore;

pub use jwt::Claims;
pub use store::UserView;

/// Errors from the auth pipeline.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid token")]
    InvalidToken,

    #[error("token expired")]
    Expired,

    #[error("token revoked")]
    Revoked,

    #[error("forbidden")]
    Forbidden,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("i/o error: {0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    Internal(String),
}

impl From<osb_core::io::Error> for AuthError {
    fn from(error: osb_core::io::Error) -> Self {
        AuthError::IO(error.into())
    }
}

/// How the auth store and tokens are configured.
#[derive(Clone, Debug)]
pub struct AuthConfig {
    /// Where to persist the store; `None` keeps it in memory.
    pub file: Option<PathBuf>,
    /// A bootstrap admin token. When `None`, a random one is generated on first run.
    pub admin_token: Option<String>,
    /// Access-token lifetime.
    pub access_ttl: Duration,
    /// Refresh-token lifetime.
    pub refresh_ttl: Duration,
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            file: None,
            admin_token: None,
            access_ttl: Duration::from_secs(3600),
            refresh_ttl: Duration::from_secs(60 * 60 * 24 * 30),
        }
    }
}

/// Credentials presented to `POST /v1/auth/login`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    /// Log in with the bootstrap admin token instead of a user account.
    #[serde(default)]
    pub admin_token: Option<String>,
}

/// An issued access/refresh token pair.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
}

/// The auth façade used by the HTTP layer.
pub struct Auth {
    store: AuthStore,
    access_ttl: u64,
    refresh_ttl: u64,
}

impl Auth {
    /// Builds the auth pipeline, returning a generated admin token if one was created.
    pub fn new(config: AuthConfig) -> Result<(Auth, Option<String>), AuthError> {
        let (store, generated) = AuthStore::open(config.file, config.admin_token)?;
        Ok((
            Auth {
                store,
                access_ttl: config.access_ttl.as_secs().max(1),
                refresh_ttl: config.refresh_ttl.as_secs().max(1),
            },
            generated,
        ))
    }

    /// Exchanges credentials (admin token, or username/password) for a token pair.
    pub fn login(&self, request: &LoginRequest) -> Result<TokenPair, AuthError> {
        if let Some(token) = request.admin_token.as_deref() {
            return if self.store.verify_admin_token(token) {
                self.issue("admin", "admin")
            } else {
                Err(AuthError::InvalidCredentials)
            };
        }

        let (Some(username), Some(password)) =
            (request.username.as_deref(), request.password.as_deref())
        else {
            return Err(AuthError::InvalidCredentials);
        };
        let user = self
            .store
            .authenticate(username, password)
            .ok_or(AuthError::InvalidCredentials)?;
        self.issue(&user.username, &user.role)
    }

    /// Issues a fresh access token from a valid refresh token.
    pub fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AuthError> {
        let claims = self.verify(refresh_token)?;
        if claims.typ != "refresh" {
            return Err(AuthError::InvalidToken);
        }
        self.issue(&claims.sub, &claims.role)
    }

    /// Verifies a bearer credential: a JWT, or the bootstrap admin token.
    pub fn authenticate(&self, token: &str) -> Result<Claims, AuthError> {
        if !token.contains('.') && self.store.verify_admin_token(token) {
            let now = now();
            return Ok(Claims {
                sub: "admin".into(),
                role: "admin".into(),
                jti: format!("admin-{}", credentials::random_hex(8)?),
                typ: "access".into(),
                iat: now,
                exp: now + self.access_ttl,
            });
        }
        let claims = self.verify(token)?;
        if claims.typ != "access" {
            return Err(AuthError::InvalidToken);
        }
        Ok(claims)
    }

    /// Adds `jti` to the revocation list until it would have expired anyway.
    pub fn revoke(&self, claims: &Claims) -> Result<(), AuthError> {
        self.store.revoke(&claims.jti, claims.exp)
    }

    /// Verifies a raw token (access or refresh) and revokes it.
    pub fn revoke_str(&self, token: &str) -> Result<(), AuthError> {
        let claims = self.verify(token)?;
        self.store.revoke(&claims.jti, claims.exp)
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
    ) -> Result<UserView, AuthError> {
        self.store.add_user(username, password, role)
    }

    pub fn users(&self) -> Vec<UserView> {
        self.store.users()
    }

    pub fn remove_user(&self, id: &str) -> bool {
        self.store.remove_user(id)
    }

    fn issue(&self, sub: &str, role: &str) -> Result<TokenPair, AuthError> {
        let now = now();
        let secret = self.store.secret();
        let access = Claims {
            sub: sub.to_owned(),
            role: role.to_owned(),
            jti: credentials::random_hex(16)?,
            typ: "access".into(),
            iat: now,
            exp: now + self.access_ttl,
        };
        let refresh = Claims {
            sub: sub.to_owned(),
            role: role.to_owned(),
            jti: credentials::random_hex(16)?,
            typ: "refresh".into(),
            iat: now,
            exp: now + self.refresh_ttl,
        };
        Ok(TokenPair {
            access_token: jwt::encode(&secret, &access)?,
            refresh_token: jwt::encode(&secret, &refresh)?,
            token_type: "Bearer",
            expires_in: self.access_ttl,
        })
    }

    fn verify(&self, token: &str) -> Result<Claims, AuthError> {
        let claims = jwt::decode(&self.store.secret(), token)?;
        if self.store.is_revoked(&claims.jti) {
            return Err(AuthError::Revoked);
        }
        Ok(claims)
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

/// The default auth-store path under the user config directory.
pub fn default_path() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
    };
    base.map(|base| base.join("osbtk").join("auth.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth() -> Auth {
        Auth::new(AuthConfig {
            admin_token: Some("bootstrap".into()),
            ..AuthConfig::default()
        })
        .unwrap()
        .0
    }

    fn admin_login(auth: &Auth) -> TokenPair {
        auth.login(&LoginRequest {
            admin_token: Some("bootstrap".into()),
            ..LoginRequest::default()
        })
        .unwrap()
    }

    #[test]
    fn admin_token_login_issues_a_verifiable_jwt() {
        let auth = auth();
        let pair = admin_login(&auth);
        let claims = auth.authenticate(&pair.access_token).unwrap();
        assert_eq!(claims.sub, "admin");
        assert_eq!(claims.role, "admin");
    }

    #[test]
    fn user_login_and_then_revocation() {
        let auth = auth();
        auth.create_user("bob", "pw", "user").unwrap();
        let pair = auth
            .login(&LoginRequest {
                username: Some("bob".into()),
                password: Some("pw".into()),
                ..LoginRequest::default()
            })
            .unwrap();
        let claims = auth.authenticate(&pair.access_token).unwrap();
        assert_eq!(claims.role, "user");

        auth.revoke(&claims).unwrap();
        assert!(matches!(
            auth.authenticate(&pair.access_token),
            Err(AuthError::Revoked)
        ));
    }

    #[test]
    fn refresh_yields_a_new_access_token() {
        let auth = auth();
        let pair = admin_login(&auth);
        let refreshed = auth.refresh(&pair.refresh_token).unwrap();
        assert!(auth.authenticate(&refreshed.access_token).is_ok());
    }

    #[test]
    fn refresh_tokens_are_not_accepted_as_access_tokens() {
        let auth = auth();
        let pair = admin_login(&auth);
        assert!(matches!(
            auth.authenticate(&pair.refresh_token),
            Err(AuthError::InvalidToken)
        ));
    }

    #[test]
    fn wrong_credentials_are_rejected() {
        let auth = auth();
        assert!(matches!(
            auth.login(&LoginRequest {
                username: Some("ghost".into()),
                password: Some("pw".into()),
                ..LoginRequest::default()
            }),
            Err(AuthError::InvalidCredentials)
        ));
    }

    #[test]
    fn the_bootstrap_admin_token_is_accepted_directly() {
        let auth = auth();
        let claims = auth.authenticate("bootstrap").unwrap();
        assert_eq!(claims.role, "admin");
    }
}
