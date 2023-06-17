use std::net::TcpListener;

use mail_client::configuration::get_configuration;
use mail_client::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
