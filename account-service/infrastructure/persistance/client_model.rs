use diesel::table;

table! {
    clients (client_id) {
        client_id -> Integer,
        name -> Varchar,
        email -> VarChar,
        phone -> VarChar,
    }
}