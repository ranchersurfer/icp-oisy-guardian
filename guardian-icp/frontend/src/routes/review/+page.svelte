<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
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
	let editMode = false;

	$: presetId = get(selectedPreset);
	$: preset = getPreset(presetId);
	$: editMode = $page.url.searchParams.get('mode') === 'edit';
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

			success = editMode ? 'Updated protection saved on the live guardian_config canister.' : 'Protection saved on the live guardian_config canister.';
			await goto('/dashboard');
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to save Guardian protection.';
		} finally {
			saving = false;
		}
	}

	const reviewHighlights = [
		'Written under your connected principal, not a shared account.',
		'Stored on the live guardian_config canister.',
		'Read back after save before routing you to your dashboard.',
		'Advisory only — Guardian does not automatically move or block funds.'
	];
</script>

<div class="space-y-6 sm:space-y-8">
	<div class="space-y-3">
		<div class="guardian-kicker">{editMode ? 'Review update' : 'Review & confirm'}</div>
		<h1 class="guardian-section-title">{editMode ? 'Check your updated Guardian setup' : 'Check what Guardian will save'}</h1>
		<p class="max-w-3xl text-sm leading-6 text-slate-300 sm:text-base">This is the plain-language checkpoint before your selected protection preset is saved on the live Internet Computer deployment.</p>
	</div>

	<div class="grid gap-6 xl:grid-cols-[1.12fr_0.88fr]">
		<div class="guardian-card">
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
				<div>
					<div class="text-sm text-slate-400">Chosen preset</div>
					<div class="mt-2 text-3xl font-semibold text-white">{preset.name}</div>
					<p class="mt-3 max-w-2xl text-sm leading-6 text-slate-300">{preset.description}</p>
				</div>
				{#if preset.recommended}
					<div class="guardian-badge border-cyan-300/20 bg-cyan-400 text-slate-950">Recommended</div>
				{/if}
			</div>

			<div class="mt-6 grid gap-4 sm:grid-cols-2">
				<div class="guardian-stat">
					<div class="text-sm text-slate-400">Large transfer sensitivity</div>
					<div class="mt-2 text-xl font-semibold text-white">{formatPercent(preset.config.large_transfer_pct)}</div>
					<p class="mt-2 text-sm text-slate-400">Flags transfers that look materially large compared with wallet balance.</p>
				</div>
				<div class="guardian-stat">
					<div class="text-sm text-slate-400">Rapid transaction sensitivity</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.rapid_tx_count} tx in {formatRapidWindow(preset.config.rapid_tx_window_secs)}</div>
					<p class="mt-2 text-sm text-slate-400">Useful when unusual bursts of activity suggest compromise or automation.</p>
				</div>
				<div class="guardian-stat">
					<div class="text-sm text-slate-400">New-address alerts</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.new_address_alert ? 'Enabled' : 'Disabled'}</div>
					<p class="mt-2 text-sm text-slate-400">Watch for funds moving to destinations that haven’t been seen before.</p>
				</div>
				<div class="guardian-stat">
					<div class="text-sm text-slate-400">Monitored chains</div>
					<div class="mt-2 text-xl font-semibold text-white">{preset.config.monitored_chains.join(', ')}</div>
					<p class="mt-2 text-sm text-slate-400">Protection applies across the currently exposed consumer chain set.</p>
				</div>
			</div>
		</div>

		<div class="guardian-card border-cyan-400/20 bg-cyan-400/10">
			<div class="guardian-kicker">Live save path</div>
			<h2 class="mt-3 text-2xl font-semibold text-white">Saved under your connected principal</h2>
			<div class="mt-4 break-all rounded-2xl border border-cyan-300/15 bg-slate-950/30 px-4 py-3 font-mono text-xs text-white sm:text-sm">{$authState.principal}</div>

			<ul class="mt-5 space-y-3 text-sm leading-6 text-cyan-50">
				{#each reviewHighlights as point}
					<li class="flex gap-3">
						<span class="mt-1 inline-flex h-6 w-6 shrink-0 items-center justify-center rounded-full border border-cyan-300/20 bg-cyan-300/10 text-cyan-100">•</span>
						<span>{point}</span>
					</li>
				{/each}
			</ul>

			<div class="mt-6 flex flex-col gap-3">
				<button on:click={confirmAndSave} disabled={saving} class="guardian-button-primary min-h-12 w-full disabled:opacity-60">
					{saving ? 'Saving to live canister…' : editMode ? 'Save updated protection' : 'Save protection'}
				</button>
				<a href={editMode ? '/onboarding?mode=edit' : '/onboarding'} class="guardian-button-secondary min-h-12 w-full">Back to presets</a>
			</div>
		</div>
	</div>

	{#if error}
		<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">{error}</div>
	{/if}
	{#if success}
		<div class="rounded-2xl border border-emerald-400/30 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-100">{success}</div>
	{/if}
</div>
