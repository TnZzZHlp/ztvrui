use crate::handlers::*;
use crate::state::AppState;
use axum::{
    routing::{any, get, post},
    Router,
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/check", get(check_auth))
        .route("/editprofile", post(update_profile))
}

pub fn zerotier_routes() -> Router<AppState> {
    Router::new()
        .route("/*path", any(forward_to_zerotier))
}

pub fn static_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(serve_static_files))
        .route("/*path", get(serve_static_files))
}
