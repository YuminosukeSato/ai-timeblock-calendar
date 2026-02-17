use chrono::{Duration, Utc};
use rusqlite::Connection;
use shared::schema::run_migrations;

use app_tauri::commands::time_block::{create_time_block_command, CreateTimeBlockCommandInput};

#[test]
fn create_command_success() {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    run_migrations(&conn).expect("migrations should run");
    let now = Utc::now();

    let block = create_time_block_command(
        &conn,
        CreateTimeBlockCommandInput {
            title: "task".to_string(),
            start_time: now,
            end_time: now + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("create command should succeed");

    assert_eq!(block.title, "task");
}

#[test]
fn create_command_propagates_db_error() {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    let now = Utc::now();

    let result = create_time_block_command(
        &conn,
        CreateTimeBlockCommandInput {
            title: "task".to_string(),
            start_time: now,
            end_time: now + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    );

    assert!(result.is_err());
}
