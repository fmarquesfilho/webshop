use crate::helpers::create_app;
use webshop::routes::{UserData, ProductData, ProductId, CartId};
use webshop::routes::UserId;
use std::collections::HashMap;

#[actix_rt::test]
async fn add_product_to_cart_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // insere usuário para os testes, uma vez que existe uma chave estrangeira em cart que requer que um usuário está inserido
    // usa a rota /users
    let user = UserData {
        username: String::from("joselito"),
        password: String::from(""), //temporário
    };
    let mut map = HashMap::new();
    map.insert("username", user.username);
    map.insert("password", user.password);

    let response = client
        .post(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // guarda user_id que é retornado como json
    let user_id: UserId = response.json().await.unwrap();

    // cria novo carrinho para o usuário recém-inserido
    map = HashMap::new();
    map.insert("user_id", user_id.id.to_string());

    let response = client
        .post(&format!("{}/cart", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // guarda cart_id que é retornado como json
    let cart_id: CartId = response.json().await.unwrap();

    // insere produto via rota /products
    let product = ProductData {
        name: String::from("produto A"),
    };
    map = HashMap::new();
    map.insert("name", product.name);

    let response = client
        .post(&format!("{}/products", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // guarda id que é retornado como json
    let product_id: ProductId = response.json().await.unwrap();

    // finalmente insere o produto como item do carrinho
    map = HashMap::new();
    map.insert("product_id", product_id.id.to_string());
    map.insert("cart_id", cart_id.id.to_string());

    let response = client
        .post(&format!("{}/cart/products", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    //assert_eq!(200, 200);
}