use axum::{Router, routing::get, serve};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::middleware::from_fn;

use brokerx_auth::middleware::basic_auth;

pub mod routes;
pub mod handlers;
pub mod api_doc;

use prometheus::Encoder;

async fn metrics_handler() -> String {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

pub async fn run() {
    // CORS global
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Routes protégées par le middleware Basic Auth
    let protected_routes = routes::routes()
        .layer(from_fn(basic_auth));

    let app: Router = Router::new()
        .route("/", get(|| async { "Portefeuille-Service UP!" }))
        .route("/metrics", get(metrics_handler))
        .merge(protected_routes)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("HTTP server launched on http://{}", addr);

    // Lancement du serveur
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
