use crate::domain::client::Client; 
use crate::domain::account::Account;
use crate::domain::{portefeuille::Portefeuille};
use crate::services::db::get_conn;

pub fn create_client_and_account(
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
    let portefeuille = crate::domain::portefeuille::Portefeuille::create_portefeuille(&mut conn, 0)
        .map_err(|e| format!("Erreur portefeuille: {}", e))?;

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


pub fn voir_solde(username : &str)-> Result<i32, String> {
    let mut conn = get_conn();
    let account = Account::get_by_username(&mut conn, username);

    let portefeuille = Portefeuille::search_portefeuille_by_id(&mut conn, account.unwrap().unwrap().portefeuille_id)
        .map_err(|e| format!("Portefeuille non trouvé :{}", e))?;

    Ok(portefeuille.balance)
}

pub async fn approvisionner(username: &str, amount: i32) -> Result<(), String> {
    let mut conn = get_conn();
    let account = Account::get_by_username(&mut conn, username); // récupère le compte
    if amount < 0 {
        return Err("Montant invalide.".to_string());
    }

    Portefeuille::approvisionner(&mut conn, account.unwrap().unwrap().portefeuille_id, amount)
        .map_err(|e| format!("Erreur lors de l'approvisionnement: {}", e))?;
    Ok(())
}