# Guardian-Dev Log

## Phase 2c: Config Canister Sync — 2026-03-04

### Session: guardian-phase2c (Subagent)
**Time**: 2026-03-04 12:00 PST  
**Duration**: ~45 minutes  
**Status**: ✅ COMPLETE

---

### What Was Implemented

#### TASK 1: Inter-Canister Call Setup (Query Config Canister)

**`src/guardian_engine/src/lib.rs`**:
- Added `UserChannelEntry` struct (Storable) with `channels: Vec<AlertChannel>` + `cached_at: u64`
- Added `CHANNEL_CACHE_TTL_NS = 300 * 1_000_000_000` (5-minute cache expiry)
- Added `USER_ALERT_CHANNELS: StableBTreeMap<String, UserChannelEntry, Memory>` (MemoryId 6)
- Added `get_cached_channels(user, now_ns)` — checks cache validity (< 5 min TTL)
- Added `store_cached_channels(user, channels, now_ns)` — writes to stable cache
- Added `channel_cache_len()` — utility for test/monitoring
- Added `fetch_user_alert_channels(user)` (non-test async):
  - Checks cache first (cache hit path)
  - Calls `get_config_for_user(user)` on config canister on cache miss
  - Retry logic: up to 3 attempts with IC round-trip backoff between each
  - On error/not-found: caches empty vec and returns gracefully (fail-open)
- Test stub: always returns `vec![]` (no IC runtime in tests)

