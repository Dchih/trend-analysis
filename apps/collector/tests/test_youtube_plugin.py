from src.plugins.youtube import YouTubePlugin


def test_youtube_plugin_declares_capability_and_normalizes_entities():
    plugin = YouTubePlugin()
    raw_items = [
        {
            "id": {"videoId": "abc123"},
            "snippet": {
                "channelId": "channel-1",
                "channelTitle": "Kitchen Lab",
                "title": "Ninja Creami review",
                "description": "Testing the ice cream maker",
                "publishedAt": "2026-03-11T15:00:00Z",
                "thumbnails": {
                    "high": {"url": "https://img.youtube.com/vi/abc123/hqdefault.jpg"}
                },
            },
            "statistics": {
                "viewCount": "32000",
                "likeCount": "1200",
                "commentCount": "88",
            },
        }
    ]

    normalized = plugin.normalize(raw_items, keyword_id=1)

    assert plugin.platform == "youtube"
    assert len(normalized["creators"]) == 1
    assert len(normalized["content_items"]) == 1
    assert normalized["creators"][0]["platform_creator_id"] == "channel-1"
    assert normalized["content_items"][0]["platform_content_id"] == "abc123"
