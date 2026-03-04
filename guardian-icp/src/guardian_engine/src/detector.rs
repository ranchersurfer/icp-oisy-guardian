/// detector.rs — Rule evaluation engine for Guardian.
///
/// Rules:
///   A1: Large outgoing transfer  (weight 7 = CRITICAL)
///   A3: Rapid successive txs     (weight 3 = WARN)
///   A4: New destination address  (weight 1 = INFO)
use crate::{Direction, UnifiedEvent};
use candid::{CandidType, Deserialize};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Severity
// ---------------------------------------------------------------------------

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Severity {
    Info,      // score 1–2
    Warn,      // score 3–6
    Critical,  // score 7–14
    Emergency, // score 15+
}

impl Severity {
    pub fn from_score(score: u8) -> Self {
        if score >= 15 {
            Severity::Emergency
        } else if score >= 7 {
            Severity::Critical
        } else if score >= 3 {
            Severity::Warn
        } else {
            Severity::Info
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Severity::Info => 1,
            Severity::Warn => 3,
            Severity::Critical => 7,
            Severity::Emergency => 15,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::Critical => "CRITICAL",
            Severity::Emergency => "EMERGENCY",
        }
    }
}

// ---------------------------------------------------------------------------
// RuleMatch
// ---------------------------------------------------------------------------

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RuleMatch {
    pub rule_id: String,
    pub description: String,
    pub weight: u8,
}

// ---------------------------------------------------------------------------
// DetectionResult
// ---------------------------------------------------------------------------

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DetectionResult {
    pub score: u8,
    pub severity: Severity,
    pub rules_triggered: Vec<RuleMatch>,
    pub should_alert: bool,
}

// ---------------------------------------------------------------------------
// Rule A1 — Large outgoing transfer
// ---------------------------------------------------------------------------

/// Triggered when any single outgoing transfer > 50% of estimated balance.
/// `estimated_balance_e8s`: estimated total balance (sum of all amounts seen).
/// Returns Some(RuleMatch) if triggered.
pub fn rule_a1_large_transfer(
    events: &[UnifiedEvent],
    estimated_balance_e8s: u64,
) -> Option<RuleMatch> {
    if estimated_balance_e8s == 0 {
        return None;
    }
    let threshold = estimated_balance_e8s / 2; // 50% — integer division gives floor
    for ev in events {
        if ev.direction == Direction::Out && ev.amount_e8s > threshold {
            return Some(RuleMatch {
                rule_id: "A1".to_string(),
                description: format!(
                    "Large outgoing transfer: {} e8s (>{} e8s threshold, balance {})",
                    ev.amount_e8s, threshold, estimated_balance_e8s
                ),
                weight: 7,
            });
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Rule A3 — Rapid successive transactions
// ---------------------------------------------------------------------------

/// Triggered when the same principal sent >5 outgoing txs within any 10-minute window.
/// `now_ns`: current time in nanoseconds (used to define the window end).
/// Window = last 10 minutes from the latest tx timestamp in `events`.
pub fn rule_a3_rapid_transactions(events: &[UnifiedEvent]) -> Option<RuleMatch> {
    // Collect outgoing event timestamps
    let mut out_timestamps: Vec<u64> = events
        .iter()
        .filter(|e| e.direction == Direction::Out)
        .map(|e| e.timestamp)
        .collect();

    if out_timestamps.len() <= 5 {
        return None;
    }

    out_timestamps.sort_unstable();

    let window_ns: u64 = 10 * 60 * 1_000_000_000; // 10 minutes in nanoseconds

    // Sliding window: count how many fall within any 10-minute span
    for i in 0..out_timestamps.len() {
        let window_end = out_timestamps[i] + window_ns;
        let count = out_timestamps
            .iter()
            .filter(|&&t| t >= out_timestamps[i] && t <= window_end)
            .count();
        if count > 5 {
            return Some(RuleMatch {
                rule_id: "A3".to_string(),
                description: format!(
                    "Rapid successive transactions: {} outgoing txs within 10 minutes",
                    count
                ),
                weight: 3,
            });
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Rule A4 — New destination address
// ---------------------------------------------------------------------------

/// Triggered when any outgoing tx goes to an address not in `allowlisted_addresses`.
/// `allowlisted_addresses`: list of principal text representations.
pub fn rule_a4_new_address(
    events: &[UnifiedEvent],
    allowlisted_addresses: &[String],
) -> Option<RuleMatch> {
    for ev in events {
        if ev.direction == Direction::Out {
            let addr_text = ev.counterparty.to_text();
            if !allowlisted_addresses.contains(&addr_text) {
                return Some(RuleMatch {
                    rule_id: "A4".to_string(),
                    description: format!(
                        "New destination address: {} not in allowlist",
                        addr_text
                    ),
                    weight: 1,
                });
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// A2 — Known scam address matching (planned Phase 3: requires on-chain scam address registry)
// Skipped in Phase 1 MVP. Rules are numbered per OISY_GUARDIAN_SPEC section 6.
// ---------------------------------------------------------------------------

/// Stub for A2 — returns None until Phase 3 on-chain scam registry is implemented.
#[allow(dead_code)]
pub fn rule_a2_known_scam_address(_events: &[UnifiedEvent]) -> Option<RuleMatch> {
    // A2 — Known scam address matching (planned Phase 3: requires on-chain scam address registry)
    // Skipped in Phase 1 MVP. Rules are numbered per OISY_GUARDIAN_SPEC section 6.
    None
}

// ---------------------------------------------------------------------------
// Evaluate all rules → DetectionResult
// ---------------------------------------------------------------------------

pub struct DetectionContext<'a> {
    pub events: &'a [UnifiedEvent],
    pub estimated_balance_e8s: u64,
    /// Actual balance from icrc1_balance_of, if the call succeeded.
    /// When present, used instead of estimated_balance_e8s for A1 evaluation.
    pub balance_e8s: Option<u64>,
    pub allowlisted_addresses: &'a [String],
    pub alert_threshold: u8,
}

pub fn evaluate(ctx: &DetectionContext) -> DetectionResult {
    let mut rules_triggered: Vec<RuleMatch> = Vec::new();

    // Use actual balance when available, fall back to tx-history estimate.
    let effective_balance = ctx.balance_e8s.unwrap_or(ctx.estimated_balance_e8s);

    if let Some(m) = rule_a1_large_transfer(ctx.events, effective_balance) {
        rules_triggered.push(m);
    }
    if let Some(m) = rule_a3_rapid_transactions(ctx.events) {
        rules_triggered.push(m);
    }
    if let Some(m) = rule_a4_new_address(ctx.events, ctx.allowlisted_addresses) {
        rules_triggered.push(m);
    }

    // A2 stub is intentionally not called here (Phase 3 placeholder).
    let _ = rule_a2_known_scam_address(ctx.events);

    let score: u8 = rules_triggered
        .iter()
        .map(|r| r.weight)
        .fold(0u8, |acc, w| acc.saturating_add(w));

    let severity = Severity::from_score(score);
    let should_alert = score >= ctx.alert_threshold;

    DetectionResult {
        score,
        severity,
        rules_triggered,
        should_alert,
    }
}
