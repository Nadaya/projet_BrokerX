use diesel::table;

table! {
    client (id) {
        id -> Integer,
        name -> Varchar,
        email -> VarChar,
        phone -> Integer,
    }
}