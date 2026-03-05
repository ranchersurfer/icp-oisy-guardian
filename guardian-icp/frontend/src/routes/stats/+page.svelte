<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchStats, fetchAlerts } from '$lib/mock';
	import { timeAgo } from '$lib/utils';
	import type { SystemStats, AlertRecord } from '$lib/types';

	let stats: SystemStats | null = null;
	let alerts: AlertRecord[] = [];
	let loading = true;

	onMount(async () => {
		[stats, alerts] = await Promise.all([fetchStats(), fetchAlerts()]);
		loading = false;
	});

	$: successRate = stats
		? stats.alerts_sent + stats.alerts_failed > 0
			? ((stats.alerts_sent / (stats.alerts_sent + stats.alerts_failed)) * 100).toFixed(1)
			: '—'
		: '—';

	$: chainBreakdown = alerts.reduce((acc, a) => {
		acc[a.chain] = (acc[a.chain] ?? 0) + 1;
		return acc;
	}, {} as Record<string, number>);

	$: severityBreakdown = alerts.reduce((acc, a) => {
		acc[a.severity] = (acc[a.severity] ?? 0) + 1;
		return acc;
	}, {} as Record<string, number>);
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold text-white">📊 System Stats</h1>

	{#if loading}
		<div class="text-gray-500 animate-pulse">Loading stats…</div>
	{:else if stats}
		<!-- Stat cards -->
		<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
			{#each [
				{ label: 'Total Users', value: stats.total_users, color: 'text-white' },
				{ label: 'Total Alerts', value: stats.total_alerts_queued, color: 'text-white' },
				{ label: 'Success Rate', value: successRate + '%', color: 'text-green-400' },
				{ label: 'Uptime Ticks', value: stats.uptime_ticks.toLocaleString(), color: 'text-indigo-300' }
			] as card}
				<div class="bg-gray-900 border border-gray-800 rounded-lg p-4">
					<div class="text-gray-500 text-xs mb-1">{card.label}</div>
					<div class="text-2xl font-bold {card.color}">{card.value}</div>
				</div>
			{/each}
		</div>

		<!-- Alert status breakdown -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
				<h2 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-4">Alert Delivery Status</h2>
				<div class="space-y-3">
					{#each [
						{ label: 'Sent', count: stats.alerts_sent, color: 'bg-green-500', badge: 'text-green-400' },
						{ label: 'Pending', count: stats.alerts_pending, color: 'bg-gray-500', badge: 'text-gray-400' },
						{ label: 'Failed', count: stats.alerts_failed, color: 'bg-red-500', badge: 'text-red-400' }
					] as row}
						<div class="flex items-center gap-3">
							<div class="w-24 text-sm text-gray-400">{row.label}</div>
							<div class="flex-1 bg-gray-800 rounded-full h-2">
								<div
									class="h-2 rounded-full {row.color}"
									style="width: {stats.total_alerts_queued > 0 ? (row.count / stats.total_alerts_queued * 100).toFixed(1) : 0}%"
								></div>
							</div>
							<div class="w-10 text-right text-sm {row.badge}">{row.count}</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
				<h2 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-4">Alerts by Chain</h2>
				<div class="space-y-3">
					{#each Object.entries(chainBreakdown) as [chain, count]}
						<div class="flex items-center gap-3">
							<div class="w-24 text-sm text-gray-400">{chain}</div>
							<div class="flex-1 bg-gray-800 rounded-full h-2">
								<div
									class="h-2 rounded-full bg-indigo-500"
									style="width: {(count / alerts.length * 100).toFixed(1)}%"
								></div>
							</div>
							<div class="w-10 text-right text-sm text-indigo-300">{count}</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
				<h2 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-4">Alerts by Severity</h2>
				<div class="space-y-3">
					{#each ['EMERGENCY', 'CRITICAL', 'WARN', 'INFO'] as sev}
						{@const count = severityBreakdown[sev] ?? 0}
						{@const colors: Record<string, string> = { EMERGENCY: 'bg-red-500', CRITICAL: 'bg-orange-500', WARN: 'bg-yellow-500', INFO: 'bg-blue-500' }}
						{@const textColors: Record<string, string> = { EMERGENCY: 'text-red-400', CRITICAL: 'text-orange-400', WARN: 'text-yellow-400', INFO: 'text-blue-400' }}
						<div class="flex items-center gap-3">
							<div class="w-24 text-sm text-gray-400">{sev}</div>
							<div class="flex-1 bg-gray-800 rounded-full h-2">
								<div
									class="h-2 rounded-full {colors[sev]}"
									style="width: {alerts.length > 0 ? (count / alerts.length * 100).toFixed(1) : 0}%"
								></div>
							</div>
							<div class="w-10 text-right text-sm {textColors[sev]}">{count}</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
				<h2 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-4">System Info</h2>
				<div class="space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-gray-500">Last config sync</span>
						<span class="text-white">{timeAgo(stats.last_sync)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-500">Monitoring ticks</span>
						<span class="text-white">{stats.uptime_ticks.toLocaleString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-500">Tick interval</span>
						<span class="text-white">30s (monitoring), 60s (delivery), 300s (config sync)</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-500">Data mode</span>
						<span class="text-yellow-400">Mock (dev)</span>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
