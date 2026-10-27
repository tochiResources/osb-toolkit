//! JWT (HS256) issuance and verification.

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

use super::AuthError;

/// The claims carried by an osbTK access or refresh token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: the username, or `admin` for the bootstrap token.
    pub sub: String,
    /// `admin` or `user`.
    pub role: String,
    /// Unique token id, used for revocation.
    pub jti: String,
    /// `access` or `refresh`.
    pub typ: String,
    pub iat: u64,
    pub exp: u64,
}

/// Signs `claims` with the store secret.
pub fn encode(secret: &str, claims: &Claims) -> Result<String, AuthError> {
    jwt_encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| AuthError::Internal(error.to_string()))
}

/// Verifies a token's signature and expiry.
pub fn decode(secret: &str, token: &str) -> Result<Claims, AuthError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    jwt_decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|error| match error.kind() {
        ErrorKind::ExpiredSignature => AuthError::Expired,
        _ => AuthError::InvalidToken,
    })
}
