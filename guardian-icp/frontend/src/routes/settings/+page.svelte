<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authState } from '$lib/auth';
	import { getMyConfig, saveConfig } from '$lib/canister';
	import {
		PRESETS,
		detectPreset,
		formatLineList,
		parseLineList,
		formatPercent,
		formatRapidWindow,
		getPreset
	} from '$lib/guardian';
	import type { GuardianConfigRecord, GuardianPresetId } from '$lib/types';

	let loading = true;
	let saving = false;
	let error = '';
	let success = '';
	let existing: GuardianConfigRecord | null = null;
	let presetId: GuardianPresetId = 'balanced';
	let largeTransferPct = 0.5;
	let rapidTxCount = 5;
	let rapidTxWindowSecs = 600;
	let newAddressAlert = true;
	let alertThreshold = 7;
	let emergencyThreshold = 15;
	let monitoredChainsText = 'ICP\nckBTC\nckETH';
	let alertChannelsText = '';
	let allowlistedAddressesText = '';

	function applyPreset(id: GuardianPresetId) {
		presetId = id;
		const preset = getPreset(id);
		largeTransferPct = preset.config.large_transfer_pct;
		rapidTxCount = preset.config.rapid_tx_count;
		rapidTxWindowSecs = preset.config.rapid_tx_window_secs;
		newAddressAlert = preset.config.new_address_alert;
		alertThreshold = preset.config.alert_threshold;
		emergencyThreshold = preset.config.emergency_threshold;
		monitoredChainsText = formatLineList(preset.config.monitored_chains);
	}

	function hydrateFromConfig(config: GuardianConfigRecord) {
		existing = config;
		presetId = detectPreset(config) ?? 'balanced';
		largeTransferPct = config.large_transfer_pct;
		rapidTxCount = config.rapid_tx_count;
		rapidTxWindowSecs = Number(config.rapid_tx_window_secs);
		newAddressAlert = config.new_address_alert;
		alertThreshold = config.alert_threshold;
		emergencyThreshold = config.emergency_threshold;
		monitoredChainsText = formatLineList(config.monitored_chains);
		alertChannelsText = formatLineList(config.alert_channels);
		allowlistedAddressesText = formatLineList(config.allowlisted_addresses);
	}

	async function loadSettings() {
		if (!$authState.isAuthenticated) {
			await goto('/');
			return;
		}

		loading = true;
		error = '';
		try {
			const result = await getMyConfig();
			if ('Err' in result) {
				await goto('/onboarding');
				return;
			}
			hydrateFromConfig(result.Ok);
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load your settings.';
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		if (!existing) return;
		saving = true;
		error = '';
		success = '';

		try {
			const config: GuardianConfigRecord = {
				...existing,
				updated_at: BigInt(Date.now()) * BigInt(1_000_000),
				monitored_chains: parseLineList(monitoredChainsText),
				large_transfer_pct: Number(largeTransferPct),
				daily_outflow_pct: existing.daily_outflow_pct,
				rapid_tx_count: Number(rapidTxCount),
				rapid_tx_window_secs: BigInt(rapidTxWindowSecs),
				new_address_alert: newAddressAlert,
				alert_threshold: Number(alertThreshold),
				emergency_threshold: Number(emergencyThreshold),
				alert_channels: parseLineList(alertChannelsText),
				allowlisted_addresses: parseLineList(allowlistedAddressesText)
			};

			const writeResult = await saveConfig(config);
			if ('Err' in writeResult) throw new Error(writeResult.Err);

			const readBack = await getMyConfig();
			if ('Err' in readBack) throw new Error(`Saved, but read-back failed: ${readBack.Err}`);
			hydrateFromConfig(readBack.Ok);
			success = 'Settings saved to the live guardian_config canister.';
			await goto('/dashboard');
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to save settings.';
		} finally {
			saving = false;
		}
	}

	onMount(loadSettings);
</script>

<div class="space-y-8">
	<div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
		<div>
			<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">Settings</div>
			<h1 class="text-4xl font-semibold text-white">Edit your Guardian settings</h1>
			<p class="mt-2 max-w-3xl text-slate-300">Update your preset, tune alert sensitivity, and manage trusted destinations without creating a new identity.</p>
		</div>
		<a href="/onboarding?mode=edit" class="inline-flex rounded-full border border-white/15 px-5 py-3 text-sm text-slate-200 transition hover:bg-white/10">Re-run onboarding</a>
	</div>

	{#if loading}
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6 text-slate-300">Loading your live settings…</div>
	{:else}
		<div class="grid gap-4 lg:grid-cols-3">
			{#each PRESETS as preset}
				<button
					on:click={() => applyPreset(preset.id)}
					class={`rounded-[1.5rem] border p-5 text-left transition ${presetId === preset.id ? 'border-cyan-300 bg-cyan-400/10 shadow-[0_0_0_1px_rgba(103,232,249,0.3)]' : 'border-white/10 bg-slate-900/70 hover:border-white/20'}`}
				>
					<div class="flex items-start justify-between gap-3">
						<div>
							<div class="text-xl font-semibold text-white">{preset.name}</div>
							<div class="mt-1 text-sm text-slate-400">{preset.tagline}</div>
						</div>
						{#if preset.recommended}
							<div class="rounded-full bg-cyan-400 px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.2em] text-slate-950">Recommended</div>
						{/if}
					</div>
					<div class="mt-4 space-y-2 text-sm text-slate-200">
						<div>Large transfer alert: {formatPercent(preset.config.large_transfer_pct)}</div>
						<div>Rapid tx rule: {preset.config.rapid_tx_count} tx in {formatRapidWindow(preset.config.rapid_tx_window_secs)}</div>
					</div>
				</button>
			{/each}
		</div>

		<div class="grid gap-6 lg:grid-cols-[1.1fr_0.9fr]">
			<div class="space-y-6 rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
				<div>
					<h2 class="text-xl font-semibold text-white">Advanced controls</h2>
					<p class="mt-2 text-sm text-slate-400">Preset changes are safe and reversible. You can also fine-tune thresholds below.</p>
				</div>

				<div class="grid gap-4 md:grid-cols-2">
					<label class="space-y-2">
						<span class="text-sm text-slate-300">Large transfer alert (% of balance)</span>
						<input bind:value={largeTransferPct} type="number" min="0.01" max="1" step="0.01" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white" />
					</label>
					<label class="space-y-2">
						<span class="text-sm text-slate-300">Rapid transaction count</span>
						<input bind:value={rapidTxCount} type="number" min="1" step="1" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white" />
					</label>
					<label class="space-y-2">
						<span class="text-sm text-slate-300">Rapid transaction window (seconds)</span>
						<input bind:value={rapidTxWindowSecs} type="number" min="60" step="60" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white" />
					</label>
					<label class="space-y-2">
						<span class="text-sm text-slate-300">Alert sensitivity threshold</span>
						<input bind:value={alertThreshold} type="number" min="1" step="1" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white" />
					</label>
					<label class="space-y-2 md:col-span-2">
						<span class="text-sm text-slate-300">Emergency threshold</span>
						<input bind:value={emergencyThreshold} type="number" min="1" step="1" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white" />
					</label>
				</div>

				<label class="flex items-center gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm text-slate-200">
					<input bind:checked={newAddressAlert} type="checkbox" class="h-4 w-4 rounded border-white/20 bg-slate-950 text-cyan-400" />
					Alert me when funds move to a new address
				</label>
			</div>

			<div class="space-y-6 rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
				<div>
					<h2 class="text-xl font-semibold text-white">Destinations and allowlist</h2>
					<p class="mt-2 text-sm text-slate-400">One value per line. Example channel formats: <code>email;address=name@example.com</code> or <code>discord;url=https://discord.com/api/webhooks/...</code></p>
				</div>

				<label class="space-y-2 block">
					<span class="text-sm text-slate-300">Monitored chains</span>
					<textarea bind:value={monitoredChainsText} rows="4" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white"></textarea>
				</label>
				<label class="space-y-2 block">
					<span class="text-sm text-slate-300">Alert destinations</span>
					<textarea bind:value={alertChannelsText} rows="6" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white"></textarea>
				</label>
				<label class="space-y-2 block">
					<span class="text-sm text-slate-300">Trusted / allowlisted addresses</span>
					<textarea bind:value={allowlistedAddressesText} rows="6" class="w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-white"></textarea>
				</label>
			</div>
		</div>

		<div class="flex flex-wrap items-center gap-3 rounded-3xl border border-cyan-400/20 bg-cyan-400/10 p-5">
			<button on:click={saveSettings} disabled={saving || !existing} class="rounded-full bg-cyan-300 px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200 disabled:opacity-60">
				{saving ? 'Saving to live canister…' : 'Save settings'}
			</button>
			<a href="/dashboard" class="rounded-full border border-white/15 px-5 py-3 text-sm text-slate-200 transition hover:bg-white/10">Back to dashboard</a>
			<div class="text-sm text-cyan-50">Writes to live <code>guardian_config.set_config()</code> and returns you to your dashboard.</div>
		</div>

		{#if error}
			<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">{error}</div>
		{/if}
		{#if success}
			<div class="rounded-2xl border border-emerald-400/30 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-100">{success}</div>
		{/if}
	{/if}
</div>
