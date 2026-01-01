use crate::services::auth::Claims;
use crate::state::AppState;
use crate::utils::is_private_ip;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::{IpAddr, SocketAddr};

/// Helper function to extract the real IP address from request headers or connection information
/// Priority: If ConnectInfo IP is public, use it; otherwise check X-Real-IP and X-Forwarded-For headers.
fn extract_real_ip(headers: &HeaderMap, fallback_addr: SocketAddr) -> IpAddr {
    let connect_ip = fallback_addr.ip();

    // If the IP from ConnectInfo is not private, use it directly
    if !is_private_ip(&connect_ip) {
        tracing::debug!("Using public IP from ConnectInfo: {}", connect_ip);
        return connect_ip;
    }

    // Otherwise, check proxy headers
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                tracing::debug!("Got IP from X-Real-IP header: {}", ip);
                return ip;
            }
        }
    }

    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                let ip_str = first_ip.trim();
                if let Ok(ip) = ip_str.parse::<IpAddr>() {
                    tracing::debug!("Got IP from X-Forwarded-For header: {}", ip);
                    return ip;
                }
            }
        }
    }

    // Fallback to ConnectInfo IP (even if private)
    tracing::debug!("Using fallback private IP from ConnectInfo: {}", connect_ip);
    connect_ip
}

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
