use std::net::TcpListener;

use crate::routes::{health_check, user_registration};
use actix_web::{dev::Server, guard, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

/// Responsible for launching the http server and defining routes.
#[tracing::instrument("launching actix http server instance", skip(pool_postgres))]
pub fn run(listener: TcpListener, pool_postgres: PgPool) -> Result<Server, std::io::Error> {
    let pool_postgres = web::Data::new(pool_postgres);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .guard(guard::Header("content-type", "application/json"))
                            .route("/register", web::post().to(user_registration)),
                    )
                    .route("/health_check", web::get().to(health_check)),
            )
            .app_data(pool_postgres.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
