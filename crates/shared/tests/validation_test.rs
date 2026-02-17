use chrono::{Duration, Utc};
use shared::validation::{detect_circular_dependency, validate_time_range};

#[test]
fn validate_time_range_ok_for_valid_range() {
    let start = Utc::now();
    let end = start + Duration::hours(1);

    assert!(validate_time_range(start, end).is_ok());
}

#[test]
fn validate_time_range_err_for_reversed_range() {
    let start = Utc::now();
    let end = start - Duration::hours(1);

    assert!(validate_time_range(start, end).is_err());
}

#[test]
fn validate_time_range_err_for_equal_timestamps() {
    let start = Utc::now();

    assert!(validate_time_range(start, start).is_err());
}

#[test]
fn detect_circular_dependency_err_when_cycle_exists() {
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
        ("c".to_string(), "a".to_string()),
    ];

    assert!(detect_circular_dependency(&edges).is_err());
}

#[test]
fn detect_circular_dependency_ok_when_no_cycle() {
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
    ];

    assert!(detect_circular_dependency(&edges).is_ok());
}

#[test]
fn detect_circular_dependency_err_on_self_reference() {
    let edges = vec![("a".to_string(), "a".to_string())];

    assert!(detect_circular_dependency(&edges).is_err());
}
