use actix_web::{App, HttpResponse, HttpServer, web};

mod config;
mod db;
mod handler;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
