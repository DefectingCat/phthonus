use dotenvy::dotenv;
use tracing::info;
use utils::init_logger;

mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logger();
    info!("Hello, world!");
}
