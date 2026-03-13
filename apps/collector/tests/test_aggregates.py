from src.aggregates import recompute_keyword_daily_stats
from src.persistence import InMemoryStore


def test_persistence_upserts_entities_and_recomputes_daily_stats():
    store = InMemoryStore()
    creators = [
        {
            "platform": "youtube",
            "platform_creator_id": "channel-1",
            "display_name": "Kitchen Lab",
        }
    ]
    content_items = [
        {
            "platform": "youtube",
            "platform_content_id": "abc123",
            "keyword_id": 1,
            "creator_id": "channel-1",
            "title": "Ninja Creami review",
            "url": "https://www.youtube.com/watch?v=abc123",
            "published_at": "2026-03-11T15:00:00Z",
            "view_count": 32000,
            "like_count": 1200,
            "comment_count": 88,
        }
    ]

    store.upsert_creators(creators)
    store.upsert_content_items(content_items)
    stats = recompute_keyword_daily_stats(store.content_items())

    assert len(store.creators()) == 1
    assert len(store.content_items()) == 1
    assert stats == [
        {
            "keyword_id": 1,
            "platform": "youtube",
            "date": "2026-03-11",
            "new_content_count": 1,
            "total_views": 32000,
            "total_likes": 1200,
            "active_creator_count": 1,
        }
    ]
