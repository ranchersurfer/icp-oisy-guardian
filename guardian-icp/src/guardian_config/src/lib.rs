use ic_cdk::{query, update, caller, init, api};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use std::cell::RefCell;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::VecDeque;

/// Type alias for the virtual memory backed by DefaultMemoryImpl.
type Memory = VirtualMemory<DefaultMemoryImpl>;

const CONFIGS_MEM_ID: MemoryId = MemoryId::new(0);
const UPDATE_TIMESTAMPS_MEM_ID: MemoryId = MemoryId::new(1);
const EMAIL_VERIFICATIONS_MEM_ID: MemoryId = MemoryId::new(2);

const MAX_UPDATES_PER_HOUR: usize = 10;
const HOUR_IN_NANOS: u64 = 3600 * 1_000_000_000;
const MAX_PAYLOAD_SIZE: u64 = 1_000_000;
const CYCLES_PER_DAY_ESTIMATE: u128 = 50_000_000_000;
const EMAIL_VERIFICATION_CODE_TTL_NS: u64 = 15 * 60 * 1_000_000_000;
const EMAIL_VERIFICATION_CODE_DIGITS: u32 = 6;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GuardianConfig {
    pub owner: Principal,
    pub created_at: u64,
    pub updated_at: u64,
    pub monitored_chains: Vec<String>,
    pub large_transfer_pct: f64,
    pub daily_outflow_pct: f64,
    pub rapid_tx_count: u32,
    pub rapid_tx_window_secs: u64,
    pub new_address_alert: bool,
    pub alert_threshold: u32,
    pub emergency_threshold: u32,
    pub alert_channels: Vec<String>,
    pub allowlisted_addresses: Vec<String>,
}

impl Storable for GuardianConfig {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes)
            .expect("GuardianConfig::from_bytes: failed to decode — stable memory may be corrupt")
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTimestamps {
    pub timestamps: VecDeque<u64>,
}

impl Storable for UpdateTimestamps {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes)
            .expect("UpdateTimestamps::from_bytes: failed to decode — stable memory may be corrupt")
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct EmailVerificationRecord {
    pub pending_email: Option<String>,
    pub pending_code: Option<String>,
    pub requested_at: Option<u64>,
    pub verified_email: Option<String>,
    pub verified_at: Option<u64>,
}

