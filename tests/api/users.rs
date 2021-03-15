use crate::helpers::create_app;
use std::collections::HashMap;
use sqlx::{types::Uuid};

#[actix_rt::test]
async fn create_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

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

    let saved = sqlx::query!("SELECT id, username, password FROM users WHERE id = {}", response.id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved cart.");

    assert_eq!(saved.username, user.username);
    assert_eq!(saved.password, user.password);
}