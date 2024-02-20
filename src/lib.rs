pub mod config;
pub mod routes;
pub mod startup;

pub use config::*;
pub use routes::*;
pub use startup::*;

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
