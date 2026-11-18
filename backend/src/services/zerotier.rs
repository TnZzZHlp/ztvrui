use crate::error::{AppError, Result};
use crate::models::ZeroTierConfig;
use axum::body::Bytes;
use axum::http::HeaderValue;
use reqwest::{Client, Method, Response};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct ZeroTierService {
    client: Client,
    config: Arc<ZeroTierConfig>,
}

pub fn validate_zerotier_endpoint(endpoint: &str) -> Result<()> {
    if !endpoint.starts_with('/') || endpoint.starts_with("//") {
        return Err(AppError::BadRequest(
            "Invalid ZeroTier API path".to_string(),
        ));
    }

    if endpoint.contains("..")
        || endpoint.contains('\\')
        || endpoint.contains('@')
        || endpoint.contains('#')
        || endpoint
            .bytes()
            .any(|b| b.is_ascii_control() || b.is_ascii_whitespace())
    {
        return Err(AppError::BadRequest(
            "Invalid ZeroTier API path".to_string(),
        ));
    }

    let lower = endpoint.to_ascii_lowercase();
    if lower.contains("%2e") || lower.contains("%2f") || lower.contains("%5c") {
        return Err(AppError::BadRequest(
            "Invalid ZeroTier API path".to_string(),
        ));
    }

    Ok(())
}

impl ZeroTierService {
    pub fn new(config: ZeroTierConfig) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            config: Arc::new(config),
        }
    }

    pub async fn forward_request(
        &self,
        endpoint: &str,
        query: Option<&str>,
        method: Method,
        content_type: Option<&HeaderValue>,
        body: Bytes,
    ) -> Result<Response> {
        validate_zerotier_endpoint(endpoint)?;

        let base = self.config.address.trim_end_matches('/');
        let url = match query {
            Some(q) if !q.is_empty() => {
                if q.contains('#') || q.bytes().any(|b| b.is_ascii_control()) {
                    return Err(AppError::BadRequest(
                        "Invalid ZeroTier query string".to_string(),
                    ));
                }
                format!("{}{}?{}", base, endpoint, q)
            }
            _ => format!("{}{}", base, endpoint),
        };

        let mut request = self
            .client
            .request(method, &url)
            .header("X-ZT1-AUTH", &self.config.auth_token);

        if !body.is_empty() {
            let ct = content_type
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/json");
            request = request.header("Content-Type", ct);
        }

        request = request.body(body);

        request
            .send()
            .await
            .map_err(|e| AppError::ZeroTierError(format!("Failed to forward request: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_zerotier_endpoints() {
        assert!(validate_zerotier_endpoint("/status").is_ok());
        assert!(validate_zerotier_endpoint("/controller/network/0123456789abcdef").is_ok());

        for bad in [
            "status",
            "//evil.com/status",
            "/../etc/passwd",
            "/controller/%2e%2e/status",
            "/controller/%2f/status",
            "/controller\\status",
            "/status#frag",
            "/status with space",
        ] {
            assert!(
                validate_zerotier_endpoint(bad).is_err(),
                "Expected {bad} to be rejected"
            );
        }
    }
}
