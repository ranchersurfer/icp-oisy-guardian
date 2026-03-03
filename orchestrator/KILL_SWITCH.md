# KILL_SWITCH

## Emergency Pause Protocol

If Moises says "PAUSE ALL AGENTS", immediately:
1. Set `GLOBAL_STATUS=PAUSED` below
2. Notify all sub-agents via agentToAgent: "STOP all execution. Global pause in effect."
3. Telemetry-Monitor: Stop monitoring; do not execute pauses.
4. All money-touching agents: Check this file before any trade/withdrawal.

## Current Status
GLOBAL_STATUS=ACTIVE

## Last Activated
(none)

## Manual Override (Moises Only)
To pause: Replace ACTIVE with PAUSED above, save, and message Orchestrator "execute kill switch"
To resume: Replace PAUSED with ACTIVE and message Orchestrator "resume operations"

## Last Updated
2026-02-27 UTC
