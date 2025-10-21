use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest{
    pub username: String, 
    pub password: String, 
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse{
    pub message: String, 
    pub mfa_required: bool,
    pub success: bool,
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Connexion réussie")
    )
)]
pub async fn login_user(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>){    
    match auth::login(&payload.username, &payload.password).await {
        Ok(Some(account)) => {
            if account.status != "Active" {
                return (StatusCode::BAD_REQUEST,Json(LoginResponse {
                    success: false,
                    message: format!("Compte {} non actif (état: {})", account.username, account.status),
                    mfa_required: false,
                }));
            }
            if account.mfa_enabled {
                let otp = mfa::send_otp(&account.username);
                println!("[SIMULATION] Code OTP envoyé à {}: {}", account.username, otp);

                (StatusCode::OK, Json(LoginResponse {
                    success: true,
                    message: "MFA requis. Un code OTP a été envoyé.".to_string(),
                    mfa_required: true,
                }))
            }else{
                (StatusCode::OK, Json(LoginResponse {
                    success: true,
                    message: format!("Connexion réussie, bienvenue {}!", account.username),
                    mfa_required: false,
                }))
            }
        }Ok(None) => (StatusCode::BAD_REQUEST, Json(LoginResponse {
            success: false,
            message: "Identifiants invalides".to_string(),
            mfa_required: false,
        })),
        Err(err) => (StatusCode::BAD_REQUEST, Json(LoginResponse {
            success: false,
            message: format!("Erreur lors du login: {}", err),
            mfa_required: false,
        })),    
    }
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyMfaRequest {
    pub username: String,
    pub otp: String,
}

#[derive(Serialize, ToSchema)]
pub struct VerifyMfaResponse {
    pub success: bool,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/verify_mfa",
    request_body = VerifyMfaRequest,
    responses(
        (status = 200, description = "Vérification MFA réussie")
    )
)]
pub async fn verify_mfa_user(Json(payload): Json<VerifyMfaRequest>) -> (StatusCode, Json<VerifyMfaResponse>) {
    if mfa::verify_otp(&payload.username, &payload.otp) {
        (StatusCode::OK, Json(VerifyMfaResponse {
            success: true,
            message: "MFA vérifié, connexion réussie.".to_string(),
        }))
    } else {
        (StatusCode::BAD_REQUEST, Json(VerifyMfaResponse {
            success: false,
            message: "Code OTP invalide ou expiré.".to_string(),
        }))
    }
}
