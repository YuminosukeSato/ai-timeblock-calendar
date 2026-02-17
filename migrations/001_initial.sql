PRAGMA journal_mode=WAL;
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS categories (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    color       TEXT NOT NULL DEFAULT '#3B82F6',
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS projects (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    color       TEXT NOT NULL DEFAULT '#10B981',
    start_date  TEXT NOT NULL,
    end_date    TEXT,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active','completed','archived')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS calendar_accounts (
    id                      TEXT PRIMARY KEY,
    email                   TEXT NOT NULL UNIQUE,
    access_token_encrypted  TEXT,
    refresh_token_encrypted TEXT NOT NULL,
    token_expires_at        TEXT,
    is_active               INTEGER NOT NULL DEFAULT 1,
    created_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS time_blocks (
    id                TEXT PRIMARY KEY,
    title             TEXT NOT NULL,
    description       TEXT,
    start_time        TEXT NOT NULL,
    end_time          TEXT NOT NULL,
    category_id       TEXT REFERENCES categories(id) ON DELETE SET NULL,
    project_id        TEXT REFERENCES projects(id) ON DELETE SET NULL,
    google_event_id   TEXT,
    calendar_id       TEXT,
    account_id        TEXT REFERENCES calendar_accounts(id) ON DELETE SET NULL,
    color             TEXT,
    is_all_day        INTEGER NOT NULL DEFAULT 0,
    status            TEXT NOT NULL DEFAULT 'confirmed'
                      CHECK(status IN ('confirmed','tentative','cancelled')),
    sync_status       TEXT NOT NULL DEFAULT 'local'
                      CHECK(sync_status IN ('local','synced','pending_push','pending_delete','conflict')),
    progress          INTEGER NOT NULL DEFAULT 0
                      CHECK(progress >= 0 AND progress <= 100),
    local_updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    google_updated_at TEXT,
    etag              TEXT,
    created_at        TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at        TEXT NOT NULL DEFAULT (datetime('now')),
    CHECK(end_time > start_time)
);

CREATE TABLE IF NOT EXISTS time_block_dependencies (
    time_block_id TEXT NOT NULL REFERENCES time_blocks(id) ON DELETE CASCADE,
    depends_on_time_block_id TEXT NOT NULL REFERENCES time_blocks(id) ON DELETE CASCADE,
    PRIMARY KEY (time_block_id, depends_on_time_block_id),
    CHECK (time_block_id != depends_on_time_block_id)
);

CREATE TABLE IF NOT EXISTS sync_state (
    calendar_id    TEXT PRIMARY KEY,
    account_id     TEXT NOT NULL REFERENCES calendar_accounts(id) ON DELETE CASCADE,
    sync_token     TEXT,
    last_synced_at TEXT,
    full_sync_done INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_time_blocks_start ON time_blocks(start_time);
CREATE INDEX IF NOT EXISTS idx_time_blocks_sync ON time_blocks(sync_status);
CREATE INDEX IF NOT EXISTS idx_time_blocks_google ON time_blocks(google_event_id);
CREATE INDEX IF NOT EXISTS idx_time_blocks_project ON time_blocks(project_id);
CREATE INDEX IF NOT EXISTS idx_time_block_deps_dep ON time_block_dependencies(depends_on_time_block_id);
