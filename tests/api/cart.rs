use crate::helpers::create_app;
use webshop::routes::UserData;
use webshop::routes::UserId;
use std::collections::HashMap;

#[actix_rt::test]
async fn create_cart_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // insere usuário para os testes, uma vez que existe uma chave estrangeira em cart que requer que um usuário está inserido
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

    // insere cart
    let user_id: UserId = response.json().await.unwrap();
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

    let saved = sqlx::query!("SELECT id, user_id, active FROM carts",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved cart.");

    assert_eq!(saved.user_id, user_id.id);
    assert_eq!(saved.active, true);
}