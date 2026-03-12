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

## Local Infrastructure

The repo includes a full local stack under `infra/compose/docker-compose.yml`:

- `postgres`: primary database, bootstrapped from `infra/sql/001_init.sql`
- `redis`: task queue and stream transport
- `api`: Rust Actix Web service
- `collector`: Python Redis Stream worker
- `frontend`: Vue app served by nginx with `/api` proxied to the API container

### Prerequisites

- Docker Desktop with `docker compose`
- A YouTube Data API key if you want real collection results

### Start The Stack

From the repo root:

```bash
$env:PRODUCT_RADAR_YOUTUBE_API_KEY="your-youtube-api-key"
docker compose -f infra/compose/docker-compose.yml up --build
```

If you only want the services to start without real YouTube collection, omit `PRODUCT_RADAR_YOUTUBE_API_KEY`.

### Default Local Access

- Frontend: `http://localhost`
- API health: `http://localhost:8080/health`
- PostgreSQL: `localhost:5432`
- Redis: `localhost:6379`

### Default Local Credentials

- PostgreSQL database: `product_radar`
- PostgreSQL user: `postgres`
- PostgreSQL password: `postgres`

### Typical Local Flow

1. Open `http://localhost`
2. Search a keyword such as `ninja creami`
3. The API creates a collection task and publishes it to Redis
4. The collector consumes the task and writes results into PostgreSQL
5. The overview page polls task status and then loads aggregate data from the API

### Rebuild Or Reset

Stop the stack:

```bash
docker compose -f infra/compose/docker-compose.yml down
```

Remove containers and local volumes:

```bash
docker compose -f infra/compose/docker-compose.yml down -v
```
