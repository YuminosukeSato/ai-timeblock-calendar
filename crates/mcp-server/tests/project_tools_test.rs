use chrono::{Duration, Utc};
use rusqlite::Connection;
use shared::schema::run_migrations;

use mcp_server::tools::project::{
    create_project, get_gantt_data, list_projects, CreateProjectInput,
};
use mcp_server::tools::time_block::{create_time_block, CreateTimeBlockInput};

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    run_migrations(&conn).expect("migrations should succeed");
    conn
}

#[test]
fn create_project_success() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    let project = create_project(
        &conn,
        CreateProjectInput {
            name: "app".to_string(),
            start_date: date,
            end_date: None,
            description: None,
            color: None,
        },
    )
    .expect("project should be created");

    assert_eq!(project.name, "app");
}

#[test]
fn list_projects_respects_status_filter() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    create_project(
        &conn,
        CreateProjectInput {
            name: "app".to_string(),
            start_date: date,
            end_date: None,
            description: None,
            color: None,
        },
    )
    .expect("project should be created");

    let projects = list_projects(&conn, Some("active")).expect("list should succeed");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "app");
}

#[test]
fn get_gantt_data_returns_tasks_with_dependencies() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    let project = create_project(
        &conn,
        CreateProjectInput {
            name: "app".to_string(),
            start_date: date,
            end_date: None,
            description: None,
            color: None,
        },
    )
    .expect("project should be created");

    let base = Utc::now();
    let first = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "design".to_string(),
            start_time: base,
            end_time: base + Duration::hours(1),
            category_id: None,
            project_id: Some(project.id.clone()),
            progress: 100,
        },
    )
    .expect("time block should be created");

    let second = create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "impl".to_string(),
            start_time: base + Duration::hours(2),
            end_time: base + Duration::hours(4),
            category_id: None,
            project_id: Some(project.id.clone()),
            progress: 10,
        },
    )
    .expect("time block should be created");

    conn.execute(
        "INSERT INTO time_block_dependencies (time_block_id, depends_on_time_block_id) VALUES (?1, ?2)",
        [&second.id, &first.id],
    )
    .expect("dependency insert should succeed");

    let gantt = get_gantt_data(&conn, &project.id).expect("gantt should be fetched");

    assert_eq!(gantt.len(), 2);
    let impl_task = gantt
        .iter()
        .find(|t| t.id == second.id)
        .expect("impl task should exist");
    assert_eq!(impl_task.depends_on, vec![first.id]);
}

#[test]
fn get_gantt_data_returns_error_for_missing_project() {
    let conn = setup_db();

    let result = get_gantt_data(&conn, "missing-project");

    assert!(result.is_err());
}
