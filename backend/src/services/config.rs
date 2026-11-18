use crate::error::{AppError, Result};
use crate::models::AppConfig;
use crate::utils::constant_time_eq;
use arc_swap::ArcSwap;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

const MIN_PASSWORD_CHARS: usize = 6;
const MAX_BCRYPT_PASSWORD_BYTES: usize = 72;
const MAX_USERNAME_CHARS: usize = 64;

#[derive(Clone)]
pub struct ConfigService {
    config: Arc<ArcSwap<AppConfig>>,
    config_path: String,
    write_lock: Arc<Mutex<()>>,
}

impl ConfigService {
    pub fn new(config_path: String) -> Result<Self> {
        let config_content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig = serde_json::from_str(&config_content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))?;

        validate_config(&config)?;

        Ok(Self {
            config: Arc::new(ArcSwap::new(Arc::new(config))),
            config_path,
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    pub fn get_config(&self) -> Arc<AppConfig> {
        self.config.load_full()
    }

    /// Verify both username and password without blocking the async runtime.
    pub async fn verify_user(&self, username: &str, password: &str) -> Result<bool> {
        let config = self.get_config();
        let expected_username = config.info.username.clone();
        let expected_hash = config.info.password.clone();
        let provided_username = username.to_string();
        let provided_password = password.to_string();

        tokio::task::spawn_blocking(move || {
            let username_ok =
                constant_time_eq(provided_username.as_bytes(), expected_username.as_bytes());
            let password_ok = verify_password_constant_time(&provided_password, &expected_hash);
            username_ok && password_ok
        })
        .await
        .map_err(|_| AppError::InternalServerError("Credential verification failed".to_string()))
    }

    /// Verify only the current administrator password without blocking the async runtime.
    pub async fn verify_current_password(&self, password: &str) -> Result<bool> {
        let expected_hash = self.get_config().info.password.clone();
        let provided_password = password.to_string();

        tokio::task::spawn_blocking(move || {
            verify_password_constant_time(&provided_password, &expected_hash)
        })
        .await
        .map_err(|_| AppError::InternalServerError("Password verification failed".to_string()))
    }

    /// Verify an API key in constant time across all configured keys.
    pub fn verify_api_key(&self, api_key: &str) -> bool {
        if api_key.is_empty() {
            return false;
        }

        let config = self.get_config();
        let mut matched = false;
        for configured_key in &config.api_keys {
            if !configured_key.is_empty() {
                matched |= constant_time_eq(configured_key.as_bytes(), api_key.as_bytes());
            }
        }
        matched
    }

    /// Validate, hash, and atomically persist updated administrator credentials.
    /// Returns the new bcrypt password hash on success.
    pub async fn update_user_info(&self, username: &str, password: &str) -> Result<String> {
        let username = username.trim();
        if username.is_empty() || username.chars().count() > MAX_USERNAME_CHARS {
            return Err(AppError::BadRequest(format!(
                "Username must be between 1 and {} characters",
                MAX_USERNAME_CHARS
            )));
        }

        if password.chars().count() < MIN_PASSWORD_CHARS {
            return Err(AppError::BadRequest(format!(
                "Password must be at least {} characters",
                MIN_PASSWORD_CHARS
            )));
        }

        if password.len() > MAX_BCRYPT_PASSWORD_BYTES {
            return Err(AppError::BadRequest(format!(
                "Password must not exceed {} bytes",
                MAX_BCRYPT_PASSWORD_BYTES
            )));
        }

        let _guard = self.write_lock.lock().await;

        let password_owned = password.to_string();
        let new_password_hash =
            tokio::task::spawn_blocking(move || hash(password_owned, DEFAULT_COST))
                .await
                .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?
                .map_err(|e| {
                    AppError::InternalServerError(format!("Failed to hash password: {}", e))
                })?;

        let mut config = (*self.get_config()).clone();
        config.info.username = username.to_string();
        config.info.password = new_password_hash.clone();

        let config_json = serde_json::to_string_pretty(&config)?;
        let config_path = self.config_path.clone();

        tokio::task::spawn_blocking(move || write_config_atomically(&config_path, &config_json))
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to write config file".to_string())
            })??;

        self.config.store(Arc::new(config));
        Ok(new_password_hash)
    }
}

