import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface GuardianConfig {
  'rapid_tx_window_secs' : bigint,
  'daily_outflow_pct' : number,
  'updated_at' : bigint,
  'new_address_alert' : boolean,
  'owner' : Principal,
  'alert_threshold' : number,
  'alert_channels' : Array<string>,
  'allowlisted_addresses' : Array<string>,
  'created_at' : bigint,
  'monitored_chains' : Array<string>,
  'emergency_threshold' : number,
  'rapid_tx_count' : number,
  'large_transfer_pct' : number,
}
export interface HealthStatus {
  'status' : string,
  'cycles_per_day' : bigint,
  'cycle_balance' : bigint,
  'timestamp' : bigint,
  'days_until_freeze' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : GuardianConfig } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : HealthStatus } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : [bigint, bigint] } |
  { 'Err' : string };
export interface _SERVICE {
  'get_config' : ActorMethod<[], Result_1>,
  'get_stats' : ActorMethod<[], Result_3>,
  'health' : ActorMethod<[], Result_2>,
  'set_config' : ActorMethod<[GuardianConfig], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
