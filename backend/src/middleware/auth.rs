use crate::error::AppError;
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = jar
        .get("Token")
        .and_then(|cookie| Some(cookie.value()))
        .ok_or(AppError::Unauthorized)?;

    if !app_state.auth.validate_session(token).await {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}
