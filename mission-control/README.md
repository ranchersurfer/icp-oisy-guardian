# Mission Control Dashboard

A local web dashboard for monitoring and managing OpenClaw agents, tasks, projects, memories, and docs.

## What It Is

A Next.js app that reads directly from your `~/.openclaw/workspace/` files and presents them in a Linear-style UI. No cloud, no auth, no database required.

## Screens

| Screen | Purpose |
|---|---|
| Task Board | Kanban for agent + human tasks, live activity feed |
| Calendar | Cron jobs and scheduled tasks |
| Projects | Project cards with linked tasks/docs/memories |
| Memories | Day-by-day timeline of memory files |
| Docs | Searchable library of workspace markdown |
| Team | Agent org chart from agents_state.json |

## Quick Start

```bash
cd mission-control
npx create-next-app@latest . --app --tailwind --typescript
npm run dev
# Open http://localhost:3000
```

## Docs

- [SPEC.md](./SPEC.md) — Full product spec (screens, data models, UI behavior)
- [BUILD_PLAN.md](./BUILD_PLAN.md) — Phased build plan, tech decisions, risks

## Status

🚧 Pre-build — spec and plan complete, implementation not started.
