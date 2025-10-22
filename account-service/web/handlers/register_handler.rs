use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use crate::services::metrics::{HTTP_REQ_COUNTER, HTTP_REQ_HISTOGRAM, HTTP_ERR_COUNTER};
use utoipa::ToSchema;
use tracing::{info, error};
use prometheus::HistogramTimer;

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String, 
    pub password: String,
    pub email: String, 
    pub phone: String,
    pub name: String,
    pub mfa_enabled: bool,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse{
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Utilisateur créé avec succès")
    )
)]
pub async fn register_user(Json(payload): Json<RegisterRequest>) -> (StatusCode, Json<RegisterResponse>) {

    // Metrics Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/register"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/register"]).inc();

    info!(action = "register_user_attempt", username = %payload.username, "Tentative de création de compte reçue");

    let res = account_service::create_client_and_account(
        &payload.name,
        &payload.email,
        &payload.phone,
        &payload.username,
        &payload.password,
        payload.mfa_enabled,
    );

    match &res {
        Ok(_) => info!(action = "register_success", username = %payload.username, "Utilisateur enregistré avec succès"),
        Err(e) => {
            error!(action = "register_failed", username = %payload.username, error = %e, "Erreur lors de la création du compte");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/register", "400"]).inc();
        }
    }

    timer.observe_duration();

    match res {
        Ok(_) => (StatusCode::CREATED, Json(RegisterResponse { 
            message: "Utilisateur enregistré".to_string() 
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(RegisterResponse { 
            message: format!("Erreur: {}", e) 
        })),
    }
}
