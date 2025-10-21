use axum::{Router, routing::post}; 
use crate::web::handlers::*;
use crate::web::api_doc::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes() -> Router{
    Router::new()
        .route("/api/v1/register", post(register_user))
        .route("/api/v1/auth/verify_mfa", post(verify_mfa_user))
        .route("/api/v1/auth/login", post(login_user))
        .route ("/api/v1/wallet/deposit", post(deposit_funds))
        .route("/api/v1/wallet/balance", post(get_balance))
        .route("/api/v1/account/delete", post(delete_account))
        // Swagger UI
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", ApiDoc::openapi())
        )
}