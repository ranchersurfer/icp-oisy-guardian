<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { authState, initAuth, login, logout } from '$lib/auth';
	import { getActiveCanisterIds, getActiveHost, isLiveMode, isOperatorModeEnabled } from '$lib/canister';
	import { shortenPrincipal } from '$lib/guardian';

	const liveMode = isLiveMode();
	const operatorMode = isOperatorModeEnabled();
	const activeHost = getActiveHost();
	const canisterIds = getActiveCanisterIds();

	const consumerNav = [
		{ href: '/', label: 'Home' },
		{ href: '/onboarding', label: 'Onboarding' },
		{ href: '/review', label: 'Review' },
		{ href: '/dashboard', label: 'Dashboard' },
		{ href: '/settings', label: 'Settings' }
	];

	const operatorNav = [
		{ href: '/config', label: 'Config' },
		{ href: '/alerts', label: 'Alerts' },
		{ href: '/stats', label: 'Stats' }
	];

	onMount(() => {
		initAuth();
	});
</script>

<div class="min-h-screen bg-[#08111f] text-slate-100">
	<header class="border-b border-white/10 bg-slate-950/90 backdrop-blur">
		<div class="mx-auto flex max-w-7xl items-center gap-4 px-6 py-4">
			<a href="/" class="flex items-center gap-3">
				<div class="flex h-11 w-11 items-center justify-center rounded-2xl bg-cyan-500/20 text-xl">🛡️</div>
				<div>
					<div class="text-lg font-semibold text-white">Guardian</div>
					<div class="text-xs text-slate-400">Consumer onboarding on live IC canisters</div>
				</div>
			</a>

			<nav class="ml-6 hidden items-center gap-2 lg:flex">
				{#each consumerNav as item}
					<a
						href={item.href}
						class={`rounded-full px-3 py-2 text-sm transition ${$page.url.pathname === item.href ? 'bg-white text-slate-950' : 'text-slate-300 hover:bg-white/10 hover:text-white'}`}
					>
						{item.label}
					</a>
				{/each}
			</nav>

			<div class="ml-auto flex items-center gap-3">
				<div class="hidden rounded-full border border-white/10 px-3 py-2 text-xs text-slate-400 md:block">
					{#if liveMode}
						Live · {canisterIds.config} · {activeHost.replace('https://', '').replace('http://', '')}
					{:else}
						Mock mode
					{/if}
				</div>

				{#if $authState.isAuthenticated}
					<div class="hidden rounded-full border border-emerald-400/30 bg-emerald-400/10 px-3 py-2 text-sm text-emerald-200 sm:block">
						II · {shortenPrincipal($authState.principal ?? '')}
					</div>
					<button
						on:click={logout}
						class="rounded-full border border-white/15 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
					>
						Disconnect
					</button>
				{:else}
					<button
						disabled={$authState.connecting}
						on:click={login}
						class="rounded-full bg-cyan-400 px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-cyan-300 disabled:opacity-60"
					>
						{$authState.connecting ? 'Connecting…' : 'Connect with Internet Identity'}
					</button>
				{/if}
			</div>
		</div>

		<div class="mx-auto max-w-7xl px-6 pb-4">
			{#if operatorMode}
				<div class="flex flex-wrap items-center gap-2 text-xs text-slate-500">
					<span class="uppercase tracking-[0.2em] text-amber-300">Operator routes enabled</span>
					{#each operatorNav as item}
						<a href={item.href} class="rounded-full border border-amber-400/20 px-2.5 py-1 text-amber-100 transition hover:border-amber-300/40 hover:text-white">{item.label}</a>
					{/each}
				</div>
			{:else}
				<div class="rounded-2xl border border-amber-400/20 bg-amber-400/10 px-4 py-3 text-xs text-amber-100">
					Operator-only routes are hidden in normal consumer mode. Enable <code>VITE_ENABLE_OPERATOR_ROUTES=true</code> only for reviewed operator sessions.
				</div>
			{/if}
		</div>
	</header>

	<main class="mx-auto max-w-7xl px-6 py-8">
		<slot />
	</main>
</div>
