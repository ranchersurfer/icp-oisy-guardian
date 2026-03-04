use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

/// An ICRC-1 compatible account (owner + optional subaccount).
/// subaccount uses Vec<u8> for wire-format compatibility (candid `blob` type).
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IcrcAccount {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
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
            subaccount: Some(subaccount.to_vec()),
        }
    }
}

/// A normalised ICRC transaction record — our INTERNAL representation.
/// Converted from wire types (IcrcTransactionWithIdWire or IcpTransactionWithId).
/// `amount` is u128 to support ckETH (18 decimals, max ~10^21 Wei).
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct IcrcTransaction {
    pub id: u64,
    pub timestamp: u64,
    pub amount: u128,
    pub from: IcrcAccount,
    pub to: IcrcAccount,
    pub memo: Option<Vec<u8>>,
    pub kind: String, // "transfer", "mint", "burn", etc.
}

// ---------------------------------------------------------------------------
// Wire types: GetTransactionsRequest (same for all index canisters)
// ---------------------------------------------------------------------------

/// FIXED: `start` is `opt nat` and `max_results` is `nat` on-chain (not u64).
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetTransactionsRequest {
    pub account: IcrcAccount,
    /// The txid of the last transaction seen (exclusive start for pagination).
    /// `opt nat` on the wire — use None for most-recent.
    pub start: Option<Nat>,
    /// Maximum number of transactions to fetch. `nat` on the wire.
    pub max_results: Nat,
}

// ---------------------------------------------------------------------------
// Wire types: ckBTC/ckETH Index (ICRC-1 Index NG format)
// These match the deployed Candid for n5wcd-faaaa-aaaar-qaaea-cai (ckBTC)
// and s3zol-vqaaa-aaaar-qacpa-cai (ckETH).
// ---------------------------------------------------------------------------

/// `Tokens = nat` in the ckBTC/ckETH Index Candid.
pub type IcrcNat = Nat;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcTransferWire {
    pub from: IcrcAccount,
    pub to: IcrcAccount,
    pub amount: IcrcNat,
    pub fee: Option<IcrcNat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub spender: Option<IcrcAccount>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcMintWire {
    pub to: IcrcAccount,
    pub amount: IcrcNat,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub fee: Option<IcrcNat>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcBurnWire {
    pub from: IcrcAccount,
    pub amount: IcrcNat,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub spender: Option<IcrcAccount>,
    pub fee: Option<IcrcNat>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcApproveWire {
    pub from: IcrcAccount,
    pub spender: IcrcAccount,
    pub amount: IcrcNat,
    pub fee: Option<IcrcNat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub expected_allowance: Option<IcrcNat>,
    pub expires_at: Option<u64>,
}

/// Transaction body as returned by ckBTC/ckETH index canisters.
/// Note: `timestamp` is nanoseconds since epoch (nat64).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcTransactionBodyWire {
    pub kind: String,
    pub timestamp: u64,
    pub transfer: Option<IcrcTransferWire>,
    pub mint: Option<IcrcMintWire>,
    pub burn: Option<IcrcBurnWire>,
    pub approve: Option<IcrcApproveWire>,
}

/// `id: BlockIndex = nat` in the ckBTC/ckETH Candid.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcTransactionWithIdWire {
    pub id: Nat,
    pub transaction: IcrcTransactionBodyWire,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcGetTransactionsOk {
    pub balance: IcrcNat,
    pub transactions: Vec<IcrcTransactionWithIdWire>,
    /// `opt BlockIndex = opt nat`
    pub oldest_tx_id: Option<Nat>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcrcGetTransactionsErr {
    pub message: String,
}

/// Full response from ckBTC/ckETH `get_account_transactions`.
/// Candid: `variant { Ok : GetTransactions; Err : GetTransactionsErr }`
pub type IcrcGetTransactionsResult = Result<IcrcGetTransactionsOk, IcrcGetTransactionsErr>;

// ---------------------------------------------------------------------------
// Wire types: ICP Index (qhbym-qaaaa-aaaaa-aaafq-cai)
// Different structure: uses text AccountIdentifiers and Operation variant.
// ---------------------------------------------------------------------------

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpTokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpTimeStamp {
    pub timestamp_nanos: u64,
}

/// ICP Index Operation variant — `from`/`to` are text AccountIdentifiers (hex).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum IcpOperation {
    Transfer {
        to: String,
        fee: IcpTokens,
        from: String,
        amount: IcpTokens,
        spender: Option<String>,
    },
    Mint {
        to: String,
        amount: IcpTokens,
    },
    Burn {
        from: String,
        amount: IcpTokens,
        spender: Option<String>,
    },
    Approve {
        fee: IcpTokens,
        from: String,
        allowance: IcpTokens,
        expires_at: Option<IcpTimeStamp>,
        spender: String,
        expected_allowance: Option<IcpTokens>,
    },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpTransactionWire {
    pub memo: u64,
    pub icrc1_memo: Option<Vec<u8>>,
    pub operation: IcpOperation,
    pub created_at_time: Option<IcpTimeStamp>,
    pub timestamp: Option<IcpTimeStamp>,
}

/// ICP index uses `id: nat64` (not nat).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpTransactionWithId {
    pub id: u64,
    pub transaction: IcpTransactionWire,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpGetTransactionsResponse {
    pub balance: u64,
    pub transactions: Vec<IcpTransactionWithId>,
    pub oldest_tx_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IcpGetTransactionsError {
    pub message: String,
}

/// Full response from ICP `get_account_transactions`.
/// Candid: `variant { Ok : GetAccountIdentifierTransactionsResponse; Err : ... }`
pub type IcpGetTransactionsResult = Result<IcpGetTransactionsResponse, IcpGetTransactionsError>;

// ---------------------------------------------------------------------------
// Conversion: wire types → internal IcrcTransaction
// ---------------------------------------------------------------------------

/// Convert a ckBTC/ckETH wire transaction to our internal `IcrcTransaction`.
/// Returns None for approve/unknown kinds where from/to can't be cleanly extracted.
pub fn icrc_wire_to_internal(wire: &IcrcTransactionWithIdWire) -> Option<IcrcTransaction> {
    let id: u64 = wire.id.0.clone().try_into().unwrap_or(u64::MAX);
    let tx = &wire.transaction;
    let timestamp = tx.timestamp;
    let kind = tx.kind.clone();

    if let Some(transfer) = &tx.transfer {
        let amount_u128: u128 = transfer.amount.0.clone().try_into().unwrap_or(u128::MAX);
        return Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount_u128,
            from: transfer.from.clone(),
            to: transfer.to.clone(),
            memo: transfer.memo.clone(),
            kind,
        });
    }

    if let Some(mint) = &tx.mint {
        let amount_u128: u128 = mint.amount.0.clone().try_into().unwrap_or(u128::MAX);
        let anon = IcrcAccount::new(Principal::anonymous());
        return Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount_u128,
            from: anon.clone(),
            to: mint.to.clone(),
            memo: mint.memo.clone(),
            kind,
        });
    }

    if let Some(burn) = &tx.burn {
        let amount_u128: u128 = burn.amount.0.clone().try_into().unwrap_or(u128::MAX);
        let anon = IcrcAccount::new(Principal::anonymous());
        return Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount_u128,
            from: burn.from.clone(),
            to: anon,
            memo: burn.memo.clone(),
            kind,
        });
    }

    None // approve or unknown — skip
}

