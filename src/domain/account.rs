use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::infrastructure::persistance::account;

// --- Structures ---

#[derive(Queryable, Selectable)]
#[diesel(table_name = account)]
pub struct Account {
    pub account_id: i32,
    pub username: String,
    pub password: String,
    pub client_id: i32,
    pub portefeuille_id: i32,
    pub status : String,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub username: String,
    pub password: String,
    pub client_id: i32,
    pub portefeuille_id: i32,
    pub status : String,

}

// --- Implémentations ---

impl Account {
    /* UC-01 : Create an account and check ID */
    pub fn create_account(
        conn: &mut PgConnection,
        username: &str,
        password: &str,
        client_id: i32,
        portefeuille_id: i32,
    ) -> QueryResult<Account> {

        let new_account = NewAccount {
            username: username.to_string(),
            password: password.to_string(),
            client_id,
            portefeuille_id,
            status : ("Pending").to_string(),
        };

        diesel::insert_into(account::table)
            .values(&new_account)
            .get_result(conn)
    }

    pub fn search_account_by_id(
        conn: &mut PgConnection,
        account_id: i32,
    ) -> QueryResult<Account> {

        account::table.find(account_id).first::<Account>(conn)
    }

    pub fn search_account_by_username(
        conn: &mut PgConnection,
        account_username: &str,
    ) -> QueryResult<Vec<Account>> {

        let pattern = format!("%{}%", account_username);
        account::table
            .filter(account::username.ilike(pattern))
            .load::<Account>(conn)
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

    pub fn reject(conn: &mut PgConnection, _account_id: i32) -> QueryResult<usize> {
        use crate::infrastructure::persistance::account::dsl::*;
        diesel::update(account.filter(account_id.eq(_account_id)))
            .set((
                status.eq("Rejected"),
            ))
            .execute(conn)
    }

    // pub fn login(
    //     conn: &mut PgConnection,
    //     username: &str,
    //     password: &str,
    // )-> Result<Option<Account>, diesel::result::Error> {
    //     use crate::traduction::account::dsl::*;

    //     match account.filter(username.eq(username))
    //                  .filter(password.eq(password))
    //                  .first::<Account>(conn)
    //                  .optional()? 
    //     {
    //         Some(acc) => Ok(Some(acc)),  // compte trouvé et password correct
    //         None => Ok(None),            // compte non trouvé ou password incorrect
    //     }
    // }

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
                Some(acc) => Ok(Some(acc)),  // compte trouvé et password correct
                None => Ok(None),    // compte non trouvé ou password incorrect
            }
    }

}
