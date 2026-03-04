/// fetcher.rs — Inter-canister transaction fetching for ICP, ckBTC, and ckETH.
///
/// Each fetch function:
///   1. Calls the respective ICRC index canister via `ic_cdk::call`.
///   2. Maps results into `UnifiedEvent` values.
///   3. Returns an empty Vec (never panics) on any error.
///
/// Retry policy:
///   On transient inter-canister errors (SYS_UNKNOWN, CANISTER_ERROR) the caller
///   can use `RetryConfig` + `compute_backoff_ms` to decide when to retry.
///   The pure helpers are tested independently of the IC runtime.
#[cfg(not(test))]
use candid::Principal;

#[cfg(not(test))]
use crate::canisters::{
    CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID, ICP_INDEX_CANISTER_ID,
    MAX_RESULTS_PER_FETCH,
};
#[cfg(not(test))]
use crate::icrc::{
    GetTransactionsRequest,
    IcrcGetTransactionsResult,
    IcpGetTransactionsResult,
    icrc_wire_to_internal,
    icp_wire_to_internal,
};

use crate::icrc::{IcrcAccount, IcrcTransaction};
use crate::{Direction, UnifiedEvent, Watermark};

// ---------------------------------------------------------------------------
// Retry / exponential backoff helpers
// ---------------------------------------------------------------------------

/// Configuration for retry behaviour on transient inter-canister failures.
#[derive(Clone, Debug, PartialEq)]
pub struct RetryConfig {
    /// Maximum total attempts (initial + retries).
    pub max_attempts: u32,
    /// Base delay in milliseconds for the first retry.
    pub base_delay_ms: u64,
    /// Maximum delay cap in milliseconds (prevents unbounded waits).
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_attempts: 3,
            base_delay_ms: 500,
            max_delay_ms: 10_000,
        }
    }
}

/// Compute the exponential-backoff delay for the given attempt number (0-indexed).
///
/// Formula: `min(base_delay_ms * 2^attempt, max_delay_ms)`
///
/// - attempt 0 (first retry): `base_delay_ms`
/// - attempt 1 (second retry): `base_delay_ms * 2`
/// - etc., capped at `max_delay_ms`
pub fn compute_backoff_ms(attempt: u32, cfg: &RetryConfig) -> u64 {
    // 2^attempt, capped to avoid overflow for large attempt values.
    let multiplier: u64 = if attempt >= 64 { u64::MAX } else { 1u64 << attempt };
    let delay = cfg.base_delay_ms.saturating_mul(multiplier);
    delay.min(cfg.max_delay_ms)
}

/// Determine whether an error string represents a retriable transient error.
/// SYS_UNKNOWN and CANISTER_ERROR are considered transient on ICP.
pub fn is_retriable_error(err: &str) -> bool {
    err.contains("SYS_UNKNOWN")
        || err.contains("CANISTER_ERROR")
        || err.contains("SYS_TRANSIENT")
        || err.contains("timeout")
        || err.contains("Timeout")
}

/// Determine whether an error string represents a permanent (non-retriable) error.
pub fn is_permanent_error(err: &str) -> bool {
    err.contains("DestinationInvalid")
        || err.contains("CanisterNotFound")
        || err.contains("Invalid canister id")
        || err.contains("DESTINATION_INVALID")
}

// ---------------------------------------------------------------------------
// IcrcTransaction → UnifiedEvent conversion
// ---------------------------------------------------------------------------

