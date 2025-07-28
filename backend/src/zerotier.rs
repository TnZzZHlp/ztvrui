use reqwest::{ Method, Response };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Default)]
pub struct ZeroTier {
    pub auth_token: String,
    pub address: String,
}

impl ZeroTier {
    pub fn new() -> Self {
        ZeroTier {
            auth_token: "".to_string(),
            address: "".to_string(),
        }
    }

    pub fn init(&mut self, zerotier: &ZeroTier) {
        self.auth_token = zerotier.auth_token.clone();
        self.address = zerotier.address.clone();
    }

    // Forward a request to the ZeroTier API
    pub async fn forward(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<serde_json::Value>
    ) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}", self.address, endpoint);

        reqwest::Client
            ::new()
            .request(method, &url)
            .header("X-ZT1-AUTH", self.auth_token.clone())
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await
    }
}
