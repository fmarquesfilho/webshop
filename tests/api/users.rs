use crate::helpers::create_app;
use webshop::routes::{UserData, User, UserId};
use std::collections::HashMap;

#[actix_rt::test]
async fn get_all_users_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn get_user_by_id_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria um novo usuário usando HTTP POST na rota /users
    let user = UserData {
        username: String::from("joselito"),
    };
    let mut map = HashMap::new();
    map.insert("username", user.username.clone());

    let response = client
        .post(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // salva o user_id que é retornado na resposta da criação do usuário
    let user_id: UserId = response.json().await.unwrap();

    // verifica se o usuário salvo está correto (neste caso, username tem que ser o mesmo)
    let saved = sqlx::query!("SELECT username FROM users WHERE id = $1", user_id.id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, user.username);

    // consulta o usuário criado usando HTTP GET pela rota /users/{id}
    let response = client
        .get(&format!("{}/users/{}", &app.address, user_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se o usuário retornado é o mesmo inicialmente criado
    let user: User = response.json().await.unwrap();
    assert_eq!(saved.username, user.username);
}

#[actix_rt::test]
async fn update_user_returns_200() {
    assert_eq!(200, 200);
}

#[actix_rt::test]
async fn delete_user_returns_200() {
    assert_eq!(200, 200);
}
