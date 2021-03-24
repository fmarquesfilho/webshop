use actix_web::{HttpResponse};

#[tracing::instrument(name = "Ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}