/// Convert an ICP index wire transaction to our internal `IcrcTransaction`.
/// NOTE: ICP operations use text AccountIdentifiers (hex), not Principals.
/// We try Principal::from_text() as a best-effort; fall back to anonymous().
/// This is a known limitation — ICP AccountIdentifiers are not directly
/// convertible to Principals without the ICP ledger's account-to-principal map.
pub fn icp_wire_to_internal(wire: &IcpTransactionWithId) -> Option<IcrcTransaction> {
    let id = wire.id;
    let tx = &wire.transaction;

    // Use created_at_time first, then timestamp, then 0
    let timestamp = tx.created_at_time
        .as_ref()
        .map(|t| t.timestamp_nanos)
        .or_else(|| tx.timestamp.as_ref().map(|t| t.timestamp_nanos))
        .unwrap_or(0);

    let anon = || IcrcAccount::new(Principal::anonymous());

    // Best-effort: try to parse text as a principal (works for raw principal text form)
    let try_principal = |text: &str| -> IcrcAccount {
        Principal::from_text(text)
            .map(IcrcAccount::new)
            .unwrap_or_else(|_| anon())
    };

    match &tx.operation {
        IcpOperation::Transfer { to, from, amount, .. } => Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount.e8s as u128,
            from: try_principal(from),
            to: try_principal(to),
            memo: None,
            kind: "transfer".to_string(),
        }),
        IcpOperation::Mint { to, amount } => Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount.e8s as u128,
            from: anon(),
            to: try_principal(to),
            memo: None,
            kind: "mint".to_string(),
        }),
        IcpOperation::Burn { from, amount, .. } => Some(IcrcTransaction {
            id,
            timestamp,
            amount: amount.e8s as u128,
            from: try_principal(from),
            to: anon(),
            memo: None,
            kind: "burn".to_string(),
        }),
        IcpOperation::Approve { .. } => None, // Skip approve operations
    }
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
        assert_eq!(acc.subaccount, Some(sub.to_vec()));
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
    fn test_icrc_transaction_amount_u128() {
        // Verify that amount can hold ckETH scale values (1000 ETH = 10^21 Wei)
        let p = Principal::anonymous();
        let one_thousand_eth: u128 = 1_000 * 10u128.pow(18);
        let tx = IcrcTransaction {
            id: 1,
            timestamp: 0,
            amount: one_thousand_eth,
            from: IcrcAccount::new(p),
            to: IcrcAccount::new(p),
            memo: None,
            kind: "transfer".to_string(),
        };
        assert_eq!(tx.amount, 1_000_000_000_000_000_000_000u128);
        // This would have overflowed u64 (max ~1.8 * 10^19)
        assert!(u64::MAX < one_thousand_eth as u64 || one_thousand_eth > u64::MAX as u128);
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
    fn test_get_transactions_request_uses_nat() {
        let acc = IcrcAccount::new(Principal::anonymous());
        let req = GetTransactionsRequest {
            account: acc.clone(),
            start: Some(Nat::from(100u64)),
            max_results: Nat::from(50u64),
        };
        assert_eq!(req.start, Some(Nat::from(100u64)));
        assert_eq!(req.max_results, Nat::from(50u64));
    }

    #[test]
    fn test_get_transactions_request_no_start() {
        let acc = IcrcAccount::new(Principal::anonymous());
        let req = GetTransactionsRequest {
            account: acc,
            start: None,
            max_results: Nat::from(100u64),
        };
        assert!(req.start.is_none());
    }

    #[test]
    fn test_icrc_wire_to_internal_transfer() {
        let from = IcrcAccount::new(Principal::from_slice(&[1u8; 29]));
        let to = IcrcAccount::new(Principal::from_slice(&[2u8; 29]));
        let wire = IcrcTransactionWithIdWire {
            id: Nat::from(42u64),
            transaction: IcrcTransactionBodyWire {
                kind: "transfer".to_string(),
                timestamp: 999_000,
                transfer: Some(IcrcTransferWire {
                    from: from.clone(),
                    to: to.clone(),
                    amount: Nat::from(1_000_000u64),
                    fee: None,
                    memo: None,
                    created_at_time: None,
                    spender: None,
                }),
                mint: None,
                burn: None,
                approve: None,
            },
        };
        let internal = icrc_wire_to_internal(&wire).unwrap();
        assert_eq!(internal.id, 42);
        assert_eq!(internal.amount, 1_000_000u128);
        assert_eq!(internal.from.owner, from.owner);
        assert_eq!(internal.to.owner, to.owner);
        assert_eq!(internal.kind, "transfer");
    }

    #[test]
    fn test_icrc_wire_to_internal_mint() {
        let to = IcrcAccount::new(Principal::from_slice(&[3u8; 29]));
        let wire = IcrcTransactionWithIdWire {
            id: Nat::from(10u64),
            transaction: IcrcTransactionBodyWire {
                kind: "mint".to_string(),
                timestamp: 100,
                transfer: None,
                mint: Some(IcrcMintWire {
                    to: to.clone(),
                    amount: Nat::from(500u64),
                    memo: None,
                    created_at_time: None,
                    fee: None,
                }),
                burn: None,
                approve: None,
            },
        };
        let internal = icrc_wire_to_internal(&wire).unwrap();
        assert_eq!(internal.kind, "mint");
        assert_eq!(internal.amount, 500u128);
        assert_eq!(internal.to.owner, to.owner);
        assert_eq!(internal.from.owner, Principal::anonymous());
    }

    #[test]
    fn test_icrc_wire_to_internal_cketh_large_amount() {
        // 1000 ETH in Wei — would overflow u64
        let one_thousand_eth = 1_000u128 * 10u128.pow(18);
        let from = IcrcAccount::new(Principal::from_slice(&[1u8; 29]));
        let to = IcrcAccount::new(Principal::from_slice(&[2u8; 29]));
        let wire = IcrcTransactionWithIdWire {
            id: Nat::from(1u64),
            transaction: IcrcTransactionBodyWire {
                kind: "transfer".to_string(),
                timestamp: 1_000,
                transfer: Some(IcrcTransferWire {
                    from: from.clone(),
                    to: to.clone(),
                    amount: Nat::from(one_thousand_eth),
                    fee: None,
                    memo: None,
                    created_at_time: None,
                    spender: None,
                }),
                mint: None,
                burn: None,
                approve: None,
            },
        };
        let internal = icrc_wire_to_internal(&wire).unwrap();
        assert_eq!(internal.amount, one_thousand_eth);
        // Confirm this exceeds u64 max
        assert!(internal.amount > u64::MAX as u128);
    }

    #[test]
    fn test_icrc_wire_to_internal_approve_returns_none() {
        let account = IcrcAccount::new(Principal::anonymous());
        let wire = IcrcTransactionWithIdWire {
            id: Nat::from(5u64),
            transaction: IcrcTransactionBodyWire {
                kind: "approve".to_string(),
                timestamp: 100,
                transfer: None,
                mint: None,
                burn: None,
                approve: Some(IcrcApproveWire {
                    from: account.clone(),
                    spender: account.clone(),
                    amount: Nat::from(1000u64),
                    fee: None,
                    memo: None,
                    created_at_time: None,
                    expected_allowance: None,
                    expires_at: None,
                }),
            },
        };
        assert!(icrc_wire_to_internal(&wire).is_none());
    }

    #[test]
    fn test_icrc_account_serialization_roundtrip() {
        let sub = vec![0xABu8; 32];
        let acc = IcrcAccount {
            owner: Principal::from_slice(&[7u8; 29]),
            subaccount: Some(sub.clone()),
        };
        let encoded = candid::encode_one(&acc).expect("encode");
        let decoded: IcrcAccount = candid::decode_one(&encoded).expect("decode");
        assert_eq!(decoded, acc);
    }
}
