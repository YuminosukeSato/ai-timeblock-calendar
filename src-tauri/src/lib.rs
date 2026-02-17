pub mod commands;
pub mod setup;

use std::sync::Mutex;

use chrono::NaiveDate;
use rusqlite::Connection;
use serde::Deserialize;
use shared::models::{Category, Project, TimeBlock};
use shared::repository::TimeBlockPatch;

use commands::category::{create_category_command, list_categories_command};
use commands::project::{create_project_command, get_gantt_data_command, list_projects_command};
use commands::time_block::{
    create_time_block_command, delete_time_block_command, list_time_blocks_command,
    update_time_block_command, CreateTimeBlockCommandInput, ListTimeBlocksCommandInput,
};
use mcp_server::tools::project::GanttTask;

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[tauri::command]
fn create_time_block(
    state: tauri::State<'_, AppState>,
    input: CreateTimeBlockCommandInput,
) -> Result<TimeBlock, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    create_time_block_command(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_time_blocks(
    state: tauri::State<'_, AppState>,
    input: ListTimeBlocksCommandInput,
) -> Result<Vec<TimeBlock>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    list_time_blocks_command(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_time_block(
    state: tauri::State<'_, AppState>,
    id: String,
    patch: TimeBlockPatch,
) -> Result<TimeBlock, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    update_time_block_command(&conn, &id, patch).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_time_block(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    delete_time_block_command(&conn, &id).map_err(|e| e.to_string())
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
}

#[tauri::command]
fn create_project(
    state: tauri::State<'_, AppState>,
    input: CreateProjectInput,
) -> Result<Project, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    create_project_command(
        &conn,
        input.name,
        input.start_date,
        input.end_date,
        input.description,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_projects(
    state: tauri::State<'_, AppState>,
    status: Option<String>,
) -> Result<Vec<Project>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    list_projects_command(&conn, status.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_gantt_data(
    state: tauri::State<'_, AppState>,
    project_id: String,
) -> Result<Vec<GanttTask>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    get_gantt_data_command(&conn, &project_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_category(
    state: tauri::State<'_, AppState>,
    name: String,
    color: Option<String>,
) -> Result<Category, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    create_category_command(&conn, name, color).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_categories(state: tauri::State<'_, AppState>) -> Result<Vec<Category>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    list_categories_command(&conn).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = shared::schema::open_default_db().expect("Failed to open database");
    let state = AppState {
        db: Mutex::new(conn),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_time_block,
            list_time_blocks,
            update_time_block,
            delete_time_block,
            create_project,
            list_projects,
            get_gantt_data,
            create_category,
            list_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
