use actix_web::{App, HttpServer};
mod posts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(posts::views::home))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
