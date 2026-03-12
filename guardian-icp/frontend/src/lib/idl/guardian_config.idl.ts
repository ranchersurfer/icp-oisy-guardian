// Auto-generated IDL factory for guardian_config canister
// Matches src/guardian.did

export const idlFactory = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	const GuardianConfig = IDL.Record({
		owner: IDL.Principal,
		created_at: IDL.Nat64,
		updated_at: IDL.Nat64,
		monitored_chains: IDL.Vec(IDL.Text),
		large_transfer_pct: IDL.Float64,
		daily_outflow_pct: IDL.Float64,
		rapid_tx_count: IDL.Nat32,
		rapid_tx_window_secs: IDL.Nat64,
		new_address_alert: IDL.Bool,
		alert_threshold: IDL.Nat32,
		emergency_threshold: IDL.Nat32,
		alert_channels: IDL.Vec(IDL.Text),
		allowlisted_addresses: IDL.Vec(IDL.Text)
	});

	const HealthStatus = IDL.Record({
		status: IDL.Text,
		timestamp: IDL.Nat64,
		cycle_balance: IDL.Nat,
		cycles_per_day: IDL.Nat,
		days_until_freeze: IDL.Nat
	});

	const EmailVerificationStatus = IDL.Record({
		pending_email_masked: IDL.Opt(IDL.Text),
		pending_requested_at: IDL.Opt(IDL.Nat64),
		verified_email_masked: IDL.Opt(IDL.Text),
		verified_at: IDL.Opt(IDL.Nat64),
		delivery_active: IDL.Bool,
		code_expires_at: IDL.Opt(IDL.Nat64)
	});

	const EmailVerificationChallenge = IDL.Record({
		status: EmailVerificationStatus,
		verification_code: IDL.Text,
		demo_only: IDL.Bool
	});

	const Result = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
	const Result_1 = IDL.Variant({ Ok: GuardianConfig, Err: IDL.Text });
	const Result_2 = IDL.Variant({ Ok: HealthStatus, Err: IDL.Text });
	const Result_3 = IDL.Variant({ Ok: IDL.Tuple(IDL.Nat64, IDL.Nat64), Err: IDL.Text });
	const Result_4 = IDL.Variant({ Ok: EmailVerificationStatus, Err: IDL.Text });
	const Result_5 = IDL.Variant({ Ok: EmailVerificationChallenge, Err: IDL.Text });

	return IDL.Service({
		set_config: IDL.Func([GuardianConfig], [Result], []),
		get_config: IDL.Func([], [Result_1], ['query']),
		health: IDL.Func([], [Result_2], ['query']),
		get_stats: IDL.Func([], [Result_3], ['query']),
		begin_email_verification: IDL.Func([IDL.Text], [Result_5], []),
		confirm_email_verification: IDL.Func([IDL.Text], [Result_4], []),
		clear_verified_email: IDL.Func([], [Result_4], []),
		get_email_verification_status: IDL.Func([], [Result_4], ['query'])
	});
};

export const init = ({ IDL }: { IDL: typeof import('@dfinity/candid').IDL }) => {
	return [];
};
