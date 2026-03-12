# Shared Schemas

P0 keeps the frontend, API, and collector aligned through JSON Schema contracts.

## Required Schema Files

- `tasks/collect-task.schema.json`
  - Required fields: `task_id`, `keyword`, `platforms`, `trigger`, `requested_at`, `time_range`, `priority`
- `domain/content-item.schema.json`
  - Required fields: `platform`, `platform_content_id`, `keyword_id`, `creator_id`, `title`, `url`, `published_at`
- `domain/creator.schema.json`
  - Required fields: `platform`, `platform_creator_id`, `display_name`
- `api/keyword-overview.schema.json`
  - Required fields: `keyword`, `total_contents`, `total_creators`, `total_views`, `last_collected_at`

## Notes

- `platform` is an enum even though P0 only ships with `youtube`.
- Task status values should be shared wherever task lifecycle is exposed.
- API responses should be page-oriented rather than table-oriented.
