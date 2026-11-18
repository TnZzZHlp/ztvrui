use crate::services::{AuthService, ConfigService, IpBanService, ZeroTierService};
use crate::utils::extract_real_ip;
use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};

#[derive(Clone)]
pub struct AppState {
    pub config: ConfigService,
    pub auth: AuthService,
    pub zerotier: ZeroTierService,
    pub ip_ban: IpBanService,
}

impl AppState {
    pub fn new(config: ConfigService) -> Self {
        let app_config = config.get_config();
        let base_secret = app_config
            .jwt_secret
            .clone()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| app_config.zerotier.auth_token.clone());

        let auth = AuthService::new(
            base_secret,
            &app_config.info.username,
            &app_config.info.password,
        );
        let zerotier = ZeroTierService::new(app_config.zerotier.clone());

        Self {
            config,
            auth,
            zerotier,
            ip_ban: IpBanService::new(),
        }
    }

    pub fn extract_client_ip(&self, headers: &HeaderMap, fallback_addr: SocketAddr) -> IpAddr {
        let config = self.config.get_config();
        extract_real_ip(headers, fallback_addr, config.trusted_proxies.as_deref())
    }
}