fn validate_config(config: &AppConfig) -> Result<()> {
    if config.listen.trim().is_empty() {
        return Err(AppError::ConfigError(
            "Configuration 'listen' address cannot be empty".to_string(),
        ));
    }
    if config.info.username.trim().is_empty() {
        return Err(AppError::ConfigError(
            "Configuration 'info.username' cannot be empty".to_string(),
        ));
    }
    if config.info.password.trim().is_empty() {
        return Err(AppError::ConfigError(
            "Configuration 'info.password' cannot be empty".to_string(),
        ));
    }
    if config.zerotier.auth_token.trim().is_empty() {
        return Err(AppError::ConfigError(
            "Configuration 'zerotier.auth_token' cannot be empty".to_string(),
        ));
    }
    let address = config.zerotier.address.trim();
    if !address.starts_with("http://") && !address.starts_with("https://") {
        return Err(AppError::ConfigError(
            "Configuration 'zerotier.address' must start with http:// or https://".to_string(),
        ));
    }
    Ok(())
}

fn verify_password_constant_time(password: &str, password_hash: &str) -> bool {
    if password.is_empty() || password.len() > MAX_BCRYPT_PASSWORD_BYTES {
        // Perform a dummy bcrypt verify so timing stays comparable
        let _ = verify("dummy_password_for_timing", password_hash);
        return false;
    }
    verify(password, password_hash).unwrap_or(false)
}

fn write_config_atomically(config_path: &str, content: &str) -> Result<()> {
    let tmp_path = format!("{}.tmp", config_path);
    let existing_perms = std::fs::metadata(config_path).ok().map(|m| m.permissions());

    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    {
        let mut file = options.open(&tmp_path)?;
        file.write_all(content.as_bytes())?;
        let _ = file.sync_all();
    }

    if let Some(perms) = existing_perms {
        let _ = std::fs::set_permissions(&tmp_path, perms);
    }

    if std::fs::rename(&tmp_path, config_path).is_err() {
        let _ = std::fs::remove_file(&tmp_path);
        std::fs::write(config_path, content)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{UserInfo, ZeroTierConfig};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("ztvrui_test_config_{}_{}.json", suffix, nanos))
    }

    fn write_sample_config(path: &PathBuf) {
        let config = AppConfig {
            info: UserInfo {
                username: "admin".to_string(),
                password: hash("password", 4).unwrap(),
            },
            listen: "127.0.0.1:7000".to_string(),
            api_keys: vec!["valid-api-key-123".to_string()],
            trusted_proxies: None,
            jwt_secret: None,
            zerotier: ZeroTierConfig {
                auth_token: "zt-token-abc".to_string(),
                address: "http://127.0.0.1:9993".to_string(),
            },
        };
        std::fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
    }

    #[tokio::test]
    async fn verify_user_api_keys_and_update_user_info() {
        let path = temp_config_path("update");
        write_sample_config(&path);

        let service = ConfigService::new(path.to_string_lossy().to_string()).unwrap();

        assert!(service.verify_user("admin", "password").await.unwrap());
        assert!(!service.verify_user("admin", "wrong").await.unwrap());
        assert!(!service.verify_user("other", "password").await.unwrap());

        assert!(service.verify_api_key("valid-api-key-123"));
        assert!(!service.verify_api_key(""));
        assert!(!service.verify_api_key("invalid-key"));

        // Reject too short or > 72 byte passwords
        assert!(matches!(
            service.update_user_info("newadmin", "12345").await,
            Err(AppError::BadRequest(_))
        ));
        let too_long = "a".repeat(73);
        assert!(matches!(
            service.update_user_info("newadmin", &too_long).await,
            Err(AppError::BadRequest(_))
        ));

        let new_hash = service
            .update_user_info(" newadmin ", "strongpass123")
            .await
            .unwrap();
        assert!(!new_hash.is_empty());
        assert!(service
            .verify_user("newadmin", "strongpass123")
            .await
            .unwrap());
        assert!(!service.verify_user("admin", "password").await.unwrap());
        assert!(service
            .verify_current_password("strongpass123")
            .await
            .unwrap());

        let _ = std::fs::remove_file(path);
    }
}
