# Guardian Phase 1 — Quick Start Guide

**Status:** Ready to begin Phase 1a (Config Canister hardening)  
**Timeline:** 4 weeks (Mar 2 – Apr 1, 2026)  
**Codebase:** Existing skeleton + improvements  

---

## Set Up Your Environment (Today)

### 1. Add dfx to PATH
```bash
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"
dfx --version  # Should show 0.30.2+
```

### 2. Verify Rust
```bash
rustc --version  # 1.70+
cargo --version
```

### 3. Start Local Replica
```bash
cd /home/ranch/.openclaw/workspace/guardian-icp
dfx start --clean  # First time: --clean wipes old state
```
(Keep this running in a separate terminal)

### 4. Build the Project
```bash
cargo build --target wasm32-unknown-unknown --release
```
Should complete without errors.

### 5. Deploy Locally
```bash
dfx deploy guardian_config --network local
```
Should output something like:
```
Deployed canisters.
Canister ID: rwq3b-zqaaa-aaaar-qaaba-cai
```

### 6. Test the Canister
```bash
dfx canister call guardian_config health --network local
# Should return: (true)
```

---

## What You're Working On (Phase 1a)

**Goal:** Harden the Config Canister for production use

**Checklist (in order):**

1. **Rate Limiting** (2 days)
   - Add `last_update_times: BTreeMap<Principal, Vec<u64>>` to track updates per hour
   - Reject if principal has >10 updates in the last 3600 seconds
   - Return descriptive error: `Err("Rate limit exceeded. Max 10 updates per hour.")`

2. **Input Validation** (2 days)
   - In `set_config()`, validate before storing:
     - `alert_channels.len() <= 5`
     - `allowlisted_addresses.len() <= 500`
     - `alert_threshold` and `emergency_threshold` <= 255
     - `large_transfer_pct` and `daily_outflow_pct` between 0.0 and 1.0
   - Return descriptive errors for each failure

3. **canister_inspect_message Enhancement** (1 day)
   - Reject payloads >1MB
   - Reject non-update calls that aren't read-only
   - Log rejections for audit

4. **Cycle Cost Monitoring** (1 day)
   - Query `ic0.canister_cycle_balance()` in `get_health()`
   - Return warning if <30 days of runway at current rate
   - Set `freezing_threshold` in `dfx.json` to 90 days of cycles

5. **Unit Tests** (2 days)
   - Write 20+ test cases covering all validation paths
   - Test rate limiting, bounds, authorization
   - Use `#[test]` or `#[ctor::test]` macro
   - Run: `cargo test`

---

## Key Files to Review

| File | Purpose |
|------|---------|
| `src/lib.rs` | Guardian Config Canister (your main focus) |
| `src/guardian.did` | Candid interface definition |
| `Cargo.toml` | Dependencies + build config |
| `dfx.json` | ICP build + network config |
| `/guardian-dev/DEV_PLAN.md` | Full Phase 1 breakdown |
| `/guardian-dev/OISY_GUARDIAN_SPEC.md` | Full spec (read Sections 7–9) |

---

## Daily Workflow

**Morning:**
```bash
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"
cd /home/ranch/.openclaw/workspace/guardian-icp
dfx start --clean  # in Terminal 1

# In Terminal 2:
cargo build --target wasm32-unknown-unknown --release
dfx deploy guardian_config --network local
```

**Work:**
- Edit `src/lib.rs`
- Run tests: `cargo test`
- Re-deploy: `dfx deploy guardian_config --network local`
- Manually test: `dfx canister call guardian_config <method> ...`

**Evening:**
- Commit progress: `git add -A && git commit -m "feat: Phase 1a - add rate limiting"`
- Update `/guardian-dev/DEV_LOG.md` with what you did
- Note blockers or questions

---

## Testing Locally

### Manual Test: Set Config
```bash
dfx canister call guardian_config set_config \
  '(record { 
      owner = principal "5lok2-xvf24-onx6j-zldh6-ss6u5-xinwf-5m7u2-gzaiq-lfdpo-ivagh-aae";
      created_at = 0;
      updated_at = 0;
      monitored_chains = vec { "icp"; "ethereum" };
      large_transfer_pct = 0.5;
      daily_outflow_pct = 0.8;
      rapid_tx_count = 5;
      rapid_tx_window_secs = 600;
      new_address_alert = true;
      alert_threshold = 7;
      emergency_threshold = 15;
   })' \
  --network local
```

### Manual Test: Get Config
```bash
dfx canister call guardian_config get_config --network local
```

### Manual Test: Test Rate Limiting
```bash
# Call set_config 11 times in a loop
# The 11th should fail with "Rate limit exceeded"
```

---

## Common Issues

| Issue | Solution |
|-------|----------|
| `dfx: command not found` | Add to PATH: `export PATH="/home/ranch/.local/share/dfx/bin:$PATH"` |
| `cargo build` fails | Run `cargo clean`, then `cargo build --target wasm32-unknown-unknown --release` |
| `dfx deploy` fails | Stop replica (`dfx stop`), then restart (`dfx start --clean`) |
| Tests fail | Run `cargo test` and check stack trace; add `println!` for debugging |
| Candid type error | Check `src/guardian.did` matches struct in `src/lib.rs` |

---

## Code Standards (From Section 9 of Spec)

**Do:**
- ✅ Use `ic-stable-structures` for all persistent data
- ✅ Validate all inputs before storing
- ✅ Handle inter-canister rejects explicitly
- ✅ Log all security-relevant events
- ✅ Write tests for happy path + error cases

**Don't:**
- ❌ Store secrets or private keys
- ❌ Use `unwrap()` without documenting why it's safe
- ❌ Trust data from canisters without validation
- ❌ Skip error handling in inter-canister calls

---

## Resources

- **OISY Guardian Spec:** `/guardian-dev/OISY_GUARDIAN_SPEC.md` (40KB, read Sections 7–9)
- **ICP Dev Docs:** https://docs.internetcomputer.org/
- **Candid Guide:** https://docs.internetcomputer.org/building-apps/backend/candid/
- **Rust + ICP:** https://docs.rs/ic-cdk/
- **Stable Structures:** https://docs.rs/ic-stable-structures/

---

## Check-In Points

| Checkpoint | Date | Criteria |
|------------|------|----------|
| **1a Complete** | Mar 8 | Config canister builds, passes 20+ tests, rate limiting works |
| **1b Complete** | Mar 15 | Engine canister skeleton compiles, timer loop runs |
| **1c Complete** | Mar 22 | Can query mock ICRC index, watermarks persist |
| **1d Complete** | Mar 29 | Detection rules evaluate correctly, 85%+ coverage |
| **1e Complete** | Apr 1 | Local deployment works, README complete, ready for mainnet |

---

**Status:** Ready to start Phase 1a today (Mar 2)  
**Next:** Review the DEV_PLAN + start on rate limiting
