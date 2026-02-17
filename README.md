# AI Time Block Calendar

A desktop calendar where AI creates actionable schedules via MCP server.

Not just a schedule viewer — it provides a full execution loop:

1. AI generates task placement proposals via MCP tools
2. Results persist to SQLite
3. Desktop UI syncs for real calendar operations

## Quick Start

### Claude Code Plugin (Recommended)

```bash
# 1. Install MCP server
npm i -g @aitimeblock/ai-timeblock-installer
ai-timeblock install --with-mcp

# 2. Install plugin in Claude Code
/plugin install github:YuminosukeSato/ai-timeblock-calendar
```

After plugin installation, MCP tools and skills are automatically enabled.

### npm

```bash
npm i -g @aitimeblock/ai-timeblock-installer
ai-timeblock install --with-gui --with-mcp
ai-timeblock doctor
```

### Homebrew

```bash
brew tap YuminosukeSato/ai-timeblock
brew install --cask ai-timeblock-calendar   # GUI app
brew install ai-timeblock-mcp               # MCP server only
```

## Usage

### Let AI manage your schedule

Use natural language from Claude Code (or any MCP-compatible AI client):

```
> Find free slots today and add a 1-hour focus block
> Create a "Weekly Meeting" from 9am to 10am tomorrow
> Show today's schedule summary
> Display project progress as a Gantt chart
```

### View and edit in the desktop app

The Tauri desktop app provides visual time block management.
It shares SQLite (WAL mode) with the MCP server, so AI-created schedules appear in real time.

## Architecture

```text
Claude/Codex --stdio--> mcp-server (Rust)
                             |
                             v
                     shared SQLite (WAL)
                             ^
                             |
React UI --invoke--> src-tauri commands (Rust)
```

Components:

- mcp-server: Rust MCP server (rmcp v0.15, stdio transport)
- shared: SQLite access layer and domain models (crates/shared)
- src-tauri: Tauri application (Rust backend)
- src: React + TypeScript frontend

## MCP Tools Reference

### Time Block CRUD

| Tool | Description |
|------|-------------|
| `create_time_block` | Create a time block (title, start_time, end_time, category_id, project_id, progress) |
| `list_time_blocks` | List time blocks for a date range (start, end, category_id, project_id) |
| `update_time_block` | Update a time block (id + optional fields) |
| `delete_time_block` | Delete a time block (id) |

### Scheduling

| Tool | Description |
|------|-------------|
| `find_free_slots` | Find free time slots on a given date (date, duration_minutes, count) |
| `suggest_schedule` | Suggest schedule placement for a task list (tasks, target_date) |
| `get_day_summary` | Get schedule summary for a given date (date) |

### Projects

| Tool | Description |
|------|-------------|
| `create_project` | Create a project (name, start_date, end_date, description, color) |
| `list_projects` | List projects (status) |
| `get_gantt_data` | Get Gantt chart data for a project (project_id) |

## Development

### Prerequisites

- Rust stable
- Node.js 20+
- pnpm 10+

### Setup

```bash
git clone https://github.com/YuminosukeSato/ai-timeblock-calendar.git
cd ai-timeblock-calendar
pnpm install
```

### Test / Lint / Format

```bash
# All tests
cargo test --workspace
pnpm test

# Lint (TypeScript + Rust)
pnpm lint

# Format
pnpm format

# Format check
pnpm format:check
```

### Run MCP server standalone

```bash
cargo run -p mcp-server
```

### Desktop app development

```bash
pnpm tauri dev
```

## License

Apache License 2.0. See [LICENSE](LICENSE) for details.
