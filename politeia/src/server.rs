use super::types;
use actix_cors::Cors;
use actix_files::{Files as fs, NamedFile};
use actix_web::{
    get, http::header, http::StatusCode, post, web, App, HttpServer, Responder, Result,
};
use askama_actix::{Template, TemplateIntoResponse};
use politeia_api::{v1::errors::ErrorCode, v1::types as v1types};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

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

    let proposals: HashMap<String, v1types::Proposal> = HashMap::new();
    let proposal_mapper = Arc::new(RwLock::new(proposals));

    let tokens = Arc::new(RwLock::new(v1types::TokenInventory::default()));
    let policy = Arc::new(RwLock::new(v1types::Policy::default()));

    tokio::spawn(super::model::update_proposals(
        tokens.clone(),
        policy.clone(),
        proposal_mapper.clone(),
    ));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_header(header::CONTENT_TYPE)
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .data(tokens.clone())
            .data(policy.clone())
            .data(proposal_mapper.clone())
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
async fn fetch_tokens(tokens: web::Data<Arc<RwLock<v1types::TokenInventory>>>) -> impl Responder {
    let tokens = tokens.read().await;

    match serde_json::to_string(&*tokens) {
        Ok(e) => e.with_status(StatusCode::OK),

        Err(e) => {
            log::error!("Error marshalling token inventory struct, error: {}", e);

            "error sending tokens"
                .to_string()
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[post("/api/v1/fetchproposals")]
async fn fetch_proposals(
    tokens: actix_web::web::Json<types::Tokens>,
    policy: web::Data<Arc<RwLock<v1types::Policy>>>,
    proposals: web::Data<Arc<RwLock<HashMap<String, v1types::Proposal>>>>,
) -> impl Responder {
    if tokens.tokens.is_empty() {
        return "{}".to_string().with_status(StatusCode::OK);
    }

    // Ensure number of proposals requested does not pass limit.
    if tokens.tokens.len() > policy.read().await.proposal_list_page_size {
        let error_code: u8 = ErrorCode::StatusMaxProposalsExceededPolicy.into();
        return serde_json::json!({
            "code" : error_code,
        })
        .to_string()
        .with_status(StatusCode::BAD_REQUEST);
    }

    let mut proposal_result = v1types::ProposalsResult::default();
    let proposals = proposals.read().await;

    for token in &tokens.tokens {
        match proposals.get(token) {
            Some(e) => {
                proposal_result.proposals.push(e.clone());
            }

            None => {
                let error_code: u8 = ErrorCode::StatusInvalidCensorshipToken.into();
                return serde_json::json!({
                    "code" : error_code,
                })
                .to_string()
                .with_status(StatusCode::BAD_REQUEST);
            }
        }
    }

    drop(proposals);

    match serde_json::to_string(&proposal_result) {
        Ok(e) => e.with_status(StatusCode::OK),

        Err(e) => {
            log::error!("Error marshalling proposal result struct, error: {}", e);

            "error sending proposals"
                .to_string()
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
