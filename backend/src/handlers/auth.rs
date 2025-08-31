use crate::error::{AppError, Result};
use crate::models::{LoginRequest, UpdateProfileRequest};
use crate::state::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    if app_state
        .config
        .verify_user(&request.username, &request.password)
        .await
    {
        let session_id = app_state.auth.create_session().await;
        
        let cookie = Cookie::build(("Token", session_id))
            .path("/")
            .permanent()
            .build();

        let jar = CookieJar::new().add(cookie);
        
        Ok((jar, StatusCode::NO_CONTENT))
    } else {
        Err(AppError::Unauthorized)
    }
}

pub async fn logout(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    // Check authentication
    let token = jar
        .get("Token")
        .and_then(|cookie| Some(cookie.value()))
        .ok_or(AppError::Unauthorized)?;

    if !app_state.auth.validate_session(token).await {
        return Err(AppError::Unauthorized);
    }

    app_state.auth.remove_session(token).await;

    let jar = jar.remove(Cookie::from("Token"));
    Ok((jar, StatusCode::NO_CONTENT))
}

pub async fn check_auth(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let token = jar
        .get("Token")
        .and_then(|cookie| Some(cookie.value()))
        .ok_or(AppError::Unauthorized)?;

    if !app_state.auth.validate_session(token).await {
        return Err(AppError::Unauthorized);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_profile(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    // Check authentication
    let token = jar
        .get("Token")
        .and_then(|cookie| Some(cookie.value()))
        .ok_or(AppError::Unauthorized)?;

    if !app_state.auth.validate_session(token).await {
        return Err(AppError::Unauthorized);
    }
    
    app_state
        .config
        .update_user_info(&request.username, &request.password)
        .await?;
    
    Ok(StatusCode::NO_CONTENT)
}
