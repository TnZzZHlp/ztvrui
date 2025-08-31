use axum::{middleware::from_fn_with_state, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use services::ConfigService;
use state::AppState;

#[derive(Parser)]
#[command(name = "backend")]
#[command(about = "ZeroTier UI Backend Server")]
struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let args = Args::parse();

    // Initialize services
    let config_service = ConfigService::new(args.config)?;
    let app_state = AppState::new(config_service.clone());

    // Build the application router
    let app = Router::new()
        // Public API routes
        .nest("/api", routes::public_api_routes())
        // Protected API routes with authentication middleware
        .nest(
            "/api",
            routes::protected_api_routes().layer(from_fn_with_state(
                app_state.clone(),
                crate::middleware::auth_middleware,
            )),
        )
        // ZeroTier routes with authentication middleware
        .nest(
            "/ztapi",
            routes::zerotier_routes().layer(from_fn_with_state(
                app_state.clone(),
                crate::middleware::auth_middleware,
            )),
        )
        .fallback(handlers::serve_static_files)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Get listen address from config
    let listen_addr = config_service.get_listen_address();
    tracing::info!("Starting server on {}", listen_addr);

    // Start the server
    let listener = TcpListener::bind(&listen_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
