use actix_cors::Cors;
use actix_files::{Files as fs, NamedFile};
use actix_web::{get, http::StatusCode, post, web, App, HttpServer, Responder, Result};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "./dist/home.html")]
struct HomeTemplate {}

async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("politeia/templates/dist/favicon.ico")?)
}

pub async fn start_server() -> std::io::Result<()> {
    let port = std::env::var("PORT").expect("Port env not set.");
    if port == "" {
        panic!("Port env is an empty string");
    }

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .service(index)
            .service(fetch_tokens)
            .service(fetch_proposals)
            .route("/favicon.ico", web::get().to(favicon))
            .service(fs::new("/css", "politeia/templates/dist/css"))
            .service(fs::new("/js", "politeia/templates/dist/js"))
            .service(fs::new("/img", "politeia/templates/dist/img"))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    let a = HomeTemplate {};
    a.into_response()
}

#[get("/api/v1/fetchtokens")]
async fn fetch_tokens() -> impl Responder {
    let client = super::model::new().expect("Unable to create a new client");

    match client.fetch_tokens().await {
        Ok(e) => match serde_json::to_string(&e) {
            Ok(e) => e.with_status(StatusCode::OK),

            Err(e) => {
                log::error!("Error marshalling token inventory struct, error: {}", e);

                "Error marshalling tokens"
                    .to_string()
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },

        Err(e) => {
            log::error!("Error fetching proposal tokens, error: {}", e);

            "Error fetching tokens"
                .to_string()
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[post("/api/v1/fetchproposals")]
async fn fetch_proposals(tokens: web::Form<Vec<String>>) -> impl Responder {
    let mut client = super::model::new().expect("Unable to create a new client");

    if tokens.len() == 0 {
        return "{}".to_string().with_status(StatusCode::OK);
    }

    match client.fetch_batch_proposal(tokens.0).await {
        Ok(e) => format!("{:#?}", e).with_status(StatusCode::OK),

        Err(e) => {
            log::error!("Error fetching proposals, error: {}", e);

            "Error fetching proposals"
                .to_string()
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
