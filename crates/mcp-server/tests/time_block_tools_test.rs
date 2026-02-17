use chrono::{Duration, Utc};
use rusqlite::Connection;
use shared::repository::TimeBlockPatch;
use shared::schema::run_migrations;

use mcp_server::tools::project::{create_project, CreateProjectInput};
use mcp_server::tools::time_block::{
    create_time_block, delete_time_block, list_time_blocks, update_time_block,
    CreateTimeBlockInput, ListTimeBlocksInput,
};

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    run_migrations(&conn).expect("migrations should succeed");
    conn
}

#[test]
fn create_time_block_returns_created_id() {
    let conn = setup_db();
    let now = Utc::now();

    let block = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "design".to_string(),
            start_time: now,
            end_time: now + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("create should succeed");

    assert!(!block.id.is_empty());
}

#[test]
fn create_time_block_returns_error_when_start_not_before_end() {
    let conn = setup_db();
    let now = Utc::now();

    let result = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "design".to_string(),
            start_time: now,
            end_time: now,
            category_id: None,
            project_id: None,
            progress: 0,
        },
    );

    assert!(result.is_err());
}

#[test]
fn create_time_block_returns_error_when_title_empty() {
    let conn = setup_db();
    let now = Utc::now();

    let result = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "   ".to_string(),
            start_time: now,
            end_time: now + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    );

    assert!(result.is_err());
}

#[test]
fn list_time_blocks_respects_period_filter() {
    let conn = setup_db();
    let base = Utc::now();

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "in-range".to_string(),
            start_time: base + Duration::hours(1),
            end_time: base + Duration::hours(2),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("seed should succeed");

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "out-range".to_string(),
            start_time: base + Duration::days(5),
            end_time: base + Duration::days(5) + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("seed should succeed");

    let list = list_time_blocks(
        &conn,
        ListTimeBlocksInput {
            start: base,
            end: base + Duration::days(1),
            category_id: None,
            project_id: None,
        },
    )
    .expect("list should succeed");

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].title, "in-range");
}

#[test]
fn list_time_blocks_respects_project_filter() {
    let conn = setup_db();
    let base = Utc::now();

    let date = base.date_naive();
    let p1 = create_project(
        &conn,
        CreateProjectInput {
            name: "p1".to_string(),
            start_date: date,
            end_date: None,
            description: None,
            color: None,
        },
    )
    .expect("project p1 should be created");

    let p2 = create_project(
        &conn,
        CreateProjectInput {
            name: "p2".to_string(),
            start_date: date,
            end_date: None,
            description: None,
            color: None,
        },
    )
    .expect("project p2 should be created");

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "target-project".to_string(),
            start_time: base + Duration::hours(1),
            end_time: base + Duration::hours(2),
            category_id: None,
            project_id: Some(p1.id.clone()),
            progress: 0,
        },
    )
    .expect("seed should succeed");

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "other-project".to_string(),
            start_time: base + Duration::hours(3),
            end_time: base + Duration::hours(4),
            category_id: None,
            project_id: Some(p2.id.clone()),
            progress: 0,
        },
    )
    .expect("seed should succeed");

    let list = list_time_blocks(
        &conn,
        ListTimeBlocksInput {
            start: base,
            end: base + Duration::days(1),
            category_id: None,
            project_id: Some(p1.id),
        },
    )
    .expect("list should succeed");

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].title, "target-project");
}

#[test]
fn list_time_blocks_returns_empty_for_empty_period() {
    let conn = setup_db();
    let base = Utc::now();

    let list = list_time_blocks(
        &conn,
        ListTimeBlocksInput {
            start: base,
            end: base + Duration::hours(1),
            category_id: None,
            project_id: None,
        },
    )
    .expect("list should succeed");

    assert!(list.is_empty());
}

#[test]
fn update_time_block_returns_error_for_missing_id() {
    let conn = setup_db();

    let result = update_time_block(
        &conn,
        "missing",
        TimeBlockPatch {
            title: Some("new".to_string()),
            ..TimeBlockPatch::default()
        },
    );

    assert!(result.is_err());
}

#[test]
fn update_time_block_supports_partial_update() {
    let conn = setup_db();
    let base = Utc::now();

    let created = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "old".to_string(),
            start_time: base,
            end_time: base + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("create should succeed");

    let updated = update_time_block(
        &conn,
        &created.id,
        TimeBlockPatch {
            title: Some("new".to_string()),
            ..TimeBlockPatch::default()
        },
    )
    .expect("update should succeed");

    assert_eq!(updated.title, "new");
    assert_eq!(updated.start_time, created.start_time);
}

#[test]
fn delete_time_block_removes_record() {
    let conn = setup_db();
    let base = Utc::now();

    let created = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "delete-target".to_string(),
            start_time: base,
            end_time: base + Duration::hours(1),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("create should succeed");

    delete_time_block(&conn, &created.id).expect("delete should succeed");

    let listed = list_time_blocks(
        &conn,
        ListTimeBlocksInput {
            start: base - Duration::hours(1),
            end: base + Duration::hours(2),
            category_id: None,
            project_id: None,
        },
    )
    .expect("list should succeed");

    assert!(listed.is_empty());
}

#[test]
fn delete_time_block_returns_error_for_missing_id() {
    let conn = setup_db();

    let result = delete_time_block(&conn, "missing");

    assert!(result.is_err());
}
