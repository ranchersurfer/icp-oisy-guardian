# Guardian-Dev: Phase 1 MVP Development Plan (Integrated from Existing Codebase)

**Project:** OISY Guardian (Wallet Safety + Transaction Guardian for ICP)  
**Scope:** Phase 1 MVP (devnet + local testing + ICP mainnet)  
**Timeline:** 4 weeks (Mar 2 – Apr 1, 2026)  
**Agent:** Guardian-Dev (Sonnet planning, Opus coding)  
**Status:** INTEGRATION PHASE (existing skeleton exists, needs completion)

---

## Integration Note (Mar 2, 2026)

**Good news:** The previous agent created a solid skeleton:
- ✅ dfx project structure (`dfx.json`, `Cargo.toml`)
- ✅ Guardian Config Canister (Rust, stable structures, basic CRUD)
- ✅ `canister_inspect_message` (anonymous check)
- ✅ Full schema matching spec (owner, thresholds, alert channels, etc.)
- ✅ Git repo with clean history

**Still needed (Phase 1a–1e):**
- [ ] Rate limiting (max 10 updates/hour per principal)
- [ ] Full validation (bounds checking, oversized payloads)
- [ ] Guardian Engine Canister (timer loop, monitoring)
- [ ] Detection engine (rule evaluation)
- [ ] Alert dispatch (HTTPS outcalls)
- [ ] Comprehensive tests (85%+ coverage)

**Codebase location:** `/home/ranch/.openclaw/workspace/guardian-icp/`

---

## Phase 1 Scope: Minimum Viable Product

### Core Features (MVP)
1. **Guardian Config Canister** (Rust) ✅ Skeleton exists
   - Store guardian configuration for each wallet
   - Define transaction rules (spending limits, recipient whitelist, time-based restrictions)
   - Validate transactions against rules
   - Logging & audit trail

2. **OISY Integration Points** ⏳ To be implemented
   - Read OISY wallet state (balance, transaction history)
   - Hook into OISY transaction flow (pre-signing validation)
   - Secure key derivation (guardian-specific credentials separate from primary wallet keys)

3. **Simple GUI (Frontend)** ❌ Not started
   - Admin dashboard (read-only view of rules + logs)
   - Rule configuration interface (set limits, manage whitelist)
   - Transaction history viewer
   - Status page (guardian health, last update)

4. **Core Security** ✅ Partially done
   - Canister inspection messages (non-certified queries)
   - Stable storage (persist rules across upgrades)
   - Basic encryption for sensitive data (no HSM in MVP)
   - Minimal external dependencies

### Out of Scope (Phase 2+)
- Multi-sig approvals (requires consensus, defer)
- ML-based anomaly detection (research phase)
- Hardware security modules (too complex for MVP)
- Mainnet deployment (devnet only)
- Advanced biometric/2FA (authentication layer, defer)

---

## Development Phases

### Phase 1a: Config Canister Hardening (Week 1, Mar 2–8)

**Starting point:** Config Canister skeleton exists

**Deliverables:**
- [ ] Add rate limiting: max 10 config updates per hour per principal
  - Tracking: Use stable memory counter per (principal, hour)
- [ ] Add validation:
  - [ ] Bounds check all fields (alert_channels max 5, allowlist max 500)
  - [ ] Reject oversized payloads in `canister_inspect_message` (>1MB)
  - [ ] Validate principal format (owner must be non-anonymous)
  - [ ] Validate thresholds (percentages 0–100, durations sensible)
- [ ] Improve error handling:
  - [ ] Return descriptive errors (not just "Error")
  - [ ] Log all rejections for audit trail
- [ ] Add cycle cost monitoring:
  - [ ] Track cycle balance via `ic0.canister_cycle_balance()`
  - [ ] Alert if below 30-day runway
- [ ] Build locally: `cargo build --target wasm32-unknown-unknown --release`
- [ ] Unit tests (20+ cases):
  - [ ] Valid config acceptance
  - [ ] Invalid config rejection
  - [ ] Rate limit enforcement
  - [ ] Caller authentication
  - [ ] Upgrade safety (data persistence)

**Owner:** Guardian-Dev (Opus for complex validation logic)  
**Acceptance:** Unit tests pass 100%, cargo build succeeds, no clippy warnings

---

### Phase 1b: Guardian Engine Canister Skeleton (Week 2, Mar 8–15)

**Deliverables:**
- [ ] Create new canister `guardian_engine` in same project
- [ ] Data structures:
  - [ ] `UnifiedEvent` (chain, timestamp, direction, amount_usd, counterparty, tx_id)
  - [ ] `AlertRecord` (alert_id, timestamp, user, rules_triggered, severity, status)
  - [ ] `Watermark` (per-user per-chain tracking of last checked tx ID / block)
- [ ] Timer setup:
  - [ ] `timer_tick()` function (runs every 30s)
  - [ ] Load user configs from Config Canister
  - [ ] For each user, initialize monitoring cycle (placeholder)
