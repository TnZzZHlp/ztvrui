use std::net::SocketAddr;

use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};

use crate::error::{AppError, Result};
use crate::services::network_deletion::{network_deletion_target, verify_network_deletion};
use crate::state::AppState;
use crate::utils::extract_real_ip;

pub async fn forward_to_zerotier(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    mut body: Bytes,
) -> Result<impl IntoResponse> {
    if method == Method::DELETE {
        if let Some(network_id) = network_deletion_target(uri.path())? {
            verify_network_deletion(
                &app_state.config,
                &app_state.ip_ban,
                extract_real_ip(&headers, addr),
                network_id,
                &body,
            )
            .await?;
            body = Bytes::new();
        }
    }

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
