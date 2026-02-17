use anyhow::bail;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use shared::models::{Project, ProjectStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GanttTask {
    pub id: String,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub progress: i32,
    pub depends_on: Vec<String>,
}

pub fn create_project(conn: &Connection, input: CreateProjectInput) -> anyhow::Result<Project> {
    let project = Project::new(
        input.name,
        input.start_date,
        input.end_date,
        input.description,
        input.color,
    )
    .map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;

    conn.execute(
        "INSERT INTO projects (id, name, description, color, start_date, end_date, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            project.id,
            project.name,
            project.description,
            project.color,
            project.start_date.to_string(),
            project.end_date.map(|d| d.to_string()),
            project.status.as_db_str(),
            project.created_at.to_rfc3339(),
            project.updated_at.to_rfc3339(),
        ],
    )?;

    Ok(project)
}

pub fn list_projects(conn: &Connection, status: Option<&str>) -> anyhow::Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, color, start_date, end_date, status, created_at, updated_at
         FROM projects
         WHERE (?1 IS NULL OR status = ?1)
         ORDER BY created_at ASC",
    )?;

    let rows = stmt.query_map(params![status], |row| {
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "completed" => ProjectStatus::Completed,
            "archived" => ProjectStatus::Archived,
            _ => ProjectStatus::Active,
        };
        let start_date =
            NaiveDate::parse_from_str(&row.get::<_, String>(4)?, "%Y-%m-%d").map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let end_date = row
            .get::<_, Option<String>>(5)?
            .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    5,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let parse_dt = |s: String, col| {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        col,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })
        };

        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            color: row.get(3)?,
            start_date,
            end_date,
            status,
            created_at: parse_dt(row.get(7)?, 7)?,
            updated_at: parse_dt(row.get(8)?, 8)?,
        })
    })?;

    let projects: Result<Vec<_>, _> = rows.collect();
    Ok(projects?)
}

pub fn get_gantt_data(conn: &Connection, project_id: &str) -> anyhow::Result<Vec<GanttTask>> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(1) FROM projects WHERE id = ?1",
        params![project_id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        bail!("project not found: {project_id}");
    }

    let mut stmt = conn.prepare(
        "SELECT tb.id, tb.title, tb.start_time, tb.end_time, tb.progress,
                dep.depends_on_time_block_id
         FROM time_blocks tb
         LEFT JOIN time_block_dependencies dep ON dep.time_block_id = tb.id
         WHERE tb.project_id = ?1
         ORDER BY tb.start_time ASC",
    )?;

    let mut rows = stmt.query(params![project_id])?;
    let mut map: std::collections::HashMap<String, GanttTask> = std::collections::HashMap::new();

    while let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        let title: String = row.get(1)?;
        let start = DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)?.with_timezone(&Utc);
        let progress: i32 = row.get(4)?;
        let depends_on: Option<String> = row.get(5)?;

        let entry = map.entry(id.clone()).or_insert_with(|| GanttTask {
            id,
            title,
            start,
            end,
            progress,
            depends_on: Vec::new(),
        });

        if let Some(dep) = depends_on {
            entry.depends_on.push(dep);
        }
    }

    let mut tasks: Vec<GanttTask> = map.into_values().collect();
    tasks.sort_by_key(|task| task.start);
    Ok(tasks)
}
