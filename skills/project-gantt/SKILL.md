---
name: project-gantt
description: Project management - create, list, get Gantt chart data
tools:
  - create_project
  - list_projects
  - get_gantt_data
---

# Project & Gantt Management

Use the MCP server project tools to manage projects and Gantt charts.

## When to use

Use this skill when the user asks:

- "Create a new project"
- "Show project list"
- "Display project progress as Gantt chart"

## Tools

### create_project

Create a project.

Parameters:
- name: project name
- start_date: start date (YYYY-MM-DD)
- end_date: end date (YYYY-MM-DD)
- description: description (optional)
- color: color code (optional)

### list_projects

List projects.

Parameters:
- status: filter (active / completed / archived, optional)

### get_gantt_data

Get Gantt chart data for a project.

Parameters:
- project_id: target project ID

## Steps

1. Understand the user's request
2. Use `list_projects` to check existing projects if needed
3. Use `create_project` to create new, or `get_gantt_data` to view progress
4. Present results to the user
