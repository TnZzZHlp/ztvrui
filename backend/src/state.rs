use crate::services::{AuthService, ConfigService, IpBanService, ZeroTierService};

#[derive(Clone)]
pub struct AppState {
    pub config: ConfigService,
    pub auth: AuthService,
    pub zerotier: ZeroTierService,
    pub ip_ban: IpBanService,
}

impl AppState {
    pub fn new(config: ConfigService) -> Self {
        let zerotier_config = config.get_config().zerotier.clone();
        let auth = AuthService::new(zerotier_config.auth_token.clone());
        let zerotier = ZeroTierService::new(zerotier_config);

        Self {
            config,
            auth,
            zerotier,
            ip_ban: IpBanService::new(),
        }
    }
}
