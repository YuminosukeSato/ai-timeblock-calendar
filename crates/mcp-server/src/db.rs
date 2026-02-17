use rusqlite::Connection;

pub fn open_shared_db() -> anyhow::Result<Connection> {
    shared::schema::open_default_db()
}
