export interface EngineHealthStatus {
	cycle_balance: bigint;
	last_tick: bigint;
	is_running: boolean;
	watermark_count: bigint;
}

export interface AlertChannel {
	type: 'Discord' | 'Slack' | 'Webhook' | 'Email';
	target: string;
}

export interface UserConfig {
	principal: string;
	alert_threshold: number;
	alert_channels: AlertChannel[];
	detection_rules: DetectionRule[];
	created_at: bigint;
	updated_at: bigint;
}

export interface DetectionRule {
	id: string;
	name: string;
	enabled: boolean;
	severity: 'INFO' | 'WARN' | 'CRITICAL' | 'EMERGENCY';
	description: string;
}

export type AlertStatus = 'Pending' | 'Sent' | 'Failed';
export type AlertSeverity = 'INFO' | 'WARN' | 'CRITICAL' | 'EMERGENCY';

export interface AlertRecord {
	alert_id: string;
	timestamp: bigint;
	user: string;
	rules_triggered: string[];
	severity: AlertSeverity;
	severity_score: number;
	status: AlertStatus;
	events_summary: string;
	recommended_action: string;
	chain: 'ICP' | 'CkBTC' | 'CkETH';
}

export interface SystemStats {
	total_users: number;
	total_alerts_queued: number;
	alerts_sent: number;
	alerts_failed: number;
	alerts_pending: number;
	uptime_ticks: number;
	last_sync: bigint;
}

export interface CanisterHealth {
	engine: EngineHealthStatus;
	config_canister_id: string | null;
	alert_queue_len: bigint;
}

export interface GuardianConfigRecord {
	owner: { toString(): string };
	created_at: bigint;
	updated_at: bigint;
	monitored_chains: string[];
	large_transfer_pct: number;
	daily_outflow_pct: number;
	rapid_tx_count: number;
	rapid_tx_window_secs: bigint;
	new_address_alert: boolean;
	alert_threshold: number;
	emergency_threshold: number;
	alert_channels: string[];
	allowlisted_addresses: string[];
}

export type GuardianConfigResult = { Ok: GuardianConfigRecord } | { Err: string };
export type GuardianWriteResult = { Ok: null } | { Err: string };

export type GuardianPresetId = 'safe' | 'balanced' | 'aggressive';

export interface GuardianPreset {
	id: GuardianPresetId;
	name: string;
	tagline: string;
	description: string;
	recommended?: boolean;
	config: {
		large_transfer_pct: number;
		daily_outflow_pct: number;
		rapid_tx_count: number;
		rapid_tx_window_secs: number;
		new_address_alert: boolean;
		alert_threshold: number;
		emergency_threshold: number;
		monitored_chains: string[];
	};
}

export interface GuardianConfigView {
	preset: GuardianPresetId | null;
	status: 'active' | 'not_configured';
	owner: string;
	lastUpdated: bigint;
	createdAt: bigint;
	monitoredChains: string[];
	largeTransferPct: number;
	rapidTxCount: number;
	rapidTxWindowSecs: number;
	newAddressAlert: boolean;
	alertThreshold: number;
	emergencyThreshold: number;
	alertChannels: string[];
	allowlistedAddresses: string[];
}
