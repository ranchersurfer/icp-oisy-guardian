<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchStats, isOperatorModeEnabled } from '$lib/canister';
	import { timeAgo } from '$lib/utils';
	import type { SystemStats } from '$lib/types';

	let stats: SystemStats | null = null;
	let loading = true;
	let error = '';
	const operatorMode = isOperatorModeEnabled();

	onMount(async () => {
		if (!operatorMode) {
			loading = false;
			return;
		}

		try {
			stats = await fetchStats();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load operator stats.';
		} finally {
			loading = false;
		}
	});

	$: successRate = stats
		? stats.alerts_sent + stats.alerts_failed > 0
			? ((stats.alerts_sent / (stats.alerts_sent + stats.alerts_failed)) * 100).toFixed(1)
			: '—'
		: '—';
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold text-white">📊 Operator System Stats</h1>
	<p class="text-sm text-gray-500">Operator-only route. Aggregates stay off the normal consumer nav and fail closed on load errors.</p>

	{#if !operatorMode}
		<div class="rounded-2xl border border-amber-400/30 bg-amber-400/10 p-5 text-amber-100">This route is hidden in normal consumer mode.</div>
	{:else if loading}
		<div class="text-gray-500 animate-pulse">Loading operator stats…</div>
	{:else if error}
		<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 p-5 text-rose-100">{error}</div>
	{:else if stats}
		<div class="grid grid-cols-2 gap-4 md:grid-cols-4">
			{#each [
				{ label: 'Visible Users', value: stats.total_users, color: 'text-white' },
				{ label: 'Queued Alerts', value: stats.total_alerts_queued, color: 'text-white' },
				{ label: 'Delivery Rate', value: successRate === '—' ? '—' : `${successRate}%`, color: 'text-green-400' },
				{ label: 'Uptime Ticks', value: stats.uptime_ticks.toLocaleString(), color: 'text-indigo-300' }
			] as card}
				<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
					<div class="mb-1 text-xs text-gray-500">{card.label}</div>
					<div class="text-2xl font-bold {card.color}">{card.value}</div>
				</div>
			{/each}
		</div>

		<div class="grid gap-6 md:grid-cols-2">
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-5">
				<h2 class="mb-4 text-xs font-semibold uppercase tracking-wider text-gray-400">Delivery Status</h2>
				<div class="space-y-3 text-sm">
					<div class="flex justify-between"><span class="text-gray-500">Sent</span><span class="text-green-400">{stats.alerts_sent}</span></div>
					<div class="flex justify-between"><span class="text-gray-500">Pending</span><span class="text-gray-300">{stats.alerts_pending}</span></div>
					<div class="flex justify-between"><span class="text-gray-500">Failed</span><span class="text-red-400">{stats.alerts_failed}</span></div>
				</div>
			</div>

			<div class="rounded-lg border border-gray-800 bg-gray-900 p-5">
				<h2 class="mb-4 text-xs font-semibold uppercase tracking-wider text-gray-400">System Info</h2>
				<div class="space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-gray-500">Last sync signal</span>
						<span class="text-white">{timeAgo(stats.last_sync)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-500">Monitoring ticks</span>
						<span class="text-white">{stats.uptime_ticks.toLocaleString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-500">Data source</span>
						<span class="text-white">Live canister health only</span>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