impl Storable for EmailVerificationRecord {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect(
            "EmailVerificationRecord::from_bytes: failed to decode — stable memory may be corrupt",
        )
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ApiResult<T> {
    Ok(T),
    Err(String),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ApiResultUnit {
    Ok,
    Err(String),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: u64,
    pub cycle_balance: u128,
    pub cycles_per_day: u128,
    pub days_until_freeze: u128,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EmailVerificationStatus {
    pub pending_email_masked: Option<String>,
    pub pending_requested_at: Option<u64>,
    pub verified_email_masked: Option<String>,
    pub verified_at: Option<u64>,
    pub delivery_active: bool,
    pub code_expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EmailVerificationChallenge {
    pub status: EmailVerificationStatus,
    pub verification_code: String,
    pub demo_only: bool,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CONFIGS: RefCell<StableBTreeMap<Principal, GuardianConfig, Memory>> =
        RefCell::new(StableBTreeMap::new(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(CONFIGS_MEM_ID))
        ));

    static UPDATE_TIMESTAMPS: RefCell<StableBTreeMap<Principal, UpdateTimestamps, Memory>> =
        RefCell::new(StableBTreeMap::new(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(UPDATE_TIMESTAMPS_MEM_ID))
        ));

    static EMAIL_VERIFICATIONS: RefCell<StableBTreeMap<Principal, EmailVerificationRecord, Memory>> =
        RefCell::new(StableBTreeMap::new(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(EMAIL_VERIFICATIONS_MEM_ID))
        ));
}

#[init]
fn init() {}

#[ic_cdk::inspect_message]
fn inspect_message() {
    if caller() == Principal::anonymous() {
        ic_cdk::trap("Anonymous callers are rejected");
    }

    let arg_size = ic_cdk::api::call::arg_data_raw().len() as u64;
    if arg_size > MAX_PAYLOAD_SIZE {
        ic_cdk::trap(&format!(
            "Payload too large: {} bytes exceeds maximum of {} bytes",
            arg_size, MAX_PAYLOAD_SIZE
        ));
    }

    ic_cdk::api::call::accept_message();
}

fn get_recent_timestamps(principal: Principal) -> VecDeque<u64> {
    UPDATE_TIMESTAMPS.with(|ts| {
        let now = api::time();
        if let Some(mut entry) = ts.borrow_mut().get(&principal) {
            entry.timestamps.retain(|&t| now - t < HOUR_IN_NANOS);
            entry.timestamps
        } else {
            VecDeque::new()
        }
    })
}

fn record_update(principal: Principal) {
    UPDATE_TIMESTAMPS.with(|ts| {
        let now = api::time();
        let mut entry = get_recent_timestamps(principal);
        entry.push_back(now);

        ts.borrow_mut().insert(principal, UpdateTimestamps { timestamps: entry });
    });
}

fn validate_config(config: &GuardianConfig) -> std::result::Result<(), String> {
    if config.owner == Principal::anonymous() {
        return Err("Owner cannot be anonymous principal".to_string());
    }

    if config.alert_channels.len() > 5 {
        return Err(format!(
            "Alert channels count {} exceeds maximum of 5",
            config.alert_channels.len()
        ));
    }

    if config.allowlisted_addresses.len() > 500 {
        return Err(format!(
            "Allowlisted addresses count {} exceeds maximum of 500",
            config.allowlisted_addresses.len()
        ));
    }

    if config.alert_threshold > 255 {
        return Err("alert_threshold must be <= 255".to_string());
    }
    if config.emergency_threshold > 255 {
        return Err("emergency_threshold must be <= 255".to_string());
    }

    if !config.large_transfer_pct.is_finite()
        || config.large_transfer_pct < 0.0
        || config.large_transfer_pct > 1.0
    {
        return Err("large_transfer_pct must be a finite value between 0.0 and 1.0".to_string());
    }
    if !config.daily_outflow_pct.is_finite()
        || config.daily_outflow_pct < 0.0
        || config.daily_outflow_pct > 1.0
    {
        return Err("daily_outflow_pct must be a finite value between 0.0 and 1.0".to_string());
    }

    if config.monitored_chains.is_empty() {
        return Err("monitored_chains cannot be empty".to_string());
    }

    if config.rapid_tx_count == 0 {
        return Err("rapid_tx_count must be at least 1".to_string());
    }

    if config.rapid_tx_window_secs == 0 {
        return Err("rapid_tx_window_secs must be at least 1".to_string());
    }

    Ok(())
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn validate_email_address(email: &str) -> std::result::Result<String, String> {
    let normalized = normalize_email(email);
    if normalized.is_empty() {
        return Err("Email address cannot be empty".to_string());
    }
    let parts: Vec<&str> = normalized.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err("Email address must look like name@example.com".to_string());
    }
    if !parts[1].contains('.') {
        return Err("Email domain must include a dot".to_string());
    }
    Ok(normalized)
}

fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return "hidden".to_string();
    }
    let local = parts[0];
    let domain = parts[1];
    let local_prefix = local.chars().next().unwrap_or('•');
    let domain_parts: Vec<&str> = domain.split('.').collect();
    let root = domain_parts.first().copied().unwrap_or(domain);
    let root_prefix = root.chars().next().unwrap_or('•');
    let suffix = if domain_parts.len() > 1 {
        format!(".{}", domain_parts[1..].join("."))
    } else {
        String::new()
    };
    format!("{}••••@{}••••{}", local_prefix, root_prefix, suffix)
}

fn generate_email_verification_code(now: u64, principal: Principal) -> String {
    let seed = now ^ (principal.as_slice().iter().fold(0u64, |acc, byte| acc.wrapping_mul(131).wrapping_add(*byte as u64)));
    let modulus = 10u64.pow(EMAIL_VERIFICATION_CODE_DIGITS);
    format!("{:06}", (seed % modulus) as u32)
}

fn email_channel_string(email: &str) -> String {
    format!("email;address={};verified=true", email)
}

fn active_verified_email_channel(record: &EmailVerificationRecord) -> Option<String> {
    record
        .verified_email
        .as_ref()
        .map(|email| email_channel_string(email))
}

fn sanitize_alert_channels(principal: Principal, channels: &[String]) -> Vec<String> {
    let verified_email_channel = EMAIL_VERIFICATIONS.with(|records| {
        records
            .borrow()
            .get(&principal)
            .and_then(|record| active_verified_email_channel(&record))
    });

    let mut sanitized: Vec<String> = channels
        .iter()
        .filter(|value| !value.trim_start().starts_with("email;"))
        .cloned()
        .collect();

    if let Some(channel) = verified_email_channel {
        sanitized.push(channel);
    }

    sanitized.truncate(5);
    sanitized
}

fn email_status_from_record(record: &EmailVerificationRecord) -> EmailVerificationStatus {
    let code_expires_at = record.requested_at.map(|ts| ts + EMAIL_VERIFICATION_CODE_TTL_NS);
    EmailVerificationStatus {
        pending_email_masked: record.pending_email.as_ref().map(|value| mask_email(value)),
        pending_requested_at: record.requested_at,
        verified_email_masked: record.verified_email.as_ref().map(|value| mask_email(value)),
        verified_at: record.verified_at,
        delivery_active: record.verified_email.is_some(),
        code_expires_at,
    }
}

