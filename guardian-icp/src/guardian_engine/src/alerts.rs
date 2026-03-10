/// alerts.rs — Alert payload formatting and stable storage for Guardian.
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::detector::{DetectionResult, Severity};
use crate::{AlertRecord, AlertStatus, UnifiedEvent, ALERTS};

// ---------------------------------------------------------------------------
// Alert payload / safe consumer view
// ---------------------------------------------------------------------------

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AlertPayload {
    pub alert_id: String,
    pub timestamp: u64,
    pub user: Principal,
    pub severity: String,
    pub severity_score: u8,
    pub rules_triggered: Vec<String>,
    pub events_summary: String,
    pub recommended_action: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConsumerAlertRecord {
    pub alert_id: String,
    pub timestamp: u64,
    pub severity: String,
    pub severity_score: u8,
    pub rules_triggered: Vec<String>,
    pub events_summary: String,
    pub recommended_action: String,
}

// ---------------------------------------------------------------------------
// format_alert
// ---------------------------------------------------------------------------

/// Build an `AlertPayload` from a detection result and store it in stable memory.
pub fn format_alert(
    user: Principal,
    result: DetectionResult,
    events: Vec<UnifiedEvent>,
    timestamp: u64,
) -> AlertPayload {
    // Derive a uuid-style alert id from timestamp + last 4 bytes of principal
    let p_bytes = user.as_slice();
    let p_suffix = if p_bytes.len() >= 4 {
        u32::from_be_bytes([
            p_bytes[p_bytes.len() - 4],
            p_bytes[p_bytes.len() - 3],
            p_bytes[p_bytes.len() - 2],
            p_bytes[p_bytes.len() - 1],
        ])
    } else {
        0u32
    };
    let alert_id = format!("alert-{:x}-{:08x}", timestamp / 1_000_000_000, p_suffix);

    let rules_triggered: Vec<String> = result
        .rules_triggered
        .iter()
        .map(|r| format!("{}: {}", r.rule_id, r.description))
        .collect();

    // Build a short events summary
    let events_summary = if events.is_empty() {
        "No events".to_string()
    } else {
        format!(
            "{} event(s); latest tx_id={}",
            events.len(),
            events.last().map(|e| e.tx_id.as_str()).unwrap_or("unknown")
        )
    };

    let recommended_action = match result.severity {
        Severity::Emergency => "IMMEDIATE ACTION: Pause all transactions, contact support".to_string(),
        Severity::Critical => "Review and confirm recent large transactions".to_string(),
        Severity::Warn => "Monitor account for continued suspicious activity".to_string(),
        Severity::Info => "Informational: new destination address detected".to_string(),
    };

    let payload = AlertPayload {
        alert_id: alert_id.clone(),
        timestamp,
        user,
        severity: result.severity.as_str().to_string(),
        severity_score: result.score,
        rules_triggered: rules_triggered.clone(),
        events_summary,
        recommended_action,
    };

    // Store in stable ALERTS map
    let record = AlertRecord {
        alert_id: alert_id.clone(),
        timestamp,
        user,
        rules_triggered,
        severity: result.score,
        status: AlertStatus::Pending,
        events_summary: payload.events_summary.clone(),
        recommended_action: payload.recommended_action.clone(),
    };

    ALERTS.with(|a| {
        a.borrow_mut().insert(alert_id, record);
    });

    payload
}
