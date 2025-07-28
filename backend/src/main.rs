mod config;
mod zerotier;

mod statics;

use statics::index;

mod api;
use api::*;

use clap::Parser;
use salvo::logging::Logger;
use salvo::prelude::*;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::Instant;

lazy_static::lazy_static! {
    static ref CONFIG: RwLock<config::AppConfig> = RwLock::new(
        config::AppConfig::init(Args::parse().config)
    );
    static ref ZEROTIER: RwLock<zerotier::ZeroTier> = RwLock::new(zerotier::ZeroTier::new());
    static ref CONFIG_PATH: String = Args::parse().config;
    static ref COOKIE: RwLock<HashMap<String, Instant>> = RwLock::new(HashMap::new());
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = CONFIG.read().await;

    // Initialize ZeroTier
    {
        let mut zerotier = ZEROTIER.write().await;
        zerotier.init(&config.zerotier);
        drop(zerotier);
    }

    let listen = config.listen.clone();
    drop(config);

    let router = Router::new()
        .push(
            Router::with_path("api")
                .push(Router::with_path("login").post(login))
                .push(Router::with_path("logout").hoop(auth).get(logout))
                .push(Router::with_path("check").hoop(auth).get(check))
                .push(Router::with_path("editprofile").hoop(auth).post(modify)),
        )
        .push(
            Router::with_path("ztapi/{**}")
                .hoop(auth)
                .goal(forward_to_zt),
        )
        .push(Router::with_path("{**}").get(index));
    let service = Service::new(router).hoop(Logger::new());
    let acceptor = TcpListener::new(listen).bind().await;
    Server::new(acceptor).serve(service).await;
}

#[handler]
async fn forward_to_zt(req: &mut Request, res: &mut Response) {
    let result = (async {
        let path = req.uri().path().replace("/ztapi/", "");
        let body = req.parse_json::<serde_json::Value>().await.ok();

        let zt_response = ZEROTIER
            .read()
            .await
            .forward(&path, req.method().clone(), body)
            .await
            .map_err(|e| StatusError::bad_request().brief(e.to_string()))?;

        let zt_status = zt_response.status();

        let response_body = zt_response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| StatusError::internal_server_error().brief("Failed to parse response"))?;

        Ok::<_, StatusError>((zt_status, response_body))
    })
    .await;

    match result {
        Ok((status, body)) => {
            res.status_code(status);
            res.render(Json(body));
        }
        Err(err) => res.render(err),
    }
}
