use crate::domain::client::Client; 
use crate::domain::account::Account;
use crate::service::db::get_conn;
use reqwest::Client as HttpClient;
use serde::{Deserialize};


#[derive(Deserialize)]
struct PortefeuilleResponse {
    portefeuille_id: i32,
}

pub async fn create_client_and_account(
    name: &str,
    email: &str,
    phone: &str,
    username: &str,
    password: &str,
    mfa_enabled: bool,
) -> Result<i32, String> {
    let mut conn = get_conn();

    // Création du client
    let client = Client::create_client(&mut conn, name, email, phone)
        .map_err(|e| format!("Erreur client: {}", e))?;

    // Création du portefeuille
    let portfeuille = HttpClient::new();
    let resp =  portfeuille
        .post("http://portefeuille-service:8080/api/v1/wallet/create")
        .send()
        .await
        .map_err(|e| format!("Erreur appel portefeuille-service: {}", e))?;


    if !resp.status().is_success() {
        return Err(format!("Erreur lors de la création du portefeuille: {}", resp.status()));
    }

    let portefeuille: PortefeuilleResponse = resp
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON portefeuille: {}", e))?;

    // Création du compte
    let account = Account::create_account(
        &mut conn,
        username,
        password,
        client.client_id,
        portefeuille.portefeuille_id,
        mfa_enabled,
    ).map_err(|e| format!("Erreur compte: {}", e))?;

    // Activation du compte
    Account::activate(&mut conn, account.account_id)
        .map_err(|e| format!("Erreur activation: {}", e))?;

    Ok(account.account_id)
}


pub fn delete_account(username: &str, password: &str) -> Result<(), String> {
    let mut conn = get_conn();
    match Account::login(&mut conn, username, password){
        Ok(Some(_account)) => {
            Account::delete_account(&mut conn, username )
                .map_err(|e| format!("Erreur lors de la suppression du compte: {}", e))?; 
        }
        Ok(None) => Err("Username/password invalides.".to_string())?,
        Err(err) => Err(format!("Erreur lors de la vérification des identifiants: {}", err))?,
    }
    Ok(())   
}