//! src/main.rs
use projetoweb2::configuration::get_configuration;
use projetoweb2::startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}