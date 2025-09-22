use diesel::table;

table! {
    account (id) {
        id -> Integer,
        username -> Varchar,
        password -> VarChar,
        role -> VarChar,
        client_id -> Integer,
    }
}