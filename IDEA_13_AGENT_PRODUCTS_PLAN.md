# IDEA #13: AI Agent Products (ICP SaaS) — Detailed Execution Plan

**Timeline:** 4 weeks (Feb 27 - Mar 27, 2026)  
**Total effort:** 36 hours (22.5% of weekly capacity)  
**Product focus:** ckBTC Dollar-Cost Averaging (DCA) Agent  
**Revenue model:** $9.99/month recurring subscription  
**Grant target:** DFINITY Builders Program ($5K-25K)  

---

## Strategic Context: The ICP Agent Ecosystem

### Portfolio Structure
- **Total agents planned:** 10 agents across crypto, finance, and automation niches
- **Flagship agents (revenue-focused):** 3 agents = primary revenue drivers
- **Portfolio agents (positioning):** 7 agents = demonstrate breadth, credibility, network effects

### Month 1 Focus: ckBTC DCA Agent (Flagship #1)

**Why ckBTC DCA first?**
1. **Recurring revenue:** $9.99/month = sustainable business model
2. **Immediate market:** ICP community is bullish on Bitcoin, wants DCA strategy
3. **Low implementation risk:** No custody, no financial advisory, pure execution
4. **Grant-friendly:** DFINITY loves Bitcoin + ICP cross-chain applications
5. **Network effect:** More users → more ecosystem data → better pricing intelligence

**Competitive advantage:**
- First ICP-native DCA agent (as far as we know)
- Non-custodial (security-focused)
- Automated, transparent, auditable on-chain

---

## Product Definition: ckBTC DCA Agent

### What It Does
**Scheduled purchasing:** Users set a weekly/daily budget (e.g., $100/week), and the agent automatically:
1. Checks current ckBTC price (via Coingecko, Binance API)
2. Calculates amount to buy (e.g., 0.5 ckBTC at current price)
3. Initiates buy order on ICP DEX (ic-swap, sonic, or DEX router)
4. Transfers ckBTC to user's wallet
5. Logs transaction for reporting

### Key Constraints (Non-Negotiable)
- **Non-custodial:** Agent never holds user's funds
  - User pre-approves budget per cycle
  - Agent initiates transfer, user wallet completes it
  - User maintains full key control at all times
- **No investment advice:** Agent is a tool, not an advisor
  - Disclaimer: "DCA is a strategy, not a guarantee"
  - No performance claims or projections
- **Price feed transparency:** Use public APIs (Coingecko, Binance)
  - Auditable, no hidden pricing
  - Fallback if primary feed fails

### User Experience (Simplified)

```
SETUP (Week 1):
1. User creates ICP principal with ckBTC balance
2. User connects via agent interface
3. User sets DCA schedule: "Buy 100 ICP worth every Sunday"
4. User pre-approves weekly transfer limit (e.g., 0.1 ckBTC max)
5. Agent monitors and executes every Sunday at 12 PM UTC

ONGOING:
- User gets weekly notification: "DCA executed: 0.05 ckBTC @ $67K"
- User can pause/resume anytime
- User pays $9.99/month subscription

REPORTING:
- Dashboard shows: Total invested, avg price, current value, gain/loss
- Export CSV for tax purposes
```

---

## Week 1: Specification & Architecture (10 hours)

### Day 1-2: Product Spec & User Stories (3 hours)

**Deliverable:** One-page product specification + 5-10 user stories

**Spec template:**
```
# ckBTC DCA Agent — Product Spec

## Overview
Non-custodial, automated Dollar-Cost Averaging for ckBTC on ICP.

## User Story Examples
- As a Bitcoin hodler, I want to invest $100 weekly in ckBTC without checking prices
- As a risk-averse investor, I want proof my keys never leave my wallet
- As a trader, I want a dashboard showing my DCA history and cost basis

## Feature Set (MVP)
1. Schedule DCA: Weekly or daily frequency
2. Budget control: Set max spend per cycle
3. Execution: Automated buy on DEX
4. Notifications: Email/Discord on execution
5. Dashboard: Simple portfolio view (invested, current value)

## Non-Features (Scope Boundaries)
- Does NOT provide financial advice
- Does NOT hold customer funds
- Does NOT manage customer private keys
- Does NOT guarantee fills (market orders only)

## Success Metrics (Month 1)
- Alpha users: 5-10 engaged testers
- Uptime: 99%+ (scheduled jobs execute)
- User retention: 70%+ (active after week 1)
```

