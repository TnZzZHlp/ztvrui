use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::zerotier::ZeroTier;
use crate::CONFIG_PATH;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub info: Info,
    pub listen: String,
    pub zerotier: ZeroTier,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn init<T: AsRef<Path>>(path: T) -> Self {
        let conifg = std::fs::read_to_string(path).expect("Failed to read config file");

        serde_json::from_str(&conifg).expect("Failed to parse config file")
    }

    pub async fn verify(&self, username: &str, password: &str) -> bool {
        username == self.info.username && verify(password, &self.info.password).unwrap()
    }

    pub async fn update_user_info(&mut self, username: &str, password: &str) {
        self.info.username = username.to_string();
        self.info.password = hash(password, 8).unwrap();

        let config = serde_json::to_string(&self).unwrap();

        std::fs::write(CONFIG_PATH.clone(), config).unwrap();
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            info: Info {
                username: "".to_string(),
                password: "".to_string(),
            },
            listen: "".to_string(),
            zerotier: ZeroTier {
                auth_token: "".to_string(),
                address: "".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let file = std::fs::File::create("test.json").unwrap();

        let config = AppConfig {
            info: Info {
                username: "admin".to_string(),
                password: "password".to_string(),
            },
            listen: "127.0.0.1:5800".to_string(),
            zerotier: ZeroTier {
                auth_token: "test_token".to_string(),
                address: "12312".to_string(),
            },
        };

        serde_json::to_writer(file, &config).unwrap();

        let config = AppConfig::init("test.json");

        assert_eq!(config.info.username, "admin");
        assert_eq!(config.info.password, "password");
        assert_eq!(config.listen, "127.0.0.1:5800");
        assert_eq!(config.zerotier.auth_token, "test_token");
        assert_eq!(config.zerotier.address, "12312");

        assert_ne!(config.info.username, "admin2");
        assert_ne!(config.info.password, "password2");
        assert_ne!(config.listen, "6351156");
        assert_ne!(config.zerotier.auth_token, "test_token2");
        assert_ne!(config.zerotier.address, "123122");

        std::fs::remove_file("test.json").unwrap();
    }
}
