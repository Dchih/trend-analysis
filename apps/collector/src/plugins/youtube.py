from __future__ import annotations

import json
import os
from datetime import datetime, timedelta, timezone
from typing import Callable
from urllib.parse import urlencode
from urllib.request import urlopen

from src.plugins.base import BasePlugin


class YouTubePlugin(BasePlugin):
    platform = "youtube"

    def __init__(
        self,
        api_key: str | None = None,
        fetch_json: Callable[[str], dict] | None = None,
    ) -> None:
        self.api_key = api_key or os.getenv("PRODUCT_RADAR_YOUTUBE_API_KEY", "")
        self.fetch_json = fetch_json or self._default_fetch_json

    def search(self, keyword: str, time_range: str) -> list[dict]:
        if not self.api_key:
            return []

        search_params = urlencode(
            {
                "part": "snippet",
                "type": "video",
                "q": keyword,
                "maxResults": 10,
                "order": "date",
                "publishedAfter": self._published_after(time_range),
                "key": self.api_key,
            }
        )
        search_response = self.fetch_json(
            f"https://www.googleapis.com/youtube/v3/search?{search_params}"
        )

        items = search_response.get("items", [])
        video_ids = [
            item.get("id", {}).get("videoId")
            for item in items
            if item.get("id", {}).get("videoId")
        ]
        if not video_ids:
            return items

        videos_params = urlencode(
            {
                "part": "statistics,snippet",
                "id": ",".join(video_ids),
                "key": self.api_key,
            }
        )
        videos_response = self.fetch_json(
            f"https://www.googleapis.com/youtube/v3/videos?{videos_params}"
        )
        statistics_by_id = {
            item["id"]: item.get("statistics", {})
            for item in videos_response.get("items", [])
        }

        merged_items = []
        for item in items:
            video_id = item.get("id", {}).get("videoId")
            merged_item = dict(item)
            merged_item["statistics"] = statistics_by_id.get(video_id, {})
            merged_items.append(merged_item)

        return merged_items

    def normalize(self, raw_items: list[dict], keyword_id: int) -> dict:
        creators = []
        content_items = []

        for raw_item in raw_items:
            snippet = raw_item.get("snippet", {})
            statistics = raw_item.get("statistics", {})
            video_id = raw_item.get("id", {}).get("videoId")
            channel_id = snippet.get("channelId")

            creators.append(
                {
                    "platform": "youtube",
                    "platform_creator_id": channel_id,
                    "display_name": snippet.get("channelTitle", ""),
                    "handle": None,
                    "avatar_url": None,
                    "subscriber_count": 0,
                    "video_count": 0,
                    "creator_score": 0,
                    "raw_payload": raw_item,
                }
            )
            content_items.append(
                {
                    "platform": "youtube",
                    "platform_content_id": video_id,
                    "keyword_id": keyword_id,
                    "creator_id": channel_id,
                    "title": snippet.get("title", ""),
                    "description": snippet.get("description", ""),
                    "url": f"https://www.youtube.com/watch?v={video_id}",
                    "thumbnail_url": snippet.get("thumbnails", {})
                    .get("high", {})
                    .get("url"),
                    "published_at": snippet.get("publishedAt"),
                    "view_count": int(statistics.get("viewCount", 0)),
                    "like_count": int(statistics.get("likeCount", 0)),
                    "comment_count": int(statistics.get("commentCount", 0)),
                    "engagement_score": float(
                        int(statistics.get("likeCount", 0))
                        + int(statistics.get("commentCount", 0))
                    ),
                    "raw_payload": raw_item,
                }
            )

        return {
            "creators": creators,
            "content_items": content_items,
        }

    def _default_fetch_json(self, url: str) -> dict:
        with urlopen(url) as response:
            return json.loads(response.read().decode("utf-8"))

    def _published_after(self, time_range: str) -> str:
        days = {"7d": 7, "30d": 30, "90d": 90}.get(time_range, 30)
        published_after = datetime.now(timezone.utc) - timedelta(days=days)
        return published_after.replace(microsecond=0).isoformat().replace("+00:00", "Z")
