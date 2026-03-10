<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { authState } from '$lib/auth';
	import { getMyConfig } from '$lib/canister';
	import { PRESETS, detectPreset, selectedPreset } from '$lib/guardian';
	import type { GuardianPresetId } from '$lib/types';

	let selected: GuardianPresetId = 'balanced';
	let editMode = false;
	let helperCopy = 'Pick a starting point. Balanced is the recommended default for most Guardian users.';
	selectedPreset.set(selected);

	$: editMode = $page.url.searchParams.get('mode') === 'edit';
	$: if (!$authState.isAuthenticated) {
		goto('/');
	}

	onMount(async () => {
		if (!$authState.isAuthenticated) return;
		if (!editMode) return;
		try {
			const result = await getMyConfig();
			if ('Ok' in result) {
				selected = detectPreset(result.Ok) ?? 'balanced';
				selectedPreset.set(selected);
				helperCopy = 'We prefilled this from your current live config so you can safely switch presets or run setup again.';
			}
		} catch {
			helperCopy = 'Could not preload your current config, but you can still choose a preset and continue.';
		}
	});

	function choose(id: GuardianPresetId) {
		selected = id;
		selectedPreset.set(id);
	}

	async function continueToReview() {
		const suffix = editMode ? '?mode=edit' : '';
		await goto(`/review${suffix}`);
	}
</script>

<div class="space-y-8">
	<div class="space-y-2">
		<div class="text-sm uppercase tracking-[0.25em] text-cyan-200">{editMode ? 'Re-onboarding' : 'Onboarding'}</div>
		<h1 class="text-4xl font-semibold text-white">{editMode ? 'Choose a new protection setup' : 'Choose your protection level'}</h1>
		<p class="max-w-2xl text-slate-300">{helperCopy}</p>
	</div>

	<div class="grid gap-4 lg:grid-cols-3">
		{#each PRESETS as preset}
			<button
				on:click={() => choose(preset.id)}
				class={`rounded-[1.75rem] border p-6 text-left transition ${selected === preset.id ? 'border-cyan-300 bg-cyan-400/10 shadow-[0_0_0_1px_rgba(103,232,249,0.3)]' : 'border-white/10 bg-slate-900/70 hover:border-white/20'}`}
			>
				<div class="mb-4 flex items-center justify-between">
					<div>
						<div class="text-2xl font-semibold text-white">{preset.name}</div>
						<div class="text-sm text-slate-400">{preset.tagline}</div>
					</div>
					{#if preset.recommended}
						<div class="rounded-full bg-cyan-400 px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em] text-slate-950">Recommended</div>
					{/if}
				</div>
				<p class="mb-5 text-sm text-slate-300">{preset.description}</p>
				<ul class="space-y-2 text-sm text-slate-200">
					<li>• Large transfer alert at {Math.round(preset.config.large_transfer_pct * 100)}% of balance</li>
					<li>• Rapid tx trigger: {preset.config.rapid_tx_count} tx in {Math.round(preset.config.rapid_tx_window_secs / 60)} min</li>
					<li>• New-address alerts: {preset.config.new_address_alert ? 'On' : 'Off'}</li>
				</ul>
			</button>
		{/each}
	</div>

	<div class="flex items-center justify-between rounded-3xl border border-white/10 bg-slate-900/60 p-5">
		<div>
			<div class="text-sm text-slate-400">Connected principal</div>
			<div class="mt-1 font-mono text-sm text-slate-200">{$authState.principal}</div>
		</div>
		<button on:click={continueToReview} class="rounded-full bg-cyan-400 px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-cyan-300">
			{editMode ? 'Review updated protection' : 'Review protection'}
		</button>
	</div>
</div>
