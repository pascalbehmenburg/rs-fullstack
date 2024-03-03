use std::ops::Deref;

use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use secrecy::Secret;
use serde::{de::DeserializeOwned, Deserialize};
use unicode_segmentation::UnicodeSegmentation;

/// Implement this trait for any data which is send to the API and ensure it is safe to use.
pub trait Sanitize {
    fn sanitize(&mut self);
}

pub struct Data<T: Sanitize> {
    pub data: T,
}

impl<T: Sanitize> Deref for Data<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: DeserializeOwned + Sanitize> FromRequest for Data<T> {
    type Error = Error;
    type Future = LocalBoxFuture<'static, std::result::Result<Data<T>, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let mut payload = payload.take();

        Box::pin(async move {
            let mut data = web::Json::<T>::from_request(&req, &mut payload).await?;

            // normalize and validate the data also handles sanitization for query parameters
            data.0.sanitize();

            Ok(Data { data: data.0 })
        })
    }
}

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

/// Returns an instance of `SubscriberName` if the input satisfies all
/// our validation constraints on subscriber names.
/// It panics otherwise.
pub fn sanitize_username(s: String) -> String {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();

    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;

    // Iterate over all characters in the input `s` to check if any of them
    // matches one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        panic!("{} is not a valid subscriber name.", s)
    } else {
        s
    }
}

pub fn sanitize_email(s: String) -> String {
    s.trim().to_lowercase()
}
