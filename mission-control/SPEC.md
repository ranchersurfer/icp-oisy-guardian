Mission Control Dashboard – Product Spec
=======================================

1. High-Level Product Spec
--------------------------

Name: Mission Control Dashboard

Tech stack (baseline):
- Frontend: Next.js + React, Tailwind/Chakra-style UI (Linear-like look)
- Backend: Node/Next API routes, reading/writing your ~/.openclaw workspace
- Storage:
  - OpenClaw files (tasks, memories, docs, cron config, agents_state.json)
  - Optional: SQLite / Postgres for indexing & caching

Primary data sources:
- agents_state.json and mission_control_dashboard.md
- Task files (e.g. TASKS.md, projects.md, etc.)
- Logs and cron config (for calendar)
- Memory and docs directories (for Memories/Docs screens)

Core screens:
- Task Board
- Calendar
- Projects
- Memories
- Docs
- Team

2. Global Layout / Navigation
-----------------------------

Left sidebar:
- App name + mission statement snippet
- Icon list of screens: Task Board, Calendar, Projects, Memories, Docs, Team
- Active screen highlighted

Top bar:
- Current "primary agent" (e.g. Mission Control)
- Status pill (Idle / Working / Error)
- Quick actions dropdown:
  - "Reverse Prompt"
  - "New Task"
  - "Ask: What should we do now?"

Right sidebar (context panel):
- Live activity feed (agent events, tool calls, task transitions)
- Filters by agent / project / time window

3. Screen 1 – Task Board (Kanban + Activity Feed)
-------------------------------------------------

Purpose:
- Show everything agents and humans are working on, with status and live activity log.

Data model – Task:
- id
- title
- description
- status ∈ {backlog, doing, review, done}
- assignee_type ∈ {human, agent}
- assignee_id (e.g. moises, mission-control, dream-cycle)
- project_id (optional)
- created_at
- updated_at
- completed_at
- priority (low/med/high)
- source (manual / reverse-prompt / cron)

Data model – Activity event:
- id
- timestamp
- agent_id
- task_id (optional)
- type ∈ {started_task, updated_task, completed_task, tool_call, error, note}
- message (human-readable string)

UI / behavior – Kanban board:
- Four columns: Backlog, In Progress, In Review, Done
- Each card shows:
  - Title
  - Assignee initial (A = you, H = primary agent, others from agents_state.json)
  - Short description
  - Last update time
- Drag-and-drop between columns updates status
- "New Task" button:
  - Opens modal with title, description, project, assignee, priority

UI / behavior – Live activity feed:
- Vertical list, newest at top
- Each row:
  - Timestamp
  - Agent avatar/initial
  - Event description (e.g. "Mission Control moved task 'Refactor memory' → In Review")
- Filters:
  - All / This Task / This Agent

Heartbeat integration (logic):
- Each agent's heartbeat:
  - Calls Task API to fetch tasks assigned to it in backlog or doing
  - Endpoint example: GET /api/tasks?assignee=mission-control&status=backlog

4. Screen 2 – Calendar (Cron & Scheduled Tasks)
-----------------------------------------------

Purpose:
- Show cron jobs and scheduled tasks to verify proactive behavior.

Data model – Scheduled item:
- id
- title
- description
- schedule (cron string or ISO datetime)
- agent_id
- type ∈ {cron, one_off}
- next_run_at
- last_run_at
- source_file (e.g. scripts/dream-cycle.sh, crontab, OpenClaw automation config)
- status ∈ {active, paused, error}

UI / behavior – Calendar view:
- Month and week views
- Events colored by agent (e.g. Dream Cycle, Mission Control)
- Hover tooltip: schedule, script, last run, status

UI / behavior – List view:
- Table of all cron jobs and upcoming tasks
- Filters: by agent and status

Consistency helper:
- If OpenClaw claims "I scheduled X" but the schedule is missing, raise "inconsistency" warning

5. Screen 3 – Projects
----------------------

Purpose:
- Track major projects, linking them to tasks, docs, and memories.

