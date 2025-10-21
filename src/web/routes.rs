use axum::{Router, routing::get, routing::post}; 
use crate::web::handlers::*;

pub fn routes() -> Router{
    Router::new()
        .route("/api/v1/register", post(register_user))
        .route("/api/v1/auth/verify_mfa", post(verify_mfa_user))
        .route("/api/v1/auth/login", post(login_user))
        .route ("/api/v1/wallet/deposit", post(deposit_funds))
        .route("/api/v1/wallet/balance", post(get_balance))
}