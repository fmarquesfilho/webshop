use crate::helpers::create_app;
use webshop::routes::ProductData;
use std::collections::HashMap;

#[actix_rt::test]
async fn create_product_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let product = ProductData {
        name: String::from("product A"),
    };
    let mut map = HashMap::new();
    map.insert("name", product.name.clone());

    let response = client
        .post(&format!("{}/products", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name FROM products")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved product.");

    assert_eq!(saved.name, product.name);
}