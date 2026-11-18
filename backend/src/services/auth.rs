use crate::error::{AppError, Result};
use crate::utils::constant_time_eq;
use arc_swap::ArcSwap;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user identifier)
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
    pub username: String, // Username for convenience
}

struct JwtKeys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expected_username: String,
}

#[derive(Clone)]
pub struct AuthService {
    base_secret: Arc<String>,
    keys: Arc<ArcSwap<JwtKeys>>,
}

impl AuthService {
    pub fn new(base_secret: String, username: &str, password_hash: &str) -> Self {
        let keys = Self::derive_keys(&base_secret, username, password_hash);
        Self {
            base_secret: Arc::new(base_secret),
            keys: Arc::new(ArcSwap::new(Arc::new(keys))),
        }
    }

    fn derive_keys(base_secret: &str, username: &str, password_hash: &str) -> JwtKeys {
        // Bind signing key to both base secret and current user credentials so changing
        // username/password immediately invalidates all previously issued sessions.
        let derived_secret = format!(
            "ztvrui-jwt-v1:{}:{}:{}",
            base_secret, username, password_hash
        );
        JwtKeys {
            encoding_key: EncodingKey::from_secret(derived_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(derived_secret.as_bytes()),
            expected_username: username.to_string(),
        }
    }

    pub fn rotate_credentials(&self, username: &str, password_hash: &str) {
        let new_keys = Self::derive_keys(&self.base_secret, username, password_hash);
        self.keys.store(Arc::new(new_keys));
    }

    pub fn create_token(&self, username: &str) -> Result<(String, i64)> {
        let now = Utc::now();
        let exp = now + Duration::hours(168);

        let claims = Claims {
            sub: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            username: username.to_string(),
        };

        let keys = self.keys.load();
        let token = encode(&Header::default(), &claims, &keys.encoding_key).map_err(|e| {
            AppError::InternalServerError(format!("Failed to create JWT token: {}", e))
        })?;

        Ok((token, exp.timestamp()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        let keys = self.keys.load();

        let claims = decode::<Claims>(token, &keys.decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|_| AppError::Unauthorized)?;

        if !constant_time_eq(
            claims.username.as_bytes(),
            keys.expected_username.as_bytes(),
        ) {
            return Err(AppError::Unauthorized);
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_validation_and_rotation() {
        let auth = AuthService::new("base-secret".to_string(), "admin", "$2b$08$hash1");
        let (token, _) = auth.create_token("admin").unwrap();

        let claims = auth.validate_token(&token).unwrap();
        assert_eq!(claims.username, "admin");

        // Malformed token returns Unauthorized, not InternalServerError
        assert!(matches!(
            auth.validate_token("not-a-jwt"),
            Err(AppError::Unauthorized)
        ));

        // Credential rotation immediately invalidates old token
        auth.rotate_credentials("admin", "$2b$08$hash2");
        assert!(matches!(
            auth.validate_token(&token),
            Err(AppError::Unauthorized)
        ));

        let (new_token, _) = auth.create_token("admin").unwrap();
        assert!(auth.validate_token(&new_token).is_ok());
    }
}
