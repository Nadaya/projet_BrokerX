use diesel::table;

table! {
    portefeuille (id) {
        id -> Integer,
        balance -> Integer,
    }
}