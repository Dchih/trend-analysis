use actix_web::{HttpResponse, Responder, get, post, web};

use crate::app_state::AppState;
use crate::models::keyword::{
    KeywordHistoryItem, KeywordSearchRequest, KeywordSearchResponse, KeywordStatusResponse,
};
use crate::services::task_queue::CollectTaskMessage;

#[post("/api/v1/keywords/search")]
pub async fn search_keyword(
    state: web::Data<AppState>,
    payload: web::Json<KeywordSearchRequest>,
) -> impl Responder {
    let keyword_id = state.next_keyword_id();
    let task_id = state.next_task_id();
    let queue = state.queue();

    let publish_result = queue.publish(CollectTaskMessage {
        task_id,
        keyword_id,
        keyword: payload.keyword.clone(),
        platform: "youtube",
        trigger: "manual_search",
    });

    match publish_result {
        Ok(()) => HttpResponse::Ok().json(KeywordSearchResponse {
            id: keyword_id,
            keyword: payload.keyword.clone(),
            task_status: "pending",
        }),
        Err(error) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": error
        })),
    }
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
