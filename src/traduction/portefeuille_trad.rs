use diesel::table;

table! {
    portefeuille (id) {
        id -> Integer,
        client_id -> Integer,
        balance -> Integer,
    }
}