**User stories example:**
```
US-001: Non-Custodial Proof
AS a security-conscious user
I WANT assurance my private keys never leave my wallet
SO THAT I feel safe using the agent

ACCEPTANCE CRITERIA:
- Agent canister has zero balance (no key storage)
- Agent initiates transfers, doesn't execute them
- User signature required for each transaction
```

### Day 2: ICP Canister Architecture (2 hours)

**Decision: Rust or Motoko?**
- **Rust:** More ecosystem libraries, better performance, steeper learning curve
- **Motoko:** ICP-native, simpler syntax, fewer production examples
- **CHOICE: Rust** (Moises is a developer, prefers systems language)

**Canister Design (3 canisters, not monolith):**

**Canister 1: Config Canister (User State)**
- Stores user settings (frequency, budget, active/paused status)
- Stores subscription status (paid/unpaid, expires)
- Stores execution history (ledger of past DCA runs)
- ~1000 lines of Rust code

**Canister 2: Execution Engine**
- Monitors scheduled times (via timer)
- Fetches price from HTTP outcall (Coingecko/Binance)
- Calculates amount to buy
- Initiates DEX order (calls ic-swap canister)
- Records result (success/failure)
- ~500 lines of Rust code

**Canister 3: Webhook Handler (Future)**
- Receives price update signals
- Triggers Execution Engine
- (Phase 1b, low priority for MVP)

### Day 2-3: Dependency Audit (2 hours)

**Research & document:**

| Dependency | Status | Notes |
|------------|--------|-------|
| **HTTP outcall (price feed)** | ✅ Stable | ICP native, free |
| **Coingecko API** | ✅ Stable | Free tier, 100 req/min |
| **Binance API** | ✅ Stable | Free tier, 1200 req/min |
| **ic-swap DEX** | ✅ Active | ~5K LOC, community-maintained |
| **ic-cdk (Rust SDK)** | ✅ Stable | Official, frequent updates |
| **Stripe (payments)** | ⚠️ Testing | Need sandbox account |
| **ckBTC ledger** | ✅ Stable | ICRC-1 standard |

**Blockers & workarounds:**
- If HTTP outcall fails: Hardcode price (MVP), move to oracle in Phase 1b
- If Stripe unavailable: Use ckBTC-native payments (hold user funds in canister, less preferred)
- If DEX unavailable: Use market order via native ICP token canister

### Day 3-4: System Design Document (3 hours)

**Deliverable:** 2-3 page design doc with diagrams

**Sections:**
1. **Architecture diagram**
   ```
   User Interface (Frontend)
        ↓
   API Gateway (canister)
        ↓ (HTTP outcall)
   Config Canister ← → Execution Engine ← → Coingecko API
        ↓ (inter-canister call)
   ckBTC Ledger ← → ic-swap DEX
   ```

2. **Data flow for execution**
   ```
   1. Timer fires (Sunday 12 PM UTC)
   2. Execution Engine wakes up
   3. Loop over all active users (from Config)
   4. For each user:
      a. Fetch price (Coingecko)
      b. Calculate amount to buy
      c. Create order on ic-swap
      d. Update ledger in Config Canister
   5. Cleanup & metrics
   ```

3. **Error handling**
   - Price feed timeout → Use cached price (max 1h old)
   - DEX order fails → Notify user, retry next cycle
   - User balance insufficient → Skip, notify user
   - Canister out of cycles → Alert for refund

4. **Security considerations**
   - HTTP outcall to Coingecko only (whitelist IP)
   - No private key storage (non-custodial)
   - User signatures required for sensitive ops
   - Rate limiting on API calls

