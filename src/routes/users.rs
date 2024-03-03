use crate::{
    crypto::hash_password,
    domain::{Data, RegisterUser},
};
use actix_web::{web, HttpResponse};

use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

/// Handles user registration.
#[tracing::instrument(
    name = "user registration",
    skip(register_user_data, pg_pool),
    fields(
        register_user_data = ?register_user_data.data
    )
)]
pub async fn user_registration(
    mut register_user_data: Data<RegisterUser>,
    pg_pool: web::Data<PgPool>,
) -> HttpResponse {
    register_user_data.data.password = hash_password(Secret::new(
        register_user_data.data.password.expose_secret().into(),
    ));

    match insert_user(&register_user_data.data, &pg_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Inserts a new user into the database.
#[tracing::instrument(name = "user insert", skip(register_user, pg_pool))]
pub async fn insert_user(
    register_user: &RegisterUser,
    pg_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        "#,
        Uuid::now_v7(),
        register_user.name,
        register_user.email,
        register_user.password.expose_secret(),
    )
    .execute(pg_pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
