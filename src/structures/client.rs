use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::traduction::client;

// --- Structures ---

#[derive(Queryable, Selectable)]
#[diesel(table_name = client)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: i32,
}

#[derive(Insertable)]
#[diesel(table_name = client)]
pub struct NewClient {
    pub name: String,
    pub email: String,
    pub phone: i32,
}

// --- Implémentations ---

impl Client {
    /* UC-01 : Create a client and check ID */
    pub fn create_client(
        conn: &mut PgConnection,
        name: &str,
        email: &str,
        phone: i32,
    ) -> QueryResult<Client> {

        let new_client = NewClient {
            name: name.to_string(),
            email: email.to_string(),
            phone,
        };

        diesel::insert_into(client::table)
            .values(&new_client)
            .get_result(conn)
    }

    pub fn search_client_by_id(
        conn: &mut PgConnection,
        client_id: i32,
    ) -> QueryResult<Client> {
        client::table.find(client_id).first::<Client>(conn)
    }

    pub fn search_client_by_name(
        conn: &mut PgConnection,
        client_name: &str,
    ) -> QueryResult<Vec<Client>> {

        let pattern = format!("%{}%", client_name);
        client::table.filter(client::name.ilike(pattern)).load::<Client>(conn)
    }

    pub fn search_client_by_email(
        conn: &mut PgConnection,
        client_email: &str,
    ) -> QueryResult<Vec<Client>> {

        let pattern = format!("%{}%", client_email);
        client::table.filter(client::email.ilike(pattern)).load::<Client>(conn)
    }
}
