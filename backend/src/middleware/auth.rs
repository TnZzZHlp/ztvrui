use crate::services::auth::Claims;
use crate::state::AppState;
use crate::utils::extract_real_ip;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

// Helper function to extract token from Authorization header
fn extract_token(headers: &HeaderMap) -> Option<String> {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }
    None
}

fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    if let Some(api_key_header) = headers.get("X-API-Key") {
        if let Ok(api_key) = api_key_header.to_str() {
            return Some(api_key.to_string());
        }
    }

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(api_key) = auth_str.strip_prefix("ApiKey ") {
                return Some(api_key.to_string());
            }
        }
    }

    None
}

// Authentication middleware
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    // Extract client IP address
    let client_ip = extract_real_ip(headers, addr);

    // Check if the IP is banned
    if app_state.ip_ban.is_banned(&client_ip).await {
        tracing::warn!("Blocked request from banned IP: {}", client_ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let token = extract_token(headers).ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate JWT token
    let claims: Claims = app_state
        .auth
        .validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Authentication middleware for ZeroTier API proxy requests.
// It accepts browser JWTs and configured API keys for automation clients.
pub async fn auth_or_api_key_middleware(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    // Extract client IP address
    let client_ip = extract_real_ip(headers, addr);
    let token = extract_token(headers);
    let api_key = extract_api_key(headers);

    // Check if the IP is banned
    if app_state.ip_ban.is_banned(&client_ip).await {
        tracing::warn!("Blocked request from banned IP: {}", client_ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    if let Some(token) = token {
        if let Ok(claims) = app_state.auth.validate_token(&token) {
            request.extensions_mut().insert(claims);
            return Ok(next.run(request).await);
        }
    }

    if let Some(api_key) = api_key {
        if app_state.config.verify_api_key(&api_key) {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
