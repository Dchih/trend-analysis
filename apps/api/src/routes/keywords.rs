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
    let repository = state.repository();
    let queue = state.queue();
    let keyword = match repository.create_or_get_keyword(&payload.keyword).await {
        Ok(keyword) => keyword,
        Err(error) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }));
        }
    };
    let task = match repository
        .create_collection_task(keyword.id, "youtube", "manual_search")
        .await
    {
        Ok(task) => task,
        Err(error) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": error
            }));
        }
    };

    let publish_result = queue
        .publish(CollectTaskMessage {
            task_id: task.id,
            keyword_id: keyword.id,
            keyword: payload.keyword.clone(),
            platform: "youtube".to_string(),
            trigger: "manual_search".to_string(),
        })
        .await;

    match publish_result {
        Ok(()) => HttpResponse::Ok().json(KeywordSearchResponse {
            id: keyword.id,
            keyword: keyword.keyword,
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
