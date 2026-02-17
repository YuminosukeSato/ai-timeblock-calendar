use std::fs;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use shared::schema::{open_db, run_migrations};

#[test]
fn run_migrations_creates_tables_on_empty_db() {
    let conn = Connection::open_in_memory().expect("in-memory db should open");

    run_migrations(&conn).expect("migrations should succeed");

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='time_blocks'")
        .expect("query should prepare");
    let table: String = stmt
        .query_row([], |row| row.get(0))
        .expect("table should exist");

    assert_eq!(table, "time_blocks");
}

#[test]
fn run_migrations_is_idempotent() {
    let conn = Connection::open_in_memory().expect("in-memory db should open");

    run_migrations(&conn).expect("first migration should succeed");
    run_migrations(&conn).expect("second migration should also succeed");
}

#[test]
fn open_db_enables_wal_mode() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("ai-timeblock-calendar-test-{unique}"));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let db_path = dir.join("app.db");
    fs::File::create(&db_path).expect("db file should be created");

    let conn = open_db(&db_path).expect("db should open");
    let mode: String = conn
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .expect("journal mode should be available");

    assert_eq!(mode.to_lowercase(), "wal");

    drop(conn);

    for attempt in 0..5 {
        match fs::remove_dir_all(&dir) {
            Ok(_) => return,
            Err(err) if attempt < 4 => {
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(err) => panic!("temp dir should be removed: {err}"),
        }
    }
}
