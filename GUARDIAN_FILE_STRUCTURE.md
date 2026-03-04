# Guardian ICP — File Structure

**Last Updated:** 2026-03-04 (Phase 2b complete)

## Source Tree

```
guardian-icp/
├── Cargo.toml                            # Workspace manifest
├── Cargo.lock                            # Dependency lock
├── dfx.json                              # Canister definitions (+ testnet config added Phase 2a)
├── README.md                             # Project documentation (Phase 2a section added)
├── scripts/
│   └── admin-view.sh                    # Admin viewer script (Phase 2a NEW)
└── src/
    ├── guardian.did                      # Config canister Candid interface
    ├── guardian_config/
    │   ├── Cargo.toml                    # guardian_config crate manifest
    │   └── src/
    │       └── lib.rs                    # Config canister (rate limiting, validation, cycle monitoring)
    ├── guardian_engine/
    │   ├── Cargo.toml                    # guardian_engine crate manifest
    │   └── src/
    │       ├── lib.rs                    # Engine canister (timer, watermarks, alerts, stable storage, 60s delivery tick Phase 2b)
    │       ├── alerts.rs                 # Alert payload formatting and storage
    │       ├── alert_queue.rs            # Persistent delivery queue (enriched fields Phase 2b)
    │       ├── canisters.rs              # ICRC canister IDs and fetch constants
    │       ├── delivery.rs               # *** NEW Phase 2b *** HTTPS outcall delivery (Discord/Slack/webhook/email)
    │       ├── detector.rs               # Rule evaluation engine (A1/A3/A4, u128 balance Phase 2a)
    │       ├── fetcher.rs                # ICRC transaction fetching + ring buffer (wire types Phase 2a)
    │       ├── icrc.rs                   # ICRC type definitions (wire types + conversions Phase 2a)
    │       └── integration_tests.rs      # Phase 1e integration tests (62 tests, u128 updates Phase 2a)
    ├── guardian_engine.did               # Engine canister Candid interface
    └── lib.rs                            # Workspace stub
```

## Phase Status

| Phase | Status | Tests | Commit |
|-------|--------|-------|--------|
| 1a: Config hardening | ✅ Complete | 14 tests | Mar 2 |
| 1b: Engine skeleton | ✅ Complete | 17 tests | Mar 2, `6f714bc` |
| 1c: ICRC integration | ✅ Complete | 48 tests | Mar 3, `bf7511f` |
| 1d: Detection engine | ✅ Complete | 81 tests | Mar 3 |
| 1e: Testing + local deploy | ✅ Complete | 157 tests | Mar 3, `8b45fdf` (v0.1-mvp) |
| **2a: Testnet + ICRC types** | ✅ Complete | **189 tests** | **Mar 4, `ee882d4`, `475f19f`** |
| **2b: Alert delivery HTTPS** | ✅ Complete | **237 tests** | **Mar 4, `e58ab93`** |
| 2c: Config canister sync | ⏳ Planned | — | TBD |

**Phase 2b Deliverables**:
- delivery.rs: new module, AlertChannel enum (Discord/Slack/Webhook/Email) ✅
- Payload builders for each channel type ✅
- DeliveryOutcome enum (Success/HttpError/TransportError/InsufficientCycles/InvalidConfig) ✅
- Cycle cost estimation (`estimate_outcall_cycles`) ✅
- `deliver_to_channel()` async HTTPS outcall with IC management canister ✅
- `run_delivery_drain()` queue drainer with retry logic ✅
- `transform_response()` deterministic transform callback ✅
- AlertQueueItem enriched with severity, rules, events_summary, recommended_action ✅
- 60s `delivery_tick()` timer added to lib.rs ✅
- `get_alert_queue_len()` query endpoint ✅
- 48 new delivery tests, 0 clippy warnings ✅

## Test Summary

| Crate | Tests | Status |
|-------|-------|--------|
| guardian_config | 14 | ✅ All passing |
| guardian_engine (unit) | 130 | ✅ All passing |
| guardian_engine (delivery) | 48 | ✅ All passing (Phase 2b NEW) |
| guardian_engine (integration) | 55 | ✅ All passing |
| **TOTAL** | **237** | **✅ 0 failures, 0 clippy warnings** |

### Phase 2b Test Coverage (delivery.rs)

- Channel parsing: 9 tests (discord, slack, webhook w/secret, webhook no secret, email, unknown, empty, missing url)
- Channel kind labels: 4 tests
- DeliveryOutcome: 6 tests (success, 4xx permanent, 5xx transient, transport, invalid config, insufficient cycles)
- Discord payload: 5 tests (content, colors for each severity)
- Slack payload: 3 tests (content, emergency emoji, critical emoji)
- Webhook payload: 4 tests (JSON structure, severity score, rules array, content)
- Email payload: 2 tests (to field, URL encoding)
- Cycle estimation: 5 tests (base, 1KB request, with response, typical, budget coverage)
- JSON escape helper: 5 tests
- URL encode helper: 4 tests
- Retry constants: 3 tests
