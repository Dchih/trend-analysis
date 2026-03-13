from __future__ import annotations

from typing import Protocol

import psycopg


class Store(Protocol):
    def upsert_creators(self, creators: list[dict]) -> None: ...
    def upsert_content_items(self, content_items: list[dict]) -> None: ...
    def creators(self) -> list[dict]: ...
    def content_items(self) -> list[dict]: ...
    def content_items_for_keyword(self, keyword_id: int) -> list[dict]: ...
    def replace_keyword_daily_stats(
        self, keyword_id: int, platform: str, stats: list[dict]
    ) -> None: ...
    def keyword_daily_stats_for_keyword(self, keyword_id: int) -> list[dict]: ...
    def mark_task_running(self, task_id: int) -> None: ...
    def mark_task_succeeded(self, task_id: int, keyword_id: int) -> None: ...
    def mark_task_failed(self, task_id: int, error_message: str) -> None: ...


class InMemoryStore:
    def __init__(self) -> None:
        self._creators: dict[tuple[str, str], dict] = {}
        self._content_items: dict[tuple[str, str], dict] = {}
        self._keyword_daily_stats: dict[tuple[int, str, str], dict] = {}

    def upsert_creators(self, creators: list[dict]) -> None:
        for creator in creators:
            key = (creator["platform"], creator["platform_creator_id"])
            self._creators[key] = creator

    def upsert_content_items(self, content_items: list[dict]) -> None:
        for content_item in content_items:
            key = (content_item["platform"], content_item["platform_content_id"])
            self._content_items[key] = content_item

    def creators(self) -> list[dict]:
        return list(self._creators.values())

    def content_items(self) -> list[dict]:
        return list(self._content_items.values())

    def content_items_for_keyword(self, keyword_id: int) -> list[dict]:
        return [
            item for item in self._content_items.values() if item["keyword_id"] == keyword_id
        ]

    def replace_keyword_daily_stats(
        self, keyword_id: int, platform: str, stats: list[dict]
    ) -> None:
        for key in list(self._keyword_daily_stats):
            if key[0] == keyword_id and key[1] == platform:
                del self._keyword_daily_stats[key]

        for row in stats:
            key = (row["keyword_id"], row["platform"], row["date"])
            self._keyword_daily_stats[key] = row

    def keyword_daily_stats_for_keyword(self, keyword_id: int) -> list[dict]:
        return [
            row
            for row in self._keyword_daily_stats.values()
            if row["keyword_id"] == keyword_id
        ]

    def mark_task_running(self, task_id: int) -> None:
        return None

    def mark_task_succeeded(self, task_id: int, keyword_id: int) -> None:
        return None

    def mark_task_failed(self, task_id: int, error_message: str) -> None:
        return None


