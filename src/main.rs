use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    tracing::info!("Starting zero2prod on {}", address);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to database.");

    match run(listener, connection_pool) {
        Ok(server) => {
            server.await.unwrap();
        }
        Err(e) => {
            println!("Server failed to start: {}", e);
        }
    }

    Ok(())
}
