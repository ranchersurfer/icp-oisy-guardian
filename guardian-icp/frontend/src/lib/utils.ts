import type { AlertSeverity, AlertStatus } from './types';

export function formatCycles(cycles: bigint): string {
	const t = Number(cycles) / 1e12;
	return `${t.toFixed(2)}T`;
}

export function formatTimestamp(ns: bigint): string {
	const ms = Number(ns / BigInt(1_000_000));
	return new Date(ms).toLocaleString();
}

export function timeAgo(ns: bigint): string {
	const ms = Number(ns / BigInt(1_000_000));
	const diff = Date.now() - ms;
	if (diff < 60_000) return `${Math.floor(diff / 1000)}s ago`;
	if (diff < 3_600_000) return `${Math.floor(diff / 60_000)}m ago`;
	if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)}h ago`;
	return `${Math.floor(diff / 86_400_000)}d ago`;
}

export function truncatePrincipal(p: string): string {
	if (p.length <= 16) return p;
	return `${p.slice(0, 8)}…${p.slice(-6)}`;
}

export function severityColor(s: AlertSeverity): string {
	switch (s) {
		case 'INFO': return 'text-blue-400';
		case 'WARN': return 'text-yellow-400';
		case 'CRITICAL': return 'text-orange-400';
		case 'EMERGENCY': return 'text-red-500';
	}
}

export function severityBadge(s: AlertSeverity): string {
	switch (s) {
		case 'INFO': return 'bg-blue-900 text-blue-300';
		case 'WARN': return 'bg-yellow-900 text-yellow-300';
		case 'CRITICAL': return 'bg-orange-900 text-orange-300';
		case 'EMERGENCY': return 'bg-red-900 text-red-300';
	}
}

export function statusBadge(s: AlertStatus): string {
	switch (s) {
		case 'Sent': return 'bg-green-900 text-green-300';
		case 'Failed': return 'bg-red-900 text-red-300';
		case 'Pending': return 'bg-gray-700 text-gray-300';
	}
}
