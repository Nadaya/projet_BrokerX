use diesel::PgConnection;

use crate::services::{auth, account_service};

pub fn menu_principal(conn: &mut PgConnection) {
    println!("=== Application BrokerX ===");
    println!("1. Créer un client + compte");
    println!("2. Login");
    println!("3. Supprimer mon compte");
    println!("4. Quitter");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Erreur de lecture");

    match choice.trim() {
        "1" => account_service::create_client_and_account(conn),
        "2" => auth::login(conn),
        "3" => account_service::delete_account(conn),
        "4" => println!("Au revoir !"),
        _ => println!("Choix invalide !"),
    }
}

pub fn display_log_menu(conn: &mut PgConnection, account: crate::domain::account::Account) {
    loop {
        println!("\n=== Interface Utilisateur ===");
        println!("1. Voir mes informations");
        println!("2. Voir le solde de mon portefeuille");
        println!("3. Approvisionner mon portefeuille");
        println!("4. Déconnexion");

        let mut sub_choice = String::new();
        std::io::stdin().read_line(&mut sub_choice).unwrap();

        match sub_choice.trim() {
            "1" => account_service::voir_infos(conn,&account),
            "2" => account_service::voir_solde(conn, &account),
            "3" => account_service::approvisionner(conn, &account),
            "4" => {
                println!("Déconnexion...");
                break;
            }
            _ => println!("Choix invalide !"),
        }
    }
}
