use axum::{Json};
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


pub async fn deposit_funds(Json(payload): Json<DepositRequest>) -> Json<DepositResponse> {

    match account_service::approvisionner(&payload.username, payload.amount).await {
        Ok(_) => Json(DepositResponse { message: "Dépôt effectué avec succès".to_string() }),
        Err(e) => Json(DepositResponse { message: format!("Erreur lors du dépôt: {}", e) }),
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

pub async fn get_balance(Json(payload ): Json<BalanceRequest>) -> Json<BalanceResponse>{
    match account_service::voir_solde(&payload.username){
        Ok(balance) => Json(BalanceResponse { balance }),
        Err(e) => Json(BalanceResponse { balance: -1 }), 
    }
}