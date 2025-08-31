use crate::error::{AppError, Result};
use crate::services::auth::Claims;
use crate::state::AppState;
use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Extension,
};

pub async fn forward_to_zerotier(
    State(app_state): State<AppState>,
    Extension(_claims): Extension<Claims>, // Claims already validated by middleware
    method: Method,
    uri: Uri,
    body: Bytes,
) -> Result<impl IntoResponse> {
    // Forward request and get response
    let zt_response = app_state
        .zerotier
        .forward_request(uri.path(), method, body)
        .await?;

    // Convert response directly
    let status = zt_response.status();
    let response_body = zt_response
        .bytes()
        .await
        .map_err(|e| AppError::ZeroTierError(format!("Failed to read response: {}", e)))?;

    let mut response = Response::new(Body::from(response_body));
    *response.status_mut() =
        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    Ok(response)
}
