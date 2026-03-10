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
		{ href: '/dashboard', label: 'Dashboard' },
		{ href: '/alerts', label: 'Alerts' },
		{ href: '/settings', label: 'Settings' }
	];

	const operatorNav = [
		{ href: '/config', label: 'Config' },
		{ href: '/alerts', label: 'Alerts' },
		{ href: '/stats', label: 'Stats' }
	];

	$: currentPath = $page.url.pathname;

	function isActive(href: string): boolean {
		return currentPath === href;
	}

	onMount(() => {
		initAuth();
	});
</script>

<div class="guardian-shell">
	<header class="sticky top-0 z-40 border-b border-white/10 bg-slate-950/75 backdrop-blur-2xl">
		<div class="mx-auto max-w-7xl px-4 py-4 sm:px-6">
			<div class="guardian-glass rounded-[1.75rem] px-4 py-4 sm:px-5">
				<div class="flex flex-col gap-4 lg:flex-row lg:items-center">
					<div class="flex items-start gap-3 sm:items-center">
						<a href="/" class="flex min-w-0 items-center gap-3">
							<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl border border-cyan-300/20 bg-cyan-400/10 text-xl text-cyan-100 shadow-[0_0_30px_rgba(34,211,238,0.15)]">🛡️</div>
							<div class="min-w-0">
								<div class="text-lg font-semibold text-white sm:text-xl">Guardian</div>
								<div class="text-xs text-slate-400 sm:text-sm">Consumer wallet protection on live IC canisters</div>
							</div>
						</a>
					</div>

					<div class="flex flex-1 flex-col gap-3 lg:items-end">
						<div class="flex w-full flex-col gap-3 lg:flex-row lg:items-center lg:justify-end">
							<nav class="flex w-full gap-2 overflow-x-auto pb-1 lg:w-auto lg:flex-wrap lg:justify-end lg:overflow-visible lg:pb-0">
								{#each consumerNav as item}
									<a
										href={item.href}
										class={`shrink-0 rounded-full px-4 py-2.5 text-sm transition ${isActive(item.href) ? 'bg-white text-slate-950 shadow-lg shadow-cyan-950/10' : 'text-slate-300 hover:bg-white/10 hover:text-white'}`}
									>
										{item.label}
									</a>
								{/each}
							</nav>

							<div class="flex flex-wrap items-center gap-2 sm:gap-3 lg:justify-end">
								<div class="guardian-badge border-white/10 bg-white/5 text-slate-300">
									{#if liveMode}
										<span class="mr-2 inline-flex h-2 w-2 rounded-full bg-emerald-400 shadow-[0_0_12px_rgba(74,222,128,0.8)]"></span>
										Live
									{:else}
										Mock
									{/if}
								</div>
								<div class="hidden rounded-full border border-white/10 bg-white/5 px-3 py-2 text-xs text-slate-400 md:block">
									{canisterIds.config} · {activeHost.replace('https://', '').replace('http://', '')}
								</div>

								{#if $authState.isAuthenticated}
									<div class="rounded-full border border-emerald-400/20 bg-emerald-400/10 px-3 py-2 text-xs text-emerald-100 sm:text-sm">
										II · {shortenPrincipal($authState.principal ?? '')}
									</div>
									<button on:click={logout} class="guardian-button-secondary px-4 py-2.5">
										Disconnect
									</button>
								{:else}
									<button disabled={$authState.connecting} on:click={login} class="guardian-button-primary px-4 py-2.5 disabled:opacity-60">
										{$authState.connecting ? 'Connecting…' : 'Connect with Internet Identity'}
									</button>
								{/if}
							</div>
						</div>

						{#if operatorMode}
							<div class="flex flex-wrap items-center gap-2 text-xs text-slate-400">
								<span class="guardian-kicker text-amber-300">Operator routes enabled</span>
								{#each operatorNav as item}
									<a href={item.href} class="rounded-full border border-amber-400/20 px-3 py-1.5 text-amber-100 transition hover:border-amber-300/40 hover:bg-amber-300/10 hover:text-white">{item.label}</a>
								{/each}
							</div>
						{:else}
							<div class="rounded-2xl border border-amber-400/20 bg-amber-400/10 px-4 py-3 text-xs leading-5 text-amber-100">
								Operator-only routes stay hidden in normal consumer mode. Enable <code>VITE_ENABLE_OPERATOR_ROUTES=true</code> only for reviewed operator sessions.
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</header>

	<main class="mx-auto max-w-7xl px-4 py-6 sm:px-6 sm:py-8">
		<slot />
	</main>
</div>
