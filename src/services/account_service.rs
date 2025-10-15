use diesel::PgConnection;
use crate::domain::client::Client; 
use crate::domain::account::Account;
use crate::services::db::get_conn;

use std::io;


// pub fn create_client_and_account(conn: &mut PgConnection) {
// let mut name = String::new();
//     let mut email = String::new();
//     let mut phone = String::new();
//     let mut username = String::new();
//     let mut password = String::new();

//     println!("--- Création d'un client ---");
//     println!("Nom : ");
//     io::stdin().read_line(&mut name).unwrap();
//     println!("Email : ");
//     io::stdin().read_line(&mut email).unwrap();
//     println!("Téléphone : ");
//     io::stdin().read_line(&mut phone).unwrap();

//     let client = Client::create_client(conn, name.trim(), email.trim(), phone.trim())
//         .expect("Erreur lors de la création du client");

//     println!("Client créé ");

//     println!("--- Création du compte lié ---");
//     println!("Username : ");
//     io::stdin().read_line(&mut username).unwrap();
//     println!("Password : ");
//     io::stdin().read_line(&mut password).unwrap();

//     let portefeuille = crate::domain::portefeuille::Portefeuille::create_portefeuille(conn, 0)
//         .expect("Erreur lors de la création du portefeuille");

//     println!("Souhaitez-vous activer l'authentification MFA ? (o/n) : ");
//     let mut mfa_choice = String::new();
//     io::stdin().read_line(&mut mfa_choice).unwrap();
//     let mfa_enabled = mfa_choice.trim().eq_ignore_ascii_case("o");

//     let account = Account::create_account(
//         conn,
//         username.trim(),
//         password.trim(),
//         client.client_id, 
//         portefeuille.portefeuille_id,  
//         mfa_enabled,
//     ).expect("Erreur lors de la création du compte");

//     println!("Compte créé - état Pending");
//     thread::sleep(Duration::from_secs(4));

//     Account::activate(conn, account.account_id).expect("Erreur lors de l'activation du compte");
//     println!("Votre compte est maintenant Active");
// }

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


pub fn delete_account(conn: &mut PgConnection) {
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

pub fn voir_infos(conn: &mut PgConnection,account: &Account) {
    println!("--- Informations du compte ---");
    println!("Username: {}", account.username);
    voir_solde(conn, account);
}

pub fn voir_solde(conn: &mut PgConnection, account: &Account) {
    match crate::domain::portefeuille::Portefeuille::search_portefeuille_by_id(conn, account.portefeuille_id) {
        Ok(portefeuille) => {
            println!("Solde actuel: {}", portefeuille.balance);
        }
        Err(_) => {
            println!("Portefeuille non trouvé pour ce client.");
        }
    }
}

pub fn approvisionner(conn: &mut PgConnection, account: &Account) {
    println!("--- Approvisionner mon portefeuille ---");
    println!("Indiquer le montant à ajouter : ");
    let mut montant_str = String::new();
    io::stdin().read_line(&mut montant_str).unwrap();
    let montant: i32 = montant_str.trim().parse().unwrap_or(0);

    if montant < 0 {
        println!("Montant invalide.")
    }else{
        match crate::domain::portefeuille::Portefeuille::approvisionner(conn, account.portefeuille_id, montant) {
            Ok(_) => {
                println!("Portefeuille crédité de {}.", montant);
                match crate::domain::portefeuille::Portefeuille::search_portefeuille_by_id(conn, account.portefeuille_id) {
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
