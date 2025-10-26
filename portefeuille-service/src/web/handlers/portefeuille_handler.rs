use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::service::*;
use crate::web::metrics::{HTTP_REQ_COUNTER, HTTP_REQ_HISTOGRAM, HTTP_ERR_COUNTER};
use utoipa::ToSchema;
use tracing::{info, error};
use prometheus::HistogramTimer;
use crate::service::db::get_conn;
use crate::domain::portefeuille::Portefeuille;

#[derive(Serialize, ToSchema)]
pub struct CreatePortefeuilleResponse {
    pub portefeuille_id: i32,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/create",
    responses(
        (status = 200, description = "Portefeuille créé avec succès", body = CreatePortefeuilleResponse),
        (status = 400, description = "Erreur lors de la création du portefeuille")
    )
)]
pub async fn create_portefeuille() -> (StatusCode, Json<CreatePortefeuilleResponse>) {
    // 🔹 Démarrer la mesure de performance
    let timer: HistogramTimer = HTTP_REQ_HISTOGRAM
        .with_label_values(&["POST", "/create"])
        .start_timer();
    HTTP_REQ_COUNTER.with_label_values(&["POST", "/create"]).inc();

    info!(action = "create_portefeuille_attempt", "Tentative de création de portefeuille reçue");

    let mut conn = get_conn();

    // 🔹 Tentative de création du portefeuille
    let res = Portefeuille::create_portefeuille(&mut conn, 0);

    match &res {
        Ok(p) => info!(action = "create_portefeuille_success", portefeuille_id = p.portefeuille_id, "Portefeuille créé avec succès"),
        Err(e) => {
            error!(action = "create_portefeuille_failed", error = %e, "Erreur lors de la création du portefeuille");
            HTTP_ERR_COUNTER.with_label_values(&["POST", "/create", "400"]).inc();
        }
    }

    timer.observe_duration();

    match res {
        Ok(p) => (
            StatusCode::OK,
            Json(CreatePortefeuilleResponse {
                portefeuille_id: p.portefeuille_id,
                message: "Portefeuille créé avec succès".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(CreatePortefeuilleResponse {
                portefeuille_id: -1,
                message: format!("Erreur lors de la création du portefeuille : {}", e),
            }),
        ),
    }
}


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

    let res = portefeuille_service::voir_solde(&payload.username).await;

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
