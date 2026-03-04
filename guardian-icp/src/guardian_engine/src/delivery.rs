//! delivery.rs — Phase 2b/2d: Alert Delivery via HTTPS Outcall
//!
//! Delivers queued alerts to configured channels using IC HTTPS outcalls.
//! Supports Discord webhooks, Slack webhooks, generic webhooks, and email
//! (via HTTP-based mail APIs such as Mailgun or SendGrid).
//!
//! # Cycle Cost Estimation
//! Each HTTPS outcall costs roughly:
//!   - Base fee:  ~49_000_000 cycles (fixed per call)
//!   - Request:   ~400 cycles / byte of request body
//!   - Response:  ~800 cycles / byte of response body (transforms reduce this)
//!
//! A typical alert webhook (1 KB request, 200-byte response) costs ~49_560_000 cycles.
//! The `CYCLES_PER_OUTCALL` constant is 100_000_000 per attempt (2x safety margin).
//!
//! # Retry Policy
//! - Max 3 attempts per alert
//! - Backoff: 0s → 30s → 90s (not enforced in timer; retry happens on next 60s drain tick)
//! - Permanent failure after MAX_RETRIES → item marked Failed in ALERTS map

use candid::{CandidType, Deserialize};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum number of delivery attempts before giving up.
pub const MAX_RETRIES: u32 = 3;

/// Approximate cycle cost budget per HTTPS outcall attempt.
/// Base fee (~49M) + 1KB request (~400K) + 200B response (~160K) + 2× margin.
pub const CYCLES_PER_OUTCALL: u128 = 100_000_000; // 100M cycles

/// Maximum response body bytes to read (to limit cycle cost).
pub const MAX_RESPONSE_BYTES: u64 = 2_048;

/// Connection timeout hint for the IC HTTP gateway.
pub const OUTCALL_TIMEOUT_SECS: u64 = 30;

// ---------------------------------------------------------------------------
// AlertChannel — mirrors the Candid spec (Section 6.1)
// ---------------------------------------------------------------------------

/// Alert delivery channel configuration.
///
/// In the current config canister the channels are stored as free-form strings
/// for simplicity. This enum provides the typed representation used by the
/// delivery engine. Parse from strings using [`AlertChannel::from_str_config`].
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum AlertChannel {
    /// Discord incoming webhook: POST JSON to `webhook_url`.
    Discord { webhook_url: String },

    /// Slack incoming webhook: POST JSON to `webhook_url`.
    Slack { webhook_url: String },

    /// Generic HTTPS webhook: POST JSON to `url` with optional HMAC-SHA256 `secret`.
    Webhook { url: String, secret: Option<String> },

    /// Email delivery via HTTP mail API (Mailgun / SendGrid compatible).
    Email { address: String, api_url: String, api_key: String },
}

impl AlertChannel {
    /// Parse a channel descriptor string into a typed [`AlertChannel`].
    ///
    /// Expected formats (URL-encoded key=value pairs separated by `;`):
    /// - `discord;url=https://discord.com/api/webhooks/…`
    /// - `slack;url=https://hooks.slack.com/services/…`
    /// - `webhook;url=https://example.com/hook;secret=mysecret`
    /// - `email;address=user@example.com;api_url=https://api.mailgun.net/v3/…;api_key=key-…`
    ///
    /// Returns `None` for unrecognised or malformed descriptors.
    pub fn from_str_config(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.splitn(2, ';').collect();
        if parts.is_empty() {
            return None;
        }
        let kind = parts[0].trim().to_lowercase();
        let params = if parts.len() > 1 { parts[1] } else { "" };

        // Parse key=value pairs from the remainder.
        let kv: std::collections::HashMap<&str, &str> = params
            .split(';')
            .filter_map(|p| {
                let mut it = p.splitn(2, '=');
                let k = it.next()?.trim();
                let v = it.next()?.trim();
                Some((k, v))
            })
            .collect();

        match kind.as_str() {
            "discord" => {
                let url = kv.get("url")?.to_string();
                Some(AlertChannel::Discord { webhook_url: url })
            }
            "slack" => {
                let url = kv.get("url")?.to_string();
                Some(AlertChannel::Slack { webhook_url: url })
            }
            "webhook" => {
                let url = kv.get("url")?.to_string();
                let secret = kv.get("secret").map(|s| s.to_string());
                Some(AlertChannel::Webhook { url, secret })
            }
            "email" => {
                let address = kv.get("address")?.to_string();
                let api_url = kv.get("api_url")?.to_string();
                let api_key = kv.get("api_key")?.to_string();
                Some(AlertChannel::Email { address, api_url, api_key })
            }
            _ => None,
        }
    }

