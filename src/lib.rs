use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Data<T> {
    data: T,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RegisterUser {
    name: String,
    email: String,
    password: String,
}

async fn user_register(user: web::Json<Data<RegisterUser>>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

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
