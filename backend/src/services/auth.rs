use crate::error::{AppError, Result};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCheckResponse {
    pub authenticated: bool,
    pub username: String,
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct AuthService {
    encoding_key: Arc<EncodingKey>,
    decoding_key: Arc<DecodingKey>,
    token_duration: Duration,
}

impl AuthService {
    pub fn new(secret: String) -> Self {
        Self {
            encoding_key: Arc::new(EncodingKey::from_secret(secret.as_ref())),
            decoding_key: Arc::new(DecodingKey::from_secret(secret.as_ref())),
            token_duration: Duration::hours(168),
        }
    }

    pub fn create_token(&self, username: &str) -> Result<(String, i64)> {
        let now = Utc::now();
        let exp = now + self.token_duration;

        let claims = Claims {
            sub: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            username: username.to_string(),
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key).map_err(|e| {
            AppError::InternalServerError(format!("Failed to create JWT token: {}", e))
        })?;

        Ok((token, exp.timestamp()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::Unauthorized,
                jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::Unauthorized,
                _ => AppError::InternalServerError(format!("JWT validation error: {}", e)),
            })
    }
}
