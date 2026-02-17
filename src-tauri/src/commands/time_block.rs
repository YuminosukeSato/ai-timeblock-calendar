use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use shared::{
    models::TimeBlock,
    repository::{SqliteTimeBlockRepository, TimeBlockFilter, TimeBlockPatch, TimeBlockRepository},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeBlockCommandInput {
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub category_id: Option<String>,
    pub project_id: Option<String>,
    pub progress: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTimeBlocksCommandInput {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub category_id: Option<String>,
    pub project_id: Option<String>,
}

pub fn create_time_block_command(
    conn: &Connection,
    input: CreateTimeBlockCommandInput,
) -> anyhow::Result<TimeBlock> {
    let repo = SqliteTimeBlockRepository::new(conn);
    let block = TimeBlock::new(
        input.title,
        None,
        input.start_time,
        input.end_time,
        input.category_id,
        input.project_id,
        input.progress,
    )
    .map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;

    repo.create_time_block(&block)?;
    Ok(block)
}

pub fn list_time_blocks_command(
    conn: &Connection,
    input: ListTimeBlocksCommandInput,
) -> anyhow::Result<Vec<TimeBlock>> {
    let repo = SqliteTimeBlockRepository::new(conn);
    repo.list_time_blocks(&TimeBlockFilter {
        start: input.start,
        end: input.end,
        category_id: input.category_id,
        project_id: input.project_id,
    })
}

pub fn update_time_block_command(
    conn: &Connection,
    id: &str,
    patch: TimeBlockPatch,
) -> anyhow::Result<TimeBlock> {
    let repo = SqliteTimeBlockRepository::new(conn);
    repo.update_time_block(id, &patch)?
        .ok_or_else(|| anyhow::anyhow!("time block not found: {id}"))
}

pub fn delete_time_block_command(conn: &Connection, id: &str) -> anyhow::Result<()> {
    let repo = SqliteTimeBlockRepository::new(conn);
    let deleted = repo.delete_time_block(id)?;
    if !deleted {
        return Err(anyhow::anyhow!("time block not found: {id}"));
    }
    Ok(())
}
