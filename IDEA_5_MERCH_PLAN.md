# IDEA #5: Custom AI Merch & Print-on-Demand — Detailed Execution Plan

**Timeline:** 4 weeks (Feb 27 - Mar 27, 2026)  
**Total effort:** 30 hours (18.75% of weekly capacity)  
**Revenue target Month 1:** $100-300  
**Concept:** AI-generated designs sold via Etsy + Printful, seeding audience for downstream products (horror content, games, etc.)  

---

## Why This Idea? (Strategic Rationale)

### Lowest Barrier to Entry
- Minimal upfront capital: $0-50 platform fees (Etsy listing creation)
- No inventory risk: Print-on-demand ships directly from supplier
- Time-to-market: 2 weeks from idea to first sale (realistic)

### Audience Synergy
- Merch buyers = potential horror content viewers, Android app users, AI agent customers
- Merchandise is a trust signal: "Real business, not just ideas"
- Email list opportunity: Every merch order = customer contact

### Skill-Building Opportunity
- Designs can be templated and batched via OpenClaw
- Platform APIs (Etsy, Printful) are straightforward
- Machine learning angle: Use customer feedback to improve AI prompts

### Revenue Profile
- **Month 1:** $100-300 (realistic for cold launch)
- **Month 3:** $500-2K (if design-market fit found + minor promotion)
- **Month 6+:** $2-5K/mo (with 20+ designs, cross-promotion to audiences)
- **Unit economics:** ~$10-15 gross profit per shirt sold

---

## Week 1: Supplier & Platform Research (8 hours)

### Day 1-2: Platform Evaluation (2 hours)

#### Option A: Printful + Etsy API
**Pros:**
- Highest margins (~50-60% after printing + shipping)
- API-driven automation possible
- Direct integration with Etsy inventory

**Cons:**
- Requires API integration (1-2 hours dev)
- Per-order fulfillment (not inventory-based)
- Minimum API docs/learning curve

**Timing:** Setup day 2-3, go live week 2

#### Option B: Redbubble (No Integration)
**Pros:**
- Zero setup cost, immediate start
- Redbubble handles all marketing/SEO
- Automatic payment processing

**Cons:**
- Lower margins (10-15% royalty per shirt, $18-25 retail)
- No API access to automate
- Redbubble owns customer relationship

**Timing:** Setup same day, go live same day

#### Option C: Teespring/Spring
**Pros:**
- Campaign-based model (collect pre-orders, then print)
- No inventory/shipping headaches

**Cons:**
- Requires audience to drive pre-orders
- Slower cash flow (pay after campaign ends)
- Cold-start difficult without existing followers

**Timing:** Not recommended for Month 1 (need audience first)

**RECOMMENDATION:** **Printful + Etsy** for Month 1 (prioritize margin + automation potential), with Redbubble as fallback if Printful integration stalls.

### Day 2: Competitive Analysis (1 hour)
Research top-performing AI merch on Etsy:

**Search queries to investigate:**
- "AI generated art"
- "horror aesthetic t-shirt"
- "crypto bitcoin shirt"
- "tech startup merch"
- "psychedelic art apparel"

**Data points to collect (per design):**
- Listing price ($15-50)
- Estimated sales (based on reviews, if visible)
- Design style (minimalist vs. detailed)
- Niche focus (horror, crypto, tech, meme culture)
- Product types (t-shirt, hoodie, mug, sticker)
- Thumbnail quality (high contrast = higher CTR)

**Tool:** Use Etsy search, Ahrefs/SimilarWeb for traffic estimation, or manual count of reviews.

**Takeaway:** Identify 2-3 winning design styles + niches to emulate.

### Day 3: Decision & Account Setup (2 hours)

**Decision point:** Commit to Printful + Etsy (Plan A) by end of day 3.

**Setup tasks:**
- [ ] Create Etsy seller account (free, ~15 min)
- [ ] Generate Etsy API key and secret
- [ ] Create Printful account (free, ~15 min)
- [ ] Link Etsy shop to Printful
- [ ] Upload 1 test design (logo or simple graphic) to validate flow
- [ ] Test order end-to-end (place mock order, verify Printful receives it)

**Deliverable:** Etsy shop URL + Printful API credentials documented (securely stored in workspace)

### Day 4-5: Integration Testing & Fallback Planning (2 hours)

**Integration scope:**
- Etsy shop can pull from Printful inventory
- Printful auto-fulfills orders placed on Etsy
- Shipping cost calculated correctly

**If integration fails:**
- Document blocker (e.g., "Etsy API rate limiting," "Printful onboarding delayed")
- Fallback: Upload designs to Redbubble manually (still works, less automation, smaller margins)

