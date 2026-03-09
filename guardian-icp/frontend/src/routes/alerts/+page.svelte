<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchAlerts, isOperatorModeEnabled } from '$lib/canister';
	import type { AlertRecord } from '$lib/types';

	let alerts: AlertRecord[] = [];
	let loading = true;
	let error = '';
	const operatorMode = isOperatorModeEnabled();

	onMount(async () => {
		if (!operatorMode) {
			loading = false;
			return;
		}

		try {
			alerts = await fetchAlerts();
			if (alerts.length === 0) {
				error = 'Broad alert history is intentionally unavailable in the consumer frontend until the backend exposes an explicitly caller-scoped or controller-scoped method.';
			}
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load alert history.';
		} finally {
			loading = false;
		}
	});
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold text-white">🚨 Operator Alert History</h1>
	<p class="text-sm text-gray-500">This path is now gated. It does not show mock multi-user history and stays disabled until the source API is explicitly scoped and reviewed.</p>

	{#if !operatorMode}
		<div class="rounded-2xl border border-amber-400/30 bg-amber-400/10 p-5 text-amber-100">
			This route is hidden in normal consumer mode.
		</div>
	{:else if loading}
		<div class="text-gray-500 animate-pulse">Checking alert history availability…</div>
	{:else}
		<div class="rounded-2xl border border-white/10 bg-slate-900/70 p-6">
			<div class="text-lg font-semibold text-white">Alert history gated</div>
			<p class="mt-3 max-w-3xl text-sm text-slate-300">
				{error || 'The frontend is intentionally not rendering broad alert history here. Expose a reviewed get_my_alerts() or controller-only endpoint before re-enabling this screen.'}
			</p>
			<ul class="mt-4 list-disc space-y-2 pl-5 text-sm text-slate-400">
				<li>No mock fallback is used on this private route.</li>
				<li>No other users’ alert summaries are rendered.</li>
				<li>Preferred follow-up: add a caller-scoped <code>get_my_alerts()</code> method for consumer use.</li>
			</ul>
		</div>
	{/if}
</div>
