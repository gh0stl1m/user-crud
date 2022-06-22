use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
