use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::traduction::portefeuille;

// --- Structures ---
#[derive(Queryable, Selectable)]
#[diesel(table_name = portefeuille)]
pub struct Portefeuille {
    pub id: i32,
    pub balance: i32,
}

#[derive(Insertable)]
#[diesel(table_name = portefeuille)]
pub struct NewPortefeuille {
    pub balance: i32,
}

// --- Implémentations --- 
impl Portefeuille {
    /* UC-02 : Create a portefeuille and check ID */
    pub fn create_portefeuille(
        conn: &mut PgConnection,
        balance: i32,
    ) -> QueryResult<Portefeuille> {

        let new_portefeuille = NewPortefeuille {
            balance,
        };

        diesel::insert_into(portefeuille::table)
            .values(&new_portefeuille)
            .get_result(conn)
    }

    pub fn search_portefeuille_by_id(
        conn: &mut PgConnection,
        portefeuille_id: i32,
    ) -> QueryResult<Portefeuille> {
        portefeuille::table.find(portefeuille_id).first::<Portefeuille>(conn)
    }

    // pub fn search_portefeuille_by_client_id(
    //     conn: &mut PgConnection,
    // ) -> QueryResult<Portefeuille> {
    //     portefeuille::table
    //         .filter(portefeuille::client_id.eq(client_id))
    //         .first::<Portefeuille>(conn)}

    pub fn update_balance(
        conn: &mut PgConnection,
        portefeuille_id: i32,
        new_balance: i32,
    ) -> QueryResult<Portefeuille> {
        diesel::update(portefeuille::table.find(portefeuille_id))
            .set(portefeuille::balance.eq(new_balance))
            .get_result(conn)
    }

    pub fn delete_portefeuille(
        conn: &mut PgConnection,
        portefeuille_id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(portefeuille::table.find(portefeuille_id)).execute(conn)
    }

    pub fn list_all_portefeuilles(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<Portefeuille>> {
        portefeuille::table.load::<Portefeuille>(conn)
    }

    pub fn apply_deposit(
        conn: &mut PgConnection,
        portefeuille_id: i32,
        amount: i32,
    ) -> QueryResult<Portefeuille> {
        let portefeuille = Self::search_portefeuille_by_id(conn, portefeuille_id)?;
        let new_balance = portefeuille.balance + amount;
        Self::update_balance(conn, portefeuille_id, new_balance)
    }
}
