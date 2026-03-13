# ProductRadar P0 Architecture Design

**Date:** 2026-03-12

**Scope:** P0 architecture landing for a YouTube-first product intelligence platform with plugin-based collector expansion.

---

## 1. Goal

P0 focuses on a single end-to-end loop:

`keyword search -> YouTube collection -> creator aggregation -> timeline aggregation -> overview page`

The purpose of this phase is to land a stable architecture that is small enough to implement quickly, while preserving clean expansion points for future platforms.

---

## 2. Product Boundary

P0 includes:

- Search page
- Keyword detail page `Overview`
- YouTube as the only active data source
- Plugin-style collector contract for future platforms
- Async collection flow
- Creator aggregation at the YouTube channel level

P0 excludes:

- TikTok, Instagram, Google collectors
- Cross-platform creator identity merge
- ClickHouse
- Dedicated analysis microservice
- Billing, auth, export, alerts

---

## 3. Recommended Architecture

The recommended architecture is:

- `frontend`: Vue 3 + TypeScript SPA
- `apps/api`: Rust Actix Web API as the only backend entry point
- `apps/collector`: Python worker for collection and normalization
- `postgres`: primary persistence
- `redis`: task queue and lightweight cache/status
- `packages/schemas`: shared API and task schemas

This is a unified monorepo architecture, but not a unified runtime architecture. Code organization and data contracts are shared; processes remain independently deployable.

---

## 4. Monorepo Structure

```text
product-radar/
├── frontend/
├── apps/
│   ├── api/
│   └── collector/
├── packages/
│   ├── schemas/
│   └── docs/
├── infra/
│   ├── docker/
│   ├── compose/
│   └── sql/
├── docs/
│   └── plans/
└── README.md
```

Responsibilities:

- `frontend`: page rendering and state orchestration
- `apps/api`: keyword lifecycle, task dispatch, overview aggregation, timeline queries
- `apps/collector`: Redis task consumption, plugin loading, API calls, normalization, persistence
- `packages/schemas`: OpenAPI fragments, JSON Schemas, canonical collector payload contracts
- `infra`: local/dev deployment and SQL bootstrap assets

---

## 5. System Boundaries

### Frontend

P0 frontend is page-oriented and should only consume aggregated APIs. It should not reconstruct domain models from low-level resource endpoints.

Initial pages:

- Search page
- Keyword detail `Overview`

### API Service

The API owns:

- keyword create/find
- search history
- task enqueue
- task status query
- overview aggregation
- timeline query
- top creators query
- latest content query

The API does not call YouTube directly.

### Collector Service

The collector owns:

- Redis task consumption
- plugin registry
- plugin execution
- raw response normalization
- database upsert
- daily stat recomputation

This service is where future platforms are added.

---

## 6. Plugin Model

P0 only ships with a YouTube plugin, but collector internals must be plugin-oriented from day one.

Collector pipeline:

```text
task consumer
-> plugin registry
-> platform plugin
-> normalizer
-> persistence
```

Minimal plugin contract:

- `search(keyword, time_range) -> raw items`
- `normalize(raw items) -> canonical entities`
- `capabilities()`
- `rate_limit_policy()`

The API must only depend on canonical entities, never on YouTube-specific field names.

---

## 7. Canonical Data Model

P0 canonical entities:

### Keyword

- keyword text
- status
- created time
- last collected time

### ContentItem

- platform
- platform content id
- keyword id
- creator id
- title
- description
- url
- thumbnail url
- published at
- view count
- like count
- comment count
- engagement score
- raw payload

### Creator

P0 creator means a platform-local creator. On YouTube, this maps to a channel.

- platform
- platform creator id
- display name
- handle
- avatar url
- subscriber count
- video count
- creator score
- raw payload

### KeywordDailyStat

- date
- keyword id
- platform
- new content count
- total views
- total likes
- active creator count

### CollectionTask

- task id
- keyword
- platforms
- trigger type
- requested at
- time range
- priority

---

## 8. API Design

P0 APIs are page-oriented.

### Search Page

- `POST /api/v1/keywords/search`
- `GET /api/v1/keywords/history`
- `GET /api/v1/keywords/:id/status`

Search is asynchronous:

`submit -> enqueue -> poll status -> navigate when data exists`

### Keyword Overview Page

- `GET /api/v1/keywords/:id/overview?range=30d`
- `GET /api/v1/keywords/:id/timeline?range=30d`
- `GET /api/v1/keywords/:id/creators/top?range=30d&limit=10`
- `GET /api/v1/keywords/:id/contents/latest?range=30d&limit=20`

Response shape should be component-ready. The frontend should not need extra calls to enrich cards with creator summary data.

---

## 9. Storage Design

P0 uses:

- PostgreSQL for persistence
- Redis for queue/status/cache

P0 does not use ClickHouse.

Core tables:

- `keywords`
- `collection_tasks`
- `creators`
- `content_items`
- `keyword_daily_stats`

Key rules:

- `creators` upsert by `(platform, platform_creator_id)`
- `content_items` upsert by `(platform, platform_content_id)`
- `keyword_daily_stats` recomputed per keyword and day window after collection

Suggested indexes:

- unique `keywords(keyword)`
- unique `creators(platform, platform_creator_id)`
- unique `content_items(platform, platform_content_id)`
- index `content_items(keyword_id, published_at desc)`
- index `keyword_daily_stats(keyword_id, date)`
- index `collection_tasks(keyword_id, requested_at desc)`

---

## 10. Task Flow

```text
Frontend submits keyword
-> API creates/finds keyword
-> API inserts collection task
-> API enqueues Redis message
-> Collector loads youtube plugin
-> Plugin fetches raw data
-> Normalizer maps canonical entities
-> Persistence upserts creators/content
-> Collector recomputes daily stats
-> Task status updated
-> Frontend polls and renders overview
```

---

## 11. Error Handling

Task states:

- `pending`
- `running`
- `succeeded`
- `failed`
- `partial_success`

Failure domains:

- API validation failure
- Redis enqueue failure
- plugin execution failure
- rate limit or auth failure
- persistence failure
- aggregation recompute failure

Observability requirements:

- persist `error_message` on `collection_tasks`
- include `task_id` in logs
- expose latest task status through API

---

## 12. Domain Terminology

To avoid future ambiguity:

- `content_item` = one video
- `creator` = platform-local creator
- On YouTube, `creator` = channel
- Cross-platform creator identity is intentionally deferred

If future platforms are added, introduce a separate cross-platform layer such as `identity` or `persona`, rather than overloading `creator`.

---

## 13. Evolution Path

### P0

- YouTube-only
- search page
- keyword overview
- PostgreSQL + Redis

### P1

- add more collector plugins
- multi-platform timeline dimensions

### P2

- introduce cross-platform identity merge
- creator detail page

### P3

- split heavy analysis into dedicated module or service
- add ClickHouse only if aggregation volume justifies it

---

## 14. Final Recommendation

Adopt a monorepo with:

- Vue frontend
- one Rust API
- one Python collector runtime
- PostgreSQL
- Redis
- shared schemas

Design for plugin expansion at the collector layer, not for premature microservice decomposition.

This provides the fastest path to a usable P0 while preserving future expansion for additional platforms and deeper analytics.
