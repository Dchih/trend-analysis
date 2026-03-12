from src.persistence import InMemoryStore
from src.registry import PluginRegistry
from src.runtime import CollectorRuntime
from src.worker import CollectorWorker


class DummyPlugin:
    platform = "youtube"

    def search(self, keyword: str, time_range: str) -> list[dict]:
        return [
            {
                "id": {"videoId": "abc123"},
                "snippet": {
                    "channelId": "channel-1",
                    "channelTitle": "Kitchen Lab",
                    "title": "Ninja Creami review",
                    "description": "Testing the ice cream maker",
                    "publishedAt": "2026-03-11T15:00:00Z",
                    "thumbnails": {
                        "high": {
                            "url": "https://img.youtube.com/vi/abc123/hqdefault.jpg"
                        }
                    },
                },
                "statistics": {
                    "viewCount": "32000",
                    "likeCount": "1200",
                    "commentCount": "88",
                },
            }
        ]

    def normalize(self, raw_items: list[dict], keyword_id: int) -> dict:
        return {
            "creators": [
                {
                    "platform": "youtube",
                    "platform_creator_id": "channel-1",
                    "display_name": "Kitchen Lab",
                }
            ],
            "content_items": [
                {
                    "platform": "youtube",
                    "platform_content_id": "abc123",
                    "keyword_id": keyword_id,
                    "creator_id": "channel-1",
                    "title": "Ninja Creami review",
                    "url": "https://www.youtube.com/watch?v=abc123",
                    "published_at": "2026-03-11T15:00:00Z",
                    "view_count": 32000,
                    "like_count": 1200,
                    "comment_count": 88,
                }
            ],
        }


def test_worker_processes_task_and_updates_store():
    registry = PluginRegistry()
    registry.register(DummyPlugin())
    runtime = CollectorRuntime(registry)
    store = InMemoryStore()
    worker = CollectorWorker(runtime, store)

    result = worker.process(
        {
            "platform": "youtube",
            "keyword": "ninja creami",
            "keyword_id": 1,
            "time_range": "30d",
        }
    )

    assert len(store.creators()) == 1
    assert len(store.content_items()) == 1
    assert result["stats"][0]["total_views"] == 32000
