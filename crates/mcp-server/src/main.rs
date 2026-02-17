use std::sync::{Arc, Mutex};

use chrono::NaiveDate;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::ServiceExt,
    tool, tool_router,
    ErrorData as McpError,
};
use rusqlite::Connection;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use mcp_server::tools::{
    project::{self, CreateProjectInput},
    scheduling::{self, FindFreeSlotsInput},
    time_block::{self, CreateTimeBlockInput, ListTimeBlocksInput},
};
use shared::models::TimeBlock;
use shared::repository::TimeBlockPatch;
use shared::services::ScheduleTask;

// --- Parameter types with JsonSchema ---

#[derive(Debug, Deserialize, JsonSchema)]
struct CreateTimeBlockParams {
    /// タイムブロックのタイトル
    title: String,
    /// 開始時刻 (RFC3339形式, e.g. "2026-02-17T09:00:00Z")
    start_time: String,
    /// 終了時刻 (RFC3339形式, e.g. "2026-02-17T10:00:00Z")
    end_time: String,
    /// カテゴリID
    category_id: Option<String>,
    /// プロジェクトID
    project_id: Option<String>,
    /// 進捗率 (0-100, デフォルト: 0)
    progress: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListTimeBlocksParams {
    /// 開始日時 (RFC3339)
    start: String,
    /// 終了日時 (RFC3339)
    end: String,
    /// カテゴリIDでフィルタ
    category_id: Option<String>,
    /// プロジェクトIDでフィルタ
    project_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UpdateTimeBlockParams {
    /// 更新対象のタイムブロックID
    id: String,
    /// 新しいタイトル
    title: Option<String>,
    /// 新しい開始時刻 (RFC3339)
    start_time: Option<String>,
    /// 新しい終了時刻 (RFC3339)
    end_time: Option<String>,
    /// 新しい進捗率 (0-100)
    progress: Option<i32>,
    /// 新しいプロジェクトID (nullでクリア)
    project_id: Option<Option<String>>,
    /// 新しいカテゴリID (nullでクリア)
    category_id: Option<Option<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeleteTimeBlockParams {
    /// 削除対象のタイムブロックID
    id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindFreeSlotsParams {
    /// 検索対象の日付 (YYYY-MM-DD)
    date: String,
    /// 必要な空き時間（分）
    duration_minutes: i64,
    /// 返却する最大スロット数
    count: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SuggestScheduleParams {
    /// スケジュールするタスク一覧
    tasks: Vec<SuggestTask>,
    /// 対象日 (YYYY-MM-DD)
    target_date: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SuggestTask {
    /// タスクID
    id: String,
    /// タスク名
    title: String,
    /// 所要時間（分）
    duration_minutes: i64,
    /// 優先度 (数値が大きいほど高い)
    priority: i32,
    /// 集中力が必要なタスクか (trueなら午前に配置)
    focus: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetDaySummaryParams {
    /// サマリー対象の日付 (YYYY-MM-DD)
    date: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CreateProjectParams {
    /// プロジェクト名
    name: String,
    /// 開始日 (YYYY-MM-DD)
    start_date: String,
    /// 終了日 (YYYY-MM-DD, optional)
    end_date: Option<String>,
    /// 説明
    description: Option<String>,
    /// カラーコード
    color: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListProjectsParams {
    /// ステータスでフィルタ ("active", "completed", "archived")
    status: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetGanttDataParams {
    /// プロジェクトID
    project_id: String,
}

// --- Server ---

#[derive(Clone)]
struct CalendarMcpServer {
    db: Arc<Mutex<Connection>>,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

fn parse_rfc3339(s: &str) -> Result<chrono::DateTime<chrono::Utc>, McpError> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| McpError::invalid_params(format!("invalid RFC3339 datetime: {e}"), None))
}

fn parse_date(s: &str) -> Result<NaiveDate, McpError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| McpError::invalid_params(format!("invalid date: {e}"), None))
}

fn to_text_result<T: Serialize>(val: &T) -> Result<CallToolResult, McpError> {
    let json = serde_json::to_string_pretty(val)
        .map_err(|e| McpError::internal_error(format!("serialization error: {e}"), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

fn db_err(e: anyhow::Error) -> McpError {
    McpError::internal_error(e.to_string(), None)
}

#[tool_router]
impl CalendarMcpServer {
    fn new(conn: Connection) -> Self {
        Self {
            db: Arc::new(Mutex::new(conn)),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "タイムブロックを作成する")]
    async fn create_time_block(
        &self,
        Parameters(p): Parameters<CreateTimeBlockParams>,
    ) -> Result<CallToolResult, McpError> {
        let start = parse_rfc3339(&p.start_time)?;
        let end = parse_rfc3339(&p.end_time)?;
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let block = time_block::create_time_block(
            &conn,
            CreateTimeBlockInput {
                title: p.title,
                start_time: start,
                end_time: end,
                category_id: p.category_id,
                project_id: p.project_id,
                progress: p.progress.unwrap_or(0),
            },
        )
        .map_err(db_err)?;
        to_text_result(&block)
    }

    #[tool(description = "指定期間のタイムブロック一覧を取得する")]
    async fn list_time_blocks(
        &self,
        Parameters(p): Parameters<ListTimeBlocksParams>,
    ) -> Result<CallToolResult, McpError> {
        let start = parse_rfc3339(&p.start)?;
        let end = parse_rfc3339(&p.end)?;
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let blocks = time_block::list_time_blocks(
            &conn,
            ListTimeBlocksInput {
                start,
                end,
                category_id: p.category_id,
                project_id: p.project_id,
            },
        )
        .map_err(db_err)?;
        to_text_result(&blocks)
    }

    #[tool(description = "タイムブロックを更新する")]
    async fn update_time_block(
        &self,
        Parameters(p): Parameters<UpdateTimeBlockParams>,
    ) -> Result<CallToolResult, McpError> {
        let start = p.start_time.as_deref().map(parse_rfc3339).transpose()?;
        let end = p.end_time.as_deref().map(parse_rfc3339).transpose()?;
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let patch = TimeBlockPatch {
            title: p.title,
            description: None,
            start_time: start,
            end_time: end,
            progress: p.progress,
            project_id: p.project_id,
            category_id: p.category_id,
        };
        let updated = time_block::update_time_block(&conn, &p.id, patch).map_err(db_err)?;
        to_text_result(&updated)
    }

    #[tool(description = "タイムブロックを削除する")]
    async fn delete_time_block(
        &self,
        Parameters(p): Parameters<DeleteTimeBlockParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        time_block::delete_time_block(&conn, &p.id).map_err(db_err)?;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "deleted: {}",
            p.id
        ))]))
    }

    #[tool(description = "指定日の空き時間スロットを検索する")]
    async fn find_free_slots(
        &self,
        Parameters(p): Parameters<FindFreeSlotsParams>,
    ) -> Result<CallToolResult, McpError> {
        let date = parse_date(&p.date)?;
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let slots = scheduling::find_free_slots(
            &conn,
            FindFreeSlotsInput {
                date,
                duration_minutes: p.duration_minutes,
                count: p.count,
            },
        )
        .map_err(db_err)?;
        to_text_result(&slots)
    }

    #[tool(description = "タスク一覧からスケジュール配置を提案する")]
    async fn suggest_schedule(
        &self,
        Parameters(p): Parameters<SuggestScheduleParams>,
    ) -> Result<CallToolResult, McpError> {
        let target_date = parse_date(&p.target_date)?;
        let tasks: Vec<ScheduleTask> = p
            .tasks
            .into_iter()
            .map(|t| ScheduleTask {
                id: t.id,
                title: t.title,
                duration_minutes: t.duration_minutes,
                priority: t.priority,
                focus: t.focus,
            })
            .collect();
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let suggestion =
            scheduling::suggest_schedule(&conn, tasks, target_date).map_err(db_err)?;
        to_text_result(&suggestion)
    }

    #[tool(description = "指定日のスケジュールサマリーを取得する")]
    async fn get_day_summary(
        &self,
        Parameters(p): Parameters<GetDaySummaryParams>,
    ) -> Result<CallToolResult, McpError> {
        let date = parse_date(&p.date)?;
        let day_start = chrono::Utc
            .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
            .single()
            .unwrap();
        let day_end = chrono::Utc
            .from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap())
            .single()
            .unwrap();
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let blocks = time_block::list_time_blocks(
            &conn,
            ListTimeBlocksInput {
                start: day_start,
                end: day_end,
                category_id: None,
                project_id: None,
            },
        )
        .map_err(db_err)?;

        let total = blocks.len();
        let total_minutes: i64 = blocks
            .iter()
            .map(|b| (b.end_time - b.start_time).num_minutes())
            .sum();

        #[derive(Serialize)]
        struct DaySummary {
            date: String,
            total_blocks: usize,
            total_hours: f64,
            blocks: Vec<TimeBlock>,
        }

        let summary = DaySummary {
            date: date.to_string(),
            total_blocks: total,
            total_hours: total_minutes as f64 / 60.0,
            blocks,
        };
        to_text_result(&summary)
    }

    #[tool(description = "プロジェクトを作成する")]
    async fn create_project(
        &self,
        Parameters(p): Parameters<CreateProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        let start_date = parse_date(&p.start_date)?;
        let end_date = p.end_date.as_deref().map(parse_date).transpose()?;
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let proj = project::create_project(
            &conn,
            CreateProjectInput {
                name: p.name,
                start_date,
                end_date,
                description: p.description,
                color: p.color,
            },
        )
        .map_err(db_err)?;
        to_text_result(&proj)
    }

    #[tool(description = "プロジェクト一覧を取得する")]
    async fn list_projects(
        &self,
        Parameters(p): Parameters<ListProjectsParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let projects =
            project::list_projects(&conn, p.status.as_deref()).map_err(db_err)?;
        to_text_result(&projects)
    }

    #[tool(description = "プロジェクトのガントチャート用データを取得する")]
    async fn get_gantt_data(
        &self,
        Parameters(p): Parameters<GetGanttDataParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = self.db.lock().map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let tasks = project::get_gantt_data(&conn, &p.project_id).map_err(db_err)?;
        to_text_result(&tasks)
    }
}

use chrono::TimeZone;
use rmcp::handler::server::ServerHandler;

impl ServerHandler for CalendarMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "AI Time Block Calendar MCP Server. \
                 タイムブロックの作成・管理、空き時間検索、スケジュール提案、\
                 プロジェクト管理、ガントチャートデータ取得が可能です。"
                    .to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--health") {
        println!("ok");
        return Ok(());
    }

    let conn = shared::schema::open_default_db()?;
    let server = CalendarMcpServer::new(conn);

    let service = server
        .serve(rmcp::transport::io::stdio())
        .await?;

    service.waiting().await?;
    Ok(())
}
