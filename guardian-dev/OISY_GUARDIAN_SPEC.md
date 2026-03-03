# OISY Wallet Guardian Agent — Full Specification v1.0

> **Purpose:** A comprehensive spec for an OpenClaw-orchestrated security agent system that monitors OISY wallets for suspicious activity, alerts owners, and enforces configurable protection rules.
>
> **Audience:** OpenClaw agents (Crypto-Orchestrator, Crypto-Dev, Crypto-Risk, Compliance-Guardrail) and you (the human operator).
>
> **Status:** SPEC — requires Strategy + Compliance + Risk sign-off before moving to TEST.

---

## Table of Contents

1. [Product Overview](#1-product-overview)
2. [OISY Wallet Architecture (Reference)](#2-oisy-wallet-architecture-reference)
3. [Guardian Architecture](#3-guardian-architecture)
4. [Monitoring Targets & Signals](#4-monitoring-targets--signals)
5. [Detection Engine — Rules & Heuristics](#5-detection-engine--rules--heuristics)
6. [Alert & Response System](#6-alert--response-system)
7. [User Configuration Schema](#7-user-configuration-schema)
8. [ICP Canister Design](#8-icp-canister-design)
9. [ICP Security Implementation (from DFINITY Developer Docs)](#9-icp-security-implementation-from-dfinity-developer-docs)
10. [Chain Fusion Integration (Multi-Chain Monitoring)](#10-chain-fusion-integration-multi-chain-monitoring)
11. [OpenClaw Agent Roles & Workspaces](#11-openclaw-agent-roles--workspaces)
12. [Data Flow & Sequence Diagrams](#12-data-flow--sequence-diagrams)
13. [Cycle Budget & Cost Model](#13-cycle-budget--cost-model)
14. [Testing Plan](#14-testing-plan)
15. [Business Model & Pricing](#15-business-model--pricing)
16. [Risk Register](#16-risk-register)
17. [Roadmap & Milestones](#17-roadmap--milestones)
18. [Appendix A: ICRC Standards Referenced](#appendix-a-icrc-standards-referenced)
19. [Appendix B: Known OISY Canister IDs](#appendix-b-known-oisy-canister-ids)
20. [Appendix C: ICP Bitcoin API Pricing](#appendix-c-icp-bitcoin-api-pricing)

---

## 1. Product Overview

### 1.1 What It Is

The **OISY Guardian** is a suite of ICP canisters + an OpenClaw orchestration layer that:

- **Watches** an OISY wallet's transaction history across all supported chains (BTC, ETH, SOL, ICP, ERC-20s, BSC, Base, Polygon, Arbitrum).
- **Detects** suspicious or anomalous activity using configurable rules and behavioral heuristics.
- **Alerts** the wallet owner via Telegram, WhatsApp, Discord, email, or in-app notification.
- **Optionally enforces** protection rules: transaction delay, spend caps, address allow-lists, and a full emergency pause.

### 1.2 What It Is NOT

- **Not a custodian.** Guardian never holds private keys. OISY's private keys are controlled by the NNS via threshold cryptography through the Chain Fusion Signer canister (`grghe-syaaa-aaaar-qabyq-cai`). Guardian is read-only with respect to keys.
- **Not a replacement for OISY's built-in security.** Guardian is an additional monitoring layer on top of OISY's existing network-custody model.
- **Not an investment advisor.** Guardian monitors security signals, not market conditions.

### 1.3 Target Users

| Segment | Why They Need Guardian |
|---|---|
| Power users with >$5K in OISY | Want proactive alerting for large or unusual transactions |
| ICP DeFi participants | Interact with multiple dApps via WalletConnect; want consent-message validation |
| Small DAOs / treasuries using OISY + II | Need multi-layer approval and audit trail |
| Users new to network custody | Want a "safety net" that explains what's happening |

### 1.4 Core Value Proposition

OISY provides excellent network custody. Guardian adds **runtime behavioral security** — the difference between a locked door (OISY) and a security camera + alarm system (Guardian).

---

## 2. OISY Wallet Architecture (Reference)

Understanding OISY's internals is mandatory for Guardian development. This section summarizes what you need to know from the official OISY docs and GitHub.

### 2.1 Key Components

| Component | Canister ID | Controller | Role |
|---|---|---|---|
| OISY Frontend | `cha4i-riaaa-aaaan-qeccq-cai` | OISY dev team | Serves the browser-based wallet UI from chain |
| OISY Backend | `doked-biaaa-aaaar-qag2a-cai` | OISY dev team | Handles wallet logic, account management |
| Chain Fusion Signer | `grghe-syaaa-aaaar-qabyq-cai` | NNS (DAO-governed) | Signs transactions via threshold ECDSA/Schnorr; generates per-user addresses |
| Internet Identity | NNS-managed | NNS | Authentication via WebAuthn/passkeys; the user's master identity |

### 2.2 How Transactions Work

1. **User initiates** a transaction in the OISY browser UI (or via a dApp connected through WalletConnect / ICRC signer standards).
2. **Consent message** is fetched from the target canister per ICRC-21. User reviews and approves.
3. **OISY Backend** constructs the transaction payload.
4. **Chain Fusion Signer** signs the transaction using threshold ECDSA (for BTC/ETH) or threshold Schnorr (for some chains). The private key never exists in complete form — it's secret-shared across ICP subnet nodes.
5. **Signed transaction** is submitted to the target chain (Bitcoin network via ICP Bitcoin adapter, Ethereum/EVM via EVM RPC canister, Solana via SOL RPC canister).

### 2.3 Key Security Properties

- **Network custody:** Private keys are threshold-distributed across ICP nodes. No single entity (including OISY) can access them.
- **User sovereignty:** Only the Internet Identity holder can authorize transactions.
- **Assets live on native chains:** OISY canisters don't hold funds. Assets reside on BTC/ETH/SOL/ICP ledgers. Even if OISY canisters are removed, assets remain accessible.
- **Frozen, not lost:** If an OISY canister runs out of cycles, it freezes (stops executing) but can be refueled by anyone. No data is lost.
- **Open source:** OISY is Apache 2.0 licensed. The full codebase is at `github.com/dfinity/oisy-wallet`.

### 2.4 Supported Chains (as of Feb 2026)

Bitcoin, Ethereum, Solana, ICP, Binance Smart Chain, Base, Polygon, Arbitrum — with more planned.

### 2.5 OISY Signer Standards

OISY implements the following ICRC standards via `@dfinity/oisy-wallet-signer`:

| Standard | Purpose |
|---|---|
| ICRC-25 | Signer Interaction Standard — permission management (grant/deny/expire after 7 days) |
| ICRC-27 | Accounts — dApp requests list of wallet accounts |
| ICRC-21 | Canister Call Consent Messages — human-readable explanation of what a tx does |
| ICRC-49 | Call Canister — actual execution of approved canister calls |

**Guardian implication:** Guardian can observe the ICRC-21 consent messages to validate what dApps are requesting. If a consent message looks suspicious (e.g., unlimited token approval), Guardian can flag it.

---

## 3. Guardian Architecture

### 3.1 High-Level Components

```
┌─────────────────────────────────────────────────────────┐
│                    OpenClaw Layer                         │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────┐ │
│  │Orchestrat│ │ Strategy │ │   Risk   │ │ Compliance │ │
│  │   -or    │ │  Agent   │ │  Agent   │ │  Agent     │ │
│  └────┬─────┘ └──────────┘ └──────────┘ └────────────┘ │
│       │ Manages lifecycle, deploys, updates configs      │
└───────┼─────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────┐
│                 ICP Canister Layer                        │
│                                                          │
│  ┌──────────────────┐    ┌──────────────────┐           │
│  │  Guardian Config  │    │  Guardian Engine  │           │
│  │    Canister       │◄──►│    Canister       │           │
│  │ (user rules,      │    │ (polling, detect, │           │
│  │  preferences)     │    │  alert dispatch)  │           │
│  └──────────────────┘    └───────┬──────────┘           │
│                                  │                       │
│              ┌───────────────────┼───────────────────┐   │
│              ▼                   ▼                   ▼   │
│  ┌──────────────┐   ┌──────────────┐   ┌───────────┐   │
│  │ ICP Ledger + │   │ EVM RPC      │   │ Bitcoin   │   │
│  │ Index Canist.│   │ Canister     │   │ Canister  │   │
│  └──────────────┘   └──────────────┘   └───────────┘   │
│              │                   │                   │   │
│  ┌──────────────┐   ┌──────────────┐   ┌───────────┐   │
│  │ SOL RPC      │   │ ckBTC/ckETH  │   │ ICRC      │   │
│  │ Canister     │   │ Ledgers      │   │ Ledgers   │   │
│  └──────────────┘   └──────────────┘   └───────────┘   │
└─────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────┐
│               Alert Delivery Layer                       │
│  Telegram │ WhatsApp │ Discord │ Email │ HTTPS webhook   │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Separation of Concerns

| Component | Has Keys? | Can Sign Txs? | Can Read Tx History? | Can Send Alerts? |
|---|---|---|---|---|
| OISY Wallet | Via Chain Fusion Signer | Yes (user-authorized) | Yes | No |
| Guardian Config Canister | **No** | **No** | No (stores rules only) | No |
| Guardian Engine Canister | **No** | **No** | **Yes** (read-only via index canisters + RPC) | **Yes** (via HTTPS outcalls) |
| OpenClaw Agents | **No** | **No** | Yes (via browser) | Yes (via messaging integrations) |

**Critical invariant:** Guardian canisters NEVER hold private keys, seed phrases, or signing authority. They are strictly observational and advisory.

---

## 4. Monitoring Targets & Signals

### 4.1 ICP-Native Tokens (ICP, ICRC-1/ICRC-2 tokens, ckBTC, ckETH, ckUSDT)

**Data source:** ICRC Index Canisters

Each ICRC-compliant ledger has a companion Index Canister that enables `get_account_transactions` queries for a specific account. Guardian Engine calls this periodically.

```
// Pseudocode for querying ICRC Index Canister
let txs = await ICRC_INDEX.get_account_transactions({
    account: { owner: user_principal, subaccount: null },
    start: last_checked_tx_id,
    max_results: 100,
});
```

**Signals extracted per transaction:**
- `amount`: absolute value transferred
- `to` / `from`: destination/source account (principal + subaccount)
- `timestamp`: when the transaction was recorded
- `memo`: optional metadata (useful for pattern matching)
- `fee`: transaction fee paid
- Transaction type: `Transfer`, `Mint`, `Burn`, `Approve` (ICRC-2)

### 4.2 Bitcoin (via ckBTC or native BTC)

**Data source:** ICP Bitcoin API (for native BTC held by OISY via Chain Fusion) and ckBTC ICRC Index Canister (for ckBTC)

For native BTC monitoring:
```
// Get UTXOs for the user's OISY-generated BTC address
let utxos = await ic.bitcoin_get_utxos({
    address: user_btc_address,
    network: #mainnet,
    filter: ?#min_confirmations(1),
});
```

**Signals:**
- UTXO changes (new incoming, spent outgoing)
- Transaction amounts
- Destination addresses (can check against known exchange addresses, flagged addresses)
- Confirmation count

**Cost note:** Bitcoin mainnet API calls cost 50M+ cycles per UTXO query. Budget accordingly.

### 4.3 Ethereum & EVM Chains (ETH, ERC-20s, Base, Polygon, Arbitrum, BSC)

**Data source:** EVM RPC Canister (`7hfb6-caaaa-aaaar-qadga-cai`)

```
// Query ETH logs for transfers involving user's address
let logs = await evm.eth_getLogs(
    #EthMainnet(null),
    null,
    {
        addresses: [user_eth_address],
        fromBlock: ?#Number(last_checked_block),
        toBlock: ?#Latest,
        topics: ?[[TRANSFER_EVENT_TOPIC]],
    },
);
```

**Signals:**
- ERC-20 Transfer events (amount, token, to/from)
- ERC-20 Approval events (spender, allowance amount — **critical for detecting unlimited approvals**)
- Native ETH transfers
- Contract interactions (which contract, what function)

### 4.4 Solana

**Data source:** SOL RPC Canister (NNS-managed, uses Helius/Alchemy/Ankr/dRPC via HTTPS outcalls)

**Signals:**
- SOL transfers
- SPL token transfers
- Program interactions

### 4.5 Cross-Chain Behavioral Signals

Beyond individual chain monitoring, Guardian tracks **cross-chain patterns**:
- Rapid fund movement across chains (BTC → ckBTC → swap → ETH → withdraw)
- Simultaneous draining across multiple chains
- New dApp connections via WalletConnect appearing on multiple chains

---

## 5. Detection Engine — Rules & Heuristics

### 5.1 Rule Categories

#### Category A: Threshold Rules (User-Configurable)

| Rule ID | Name | Default | User Can Adjust |
|---|---|---|---|
| `A1` | Large outgoing transfer | >50% of chain balance | Yes (absolute or %) |
| `A2` | Cumulative daily outflow | >80% of chain balance in 24h | Yes |
| `A3` | Rapid successive transactions | >5 outgoing txs in 10 min | Yes (count + window) |
| `A4` | New destination address | First-ever send to this address | Yes (on/off) |
| `A5` | Late-night transaction | Transfer outside user's configured active hours | Yes (timezone + hours) |
| `A6` | Maximum single transaction | Absolute cap (e.g., $10,000) | Yes |

#### Category B: Behavioral Heuristics (System-Managed)

| Rule ID | Name | Description |
|---|---|---|
| `B1` | Velocity anomaly | Outgoing tx frequency > 3σ above user's 30-day rolling average |
| `B2` | Address cluster risk | Destination address has on-chain links to known exploit contracts or flagged addresses |
| `B3` | Token approval abuse | ERC-20 `approve()` with unlimited allowance (`uint256.max`) to an unrecognized spender |
| `B4` | Drainer pattern | Multiple token approvals to the same address within a short window |
| `B5` | Bridge sweep | Rapid conversion from native tokens to ckTokens followed by transfers |
| `B6` | Consent message anomaly | ICRC-21 consent message contains unusual or obfuscated transfer requests |

#### Category C: System Health Rules

| Rule ID | Name | Description |
|---|---|---|
| `C1` | Guardian cycle balance low | Guardian Engine canister below 30-day cycle runway |
| `C2` | Polling failure | Unable to reach an index canister or RPC for >3 consecutive attempts |
| `C3` | Config canister desync | Config hash mismatch between what OpenClaw set and what's on-chain |

### 5.2 Severity Levels

| Level | Name | Response |
|---|---|---|
| `INFO` | Informational | Log only; include in daily digest |
| `WARN` | Warning | Real-time alert via user's preferred channel |
| `CRITICAL` | Critical | Real-time alert + trigger configured protection action (if enabled) |
| `EMERGENCY` | Emergency | Real-time alert on ALL channels + auto-pause if enabled + escalate to human |

### 5.3 Scoring Logic

Each triggered rule contributes a weighted score:

```
alert_score = Σ (rule_weight × rule_severity_multiplier)

Severity multipliers:
  INFO      = 1
  WARN      = 3
  CRITICAL  = 7
  EMERGENCY = 15

If alert_score >= user_threshold (default: 7), send real-time alert.
If alert_score >= emergency_threshold (default: 15), trigger protection actions.
```

Users can adjust `user_threshold` and `emergency_threshold` in their config.

---

## 6. Alert & Response System

### 6.1 Alert Channels

Guardian Engine sends alerts via **HTTPS outcalls** from ICP canisters:

| Channel | Mechanism | Latency |
|---|---|---|
| Telegram | Bot API via HTTPS outcall | ~2-5 seconds |
| Discord | Webhook via HTTPS outcall | ~2-5 seconds |
| Email | SendGrid/Mailgun API via HTTPS outcall | ~5-30 seconds |
| WhatsApp | Via OpenClaw integration (OpenClaw sends, not canister) | ~5-10 seconds |
| Webhook | User-provided URL, POST with JSON payload | ~2-5 seconds |

**Note on HTTPS outcalls:** As of mid-2025, ICP supports IPv4 via automatic SOCKS proxy fallback, so virtually all external APIs are reachable. TLS is end-to-end between the ICP node and the destination server — the proxy cannot read or alter traffic. Non-replicated outcalls (`is_replicated = Some(false)`) can be used for alerts where you only need one node to send the message, reducing cycle cost.

### 6.2 Alert Payload Schema

```json
{
  "guardian_version": "1.0.0",
  "alert_id": "uuid-v4",
  "timestamp_utc": "2026-02-26T20:15:30Z",
  "wallet_principal": "xxxx-xxx-...",
  "chain": "ethereum",
  "severity": "CRITICAL",
  "alert_score": 10,
  "rules_triggered": [
    {
      "rule_id": "A1",
      "rule_name": "Large outgoing transfer",
      "details": "Transfer of 12.5 ETH ($42,000) to 0xABC...DEF. This is 78% of your ETH balance.",
      "severity": "CRITICAL"
    },
    {
      "rule_id": "A4",
      "rule_name": "New destination address",
      "details": "You have never sent funds to 0xABC...DEF before.",
      "severity": "WARN"
    }
  ],
  "recommended_action": "Review this transaction. If you did not initiate it, consider revoking dApp permissions.",
  "tx_hash": "0x123...789",
  "dashboard_url": "https://guardian.oisy.app/alert/uuid-v4"
}
```

### 6.3 Protection Actions (Optional, User-Enabled)

These are **not** automatic by default. Users must explicitly opt-in to each protection action in their config.

| Action | How It Works | Limitation |
|---|---|---|
| **Alert-only** (default) | Send notification, no further action | None |
| **Transaction delay** | Guardian does NOT delay transactions (it has no signing authority). Instead, it alerts the user BEFORE consent is given if monitoring ICRC-21 consent messages in real time. | Requires OISY signer integration or dApp-side plugin |
| **Address allowlist** | Guardian alerts on any transfer to an address NOT on the user's approved list | Monitoring only; cannot block |
| **Daily spend cap** | Guardian alerts when cumulative daily outflows exceed a configured USD amount | Monitoring only; cannot block |
| **Emergency pause advisory** | Guardian sends urgent alerts on all channels recommending the user revoke all dApp permissions via OISY | Advisory only |

**Important design constraint:** Because Guardian has NO signing authority and NO access to private keys, it cannot _block_ transactions. It can only _detect and alert_. True transaction blocking would require integration at the OISY signer level (future roadmap item requiring DFINITY collaboration or an NNS proposal).

---

## 7. User Configuration Schema

Stored in the Guardian Config Canister, keyed by user principal.

```candid
type GuardianConfig = record {
  // Identity
  owner : principal;
  created_at : nat64;
  updated_at : nat64;

  // Chains to monitor
  monitored_chains : vec Chain;

  // Addresses (derived from OISY — user provides or Guardian queries)
  addresses : vec ChainAddress;

  // Alert preferences
  alert_channels : vec AlertChannel;
  alert_timezone : text;         // e.g., "America/Los_Angeles"
  active_hours : opt record { start_hour : nat8; end_hour : nat8 };
  daily_digest : bool;           // send daily summary at 9 AM user-local
  digest_hour : nat8;

  // Threshold rules (Category A overrides)
  large_transfer_pct : float64;  // default 0.50
  large_transfer_abs : opt nat;  // absolute amount in smallest unit
  daily_outflow_pct : float64;   // default 0.80
  rapid_tx_count : nat;          // default 5
  rapid_tx_window_secs : nat;    // default 600
  new_address_alert : bool;      // default true
  max_single_tx_usd : opt float64;

  // Scoring
  alert_threshold : nat;         // default 7
  emergency_threshold : nat;     // default 15

  // Address allowlist
  allowlisted_addresses : vec text;

  // Protection actions opt-in
  protections_enabled : vec ProtectionAction;

  // Subscription
  tier : SubscriptionTier;
  subscription_expires : nat64;
};

type Chain = variant { ICP; Bitcoin; Ethereum; Solana; Base; Polygon; Arbitrum; BSC };
type ChainAddress = record { chain : Chain; address : text };
type AlertChannel = variant {
  Telegram : record { chat_id : text; bot_token : text };
  Discord : record { webhook_url : text };
  Email : record { address : text };
  Webhook : record { url : text; secret : text };
};
type ProtectionAction = variant { AlertOnly; AddressAllowlist; DailySpendCap; EmergencyAdvisory };
type SubscriptionTier = variant { Free; Basic; Pro; Enterprise };
```

---

## 8. ICP Canister Design

### 8.1 Guardian Config Canister

**Language:** Rust (recommended for performance and security)
**Purpose:** Stores user configurations. CRUD operations gated by caller principal.

**Public API:**

```candid
service : {
  // Config management (caller = owner only)
  set_config : (GuardianConfig) -> (Result);
  get_config : () -> (opt GuardianConfig) query;
  delete_config : () -> (Result);

  // Admin (controller only)
  get_all_users : () -> (vec principal) query;
  get_stats : () -> (GuardianStats) query;
};
```

**Security:**
- `canister_inspect_message` rejects unauthenticated or anonymous callers.
- Every mutation verifies `ic_cdk::caller() == config.owner`.
- Rate limiting: max 10 config updates per hour per principal.

### 8.2 Guardian Engine Canister

**Language:** Rust
**Purpose:** The core monitoring loop. Runs on a timer, polls index canisters and RPC endpoints, evaluates rules, dispatches alerts.

**Public API:**

```candid
service : {
  // Manual trigger (owner or admin)
  check_now : (principal) -> (CheckResult);

  // Status
  get_health : () -> (HealthStatus) query;
  get_alert_history : (principal, nat) -> (vec AlertRecord) query;

  // Admin
  set_polling_interval : (nat64) -> ();  // seconds
  pause_monitoring : (principal) -> ();
  resume_monitoring : (principal) -> ();
  global_kill_switch : () -> ();        // controller only
};
```

**Internal Flow (Timer-driven):**

```
Every {polling_interval} seconds:
  1. Load batch of user configs from Config Canister
  2. For each user:
     a. For each monitored chain:
        - Query the appropriate index canister / RPC for new transactions since last check
        - Store the latest tx ID / block number in stable memory as a watermark
     b. Feed transactions into the Detection Engine
     c. If any rules trigger:
        - Compute alert_score
        - If score >= user's alert_threshold:
            - Format alert payload
            - Dispatch via HTTPS outcalls to user's configured channels
            - Log the alert in alert_history (stable storage)
     d. Update telemetry counters
  3. Schedule next timer
```

**Cycle Management:**
- Use `canister_inspect_message` to reject junk ingress.
- Set `freezing_threshold` to ≥90 days of projected cycle burn.
- Monitor cycle balance via `ic0.canister_cycle_balance()` and auto-alert the operator (you) if below threshold.

### 8.3 Stable Memory & Upgrades

- User watermarks (last checked tx ID / block number) stored in **stable memory** using `ic-stable-structures`.
- Alert history stored in stable memory (bounded ring buffer, e.g., last 1,000 alerts per user).
- Timer reinstated in `canister_post_upgrade` hook.
- Config canister uses stable `BTreeMap<Principal, GuardianConfig>`.

---

## 9. ICP Security Implementation (from DFINITY Developer Docs)

This section translates every relevant security best practice from the official DFINITY docs into concrete Guardian implementation requirements.

### 9.1 Inter-Canister Call Safety

Guardian Engine calls Config Canister and various Index/RPC canisters.

**Requirements:**
1. **Journaling for state changes:** Before any state change that spans an `await` (e.g., "about to update watermark after querying index canister"), journal the intent. If the callback traps, the journal enables recovery.
2. **CallerGuard pattern:** Use per-principal locking (Rust `CallerGuard` with `Drop` implementation) to prevent reentrancy when processing a user's monitoring cycle.
3. **TOCTOU prevention:** Never check a condition before an inter-canister call and assume it holds after. Re-validate post-await.
4. **Handle rejects explicitly:** Every inter-canister call must handle `SYS_UNKNOWN`, `CANISTER_ERROR`, `CANISTER_REJECT` reject codes. For `SYS_UNKNOWN`, implement safe retry with idempotency.

### 9.2 Cycle Drain Protection

**Requirements:**
1. Implement `canister_inspect_message` on BOTH canisters. Reject:
   - Anonymous callers
   - Callers not in the registered user list (for Engine canister)
   - Payloads exceeding a size limit (e.g., 1 MB)
2. Rate limit all update methods:
   - `check_now`: max 1 per minute per principal
   - `set_config`: max 10 per hour per principal
3. Set minimum cycle balance alerts at 30-day runway.
4. Use `freezing_threshold` conservatively (90-180 days).

### 9.3 Upgrade Safety

**Requirements:**
1. Store ALL persistent data in stable memory (not heap variables).
2. Keep `canister_pre_upgrade` logic minimal — do NOT serialize large data structures there. Use `ic-stable-structures` which persist across upgrades without pre/post hooks.
3. Reinstate all timers in `canister_post_upgrade`.
4. Load-test upgrades with synthetic datasets (1,000+ users, 100K+ alert records) to ensure upgrade doesn't trap.
5. Stop canister before upgrading to drain pending inter-canister calls.

### 9.4 Time Handling

**Requirements:**
1. Always capture `ic_cdk::api::time()` AFTER `await`, not before.
2. Do not rely on timestamp ordering within the same block. Use a monotonic logical counter for sequencing alerts.
3. Use timers (not heartbeats) for periodic execution. Heartbeats are legacy and cost more.

### 9.5 Untrustworthy Canister Calls

Guardian calls Index canisters and RPC canisters that are NNS-managed and trustworthy. However:
1. If Guardian ever integrates with third-party canisters (e.g., a price oracle), use a **state-free proxy canister** to isolate the call.
2. Always sanitize data returned from any canister call.
3. Avoid call graph loops. Guardian → Config is one-way. Guardian → Index/RPC is one-way. No loops.

### 9.6 Data Size Limits

**Requirements:**
1. Bound all user-provided data: `allowlisted_addresses` max 500 entries, `alert_channels` max 5, etc.
2. Validate Candid payloads in `canister_inspect_message` — reject `[Null]` bombs.
3. Use bounded data structures for alert history (ring buffer).

### 9.7 Decentralization Path

**Phase 1 (MVP):** Guardian canisters controlled by your developer principal.
**Phase 2:** Transfer control to a multi-sig (Orbit station canister) requiring 2-of-3 approval for upgrades.
**Phase 3:** Consider SNS (Service Nervous System) if Guardian becomes a community product. This puts all upgrades under DAO vote.

---

## 10. Chain Fusion Integration (Multi-Chain Monitoring)

### 10.1 ICP Native Tokens

- **Mechanism:** Call ICRC Index Canister `get_account_transactions`.
- **Polling interval:** Every 30 seconds (configurable).
- **Cost:** Low. Index canister queries are cheap (~4M cycles per query call).

### 10.2 Bitcoin

Two paths:
1. **ckBTC (preferred):** Monitor the ckBTC ICRC Index Canister, same as any ICRC-1 token. Cheap and fast.
2. **Native BTC:** Use `bitcoin_get_utxos` and `bitcoin_get_balance` APIs. Costs 50M+ cycles per call on mainnet. Use sparingly (poll every 5 minutes).

### 10.3 Ethereum & EVM Chains

- **Mechanism:** EVM RPC Canister (`7hfb6-caaaa-aaaar-qadga-cai`) — NNS-managed, contacts 3+ independent RPC providers (Ankr, BlockPI, Alchemy, etc.) and aggregates responses.
- **Key calls:** `eth_getLogs` (for Transfer/Approval events), `eth_getBalance`.
- **Cost:** 10B+ cycles per `eth_getLogs` call. Batch events and poll every 60 seconds.
- **Multi-chain:** Same canister supports EthMainnet, Base, Polygon, Arbitrum, BSC by specifying `RpcServices` variant.

### 10.4 Solana

- **Mechanism:** SOL RPC Canister (NNS-managed, uses Helius/Alchemy/Ankr/dRPC).
- **Key calls:** `getSignaturesForAddress`, `getTransaction`.
- **Cost:** Similar to EVM RPC. Poll every 60 seconds.

### 10.5 Cross-Chain Correlation

The Guardian Engine maintains a unified event stream across all chains:

```
struct UnifiedEvent {
    chain: Chain,
    timestamp: u64,
    direction: Direction,  // Incoming or Outgoing
    amount_usd: f64,       // Normalized to USD via CoinGecko HTTPS outcall
    counterparty: String,  // Address on the respective chain
    tx_id: String,
    raw: RawChainEvent,
}
```

The detection engine runs rules against this unified stream, enabling cross-chain rules like "total outflow across all chains exceeds $X in 24 hours."

---

## 11. OpenClaw Agent Roles & Workspaces

### 11.1 Agent: `Guardian-Orchestrator`

**Workspace:** `~/openclaw_crypto/guardian/orchestrator`
**Tools:** Filesystem, agentToAgent messaging, Browser
**NO:** Shell/exec, secrets

**Files:**
- `GUARDIAN_STATUS.md` — current stage (SPEC / DEV / TEST / LIVE), blockers
- `GUARDIAN_TASKS.md` — task board with owners and status
- `GUARDIAN_DECISIONS.md` — key decisions and their rationale

**Responsibilities:**
- Coordinate all Guardian development across agents
- Enforce the stage gate: SPEC → Strategy+Risk+Compliance sign-off → DEV → TEST → LIVE
- Escalate to human when: >$0 in real funds at risk, regulatory uncertainty, cycle budget exceeded

### 11.2 Agent: `Guardian-Dev`

**Workspace:** `~/openclaw_crypto/guardian/dev`
**Tools:** Shell/exec (in dev workspace only), Filesystem, Browser (docs, GitHub)
**NO:** Mainnet keys

**Files:**
- `DEV_PLAN.md` — implementation plan for each canister
- `DEV_LOG.md` — what changed
- `CANISTER_MANIFEST.md` — canister IDs, environments (local/testnet/mainnet)

**Responsibilities:**
- Implement Guardian Config Canister and Guardian Engine Canister in Rust
- Write comprehensive tests (unit, integration, property-based)
- Deploy to local dfx environment first, then ICP mainnet with Bitcoin testnet
- Follow ALL security practices from Section 9

### 11.3 Agent: `Guardian-Risk`

**Workspace:** `~/openclaw_crypto/guardian/risk`
**Tools:** Filesystem

**Files:**
- `RISK_CONFIG.md` — default rule weights, thresholds, severity multipliers
- `RISK_SCENARIOS.md` — attack scenarios and how Guardian responds
- `RISK_REVIEW_LOG.md` — verdicts on each development milestone

**Responsibilities:**
- Define and tune default detection rules
- Simulate attack scenarios (wallet drain, phishing approval, etc.)
- Review cycle budget to ensure Guardian doesn't drain itself
- Sign off before any stage transition

### 11.4 Agent: `Guardian-Compliance`

**Workspace:** `~/openclaw_crypto/guardian/compliance`
**Tools:** Filesystem

**Files:**
- `COMPLIANCE_RULES.md`
- `COMPLIANCE_REVIEW_LOG.md`

**Key Rules:**
- Guardian is a **monitoring tool**, not a financial service. No custody, no advice.
- Disclaimers required: "Guardian cannot prevent transactions. It provides alerts only."
- No collection of private keys, seed phrases, or signing authority.
- Privacy: User configs stored on-chain (user's principal can delete). No off-chain data retention.
- Marketing must not use: "guaranteed protection", "hack-proof", "insured", etc.

---

## 12. Data Flow & Sequence Diagrams

### 12.1 Normal Monitoring Cycle

```
Timer fires
    │
    ▼
Guardian Engine loads user config from Config Canister
    │
    ▼
For chain = ICP:
    │── Call ICRC Index: get_account_transactions(principal, last_watermark)
    │── Receive txs
    │── Update watermark
    │
For chain = Ethereum:
    │── Call EVM RPC: eth_getLogs(user_eth_address, last_block)
    │── Receive logs
    │── Update block watermark
    │
For chain = Bitcoin:
    │── Call ckBTC Index: get_account_transactions(...)
    │── (If native BTC enabled) Call bitcoin_get_utxos(user_btc_addr)
    │── Update watermarks
    │
    ▼
Feed all new events into Detection Engine
    │
    ├── No rules triggered → Log "all clear", return
    │
    └── Rules triggered → Compute alert_score
         │
         ├── Score < alert_threshold → Log, include in daily digest
         │
         └── Score >= alert_threshold → Format alert payload
              │
              ▼
         HTTPS outcall to Telegram/Discord/Email/Webhook
              │
              ▼
         Log alert in stable memory
```

### 12.2 User Config Update

```
User (via Guardian Web UI or API)
    │
    ▼
Call Guardian Config Canister: set_config(new_config)
    │
    ├── canister_inspect_message: verify caller == config.owner, check rate limit
    │
    ├── Validate config: bounds check all fields, reject oversized data
    │
    └── Store in stable BTreeMap<Principal, GuardianConfig>
         │
         ▼
    Return Ok
```

---

## 13. Cycle Budget & Cost Model

### 13.1 Per-User Per-Cycle Cost Estimate

| Operation | Frequency | Cycles per Call | Monthly Cost (cycles) |
|---|---|---|---|
| ICRC Index query (ICP) | Every 30s | ~4M | ~350B |
| ICRC Index query (ckBTC) | Every 30s | ~4M | ~350B |
| ICRC Index query (ckETH) | Every 30s | ~4M | ~350B |
| EVM RPC eth_getLogs (ETH) | Every 60s | ~10B | ~432T |
| EVM RPC eth_getLogs (Base) | Every 60s | ~10B | ~432T |
| Bitcoin UTXO query | Every 5min | ~50M | ~14.4B |
| SOL RPC query | Every 60s | ~10B | ~432T |
| HTTPS outcall (alert) | ~5/day avg | ~50M | ~7.5B |
| Config canister query | Every 30s | ~1M | ~87B |

**Total per user (all chains):** ~1.3T cycles/month ≈ **$1.76 USD/month**

(Using 1T cycles ≈ $1.354 at current SDR rate)

**For ICP-only monitoring:** ~700B cycles/month ≈ **$0.95 USD/month**

### 13.2 Scale Economics

| Users | Monthly Cycle Cost | Monthly USD |
|---|---|---|
| 10 | ~13T | ~$17.60 |
| 100 | ~130T | ~$176 |
| 1,000 | ~1,300T | ~$1,760 |
| 10,000 | ~13,000T | ~$17,600 |

At $5/user/month (Basic tier), 100 users = $500 revenue vs $176 cost = **65% margin**.

---

## 14. Testing Plan

### 14.1 Unit Tests

- Each detection rule tested in isolation with mock transactions
- Config validation: boundary cases, oversized payloads, invalid principals
- Watermark logic: correct incrementing, handling of gaps

### 14.2 Integration Tests (Local dfx)

- Deploy Config + Engine canisters locally
- Deploy local ICRC ledger + index canister
- Simulate transactions, verify alerts fire correctly
- Test upgrade path: deploy v1, populate data, upgrade to v2, verify data intact
- Test cycle drain attack: spam unauthenticated requests, verify `canister_inspect_message` blocks them

### 14.3 Testnet Tests (ICP mainnet, Bitcoin testnet)

- Deploy to ICP mainnet with Bitcoin testnet API
- Use real OISY wallet on testnet
- Execute BTC testnet transactions, verify Guardian detects them
- Test Ethereum (Sepolia testnet) via EVM RPC

### 14.4 Load Tests

- Simulate 1,000 users with synthetic configs
- Run monitoring loop at 30s intervals for 24 hours
- Measure: cycle consumption, latency, false positive rate
- Attempt canister upgrade with full dataset

### 14.5 Security Audit

- Pre-launch: Internal review by Compliance-Guardrail and Risk agents
- Post-launch: Engage a third-party ICP security auditor (e.g., BlockApex, Solidified)

---

## 15. Business Model & Pricing

### 15.1 Tiers

| Tier | Price | Chains | Features |
|---|---|---|---|
| **Free** | $0 | ICP only | 3 basic rules, daily digest only, no real-time alerts |
| **Basic** | $5/mo | ICP + 1 EVM chain | All Category A rules, real-time alerts, 1 alert channel |
| **Pro** | $15/mo | All chains | All rules (A+B), real-time alerts, 3 channels, daily digest, address allowlist |
| **Enterprise** | $50/mo+ | All chains + custom | Custom rules, webhook integration, SLA, priority support |

### 15.2 Payment Mechanism

- **Option A (MVP):** Manual ICP transfer to a designated principal. User gets a subscription NFT or config flag.
- **Option B (v2):** ICRC-2 `approve` + periodic `transfer_from` for recurring payments in ICP/ckUSDT.
- **Option C (v3):** x402 protocol integration for pay-per-check micropayments (see Crypto Orchestrator spec).

### 15.3 Revenue Projections

| Scenario | Month 1 | Month 6 | Month 12 |
|---|---|---|---|
| **Floor** (10 users, mostly Free) | $25 | $250 | $750 |
| **Base** (50 users, mixed tiers) | $150 | $1,500 | $5,000 |
| **Ceiling** (200+ users, Pro/Enterprise) | $500 | $5,000 | $20,000+ |

---

## 16. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| OISY changes its backend API/canister IDs | Medium | High | Pin to known canister IDs. Monitor OISY GitHub releases. Use ICRC standards (stable interfaces). |
| EVM RPC canister rate limits or cost increases | Medium | Medium | Cache results. Use polling intervals as tuning knob. Fall back to HTTPS outcalls to direct RPC providers. |
| False positives erode user trust | High initially | Medium | Start with conservative defaults. Let users tune thresholds. Track false positive rate as a KPI. |
| Regulatory classification as a financial service | Low | High | Maintain "monitoring tool" positioning. No custody, no advice. Clear disclaimers. Compliance agent review. |
| Guardian canister cycle exhaustion | Low | High | `freezing_threshold` at 90+ days. Automated cycle top-up. Alert operator at 30-day runway. |
| Security vulnerability in Guardian canister | Medium | Critical | Follow ALL Section 9 practices. Pre-launch audit. Bug bounty post-launch. |
| OISY adds native Guardian features | Medium | High | Position as complementary / cross-chain. Open-source Guardian as a community tool. |

---

## 17. Roadmap & Milestones

### Phase 1: MVP (Weeks 1-4)

- [ ] Guardian Config Canister: Rust, full CRUD, `canister_inspect_message`, stable storage
- [ ] Guardian Engine Canister: ICP-only monitoring via ICRC Index
- [ ] 3 detection rules: A1 (large transfer), A3 (rapid txs), A4 (new address)
- [ ] Telegram alerts via HTTPS outcall
- [ ] Local dfx tests passing
- [ ] Deploy to ICP mainnet
- [ ] Compliance + Risk sign-off

### Phase 2: Multi-Chain (Weeks 5-8)

- [ ] Add EVM chain monitoring (ETH, Base, Polygon) via EVM RPC canister
- [ ] Add ckBTC/ckETH monitoring via ICRC Index
- [ ] Add Bitcoin native UTXO monitoring
- [ ] Add behavioral rules B1-B4
- [ ] Add Discord + Email alert channels
- [ ] Subscription payment via ICP transfer

### Phase 3: Advanced Detection (Weeks 9-12)

- [ ] Add Solana monitoring via SOL RPC canister
- [ ] Cross-chain correlation engine
- [ ] Address risk scoring (via public APIs / HTTPS outcalls)
- [ ] ICRC-21 consent message validation
- [ ] Web dashboard (canister-served frontend)
- [ ] Public launch

### Phase 4: Scale & Harden (Months 4-6)

- [ ] Third-party security audit
- [ ] Multi-sig canister control (Orbit)
- [ ] x402 payment integration
- [ ] API for third-party dApps to query Guardian status
- [ ] Consider SNS for full decentralization

---

## Appendix A: ICRC Standards Referenced

| Standard | Name | Used For |
|---|---|---|
| ICRC-1 | Fungible Token Standard | Token transfer monitoring |
| ICRC-2 | Approve and Transfer From | Detecting unlimited approvals |
| ICRC-21 | Canister Call Consent Messages | Validating transaction explanations |
| ICRC-25 | Signer Interaction Standard | Understanding permission grants |
| ICRC-27 | Accounts | Querying wallet accounts |
| ICRC-49 | Call Canister | Monitoring canister call execution |

---

## Appendix B: Known OISY Canister IDs

| Canister | ID | Controller |
|---|---|---|
| OISY Frontend | `cha4i-riaaa-aaaan-qeccq-cai` | OISY dev team |
| OISY Backend | `doked-biaaa-aaaar-qag2a-cai` | OISY dev team |
| Chain Fusion Signer | `grghe-syaaa-aaaar-qabyq-cai` | NNS |
| EVM RPC Canister | `7hfb6-caaaa-aaaar-qadga-cai` | NNS |
| ICP Ledger | `ryjl3-tyaaa-aaaaa-aaaba-cai` | NNS |
| ckBTC Minter | `mqygn-kiaaa-aaaar-qaadq-cai` | NNS |
| ckETH Minter | `sv3dd-oaaaa-aaaar-qacoa-cai` | NNS |
| Internet Identity | `rdmx6-jaaaa-aaaaa-aaadq-cai` | NNS |

---

## Appendix C: ICP Bitcoin API Pricing (Mainnet)

| API Call | Cycles | USD |
|---|---|---|
| `bitcoin_get_utxos` | 50M + 1 cycle/Wasm instruction | ~$0.000068 + compute |
| `bitcoin_get_balance` | 10M | ~$0.000014 |
| `bitcoin_get_current_fee_percentiles` | 10M | ~$0.000014 |
| `bitcoin_send_transaction` | 5B + 20M/byte | ~$0.0068 + payload |
| `bitcoin_get_block_headers` | 50M + 1 cycle/Wasm instruction | ~$0.000068 + compute |

Minimum cycles to attach: 10B for UTXO/block header queries, 100M for balance/fee queries.

---

## Appendix D: Guardian OpenClaw Boot Instructions

When the Crypto-Orchestrator starts work on Guardian, it should:

1. Read this spec in full: `~/openclaw_crypto/guardian/OISY_GUARDIAN_SPEC.md`
2. Create task board: `~/openclaw_crypto/guardian/orchestrator/GUARDIAN_TASKS.md`
3. Assign Phase 1 tasks to Guardian-Dev
4. Request Risk sign-off on default rule weights (Section 5)
5. Request Compliance sign-off on disclaimer language and marketing constraints
6. Only after both sign-offs: authorize Guardian-Dev to begin coding
7. After Dev completes Phase 1 and local tests pass: review code, then authorize testnet deployment
8. After testnet validation: authorize mainnet deployment with ICP-only monitoring
9. After 2 weeks stable mainnet operation: authorize Phase 2

**Never skip a sign-off gate. Never deploy with real user funds at risk without explicit human confirmation.**

---

*End of OISY Guardian Agent Specification v1.0*
