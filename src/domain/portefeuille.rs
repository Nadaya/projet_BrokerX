use diesel::{
    prelude::*,
    PgConnection,
    QueryResult,
};
use crate::infrastructure::persistance::portefeuille;

// --- Structures ---
#[derive(Queryable, Selectable)]
#[diesel(table_name = portefeuille)]
pub struct Portefeuille {
    pub portefeuille_id: i32,
    pub balance: i32,
}

#[derive(Insertable)]
#[diesel(table_name = portefeuille)]
pub struct NewPortefeuille {
    pub balance: i32,
}

// --- Implémentations --- 
impl Portefeuille {
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

    pub fn approvisionner(
                conn: &mut PgConnection,
        _portefeuille_id: i32,
        montant: i32,
    ) -> QueryResult<usize> {
        use crate::infrastructure::persistance::portefeuille::dsl::*;
        diesel::update(portefeuille.filter(portefeuille_id.eq(_portefeuille_id)))
            .set(balance.eq(balance + montant))
            .execute(conn)
    }
}
