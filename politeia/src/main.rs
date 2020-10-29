pub mod model;
mod server;
mod types;

use actix_files::{Files as fs, NamedFile};
use actix_web::{web, App, HttpServer, Result};

async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("politeia/templates/dist/favicon.ico")?)
}

#[actix_web::main]
async fn main() {
    HttpServer::new(move || {
        App::new()
            .service(server::index)
            .route("/favicon.ico", web::get().to(favicon))
            .service(fs::new("/css", "politeia/templates/dist/css"))
            .service(fs::new("/js", "politeia/templates/dist/js"))
            .service(fs::new("/img", "politeia/templates/dist/img"))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