- [ ] Stable storage:
  - [ ] Watermarks map: `StableBTreeMap<(Principal, Chain), Watermark>`
  - [ ] Alert history ring buffer: last 1,000 alerts per user
- [ ] Basic health check:
  - [ ] `get_health() -> HealthStatus` query
  - [ ] Reports cycle balance, last timer run, user count
- [ ] Unit tests (15+ cases):
  - [ ] Timer initialization
  - [ ] Watermark read/write
  - [ ] Stable storage upgrade safety
  - [ ] Inter-canister call error handling

**Owner:** Guardian-Dev (Sonnet for planning, Opus for timer/storage patterns)  
**Acceptance:** Canister builds, health query returns OK, no panics on startup

---

### Phase 1c: ICRC Index Integration (Week 3, Mar 15–22)

**Deliverables:**
- [ ] Implement transaction fetching for ICP chain:
  - [ ] Call ICRC Index Canister for user's account
  - [ ] Parse returned transactions (amount, to/from, timestamp, memo)
  - [ ] Update watermark after successful query
- [ ] Implement transaction fetching for ckBTC:
  - [ ] Same as ICP, but query ckBTC ledger index
- [ ] Implement transaction fetching for ckETH:
  - [ ] Same as ICP/ckBTC
- [ ] Error handling:
  - [ ] Handle inter-canister rejects (SYS_UNKNOWN, CANISTER_ERROR)
  - [ ] Retry logic with exponential backoff
  - [ ] Log failures for operator
- [ ] Feed into detection engine (placeholder, log transactions only)
- [ ] Integration tests (20+ cases):
  - [ ] Mock ICRC Index responses
  - [ ] Watermark incrementing
  - [ ] Error scenarios (timeout, malformed response)
  - [ ] Large transaction batches (1000+ txs)

**Owner:** Guardian-Dev (Sonnet for inter-canister patterns)  
**Acceptance:** Can query mock ICRC index, watermarks persist across upgrades, tests pass

---

### Phase 1d: Detection Engine (Week 4, Mar 22–29)

**Deliverables:**
- [ ] Implement rule evaluation:
  - [ ] A1: Large outgoing transfer (>50% of balance)
  - [ ] A3: Rapid successive transactions (>5 in 10 min)
  - [ ] A4: New destination address
- [ ] Implement severity scoring:
  - [ ] Score = Σ(rule_weight × severity_multiplier)
  - [ ] INFO=1, WARN=3, CRITICAL=7, EMERGENCY=15
  - [ ] Compare against user's alert_threshold (default 7)
- [ ] Alert payload formatting per spec (Section 6.2)
- [ ] Placeholder for HTTPS outcalls (will implement in Phase 2)
- [ ] Unit tests (30+ cases):
  - [ ] Each rule in isolation
  - [ ] Score calculation
  - [ ] Alert filtering by threshold
  - [ ] Edge cases (empty txs, 0 balances, new users)

**Owner:** Guardian-Dev (Opus for complex rule logic, Sonnet for scoring)  
**Acceptance:** All unit tests pass, 85%+ coverage, rules correctly identify signals

---

### Phase 1e: Testing & Local Deployment (Week 4–5, Mar 29–Apr 1)

**Deliverables:**
- [ ] Local dfx setup:
  - [ ] Start local replica: `dfx start`
  - [ ] Deploy both canisters locally
  - [ ] Seed with mock data (test users, configs, transactions)
- [ ] Integration tests (50+ total):
  - [ ] Config canister + Engine canister interaction
  - [ ] Full monitoring cycle (load config → query mock index → detect rules → log alerts)
  - [ ] Upgrade scenario: deploy v1 → populate → upgrade to v2 → verify data
  - [ ] Rate limit enforcement under load
  - [ ] Cycle cost tracking
- [ ] Security review:
  - [ ] No hardcoded secrets
  - [ ] All inputs validated
  - [ ] Error messages don't leak sensitive info
  - [ ] `canister_inspect_message` rejects junk
- [ ] Documentation:
  - [ ] README with setup + deployment steps
  - [ ] Canister API reference (auto-generated from Candid)
  - [ ] Rule configuration guide
- [ ] Commit & tag:
  - [ ] `git commit -m "feat: Phase 1 MVP complete - local deployment working"`
  - [ ] `git tag v0.1-mvp`
  - [ ] Push to origin

**Owner:** Guardian-Dev + Moises (code review)  
**Acceptance:** All tests pass locally, README complete, no panics, ready for mainnet

---

## Development Tools & Environment

### Required
- **dfx** (0.30.2+) — ICP canister SDK  ✅ Already installed
- **Rust** (1.70+) — for canister development ✅
- **Git** — version control ✅
- **Node.js** (18+) — for frontend build (Phase 2)

### Optional (but recommended)
- **Docker** — Reproducible environment
- **GitHub Actions** — CI/CD for tests
- **Prettier / ESLint** — Code formatting

