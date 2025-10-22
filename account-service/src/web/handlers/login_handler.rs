use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::service::*;
use crate::service::metrics::{HTTP_REQ_COUNTER, HTTP_REQ_HISTOGRAM, HTTP_ERR_COUNTER};
use utoipa::ToSchema;
use tracing::{info, error};
use prometheus::HistogramTimer;

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
    // Metrics Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/login"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/login"]).inc();

    info!(action = "login_attempt", username = %payload.username, "Tentative de connexion reçue");

    let res = login::login(&payload.username, &payload.password).await;

    match &res {
        Ok(Some(account)) => {
            if account.status != "Active" {
                error!(action = "login_failed", username = %account.username, status = %account.status, "Compte non actif");
                HTTP_ERR_COUNTER.with_label_values(&["POST", "/login", "400"]).inc();
            }
        },
        Ok(None) => {
            error!(action = "login_failed", username = %payload.username, "Identifiants invalides");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/login", "400"]).inc();
        },
        Err(err) => {
            error!(action = "login_error", username = %payload.username, error = %err, "Erreur lors du login");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/login", "500"]).inc();
        },
    }

    timer.observe_duration();

    match res {
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
                info!(action = "mfa_sent", username = %account.username, otp = %otp, "Code OTP envoyé");

                (StatusCode::OK, Json(LoginResponse {
                    success: true,
                    message: "MFA requis. Un code OTP a été envoyé.".to_string(),
                    mfa_required: true,
                }))
            } else {
                info!(action = "login_success", username = %account.username, "Connexion réussie sans MFA");
                (StatusCode::OK, Json(LoginResponse {
                    success: true,
                    message: format!("Connexion réussie, bienvenue {}!", account.username),
                    mfa_required: false,
                }))
            }
        }
        Ok(None) => {
            (StatusCode::BAD_REQUEST, Json(LoginResponse {
                success: false,
                message: "Identifiants invalides".to_string(),
                mfa_required: false,
            }))
        },
        Err(err) => {
            (StatusCode::BAD_REQUEST, Json(LoginResponse {
                success: false,
                message: format!("Erreur lors du login: {}", err),
                mfa_required: false,
            }))
        },
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
    // Metrics Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/verify_mfa"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/verify_mfa"]).inc();

    info!(action = "verify_mfa_attempt", username = %payload.username, "Tentative de vérification MFA reçue");

    let res = if mfa::verify_otp(&payload.username, &payload.otp) { Ok(true) } else { Err(()) };

    match &res {
        Ok(_) => info!(action = "verify_mfa_success", username = %payload.username, "MFA vérifié avec succès"),
        Err(_) => {
            error!(action = "verify_mfa_failed", username = %payload.username, "Code OTP invalide ou expiré");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/verify_mfa", "400"]).inc();
        }
    }

    timer.observe_duration();

    match res {
        Ok(_) => (StatusCode::OK, Json(VerifyMfaResponse {
            success: true,
            message: "MFA vérifié, connexion réussie.".to_string(),
        })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(VerifyMfaResponse {
            success: false,
            message: "Code OTP invalide ou expiré.".to_string(),
        })),
    }
}
