use crate::domain::{portefeuille::Portefeuille};
use crate::service::db::get_conn;
use reqwest::Client;
use brokerx_types::account::Account;


pub async fn voir_solde(username : &str)-> Result<i32, String> {

    let client = Client::new();
    let account_resp = client
        .get(&format!("http://account-service:8080/get_account/{}", username))
        .send()
        .await
        .map_err(|e| format!("Erreur appel account-service: {}", e))?;

    if !account_resp.status().is_success() {
        return Err(format!("Compte {} introuvable", username));
    }

    let account: Account = account_resp
        .json::<Account>()
        .await
        .map_err(|e| format!("Erreur lecture réponse account-service: {}", e))?;


    let mut conn = get_conn();

    let portefeuille = Portefeuille::search_portefeuille_by_id(&mut conn, account.portefeuille_id)
        .map_err(|e| format!("Portefeuille non trouvé :{}", e))?;

    Ok(portefeuille.balance)
}

pub async fn approvisionner(username: &str, amount: i32) -> Result<(), String> {
    if amount < 0 {
        return Err("Montant invalide.".to_string());
    }

    let client = Client::new();
    let account_resp = client
        .get(&format!("http://account-service:8080/get_account/{}", username))
        .send()
        .await
        .map_err(|e| format!("Erreur appel account-service: {}", e))?;

    if !account_resp.status().is_success() {
        return Err(format!("Compte {} introuvable", username));
    }

    let account: Account = account_resp
        .json()
        .await
        .map_err(|e| format!("Erreur lecture réponse account-service: {}", e))?;

    let mut conn = get_conn();
    Portefeuille::approvisionner(&mut conn, account.portefeuille_id, amount)
        .map_err(|e| format!("Erreur lors de l'approvisionnement: {}", e))?;

    Ok(())
}