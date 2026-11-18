use crate::error::{AppError, Result};
use crate::services::auth::Claims;
use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?.trim();
    let (scheme, token) = auth_header.split_once(' ')?;
    if scheme.eq_ignore_ascii_case("bearer") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    if let Some(api_key_header) = headers.get("X-API-Key") {
        if let Ok(api_key) = api_key_header.to_str() {
            let trimmed = api_key.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some((scheme, api_key)) = auth_str.trim().split_once(' ') {
                if scheme.eq_ignore_ascii_case("apikey") {
                    let trimmed = api_key.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
        }
    }

    None
}

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let headers = request.headers();
    let client_ip = app_state.extract_client_ip(headers, addr);

    if app_state.ip_ban.is_banned(&client_ip).await {
        tracing::warn!("Blocked request from banned IP: {}", client_ip);
        return Err(AppError::TooManyRequests(
            "Too many failed attempts. Please try again later.".to_string(),
        ));
    }

    let token = extract_bearer_token(headers).ok_or(AppError::Unauthorized)?;
    let claims: Claims = app_state.auth.validate_token(&token)?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

/// Authentication middleware for ZeroTier API proxy requests.
/// Accepts browser JWTs and configured API keys for automation clients.
pub async fn auth_or_api_key_middleware(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let headers = request.headers();
    let client_ip = app_state.extract_client_ip(headers, addr);
    let token = extract_bearer_token(headers);
    let api_key = extract_api_key(headers);

    if app_state.ip_ban.is_banned(&client_ip).await {
        tracing::warn!("Blocked request from banned IP: {}", client_ip);
        return Err(AppError::TooManyRequests(
            "Too many failed attempts. Please try again later.".to_string(),
        ));
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

    Err(AppError::Unauthorized)
}
