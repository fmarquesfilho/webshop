use crate::helpers::create_app;
use webshop::routes::{User, UserId};
use reqwest::Response;
use std::collections::HashMap;
use sqlx::{types::Uuid, Row};

#[actix_rt::test]
async fn post_users_returns_200() {
    let app = create_app().await;

    // cria um novo usuário usando a função auxiliar post_users
    let username = String::from("joselito");

    let saved = {
        let response: Response = app.post_users(username.clone()).await;
        assert_eq!(200, response.status().as_u16());

        // salva o user_id que é retornado na resposta da criação do usuário
        let user_id: UserId = response.json().await.unwrap();

        // verifica se o usuário salvo está correto (neste caso, username tem que ser o mesmo)
        sqlx::query!("SELECT username FROM users WHERE id = $1", user_id.id)
            .fetch_one(&app.db_pool)
            .await
            .expect("Failed to fetch saved user.")
    };

    assert_eq!(saved.username, username);
}

#[actix_rt::test]
async fn get_all_users_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria dois novos usuários que serão depois consultados
    let response: Response = app.post_users(String::from("joselito")).await;
    assert_eq!(200, response.status().as_u16());
    let response: Response = app.post_users(String::from("maria")).await;
    assert_eq!(200, response.status().as_u16());

    // faz o pedido de todos os usuários na rota /users
    let response = client
        .get(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se foram retornados 2 usuários
    let users: Vec<User> = response.json().await.unwrap();
    assert_eq!(2, users.len());
}

#[actix_rt::test]
async fn get_user_by_id_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria usuário
    let username = String::from("maria");
    let response: Response = app.post_users(username.clone()).await;
    let user_id: UserId = response.json().await.unwrap();

    // consulta o usuário criado usando HTTP GET pela rota /users/{id}
    let response = client
        .get(&format!("{}/users/{}", &app.address, user_id.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved_user: User = response.json().await.unwrap();

    // verifica se o username do usuário retornado é igual ao que foi criado
    assert_eq!(saved_user.username, username);
}

#[actix_rt::test]
async fn update_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria usuário
    let username = String::from("maria");
    let response: Response = app.post_users(username.clone()).await;
    let user_id: UserId = response.json().await.unwrap();
    let id: Uuid = user_id.id;

    // instancia um usuário e modifica o username, mantendo o mesmo id
    let user = User {
        id,
        username: String::from("joselito"),
    };

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut map = HashMap::new();
    map.insert("id", user.id.to_string());
    map.insert("username", user.username.clone());

    let response = client
        .put(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT username FROM users WHERE id = $1", user.id)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, user.username);
}

#[actix_rt::test]
async fn delete_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // cria usuário
    let username = String::from("maria");
    let response: Response = app.post_users(username.clone()).await;
    let user_id: UserId = response.json().await.unwrap();
    let id: Uuid = user_id.id;

    let response = client
        .delete(&format!("{}/users/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi removido
    let count: i64 = sqlx::query("SELECT COUNT(username) as count FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o usuário não foi removido, levantando falha
    assert_eq!(count, 0);
}