    /// Return a short human-readable channel type label.
    pub fn kind_label(&self) -> &'static str {
        match self {
            AlertChannel::Discord { .. } => "discord",
            AlertChannel::Slack { .. } => "slack",
            AlertChannel::Webhook { .. } => "webhook",
            AlertChannel::Email { .. } => "email",
        }
    }
}

// ---------------------------------------------------------------------------
// DeliveryResult
// ---------------------------------------------------------------------------

/// Outcome of a single delivery attempt.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum DeliveryOutcome {
    /// HTTP 2xx received — delivery confirmed.
    Success { status: u16 },
    /// HTTP error (4xx/5xx) — may be permanent or transient.
    HttpError { status: u16, body: String },
    /// Transport/network error — always transient, retry.
    TransportError { message: String },
    /// Cycle budget insufficient — canister critically low.
    InsufficientCycles,
    /// Channel config is invalid — permanent failure, skip retries.
    InvalidConfig { reason: String },
}

impl DeliveryOutcome {
    /// Returns `true` if this outcome is a permanent failure (no point retrying).
    pub fn is_permanent_failure(&self) -> bool {
        matches!(
            self,
            DeliveryOutcome::HttpError { status, .. } if (400..500).contains(status)
        ) || matches!(self, DeliveryOutcome::InvalidConfig { .. })
        || matches!(self, DeliveryOutcome::InsufficientCycles)
    }

    /// Returns `true` if delivery succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, DeliveryOutcome::Success { .. })
    }
}

// ---------------------------------------------------------------------------
// Alert message builders
// ---------------------------------------------------------------------------

/// Build a Discord webhook JSON payload for an alert.
///
/// Uses the Discord "embeds" format for rich formatting.
pub fn build_discord_payload(
    alert_id: &str,
    severity: &str,
    severity_score: u8,
    rules: &[String],
    events_summary: &str,
    recommended_action: &str,
) -> String {
    let color = match severity.to_uppercase().as_str() {
        "EMERGENCY" => 16711680u32, // red
        "CRITICAL"  => 16744272u32, // orange-red
        "WARN"      => 16776960u32, // yellow
        _           => 3447003u32,  // blue (INFO)
    };

    let rules_str = rules.join(", ");
    format!(
        r#"{{"embeds":[{{"title":"🚨 Guardian Alert [{severity}]","color":{color},"fields":[{{"name":"Alert ID","value":"{alert_id}","inline":true}},{{"name":"Severity Score","value":"{severity_score}","inline":true}},{{"name":"Rules Triggered","value":"{rules_str}"}},{{"name":"Events","value":"{events_summary}"}},{{"name":"Recommended Action","value":"{recommended_action}"}}]}}]}}"#,
        severity = escape_json(severity),
        color = color,
        alert_id = escape_json(alert_id),
        severity_score = severity_score,
        rules_str = escape_json(&rules_str),
        events_summary = escape_json(events_summary),
        recommended_action = escape_json(recommended_action),
    )
}

/// Build a Slack webhook JSON payload (Block Kit) for an alert.
pub fn build_slack_payload(
    alert_id: &str,
    severity: &str,
    severity_score: u8,
    rules: &[String],
    events_summary: &str,
    recommended_action: &str,
) -> String {
    let emoji = match severity.to_uppercase().as_str() {
        "EMERGENCY" => "🆘",
        "CRITICAL"  => "🚨",
        "WARN"      => "⚠️",
        _           => "ℹ️",
    };

    let rules_str = rules.join(", ");
    format!(
        r#"{{"text":"{emoji} *Guardian Alert [{severity}]*\nAlert ID: `{alert_id}` | Score: {severity_score}\nRules: {rules_str}\nEvents: {events_summary}\nAction: {recommended_action}"}}"#,
        emoji = emoji,
        severity = escape_json(severity),
        alert_id = escape_json(alert_id),
        severity_score = severity_score,
        rules_str = escape_json(&rules_str),
        events_summary = escape_json(events_summary),
        recommended_action = escape_json(recommended_action),
    )
}

