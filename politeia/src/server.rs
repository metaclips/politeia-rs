use actix_web::{get, Responder};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "./dist/home.html")]
struct HomeTemplate {}

#[get("/")]
async fn index() -> impl Responder {
    let a = HomeTemplate {};
    a.into_response()
}
