use std::net::TcpListener;

use mail_client::configuration::get_configuration;
use mail_client::startup::run;
use sqlx::{Connection, PgPool};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
