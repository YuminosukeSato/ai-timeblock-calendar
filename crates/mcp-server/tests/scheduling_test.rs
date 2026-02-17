use chrono::{Duration, NaiveDate, Utc};
use rusqlite::Connection;
use shared::schema::run_migrations;
use shared::services::ScheduleTask;

use mcp_server::tools::scheduling::{find_free_slots, suggest_schedule, FindFreeSlotsInput};
use mcp_server::tools::time_block::{create_time_block, CreateTimeBlockInput};

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    run_migrations(&conn).expect("migrations should succeed");
    conn
}

#[test]
fn find_free_slots_returns_full_business_hours_when_no_blocks() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 30,
            count: None,
        },
    )
    .expect("find should succeed");

    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].start.date_naive(), date);
    assert_eq!(slots[0].end.date_naive(), date);
}

#[test]
fn find_free_slots_returns_empty_when_all_day_occupied() {
    let conn = setup_db();
    let date = Utc::now().date_naive();
    let day_start = date.and_hms_opt(9, 0, 0).expect("valid datetime").and_utc();

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "busy".to_string(),
            start_time: day_start,
            end_time: day_start + Duration::hours(12),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("seed should succeed");

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 30,
            count: None,
        },
    )
    .expect("find should succeed");

    assert!(slots.is_empty());
}

#[test]
fn find_free_slots_returns_empty_when_duration_larger_than_remaining() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 24 * 60,
            count: None,
        },
    )
    .expect("find should succeed");

    assert!(slots.is_empty());
}

#[test]
fn find_free_slots_detects_exact_gap() {
    let conn = setup_db();
    let date = Utc::now().date_naive();
    let day_start = date.and_hms_opt(9, 0, 0).expect("valid datetime").and_utc();

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "morning".to_string(),
            start_time: day_start,
            end_time: day_start + Duration::hours(2),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("seed should succeed");

    create_time_block(
        &conn,
        CreateTimeBlockInput {
            title: "afternoon".to_string(),
            start_time: day_start + Duration::hours(3),
            end_time: day_start + Duration::hours(12),
            category_id: None,
            project_id: None,
            progress: 0,
        },
    )
    .expect("seed should succeed");

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 60,
            count: None,
        },
    )
    .expect("find should succeed");

    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].start, day_start + Duration::hours(2));
    assert_eq!(slots[0].end, day_start + Duration::hours(3));
}

#[test]
fn find_free_slots_returns_all_fragmented_slots() {
    let conn = setup_db();
    let date = Utc::now().date_naive();
    let day_start = date.and_hms_opt(9, 0, 0).expect("valid datetime").and_utc();

    for hour in [1, 3, 5] {
        create_time_block(
            &conn,
            CreateTimeBlockInput {
                title: format!("busy-{hour}"),
                start_time: day_start + Duration::hours(hour),
                end_time: day_start + Duration::hours(hour + 1),
                category_id: None,
                project_id: None,
                progress: 0,
            },
        )
        .expect("seed should succeed");
    }

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 30,
            count: None,
        },
    )
    .expect("find should succeed");

    assert!(slots.len() >= 3);
}

#[test]
fn find_free_slots_respects_count_limit() {
    let conn = setup_db();
    let date = Utc::now().date_naive();

    let slots = find_free_slots(
        &conn,
        FindFreeSlotsInput {
            date,
            duration_minutes: 30,
            count: Some(1),
        },
    )
    .expect("find should succeed");

    assert_eq!(slots.len(), 1);
}

#[test]
fn suggest_schedule_places_high_priority_first() {
    let conn = setup_db();
    let date = NaiveDate::from_ymd_opt(2026, 2, 17).expect("date should be valid");

    let result = suggest_schedule(
        &conn,
        vec![
            ScheduleTask {
                id: "low".to_string(),
                title: "low".to_string(),
                duration_minutes: 60,
                priority: 1,
                focus: false,
            },
            ScheduleTask {
                id: "high".to_string(),
                title: "high".to_string(),
                duration_minutes: 60,
                priority: 10,
                focus: true,
            },
        ],
        date,
    )
    .expect("suggest should succeed");

    assert_eq!(result.scheduled.first().expect("one task").task_id, "high");
}

#[test]
fn suggest_schedule_returns_warning_on_capacity_overflow() {
    let conn = setup_db();
    let date = NaiveDate::from_ymd_opt(2026, 2, 17).expect("date should be valid");

    let result = suggest_schedule(
        &conn,
        vec![
            ScheduleTask {
                id: "big".to_string(),
                title: "big".to_string(),
                duration_minutes: 13 * 60,
                priority: 10,
                focus: true,
            },
            ScheduleTask {
                id: "small".to_string(),
                title: "small".to_string(),
                duration_minutes: 60,
                priority: 1,
                focus: false,
            },
        ],
        date,
    )
    .expect("suggest should succeed");

    assert!(!result.warnings.is_empty());
}

#[test]
fn suggest_schedule_inserts_buffer_between_tasks() {
    let conn = setup_db();
    let date = NaiveDate::from_ymd_opt(2026, 2, 17).expect("date should be valid");

    let result = suggest_schedule(
        &conn,
        vec![
            ScheduleTask {
                id: "a".to_string(),
                title: "a".to_string(),
                duration_minutes: 60,
                priority: 10,
                focus: true,
            },
            ScheduleTask {
                id: "b".to_string(),
                title: "b".to_string(),
                duration_minutes: 60,
                priority: 9,
                focus: true,
            },
        ],
        date,
    )
    .expect("suggest should succeed");

    assert!(result.scheduled.len() >= 2);
    let gap = result.scheduled[1].start - result.scheduled[0].end;
    assert!(gap >= Duration::minutes(15));
}

#[test]
fn suggest_schedule_places_focus_task_in_morning() {
    let conn = setup_db();
    let date = NaiveDate::from_ymd_opt(2026, 2, 17).expect("date should be valid");

    let result = suggest_schedule(
        &conn,
        vec![ScheduleTask {
            id: "focus".to_string(),
            title: "focus".to_string(),
            duration_minutes: 60,
            priority: 10,
            focus: true,
        }],
        date,
    )
    .expect("suggest should succeed");

    assert!(result.scheduled[0].start.time().hour() < 12);
}

trait HourExt {
    fn hour(&self) -> u32;
}

impl HourExt for chrono::NaiveTime {
    fn hour(&self) -> u32 {
        chrono::Timelike::hour(self)
    }
}
