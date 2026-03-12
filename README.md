# ProductRadar

YouTube-first product and creator intelligence platform with a plugin-based collector architecture.

## Monorepo Layout

- `frontend/`: Vue 3 SPA for the search page and keyword overview page
- `apps/api/`: Rust Actix Web service for keyword workflows, task dispatch, and aggregate APIs
- `apps/collector/`: Python worker for plugin execution, normalization, and persistence
- `packages/schemas/`: Shared JSON Schemas for task payloads and API responses
- `infra/docker/`: Dockerfiles for local and deployment builds
- `infra/compose/`: Docker Compose assets for local infrastructure
- `infra/sql/`: SQL bootstrap and migration files
- `docs/plans/`: Architecture notes and implementation plans

## Responsibilities

- Frontend consumes page-oriented aggregate APIs.
- API enqueues collection work and serves component-ready overview data.
- Collector runs the YouTube plugin and writes canonical data into storage.
- Shared schemas keep the frontend, API, and collector aligned.
