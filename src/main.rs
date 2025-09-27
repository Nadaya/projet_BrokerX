mod domain;   
mod infrastructure;

mod ui;
mod services;

use diesel::PgConnection;
use diesel::Connection;

fn main(){
    let database_url: &str = "postgresql://postgres:postgres@localhost:5432/BrokerX";
    let mut conn = PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    ui::menu_principal(&mut conn);
}