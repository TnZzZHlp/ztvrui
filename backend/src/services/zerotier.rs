use crate::error::{AppError, Result};
use crate::models::ZeroTierConfig;
use axum::body::Bytes;
use reqwest::{Client, Method, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct ZeroTierService {
    client: Client,
    config: Arc<ZeroTierConfig>,
}

impl ZeroTierService {
    pub fn new(config: ZeroTierConfig) -> Self {
        Self {
            client: Client::new(),
            config: Arc::new(config),
        }
    }

    pub async fn forward_request(
        &self,
        endpoint: &str,
        method: Method,
        body: Bytes,
    ) -> Result<Response> {
        let url = format!("{}{}", self.config.address, endpoint);

        let mut request = self
            .client
            .request(method, &url)
            .header("X-ZT1-AUTH", &self.config.auth_token);

        // Forward body
        request = request.body(body);

        request
            .send()
            .await
            .map_err(|e| AppError::ZeroTierError(format!("Failed to forward request: {}", e)))
    }
}
