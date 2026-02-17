use chrono::{Duration, Utc};
use shared::models::{Category, Project, TimeBlock};

#[test]
fn time_block_new_errors_when_start_not_before_end() {
    let now = Utc::now();

    let result = TimeBlock::new("focus".to_string(), None, now, now, None, None, 10);

    assert!(result.is_err());
}

#[test]
fn time_block_new_errors_when_title_is_empty() {
    let now = Utc::now();

    let result = TimeBlock::new(
        "   ".to_string(),
        None,
        now,
        now + Duration::hours(1),
        None,
        None,
        10,
    );

    assert!(result.is_err());
}

#[test]
fn time_block_new_errors_when_progress_out_of_range() {
    let now = Utc::now();

    let result = TimeBlock::new(
        "focus".to_string(),
        None,
        now,
        now + Duration::hours(1),
        None,
        None,
        101,
    );

    assert!(result.is_err());
}

#[test]
fn project_new_errors_when_start_after_end() {
    let start = Utc::now().date_naive();
    let end = start - Duration::days(1);

    let result = Project::new("app".to_string(), start, Some(end), None, None);

    assert!(result.is_err());
}

#[test]
fn category_new_success() {
    let result = Category::new("work".to_string(), None);

    assert!(result.is_ok());
    let category = result.expect("category should be created");
    assert_eq!(category.name, "work");
    assert_eq!(category.color, "#3B82F6");
}