fn load_email_record(principal: Principal) -> EmailVerificationRecord {
    EMAIL_VERIFICATIONS.with(|records| records.borrow().get(&principal).unwrap_or_default())
}

fn save_email_record(principal: Principal, record: &EmailVerificationRecord) {
    EMAIL_VERIFICATIONS.with(|records| {
        records.borrow_mut().insert(principal, record.clone());
    });
}

#[update]
fn set_config(mut config: GuardianConfig) -> ApiResultUnit {
    let caller_principal = caller();

    if config.owner != caller_principal {
        return ApiResultUnit::Err("Caller is not the config owner".to_string());
    }

    let recent = get_recent_timestamps(caller_principal);
    if recent.len() >= MAX_UPDATES_PER_HOUR {
        return ApiResultUnit::Err(format!(
            "Rate limit exceeded: maximum {} updates per hour",
            MAX_UPDATES_PER_HOUR
        ));
    }

    config.alert_channels = sanitize_alert_channels(caller_principal, &config.alert_channels);

    if let Err(e) = validate_config(&config) {
        return ApiResultUnit::Err(format!("Configuration validation failed: {}", e));
    }

    let timestamp = api::time();
    config.updated_at = timestamp;
    if config.created_at == 0 {
        config.created_at = timestamp;
    }

    record_update(caller_principal);

    CONFIGS.with(|configs| {
        configs.borrow_mut().insert(caller_principal, config);
    });

    ApiResultUnit::Ok
}

#[query]
fn get_config() -> ApiResult<GuardianConfig> {
    let caller_principal = caller();
    CONFIGS.with(|configs| match configs.borrow().get(&caller_principal) {
        Some(mut config) => {
            config.alert_channels = sanitize_alert_channels(caller_principal, &config.alert_channels);
            ApiResult::Ok(config)
        }
        None => ApiResult::Err("No config found for this principal".to_string()),
    })
}

#[update]
fn begin_email_verification(email: String) -> ApiResult<EmailVerificationChallenge> {
    let caller_principal = caller();
    let normalized = match validate_email_address(&email) {
        Ok(value) => value,
        Err(err) => return ApiResult::Err(err),
    };

    let now = api::time();
    let mut record = load_email_record(caller_principal);
    record.pending_email = Some(normalized.clone());
    record.pending_code = Some(generate_email_verification_code(now, caller_principal));
    record.requested_at = Some(now);

    if record.verified_email.as_ref() == Some(&normalized) {
        record.verified_email = None;
        record.verified_at = None;
    }

    save_email_record(caller_principal, &record);

    if let Some(mut config) = CONFIGS.with(|configs| configs.borrow().get(&caller_principal)) {
        config.alert_channels = sanitize_alert_channels(caller_principal, &config.alert_channels);
        config.updated_at = now;
        CONFIGS.with(|configs| {
            configs.borrow_mut().insert(caller_principal, config);
        });
    }

    ApiResult::Ok(EmailVerificationChallenge {
        status: email_status_from_record(&record),
        verification_code: record.pending_code.clone().unwrap_or_default(),
        demo_only: true,
    })
}

#[update]
fn confirm_email_verification(code: String) -> ApiResult<EmailVerificationStatus> {
    let caller_principal = caller();
    let now = api::time();
    let mut record = load_email_record(caller_principal);

    let pending_code = match record.pending_code.clone() {
        Some(value) => value,
        None => return ApiResult::Err("No pending email verification exists".to_string()),
    };
    let pending_email = match record.pending_email.clone() {
        Some(value) => value,
        None => return ApiResult::Err("No pending email verification exists".to_string()),
    };
    let requested_at = match record.requested_at {
        Some(value) => value,
        None => return ApiResult::Err("No pending email verification exists".to_string()),
    };

    if now.saturating_sub(requested_at) > EMAIL_VERIFICATION_CODE_TTL_NS {
        record.pending_code = None;
        save_email_record(caller_principal, &record);
        return ApiResult::Err("Verification code expired. Request a new one.".to_string());
    }

    if code.trim() != pending_code {
        return ApiResult::Err("Verification code does not match".to_string());
    }

    record.pending_code = None;
    record.pending_email = None;
    record.requested_at = None;
    record.verified_email = Some(pending_email.clone());
    record.verified_at = Some(now);
    save_email_record(caller_principal, &record);

    if let Some(mut config) = CONFIGS.with(|configs| configs.borrow().get(&caller_principal)) {
        config.alert_channels = sanitize_alert_channels(caller_principal, &config.alert_channels);
        config.updated_at = now;
        CONFIGS.with(|configs| {
            configs.borrow_mut().insert(caller_principal, config);
        });
    }

    ApiResult::Ok(email_status_from_record(&record))
}

