use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/health", get(|| async { "✅ API OK" }))
}
