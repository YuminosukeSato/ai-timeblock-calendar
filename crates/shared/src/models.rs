use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::validation::{
    validate_date_range, validate_non_empty, validate_progress, validate_time_range,
    ValidationError,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

impl Category {
    pub fn new(name: String, color: Option<String>) -> Result<Self, ValidationError> {
        validate_non_empty(&name, "name")?;

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            color: color.unwrap_or_else(|| "#3B82F6".to_string()),
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Self, ValidationError> {
        validate_non_empty(&name, "name")?;
        validate_date_range(start_date, end_date)?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            color: color.unwrap_or_else(|| "#10B981".to_string()),
            start_date,
            end_date,
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Active,
    Completed,
    Archived,
}

impl ProjectStatus {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeBlockStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

impl TimeBlockStatus {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Tentative => "tentative",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Local,
    Synced,
    PendingPush,
    PendingDelete,
    Conflict,
}

impl SyncStatus {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Synced => "synced",
            Self::PendingPush => "pending_push",
            Self::PendingDelete => "pending_delete",
            Self::Conflict => "conflict",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeBlock {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub category_id: Option<String>,
    pub project_id: Option<String>,
    pub google_event_id: Option<String>,
    pub calendar_id: Option<String>,
    pub account_id: Option<String>,
    pub color: Option<String>,
    pub is_all_day: bool,
    pub status: TimeBlockStatus,
    pub sync_status: SyncStatus,
    pub progress: i32,
    pub local_updated_at: DateTime<Utc>,
    pub google_updated_at: Option<DateTime<Utc>>,
    pub etag: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimeBlock {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        description: Option<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        category_id: Option<String>,
        project_id: Option<String>,
        progress: i32,
    ) -> Result<Self, ValidationError> {
        validate_non_empty(&title, "title")?;
        validate_time_range(start_time, end_time)?;
        validate_progress(progress)?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            start_time,
            end_time,
            category_id,
            project_id,
            google_event_id: None,
            calendar_id: None,
            account_id: None,
            color: None,
            is_all_day: false,
            status: TimeBlockStatus::Confirmed,
            sync_status: SyncStatus::Local,
            progress,
            local_updated_at: now,
            google_updated_at: None,
            etag: None,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeBlockDependency {
    pub time_block_id: String,
    pub depends_on_time_block_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarAccount {
    pub id: String,
    pub email: String,
    pub access_token_encrypted: Option<String>,
    pub refresh_token_encrypted: String,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyncState {
    pub calendar_id: String,
    pub account_id: String,
    pub sync_token: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub full_sync_done: bool,
}