#[update]
fn clear_verified_email() -> ApiResult<EmailVerificationStatus> {
    let caller_principal = caller();
    let now = api::time();
    let mut record = load_email_record(caller_principal);
    record.verified_email = None;
    record.verified_at = None;
    save_email_record(caller_principal, &record);

    if let Some(mut config) = CONFIGS.with(|configs| configs.borrow().get(&caller_principal)) {
        config.alert_channels = sanitize_alert_channels(caller_principal, &config.alert_channels);
        config.updated_at = now;
        CONFIGS.with(|configs| {
            configs.borrow_mut().insert(caller_principal, config);
        });
    }

    ApiResult::Ok(email_status_from_record(&record))
}

#[query]
fn get_email_verification_status() -> ApiResult<EmailVerificationStatus> {
    let caller_principal = caller();
    let record = load_email_record(caller_principal);
    ApiResult::Ok(email_status_from_record(&record))
}

#[query]
fn health() -> ApiResult<HealthStatus> {
    let timestamp = api::time();
    let cycle_balance = api::canister_balance() as u128;

    let days_until_freeze = if cycle_balance > CYCLES_PER_DAY_ESTIMATE {
        cycle_balance / CYCLES_PER_DAY_ESTIMATE
    } else {
        0
    };

    let status = if days_until_freeze < 30 {
        format!(
            "WARNING: Low cycle balance. {} days of runway remaining.",
            days_until_freeze
        )
    } else {
        "Guardian OK".to_string()
    };

    ApiResult::Ok(HealthStatus {
        status,
        timestamp,
        cycle_balance,
        cycles_per_day: CYCLES_PER_DAY_ESTIMATE,
        days_until_freeze,
    })
}

#[query]
fn get_stats() -> ApiResult<(u64, u64)> {
    CONFIGS.with(|configs| {
        let config_count = configs.borrow().len();
        let timestamp = api::time();
        ApiResult::Ok((config_count, timestamp))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> GuardianConfig {
        GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        }
    }

    #[test]
    fn test_validate_config_valid() {
        assert!(validate_config(&sample_config()).is_ok());
    }

    #[test]
    fn test_validate_config_anonymous_owner() {
        let mut config = sample_config();
        config.owner = Principal::anonymous();
        let err = validate_config(&config).unwrap_err();
        assert!(err.contains("anonymous"));
    }

    #[test]
    fn test_validate_config_too_many_alert_channels() {
        let mut config = sample_config();
        config.alert_channels = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()];
        let err = validate_config(&config).unwrap_err();
        assert!(err.contains("exceeds maximum of 5"));
    }

    #[test]
    fn test_validate_config_too_many_allowlisted_addresses() {
        let mut config = sample_config();
        config.allowlisted_addresses = (0..501).map(|i| format!("addr_{}", i)).collect();
        let err = validate_config(&config).unwrap_err();
        assert!(err.contains("exceeds maximum of 500"));
    }

    #[test]
    fn test_validate_email_address_accepts_normal_email() {
        assert_eq!(validate_email_address(" Alice@Example.com ").unwrap(), "alice@example.com");
    }

    #[test]
    fn test_validate_email_address_rejects_bad_shape() {
        assert!(validate_email_address("aliceexample.com").is_err());
        assert!(validate_email_address("alice@").is_err());
        assert!(validate_email_address("@example.com").is_err());
    }

    #[test]
    fn test_mask_email_masks_both_local_and_domain() {
        let masked = mask_email("alice@example.com");
        assert!(masked.starts_with("a••••@e••••"));
        assert!(masked.ends_with(".com"));
    }

    #[test]
    fn test_email_channel_string_marks_verified() {
        assert_eq!(email_channel_string("alice@example.com"), "email;address=alice@example.com;verified=true");
    }

    #[test]
    fn test_email_status_reports_delivery_only_for_verified_email() {
        let record = EmailVerificationRecord {
            pending_email: Some("pending@example.com".into()),
            pending_code: Some("123456".into()),
            requested_at: Some(100),
            verified_email: Some("verified@example.com".into()),
            verified_at: Some(200),
        };
        let status = email_status_from_record(&record);
        assert!(status.delivery_active);
        assert!(status.pending_email_masked.is_some());
        assert!(status.verified_email_masked.is_some());
    }

    #[test]
    fn test_generate_email_verification_code_is_six_digits() {
        let code = generate_email_verification_code(123456789, Principal::from_slice(&[7; 29]));
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|ch| ch.is_ascii_digit()));
    }
}

ic_cdk::export_candid!();
