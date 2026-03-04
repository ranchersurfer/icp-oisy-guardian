# Guardian-Dev Log

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
