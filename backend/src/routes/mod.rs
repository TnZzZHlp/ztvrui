use crate::error::AppError;
use crate::handlers::{self, *};
use crate::state::AppState;
use axum::{
    extract::{DefaultBodyLimit, Request},
    http::{header, HeaderValue},
    middleware::{from_fn, from_fn_with_state, Next},
    response::Response,
    routing::{any, post},
    Router,
};

const MAX_BODY_BYTES: usize = 2 * 1024 * 1024; // 2 MiB

async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'none'",
        ),
    );
    response
}

async fn no_store_cache_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

pub fn app_routes(app_state: AppState) -> Router {
    let protected_api_routes = Router::new()
        .route("/editprofile", post(update_profile))
        .route("/refresh", post(refresh_token))
        .layer(from_fn_with_state(
            app_state.clone(),
            crate::middleware::auth_middleware,
        ));

    let api_routes = Router::new()
        .route("/login", post(login))
        .merge(protected_api_routes)
        .fallback(|| async { AppError::NotFound })
        .layer(from_fn(no_store_cache_middleware));

    let zerotier_routes = Router::new()
        .route("/{*wildcard}", any(forward_to_zerotier))
        .fallback(|| async { AppError::NotFound })
        .layer(from_fn_with_state(
            app_state.clone(),
            crate::middleware::auth_or_api_key_middleware,
        ))
        .layer(from_fn(no_store_cache_middleware));

    Router::new()
        .nest("/api", api_routes)
        .nest("/ztapi", zerotier_routes)
        .fallback(handlers::serve_static_files)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(from_fn(security_headers_middleware))
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
