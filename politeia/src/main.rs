pub mod model;
mod server;
mod types;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(server::index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();
}
