use crate::handlers::{self, *};
use crate::state::AppState;
use axum::{
    middleware::from_fn_with_state,
    routing::{any, get, post},
    Router,
};

// Public API routes (no authentication required)
pub fn public_api_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
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
    Router::new().route("/{*wildcard}", any(forward_to_zerotier))
}

pub fn app_routes(app_state: AppState) -> Router {
    Router::new()
        // Public API routes
        .nest("/api", public_api_routes())
        // Protected API routes with authentication middleware
        .nest(
            "/api",
            protected_api_routes().layer(from_fn_with_state(
                app_state.clone(),
                crate::middleware::auth_middleware,
            )),
        )
        // ZeroTier routes with authentication middleware
        .nest(
            "/ztapi",
            zerotier_routes().layer(from_fn_with_state(
                app_state.clone(),
                crate::middleware::auth_middleware,
            )),
        )
        .fallback(handlers::serve_static_files)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                )
                .on_failure(
                    tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR),
                ),
        )
        .with_state(app_state)
}
