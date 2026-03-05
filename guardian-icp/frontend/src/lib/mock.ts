// Guardian ICP Admin Dashboard — Mock Data Layer
// Used during development and when canister is unreachable.
// In production, replace with actual @dfinity/agent calls.

import type {
	AlertRecord,
	CanisterHealth,
	SystemStats,
	UserConfig
} from './types';

const NOW_NS = BigInt(Date.now()) * BigInt(1_000_000);

export const mockHealth: CanisterHealth = {
	engine: {
		cycle_balance: BigInt('1_200_000_000_000'.replace(/_/g, '')),
		last_tick: NOW_NS - BigInt(15_000_000_000),
		is_running: true,
		watermark_count: BigInt(12)
	},
	config_canister_id: 'uxrrr-q7777-77774-qaaaq-cai',
	alert_queue_len: BigInt(3)
};

export const mockUsers: UserConfig[] = [
	{
		principal: '5lok2-xvf24-onx6j-zldh6-ss6u5-xinwf-5m7u2-gzaiq-lfdpo-ivagh-aae',
		alert_threshold: 7,
		alert_channels: [
			{ type: 'Discord', target: 'https://discord.com/api/webhooks/123/**REDACTED**' },
			{ type: 'Email', target: 'alice@example.com' }
		],
		detection_rules: [
			{ id: 'A1', name: 'Large Transfer (>50% balance)', enabled: true, severity: 'CRITICAL', description: 'Flags outgoing transfers exceeding 50% of wallet balance' },
			{ id: 'A3', name: 'Rapid Transactions (>5 in 10 min)', enabled: true, severity: 'WARN', description: 'Detects burst of transactions in a short window' },
			{ id: 'A4', name: 'New Destination Address', enabled: false, severity: 'INFO', description: 'Alerts on transactions to previously unseen addresses' }
		],
		created_at: NOW_NS - BigInt(86_400_000_000_000),
		updated_at: NOW_NS - BigInt(3_600_000_000_000)
	},
	{
		principal: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
		alert_threshold: 3,
		alert_channels: [
			{ type: 'Slack', target: 'https://hooks.slack.com/services/T00/**REDACTED**' }
		],
		detection_rules: [
			{ id: 'A1', name: 'Large Transfer (>50% balance)', enabled: true, severity: 'CRITICAL', description: 'Flags outgoing transfers exceeding 50% of wallet balance' },
			{ id: 'A3', name: 'Rapid Transactions (>5 in 10 min)', enabled: false, severity: 'WARN', description: 'Detects burst of transactions in a short window' },
			{ id: 'A4', name: 'New Destination Address', enabled: true, severity: 'INFO', description: 'Alerts on transactions to previously unseen addresses' }
		],
		created_at: NOW_NS - BigInt(172_800_000_000_000),
		updated_at: NOW_NS - BigInt(7_200_000_000_000)
	},
	{
		principal: 'aaaaa-aa',
		alert_threshold: 15,
		alert_channels: [
			{ type: 'Webhook', target: 'https://my-server.example.com/guardian-hook' }
		],
		detection_rules: [
			{ id: 'A1', name: 'Large Transfer (>50% balance)', enabled: true, severity: 'EMERGENCY', description: 'Flags outgoing transfers exceeding 50% of wallet balance' },
			{ id: 'A3', name: 'Rapid Transactions (>5 in 10 min)', enabled: true, severity: 'CRITICAL', description: 'Detects burst of transactions in a short window' },
			{ id: 'A4', name: 'New Destination Address', enabled: true, severity: 'WARN', description: 'Alerts on transactions to previously unseen addresses' }
		],
		created_at: NOW_NS - BigInt(259_200_000_000_000),
		updated_at: NOW_NS - BigInt(1_800_000_000_000)
	}
];

function randomTs(offsetMs: number): bigint {
	return NOW_NS - BigInt(offsetMs) * BigInt(1_000_000);
}

