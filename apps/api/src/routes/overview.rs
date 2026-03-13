use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct OverviewQuery {
    pub range: Option<String>,
    pub limit: Option<u64>,
}

#[get("/api/v1/keywords/{id}/overview")]
pub async fn keyword_overview(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    query: web::Query<OverviewQuery>,
) -> impl Responder {
    let range = query.range.as_deref().unwrap_or("30d");
    match state.repository().fetch_overview(path.into_inner(), range).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        })),
    }
}

#[get("/api/v1/keywords/{id}/timeline")]
pub async fn keyword_timeline(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    query: web::Query<OverviewQuery>,
) -> impl Responder {
    let range = query.range.as_deref().unwrap_or("30d");
    match state.repository().fetch_timeline(path.into_inner(), range).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        })),
    }
}

#[get("/api/v1/keywords/{id}/creators/top")]
pub async fn top_creators(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    query: web::Query<OverviewQuery>,
) -> impl Responder {
    let range = query.range.as_deref().unwrap_or("30d");
    let limit = query.limit.unwrap_or(10);
    match state
        .repository()
        .fetch_top_creators(path.into_inner(), range, limit)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        })),
    }
}

#[get("/api/v1/keywords/{id}/contents/latest")]
pub async fn latest_contents(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    query: web::Query<OverviewQuery>,
) -> impl Responder {
    let range = query.range.as_deref().unwrap_or("30d");
    let limit = query.limit.unwrap_or(20);
    match state
        .repository()
        .fetch_latest_contents(path.into_inner(), range, limit)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        })),
    }
}
