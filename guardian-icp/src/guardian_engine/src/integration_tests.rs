/// integration_tests.rs — Phase 1e integration & scenario tests.
///
/// These tests exercise multi-component interactions in pure Rust (no IC runtime).
/// They simulate realistic end-to-end scenarios: detect → alert, data persistence, etc.
#[cfg(test)]
mod integration_tests {
    use crate::alerts::format_alert;
    use crate::detector::{
        evaluate, DetectionContext, DetectionResult, RuleMatch, Severity,
    };
    use crate::fetcher::{
        icrc_tx_to_unified_event, merge_into_ring_buffer, update_watermark_after_fetch,
    };
    use crate::icrc::{IcrcAccount, IcrcTransaction};
    use crate::{AlertRecord, AlertStatus, Chain, Direction, UnifiedEvent, Watermark, WatermarkKey};
    use candid::Principal;
    use ic_stable_structures::Storable;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_principal(seed: u8) -> Principal {
        Principal::from_slice(&[seed; 29])
    }

    fn make_out_event(ts: u64, amount: u64, counterparty: Principal) -> UnifiedEvent {
        UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: ts,
            direction: Direction::Out,
            amount_e8s: amount,
            counterparty,
            tx_id: format!("{}", ts / 1_000 + 1),
        }
    }

    fn make_in_event(ts: u64, amount: u64, sender: Principal) -> UnifiedEvent {
        UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: ts,
            direction: Direction::In,
            amount_e8s: amount,
            counterparty: sender,
            tx_id: format!("in-{}", ts),
        }
    }

    fn make_ctx<'a>(
        events: &'a [UnifiedEvent],
        balance: u64,
        allowlist: &'a [String],
        threshold: u8,
    ) -> DetectionContext<'a> {
        DetectionContext {
            events,
            estimated_balance_e8s: balance,
            allowlisted_addresses: allowlist,
            alert_threshold: threshold,
        }
    }

    fn default_icrc_tx(id: u64, amount: u64, from_seed: u8, to_seed: u8) -> IcrcTransaction {
        IcrcTransaction {
            id,
            timestamp: 1_700_000_000_000_000_000 + id * 1_000_000_000,
            amount,
            from: IcrcAccount {
                owner: make_principal(from_seed),
                subaccount: None,
            },
            to: IcrcAccount {
                owner: make_principal(to_seed),
                subaccount: None,
            },
            memo: None,
            kind: "transfer".to_string(),
        }
    }

    // -----------------------------------------------------------------------
    // 1. Full monitoring cycle: detect → alert
    // -----------------------------------------------------------------------

    #[test]
    fn test_full_cycle_no_alert_small_tx() {
        let unknown = make_principal(2);
        let events = vec![make_out_event(1000, 10, unknown)];
        let ctx = make_ctx(&events, 1000, &[], 7);
        let result = evaluate(&ctx);
        // A4 fires (weight 1) but threshold=7 → no alert
        assert!(!result.should_alert, "Score 1 < threshold 7 → no alert");
    }

    #[test]
    fn test_full_cycle_alert_large_tx() {
        let user = make_principal(1);
        let unknown = make_principal(99);
        let events = vec![make_out_event(1000, 600, unknown)];
        let ctx = make_ctx(&events, 1000, &[], 7);
        let result = evaluate(&ctx);
        assert!(result.should_alert);
        assert_eq!(result.severity, Severity::Critical);
        let payload = format_alert(user, result, events, 1_700_000_000_000_000_000);
        assert!(!payload.alert_id.is_empty());
        assert_eq!(payload.severity, "CRITICAL");
    }

    #[test]
    fn test_full_cycle_rapid_tx_alert() {
        let known = make_principal(3);
        let allowlist = vec![known.to_text()];
        let events: Vec<UnifiedEvent> = (0..6)
            .map(|i| make_out_event(i * 60_000_000_000, 1, known))
            .collect();
        let ctx = make_ctx(&events, 10_000, &allowlist, 3);
        let result = evaluate(&ctx);
        assert!(result.should_alert);
        assert_eq!(result.severity, Severity::Warn);
    }

    #[test]
    fn test_full_cycle_new_address_info_alert() {
        let unknown = make_principal(77);
        let events = vec![make_out_event(1000, 1, unknown)];
        let ctx = make_ctx(&events, 10_000, &[], 1);
        let result = evaluate(&ctx);
        assert!(result.should_alert);
        assert_eq!(result.severity, Severity::Info);
        assert!(result.rules_triggered.iter().any(|r| r.rule_id == "A4"));
    }

    #[test]
    fn test_full_cycle_combined_a1_a4_alert() {
        let unknown = make_principal(55);
        let events = vec![make_out_event(1000, 600, unknown)]; // A1 + A4
        let ctx = make_ctx(&events, 1000, &[], 7);
        let result = evaluate(&ctx);
        assert!(result.should_alert);
        assert!(result.rules_triggered.iter().any(|r| r.rule_id == "A1"));
        assert!(result.rules_triggered.iter().any(|r| r.rule_id == "A4"));
        assert_eq!(result.score, 8); // A1(7) + A4(1)
    }

    // -----------------------------------------------------------------------
    // 2. Config + Engine interaction
    // -----------------------------------------------------------------------

    #[test]
    fn test_config_alert_threshold_gates_out_a4() {
        let unknown = make_principal(50);
        let events = vec![make_out_event(1000, 1, unknown)]; // A4 fires, weight=1
        let ctx = make_ctx(&events, 100, &[], 7); // threshold=7 > score=1
        let result = evaluate(&ctx);
        assert!(!result.should_alert, "Score 1 < threshold 7 → no alert");
    }

    #[test]
    fn test_config_threshold_exactly_met_alerts() {
        let unknown = make_principal(50);
        let events = vec![make_out_event(1000, 1, unknown)]; // A4, score=1
        let ctx = make_ctx(&events, 100, &[], 1); // threshold=1
        let result = evaluate(&ctx);
        assert!(result.should_alert, "Score == threshold → alert");
    }

    #[test]
    fn test_config_allowlisted_addr_suppresses_a4() {
        let known = make_principal(5);
        let allowlist = vec![known.to_text()];
        let events = vec![make_out_event(1000, 1, known)];
        let ctx = make_ctx(&events, 10_000, &allowlist, 1);
        let result = evaluate(&ctx);
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A4"),
            "Allowlisted address should not trigger A4");
    }

    #[test]
    fn test_config_multiple_allowlisted_addrs() {
        let k1 = make_principal(1);
        let k2 = make_principal(2);
        let allowlist = vec![k1.to_text(), k2.to_text()];
        let events = vec![
            make_out_event(1000, 1, k1),
            make_out_event(2000, 1, k2),
        ];
        let ctx = make_ctx(&events, 10_000, &allowlist, 1);
        let result = evaluate(&ctx);
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A4"),
            "All known addrs → no A4");
    }

    // -----------------------------------------------------------------------
    // 3. Upgrade safety: data structure roundtrips
    // -----------------------------------------------------------------------

    #[test]
    fn test_watermark_roundtrip_across_upgrade() {
        let wm = Watermark {
            last_tx_id: "tx-abc-123".to_string(),
            last_checked: 1_700_000_000_000_000_000,
            block_height: 12345,
        };
        let bytes = wm.to_bytes();
        let restored = Watermark::from_bytes(bytes);
        assert_eq!(restored.last_tx_id, "tx-abc-123");
        assert_eq!(restored.last_checked, 1_700_000_000_000_000_000);
        assert_eq!(restored.block_height, 12345);
    }

    #[test]
    fn test_alert_record_roundtrip_across_upgrade() {
        let record = AlertRecord {
            alert_id: "alert-upgrade-test".to_string(),
            timestamp: 9999999,
            user: make_principal(8),
            rules_triggered: vec!["A1".to_string(), "A3".to_string()],
            severity: 7,
            status: AlertStatus::Pending,
        };
        let bytes = record.to_bytes();
        let restored = AlertRecord::from_bytes(bytes);
        assert_eq!(restored.alert_id, "alert-upgrade-test");
        assert_eq!(restored.rules_triggered.len(), 2);
        assert_eq!(restored.status, AlertStatus::Pending);
    }

    #[test]
    fn test_watermark_key_roundtrip_across_upgrade() {
        let key = WatermarkKey::new(&make_principal(9), &Chain::CkBTC);
        let bytes = key.to_bytes();
        let restored = WatermarkKey::from_bytes(bytes);
        assert_eq!(restored.0, key.0);
    }

    #[test]
    fn test_watermark_roundtrip_empty_tx_id() {
        let wm = Watermark {
            last_tx_id: "".to_string(),
            last_checked: 0,
            block_height: 0,
        };
        let restored = Watermark::from_bytes(wm.to_bytes());
        assert_eq!(restored.last_tx_id, "");
        assert_eq!(restored.block_height, 0);
    }

    #[test]
    fn test_alert_record_sent_status_roundtrip() {
        let record = AlertRecord {
            alert_id: "sent-test".to_string(),
            timestamp: 42,
            user: make_principal(1),
            rules_triggered: vec![],
            severity: 3,
            status: AlertStatus::Sent,
        };
        let restored = AlertRecord::from_bytes(record.to_bytes());
        assert_eq!(restored.status, AlertStatus::Sent);
    }

    #[test]
    fn test_alert_record_failed_status_roundtrip() {
        let record = AlertRecord {
            alert_id: "fail-test".to_string(),
            timestamp: 42,
            user: make_principal(1),
            rules_triggered: vec!["A1".to_string()],
            severity: 7,
            status: AlertStatus::Failed,
        };
        let restored = AlertRecord::from_bytes(record.to_bytes());
        assert_eq!(restored.status, AlertStatus::Failed);
    }

    // -----------------------------------------------------------------------
    // 4. Rate limit enforcement simulation
    // -----------------------------------------------------------------------

    #[test]
    fn test_rate_limit_10_updates_at_limit() {
        let now: u64 = 1_700_000_000_000_000_000;
        let hour: u64 = 3600 * 1_000_000_000;
        let recent: Vec<u64> = (0..10).map(|i| now - i * 60_000_000_000).collect();
        let in_window: Vec<&u64> = recent.iter().filter(|&&t| now - t < hour).collect();
        assert_eq!(in_window.len(), 10);
        assert!(in_window.len() <= 10, "10 should be at or under limit");
    }

    #[test]
    fn test_rate_limit_11th_update_exceeds() {
        let now: u64 = 1_700_000_000_000_000_000;
        let hour: u64 = 3600 * 1_000_000_000;
        let recent: Vec<u64> = (0..11).map(|i| now - i * 60_000_000_000).collect();
        let in_window: Vec<&u64> = recent.iter().filter(|&&t| now - t < hour).collect();
        assert!(in_window.len() > 10, "11 in window → exceeds limit");
    }

    #[test]
    fn test_rate_limit_old_timestamps_expire() {
        let now: u64 = 1_700_000_000_000_000_000;
        let hour: u64 = 3600 * 1_000_000_000;
        let recent: Vec<u64> = (0..5).map(|i| now - i * 60_000_000_000).collect();
        let old: Vec<u64> = (1..=5).map(|i| now - hour - i * 60_000_000_000).collect();
        let all: Vec<u64> = recent.iter().chain(old.iter()).copied().collect();
        let in_window: Vec<&u64> = all.iter().filter(|&&t| now - t < hour).collect();
        assert_eq!(in_window.len(), 5, "Old timestamps should expire");
    }

    #[test]
    fn test_rate_limit_all_old_timestamps_zero_in_window() {
        let now: u64 = 1_700_000_000_000_000_000;
        let hour: u64 = 3600 * 1_000_000_000;
        let old: Vec<u64> = (1..=10).map(|i| now - hour - i * 60_000_000_000).collect();
        let in_window: Vec<&u64> = old.iter().filter(|&&t| now - t < hour).collect();
        assert_eq!(in_window.len(), 0, "All old → none in window");
    }

    // -----------------------------------------------------------------------
    // 5. ICRC transaction conversion
    // -----------------------------------------------------------------------

    #[test]
    fn test_icrc_tx_converts_to_outgoing_unified_event() {
        let user = make_principal(1);
        let tx = default_icrc_tx(42, 1_000_000, 1, 2);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(event.direction, Direction::Out);
        assert_eq!(event.amount_e8s, 1_000_000);
        assert_eq!(event.chain, "ICP");
        assert_eq!(event.tx_id, "42");
    }

    #[test]
    fn test_icrc_tx_converts_to_incoming_unified_event() {
        let user = make_principal(2); // user is the receiver
        let tx = default_icrc_tx(43, 500_000, 1, 2);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(event.direction, Direction::In);
        assert_eq!(event.amount_e8s, 500_000);
    }

    #[test]
    fn test_icrc_tx_ckbtc_chain_label() {
        let user = make_principal(5);
        let tx = default_icrc_tx(100, 999, 5, 6);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ckBTC");
        assert_eq!(event.chain, "ckBTC");
    }

    #[test]
    fn test_icrc_tx_cketh_chain_label() {
        let user = make_principal(5);
        let tx = default_icrc_tx(101, 100, 5, 6);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ckETH");
        assert_eq!(event.chain, "ckETH");
    }

    #[test]
    fn test_icrc_tx_zero_amount() {
        let user = make_principal(1);
        let tx = default_icrc_tx(200, 0, 1, 2);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(event.amount_e8s, 0);
    }

    #[test]
    fn test_icrc_tx_max_amount() {
        let user = make_principal(1);
        let tx = default_icrc_tx(300, u64::MAX, 1, 2);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(event.amount_e8s, u64::MAX);
    }

    #[test]
    fn test_icrc_tx_id_preserved() {
        let user = make_principal(1);
        let tx = default_icrc_tx(9999, 100, 1, 2);
        let account = IcrcAccount { owner: user, subaccount: None };
        let event = icrc_tx_to_unified_event(&tx, &account, "ICP");
        assert_eq!(event.tx_id, "9999");
    }

    // -----------------------------------------------------------------------
    // 6. Ring buffer merge behavior
    // -----------------------------------------------------------------------

    #[test]
    fn test_ring_buffer_append_events() {
        let mut existing: Vec<UnifiedEvent> = (0..5)
            .map(|i| make_out_event(i * 1000, 100, make_principal(20)))
            .collect();
        let new_events = vec![make_out_event(9999, 200, make_principal(21))];
        merge_into_ring_buffer(&mut existing, new_events, 100);
        assert_eq!(existing.len(), 6);
    }

    #[test]
    fn test_ring_buffer_trim_to_max() {
        let mut existing: Vec<UnifiedEvent> = (0..95)
            .map(|i| make_out_event(i * 1000, 100, make_principal(20)))
            .collect();
        let new_events: Vec<UnifiedEvent> = (0..10)
            .map(|i| make_out_event(100_000 + i * 1000, 200, make_principal(21)))
            .collect();
        merge_into_ring_buffer(&mut existing, new_events, 100);
        assert_eq!(existing.len(), 100, "Should trim to max 100");
    }

    #[test]
    fn test_ring_buffer_empty_new_events() {
        let mut existing: Vec<UnifiedEvent> = (0..5)
            .map(|i| make_out_event(i * 1000, 100, make_principal(20)))
            .collect();
        merge_into_ring_buffer(&mut existing, vec![], 100);
        assert_eq!(existing.len(), 5, "No change when no new events");
    }

    #[test]
    fn test_ring_buffer_empty_existing() {
        let mut existing: Vec<UnifiedEvent> = vec![];
        let new_events = vec![make_out_event(1000, 100, make_principal(20))];
        merge_into_ring_buffer(&mut existing, new_events, 100);
        assert_eq!(existing.len(), 1);
    }

    #[test]
    fn test_ring_buffer_exactly_at_max_no_trim() {
        let mut existing: Vec<UnifiedEvent> = (0..99)
            .map(|i| make_out_event(i * 1000, 100, make_principal(20)))
            .collect();
        let new_events = vec![make_out_event(100_000, 100, make_principal(21))];
        merge_into_ring_buffer(&mut existing, new_events, 100);
        assert_eq!(existing.len(), 100);
    }

    #[test]
    fn test_ring_buffer_max_1_keeps_newest() {
        let mut existing: Vec<UnifiedEvent> = vec![make_out_event(100, 1, make_principal(1))];
        let new_events = vec![make_out_event(200, 2, make_principal(2))];
        merge_into_ring_buffer(&mut existing, new_events, 1);
        assert_eq!(existing.len(), 1);
        assert_eq!(existing[0].timestamp, 200, "Should keep newest");
    }

    // -----------------------------------------------------------------------
    // 7. Watermark advance logic
    // -----------------------------------------------------------------------

    #[test]
    fn test_watermark_advances_with_higher_tx_id() {
        let mut wm = Watermark {
            last_tx_id: "50".to_string(),
            last_checked: 1000,
            block_height: 50,
        };
        let events = vec![make_out_event(2000, 100, make_principal(1))]; // tx_id = "3" (2000/1000+1)
        // Use a tx with id > 50
        let mut higher_events = vec![UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: 2000,
            direction: Direction::Out,
            amount_e8s: 100,
            counterparty: make_principal(1),
            tx_id: "100".to_string(),
        }];
        update_watermark_after_fetch(&mut wm, &higher_events, 9999);
        assert_eq!(wm.block_height, 100);
        assert_eq!(wm.last_tx_id, "100");
        assert_eq!(wm.last_checked, 9999);
    }

    #[test]
    fn test_watermark_does_not_regress_with_lower_tx_id() {
        let mut wm = Watermark {
            last_tx_id: "100".to_string(),
            last_checked: 1000,
            block_height: 100,
        };
        let old_events = vec![UnifiedEvent {
            chain: "ICP".to_string(),
            timestamp: 500,
            direction: Direction::Out,
            amount_e8s: 50,
            counterparty: make_principal(1),
            tx_id: "50".to_string(),
        }];
        update_watermark_after_fetch(&mut wm, &old_events, 5000);
        assert_eq!(wm.block_height, 100, "Should not regress");
        assert_eq!(wm.last_checked, 5000, "Should update last_checked");
    }

    #[test]
    fn test_watermark_updates_last_checked_even_with_no_new_events() {
        let mut wm = Watermark {
            last_tx_id: "100".to_string(),
            last_checked: 1000,
            block_height: 100,
        };
        update_watermark_after_fetch(&mut wm, &[], 9999);
        assert_eq!(wm.last_checked, 9999);
        assert_eq!(wm.block_height, 100, "Block height unchanged");
    }

    // -----------------------------------------------------------------------
    // 8. Alert payload structure and integrity
    // -----------------------------------------------------------------------

    #[test]
    fn test_alert_id_unique_different_principals() {
        let p1 = make_principal(1);
        let p2 = make_principal(2);
        let result1 = DetectionResult {
            score: 7,
            severity: Severity::Critical,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 7 }],
            should_alert: true,
        };
        let result2 = result1.clone();
        let a1 = format_alert(p1, result1, vec![], 1000);
        let a2 = format_alert(p2, result2, vec![], 1000);
        assert_ne!(a1.alert_id, a2.alert_id, "Different principals → different IDs");
    }

    #[test]
    fn test_alert_id_unique_different_timestamps() {
        let p = make_principal(1);
        let result1 = DetectionResult {
            score: 7,
            severity: Severity::Critical,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 7 }],
            should_alert: true,
        };
        let result2 = result1.clone();
        // alert_id uses timestamp / 1_000_000_000, so need 1s+ apart
        let t1 = 1_700_000_000_000_000_000u64;
        let t2 = 1_700_000_001_000_000_000u64;
        let a1 = format_alert(p, result1, vec![], t1);
        let a2 = format_alert(p, result2, vec![], t2);
        assert_ne!(a1.alert_id, a2.alert_id, "Different timestamps → different IDs");
    }

    #[test]
    fn test_alert_emergency_severity_label() {
        let p = make_principal(1);
        let result = DetectionResult {
            score: 15,
            severity: Severity::Emergency,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 15 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, vec![], 1000);
        assert_eq!(payload.severity, "EMERGENCY");
        assert!(!payload.recommended_action.is_empty());
    }

    #[test]
    fn test_alert_warn_severity_label() {
        let p = make_principal(1);
        let result = DetectionResult {
            score: 3,
            severity: Severity::Warn,
            rules_triggered: vec![RuleMatch { rule_id: "A3".into(), description: "x".into(), weight: 3 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, vec![], 1000);
        assert_eq!(payload.severity, "WARN");
        assert!(!payload.recommended_action.is_empty());
    }

    #[test]
    fn test_alert_info_severity_label() {
        let p = make_principal(1);
        let result = DetectionResult {
            score: 1,
            severity: Severity::Info,
            rules_triggered: vec![RuleMatch { rule_id: "A4".into(), description: "x".into(), weight: 1 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, vec![], 1000);
        assert_eq!(payload.severity, "INFO");
    }

    #[test]
    fn test_alert_events_summary_multiple_events() {
        let p = make_principal(1);
        let events = vec![
            make_out_event(1000, 500, make_principal(2)),
            make_out_event(2000, 300, make_principal(3)),
        ];
        let result = DetectionResult {
            score: 7,
            severity: Severity::Critical,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 7 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, events, 3000);
        assert_ne!(payload.events_summary, "No events");
    }

    #[test]
    fn test_alert_no_events_summary_string() {
        let p = make_principal(1);
        let result = DetectionResult {
            score: 7,
            severity: Severity::Critical,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 7 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, vec![], 1000);
        assert_eq!(payload.events_summary, "No events");
    }

    #[test]
    fn test_alert_user_field_matches_principal() {
        let p = make_principal(42);
        let result = DetectionResult {
            score: 7,
            severity: Severity::Critical,
            rules_triggered: vec![RuleMatch { rule_id: "A1".into(), description: "x".into(), weight: 7 }],
            should_alert: true,
        };
        let payload = format_alert(p, result, vec![], 1000);
        assert_eq!(payload.user, p);
    }

    // -----------------------------------------------------------------------
    // 9. Security: input validation scenarios
    // -----------------------------------------------------------------------

    #[test]
    fn test_no_alert_for_incoming_only_events() {
        let sender = make_principal(99);
        let events: Vec<UnifiedEvent> = (0..10)
            .map(|i| make_in_event(i * 60_000_000_000, 1_000_000, sender))
            .collect();
        let ctx = make_ctx(&events, 1000, &[], 1);
        let result = evaluate(&ctx);
        // A1, A3, A4 only check outgoing
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A1"));
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A3"));
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A4"));
    }

    #[test]
    fn test_empty_events_no_alert() {
        let ctx = make_ctx(&[], 1000, &[], 1);
        let result = evaluate(&ctx);
        assert!(!result.should_alert);
        assert_eq!(result.score, 0);
    }

    #[test]
    fn test_zero_balance_no_a1_trigger() {
        let unknown = make_principal(99);
        let events = vec![make_out_event(1000, 100, unknown)];
        let ctx = make_ctx(&events, 0, &[], 7);
        let result = evaluate(&ctx);
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A1"),
            "Zero balance → A1 should not fire");
    }

    #[test]
    fn test_a1_exactly_50_pct_no_trigger() {
        let unknown = make_principal(99);
        let events = vec![make_out_event(1000, 50, unknown)];
        let ctx = make_ctx(&events, 100, &[], 7);
        let result = evaluate(&ctx);
        assert!(!result.rules_triggered.iter().any(|r| r.rule_id == "A1"),
            "Exactly 50% should not trigger A1");
    }

    #[test]
    fn test_a1_51_pct_triggers() {
        let unknown = make_principal(99);
        let events = vec![make_out_event(1000, 51, unknown)];
        let ctx = make_ctx(&events, 100, &[], 7);
        let result = evaluate(&ctx);
        assert!(result.rules_triggered.iter().any(|r| r.rule_id == "A1"),
            "51% should trigger A1");
    }

    // -----------------------------------------------------------------------
    // 10. Cycle cost monitoring
    // -----------------------------------------------------------------------

    #[test]
    fn test_cycle_guard_constant_is_500b() {
        assert_eq!(crate::MIN_CYCLE_BALANCE_PUB, 500_000_000_000u64);
    }

    #[test]
    fn test_cycle_guard_constant_is_less_than_1t() {
        assert!(crate::MIN_CYCLE_BALANCE_PUB < 1_000_000_000_000u64,
            "Guard should be below 1T cycles");
    }

    // -----------------------------------------------------------------------
    // 11. Multi-user isolation
    // -----------------------------------------------------------------------

    #[test]
    fn test_different_users_independent_detection() {
        let unknown = make_principal(99);
        // User 1: large tx → alert
        let events1 = vec![make_out_event(1000, 600, unknown)];
        let ctx1 = make_ctx(&events1, 1000, &[], 7);
        let result1 = evaluate(&ctx1);
        assert!(result1.should_alert);

        // User 2: small tx → score < threshold
        let events2 = vec![make_out_event(1000, 10, unknown)];
        let ctx2 = make_ctx(&events2, 1000, &[], 7);
        let result2 = evaluate(&ctx2);
        assert!(!result2.should_alert, "User 2 small tx → no alert");
    }

    #[test]
    fn test_watermark_keys_unique_per_chain() {
        let user = make_principal(1);
        let key_icp = WatermarkKey::new(&user, &Chain::ICP);
        let key_btc = WatermarkKey::new(&user, &Chain::CkBTC);
        let key_eth = WatermarkKey::new(&user, &Chain::CkETH);
        assert_ne!(key_icp.0, key_btc.0);
        assert_ne!(key_btc.0, key_eth.0);
        assert_ne!(key_icp.0, key_eth.0);
    }

    #[test]
    fn test_watermark_keys_unique_per_user() {
        let u1 = make_principal(1);
        let u2 = make_principal(2);
        let k1 = WatermarkKey::new(&u1, &Chain::ICP);
        let k2 = WatermarkKey::new(&u2, &Chain::ICP);
        assert_ne!(k1.0, k2.0);
    }

    #[test]
    fn test_watermark_key_same_user_same_chain_equal() {
        let user = make_principal(7);
        let k1 = WatermarkKey::new(&user, &Chain::ICP);
        let k2 = WatermarkKey::new(&user, &Chain::ICP);
        assert_eq!(k1.0, k2.0);
    }

    // -----------------------------------------------------------------------
    // 12. Scoring and severity
    // -----------------------------------------------------------------------

    #[test]
    fn test_a1_a3_combination_score_is_10() {
        let known = make_principal(99);
        let allowlist = vec![known.to_text()];
        let mut events: Vec<UnifiedEvent> = (0..6)
            .map(|i| make_out_event(i * 60_000_000_000, 1, known))
            .collect();
        events[0].amount_e8s = 600; // trigger A1 (60% of 1000)
        let ctx = make_ctx(&events, 1000, &allowlist, 7);
        let result = evaluate(&ctx);
        assert_eq!(result.score, 10); // A1(7) + A3(3)
        assert!(result.should_alert);
    }

    #[test]
    fn test_severity_mapping_emergency_at_15() {
        assert_eq!(Severity::from_score(15), Severity::Emergency);
        assert_eq!(Severity::from_score(100), Severity::Emergency);
    }

    #[test]
    fn test_severity_mapping_critical_7_to_14() {
        assert_eq!(Severity::from_score(7), Severity::Critical);
        assert_eq!(Severity::from_score(14), Severity::Critical);
    }

    #[test]
    fn test_severity_mapping_warn_3_to_6() {
        assert_eq!(Severity::from_score(3), Severity::Warn);
        assert_eq!(Severity::from_score(6), Severity::Warn);
    }

    #[test]
    fn test_severity_mapping_info_1_to_2() {
        assert_eq!(Severity::from_score(1), Severity::Info);
        assert_eq!(Severity::from_score(2), Severity::Info);
    }

    #[test]
    fn test_severity_mapping_score_0_is_info() {
        // Edge: score=0 → Info (default)
        assert_eq!(Severity::from_score(0), Severity::Info);
    }

    // -----------------------------------------------------------------------
    // 13. Canister ID validation
    // -----------------------------------------------------------------------

    #[test]
    fn test_canister_ids_are_valid_principals() {
        use crate::canisters::{CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID, ICP_INDEX_CANISTER_ID};
        let _ = Principal::from_text(ICP_INDEX_CANISTER_ID).expect("ICP index canister ID valid");
        let _ = Principal::from_text(CKBTC_INDEX_CANISTER_ID).expect("ckBTC index canister ID valid");
        let _ = Principal::from_text(CKETH_INDEX_CANISTER_ID).expect("ckETH index canister ID valid");
    }

    #[test]
    fn test_canister_ids_are_different() {
        use crate::canisters::{CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID, ICP_INDEX_CANISTER_ID};
        assert_ne!(ICP_INDEX_CANISTER_ID, CKBTC_INDEX_CANISTER_ID);
        assert_ne!(CKBTC_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID);
        assert_ne!(ICP_INDEX_CANISTER_ID, CKETH_INDEX_CANISTER_ID);
    }
}
