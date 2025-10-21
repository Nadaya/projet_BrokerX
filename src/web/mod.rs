use axum::{Router, routing::get, serve};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod routes;
pub mod handlers;

pub async fn run() {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router<()> = Router::new()
        .route("/", get(|| async { "🚀 BrokerX+ API est en ligne !" }))
        .merge(routes::routes())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("HTTP server launched on http://{}", addr);

    // lance le serveur
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

}