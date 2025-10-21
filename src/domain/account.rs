use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::infrastructure::persistance::account;

#[allow(dead_code)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = account)]
pub struct Account {
    pub account_id: i32,
    pub username: String,
    pub password: String,
    pub client_id: i32,
    pub portefeuille_id: i32,
    pub status : String,
    pub mfa_enabled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub username: String,
    pub password: String,
    pub client_id: i32,
    pub portefeuille_id: i32,
    pub status : String,
    pub mfa_enabled: bool,

}

// --- Implémentations ---

impl Account {
    pub fn create_account(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
        client_id: i32,
        portefeuille_id: i32,
        mfa_enabled: bool,
    ) -> QueryResult<Account> {

        let new_account = NewAccount {
            username: username.to_string(),
            password: password.to_string(),
            client_id,
            portefeuille_id,
            status : ("Pending").to_string(),
            mfa_enabled,
        };

        diesel::insert_into(account::table)
            .values(&new_account)
            .get_result(conn)
    }

    pub fn delete_account(
        conn: &mut PgConnection,
        usern: String,
    ) -> QueryResult<usize> {
        use crate::infrastructure::persistance::account::dsl::*;
        let rows_deleted = diesel::delete(account.filter(username.eq(usern)))
            .execute(conn)?;
        Ok(rows_deleted)
    }

    pub fn activate(conn: &mut PgConnection, _account_id: i32) -> QueryResult<usize> {
        use crate::infrastructure::persistance::account::dsl::*;
        diesel::update(account.filter(account_id.eq(_account_id)))
            .set(status.eq("Active"))
            .execute(conn)
    }

    pub fn login(
        conn: &mut PgConnection, 
        usern: &str, 
        passw: &str,
    ) -> Result<Option<Account>, diesel::result::Error> {
        use crate::infrastructure::persistance::account::dsl::*;
        match account.filter(username.eq(usern))
                        .filter(password.eq(passw))
                        .first::<Account>(conn)
                        .optional()? 
            {
                Some(acc) => Ok(Some(acc)), 
                None => Ok(None),  
            }
    }

    pub fn get_by_username(
        conn: &mut PgConnection,
        usern: &str,
    ) -> Result<Option<Account>, diesel::result::Error> {
        use crate::infrastructure::persistance::account::dsl::*;
        match account.filter(username.eq(usern))
                        .first::<Account>(conn)
                        .optional()? 
            {
                Some(acc) => Ok(Some(acc)), 
                None => Ok(None),  
            }
    }
}
