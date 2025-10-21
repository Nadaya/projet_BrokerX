use axum::{Json};
use serde::{Deserialize, Serialize};
use crate::services::*;

#[derive(Deserialize)]
pub struct DeleteAccountRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct DeleteAccountResponse{
    pub success: bool, 
    pub message: String,
}

pub async fn delete_account(Json(payload): Json<DeleteAccountRequest>) -> Json<DeleteAccountResponse>{
    match account_service::delete_account(&payload.username, &payload.password){
        Ok(_) => Json(DeleteAccountResponse {
            success: true,
            message : format!("Compte {} supprimé avec succès", payload.username),
        }),
        Err(e) => Json(DeleteAccountResponse {
            success : false,
            message: format!("Erreur lors de la suppression du compte {} : {}", payload.username, e),
        })
    }

}