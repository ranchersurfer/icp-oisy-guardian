# Guardian ICP — Phase 1 MVP

Real-time wallet monitoring for OISY on the Internet Computer. Detects suspicious activity across ICP, ckBTC, and ckETH by analyzing ICRC-1/ICRC-3 transaction history.

## Architecture

```
guardian_config   — User-owned canister for monitoring rules and alert settings
guardian_engine   — On-chain detection engine with 30s polling timer
```

### Monitored Rules
| Rule | Description | Weight | Severity |
|------|-------------|--------|----------|
| A1   | Large outgoing transfer (>50% balance) | 7 | CRITICAL |
| A3   | Rapid transactions (>5 outgoing in 10min) | 3 | WARN |
| A4   | New/unknown destination address | 1 | INFO |

Alert fires when total score ≥ `alert_threshold` (default: 7).

---

## Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [DFX](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) v0.30+

```bash
rustup target add wasm32-unknown-unknown
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

---

## Setup & Local Deployment

### 1. Clone and build

```bash
git clone <repo-url>
cd guardian-icp
export PATH="$HOME/.local/share/dfx/bin:$HOME/.cargo/bin:$PATH"

# Build both canisters
cargo build --target wasm32-unknown-unknown --release
```

### 2. Start local replica

```bash
dfx start --clean --background
```

### 3. Deploy

```bash
dfx canister create --all
dfx build
dfx canister install --all
```

### 4. Verify deployment

```bash
dfx canister call guardian_config health
dfx canister call guardian_engine get_health
```

Expected output:
```
(variant { Ok = record { status = "Guardian OK"; ... } })
(record { is_running = true; watermark_count = 0 : nat64; ... })
```

---

## Usage

### Setting a monitoring configuration

```bash
dfx canister call guardian_config set_config '(
  record {
    owner = principal "YOUR_PRINCIPAL";
    created_at = 0 : nat64;
    updated_at = 0 : nat64;
    monitored_chains = vec { "ICP"; "ckBTC"; "ckETH" };
    large_transfer_pct = 0.5 : float64;
    daily_outflow_pct = 0.8 : float64;
    rapid_tx_count = 5 : nat32;
    rapid_tx_window_secs = 600 : nat64;
    new_address_alert = true;
    alert_threshold = 7 : nat32;
    emergency_threshold = 15 : nat32;
    alert_channels = vec { "telegram" };
    allowlisted_addresses = vec {};
  }
)'
```

### Retrieving your configuration

```bash
dfx canister call guardian_config get_config
```

### Connecting engine to config canister

```bash
dfx canister call guardian_engine set_config_canister_id "(principal \"$(dfx canister id guardian_config)\")"
```

---

## Rule Configuration Guide

### A1 — Large Transfer Alert

Triggers when a single outgoing transaction exceeds `large_transfer_pct` × estimated balance.

- Default threshold: 50% (`large_transfer_pct = 0.5`)
- Weight: 7 (CRITICAL alone)
- To disable: set `alert_threshold > 7` and leave A1 as-is, or increase `large_transfer_pct`

### A3 — Rapid Transaction Alert

Triggers when more than `rapid_tx_count` outgoing transactions occur within `rapid_tx_window_secs`.

- Default: >5 txs in 600 seconds (10 minutes)
- Weight: 3 (WARN alone)
- To tighten: lower `rapid_tx_count` or increase `rapid_tx_window_secs`

### A4 — New Address Alert

Triggers when an outgoing transaction targets an address not in `allowlisted_addresses`.

- Weight: 1 (INFO alone)
- To suppress: add frequently-used addresses to `allowlisted_addresses`
- Max 500 allowlisted addresses

### Severity Thresholds

| Score | Severity  |
|-------|-----------|
| 1–2   | INFO      |
| 3–6   | WARN      |
| 7–14  | CRITICAL  |
| 15+   | EMERGENCY |

Set `alert_threshold = 7` to only alert on CRITICAL/EMERGENCY events.
Set `alert_threshold = 1` to alert on all anomalies.

---

## Candid Interface Reference

### guardian_config

```candid
service guardian_config : {
  // Set or update monitoring configuration (caller must match owner)
  set_config : (GuardianConfig) -> (ApiResultUnit);

  // Retrieve caller's configuration
  get_config : () -> (ApiResult) query;

  // Cycle balance and health status
  health : () -> (ApiResult) query;

  // Admin: total user count
  get_stats : () -> (ApiResult) query;
}
```

### guardian_engine

```candid
service guardian_engine : {
  // Health check with cycle balance
  get_health : () -> (EngineHealthStatus) query;

  // Link to config canister (controller only)
  set_config_canister_id : (principal) -> ();

  // Get alerts for caller
  get_my_alerts : () -> (vec AlertRecord) query;

  // Get pending alerts count
  get_alert_count : () -> (nat64) query;
}
```

---

## Testing

```bash
# Run all 143 unit + integration tests
cargo test

# Run only integration tests
cargo test integration_tests

# Run only guardian_config tests
cargo test -p guardian_config

# Run only guardian_engine tests
cargo test -p guardian_engine
```

**Test coverage (Phase 1 MVP):**
- 14 guardian_config tests (validation, rate limiting, health)
- 129 guardian_engine tests (unit + integration)
- **143 total, 0 failures**

---

## Security

- **No hardcoded secrets** — all sensitive values from canister state
- **Input validation** — all `set_config` fields validated before storage
- **Anonymous rejection** — all update endpoints reject anonymous callers
- **Cycle drain guard** — engine refuses requests when balance < 500B cycles
- **Rate limiting** — max 10 config updates per hour per principal
- **`canister_inspect_message`** — gating enforced at ingress level
- **Error messages** — no internal state leaked in error strings

---

## Project Structure

```
guardian-icp/
├── dfx.json                          # Canister definitions
├── Cargo.toml                        # Workspace manifest
├── src/
│   ├── guardian.did                  # Config canister Candid interface
│   ├── guardian_engine.did           # Engine canister Candid interface
│   ├── guardian_config/
│   │   └── src/lib.rs               # Config canister (rate limiting, validation)
│   └── guardian_engine/
│       └── src/
│           ├── lib.rs               # Engine canister (timer, watermarks, alerts)
│           ├── alerts.rs            # Alert formatting and storage
│           ├── canisters.rs         # ICRC canister IDs and constants
│           ├── detector.rs          # Rule evaluation engine
│           ├── fetcher.rs           # ICRC transaction fetching
│           ├── icrc.rs              # ICRC type definitions
│           └── integration_tests.rs # Phase 1e integration tests
```

---

## Deployment to IC Mainnet

> ⚠️ Phase 2 — not yet configured for mainnet. Local only.

```bash
# When ready:
dfx deploy --network ic
```

Ensure `dfx identity` has sufficient cycles and the identity has controller rights.

---

## Version History

| Version | Tag | Description |
|---------|-----|-------------|
| 0.1.0   | v0.1-mvp | Phase 1 MVP — local deployment, 143 tests |

---

## License

MIT
