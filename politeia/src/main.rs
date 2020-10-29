pub mod model;
mod server;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start_server().await
}
