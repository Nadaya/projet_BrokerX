pub mod account {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Account {
        pub username: String,
        pub portefeuille_id: i32,
        pub status: String,
        pub mfa_enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateAccountRequest {
        pub username: String,
        pub password: String,
        pub client_id: i32,
        pub portefeuille_id: i32,
        pub mfa_enabled: bool,
    }
}

pub mod portefeuille {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Portefeuille {
        pub id: i32,
        pub balance: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreatePortefeuilleRequest {
        pub initial_balance: i32,
    }
}

pub mod auth {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LoginRequest<'a> {
        pub username: &'a str,
        pub password: &'a str,
    }
}