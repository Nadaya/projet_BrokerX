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
    pub status: String,
    pub mfa_enabled: bool,
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub username: String,
    pub password: String,
    pub client_id: i32,
    pub portefeuille_id: i32,
    pub status: String,
    pub mfa_enabled: bool,
}

// --- Implémentations ---
impl Account {
    // Créer un compte
    pub fn create_account(
        conn: &mut PgConnection,
        user_name: &str,
        user_password: &str,
        client_id_value: i32,
        portefeuille_id_value: i32,
        mfa_enabled_value: bool
    ) -> QueryResult<Account> {

        let new_account = NewAccount {
            username: user_name.to_string(),
            password: user_password.to_string(),
            client_id: client_id_value,
            portefeuille_id: portefeuille_id_value,
            status: "Pending".to_string(),
            mfa_enabled: mfa_enabled_value,
        };
        use crate::infrastructure::persistance::account::dsl::*;
        diesel::insert_into(account)
            .values(&new_account)
            .get_result(conn)
    }

    // Supprimer un compte
    pub fn delete_account(
        conn: &mut PgConnection,
        usern: &str,
    ) -> QueryResult<usize> {
        use crate::infrastructure::persistance::account::dsl::*;
        diesel::delete(account.filter(username.eq(usern)))
            .execute(conn)
    }

    // Activer un compte
    pub fn activate(
        conn: &mut PgConnection,
        account_id_value: i32,
    ) -> QueryResult<usize> {
        use crate::infrastructure::persistance::account::dsl::*;
        diesel::update(account.filter(account_id.eq(account_id_value)))
            .set(status.eq("Active"))
            .execute(conn)
    }

    // Connexion (login)
    pub fn login(
        conn: &mut PgConnection, 
        usern: &str, 
        passw: &str,
    ) -> QueryResult<Option<Account>> {
        use crate::infrastructure::persistance::account::dsl::*;
        account.filter(username.eq(usern))
               .filter(password.eq(passw))
               .first::<Account>(conn)
               .optional()
    }

    // Récupérer un compte par username
    pub fn get_by_username(
        conn: &mut PgConnection,
        usern: &str,
    ) -> QueryResult<Option<Account>> {
        use crate::infrastructure::persistance::account::dsl::*;
        account.filter(username.eq(usern))
               .first::<Account>(conn)
               .optional()
    }
}