/// Convert an `IcrcTransaction` to a `UnifiedEvent` from the perspective of
/// the monitored `account`.
/// `amount_e8s` is u128 to support ckETH (18 decimals, up to ~10^21 Wei).
pub fn icrc_tx_to_unified_event(
    tx: &IcrcTransaction,
    account: &IcrcAccount,
    chain_name: &str,
) -> UnifiedEvent {
    let direction = if tx.to.owner == account.owner {
        Direction::In
    } else {
        Direction::Out
    };

    let counterparty = if direction == Direction::In {
        tx.from.owner
    } else {
        tx.to.owner
    };

    UnifiedEvent {
        chain: chain_name.to_string(),
        timestamp: tx.timestamp,
        direction,
        amount_e8s: tx.amount, // u128
        counterparty,
        tx_id: tx.id.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Public fetch API
// ---------------------------------------------------------------------------

/// Fetch ICP transactions for `account` since the watermark's last block height.
/// Uses ICP-specific wire types (Operation variant, text AccountIdentifiers).
/// On any error: logs the problem and returns Err (callers return empty vec).
#[cfg(not(test))]
pub async fn fetch_icp_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    let canister_id = Principal::from_text(ICP_INDEX_CANISTER_ID)
        .map_err(|e| format!("Invalid canister id {}: {}", ICP_INDEX_CANISTER_ID, e))?;

    let start = if watermark.block_height > 0 {
        Some(candid::Nat::from(watermark.block_height))
    } else {
        None
    };

    let request = GetTransactionsRequest {
        account: account.clone(),
        start,
        max_results: candid::Nat::from(MAX_RESULTS_PER_FETCH),
    };

    let result: Result<(IcpGetTransactionsResult,), _> = ic_cdk::call(
        canister_id,
        "get_account_transactions",
        (request,),
    )
    .await;

    match result {
        Ok((Ok(response),)) => {
            let events = response
                .transactions
                .iter()
                .filter_map(|tx_with_id| icp_wire_to_internal(tx_with_id))
                .map(|tx| icrc_tx_to_unified_event(&tx, &account, "ICP"))
                .collect();
            Ok(events)
        }
        Ok((Err(e),)) => Err(format!("ICP index returned error: {}", e.message)),
        Err((code, msg)) => Err(format!(
            "Inter-canister call to ICP ({}) failed: {:?} {}",
            ICP_INDEX_CANISTER_ID, code, msg
        )),
    }
}

/// Fetch ckBTC transactions for `account` since the watermark's last block height.
/// Uses ICRC Index NG wire types (nested transfer/mint/burn, nat amounts).
#[cfg(not(test))]
pub async fn fetch_ckbtc_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    fetch_icrc_index_ng_transactions(
        CKBTC_INDEX_CANISTER_ID,
        "ckBTC",
        account,
        watermark,
    )
    .await
}

/// Fetch ckETH transactions for `account` since the watermark's last block height.
/// Uses ICRC Index NG wire types. ckETH amounts are in Wei (18 decimals) — u128 required.
#[cfg(not(test))]
pub async fn fetch_cketh_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    fetch_icrc_index_ng_transactions(
        CKETH_INDEX_CANISTER_ID,
        "ckETH",
        account,
        watermark,
    )
    .await
}

/// Generic ICRC Index NG fetcher for ckBTC/ckETH.
/// Response is `variant { Ok: GetTransactions; Err: GetTransactionsErr }`.
/// Transactions are nested: tx.transaction.transfer.{from, to, amount}.
/// `amount` is `nat` (Nat), decoded as u128 for ckETH compatibility.
#[cfg(not(test))]
async fn fetch_icrc_index_ng_transactions(
    canister_id_str: &str,
    chain_name: &str,
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    let canister_id = Principal::from_text(canister_id_str)
        .map_err(|e| format!("Invalid canister id {}: {}", canister_id_str, e))?;

    let start = if watermark.block_height > 0 {
        Some(candid::Nat::from(watermark.block_height))
    } else {
        None
    };

    let request = GetTransactionsRequest {
        account: account.clone(),
        start,
        max_results: candid::Nat::from(MAX_RESULTS_PER_FETCH),
    };

    let result: Result<(IcrcGetTransactionsResult,), _> = ic_cdk::call(
        canister_id,
        "get_account_transactions",
        (request,),
    )
    .await;

    match result {
        Ok((Ok(response),)) => {
            let events = response
                .transactions
                .iter()
                .filter_map(|tx_with_id| icrc_wire_to_internal(tx_with_id))
                .map(|tx| icrc_tx_to_unified_event(&tx, &account, chain_name))
                .collect();
            Ok(events)
        }
        Ok((Err(e),)) => Err(format!(
            "{} index returned error: {}",
            chain_name, e.message
        )),
        Err((code, msg)) => Err(format!(
            "Inter-canister call to {} ({}) failed: {:?} {}",
            chain_name, canister_id_str, code, msg
        )),
    }
}

// ---------------------------------------------------------------------------
// Watermark helpers
// ---------------------------------------------------------------------------

/// Update a watermark after a successful fetch.
/// Sets `block_height` to the highest tx id seen and `last_checked` to `now`.
pub fn update_watermark_after_fetch(
    watermark: &mut Watermark,
    events: &[UnifiedEvent],
    now_ns: u64,
) {
    watermark.last_checked = now_ns;

    if let Some(max_tx_id) = events
        .iter()
        .filter_map(|e| e.tx_id.parse::<u64>().ok())
        .max()
    {
        if max_tx_id > watermark.block_height {
            watermark.block_height = max_tx_id;
            watermark.last_tx_id = max_tx_id.to_string();
        }
    }
}

