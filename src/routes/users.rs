use actix_web::{web, HttpResponse};

use crate::{Data, RegisterUser};

pub async fn user_register(user: web::Json<Data<RegisterUser>>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
