use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::services::*;

#[derive(Deserialize)]
pub struct DepositRequest{
    pub username: String,
    pub amount : i32,
}

#[derive(Serialize)]
pub struct DepositResponse{
    pub message: String, 
}


pub async fn deposit_funds(Json(payload): Json<DepositRequest>) -> (StatusCode, Json<DepositResponse>) {

    match account_service::approvisionner(&payload.username, payload.amount).await {
        Ok(_) => (StatusCode::OK, Json(DepositResponse { message: "Dépôt effectué avec succès".to_string() })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(DepositResponse { message: format!("Erreur lors du dépôt: {}", e) })),
    }
}

#[derive(Deserialize)]
pub struct BalanceRequest{
    pub username: String,
}

#[derive(Serialize)]
pub struct BalanceResponse{
    pub balance: i32,
}

pub async fn get_balance(Json(payload ): Json<BalanceRequest>) -> (StatusCode, Json<BalanceResponse>){
    match account_service::voir_solde(&payload.username){
        Ok(balance) => (StatusCode::OK, Json(BalanceResponse { balance })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(BalanceResponse { balance: -1 })),
    }
}