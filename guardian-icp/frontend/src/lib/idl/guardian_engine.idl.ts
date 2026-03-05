// Auto-generated IDL factory for guardian_engine canister
// Matches src/guardian_engine.did

import { IDL } from '@dfinity/candid';

export const EngineHealthStatusIDL = IDL.Record({
	cycle_balance: IDL.Nat64,
	last_tick: IDL.Nat64,
	is_running: IDL.Bool,
	watermark_count: IDL.Nat64
});

export const idlFactory = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	const EngineHealthStatus = IDL.Record({
		cycle_balance: IDL.Nat64,
		last_tick: IDL.Nat64,
		is_running: IDL.Bool,
		watermark_count: IDL.Nat64
	});

	return IDL.Service({
		get_health: IDL.Func([], [EngineHealthStatus], ['query']),
		set_config_canister_id: IDL.Func([IDL.Principal], [], [])
	});
};

export const init = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	return [];
};
