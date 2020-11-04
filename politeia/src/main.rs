pub mod model;
mod server;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    server::start_server().await
}
