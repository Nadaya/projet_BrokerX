use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;

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
    match account_service::delete_account(&payload.username, &payload.password){
        Ok(_) => (StatusCode::OK, Json(DeleteAccountResponse {
            success: true,
            message : format!("Compte {} supprimé avec succès", payload.username),
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(DeleteAccountResponse {
            success : false,
            message: format!("Erreur lors de la suppression du compte {} : {}", payload.username, e),
        }))
    }

}