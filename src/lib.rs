pub mod config;
pub mod crypto;
pub mod routes;
pub mod startup;
pub mod telemetry;

use std::ops::Deref;

pub use config::*;
pub use routes::*;
use secrecy::Secret;
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    name: String,
    email: String,
    password: Secret<String>,
}
