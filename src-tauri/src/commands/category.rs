use chrono::Utc;
use rusqlite::{params, Connection};
use shared::models::Category;

pub fn create_category_command(
    conn: &Connection,
    name: String,
    color: Option<String>,
) -> anyhow::Result<Category> {
    let category =
        Category::new(name, color).map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;

    conn.execute(
        "INSERT INTO categories (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            category.id,
            category.name,
            category.color,
            Utc::now().to_rfc3339()
        ],
    )?;

    Ok(category)
}

pub fn list_categories_command(conn: &Connection) -> anyhow::Result<Vec<Category>> {
    let mut stmt =
        conn.prepare("SELECT id, name, color, created_at FROM categories ORDER BY name ASC")?;
    let rows = stmt.query_map([], |row| {
        let created_at_str: String = row.get(3)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            created_at,
        })
    })?;

    let categories: Result<Vec<_>, _> = rows.collect();
    Ok(categories?)
}
