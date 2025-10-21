use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String, 
    pub password: String,
    pub email: String, 
    pub phone: String,
    pub name: String,
    pub mfa_enabled: bool,
}

#[derive(Serialize)]
pub struct RegisterResponse{
    pub message: String,
}

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> (StatusCode, Json<RegisterResponse>) {

    match account_service::create_client_and_account(&payload.name, &payload.email, &payload.phone, &payload.username, &payload.password, payload.mfa_enabled ) {
        Ok(_) => (StatusCode::OK, Json(RegisterResponse { message: "Utilisateur enregistré".to_string() })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(RegisterResponse { message: format!("Erreur: {}", e) })),
    }
}