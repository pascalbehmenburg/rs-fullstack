use rs_fullstack::config::get_config;
use rs_fullstack::startup;
use sqlx::PgPool;
use std::net::TcpListener;

// TODO implement user
//  TODO get /v1/users get current user
//  TODO patch /v1/users patch provided user data
//  TODO post /v1/users/login log in user
//  TODO post /v1/users/register creates user

// TODO implement todo

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    let pg_connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    startup::run(listener, pg_connection)?.await
}
