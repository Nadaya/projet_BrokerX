use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use crate::services::metrics::{HTTP_REQ_COUNTER, HTTP_REQ_HISTOGRAM, HTTP_ERR_COUNTER};
use utoipa::ToSchema;
use tracing::{info, error};
use prometheus::HistogramTimer;

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

    // Metrics Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/deposit_funds"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/deposit_funds"]).inc();

    info!(action = "deposit_attempt", username = %payload.username, amount = payload.amount, "Tentative de dépôt reçue");

    let res = portefeuille_service::approvisionner(&payload.username, payload.amount).await;

    match &res {
        Ok(_) => info!(action = "deposit_success", username = %payload.username, amount = payload.amount, "Dépôt effectué avec succès"),
        Err(e) => {
            error!(action = "deposit_failed", username = %payload.username, amount = payload.amount, error = %e, "Erreur lors du dépôt");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/deposit_funds", "400"]).inc();
        }
    }

    timer.observe_duration();

    match res {
        Ok(_) => (StatusCode::OK, Json(DepositResponse { 
            message: "Dépôt effectué avec succès".to_string() 
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(DepositResponse { 
            message: format!("Erreur lors du dépôt: {}", e) 
        })),
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

    // Metrics Prometheus
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/get_balance"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/get_balance"]).inc();

    info!(action = "balance_check", username = %payload.username, "Demande de solde reçue");

    let res = portefeuille_service::voir_solde(&payload.username);

    match &res {
        Ok(balance) => info!(action = "balance_success", username = %payload.username, balance = *balance, "Solde récupéré avec succès"),
        Err(e) => {
            error!(action = "balance_failed", username = %payload.username, error = %e, "Erreur lors de la récupération du solde");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/get_balance", "400"]).inc();
        }
    }

    timer.observe_duration();

    match res {
        Ok(balance) => (StatusCode::OK, Json(BalanceResponse { balance })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(BalanceResponse { balance: -1 })),
    }
}
