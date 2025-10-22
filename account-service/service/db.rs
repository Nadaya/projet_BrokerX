use diesel::PgConnection;
use diesel::Connection;

pub fn get_conn() -> PgConnection {
    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB not set");
    let port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");

    let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
