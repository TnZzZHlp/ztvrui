use crate::error::{AppError, Result};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde_json::Value;
use std::collections::HashMap;

pub async fn forward_to_zerotier(
    State(app_state): State<AppState>,
    jar: CookieJar,
    method: Method,
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    body: Option<Json<Value>>,
) -> Result<impl IntoResponse> {
    // Check authentication
    let token = jar
        .get("Token")
        .and_then(|cookie| Some(cookie.value()))
        .ok_or(AppError::Unauthorized)?;

    if !app_state.auth.validate_session(token).await {
        return Err(AppError::Unauthorized);
    }

    let mut endpoint = path;
    
    // Add query parameters if any
    if !params.is_empty() {
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        endpoint = format!("{}?{}", endpoint, query_string);
    }

    let body_value = body.map(|Json(value)| value);

    let zt_response = app_state
        .zerotier
        .forward_request(&endpoint, method, body_value)
        .await?;

    let status = zt_response.status();
    let response_body = zt_response
        .json::<Value>()
        .await
        .map_err(|e| AppError::ZeroTierError(format!("Failed to parse response: {}", e)))?;

    Ok((StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(response_body)))
}
