use chrono::{DateTime, Utc};
use rusqlite::{params, types::Type, Connection};

use crate::{
    models::{ProjectStatus, SyncStatus, TimeBlock, TimeBlockStatus},
    validation::{validate_progress, validate_time_range, ValidationError},
};

#[derive(Debug, Clone)]
pub struct TimeBlockFilter {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub category_id: Option<String>,
    pub project_id: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TimeBlockPatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub progress: Option<i32>,
    pub project_id: Option<Option<String>>,
    pub category_id: Option<Option<String>>,
}

pub trait TimeBlockRepository {
    fn create_time_block(&self, block: &TimeBlock) -> anyhow::Result<()>;
    fn list_time_blocks(&self, filter: &TimeBlockFilter) -> anyhow::Result<Vec<TimeBlock>>;
    fn get_time_block(&self, id: &str) -> anyhow::Result<Option<TimeBlock>>;
    fn update_time_block(
        &self,
        id: &str,
        patch: &TimeBlockPatch,
    ) -> anyhow::Result<Option<TimeBlock>>;
    fn delete_time_block(&self, id: &str) -> anyhow::Result<bool>;
}

pub struct SqliteTimeBlockRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteTimeBlockRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl TimeBlockRepository for SqliteTimeBlockRepository<'_> {
    fn create_time_block(&self, block: &TimeBlock) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO time_blocks (
                id, title, description, start_time, end_time, category_id, project_id,
                google_event_id, calendar_id, account_id, color, is_all_day,
                status, sync_status, progress, local_updated_at, google_updated_at,
                etag, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            params![
                block.id,
                block.title,
                block.description,
                block.start_time.to_rfc3339(),
                block.end_time.to_rfc3339(),
                block.category_id,
                block.project_id,
                block.google_event_id,
                block.calendar_id,
                block.account_id,
                block.color,
                i32::from(block.is_all_day),
                block.status.as_db_str(),
                block.sync_status.as_db_str(),
                block.progress,
                block.local_updated_at.to_rfc3339(),
                block.google_updated_at.map(|d| d.to_rfc3339()),
                block.etag,
                block.created_at.to_rfc3339(),
                block.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn list_time_blocks(&self, filter: &TimeBlockFilter) -> anyhow::Result<Vec<TimeBlock>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, start_time, end_time, category_id, project_id,
                    google_event_id, calendar_id, account_id, color, is_all_day, status,
                    sync_status, progress, local_updated_at, google_updated_at, etag,
                    created_at, updated_at
             FROM time_blocks
             WHERE NOT (end_time <= ?1 OR start_time >= ?2)
             AND (?3 IS NULL OR category_id = ?3)
             AND (?4 IS NULL OR project_id = ?4)
             ORDER BY start_time ASC",
        )?;

        let rows = stmt.query_map(
            params![
                filter.start.to_rfc3339(),
                filter.end.to_rfc3339(),
                filter.category_id,
                filter.project_id,
            ],
            map_time_block,
        )?;

        let blocks: Result<Vec<_>, _> = rows.collect();
        Ok(blocks?)
    }

    fn get_time_block(&self, id: &str) -> anyhow::Result<Option<TimeBlock>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, start_time, end_time, category_id, project_id,
                    google_event_id, calendar_id, account_id, color, is_all_day, status,
                    sync_status, progress, local_updated_at, google_updated_at, etag,
                    created_at, updated_at
             FROM time_blocks
             WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_time_block(row)?));
        }
        Ok(None)
    }

    fn update_time_block(
        &self,
        id: &str,
        patch: &TimeBlockPatch,
    ) -> anyhow::Result<Option<TimeBlock>> {
        let mut block = match self.get_time_block(id)? {
            Some(b) => b,
            None => return Ok(None),
        };

        if let Some(title) = &patch.title {
            block.title.clone_from(title);
        }
        if let Some(description) = &patch.description {
            block.description.clone_from(description);
        }
        if let Some(start) = patch.start_time {
            block.start_time = start;
        }
        if let Some(end) = patch.end_time {
            block.end_time = end;
        }
        if let Some(progress) = patch.progress {
            validate_progress(progress).map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;
            block.progress = progress;
        }
        if let Some(project_id) = &patch.project_id {
            block.project_id.clone_from(project_id);
        }
        if let Some(category_id) = &patch.category_id {
            block.category_id.clone_from(category_id);
        }

        validate_time_range(block.start_time, block.end_time)
            .map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;

        block.updated_at = Utc::now();
        block.local_updated_at = block.updated_at;
        block.sync_status = SyncStatus::PendingPush;

        self.conn.execute(
            "UPDATE time_blocks SET
                title = ?2,
                description = ?3,
                start_time = ?4,
                end_time = ?5,
                category_id = ?6,
                project_id = ?7,
                progress = ?8,
                sync_status = ?9,
                local_updated_at = ?10,
                updated_at = ?11
            WHERE id = ?1",
            params![
                block.id,
                block.title,
                block.description,
                block.start_time.to_rfc3339(),
                block.end_time.to_rfc3339(),
                block.category_id,
                block.project_id,
                block.progress,
                block.sync_status.as_db_str(),
                block.local_updated_at.to_rfc3339(),
                block.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(Some(block))
    }

    fn delete_time_block(&self, id: &str) -> anyhow::Result<bool> {
        let affected = self
            .conn
            .execute("DELETE FROM time_blocks WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }
}

fn map_time_block(row: &rusqlite::Row<'_>) -> rusqlite::Result<TimeBlock> {
    let parse = |value: String, col: usize| {
        DateTime::parse_from_rfc3339(&value)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(col, Type::Text, Box::new(e)))
    };

    let status_from_db = |value: String| -> TimeBlockStatus {
        match value.as_str() {
            "tentative" => TimeBlockStatus::Tentative,
            "cancelled" => TimeBlockStatus::Cancelled,
            _ => TimeBlockStatus::Confirmed,
        }
    };

    let sync_status_from_db = |value: String| -> SyncStatus {
        match value.as_str() {
            "synced" => SyncStatus::Synced,
            "pending_push" => SyncStatus::PendingPush,
            "pending_delete" => SyncStatus::PendingDelete,
            "conflict" => SyncStatus::Conflict,
            _ => SyncStatus::Local,
        }
    };

    Ok(TimeBlock {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        start_time: parse(row.get(3)?, 3)?,
        end_time: parse(row.get(4)?, 4)?,
        category_id: row.get(5)?,
        project_id: row.get(6)?,
        google_event_id: row.get(7)?,
        calendar_id: row.get(8)?,
        account_id: row.get(9)?,
        color: row.get(10)?,
        is_all_day: row.get::<_, i32>(11)? == 1,
        status: status_from_db(row.get(12)?),
        sync_status: sync_status_from_db(row.get(13)?),
        progress: row.get(14)?,
        local_updated_at: parse(row.get(15)?, 15)?,
        google_updated_at: row
            .get::<_, Option<String>>(16)?
            .map(|v| parse(v, 16))
            .transpose()?,
        etag: row.get(17)?,
        created_at: parse(row.get(18)?, 18)?,
        updated_at: parse(row.get(19)?, 19)?,
    })
}

#[allow(dead_code)]
fn _project_status_to_db(status: ProjectStatus) -> &'static str {
    status.as_db_str()
}

#[allow(dead_code)]
fn _validation_passthrough(err: ValidationError) -> ValidationError {
    err
}
