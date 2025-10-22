// @generated automatically by Diesel CLI.

diesel::table! {
    account (account_id) {
        account_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        client_id -> Int4,
        portefeuille_id -> Int4,
        #[max_length = 20]
        status -> Varchar,
        mfa_enabled -> Bool,
    }
}

diesel::table! {
    clients (client_id) {
        client_id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    portefeuille (id) {
        id -> Int4,
        balance -> Int4,
    }
}

diesel::table! {
    transactions (transaction_id) {
        transaction_id -> Int4,
        portefeuille_id -> Int4,
        amount -> Int4,
        transaction_date -> Nullable<Timestamp>,
    }
}

diesel::joinable!(account -> clients (client_id));
diesel::joinable!(account -> portefeuille (portefeuille_id));
diesel::joinable!(transactions -> portefeuille (portefeuille_id));

diesel::allow_tables_to_appear_in_same_query!(account, clients, portefeuille, transactions,);
