use std::net::SocketAddr;

use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, State},
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};

use crate::error::{AppError, Result};
use crate::services::network_deletion::{network_deletion_target, verify_network_deletion};
use crate::state::AppState;

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
            let client_ip = app_state.extract_client_ip(&headers, addr);
            verify_network_deletion(
                &app_state.config,
                &app_state.ip_ban,
                client_ip,
                network_id,
                &body,
            )
            .await?;
            body = Bytes::new();
        }
    }

    let zt_response = app_state
        .zerotier
        .forward_request(
            uri.path(),
            uri.query(),
            method,
            headers.get(header::CONTENT_TYPE),
            body,
        )
        .await?;

    let status = zt_response.status();
    let content_type = zt_response.headers().get(header::CONTENT_TYPE).cloned();
    let response_body = zt_response
        .bytes()
        .await
        .map_err(|e| AppError::ZeroTierError(format!("Failed to read response: {}", e)))?;

    let mut response = Response::new(Body::from(response_body));
    *response.status_mut() =
        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    if let Some(ct) = content_type {
        response.headers_mut().insert(header::CONTENT_TYPE, ct);
    }

    Ok(response)
}
