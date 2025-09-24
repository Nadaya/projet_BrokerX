// use diesel::{
//     prelude::*,
//     PgConnection,
//     QueryResult,
// };
// use chrono::NaiveDateTime;
// use crate::traduction::transaction;

// // --- Structures ---
// #[derive(Queryable, Selectable)]
// #[diesel(table_name = transaction)]
// pub struct Transaction {
//     pub id: i32,
//     pub portfeuille_id: i32,
//     pub amount: i32,
//     pub transaction_date: NaiveDateTime,
// }

// #[derive(Insertable)]
// #[diesel(table_name = transaction)]
// pub struct NewTransaction {
//     pub portfeuille_id: i32,
//     pub amount: i32,
//     pub transaction_date: NaiveDateTime,
// }

// // --- Implémentations ---
// impl Transaction {  
//     /* UC-03 : Create a transaction and check ID */
//     pub fn create_transaction(
//         conn: &mut PgConnection,
//         portfeuille_id: i32,
//         amount: i32,
//         transaction_date: chrono::NaiveDateTime,
//     ) -> QueryResult<Transaction> {

//         let new_transaction = NewTransaction {
//             portfeuille_id,
//             amount,
//             transaction_date,
//         };

//         diesel::insert_into(transaction::table)
//             .values(&new_transaction)
//             .get_result(conn)
//     }

//     pub fn search_transaction_by_id(
//         conn: &mut PgConnection,
//         transaction_id: i32,
//     ) -> QueryResult<Transaction> {
//         transaction::table.find(transaction_id).first::<Transaction>(conn)
//     }

//     pub fn search_transactions_by_portfeuille_id(
//         conn: &mut PgConnection,
//         portfeuille_id: i32,
//     ) -> QueryResult<Vec<Transaction>> {
//         transaction::table
//             .filter(transaction::portfeuille_id.eq(portfeuille_id))
//             .load::<Transaction>(conn)
//     }

//     pub fn set_transaction_amount(
//         conn: &mut PgConnection,
//         transaction_id: i32,
//         new_amount: i32,
//     ) -> QueryResult<Transaction> {
//         diesel::update(transaction::table.find(transaction_id))
//             .set(transaction::amount.eq(new_amount))
//             .get_result(conn)
//     }
// }