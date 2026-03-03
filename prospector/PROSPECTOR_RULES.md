# Prospector Rules — Lead Generation for AI Automation Agency

**Agent:** Prospector  
**Role:** Find qualified leads, assess fit, draft personalized proposals  
**Execution:** Automated (every 6 hours via cron)  
**Output:** Ranked opportunity list + draft proposals for human approval

---

## Ideal Client Profile (ICP)

### Target Companies
- **Revenue:** $500K–$5M annually (SMBs and growing agencies)
- **Industries:** E-commerce, agencies (digital/marketing/design), SaaS, logistics, customer service ops
- **Pain Points:** Manual data entry, repetitive workflows, social media management, lead qualification, invoice processing, email triage
- **Budget:** $500–$5K/month for automation services
- **Decision Speed:** SMBs move fast; typical sales cycle 2–4 weeks

### Disqualify If:
- Requires custom compliance work (healthcare PHI, financial regulated services without explicit legal review)
- Enterprise contracts (>$1M revenue, procurement cycles >90 days)
- Prefers agency retainer models only (we want discrete projects)
- No clear automation opportunity (e.g., fully manual is a feature, not a bug)

---

## Search Keywords & Platforms

### Upwork (Priority: High)
**Search terms (every 6 hours):**
- "AI automation"
- "Computer use automation"
- "Workflow automation"
- "Data entry automation"
- "Social media management"
- "Lead generation automation"
- "Email automation"
- "Invoice processing"

**Filters:**
- Budget: $500–$10K (first project)
- Client history: Verified payment (excludes tire-kickers)
- Proposals: <100 (less saturated)
- Rating: 3.0+ (not broken)

### LinkedIn (Priority: Medium)
**Search for:**
- Hiring posts from target industries mentioning "automation", "efficiency", "scaling"
- Company pages in e-commerce, SaaS, marketing with <200 employees
- Decision makers: Ops leads, process improvement, VP/Director roles

**Outreach cadence:** 5–10 cold messages/week (quality > volume)

### Cold Email (Priority: Medium)
**Sources:**
- LinkedIn Sales Navigator exports (target list)
- Upwork leads that have been non-responsive >7 days
- Directory searches by industry + location

**Templates:** See OUTREACH_TEMPLATES/

---

## Service Tiers & Pricing

### Tier 1: Quick Win ($500–$1500)
**Time:** 5–15 hours  
**Scope:** Single workflow automation (e.g., lead capture → CRM, email cleanup, invoice scanning)  
**Pitch:** "We'll automate one of your manual processes. See the ROI in 30 days."  
**Use for:** First-time buyers, proof of concept, quick revenue

### Tier 2: Process Stack ($2K–$5K)
**Time:** 20–40 hours  
**Scope:** 2–3 connected workflows (e.g., lead gen → qualification → CRM → email)  
**Pitch:** "Let's automate your entire customer intake pipeline."  
**Use for:** Established clients, higher budget, deeper relationships

### Tier 3: Full Agency Stack ($5K–$15K)
**Time:** 60–120 hours  
**Scope:** Entire department workflows (e.g., all social media management + scheduling + analytics)  
**Pitch:** "Become 10x more efficient. Your team manages, AI executes."  
**Use for:** Agencies, SaaS ops, mature businesses ready to scale

### Pricing Notes:
- All quotes assume 4-week turnaround
- Rush (+50% add-on for 2-week delivery)
- Monthly retainer post-project: 10–15% of first project cost (optional)

---

## Proposal Template Structure

Every draft proposal should follow this format (see OUTREACH_TEMPLATES/ for examples):

1. **Opening:** Acknowledge their specific challenge (reference their job posting or LinkedIn post)
2. **Solution:** Describe the workflow we'll automate in 2–3 sentences
3. **Outcome:** Quantify expected time saved or quality improvement
4. **Our Experience:** 1–2 sentence proof point (OpenClaw automation, cost savings, etc.)
5. **Next Step:** "Let's schedule a 15-minute call to map your workflow"
6. **Closing:** Casual, confident, no hype

**Length:** Max 150 words (email) or 300 words (proposal document)

---

## Lead Scoring Rubric

When Prospector finds a lead, rank it 1–10 based on:

| Factor | Scoring |
|--------|---------|
| **ICP Fit** | Perfect (3) / Good (2) / Loose (1) |
| **Stated Budget** | $2K+ (3) / $1K–$2K (2) / <$1K (1) |
| **Urgency** | "ASAP" or "Immediate" (3) / "This month" (2) / Flexible (1) |
| **Automation Readiness** | Clear pain point + measurable (3) / Vague but interested (2) / Exploratory (1) |
| **Competition** | <50 proposals (3) / 50–150 (2) / 150+ (1) |

**Rank:** Sum score (1–10). Prioritize 8–10 first.

---

## Rejection Criteria

Do NOT propose to:
- Crypto trading bots (regulatory gray area without legal review)
- Wage arbitrage schemes (misleading labor practices)
- Lead gen for MLMs or pyramid schemes
- Anything requiring API keys / credentials to be stored insecurely
- Clients with negative payment history or bad feedback

---

## Workflow: Lead → Draft → Human Approval

```
1. Prospector scans job boards every 6 hours
2. Filters by ICP + budget + competition
3. Ranks opportunities (1-10)
4. Drafts personalized proposal (specific to their pain point)
5. Logs in PROSPECT_PIPELINE.md:
   - Job ID, client name, budget
   - Match score
   - Draft proposal (for Moises to review)
   - Recommended Tier
6. Status: "DRAFT_READY" (awaiting human approval)
7. Moises reviews, approves or rejects, then sends
8. Prospector updates status: "SENT" → "REPLIED" → "NEGOTIATING" → "CLOSED"
```

---

## Success Metrics

Track weekly:
- **Opportunities found:** Target 20–30/week (post-filter)
- **Match score (avg):** Target 7+ (high quality)
- **Proposal approval rate:** Target 70%+ (Moises approves proposals)
- **Response rate:** Target 20–30% of sent proposals
- **Close rate:** Target 30–50% of responses = 2–5 closed deals/month
- **Average deal value:** Target $2K–$5K (Tier 2)

---

## Files & References

- `PROSPECT_PIPELINE.md` — Active leads with status
- `OUTREACH_TEMPLATES/` — 3–5 proposal templates by service type
- `library/` — Past successful proposals (learn from wins)
- Upwork saved searches (auto-updated)

---

**Last Updated:** 2026-02-28  
**Next Review:** After first 2 weeks of automated scanning
