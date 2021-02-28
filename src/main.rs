//! src/main.rs
 
use projetoweb2::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Joga io::Error se o bind falhas
    // Caso contrário chama .await no servidor
    run()?.await
}