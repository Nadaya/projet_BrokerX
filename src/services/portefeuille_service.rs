use crate::domain::{portefeuille::Portefeuille};
use crate::domain::account::Account;
use crate::services::db::get_conn;

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