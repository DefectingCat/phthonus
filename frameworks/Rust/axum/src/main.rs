use std::{env, error::Error, net::SocketAddr};

use axum::Router;
use consts::DEFAULT_PORT;
use dotenvy::dotenv;
use routes::routes;
use tokio::net::TcpListener;
use tracing::info;
use utils::{init_logger, shutdown_signal};

mod consts;
mod error;
mod middlewares;
mod routes;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logger();

    let port = env::var("PHTHONUS_PORT")
        .map(|port| port.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal(shutdown))
        .await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {}

fn app() -> Router {
    Router::new().merge(routes())
}

fn shutdown() {
    info!("Server shuting down")
}
