use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;
use tracing::{info, error};

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
    info!(action = "delete_account", username = %payload.username, "Requête de suppression reçue");
    
    match account_service::delete_account(&payload.username, &payload.password){
        Ok(_) => {
            info!(action = "delete_account", username = %payload.username, status = "success", "Compte supprimé avec succès");
            (
                StatusCode::OK, Json(DeleteAccountResponse {
                    success: true,
                    message : format!("Compte {} supprimé avec succès", payload.username),
                })
            )
        },
        Err(e) => {
            error!(action = "delete_account", username = %payload.username, error = %e, "Erreur lors de la suppression");
            (
                StatusCode::BAD_REQUEST, Json(DeleteAccountResponse {
                success : false,
                message: format!("Erreur lors de la suppression du compte {} : {}", payload.username, e),
                })
            )
        }
    }
}