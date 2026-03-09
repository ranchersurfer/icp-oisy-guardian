<script lang="ts">
	import { goto } from '$app/navigation';
	import { authState } from '$lib/auth';
	import { getMyConfig, saveConfig } from '$lib/canister';
	import { buildConfigForPreset, formatPercent, formatRapidWindow, getPreset, selectedPreset } from '$lib/guardian';
	import type { GuardianConfigRecord, GuardianPresetId } from '$lib/types';
	import { get } from 'svelte/store';
	import { Principal } from '@dfinity/principal';

	let saving = false;
	let error = '';
	let success = '';
	let presetId: GuardianPresetId = get(selectedPreset);
	let existing: GuardianConfigRecord | undefined;

	$: presetId = get(selectedPreset);
	$: preset = getPreset(presetId);
	$: if (!$authState.isAuthenticated) {
		goto('/');
	}

	async function confirmAndSave() {
		if (!$authState.principal) {
			error = 'No connected principal found.';
			return;
		}

		saving = true;
		error = '';
		success = '';

		try {
			const current = await getMyConfig();
			if ('Ok' in current) existing = current.Ok;

			const config = buildConfigForPreset(presetId, Principal.fromText($authState.principal), existing);
			const writeResult = await saveConfig(config);
			if ('Err' in writeResult) {
				throw new Error(writeResult.Err);
			}

			const readBack = await getMyConfig();
			if ('Err' in readBack) {
				throw new Error(`Saved, but read-back failed: ${readBack.Err}`);
			}

			success = 'Protection saved on the live guardian_config canister.';
			await goto('/dashboard');
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to save Guardian protection.';
		} finally {
			saving = false;
		}
	}
</script>

<div class="space-y-8">
	<div class="space-y-2">
		<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">Review & confirm</div>
		<h1 class="text-4xl font-semibold text-white">Check what Guardian will save</h1>
		<p class="max-w-2xl text-slate-300">This writes your preset configuration under your connected principal on the live Internet Computer deployment.</p>
	</div>

	<div class="grid gap-6 lg:grid-cols-[1.2fr_0.8fr]">
		<div class="rounded-[1.75rem] border border-white/10 bg-slate-900/70 p-6">
			<div class="mb-5 flex items-center justify-between">
				<div>
					<div class="text-sm text-slate-400">Chosen preset</div>
					<div class="text-3xl font-semibold text-white">{preset.name}</div>
				</div>
				{#if preset.recommended}
					<div class="rounded-full bg-cyan-400 px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em] text-slate-950">Recommended</div>
				{/if}
			</div>

			<div class="grid gap-4 md:grid-cols-2">
				<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
					<div class="text-sm text-slate-400">Large transfer sensitivity</div>
					<div class="mt-2 text-xl font-semibold text-white">{formatPercent(preset.config.large_transfer_pct)}</div>
				</div>
				<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
					<div class="text-sm text-slate-400">Rapid transaction sensitivity</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.rapid_tx_count} tx in {formatRapidWindow(preset.config.rapid_tx_window_secs)}</div>
				</div>
				<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
					<div class="text-sm text-slate-400">New-address alerts</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.new_address_alert ? 'On' : 'Off'}</div>
				</div>
				<div class="rounded-2xl border border-white/10 bg-white/5 p-4">
					<div class="text-sm text-slate-400">Monitored chains</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.monitored_chains.join(', ')}</div>
				</div>
			</div>
		</div>

		<div class="space-y-4 rounded-[1.75rem] border border-cyan-400/20 bg-cyan-400/10 p-6">
			<div>
				<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">Storage note</div>
				<div class="mt-2 text-sm text-cyan-50">Saved under connected principal</div>
				<div class="mt-2 break-all font-mono text-sm text-white">{$authState.principal}</div>
			</div>
			<div class="text-sm text-cyan-50">
				Guardian stores monitoring preferences on the live <code>guardian_config</code> canister. It does not move funds or request your seed phrase.
			</div>
			<button on:click={confirmAndSave} disabled={saving} class="w-full rounded-full bg-cyan-300 px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200 disabled:opacity-60">
				{saving ? 'Saving to live canister…' : 'Save protection'}
			</button>
			<a href="/onboarding" class="block text-center text-sm text-cyan-100 underline decoration-cyan-300/30 underline-offset-4">Back to presets</a>
		</div>
	</div>

	{#if error}
		<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">{error}</div>
	{/if}
	{#if success}
		<div class="rounded-2xl border border-emerald-400/30 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-100">{success}</div>
	{/if}
</div>
