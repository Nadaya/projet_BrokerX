use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use crate::services::metrics::{HTTP_REQ_COUNTER, HTTP_REQ_HISTOGRAM, HTTP_ERR_COUNTER};
use utoipa::ToSchema;
use tracing::{info, error};
use prometheus::HistogramTimer;

#[derive(Deserialize, ToSchema)]
pub struct DeleteAccountRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct DeleteAccountResponse{
    pub success: bool, 
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/delete_account",
    request_body = DeleteAccountRequest,
    responses(
        (status = 200, description = "Compte supprimé avec succès")
    )
)]
pub async fn delete_account(Json(payload): Json<DeleteAccountRequest>) -> (StatusCode, Json<DeleteAccountResponse>){

    // Début du timer Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/delete_account"])
        .start_timer();
    
    // Incrément du compteur de requêtes
    HTTP_REQ_COUNTER
        .with_label_values(&["POST", "/delete_account"])
        .inc();

    info!(action = "delete_account", username = %payload.username, "Requête de suppression reçue");
    
    let res = account_service::delete_account(&payload.username, &payload.password);

    match &res {
        Ok(_) => {
            info!(action = "delete_account", username = %payload.username, status = "success", "Compte supprimé avec succès");
        },
        Err(e) => {
            error!(action = "delete_account", username = %payload.username, error = %e, "Erreur lors de la suppression");
            HTTP_ERR_COUNTER
                .with_label_values(&["POST", "/delete_account", "400"])
                .inc();
        }
    }

    // Fin du timer
    timer.observe_duration();

    match res {
        Ok(_) => (
            StatusCode::OK, Json(DeleteAccountResponse {
                success: true,
                message : format!("Compte {} supprimé avec succès", payload.username),
            })
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST, Json(DeleteAccountResponse {
                success : false,
                message: format!("Erreur lors de la suppression du compte {} : {}", payload.username, e),
            })
        )
    }
}
