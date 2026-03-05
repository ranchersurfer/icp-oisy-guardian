// Guardian ICP Admin Dashboard — Real Canister Integration
// Replaces mock.ts with @dfinity/agent calls where possible.
// Falls back to mock data gracefully when:
//   - VITE_USE_MOCK=true
//   - Canister unreachable
//   - Method not yet exported by canister

import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as engineIdlFactory } from './idl/guardian_engine.idl.js';
import { idlFactory as configIdlFactory } from './idl/guardian_config.idl.js';
import {
	fetchHealth as mockFetchHealth,
	fetchAlerts as mockFetchAlerts,
	fetchUsers as mockFetchUsers,
	fetchStats as mockFetchStats
} from './mock.js';
import type { AlertRecord, CanisterHealth, SystemStats, UserConfig, AlertChannel } from './types.js';

// ─── Environment ────────────────────────────────────────────────────────────

function getEnv(key: string): string | undefined {
	// Vite replaces import.meta.env at build time
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

// Local replica canister IDs (from dfx deploy output)
const LOCAL_ENGINE_ID = 'u6s2n-gx777-77774-qaaba-cai';
const LOCAL_CONFIG_ID = 'uxrrr-q7777-77774-qaaaq-cai';

// Parse canister IDs from environment (JSON object or individual vars)
function getCanisterIds(): { engine: string; config: string } {
	const raw = getEnv('VITE_CANISTER_IDS');
	if (raw) {
		try {
			const parsed = JSON.parse(raw) as Record<string, string>;
			return {
				engine: parsed.guardian_engine ?? LOCAL_ENGINE_ID,
				config: parsed.guardian_config ?? LOCAL_CONFIG_ID
			};
		} catch {
			// ignore parse errors
		}
	}
	return {
		engine: getEnv('VITE_ENGINE_CANISTER_ID') ?? LOCAL_ENGINE_ID,
		config: getEnv('VITE_CONFIG_CANISTER_ID') ?? LOCAL_CONFIG_ID
	};
}

const CANISTER_IDS = getCanisterIds();

// ─── Agent / Actor Setup ─────────────────────────────────────────────────────

let _agent: HttpAgent | null = null;

function getAgent(): HttpAgent {
	if (!_agent) {
		_agent = new HttpAgent({ host: IC_HOST });
		// Fetch root key only for local/test replicas (not production IC)
		if (NETWORK === 'local' || NETWORK === 'testnet') {
			_agent.fetchRootKey().catch(() => {
				// Non-fatal: root key fetch may fail if replica isn't running.
				// Calls will still work for certified queries that don't need it.
			});
		}
	}
	return _agent;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function createEngineActor(): any {
	return Actor.createActor(engineIdlFactory, {
		agent: getAgent(),
		canisterId: CANISTER_IDS.engine
	});
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function createConfigActor(): any {
	return Actor.createActor(configIdlFactory, {
		agent: getAgent(),
		canisterId: CANISTER_IDS.config
	});
}

// ─── Type Adapters ───────────────────────────────────────────────────────────

interface RawEngineHealth {
	cycle_balance: bigint;
	last_tick: bigint;
	is_running: boolean;
	watermark_count: bigint;
}

interface RawGuardianConfig {
	owner: { toString(): string };
	created_at: bigint;
	updated_at: bigint;
	alert_threshold: number;
	alert_channels: string[];
	// other fields omitted for admin view
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

function rawConfigToUserConfig(cfg: RawGuardianConfig): UserConfig {
	return {
		principal: cfg.owner.toString(),
		alert_threshold: Number(cfg.alert_threshold),
		alert_channels: cfg.alert_channels.map(parseAlertChannel),
		detection_rules: [
			{
				id: 'A1',
				name: 'Large Transfer (>50% balance)',
				enabled: true,
				severity: 'CRITICAL',
				description: 'Flags outgoing transfers exceeding configured % of wallet balance'
			},
			{
				id: 'A3',
				name: 'Rapid Transactions (>5 in 10 min)',
				enabled: true,
				severity: 'WARN',
				description: 'Detects burst of transactions in a short window'
			},
			{
				id: 'A4',
				name: 'New Destination Address',
				enabled: true,
				severity: 'INFO',
				description: 'Alerts on transactions to previously unseen addresses'
			}
		],
		created_at: BigInt(cfg.created_at),
		updated_at: BigInt(cfg.updated_at)
	};
}

// ─── Public Fetch Functions ───────────────────────────────────────────────────

/**
 * Fetch engine health from live canister.
 * Falls back to mock on failure.
 */
export async function fetchHealth(): Promise<CanisterHealth> {
	if (USE_MOCK) return mockFetchHealth();

	try {
		const engine = createEngineActor();
		const raw: RawEngineHealth = await engine.get_health();

		// alert_queue_len is not yet in the engine DID — graceful fallback
		let queueLen = BigInt(0);
		try {
			queueLen = BigInt(await engine.get_alert_queue_len());
		} catch {
			// method not yet exported — use 0
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
	} catch (err) {
		console.warn('[guardian] fetchHealth: canister unreachable, using mock', err);
		return mockFetchHealth();
	}
}

/**
 * Fetch user configs.
 * guardian_config.get_config() returns the caller's own config.
 * list_users() is controller-only and not in public DID.
 * → Falls back to mock gracefully.
 */
export async function fetchUsers(): Promise<UserConfig[]> {
	if (USE_MOCK) return mockFetchUsers();

	try {
		const config = createConfigActor();
		// Try get_config for the anonymous caller (will likely return Err if no config)
		const result = await config.get_config();

		if ('Ok' in result) {
			const cfg = result.Ok as RawGuardianConfig;
			return [rawConfigToUserConfig(cfg)];
		}
		// If anonymous has no config, fall back to mock
		console.info('[guardian] fetchUsers: no config for anonymous principal, using mock');
		return mockFetchUsers();
	} catch (err) {
		console.warn('[guardian] fetchUsers: canister unreachable, using mock', err);
		return mockFetchUsers();
	}
}

/**
 * Fetch alert history.
 * get_alerts() is not yet in the engine DID — always mock gracefully.
 */
export async function fetchAlerts(): Promise<AlertRecord[]> {
	if (USE_MOCK) return mockFetchAlerts();

	try {
		const engine = createEngineActor();
		// Attempt real call — will throw if method doesn't exist
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const raw = await (engine as any).get_alerts();
		// If it works, map to AlertRecord[]
		if (Array.isArray(raw) && raw.length > 0) {
			return raw as AlertRecord[];
		}
		return mockFetchAlerts();
	} catch {
		// Not yet exported — graceful mock
		return mockFetchAlerts();
	}
}

/**
 * Fetch system stats.
 * Derives from engine health + mock alert counts where real data unavailable.
 */
export async function fetchStats(): Promise<SystemStats> {
	if (USE_MOCK) return mockFetchStats();

	try {
		const health = await fetchHealth();
		const alerts = await fetchAlerts();

		return {
			total_users: 1, // Will be accurate when list_users() is available
			total_alerts_queued: Number(health.alert_queue_len),
			alerts_sent: alerts.filter((a) => a.status === 'Sent').length,
			alerts_failed: alerts.filter((a) => a.status === 'Failed').length,
			alerts_pending: alerts.filter((a) => a.status === 'Pending').length,
			uptime_ticks: Number(health.engine.watermark_count) * 30, // est. ticks
			last_sync: health.engine.last_tick
		};
	} catch (err) {
		console.warn('[guardian] fetchStats: error, using mock', err);
		return mockFetchStats();
	}
}

// ─── Mode helpers ────────────────────────────────────────────────────────────

/** Returns true when running against live canisters */
export function isLiveMode(): boolean {
	return !USE_MOCK;
}

/** Returns the current canister IDs in use */
export function getActiveCanisterIds(): { engine: string; config: string } {
	return { ...CANISTER_IDS };
}

/** Returns the current IC host */
export function getActiveHost(): string {
	return IC_HOST;
}
