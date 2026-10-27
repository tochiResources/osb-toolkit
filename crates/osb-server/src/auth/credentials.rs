//! Credential hashing and secure randomness for the auth pipeline.

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

use super::AuthError;

/// Fills `len` bytes from the operating system CSPRNG.
pub fn random_bytes(len: usize) -> Result<Vec<u8>, AuthError> {
    let mut buffer = vec![0u8; len];
    getrandom::getrandom(&mut buffer)
        .map_err(|error| AuthError::Internal(format!("randomness: {error}")))?;
    Ok(buffer)
}

/// A hex string of `len` random bytes, used for secrets, ids and nonces.
pub fn random_hex(len: usize) -> Result<String, AuthError> {
    Ok(hex(&random_bytes(len)?))
}

/// Lowercase hex encoding.
pub fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from_digit((byte >> 4) as u32, 16).unwrap_or('0'));
        out.push(char::from_digit((byte & 0x0F) as u32, 16).unwrap_or('0'));
    }
    out
}

/// Hashes a password or token with Argon2id and a fresh random salt. The salt
/// is embedded in the returned PHC string, so no separate salt column is kept.
pub fn hash_secret(secret: &str) -> Result<String, AuthError> {
    let salt = SaltString::encode_b64(&random_bytes(16)?)
        .map_err(|error| AuthError::Internal(error.to_string()))?;
    Argon2::default()
        .hash_password(secret.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| AuthError::Internal(error.to_string()))
}

/// Verifies a secret against a stored PHC hash in constant time.
pub fn verify_secret(secret: &str, phc: &str) -> bool {
    match PasswordHash::new(phc) {
        Ok(parsed) => Argon2::default()
            .verify_password(secret.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_verify_and_reject() {
        let hash = hash_secret("correct horse").unwrap();
        assert!(verify_secret("correct horse", &hash));
        assert!(!verify_secret("wrong", &hash));
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn salts_differ_per_hash() {
        let a = hash_secret("same").unwrap();
        let b = hash_secret("same").unwrap();
        assert_ne!(a, b, "each hash must carry its own salt");
        assert!(verify_secret("same", &a) && verify_secret("same", &b));
    }

    #[test]
    fn random_hex_has_the_expected_length() {
        assert_eq!(random_hex(16).unwrap().len(), 32);
        assert_ne!(random_hex(16).unwrap(), random_hex(16).unwrap());
    }
}
