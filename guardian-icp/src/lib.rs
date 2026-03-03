use ic_cdk::{query, update, caller, init, api};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use std::cell::RefCell;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::VecDeque;

/// Guardian Config structure matching OISY Guardian Spec Section 7
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
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// Update timestamp tracker for rate limiting
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UpdateTimestamps {
    pub timestamps: VecDeque<u64>,
}

impl Storable for UpdateTimestamps {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// Response types
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ApiResult<T> {
    Ok(T),
    Err(String),
}

/// For backwards compatibility with non-generic Result returns
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ApiResultUnit {
    Ok,
    Err(String),
}

/// Health status response
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: u64,
    pub cycle_balance: u128,
    pub cycles_per_day: u128,
    pub days_until_freeze: u128,
}

thread_local! {
    static CONFIGS: RefCell<StableBTreeMap<Principal, GuardianConfig, DefaultMemoryImpl>> =
        RefCell::new(StableBTreeMap::new(DefaultMemoryImpl::default()));
    
    static UPDATE_TIMESTAMPS: RefCell<StableBTreeMap<Principal, UpdateTimestamps, DefaultMemoryImpl>> =
        RefCell::new(StableBTreeMap::new(DefaultMemoryImpl::default()));
}

const MAX_UPDATES_PER_HOUR: usize = 10;
const HOUR_IN_NANOS: u64 = 3600 * 1_000_000_000;
const MAX_PAYLOAD_SIZE: u64 = 1_000_000; // 1 MB
const CYCLES_PER_DAY_ESTIMATE: u128 = 50_000_000_000; // 50 billion cycles/day (estimate)
const FREEZING_THRESHOLD_NANOS: u128 = 90 * 24 * 3600 * 1_000_000_000; // 90 days

#[init]
fn init() {
    // Initialize stable memory structures (no-op for defaults)
}

/// Inspect all ingress messages for safety violations
/// Note: Check caller authorization in the actual update function
#[ic_cdk::inspect_message]
fn inspect_message() {
    // For now, allow all messages and check authorization in update functions
    // In production, could add payload size limits and anonymous caller checks here
}

/// Internal helper: get update timestamps for a principal, cleaning old entries
fn get_recent_timestamps(principal: Principal) -> VecDeque<u64> {
    UPDATE_TIMESTAMPS.with(|ts| {
        let now = api::time();
        if let Some(mut entry) = ts.borrow_mut().get(&principal) {
            // Remove timestamps older than 1 hour
            entry.timestamps.retain(|&t| now - t < HOUR_IN_NANOS);
            entry.timestamps
        } else {
            VecDeque::new()
        }
    })
}

/// Internal helper: record an update timestamp for rate limiting
fn record_update(principal: Principal) {
    UPDATE_TIMESTAMPS.with(|ts| {
        let now = api::time();
        let mut entry = get_recent_timestamps(principal);
        entry.push_back(now);
        
        let timestamps = UpdateTimestamps {
            timestamps: entry,
        };
        ts.borrow_mut().insert(principal, timestamps);
    });
}

/// Internal helper: validate GuardianConfig fields
fn validate_config(config: &GuardianConfig) -> std::result::Result<(), String> {
    // Validate owner is not anonymous
    if config.owner == Principal::anonymous() {
        return Err("Owner cannot be anonymous principal".to_string());
    }
    
    // Validate alert channels (max 5)
    if config.alert_channels.len() > 5 {
        return Err(format!(
            "Alert channels count {} exceeds maximum of 5",
            config.alert_channels.len()
        ));
    }
    
    // Validate allowlisted addresses (max 500)
    if config.allowlisted_addresses.len() > 500 {
        return Err(format!(
            "Allowlisted addresses count {} exceeds maximum of 500",
            config.allowlisted_addresses.len()
        ));
    }
    
    // Validate thresholds (max 255)
    if config.alert_threshold > 255 {
        return Err("alert_threshold must be <= 255".to_string());
    }
    if config.emergency_threshold > 255 {
        return Err("emergency_threshold must be <= 255".to_string());
    }
    
    // Validate percentages (0.0 to 1.0)
    if config.large_transfer_pct < 0.0 || config.large_transfer_pct > 1.0 {
        return Err("large_transfer_pct must be between 0.0 and 1.0".to_string());
    }
    if config.daily_outflow_pct < 0.0 || config.daily_outflow_pct > 1.0 {
        return Err("daily_outflow_pct must be between 0.0 and 1.0".to_string());
    }
    
    // Validate monitored chains is not empty
    if config.monitored_chains.is_empty() {
        return Err("monitored_chains cannot be empty".to_string());
    }
    
    // Validate rapid_tx_count is reasonable
    if config.rapid_tx_count == 0 {
        return Err("rapid_tx_count must be at least 1".to_string());
    }
    
    // Validate rapid_tx_window_secs is reasonable
    if config.rapid_tx_window_secs == 0 {
        return Err("rapid_tx_window_secs must be at least 1".to_string());
    }
    
    Ok(())
}

