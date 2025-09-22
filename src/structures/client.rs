use crate::schema::clients;
use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = client)]
pub struct client {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: i32,
}

impl client{
    /* UC-01 : Create an account and check ID */
    pub fn create_account(conn: &mut PgConnection, name: &str, email: &str, phone: i32) -> QueryResult<client> {
        use crate::schema::clients;

        let new_client = client {
            id: 0, // This will be set by the database
            name: name.to_string(),
            email: email.to_string(),
            phone,
        };

        diesel::insert_into(clients::table)
            .values(&new_client)
            .get_result(conn)
    }

    pub fn search_client_by_id(conn: &mut PgConnection, client_id: i32) -> QueryResult<Client> {
        client::table.find(client_id).first::<CLient>(conn)
    }

    pub fn search_client_by_name(conn: &mut PgConnection, client_name: &str) -> QueryResult<Vec<Client>> {
        let pattern = format!("%{}%", client_name);
        client::table.filter(client::name.ilike(pattern)).load::<CLient>(conn)
    }

    pub fn search_client_by_email(conn: &mut PgConnection, client_email: &str) -> QueryResult<Vec<Client>> {
        let pattern = format!("%{}%", client_email);
        client::table.filter(client::category.ilike(pattern)).load::<Client>(conn)
    }
}