export const mockAlerts: AlertRecord[] = [
	{
		alert_id: 'a-001',
		timestamp: randomTs(300_000),
		user: mockUsers[0].principal,
		rules_triggered: ['A1'],
		severity: 'CRITICAL',
		severity_score: 12,
		status: 'Sent',
		events_summary: 'Outgoing 450 ICP (87% of balance) to new address',
		recommended_action: 'Verify transaction in OISY',
		chain: 'ICP'
	},
	{
		alert_id: 'a-002',
		timestamp: randomTs(600_000),
		user: mockUsers[1].principal,
		rules_triggered: ['A3', 'A4'],
		severity: 'WARN',
		severity_score: 6,
		status: 'Sent',
		events_summary: '7 transactions in 8 minutes, 3 to new addresses',
		recommended_action: 'Review recent transaction history',
		chain: 'CkBTC'
	},
	{
		alert_id: 'a-003',
		timestamp: randomTs(1_200_000),
		user: mockUsers[2].principal,
		rules_triggered: ['A1'],
		severity: 'EMERGENCY',
		severity_score: 18,
		status: 'Failed',
		events_summary: 'Transfer of 2.4 ckETH (99% of balance)',
		recommended_action: 'Immediate review required — delivery failed, re-queue pending',
		chain: 'CkETH'
	},
	{
		alert_id: 'a-004',
		timestamp: randomTs(3_600_000),
		user: mockUsers[0].principal,
		rules_triggered: ['A4'],
		severity: 'INFO',
		severity_score: 2,
		status: 'Sent',
		events_summary: 'Transaction to new address 3abc...',
		recommended_action: 'No action needed if expected',
		chain: 'ICP'
	},
	{
		alert_id: 'a-005',
		timestamp: randomTs(7_200_000),
		user: mockUsers[1].principal,
		rules_triggered: ['A1', 'A3'],
		severity: 'CRITICAL',
		severity_score: 14,
		status: 'Pending',
		events_summary: 'Large outgoing sweep + 9 rapid txs in 6 minutes',
		recommended_action: 'Suspend wallet if unrecognised activity',
		chain: 'ICP'
	},
	...Array.from({ length: 20 }, (_, i) => ({
		alert_id: `a-${String(i + 6).padStart(3, '0')}`,
		timestamp: randomTs((i + 3) * 3_600_000),
		user: mockUsers[i % 3].principal,
		rules_triggered: i % 2 === 0 ? ['A1'] : ['A3'],
		severity: (['INFO', 'WARN', 'CRITICAL', 'EMERGENCY'] as const)[i % 4],
		severity_score: (i % 4 + 1) * 3,
		status: (['Sent', 'Sent', 'Failed', 'Pending'] as const)[i % 4],
		events_summary: `Auto-generated mock alert #${i + 6}`,
		recommended_action: 'Review wallet activity',
		chain: (['ICP', 'CkBTC', 'CkETH'] as const)[i % 3]
	}))
];

export const mockStats: SystemStats = {
	total_users: mockUsers.length,
	total_alerts_queued: mockAlerts.length,
	alerts_sent: mockAlerts.filter((a) => a.status === 'Sent').length,
	alerts_failed: mockAlerts.filter((a) => a.status === 'Failed').length,
	alerts_pending: mockAlerts.filter((a) => a.status === 'Pending').length,
	uptime_ticks: 4320,
	last_sync: NOW_NS - BigInt(5_000_000_000)
};

// Simulate async canister calls
export async function fetchHealth(): Promise<CanisterHealth> {
	await delay(200);
	return { ...mockHealth };
}

export async function fetchAlerts(): Promise<AlertRecord[]> {
	await delay(300);
	return [...mockAlerts];
}

export async function fetchUsers(): Promise<UserConfig[]> {
	await delay(250);
	return [...mockUsers];
}

export async function fetchStats(): Promise<SystemStats> {
	await delay(150);
	return { ...mockStats };
}

function delay(ms: number): Promise<void> {
	return new Promise((r) => setTimeout(r, ms));
}
