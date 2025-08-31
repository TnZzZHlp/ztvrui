use axum::Router;
use clap::Parser;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Initialize services
    let config_service = ConfigService::new(args.config)?;
    let app_state = AppState::new(config_service.clone());

    // Start background task to cleanup expired sessions
    let auth_service_cleanup = app_state.auth.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Cleanup every hour
        loop {
            interval.tick().await;
            auth_service_cleanup.cleanup_expired_sessions().await;
        }
    });

    // Build the application router
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .nest("/ztapi", routes::zerotier_routes())
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
