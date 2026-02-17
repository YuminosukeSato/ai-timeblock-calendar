use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::TimeBlock,
    validation::{validate_non_empty, ValidationError},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FreeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduleTask {
    pub id: String,
    pub title: String,
    pub duration_minutes: i64,
    pub priority: i32,
    pub focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledTask {
    pub task_id: String,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduleSuggestion {
    pub scheduled: Vec<ScheduledTask>,
    pub warnings: Vec<String>,
}

pub struct ScheduleService;

impl ScheduleService {
    #[allow(clippy::too_many_arguments)]
    pub fn find_free_slots(
        blocks: &[TimeBlock],
        date: NaiveDate,
        duration_minutes: i64,
        count: Option<usize>,
        work_start: NaiveTime,
        work_end: NaiveTime,
    ) -> Vec<FreeSlot> {
        if duration_minutes <= 0 {
            return Vec::new();
        }

        let Some(day_start) = to_utc(date, work_start) else {
            return Vec::new();
        };
        let Some(day_end) = to_utc(date, work_end) else {
            return Vec::new();
        };

        if day_end <= day_start {
            return Vec::new();
        }

        let mut day_blocks: Vec<&TimeBlock> = blocks
            .iter()
            .filter(|b| b.end_time > day_start && b.start_time < day_end)
            .collect();
        day_blocks.sort_by_key(|b| b.start_time);

        let mut cursor = day_start;
        let mut slots = Vec::new();
        let min_duration = Duration::minutes(duration_minutes);

        for block in day_blocks {
            let block_start = block.start_time.max(day_start);
            let block_end = block.end_time.min(day_end);
            if block_start > cursor && block_start - cursor >= min_duration {
                slots.push(FreeSlot {
                    start: cursor,
                    end: block_start,
                });
                if count.is_some_and(|c| slots.len() >= c) {
                    return slots;
                }
            }
            if block_end > cursor {
                cursor = block_end;
            }
        }

        if day_end > cursor && day_end - cursor >= min_duration {
            slots.push(FreeSlot {
                start: cursor,
                end: day_end,
            });
        }

        if let Some(c) = count {
            slots.truncate(c);
        }
        slots
    }

    pub fn suggest_schedule(
        tasks: &[ScheduleTask],
        existing_blocks: &[TimeBlock],
        target_date: NaiveDate,
    ) -> Result<ScheduleSuggestion, ValidationError> {
        for task in tasks {
            validate_non_empty(&task.title, "title")?;
        }

        let mut tasks = tasks.to_vec();
        tasks.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| b.focus.cmp(&a.focus))
        });

        let work_start = NaiveTime::from_hms_opt(9, 0, 0).expect("static time should be valid");
        let work_end = NaiveTime::from_hms_opt(21, 0, 0).expect("static time should be valid");

        let mut occupied: Vec<(DateTime<Utc>, DateTime<Utc>)> = existing_blocks
            .iter()
            .filter(|b| {
                b.start_time.date_naive() == target_date || b.end_time.date_naive() == target_date
            })
            .map(|b| (b.start_time, b.end_time))
            .collect();
        occupied.sort_by_key(|(s, _)| *s);

        let mut scheduled = Vec::new();
        let mut warnings = Vec::new();

        for task in tasks {
            let duration = Duration::minutes(task.duration_minutes.max(1));
            let candidate_slots = Self::find_free_slots(
                &occupied
                    .iter()
                    .enumerate()
                    .map(|(idx, (start, end))| TimeBlock {
                        id: format!("occupied-{idx}"),
                        title: "occupied".to_string(),
                        description: None,
                        start_time: *start,
                        end_time: *end,
                        category_id: None,
                        project_id: None,
                        google_event_id: None,
                        calendar_id: None,
                        account_id: None,
                        color: None,
                        is_all_day: false,
                        status: crate::models::TimeBlockStatus::Confirmed,
                        sync_status: crate::models::SyncStatus::Synced,
                        progress: 0,
                        local_updated_at: Utc::now(),
                        google_updated_at: None,
                        etag: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    })
                    .collect::<Vec<_>>(),
                target_date,
                task.duration_minutes,
                None,
                work_start,
                work_end,
            );

            if let Some(slot) = candidate_slots.first() {
                let task_start = if task.focus {
                    slot.start
                } else {
                    slot.start + Duration::minutes(15)
                };
                let task_end = task_start + duration;
                if task_end <= slot.end {
                    occupied.push((task_start, task_end + Duration::minutes(15)));
                    occupied.sort_by_key(|(s, _)| *s);
                    scheduled.push(ScheduledTask {
                        task_id: task.id,
                        title: task.title,
                        start: task_start,
                        end: task_end,
                    });
                } else {
                    warnings.push(format!(
                        "task '{}' does not fit available slots",
                        task.title
                    ));
                }
            } else {
                warnings.push(format!("task '{}' exceeded daily capacity", task.title));
            }
        }

        Ok(ScheduleSuggestion {
            scheduled,
            warnings,
        })
    }
}

fn to_utc(date: NaiveDate, time: NaiveTime) -> Option<DateTime<Utc>> {
    Utc.from_local_datetime(&date.and_time(time)).single()
}
