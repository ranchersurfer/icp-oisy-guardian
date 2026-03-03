# HEARTBEAT.md

## Guardian-Dev Watchdog

Check if Guardian-Dev should be spawned for the next phase.

### How to check:
1. Read `/home/ranch/.openclaw/workspace/agent-status.json`
2. If `guardian-dev.status` is `"idle"`:
   - Read `/home/ranch/.openclaw/workspace/guardian-dev/DEV_LOG.md` to find the last completed phase
   - Check which phase comes next based on this order:
     - Phase 1a (Config hardening) → Phase 1b (Engine skeleton) → Phase 1c (ICRC integration) → Phase 1d (Detection engine) → Phase 1e (Testing + local deploy)
   - If a next phase exists and is not yet complete → spawn Guardian-Dev for that phase
   - Use the task descriptions and acceptance criteria from `/home/ranch/.openclaw/workspace/guardian-dev/DEV_PLAN.md`
3. If `guardian-dev.status` is `"working"` → do nothing, he's already on it

### Spawn instructions:
- runtime: subagent, mode: run, label: guardian-dev-phase1X
- Always include: read DEV_PLAN.md + DEV_LOG.md + current lib.rs before starting
- Always end task by:
  1. Updating `/home/ranch/.openclaw/workspace/agent-status.json` (set status back to "idle" when done)
  2. Updating `/home/ranch/.openclaw/workspace/projects.json` — set `proj-guardian` progress:
     - After 1a: 20%, After 1b: 40%, After 1c: 60%, After 1d: 80%, After 1e: 100% + status "done"
  3. Updating `/home/ranch/.openclaw/workspace/GUARDIAN_FILE_STRUCTURE.md` — reflect actual file tree (`find src -type f | sort`) + phase status table
  4. Updating `/home/ranch/.openclaw/workspace/guardian-dev/DEV_LOG.md` with what was done
  5. Updating `/home/ranch/.openclaw/workspace/GUARDIAN_LAUNCH.md` — current phase, completion %, next milestone
  6. Updating `/home/ranch/.openclaw/workspace/tasks.json` — mark completed tasks as "done" (match by assignee: guardian-dev + title keywords)
  7. Updating `/home/ranch/.openclaw/workspace/PHASE_1_ROADMAP.md` — tick off completed phases
  8. Committing all changes (including doc updates) and pushing to origin main

### After spawning:
- Update `agent-status.json`: set guardian-dev status to "working"
- Note in your heartbeat reply what phase was kicked off

### When all phases done (1e complete):
- Set guardian-dev status to "idle" in agent-status.json
- Note "Guardian Phase 1 MVP complete — awaiting Phase 2 planning" in heartbeat reply
- Do NOT spawn further without explicit instruction from Moises

---

## Dream Cycle — After Each Run (nightly 1 AM)
After consolidating memory, also update:
1. `schedule.json` — set `last_run_at` for the dream-cycle entry to current ISO timestamp
2. If it's Sunday — regenerate `mission_control_dashboard.md` with current agent statuses, phase progress, cost estimates from memory files

---

## Other checks (rotate, 2-4x per day):
- Any urgent emails or calendar events coming up?
- Dream cycle ran last night? (check memory/YYYY-MM-DD.md for today's date)
- `tasks.json` — if any tasks are assigned to an agent that just went idle, check if they should be moved to "done"
