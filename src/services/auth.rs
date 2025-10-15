use diesel::PgConnection;
use crate::domain::account::Account;
use crate::services::mfa::*;
// use crate::ui::*;
use std::io;


pub fn login(conn: &mut PgConnection) {
    let mut username = String::new();
    let mut password = String::new();

    println!("--- Connexion ---");
    println!("Username : ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Password : ");
    io::stdin().read_line(&mut password).unwrap();

    let username: &str = username.trim();
    let password = password.trim();

    match Account::login(conn, username, password) {
        Ok(Some(account)) => {
            if account.status == "Active" {
                if account.mfa_enabled {
                    if mfa_verif(){
                        println!("Connexion réussie, bienvenue, {}.", account.username);
                        // display_log_menu(conn, account);
                    }
                }else{
                    println!("Connexion réussie, bienvenue, {}.", account.username);
                }
            }else if account.status == "Pending" {
                println!("Votre compte est en attente de validation.");
            } else if account.status == "Rejected" {
                println!("Votre compte a été rejeté.");
            }
        }
        Ok(None) => {
            println!("Username/password invalides.");
        }
        Err(err) => {
            println!("Erreur lors de la connexion: {}", err);
        }
    }
}
