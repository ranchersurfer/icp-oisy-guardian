// Auto-generated IDL factory for guardian_engine canister
// Matches src/guardian_engine.did

import { IDL } from '@dfinity/candid';

export const ConsumerAlertRecordIDL = IDL.Record({
	alert_id: IDL.Text,
	timestamp: IDL.Nat64,
	severity: IDL.Text,
	severity_score: IDL.Nat8,
	rules_triggered: IDL.Vec(IDL.Text),
	events_summary: IDL.Text,
	recommended_action: IDL.Text
});

export const EngineHealthStatusIDL = IDL.Record({
	cycle_balance: IDL.Nat64,
	last_tick: IDL.Nat64,
	is_running: IDL.Bool,
	watermark_count: IDL.Nat64
});

export const idlFactory = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	const ConsumerAlertRecord = IDL.Record({
		alert_id: IDL.Text,
		timestamp: IDL.Nat64,
		severity: IDL.Text,
		severity_score: IDL.Nat8,
		rules_triggered: IDL.Vec(IDL.Text),
		events_summary: IDL.Text,
		recommended_action: IDL.Text
	});

	const EngineHealthStatus = IDL.Record({
		cycle_balance: IDL.Nat64,
		last_tick: IDL.Nat64,
		is_running: IDL.Bool,
		watermark_count: IDL.Nat64
	});

	return IDL.Service({
		get_health: IDL.Func([], [EngineHealthStatus], ['query']),
		get_my_alerts: IDL.Func([IDL.Nat64], [IDL.Vec(ConsumerAlertRecord)], ['query']),
		get_alert_queue_len: IDL.Func([], [IDL.Nat64], ['query']),
		set_config_canister_id: IDL.Func([IDL.Principal], [], [])
	});
};

export const init = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	return [];
};
