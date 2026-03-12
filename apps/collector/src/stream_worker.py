import json
import os
from typing import Any

import redis

from src.persistence import InMemoryStore
from src.runtime import CollectorRuntime
from src.worker import CollectorWorker


class StreamWorker:
    def __init__(self, runtime: CollectorRuntime, store: InMemoryStore) -> None:
        self.worker = CollectorWorker(runtime, store)

    def process_payload(self, payload: str) -> dict[str, Any]:
        task = json.loads(payload)
        return self.worker.process(task)


def consume_forever(runtime: CollectorRuntime, store: InMemoryStore) -> None:
    redis_url = os.getenv("PRODUCT_RADAR_REDIS_URL", "redis://127.0.0.1:6379/0")
    stream_key = os.getenv("PRODUCT_RADAR_REDIS_STREAM", "collect_tasks")
    group_name = os.getenv("PRODUCT_RADAR_REDIS_GROUP", "collector_group")
    consumer_name = os.getenv("PRODUCT_RADAR_REDIS_CONSUMER", "worker_1")

    client = redis.Redis.from_url(redis_url, decode_responses=True)
    stream_worker = StreamWorker(runtime, store)

    try:
        client.xgroup_create(stream_key, group_name, id="0", mkstream=True)
    except redis.ResponseError as error:
        if "BUSYGROUP" not in str(error):
            raise

    while True:
        messages = client.xreadgroup(
            group_name,
            consumer_name,
            {stream_key: ">"},
            count=1,
            block=5000,
        )

        for _, entries in messages:
            for message_id, data in entries:
                payload = data["data"]
                stream_worker.process_payload(payload)
                client.xack(stream_key, group_name, message_id)
