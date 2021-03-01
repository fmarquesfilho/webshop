use actix_web::{HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}