use crate::handlers::*;
use crate::state::AppState;
use axum::{
    routing::{any, get, post},
    Router,
};

// Public API routes (no authentication required)
pub fn public_api_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

// Protected API routes (authentication required)
pub fn protected_api_routes() -> Router<AppState> {
    Router::new()
        .route("/check", get(check_auth))
        .route("/editprofile", post(update_profile))
        .route("/refresh", post(refresh_token))
}

// ZeroTier routes (authentication required)
pub fn zerotier_routes() -> Router<AppState> {
    Router::new().route("/*path", any(forward_to_zerotier))
}