/// Build a generic webhook JSON payload for an alert.
pub fn build_webhook_payload(
    alert_id: &str,
    severity: &str,
    severity_score: u8,
    rules: &[String],
    events_summary: &str,
    recommended_action: &str,
) -> String {
    let rules_json = rules
        .iter()
        .map(|r| format!("\"{}\"", escape_json(r)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"{{"alert_id":"{alert_id}","severity":"{severity}","severity_score":{severity_score},"rules_triggered":[{rules_json}],"events_summary":"{events_summary}","recommended_action":"{recommended_action}"}}"#,
        alert_id = escape_json(alert_id),
        severity = escape_json(severity),
        severity_score = severity_score,
        rules_json = rules_json,
        events_summary = escape_json(events_summary),
        recommended_action = escape_json(recommended_action),
    )
}

/// Build an email API payload (Mailgun/SendGrid-style form body).
pub fn build_email_payload(
    address: &str,
    alert_id: &str,
    severity: &str,
    severity_score: u8,
    rules: &[String],
    events_summary: &str,
    recommended_action: &str,
) -> String {
    let rules_str = rules.join("; ");
    let subject = format!("Guardian Alert: {} (score {})", severity, severity_score);
    let text = format!(
        "Alert ID: {}\nSeverity: {} (score {})\nRules: {}\nEvents: {}\nRecommended Action: {}",
        alert_id, severity, severity_score, rules_str, events_summary, recommended_action
    );
    // Mailgun form-encoded body format
    format!(
        "to={}&subject={}&text={}",
        url_encode(address),
        url_encode(&subject),
        url_encode(&text),
    )
}

// ---------------------------------------------------------------------------
// Cycle cost estimator
// ---------------------------------------------------------------------------

/// Estimate cycle cost for a single HTTPS outcall given request and expected response sizes.
///
/// Formula from IC documentation:
///   base = 49_140_000 (per-call base)
///   request_bytes fee = 5_200 cycles/byte
///   response_bytes fee = 10_400 cycles/byte
///   (Approximate; actual costs vary with subnet size)
pub fn estimate_outcall_cycles(request_bytes: usize, max_response_bytes: u64) -> u128 {
    let base: u128 = 49_140_000;
    let req_fee: u128 = request_bytes as u128 * 5_200;
    let resp_fee: u128 = max_response_bytes as u128 * 10_400;
    base + req_fee + resp_fee
}

// ---------------------------------------------------------------------------
// HTTPS outcall delivery (non-test only — requires IC runtime)
// ---------------------------------------------------------------------------

#[cfg(not(test))]
pub use outcall::deliver_to_channel;

#[cfg(not(test))]
mod outcall {
    use super::*;
    use ic_cdk::api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
        TransformFunc,
    };

    /// Deliver an alert to a single channel via HTTPS outcall.
    /// Returns [`DeliveryOutcome`] describing the result.
    pub async fn deliver_to_channel(
        channel: &AlertChannel,
        alert_id: &str,
        severity: &str,
        severity_score: u8,
        rules: &[String],
        events_summary: &str,
        recommended_action: &str,
    ) -> DeliveryOutcome {
        let cycle_balance = ic_cdk::api::canister_balance128();
        if cycle_balance < CYCLES_PER_OUTCALL * 2 {
            return DeliveryOutcome::InsufficientCycles;
        }

        let (url, body, content_type, extra_headers) = match channel {
            AlertChannel::Discord { webhook_url } => {
                let body = build_discord_payload(
                    alert_id, severity, severity_score, rules, events_summary, recommended_action,
                );
                (webhook_url.clone(), body, "application/json", vec![])
            }
            AlertChannel::Slack { webhook_url } => {
                let body = build_slack_payload(
                    alert_id, severity, severity_score, rules, events_summary, recommended_action,
                );
                (webhook_url.clone(), body, "application/json", vec![])
            }
            AlertChannel::Webhook { url, secret } => {
                let body = build_webhook_payload(
                    alert_id, severity, severity_score, rules, events_summary, recommended_action,
                );
                let mut extra = vec![];
                if let Some(sec) = secret {
                    // HMAC-SHA256 signing — Phase 2d
                    // Header format: X-Guardian-Signature: sha256=<hex_digest>
                    let sig = build_webhook_signature(sec, body.as_bytes());
                    extra.push(HttpHeader {
                        name: "X-Guardian-Signature".to_string(),
                        value: sig,
                    });
                }
                (url.clone(), body, "application/json", extra)
            }
            AlertChannel::Email { address, api_url, api_key } => {
                let body = build_email_payload(
                    address,
                    alert_id,
                    severity,
                    severity_score,
                    rules,
                    events_summary,
                    recommended_action,
                );
                let auth_header = HttpHeader {
                    name: "Authorization".to_string(),
                    value: format!("Basic {}", base64_encode(api_key.as_bytes())),
                };
                (api_url.clone(), body, "application/x-www-form-urlencoded", vec![auth_header])
            }
        };

        // Validate URL (basic — IC will reject non-HTTPS anyway)
        if !url.starts_with("https://") {
            return DeliveryOutcome::InvalidConfig {
                reason: format!("URL must use HTTPS; got: {}", &url[..url.len().min(40)]),
            };
        }

        let request_bytes = body.len();
        let estimated_cycles = estimate_outcall_cycles(request_bytes, MAX_RESPONSE_BYTES);
        ic_cdk::println!(
            "[delivery] {} alert_id={} url={} body_bytes={} estimated_cycles={}",
            channel.kind_label(), alert_id, &url[..url.len().min(60)], request_bytes, estimated_cycles,
        );

        let mut headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: content_type.to_string(),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "Guardian-ICP/0.2b".to_string(),
            },
        ];
        headers.extend(extra_headers);

        let request = CanisterHttpRequestArgument {
            url,
            method: HttpMethod::POST,
            body: Some(body.into_bytes()),
            max_response_bytes: Some(MAX_RESPONSE_BYTES),
            headers,
            transform: Some(TransformContext {
                function: TransformFunc(candid::Func {
                    principal: ic_cdk::id(),
                    method: "transform_response".to_string(),
                }),
                context: vec![],
            }),
        };

        match http_request(request, estimated_cycles).await {
            Ok((response,)) => {
                // Convert candid::Nat (BigUint) to u16 via string parsing
                let status = response.status.0.to_string().parse::<u16>().unwrap_or(0);
                let body_str = String::from_utf8_lossy(&response.body).to_string();
                ic_cdk::println!(
                    "[delivery] {} result: status={} body={}",
                    channel.kind_label(), status, &body_str[..body_str.len().min(200)]
                );
                if (200..300).contains(&status) {
                    DeliveryOutcome::Success { status }
                } else {
                    DeliveryOutcome::HttpError {
                        status,
                        body: body_str[..body_str.len().min(512)].to_string(),
                    }
                }
            }
            Err((code, msg)) => {
                ic_cdk::println!(
                    "[delivery] transport error: code={:?} msg={}",
                    code, msg
                );
                DeliveryOutcome::TransportError {
                    message: format!("{:?}: {}", code, &msg[..msg.len().min(200)]),
                }
            }
        }
    }

    /// Minimal base64 encoder (no padding for auth headers is fine for Mailgun).
    fn base64_encode(input: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in input.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let n = (b0 << 16) | (b1 << 8) | b2;
            out.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
            out.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
            out.push(if chunk.len() > 1 { CHARS[((n >> 6) & 0x3F) as usize] as char } else { '=' });
            out.push(if chunk.len() > 2 { CHARS[(n & 0x3F) as usize] as char } else { '=' });
        }
        out
    }
}