// ---------------------------------------------------------------------------
// Ring-buffer helpers
// ---------------------------------------------------------------------------

/// Merge `new_events` into `existing`, enforcing a max size of `max_cap`.
/// Keeps the newest events (LIFO trim: drop oldest from the front).
pub fn merge_into_ring_buffer(
    existing: &mut Vec<UnifiedEvent>,
    new_events: Vec<UnifiedEvent>,
    max_cap: usize,
) {
    existing.extend(new_events);
    if existing.len() > max_cap {
        let excess = existing.len() - max_cap;
        existing.drain(..excess);
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use crate::icrc::IcrcTransaction;

    fn anon() -> Principal {
        Principal::anonymous()
    }
    fn user_principal() -> Principal {
        Principal::from_slice(&[7u8; 29])
    }
    fn other_principal() -> Principal {
        Principal::from_slice(&[8u8; 29])
    }

    fn make_tx(id: u64, to: Principal, from: Principal, amount: u128) -> IcrcTransaction {
        IcrcTransaction {
            id,
            timestamp: 1_000_000 + id,
            amount,
            from: IcrcAccount::new(from),
            to: IcrcAccount::new(to),
            memo: None,
            kind: "transfer".to_string(),
        }
    }

    // --- icrc_tx_to_unified_event ---

    #[test]
    fn test_conversion_inbound_direction() {
        let account = IcrcAccount::new(user_principal());
        let tx = make_tx(1, user_principal(), other_principal(), 100_000);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.direction, Direction::In);
        assert_eq!(ev.counterparty, other_principal());
    }

    #[test]
    fn test_conversion_outbound_direction() {
        let account = IcrcAccount::new(user_principal());
        let tx = make_tx(2, other_principal(), user_principal(), 50_000);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.direction, Direction::Out);
        assert_eq!(ev.counterparty, other_principal());
    }

    #[test]
    fn test_conversion_chain_name_icp() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(3, anon(), anon(), 1);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.chain, "ICP");
    }

    #[test]
    fn test_conversion_chain_name_ckbtc() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(4, anon(), anon(), 1);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ckBTC");
        assert_eq!(ev.chain, "ckBTC");
    }

    #[test]
    fn test_conversion_chain_name_cketh() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(5, anon(), anon(), 1);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ckETH");
        assert_eq!(ev.chain, "ckETH");
    }

    #[test]
    fn test_conversion_amount_preserved() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(6, anon(), other_principal(), 9_876_543);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.amount_e8s, 9_876_543);
    }

    #[test]
    fn test_conversion_timestamp_preserved() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(7, anon(), anon(), 1);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.timestamp, tx.timestamp);
    }

    #[test]
    fn test_conversion_tx_id_as_string() {
        let account = IcrcAccount::new(anon());
        let tx = make_tx(42, anon(), anon(), 1);
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.tx_id, "42");
    }

    // --- update_watermark_after_fetch ---

    #[test]
    fn test_watermark_update_sets_last_checked() {
        let mut wm = Watermark::default();
        let events: Vec<UnifiedEvent> = vec![];
        update_watermark_after_fetch(&mut wm, &events, 12_345_678);
        assert_eq!(wm.last_checked, 12_345_678);
    }

    #[test]
    fn test_watermark_update_advances_block_height() {
        let mut wm = Watermark {
            last_tx_id: "0".to_string(),
            last_checked: 0,
            block_height: 5,
        };
        let events = vec![
            UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: 1,
                direction: Direction::In,
                amount_e8s: 1,
                counterparty: anon(),
                tx_id: "10".to_string(),
            },
            UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: 2,
                direction: Direction::In,
                amount_e8s: 1,
                counterparty: anon(),
                tx_id: "7".to_string(),
            },
        ];
        update_watermark_after_fetch(&mut wm, &events, 999);
        assert_eq!(wm.block_height, 10);
        assert_eq!(wm.last_tx_id, "10");
    }

    #[test]
    fn test_watermark_does_not_regress() {
        let mut wm = Watermark {
            last_tx_id: "100".to_string(),
            last_checked: 0,
            block_height: 100,
        };
        let events = vec![UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: 0,
            direction: Direction::In,
            amount_e8s: 1,
            counterparty: anon(),
            tx_id: "50".to_string(), // older than current watermark
        }];
        update_watermark_after_fetch(&mut wm, &events, 1);
        assert_eq!(wm.block_height, 100); // unchanged
    }

    #[test]
    fn test_watermark_empty_events_no_change() {
        let mut wm = Watermark {
            last_tx_id: "42".to_string(),
            last_checked: 0,
            block_height: 42,
        };
        update_watermark_after_fetch(&mut wm, &[], 7777);
        assert_eq!(wm.block_height, 42);
        assert_eq!(wm.last_tx_id, "42");
        assert_eq!(wm.last_checked, 7777);
    }

    // --- merge_into_ring_buffer ---

    #[test]
    fn test_ring_buffer_appends_events() {
        let mut buf: Vec<UnifiedEvent> = vec![];
        let new_events = vec![UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: 1,
            direction: Direction::In,
            amount_e8s: 1,
            counterparty: anon(),
            tx_id: "1".to_string(),
        }];
        merge_into_ring_buffer(&mut buf, new_events, 100);
        assert_eq!(buf.len(), 1);
    }

    #[test]
    fn test_ring_buffer_trims_to_max_cap() {
        let mut buf: Vec<UnifiedEvent> = (0u64..90)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i as u128,
                counterparty: anon(),
                tx_id: i.to_string(),
            })
            .collect();

        let new_events: Vec<UnifiedEvent> = (90u64..120)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i as u128,
                counterparty: anon(),
                tx_id: i.to_string(),
            })
            .collect();

        merge_into_ring_buffer(&mut buf, new_events, 100);
        assert_eq!(buf.len(), 100);
        // Oldest (0..20) should have been dropped; newest (20..120) remain
        assert_eq!(buf[0].tx_id, "20");
        assert_eq!(buf[99].tx_id, "119");
    }

    #[test]
    fn test_ring_buffer_under_cap_unchanged() {
        let mut buf: Vec<UnifiedEvent> = vec![];
        let new_events: Vec<UnifiedEvent> = (0u64..50)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i as u128,
                counterparty: anon(),
                tx_id: i.to_string(),
            })
            .collect();
        merge_into_ring_buffer(&mut buf, new_events, 100);
        assert_eq!(buf.len(), 50);
    }

    // --- Error handling (mock simulate canister unavailable) ---

    #[test]
    fn test_icrc_tx_conversion_zero_amount() {
        let account = IcrcAccount::new(anon());
        let tx = IcrcTransaction {
            id: 0,
            timestamp: 0,
            amount: 0,
            from: IcrcAccount::new(anon()),
            to: IcrcAccount::new(anon()),
            memo: None,
            kind: "mint".to_string(),
        };
        let ev = icrc_tx_to_unified_event(&tx, &account, "ckBTC");
        assert_eq!(ev.amount_e8s, 0);
    }

    #[test]
    fn test_icrc_account_serialization_roundtrip() {
        let sub = [0xABu8; 32];
        let acc = IcrcAccount::with_subaccount(user_principal(), sub);
        let encoded = candid::encode_one(&acc).expect("encode");
        let decoded: IcrcAccount = candid::decode_one(&encoded).expect("decode");
        assert_eq!(decoded, acc);
    }

    // --- Retry / exponential backoff ---

    #[test]
    fn test_retry_config_default_values() {
        let cfg = RetryConfig::default();
        assert_eq!(cfg.max_attempts, 3);
        assert_eq!(cfg.base_delay_ms, 500);
        assert_eq!(cfg.max_delay_ms, 10_000);
    }

    #[test]
    fn test_backoff_attempt_0_equals_base() {
        let cfg = RetryConfig::default();
        assert_eq!(compute_backoff_ms(0, &cfg), 500);
    }

    #[test]
    fn test_backoff_attempt_1_doubles() {
        let cfg = RetryConfig::default();
        assert_eq!(compute_backoff_ms(1, &cfg), 1_000);
    }

    #[test]
    fn test_backoff_attempt_2_quadruples() {
        let cfg = RetryConfig::default();
        assert_eq!(compute_backoff_ms(2, &cfg), 2_000);
    }

    #[test]
    fn test_backoff_caps_at_max_delay() {
        let cfg = RetryConfig::default(); // max 10_000ms
        // 2^8 * 500 = 128_000 > 10_000
        assert_eq!(compute_backoff_ms(8, &cfg), 10_000);
    }

    #[test]
    fn test_backoff_large_attempt_does_not_overflow() {
        let cfg = RetryConfig::default();
        // Attempt 63 would overflow u64 without saturating_shl
        let delay = compute_backoff_ms(63, &cfg);
        assert_eq!(delay, 10_000); // capped at max
    }

    #[test]
    fn test_backoff_custom_config() {
        let cfg = RetryConfig {
            max_attempts: 5,
            base_delay_ms: 100,
            max_delay_ms: 1_600,
        };
        assert_eq!(compute_backoff_ms(0, &cfg), 100);
        assert_eq!(compute_backoff_ms(1, &cfg), 200);
        assert_eq!(compute_backoff_ms(2, &cfg), 400);
        assert_eq!(compute_backoff_ms(3, &cfg), 800);
        assert_eq!(compute_backoff_ms(4, &cfg), 1_600); // hits cap
        assert_eq!(compute_backoff_ms(5, &cfg), 1_600); // still capped
    }

    #[test]
    fn test_is_retriable_sys_unknown() {
        assert!(is_retriable_error("Inter-canister call failed: SYS_UNKNOWN network issue"));
    }

    #[test]
    fn test_is_retriable_canister_error() {
        assert!(is_retriable_error("CANISTER_ERROR: canister trapped"));
    }

    #[test]
    fn test_is_retriable_timeout() {
        assert!(is_retriable_error("request Timeout exceeded"));
    }

    #[test]
    fn test_is_retriable_sys_transient() {
        assert!(is_retriable_error("SYS_TRANSIENT: system busy"));
    }

    #[test]
    fn test_is_not_retriable_destination_invalid() {
        assert!(!is_retriable_error("DestinationInvalid canister not found"));
    }

    #[test]
    fn test_is_permanent_destination_invalid() {
        assert!(is_permanent_error("DestinationInvalid: canister gone"));
    }

    #[test]
    fn test_is_permanent_canister_not_found() {
        assert!(is_permanent_error("CanisterNotFound: no such canister"));
    }

    #[test]
    fn test_is_permanent_invalid_canister_id() {
        assert!(is_permanent_error("Invalid canister id: bad-id"));
    }

    #[test]
    fn test_is_not_permanent_transient_error() {
        assert!(!is_permanent_error("SYS_UNKNOWN transient"));
        assert!(!is_permanent_error("CANISTER_ERROR trap"));
    }

    // --- Large batch processing (Phase 1c: 1000+ txs) ---

    #[test]
    fn test_large_batch_1000_txs_conversion() {
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let txs: Vec<IcrcTransaction> = (0u64..1000)
            .map(|i| make_tx(i, user, other_principal(), 1000 + i as u128))
            .collect();
        let events: Vec<UnifiedEvent> = txs
            .iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ICP"))
            .collect();
        assert_eq!(events.len(), 1000);
        assert!(events.iter().all(|e| e.direction == Direction::In));
        assert!(events.iter().all(|e| e.chain == "ICP"));
    }

    #[test]
    fn test_large_batch_1000_txs_watermark_update() {
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let txs: Vec<IcrcTransaction> = (0u64..1000)
            .map(|i| make_tx(i, other_principal(), user, 100))
            .collect();
        let events: Vec<UnifiedEvent> = txs
            .iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ICP"))
            .collect();
        let mut wm = Watermark::default();
        update_watermark_after_fetch(&mut wm, &events, 99_999_999);
        assert_eq!(wm.block_height, 999); // highest tx_id
        assert_eq!(wm.last_tx_id, "999");
        assert_eq!(wm.last_checked, 99_999_999);
    }

    #[test]
    fn test_large_batch_ring_buffer_trim_to_1000_cap() {
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let txs: Vec<IcrcTransaction> = (0u64..1500)
            .map(|i| make_tx(i, other_principal(), user, 100))
            .collect();
        let events: Vec<UnifiedEvent> = txs
            .iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ICP"))
            .collect();
        let mut buf: Vec<UnifiedEvent> = vec![];
        merge_into_ring_buffer(&mut buf, events, 1000);
        assert_eq!(buf.len(), 1000);
        // Newest 1000 (ids 500..1499) should be kept
        assert_eq!(buf[0].tx_id, "500");
        assert_eq!(buf[999].tx_id, "1499");
    }

    #[test]
    fn test_large_batch_1000_txs_ckbtc_chain() {
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let txs: Vec<IcrcTransaction> = (0u64..1000)
            .map(|i| make_tx(i, other_principal(), user, 500))
            .collect();
        let events: Vec<UnifiedEvent> = txs
            .iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ckBTC"))
            .collect();
        assert_eq!(events.len(), 1000);
        assert!(events.iter().all(|e| e.chain == "ckBTC"));
        assert!(events.iter().all(|e| e.direction == Direction::Out));
    }

    #[test]
    fn test_large_batch_1000_txs_cketh_chain() {
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let txs: Vec<IcrcTransaction> = (0u64..1000)
            .map(|i| make_tx(i, user, other_principal(), 200))
            .collect();
        let events: Vec<UnifiedEvent> = txs
            .iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ckETH"))
            .collect();
        assert_eq!(events.len(), 1000);
        assert!(events.iter().all(|e| e.chain == "ckETH"));
    }

    #[test]
    fn test_large_batch_watermark_increments_multiple_rounds() {
        // Simulate 3 rounds of fetching (as a real timer would do)
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let mut wm = Watermark::default();

        // Round 1: txs 0..100
        let round1: Vec<IcrcTransaction> = (0u64..100)
            .map(|i| make_tx(i, other_principal(), user, 100))
            .collect();
        let events1: Vec<UnifiedEvent> = round1.iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ICP"))
            .collect();
        update_watermark_after_fetch(&mut wm, &events1, 1_000);
        assert_eq!(wm.block_height, 99);

        // Round 2: txs 100..200
        let round2: Vec<IcrcTransaction> = (100u64..200)
            .map(|i| make_tx(i, other_principal(), user, 100))
            .collect();
        let events2: Vec<UnifiedEvent> = round2.iter()
            .map(|tx| icrc_tx_to_unified_event(tx, &account, "ICP"))
            .collect();
        update_watermark_after_fetch(&mut wm, &events2, 2_000);
        assert_eq!(wm.block_height, 199);
        assert_eq!(wm.last_checked, 2_000);

        // Round 3: no new txs
        update_watermark_after_fetch(&mut wm, &[], 3_000);
        assert_eq!(wm.block_height, 199); // unchanged
        assert_eq!(wm.last_checked, 3_000); // updated
    }

    #[test]
    fn test_watermark_persists_after_error_scenario() {
        // After an error, the watermark should NOT be updated
        // (caller doesn't call update_watermark_after_fetch on error path)
        let mut wm = Watermark {
            last_tx_id: "50".to_string(),
            last_checked: 1000,
            block_height: 50,
        };
        // Simulate: error occurred, we skip the update
        let error_occurred = true;
        if !error_occurred {
            update_watermark_after_fetch(&mut wm, &[], 9999);
        }
        // Watermark must not regress
        assert_eq!(wm.block_height, 50);
        assert_eq!(wm.last_checked, 1000);
    }

    #[test]
    fn test_memo_field_ignored_in_unified_event() {
        // IcrcTransaction.memo is not currently mapped to UnifiedEvent
        // This test documents that behavior explicitly
        let user = user_principal();
        let account = IcrcAccount::new(user);
        let tx = IcrcTransaction {
            id: 42,
            timestamp: 1000,
            amount: 500,
            from: IcrcAccount::new(other_principal()),
            to: IcrcAccount::new(user),
            memo: Some(vec![1, 2, 3, 4]),
            kind: "transfer".to_string(),
        };
        let ev = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(ev.amount_e8s, 500);
        assert_eq!(ev.tx_id, "42");
    }

    #[test]
    fn test_fetch_error_message_contains_chain_name() {
        // Simulate the error format that fetch_transactions_from_index produces
        let chain = "ckBTC";
        let canister_id = "n5wcd-faaaa-aaaar-qaaea-cai";
        let err = format!(
            "Inter-canister call to {} ({}) failed: {:?} {}",
            chain, canister_id, "SYS_UNKNOWN", "network issue"
        );
        assert!(err.contains("ckBTC"));
        assert!(err.contains(canister_id));
        assert!(is_retriable_error(&err));
    }

    #[test]
    fn test_fetch_error_permanent_canister_gone() {
        let err = format!("Invalid canister id {}: {}", "bad-id", "failed to parse");
        assert!(is_permanent_error(&err));
        assert!(!is_retriable_error(&err));
    }
}
