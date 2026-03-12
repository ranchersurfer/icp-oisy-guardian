import {
	fetchHealth as mockFetchHealth,
	fetchStats as mockFetchStats
} from './mock.js';
import { createConfigActor, createEngineActor, CANISTER_IDS } from './auth.js';
import type {
	AlertRecord,
	CanisterHealth,
	EmailVerificationChallengeResult,
	EmailVerificationStatusResult,
	GuardianConfigRecord,
	GuardianConfigResult,
	GuardianWriteResult,
	SystemStats,
	UserConfig,
	AlertChannel
} from './types.js';

function getEnv(key: string): string | undefined {
	return (import.meta.env as Record<string, string | undefined>)[key];
}

const NETWORK = getEnv('VITE_CANISTER_NETWORK') ?? 'local';
const USE_MOCK = getEnv('VITE_USE_MOCK') === 'true';
const HOST_MAP: Record<string, string> = {
	local: 'http://127.0.0.1:4943',
	testnet: 'https://icp0.io',
	ic: 'https://icp0.io',
	mainnet: 'https://icp0.io'
};
const IC_HOST = getEnv('VITE_IC_HOST') ?? HOST_MAP[NETWORK] ?? 'https://icp0.io';

interface RawEngineHealth {
	cycle_balance: bigint;
	last_tick: bigint;
	is_running: boolean;
	watermark_count: bigint;
}

function parseAlertChannel(raw: string): AlertChannel {
	if (raw.startsWith('discord;')) {
		return { type: 'Discord', target: raw.replace('discord;url=', '') };
	} else if (raw.startsWith('slack;')) {
		return { type: 'Slack', target: raw.replace('slack;url=', '') };
	} else if (raw.startsWith('email;')) {
		return { type: 'Email', target: raw.replace('email;address=', '').split(';')[0] };
	} else if (raw.startsWith('webhook;')) {
		return { type: 'Webhook', target: raw.replace('webhook;url=', '').split(';')[0] };
	}
	return { type: 'Webhook', target: raw };
}

function rawConfigToUserConfig(cfg: GuardianConfigRecord): UserConfig {
	return {
		principal: cfg.owner.toString(),
		alert_threshold: Number(cfg.alert_threshold),
		alert_channels: cfg.alert_channels.map(parseAlertChannel),
		detection_rules: [
			{
				id: 'A1',
				name: 'Large Transfer',
				enabled: true,
				severity: 'CRITICAL',
				description: 'Flags outgoing transfers exceeding the configured share of wallet balance.'
			},
			{
				id: 'A3',
				name: 'Rapid Transactions',
				enabled: true,
				severity: 'WARN',
				description: 'Detects bursts of transactions in a short time window.'
			},
			{
				id: 'A4',
				name: 'New Destination Address',
				enabled: true,
				severity: 'INFO',
				description: 'Alerts when funds move to a previously unseen address.'
			}
		],
		created_at: cfg.created_at,
		updated_at: cfg.updated_at
	};
}

export async function fetchHealth(): Promise<CanisterHealth> {
	if (USE_MOCK) return mockFetchHealth();

	try {
		const engine = (await createEngineActor()) as any;
		const raw = (await engine.get_health()) as RawEngineHealth;
		let queueLen = BigInt(0);
		try {
			queueLen = BigInt((await engine.get_alert_queue_len()) as bigint | number | string);
		} catch {
			queueLen = BigInt(0);
		}

		return {
			engine: {
				cycle_balance: BigInt(raw.cycle_balance),
				last_tick: BigInt(raw.last_tick),
				is_running: Boolean(raw.is_running),
				watermark_count: BigInt(raw.watermark_count)
			},
			config_canister_id: CANISTER_IDS.config,
			alert_queue_len: queueLen
		};
	} catch {
		return mockFetchHealth();
	}
}

export async function fetchUsers(): Promise<UserConfig[]> {
	if (USE_MOCK) return [];

	const result = await getMyConfig();
	if ('Ok' in result) {
		return [rawConfigToUserConfig(result.Ok)];
	}
	return [];
}

function normalizeSeverity(raw: string): AlertRecord['severity'] {
	const normalized = raw.toUpperCase();
	if (normalized === 'INFO' || normalized === 'WARN' || normalized === 'CRITICAL' || normalized === 'EMERGENCY') {
		return normalized;
	}
	return 'INFO';
}

interface RawConsumerAlertRecord {
	alert_id: string;
	timestamp: bigint;
	severity: string;
	severity_score: bigint | number;
	rules_triggered: string[];
	events_summary: string;
	recommended_action: string;
}

export async function fetchAlerts(limit = 20): Promise<AlertRecord[]> {
	if (USE_MOCK) return [];

	const engine = (await createEngineActor()) as any;
	const rawAlerts = (await engine.get_my_alerts(BigInt(limit))) as RawConsumerAlertRecord[];

	return rawAlerts.map((alert) => ({
		alert_id: alert.alert_id,
		timestamp: BigInt(alert.timestamp),
		rules_triggered: alert.rules_triggered,
		severity: normalizeSeverity(alert.severity),
		severity_score: Number(alert.severity_score),
		events_summary: alert.events_summary,
		recommended_action: alert.recommended_action
	}));
}

export async function fetchStats(): Promise<SystemStats> {
	if (USE_MOCK) return mockFetchStats();
	const health = await fetchHealth();
	return {
		total_users: 1,
		total_alerts_queued: Number(health.alert_queue_len),
		alerts_sent: 0,
		alerts_failed: 0,
		alerts_pending: 0,
		uptime_ticks: Number(health.engine.watermark_count) * 30,
		last_sync: health.engine.last_tick
	};
}

export async function getMyConfig(): Promise<GuardianConfigResult> {
	const actor = await createConfigActor();
	return (await actor.get_config()) as GuardianConfigResult;
}

export async function saveConfig(config: GuardianConfigRecord): Promise<GuardianWriteResult> {
	const actor = await createConfigActor();
	return (await actor.set_config(config)) as GuardianWriteResult;
}

export async function getEmailVerificationStatus(): Promise<EmailVerificationStatusResult> {
	const actor = await createConfigActor();
	return (await actor.get_email_verification_status()) as EmailVerificationStatusResult;
}

export async function beginEmailVerification(email: string): Promise<EmailVerificationChallengeResult> {
	const actor = await createConfigActor();
	return (await actor.begin_email_verification(email)) as EmailVerificationChallengeResult;
}

export async function confirmEmailVerification(code: string): Promise<EmailVerificationStatusResult> {
	const actor = await createConfigActor();
	return (await actor.confirm_email_verification(code)) as EmailVerificationStatusResult;
}

export async function clearVerifiedEmail(): Promise<EmailVerificationStatusResult> {
	const actor = await createConfigActor();
	return (await actor.clear_verified_email()) as EmailVerificationStatusResult;
}

export function isLiveMode(): boolean {
	return !USE_MOCK;
}

export function isOperatorModeEnabled(): boolean {
	return getEnv('VITE_ENABLE_OPERATOR_ROUTES') === 'true';
}

export function getActiveCanisterIds(): { engine: string; config: string } {
	return { ...CANISTER_IDS };
}

export function getActiveHost(): string {
	return IC_HOST;
}
