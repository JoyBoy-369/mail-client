use std::net::TcpListener;

use mail_client::configuration::get_configuration;
use mail_client::startup::run;
use mail_client::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("mail-client".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