### Day 4-5: Tech Stack Finalization (1 hour)

**Technology choices (frozen for Month 1):**

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Performance, ecosystem, Moises's comfort |
| Frontend | SvelteKit (or Next.js) | Fast iteration, reactive UI |
| State | Config Canister | Persistent, on-chain |
| Price feed | Coingecko primary, Binance fallback | Free, reliable, 100+ req/min |
| DEX | ic-swap | Active, documented, good test data |
| Payments | Stripe (MVP) | Easier than ckBTC custody |
| Testing | cargo test + ic-repl | Native ICP tooling |

**Delivery by end of Day 5:**
- [ ] Specification document (1 page, frozen)
- [ ] System design with diagrams (2-3 pages)
- [ ] Technology choices locked
- [ ] Risk register (HTTP outcall, payment processing)

---

## Week 2: Alpha Development & User Recruitment (8 hours)

### Day 1: Kickoff & Dev Environment (1 hour)

**Tasks:**
- [ ] Clone ICP starter template (or create from scratch)
- [ ] Set up Rust project with ic-cdk + cargo
- [ ] Create dfx.json with 3 canisters
- [ ] Git repo initialized (icp-dcaagent or similar)
- [ ] Ci/CD pipeline stub (for testnet later)

**Deliverable:** Buildable empty project, no code yet.

### Day 1-2: Config Canister Implementation (3 hours)

**Scope:** User settings CRUD + state management

**Code structure:**
```rust
// src/config/lib.rs

use ic_cdk::api::caller;
use std::collections::HashMap;

#[derive(Clone, Candid)]
pub struct User {
    pub principal: Principal,
    pub dca_frequency: String, // "daily" | "weekly"
    pub budget_per_cycle: u64, // in ICP satoshis
    pub active: bool,
    pub subscribed: bool,
    pub created_at: u64,
}

#[derive(Clone, Candid)]
pub struct ExecutionRecord {
    pub timestamp: u64,
    pub amount_bought: u64,
    pub price_at_execution: f64,
    pub success: bool,
    pub tx_id: String,
}

#[ic_cdk::update]
pub fn register_user(frequency: String, budget: u64) -> Result<User, String> {
    // Validation: frequency in ["daily", "weekly"]
    // Create User struct
    // Store in state
}

#[ic_cdk::update]
pub fn pause_dca() -> Result<(), String> {
    // Mark user as inactive
}

#[ic_cdk::update]
pub fn update_budget(new_budget: u64) -> Result<(), String> {
    // Validate, update
}

#[ic_cdk::query]
pub fn get_user() -> Option<User> {
    // Return current user's settings
}

#[ic_cdk::query]
pub fn get_execution_history() -> Vec<ExecutionRecord> {
    // Return all DCA executions for current user
}
```

**Testing:**
- Unit tests for registration (valid/invalid inputs)
- Test pause/resume logic
- Test budget update validation

### Day 2-3: Execution Engine (Mock) (2 hours)

**Scope:** Price fetching + order calculation (mock DEX for Week 2)

```rust
// src/engine/lib.rs

use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument};

#[ic_cdk::update]
pub async fn fetch_price() -> Result<f64, String> {
    // Call Coingecko API via HTTP outcall
    // Parse JSON response
    // Return current ckBTC price in USD
}

pub fn calculate_amount(budget_usd: f64, price: f64) -> f64 {
    // amount = budget / price
    // Return in ckBTC
}

#[ic_cdk::update]
pub async fn execute_dca(user_principal: Principal) -> Result<ExecutionResult, String> {
    let user = get_user(user_principal)?; // Call config canister
    let price = fetch_price().await?;
    let amount = calculate_amount(user.budget_per_cycle as f64, price);
    
    // Mock: Don't actually buy yet
    Ok(ExecutionResult {
        timestamp: ic_cdk::api::time(),
        amount: amount,
        price: price,
        status: "mock_success".to_string(),
    })
}
```

**Mock for Week 2:** Log to ledger, don't call DEX.

### Day 3: User Recruitment Email (1 hour)

