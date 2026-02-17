use std::{fs, path::Path};

use serde_json::{json, Value};

pub fn register_mcp_server(config_path: &Path, server_binary_path: &Path) -> anyhow::Result<()> {
    let existing = if config_path.exists() {
        let text = fs::read_to_string(config_path)?;
        serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    let mut root = if existing.is_object() {
        existing
    } else {
        json!({})
    };
    if root.get("mcpServers").and_then(Value::as_object).is_none() {
        root["mcpServers"] = json!({});
    }

    root["mcpServers"]["ai-timeblock-calendar"] = json!({
        "command": server_binary_path.display().to_string(),
        "args": [],
    });

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(config_path, serde_json::to_string_pretty(&root)?)?;
    Ok(())
}
