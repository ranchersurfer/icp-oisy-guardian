<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	const nav = [
		{ href: '/', label: '🏥 Health', id: 'health' },
		{ href: '/config', label: '⚙️ Config', id: 'config' },
		{ href: '/alerts', label: '🚨 Alerts', id: 'alerts' },
		{ href: '/stats', label: '📊 Stats', id: 'stats' }
	];
</script>

<div class="min-h-screen bg-gray-950 text-gray-100 font-mono">
	<!-- Top bar -->
	<header class="bg-gray-900 border-b border-gray-800 px-6 py-3 flex items-center gap-6">
		<div class="flex items-center gap-2">
			<span class="text-2xl">🛡️</span>
			<span class="text-lg font-bold text-white">Guardian</span>
			<span class="text-gray-500 text-sm">Admin Dashboard</span>
		</div>
		<nav class="flex gap-2 ml-4">
			{#each nav as { href, label }}
				<a
					{href}
					class="px-3 py-1.5 rounded text-sm transition-colors
						{$page.url.pathname === href
							? 'bg-indigo-700 text-white'
							: 'text-gray-400 hover:text-white hover:bg-gray-800'}"
				>
					{label}
				</a>
			{/each}
		</nav>
		<div class="ml-auto flex items-center gap-2 text-xs text-gray-500">
			<span class="inline-block w-2 h-2 rounded-full bg-green-500"></span>
			<span>Mock Data Mode</span>
		</div>
	</header>

	<!-- Main content -->
	<main class="max-w-7xl mx-auto px-6 py-8">
		<slot />
	</main>

	<footer class="text-center text-gray-700 text-xs py-4 border-t border-gray-900">
		Guardian ICP Admin Dashboard · Read-only · All mutations require backend auth
	</footer>
</div>
