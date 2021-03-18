//! tests/helpers.rs
use webshop::configuration::{get_configuration, DatabaseSettings};
use webshop::startup::{get_connection_pool, Application};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use webshop::routes::{UserData};
use std::collections::HashMap;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    // cria um novo usuário usando HTTP POST na rota /users
    // note que o username do usuário é uma String fake gerada automaticamente
    pub async fn post_users(&self, username: String) -> reqwest::Response {
        let user = UserData { username };
        let mut map = HashMap::new();
        map.insert("username", user.username.clone());

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/users", &self.address))
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }
}

// Cria uma nova instância da API
pub async fn create_app() -> TestApp {
    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let address = format!("http://localhost:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to the database"),
    }
}

// Configura um novo banco de dados a cada teste executado, 
// promovento isolamento entre os testes
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Cria base de dados
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Executa migração
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
