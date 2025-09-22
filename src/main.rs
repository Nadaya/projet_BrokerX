use std::io;
fn main() {
    println!("=== Récupération des Informations Client ===\n");

    println!("Creation du compte"); 
    let info_compte = recuperer_info_client(); 
    println!("Informations du compte: {:?}", info_compte);
}

fn recuperer_info_client() -> InfosCreationCompte {
    println!("Veuillez saisir vos informations pour créer un compte:");
    let prenom = lire_entree("Prenom: ");
    let nom = lire_entree("Nom: ");
    let email = lire_entree("Email: ");
    let telephone = lire_entree("Numero de telephone: ");
    let date_naissance = lire_entree("Date de naissance (jj/mm/aaaa): ");   

    InfosCreationCompte { prenom, nom, date_naissance, email, telephone}
}

// Fonction utilitaire pour lire l'entrée utilisateur
fn lire_entree(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    
    let mut entree = String::new();
    io::stdin().read_line(&mut entree).expect("Erreur de lecture");
    entree.trim().to_string()
}

#[derive(Debug)]
struct InfosCreationCompte{
    prenom : String, 
    nom : String,
    date_naissance: String,
    email: String,
    telephone: String,
}