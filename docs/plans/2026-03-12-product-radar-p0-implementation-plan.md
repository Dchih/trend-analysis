# ProductRadar P0 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the P0 YouTube-first ProductRadar architecture with a Vue frontend, a Rust API, a Python plugin-based collector, PostgreSQL persistence, and Redis-backed async collection.

**Architecture:** Use a monorepo with one frontend app, one Rust API, and one Python collector. Keep the system page-oriented and schema-driven: the API serves component-ready aggregates, while the collector owns plugin execution and canonical normalization.

**Tech Stack:** Vue 3, TypeScript, Vite, Rust, Actix Web, Python, Redis, PostgreSQL, Docker Compose, JSON Schema

---

### Task 1: Restructure the Repository

**Files:**
- Create: `frontend/`
- Create: `apps/api/`
- Create: `apps/collector/`
- Create: `packages/schemas/`
- Create: `infra/docker/`
- Create: `infra/compose/`
- Create: `infra/sql/`
- Modify: `README.md`

**Step 1: Write the failing test**

Document a structure check in `README.md` that lists the expected top-level directories. The test here is a manual repo-shape assertion because no automation exists yet.

**Step 2: Run test to verify it fails**

Run: `rg "apps/api|apps/collector|frontend|packages/schemas" README.md -n`
Expected: FAIL to find the documented structure because `README.md` does not exist yet.

**Step 3: Write minimal implementation**

Create the top-level directories and add a minimal `README.md` that documents the monorepo layout and application responsibilities.

**Step 4: Run test to verify it passes**

Run: `rg "apps/api|apps/collector|frontend|packages/schemas" README.md -n`
Expected: PASS with matching lines in `README.md`.

**Step 5: Commit**

```bash
git add README.md frontend apps packages infra
git commit -m "chore: initialize monorepo structure"
```

### Task 2: Define Shared Schemas

**Files:**
- Create: `packages/schemas/tasks/collect-task.schema.json`
- Create: `packages/schemas/domain/content-item.schema.json`
- Create: `packages/schemas/domain/creator.schema.json`
- Create: `packages/schemas/api/keyword-overview.schema.json`
- Create: `packages/schemas/README.md`

**Step 1: Write the failing test**

Create a schema checklist in `packages/schemas/README.md` describing the required schema files and required fields for each.

**Step 2: Run test to verify it fails**

Run: `rg "\"platform\"|\"keyword_id\"|\"platform_content_id\"" packages/schemas -g "*.json"`
Expected: FAIL because the schema files do not exist yet.

**Step 3: Write minimal implementation**

Create JSON Schema files for:

- collect task message
- canonical content item
- canonical creator
- keyword overview API response

Include required fields and enums for `platform` and task status.

**Step 4: Run test to verify it passes**

Run: `rg "\"platform\"|\"keyword_id\"|\"platform_content_id\"" packages/schemas -g "*.json"`
Expected: PASS with matches from the new schema files.

**Step 5: Commit**

```bash
git add packages/schemas
git commit -m "feat: add shared schemas for tasks and api contracts"
```

### Task 3: Bootstrap the Rust API

**Files:**
- Create: `apps/api/Cargo.toml`
- Create: `apps/api/src/main.rs`
- Create: `apps/api/src/routes/mod.rs`
- Create: `apps/api/src/routes/keywords.rs`
- Create: `apps/api/src/routes/health.rs`
- Create: `apps/api/src/models/mod.rs`
- Create: `apps/api/src/app_state.rs`
- Modify: `Cargo.toml`

**Step 1: Write the failing test**

Add a minimal health and routing test in `apps/api/src/main.rs` or `apps/api/tests/health.rs` that expects:

```rust
#[actix_web::test]
async fn health_returns_ok() {
    // request GET /health
    // assert 200
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p api`
Expected: FAIL because the crate and routes do not exist yet.

**Step 3: Write minimal implementation**

Create the Rust app with:

- `/health`
- `POST /api/v1/keywords/search`
- `GET /api/v1/keywords/history`
- `GET /api/v1/keywords/{id}/status`

Stub responses are acceptable at this step as long as route shapes compile and tests pass.

