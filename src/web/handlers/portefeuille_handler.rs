use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;
use utoipa::ToSchema;


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

    match account_service::approvisionner(&payload.username, payload.amount).await {
        Ok(_) => (StatusCode::OK, Json(DepositResponse { message: "Dépôt effectué avec succès".to_string() })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(DepositResponse { message: format!("Erreur lors du dépôt: {}", e) })),
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
pub async fn get_balance(Json(payload ): Json<BalanceRequest>) -> (StatusCode, Json<BalanceResponse>){
    match account_service::voir_solde(&payload.username){
        Ok(balance) => (StatusCode::OK, Json(BalanceResponse { balance })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(BalanceResponse { balance: -1 })),
    }
}