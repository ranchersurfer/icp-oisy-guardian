export const idlFactory = ({ IDL }) => {
  const GuardianConfig = IDL.Record({
    'rapid_tx_window_secs' : IDL.Nat64,
    'daily_outflow_pct' : IDL.Float64,
    'updated_at' : IDL.Nat64,
    'new_address_alert' : IDL.Bool,
    'owner' : IDL.Principal,
    'alert_threshold' : IDL.Nat32,
    'alert_channels' : IDL.Vec(IDL.Text),
    'allowlisted_addresses' : IDL.Vec(IDL.Text),
    'created_at' : IDL.Nat64,
    'monitored_chains' : IDL.Vec(IDL.Text),
    'emergency_threshold' : IDL.Nat32,
    'rapid_tx_count' : IDL.Nat32,
    'large_transfer_pct' : IDL.Float64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : GuardianConfig, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'Err' : IDL.Text,
  });
  const HealthStatus = IDL.Record({
    'status' : IDL.Text,
    'cycles_per_day' : IDL.Nat,
    'cycle_balance' : IDL.Nat,
    'timestamp' : IDL.Nat64,
    'days_until_freeze' : IDL.Nat,
  });
  const Result_2 = IDL.Variant({ 'Ok' : HealthStatus, 'Err' : IDL.Text });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'get_config' : IDL.Func([], [Result_1], ['query']),
    'get_stats' : IDL.Func([], [Result_3], ['query']),
    'health' : IDL.Func([], [Result_2], ['query']),
    'set_config' : IDL.Func([GuardianConfig], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
