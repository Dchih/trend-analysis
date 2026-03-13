# SQL Bootstrap

P0 requires one initial PostgreSQL migration with the core ProductRadar tables.

## Required Tables

- `keywords`
- `collection_tasks`
- `creators`
- `content_items`
- `keyword_daily_stats`

## Required Indexes

- unique index on `keywords(keyword)`
- unique index on `creators(platform, platform_creator_id)`
- unique index on `content_items(platform, platform_content_id)`
- index on `content_items(keyword_id, published_at desc)`
- index on `keyword_daily_stats(keyword_id, date)`
- index on `collection_tasks(keyword_id, requested_at desc)`