**Step 4: Run test to verify it passes**

Run: `cargo test -p api`
Expected: PASS for route boot and health checks.

**Step 5: Commit**

```bash
git add Cargo.toml apps/api
git commit -m "feat: bootstrap api service routes"
```

### Task 4: Add API Aggregate Endpoints

**Files:**
- Modify: `apps/api/src/routes/keywords.rs`
- Create: `apps/api/src/routes/overview.rs`
- Create: `apps/api/src/models/overview.rs`
- Create: `apps/api/tests/overview.rs`

**Step 1: Write the failing test**

Create tests for:

- `GET /api/v1/keywords/{id}/overview`
- `GET /api/v1/keywords/{id}/timeline`
- `GET /api/v1/keywords/{id}/creators/top`
- `GET /api/v1/keywords/{id}/contents/latest`

Assert response status and JSON keys required by shared schemas.

**Step 2: Run test to verify it fails**

Run: `cargo test -p api overview -- --nocapture`
Expected: FAIL because these routes and models do not exist.

**Step 3: Write minimal implementation**

Add page-oriented aggregate handlers returning schema-shaped placeholder payloads. Keep these handlers separate from database code for now.

**Step 4: Run test to verify it passes**

Run: `cargo test -p api overview -- --nocapture`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/api
git commit -m "feat: add overview aggregate api endpoints"
```

### Task 5: Add PostgreSQL Schema and Migrations

**Files:**
- Create: `infra/sql/001_init.sql`
- Create: `infra/sql/README.md`
- Create: `apps/api/src/models/keyword.rs`
- Create: `apps/api/src/models/task.rs`
- Create: `apps/api/src/models/content.rs`
- Create: `apps/api/src/models/creator.rs`

**Step 1: Write the failing test**

Add a SQL verification checklist to `infra/sql/README.md` covering required tables and indexes:

- `keywords`
- `collection_tasks`
- `creators`
- `content_items`
- `keyword_daily_stats`

**Step 2: Run test to verify it fails**

Run: `rg "CREATE TABLE (keywords|collection_tasks|creators|content_items|keyword_daily_stats)" infra/sql/001_init.sql`
Expected: FAIL because the migration file does not exist yet.

**Step 3: Write minimal implementation**

Create `001_init.sql` with the five core tables and key unique/index definitions from the design.

**Step 4: Run test to verify it passes**

Run: `rg "CREATE TABLE (keywords|collection_tasks|creators|content_items|keyword_daily_stats)" infra/sql/001_init.sql`
Expected: PASS.

**Step 5: Commit**

```bash
git add infra/sql apps/api/src/models
git commit -m "feat: add initial postgres schema"
```

### Task 6: Wire Redis Task Dispatch from the API

**Files:**
- Modify: `apps/api/Cargo.toml`
- Modify: `apps/api/src/routes/keywords.rs`
- Create: `apps/api/src/services/task_queue.rs`
- Create: `apps/api/tests/search_task_dispatch.rs`

**Step 1: Write the failing test**

Add a route-level test for `POST /api/v1/keywords/search` that asserts:

- a keyword is created or reused
- a collection task record is created
- a queue publish function is invoked

Use a mock task queue trait instead of Redis first.

**Step 2: Run test to verify it fails**

Run: `cargo test -p api search_task_dispatch -- --nocapture`
Expected: FAIL because queue abstraction and task creation flow do not exist.

**Step 3: Write minimal implementation**

Introduce a queue interface and implement the search route so it:

- validates keyword input
- creates or fetches the keyword
- creates a pending task
- publishes a task message for `youtube`

**Step 4: Run test to verify it passes**

Run: `cargo test -p api search_task_dispatch -- --nocapture`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/api
git commit -m "feat: enqueue collection tasks from search api"
```

### Task 7: Bootstrap the Python Collector

**Files:**
- Create: `apps/collector/pyproject.toml`
- Create: `apps/collector/src/__init__.py`
- Create: `apps/collector/src/worker.py`
- Create: `apps/collector/src/runtime.py`
- Create: `apps/collector/src/registry.py`
- Create: `apps/collector/tests/test_registry.py`

