<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchUsers } from '$lib/canister';
	import { truncatePrincipal, formatTimestamp } from '$lib/utils';
	import type { UserConfig } from '$lib/types';

	let users: UserConfig[] = [];
	let loading = true;
	let selected: UserConfig | null = null;

	onMount(async () => {
		users = await fetchUsers();
		loading = false;
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
	<h1 class="text-2xl font-bold text-white">⚙️ Configuration</h1>
	<p class="text-gray-500 text-sm">Read-only view of active user configurations. All mutations require backend authorization.</p>

	{#if loading}
		<div class="text-gray-500 animate-pulse">Loading user configs…</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- User list -->
			<div class="space-y-3">
				<h2 class="text-gray-400 text-xs font-semibold uppercase tracking-wider">
					{users.length} Active Users
				</h2>
				{#each users as user}
					<button
						on:click={() => selected = user}
						class="w-full text-left bg-gray-900 border rounded-lg p-4 transition-colors
							{selected?.principal === user.principal
								? 'border-indigo-600 bg-indigo-950'
								: 'border-gray-800 hover:border-gray-600'}"
					>
						<div class="font-mono text-sm text-white">{truncatePrincipal(user.principal)}</div>
						<div class="flex gap-2 mt-2 flex-wrap">
							{#each user.alert_channels as ch}
								<span class="text-xs bg-gray-800 px-2 py-0.5 rounded">
									{channelIcons[ch.type]} {ch.type}
								</span>
							{/each}
						</div>
						<div class="text-xs text-gray-600 mt-2">
							Threshold: {user.alert_threshold} · Updated: {formatTimestamp(user.updated_at).split(',')[0]}
						</div>
					</button>
				{/each}
			</div>

			<!-- Detail panel -->
			<div class="lg:col-span-2">
				{#if selected}
					<div class="bg-gray-900 border border-gray-800 rounded-lg p-5 space-y-5">
						<div>
							<h3 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-2">Principal</h3>
							<code class="text-indigo-300 text-sm break-all">{selected.principal}</code>
						</div>

						<div class="grid grid-cols-2 gap-4 text-sm">
							<div>
								<span class="text-gray-500">Alert Threshold:</span>
								<span class="text-white ml-2">{selected.alert_threshold}</span>
							</div>
							<div>
								<span class="text-gray-500">Created:</span>
								<span class="text-white ml-2">{formatTimestamp(selected.created_at)}</span>
							</div>
						</div>

						<!-- Alert channels -->
						<div>
							<h3 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-3">Alert Channels</h3>
							<div class="space-y-2">
								{#each selected.alert_channels as ch}
									<div class="flex items-center gap-3 bg-gray-800 rounded p-3 text-sm">
										<span class="text-lg">{channelIcons[ch.type]}</span>
										<div>
											<div class="text-white font-semibold">{ch.type}</div>
											<div class="text-gray-500 font-mono text-xs truncate max-w-xs">{ch.target}</div>
										</div>
									</div>
								{/each}
							</div>
						</div>

						<!-- Detection rules -->
						<div>
							<h3 class="text-gray-400 text-xs font-semibold uppercase tracking-wider mb-3">Detection Rules</h3>
							<div class="space-y-2">
								{#each selected.detection_rules as rule}
									<div class="bg-gray-800 rounded p-3 text-sm flex items-start gap-3">
										<span class="mt-0.5 text-lg">{rule.enabled ? '✅' : '⬜'}</span>
										<div class="flex-1">
											<div class="flex items-center gap-2 flex-wrap">
												<span class="text-white font-semibold">[{rule.id}] {rule.name}</span>
												<span class="text-xs px-2 py-0.5 rounded {severityColors[rule.severity]}">{rule.severity}</span>
											</div>
											<div class="text-gray-500 text-xs mt-1">{rule.description}</div>
										</div>
									</div>
								{/each}
							</div>
						</div>
					</div>
				{:else}
					<div class="bg-gray-900 border border-gray-800 border-dashed rounded-lg p-12 text-center text-gray-600">
						Select a user to view their configuration
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