**Objective:** Find 5-10 alpha testers by end of Week 2

**Channels:**
- **ICP Discord community** (IC Guilds, ICP Builders, etc.)
- **Twitter/X:** @dfinity_icp community, #InternetComputer hashtag
- **Reddit:** r/InternetComputer, r/icp_cryptocurrency
- **Personal network:** DM crypto friends, Bitcoin maximalists

**Recruitment message:**
```
Launching ckBTC DCA Agent (testnet beta). Non-custodial, automated 
DCA for Bitcoin lovers on ICP. $9.99/mo when mainnet launches.

Need 5-10 alpha testers for testnet validation (Feb 27 - Mar 15).

Perks:
- Lifetime 50% discount when we launch mainnet
- Your feedback shapes the product
- Help secure a Bitcoin + ICP future

Interested? Reply here or DM.
```

**Signup form** (or Discord form):
- [ ] Name
- [ ] ICP wallet address (for testnet credits if needed)
- [ ] Experience with DCA / crypto
- [ ] Preferred frequency (daily/weekly)
- [ ] Availability for feedback (2-3 calls in March)

### Day 4-5: Security & Permission Review (1 hour)

**Security checklist:**
- [ ] No private key storage in canister (audit code)
- [ ] HTTP outcall only to whitelisted URLs (Coingecko)
- [ ] User signatures required for state-changing operations
- [ ] Rate limiting on Coingecko calls (max 100/min)
- [ ] Error handling for failed price feeds

**Deliverable by end of Week 2:**
- [ ] Config + Execution Engine code (buildable, not fully optimized)
- [ ] 5-10 alpha testers recruited
- [ ] Unit tests passing
- [ ] Security audit checklist completed

---

## Week 3: Testnet Deployment & Alpha Testing (8 hours)

### Day 1-2: Complete Execution Engine + DEX Integration (3 hours)

**Remaining work:**
- Connect to real ic-swap API (not mock)
- Handle actual ckBTC transfers
- Error handling for failed orders
- Integration tests (Config + Engine together)

```rust
// Integration: Call ic-swap to actually buy ckBTC
#[ic_cdk::update]
pub async fn execute_dca_v2(user_principal: Principal) -> Result<TransactionId, String> {
    let user = get_user(user_principal)?;
    let price = fetch_price().await?;
    let amount = calculate_amount(user.budget as f64, price);
    
    // Call ic-swap canister to create order
    let order_response = ic_swap::create_order(
        principal: user_principal,
        amount_usd: amount,
        token_out: "ckBTC", // Want ckBTC
    ).await?;
    
    // Record in history
    record_execution(ExecutionRecord {
        timestamp: ic_cdk::api::time(),
        amount_bought: order_response.amount,
        price_at_execution: price,
        success: true,
        tx_id: order_response.tx_id,
    })?;
    
    Ok(order_response.tx_id)
}
```

**Testing:**
- End-to-end test: Register user → Execute DCA → Check ledger
- Failure cases: Insufficient balance, DEX timeout, price feed failure

### Day 2: Deploy to ICP Testnet (2 hours)

**Deployment steps:**
```bash
# 1. Build Rust canisters
cargo build --release --target wasm32-unknown-unknown

# 2. Create dfx project if not exists
dfx new icp_dca_agent --type=rust

# 3. Generate candid interfaces
candid generate src/config/lib.rs > src/config/config.did

# 4. Deploy to local first (validation)
dfx start --background
dfx deploy config
dfx deploy engine

# 5. Deploy to ICP testnet (ic.dfinity.network)
dfx identity use default  # Or create new identity
dfx canister create --network=ic config
dfx canister create --network=ic engine
dfx deploy --network=ic config
dfx deploy --network=ic engine

# 6. Capture canister IDs
dfx canister id config --network=ic  # e.g., 5w5x7-7qaaa-...
dfx canister id engine --network=ic
```

**Output artifacts:**
- Config Canister ID: `[captured]`
- Engine Canister ID: `[captured]`
- Candid interface URLs (for Candid UI exploration)
- Public API docs (link to GitHub README)

