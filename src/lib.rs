//! src/lib.rs

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;

async fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Notem a assinatura diferente da função (adicionamos pub)
// Nós retornamos a variável 'Server' quando dá tudo certo e não usamos mais o async 
// Não há mais a chamada para .async, pois não é mais necessária.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/ping", web::get().to(ping))
                .route("/subscriptions", web::post().to(subscribe))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // Retiramos o .await
    Ok(server)
}