/// Transform callback registered with the IC HTTP gateway.
/// Strips non-deterministic headers so all subnet nodes agree on response.
#[cfg(not(test))]
#[ic_cdk::query]
fn transform_response(args: ic_cdk::api::management_canister::http_request::TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    // Keep only the body; drop all response headers (non-deterministic)
    ic_cdk::api::management_canister::http_request::HttpResponse {
        status: args.response.status,
        headers: vec![],
        body: args.response.body,
    }
}

// ---------------------------------------------------------------------------
// Delivery runner — called from the 60s timer tick in lib.rs
// ---------------------------------------------------------------------------

/// Drain up to `max_items` from the alert queue and attempt delivery.
/// Returns a summary: (delivered, retried, failed_permanently).
///
/// In test builds this is a no-op (HTTPS outcalls require IC runtime).
#[cfg(not(test))]
pub async fn run_delivery_drain(
    max_items: usize,
    channels: &[AlertChannel],
) -> (u32, u32, u32) {
    use crate::alert_queue::{dequeue_alerts, enqueue_alert};
    use crate::ALERTS;
    use crate::AlertStatus;

    if channels.is_empty() {
        return (0, 0, 0);
    }

    let items = dequeue_alerts(max_items);
    let mut delivered = 0u32;
    let mut retried = 0u32;
    let mut failed_perm = 0u32;

    for mut item in items {
        let mut any_success = false;
        let mut perm_fail = false;

        for channel in channels {
            let outcome = deliver_to_channel(
                channel,
                &item.alert_id,
                &item.severity,
                item.severity_score,
                &item.rules_triggered,
                &item.events_summary,
                &item.recommended_action,
            ).await;

            if outcome.is_success() {
                any_success = true;
                ic_cdk::println!(
                    "[delivery] ✓ alert {} delivered via {}",
                    item.alert_id, channel.kind_label()
                );
            } else if outcome.is_permanent_failure() {
                perm_fail = true;
                ic_cdk::println!(
                    "[delivery] ✗ permanent failure for alert {} via {}: {:?}",
                    item.alert_id, channel.kind_label(), outcome
                );
            } else {
                ic_cdk::println!(
                    "[delivery] ~ transient failure for alert {} via {}: {:?}",
                    item.alert_id, channel.kind_label(), outcome
                );
            }
        }

        if any_success {
            delivered += 1;
            // Mark alert as Sent in stable ALERTS map
            ALERTS.with(|a| {
                if let Some(mut rec) = a.borrow().get(&item.alert_id) {
                    rec.status = AlertStatus::Sent;
                    a.borrow_mut().insert(item.alert_id.clone(), rec);
                }
            });
        } else if perm_fail || item.retry_count + 1 >= MAX_RETRIES {
            failed_perm += 1;
            // Mark alert as Failed in stable ALERTS map
            ALERTS.with(|a| {
                if let Some(mut rec) = a.borrow().get(&item.alert_id) {
                    rec.status = AlertStatus::Failed;
                    a.borrow_mut().insert(item.alert_id.clone(), rec);
                }
            });
            ic_cdk::println!(
                "[delivery] ✗ alert {} permanently failed after {} attempts",
                item.alert_id, item.retry_count + 1
            );
        } else {
            // Transient failure — re-enqueue with incremented retry count
            item.retry_count += 1;
            ic_cdk::println!(
                "[delivery] ↩ alert {} retry {}/{} scheduled",
                item.alert_id, item.retry_count, MAX_RETRIES
            );
            enqueue_alert(item);
            retried += 1;
        }
    }

    (delivered, retried, failed_perm)
}

