use std::net::IpAddr;

use axum::body::Bytes;
use serde::Deserialize;

use crate::error::{AppError, Result};
use crate::services::{ConfigService, IpBanService};

#[derive(Deserialize)]
struct DeleteNetworkConfirmation {
    password: String,
    confirmation_network_id: String,
}

pub fn network_deletion_target(path: &str) -> Result<Option<&str>> {
    if path.contains("//")
        || path.ends_with('/')
        || !path
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"/-_".contains(&byte))
    {
        return Err(AppError::BadRequest("Invalid deletion path".to_string()));
    }

    if path == "/controller/network" {
        return Err(AppError::BadRequest("Network ID is required".to_string()));
    }

    let Some(target) = path.strip_prefix("/controller/network/") else {
        return Ok(None);
    };
    let segments: Vec<&str> = target.split('/').collect();
    match segments.as_slice() {
        [network_id]
            if network_id.len() == 16
                && network_id.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            Ok(Some(network_id))
        }
        [network_id, "member", member_id]
            if network_id.len() == 16
                && network_id.bytes().all(|byte| byte.is_ascii_hexdigit())
                && member_id.len() == 10
                && member_id.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            Ok(None)
        }
        _ => Err(AppError::BadRequest("Invalid deletion path".to_string())),
    }
}

pub async fn verify_network_deletion(
    config: &ConfigService,
    ip_ban: &IpBanService,
    client_ip: IpAddr,
    network_id: &str,
    body: &Bytes,
) -> Result<()> {
    if ip_ban.is_banned(&client_ip).await {
        return Err(AppError::TooManyRequests(
            "Too many failed password attempts. Please try again later.".to_string(),
        ));
    }

    let confirmation: DeleteNetworkConfirmation = serde_json::from_slice(body)
        .map_err(|_| AppError::BadRequest("Password and network ID are required".to_string()))?;
    if !confirmation
        .confirmation_network_id
        .trim()
        .eq_ignore_ascii_case(network_id)
    {
        return Err(AppError::BadRequest(
            "Network ID confirmation does not match".to_string(),
        ));
    }

    if confirmation.password.is_empty() {
        return Err(AppError::BadRequest("Password is required".to_string()));
    }

    let verified = config
        .verify_current_password(&confirmation.password)
        .await?;

    if !verified {
        ip_ban.record_failure(&client_ip).await;
        return Err(AppError::PasswordVerificationFailed);
    }

    ip_ban.record_success(&client_ip).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_network_and_member_deletion_paths() {
        assert_eq!(
            network_deletion_target("/controller/network/0123456789abcdef").unwrap(),
            Some("0123456789abcdef")
        );
        assert_eq!(
            network_deletion_target("/controller/network/0123456789abcdef/member/deadbeef01")
                .unwrap(),
            None
        );
        assert!(matches!(
            network_deletion_target("/controller/network"),
            Err(AppError::BadRequest(_))
        ));
        assert!(matches!(
            network_deletion_target("/controller/network/0123456789abcdef/"),
            Err(AppError::BadRequest(_))
        ));
        assert!(matches!(
            network_deletion_target("/controller/network//0123456789abcdef"),
            Err(AppError::BadRequest(_))
        ));
        assert!(matches!(
            network_deletion_target("/controller/network/short"),
            Err(AppError::BadRequest(_))
        ));
    }
}
