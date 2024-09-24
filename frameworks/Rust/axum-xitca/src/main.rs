use std::env;

use consts::{DEFAULT_PORT, NAME};
use dotenvy::dotenv;
use tower_compat::TowerHttp;
use utils::init_logger;

mod consts;
mod error;
mod middlewares;
mod routes;
mod tower_compat;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_logger();

    let service = TowerHttp::service(|| async { Ok(routes::routes()) });

    let port = env::var("PHTHONUS_PORT")
        .map(|port| port.parse::<u16>().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);

    xitca_server::Builder::new()
        .bind(NAME, format!("0.0.0.0:{port}"), service)?
        .build()
        .await
}
