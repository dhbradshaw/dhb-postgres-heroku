//! # dhb-heroku-postgres-client
//! Given a DATABASE_URL, it should be dead simple to connect to a Heroku postgres database.
//! 
//! This crate makes it dead simple:
//!
//! You pass a DATABASE_URL to the postgres_client function and get a working client back, as in
//! ```rust,no_run
//! let mut client = get_client(&database_url);
//! ```
//! 
//! The reason I found the work to create this crate necessary is that connecting to Heroku has 2 quirks.
//! 1. On the one hand, it requires that we have a secure connection.
//! 2. On the other hand, it uses self-verified certificates.  So we have to enable ssl, but turn off verification.  

// postgres connection
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
pub use postgres::Client;
use postgres_openssl::MakeTlsConnector;

pub use postgres;

/// Get a working client from a postgres url.
///
/// # Example:
/// ```rust,no_run
/// let database_url = "postgres://username:password@host:port/db_name";
/// let mut client = dhb_postgres_heroku::get_client(&database_url);
/// ```
/// # Panics
/// This will panic if it can't connect.  
/// That could be because your database_url is wrong, because your database is down, because your internet connection is failing, etc.
pub fn get_client(database_url: &str) -> Client {
    // Create Ssl postgres connector without verification as required to connect to Heroku.
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    // Create client with Heroku DATABASE_URL
    Client::connect(
        database_url,
        connector,
    ).unwrap()
}

/// Try out a client by running through a set of postgres commands to create a table, insert a row, read the row, and drop the table.
/// 
/// # Example:
/// ```rust,no_run
/// let database_url = "postgres://username:password@host:port/db_name";
/// let mut client = dhb_postgres_heroku::get_client(&database_url);
/// dhb_postgres_heroku::smoke_test(&mut client);
/// ```
pub fn smoke_test(client: &mut Client) {
    // 1. Create table. 
    client.simple_query("
        CREATE TABLE IF NOT EXISTS person_nonconflicting (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").unwrap();

    // 2. Save a row.
    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person_nonconflicting (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).unwrap();

    // 3. Retrieve a row and verify by printing.
    for row in client.query("SELECT id, name, data FROM person_nonconflicting", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person_nonconflicting: {} {} {:?}", id, name, data);
    }

    // 4. Clean up your mess by dropping the table.
    client.simple_query("DROP TABLE person_nonconflicting").unwrap();
} 