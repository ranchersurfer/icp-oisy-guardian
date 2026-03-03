# Dream Cycle Documentation

## What is the Dream Cycle?

**Dream cycle** = automated nightly memory consolidation task.

**When:** Daily at 1 AM (runs in background, doesn't interrupt your work)
**What it does:**
1. Reads today's session log (`memory/YYYY-MM-DD.md`)
2. Extracts high-signal facts, decisions, lessons, patterns
3. Updates relevant Tier 2 topic files (memory/projects.md, memory/preferences.md, etc.)
4. Trims any bloated Tier 1 files if needed
5. Writes morning summary for tomorrow

**Why:** Without automated consolidation, memories get lost when sessions end. Dream cycle ensures important stuff gets saved to durable files.

---

## Setup Status

✅ **Scripts created & tested**
⏳ **Cron deployment:** Choose one method below

- Script: `/home/ranch/.openclaw/workspace/scripts/dream-cycle.sh` (tested, working)
- Logs: `/home/ranch/.openclaw/workspace/logs/dream-cycle.log`
- Model: Sonnet 4.6 (for complex reasoning)

**Deployment Status:**
- Script handles both local API key execution and fallback instructions
- Cron entry in system crontab (created but needs OpenClaw session context)
- Recommended: Use OpenClaw's built-in `openclaw cron` feature instead

---

## Deployment Options

### Option 1: OpenClaw Built-in Cron (Recommended)

From your main session:

```bash
openclaw cron add \
  --name dream-cycle \
  --schedule "0 1 * * *" \
  --message "Consolidate daily memory (date: $(date +%Y-%m-%d)). 

Steps:
1. If memory/$(date +%Y-%m-%d).md exists, read it thoroughly
2. Extract high-signal insights: facts, decisions, patterns, lessons
3. Update relevant Tier 2 files in memory/
4. If Tier 1 files grown >3KB, trim redundant content
5. Write morning summary to memory/morning-brief.md"
```

Then verify:
```bash
openclaw cron list
```

### Option 2: Shell Cron with Script (Current)

Already installed. To verify:
```bash
crontab -l
```

If system crontab is installed, it will run automatically at 1 AM.

### Option 3: Manual (For Testing)

From main session:
```bash
/home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

Or paste this directly to the agent:
> "Consolidate daily memory. Read memory/2026-02-28.md, extract insights, update memory/ topic files, trim Tier 1 if >3KB, write morning summary."

---

## How It Works

### The Script

```bash
/home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

**Steps:**
1. Gets today's date
2. Constructs a detailed task for the OpenClaw agent
3. Invokes agent with Sonnet 4.6 (better reasoning for complex consolidation)
4. Logs output to dream-cycle.log

### The Cron Entry

```
0 1 * * * /home/ranch/.openclaw/workspace/scripts/dream-cycle.sh >> /home/ranch/.openclaw/workspace/logs/dream-cycle.log 2>&1
```

Translates to: **Every day at 1:00 AM**, run the script and append logs.

### The Agent Task

When dream cycle runs, the agent receives this prompt:

> "Consolidate daily memory (date: 2026-02-28). 
> 
> 1. Read memory/2026-02-28.md thoroughly
> 2. Extract high-signal insights: facts, decisions, patterns, lessons
> 3. Update relevant Tier 2 topic files (projects.md, preferences.md, security.md, etc.)
> 4. Trim any Tier 1 files if grown >3KB
> 5. Write brief morning summary (3-5 lines) in memory/morning-brief.md"

---

## Monitoring & Troubleshooting

### Check if Cron is Running

```bash
# List scheduled jobs
crontab -l

# Check if cron daemon is active
sudo systemctl status cron
# or
sudo service cron status
```

### View Dream Cycle Logs

```bash
# Latest execution
tail -f /home/ranch/.openclaw/workspace/logs/dream-cycle.log

# Last 20 lines
tail -20 /home/ranch/.openclaw/workspace/logs/dream-cycle.log

# Search for errors
grep -i "error" /home/ranch/.openclaw/workspace/logs/dream-cycle.log
```

### Test Manually (Don't Wait for 1 AM)

```bash
/home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

This runs immediately and outputs to stdout + logs/dream-cycle.log.

### Verify OpenClaw Can Be Invoked from Cron

```bash
# Test from cron context (minimal env)
env -i HOME=$HOME /home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

If this fails, the cron job will fail. Common issue: PATH not set in cron, so `openclaw` can't be found.

**Fix:** Update `dream-cycle.sh` to use full path:

```bash
# Instead of:
openclaw invoke main ...

# Use:
/usr/local/bin/openclaw invoke main ...
# or find the path:
which openclaw
```

---

## Customization

### Change Time (Currently 1 AM)

Edit crontab:
```bash
crontab -e
```

Change the first two fields of:
```
0 1 * * * /home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

Examples:
- `30 2 * * *` = 2:30 AM
- `0 6 * * *` = 6:00 AM (morning, right after waking)
- `0 0 * * *` = Midnight
- `0 */6 * * *` = Every 6 hours

### Change Model (Currently Sonnet)

Edit `scripts/dream-cycle.sh`, find this line:

```bash
--model anthropic/claude-sonnet-4-6 \
```

Change to:
- `anthropic/claude-opus-4-6` (more powerful, slower, more expensive)
- `anthropic/claude-haiku-4-5-20251001` (cheaper, faster, less powerful)
- `ollama/llama3.2` (if you want local inference, requires QMD CLI)

### Change Task Details

Edit `scripts/dream-cycle.sh`, modify the `DREAM_TASK` variable.

---

## Expected Behavior

### First Run (Nightly at 1 AM)

Cron executes the script:
1. Captures today's date
2. Invokes OpenClaw with Sonnet
3. Agent reads memory/2026-02-28.md
4. Agent extracts insights and updates topic files
5. Agent writes morning brief
6. Logs output to dream-cycle.log

### Result

- New entries in memory/projects.md, memory/preferences.md, etc. (if applicable)
- New file: memory/morning-brief.md (or updated if exists)
- Log entry in logs/dream-cycle.log with completion status

---

## When to Disable Dream Cycle

You might want to turn it off if:

1. **Testing memory features** — Don't want automated changes interfering
2. **Manual memory edits** — You're actively reorganizing memory files
3. **Debugging** — Need to inspect logs without new consolidations happening

**Disable temporarily:**
```bash
crontab -e
# Comment out the line:
# 0 1 * * * /home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

**Re-enable:**
```bash
crontab -e
# Uncomment the line
0 1 * * * /home/ranch/.openclaw/workspace/scripts/dream-cycle.sh
```

---

## Integration with QMD & Hybrid Search

Dream cycle updates Tier 2 memory files, which are automatically indexed by QMD.

When you use `memory_search()` in subsequent sessions:
1. QMD searches the updated files
2. Hybrid search finds both keyword matches + semantic similarity
3. MMR ensures results aren't redundant
4. Temporal decay weights recent consolidations higher

**Example workflow:**
1. Work through the day (session logs to memory/2026-02-28.md)
2. 1 AM: Dream cycle consolidates → updates memory/projects.md
3. Next morning: `memory_search("project status")` retrieves updated project info

---

## Last Updated

**2026-02-28:** Dream cycle deployed with Sonnet 4.6, cron at 1 AM

Next: Monitor first execution (March 1, 1 AM) and validate consolidation quality.
