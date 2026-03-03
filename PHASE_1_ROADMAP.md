# Phase 1 Execution Roadmap — 5 Core Business Ideas (4-Week Sprint)

**Planning Period:** Week 1-4 (Feb 27 - Mar 27, 2026)  
**Available Capacity:** ~40 hours/week (Moises)  
**Orchestration:** OpenClaw + multi-agent system  

---

## Executive Summary

This roadmap sequences 5 concurrent business ideas across 4 weeks, prioritizing quick revenue (#5 Merch, #14 Horror Content) alongside long-term strategic foundations (#13 AI Agent Products). Total effort target: 160 hours allocated across 5 ideas, 5 days/week.

**Quick Wins (Weeks 1-2):**
- #5 Merch: Platform research → design pipeline → Etsy launch
- #14 Horror Content: Script drafting → production planning

**Strategic Builds (Weeks 1-4):**
- #13 AI Agent Products: Spec → alpha → testnet (ckBTC DCA Agent focus)
- #7 YouTube: Content funnel architecture + upload pipeline
- #12 Android: MVP scope clarification + tech stack decision

**Month-End Targets:**
- #5 Merch: $100-300 revenue (realistic for cold launch)
- #13 Agents: Alpha users testing ckBTC DCA Agent
- #14 Horror: 1-2 scripts finalized, production scheduled
- #7 YouTube: 3-5 videos published, subscriber baseline established
- #12 Android: Scope frozen, dev environment ready

---

## Resource Allocation (40 hrs/week distributed)

| Idea | Week 1 | Week 2 | Week 3 | Week 4 | Total (hrs) | % of Capacity |
|------|--------|--------|--------|--------|-------------|--------------|
| #5 Merch | 8 | 10 | 8 | 4 | 30 | 18.75% |
| #13 Agents | 10 | 8 | 8 | 10 | 36 | 22.5% |
| #14 Horror | 6 | 6 | 8 | 8 | 28 | 17.5% |
| #7 YouTube | 8 | 8 | 8 | 6 | 30 | 18.75% |
| #12 Android | 6 | 6 | 4 | 4 | 20 | 12.5% |
| **Buffer/Sync** | **2** | **2** | **4** | **8** | **16** | **10% |
| **TOTAL** | **40** | **40** | **40** | **40** | **160** | **100%** |

**Buffer Purpose:** Cross-idea coordination, blocking issue resolution, iteration on failed experiments.

---

## Idea #5: Custom AI Merch & Print-on-Demand

### Phase 1 Goal
Launch first product collection on Etsy + Printful by end of Week 4. Target: $100-300 revenue month 1.

### Week 1: Platform & Supplier Research (8 hrs)
- **Mon:** Evaluate Printful, Redbubble, Etsy API integration
  - Cost structure, margins, shipping times
  - API capabilities + difficulty (2h)
- **Tue-Wed:** Research top-performing AI merch designs on Etsy (2h)
  - Trending niches: horror aesthetic, crypto/Bitcoin themes, tech humor
  - Pricing patterns ($15-40 for t-shirts)
- **Wed:** Decide on 2 suppliers (Printful + Etsy directly OR Redbubble)
  - Printful: Better API, recurring revenue potential
  - Redbubble: Faster onboarding, no setup fees
  - Decision: **Printful + Etsy (API for automation)**
- **Thu-Fri:** Account setup, API key procurement, integration testing (2h)

### Week 2: Design Pipeline & Art Generation (10 hrs)
- **Mon-Tue:** Design brief creation (2h)
  - 10 designs planned (split across 3-4 themes)
  - Horror + crypto mashup angle
  - Batch generation strategy
- **Tue-Fri:** AI art generation (Caffeine/Midjourney) (6h)
  - 10 designs generated using consistent art style
  - Mockup generation (t-shirt preview images)
- **Fri:** Design selection + quality review (2h)
  - Final 8-10 selected for launch
  - Mockups staged on Etsy draft

### Week 3: Etsy + Printful Launch (8 hrs)
- **Mon-Tue:** Upload 8 designs to Etsy (3h)
  - Product descriptions, keywords, tags
  - Price setting ($22-28 wholesale cost + $8-12 markup)
- **Tue-Wed:** Printful integration + fulfillment setup (2h)
  - Connect Etsy shop to Printful
  - Test order flow end-to-end
  - Configure shipping profiles
- **Wed-Thu:** Pricing optimization + SEO (2h)
  - Etsy shop optimization (banner, shop description)
  - Keyword research for discoverability
  - Launch 5 designs publicly
- **Thu-Fri:** Monitor + initial marketing (1h)
  - Share shop link in relevant Discord communities
  - Reddit r/Etsy, crypto forums

### Week 4: Iteration & OpenClaw Automation Skeleton (4 hrs)
- **Mon-Tue:** Analyze Week 3 orders (if any) (1h)
  - Assess which designs perform
  - Gather customer feedback
- **Tue-Wed:** Design OpenClaw skill outline (2h)
  - Planned skill: AI Merch Design Generator
  - Input: Design brief → Output: Mockup image + Etsy metadata
  - Future automation: Etsy + Printful API integration
- **Wed-Fri:** Design refinement + second wave (1h)
  - Adjust designs based on early data
  - Queue 5 more designs for Week 5

### Month 1 Revenue Target
- **Assumption:** Cold launch, 0 marketing spend, organic growth
- **Realistic target:** $100-300 (3-10 units sold)
- **Upside case:** $500+ if one design goes viral
- **Downside case:** $0-50 (learning phase)

### Dependencies & Risks
| Risk | Mitigation |
|------|-----------|
| Low design quality | Test with 2-3 Etsy shops first; iterate fast |
| Printful integration fails | Fallback to manual upload to Redbubble |
| No sales month 1 | Expected; use data to pivot designs |
| API complexity | Start manual; automate in Month 2 |

---

## Idea #13: AI Agent Products (ICP SaaS)

### Phase 1 Goal
Spec flagship ckBTC DCA Agent, recruit alpha users, deploy to ICP testnet by week 4.

### Strategic Context
- **ICP Portfolio:** 10 agents planned; top 3 are flagship revenue drivers
- **Flagship Focus:** ckBTC DCA (Dollar Cost Averaging) Agent
  - Recurring revenue model: $9.99/month subscription
  - Target user: Bitcoin hodlers on ICP looking to automate purchases
  - First revenue product for AI Agent Products line

### Week 1: Specification & Architecture (10 hrs)
- **Mon-Tue:** Define ckBTC DCA Agent spec (3h)
  - Trigger: Weekly/daily buy schedule
  - Input: Budget per cycle, wallet address, ICP price data source
  - Output: Automate ckBTC purchase via DEX (ic-swap, sonic, etc.)
  - Constraints: Non-custodial (user controls wallet), read-only to price feeds
- **Tue-Wed:** Research ICP canister architecture (2h)
  - Motoko vs. Rust (choose Rust for reliability)
  - HTTP outcall capabilities for price data
  - State management (user settings: frequency, amount, active/paused)
- **Wed-Thu:** Draft system design doc (3h)
  - Canister 1: User Config (CRUD settings, subscriptions)
  - Canister 2: Execution Engine (price checks, initiate orders)
  - Canister 3: Webhook Handler (receive pricing updates)
  - Stripe/ckBTC payment integration decision
- **Thu-Fri:** Technical dependency audit (2h)
  - HTTP outcall for Coingecko/Binance pricing
  - DEX integration library review (ic-swap contracts)
  - Payment processing options (Stripe vs ckBTC-native)

### Week 2: Alpha Development & User Recruitment (8 hrs)
- **Mon:** Finalize tech stack & coding kickoff (1h)
  - Rust canisters confirmed
  - ic-cdk version pinned
- **Mon-Tue:** Implement Config Canister (3h)
  - Create user account (principal), store settings (frequency, amount, wallet)
  - Update/pause/resume subscription logic
  - Unit tests
- **Tue-Wed:** Implement Execution Engine skeleton (2h)
  - Fetch price from Coingecko (mock for Week 2)
  - Calculate ckBTC to purchase
  - Log to event ledger (don't execute yet)
- **Wed-Thu:** Recruiting alpha users (1h)
  - Post in ICP Discord, IC Guild, crypto Twitter
  - Target: 5-10 beta testers for testnet validation
- **Thu-Fri:** Security & permission review (1h)
  - Ensure non-custodial (user controls keys)
  - HTTP outcall safety audit

### Week 3: Testnet Deployment & Alpha Testing (8 hrs)
- **Mon-Tue:** Complete Execution Engine + DEX integration (3h)
  - Actual price fetches (Coingecko, Binance)
  - Mock DEX order simulation (don't execute real trades yet)
  - Integration tests
- **Tue-Wed:** Deploy to ICP testnet (2h)
  - Build WASM binaries
  - Create canisters via dfx
  - Upload to ic.dfinity.network testnet
  - Publish canister URLs + public API docs
- **Wed-Thu:** Alpha user onboarding (2h)
  - Send testnet canisters to beta testers
  - Document setup process (create account, set budget, activate)
  - Gather feedback: UI clarity, feature priorities, trust concerns
- **Thu-Fri:** Issue triage & hot fixes (1h)
  - Bug fixes from alpha feedback
  - Patch testnet canisters

### Week 4: Roadmap Refinement & Mainnet Planning (10 hrs)
- **Mon-Tue:** Analyze alpha feedback (2h)
  - Which features resonated?
  - What's blocking adoption?
  - Price sensitivity: $9.99/mo vs $19.99/mo
- **Tue-Wed:** Payment processing decision (2h)
  - Stripe integration vs ckBTC-native option
  - Determine which payment method for launch
  - Set up Stripe sandbox or ckBTC ledger testing
- **Wed-Thu:** Draft mainnet roadmap (2h)
  - Legal/compliance considerations (fintech in ICP space)
  - Security audit plan (before mainnet launch)
  - Grant application prep (DFINITY $5K-25K range)
- **Thu-Fri:** Grant application draft (2h)
  - Research DFINITY grants (Builders Program, Developer Grant)
  - Draft proposal: AI DCA Agent, $15K funding request
  - Identify grant deadline (typically rolling)
- **Fri:** Testnet metrics snapshot (0.5h)
  - Canister size, cycle consumption estimate
  - Alpha user feedback summary

### Month 1 Success Metrics
| Metric | Target | Notes |
|--------|--------|-------|
| Spec completion | 100% | ckBTC DCA spec finalized by end W1 |
| Alpha users recruited | 5-10 | Active testers on testnet |
| Testnet deployment | Success | Both canisters live, public URLs published |
| Bug reports | <5 critical | Issues filed by alpha users |
| Feedback sentiment | Positive | Core concept resonates |
| Grant application | Submitted | Draft to DFINITY by end of month |

### Dependencies & Risks
| Risk | Mitigation |
|------|-----------|
| ICP price feed unavailable | Use Coingecko with failover to Binance API |
| DEX API changes | Start with mock DEX, real integration in Phase 2 |
| Alpha users flake | Have backup tester list; incentivize with early discounts |
| Mainnet legal complexity | Consult ICP fintech attorney before mainnet (budget $2-5K) |
| Payment processing delays | Decide Stripe vs ckBTC by end W1; prototype by W3 |

---

## Idea #14: Horror Content (Video Essays)

### Phase 1 Goal
Complete 3-4 scripts, establish upload pipeline, publish first 2 videos by Week 4.

### Strategic Role
Primary funnel for broader business ecosystem:
- Video content builds audience → merchandise cross-sell
- SEO long-tail keywords → affiliate revenue potential
- Audience data → future horror game marketing

### Week 1: Scripting & Pre-Production (6 hrs)
- **Mon-Tue:** Brainstorm + outline 3 video topics (2h)
  - Topic ideas: horror game mechanics, film analysis, internet folklore, creepypasta deep-dives
  - Target length: 8-15 min essays (YouTube sweet spot for horror niche)
  - SEO angle: Evergreen topics with search volume
- **Tue-Wed:** Research first video (2h)
  - Gather source material, footage clips, interviews
  - Create detailed outline with timestamp markers
- **Thu-Fri:** Begin first script draft (2h)
  - Target: 2000-3000 words (8-12 min read time)
  - Voice: Analytical, engaging, slightly dark humor

### Week 2: Script Completion & Production Setup (6 hrs)
- **Mon:** Finish first script + second script outline (2h)
  - First script: Complete draft, ready for voiceover
  - Second script: Detailed outline, ready for research
- **Tue-Wed:** Third script outline + asset gathering (2h)
  - Asset list: Background music, footage sources, graphics templates
  - Secure licenses (YouTube Audio Library, Pexels, Pixabay)
- **Thu:** Production setup (1h)
  - Recording software decision (Audacity, Adobe Audition, or native)
  - Video editing software (DaVinci Resolve free tier, or Premiere)
  - Asset organization folder structure
- **Fri:** First voiceover recording (1h)
  - Record narration for first script
  - Quality check, re-record problematic sections

### Week 3: Video Production & Publishing Pipeline (8 hrs)
- **Mon-Tue:** Edit first video (3h)
  - Sync audio to B-roll/footage
  - Add graphics, text overlays, transitions
  - Color grading (dark aesthetic for horror)
  - Render final export
- **Tue-Wed:** YouTube setup + publish first video (2h)
  - Create YouTube channel (if not exists)
  - Upload video with optimized metadata (title, description, tags, thumbnail)
  - Schedule publish for best time (Thu/Fri evening for US audience)
  - Thumbnail design: High contrast, emotion, curiosity gap
- **Wed-Thu:** Record & edit second video (2h)
  - Voiceover for second script
  - Quick edit, quality check
  - Publish or schedule
- **Thu-Fri:** Plan Week 4 batch (1h)
  - Prep third script for voiceover
  - Build content calendar for Month 2

### Week 4: Optimization & Momentum Building (8 hrs)
- **Mon-Tue:** Monitor first 2 videos (2h)
  - Analyze YouTube Analytics: CTR, retention, demographics
  - Respond to comments, engage with audience
  - Identify what worked (hooks, length, topics)
- **Tue-Wed:** Third video production sprint (3h)
  - Complete voiceover, edit, publish
  - Target: 3 videos published by end of week
- **Wed-Thu:** Cross-promotion setup (2h)
  - Discord announcements (Moises's server, relevant communities)
  - Reddit posts (r/horror, r/creepypodcast, niche subreddits)
  - Twitter/X thread for each video
- **Thu-Fri:** Analytics deep-dive & roadmap (1h)
  - Assess which topics gained traction
  - Plan Month 2 content strategy

### Month 1 Success Metrics
| Metric | Target |
|--------|--------|
| Scripts completed | 3-4 finalized |
| Videos published | 2-3 live on YouTube |
| YouTube subscribers | 50-200 baseline |
| Average view count | 50-200 views per video |
| Engagement rate | 5%+ (likes + comments per view) |
| Content calendar | 4 weeks planned ahead |

### Dependencies & Risks
| Risk | Mitigation |
|------|-----------|
| Script quality low | Revise ruthlessly; get feedback from horror community |
| Audio/video quality issues | Test equipment first; record in quiet space |
| Copyright strikes | Only use licensed music (YouTube Audio Library) |
| Low discoverability | Optimize titles/tags for "horror essay," "creepypasta" queries |
| Burnout on writing | Batch writing; aim for 1 script per week, not daily |

---

## Idea #7: YouTube Automation & Content Funnel

### Phase 1 Goal
Establish publishing pipeline, create 3-5 additional videos (beyond horror essays), build automation scaffolding.

### Strategic Role
- Monetization: AdSense, affiliate links (horror games, books, streaming), sponsorships
- SEO funnel: Drives traffic to all other businesses
- Vertical expansion: Gaming walkthroughs, Kickstarter coverage, product reviews

### Week 1: Pipeline Architecture & Content Calendar (8 hrs)
- **Mon-Tue:** Define content pillars (2h)
  - Pillar 1: Horror essays (Week 1-4, covered under #14)
  - Pillar 2: Game mechanics breakdowns (8-10 min)
  - Pillar 3: Product reviews (AI tools, horror games, related tech)
  - Pillar 4: Reaction/commentary (trending horror media)
- **Tue-Wed:** Content calendar + SEO research (3h)
  - Generate 12-15 video ideas for Month 1-2
  - Research keywords: search volume, competition, CPC
  - Prioritize: High-intent keywords (e.g., "best horror games 2026" vs. generic)
- **Wed-Thu:** Automation framework design (2h)
  - Workflow: Script → Record → Edit → Publish → Monitor
  - Tool stack: Audacity (audio), DaVinci Resolve (video), YouTube Data API (scheduling)
  - OpenClaw skill opportunity: YouTube metadata generator + SEO optimizer
- **Thu-Fri:** Team/resource planning (1h)
  - Can Moises handle all production solo? (Likely for Month 1)
  - Future: Consider text-to-speech for voiceovers (ElevenLabs) to scale

### Week 2: Content Production Sprint #1 (8 hrs)
- **Mon:** Record 2 game mechanics videos (3h)
  - Topic 1: Indie horror game design patterns
  - Topic 2: AI in horror game development
  - Quick voiceover + B-roll editing
- **Tue-Wed:** Edit + publish (2h)
  - Fast turnaround: Record → Edit → Publish (same day if possible)
  - Use templates to speed up editing
- **Wed-Thu:** Research & outline Pillar 3 + 4 content (2h)
  - 2 product reviews (outline, research sources)
  - 2 reaction topics (latest horror releases)
- **Thu-Fri:** Buffer day / analytics review (1h)

### Week 3: Cross-Promotion & Audience Building (8 hrs)
- **Mon-Tue:** Advanced SEO optimization (2h)
  - Analyze top-performing competitors' titles, thumbnails, descriptions
  - A/B test thumbnail designs (high contrast, emotional, curiosity gap)
  - Optimize existing videos for higher ranking
- **Tue-Wed:** Community engagement (2h)
  - Comment on related channels (horror, gaming, AI)
  - Post in relevant subreddits, Discord communities
  - Respond to all comments on Moises's videos (build loyalty)
- **Wed-Thu:** Collaboration outreach (2h)
  - Email 3-5 other horror creators (suggest collaboration, shoutout trades)
  - Identify podcast guests (horror, gaming, AI hosts)
- **Thu-Fri:** Publish 2 more videos (0h production, covered earlier)

### Week 4: Analytics & Automation Roadmap (6 hrs)
- **Mon-Tue:** YouTube Analytics deep-dive (2h)
  - Which pillars have highest CTR/retention?
  - Audience demographics, traffic sources
  - Keyword winners (invest more here)
- **Tue-Wed:** OpenClaw skill planning (2h)
  - Scope: YouTube Metadata Generator
  - Input: Topic + keywords + video length → Output: SEO-optimized title, description, tags
  - Integration: Automated publishing + scheduling
- **Wed-Thu:** Month 2 roadmap (1h)
  - Ramp to 2-3 videos/week if automation enables
  - Plan vertical expansion (reviews, reactions, collabs)
- **Thu-Fri:** Buffer / contingency (1h)

### Month 1 Success Metrics
| Metric | Target |
|--------|--------|
| Videos published | 5-7 total (2-3 horror essays + 3-4 other content) |
| Total subscribers | 100-300 across all verticals |
| Average views per video | 100-300 |
| Playlist setup | Created 3-4 playlists (Horror Essays, Game Mechanics, Reviews) |
| Automation spec | Drafted OpenClaw skill for YouTube SEO |
| AdSense eligibility | On track (need 1000 subs, 4000 watch hours for approval) |

---

## Idea #12: Android Mobile App MVP

### Phase 1 Goal
Scope MVP, finalize tech stack, set up dev environment, code foundation by Week 4.

### Strategic Context
- Related to: Horror content funnel, game tie-in potential
- Long-term vision: Companion app for horror content or game ecosystem
- Month 1 focus: MVP scope definition, NOT shipping app

### Week 1: Scope & Competitive Analysis (6 hrs)
- **Mon-Tue:** Define Android MVP (2h)
  - Clarify: Is this a horror game companion? Content aggregator? Community app?
  - Feature set: 3-5 core features only (not bloat)
  - Target API level: Android 10+ (85%+ market coverage)
- **Tue-Wed:** Competitive analysis (2h)
  - Analyze 5-10 similar apps (horror, gaming, content)
  - Identify design patterns, monetization strategies
  - Assess App Store policies (horror content, ratings, etc.)
- **Thu-Fri:** Finalize MVP spec document (2h)
  - 1-page spec: Purpose, user stories, feature list, success metrics
  - Non-goals (what's NOT in MVP)

### Week 2: Tech Stack & Dev Environment Setup (6 hrs)
- **Mon:** Kotlin vs Java decision (1h)
  - Recommendation: Kotlin (modern, Moises likely prefers)
- **Mon-Tue:** Architecture decision (2h)
  - MVVM pattern
  - Room database for local storage
  - Retrofit for API calls
  - Jetpack Compose for UI (if Kotlin)
- **Tue-Wed:** Project scaffolding (2h)
  - Create Android Studio project
  - Add dependencies (Room, Retrofit, Compose, Unit test libs)
  - Set up Git repo
- **Wed-Thu:** Development environment hardening (1h)
  - Emulator configuration (Pixel 5, Android 13)
  - Build optimization (ProGuard/R8)

### Week 3: Foundation Code & MVP Prototype (4 hrs)
- **Mon-Tue:** Implement base screens (2h)
  - Splash screen
  - Onboarding flow (2-3 screens)
  - Tab/navigation structure
- **Tue-Wed:** Data layer skeleton (1h)
  - Room database setup (basic entities)
  - Mock API client (hardcoded responses)
- **Thu-Fri:** Prototype UI iteration (1h)
  - Basic UI for 2-3 key screens
  - Navigation flow demo

### Week 4: Testing & Documentation (4 hrs)
- **Mon-Tue:** Unit tests foundation (1h)
  - Test utilities, mock data setup
  - Basic ViewModel tests
- **Tue-Wed:** Documentation & roadmap (2h)
  - Architecture decision record (ADR)
  - Setup guide for future developers
  - Month 2 feature roadmap
- **Thu-Fri:** Code review & cleanup (1h)
  - Self-code review for quality
  - Tech debt log (note any intentional shortcuts)

### Month 1 Success Metrics
| Metric | Target |
|--------|--------|
| MVP scope finalized | 1-page doc + user stories |
| Tech stack decided | Kotlin + MVVM + Compose |
| Dev environment ready | Android Studio configured, emulator working |
| Foundation code | Buildable, no crashes |
| Git repo initialized | Clean history, README.md |
| Documentation | ADR + setup guide + roadmap |
| Feature roadmap | 3-month pipeline defined |

### Dependencies & Risks
| Risk | Mitigation |
|------|-----------|
| Unclear product-market fit | Validate with 2-3 potential users before building (Week 1) |
| Scope creep | Enforce MVP spec strictly; defer all "nice-to-haves" |
| Dev environment issues | Test emulator early (Week 1); have fallback (genymotion) |
| API delays block dev | Use mock APIs in Week 1-3; real API in Week 4+ |

---

## Cross-Idea Coordination & Buffer Time (16 hrs total)

### Coordination Points
- **Week 1 sync:** All ideas are in research/planning mode; low dependency risk
- **Week 2 sync:** Merch & Horror have content ready; Agents in build phase
- **Week 3 sync:** All ideas have concrete deliverables; share wins + blockers
- **Week 4 sync:** Review all KPIs, plan Month 2 resource reallocation

### Buffer Activities (rotate per week)
- **Week 1:** Planning/decision support (1-2h) + OpenClaw skill roadmapping (1h)
- **Week 2:** Cross-idea issue resolution + platform API troubleshooting (2h)
- **Week 3:** Scaling bottleneck removal + tooling improvements (2h)
- **Week 4:** Analytics synthesis + Month 2 strategy + team retrospective (4-6h)

---

## Key Success Criteria (End of Month 1)

### Financial
- [ ] #5 Merch: $100+ revenue (or clear path to $500+ in Month 2)
- [ ] #13 Agents: Grant application submitted, alpha feedback positive
- [ ] #14 Horror: 2-3 videos published, 50+ subscribers
- [ ] #7 YouTube: 5-7 videos published, 100+ total subscribers
- [ ] #12 Android: MVP buildable, zero runtime crashes

### Strategic
- [ ] All 5 ideas have validated market/user feedback
- [ ] Automation roadmap (OpenClaw skills) drafted for #5 + #7
- [ ] Month 2 resource allocation plan finalized
- [ ] One idea is clearly "winner" by engagement/revenue

### Operational
- [ ] All repos initialized + documented
- [ ] No blocked work due to unclear requirements
- [ ] Team sync cadence established (weekly or per-idea)

---

## Month 2 Preview

Based on Month 1 outcomes:

**If #5 Merch wins:** Scale to 20+ designs, hire design partner, revenue target: $1-2K
**If #13 Agents wins:** Launch mainnet payment system, recruit 20+ alpha users, target: $500/mo revenue
**If #14 Horror wins:** Ramp to 2-3 videos/week, pursue sponsorships, build to 5K subscribers
**Likely hybrid:** Merch + Agents run in parallel, Horror becomes content funnel, YouTube scales organically, Android MVP shipped

---

## Assumptions & Constraints

1. **Moises capacity:** 40 hrs/week available for these 5 ideas (vs. other projects like OISY Guardian)
2. **OpenClaw automation:** Orchestrator handles async tasks (price monitoring, design uploads, etc.) at scale
3. **No external hiring:** Solo execution for Month 1; consider contractors for Month 2 if winning ideas emerge
4. **Platforms stable:** Etsy, YouTube, ICP network uptime assumed
5. **No major pivots:** If market data suggests pivot, execute pivot in Month 2, not mid-Month-1

---

## Document History

- **Created:** Feb 27, 2026 (Strategy-Planner subagent)
- **Approval:** Moises (subject to revision based on Day 1-2 feedback)
- **Last updated:** Feb 27, 2026
