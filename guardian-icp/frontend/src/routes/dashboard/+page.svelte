<script lang="ts">
	import { goto } from '$app/navigation';
	import { authState } from '$lib/auth';
	import { getEmailVerificationStatus, getMyConfig } from '$lib/canister';
	import {
		formatPercent,
		formatRapidWindow,
		mapConfigResultToView,
		shortenPrincipal
	} from '$lib/guardian';
	import { maskEmail, maskUrl } from '$lib/utils';
	import type { EmailVerificationStatus, GuardianConfigView } from '$lib/types';

	let loading = true;
	let error = '';
	let view: GuardianConfigView | null = null;
	let emailStatus: EmailVerificationStatus | null = null;

	function formatDate(value: bigint): string {
		const millis = Number(value / BigInt(1_000_000));
		return new Date(millis).toLocaleString();
	}

	function optText(value: [] | [string] | undefined): string {
		return value?.[0] ?? '';
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
			const [configResult, emailResult] = await Promise.all([getMyConfig(), getEmailVerificationStatus()]);
			view = mapConfigResultToView(configResult);
			if (!view) {
				await goto('/onboarding');
				return;
			}
			if ('Ok' in emailResult) emailStatus = emailResult.Ok;
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load Guardian dashboard.';
		} finally {
			loading = false;
		}
	}

	loadDashboard();
</script>

<div class="space-y-6 sm:space-y-8">
	<div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
		<div>
			<div class="guardian-kicker">Dashboard</div>
			<h1 class="guardian-section-title">Your Guardian protection state</h1>
			<p class="mt-3 max-w-3xl text-sm leading-6 text-slate-300 sm:text-base">A cleaner personal view of what’s active right now, when it was updated, and which settings are shaping your alerts.</p>
		</div>
		<div class="flex flex-col gap-3 sm:flex-row">
			<a href="/settings" class="guardian-button-primary min-h-12">Edit settings</a>
			<a href="/alerts" class="guardian-button-secondary min-h-12">View alerts</a>
			<a href="/onboarding?mode=edit" class="guardian-button-secondary min-h-12">Re-run onboarding</a>
		</div>
	</div>

	{#if loading}
		<div class="guardian-card text-slate-300">Loading your live Guardian config…</div>
	{:else if error}
		<div class="rounded-3xl border border-rose-400/30 bg-rose-400/10 p-6 text-rose-100">{error}</div>
	{:else if view}
		<div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
			<div class="rounded-[1.75rem] border border-emerald-400/20 bg-emerald-400/10 p-5 sm:p-6">
				<div class="text-sm text-emerald-200">Protection status</div>
				<div class="mt-2 text-2xl font-semibold text-white">Guardian active</div>
				<div class="mt-2 text-sm text-emerald-100">Monitoring preferences are saved on-chain and linked to your identity.</div>
			</div>
			<div class="guardian-card">
				<div class="text-sm text-slate-400">Active preset</div>
				<div class="mt-2 text-2xl font-semibold text-white">{view.preset ? view.preset[0].toUpperCase() + view.preset.slice(1) : 'Custom'}</div>
				<div class="mt-2 text-sm text-slate-400">Switch presets or fine-tune thresholds from settings.</div>
			</div>
			<div class="guardian-card">
				<div class="text-sm text-slate-400">Owner principal</div>
				<div class="mt-2 text-2xl font-semibold text-white">{shortenPrincipal(view.owner)}</div>
				<div class="mt-2 break-all font-mono text-xs text-slate-500">{view.owner}</div>
			</div>
			<div class="guardian-card">
				<div class="text-sm text-slate-400">Last updated</div>
				<div class="mt-2 text-xl font-semibold leading-snug text-white">{formatDate(view.lastUpdated)}</div>
				<div class="mt-2 text-sm text-slate-400">This reflects the latest successful read from the live config canister.</div>
			</div>
		</div>

		<div class="grid gap-6 xl:grid-cols-[1.08fr_0.92fr]">
			<div class="guardian-card">
				<h2 class="text-2xl font-semibold text-white">Protection summary</h2>
				<div class="mt-5 grid gap-4 sm:grid-cols-2">
					<div class="guardian-stat"><div class="text-sm text-slate-400">Large transfer trigger</div><div class="mt-2 text-xl font-semibold text-white">{formatPercent(view.largeTransferPct)}</div></div>
					<div class="guardian-stat"><div class="text-sm text-slate-400">Rapid tx rule</div><div class="mt-2 text-xl font-semibold text-white">{view.rapidTxCount} tx in {formatRapidWindow(view.rapidTxWindowSecs)}</div></div>
					<div class="guardian-stat"><div class="text-sm text-slate-400">New-address alerts</div><div class="mt-2 text-xl font-semibold text-white">{view.newAddressAlert ? 'On' : 'Off'}</div></div>
					<div class="guardian-stat"><div class="text-sm text-slate-400">Monitored chains</div><div class="mt-2 text-xl font-semibold text-white break-words">{view.monitoredChains.join(', ')}</div></div>
				</div>

				<div class="mt-6 rounded-[1.5rem] border border-white/10 bg-white/5 p-4 sm:p-5">
					<div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
						<div>
							<div class="text-sm text-slate-400">Configured alert destinations</div>
							<p class="mt-1 text-sm text-slate-500">Destinations stay masked in the dashboard for privacy.</p>
						</div>
						<a href="/settings" class="text-sm text-cyan-200 underline decoration-cyan-300/30 underline-offset-4">Manage in settings</a>
					</div>
					{#if view.alertChannels.length > 0}
						<div class="mt-4 grid gap-2">
							{#each view.alertChannels as channel}
								<div class="rounded-2xl border border-white/10 bg-slate-950/40 px-4 py-3 font-mono text-xs text-slate-200 sm:text-sm">{maskSavedChannel(channel)}</div>
							{/each}
						</div>
					{:else}
						<div class="mt-4 rounded-2xl border border-dashed border-white/10 bg-slate-950/30 px-4 py-5 text-sm text-slate-400">No alert destination configured yet. Add one from settings when you’re ready to receive notifications.</div>
					{/if}
				</div>
			</div>

			<div class="space-y-4">
				<div class="guardian-card border-cyan-400/20 bg-cyan-400/10">
					<h2 class="text-2xl font-semibold text-white">Verified email state</h2>
					<ul class="mt-4 space-y-3 text-sm leading-6 text-cyan-50">
						<li>• Verified email: {optText(emailStatus?.verified_email_masked) || 'None'}</li>
						<li>• Pending email: {optText(emailStatus?.pending_email_masked) || 'None'}</li>
						<li>• Delivery gate: {emailStatus?.delivery_active ? 'Only verified email is eligible for active delivery.' : 'No verified email delivery is active.'}</li>
						<li>• Raw destination values remain masked in the UI for privacy.</li>
					</ul>
				</div>

				<div class="guardian-card">
					<h2 class="text-2xl font-semibold text-white">What is live right now</h2>
					<ul class="mt-4 space-y-3 text-sm leading-6 text-slate-300">
						<li>• Internet Identity auth is providing the current caller principal.</li>
						<li>• <code>guardian_config.get_config()</code> loaded your saved state.</li>
						<li>• Settings saves sanitize raw email entries so unverified email never appears as an active channel.</li>
						<li>• Guardian remains advisory only — there is no automatic fund movement.</li>
					</ul>
				</div>
			</div>
		</div>
	{/if}
</div>
