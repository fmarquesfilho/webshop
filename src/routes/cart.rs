use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
//use uuid::Uuid;
use serde::{Serialize, Deserialize};
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct Cart {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    #[serde(with = "my_uuid")]
    pub user_id: Uuid,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CartData {
    #[serde(with = "my_uuid")]
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CartProduct {
    #[serde(with = "my_uuid")]
    pub cart_id: Uuid,
    #[serde(with = "my_uuid")]
    pub product_id: Uuid,
}


//impl Cart {
pub async fn create_cart(
    cart: web::Json<CartData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {

    sqlx::query_as!(
        Cart,
        r#"
        INSERT INTO carts (id, user_id, active)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        cart.user_id,
        true
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_product_to_cart(
    cart_product: web::Json<CartProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query_as!(
        CartProduct,
        r#"
        INSERT INTO cart_products (cart_id, product_id)
        VALUES ($1, $2)
        "#,
        cart_product.cart_id,
        cart_product.product_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}

//}