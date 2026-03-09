import { writable } from 'svelte/store';
import { Principal } from '@dfinity/principal';
import type {
	GuardianConfigRecord,
	GuardianConfigResult,
	GuardianConfigView,
	GuardianPreset,
	GuardianPresetId,
	GuardianWriteResult
} from './types.js';

export const SUPPORTED_CHAINS = ['ICP', 'ckBTC', 'ckETH'];
export const DEFAULT_MONITORED_CHAINS = [...SUPPORTED_CHAINS];

export const PRESETS: GuardianPreset[] = [
	{
		id: 'safe',
		name: 'Safe',
		tagline: 'Low-noise protection for normal wallet use.',
		description: 'Fewer alerts and more tolerance for everyday activity.',
		config: {
			large_transfer_pct: 0.75,
			daily_outflow_pct: 0.5,
			rapid_tx_count: 8,
			rapid_tx_window_secs: 600,
			new_address_alert: true,
			alert_threshold: 7,
			emergency_threshold: 15,
			monitored_chains: DEFAULT_MONITORED_CHAINS
		}
	},
	{
		id: 'balanced',
		name: 'Balanced',
		tagline: 'Best default tradeoff between signal and noise.',
		description: 'Recommended for most users who want practical protection without too many alerts.',
		recommended: true,
		config: {
			large_transfer_pct: 0.5,
			daily_outflow_pct: 0.5,
			rapid_tx_count: 5,
			rapid_tx_window_secs: 600,
			new_address_alert: true,
			alert_threshold: 7,
			emergency_threshold: 15,
			monitored_chains: DEFAULT_MONITORED_CHAINS
		}
	},
	{
		id: 'aggressive',
		name: 'Aggressive',
		tagline: 'Higher sensitivity for higher-risk or power users.',
		description: 'More alerts, faster triggers, and tighter thresholds.',
		config: {
			large_transfer_pct: 0.25,
			daily_outflow_pct: 0.5,
			rapid_tx_count: 3,
			rapid_tx_window_secs: 900,
			new_address_alert: true,
			alert_threshold: 3,
			emergency_threshold: 15,
			monitored_chains: DEFAULT_MONITORED_CHAINS
		}
	}
];

export const selectedPreset = writable<GuardianPresetId>('balanced');

export function getPreset(id: GuardianPresetId): GuardianPreset {
	const preset = PRESETS.find((item) => item.id === id);
	if (!preset) throw new Error(`Unknown preset: ${id}`);
	return preset;
}

export function detectPreset(config: GuardianConfigRecord): GuardianPresetId | null {
	for (const preset of PRESETS) {
		const candidate = preset.config;
		if (
			config.large_transfer_pct === candidate.large_transfer_pct &&
			config.rapid_tx_count === candidate.rapid_tx_count &&
			Number(config.rapid_tx_window_secs) === candidate.rapid_tx_window_secs &&
			config.new_address_alert === candidate.new_address_alert &&
			config.alert_threshold === candidate.alert_threshold &&
			config.emergency_threshold === candidate.emergency_threshold
		) {
			return preset.id;
		}
	}
	return null;
}

export function shortenPrincipal(value: string, head = 6, tail = 5): string {
	if (value.length <= head + tail + 1) return value;
	return `${value.slice(0, head)}…${value.slice(-tail)}`;
}

export function formatPercent(value: number): string {
	return `${Math.round(value * 100)}%`;
}

export function formatRapidWindow(seconds: number | bigint): string {
	const numeric = Number(seconds);
	const minutes = Math.round(numeric / 60);
	return `${minutes} min`;
}

export function buildConfigForPreset(presetId: GuardianPresetId, owner: Principal, existing?: GuardianConfigRecord): GuardianConfigRecord {
	const preset = getPreset(presetId);
	const now = BigInt(Date.now()) * BigInt(1_000_000);

	return {
		owner,
		created_at: existing?.created_at ?? now,
		updated_at: existing?.updated_at ?? now,
		monitored_chains: [...preset.config.monitored_chains],
		large_transfer_pct: preset.config.large_transfer_pct,
		daily_outflow_pct: preset.config.daily_outflow_pct,
		rapid_tx_count: preset.config.rapid_tx_count,
		rapid_tx_window_secs: BigInt(preset.config.rapid_tx_window_secs),
		new_address_alert: preset.config.new_address_alert,
		alert_threshold: preset.config.alert_threshold,
		emergency_threshold: preset.config.emergency_threshold,
		alert_channels: existing?.alert_channels ?? [],
		allowlisted_addresses: existing?.allowlisted_addresses ?? []
	};
}

export function mapConfigResultToView(result: GuardianConfigResult): GuardianConfigView | null {
	if ('Err' in result) return null;
	const config = result.Ok;
	return {
		preset: detectPreset(config),
		status: 'active',
		owner: config.owner.toString(),
		lastUpdated: config.updated_at,
		createdAt: config.created_at,
		monitoredChains: config.monitored_chains,
		largeTransferPct: config.large_transfer_pct,
		rapidTxCount: config.rapid_tx_count,
		rapidTxWindowSecs: Number(config.rapid_tx_window_secs),
		newAddressAlert: config.new_address_alert,
		alertThreshold: config.alert_threshold,
		emergencyThreshold: config.emergency_threshold,
		alertChannels: config.alert_channels,
		allowlistedAddresses: config.allowlisted_addresses
	};
}

export function isWriteSuccess(result: GuardianWriteResult): boolean {
	return 'Ok' in result;
}
