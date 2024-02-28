use rsfullstack::startup;
use rsfullstack::{config::get_config, telemetry::init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(Level::INFO, "rs-fullstack".into());

    let config = get_config().expect("failed to read configuration");

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;

    let pg_connection = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("failed to connect to postgres");

    startup::run(listener, pg_connection)?.await
}
