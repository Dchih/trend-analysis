CREATE TABLE keywords (
    id BIGSERIAL PRIMARY KEY,
    keyword VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_collected_at TIMESTAMPTZ
);

CREATE TABLE collection_tasks (
    id BIGSERIAL PRIMARY KEY,
    keyword_id BIGINT NOT NULL REFERENCES keywords(id) ON DELETE CASCADE,
    platform VARCHAR(32) NOT NULL,
    trigger_type VARCHAR(32) NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    error_message TEXT
);

CREATE TABLE creators (
    id BIGSERIAL PRIMARY KEY,
    platform VARCHAR(32) NOT NULL,
    platform_creator_id VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    handle VARCHAR(255),
    avatar_url TEXT,
    subscriber_count BIGINT NOT NULL DEFAULT 0,
    video_count BIGINT NOT NULL DEFAULT 0,
    creator_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    raw_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (platform, platform_creator_id)
);

CREATE TABLE content_items (
    id BIGSERIAL PRIMARY KEY,
    platform VARCHAR(32) NOT NULL,
    platform_content_id VARCHAR(255) NOT NULL,
    keyword_id BIGINT NOT NULL REFERENCES keywords(id) ON DELETE CASCADE,
    creator_id BIGINT NOT NULL REFERENCES creators(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    url TEXT NOT NULL,
    thumbnail_url TEXT,
    published_at TIMESTAMPTZ NOT NULL,
    view_count BIGINT NOT NULL DEFAULT 0,
    like_count BIGINT NOT NULL DEFAULT 0,
    comment_count BIGINT NOT NULL DEFAULT 0,
    engagement_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    raw_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    collected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (platform, platform_content_id)
);

CREATE TABLE keyword_daily_stats (
    id BIGSERIAL PRIMARY KEY,
    keyword_id BIGINT NOT NULL REFERENCES keywords(id) ON DELETE CASCADE,
    platform VARCHAR(32) NOT NULL,
    date DATE NOT NULL,
    new_content_count BIGINT NOT NULL DEFAULT 0,
    total_views BIGINT NOT NULL DEFAULT 0,
    total_likes BIGINT NOT NULL DEFAULT 0,
    active_creator_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (keyword_id, platform, date)
);

CREATE INDEX idx_collection_tasks_keyword_requested_at
    ON collection_tasks(keyword_id, requested_at DESC);

CREATE INDEX idx_content_items_keyword_published_at
    ON content_items(keyword_id, published_at DESC);

CREATE INDEX idx_keyword_daily_stats_keyword_date
    ON keyword_daily_stats(keyword_id, date);
