mod app_state;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer, web};

use crate::app_state::AppState;
use crate::routes::health::health;
use crate::routes::keywords::{keyword_history, keyword_status, search_keyword};
use crate::routes::overview::{keyword_overview, keyword_timeline, latest_contents, top_creators};

pub fn build_app(state: AppState) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(state))
        .service(health)
        .service(search_keyword)
        .service(keyword_history)
        .service(keyword_status)
        .service(keyword_overview)
        .service(keyword_timeline)
        .service(top_creators)
        .service(latest_contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| build_app(AppState::default()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
