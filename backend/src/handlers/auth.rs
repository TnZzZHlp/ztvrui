use crate::error::{AppError, Result};
use crate::models::{LoginRequest, UpdateProfileRequest};
use crate::services::auth::Claims;
use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde_json::json;

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    if app_state
        .config
        .verify_user(&request.username, &request.password)
        .await
    {
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
        Err(AppError::Unauthorized)
    }
}

pub async fn logout() -> Result<impl IntoResponse> {
    // With JWT, logout is handled client-side by removing the token
    // In production, you might want to implement a token blacklist
    Ok(Json(json!({
        "message": "Logout successful"
    })))
}

pub async fn check_auth(Extension(claims): Extension<Claims>) -> Result<impl IntoResponse> {
    Ok(Json(json!({
        "authenticated": true,
        "username": claims.username,
        "expires_at": claims.exp,
        "issued_at": claims.iat
    })))
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
