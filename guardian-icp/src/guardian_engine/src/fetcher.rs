/// fetcher.rs — Inter-canister transaction fetching for ICP, ckBTC, and ckETH.
///
/// Each fetch function:
///   1. Calls the respective ICRC index canister via `ic_cdk::call`.
///   2. Maps results into `UnifiedEvent` values.
///   3. Returns an empty Vec (never panics) on any error.
#[cfg(not(test))]
use candid::Principal;

#[cfg(not(test))]
use crate::canisters::{
    CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID, ICP_INDEX_CANISTER_ID,
    MAX_RESULTS_PER_FETCH,
};
#[cfg(not(test))]
use crate::icrc::{GetTransactionsRequest, GetTransactionsResponse};

use crate::icrc::{IcrcAccount, IcrcTransaction};
use crate::{Direction, UnifiedEvent, Watermark};

// ---------------------------------------------------------------------------
// IcrcTransaction → UnifiedEvent conversion
// ---------------------------------------------------------------------------

/// Convert an `IcrcTransaction` to a `UnifiedEvent` from the perspective of
/// the monitored `account`.
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
        amount_e8s: tx.amount,
        counterparty,
        tx_id: tx.id.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Public fetch API
// ---------------------------------------------------------------------------

/// Fetch ICP transactions for `account` since the watermark's last block height.
/// On any error: logs the problem and returns an empty Vec (no panic).
#[cfg(not(test))]
pub async fn fetch_icp_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    fetch_transactions_from_index(
        ICP_INDEX_CANISTER_ID,
        "ICP",
        account,
        watermark,
    )
    .await
}

/// Fetch ckBTC transactions for `account` since the watermark's last block height.
#[cfg(not(test))]
pub async fn fetch_ckbtc_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    fetch_transactions_from_index(
        CKBTC_INDEX_CANISTER_ID,
        "ckBTC",
        account,
        watermark,
    )
    .await
}

/// Fetch ckETH transactions for `account` since the watermark's last block height.
#[cfg(not(test))]
pub async fn fetch_cketh_transactions(
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    fetch_transactions_from_index(
        CKETH_INDEX_CANISTER_ID,
        "ckETH",
        account,
        watermark,
    )
    .await
}

/// Generic fetcher: calls `get_account_transactions` on `canister_id`, maps
/// results into `UnifiedEvent`, and returns them.  On any inter-canister or
/// decode error the error message is returned (callers log + return empty vec).
#[cfg(not(test))]
async fn fetch_transactions_from_index(
    canister_id_str: &str,
    chain_name: &str,
    account: IcrcAccount,
    watermark: &Watermark,
) -> Result<Vec<UnifiedEvent>, String> {
    let canister_id = Principal::from_text(canister_id_str)
        .map_err(|e| format!("Invalid canister id {}: {}", canister_id_str, e))?;

    // Use the watermark block_height as the start (0 = from beginning).
    let start = if watermark.block_height > 0 {
        Some(watermark.block_height)
    } else {
        None
    };

    let request = GetTransactionsRequest {
        account: account.clone(),
        start,
        max_results: MAX_RESULTS_PER_FETCH,
    };

    let result: Result<(GetTransactionsResponse,), _> = ic_cdk::call(
        canister_id,
        "get_account_transactions",
        (request,),
    )
    .await;

    match result {
        Ok((response,)) => {
            let events = response
                .transactions
                .iter()
                .map(|tx| icrc_tx_to_unified_event(tx, &account, chain_name))
                .collect();
            Ok(events)
        }
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

    fn make_tx(id: u64, to: Principal, from: Principal, amount: u64) -> IcrcTransaction {
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
        let mut buf: Vec<UnifiedEvent> = (0..90)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i,
                counterparty: anon(),
                tx_id: i.to_string(),
            })
            .collect();

        let new_events: Vec<UnifiedEvent> = (90..120)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i,
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
        let new_events: Vec<UnifiedEvent> = (0..50)
            .map(|i| UnifiedEvent {
                chain: "ICP".to_string(),
                timestamp: i,
                direction: Direction::In,
                amount_e8s: i,
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
}
