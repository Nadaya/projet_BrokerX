use std::io;
use diesel::dsl::delete;
use diesel::{Connection, PgConnection};
use std::thread;
use std::time::Duration;
use rand::Rng;

mod domain;
mod infrastructure;

use domain::client::Client;
use domain::account::Account;

use crate::domain::portefeuille;

fn main() {

    let database_url: &str = "postgresql://postgres:postgres@localhost:5432/BrokerX";
    let mut conn = PgConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("=== Application BrokerX ===");
    println!("1. Créer un client + compte");
    println!("2. Login");
    println!("3. Supprimer mon compte");
    println!("4. Quitter");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Erreur de lecture");

    match choice.trim() {
        "1" => {
            create_client_and_account(&mut conn);
        }
        "2" => {
            login(&mut conn);
        }
        "3" => {
            delete_account(&mut conn);
        }
        "4" => {
            println!("Au revoir !");
        }
        _ => {
            println!("Choix invalide !");
        }
    }
}

fn create_client_and_account(conn: &mut PgConnection) {
    let mut name = String::new();
    let mut email = String::new();
    let mut phone = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("--- Création d'un client ---");
    println!("Nom : ");
    io::stdin().read_line(&mut name).unwrap();
    println!("Email : ");
    io::stdin().read_line(&mut email).unwrap();
    println!("Téléphone : ");
    io::stdin().read_line(&mut phone).unwrap();

    // Création du client
    let client = Client::create_client(conn, name.trim(), email.trim(), phone.trim())
        .expect("Erreur lors de la création du client");

    println!("Client créé ");

    println!("--- Création du compte lié ---");
    println!("Username : ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Password : ");
    io::stdin().read_line(&mut password).unwrap();

    let portefeuille = portefeuille::Portefeuille::create_portefeuille(conn, 0)
        .expect("Erreur lors de la création du portefeuille");

    println!("Souhaitez-vous activer l'authentification MFA ? (o/n) : ");
    let mut mfa_choice = String::new();
    io::stdin().read_line(&mut mfa_choice).unwrap();
    let mfa_enabled = mfa_choice.trim().eq_ignore_ascii_case("o");

    let account = Account::create_account(
        conn,
        username.trim(),
        password.trim(),
        client.client_id, // Lien avec le client créé
        portefeuille.id,  // Lien avec le portefeuille créé
        mfa_enabled,
    ).expect("Erreur lors de la création du compte");

    println!("Compte créé - état Pending");
    thread::sleep(Duration::from_secs(4));

    Account::activate(conn, account.account_id).expect("Erreur lors de l'activation du compte");
    println!("Votre compte est maintenant Active");
}

fn display_log_menu(conn: &mut PgConnection, account : Account){
    loop {
        println!("\n=== Interface Utilisateur ===");
        println!("1. Voir mes informations");
        println!("2. Voir le solde de mon portefeuille");
        println!("3. Approvisionner mon portefeuille");
        println!("4. Déconnexion");

        let mut sub_choice = String::new();
        io::stdin().read_line(&mut sub_choice).unwrap();

        match sub_choice.trim() {
            "1" => {
                println!("--- Informations du compte ---");
                println!("ID: {}", account.account_id);
                println!("Username: {}", account.username);
                println!("Client ID: {}", account.client_id);
            }
            "2" => {
                println!("--- Solde du portefeuille ---");
                // Logique pour afficher le solde du portefeuille ici
                match domain::portefeuille::Portefeuille::search_portefeuille_by_id(conn, account.portefeuille_id) {
                    Ok(portefeuille) => {
                        println!("Solde actuel: {}", portefeuille.balance);
                    }
                    Err(_) => {
                        println!("Portefeuille non trouvé pour ce client.");
                    }
                }
            }
            "3" => {
                println!("--- Approvisionner mon portefeuille ---");
                println!("Indiquer le montant à ajouter : ");
                let mut montant_str = String::new();
                io::stdin().read_line(&mut montant_str).unwrap();
                let montant: i32 = montant_str.trim().parse().unwrap_or(0);

                if montant < 0 {
                    println!("Montant invalide.")
                }else{
                    match domain::portefeuille::Portefeuille::approvisionner(conn, account.portefeuille_id, montant) {
                        Ok(_) => {
                            println!("Portefeuille crédité de {}.", montant);
                            match domain::portefeuille::Portefeuille::search_portefeuille_by_id(conn, account.portefeuille_id) {
                                Ok(portefeuille) => println!("Nouveau solde: {}", portefeuille.balance),
                                Err(_) => println!("Erreur lors de la récupération du portefeuille."),
                            }
                        }
                        Err(err) => {
                            println!("Erreur lors de l'approvisionnement: {}", err);
                        }
                    }
                }
            }
            "4" => {
                println!("Déconnexion...");
                break;
            }
            _ => {
                println!("Choix invalide !");
            }
        }
    }
    
}

fn generate_otp() -> String{
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1_000_000))
}

fn mfa_verif() -> bool{
    let otp = generate_otp();
    println!("[SIMULATION] Code OTP envoyé: {}", otp);

    let mut attempts = 0;

    while attempts < 3 {
        let mut user_input = String::new();
        println!("Entrez le code OTP : ");
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == otp {
            println!("Authentification MFA réussie !");
            return true;
        } else {
            attempts += 1;
            println!("Code incorrect (tentative {}/3)", attempts);
        }
    }
        println!("Trop de tentatives échouées. Réessayez plus tard.");
        return false;
}

fn login(conn: &mut PgConnection){
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
                        display_log_menu(conn, account);
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


fn delete_account(conn: &mut PgConnection) {
    let mut username = String::new();

    println!("--- Suppression d'un compte ---");
    println!("Entrez votre username : ");
    io::stdin().read_line(&mut username).unwrap();
    let username: String = username.trim().parse().unwrap_or("".to_string());

    match Account::delete_account(conn,username) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                println!("Compte supprimé");
            } else {
                println!("Aucun compte trouvé ");
            }
        }
        Err(err) => {
            println!("Erreur lors de la suppression du compte: {}", err);
        }
    }
}