use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

/// An ICRC-1 compatible account (owner + optional subaccount).
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IcrcAccount {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}

impl IcrcAccount {
    pub fn new(owner: Principal) -> Self {
        IcrcAccount {
            owner,
            subaccount: None,
        }
    }

    pub fn with_subaccount(owner: Principal, subaccount: [u8; 32]) -> Self {
        IcrcAccount {
            owner,
            subaccount: Some(subaccount),
        }
    }
}

/// A normalised ICRC transaction record as returned from index canisters.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct IcrcTransaction {
    pub id: u64,
    pub timestamp: u64,
    pub amount: u64,
    pub from: IcrcAccount,
    pub to: IcrcAccount,
    pub memo: Option<Vec<u8>>,
    pub kind: String, // "transfer", "mint", "burn", etc.
}

/// Request type for `get_account_transactions` on ICRC index canisters.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetTransactionsRequest {
    pub account: IcrcAccount,
    pub start: Option<u64>,
    pub max_results: u64,
}

/// Response type for `get_account_transactions` on ICRC index canisters.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetTransactionsResponse {
    pub transactions: Vec<IcrcTransaction>,
    pub oldest_tx_id: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icrc_account_new() {
        let p = Principal::anonymous();
        let acc = IcrcAccount::new(p);
        assert_eq!(acc.owner, p);
        assert!(acc.subaccount.is_none());
    }

    #[test]
    fn test_icrc_account_with_subaccount() {
        let p = Principal::from_slice(&[1u8; 29]);
        let sub = [42u8; 32];
        let acc = IcrcAccount::with_subaccount(p, sub);
        assert_eq!(acc.subaccount, Some(sub));
    }

    #[test]
    fn test_icrc_account_equality() {
        let p = Principal::anonymous();
        let a1 = IcrcAccount::new(p);
        let a2 = IcrcAccount::new(p);
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_icrc_transaction_fields() {
        let p = Principal::anonymous();
        let tx = IcrcTransaction {
            id: 1001,
            timestamp: 1_700_000_000_000_000_000,
            amount: 50_000_000,
            from: IcrcAccount::new(p),
            to: IcrcAccount::new(p),
            memo: Some(vec![0, 1, 2]),
            kind: "transfer".to_string(),
        };
        assert_eq!(tx.id, 1001);
        assert_eq!(tx.kind, "transfer");
        assert_eq!(tx.memo, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_icrc_transaction_no_memo() {
        let p = Principal::anonymous();
        let tx = IcrcTransaction {
            id: 2,
            timestamp: 0,
            amount: 100,
            from: IcrcAccount::new(p),
            to: IcrcAccount::new(p),
            memo: None,
            kind: "mint".to_string(),
        };
        assert!(tx.memo.is_none());
        assert_eq!(tx.kind, "mint");
    }

    #[test]
    fn test_get_transactions_request() {
        let acc = IcrcAccount::new(Principal::anonymous());
        let req = GetTransactionsRequest {
            account: acc.clone(),
            start: Some(100),
            max_results: 50,
        };
        assert_eq!(req.start, Some(100));
        assert_eq!(req.max_results, 50);
        assert_eq!(req.account, acc);
    }

    #[test]
    fn test_get_transactions_response_empty() {
        let resp = GetTransactionsResponse {
            transactions: vec![],
            oldest_tx_id: None,
        };
        assert!(resp.transactions.is_empty());
        assert!(resp.oldest_tx_id.is_none());
    }

    #[test]
    fn test_get_transactions_response_with_data() {
        let p = Principal::anonymous();
        let tx = IcrcTransaction {
            id: 5,
            timestamp: 999,
            amount: 1,
            from: IcrcAccount::new(p),
            to: IcrcAccount::new(p),
            memo: None,
            kind: "burn".to_string(),
        };
        let resp = GetTransactionsResponse {
            transactions: vec![tx],
            oldest_tx_id: Some(5),
        };
        assert_eq!(resp.transactions.len(), 1);
        assert_eq!(resp.oldest_tx_id, Some(5));
    }
}
