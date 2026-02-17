use chrono::NaiveDate;
use rusqlite::Connection;

use mcp_server::tools::project::{
    create_project, get_gantt_data, list_projects, CreateProjectInput, GanttTask,
};
use shared::models::Project;

pub fn create_project_command(
    conn: &Connection,
    name: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    description: Option<String>,
) -> anyhow::Result<Project> {
    create_project(
        conn,
        CreateProjectInput {
            name,
            start_date,
            end_date,
            description,
            color: None,
        },
    )
}

pub fn list_projects_command(
    conn: &Connection,
    status: Option<&str>,
) -> anyhow::Result<Vec<Project>> {
    list_projects(conn, status)
}

pub fn get_gantt_data_command(
    conn: &Connection,
    project_id: &str,
) -> anyhow::Result<Vec<GanttTask>> {
    get_gantt_data(conn, project_id)
}
