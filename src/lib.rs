pub mod config;
pub mod crypto;
pub mod routes;
pub mod startup;

use std::ops::Deref;

pub use config::*;
pub use routes::*;
pub use startup::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Data<T> {
    data: T,
}

impl<T> Deref for Data<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterUser {
    name: String,
    email: String,
    password: String,
}
