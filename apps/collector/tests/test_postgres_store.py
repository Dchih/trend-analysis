from src.aggregates import recompute_keyword_daily_stats
from src.persistence import PostgresStore


def test_postgres_store_upserts_entities_and_stats():
    store = PostgresStore(
        "dbname=product_radar user=postgres password=Dz0504.. host=127.0.0.1 port=5432"
    )
    creators = [
        {
            "platform": "youtube",
            "platform_creator_id": "channel-it-postgres",
            "display_name": "Kitchen Lab IT",
        }
    ]
    content_items = [
        {
            "platform": "youtube",
            "platform_content_id": "video-it-postgres",
            "keyword_id": 1,
            "creator_id": "channel-it-postgres",
            "title": "Ninja Creami integration test",
            "url": "https://www.youtube.com/watch?v=video-it-postgres",
            "published_at": "2026-03-11T15:00:00Z",
            "view_count": 32000,
            "like_count": 1200,
            "comment_count": 88,
        }
    ]

    try:
        store.upsert_creators(creators)
        store.upsert_content_items(content_items)
        stats = recompute_keyword_daily_stats(store.content_items_for_keyword(1))
        store.replace_keyword_daily_stats(1, "youtube", stats)

        persisted_creators = store.creators()
        persisted_content = store.content_items_for_keyword(1)
        persisted_stats = store.keyword_daily_stats_for_keyword(1)

        assert any(
            creator["platform_creator_id"] == "channel-it-postgres"
            for creator in persisted_creators
        )
        assert any(
            item["platform_content_id"] == "video-it-postgres"
            for item in persisted_content
        )
        assert any(
            row["date"] == "2026-03-11" and row["total_views"] == 32000
            for row in persisted_stats
        )
    finally:
        store.delete_content_item("youtube", "video-it-postgres")
        store.delete_creator("youtube", "channel-it-postgres")
