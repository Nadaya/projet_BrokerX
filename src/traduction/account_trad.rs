use diesel::table;

table! {
    account (account_id) {
        account_id -> Integer,
        username -> Varchar,
        password -> VarChar,
        client_id -> Integer,
        portefeuille_id -> Integer,
    }
}