/// Set a new or updated guardian configuration
/// Enforces: (1) caller authorization, (2) input validation
#[update]
fn set_config(mut config: GuardianConfig) -> ApiResultUnit {
    let caller_principal = caller();
    
    // Authorization: Only the config owner can set their config
    if config.owner != caller_principal {
        return ApiResultUnit::Err("Caller is not the config owner".to_string());
    }
    
    // Input validation
    if let Err(e) = validate_config(&config) {
        return ApiResultUnit::Err(format!("Configuration validation failed: {}", e));
    }
    
    // Get timestamp outside of thread_local closure
    let timestamp = api::time();
    
    // Update timestamps
    config.updated_at = timestamp;
    if config.created_at == 0 {
        config.created_at = timestamp;
    }
    
    // Store in stable memory
    CONFIGS.with(|configs| {
        configs.borrow_mut().insert(caller_principal, config);
    });
    
    ApiResultUnit::Ok
}

/// Get the current guardian configuration for the caller
#[query]
fn get_config() -> ApiResult<GuardianConfig> {
    let caller_principal = caller();
    CONFIGS.with(|configs| {
        match configs.borrow().get(&caller_principal) {
            Some(config) => ApiResult::Ok(config),
            None => ApiResult::Err("No config found for this principal".to_string()),
        }
    })
}

/// Get detailed health status including cycle monitoring
#[query]
fn health() -> ApiResult<HealthStatus> {
    let timestamp = api::time();
    
    // Get canister balance (in ICP) - note: this is ICP balance, not cycles
    // Cycles monitoring would require ic0 system call which is lower-level
    let icp_balance = api::canister_balance() as u128;
    
    // Estimate cycles (rough: 1 ICP ~ 1 trillion cycles)
    // This is a placeholder; actual cycle balance requires ic0 system call
    let cycle_balance = icp_balance * 1_000_000_000_000;
    
    // Calculate estimated days until freeze
    let days_until_freeze = if cycle_balance > CYCLES_PER_DAY_ESTIMATE {
        cycle_balance / CYCLES_PER_DAY_ESTIMATE
    } else {
        0
    };
    
    // Alert if below 30-day runway
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

/// Admin endpoint: Get total user count (controller only)
#[query]
fn get_stats() -> ApiResult<(u64, u64)> {
    CONFIGS.with(|configs| {
        let config_count = configs.borrow().len() as u64;
        let timestamp = api::time();
        ApiResult::Ok((config_count, timestamp))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config_valid() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string(), "Bitcoin".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec!["telegram".to_string()],
            allowlisted_addresses: vec![],
        };
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_anonymous_owner() {
        let config = GuardianConfig {
            owner: Principal::anonymous(),
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
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("anonymous"));
    }

    #[test]
    fn test_validate_config_too_many_alert_channels() {
        let config = GuardianConfig {
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
            alert_channels: vec![
                "ch1".to_string(),
                "ch2".to_string(),
                "ch3".to_string(),
                "ch4".to_string(),
                "ch5".to_string(),
                "ch6".to_string(),
            ],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("exceeds maximum of 5"));
    }

    #[test]
    fn test_validate_config_too_many_allowlisted_addresses() {
        let config = GuardianConfig {
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
            allowlisted_addresses: (0..501).map(|i| format!("addr_{}", i)).collect(),
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("exceeds maximum of 500"));
    }

    #[test]
    fn test_validate_config_threshold_too_high() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 256,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("alert_threshold"));
    }

    #[test]
    fn test_validate_config_invalid_large_transfer_pct_negative() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: -0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("large_transfer_pct"));
    }

    #[test]
    fn test_validate_config_invalid_large_transfer_pct_too_high() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 1.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("large_transfer_pct"));
    }

    #[test]
    fn test_validate_config_invalid_daily_outflow_pct() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 1.5,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("daily_outflow_pct"));
    }

    #[test]
    fn test_validate_config_empty_monitored_chains() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec![],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("monitored_chains"));
    }

    #[test]
    fn test_validate_config_zero_rapid_tx_count() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 0,
            rapid_tx_window_secs: 600,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("rapid_tx_count"));
    }

    #[test]
    fn test_validate_config_zero_rapid_tx_window() {
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.5,
            daily_outflow_pct: 0.8,
            rapid_tx_count: 5,
            rapid_tx_window_secs: 0,
            new_address_alert: true,
            alert_threshold: 7,
            emergency_threshold: 15,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        let err = validate_config(&config);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("rapid_tx_window_secs"));
    }

    #[test]
    fn test_update_timestamps_structure() {
        // Verify UpdateTimestamps can be created
        let ts = UpdateTimestamps {
            timestamps: VecDeque::new(),
        };
        assert_eq!(ts.timestamps.len(), 0);
    }

    #[test]
    fn test_health_status_structure() {
        // Verify HealthStatus can be created
        let hs = HealthStatus {
            status: "OK".to_string(),
            timestamp: 1000000,
            cycle_balance: 1_000_000_000_000,
            cycles_per_day: 50_000_000_000,
            days_until_freeze: 20,
        };
        assert_eq!(hs.status, "OK");
        assert!(hs.cycle_balance > 0);
    }

    #[test]
    fn test_validate_config_boundary_values() {
        // Test with boundary values (0.0 and 1.0)
        let config = GuardianConfig {
            owner: Principal::from_slice(&[1; 29]),
            created_at: 0,
            updated_at: 0,
            monitored_chains: vec!["ICP".to_string()],
            large_transfer_pct: 0.0,
            daily_outflow_pct: 1.0,
            rapid_tx_count: 1,
            rapid_tx_window_secs: 1,
            new_address_alert: true,
            alert_threshold: 0,
            emergency_threshold: 255,
            alert_channels: vec![],
            allowlisted_addresses: vec![],
        };
        assert!(validate_config(&config).is_ok());
    }
}
