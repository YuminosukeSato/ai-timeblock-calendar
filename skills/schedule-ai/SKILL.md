---
name: schedule-ai
description: AI Scheduling - find free slots, suggest schedule, get day summary
tools:
  - find_free_slots
  - suggest_schedule
  - get_day_summary
---

# AI Scheduling

Use the MCP server scheduling tools to optimize the user's schedule.

## When to use

Use this skill when the user asks:

- "Find free time today"
- "Place these tasks optimally"
- "Show today's schedule summary"

## Tools

### find_free_slots

Find free time slots on a given date.

Parameters:
- date: target date (YYYY-MM-DD)
- duration_minutes: required free time in minutes
- count: max number of slots to return

### suggest_schedule

Suggest schedule placement for a task list.

Parameters:
- tasks: array of {id, title, duration_minutes, priority, focus}
- target_date: target date (YYYY-MM-DD)

### get_day_summary

Get schedule summary for a given date.

Parameters:
- date: target date (YYYY-MM-DD)

Returns: {date, total_blocks, total_hours, blocks}

## Steps

1. Understand the user's request
2. Use `get_day_summary` to check the current state if needed
3. Use `find_free_slots` to search for openings, or `suggest_schedule` to propose task placement
4. Present results clearly to the user
