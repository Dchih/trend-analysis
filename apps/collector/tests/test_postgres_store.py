import psycopg

from src.aggregates import recompute_keyword_daily_stats
from src.persistence import PostgresStore


def test_postgres_store_upserts_entities_and_stats():
    store = PostgresStore(
        "dbname=product_radar user=postgres password=Dz0504.. host=127.0.0.1 port=5432"
    )
    task_id = None
    with psycopg.connect(store.dsn) as connection:
        with connection.cursor() as cursor:
            cursor.execute(
                """
                INSERT INTO keywords (keyword, status)
                VALUES ('collector-postgres-it', 'active')
                ON CONFLICT (keyword) DO UPDATE SET status = EXCLUDED.status
                RETURNING id
                """
            )
            keyword_id = cursor.fetchone()[0]
            cursor.execute(
                """
                INSERT INTO collection_tasks (keyword_id, platform, trigger_type, status)
                VALUES (%s, 'youtube', 'manual_search', 'pending')
                RETURNING id
                """,
                (keyword_id,),
            )
            task_id = cursor.fetchone()[0]

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
            "keyword_id": keyword_id,
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
        stats = recompute_keyword_daily_stats(store.content_items_for_keyword(keyword_id))
        store.replace_keyword_daily_stats(keyword_id, "youtube", stats)
        store.mark_task_running(task_id)
        store.mark_task_succeeded(task_id, keyword_id)

        persisted_creators = store.creators()
        persisted_content = store.content_items_for_keyword(keyword_id)
        persisted_stats = store.keyword_daily_stats_for_keyword(keyword_id)

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
        with psycopg.connect(store.dsn) as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    "SELECT status, started_at IS NOT NULL, finished_at IS NOT NULL FROM collection_tasks WHERE id = %s",
                    (task_id,),
                )
                status, has_started_at, has_finished_at = cursor.fetchone()
                cursor.execute(
                    "SELECT last_collected_at IS NOT NULL FROM keywords WHERE id = %s",
                    (keyword_id,),
                )
                (has_last_collected_at,) = cursor.fetchone()

        assert status == "succeeded"
        assert has_started_at is True
        assert has_finished_at is True
        assert has_last_collected_at is True
    finally:
        store.delete_content_item("youtube", "video-it-postgres")
        store.delete_creator("youtube", "channel-it-postgres")
        with psycopg.connect(store.dsn) as connection:
            with connection.cursor() as cursor:
                if task_id is not None:
                    cursor.execute("DELETE FROM collection_tasks WHERE id = %s", (task_id,))
                cursor.execute(
                    "DELETE FROM keyword_daily_stats WHERE keyword_id = %s",
                    (keyword_id,),
                )
                cursor.execute("DELETE FROM keywords WHERE id = %s", (keyword_id,))
