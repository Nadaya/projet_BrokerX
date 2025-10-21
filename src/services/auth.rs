// use diesel::PgConnection;
// use crate::domain::account::Account;
// use crate::services::mfa::*;
// // use crate::ui::*;
// use std::io;

use crate::services::db::get_conn;
use crate::domain::account::Account;

pub async fn login(username: &str, password: &str) -> Result<std::option::Option<Account>, std::string::String>  {
    let mut conn = get_conn();

    match Account::login(&mut conn, username, password) {
        Ok(Some(account)) => Ok(Some(account)),
        Ok(None) => Err("Username/password invalides.".to_string()),
        Err(err) => Err(format!("Erreur lors de la connexion: {}", err)),
    }
}