mod domain;
mod infrastructure;
mod services;
mod web;

use tracing::{info};
use tracing_subscriber::fmt;
use tracing::Level;
use dotenv;

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();

        // Initialiser le logging JSON structuré
    tracing_subscriber::fmt()
        .event_format(fmt::format().json()) // <-- format JSON
        .with_max_level(Level::INFO)
        .init();

    info!(action = "startup", service = "BrokerX+", "Service démarré");


    web::run().await;
}
