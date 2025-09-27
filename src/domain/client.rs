use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::infrastructure::persistance::clients;

// --- Structures ---
#[allow(dead_code)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = clients)]
pub struct Client {
    pub client_id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
}
#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub name: String,
    pub email: String,
    pub phone: String,
}


impl Client {
    pub fn create_client(
        conn: &mut PgConnection,
        name: &str,
        email: &str,
        phone: &str,
    ) -> QueryResult<Client> {

        let new_client = NewClient {
            name: name.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
        };

        diesel::insert_into(clients::table)
            .values(&new_client)
            .get_result(conn)
    }
}
