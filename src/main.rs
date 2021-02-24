use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// funções para cada rota (endpoint)
async fn ping() -> impl Responder {
    format!("pong")
    //HttpResponse::Ok()
}

async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Olá {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/hello", web::get().to(hello))
            .route("/{name}", web::get().to(hello))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
