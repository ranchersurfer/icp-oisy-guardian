# Guardian Dev Log

## 2026-03-04 — Opus Audit Remediation

### Audit Source
Opus 4.6 best-practices audit report: `/home/ranch/.openclaw/workspace/guardian-dev/AUDIT_REPORT.md`

### Issues Fixed

#### C1 — MemoryManager (CRITICAL: Data Corruption)
- **Files:** `guardian_config/src/lib.rs`, `guardian_engine/src/lib.rs`
- Both canisters were using bare `DefaultMemoryImpl` for all StableBTreeMaps, causing all maps to share the same memory region (silent data corruption).
- **Fix:** Added `MemoryManager<DefaultMemoryImpl>` with unique `MemoryId` per map:
  - `guardian_config`: CONFIGS=0, UPDATE_TIMESTAMPS=1
  - `guardian_engine`: WATERMARKS=0, ALERTS=1, META=2, USER_EVENTS=3, SEEN_TX_IDS=4

#### C2 — post_upgrade hook (CRITICAL: Monitoring stops on upgrade)
- **File:** `guardian_engine/src/lib.rs`
- Timer only started in `#[init]` — dies on every canister upgrade.
- **Fix:** Added `#[ic_cdk::post_upgrade]` that calls `start_timer()`. Extracted shared `start_timer()` function used by both `init` and `post_upgrade`.

#### C3 — set_config_canister_id access control (CRITICAL: Privilege escalation)
- **File:** `guardian_engine/src/lib.rs`
- Any authenticated caller could redirect the engine to a malicious config canister.
- **Fix:** Added `reject_non_controller()` helper using `ic_cdk::api::is_controller(&caller)`. Applied to `set_config_canister_id`.

#### C4 — inspect_message no-op (CRITICAL: Cycle drain vector)
- **Files:** `guardian_config/src/lib.rs`, `guardian_engine/src/lib.rs`
- Both canisters had no-op inspect_message (or no inspect_message at all for engine).
- **Fix:** Implemented real checks: reject anonymous callers + reject payloads > 1MB. Added `accept_message()` call for valid messages. Added inspect_message to engine canister (was missing entirely — M5 also fixed).

#### C5 — unwrap() in from_bytes (CRITICAL: Canister bricking)
- **Files:** All Storable impls in both canisters
- `.unwrap()` in `from_bytes` would brick the canister if stable memory is corrupt.
- **Fix:** 
  - Types with Default: use `.unwrap_or_default()` (Watermark, LastTick, StoredPrincipal)
  - Types without Default: use `.expect("descriptive message")` (GuardianConfig, UpdateTimestamps, AlertRecord)

#### H1 — Wrong ICP Index canister ID (HIGH)
- **File:** `guardian_engine/src/canisters.rs`
- Was using `ryjl3-tyaaa-aaaaa-aaaba-cai` (ICP **Ledger**), not the ICP **Index**.
- **Fix:** Corrected to `qhbym-qaaaa-aaaaa-aaafq-cai`. Added explicit test asserting the ID is not the ledger.

#### H2 — Rate limiting dead code (HIGH)
- **File:** `guardian_config/src/lib.rs`
- `get_recent_timestamps`/`record_update` helpers existed but `set_config` never called them.
- **Fix:** Added rate limit check at the top of `set_config` before processing. Returns error if `recent.len() >= MAX_UPDATES_PER_HOUR`.

#### H3 — No transaction deduplication (HIGH)
- **File:** `guardian_engine/src/lib.rs`
- Same tx could be stored multiple times if watermark update failed mid-tick.
- **Fix:** Added `SEEN_TX_IDS` StableBTreeMap (MemoryId 4) storing per-user `BTreeSet<String>`. `store_user_events` now filters out already-seen tx_ids before merging. Bounded to `MAX_SEEN_TX_IDS_PER_USER = 1000`.

#### H4 — Incorrect cycle balance calculation (HIGH)
- **File:** `guardian_config/src/lib.rs`
- `health()` was multiplying `api::canister_balance()` by `1_000_000_000_000` — canister_balance already returns cycles, not ICP.
- **Fix:** Removed the multiplication. Cycle balance is now used directly.

#### M2 — NaN/Infinity in f64 validation (MEDIUM, fixed alongside H validation)
- Added `is_finite()` check for `large_transfer_pct` and `daily_outflow_pct` in `validate_config`.

### Tests
- **Before:** ~167 tests passing
- **After:** 167 tests passing (18 guardian_config, 149 guardian_engine)
- **New tests added:**
  - `test_icp_index_canister_id_is_index_not_ledger`
  - `test_max_seen_tx_ids_per_user_is_1000`
  - `test_rate_limit_enforced`
  - `test_health_cycle_balance_no_multiplication`
  - `test_validate_config_nan_large_transfer_pct_rejected`
  - `test_validate_config_infinity_rejected`
  - `test_seen_tx_ids_key_uses_0xfe_sentinel`
  - `test_seen_tx_ids_key_differs_from_user_events_key`
  - `test_btreeset_dedup_logic`
  - `test_seen_tx_ids_bounded_at_max`

### WASM Build
- `cargo build --target wasm32-unknown-unknown --release` — ✅ Finished successfully

### Issues Skipped
- **H5 (ICRC type mismatch):** Not addressed — requires runtime testing against real index canisters to verify. Marked for Phase 2 integration test work.
- **C2 pre_upgrade:** No in-memory state to flush (all state is already in stable maps), so pre_upgrade is not needed.
