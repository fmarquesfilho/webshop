use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
use serde::{Serialize, Deserialize};
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CartData {
    #[serde(with = "my_uuid")]
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartItem {
    #[serde(with = "my_uuid")]
    pub cart_id: Uuid,
    #[serde(with = "my_uuid")]
    pub product_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CartId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

pub async fn create_cart(
    cart: web::Json<CartData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {

    let id = Uuid::new_v4();
    let rec = sqlx::query!(
        r#"
        INSERT INTO carts (id, user_id)
        VALUES ($1, $2)
        RETURNING id
        "#,
        id,
        cart.user_id,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;


    Ok(HttpResponse::Ok().json(CartId {
        id: rec.id,
    }))
}

pub async fn add_product_to_cart(
    cart_item: web::Json<CartItem>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {

    //let cart_id = req.match_info().get("cart_id");
    //let product_id = req.match_info().get("product_id");

    sqlx::query!(
        r#"
        INSERT INTO cart_items (cart_id, product_id)
        VALUES ($1, $2)
        RETURNING cart_id, product_id
        "#,
        cart_item.cart_id,
        cart_item.product_id,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}
