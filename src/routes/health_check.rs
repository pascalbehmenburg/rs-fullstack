use actix_web::HttpResponse;

/// Health check endpoint.
#[tracing::instrument(name = "health check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
