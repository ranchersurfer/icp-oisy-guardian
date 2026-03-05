// Guardian ICP Admin Dashboard — Type Definitions
// Mirrors guardian_engine canister response types

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
