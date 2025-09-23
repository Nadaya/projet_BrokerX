use diesel::table;

table! {
    clients (id) {
        id -> Integer,
        name -> Varchar,
        email -> VarChar,
        phone -> VarChar,
    }
}