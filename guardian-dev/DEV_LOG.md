# Guardian-Dev Phase 1 Development Log

## Timeline
- **Phase 1a Deadline:** March 8, 2026
- **Start Date:** March 2, 2026
- **Status:** IN PROGRESS

---

## Daily Commits & Progress

### March 2, 2026 – Day 1: Setup + Skeleton Review

**Completed:**
- ✅ Read QUICKSTART.md
- ✅ Set up environment (PATH, Rust, dfx)
- ✅ Verified dfx version 0.30.2
- ✅ Started local replica (dfx start --clean)
- ✅ Built project (cargo build --target wasm32-unknown-unknown --release)
- ✅ Deployed guardian_config locally
- ✅ Tested health endpoint
- ✅ Fixed Storable trait implementation for GuardianConfig
- ✅ Fixed Cargo.toml duplicate bin/lib conflict
- ✅ Read DEV_PLAN.md and OISY_GUARDIAN_SPEC.md sections 7-9

**Current Status:**
- guardian_config canister deployed locally: `uxrrr-q7777-77774-qaaaq-cai`
- Health check working: `"Guardian OK. Time: ..."`
- Ready for Phase 1a implementation

**Next Steps:**
1. Implement rate limiting (max 10 updates/hour per principal)
2. Add comprehensive input validation
3. Enhance canister_inspect_message
4. Add cycle cost monitoring
5. Write 20+ unit tests

---

## Phase 1b Implementation Log

### March 2, 2026 – Guardian Engine Canister Skeleton

**Completed:**
- ✅ Converted project to Cargo workspace (`src/guardian_config/`, `src/guardian_engine/`)
- ✅ Created `guardian_engine` crate with all required data structures
- ✅ Data structures: `UnifiedEvent`, `AlertRecord`, `Watermark`, `Chain` enum, `Direction` enum, `AlertStatus` enum
- ✅ Stable storage: `StableBTreeMap<WatermarkKey, Watermark>` and `StableBTreeMap<String, AlertRecord>`
- ✅ `WatermarkKey` struct: 30-byte stable key (29 principal bytes + 1 chain discriminant)
- ✅ Timer: `set_timer_interval(30s, timer_tick)` started in `#[init]`
- ✅ Timer tick: logs timestamp, checks cycle balance, updates `last_tick` in stable memory
- ✅ Health endpoint: `#[query] get_health() -> EngineHealthStatus` (cycle_balance, last_tick, is_running, watermark_count)
- ✅ Inter-canister interface: `#[update] set_config_canister_id(id)`, stub `fetch_user_configs()`
- ✅ Security: `reject_anonymous()` on all update calls, `guard_cycles()` rejects if balance < 500B
- ✅ Updated `dfx.json` with `guardian_engine` canister entry
- ✅ Created `src/guardian_engine.did` Candid interface
- ✅ **17 unit tests** covering: Watermark serialization, AlertRecord creation/transitions, Chain discriminants, WatermarkKey encoding/ordering, UnifiedEvent fields, EngineHealthStatus structure, timer flag, LastTick roundtrip
- ✅ All 31 tests pass (14 guardian_config + 17 guardian_engine)
- ✅ WASM build successful: `cargo build --target wasm32-unknown-unknown --release`
- ✅ Committed and pushed: `6f714bc`

**Commit:** `6f714bc` — "feat: Phase 1b - Guardian Engine Canister skeleton with timer, stable storage, health endpoint"

**Notes:**
- Warnings only (elided lifetime suggestions from rustc) — no errors
- `guardian_config` rate-limiting helpers (`get_recent_timestamps`, `record_update`) not yet wired to `set_config`; left as-is (Phase 1a concern)
- Phase 1c: wire `fetch_user_configs()` to actual inter-canister call

---

## Phase 1a Implementation Log

### Feature 1: Rate Limiting (Max 10 config updates/hour per principal)

**Plan:**
- Use `StableBTreeMap<Principal, Vec<u64>>` to track last 10 update timestamps per principal
- In `set_config()`, check if caller has >10 updates in last 3600 seconds
- Reject with descriptive error if limit exceeded
- Clean old timestamps on each call

**Status:** IN PROGRESS

---

## Known Blockers / Questions

None yet.

---

## Test Coverage Target

- Phase 1a: 20+ unit tests covering:
  - ✓ Rate limiting enforcement
  - ✓ All validation rules
  - ✓ Cycle monitoring
  - ✓ Authorization checks

---

## Git Commits

```
Mar 2: "fix: Storable implementation + build fixes for guardian_config"
Mar 3: "feat: Phase 1c - ICRC Index integration for ICP/ckBTC/ckETH transaction fetching" (bf7511f)
```

---

## Phase 1c Completion (Mar 3, 2026)

**Status:** ✅ COMPLETE  
**Test Results:** 48/48 passing (17 new tests added across 3 new modules)  
**Build Status:** ✅ `cargo build --target wasm32-unknown-unknown --release` - SUCCESS (warnings only)

