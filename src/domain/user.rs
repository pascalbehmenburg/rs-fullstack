use secrecy::Secret;
use serde::Deserialize;

use super::sanitize::{sanitize_email, sanitize_username, Sanitize};

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

impl Sanitize for RegisterUser {
    fn sanitize(&mut self) {
        self.name = sanitize_username(self.name.clone());
        self.email = sanitize_email(self.email.clone());
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<Secret<String>>,
}

impl Sanitize for UpdateUser {
    fn sanitize(&mut self) {
        if let Some(name) = &self.name {
            self.name = Some(sanitize_username(name.clone()));
        }
        if let Some(email) = &self.email {
            self.email = Some(sanitize_email(email.clone()));
        }
    }
}
