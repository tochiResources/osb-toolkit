//! The persisted auth state: the JWT secret, the bootstrap admin token hash,
//! user accounts and the token revocation list.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::credentials;
use super::AuthError;

/// On-disk shape of the auth store.
#[derive(Debug, Default, Serialize, Deserialize)]
struct AuthData {
    /// Hex-encoded HMAC secret for signing JWTs.
    jwt_secret: String,
    /// Argon2 PHC hash of the bootstrap admin token.
    admin_token_hash: Option<String>,
    #[serde(default)]
    users: Vec<User>,
    #[serde(default)]
    revoked: Vec<Revoked>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    role: String,
    password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Revoked {
    jti: String,
    exp: u64,
}

/// A user as exposed over the API (no credential material).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub id: String,
    pub username: String,
    pub role: String,
}

impl From<&User> for UserView {
    fn from(user: &User) -> Self {
        UserView {
            id: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
        }
    }
}

/// The auth store, optionally backed by a JSON file.
pub struct AuthStore {
    path: Option<PathBuf>,
    data: Mutex<AuthData>,
}

impl AuthStore {
    /// Loads (or creates) the store. When no admin token is supplied and none is
    /// stored, a fresh one is generated and returned so the caller can surface it
    /// exactly once.
    pub fn open(
        path: Option<PathBuf>,
        admin_token: Option<String>,
    ) -> Result<(AuthStore, Option<String>), AuthError> {
        let mut data = match &path {
            Some(path) if path.is_file() => {
                let bytes = osb_core::io::read(path)?;
                serde_json::from_slice(&bytes)
                    .map_err(|error| AuthError::Internal(format!("auth store: {error}")))?
            }
            _ => AuthData::default(),
        };

        let mut dirty = false;
        if data.jwt_secret.is_empty() {
            data.jwt_secret = credentials::random_hex(32)?;
            dirty = true;
        }

        let supplied = admin_token.filter(|token| !token.is_empty());
        let generated = match supplied {
            Some(token) => {
                data.admin_token_hash = Some(credentials::hash_secret(&token)?);
                dirty = true;
                None
            }
            None if data.admin_token_hash.is_none() => {
                let token = credentials::random_hex(24)?;
                data.admin_token_hash = Some(credentials::hash_secret(&token)?);
                dirty = true;
                Some(token)
            }
            None => None,
        };

        let store = AuthStore {
            path,
            data: Mutex::new(data),
        };
        if dirty {
            store.save()?;
        }
        Ok((store, generated))
    }

    pub fn secret(&self) -> String {
        self.data.lock().expect("auth lock").jwt_secret.clone()
    }

    pub fn verify_admin_token(&self, token: &str) -> bool {
        let data = self.data.lock().expect("auth lock");
        data.admin_token_hash
            .as_deref()
            .map(|hash| credentials::verify_secret(token, hash))
            .unwrap_or(false)
    }

    /// Verifies a username/password pair against the stored hash.
    pub fn authenticate(&self, username: &str, password: &str) -> Option<UserView> {
        let data = self.data.lock().expect("auth lock");
        data.users
            .iter()
            .find(|user| user.username == username)
            .filter(|user| credentials::verify_secret(password, &user.password_hash))
            .map(UserView::from)
    }

    pub fn users(&self) -> Vec<UserView> {
        let data = self.data.lock().expect("auth lock");
        data.users.iter().map(UserView::from).collect()
    }

    pub fn add_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
    ) -> Result<UserView, AuthError> {
        if username.trim().is_empty() || password.is_empty() {
            return Err(AuthError::Conflict(
                "username and password are required".into(),
            ));
        }
        {
            let data = self.data.lock().expect("auth lock");
            if data.users.iter().any(|user| user.username == username) {
                return Err(AuthError::Conflict(format!("user `{username}` exists")));
            }
        }
        let user = User {
            id: credentials::random_hex(8)?,
            username: username.to_owned(),
            role: role.to_owned(),
            password_hash: credentials::hash_secret(password)?,
        };
        let view = UserView::from(&user);
        {
            let mut data = self.data.lock().expect("auth lock");
            data.users.push(user);
        }
        self.save()?;
        Ok(view)
    }

    pub fn remove_user(&self, id: &str) -> bool {
        let removed = {
            let mut data = self.data.lock().expect("auth lock");
            let before = data.users.len();
            data.users
                .retain(|user| user.id != id && user.username != id);
            data.users.len() != before
        };
        if removed {
            let _ = self.save();
        }
        removed
    }

    pub fn is_revoked(&self, jti: &str) -> bool {
        let now = now();
        let mut data = self.data.lock().expect("auth lock");
        data.revoked.retain(|entry| entry.exp > now);
        data.revoked.iter().any(|entry| entry.jti == jti)
    }

    pub fn revoke(&self, jti: &str, exp: u64) -> Result<(), AuthError> {
        {
            let mut data = self.data.lock().expect("auth lock");
            if !data.revoked.iter().any(|entry| entry.jti == jti) {
                data.revoked.push(Revoked {
                    jti: jti.to_owned(),
                    exp,
                });
            }
        }
        self.save()
    }

    /// Persists the store, restricting file permissions on Unix.
    fn save(&self) -> Result<(), AuthError> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        if let Some(parent) = path.parent() {
            osb_core::io::create_dir_all(parent)?;
        }
        let data = self.data.lock().expect("auth lock");
        let bytes = serde_json::to_vec_pretty(&*data)
            .map_err(|error| AuthError::Internal(error.to_string()))?;
        drop(data);
        write_private(path, &bytes)?;
        Ok(())
    }
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), AuthError> {
    osb_core::io::write(path, bytes)?;
    // Restrict to the owner where the platform supports it.
    let _ = osb_core::io::set_mode(path, 0o600);
    Ok(())
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> AuthStore {
        AuthStore::open(None, Some("bootstrap".into())).unwrap().0
    }

    #[test]
    fn generates_an_admin_token_when_none_is_supplied() {
        let (store, generated) = AuthStore::open(None, None).unwrap();
        let token = generated.expect("a token is generated once");
        assert!(store.verify_admin_token(&token));
        assert!(!store.verify_admin_token("nope"));
    }

    #[test]
    fn accepts_a_supplied_admin_token() {
        let store = store();
        assert!(store.verify_admin_token("bootstrap"));
        assert!(!store.verify_admin_token("other"));
    }

    #[test]
    fn users_round_trip_and_reject_duplicates() {
        let store = store();
        let user = store.add_user("alice", "hunter2", "user").unwrap();
        assert_eq!(user.username, "alice");
        assert!(store.authenticate("alice", "hunter2").is_some());
        assert!(store.authenticate("alice", "wrong").is_none());
        assert!(store.add_user("alice", "again", "user").is_err());
        assert!(store.remove_user("alice"));
        assert!(store.users().is_empty());
    }

    #[test]
    fn revoked_ids_are_tracked_and_expired_ones_pruned() {
        let store = store();
        store.revoke("abc", now() + 60).unwrap();
        assert!(store.is_revoked("abc"));
        assert!(!store.is_revoked("def"));
        store.revoke("old", now() - 1).unwrap();
        assert!(!store.is_revoked("old"));
    }
}
