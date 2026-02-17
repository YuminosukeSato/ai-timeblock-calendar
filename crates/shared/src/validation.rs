use std::collections::{HashMap, HashSet};

use chrono::{DateTime, NaiveDate, Utc};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("{field} must not be empty")]
    Empty { field: &'static str },
    #[error("time range is invalid: end must be after start")]
    InvalidTimeRange,
    #[error("date range is invalid: end must be on or after start")]
    InvalidDateRange,
    #[error("progress must be between 0 and 100")]
    ProgressOutOfRange,
    #[error("circular dependency detected")]
    CircularDependency,
}

pub fn validate_non_empty(value: &str, field: &'static str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::Empty { field });
    }
    Ok(())
}

pub fn validate_progress(progress: i32) -> Result<(), ValidationError> {
    if !(0..=100).contains(&progress) {
        return Err(ValidationError::ProgressOutOfRange);
    }
    Ok(())
}

pub fn validate_time_range(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<(), ValidationError> {
    if end <= start {
        return Err(ValidationError::InvalidTimeRange);
    }
    Ok(())
}

pub fn validate_date_range(
    start: NaiveDate,
    end: Option<NaiveDate>,
) -> Result<(), ValidationError> {
    if let Some(end) = end {
        if end < start {
            return Err(ValidationError::InvalidDateRange);
        }
    }
    Ok(())
}

pub fn detect_circular_dependency(edges: &[(String, String)]) -> Result<(), ValidationError> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();

    for (from, to) in edges {
        if from == to {
            return Err(ValidationError::CircularDependency);
        }
        graph.entry(from).or_default().push(to);
        nodes.insert(from);
        nodes.insert(to);
    }

    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visiting.insert(node);
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if dfs(neighbor, graph, visiting, visited) {
                    return true;
                }
            }
        }
        visiting.remove(node);
        visited.insert(node);
        false
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();

    for node in nodes {
        if dfs(node, &graph, &mut visiting, &mut visited) {
            return Err(ValidationError::CircularDependency);
        }
    }

    Ok(())
}
