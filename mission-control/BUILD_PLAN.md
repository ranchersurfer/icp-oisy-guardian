# Mission Control Dashboard – Build Plan

## Tech Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Framework | Next.js 14 (App Router) | SSR + API routes in one project, file-based routing maps cleanly to screens |
| Styling | Tailwind CSS + shadcn/ui | Linear-like aesthetic out of the box, fast iteration |
| Data layer | File-based (read workspace directly) | No DB needed for v1; workspace files are the source of truth |
| State | React Query (TanStack) | Cache + refetch for live-ish updates without a websocket |
| Rendering | Server Components where static, Client where interactive | Kanban, calendar need client; memory/docs views can be server-rendered |
| Deployment | `next dev` locally (localhost:3000) | Solo dev tool, no cloud needed |

### File Access Strategy
All API routes read directly from `~/.openclaw/workspace/`:
- `agents_state.json` → Team screen + status pill
- `memory/*.md` → Memories screen
- `mission_control_dashboard.md` → Metrics
- Cron config / task files → Calendar + Task Board
- Any `.md` files in workspace → Docs screen

Write operations (task status changes) write back to workspace files.

---

## Phase 1 — Scaffold + Task Board
**Estimated effort: 1–2 days**

### Goals
- Working Next.js app with left sidebar navigation
- Task Board (Kanban) fully functional
- Activity feed (static / file-based, no live updates yet)
- API routes wired to workspace files

### Tasks
- [ ] `npx create-next-app@latest mission-control --app --tailwind --typescript`
- [ ] Install shadcn/ui, lucide-react, @tanstack/react-query
- [ ] Build shell: sidebar nav (6 icons), top bar with status pill, main content area
- [ ] `GET /api/tasks` — reads TASKS.md or tasks.json from workspace
- [ ] `PATCH /api/tasks/:id` — updates task status in file
- [ ] Kanban board UI: 4 columns, drag-and-drop (dnd-kit)
- [ ] Task cards: title, assignee initial, priority badge, updated_at
- [ ] "New Task" modal
- [ ] Activity feed: reads from a simple activity log file (append-only JSON lines)

### Risks
- Task data format TBD — start with a simple `tasks.json` in workspace root, migrate later
- Drag-and-drop write-back needs atomic file writes (use tmp file + rename)

---

## Phase 2 — Calendar + Projects
**Estimated effort: 2–3 days**

### Goals
- Calendar screen showing cron jobs and scheduled tasks
- Projects screen with grid/list + detail view
- Both screens backed by real workspace data

### Tasks
- [ ] `GET /api/schedule` — parse crontab + OpenClaw automation config
- [ ] Calendar UI: month/week toggle (react-big-calendar or custom), color by agent
- [ ] Cron list view: table with agent, schedule string, next/last run, status
- [ ] Inconsistency detector: flag cron entries that don't match claimed schedules
- [ ] `GET /api/projects` — reads projects.md or projects.json
- [ ] Projects grid: cards with name, owner, progress bar, status tag
- [ ] Project detail drawer: linked tasks + docs + memories (IDs only for now)
- [ ] "Ask Mission Control" reverse prompt button (opens prompt prefilled, copies to clipboard)

### Risks
- Cron parsing is fragile if configs are in multiple formats — scope to a single source first
- Project → task linking requires consistent IDs across files (define schema early)

---

## Phase 3 — Memories + Docs + Team
**Estimated effort: 2–3 days**

### Goals
- Memories screen: day timeline view of memory/*.md files
- Docs screen: searchable library of workspace markdown files
- Team screen: org chart from agents_state.json
- Optional: Metrics panel

### Tasks

**Memories**
- [ ] `GET /api/memories` — scan `memory/` dir, parse daily files, return structured entries
- [ ] Day timeline UI: left day list, right memory cards
- [ ] Tag parsing from memory file content (extract `project:x` style tags)
- [ ] Keyword search (client-side filter for v1)
- [ ] Long-term memory section (reads MEMORY.md)

**Docs**
- [ ] `GET /api/docs` — walk workspace, index .md/.txt files with metadata
- [ ] Docs list UI: filter by type/project/agent/date
- [ ] Preview panel: render Markdown (react-markdown)
- [ ] Full-text search (client-side for v1, consider lunr.js if corpus is large)

**Team**
- [ ] `GET /api/agents` — reads agents_state.json
- [ ] Org tree UI: root → children hierarchy (react-organizational-chart or custom)
- [ ] Agent detail panel: tools, current task, status, recent activity
- [ ] Mission statement banner at top of screen

**Metrics (optional)**
- [ ] Collapsible right drawer on Task Board
- [ ] Read mission_control_dashboard.md for metrics data
- [ ] Simple stat cards + recharts line/bar charts

### Risks
- Memory file parsing is format-dependent — write a resilient parser that degrades gracefully
- Docs index could be slow on large workspaces — add a simple file cache (mtime-based)
- agents_state.json schema may not exist yet — may need to define it

---

## Dependencies

| Dependency | Needed By | Notes |
|---|---|---|
| `tasks.json` schema defined | Phase 1 | Keep simple: array of Task objects |
| `projects.json` schema defined | Phase 2 | Can start from projects.md if it exists |
| `agents_state.json` exists | Phase 3 (Team) | Define minimal schema if missing |
| Cron config location known | Phase 2 | Check `~/.openclaw/` for automation config |
| `memory/` directory populated | Phase 3 | Already exists |

---

## Non-Goals (v1)

- No auth — local-only tool, no login
- No real-time websockets — polling every 10–30s is fine
- No cloud deployment
- No pixel art office screen
- No SQLite/Postgres indexing (file reads are fast enough for local workspace)

---

## File Structure (target)

```
mission-control/
├── app/
│   ├── layout.tsx          # Shell: sidebar + top bar
│   ├── page.tsx            # Redirect to /tasks
│   ├── tasks/page.tsx      # Task Board
│   ├── calendar/page.tsx   # Calendar
│   ├── projects/page.tsx   # Projects
│   ├── memories/page.tsx   # Memories
│   ├── docs/page.tsx       # Docs
│   └── team/page.tsx       # Team
├── app/api/
│   ├── tasks/route.ts
│   ├── schedule/route.ts
│   ├── projects/route.ts
│   ├── memories/route.ts
│   ├── docs/route.ts
│   └── agents/route.ts
├── components/
│   ├── Sidebar.tsx
│   ├── TopBar.tsx
│   ├── kanban/
│   ├── calendar/
│   └── ...
├── lib/
│   ├── workspace.ts        # Workspace path resolver
│   ├── tasks.ts            # Task file read/write
│   ├── memories.ts         # Memory parser
│   └── agents.ts           # agents_state.json reader
├── SPEC.md
├── BUILD_PLAN.md
└── README.md
```
