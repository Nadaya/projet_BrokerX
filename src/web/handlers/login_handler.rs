use axum::{Json};
use serde::{Deserialize, Serialize};
use crate::services::*;

#[derive(Deserialize)]
pub struct LoginRequest{
    pub username: String, 
    pub password: String, 
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub message: String, 
    pub mfa_required: bool,
    pub success: bool,
}

pub async fn login_user(Json(payload): Json<LoginRequest>) -> Json<LoginResponse>{    
    match auth::login(&payload.username, &payload.password).await {
        Ok(Some(account)) => {
            if account.status != "Active" {
                return Json(LoginResponse {
                    success: false,
                    message: format!("Compte {} non actif (état: {})", account.username, account.status),
                    mfa_required: false,
                });
            }
            if account.mfa_enabled {
                let otp = mfa::send_otp(&account.username);
                println!("[SIMULATION] Code OTP envoyé à {}: {}", account.username, otp);

                Json(LoginResponse {
                    success: true,
                    message: "MFA requis. Un code OTP a été envoyé.".to_string(),
                    mfa_required: true,
                })
            }else{
                Json(LoginResponse {
                    success: true,
                    message: format!("Connexion réussie, bienvenue {}!", account.username),
                    mfa_required: false,
                })
            }
        }Ok(None) => Json(LoginResponse {
            success: false,
            message: "Identifiants invalides".to_string(),
            mfa_required: false,
        }),
        Err(err) => Json(LoginResponse {
            success: false,
            message: format!("Erreur lors du login: {}", err),
            mfa_required: false,
        }),    
    }
}

#[derive(Deserialize)]
pub struct VerifyMfaRequest {
    pub username: String,
    pub otp: String,
}

#[derive(Serialize)]
pub struct VerifyMfaResponse {
    pub success: bool,
    pub message: String,
}

pub async fn verify_mfa_user(Json(payload): Json<VerifyMfaRequest>) -> Json<VerifyMfaResponse> {
    if mfa::verify_otp(&payload.username, &payload.otp) {
        Json(VerifyMfaResponse {
            success: true,
            message: "MFA vérifié, connexion réussie.".to_string(),
        })
    } else {
        Json(VerifyMfaResponse {
            success: false,
            message: "Code OTP invalide ou expiré.".to_string(),
        })
    }
}
