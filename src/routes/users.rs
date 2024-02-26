use crate::{Data, RegisterUser};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn user_register(
    user: web::Json<Data<RegisterUser>>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        "#,
        Uuid::new_v4(),
        user.name,
        user.email,
        user.password,
    )
    .execute(connection.get_ref())
    .await
    .expect("Failed to execute query.");

    HttpResponse::Ok().finish()
}
