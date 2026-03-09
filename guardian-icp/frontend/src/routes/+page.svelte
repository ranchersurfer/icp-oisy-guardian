<script lang="ts">
	import { goto } from '$app/navigation';
	import { authState, login } from '$lib/auth';
	import { getMyConfig } from '$lib/canister';

	let checking = false;
	let routeError = '';

	$: if ($authState.isAuthenticated && !checking) {
		checkExistingConfig();
	}

	async function connectAndContinue() {
		routeError = '';
		try {
			await login();
		} catch (error) {
			routeError = error instanceof Error ? error.message : 'Internet Identity connection failed.';
		}
	}

	async function checkExistingConfig() {
		checking = true;
		routeError = '';
		try {
			const result = await getMyConfig();
			if ('Ok' in result) {
				await goto('/dashboard');
			} else {
				await goto('/onboarding');
			}
		} catch (error) {
			routeError = error instanceof Error ? error.message : 'Failed to load your Guardian state.';
		} finally {
			checking = false;
		}
	}
</script>

<div class="space-y-8">
	<section class="grid gap-8 rounded-[2rem] border border-cyan-400/20 bg-[radial-gradient(circle_at_top_left,_rgba(34,211,238,0.14),_transparent_30%),linear-gradient(180deg,rgba(15,23,42,0.95),rgba(2,6,23,0.98))] p-8 lg:grid-cols-[1.2fr_0.8fr] lg:p-12">
		<div class="space-y-6">
			<div class="inline-flex rounded-full border border-cyan-300/20 bg-cyan-400/10 px-4 py-1 text-xs uppercase tracking-[0.25em] text-cyan-200">
				Phase 5 · Sprint 1
			</div>
			<div class="space-y-4">
				<h1 class="max-w-3xl text-4xl font-semibold tracking-tight text-white lg:text-6xl">
					Protect your ICP wallet from suspicious activity
				</h1>
				<p class="max-w-2xl text-lg text-slate-300">
					A safety layer that watches your wallet activity and alerts you when something looks off.
				</p>
			</div>

			<div class="flex flex-wrap gap-3">
				<button
					on:click={connectAndContinue}
					disabled={$authState.connecting || checking}
					class="rounded-full bg-cyan-400 px-6 py-3 text-sm font-semibold text-slate-950 transition hover:bg-cyan-300 disabled:opacity-60"
				>
					{$authState.connecting || checking ? 'Checking account…' : 'Connect with Internet Identity'}
				</button>
				<a href="#how-it-works" class="rounded-full border border-white/15 px-6 py-3 text-sm text-slate-200 transition hover:bg-white/10">
					How it works
				</a>
			</div>

			{#if routeError}
				<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">
					{routeError}
				</div>
			{/if}
		</div>

		<div class="rounded-[1.75rem] border border-white/10 bg-white/5 p-6">
			<div class="space-y-4">
				<div class="text-sm font-semibold uppercase tracking-[0.25em] text-slate-400">Trust</div>
				<ul class="space-y-3 text-sm text-slate-200">
					<li>• Non-custodial — Guardian never holds your funds</li>
					<li>• No seed phrase required</li>
					<li>• Settings stored on live Internet Computer canisters</li>
					<li>• Advisory alerts, not automatic fund movement</li>
				</ul>
				<div class="rounded-2xl border border-cyan-400/20 bg-cyan-400/10 p-4 text-sm text-cyan-100">
					If a large transfer suddenly moves 60% of your balance to a new address, Guardian can flag it and alert you.
				</div>
			</div>
		</div>
	</section>

	<section id="how-it-works" class="grid gap-4 md:grid-cols-3">
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
			<div class="mb-3 text-sm uppercase tracking-[0.25em] text-cyan-200">1 · Connect</div>
			<h2 class="mb-2 text-xl font-semibold text-white">Internet Identity first</h2>
			<p class="text-sm text-slate-300">Connect with Internet Identity so Guardian can save protection settings under your principal.</p>
		</div>
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
			<div class="mb-3 text-sm uppercase tracking-[0.25em] text-cyan-200">2 · Choose</div>
			<h2 class="mb-2 text-xl font-semibold text-white">Safe, Balanced, or Aggressive</h2>
			<p class="text-sm text-slate-300">Start with a preset now, then fine-tune later once the broader settings UI lands.</p>
		</div>
		<div class="rounded-3xl border border-white/10 bg-slate-900/70 p-6">
			<div class="mb-3 text-sm uppercase tracking-[0.25em] text-cyan-200">3 · Save live</div>
			<h2 class="mb-2 text-xl font-semibold text-white">Stored on-chain</h2>
			<p class="text-sm text-slate-300">Guardian writes your configuration to the live <code class="text-cyan-200">guardian_config</code> canister, then reads it back before showing your dashboard.</p>
		</div>
	</section>
</div>
