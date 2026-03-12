use actix_web::{App, http::StatusCode, test, web};

#[path = "../src/app_state.rs"]
mod app_state;
#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/repositories/mod.rs"]
mod repositories;
#[path = "../src/routes/mod.rs"]
mod routes;
#[path = "../src/services/mod.rs"]
mod services;

use app_state::AppState;
use routes::health::health;
use routes::keywords::{keyword_history, keyword_status, search_keyword};
use routes::overview::{keyword_overview, keyword_timeline, latest_contents, top_creators};

#[actix_web::test]
async fn overview_endpoint_returns_expected_shape() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(health)
            .service(search_keyword)
            .service(keyword_history)
            .service(keyword_status)
            .service(keyword_overview)
            .service(keyword_timeline)
            .service(top_creators)
            .service(latest_contents),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/api/v1/keywords/1/overview?range=30d")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["keyword"], "ninja creami");
    assert!(body.get("total_contents").is_some());
    assert!(body.get("total_creators").is_some());
    assert!(body.get("total_views").is_some());
    assert!(body.get("last_collected_at").is_some());
}

#[actix_web::test]
async fn timeline_endpoint_returns_points() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(health)
            .service(search_keyword)
            .service(keyword_history)
            .service(keyword_status)
            .service(keyword_overview)
            .service(keyword_timeline)
            .service(top_creators)
            .service(latest_contents),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/api/v1/keywords/1/timeline?range=30d")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    assert!(body.as_array().is_some());
}

#[actix_web::test]
async fn top_creators_endpoint_returns_ranked_creators() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(health)
            .service(search_keyword)
            .service(keyword_history)
            .service(keyword_status)
            .service(keyword_overview)
            .service(keyword_timeline)
            .service(top_creators)
            .service(latest_contents),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/api/v1/keywords/1/creators/top?range=30d&limit=10")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    let items = body.as_array().expect("expected array");
    assert!(!items.is_empty());
    assert!(items[0].get("display_name").is_some());
}

#[actix_web::test]
async fn latest_contents_endpoint_returns_cards() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(health)
            .service(search_keyword)
            .service(keyword_history)
            .service(keyword_status)
            .service(keyword_overview)
            .service(keyword_timeline)
            .service(top_creators)
            .service(latest_contents),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/api/v1/keywords/1/contents/latest?range=30d&limit=20")
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    let items = body.as_array().expect("expected array");
    assert!(!items.is_empty());
    assert!(items[0].get("title").is_some());
    assert!(items[0].get("creator").is_some());
}
