use axum::{Router, routing::post}; 
use crate::web::handlers::*;
use crate::web::api_doc::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes() -> Router{
    Router::new()

        .route ("/api/v1/wallet/deposit", post(deposit_funds))
        .route("/api/v1/wallet/balance", post(get_balance))
        // Swagger UI
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", ApiDoc::openapi())
        )
}