<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchUsers, isOperatorModeEnabled } from '$lib/canister';
	import { truncatePrincipal, formatTimestamp, maskAlertChannel } from '$lib/utils';
	import type { UserConfig } from '$lib/types';

	let users: UserConfig[] = [];
	let loading = true;
	let error = '';
	let selected: UserConfig | null = null;
	const operatorMode = isOperatorModeEnabled();

	onMount(async () => {
		if (!operatorMode) {
			loading = false;
			return;
		}

		try {
			users = await fetchUsers();
			selected = users[0] ?? null;
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load operator configuration view.';
		} finally {
			loading = false;
		}
	});

	const severityColors: Record<string, string> = {
		INFO: 'bg-blue-900 text-blue-300',
		WARN: 'bg-yellow-900 text-yellow-300',
		CRITICAL: 'bg-orange-900 text-orange-300',
		EMERGENCY: 'bg-red-900 text-red-300'
	};

	const channelIcons: Record<string, string> = {
		Discord: '💬',
		Slack: '🟦',
		Webhook: '🔗',
		Email: '📧'
	};
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold text-white">⚙️ Operator Configuration View</h1>
	<p class="text-sm text-gray-500">Operator-only route. Private destinations stay masked in the UI. If live data cannot be read, this page fails closed.</p>

	{#if !operatorMode}
		<div class="rounded-2xl border border-amber-400/30 bg-amber-400/10 p-5 text-amber-100">
			This route is hidden in normal consumer mode. Enable operator mode explicitly for reviewed internal sessions only.
		</div>
	{:else if loading}
		<div class="text-gray-500 animate-pulse">Loading user configs…</div>
	{:else if error}
		<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 p-5 text-rose-100">{error}</div>
	{:else if users.length === 0}
		<div class="rounded-2xl border border-white/10 bg-slate-900/70 p-5 text-slate-300">No live config was returned for this operator view.</div>
	{:else}
		<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
			<div class="space-y-3">
				<h2 class="text-xs font-semibold uppercase tracking-wider text-gray-400">{users.length} visible config record{users.length === 1 ? '' : 's'}</h2>
				{#each users as user}
					<button
						on:click={() => (selected = user)}
						class="w-full rounded-lg border bg-gray-900 p-4 text-left transition-colors
							{selected?.principal === user.principal ? 'border-indigo-600 bg-indigo-950' : 'border-gray-800 hover:border-gray-600'}"
					>
						<div class="font-mono text-sm text-white">{truncatePrincipal(user.principal)}</div>
						<div class="mt-2 flex flex-wrap gap-2">
							{#each user.alert_channels as ch}
								<span class="rounded bg-gray-800 px-2 py-0.5 text-xs">{channelIcons[ch.type]} {ch.type}</span>
							{/each}
						</div>
						<div class="mt-2 text-xs text-gray-600">Threshold: {user.alert_threshold} · Updated: {formatTimestamp(user.updated_at).split(',')[0]}</div>
					</button>
				{/each}
			</div>

			<div class="lg:col-span-2">
				{#if selected}
					<div class="space-y-5 rounded-lg border border-gray-800 bg-gray-900 p-5">
						<div>
							<h3 class="mb-2 text-xs font-semibold uppercase tracking-wider text-gray-400">Principal</h3>
							<code class="break-all text-sm text-indigo-300">{truncatePrincipal(selected.principal)}</code>
						</div>

						<div class="grid grid-cols-2 gap-4 text-sm">
							<div>
								<span class="text-gray-500">Alert Threshold:</span>
								<span class="ml-2 text-white">{selected.alert_threshold}</span>
							</div>
							<div>
								<span class="text-gray-500">Created:</span>
								<span class="ml-2 text-white">{formatTimestamp(selected.created_at)}</span>
							</div>
						</div>

						<div>
							<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider text-gray-400">Alert Channels</h3>
							<div class="space-y-2">
								{#each selected.alert_channels as ch}
									<div class="flex items-center gap-3 rounded bg-gray-800 p-3 text-sm">
										<span class="text-lg">{channelIcons[ch.type]}</span>
										<div>
											<div class="font-semibold text-white">{ch.type}</div>
											<div class="max-w-xs truncate font-mono text-xs text-gray-400">{maskAlertChannel(ch)}</div>
										</div>
									</div>
								{/each}
							</div>
							<p class="mt-3 text-xs text-amber-200">Future privacy note: raw destination values should move to encrypted storage before shipping vetKeys-backed secret handling.</p>
						</div>

						<div>
							<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider text-gray-400">Detection Rules</h3>
							<div class="space-y-2">
								{#each selected.detection_rules as rule}
									<div class="flex items-start gap-3 rounded bg-gray-800 p-3 text-sm">
										<span class="mt-0.5 text-lg">{rule.enabled ? '✅' : '⬜'}</span>
										<div class="flex-1">
											<div class="flex flex-wrap items-center gap-2">
												<span class="font-semibold text-white">[{rule.id}] {rule.name}</span>
												<span class="rounded px-2 py-0.5 text-xs {severityColors[rule.severity]}">{rule.severity}</span>
											</div>
											<div class="mt-1 text-xs text-gray-500">{rule.description}</div>
										</div>
									</div>
								{/each}
							</div>
						</div>
					</div>
				{:else}
					<div class="rounded-lg border border-dashed border-gray-800 bg-gray-900 p-12 text-center text-gray-600">No live config selected</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
