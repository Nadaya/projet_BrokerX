use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;
use tracing::{info, error};

#[derive(Deserialize, ToSchema)]
pub struct DepositRequest{
    pub username: String,
    pub amount : i32,
}

#[derive(Serialize, ToSchema)]
pub struct DepositResponse{
    pub message: String, 
}

#[utoipa::path(
    post,
    path = "/deposit_funds",
    request_body = DepositRequest,
    responses(
        (status = 200, description = "Dépôt effectué avec succès")
    )
)]
pub async fn deposit_funds(Json(payload): Json<DepositRequest>) -> (StatusCode, Json<DepositResponse>) {
    info!(action = "deposit_attempt", username = %payload.username, amount = payload.amount, "Tentative de dépôt reçue");

    match account_service::approvisionner(&payload.username, payload.amount).await {
        Ok(_) => {
            info!(action = "deposit_success", username = %payload.username, amount = payload.amount, "Dépôt effectué avec succès");
            (StatusCode::OK, Json(DepositResponse { 
                message: "Dépôt effectué avec succès".to_string() 
            }))
        },
        Err(e) => {
            error!(action = "deposit_failed", username = %payload.username, amount = payload.amount, error = %e, "Erreur lors du dépôt");
            (StatusCode::BAD_REQUEST, Json(DepositResponse { 
                message: format!("Erreur lors du dépôt: {}", e) 
            }))
        },
    }
}

#[derive(Deserialize, ToSchema)]
pub struct BalanceRequest{
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct BalanceResponse{
    pub balance: i32,
}

#[utoipa::path(
    post,
    path = "/get_balance",
    request_body = BalanceRequest,
    responses(
        (status = 200, description = "Solde récupéré avec succès")
    )
)]
pub async fn get_balance(Json(payload ): Json<BalanceRequest>) -> (StatusCode, Json<BalanceResponse>) {
    info!(action = "balance_check", username = %payload.username, "Demande de solde reçue");

    match account_service::voir_solde(&payload.username) {
        Ok(balance) => {
            info!(action = "balance_success", username = %payload.username, balance = balance, "Solde récupéré avec succès");
            (StatusCode::OK, Json(BalanceResponse { balance }))
        },
        Err(e) => {
            error!(action = "balance_failed", username = %payload.username, error = %e, "Erreur lors de la récupération du solde");
            (StatusCode::BAD_REQUEST, Json(BalanceResponse { balance: -1 }))
        },
    }
}
