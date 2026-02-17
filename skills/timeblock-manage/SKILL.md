---
name: timeblock-manage
description: Time Block CRUD - create, list, update, delete
tools:
  - create_time_block
  - list_time_blocks
  - update_time_block
  - delete_time_block
---

# Time Block Management

Use the MCP server CRUD tools to manage time blocks.

## When to use

Use this skill when the user asks:

- "Create a meeting at 9am tomorrow"
- "Show this week's blocks"
- "Move this event back 1 hour"
- "Delete this event"

## Tools

### create_time_block

Create a time block.

Parameters:
- title: block title
- start_time: start time (RFC3339)
- end_time: end time (RFC3339)
- category_id: category ID (optional)
- project_id: project ID (optional)
- progress: progress percentage (optional)

### list_time_blocks

List time blocks for a date range.

Parameters:
- start: range start (RFC3339)
- end: range end (RFC3339)
- category_id: filter by category (optional)
- project_id: filter by project (optional)

### update_time_block

Update a time block.

Parameters:
- id: target block ID
- title, start_time, end_time, progress, project_id, category_id (all optional)

### delete_time_block

Delete a time block.

Parameters:
- id: target block ID

## Steps

1. Identify the operation (create/read/update/delete)
2. Ask for missing information if needed (datetime, title, etc.)
3. Execute the appropriate tool
4. Report results to the user
