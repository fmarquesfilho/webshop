use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
//use uuid::Uuid;
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct Product {
    #[serde(with = "my_uuid")]
    id: Uuid,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductData {
    name: String,
}

pub async fn create_product(
    product: web::Json<ProductData>,
    pool: web::Data<PgPool>, 
) -> Result<HttpResponse, HttpResponse> {

    sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (id, name)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        product.name
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}

