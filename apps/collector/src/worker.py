from src.aggregates import recompute_keyword_daily_stats
from src.persistence import InMemoryStore
from src.runtime import CollectorRuntime


class CollectorWorker:
    def __init__(self, runtime: CollectorRuntime, store: InMemoryStore) -> None:
        self.runtime = runtime
        self.store = store

    def process(self, task: dict) -> dict:
        platform = task["platform"]
        plugin = self.runtime.resolve_plugin(platform)
        if plugin is None:
            raise ValueError(f"unsupported platform: {platform}")

        raw_items = plugin.search(task["keyword"], task["time_range"])
        normalized = plugin.normalize(raw_items, keyword_id=task["keyword_id"])

        self.store.upsert_creators(normalized["creators"])
        self.store.upsert_content_items(normalized["content_items"])
        stats = recompute_keyword_daily_stats(self.store.content_items())

        return {
            "creators": normalized["creators"],
            "content_items": normalized["content_items"],
            "stats": stats,
        }


def build_default_worker() -> CollectorWorker:
    from src.plugins.youtube import YouTubePlugin
    from src.registry import PluginRegistry

    registry = PluginRegistry()
    registry.register(YouTubePlugin())
    runtime = CollectorRuntime(registry)
    store = InMemoryStore()
    return CollectorWorker(runtime, store)


if __name__ == "__main__":
    from src.stream_worker import consume_forever

    worker = build_default_worker()
    consume_forever(worker.runtime, worker.store)
