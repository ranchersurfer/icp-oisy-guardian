/// ICP Ledger canister (mainnet) — used for icrc1_balance_of queries.
pub const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

/// ckBTC Ledger canister (mainnet) — used for icrc1_balance_of queries.
pub const CKBTC_LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

/// ckETH Ledger canister (mainnet) — used for icrc1_balance_of queries.
pub const CKETH_LEDGER_CANISTER_ID: &str = "ss2fx-dyaaa-aaaar-qacoq-cai";

/// ICP Index canister (mainnet).
/// Provides ICRC-1 transaction history for the ICP token.
/// Note: This is the ICP *Index* canister (qhbym-qaaaa-aaaaa-aaafq-cai),
/// NOT the ICP Ledger (ryjl3-tyaaa-aaaaa-aaaba-cai).
pub const ICP_INDEX_CANISTER_ID: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";

/// ckBTC Index canister (mainnet).
/// Provides ICRC-1 transaction history for chain-key Bitcoin.
pub const CKBTC_INDEX_CANISTER_ID: &str = "n5wcd-faaaa-aaaar-qaaea-cai";

/// ckETH Index canister (mainnet).
/// Provides ICRC-1 transaction history for chain-key Ethereum.
pub const CKETH_INDEX_CANISTER_ID: &str = "s3zol-vqaaa-aaaar-qacpa-cai";

/// Maximum number of results to request per fetch call.
pub const MAX_RESULTS_PER_FETCH: u64 = 100;

/// Maximum events stored per user in the ring buffer.
pub const MAX_EVENTS_PER_USER: usize = 100;

/// Maximum seen tx IDs stored per user for deduplication.
pub const MAX_SEEN_TX_IDS_PER_USER: usize = 1000;

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_icp_ledger_canister_id_valid() {
        let result = Principal::from_text(ICP_LEDGER_CANISTER_ID);
        assert!(result.is_ok(), "ICP_LEDGER_CANISTER_ID should be a valid principal");
        assert_eq!(ICP_LEDGER_CANISTER_ID, "ryjl3-tyaaa-aaaaa-aaaba-cai");
    }

    #[test]
    fn test_ckbtc_ledger_canister_id_valid() {
        let result = Principal::from_text(CKBTC_LEDGER_CANISTER_ID);
        assert!(result.is_ok(), "CKBTC_LEDGER_CANISTER_ID should be a valid principal");
    }

    #[test]
    fn test_cketh_ledger_canister_id_valid() {
        let result = Principal::from_text(CKETH_LEDGER_CANISTER_ID);
        assert!(result.is_ok(), "CKETH_LEDGER_CANISTER_ID should be a valid principal");
    }

    #[test]
    fn test_ledger_ids_distinct_from_index_ids() {
        assert_ne!(ICP_LEDGER_CANISTER_ID, ICP_INDEX_CANISTER_ID);
        assert_ne!(CKBTC_LEDGER_CANISTER_ID, CKBTC_INDEX_CANISTER_ID);
        assert_ne!(CKETH_LEDGER_CANISTER_ID, CKETH_INDEX_CANISTER_ID);
    }

    #[test]
    fn test_icp_index_canister_id_valid() {
        let result = Principal::from_text(ICP_INDEX_CANISTER_ID);
        assert!(result.is_ok(), "ICP_INDEX_CANISTER_ID should be a valid principal");
    }

    #[test]
    fn test_icp_index_canister_id_is_index_not_ledger() {
        // The ledger is ryjl3-tyaaa-aaaaa-aaaba-cai — make sure we're not using it
        assert_ne!(
            ICP_INDEX_CANISTER_ID, "ryjl3-tyaaa-aaaaa-aaaba-cai",
            "ICP_INDEX_CANISTER_ID must be the Index canister, not the Ledger"
        );
        // Correct index canister
        assert_eq!(ICP_INDEX_CANISTER_ID, "qhbym-qaaaa-aaaaa-aaafq-cai");
    }

    #[test]
    fn test_ckbtc_index_canister_id_valid() {
        let result = Principal::from_text(CKBTC_INDEX_CANISTER_ID);
        assert!(result.is_ok(), "CKBTC_INDEX_CANISTER_ID should be a valid principal");
    }

    #[test]
    fn test_cketh_index_canister_id_valid() {
        let result = Principal::from_text(CKETH_INDEX_CANISTER_ID);
        assert!(result.is_ok(), "CKETH_INDEX_CANISTER_ID should be a valid principal");
    }

    #[test]
    fn test_canister_ids_are_distinct() {
        assert_ne!(ICP_INDEX_CANISTER_ID, CKBTC_INDEX_CANISTER_ID);
        assert_ne!(ICP_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID);
        assert_ne!(CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID);
    }

    #[test]
    fn test_max_results_per_fetch_reasonable() {
        assert!(MAX_RESULTS_PER_FETCH > 0);
        assert!(MAX_RESULTS_PER_FETCH <= 1000);
    }

    #[test]
    fn test_max_events_per_user_is_100() {
        assert_eq!(MAX_EVENTS_PER_USER, 100);
    }

    #[test]
    fn test_max_seen_tx_ids_per_user_is_1000() {
        assert_eq!(MAX_SEEN_TX_IDS_PER_USER, 1000);
    }
}
