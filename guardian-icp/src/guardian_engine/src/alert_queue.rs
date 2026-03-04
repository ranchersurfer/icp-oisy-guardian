//! alert_queue.rs — Persistent alert delivery queue for Guardian Engine.
//!
//! HTTPS outcalls will be implemented in Phase 2b.
//! Queue is populated by detector, drained by a separate 60s timer.

use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableBTreeMap, Storable};
use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;

use crate::Memory;

/// MemoryId for the alert queue stable map (must not conflict with lib.rs IDs 0–4).
const ALERT_QUEUE_MEM_ID: MemoryId = MemoryId::new(5);

// ---------------------------------------------------------------------------
// AlertQueueItem
// ---------------------------------------------------------------------------

/// A queued alert item awaiting HTTPS delivery.
///
/// Phase 2b: contains all fields needed to build delivery payloads for
/// Discord / Slack / webhook / email without re-fetching alert data.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AlertQueueItem {
    /// Unique alert identifier (matches AlertRecord.alert_id).
    pub alert_id: String,
    /// The user principal that triggered the alert.
    pub user: Principal,
    /// Severity label: "INFO", "WARN", "CRITICAL", "EMERGENCY".
    pub severity: String,
    /// Numeric severity score (0–255).
    pub severity_score: u8,
    /// List of rule IDs and descriptions that fired.
    pub rules_triggered: Vec<String>,
    /// Short human-readable summary of triggering events.
    pub events_summary: String,
    /// Recommended remediation action for this alert.
    pub recommended_action: String,
    /// Number of delivery attempts so far.
    pub retry_count: u32,
    /// Timestamp (nanoseconds) when the item was enqueued.
    pub created_at: u64,
}

impl Storable for AlertQueueItem {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes)
            .expect("AlertQueueItem::from_bytes: failed to decode — stable memory may be corrupt")
    }
    const BOUND: Bound = Bound::Unbounded;
}

// ---------------------------------------------------------------------------
// Stable storage
// ---------------------------------------------------------------------------

thread_local! {
    /// Stable alert queue: alert_id → AlertQueueItem (MemoryId 5).
    /// Populated by the detector; drained by a future 60s delivery timer.
    static ALERT_QUEUE: RefCell<StableBTreeMap<String, AlertQueueItem, Memory>> =
        RefCell::new(StableBTreeMap::new(
            crate::MEMORY_MANAGER.with(|mm| mm.borrow().get(ALERT_QUEUE_MEM_ID))
        ));
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Enqueue an alert item for later delivery.
pub fn enqueue_alert(item: AlertQueueItem) {
    ALERT_QUEUE.with(|q| {
        q.borrow_mut().insert(item.alert_id.clone(), item);
    });
}

/// Dequeue up to `max` alert items, removing them from the queue.
/// Items are returned in ascending alert_id order (BTreeMap iteration order).
pub fn dequeue_alerts(max: usize) -> Vec<AlertQueueItem> {
    ALERT_QUEUE.with(|q| {
        let mut queue = q.borrow_mut();
        let keys: Vec<String> = queue.iter().take(max).map(|(k, _)| k.clone()).collect();
        keys.into_iter()
            .filter_map(|k| queue.remove(&k))
            .collect()
    })
}

/// Return the current queue depth without removing items.
#[allow(dead_code)]
pub fn queue_len() -> u64 {
    ALERT_QUEUE.with(|q| q.borrow().len())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    fn make_item(id: &str, ts: u64) -> AlertQueueItem {
        AlertQueueItem {
            alert_id: id.to_string(),
            user: Principal::anonymous(),
            severity: "WARN".to_string(),
            severity_score: 3,
            rules_triggered: vec!["A3: rapid tx".to_string()],
            events_summary: "6 events in 10 min".to_string(),
            recommended_action: "Monitor activity".to_string(),
            retry_count: 0,
            created_at: ts,
        }
    }

    #[test]
    fn test_alert_queue_item_storable_roundtrip() {
        let item = make_item("alert-001", 999_000_000);
        let bytes = item.to_bytes();
        let item2 = AlertQueueItem::from_bytes(bytes);
        assert_eq!(item2.alert_id, "alert-001");
        assert_eq!(item2.retry_count, 0);
        assert_eq!(item2.created_at, 999_000_000);
    }

    #[test]
    fn test_alert_queue_item_fields() {
        let p = Principal::from_slice(&[1u8; 29]);
        let item = AlertQueueItem {
            alert_id: "alert-xyz".to_string(),
            user: p,
            severity: "CRITICAL".to_string(),
            severity_score: 7,
            rules_triggered: vec!["A1: large transfer".to_string()],
            events_summary: "1 large outgoing tx".to_string(),
            recommended_action: "Review recent transactions".to_string(),
            retry_count: 3,
            created_at: 1_700_000_000_000_000_000,
        };
        assert_eq!(item.alert_id, "alert-xyz");
        assert_eq!(item.user, p);
        assert_eq!(item.severity, "CRITICAL");
        assert_eq!(item.severity_score, 7);
        assert_eq!(item.rules_triggered.len(), 1);
        assert_eq!(item.retry_count, 3);
    }
}
