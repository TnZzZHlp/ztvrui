use crate::services::{AuthService, ConfigService, ZeroTierService};
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub config: ConfigService,
    pub auth: AuthService,
    pub zerotier: ZeroTierService,
}

impl AppState {
    pub fn new(config: ConfigService) -> Self {
        let auth = AuthService::new(config.get_zerotier_config().auth_token);
        let zerotier = ZeroTierService::new(config.get_zerotier_config());

        Self {
            config,
            auth,
            zerotier,
        }
    }
}

// Implement FromRef for each service so they can be extracted individually
impl FromRef<AppState> for ConfigService {
    fn from_ref(app_state: &AppState) -> ConfigService {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for AuthService {
    fn from_ref(app_state: &AppState) -> AuthService {
        app_state.auth.clone()
    }
}

impl FromRef<AppState> for ZeroTierService {
    fn from_ref(app_state: &AppState) -> ZeroTierService {
        app_state.zerotier.clone()
    }
}
