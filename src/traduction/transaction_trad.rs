use diesel::table;

table! {
    transactions (id) {
        id -> Integer,
        portfeuille_id -> Integer,
        amount -> Integer,
        transaction_date -> Timestamp,
    }
}