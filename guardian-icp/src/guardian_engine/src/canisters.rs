/// ICP Ledger Index canister (mainnet).
/// Provides ICRC-1 transaction history for the ICP token.
pub const ICP_INDEX_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_icp_index_canister_id_valid() {
        let result = Principal::from_text(ICP_INDEX_CANISTER_ID);
        assert!(result.is_ok(), "ICP_INDEX_CANISTER_ID should be a valid principal");
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
}