**`src/guardian_engine/src/canisters.rs`**:
- Added `ApiResult<T>` enum (mirrors config canister's ApiResult for decoding)
- Added `GuardianConfigChannels` struct (minimal projection: only `alert_channels: Vec<String>`)

**`src/lib.rs` (config canister)**:
- Added `get_config_for_user(user: Principal) -> ApiResult<GuardianConfig>` query endpoint
- Controller-only access (only the guardian_engine or operator can call this)

#### TASK 2: Alert Routing

**`src/guardian_engine/src/lib.rs`**:
- Added `run_per_user_delivery_drain(max_items)` (non-test async):
  - Dequeues alerts from ALERT_QUEUE
  - Fetches user-specific channels via `fetch_user_alert_channels()`
  - If no channels: logs "no channels configured for user X", re-enqueues or marks Failed
  - If channels exist: calls `deliver_to_channel()` for EACH channel
  - Marks alert Sent if ANY channel succeeds; Failed only if permanent error or retries exhausted
- Updated `delivery_tick()` to call `run_per_user_delivery_drain(10)` instead of the previous flat-channel drain

#### TASK 3: Config Sync Tick

**`src/guardian_engine/src/lib.rs`**:
- Added `config_sync_tick()` function — spawns `config_sync_async()`
- `config_sync_async()` iterates all active users from WATERMARKS, calls `fetch_user_alert_channels()` for each, caches results
- Logs: "Synced channels for N users, M total channels"
- Registered in `start_timer()` as a 300s timer (cfg(not(test)))

---

### Test Results

| Metric | Before Phase 2c | After Phase 2c |
|--------|----------------|----------------|
| Tests  | 237            | 262            |
| Phase 2c tests | 0       | 25             |
| Clippy warnings | 0    | 0              |
| Failures | 0            | 0              |

**New tests added (25)**:
- `test_user_channel_entry_default_is_empty`
- `test_user_channel_entry_storable_roundtrip`
- `test_user_channel_entry_storable_empty_channels`
- `test_user_channel_entry_all_channel_types`
- `test_channel_cache_ttl_is_5_minutes_in_ns`
- `test_get_cached_channels_returns_none_when_empty`
- `test_store_and_get_cached_channels_fresh_entry`
- `test_fetch_user_channels_cache_invalidation`
- `test_cache_still_valid_at_exactly_ttl_minus_1`
- `test_cache_expired_exactly_at_ttl`
- `test_store_channels_overwrites_previous_entry`
- `test_channel_cache_len_increments`
- `test_no_channels_configured_skips_delivery`
- `test_delivery_to_multiple_channels_routing`
- `test_inter_canister_call_retry_on_transient_error`
- `test_retry_stops_after_max_attempts`
- `test_retry_succeeds_on_second_attempt`
- `test_different_users_have_independent_channel_caches`
- `test_cache_key_is_principal_text`
- `test_empty_channel_list_cached_separately`
- `test_cache_with_five_channels_max`
- `test_alert_not_sent_when_all_channels_fail`
- `test_alert_sent_when_at_least_one_channel_succeeds`
- `test_channel_cache_ttl_boundary_just_before_expiry`
- `test_channel_cache_ttl_boundary_at_expiry`

---

### Files Changed

| File | Change |
|------|--------|
| `src/guardian_engine/src/lib.rs` | UserChannelEntry, USER_ALERT_CHANNELS, cache helpers, fetch_user_alert_channels, run_per_user_delivery_drain, config_sync_tick, 25 tests |
| `src/guardian_engine/src/canisters.rs` | ApiResult<T>, GuardianConfigChannels types |
| `src/lib.rs` (config canister) | get_config_for_user() query endpoint |
| `guardian-dev/DEV_LOG.md` | This entry |

---

### Architecture Notes

- **Cache invalidation**: TTL-based, 5 minutes. On miss: inter-canister call with 3 retries.
- **Fail-open policy**: If config canister unreachable, channels = [] and alert is re-queued (not dropped permanently until retry_count ≥ MAX_RETRIES=3).
- **Per-user independence**: Each user's channels are cached independently (String key = Principal text).
- **Controller-only config access**: `get_config_for_user` is restricted to controllers to prevent data leaks.
- **Config sync pre-warming**: 300s tick pre-fetches channels for all active users so delivery tick has warm cache.

---

### Known Phase 2c Limitations

1. **Retry delay**: IC canisters cannot sleep; retry "delay" is a self-call round-trip (~100ms natural latency)
2. **Config canister auth**: `get_config_for_user` requires guardian_engine to be a controller of guardian_config — must be set up during deployment
3. **Empty-channel caching**: On "config not found", we cache an empty vec for 5 min (avoids hammering, but delays detection if user creates config)
4. **No HMAC signing** for webhook secret (Phase 2d)

---

### Next Steps (Phase 2d)

- [ ] Testnet deployment with real cycles (dfx cycles convert)
- [ ] Set guardian_engine as controller of guardian_config on testnet
- [ ] Smoke test with real Discord webhook
- [ ] HMAC-SHA256 signing for Webhook channel secret
- [ ] Frontend dashboard (Phase 3)

---

**Guardian-Dev Status**: 🟢 Ready for Phase 2d

---

## Phase 2b: Alert Delivery via HTTPS Outcall — 2026-03-04

### Session: guardian-phase2b (Subagent)
**Time**: 2026-03-04 19:30 PST  
**Duration**: ~45 minutes  
**Status**: ✅ COMPLETE

---

### What Was Implemented

#### New Module: `src/guardian_engine/src/delivery.rs`

Full HTTPS outcall delivery engine for Guardian alerts.

**Features:**
- `AlertChannel` enum: `Discord { webhook_url }`, `Slack { webhook_url }`, `Webhook { url, secret }`, `Email { address, api_url, api_key }`
- `AlertChannel::from_str_config()`: parses semicolon-delimited config strings into typed channels
- Payload builders for each channel type:
  - `build_discord_payload()` — Discord embed JSON with color-coded severity
  - `build_slack_payload()` — Slack Block Kit text with emoji severity indicators
  - `build_webhook_payload()` — Generic JSON with all alert fields
  - `build_email_payload()` — URL-encoded form body (Mailgun/SendGrid compatible)
- `DeliveryOutcome` enum: `Success`, `HttpError`, `TransportError`, `InsufficientCycles`, `InvalidConfig`
- `estimate_outcall_cycles(request_bytes, max_response_bytes) -> u128` — IC cost formula
- `deliver_to_channel()` async (non-test): makes real HTTPS outcall via IC management canister
- `run_delivery_drain(max_items, channels)`: drains alert queue, retries on transient failure, marks Sent/Failed in stable ALERTS map
- `transform_response()`: deterministic HTTP response transform (strips headers)
- `escape_json()` and `url_encode()` helpers
- Retry policy: max 3 attempts; 4xx/invalid config = permanent failure; 5xx/transport = retry

#### `AlertQueueItem` enriched (alert_queue.rs)
Added fields: `severity`, `severity_score`, `rules_triggered`, `events_summary`, `recommended_action`

#### `lib.rs` changes
- Added `pub mod delivery`
- Updated enqueue call to populate new `AlertQueueItem` fields
- Added 60s `delivery_tick()` timer (separate from 30s monitoring tick)
- Added `get_alert_queue_len()` query endpoint

#### `fetcher.rs` — pre-existing clippy fixes
- Fixed `redundant_closure` warnings on `filter_map` calls

---

### Test Results

| Metric | Before Phase 2b | After Phase 2b |
|--------|----------------|----------------|
| Tests  | 189            | 237            |
| Delivery tests | 0       | 48             |
| Clippy warnings | 2     | 0              |
| Failures | 0            | 0              |

---

### Cycle Cost Model (per outcall)
- Base fee: 49,140,000 cycles
- Request bytes: 5,200 cycles/byte
- Response bytes: 10,400 cycles/byte
- Typical webhook (~1KB req, 2KB resp): ~70M cycles
- Budget: 100M cycles per attempt (2x safety margin)

---

### Commit
`e58ab93` — feat: Phase 2b — Alert delivery via HTTPS outcall (Discord, Slack, webhook, email)

---

### Known Phase 2b Limitations

1. **Channels are currently empty** in `delivery_tick()` — will be populated from config canister in Phase 2c
2. **Email auth**: uses basic base64 encoding; production should use proper Mailgun SDK
3. **No HMAC signing** for webhook secret (uses plain header); production should sign with HMAC-SHA256
4. **No per-user channel routing**: all users share the same channels list (Phase 2c will load per-user configs)

---

### Next Steps (Phase 2c)

- [ ] Engine polls config canister for per-user alert channel settings
- [ ] Route alerts to each user's configured channels
- [ ] Testnet deployment with real cycles

---

**Guardian-Dev Status**: 🟢 Ready for Phase 2c



## Phase 2a: Testnet Deployment — 2026-03-04

### Session: guardian-phase2a (Subagent)
**Time**: 2026-03-04 10:38 PST  
**Duration**: ~1.5 hours  
**Status**: ✅ COMPLETE

---

### TASK 1: ICRC Type Verification (HARD BLOCKER)

**Objective**: Fetch and compare actual mainnet Candid types vs. internal type definitions.

**Findings**:

#### ICP Index (qhbym-qaaaa-aaaaa-aaafq-cai)
- **API**: Different from ckBTC/ckETH — uses `GetAccountIdentifierTransactionsResult` variant
- **Response structure**: Contains `GetAccountIdentifierTransactionsResponse` with balance (nat64), transactions, oldest_tx_id
- **Transactions**: Wrapped `TransactionWithId` containing `Operation` variant (Transfer, Mint, Burn, Approve)
- **Account IDs**: Text-based (hex AccountIdentifier), NOT Principals
  - **Limitation**: Cannot reliably convert text IDs to Principals without ICP Ledger's account map
  - **Solution**: Best-effort `Principal::from_text()` with fallback to `Principal::anonymous()`

#### ckBTC/ckETH Index NG (n5wcd-..., s3zol-vqaaa-...)
- **API**: Identical Candid structure between ckBTC and ckETH
- **Response**: `GetTransactionsResult = variant { Ok: GetTransactions; Err: GetTransactionsErr }`
- **Transaction structure**: Nested under `transaction.{transfer|mint|burn|approve}` fields
- **Type fixes required**:
  - `start: opt nat` (was expecting `opt u64`)
  - `max_results: nat` (was expecting `u64`)
  - `id: BlockIndex = nat` (was expecting direct u64)
  - Transfer/mint/burn nesting required careful deserialization

#### Changes Made
1. Added wire types: `IcrcTransactionWithIdWire`, `IcrcTransactionBodyWire`, `IcrcTransferWire`, etc.
2. Added ICP-specific types: `IcpOperation`, `IcpTransactionWithId`, `IcpGetTransactionsResult`
3. **Fixed `GetTransactionsRequest`**: `start: Option<Nat>`, `max_results: Nat`
4. Added conversion functions: `icrc_wire_to_internal()`, `icp_wire_to_internal()`
5. Updated fetcher with separate code paths for ICP vs. ckBTC/ckETH
6. **Commit**: `ee882d4` (combined with Task 2)

---

### TASK 2: Balance u128 Migration for ckETH

**Objective**: Fix u64 overflow for 18-decimal token values.

**Math Check**:
- 1000 ETH in Wei = 1000 × 10^18 = 10^21
- u64::MAX ≈ 1.8 × 10^19
- **Result**: Overflow by ~5500x, clearly unacceptable

**Changes Made**:
1. `IcrcTransaction.amount: u64` → `u128`
2. `UnifiedEvent.amount_e8s: u64` → `u128`
3. `DetectionContext.estimated_balance_e8s: u64` → `u128`
4. `DetectionContext.balance_e8s: Option<u64>` → `Option<u128>`
5. Updated `rule_a1_large_transfer()` to accept u128 balance
6. Updated `icrc1_balance_of` parsing to decode `Nat` as `u128`
7. Fixed balance arithmetic (`saturating_add`, `saturating_sub`) to use u128
8. Updated all test helpers (`make_out_event`, `make_in_event`, `make_tx`, etc.)
9. **Added tests**:
   - `test_cketh_balance_overflow_u64`: Demonstrates u64 overflow for 1000 ETH
   - `test_cketh_balance_u128_handles_1000_eth`: Verifies u128 correctly handles large values
10. **Result**: 189 total tests passing (187 existing + 2 new)
11. **Commit**: `ee882d4`

---

### TASK 3: Testnet Deployment

**Objective**: Deploy to testnet or document fallback.

**Testnet Attempt**:
```
Command: dfx canister create --all --network testnet
Error: Insufficient cycles balance to create the canister.
Advice: dfx cycles convert --amount=0.123 --network testnet
```
- **Status**: ⚠️ FAILED (as expected — default identity has no cycles)
- **Recovery**: Documented in CANISTER_IDS.md; can retry with `dfx cycles convert`

**Local Deployment** (Fallback):
- **Status**: ✅ SUCCESS
- **Replica**: Started at 127.0.0.1:4943 (clean)
- **Build**: cargo build released successfully
- **Install**: Both canisters installed without errors
- **Canister IDs**:
  - guardian_config: `uxrrr-q7777-77774-qaaaq-cai`
  - guardian_engine: `u6s2n-gx777-77774-qaaba-cai`

**Smoke Tests**:
- ✅ Engine health: Running, 0 watermarks, cycles available
- ✅ Config deployment: Deployment successful
- ✅ Config setter: Allows setting all required fields (with correct percentage bounds 0-1)
- ✅ Config getter: Retrieves full config correctly
- ✅ Config health: OK, 59 days until freeze
- ⚠️ Alert methods: Phase 2b stubs (not exported as expected)

**Changes Made**:
1. Updated `dfx.json`: Added testnet network config with icp0.io provider
2. Updated local network bind address to `127.0.0.1:4943` (standard)
3. **Commit**: `ee882d4` (included in ICRC + balance migration commit)

---

### TASK 4: Admin Viewer Script

**Objective**: Create `scripts/admin-view.sh` for operational debugging.

**Features**:
- Network-aware: `./scripts/admin-view.sh [local|testnet]`
- Config health display (cycle balance, status, days until freeze)
- Engine health display (cycles, last_tick, running status, watermark count)
- Alert queue status (handles Phase 2b stub gracefully)
- Cycle balance summaries for both canisters
- Watermark sync status

**Execution**: ✅ Tested on local deployment, all outputs working
**Commit**: `475f19f`

---

### TASK 5: README Update

**Objective**: Document Phase 2a completion in README.

**Changes Made**:
1. Added comprehensive "Phase 2a — Testnet Deployment" section
2. Documented all 5 TAB​Ks with status and commit hashes
3. Added smoke test walkthrough with exact commands
4. Listed known Phase 2a limitations clearly
5. Updated project version descriptor
6. **Commit**: `475f19f` (same as admin-view.sh)

---

### Status Files Updated

| File | Update |
|------|--------|
| `/home/ranch/.openclaw/workspace/agent-status.json` | guardian-dev: "idle", current_task: null |
| `/home/ranch/.openclaw/workspace/projects.json` | proj-guardian: "in_progress", progress: 55 |
| `/home/ranch/.openclaw/workspace/guardian-dev/CANISTER_IDS.md` | NEW — canister IDs + deployment info |
| `/home/ranch/.openclaw/workspace/guardian-icp/scripts/admin-view.sh` | NEW — admin viewer script |
| `/home/ranch/.openclaw/workspace/guardian-icp/README.md` | Phase 2a section added |

---

### Commit Summary

| Hash | Message |
|------|---------|
| `ee882d4` | fix: migrate balance fields to u128 for ckETH 18-decimal compatibility (+ TASK 1 ICRC types) |
| `475f19f` | feat: add admin-view.sh script for testnet debugging |

---

### Test Results

**Before Phase 2a**: 187 tests (guardian_engine)  
**After Phase 2a**: 189 tests  
**Added**: `test_cketh_balance_overflow_u64`, `test_cketh_balance_u128_handles_1000_eth`  
**Status**: ✅ All passing, 0 failures, 0 clippy warnings

---

### Known Phase 2a Limitations

1. **ICP AccountIdentifiers**: Text-based, not convertible to Principals without on-chain ICP ledger lookup
2. **Alert delivery**: `get_alert_queue`, `dequeue_alerts` are Phase 2b stubs
3. **Config sync**: Engine doesn't fetch configs from config canister yet (Phase 2c)
4. **Testnet cycles**: Requires `dfx cycles convert` before testnet deployment
5. **Subaccount encoding**: Changed `[u8; 32]` → `Vec<u8>` for wire compatibility; internal API still accepts arrays via `to_vec()`

---

### Next Steps (Phase 2b-2c)

- [ ] Implement alert delivery via HTTPS outcall (Phase 2b)
- [ ] Engine polls config canister for per-user settings (Phase 2c)
- [ ] Testnet deployment with real cycles
- [ ] Mainnet preparation

---

**Guardian-Dev Status**: 🟢 Ready for Phase 2b
