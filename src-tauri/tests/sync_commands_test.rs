use chrono::{Duration, Utc};
use rusqlite::{params, Connection};
use shared::schema::run_migrations;

use app_tauri::commands::sync::{
    pull_sync, push_sync, refresh_token_if_expired, GoogleCalendarClient, GoogleEvent,
    PullSyncMode, PullSyncResult, TokenRefreshResponse,
};

#[derive(Default)]
struct FakeGoogleClient {
    events: Vec<GoogleEvent>,
    next_token: Option<String>,
    force_etag_mismatch: bool,
}

impl GoogleCalendarClient for FakeGoogleClient {
    fn list_events(
        &self,
        _calendar_id: &str,
        _sync_token: Option<&str>,
        _time_min: Option<&str>,
    ) -> anyhow::Result<(Vec<GoogleEvent>, Option<String>)> {
        Ok((self.events.clone(), self.next_token.clone()))
    }

    fn patch_event(
        &self,
        _calendar_id: &str,
        _event_id: &str,
        _title: &str,
        _start: &str,
        _end: &str,
        _if_match_etag: Option<&str>,
    ) -> anyhow::Result<GoogleEvent> {
        if self.force_etag_mismatch {
            return Err(anyhow::anyhow!("etag_mismatch"));
        }
        Ok(GoogleEvent {
            id: "patched-1".to_string(),
            summary: "patched".to_string(),
            start: Utc::now().to_rfc3339(),
            end: (Utc::now() + Duration::hours(1)).to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            etag: Some("etag-2".to_string()),
            deleted: false,
        })
    }

    fn insert_event(
        &self,
        _calendar_id: &str,
        _title: &str,
        _start: &str,
        _end: &str,
    ) -> anyhow::Result<GoogleEvent> {
        Ok(GoogleEvent {
            id: "inserted-1".to_string(),
            summary: "inserted".to_string(),
            start: Utc::now().to_rfc3339(),
            end: (Utc::now() + Duration::hours(1)).to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            etag: Some("etag-insert".to_string()),
            deleted: false,
        })
    }

    fn refresh_access_token(&self, _refresh_token: &str) -> anyhow::Result<TokenRefreshResponse> {
        Ok(TokenRefreshResponse {
            access_token: "new-access".to_string(),
            expires_in_seconds: 3600,
        })
    }
}

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().expect("in-memory db should open");
    run_migrations(&conn).expect("migrations should run");

    conn.execute(
        "INSERT INTO calendar_accounts (id, email, refresh_token_encrypted, access_token_encrypted, token_expires_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            "acc-1",
            "a@example.com",
            "refresh-token",
            "access-token",
            (Utc::now() - Duration::hours(2)).to_rfc3339()
        ],
    )
    .expect("account should be seeded");

    conn
}

#[test]
fn pull_sync_without_sync_token_uses_full_mode() {
    let conn = setup_db();
    let client = FakeGoogleClient {
        next_token: Some("next-1".to_string()),
        ..FakeGoogleClient::default()
    };

    let result = pull_sync(&conn, &client, "primary", "acc-1").expect("pull should succeed");

    assert_eq!(result.mode, PullSyncMode::Full);
}

#[test]
fn pull_sync_with_sync_token_uses_delta_mode() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO sync_state (calendar_id, account_id, sync_token, full_sync_done) VALUES (?1, ?2, ?3, 1)",
        params!["primary", "acc-1", "token-1"],
    )
    .expect("sync state should be seeded");

    let client = FakeGoogleClient {
        next_token: Some("next-2".to_string()),
        ..FakeGoogleClient::default()
    };

    let result = pull_sync(&conn, &client, "primary", "acc-1").expect("pull should succeed");

    assert_eq!(result.mode, PullSyncMode::Delta);
}

#[test]
fn pull_sync_marks_deleted_events_as_cancelled() {
    let conn = setup_db();

    conn.execute(
        "INSERT INTO time_blocks (id, title, start_time, end_time, account_id, google_event_id, status, sync_status, local_updated_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'confirmed', 'synced', ?7, ?7, ?7)",
        params![
            "tb-1",
            "task",
            Utc::now().to_rfc3339(),
            (Utc::now() + Duration::hours(1)).to_rfc3339(),
            "acc-1",
            "google-1",
            Utc::now().to_rfc3339()
        ],
    )
    .expect("time block should be seeded");

    let client = FakeGoogleClient {
        events: vec![GoogleEvent {
            id: "google-1".to_string(),
            summary: "task".to_string(),
            start: Utc::now().to_rfc3339(),
            end: (Utc::now() + Duration::hours(1)).to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            etag: Some("etag-1".to_string()),
            deleted: true,
        }],
        next_token: Some("next-3".to_string()),
        ..FakeGoogleClient::default()
    };

    pull_sync(&conn, &client, "primary", "acc-1").expect("pull should succeed");

    let status: String = conn
        .query_row(
            "SELECT status FROM time_blocks WHERE id = 'tb-1'",
            [],
            |row| row.get(0),
        )
        .expect("status should exist");
    assert_eq!(status, "cancelled");
}

#[test]
fn push_sync_sends_pending_push_with_patch() {
    let conn = setup_db();
    let now = Utc::now();

    conn.execute(
        "INSERT INTO time_blocks (id, title, start_time, end_time, account_id, google_event_id, status, sync_status, etag, local_updated_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'confirmed', 'pending_push', ?7, ?8, ?8, ?8)",
        params!["tb-2", "task", now.to_rfc3339(), (now + Duration::hours(1)).to_rfc3339(), "acc-1", "google-2", "etag-1", now.to_rfc3339()],
    )
    .expect("time block should be seeded");

    let client = FakeGoogleClient::default();

    let result = push_sync(&conn, &client, "primary", "acc-1").expect("push should succeed");

    assert!(result.pushed > 0);
}

#[test]
fn push_sync_marks_conflict_on_etag_mismatch() {
    let conn = setup_db();
    let now = Utc::now();

    conn.execute(
        "INSERT INTO time_blocks (id, title, start_time, end_time, account_id, google_event_id, status, sync_status, etag, local_updated_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'confirmed', 'pending_push', ?7, ?8, ?8, ?8)",
        params!["tb-3", "task", now.to_rfc3339(), (now + Duration::hours(1)).to_rfc3339(), "acc-1", "google-3", "etag-1", now.to_rfc3339()],
    )
    .expect("time block should be seeded");

    let client = FakeGoogleClient {
        force_etag_mismatch: true,
        ..FakeGoogleClient::default()
    };

    push_sync(&conn, &client, "primary", "acc-1").expect("push should succeed");

    let sync_status: String = conn
        .query_row(
            "SELECT sync_status FROM time_blocks WHERE id = 'tb-3'",
            [],
            |row| row.get(0),
        )
        .expect("sync status should exist");
    assert_eq!(sync_status, "conflict");
}

#[test]
fn refresh_token_updates_expired_token() {
    let conn = setup_db();
    let client = FakeGoogleClient::default();

    refresh_token_if_expired(&conn, &client, "acc-1").expect("refresh should succeed");

    let access: String = conn
        .query_row(
            "SELECT access_token_encrypted FROM calendar_accounts WHERE id = 'acc-1'",
            [],
            |row| row.get(0),
        )
        .expect("access token should exist");
    assert_eq!(access, "new-access");
}

fn _assert_pull_result_type(_: PullSyncResult) {}