**Step 1: Write the failing test**

Create a test that asserts the collector runtime can load a plugin from the registry and reject an unknown platform.

**Step 2: Run test to verify it fails**

Run: `pytest apps/collector/tests/test_registry.py -v`
Expected: FAIL because the package and registry do not exist.

**Step 3: Write minimal implementation**

Create the collector package, plugin registry, and a worker skeleton that accepts queue messages and resolves a plugin by platform name.

**Step 4: Run test to verify it passes**

Run: `pytest apps/collector/tests/test_registry.py -v`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/collector
git commit -m "feat: bootstrap plugin-based collector runtime"
```

### Task 8: Implement the YouTube Plugin Contract

**Files:**
- Create: `apps/collector/src/plugins/__init__.py`
- Create: `apps/collector/src/plugins/base.py`
- Create: `apps/collector/src/plugins/youtube.py`
- Create: `apps/collector/tests/test_youtube_plugin.py`

**Step 1: Write the failing test**

Create tests that assert the YouTube plugin:

- declares `youtube` capability
- exposes a search method
- normalizes raw input into canonical creator and content entities

**Step 2: Run test to verify it fails**

Run: `pytest apps/collector/tests/test_youtube_plugin.py -v`
Expected: FAIL because the plugin files do not exist.

**Step 3: Write minimal implementation**

Add a base plugin interface and a YouTube plugin that normalizes fixture-like raw payloads into canonical dictionaries aligned with shared schemas.

**Step 4: Run test to verify it passes**

Run: `pytest apps/collector/tests/test_youtube_plugin.py -v`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/collector
git commit -m "feat: add youtube collector plugin"
```

### Task 9: Persist Canonical Entities and Daily Stats

**Files:**
- Create: `apps/collector/src/persistence.py`
- Create: `apps/collector/src/aggregates.py`
- Create: `apps/collector/tests/test_aggregates.py`
- Modify: `apps/collector/src/worker.py`

**Step 1: Write the failing test**

Add tests for:

- creator upsert by `(platform, platform_creator_id)`
- content upsert by `(platform, platform_content_id)`
- recompute of `keyword_daily_stats` from normalized content items

**Step 2: Run test to verify it fails**

Run: `pytest apps/collector/tests/test_aggregates.py -v`
Expected: FAIL because persistence and aggregate recompute are not implemented.

**Step 3: Write minimal implementation**

Implement persistence helpers and a daily stat recompute function for:

- `new_content_count`
- `total_views`
- `total_likes`
- `active_creator_count`

Update the worker to call persistence and aggregate recomputation after plugin normalization.

**Step 4: Run test to verify it passes**

