use actix_web::{App, http::StatusCode, test, web};

#[path = "../src/app_state.rs"]
mod app_state;
#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/routes/mod.rs"]
mod routes;
#[path = "../src/services/mod.rs"]
mod services;

use app_state::AppState;
use routes::health::health;
use routes::keywords::{keyword_history, keyword_status, search_keyword};

#[actix_web::test]
async fn health_returns_ok() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(health)
            .service(search_keyword)
            .service(keyword_history)
            .service(keyword_status),
    )
    .await;

    let request = test::TestRequest::get().uri("/health").to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);
}
