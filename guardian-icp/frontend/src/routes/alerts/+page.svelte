<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchAlerts } from '$lib/canister';
	import { authState } from '$lib/auth';
	import type { AlertRecord } from '$lib/types';

	let alerts: AlertRecord[] = [];
	let loading = true;
	let error = '';

	function formatTimestamp(value: bigint): string {
		return new Date(Number(value / BigInt(1_000_000))).toLocaleString();
	}

	function severityClasses(severity: AlertRecord['severity']): string {
		switch (severity) {
			case 'EMERGENCY':
				return 'border-rose-400/30 bg-rose-400/10 text-rose-100';
			case 'CRITICAL':
				return 'border-amber-400/30 bg-amber-400/10 text-amber-100';
			case 'WARN':
				return 'border-yellow-400/30 bg-yellow-400/10 text-yellow-100';
			default:
				return 'border-cyan-400/30 bg-cyan-400/10 text-cyan-100';
		}
	}

	onMount(async () => {
		try {
			alerts = await fetchAlerts(20);
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load your alerts.';
		} finally {
			loading = false;
		}
	});
</script>

<div class="space-y-6">
	<div class="space-y-2">
		<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">Alerts</div>
		<h1 class="text-4xl font-semibold text-white">Your Guardian alert history</h1>
		<p class="max-w-3xl text-slate-300">
			These alerts belong only to your connected Internet Identity
			{#if $authState.principal}
				<span class="font-mono text-slate-200">{$authState.principal}</span>
			{/if}.
			Guardian shows only your own alert history here.
		</p>
	</div>

	{#if loading}
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6 text-slate-300">Loading your alert history…</div>
	{:else if error}
		<div class="rounded-3xl border border-rose-400/30 bg-rose-400/10 p-6 text-rose-100">{error}</div>
	{:else if alerts.length === 0}
		<div class="rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
			<h2 class="text-xl font-semibold text-white">No alerts yet</h2>
			<p class="mt-3 max-w-2xl text-sm text-slate-300">
				You’re all clear for now. When Guardian detects suspicious activity for your connected identity, your personal alert history will appear here.
			</p>
		</div>
	{:else}
		<div class="grid gap-4">
			{#each alerts as alert}
				<article class="rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
					<div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
						<div>
							<div class={`inline-flex rounded-full border px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em] ${severityClasses(alert.severity)}`}>
								{alert.severity}
							</div>
							<div class="mt-3 text-lg font-semibold text-white">{alert.events_summary}</div>
							<div class="mt-2 text-sm text-slate-400">Detected {formatTimestamp(alert.timestamp)}</div>
						</div>
						<div class="text-sm text-slate-400">Score {alert.severity_score}</div>
					</div>

					<div class="mt-5 grid gap-4 lg:grid-cols-[1fr_1fr]">
						<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
							<div class="text-sm text-slate-400">Rules triggered</div>
							<ul class="mt-3 space-y-2 text-sm text-slate-200">
								{#each alert.rules_triggered as rule}
									<li>• {rule}</li>
								{/each}
							</ul>
						</div>

						<div class="rounded-2xl border border-cyan-400/20 bg-cyan-400/10 p-4">
							<div class="text-sm text-cyan-100">Recommended action</div>
							<p class="mt-3 text-sm text-cyan-50">{alert.recommended_action}</p>
						</div>
					</div>
				</article>
			{/each}
		</div>
	{/if}
</div>
