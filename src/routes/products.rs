use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
//use uuid::Uuid;
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct Product {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductData {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

pub async fn create_product(
    product: web::Json<ProductData>,
    pool: web::Data<PgPool>, 
) -> Result<HttpResponse, HttpResponse> {

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO products (id, name)
        VALUES ($1, $2)
        RETURNING id
        "#,
        id,
        product.name
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(ProductId {
        id: id,
    }))
}

