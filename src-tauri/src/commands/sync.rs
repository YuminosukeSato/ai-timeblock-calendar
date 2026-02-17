use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoogleEvent {
    pub id: String,
    pub summary: String,
    pub start: String,
    pub end: String,
    pub updated: String,
    pub etag: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PullSyncMode {
    Full,
    Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullSyncResult {
    pub mode: PullSyncMode,
    pub pulled: usize,
    pub next_sync_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PushSyncResult {
    pub pushed: usize,
    pub conflicts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenRefreshResponse {
    pub access_token: String,
    pub expires_in_seconds: i64,
}

pub trait GoogleCalendarClient {
    fn list_events(
        &self,
        calendar_id: &str,
        sync_token: Option<&str>,
        time_min: Option<&str>,
    ) -> anyhow::Result<(Vec<GoogleEvent>, Option<String>)>;

    fn patch_event(
        &self,
        calendar_id: &str,
        event_id: &str,
        title: &str,
        start: &str,
        end: &str,
        if_match_etag: Option<&str>,
    ) -> anyhow::Result<GoogleEvent>;

    fn insert_event(
        &self,
        calendar_id: &str,
        title: &str,
        start: &str,
        end: &str,
    ) -> anyhow::Result<GoogleEvent>;

    fn refresh_access_token(&self, refresh_token: &str) -> anyhow::Result<TokenRefreshResponse>;
}

pub fn pull_sync(
    conn: &Connection,
    client: &dyn GoogleCalendarClient,
    calendar_id: &str,
    account_id: &str,
) -> anyhow::Result<PullSyncResult> {
    let sync_token: Option<String> = conn
        .query_row(
            "SELECT sync_token FROM sync_state WHERE calendar_id = ?1 AND account_id = ?2",
            params![calendar_id, account_id],
            |row| row.get(0),
        )
        .optional()?;

    let mode = if sync_token.is_some() {
        PullSyncMode::Delta
    } else {
        PullSyncMode::Full
    };

    let time_min = if sync_token.is_none() {
        Some((Utc::now() - Duration::days(365)).to_rfc3339())
    } else {
        None
    };

    let (events, next_sync_token) =
        client.list_events(calendar_id, sync_token.as_deref(), time_min.as_deref())?;

    for event in &events {
        if event.deleted {
            conn.execute(
                "UPDATE time_blocks
                 SET status = 'cancelled', sync_status = 'synced', updated_at = ?1
                 WHERE google_event_id = ?2 AND account_id = ?3",
                params![Utc::now().to_rfc3339(), event.id, account_id],
            )?;
            continue;
        }

        let now = Utc::now().to_rfc3339();
        let existing_id: Option<String> = conn
            .query_row(
                "SELECT id FROM time_blocks WHERE google_event_id = ?1 AND account_id = ?2",
                params![event.id, account_id],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(id) = existing_id {
            conn.execute(
                "UPDATE time_blocks
                 SET title = ?2, start_time = ?3, end_time = ?4, etag = ?5, google_updated_at = ?6, sync_status = 'synced', updated_at = ?6
                 WHERE id = ?1",
                params![id, event.summary, event.start, event.end, event.etag, event.updated],
            )?;
        } else {
            conn.execute(
                "INSERT INTO time_blocks (
                    id, title, start_time, end_time, account_id, google_event_id, status, sync_status,
                    local_updated_at, google_updated_at, etag, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'confirmed', 'synced', ?7, ?8, ?9, ?7, ?7)",
                params![
                    Uuid::new_v4().to_string(),
                    event.summary,
                    event.start,
                    event.end,
                    account_id,
                    event.id,
                    now,
                    event.updated,
                    event.etag,
                ],
            )?;
        }
    }

    conn.execute(
        "INSERT INTO sync_state (calendar_id, account_id, sync_token, last_synced_at, full_sync_done)
         VALUES (?1, ?2, ?3, ?4, 1)
         ON CONFLICT(calendar_id)
         DO UPDATE SET
             account_id = excluded.account_id,
             sync_token = excluded.sync_token,
             last_synced_at = excluded.last_synced_at,
             full_sync_done = 1",
        params![calendar_id, account_id, next_sync_token, Utc::now().to_rfc3339()],
    )?;

    Ok(PullSyncResult {
        mode,
        pulled: events.len(),
        next_sync_token,
    })
}

pub fn push_sync(
    conn: &Connection,
    client: &dyn GoogleCalendarClient,
    calendar_id: &str,
    account_id: &str,
) -> anyhow::Result<PushSyncResult> {
    let mut stmt = conn.prepare(
        "SELECT id, title, start_time, end_time, google_event_id, etag, sync_status
         FROM time_blocks
         WHERE account_id = ?1 AND sync_status IN ('pending_push', 'pending_delete')
         ORDER BY updated_at ASC",
    )?;

    let rows = stmt.query_map(params![account_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    let pending: Result<Vec<_>, _> = rows.collect();
    let pending = pending?;

    let mut pushed = 0usize;
    let mut conflicts = 0usize;

    for (id, title, start, end, google_event_id, etag, sync_status) in pending {
        if sync_status == "pending_delete" {
            conn.execute(
                "UPDATE time_blocks SET sync_status = 'synced', updated_at = ?2 WHERE id = ?1",
                params![id, Utc::now().to_rfc3339()],
            )?;
            pushed += 1;
            continue;
        }

        let result = if let Some(event_id) = google_event_id {
            client.patch_event(
                calendar_id,
                &event_id,
                &title,
                &start,
                &end,
                etag.as_deref(),
            )
        } else {
            client.insert_event(calendar_id, &title, &start, &end)
        };

        match result {
            Ok(remote) => {
                conn.execute(
                    "UPDATE time_blocks
                     SET google_event_id = COALESCE(google_event_id, ?2),
                         sync_status = 'synced',
                         etag = ?3,
                         google_updated_at = ?4,
                         updated_at = ?4
                     WHERE id = ?1",
                    params![id, remote.id, remote.etag, remote.updated],
                )?;
                pushed += 1;
            }
            Err(err) => {
                if err.to_string().contains("etag_mismatch") {
                    conn.execute(
                        "UPDATE time_blocks SET sync_status = 'conflict', updated_at = ?2 WHERE id = ?1",
                        params![id, Utc::now().to_rfc3339()],
                    )?;
                    conflicts += 1;
                    continue;
                }
                return Err(err);
            }
        }
    }

    Ok(PushSyncResult { pushed, conflicts })
}

pub fn refresh_token_if_expired(
    conn: &Connection,
    client: &dyn GoogleCalendarClient,
    account_id: &str,
) -> anyhow::Result<()> {
    let (refresh_token, expires_at): (String, Option<String>) = conn.query_row(
        "SELECT refresh_token_encrypted, token_expires_at FROM calendar_accounts WHERE id = ?1",
        params![account_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    let is_expired = match expires_at {
        Some(ref value) => DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc) <= Utc::now())
            .unwrap_or(true),
        None => true,
    };

    if !is_expired {
        return Ok(());
    }

    let refreshed = client.refresh_access_token(&refresh_token)?;
    let new_expiry = Utc::now() + Duration::seconds(refreshed.expires_in_seconds);

    conn.execute(
        "UPDATE calendar_accounts
         SET access_token_encrypted = ?2, token_expires_at = ?3
         WHERE id = ?1",
        params![account_id, refreshed.access_token, new_expiry.to_rfc3339()],
    )?;

    Ok(())
}
