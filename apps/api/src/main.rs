mod app_state;
mod config;
mod models;
mod repositories;
mod routes;
mod services;

use std::{io, sync::Arc};

use actix_web::{App, HttpServer, web};
use redis::Client as RedisClient;
use tokio_postgres::NoTls;

use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::repositories::keywords::PgKeywordRepository;
use crate::routes::health::health;
use crate::routes::keywords::{keyword_history, keyword_status, search_keyword};
use crate::routes::overview::{keyword_overview, keyword_timeline, latest_contents, top_creators};
use crate::services::task_queue::RedisTaskQueue;

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
    let config = AppConfig::from_env();

    let (pg_client, pg_connection) = tokio_postgres::connect(&config.database_url, NoTls)
        .await
        .map_err(io::Error::other)?;
    tokio::spawn(async move {
        if let Err(error) = pg_connection.await {
            eprintln!("postgres connection error: {error}");
        }
    });

    let redis_client = RedisClient::open(config.redis_url.clone()).map_err(io::Error::other)?;
    let state = AppState::new(
        Arc::new(PgKeywordRepository::new(pg_client)),
        Arc::new(RedisTaskQueue::new(redis_client, config.redis_stream.clone())),
    );

    HttpServer::new(move || build_app(state.clone()))
        .bind(&config.bind_addr)?
        .run()
        .await
}