**Success criteria:** 1 test design live on both Etsy + Printful, fully integrated.

---

## Week 2: Design Pipeline & Content Generation (10 hours)

### Strategic Design Plan

**Merch themes to explore:**
1. **Horror + AI mashup** (aligned with #14 Horror Content)
   - "AI Generated Nightmares" typography
   - Creepy AI-art aesthetic (glitch/digital horror)
   - Example: Distorted faces, uncanny valley robots, surreal landscapes
   
2. **Crypto + Humor** (aligned with Moises's ICP/Bitcoin interest)
   - "hodl forever" minimal designs
   - Bitcoin symbol reimagined (art deco, retro, cyberpunk styles)
   - "Stack sats" and other industry memes
   
3. **Tech culture** (general appeal)
   - "console.log('just vibing')" minimal tech humor
   - Neural network diagrams
   - Minimal AI/ML iconography

**Design count target:** 10 designs (split across 3 themes)
- Horror: 4 designs
- Crypto: 3 designs
- Tech: 3 designs

### Day 1: Design Brief & Prompt Engineering (2 hours)

**Output:** Design brief document (shared with art generator)

**For each theme, define:**
- Visual style (color palette, composition, line weight)
- Target audience persona
- Mood/tone
- Example reference images (Pinterest, existing merch)

**Example: Horror Theme**
```
Style: Digital glitch + surrealism
Colors: Deep blacks, sickly greens, blood reds
Composition: Centered focal point, asymmetric balance
Mood: Unsettling but visually appealing (not gratuitously violent)
Audience: Horror fans, 18-35, alternative fashion sense
References: Junji Ito aesthetic, synth-horror, VHS tape artifacts
```

**Prompt examples (for Midjourney/Caffeine):**
```
"AI generated horror artwork, glitch effect, digital nightmares, 
dark aesthetic, trending on artstation, 8k, cinematic lighting, 
fear-inducing but beautiful,--ar 1:1 --niji --style raw"

"Bitcoin symbol reimagined as art deco sculpture, gold and black, 
geometric shapes, luxury aesthetic, 3d render, professional product photo, 
--ar 1:1"
```

### Day 2-3: AI Art Generation (4 hours)

**Tools:**
- **Primary:** Midjourney ($10-20/month, high quality, fast)
- **Fallback:** Caffeine.AI (since Moises uses it)
- **Free option:** Stable Diffusion (self-hosted, lower quality)

**Workflow:**
1. Batch 5 prompts per theme
2. Generate 3 variations per prompt (~15 candidates per theme)
3. Select best 4-5 per theme (total ~12-15 candidates)
4. Export high-res versions (4K if possible)

**Quality checks:**
- No blurry text or deformed objects
- Colors vibrant and on-brand
- Composition balanced and eye-catching
- Unique enough to not feel AI-generic

### Day 4: Mockup Generation & T-Shirt Preview (2 hours)

**Create visual mockups:** Generate product previews showing designs on actual t-shirt mockups.

**Tools:**
- Printful has built-in mockup generator (free with account)
- Canva templates (if need quick backup)
- Photoshop (if have time for perfection)

**For each design:**
- Generate 2-3 mockups (t-shirt, hoodie, mug if applicable)
- Show color variations (black tee, white tee, heather grey)
- Export for Etsy listing

**Deliverable:** 12-15 mockup images ready for Etsy uploads

### Day 5: Selection & Quality Review (2 hours)

**Final curation:** Select 8-10 designs for launch.

**Criteria:**
- Unique and visually distinct from each other
- No duplicates or near-duplicates
- Professional mockup quality
- Aligned with brand
- At least 3 designs represent each theme

**Decisions to make:**
- Which designs go live Week 3 (5-8)?
- Which are backup/reserve (2-3 for second wave)?
- Pricing per design (all $22-28 or vary by complexity)?

**Deliverable:** Final design selection + mockup images + pricing strategy

---

## Week 3: Etsy Launch & Fulfillment Setup (8 hours)

### Day 1-2: Etsy Shop Listing Creation (3 hours)

**For each design, create Etsy listing:**

**Required fields:**
- **Product title** (SEO-critical):
  - Format: "[Adjective] [Product] [Niche] - AI Generated Design"
  - Example: "Haunting Digital Horror T-Shirt - AI Art"
  - Include keywords: "horror," "AI generated," "gothic," "apparel"

- **Product description** (150-300 words):
  - Pitch the story: "AI-generated original design by [artist name]"
  - Material specs: "100% cotton, pre-shrunk, comfortable fit"
  - Sizing info: "Unisex, runs true to size"
  - Care instructions: "Machine wash cold"
  - Shipping info: "Ships from print partner in [location]"

- **Tags** (13 max, all important):
  - "horror art," "AI generated," "gothic shirt," "apparel," "cryptocurrency," "tech shirt," etc.
  - Think like customer: What words would YOU search for?

- **Category:** Apparel → T-Shirts (or Hoodies, depending)

- **Pricing:**
  - Cost (from Printful): ~$10-15 per shirt
  - Markup: $8-12 profit
  - **Retail price: $22-28 per shirt** (competitive, not overpriced)

- **Shipping:** Managed by Printful (automatic)

- **Variations:** Offer S/M/L/XL/2XL; offer 2-3 colors per design

**Tool:** Etsy Editor or bulk upload via CSV (if multiple listings)

### Day 2-3: Printful Integration (2 hours)

**Setup fulfillment:**
- [ ] Verify Printful account linked to Etsy shop
- [ ] Enable automatic order syncing (Etsy → Printful)
- [ ] Test 1 order end-to-end:
  - Buy own product from Etsy
  - Verify order appears in Printful within 5 min
  - Verify Printful can auto-print and ship
  - Cancel test order and refund

**Pricing accuracy:**
- Confirm Etsy shipping cost matches Printful base price + markup
- Verify no double-charging or missing fees

**Contingency:** If integration fails, enable manual order routing (monitor Etsy for orders, manually create in Printful).

### Day 3: Shop Optimization (1 hour)

**Etsy shop appearance:**
- [ ] Upload shop banner (500x160px) with branding
- [ ] Write shop description (150 words): "Welcome to [Shop Name]. AI-generated designs, hand-printed merch. Free shipping on orders >$50."
- [ ] Add shop policies: Refund policy, shipping, custom order inquiries
- [ ] Create shop announcement (featured product or sale)

### Day 4: Public Launch (1 hour)

**Publishing strategy:**
- [ ] Mark all 5-8 designs as "Public" (not drafts)
- [ ] Verify listings appear in Etsy search
- [ ] Share shop link on personal channels:
  - Discord server (relevant communities)
  - Reddit (r/Etsy, r/shopify, niche subreddits like r/horror, r/cryptocurrency)
  - Twitter/X (teaser tweet with shop link)
- [ ] Solicit feedback from 2-3 trusted friends

**Deliverable:** Etsy shop with 5-8 live listings

### Day 5: Monitoring & Quick Fixes (1 hour)

**First-week maintenance:**
- [ ] Monitor Etsy dashboard for orders
- [ ] Check Printful for fulfillment issues
- [ ] Respond to any customer questions (if any)
- [ ] Fix typos or broken mockups if spotted

---

## Week 4: Iteration & Automation Roadmap (4 hours)

### Day 1-2: Early Performance Analysis (1 hour)

**If sales > 0:**
- Analyze which designs sold
- Check customer feedback in reviews
- Understand which keywords brought traffic

**If sales = 0 (expected):**
- Don't panic; this is a learning phase
- Check traffic metrics (Etsy shop stats: views, clicks)
- Identify which designs got most impressions

**Key metrics to track:**
- Views per listing
- Click-through rate (CTR)
- Conversion rate (clicks → purchases)
- Average order value

**Pivot decision:** If clear loser (0 views), consider replacing with variant design.

### Day 2: Design Refinement (1 hour)

**Second wave planning:**
- Generate 5 new designs (different themes or improvements)
- Based on early customer feedback or design trends
- Aim for publication in Week 1 of Month 2

### Day 3-4: OpenClaw Automation Skill Roadmap (2 hours)

**Planned skill:** "AI Merch Design Generator + Etsy Uploader"

**What the skill does:**
```
INPUT:
- Design brief (theme, style, target audience)
- Number of variations (e.g., 5)

PROCESS:
1. Generate prompt variants (Midjourney/Caffeine)
2. Call AI art generator API
3. Retrieve generated images
4. Create mockups (Printful API)
5. Generate Etsy metadata (title, description, tags, SEO)

OUTPUT:
- 5 mockup images
- Etsy listing draft (ready for copy-paste)
- Suggested pricing
- SEO-optimized title + tags
```

**Implementation timeline:**
- Month 2: Skeleton (Midjourney integration)
- Month 3: Full automation (Etsy API upload)
- Month 4: Scheduling + A/B testing (test 2 design variants per week)

**Expected ROI:**
- Manual design: 1 design/day (10 hours/week)
- Automated design: 10 designs/week (2 hours/week setup + monitoring)
- Time saved: ~8 hours/week → could redirect to marketing or other business ideas

### Day 5: Month 2 Planning (0 hours, covered above)

---

## Revenue Model & Month 1 Targets

### Unit Economics

| Metric | Amount |
|--------|--------|
| Printful base cost per shirt | $10-14 |
| Etsy listing fee | $0.20 per listing (1-time) |
| Etsy transaction fee | 6.5% of sale price |
| Etsy payment processing | 3% + $0.20 per transaction |
| **Total fees per $25 sale** | ~$2.50 (10%) |
| **Net profit per shirt** | $25 - $12 (cost) - $2.50 (fees) = **$10.50** |

### Realistic Revenue Projections

**Month 1 (Cold Launch):**
- Expected sales: 3-10 units
- Realistic revenue: **$75-250**
- Target: **$100-300** (accounting for lucky viral hit)

**Month 2 (Organic Growth + PR):**
- Expected sales: 10-30 units (if first design converts)
- Realistic revenue: **$100-750**
- Stretch goal: **$500** (with minor promotion)

**Month 3+ (Established):**
- 20+ designs in catalog
- Cross-promotion via horror content + social channels
- Expected sales: 30-100+ units/month
- Realistic revenue: **$300-2,500/month**

**Payback period:** Immediate (no upfront capital), but break-even on time investment happens Month 2-3.

---

## Risks & Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Designs don't sell | High | Low ($0 revenue, but 30 hours invested) | Validate 2-3 designs with beta users Week 1 before full launch |
| Printful integration fails | Medium | Medium (fallback to manual Redbubble) | Have Redbubble account ready as 24-hour pivot |
| Etsy suspends shop (TOS violation) | Low | High (account banned) | Ensure designs are original, no copyright, clear AI attribution |
| Customer complaints (quality/design) | Medium | Low (easy refund, improve future) | Use high-quality mockups, manage expectations |
| No traffic to shop | High | Low (expected, focus on organic SEO) | Optimize titles/tags, leverage Discord/Reddit communities |
| Profitability too low | Medium | Low (lean model, worth learning) | Data shows micro-margins OK for hobby; scale via volume + variety |

---

## Success Metrics (Month 1)

| Metric | Target | How to Measure |
|--------|--------|-----------------|
| Shop live | 1 Etsy shop | Etsy shop URL created and public |
| Designs published | 5-8 live listings | Count public listings on Etsy |
| Minimum revenue | $100 | Etsy dashboard → Total sales |
| Orders processed | 3-10 | Count fulfilled orders in Printful |
| Customer feedback | 3-5 reviews | Etsy review section |
| Design variants generated | 8-10 | Count final mockup images |
| Automation spec drafted | 1 document | OpenClaw skill outline created |
| SEO traction | Top 50 for 1 keyword | Etsy shop views > 50, identify top-traffic keywords |

---

## Dependencies & Assumptions

**Assumptions:**
- Etsy account creation is instant (1-2 hours)
- Printful API integration works within 2 hours (or fallback to manual)
- AI art generation quality is acceptable first-time (Midjourney/Caffeine)
- Customer demand exists for AI-generated horror/crypto merch (low-risk, proven niche)

**Dependencies:**
- Etsy API access (may require verification, but should work for new sellers)
- Printful account verification (usually instant)
- API key security (keep in workspace, not in git)
- Design tools availability (Midjourney, Caffeine, or local Stable Diffusion)

---

## Key Files & Locations

- **Designs folder:** `/home/node/.openclaw/workspace/merch/designs/` (locally manage mockups)
- **Etsy credentials:** Environment variable `ETSY_API_KEY` + `ETSY_SHOP_ID` (in OpenClaw config, not git)
- **Printful credentials:** Environment variable `PRINTFUL_API_KEY` (same)
- **Design briefs:** `/home/node/.openclaw/workspace/merch/design-briefs.md` (document AI prompts)
- **Launch checklist:** `/home/node/.openclaw/workspace/merch/launch-checklist.md` (track status)

---

## Decision Log

| Decision | Date | Rationale | Owner |
|----------|------|-----------|-------|
| Printful + Etsy (vs Redbubble) | Feb 27 | Higher margins + API automation opportunity | Strategy-Planner |
| Horror + Crypto + Tech themes | Feb 27 | Aligned with Moises's interests + audience | Strategy-Planner |
| 10 designs per Week 2 | Feb 27 | Batch efficiency + variety testing | Strategy-Planner |
| Launch Week 3 (not Week 2) | Feb 27 | Allows time for design refinement | Strategy-Planner |

---

## Document History

- **Created:** Feb 27, 2026 (Strategy-Planner subagent)
- **Last updated:** Feb 27, 2026
- **Owner:** Moises (subject to weekly iteration)
