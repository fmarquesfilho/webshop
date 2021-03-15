use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, types::Uuid};
//use uuid::Uuid;
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "my_uuid")]
    id: Uuid,
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
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

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        id,
        user.username,
        user.password, // temporário
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    //Ok(HttpResponse::Ok().finish())
    Ok(HttpResponse::Ok().json(UserId {
        id: id,
    }))
}

