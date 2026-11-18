use crate::error::{AppError, Result};
use crate::models::{LoginRequest, UpdateProfileRequest};
use crate::services::auth::Claims;
use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use std::net::SocketAddr;

pub async fn login(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let client_ip = app_state.extract_client_ip(&headers, addr);

    if app_state.ip_ban.is_banned(&client_ip).await {
        let remaining_seconds = app_state
            .ip_ban
            .get_ban_remaining_seconds(&client_ip)
            .await
            .unwrap_or(1);
        return Err(AppError::TooManyRequests(format!(
            "Too many failed login attempts. Please try again in {} seconds.",
            remaining_seconds
        )));
    }

    if app_state
        .config
        .verify_user(&request.username, &request.password)
        .await?
    {
        app_state.ip_ban.record_success(&client_ip).await;

        let canonical_username = app_state.config.get_config().info.username.clone();
        let (token, expires_at) = app_state.auth.create_token(&canonical_username)?;

        Ok(Json(json!({
            "token": token,
            "message": "Login successful",
            "expires_at": expires_at,
            "username": canonical_username
        })))
    } else {
        app_state.ip_ban.record_failure(&client_ip).await;
        Err(AppError::Unauthorized)
    }
}

pub async fn update_profile(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    let client_ip = app_state.extract_client_ip(&headers, addr);

    if app_state.ip_ban.is_banned(&client_ip).await {
        return Err(AppError::TooManyRequests(
            "Too many failed password attempts. Please try again later.".to_string(),
        ));
    }

    if request.current_password.is_empty() {
        return Err(AppError::BadRequest(
            "Current password is required".to_string(),
        ));
    }

    let verified = app_state
        .config
        .verify_current_password(&request.current_password)
        .await?;

    if !verified {
        app_state.ip_ban.record_failure(&client_ip).await;
        return Err(AppError::PasswordVerificationFailed);
    }

    app_state.ip_ban.record_success(&client_ip).await;

    let trimmed_username = request.username.trim().to_string();
    let new_hash = app_state
        .config
        .update_user_info(&trimmed_username, &request.password)
        .await?;

    // Immediately rotate JWT signing key so older tokens are invalidated
    app_state
        .auth
        .rotate_credentials(&trimmed_username, &new_hash);

    let (token, expires_at) = app_state.auth.create_token(&trimmed_username)?;

    Ok(Json(json!({
        "message": "Profile updated successfully",
        "token": token,
        "expires_at": expires_at,
        "username": trimmed_username
    })))
}

pub async fn refresh_token(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let username = &claims.username;
    let (new_token, expires_at) = app_state.auth.create_token(username)?;

    Ok(Json(json!({
        "token": new_token,
        "expires_at": expires_at,
        "message": "Token refreshed successfully"
    })))
}