**Success criteria:**
- [ ] Canisters deployed to testnet
- [ ] No error logs on deployment
- [ ] Canisters are callable via dfx
- [ ] Canister IDs published to team

### Day 2-3: Alpha User Onboarding (2 hours)

**Deliverables:**
- [ ] Setup guide (1 page)
- [ ] Video walkthrough (5-10 min)
- [ ] Discord onboarding channel
- [ ] Feedback form (Google Form or Typeform)

**Setup guide outline:**
```
# ckBTC DCA Agent — Alpha Setup Guide

## Step 1: Create ICP Wallet
- Use Plug Wallet or NNS Dapp (free)
- Request testnet ICP (faucet link)

## Step 2: Connect to Agent
- Visit: https://dca-agent-alpha.example.com
- Click "Connect with Plug"
- Approve permissions

## Step 3: Set DCA Schedule
- Select frequency: Weekly
- Set budget: 10 ICP
- Click "Activate"

## Step 4: Monitor Execution
- Dashboard shows scheduled date/time
- Check back Friday 12 PM UTC for execution
- View transaction ID in history

## Step 5: Feedback
- How intuitive was setup? (5-point scale)
- Any bugs or confusing steps?
- Would you pay $9.99/month?
```

**Feedback form questions:**
```
1. How easy was setup? (1-5)
2. Do you understand how DCA works now? (Yes/No)
3. What confused you most?
4. Would you use this if it were free? (Yes/No)
5. Would you pay $9.99/month? (Yes/No/Maybe)
6. What feature would make you more likely to use it?
7. Any bugs or crashes?
```

### Day 4-5: Issue Triage & Hot Fixes (1 hour)

**Expected issues (first week):**
- UI/UX confusion (feedback from users)
- Price feed timeout (edge case)
- Deposit/withdrawal issues (if payment system not ready yet)

**Triage process:**
1. Categorize: Critical (blocks usage), High (degrades UX), Low (cosmetic)
2. Assign priority to fixes
3. Deploy hotfixes to testnet (same day for critical)

**Deliverable by end of Week 3:**
- [ ] Both canisters live on testnet
- [ ] 5-10 alpha users have access
- [ ] Setup documentation published
- [ ] Feedback form active
- [ ] 0-2 critical bugs remaining (known and being fixed)

---

## Week 4: Roadmap Refinement & Mainnet Planning (10 hours)

### Day 1-2: Alpha Feedback Analysis (2 hours)

**Data collection:**
- Feedback form responses (quantitative)
- User testing calls (qualitative, 1-2 sessions)
- Canister metrics (uptime, execution count, errors)

**Analysis questions:**
1. Which features confused users most?
2. What would increase adoption (price, features, UX)?
3. Is $9.99/month attractive or too high?
4. Did users feel comfortable with non-custodial model?

**Output:** Summary doc (1-2 pages)

### Day 2: Payment Processing Decision (2 hours)

**Option A: Stripe subscription**
- Pros: Industry-standard, customers familiar, recurring billing easy
- Cons: CMS onboarding (SSN, bank account for Moises), 2.9% + $0.30 fee per transaction
- Timing: Setup account same day, live by Week 3 of Month 2

**Option B: ckBTC-native payments**
- Pros: On-chain, transparent, no middleman
- Cons: Requires holding user funds (custodial risk), user education needed
- Timing: Build in Month 2, live Month 3

**Option C: Free Month 1, Stripe in Month 2**
- Pros: Launch faster, gather more feedback, monetize later
- Cons: Habit formation (free → harder to convert to paid)

**CHOICE:** **Option A (Stripe)** or **Option C (defer to Month 2)**
- Decision point: Is payment processing a blocker for Month 1? (No, optional for alpha)
- **GO WITH OPTION C:** Free testnet alpha, Stripe sandbox in Month 2, mainnet with both options

### Day 2-3: Mainnet Readiness Checklist (2 hours)

