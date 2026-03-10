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

	const trustPoints = [
		'Non-custodial. Guardian never holds funds or asks for seed phrases.',
		'Your settings are tied to your connected principal and saved on-chain.',
		'You choose the alert sensitivity level and can change it later.',
		'Guardian alerts and explains risk signals — it does not move funds for you.'
	];

	const steps = [
		{
			label: 'Connect',
			title: 'Internet Identity first',
			body: 'Connect once, restore your session on return, and keep your Guardian settings linked to your principal.'
		},
		{
			label: 'Choose',
			title: 'Pick a protection preset',
			body: 'Start with Safe, Balanced, or Aggressive. Balanced stays the recommended default for most wallets.'
		},
		{
			label: 'Review',
			title: 'Confirm before saving live',
			body: 'See the exact protection behavior in plain language before anything is written to the live guardian_config canister.'
		}
	];
</script>

<div class="space-y-6 sm:space-y-8">
	<section class="overflow-hidden rounded-[2rem] border border-cyan-400/20 bg-[radial-gradient(circle_at_top_left,_rgba(34,211,238,0.16),_transparent_30%),linear-gradient(180deg,rgba(15,23,42,0.94),rgba(2,6,23,0.98))] p-6 shadow-[0_24px_80px_rgba(2,6,23,0.38)] sm:p-8 lg:p-10">
		<div class="grid gap-8 xl:grid-cols-[1.15fr_0.85fr] xl:items-center">
			<div class="space-y-6">
				<div class="guardian-badge border-cyan-300/20 bg-cyan-400/10 text-cyan-100">2026 consumer protection flow</div>
				<div class="space-y-4">
					<h1 class="max-w-3xl text-4xl font-semibold tracking-tight text-white sm:text-5xl lg:text-6xl">
						Guardian helps you catch risky wallet behavior earlier.
					</h1>
					<p class="max-w-2xl text-base leading-7 text-slate-300 sm:text-lg">
						A calmer, consumer-friendly safety layer for ICP wallets — designed to feel trustworthy, understandable, and usable on mobile from the first tap.
					</p>
				</div>

				<div class="flex flex-col gap-3 sm:flex-row">
					<button on:click={connectAndContinue} disabled={$authState.connecting || checking} class="guardian-button-primary min-h-12 disabled:opacity-60">
						{$authState.connecting || checking ? 'Checking your account…' : 'Connect with Internet Identity'}
					</button>
					<a href="#how-it-works" class="guardian-button-secondary min-h-12">How it works</a>
				</div>

				<div class="grid gap-3 sm:grid-cols-3">
					<div class="guardian-stat">
						<div class="text-sm text-slate-400">Setup feel</div>
						<div class="mt-2 text-xl font-semibold text-white">Under 2 minutes</div>
					</div>
					<div class="guardian-stat">
						<div class="text-sm text-slate-400">Storage path</div>
						<div class="mt-2 text-xl font-semibold text-white">Live IC canisters</div>
					</div>
					<div class="guardian-stat">
						<div class="text-sm text-slate-400">Wallet model</div>
						<div class="mt-2 text-xl font-semibold text-white">Non-custodial</div>
					</div>
				</div>

				{#if routeError}
					<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">
						{routeError}
					</div>
				{/if}
			</div>

			<div class="guardian-glass-strong rounded-[1.75rem] p-5 sm:p-6">
				<div class="space-y-5">
					<div>
						<div class="guardian-kicker">Trust at a glance</div>
						<h2 class="mt-3 text-2xl font-semibold text-white">Built to explain what Guardian does — and what it doesn’t.</h2>
					</div>

					<ul class="space-y-3 text-sm leading-6 text-slate-200">
						{#each trustPoints as point}
							<li class="flex gap-3">
								<span class="mt-1 inline-flex h-6 w-6 shrink-0 items-center justify-center rounded-full border border-cyan-300/20 bg-cyan-400/10 text-cyan-100">✓</span>
								<span>{point}</span>
							</li>
						{/each}
					</ul>

					<div class="rounded-[1.5rem] border border-cyan-400/20 bg-cyan-400/10 p-4 text-sm leading-6 text-cyan-50">
						If a sudden transfer sends a large share of your balance to a new address, Guardian can surface that behavior, explain why it was flagged, and route an alert to your configured destinations.
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="how-it-works" class="grid gap-4 lg:grid-cols-3">
		{#each steps as step}
			<div class="guardian-card">
				<div class="guardian-kicker">{step.label}</div>
				<h2 class="mt-3 text-2xl font-semibold text-white">{step.title}</h2>
				<p class="mt-3 text-sm leading-6 text-slate-300">{step.body}</p>
			</div>
		{/each}
	</section>

	<section class="grid gap-4 lg:grid-cols-[1.1fr_0.9fr]">
		<div class="guardian-card">
			<div class="guardian-kicker">Why people start here</div>
			<h2 class="mt-3 text-2xl font-semibold text-white">Opinionated defaults first, advanced controls later.</h2>
			<p class="mt-3 max-w-2xl text-sm leading-6 text-slate-300">
				Guardian avoids dumping raw config fields on first-time users. Start with a preset, review the effect in plain language, then fine-tune settings once your protection is already live.
			</p>
		</div>
		<div class="guardian-card border-emerald-400/20 bg-emerald-400/10">
			<div class="guardian-kicker text-emerald-200">Live deployment posture</div>
			<h2 class="mt-3 text-2xl font-semibold text-white">No fake “protected” state.</h2>
			<p class="mt-3 text-sm leading-6 text-emerald-50">
				Guardian’s consumer routes are wired for the live deployment path. Private views fail closed instead of pretending broad mock data is yours.
			</p>
		</div>
	</section>
</div>
