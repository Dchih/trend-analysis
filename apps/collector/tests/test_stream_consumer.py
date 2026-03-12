from src.persistence import InMemoryStore
from src.registry import PluginRegistry
from src.runtime import CollectorRuntime
from src.stream_worker import StreamWorker, process_entries, process_pending_entries


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


class ExplodingPlugin(DummyPlugin):
    def search(self, keyword: str, time_range: str) -> list[dict]:
        raise RuntimeError("boom")


class RecordingRedisClient:
    def __init__(self, pending_messages=None) -> None:
        self.acked: list[tuple[str, str, str]] = []
        self.pending_messages = pending_messages or []

    def xack(self, stream_key: str, group_name: str, message_id: str) -> None:
        self.acked.append((stream_key, group_name, message_id))

    def xreadgroup(self, group_name, consumer_name, streams, count, block):
        return self.pending_messages


def build_stream_worker(plugin) -> StreamWorker:
    registry = PluginRegistry()
    registry.register(plugin)
    runtime = CollectorRuntime(registry)
    return StreamWorker(runtime, InMemoryStore())


def test_process_entries_acks_successful_messages():
    redis_client = RecordingRedisClient()
    worker = build_stream_worker(DummyPlugin())

    process_entries(
        redis_client,
        worker,
        "collect_tasks",
        "collector_group",
        [("collect_tasks", [("1-0", {"data": '{"platform":"youtube","keyword":"ninja creami","keyword_id":1,"time_range":"30d"}'})])],
    )

    assert redis_client.acked == [("collect_tasks", "collector_group", "1-0")]


def test_process_entries_skips_ack_when_processing_fails():
    redis_client = RecordingRedisClient()
    worker = build_stream_worker(ExplodingPlugin())

    process_entries(
        redis_client,
        worker,
        "collect_tasks",
        "collector_group",
        [("collect_tasks", [("1-0", {"data": '{"platform":"youtube","keyword":"ninja creami","keyword_id":1,"time_range":"30d"}'})])],
    )

    assert redis_client.acked == []


def test_process_pending_entries_replays_unacked_messages():
    redis_client = RecordingRedisClient(
        pending_messages=[
            (
                "collect_tasks",
                [
                    (
                        "1-0",
                        {
                            "data": '{"platform":"youtube","keyword":"ninja creami","keyword_id":1,"time_range":"30d"}'
                        },
                    )
                ],
            )
        ]
    )
    worker = build_stream_worker(DummyPlugin())

    process_pending_entries(
        redis_client,
        worker,
        "collect_tasks",
        "collector_group",
        "worker_1",
    )

    assert redis_client.acked == [("collect_tasks", "collector_group", "1-0")]
