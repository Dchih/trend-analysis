def recompute_keyword_daily_stats(content_items: list[dict]) -> list[dict]:
    buckets: dict[tuple[int, str, str], dict] = {}

    for item in content_items:
        date = item["published_at"][:10]
        key = (item["keyword_id"], item["platform"], date)

        if key not in buckets:
            buckets[key] = {
                "keyword_id": item["keyword_id"],
                "platform": item["platform"],
                "date": date,
                "new_content_count": 0,
                "total_views": 0,
                "total_likes": 0,
                "active_creator_ids": set(),
            }

        bucket = buckets[key]
        bucket["new_content_count"] += 1
        bucket["total_views"] += item.get("view_count", 0)
        bucket["total_likes"] += item.get("like_count", 0)
        bucket["active_creator_ids"].add(item["creator_id"])

    results = []
    for bucket in buckets.values():
        results.append(
            {
                "keyword_id": bucket["keyword_id"],
                "platform": bucket["platform"],
                "date": bucket["date"],
                "new_content_count": bucket["new_content_count"],
                "total_views": bucket["total_views"],
                "total_likes": bucket["total_likes"],
                "active_creator_count": len(bucket["active_creator_ids"]),
            }
        )

    return sorted(results, key=lambda row: (row["keyword_id"], row["platform"], row["date"]))
