use crate::traduction::account;
use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = Account)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl Account{
    /* UC-01 : Create an account and check ID */
    pub fn create_account(conn: &mut PgConnection, username: &str, password: &str, role: &str) -> QueryResult<Account> {
        use crate::traduction::account;

        let new_account = Account {
            id: 0, // This will be set by the database
            username: username.to_string(),
            password: password.to_string(),
            role: role.to_string(),
        };

        diesel::insert_into(Account::table)
            .values(&new_account)
            .get_result(conn)
    }

    pub fn search_account_by_id(conn: &mut PgConnection, account_id: i32) -> QueryResult<Account> {
        account::table.find(account_id).first::<Account>(conn)
    }

    pub fn search_account_by_username(conn: &mut PgConnection, account_username: &str) -> QueryResult<Vec<Account>> {
        let pattern = format!("%{}%", account_username);
        account::table.filter(account::username.ilike(pattern)).load::<Account>(conn)
    }
}