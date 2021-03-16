use crate::helpers::create_app;
use webshop::routes::UserData;
use std::collections::HashMap;

#[actix_rt::test]
async fn create_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let user = UserData {
        username: String::from("joselito"),
        password: String::from("abc"), //temporário
    };
    let mut map = HashMap::new();
    map.insert("username", user.username.clone());
    map.insert("password", user.password.clone());

    let response = client
        .post(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT username FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, user.username);
}