### What Was Built

**`src/guardian_engine/src/icrc.rs`**  
- `IcrcAccount { owner: Principal, subaccount: Option<[u8;32]> }`  
- `IcrcTransaction { id, timestamp, amount, from, to, memo, kind }`  
- `GetTransactionsRequest` / `GetTransactionsResponse`  
- 8 unit tests for type construction, serialization, and edge cases

**`src/guardian_engine/src/canisters.rs`**  
- ICP Index: `ryjl3-tyaaa-aaaaa-aaaba-cai`  
- ckBTC Index: `n5wcd-faaaa-aaaar-qaaea-cai`  
- ckETH Index: `s3zol-vqaaa-aaaar-qacpa-cai` (corrected from spec - verified against dfinity/ic mainnet canister_ids.json)  
- `MAX_RESULTS_PER_FETCH = 100`, `MAX_EVENTS_PER_USER = 100`  
- 6 unit tests validating all canister IDs via `Principal::from_text`

**`src/guardian_engine/src/fetcher.rs`**  
- `icrc_tx_to_unified_event()` — converts ICRC transactions to `UnifiedEvent`  
- `fetch_icp_transactions()` / `fetch_ckbtc_transactions()` / `fetch_cketh_transactions()` — async inter-canister calls via `ic_cdk::call`  
- `update_watermark_after_fetch()` — advances watermark without regression  
- `merge_into_ring_buffer()` — LIFO trim ring buffer (max 100 events/user)  
- 17 unit tests (all cfg(test) mock-based, no IC runtime needed)

**`src/guardian_engine/src/lib.rs` updates:**  
- Added `pub mod canisters`, `pub mod fetcher`, `pub mod icrc`  
- Added `USER_EVENTS` stable BTreeMap (per-user ring buffer)  
- `timer_tick()` now spawns `run_fetch_cycle()` via `ic_cdk::spawn`  
- `run_fetch_cycle()` iterates all known principals → fetches all 3 chains → stores events → updates watermarks  
- `store_user_events()` and `user_events_key()` helpers  
- All async fetch code gated with `#[cfg(not(test))]` to avoid IC runtime dependency in tests

### Security Notes
- All fetch errors logged + return empty Vec (no panics)
- Cycle drain guard (500B) enforced on all update endpoints
- Anonymous caller rejection on all update endpoints
- No mainnet deployment (local/devnet only)

**Last Updated:** 2026-03-03 PST  
**Next Review:** 2026-03-22 (Phase 1c deadline) / Phase 1d start

---

## March 3, 2026 – Phase 1d: Detection Engine

**Completed:**
- ✅ Created `src/guardian_engine/src/detector.rs` — Rule evaluation engine
  - `rule_a1_large_transfer()` — triggers when outgoing tx > 50% of estimated balance (weight 7, CRITICAL)
  - `rule_a3_rapid_transactions()` — triggers when >5 outgoing txs within any 10-minute window (weight 3, WARN)
  - `rule_a4_new_address()` — triggers when counterparty not in allowlisted_addresses (weight 1, INFO)
  - `Severity` enum: Info/Warn/Critical/Emergency with `from_score()` mapping
  - `evaluate(DetectionContext)` → `DetectionResult { score, severity, rules_triggered, should_alert }`
- ✅ Created `src/guardian_engine/src/alerts.rs` — Alert payload formatting
  - `format_alert(user, result, events, timestamp)` → `AlertPayload`
  - UUID-style `alert_id` derived from timestamp + principal bytes
  - `recommended_action` keyed to severity level
  - Stores `AlertRecord` in stable ALERTS map
- ✅ Wired detection into `run_fetch_cycle()` in lib.rs:
  - After all chains fetched for a user, loads stored events, estimates balance
  - Runs `evaluate()` with default threshold 7
  - If `should_alert` → calls `format_alert()` → stores → logs "ALERT: {severity} for {user}"
  - Placeholder comment for Phase 2 HTTPS outcall
- ✅ Added `load_user_events()` helper
- ✅ 81 unit tests total (all passing), including 35 new Phase 1d tests:
  - A1: large/small/exactly-50%/51%/zero-balance/incoming/empty
  - A3: 5tx/6tx/6tx-outside-window/empty/incoming-only/4tx
  - A4: known/unknown/incoming/empty
  - Scoring: A1-only=7, A3-only=3, A1+A3=10, all-three=11
  - Threshold: score≥threshold→alert, score<threshold→no alert
  - Severity: Info/Warn/Critical/Emergency from score ranges
  - Alert payload: fields populated correctly, no-events summary
  - Edge cases: empty events, zero balance, no rules triggered

**Build status:** ✅ WASM release build successful (5 warnings, 0 errors)
**Test count:** 81/81 passing
**New files:** detector.rs, alerts.rs
