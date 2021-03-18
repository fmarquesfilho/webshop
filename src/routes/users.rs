use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
//use uuid::Uuid;
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

pub async fn create_user(
    user: web::Json<UserData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse>  {

    let row = sqlx::query!(
        r#"
        INSERT INTO users (id, username)
        VALUES ($1, $2)
        RETURNING id
        "#,
        Uuid::new_v4(),
        user.username,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(UserId {
        id: row.id,
    }))
}

pub async fn get_all_users(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {
    let rows = sqlx::query!(
        r#"
        SELECT id, username
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut users: Vec<User> = Vec::new();
    for row in rows {
        let user = User {
            id: row.id,
            username: row.username,
        };
        users.push(user);
    }

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let row = sqlx::query!(
        r#"
        SELECT id, username
        FROM users
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(User {
        id: row.id,
        username: row.username,
    }))
}

pub async fn update_user(
    user: web::Json<User>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    sqlx::query!(
        r#"
        UPDATE users
        SET username = $1
        WHERE id = $2
        "#,
        user.username,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_user(
    req: web::HttpRequest,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse>  {

    let id:Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}



