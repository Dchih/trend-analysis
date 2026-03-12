mod app_state;
mod models;
mod routes;

use actix_web::{App, HttpServer, web};

use crate::app_state::AppState;
use crate::routes::health::health;
use crate::routes::keywords::{keyword_history, keyword_status, search_keyword};

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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| build_app(AppState))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
