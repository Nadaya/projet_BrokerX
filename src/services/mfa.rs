use rand::Rng;
use std::io;

fn generate_otp() -> String {
    let mut rng = rand::rng();
    format!("{:06}", rng.random_range(0..1_000_000))
}

pub fn mfa_verif() -> bool {
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
