use clap::Parser;
use tokio::net::TcpListener;

mod error;
mod handlers;
mod logger;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use services::ConfigService;
use state::AppState;

use crate::routes::app_routes;

#[derive(Parser)]
#[command(name = "backend")]
#[command(about = "ZeroTier UI Backend Server")]
struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,

    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger
    logger::init_logger(&args.log_level);

    // Initialize services
    let config_service = ConfigService::new(args.config)?;
    let app_state = AppState::new(config_service.clone());

    // Build the application router
    let app = app_routes(app_state.clone());

    // Get listen address from config
    let listen_addr = config_service.get_listen_address();
    tracing::info!("Starting server on {}", listen_addr);

    // Start the server
    let listener = TcpListener::bind(&listen_addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
