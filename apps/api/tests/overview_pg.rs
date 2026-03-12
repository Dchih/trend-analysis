use tokio_postgres::NoTls;

#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/repositories/mod.rs"]
mod repositories;

use repositories::keywords::{KeywordRepository, PgKeywordRepository};

#[actix_web::test]
async fn postgres_repository_reads_overview_aggregates() {
    let (client, connection) = tokio_postgres::connect(
        "postgres://postgres:Dz0504..@127.0.0.1:5432/product_radar",
        NoTls,
    )
    .await
    .expect("postgres connect");
    actix_web::rt::spawn(async move {
        let _ = connection.await;
    });

    let keyword_row = client
        .query_one(
            "
            INSERT INTO keywords (keyword, status)
            VALUES ('integration-keyword-api', 'active')
            ON CONFLICT (keyword) DO UPDATE SET status = EXCLUDED.status
            RETURNING id
            ",
            &[],
        )
        .await
        .expect("insert keyword");
    let keyword_id = keyword_row.get::<_, i64>(0);

    let creator_row = client
        .query_one(
            "
            INSERT INTO creators (
                platform,
                platform_creator_id,
                display_name,
                subscriber_count,
                video_count,
                creator_score,
                raw_payload
            )
            VALUES ('youtube', 'api-int-creator', 'API Brew Lab', 1000, 2, 55.0, '{}'::jsonb)
            ON CONFLICT (platform, platform_creator_id)
            DO UPDATE SET display_name = EXCLUDED.display_name
            RETURNING id
            ",
            &[],
        )
        .await
        .expect("insert creator");
    let creator_id = creator_row.get::<_, i64>(0);

    client
        .execute(
            "
            INSERT INTO content_items (
                platform,
                platform_content_id,
                keyword_id,
                creator_id,
                title,
                description,
                url,
                thumbnail_url,
                published_at,
                view_count,
                like_count,
                comment_count,
                engagement_score,
                raw_payload
            )
            VALUES (
                'youtube',
                'api-int-video',
                $1,
                $2,
                'API integration video',
                '',
                'https://www.youtube.com/watch?v=api-int-video',
                'https://img.youtube.com/vi/api-int-video/hqdefault.jpg',
                '2026-03-11T15:00:00Z',
                32000,
                1200,
                88,
                1288,
                '{}'::jsonb
            )
            ON CONFLICT (platform, platform_content_id)
            DO UPDATE SET keyword_id = EXCLUDED.keyword_id, creator_id = EXCLUDED.creator_id
            ",
            &[&keyword_id, &creator_id],
        )
        .await
        .expect("insert content");

    client
        .execute(
            "
            INSERT INTO keyword_daily_stats (
                keyword_id,
                platform,
                date,
                new_content_count,
                total_views,
                total_likes,
                active_creator_count
            )
            VALUES ($1, 'youtube', '2026-03-11', 1, 32000, 1200, 1)
            ON CONFLICT (keyword_id, platform, date)
            DO UPDATE SET
                total_views = EXCLUDED.total_views,
                total_likes = EXCLUDED.total_likes,
                active_creator_count = EXCLUDED.active_creator_count
            ",
            &[&keyword_id],
        )
        .await
        .expect("insert stats");

    let repository = PgKeywordRepository::new(client);

    let overview = repository
        .fetch_overview(keyword_id as u64, "30d")
        .await
        .expect("overview");
    let timeline = repository
        .fetch_timeline(keyword_id as u64, "30d")
        .await
        .expect("timeline");
    let creators = repository
        .fetch_top_creators(keyword_id as u64, "30d", 10)
        .await
        .expect("creators");
    let latest = repository
        .fetch_latest_contents(keyword_id as u64, "30d", 10)
        .await
        .expect("latest");

    assert_eq!(overview.keyword, "integration-keyword-api");
    assert_eq!(overview.total_contents, 1);
    assert_eq!(overview.total_views, 32000);
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].total_views, 32000);
    assert_eq!(creators.len(), 1);
    assert_eq!(creators[0].display_name, "API Brew Lab");
    assert_eq!(latest.len(), 1);
    assert_eq!(latest[0].title, "API integration video");

    let (cleanup_client, cleanup_connection) = tokio_postgres::connect(
        "postgres://postgres:Dz0504..@127.0.0.1:5432/product_radar",
        NoTls,
    )
    .await
    .expect("postgres reconnect");
    actix_web::rt::spawn(async move {
        let _ = cleanup_connection.await;
    });

    cleanup_client
        .execute(
            "DELETE FROM keyword_daily_stats WHERE keyword_id = $1",
            &[&keyword_id],
        )
        .await
        .expect("cleanup stats");
    cleanup_client
        .execute(
            "DELETE FROM content_items WHERE platform = 'youtube' AND platform_content_id = 'api-int-video'",
            &[],
        )
        .await
        .expect("cleanup content");
    cleanup_client
        .execute(
            "DELETE FROM creators WHERE platform = 'youtube' AND platform_creator_id = 'api-int-creator'",
            &[],
        )
        .await
        .expect("cleanup creator");
    cleanup_client
        .execute("DELETE FROM keywords WHERE id = $1", &[&keyword_id])
        .await
        .expect("cleanup keyword");
}
