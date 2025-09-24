use diesel::table;

table! {
    transaction (id) {
        id -> Integer,
        portfeuille_id -> Integer,
        amount -> Integer,
        transaction_date -> Timestamp,
    }
}