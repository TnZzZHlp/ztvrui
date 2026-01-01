use crate::error::{AppError, Result};
use crate::models::{LoginRequest, UpdateProfileRequest};
use crate::services::auth::Claims;
use crate::state::AppState;
use crate::utils::is_private_ip;
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use std::net::{IpAddr, SocketAddr};

/// Extract the real IP address from the request headers or connection information.
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

pub async fn login(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let client_ip = extract_real_ip(&headers, addr);

    // Check if the IP is blocked.
    if app_state.ip_ban.is_banned(&client_ip).await {
        if let Some(remaining_seconds) =
            app_state.ip_ban.get_ban_remaining_seconds(&client_ip).await
        {
            return Err(AppError::TooManyRequests(format!(
                "Too many failed login attempts. Please try again in {} seconds.",
                remaining_seconds
            )));
        }
    }

    if app_state
        .config
        .verify_user(&request.username, &request.password)
        .await
    {
        // Login successful, clear failure records
        app_state.ip_ban.record_success(&client_ip).await;

        // Create JWT token
        let (token, expires_at) = app_state.auth.create_token(&request.username)?;

        // Return structured response with token info
        Ok(Json(json!({
            "token": token,
            "message": "Login successful",
            "expires_at": expires_at,
            "username": request.username
        })))
    } else {
        // Login failed, record failure attempt
        app_state.ip_ban.record_failure(&client_ip).await;
        Err(AppError::Unauthorized)
    }
}

pub async fn update_profile(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    // Claims are already validated by the middleware
    let _username = &claims.username;

    app_state
        .config
        .update_user_info(&request.username, &request.password)
        .await?;

    Ok(Json(json!({
        "message": "Profile updated successfully"
    })))
}

#[allow(dead_code)]
pub async fn refresh_token(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    // Use the validated claims to get the username
    let username = &claims.username;

    // Create a new token
    let (new_token, expires_at) = app_state.auth.create_token(username)?;

    Ok(Json(json!({
        "token": new_token,
        "expires_at": expires_at,
        "message": "Token refreshed successfully"
    })))
}