/// Test stub: no-op delivery drain.
#[cfg(test)]
pub async fn run_delivery_drain(
    _max_items: usize,
    _channels: &[AlertChannel],
) -> (u32, u32, u32) {
    (0, 0, 0)
}

// ---------------------------------------------------------------------------
// Helper: escape JSON special characters
// ---------------------------------------------------------------------------

/// Escape a string for safe embedding in a JSON string value.
pub fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c    => out.push(c),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// HMAC-SHA256 signing (Phase 2d)
// ---------------------------------------------------------------------------

/// Compute HMAC-SHA256 of `payload` using `secret` as the key.
/// Returns the signature as a lowercase hex string.
///
/// This is used to sign webhook payloads so receivers can verify authenticity.
/// The canonical header is: `X-Guardian-Signature: sha256=<hex_digest>`
///
/// Example verification (receiver side):
/// ```ignore
/// expected = HMAC-SHA256(secret, body_bytes)
/// if constant_time_eq(request.header("X-Guardian-Signature"), "sha256=" + hex(expected)):
///     accept
/// ```
pub fn hmac_sha256_hex(secret: &str, payload: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC accepts any key length");
    mac.update(payload);
    let result = mac.finalize().into_bytes();

    // Convert to lowercase hex
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Build the `X-Guardian-Signature` header value for a webhook payload.
/// Format: `sha256=<hex_digest>` (compatible with GitHub/Discord webhook verification).
pub fn build_webhook_signature(secret: &str, payload: &[u8]) -> String {
    format!("sha256={}", hmac_sha256_hex(secret, payload))
}

/// URL-encode a string (percent-encoding for form body fields).
pub fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            b' ' => out.push('+'),
            b => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // AlertChannel::from_str_config
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_discord_channel() {
        let ch = AlertChannel::from_str_config(
            "discord;url=https://discord.com/api/webhooks/123/abc"
        ).unwrap();
        assert_eq!(
            ch,
            AlertChannel::Discord {
                webhook_url: "https://discord.com/api/webhooks/123/abc".to_string()
            }
        );
    }

    #[test]
    fn test_parse_slack_channel() {
        let ch = AlertChannel::from_str_config(
            "slack;url=https://hooks.slack.com/services/T0/B1/xyz"
        ).unwrap();
        assert_eq!(
            ch,
            AlertChannel::Slack {
                webhook_url: "https://hooks.slack.com/services/T0/B1/xyz".to_string()
            }
        );
    }

    #[test]
    fn test_parse_webhook_with_secret() {
        let ch = AlertChannel::from_str_config(
            "webhook;url=https://example.com/hook;secret=mysupersecret"
        ).unwrap();
        assert_eq!(
            ch,
            AlertChannel::Webhook {
                url: "https://example.com/hook".to_string(),
                secret: Some("mysupersecret".to_string()),
            }
        );
    }

    #[test]
    fn test_parse_webhook_no_secret() {
        let ch = AlertChannel::from_str_config(
            "webhook;url=https://example.com/hook"
        ).unwrap();
        assert_eq!(
            ch,
            AlertChannel::Webhook {
                url: "https://example.com/hook".to_string(),
                secret: None,
            }
        );
    }

    #[test]
    fn test_parse_email_channel() {
        let ch = AlertChannel::from_str_config(
            "email;address=user@example.com;api_url=https://api.mailgun.net/v3/mg.example.com/messages;api_key=key-abc123"
        ).unwrap();
        assert_eq!(
            ch,
            AlertChannel::Email {
                address: "user@example.com".to_string(),
                api_url: "https://api.mailgun.net/v3/mg.example.com/messages".to_string(),
                api_key: "key-abc123".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_unknown_channel_type_returns_none() {
        assert!(AlertChannel::from_str_config("telegram;chat_id=123").is_none());
    }

    #[test]
    fn test_parse_empty_string_returns_none() {
        assert!(AlertChannel::from_str_config("").is_none());
    }

    #[test]
    fn test_parse_discord_missing_url_returns_none() {
        assert!(AlertChannel::from_str_config("discord").is_none());
    }

    #[test]
    fn test_channel_kind_label_discord() {
        let ch = AlertChannel::Discord { webhook_url: "https://x".to_string() };
        assert_eq!(ch.kind_label(), "discord");
    }

    #[test]
    fn test_channel_kind_label_slack() {
        let ch = AlertChannel::Slack { webhook_url: "https://x".to_string() };
        assert_eq!(ch.kind_label(), "slack");
    }

    #[test]
    fn test_channel_kind_label_webhook() {
        let ch = AlertChannel::Webhook { url: "https://x".to_string(), secret: None };
        assert_eq!(ch.kind_label(), "webhook");
    }

    #[test]
    fn test_channel_kind_label_email() {
        let ch = AlertChannel::Email {
            address: "a@b.com".to_string(),
            api_url: "https://x".to_string(),
            api_key: "k".to_string(),
        };
        assert_eq!(ch.kind_label(), "email");
    }

    // -----------------------------------------------------------------------
    // DeliveryOutcome
    // -----------------------------------------------------------------------

    #[test]
    fn test_success_is_not_permanent_failure() {
        let o = DeliveryOutcome::Success { status: 200 };
        assert!(o.is_success());
        assert!(!o.is_permanent_failure());
    }

    #[test]
    fn test_http_4xx_is_permanent_failure() {
        let o = DeliveryOutcome::HttpError { status: 404, body: "not found".to_string() };
        assert!(o.is_permanent_failure());
        assert!(!o.is_success());
    }

    #[test]
    fn test_http_5xx_is_not_permanent_failure() {
        // 5xx is transient — server may recover
        let o = DeliveryOutcome::HttpError { status: 500, body: "server error".to_string() };
        assert!(!o.is_permanent_failure());
    }

    #[test]
    fn test_transport_error_is_not_permanent() {
        let o = DeliveryOutcome::TransportError { message: "timeout".to_string() };
        assert!(!o.is_permanent_failure());
        assert!(!o.is_success());
    }

    #[test]
    fn test_invalid_config_is_permanent() {
        let o = DeliveryOutcome::InvalidConfig { reason: "bad url".to_string() };
        assert!(o.is_permanent_failure());
    }

    #[test]
    fn test_insufficient_cycles_is_permanent() {
        let o = DeliveryOutcome::InsufficientCycles;
        assert!(o.is_permanent_failure());
    }

    // -----------------------------------------------------------------------
    // Discord payload builder
    // -----------------------------------------------------------------------

    #[test]
    fn test_discord_payload_contains_alert_id() {
        let p = build_discord_payload(
            "alert-abc123", "CRITICAL", 7, &["A1: large transfer".to_string()],
            "1 event", "Review transactions"
        );
        assert!(p.contains("alert-abc123"), "Payload: {}", p);
    }

    #[test]
    fn test_discord_payload_uses_orange_red_for_critical() {
        let p = build_discord_payload("id", "CRITICAL", 7, &[], "summary", "action");
        assert!(p.contains("16744272"), "Expected critical color in: {}", p);
    }

    #[test]
    fn test_discord_payload_uses_red_for_emergency() {
        let p = build_discord_payload("id", "EMERGENCY", 15, &[], "summary", "action");
        assert!(p.contains("16711680"), "Expected emergency color in: {}", p);
    }

    #[test]
    fn test_discord_payload_uses_blue_for_info() {
        let p = build_discord_payload("id", "INFO", 1, &[], "summary", "action");
        assert!(p.contains("3447003"), "Expected info color in: {}", p);
    }

    #[test]
    fn test_discord_payload_is_valid_json_structure() {
        let p = build_discord_payload(
            "alert-1", "WARN", 3,
            &["A3: rapid tx".to_string()],
            "6 events", "Monitor"
        );
        // Must start with {"embeds":[{
        assert!(p.starts_with("{\"embeds\":[{"), "Payload: {}", p);
        assert!(p.ends_with("}]}"), "Payload: {}", p);
    }

    // -----------------------------------------------------------------------
    // Slack payload builder
    // -----------------------------------------------------------------------

    #[test]
    fn test_slack_payload_contains_alert_id() {
        let p = build_slack_payload("alert-xyz", "WARN", 3, &[], "summary", "action");
        assert!(p.contains("alert-xyz"), "Payload: {}", p);
    }

    #[test]
    fn test_slack_payload_emergency_emoji() {
        let p = build_slack_payload("id", "EMERGENCY", 15, &[], "summary", "action");
        assert!(p.contains("🆘"), "Expected emergency emoji in: {}", p);
    }

    #[test]
    fn test_slack_payload_critical_emoji() {
        let p = build_slack_payload("id", "CRITICAL", 7, &[], "summary", "action");
        assert!(p.contains("🚨"), "Expected critical emoji in: {}", p);
    }

    // -----------------------------------------------------------------------
    // Generic webhook payload builder
    // -----------------------------------------------------------------------

    #[test]
    fn test_webhook_payload_is_json() {
        let p = build_webhook_payload("alert-1", "INFO", 1, &[], "no events", "none");
        assert!(p.starts_with('{') && p.ends_with('}'), "Not JSON: {}", p);
    }

    #[test]
    fn test_webhook_payload_contains_severity_score() {
        let p = build_webhook_payload("a", "CRITICAL", 7, &["A1".to_string()], "ev", "act");
        assert!(p.contains("\"severity_score\":7"), "Payload: {}", p);
    }

    #[test]
    fn test_webhook_payload_contains_rules_array() {
        let p = build_webhook_payload("a", "WARN", 3, &["A3".to_string(), "A4".to_string()], "ev", "act");
        assert!(p.contains("\"rules_triggered\":["), "Payload: {}", p);
        assert!(p.contains("\"A3\""), "Payload: {}", p);
        assert!(p.contains("\"A4\""), "Payload: {}", p);
    }

    // -----------------------------------------------------------------------
    // Email payload builder
    // -----------------------------------------------------------------------

    #[test]
    fn test_email_payload_contains_to() {
        let p = build_email_payload("user@example.com", "alert-1", "CRITICAL", 7, &[], "ev", "act");
        assert!(p.starts_with("to="), "Payload: {}", p);
    }

    #[test]
    fn test_email_payload_url_encodes_at_sign() {
        let p = build_email_payload("user@example.com", "a", "W", 3, &[], "e", "a");
        // '@' should be encoded
        assert!(p.contains("%40"), "Expected %40 in: {}", p);
    }

    // -----------------------------------------------------------------------
    // Cycle cost estimation
    // -----------------------------------------------------------------------

    #[test]
    fn test_cycle_estimate_base_cost() {
        let cost = estimate_outcall_cycles(0, 0);
        assert_eq!(cost, 49_140_000);
    }

    #[test]
    fn test_cycle_estimate_1kb_request() {
        let cost = estimate_outcall_cycles(1024, 0);
        // base + 1024 * 5200 = 49_140_000 + 5_324_800 = 54_464_800
        assert_eq!(cost, 49_140_000 + 1024 * 5_200);
    }

    #[test]
    fn test_cycle_estimate_with_response() {
        let cost = estimate_outcall_cycles(0, 2048);
        // base + 2048 * 10400 = 49_140_000 + 21_299_200 = 70_439_200
        assert_eq!(cost, 49_140_000 + 2048 * 10_400);
    }

    #[test]
    fn test_cycle_estimate_typical_webhook() {
        // 1KB request, 2KB response — typical alert delivery
        let cost = estimate_outcall_cycles(1024, 2048);
        assert!(cost > 49_140_000, "Should be more than base fee");
        assert!(cost < 200_000_000, "Should be within 200M cycle budget");
    }

    #[test]
    fn test_cycles_per_outcall_constant_covers_estimate() {
        // Our CYCLES_PER_OUTCALL budget should cover a typical webhook call
        let typical = estimate_outcall_cycles(1024, MAX_RESPONSE_BYTES);
        assert!(
            CYCLES_PER_OUTCALL as u128 >= typical,
            "Budget {} must be >= typical estimate {}",
            CYCLES_PER_OUTCALL, typical
        );
    }

    // -----------------------------------------------------------------------
    // JSON escape helper
    // -----------------------------------------------------------------------

    #[test]
    fn test_escape_json_quotes() {
        assert_eq!(escape_json(r#"say "hello""#), r#"say \"hello\""#);
    }

    #[test]
    fn test_escape_json_backslash() {
        assert_eq!(escape_json(r"path\to\file"), r"path\\to\\file");
    }

    #[test]
    fn test_escape_json_newline() {
        assert_eq!(escape_json("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_escape_json_control_char() {
        assert_eq!(escape_json("\x01"), "\\u0001");
    }

    #[test]
    fn test_escape_json_normal_string_unchanged() {
        assert_eq!(escape_json("hello world"), "hello world");
    }

    // -----------------------------------------------------------------------
    // URL encode helper
    // -----------------------------------------------------------------------

    #[test]
    fn test_url_encode_space_to_plus() {
        assert_eq!(url_encode("hello world"), "hello+world");
    }

    #[test]
    fn test_url_encode_at_sign() {
        assert_eq!(url_encode("user@host"), "user%40host");
    }

    #[test]
    fn test_url_encode_alphanumeric_unchanged() {
        assert_eq!(url_encode("abc123"), "abc123");
    }

    #[test]
    fn test_url_encode_colon_encoded() {
        assert_eq!(url_encode("a:b"), "a%3Ab");
    }

    // -----------------------------------------------------------------------
    // MAX_RETRIES constant
    // -----------------------------------------------------------------------

    #[test]
    fn test_max_retries_is_3() {
        assert_eq!(MAX_RETRIES, 3);
    }

    #[test]
    fn test_retry_count_exceeds_max_at_3() {
        // Simulate retry logic: item at retry_count=2 → will fail permanently on next attempt
        let retry_count = 2u32;
        let should_give_up = retry_count + 1 >= MAX_RETRIES;
        assert!(should_give_up, "Should give up after 3 total attempts");
    }

    #[test]
    fn test_retry_count_2_still_retries() {
        let retry_count = 1u32;
        let should_give_up = retry_count + 1 >= MAX_RETRIES;
        assert!(!should_give_up, "Should still retry at count=1");
    }

    // -----------------------------------------------------------------------
    // HMAC-SHA256 signing (Phase 2d)
    // -----------------------------------------------------------------------

    #[test]
    fn test_hmac_sha256_produces_64_char_hex() {
        let sig = hmac_sha256_hex("mysecret", b"hello world");
        assert_eq!(sig.len(), 64, "HMAC-SHA256 hex should be 64 chars: {}", sig);
    }

    #[test]
    fn test_hmac_sha256_is_lowercase_hex() {
        let sig = hmac_sha256_hex("secret", b"payload");
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
            "Should be lowercase hex: {}", sig);
    }

    #[test]
    fn test_hmac_sha256_known_value() {
        // Known test vector: HMAC-SHA256("key", "The quick brown fox jumps over the lazy dog")
        // Expected: f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8
        let sig = hmac_sha256_hex("key", b"The quick brown fox jumps over the lazy dog");
        assert_eq!(sig, "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8",
            "HMAC-SHA256 test vector mismatch");
    }

    #[test]
    fn test_hmac_sha256_different_secrets_produce_different_sigs() {
        let sig1 = hmac_sha256_hex("secret1", b"payload");
        let sig2 = hmac_sha256_hex("secret2", b"payload");
        assert_ne!(sig1, sig2, "Different secrets should produce different signatures");
    }

    #[test]
    fn test_hmac_sha256_different_payloads_produce_different_sigs() {
        let sig1 = hmac_sha256_hex("secret", b"payload1");
        let sig2 = hmac_sha256_hex("secret", b"payload2");
        assert_ne!(sig1, sig2, "Different payloads should produce different signatures");
    }

    #[test]
    fn test_hmac_sha256_same_inputs_deterministic() {
        let sig1 = hmac_sha256_hex("secret", b"data");
        let sig2 = hmac_sha256_hex("secret", b"data");
        assert_eq!(sig1, sig2, "HMAC-SHA256 must be deterministic");
    }

    #[test]
    fn test_build_webhook_signature_prefix() {
        let sig = build_webhook_signature("secret", b"payload");
        assert!(sig.starts_with("sha256="),
            "Signature header value must start with 'sha256=': {}", sig);
    }

    #[test]
    fn test_build_webhook_signature_full_length() {
        let sig = build_webhook_signature("secret", b"payload");
        // "sha256=" (7) + 64 hex chars = 71 total
        assert_eq!(sig.len(), 71, "Full signature header value should be 71 chars: {}", sig);
    }

    #[test]
    fn test_build_webhook_signature_empty_payload() {
        // HMAC of empty payload should still produce valid signature
        let sig = build_webhook_signature("mysecret", b"");
        assert!(sig.starts_with("sha256="), "Empty payload signature: {}", sig);
        assert_eq!(sig.len(), 71);
    }

    #[test]
    fn test_build_webhook_signature_unicode_secret() {
        // Secrets may contain unicode — should not panic
        let sig = build_webhook_signature("🔐secret🔑", b"data");
        assert!(sig.starts_with("sha256="), "Unicode secret signature: {}", sig);
    }

    #[test]
    fn test_webhook_signature_matches_manual_hmac() {
        let secret = "webhook_secret_key";
        let payload = b"guardian alert payload";
        let manual = hmac_sha256_hex(secret, payload);
        let from_builder = build_webhook_signature(secret, payload);
        assert_eq!(from_builder, format!("sha256={}", manual));
    }
}
