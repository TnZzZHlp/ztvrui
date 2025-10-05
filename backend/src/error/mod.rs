use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    Unauthorized,

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("ZeroTier API error: {0}")]
    ZeroTierError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP client error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::ZeroTierError(e) => (StatusCode::BAD_GATEWAY, e.to_string()),
            AppError::ConfigError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::JsonError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::ReqwestError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
