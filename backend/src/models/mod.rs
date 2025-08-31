use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTierConfig {
    pub auth_token: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub info: UserInfo,
    pub listen: String,
    pub zerotier: ZeroTierConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            info: UserInfo {
                username: String::new(),
                password: String::new(),
            },
            listen: "127.0.0.1:3000".to_string(),
            zerotier: ZeroTierConfig {
                auth_token: String::new(),
                address: String::new(),
            },
        }
    }
}
