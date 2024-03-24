use std::ops::Deref;

use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;

use super::sanitize::Sanitize;

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
