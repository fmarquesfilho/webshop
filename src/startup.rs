use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use crate::routes::{ping, subscribe};

// Notem a assinatura diferente da função (adicionamos pub)
// Nós retornamos a variável 'Server' quando dá tudo certo e não usamos mais o async 
// Não há mais a chamada para .async, pois não é mais necessária.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/ping", web::get().to(ping))
                .route("/subscriptions", web::get().to(subscribe))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // Retiramos o .await
    Ok(server)
}