Run: `pytest apps/collector/tests/test_aggregates.py -v`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/collector
git commit -m "feat: persist normalized entities and daily stats"
```

### Task 10: Connect API Aggregates to the Database

**Files:**
- Modify: `apps/api/src/routes/overview.rs`
- Create: `apps/api/src/repositories/overview_repo.rs`
- Create: `apps/api/src/repositories/mod.rs`
- Create: `apps/api/tests/overview_repo.rs`

**Step 1: Write the failing test**

Add repository and handler tests that assert aggregate endpoints return:

- overview cards from persisted rows
- timeline points from `keyword_daily_stats`
- ranked creators from joined creator/content data
- latest content with embedded creator summary

**Step 2: Run test to verify it fails**

Run: `cargo test -p api overview_repo -- --nocapture`
Expected: FAIL because repository-backed aggregate queries do not exist.

**Step 3: Write minimal implementation**

Implement SQL-backed aggregate queries and wire handlers to repository calls.

**Step 4: Run test to verify it passes**

Run: `cargo test -p api overview_repo -- --nocapture`
Expected: PASS.

**Step 5: Commit**

```bash
git add apps/api
git commit -m "feat: serve overview aggregates from postgres"
```

### Task 11: Bootstrap the Vue Frontend

**Files:**
- Create: `frontend/package.json`
- Create: `frontend/vite.config.ts`
- Create: `frontend/src/main.ts`
- Create: `frontend/src/App.vue`
- Create: `frontend/src/router/index.ts`
- Create: `frontend/src/pages/search/SearchPage.vue`
- Create: `frontend/src/pages/keyword/KeywordOverviewPage.vue`
- Create: `frontend/src/api/keywords.ts`

**Step 1: Write the failing test**

Create component tests or a minimal route smoke test that asserts:

- search page renders
- keyword overview route renders

**Step 2: Run test to verify it fails**

Run: `npm test -- --runInBand`
Expected: FAIL because the frontend app does not exist yet.

**Step 3: Write minimal implementation**

Create the Vue app, router, and two pages with placeholder data loading hooks for:

- search submit
- task status polling
- overview fetch

**Step 4: Run test to verify it passes**

Run: `npm test -- --runInBand`
Expected: PASS for route smoke tests.

**Step 5: Commit**

```bash
git add frontend
git commit -m "feat: bootstrap frontend pages for search and overview"
```

### Task 12: Wire End-to-End Search and Status Polling

**Files:**
- Modify: `frontend/src/pages/search/SearchPage.vue`
- Modify: `frontend/src/pages/keyword/KeywordOverviewPage.vue`
- Modify: `frontend/src/api/keywords.ts`
- Create: `frontend/src/stores/search.ts`
- Create: `frontend/tests/search-flow.spec.ts`

**Step 1: Write the failing test**

Create a frontend flow test that asserts:

- submitting a keyword calls search API
- status is polled
- navigation occurs when data becomes available

**Step 2: Run test to verify it fails**

Run: `npm test -- search-flow`
Expected: FAIL because the data flow is not connected.

**Step 3: Write minimal implementation**

Implement the search form flow and polling behavior. Keep state centralized enough to share the active keyword and polling state between pages.

**Step 4: Run test to verify it passes**

Run: `npm test -- search-flow`
Expected: PASS.

**Step 5: Commit**

```bash
git add frontend
git commit -m "feat: add end-to-end keyword search flow"
```

### Task 13: Add Local Infrastructure

**Files:**
- Create: `infra/docker/Dockerfile.api`
- Create: `infra/docker/Dockerfile.collector`
- Create: `infra/docker/Dockerfile.frontend`
- Create: `infra/compose/docker-compose.yml`
- Modify: `README.md`

**Step 1: Write the failing test**

Add local run instructions in `README.md` for:

- postgres
- redis
- api
- collector
- frontend

**Step 2: Run test to verify it fails**

Run: `rg "docker-compose|postgres|redis|api|collector|frontend" README.md -n`
Expected: FAIL because the runbook is incomplete.

**Step 3: Write minimal implementation**

Create Dockerfiles and a compose file that starts:

- postgres
- redis
- api
- collector
- frontend

Document startup and migration commands in `README.md`.

**Step 4: Run test to verify it passes**

Run: `rg "docker-compose|postgres|redis|api|collector|frontend" README.md -n`
Expected: PASS.

**Step 5: Commit**

```bash
git add infra README.md
git commit -m "chore: add local infrastructure setup"
```

### Task 14: Add End-to-End Verification and Docs

**Files:**
- Modify: `README.md`
- Create: `docs/plans/verification-checklist.md`
- Create: `apps/api/tests/e2e_contracts.rs`
- Create: `apps/collector/tests/test_contract_alignment.py`

**Step 1: Write the failing test**

Create a verification checklist that requires:

- API responses align with schema
- collector normalized output aligns with schema
- keyword search to overview works locally

**Step 2: Run test to verify it fails**

Run: `cargo test -p api e2e_contracts -- --nocapture`
Run: `pytest apps/collector/tests/test_contract_alignment.py -v`
Expected: FAIL because cross-layer verification is not implemented.

**Step 3: Write minimal implementation**

Add contract-alignment tests and a short operator checklist for local validation.

**Step 4: Run test to verify it passes**

Run: `cargo test -p api e2e_contracts -- --nocapture`
Run: `pytest apps/collector/tests/test_contract_alignment.py -v`
Expected: PASS.

**Step 5: Commit**

```bash
git add README.md docs/plans apps/api apps/collector
git commit -m "test: add cross-layer verification coverage"
```
