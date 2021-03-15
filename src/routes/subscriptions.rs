use actix_web::{web, HttpResponse};
use sqlx::{PgPool};

#[derive(serde::Deserialize)]
pub struct Subscription {
    email: String,
    name: String
}

pub async fn subscribe(
    subscription: web::Json<Subscription>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query_as!(
        Subcription,
        r#"
        INSERT INTO subscriptions (email, name)
        VALUES ($1, $2)
        "#,
        subscription.email,
        subscription.name
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}