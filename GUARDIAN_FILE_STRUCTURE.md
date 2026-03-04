# Guardian ICP — File Structure

**Last Updated:** 2026-03-04 (Phase 2a complete)

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
    │       ├── lib.rs                    # Engine canister (timer, watermarks, alerts, stable storage)
    │       ├── alerts.rs                 # Alert payload formatting and storage
    │       ├── canisters.rs              # ICRC canister IDs and fetch constants
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

**Phase 2a Deliverables**:
- TASK 1: ICRC Type Verification ✅ (wire types, ICP/ckBTC/ckETH variants fixed)
- TASK 2: Balance u128 Migration ✅ (supports ckETH 18-decimal, 2 new tests)
- TASK 3: Testnet Deployment ✅ (local success, testnet documented)
- TASK 4: Admin Viewer Script ✅ (`scripts/admin-view.sh`)
- TASK 5: README Documentation ✅ (Phase 2a section + walkthrough)

## Test Summary

| Crate | Tests | Status |
|-------|-------|--------|
| guardian_config | 14 | ✅ All passing |
| guardian_engine (unit) | 81 | ✅ All passing |
| guardian_engine (integration) | 62 | ✅ All passing |
| **ICRC wire types** (Phase 2a) | **2 new** | ✅ Added |
| **TOTAL** | **189** | **✅ 0 failures, 0 clippy warnings** |

### Phase 2a Test Additions

- `test_cketh_balance_overflow_u64`: Confirms u64 overflow for 1000 ETH
- `test_cketh_balance_u128_handles_1000_eth`: Verifies u128 correctly handles large values
