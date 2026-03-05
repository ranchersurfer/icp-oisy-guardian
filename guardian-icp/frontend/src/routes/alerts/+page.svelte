<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchAlerts } from '$lib/mock';
	import { severityBadge, statusBadge, timeAgo, truncatePrincipal } from '$lib/utils';
	import type { AlertRecord, AlertSeverity, AlertStatus } from '$lib/types';

	let allAlerts: AlertRecord[] = [];
	let loading = true;

	// Filter state
	let filterUser = '';
	let filterSeverity: AlertSeverity | '' = '';
	let filterStatus: AlertStatus | '' = '';
	let search = '';

	// Pagination
	let page = 1;
	const PAGE_SIZE = 10;

	// Sort
	let sortKey: 'timestamp' | 'severity_score' = 'timestamp';
	let sortDir: 'asc' | 'desc' = 'desc';

	onMount(async () => {
		allAlerts = await fetchAlerts();
		loading = false;
	});

	function toggleSort(key: typeof sortKey) {
		if (sortKey === key) sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		else { sortKey = key; sortDir = 'desc'; }
		page = 1;
	}

	const severityOrder: Record<AlertSeverity, number> = { INFO: 1, WARN: 2, CRITICAL: 3, EMERGENCY: 4 };

	$: filtered = allAlerts
		.filter(a => {
			if (filterUser && a.user !== filterUser) return false;
			if (filterSeverity && a.severity !== filterSeverity) return false;
			if (filterStatus && a.status !== filterStatus) return false;
			if (search) {
				const q = search.toLowerCase();
				if (!a.alert_id.includes(q) && !a.events_summary.toLowerCase().includes(q) && !a.user.toLowerCase().includes(q)) return false;
			}
			return true;
		})
		.sort((a, b) => {
			let cmp = 0;
			if (sortKey === 'timestamp') cmp = Number(a.timestamp - b.timestamp);
			else cmp = a.severity_score - b.severity_score;
			return sortDir === 'asc' ? cmp : -cmp;
		});

	$: totalPages = Math.max(1, Math.ceil(filtered.length / PAGE_SIZE));
	$: pageAlerts = filtered.slice((page - 1) * PAGE_SIZE, page * PAGE_SIZE);
	$: uniqueUsers = [...new Set(allAlerts.map(a => a.user))];

	function resetPage() { page = 1; }
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold text-white">🚨 Alert History</h1>

	{#if loading}
		<div class="text-gray-500 animate-pulse">Loading alerts…</div>
	{:else}
		<!-- Filters -->
		<div class="bg-gray-900 border border-gray-800 rounded-lg p-4 flex flex-wrap gap-3">
			<input
				type="text"
				placeholder="Search alerts…"
				bind:value={search}
				on:input={resetPage}
				class="bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-indigo-500 w-48"
			/>
			<select
				bind:value={filterUser}
				on:change={resetPage}
				class="bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm text-white focus:outline-none focus:border-indigo-500"
			>
				<option value="">All users</option>
				{#each uniqueUsers as u}
					<option value={u}>{truncatePrincipal(u)}</option>
				{/each}
			</select>
			<select
				bind:value={filterSeverity}
				on:change={resetPage}
				class="bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm text-white focus:outline-none focus:border-indigo-500"
			>
				<option value="">All severities</option>
				<option value="INFO">INFO</option>
				<option value="WARN">WARN</option>
				<option value="CRITICAL">CRITICAL</option>
				<option value="EMERGENCY">EMERGENCY</option>
			</select>
			<select
				bind:value={filterStatus}
				on:change={resetPage}
				class="bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm text-white focus:outline-none focus:border-indigo-500"
			>
				<option value="">All statuses</option>
				<option value="Sent">Sent</option>
				<option value="Failed">Failed</option>
				<option value="Pending">Pending</option>
			</select>
			<span class="text-gray-500 text-sm self-center ml-auto">{filtered.length} alerts</span>
		</div>

		<!-- Table -->
		<div class="bg-gray-900 border border-gray-800 rounded-lg overflow-auto">
			<table class="w-full text-sm">
				<thead class="border-b border-gray-800 text-gray-500 text-xs uppercase">
					<tr>
						<th class="px-4 py-3 text-left">
							<button on:click={() => toggleSort('timestamp')} class="hover:text-white">
								Time {sortKey === 'timestamp' ? (sortDir === 'desc' ? '↓' : '↑') : ''}
							</button>
						</th>
						<th class="px-4 py-3 text-left">User</th>
						<th class="px-4 py-3 text-left">Chain</th>
						<th class="px-4 py-3 text-left">Rules</th>
						<th class="px-4 py-3 text-left">Severity</th>
						<th class="px-4 py-3 text-left">
							<button on:click={() => toggleSort('severity_score')} class="hover:text-white">
								Score {sortKey === 'severity_score' ? (sortDir === 'desc' ? '↓' : '↑') : ''}
							</button>
						</th>
						<th class="px-4 py-3 text-left">Status</th>
						<th class="px-4 py-3 text-left">Summary</th>
					</tr>
				</thead>
				<tbody>
					{#each pageAlerts as alert}
						<tr class="border-b border-gray-800 hover:bg-gray-800/50 transition-colors">
							<td class="px-4 py-3 text-gray-400 whitespace-nowrap">{timeAgo(alert.timestamp)}</td>
							<td class="px-4 py-3 font-mono text-xs text-indigo-300">{truncatePrincipal(alert.user)}</td>
							<td class="px-4 py-3 text-gray-300">{alert.chain}</td>
							<td class="px-4 py-3 text-gray-400 text-xs">{alert.rules_triggered.join(', ')}</td>
							<td class="px-4 py-3">
								<span class="px-2 py-0.5 rounded text-xs {severityBadge(alert.severity)}">{alert.severity}</span>
							</td>
							<td class="px-4 py-3 text-gray-300">{alert.severity_score}</td>
							<td class="px-4 py-3">
								<span class="px-2 py-0.5 rounded text-xs {statusBadge(alert.status)}">{alert.status}</span>
							</td>
							<td class="px-4 py-3 text-gray-400 text-xs max-w-xs truncate">{alert.events_summary}</td>
						</tr>
					{/each}
					{#if pageAlerts.length === 0}
						<tr>
							<td colspan="8" class="px-4 py-8 text-center text-gray-600">No alerts match the current filters</td>
						</tr>
					{/if}
				</tbody>
			</table>
		</div>

		<!-- Pagination -->
		<div class="flex items-center justify-between text-sm">
			<span class="text-gray-500">Page {page} of {totalPages}</span>
			<div class="flex gap-2">
				<button
					on:click={() => page = Math.max(1, page - 1)}
					disabled={page === 1}
					class="px-3 py-1.5 bg-gray-800 hover:bg-gray-700 disabled:opacity-30 rounded border border-gray-700"
				>← Prev</button>
				<button
					on:click={() => page = Math.min(totalPages, page + 1)}
					disabled={page === totalPages}
					class="px-3 py-1.5 bg-gray-800 hover:bg-gray-700 disabled:opacity-30 rounded border border-gray-700"
				>Next →</button>
			</div>
		</div>
	{/if}
</div>
