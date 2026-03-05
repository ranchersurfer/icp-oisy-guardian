<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchHealth } from '$lib/canister';
	import { formatCycles, formatTimestamp, timeAgo } from '$lib/utils';
	import type { CanisterHealth } from '$lib/types';

	let health: CanisterHealth | null = null;
	let loading = true;
	let error = '';
	let refreshInterval: ReturnType<typeof setInterval>;

	async function load() {
		try {
			health = await fetchHealth();
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		load();
		refreshInterval = setInterval(load, 30_000);
		return () => clearInterval(refreshInterval);
	});
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold text-white">🏥 Health Status</h1>
		<button
			on:click={load}
			class="px-3 py-1.5 bg-gray-800 hover:bg-gray-700 text-sm rounded border border-gray-700"
		>
			↻ Refresh
		</button>
	</div>

	{#if loading}
		<div class="text-gray-500 animate-pulse">Loading canister health…</div>
	{:else if error}
		<div class="bg-red-950 border border-red-800 text-red-300 p-4 rounded">{error}</div>
	{:else if health}
		<!-- Engine Status -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
			<div class="bg-gray-900 border border-gray-800 rounded-lg p-4">
				<div class="text-gray-500 text-xs mb-1">Status</div>
				<div class="flex items-center gap-2 text-lg font-bold">
					{#if health.engine.is_running}
						<span class="w-3 h-3 bg-green-500 rounded-full inline-block"></span>
						<span class="text-green-400">Running</span>
					{:else}
						<span class="w-3 h-3 bg-red-500 rounded-full inline-block"></span>
						<span class="text-red-400">Stopped</span>
					{/if}
				</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-4">
				<div class="text-gray-500 text-xs mb-1">Cycle Balance</div>
				<div class="text-lg font-bold text-white">{formatCycles(health.engine.cycle_balance)}</div>
				<div class="text-xs text-gray-500 mt-1">
					{#if health.engine.cycle_balance > BigInt('500000000000')}
						<span class="text-green-400">✓ Above safety threshold</span>
					{:else}
						<span class="text-red-400">⚠ Below 500B threshold</span>
					{/if}
				</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-4">
				<div class="text-gray-500 text-xs mb-1">Watermarks Tracked</div>
				<div class="text-lg font-bold text-white">{health.engine.watermark_count.toString()}</div>
				<div class="text-xs text-gray-500 mt-1">Active user×chain monitors</div>
			</div>

			<div class="bg-gray-900 border border-gray-800 rounded-lg p-4">
				<div class="text-gray-500 text-xs mb-1">Alert Queue</div>
				<div class="text-lg font-bold {health.alert_queue_len > BigInt(10) ? 'text-yellow-400' : 'text-white'}">
					{health.alert_queue_len.toString()}
				</div>
				<div class="text-xs text-gray-500 mt-1">Pending delivery</div>
			</div>
		</div>

		<!-- Timer info -->
		<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
			<h2 class="text-gray-400 text-sm font-semibold mb-4 uppercase tracking-wider">Timer Details</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
				<div>
					<span class="text-gray-500">Last tick:</span>
					<span class="text-white ml-2">{timeAgo(health.engine.last_tick)}</span>
					<span class="text-gray-600 ml-2 text-xs">({formatTimestamp(health.engine.last_tick)})</span>
				</div>
				<div>
					<span class="text-gray-500">Config canister:</span>
					<span class="text-white ml-2 font-mono text-xs">{health.config_canister_id ?? '—'}</span>
				</div>
			</div>
		</div>

		<!-- Canister IDs -->
		<div class="bg-gray-900 border border-gray-800 rounded-lg p-5">
			<h2 class="text-gray-400 text-sm font-semibold mb-4 uppercase tracking-wider">Local Canister IDs (devnet)</h2>
			<div class="space-y-2 text-sm">
				<div class="flex items-center gap-3">
					<span class="text-gray-500 w-36">guardian_config:</span>
					<code class="text-indigo-300">uxrrr-q7777-77774-qaaaq-cai</code>
				</div>
				<div class="flex items-center gap-3">
					<span class="text-gray-500 w-36">guardian_engine:</span>
					<code class="text-indigo-300">u6s2n-gx777-77774-qaaba-cai</code>
				</div>
			</div>
		</div>
	{/if}
</div>