class PostgresStore:
    def __init__(self, dsn: str) -> None:
        self.dsn = dsn

    def _connect(self):
        return psycopg.connect(self.dsn)

    def upsert_creators(self, creators: list[dict]) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                for creator in creators:
                    cursor.execute(
                        """
                        INSERT INTO creators (
                            platform,
                            platform_creator_id,
                            display_name,
                            handle,
                            avatar_url,
                            subscriber_count,
                            video_count,
                            creator_score,
                            raw_payload
                        )
                        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s::jsonb)
                        ON CONFLICT (platform, platform_creator_id)
                        DO UPDATE SET
                            display_name = EXCLUDED.display_name,
                            handle = EXCLUDED.handle,
                            avatar_url = EXCLUDED.avatar_url,
                            subscriber_count = EXCLUDED.subscriber_count,
                            video_count = EXCLUDED.video_count,
                            creator_score = EXCLUDED.creator_score,
                            raw_payload = EXCLUDED.raw_payload,
                            updated_at = NOW()
                        """,
                        (
                            creator["platform"],
                            creator["platform_creator_id"],
                            creator["display_name"],
                            creator.get("handle"),
                            creator.get("avatar_url"),
                            creator.get("subscriber_count", 0),
                            creator.get("video_count", 0),
                            creator.get("creator_score", 0),
                            "{}",
                        ),
                    )

    def upsert_content_items(self, content_items: list[dict]) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                for item in content_items:
                    cursor.execute(
                        """
                        SELECT id
                        FROM creators
                        WHERE platform = %s AND platform_creator_id = %s
                        """,
                        (item["platform"], item["creator_id"]),
                    )
                    creator_row = cursor.fetchone()
                    if creator_row is None:
                        raise ValueError(
                            f"creator not found for {item['platform']}:{item['creator_id']}"
                        )

                    cursor.execute(
                        """
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
                        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s::jsonb)
                        ON CONFLICT (platform, platform_content_id)
                        DO UPDATE SET
                            keyword_id = EXCLUDED.keyword_id,
                            creator_id = EXCLUDED.creator_id,
                            title = EXCLUDED.title,
                            description = EXCLUDED.description,
                            url = EXCLUDED.url,
                            thumbnail_url = EXCLUDED.thumbnail_url,
                            published_at = EXCLUDED.published_at,
                            view_count = EXCLUDED.view_count,
                            like_count = EXCLUDED.like_count,
                            comment_count = EXCLUDED.comment_count,
                            engagement_score = EXCLUDED.engagement_score,
                            raw_payload = EXCLUDED.raw_payload,
                            collected_at = NOW()
                        """,
                        (
                            item["platform"],
                            item["platform_content_id"],
                            item["keyword_id"],
                            creator_row[0],
                            item["title"],
                            item.get("description", ""),
                            item["url"],
                            item.get("thumbnail_url"),
                            item["published_at"],
                            item.get("view_count", 0),
                            item.get("like_count", 0),
                            item.get("comment_count", 0),
                            item.get("engagement_score", 0),
                            "{}",
                        ),
                    )

    def creators(self) -> list[dict]:
        with self._connect() as connection:
            with connection.cursor(row_factory=psycopg.rows.dict_row) as cursor:
                cursor.execute(
                    """
                    SELECT platform, platform_creator_id, display_name
                    FROM creators
                    ORDER BY id DESC
                    """
                )
                return list(cursor.fetchall())

    def content_items(self) -> list[dict]:
        with self._connect() as connection:
            with connection.cursor(row_factory=psycopg.rows.dict_row) as cursor:
                cursor.execute(
                    """
                    SELECT
                        c.platform,
                        c.platform_content_id,
                        c.keyword_id,
                        cr.platform_creator_id AS creator_id,
                        c.title,
                        c.url,
                        c.published_at::text AS published_at,
                        c.view_count,
                        c.like_count,
                        c.comment_count
                    FROM content_items c
                    JOIN creators cr ON cr.id = c.creator_id
                    ORDER BY c.id DESC
                    """
                )
                return list(cursor.fetchall())

    def content_items_for_keyword(self, keyword_id: int) -> list[dict]:
        with self._connect() as connection:
            with connection.cursor(row_factory=psycopg.rows.dict_row) as cursor:
                cursor.execute(
                    """
                    SELECT
                        c.platform,
                        c.platform_content_id,
                        c.keyword_id,
                        cr.platform_creator_id AS creator_id,
                        c.title,
                        c.url,
                        c.published_at::text AS published_at,
                        c.view_count,
                        c.like_count,
                        c.comment_count
                    FROM content_items c
                    JOIN creators cr ON cr.id = c.creator_id
                    WHERE c.keyword_id = %s
                    ORDER BY c.id DESC
                    """,
                    (keyword_id,),
                )
                return list(cursor.fetchall())

    def replace_keyword_daily_stats(
        self, keyword_id: int, platform: str, stats: list[dict]
    ) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    DELETE FROM keyword_daily_stats
                    WHERE keyword_id = %s AND platform = %s
                    """,
                    (keyword_id, platform),
                )
                for row in stats:
                    cursor.execute(
                        """
                        INSERT INTO keyword_daily_stats (
                            keyword_id,
                            platform,
                            date,
                            new_content_count,
                            total_views,
                            total_likes,
                            active_creator_count
                        )
                        VALUES (%s, %s, %s, %s, %s, %s, %s)
                        """,
                        (
                            row["keyword_id"],
                            row["platform"],
                            row["date"],
                            row["new_content_count"],
                            row["total_views"],
                            row["total_likes"],
                            row["active_creator_count"],
                        ),
                    )

    def keyword_daily_stats_for_keyword(self, keyword_id: int) -> list[dict]:
        with self._connect() as connection:
            with connection.cursor(row_factory=psycopg.rows.dict_row) as cursor:
                cursor.execute(
                    """
                    SELECT
                        keyword_id,
                        platform,
                        date::text AS date,
                        new_content_count,
                        total_views,
                        total_likes,
                        active_creator_count
                    FROM keyword_daily_stats
                    WHERE keyword_id = %s
                    ORDER BY date DESC
                    """,
                    (keyword_id,),
                )
                return list(cursor.fetchall())

    def mark_task_running(self, task_id: int) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    UPDATE collection_tasks
                    SET status = 'running', started_at = NOW(), error_message = NULL
                    WHERE id = %s
                    """,
                    (task_id,),
                )

    def mark_task_succeeded(self, task_id: int, keyword_id: int) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    UPDATE collection_tasks
                    SET status = 'succeeded', finished_at = NOW(), error_message = NULL
                    WHERE id = %s
                    """,
                    (task_id,),
                )
                cursor.execute(
                    """
                    UPDATE keywords
                    SET last_collected_at = NOW()
                    WHERE id = %s
                    """,
                    (keyword_id,),
                )

    def mark_task_failed(self, task_id: int, error_message: str) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    UPDATE collection_tasks
                    SET status = 'failed', finished_at = NOW(), error_message = %s
                    WHERE id = %s
                    """,
                    (error_message[:1000], task_id),
                )

    def delete_content_item(self, platform: str, platform_content_id: str) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    DELETE FROM content_items
                    WHERE platform = %s AND platform_content_id = %s
                    """,
                    (platform, platform_content_id),
                )

    def delete_creator(self, platform: str, platform_creator_id: str) -> None:
        with self._connect() as connection:
            with connection.cursor() as cursor:
                cursor.execute(
                    """
                    DELETE FROM creators
                    WHERE platform = %s AND platform_creator_id = %s
                    """,
                    (platform, platform_creator_id),
                )
