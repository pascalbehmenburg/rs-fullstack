use std::net::TcpListener;

use crate::routes::{health_check, user_register};
use actix_web::{dev::Server, web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/users/register", web::post().to(user_register))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
