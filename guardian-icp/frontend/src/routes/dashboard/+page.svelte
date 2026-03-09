<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authState } from '$lib/auth';
	import { getMyConfig } from '$lib/canister';
	import {
		formatPercent,
		formatRapidWindow,
		mapConfigResultToView,
		shortenPrincipal
	} from '$lib/guardian';
	import { maskEmail, maskUrl } from '$lib/utils';
	import type { GuardianConfigView } from '$lib/types';

	let loading = true;
	let error = '';
	let view: GuardianConfigView | null = null;

	function formatDate(value: bigint): string {
		const millis = Number(value / BigInt(1_000_000));
		return new Date(millis).toLocaleString();
	}

	function maskSavedChannel(raw: string): string {
		if (raw.startsWith('email;address=')) return maskEmail(raw.replace('email;address=', '').split(';')[0] ?? '');
		if (raw.startsWith('discord;url=')) return maskUrl(raw.replace('discord;url=', '').split(';')[0] ?? '');
		if (raw.startsWith('slack;url=')) return maskUrl(raw.replace('slack;url=', '').split(';')[0] ?? '');
		if (raw.startsWith('webhook;url=')) return maskUrl(raw.replace('webhook;url=', '').split(';')[0] ?? '');
		return 'Hidden destination';
	}

	async function loadDashboard() {
		if (!$authState.isAuthenticated) {
			await goto('/');
			return;
		}
		loading = true;
		error = '';
		try {
			const result = await getMyConfig();
			view = mapConfigResultToView(result);
			if (!view) {
				await goto('/onboarding');
				return;
			}
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load Guardian dashboard.';
		} finally {
			loading = false;
		}
	}

	onMount(loadDashboard);
</script>

<div class="space-y-8">
	<div class="flex flex-col gap-2 md:flex-row md:items-end md:justify-between">
		<div>
			<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">Dashboard</div>
			<h1 class="text-4xl font-semibold text-white">Your Guardian protection state</h1>
			<p class="mt-2 max-w-2xl text-slate-300">Real saved state from the live config canister, tied to your connected Internet Identity principal.</p>
		</div>
		<a href="/onboarding" class="inline-flex rounded-full border border-white/15 px-5 py-3 text-sm text-slate-200 transition hover:bg-white/10">
			Future quick action: edit settings
		</a>
	</div>

	{#if loading}
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6 text-slate-300">Loading your live Guardian config…</div>
	{:else if error}
		<div class="rounded-3xl border border-rose-400/30 bg-rose-400/10 p-6 text-rose-100">{error}</div>
	{:else if view}
		<div class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
			<div class="rounded-3xl border border-emerald-400/20 bg-emerald-400/10 p-6">
				<div class="text-sm text-emerald-200">Guardian status</div>
				<div class="mt-2 text-2xl font-semibold text-white">Guardian active</div>
				<div class="mt-2 text-sm text-emerald-100">Monitoring preferences saved on-chain.</div>
			</div>
			<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
				<div class="text-sm text-slate-400">Active preset</div>
				<div class="mt-2 text-2xl font-semibold text-white">{view.preset ? view.preset[0].toUpperCase() + view.preset.slice(1) : 'Custom'}</div>
			</div>
			<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
				<div class="text-sm text-slate-400">Owner principal</div>
				<div class="mt-2 text-2xl font-semibold text-white">{shortenPrincipal(view.owner)}</div>
				<div class="mt-2 break-all font-mono text-xs text-slate-500">{shortenPrincipal(view.owner, 12, 8)}</div>
			</div>
			<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
				<div class="text-sm text-slate-400">Last updated</div>
				<div class="mt-2 text-2xl font-semibold text-white">{formatDate(view.lastUpdated)}</div>
			</div>
		</div>

		<div class="grid gap-6 lg:grid-cols-[1.1fr_0.9fr]">
			<div class="rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
				<h2 class="text-xl font-semibold text-white">Protection summary</h2>
				<div class="mt-5 grid gap-4 md:grid-cols-2">
					<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
						<div class="text-sm text-slate-400">Large transfer trigger</div>
						<div class="mt-2 text-xl font-semibold text-white">{formatPercent(view.largeTransferPct)}</div>
					</div>
					<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
						<div class="text-sm text-slate-400">Rapid tx rule</div>
						<div class="mt-2 text-xl font-semibold text-white">{view.rapidTxCount} tx in {formatRapidWindow(view.rapidTxWindowSecs)}</div>
					</div>
					<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
						<div class="text-sm text-slate-400">New-address alerts</div>
						<div class="mt-2 text-xl font-semibold text-white">{view.newAddressAlert ? 'On' : 'Off'}</div>
					</div>
					<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
						<div class="text-sm text-slate-400">Monitored chains</div>
						<div class="mt-2 text-xl font-semibold text-white">{view.monitoredChains.join(', ')}</div>
					</div>
				</div>

				<div class="mt-6 rounded-2xl border border-white/10 bg-white/5 p-4">
					<div class="text-sm text-slate-400">Configured alert destinations</div>
					{#if view.alertChannels.length > 0}
						<div class="mt-3 space-y-2 text-sm text-slate-200">
							{#each view.alertChannels as channel}
								<div class="rounded-xl border border-white/10 bg-slate-950/40 px-3 py-2 font-mono">{maskSavedChannel(channel)}</div>
							{/each}
						</div>
						<p class="mt-3 text-xs text-amber-200">Destinations are masked in the UI. Future privacy work: store destination secrets encrypted before any vetKeys-backed rollout.</p>
					{:else}
						<p class="mt-3 text-sm text-slate-400">No alert destination configured yet.</p>
					{/if}
				</div>
			</div>

			<div class="rounded-[1.75rem] border border-cyan-400/20 bg-cyan-400/10 p-6">
				<h2 class="text-xl font-semibold text-white">What is live right now</h2>
				<ul class="mt-4 space-y-3 text-sm text-cyan-50">
					<li>• Internet Identity auth session is being used for this principal</li>
					<li>• <code>guardian_config.get_config()</code> loaded your saved state</li>
					<li>• Onboarding saves through <code>set_config()</code> and reads back after write</li>
					<li>• Guardian remains advisory only — no automatic fund movement</li>
				</ul>
			</div>
		</div>
	{/if}
</div>
