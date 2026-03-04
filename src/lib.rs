use ic_cdk::{query, update, caller, init};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

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
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum Result<T> { Ok(T), Err(String) }
thread_local! {
static CONFIGS: RefCell<StableBTreeMap<Principal, GuardianConfig, DefaultMemoryImpl>> =
RefCell::new(StableBTreeMap::new(DefaultMemoryImpl::default()));
}

#[init]
fn init() {}

#[ic_cdk::inspect_message]
fn inspect_message() {
if caller() == Principal::anonymous() {
ic_cdk::api::trap("Anonymous callers not allowed");
}
}
#[update]
fn set_config(config: GuardianConfig) -> Result<()> {
let caller_principal = caller();
if config.owner != caller_principal {
return Result::Err("Caller is not the config owner".to_string());
}
let timestamp = ic_cdk::api::time();
let mut new_config = config;
new_config.updated_at = timestamp;
if new_config.created_at == 0 {
new_config.created_at = timestamp;
}
CONFIGS.with(|configs| {
configs.borrow_mut().insert(caller_principal, new_config);
});
Result::Ok(())
}

#[query]
fn get_config() -> Result<GuardianConfig> {
let caller_principal = caller();
CONFIGS.with(|configs| {
match configs.borrow().get(&caller_principal) {
Some(config) => Result::Ok(config),
None => Result::Err("No config found".to_string()),
}
})
}

#[query]
fn health() -> String {
format!("Guardian OK. Time: {}", ic_cdk::api::time())
}
