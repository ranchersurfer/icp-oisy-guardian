# Guardian ICP — File Structure

**Last Updated:** 2026-03-03 (Phase 1e complete)

## Source Tree

```
guardian-icp/src/
├── guardian.did                          # Config canister Candid interface
├── guardian_config/
│   ├── Cargo.toml                        # guardian_config crate manifest
│   └── src/
│       └── lib.rs                        # Config canister (rate limiting, validation, cycle monitoring)
├── guardian_engine/
│   ├── Cargo.toml                        # guardian_engine crate manifest
│   └── src/
│       ├── lib.rs                        # Engine canister (timer, watermarks, alerts, stable storage)
│       ├── alerts.rs                     # Alert payload formatting and storage
│       ├── canisters.rs                  # ICRC canister IDs and fetch constants
│       ├── detector.rs                   # Rule evaluation engine (A1/A3/A4)
│       ├── fetcher.rs                    # ICRC transaction fetching + ring buffer
│       ├── icrc.rs                       # ICRC type definitions
│       └── integration_tests.rs         # Phase 1e integration tests (62 tests)
├── guardian_engine.did                   # Engine canister Candid interface
└── lib.rs                               # Workspace stub
```

## Phase Status

| Phase | Status | Tests | Commit |
|-------|--------|-------|--------|
| 1a: Config hardening | ✅ Complete | 14 tests | Mar 2 |
| 1b: Engine skeleton | ✅ Complete | 17 tests | Mar 2, `6f714bc` |
| 1c: ICRC integration | ✅ Complete | 48 tests | Mar 3, `bf7511f` |
| 1d: Detection engine | ✅ Complete | 81 tests | Mar 3 |
| 1e: Testing + local deploy | ✅ Complete | 157 tests | Mar 3, `8b45fdf` (v0.1-mvp) |

## Test Summary

| Crate | Tests | Status |
|-------|-------|--------|
| guardian_config | 14 | ✅ All passing |
| guardian_engine (unit) | 81 | ✅ All passing |
| guardian_engine (integration) | 62 | ✅ All passing |
| **TOTAL** | **157** | **✅ 0 failures** |
