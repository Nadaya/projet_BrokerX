use rand::Rng;


fn generate_otp() -> String {
    let mut rng = rand::rng();
    format!("{:06}", rng.random_range(0..1_000_000))
}

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OTP_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn send_otp(username: &str) -> String {
    let otp = generate_otp();
    OTP_STORE.lock().unwrap().insert(username.to_string(), otp.clone());
    otp
}

pub fn verify_otp(username: &str, otp: &str) -> bool {
    if let Some(stored_otp) = OTP_STORE.lock().unwrap().get(username) {
        stored_otp == otp
    } else {
        false
    }
}

// pub fn mfa_verif() -> bool {
//     let otp = generate_otp();
//     println!("[SIMULATION] Code OTP envoyé: {}", otp);

//     let mut attempts = 0;

//     while attempts < 3 {
//         let mut user_input = String::new();
//         println!("Entrez le code OTP : ");
//         io::stdin().read_line(&mut user_input).unwrap();

//         if user_input.trim() == otp {
//             println!("Authentification MFA réussie !");
//             return true;
//         } else {
//             attempts += 1;
//             println!("Code incorrect (tentative {}/3)", attempts);
//         }
//     }
//     println!("Trop de tentatives échouées. Réessayez plus tard.");
//     false
// }

