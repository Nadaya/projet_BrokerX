use std::io;
use diesel::dsl::delete;
use diesel::{Connection, PgConnection};


mod structures;
mod traduction;

use structures::client::Client;
use structures::account::Account;

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

// fn establish_connection() -> PgConnection {
//     let database_url: &str = "postgresql://postgres:postgres@localhost:5432/BrokerX";
//     let conn = PgConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
// }

fn create_client_and_account(conn: &mut PgConnection) {
    let mut name = String::new();
    let mut email = String::new();
    let mut phone = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut role = String::new();

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

    println!("✅ Client créé avec id={}", client.id);

    println!("--- Création du compte lié ---");
    println!("Username : ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Password : ");
    io::stdin().read_line(&mut password).unwrap();
    println!("Role : ");
    io::stdin().read_line(&mut role).unwrap();

    let account = Account::create_account(
        conn,
        username.trim(),
        password.trim(),
        role.trim(),
        client.id, // 🔗 Lien avec le client créé
    ).expect("Erreur lors de la création du compte");

    println!("✅ Compte créé avec id={} lié au client_id={}", account.id, account.client_id);
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
            println!("✅ Connexion réussie ! Bienvenue, {}.", account.username);
            loop {
                println!("\n=== Interface Utilisateur ===");
                println!("1. Voir mes informations");
                println!("2. Déconnexion");

                let mut sub_choice = String::new();
                io::stdin().read_line(&mut sub_choice).unwrap();

                match sub_choice.trim() {
                    "1" => {
                        println!("--- Informations du compte ---");
                        println!("ID: {}", account.id);
                        println!("Username: {}", account.username);
                        println!("Role: {}", account.role);
                        println!("Client ID: {}", account.client_id);
                    }
                    "2" => {
                        println!("Déconnexion...");
                        break;
                    }
                    _ => {
                        println!("Choix invalide !");
                    }
                }
            }
        }
        Ok(None) => {
            println!("❌ Username/password invalides.");
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
                println!("✅ Compte supprimé");
            } else {
                println!("❌ Aucun compte trouvé ");
            }
        }
        Err(err) => {
            println!("Erreur lors de la suppression du compte: {}", err);
        }
    }
}