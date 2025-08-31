use crate::error::{AppError, Result};
use crate::models::AppConfig;
use arc_swap::ArcSwap;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use tokio::fs;

#[derive(Clone)]
pub struct ConfigService {
    config: Arc<ArcSwap<AppConfig>>,
    config_path: String,
}

impl ConfigService {
    pub fn new(config_path: String) -> Result<Self> {
        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig = serde_json::from_str(&config_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))?;

        Ok(Self {
            config: Arc::new(ArcSwap::new(Arc::new(config))),
            config_path,
        })
    }

    pub fn get_config(&self) -> Arc<AppConfig> {
        self.config.load_full()
    }

    pub async fn verify_user(&self, username: &str, password: &str) -> bool {
        let config = self.get_config();
        username == config.info.username && verify(password, &config.info.password).unwrap_or(false)
    }

    pub async fn update_user_info(&self, username: &str, password: &str) -> Result<()> {
        let mut config = (*self.get_config()).clone();

        config.info.username = username.to_string();
        config.info.password = hash(password, DEFAULT_COST).map_err(|e| {
            AppError::InternalServerError(format!("Failed to hash password: {}", e))
        })?;

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, config_json).await?;

        self.config.store(Arc::new(config));
        Ok(())
    }

    pub fn get_listen_address(&self) -> String {
        self.get_config().listen.clone()
    }

    pub fn get_zerotier_config(&self) -> crate::models::ZeroTierConfig {
        self.get_config().zerotier.clone()
    }
}
