use rs_fullstack::startup;
use rs_fullstack::{config::get_config, telemetry::init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(Level::INFO, "rs-fullstack".into());

    let config = get_config().expect("failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    let pg_connection = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("failed to connect to postgres");

    startup::run(listener, pg_connection)?.await
}
