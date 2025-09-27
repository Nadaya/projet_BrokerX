use diesel::table;

table! {
    portefeuille (portefeuille_id) {
        portefeuille_id -> Integer,
        balance -> Integer,
    }
}