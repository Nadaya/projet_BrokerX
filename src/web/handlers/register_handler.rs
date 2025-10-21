use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;
use tracing::{info, error};

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
    info!(action = "register user attemp", username = %payload.username, "Tentative de création de compte reçue");


    match account_service::create_client_and_account(&payload.name, &payload.email, &payload.phone, &payload.username, &payload.password, payload.mfa_enabled ) {
        Ok(_) => {
            info!(action = "register_success", username = %payload.username, "Utilisateur enregistré avec succès");
            (
                StatusCode::OK, Json(RegisterResponse { 
                    message: "Utilisateur enregistré".to_string() 
                })
            )
        },
        Err(e) => {
            error!(action = "register_failed", username = %payload.username, error = %e, "Erreur lors de la création du compte");
            (
                StatusCode::BAD_REQUEST, Json(RegisterResponse { 
                    message: format!("Erreur: {}", e) 
                })
            )
        }   
    }
}