Data model – Project:
- id
- name
- description
- status ∈ {not_started, in_progress, blocked, done}
- progress (0–1 or percentage)
- owner_agent_id or owner_human
- created_at
- updated_at
- related_tasks (list of task IDs)
- related_docs (doc IDs)
- related_memories (memory IDs)
- roadmap_id (link to roadmap entry in agents_state.json)

UI / behavior – Projects grid/list:
- One card per project:
  - Name
  - Owner
  - Progress bar
  - Status tag

UI / behavior – Project detail:
- Overview:
  - Project mission / goal
- Sections:
  - Linked tasks (embedded table/board subset)
  - Linked docs (from Docs screen)
  - Linked memories (from Memories screen)

Reverse prompting helper:
- Button: "Ask Mission Control: What's 1 task we can do right now for this project?"
  - Prefills a structured prompt using project context

6. Screen 4 – Memories
----------------------

Purpose:
- Journal-like view of OpenClaw's memories, grouped by day and topic.

Data model – Memory entry:
- id
- date (day bucket)
- timestamp
- summary
- raw_path (file path and offset)
- tags (e.g. project:guardian, topic:security)
- source_agent_id

UI / behavior – Day timeline:
- Left: list of days
- Right: memory cards for the selected day
- Each card:
  - Time
  - Short summary
  - Tags
  - "View full" link to raw content

Filters:
- By project
- By agent
- By tag
- By keyword search

Long-term memory view:
- "Long-term memories" section:
  - Shows stable decisions, rules, policies extracted from Tier 2 memory files

7. Screen 5 – Docs
------------------

Purpose:
- Central library of documents generated by agents (plans, specs, newsletters, scripts, etc.).

Data model – Document:
- id
- title
- path
- type ∈ {spec, plan, newsletter, script, misc}
- format ∈ {md, txt, pdf, other}
- created_at
- updated_at
- project_id (optional)
- agent_id (who generated it)

UI / behavior:
- List with filters by type, project, agent, date
- Full-text search across docs
- Preview panel:
  - Renders Markdown
  - Shows metadata (project, agent, type, timestamps)

Tagging:
- Auto-tag based on folder/keywords
- Manual tag editing in UI

8. Screen 6 – Team
------------------

Purpose:
- Org chart for all agents and sub-agents, plus mission statement at top.

Data model – Agent (from agents_state.json):
- id
- name
- role
- device (e.g. "WSL Docker", "Mac Studio", "Cloud VPS")
- model
- parent_id (hierarchy)
- tools (allow/deny lists)
- current_task
- status ∈ {idle, working, error}
- mission_statement (global, shown once at top)

UI / behavior – Org tree:
- Root node = primary agent (Mission Control)
- Children = sub-agents and sub-sub-agents

Sidebar mission:
- Single mission statement displayed at top, describing the overall "company" of agents

Agent detail view:
- Shows:
  - Capabilities and tools
  - Current tasks and status
  - Recent activity
  - Associated projects

9. Data & Integration Layer
----------------------------

Ingestion:
- Backend job or OpenClaw itself keeps:
  - agents_state.json
  - Task files
  - Cron configs
  - Memory index
  - Docs index
  in sync with reality.

APIs exposed by dashboard:
- GET /api/tasks
- PATCH /api/tasks/:id
- GET /api/schedule
- GET /api/projects
- GET /api/memories
- GET /api/docs
- GET /api/agents (reads agents_state.json)

10. Metrics Panel (Optional)
----------------------------

Purpose:
- Provide high-level operational metrics, aligned with your existing mission_control_dashboard.md.

Key metrics:
- Tokens used per agent (today / last 7 days)
- Estimated monthly cost vs baseline
- Memory system stats:
  - Tier 1 total size
  - Tier 2 file count, last update time
- Security posture:
  - Score
  - Last audit date
- Automation:
  - Number of cron jobs
  - Pass/fail status for recent runs

UI / behavior:
- Located as:
  - Right-side collapsible drawer on Task Board
  - Or dedicated "Metrics" tab
- Visuals:
  - Line chart: daily cost
  - Bar chart: tokens per agent
  - Badges / health indicators: memory, security, automation
