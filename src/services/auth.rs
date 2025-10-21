// use diesel::PgConnection;
// use crate::domain::account::Account;
// use crate::services::mfa::*;
// // use crate::ui::*;
// use std::io;

use crate::services::db::get_conn;
use crate::domain::account::Account;



// pub fn login(conn: &mut PgConnection) {
//     let mut username = String::new();
//     let mut password = String::new();

//     println!("--- Connexion ---");
//     println!("Username : ");
//     io::stdin().read_line(&mut username).unwrap();
//     println!("Password : ");
//     io::stdin().read_line(&mut password).unwrap();

//     let username: &str = username.trim();
//     let password = password.trim();

//     match Account::login(conn, username, password) {
//         Ok(Some(account)) => {
//             if account.status == "Active" {
//                 if account.mfa_enabled {
//                     if mfa_verif(){
//                         println!("Connexion réussie, bienvenue, {}.", account.username);
//                         // display_log_menu(conn, account);
//                     }
//                 }else{
//                     println!("Connexion réussie, bienvenue, {}.", account.username);
//                 }
//             }else if account.status == "Pending" {
//                 println!("Votre compte est en attente de validation.");
//             } else if account.status == "Rejected" {
//                 println!("Votre compte a été rejeté.");
//             }
//         }
//         Ok(None) => {
//             println!("Username/password invalides.");
//         }
//         Err(err) => {
//             println!("Erreur lors de la connexion: {}", err);
//         }
//     }
// }

pub async fn login(username: &str, password: &str) -> Result<std::option::Option<Account>, std::string::String>  {
    let mut conn = get_conn();

    match Account::login(&mut conn, username, password) {
        Ok(Some(account)) => Ok(Some(account)),
        Ok(None) => Err("Username/password invalides.".to_string()),
        Err(err) => Err(format!("Erreur lors de la connexion: {}", err)),
    }
}