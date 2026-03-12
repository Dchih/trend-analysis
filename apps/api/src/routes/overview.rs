use actix_web::{HttpResponse, Responder, get};

use crate::models::overview::{
    ContentCreatorSummary, KeywordOverviewResponse, LatestContentItem, TimelinePoint,
    TopCreatorSummary,
};

#[get("/api/v1/keywords/{id}/overview")]
pub async fn keyword_overview() -> impl Responder {
    HttpResponse::Ok().json(KeywordOverviewResponse {
        keyword: "ninja creami",
        total_contents: 24,
        total_creators: 9,
        total_views: 215_000,
        last_collected_at: Some("2026-03-12T08:00:00Z"),
        trend_delta: 12.4,
    })
}

#[get("/api/v1/keywords/{id}/timeline")]
pub async fn keyword_timeline() -> impl Responder {
    HttpResponse::Ok().json(vec![
        TimelinePoint {
            date: "2026-03-10",
            new_content_count: 3,
            total_views: 45_000,
            active_creator_count: 2,
        },
        TimelinePoint {
            date: "2026-03-11",
            new_content_count: 5,
            total_views: 88_000,
            active_creator_count: 4,
        },
    ])
}

#[get("/api/v1/keywords/{id}/creators/top")]
pub async fn top_creators() -> impl Responder {
    HttpResponse::Ok().json(vec![TopCreatorSummary {
        creator_id: 1,
        display_name: "Kitchen Lab",
        subscriber_count: 150_000,
        content_count: 4,
        total_views: 120_000,
        creator_score: 81.5,
    }])
}

#[get("/api/v1/keywords/{id}/contents/latest")]
pub async fn latest_contents() -> impl Responder {
    HttpResponse::Ok().json(vec![LatestContentItem {
        content_id: 1,
        title: "Ninja Creami recipes worth trying",
        thumbnail_url: "https://img.youtube.com/vi/abc123/maxresdefault.jpg",
        published_at: "2026-03-11T15:00:00Z",
        view_count: 32_000,
        creator: ContentCreatorSummary {
            creator_id: 1,
            display_name: "Kitchen Lab",
        },
    }])
}
