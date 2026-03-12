from src.plugins.base import BasePlugin


class YouTubePlugin(BasePlugin):
    platform = "youtube"

    def search(self, keyword: str, time_range: str) -> list[dict]:
        return []

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
