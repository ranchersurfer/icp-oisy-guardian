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
	let preloadState: 'idle' | 'loading' | 'ready' | 'failed' = 'idle';
	selectedPreset.set(selected);

	$: editMode = $page.url.searchParams.get('mode') === 'edit';
	$: if (!$authState.isAuthenticated) {
		goto('/');
	}

	onMount(async () => {
		if (!$authState.isAuthenticated) return;
		if (!editMode) return;
		preloadState = 'loading';
		try {
			const result = await getMyConfig();
			if ('Ok' in result) {
				selected = detectPreset(result.Ok) ?? 'balanced';
				selectedPreset.set(selected);
				helperCopy = 'We prefilled this from your current live config so you can safely switch presets or rerun setup without losing context.';
				preloadState = 'ready';
				return;
			}
			preloadState = 'failed';
			helperCopy = 'No existing config was found, so you can choose a fresh preset and continue.';
		} catch {
			preloadState = 'failed';
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

<div class="space-y-6 sm:space-y-8">
	<div class="space-y-3">
		<div class="guardian-kicker">{editMode ? 'Re-onboarding' : 'Onboarding'}</div>
		<h1 class="guardian-section-title">{editMode ? 'Choose a new protection setup' : 'Choose your protection level'}</h1>
		<p class="max-w-3xl text-sm leading-6 text-slate-300 sm:text-base">{helperCopy}</p>
	</div>

	{#if editMode && preloadState === 'loading'}
		<div class="guardian-card text-sm text-slate-300">Loading your current live configuration so the right preset can be preselected…</div>
	{/if}

	<div class="grid gap-4 xl:grid-cols-3">
		{#each PRESETS as preset}
			<button
				on:click={() => choose(preset.id)}
				class={`group rounded-[1.9rem] border p-5 text-left transition sm:p-6 ${selected === preset.id ? 'border-cyan-300 bg-cyan-400/10 shadow-[0_0_0_1px_rgba(103,232,249,0.35),0_18px_60px_rgba(14,116,144,0.22)]' : 'border-white/10 bg-slate-900/70 hover:-translate-y-0.5 hover:border-white/20 hover:bg-slate-900/90'}`}
			>
				<div class="flex items-start justify-between gap-3">
					<div>
						<div class="text-2xl font-semibold text-white">{preset.name}</div>
						<div class="mt-1 text-sm text-slate-400">{preset.tagline}</div>
					</div>
					{#if preset.recommended}
						<div class="guardian-badge border-cyan-300/20 bg-cyan-400 text-slate-950">Recommended</div>
					{/if}
				</div>

				<p class="mt-4 text-sm leading-6 text-slate-300">{preset.description}</p>

				<div class="mt-5 grid gap-3 sm:grid-cols-3 xl:grid-cols-1">
					<div class="rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
						<div class="text-xs uppercase tracking-[0.2em] text-slate-500">Large transfer</div>
						<div class="mt-2 text-lg font-semibold text-white">{Math.round(preset.config.large_transfer_pct * 100)}% of balance</div>
					</div>
					<div class="rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
						<div class="text-xs uppercase tracking-[0.2em] text-slate-500">Rapid activity</div>
						<div class="mt-2 text-lg font-semibold text-white">{preset.config.rapid_tx_count} tx in {Math.round(preset.config.rapid_tx_window_secs / 60)} min</div>
					</div>
					<div class="rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
						<div class="text-xs uppercase tracking-[0.2em] text-slate-500">New address rule</div>
						<div class="mt-2 text-lg font-semibold text-white">{preset.config.new_address_alert ? 'Enabled' : 'Disabled'}</div>
					</div>
				</div>

				<div class="mt-5 flex items-center justify-between rounded-2xl border border-white/10 bg-slate-950/40 px-4 py-3 text-sm">
					<span class="text-slate-400">Best for</span>
					<span class={`font-medium ${selected === preset.id ? 'text-cyan-100' : 'text-slate-200'}`}>{preset.tagline}</span>
				</div>
			</button>
		{/each}
	</div>

	<div class="grid gap-4 lg:grid-cols-[1.2fr_0.8fr]">
		<div class="guardian-card">
			<div class="guardian-kicker">Current selection</div>
			<h2 class="mt-3 text-2xl font-semibold text-white">{PRESETS.find((preset) => preset.id === selected)?.name}</h2>
			<p class="mt-3 text-sm leading-6 text-slate-300">Guardian will carry this preset forward to the review screen, where you’ll see the live save path and confirm before anything is written.</p>
		</div>
		<div class="guardian-card border-cyan-400/20 bg-cyan-400/10">
			<div class="text-sm text-cyan-100">Connected principal</div>
			<div class="mt-3 break-all rounded-2xl border border-cyan-300/15 bg-slate-950/30 px-4 py-3 font-mono text-xs text-white sm:text-sm">{$authState.principal}</div>
			<button on:click={continueToReview} class="guardian-button-primary mt-5 min-h-12 w-full">
				{editMode ? 'Review updated protection' : 'Review protection'}
			</button>
		</div>
	</div>
</div>
