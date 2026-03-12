use actix_web::{HttpResponse, Responder, get, post, web};

use crate::models::keyword::{
    KeywordHistoryItem, KeywordSearchRequest, KeywordSearchResponse, KeywordStatusResponse,
};

#[post("/api/v1/keywords/search")]
pub async fn search_keyword(payload: web::Json<KeywordSearchRequest>) -> impl Responder {
    HttpResponse::Ok().json(KeywordSearchResponse {
        id: 1,
        keyword: payload.keyword.clone(),
        task_status: "pending",
    })
}

#[get("/api/v1/keywords/history")]
pub async fn keyword_history() -> impl Responder {
    HttpResponse::Ok().json(vec![KeywordHistoryItem {
        id: 1,
        keyword: "ninja creami",
    }])
}

#[get("/api/v1/keywords/{id}/status")]
pub async fn keyword_status(path: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok().json(KeywordStatusResponse {
        keyword_id: path.into_inner(),
        status: "pending",
        last_collected_at: None,
    })
}
