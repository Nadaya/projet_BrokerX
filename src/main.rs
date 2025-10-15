// mod domain;   
// mod infrastructure;

// mod ui;
// mod services;

// use diesel::PgConnection;
// use diesel::Connection;

// fn main(){
//     dotenv::dotenv().ok();
//     let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
//     let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
//     let host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
//     let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB not set");
//     let port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");

//     let database_url: &str = &format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);
//     let mut conn = PgConnection::establish(database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

//     ui::menu_principal(&mut conn);
// }
mod domain;
mod infrastructure;
mod services;
mod web;

use diesel::{Connection, PgConnection};
use axum::{Router, routing::get, serve};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB not set");
    let port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");

    let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);
    let _conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("Connexion to PostgreSQL established.");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router<()> = Router::new()
        .route("/", get(|| async { "🚀 BrokerX+ API est en ligne !" }))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("HTTP server launched on http://{}", addr);

    // lance le serveur
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
