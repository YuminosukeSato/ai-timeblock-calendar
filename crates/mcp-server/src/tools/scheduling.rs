use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use shared::services::{FreeSlot, ScheduleService, ScheduleSuggestion, ScheduleTask};

use super::time_block::{list_time_blocks, ListTimeBlocksInput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindFreeSlotsInput {
    pub date: NaiveDate,
    pub duration_minutes: i64,
    pub count: Option<usize>,
}

pub fn find_free_slots(
    conn: &Connection,
    input: FindFreeSlotsInput,
) -> anyhow::Result<Vec<FreeSlot>> {
    let day_start = Utc
        .from_local_datetime(
            &input
                .date
                .and_hms_opt(0, 0, 0)
                .expect("midnight should be valid"),
        )
        .single()
        .expect("utc conversion should be deterministic");
    let day_end = Utc
        .from_local_datetime(
            &input
                .date
                .and_hms_opt(23, 59, 59)
                .expect("end of day should be valid"),
        )
        .single()
        .expect("utc conversion should be deterministic");

    let blocks = list_time_blocks(
        conn,
        ListTimeBlocksInput {
            start: day_start,
            end: day_end,
            category_id: None,
            project_id: None,
        },
    )?;

    let work_start = NaiveTime::from_hms_opt(9, 0, 0).expect("static value should be valid");
    let work_end = NaiveTime::from_hms_opt(21, 0, 0).expect("static value should be valid");

    Ok(ScheduleService::find_free_slots(
        &blocks,
        input.date,
        input.duration_minutes,
        input.count,
        work_start,
        work_end,
    ))
}

pub fn suggest_schedule(
    conn: &Connection,
    tasks: Vec<ScheduleTask>,
    target_date: NaiveDate,
) -> anyhow::Result<ScheduleSuggestion> {
    let day_start = Utc
        .from_local_datetime(
            &target_date
                .and_hms_opt(0, 0, 0)
                .expect("midnight should be valid"),
        )
        .single()
        .expect("utc conversion should be deterministic");
    let day_end = Utc
        .from_local_datetime(
            &target_date
                .and_hms_opt(23, 59, 59)
                .expect("end of day should be valid"),
        )
        .single()
        .expect("utc conversion should be deterministic");

    let blocks = list_time_blocks(
        conn,
        ListTimeBlocksInput {
            start: day_start,
            end: day_end,
            category_id: None,
            project_id: None,
        },
    )?;

    ScheduleService::suggest_schedule(&tasks, &blocks, target_date)
        .map_err(|e| anyhow::anyhow!("schedule suggestion failed: {e}"))
}
