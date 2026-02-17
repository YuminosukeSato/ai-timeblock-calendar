use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use rusqlite::Connection;

const INITIAL_MIGRATION: &str = include_str!("../../../migrations/001_initial.sql");

pub fn default_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("ai-timeblock-calendar")
        .join("app.db")
}

pub fn open_default_db() -> anyhow::Result<Connection> {
    open_db(default_db_path())
}

pub fn open_db<P: AsRef<Path>>(path: P) -> anyhow::Result<Connection> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create db directory: {}", parent.display()))?;
    }

    let conn = Connection::open(path)
        .with_context(|| format!("failed to open sqlite db: {}", path.display()))?;

    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.busy_timeout(std::time::Duration::from_secs(5))?;

    run_migrations(&conn)?;
    Ok(conn)
}

pub fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(INITIAL_MIGRATION)?;
    Ok(())
}