**Legal & Compliance:**
- [ ] Fintech attorney review (estimated cost: $2-5K)
- [ ] OFAC/sanctions check (does agent work in compliant jurisdictions?)
- [ ] SEC guidance (is DCA agent a security? Likely not, but verify)
- [ ] Terms of Service + Privacy Policy (needed before public launch)

**Technical:**
- [ ] Security audit (external firm, estimated $5-15K or internal review)
- [ ] Performance testing (can handle 1000 users?)
- [ ] Disaster recovery plan (if DEX goes down, what happens?)
- [ ] Monitoring & alerts (uptime tracking, error dashboards)

**Monetization:**
- [ ] Pricing finalized ($9.99/mo or flexible?)
- [ ] Payment system live (Stripe or ckBTC)
- [ ] Subscription management UI (pause, cancel, upgrade)
- [ ] Revenue sharing (if ICP ecosystem benefits, should users get kickback?)

**Timeline for mainnet:**
- Mid-April 2026 (6 weeks from Month 1 end)
- Requires passing attorney review + security audit

### Day 3-4: Grant Application Draft (2 hours)

**Target:** DFINITY Builders Program or Developer Grant

**Grant application outline:**
```
# DFINITY Builders Program Application
## ckBTC DCA Agent

### Problem
Bitcoin hodlers on ICP want to automate recurring purchases without 
managing private keys or timing markets.

### Solution
Non-custodial, ICP-native DCA Agent. Scheduled buys, transparent pricing, 
full user control.

### Why ICP?
- Native ckBTC support (seamless cross-chain)
- HTTP outcall enables real-time pricing
- Canister state = audit trail for users

### Why now?
- Bitcoin volatility makes DCA increasingly valuable
- ICP ecosystem lacks automation tools
- First-mover advantage in DCA space

### Funding Request
$15,000 for:
- Security audit ($8K)
- Attorney review ($3K)
- Marketing & user acquisition ($4K)

### Timeline
- Week 3-4 (March): Testnet alpha
- Week 1-2 (April): Security audit + legal review
- Week 3 (April): Mainnet launch
- Ongoing: User acquisition + iteration

### Milestones
1. Alpha: 10 engaged testers, positive feedback
2. Mainnet: Live with payment processing, 100+ users
3. Revenue: $1K/month recurring (100 users @ $9.99)
4. Impact: Enable $1M+ in automated Bitcoin purchases on ICP
```

**Submission deadline:** Check DFINITY website (rolling or quarterly)
**Estimated approval time:** 2-4 weeks after submission

### Day 5: Testnet Metrics & Month 2 Roadmap (2 hours)

**Metrics to capture:**
- [ ] Canister size (kilobytes of Wasm)
- [ ] Estimated cycle consumption per execution
- [ ] User registration count
- [ ] Execution success rate (% of scheduled jobs that succeeded)
- [ ] Average price accuracy (vs real market)
- [ ] User retention (active week 1 → week 2)

**Month 2 roadmap (draft):**
```
### Month 2: Mainnet Prep & Payment System

W1-2: Payment processing live (Stripe or ckBTC)
W2-3: Security audit (external firm)
W3: Attorney review + compliance docs
W4: Bug fixes + performance optimization

Target: Ready for mainnet launch by April 15

### Month 3: Mainnet Launch

W1: Deploy to mainnet
W2: User acquisition (Twitter, Discord, grants)
W3-4: Scale ops + iterate

Revenue goal: 50+ paying users = $500/month MRR
```

**Deliverable by end of Week 4:**
- [ ] Alpha feedback summary (1-2 pages)
- [ ] Mainnet readiness checklist (with cost estimates)
- [ ] Grant application draft (ready to submit)
- [ ] Month 2-3 roadmap (published to team)
- [ ] Testnet metrics snapshot

---

## Month 1 Success Metrics

