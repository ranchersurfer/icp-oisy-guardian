#!/bin/bash
# Dream Cycle: Nightly memory consolidation
# Note: Must be run from Moises's main OpenClaw session context to invoke agent
# Use: openclaw cron add or run from main session

set -euo pipefail

DATE_TODAY=$(date +%Y-%m-%d)
LOG_FILE="/home/ranch/.openclaw/workspace/memory/${DATE_TODAY}.md"

# Task for OpenClaw agent (Sonnet 4.6 for complex reasoning)
DREAM_TASK="Consolidate daily memory (date: ${DATE_TODAY}). 

Steps:
1. If memory/${DATE_TODAY}.md exists, read it thoroughly
2. Extract high-signal insights: facts learned, decisions made, patterns observed, lessons from mistakes
3. Update relevant Tier 2 topic files in memory/ (projects.md, preferences.md, security.md, etc.) with new information
4. If any Tier 1 file (SOUL.md, USER.md, MEMORY.md) has grown >3KB, review and trim redundant content
5. Write a brief morning summary (3-5 lines max) and append to memory/morning-brief.md

Output: Confirmed completion with count of updates made."

echo "[$(date +'%Y-%m-%d %H:%M:%S')] Dream cycle starting..."
echo "Date: ${DATE_TODAY}"
echo "Log file: ${LOG_FILE}"
echo ""

# Method 1: If you have API credentials in env, use --local
if [ -n "${ANTHROPIC_API_KEY:-}" ]; then
  echo "Running with local execution (API key detected)..."
  openclaw agent --message "${DREAM_TASK}" --local --json 2>&1 || {
    echo "[ERROR] Local execution failed. Try Method 2 instead."
    exit 1
  }
else
  echo "[INFO] No ANTHROPIC_API_KEY in environment. To run:"
  echo "  1. From Moises's main session: Copy this task and send it as a message"
  echo "  2. Or set up via: openclaw cron add --name dream-cycle --schedule '0 1 * * *' --message '...'"
  echo ""
  echo "Task to send:"
  echo "${DREAM_TASK}"
  exit 0
fi

echo ""
echo "[$(date +'%Y-%m-%d %H:%M:%S')] Dream cycle complete."