### Setup Checklist (for Guardian-Dev)
- [ ] `cd /home/ranch/.openclaw/workspace/guardian-icp`
- [ ] `export PATH="/home/ranch/.local/share/dfx/bin:$PATH"`
- [ ] `dfx --version` → should show 0.30.2 or similar
- [ ] `cargo --version` → should show 1.70+
- [ ] `dfx start` → local replica runs
- [ ] `cargo build --target wasm32-unknown-unknown --release` → builds without errors
- [ ] `dfx deploy guardian_config` → deploys locally
- [ ] `dfx canister call guardian_config health` → returns `(ok)` or health status

---

## Risk Mitigation

| Risk | Likelihood | Mitigation |
|------|------------|-----------|
| **Scope creep** | High | Stick to Phase 1 features only. Defer Phase 2+ features. |
| **ICRC Index API changed** | Low | Use ICRC standards (stable interfaces). Monitor OISY/DFINITY GitHub. |
| **Rust compilation errors** | Medium | Build frequently (daily). Use `cargo clippy` for warnings. |
| **Test data unrealistic** | Medium | Use synthetic datasets matching real OISY behavior. Load-test with 1000+ users. |
| **Cycle budget exceeded** | Low | Monitor cycles constantly. Alert at 30-day runway. Use `freezing_threshold` 90+ days. |

---

## Success Criteria (Phase 1)

✅ **Code Quality:**
- [ ] 85%+ test coverage (config + engine canisters)
- [ ] No hardcoded secrets or credentials
- [ ] Clean architecture (separation of concerns)
- [ ] Zero clippy warnings

✅ **Functionality:**
- [ ] Config Canister: full CRUD, validation, rate limiting
- [ ] Engine Canister: timer loop, ICP/ckBTC/ckETH monitoring
- [ ] Detection: A1, A3, A4 rules working correctly
- [ ] Alert payloads formatted per spec
- [ ] Stable storage persists across upgrades

✅ **Operationability:**
- [ ] Deploys locally without errors
- [ ] README with setup + testing instructions
- [ ] Health check endpoint responds
- [ ] Cycle balance monitoring works

✅ **Testing:**
- [ ] 100+ unit tests pass
- [ ] 50+ integration tests pass
- [ ] Load test with 1000 users (synthetic)
- [ ] Upgrade path verified (v1 → v2)

---

## Files & Structure

```
/home/ranch/.openclaw/workspace/guardian-icp/
├── src/
│   ├── lib.rs                  # Config Canister (core logic)
│   ├── engine.rs               # Engine Canister (new, Phase 1b+)
│   ├── detection.rs            # Rule evaluation (new, Phase 1d)
│   ├── guardian.did            # Candid interface (Guardian Config)
│   ├── engine.did              # Candid interface (Engine)
│   └── tests/
│       ├── config_tests.rs
│       ├── engine_tests.rs
│       └── integration_tests.rs
├── Cargo.toml
├── dfx.json
├── .dfx/                       # Local replica state
├── target/                     # Build artifacts
├── README.md                   # Setup + usage guide
├── DEV_LOG.md                  # This file + updates
└── OISY_GUARDIAN_SPEC.md       # Reference: full spec

Reference files (linked):
├── /home/ranch/.openclaw/workspace/guardian-dev/
│   ├── DEV_PLAN.md             # This document
│   ├── DEV_LOG.md              # Running changelog
│   ├── CANISTER_MANIFEST.md    # Canister IDs (devnet/mainnet)
│   └── OISY_GUARDIAN_SPEC.md   # Full spec (40KB)
```

---

## Timeline Summary

```
Week 1 (Mar 2–8):   Phase 1a — Config Canister hardening
Week 2 (Mar 8–15):  Phase 1b — Engine Canister skeleton
Week 3 (Mar 15–22): Phase 1c — ICRC Index integration
Week 4 (Mar 22–29): Phase 1d — Detection engine
Week 5 (Mar 29–Apr 1): Phase 1e — Testing + local deployment ✅

Expected completion: Apr 1, 2026
```

---

## Next Action (Today, Mar 2)

1. ✅ Spec integrated (`OISY_GUARDIAN_SPEC.md`)
2. ✅ Codebase integrated from existing project
3. ⏳ Guardian-Dev reviews this plan
4. ⏳ Set up dfx environment + run first build
5. ⏳ Create GitHub branch for Phase 1 work
6. ⏳ Begin Phase 1a (config validation + rate limiting)

**Who:** Guardian-Dev  
**When:** Starting today (Mar 2)  
**Where:** `/home/ranch/.openclaw/workspace/guardian-icp/`  
**Model:** Sonnet for planning, Opus for complex Rust patterns  
**Constraints:** Follow Section 9 (ICP Security) religiously

---

**Created:** 2026-02-28  
**Updated:** 2026-03-02 (integrated existing codebase)  
**Next Review:** 2026-03-08 (end of Phase 1a)
