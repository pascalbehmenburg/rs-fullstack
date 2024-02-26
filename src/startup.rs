use std::net::TcpListener;

use crate::routes::{health_check, user_register};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, pool_postgres: PgPool) -> Result<Server, std::io::Error> {
    let pool_postgres = web::Data::new(pool_postgres);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/users/register", web::post().to(user_register))
            .app_data(pool_postgres.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