| Metric | Target | Notes |
|--------|--------|-------|
| **Specification** | 100% complete | Locked design doc, no surprises |
| **Alpha testers** | 5-10 active | Engaged, providing feedback |
| **Testnet deployment** | Success | Both canisters live, accessible |
| **Execution success rate** | 95%+ | Most DCA runs succeed (allow 1-2 failures) |
| **User feedback sentiment** | Positive | 4+ out of 5 for "would recommend" |
| **Security audit** | Draft checklist | Identify 0-2 critical issues |
| **Grant application** | Submitted | DFINITY feedback received |
| **Code quality** | Audit-ready | Well-documented, no obvious exploits |
| **Pricing validation** | Confirmed | Users say $9.99/mo is fair |
| **Attorney review** | Initiated | Legal counsel engaged, advice incorporated |

---

## Risks & Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Price feed unreliable (HTTP outcall timeout) | Medium | Medium (fail-safe: use cached price) | Test failure modes Week 2, implement fallback |
| DEX liquidity insufficient for orders | Low | Medium (use different DEX, reduce order size) | Research ic-swap liquidity pools, have backup |
| User recruitment slow (0-2 testers) | Low | High (can't validate product) | Proactive recruiting Week 2, incentivize with lifetime discount |
| Mainnet legal complexity (fintech regs) | Medium | High (can't launch mainnet) | Consult attorney by end of Month 1, budget $2-5K |
| Grant rejected | Medium | Low (still have product, need to self-fund) | Apply to multiple programs, have non-grant plan |
| Crypto market crashes (user interest drops) | Low | Medium (pivot to different product) | This is a feature-market fit issue; data will show Month 1 |

---

## Dependencies & Assumptions

**Assumptions:**
- ICP HTTP outcall is stable (has been for 12+ months)
- ic-swap liquidity available for orders
- Users have testnet ICP available (faucet works)
- Stripe sandbox works for MVP (or defer to Month 2)

**Dependencies:**
- DFINITY SDK (dfx) working on dev machine
- Rust toolchain stable
- GitHub for code storage
- Discord for user communication

---

## Revenue Model & Unit Economics

### Month 1 (Alpha)
- Price: Free
- Users: 5-10
- Revenue: $0

### Month 2 (Mainnet Prep)
- Price: $9.99/month
- Expected users: 10-20
- MRR: $100-200

### Month 3 (Mainnet + Growth)
- Price: $9.99/month (or tiered: $5 / $10 / $20)
- Expected users: 50-100
- MRR: $500-1000

### Year 1 (Mature)
- Price: $9.99-19.99/month
- Expected users: 500-2000
- ARR: $60K-240K

**CAC (Customer Acquisition Cost):**
- Month 1-2: Organic (word-of-mouth, DFINITY buzz): $0
- Month 3+: Paid ads + Twitter: $20-50 per user (depends on spend)

**Payback period:** 2-5 months (for acquisition at scale)

---

## Key Files & Locations

- **Spec:** `/home/node/.openclaw/workspace/agents/dca-spec.md`
- **Repo:** `https://github.com/ranchersurfer/icp-dca-agent` (git clone to workspace)
- **Candid UI:** (Published once deployed to testnet)
- **Alpha Feedback:** Google Form responses, Discord logs
- **Grant Application:** Draft in Google Docs, submit via DFINITY portal

---

## Decision Log

| Decision | Date | Rationale | Owner |
|----------|------|-----------|-------|
| ckBTC DCA (vs other agents) | Feb 27 | Recurring revenue + grant-friendly + proven market | Strategy-Planner |
| Rust (vs Motoko) | Feb 27 | Performance + ecosystem + Moises preference | Strategy-Planner |
| 3 canisters (vs monolith) | Feb 27 | Separation of concerns, easier upgrades | Strategy-Planner |
| Coingecko primary feed | Feb 27 | Free tier sufficient, reliable, 100+ req/min | Strategy-Planner |
| Stripe deferred to Month 2 | Feb 27 | Faster alpha launch, validate product fit first | Strategy-Planner |

---

## Document History

- **Created:** Feb 27, 2026 (Strategy-Planner subagent)
- **Last updated:** Feb 27, 2026
- **Owner:** Moises (subject to weekly iteration)
