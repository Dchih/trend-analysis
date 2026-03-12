import json
import logging
import os
from typing import Any

import redis

from src.persistence import Store
from src.runtime import CollectorRuntime
from src.worker import CollectorWorker

logger = logging.getLogger(__name__)


class StreamWorker:
    def __init__(self, runtime: CollectorRuntime, store: Store) -> None:
        self.worker = CollectorWorker(runtime, store)

    def process_payload(self, payload: str) -> dict[str, Any]:
        task = json.loads(payload)
        return self.worker.process(task)


def process_entries(
    client,
    stream_worker: StreamWorker,
    stream_key: str,
    group_name: str,
    messages,
) -> None:
    for _, entries in messages:
        for message_id, data in entries:
            payload = data["data"]
            try:
                stream_worker.process_payload(payload)
            except Exception as error:  # noqa: BLE001
                logger.exception("failed to process stream message %s: %s", message_id, error)
                continue

            client.xack(stream_key, group_name, message_id)


def process_pending_entries(
    client,
    stream_worker: StreamWorker,
    stream_key: str,
    group_name: str,
    consumer_name: str,
) -> None:
    pending = client.xreadgroup(
        group_name,
        consumer_name,
        {stream_key: "0"},
        count=10,
        block=1000,
    )
    process_entries(client, stream_worker, stream_key, group_name, pending)


def consume_forever(runtime: CollectorRuntime, store: Store) -> None:
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

    process_pending_entries(client, stream_worker, stream_key, group_name, consumer_name)

    while True:
        messages = client.xreadgroup(
            group_name,
            consumer_name,
            {stream_key: ">"},
            count=1,
            block=5000,
        )
        process_entries(client, stream_worker, stream_key, group_name, messages)
