from src.aggregates import recompute_keyword_daily_stats
from src.persistence import InMemoryStore, PostgresStore, Store
from src.runtime import CollectorRuntime


class CollectorWorker:
    def __init__(self, runtime: CollectorRuntime, store: Store) -> None:
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
        stats = recompute_keyword_daily_stats(
            self.store.content_items_for_keyword(task["keyword_id"])
        )
        self.store.replace_keyword_daily_stats(task["keyword_id"], platform, stats)

        return {
            "creators": normalized["creators"],
            "content_items": normalized["content_items"],
            "stats": stats,
        }


def build_default_worker() -> CollectorWorker:
    import os

    from src.plugins.youtube import YouTubePlugin
    from src.registry import PluginRegistry

    registry = PluginRegistry()
    registry.register(YouTubePlugin())
    runtime = CollectorRuntime(registry)
    database_url = os.getenv(
        "PRODUCT_RADAR_DATABASE_URL",
        "dbname=product_radar user=postgres password=Dz0504.. host=127.0.0.1 port=5432",
    )
    store = PostgresStore(database_url)
    return CollectorWorker(runtime, store)


if __name__ == "__main__":
    from src.stream_worker import consume_forever

    worker = build_default_worker()
    consume_forever(worker.runtime, worker.store)
