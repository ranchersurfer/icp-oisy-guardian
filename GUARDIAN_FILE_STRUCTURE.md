# Guardian File Structure

**Last Updated:** 2026-03-03 (auto-updated by Guardian-Dev after each phase)

---

## Current Codebase Structure

```
guardian-icp/                        # Cargo workspace root
├── Cargo.toml                       # Workspace manifest
├── dfx.json                         # ICP canister config (guardian_config + guardian_engine)
├── .gitignore                       # Blocks memory/, .dfx/, target/, SSH keys
├── QUICKSTART.md
├── README.md
│
├── src/
│   ├── lib.rs                       # (legacy stub)
│   ├── guardian.did                 # Config canister Candid interface
│   ├── guardian_engine.did          # Engine canister Candid interface
│   │
│   ├── guardian_config/             # ✅ Phase 1a — Config Canister
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs               # GuardianConfig, rate limiting, validation, 14 tests
│   │
│   └── guardian_engine/             # ✅ Phase 1b/1c/1d — Engine Canister
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs               # Timer, stable storage, health endpoint, detection wiring, 81 tests
│           ├── icrc.rs              # ✅ Phase 1c — ICRC types
│           ├── fetcher.rs           # ✅ Phase 1c — Transaction fetchers (ICP/ckBTC/ckETH)
│           ├── canisters.rs         # Canister ID constants (ICP/ckBTC/ckETH)
│           ├── detector.rs          # ✅ Phase 1d — Rule engine (A1/A3/A4), severity scoring
│           └── alerts.rs            # ✅ Phase 1d — Alert payload formatting, stable AlertRecord storage
│
└── target/                          # Build artifacts (gitignored)

guardian-dev/                        # Planning & docs (not in guardian-icp repo)
├── DEV_PLAN.md                      # Full Phase 1a–1e breakdown
├── DEV_LOG.md                       # Running changelog
└── OISY_GUARDIAN_SPEC.md            # Full spec (40KB)
```

---

## Phase Status

| Phase | Scope | Status | Tests |
|-------|-------|--------|-------|
| 1a | Config hardening (rate limit, validation) | ✅ Done | 14/14 |
| 1b | Engine skeleton (timer, stable storage) | ✅ Done | 17/17 |
| 1c | ICRC integration (ICP/ckBTC/ckETH fetch) | ✅ Done | — |
| 1d | Detection engine (rules A1/A3/A4) | ✅ Done | 81/81 |
| 1e | Integration tests + local deploy | ⏳ Pending | — |

---

## GitHub

- **Repo:** https://github.com/ranchersurfer/icp-oisy-guardian
- **Branch:** `main`
- **Latest commit:** Phase 1d — Detection engine with rules A1/A3/A4 and severity scoring
