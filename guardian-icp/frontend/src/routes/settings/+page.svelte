<script lang="ts">
	import { goto } from '$app/navigation';
	import { authState } from '$lib/auth';
	import {
		beginEmailVerification,
		clearVerifiedEmail,
		confirmEmailVerification,
		getEmailVerificationStatus,
		getMyConfig,
		saveConfig
	} from '$lib/canister';
	import {
		PRESETS,
		detectPreset,
		formatLineList,
		parseLineList,
		formatPercent,
		formatRapidWindow,
		getPreset
	} from '$lib/guardian';
	import type { EmailVerificationStatus, GuardianConfigRecord, GuardianPresetId } from '$lib/types';

	let loading = true;
	let saving = false;
	let emailLoading = false;
	let emailSubmitting = false;
	let error = '';
	let success = '';
	let emailError = '';
	let emailSuccess = '';
	let emailChallengeCode = '';
	let existing: GuardianConfigRecord | null = null;
	let emailStatus: EmailVerificationStatus | null = null;
	let presetId: GuardianPresetId = 'balanced';
	let largeTransferPct = 0.5;
	let rapidTxCount = 5;
	let rapidTxWindowSecs = 600;
	let newAddressAlert = true;
	let alertThreshold = 7;
	let emergencyThreshold = 15;
	let monitoredChainsText = 'ICP\nckBTC\nckETH';
	let alertChannelsText = '';
	let allowlistedAddressesText = '';
	let emailInput = '';
	let verificationCode = '';

	function optText(value: [] | [string] | undefined): string {
		return value?.[0] ?? '';
	}

	function applyPreset(id: GuardianPresetId) {
		presetId = id;
		const preset = getPreset(id);
		largeTransferPct = preset.config.large_transfer_pct;
		rapidTxCount = preset.config.rapid_tx_count;
		rapidTxWindowSecs = preset.config.rapid_tx_window_secs;
		newAddressAlert = preset.config.new_address_alert;
		alertThreshold = preset.config.alert_threshold;
		emergencyThreshold = preset.config.emergency_threshold;
		monitoredChainsText = formatLineList(preset.config.monitored_chains);
	}

	function hydrateFromConfig(config: GuardianConfigRecord) {
		existing = config;
		presetId = detectPreset(config) ?? 'balanced';
		largeTransferPct = config.large_transfer_pct;
		rapidTxCount = config.rapid_tx_count;
		rapidTxWindowSecs = Number(config.rapid_tx_window_secs);
		newAddressAlert = config.new_address_alert;
		alertThreshold = config.alert_threshold;
		emergencyThreshold = config.emergency_threshold;
		monitoredChainsText = formatLineList(config.monitored_chains);
		alertChannelsText = formatLineList(config.alert_channels.filter((channel) => !channel.startsWith('email;')));
		allowlistedAddressesText = formatLineList(config.allowlisted_addresses);
	}

	async function loadEmailStatus() {
		emailLoading = true;
		try {
			const result = await getEmailVerificationStatus();
			if ('Err' in result) throw new Error(result.Err);
			emailStatus = result.Ok;
			emailInput = '';
		} finally {
			emailLoading = false;
		}
	}

	async function loadSettings() {
		if (!$authState.isAuthenticated) {
			await goto('/');
			return;
		}

		loading = true;
		error = '';
		try {
			const result = await getMyConfig();
			if ('Err' in result) {
				await goto('/onboarding');
				return;
			}
			hydrateFromConfig(result.Ok);
			await loadEmailStatus();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to load your settings.';
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		if (!existing) return;
		saving = true;
		error = '';
		success = '';

		try {
			const config: GuardianConfigRecord = {
				...existing,
				updated_at: BigInt(Date.now()) * BigInt(1_000_000),
				monitored_chains: parseLineList(monitoredChainsText),
				large_transfer_pct: Number(largeTransferPct),
				daily_outflow_pct: existing.daily_outflow_pct,
				rapid_tx_count: Number(rapidTxCount),
				rapid_tx_window_secs: BigInt(rapidTxWindowSecs),
				new_address_alert: newAddressAlert,
				alert_threshold: Number(alertThreshold),
				emergency_threshold: Number(emergencyThreshold),
				alert_channels: parseLineList(alertChannelsText),
				allowlisted_addresses: parseLineList(allowlistedAddressesText)
			};

			const writeResult = await saveConfig(config);
			if ('Err' in writeResult) throw new Error(writeResult.Err);

			const readBack = await getMyConfig();
			if ('Err' in readBack) throw new Error(`Saved, but read-back failed: ${readBack.Err}`);
			hydrateFromConfig(readBack.Ok);
			success = 'Settings saved. Unverified email is excluded from active delivery until confirmation.';
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Failed to save settings.';
		} finally {
			saving = false;
		}
	}

	async function requestVerification() {
		emailSubmitting = true;
		emailError = '';
		emailSuccess = '';
		emailChallengeCode = '';
		try {
			const result = await beginEmailVerification(emailInput);
			if ('Err' in result) throw new Error(result.Err);
			emailStatus = result.Ok.status;
			emailChallengeCode = result.Ok.verification_code;
			emailSuccess = result.Ok.demo_only
				? 'Verification challenge created. Demo build: the code is shown locally for smoke testing; delivery still stays inactive until you confirm it.'
				: 'Verification challenge created. Check your email for the confirmation code.';
			verificationCode = '';
		} catch (cause) {
			emailError = cause instanceof Error ? cause.message : 'Failed to start email verification.';
		} finally {
			emailSubmitting = false;
		}
	}

	async function submitVerificationCode() {
		emailSubmitting = true;
		emailError = '';
		emailSuccess = '';
		try {
			const result = await confirmEmailVerification(verificationCode);
			if ('Err' in result) throw new Error(result.Err);
			emailStatus = result.Ok;
			emailChallengeCode = '';
			emailSuccess = 'Email verified. It can now be treated as an active delivery destination once your backend email transport is configured.';
			await loadSettings();
		} catch (cause) {
			emailError = cause instanceof Error ? cause.message : 'Failed to verify email.';
		} finally {
			emailSubmitting = false;
		}
	}

	async function removeVerifiedEmail() {
		emailSubmitting = true;
		emailError = '';
		emailSuccess = '';
		try {
			const result = await clearVerifiedEmail();
			if ('Err' in result) throw new Error(result.Err);
			emailStatus = result.Ok;
			emailChallengeCode = '';
			emailSuccess = 'Verified email removed from active delivery.';
			await loadSettings();
		} catch (cause) {
			emailError = cause instanceof Error ? cause.message : 'Failed to clear verified email.';
		} finally {
			emailSubmitting = false;
		}
	}

	loadSettings();
</script>

<div class="space-y-6 sm:space-y-8">
	<div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
		<div>
			<div class="guardian-kicker">Settings</div>
			<h1 class="guardian-section-title">Edit your Guardian settings</h1>
			<p class="mt-3 max-w-3xl text-sm leading-6 text-slate-300 sm:text-base">Preset tuning, masked verified-email management, monitored chains, other destinations, and trusted addresses.</p>
		</div>
		<a href="/onboarding?mode=edit" class="guardian-button-secondary min-h-12">Re-run onboarding</a>
	</div>

	{#if loading}
		<div class="guardian-card text-slate-300">Loading your live settings…</div>
	{:else}
		<div class="grid gap-4 xl:grid-cols-3">
			{#each PRESETS as preset}
				<button
					on:click={() => applyPreset(preset.id)}
					class={`rounded-[1.75rem] border p-5 text-left transition sm:p-6 ${presetId === preset.id ? 'border-cyan-300 bg-cyan-400/10 shadow-[0_0_0_1px_rgba(103,232,249,0.35),0_18px_55px_rgba(8,145,178,0.18)]' : 'border-white/10 bg-slate-900/70 hover:border-white/20 hover:bg-slate-900/90'}`}
				>
					<div class="flex items-start justify-between gap-3">
						<div>
							<div class="text-xl font-semibold text-white">{preset.name}</div>
							<div class="mt-1 text-sm text-slate-400">{preset.tagline}</div>
						</div>
						{#if preset.recommended}
							<div class="guardian-badge border-cyan-300/20 bg-cyan-400 text-slate-950">Recommended</div>
						{/if}
					</div>
					<div class="mt-4 space-y-2 text-sm text-slate-200">
						<div>Large transfer alert: {formatPercent(preset.config.large_transfer_pct)}</div>
						<div>Rapid tx rule: {preset.config.rapid_tx_count} tx in {formatRapidWindow(preset.config.rapid_tx_window_secs)}</div>
					</div>
				</button>
			{/each}
		</div>

		<div class="grid gap-6 xl:grid-cols-[1.06fr_0.94fr]">
			<div class="guardian-card space-y-6">
				<div>
					<h2 class="text-2xl font-semibold text-white">Advanced controls</h2>
					<p class="mt-2 text-sm leading-6 text-slate-400">Preset changes are reversible. Use these controls when you want more granular behavior than a default profile.</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<label class="space-y-2"><span class="text-sm text-slate-300">Large transfer alert (% of balance)</span><input bind:value={largeTransferPct} type="number" min="0.01" max="1" step="0.01" class="guardian-input min-h-12" /></label>
					<label class="space-y-2"><span class="text-sm text-slate-300">Rapid transaction count</span><input bind:value={rapidTxCount} type="number" min="1" step="1" class="guardian-input min-h-12" /></label>
					<label class="space-y-2"><span class="text-sm text-slate-300">Rapid transaction window (seconds)</span><input bind:value={rapidTxWindowSecs} type="number" min="60" step="60" class="guardian-input min-h-12" /></label>
					<label class="space-y-2"><span class="text-sm text-slate-300">Alert sensitivity threshold</span><input bind:value={alertThreshold} type="number" min="1" step="1" class="guardian-input min-h-12" /></label>
					<label class="space-y-2 sm:col-span-2"><span class="text-sm text-slate-300">Emergency threshold</span><input bind:value={emergencyThreshold} type="number" min="1" step="1" class="guardian-input min-h-12" /></label>
				</div>

				<label class="flex items-start gap-3 rounded-[1.4rem] border border-white/10 bg-white/5 px-4 py-4 text-sm leading-6 text-slate-200">
					<input bind:checked={newAddressAlert} type="checkbox" class="mt-1 h-4 w-4 rounded border-white/20 bg-slate-950 text-cyan-400" />
					<span>Alert me when funds move to a new address that hasn’t been allowlisted yet.</span>
				</label>
			</div>

			<div class="guardian-card space-y-6">
				<div>
					<h2 class="text-2xl font-semibold text-white">Verified email delivery</h2>
					<p class="mt-2 text-sm leading-6 text-slate-400">Email is managed separately from the raw destination list. Pending email stays masked and is not treated as active delivery until confirmation completes.</p>
				</div>

				{#if emailLoading}
					<div class="rounded-2xl border border-white/10 bg-white/5 px-4 py-4 text-sm text-slate-300">Loading email status…</div>
				{:else}
					<div class="rounded-[1.4rem] border border-white/10 bg-white/5 px-4 py-4 text-sm text-slate-200 space-y-2">
						<div><span class="text-slate-400">Verified email:</span> {optText(emailStatus?.verified_email_masked) || 'None'}</div>
						<div><span class="text-slate-400">Pending email:</span> {optText(emailStatus?.pending_email_masked) || 'None'}</div>
						<div><span class="text-slate-400">Delivery status:</span> {emailStatus?.delivery_active ? 'Active for verified email only' : 'No verified email delivery active'}</div>
					</div>

					<label class="block space-y-2">
						<span class="text-sm text-slate-300">New email address</span>
						<input bind:value={emailInput} type="email" placeholder="name@example.com" class="guardian-input min-h-12" />
					</label>
					<div class="flex flex-col gap-3 sm:flex-row">
						<button on:click={requestVerification} disabled={emailSubmitting || !emailInput} class="guardian-button-primary min-h-12 disabled:opacity-60">{emailSubmitting ? 'Working…' : 'Send verification challenge'}</button>
						<button on:click={removeVerifiedEmail} disabled={emailSubmitting || !emailStatus?.verified_email_masked?.length} class="guardian-button-secondary min-h-12 disabled:opacity-60">Remove verified email</button>
					</div>

					<label class="block space-y-2">
						<span class="text-sm text-slate-300">Verification code</span>
						<input bind:value={verificationCode} inputmode="numeric" maxlength="6" placeholder="123456" class="guardian-input min-h-12" />
					</label>
					<button on:click={submitVerificationCode} disabled={emailSubmitting || verificationCode.trim().length < 6} class="guardian-button-secondary min-h-12 disabled:opacity-60">Confirm code</button>

					{#if emailChallengeCode}
						<div class="rounded-2xl border border-amber-400/30 bg-amber-400/10 px-4 py-3 text-sm text-amber-100">Demo-only smoke path: current verification code is <span class="font-mono">{emailChallengeCode}</span>. This is displayed locally so the verified-email gate can be tested without wiring an external mail sender yet.</div>
					{/if}
					{#if emailError}
						<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">{emailError}</div>
					{/if}
					{#if emailSuccess}
						<div class="rounded-2xl border border-emerald-400/30 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-100">{emailSuccess}</div>
					{/if}
				{/if}
			</div>
		</div>

		<div class="guardian-card space-y-6">
			<div>
				<h2 class="text-2xl font-semibold text-white">Other destinations and allowlist</h2>
				<p class="mt-2 text-sm leading-6 text-slate-400">Email no longer belongs in the raw destination list. Use Discord, Slack, or generic webhook lines here. Verified email is handled above and remains masked in the UI.</p>
			</div>

			<label class="block space-y-2"><span class="text-sm text-slate-300">Monitored chains</span><textarea bind:value={monitoredChainsText} rows="4" class="guardian-textarea"></textarea></label>
			<label class="block space-y-2"><span class="text-sm text-slate-300">Alert destinations</span><textarea bind:value={alertChannelsText} rows="6" class="guardian-textarea"></textarea></label>
			<label class="block space-y-2"><span class="text-sm text-slate-300">Trusted / allowlisted addresses</span><textarea bind:value={allowlistedAddressesText} rows="6" class="guardian-textarea"></textarea></label>
		</div>

		<div class="guardian-card border-cyan-400/20 bg-cyan-400/10">
			<div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
				<div class="text-sm leading-6 text-cyan-50">Writes to live <code>guardian_config.set_config()</code>. The backend now sanitizes raw email entries, preserving privacy hardening and ensuring only verified email can appear as an active delivery channel.</div>
				<div class="flex flex-col gap-3 sm:flex-row">
					<button on:click={saveSettings} disabled={saving || !existing} class="guardian-button-primary min-h-12 disabled:opacity-60">{saving ? 'Saving to live canister…' : 'Save settings'}</button>
					<a href="/dashboard" class="guardian-button-secondary min-h-12">Back to dashboard</a>
				</div>
			</div>
		</div>

		{#if error}
			<div class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">{error}</div>
		{/if}
		{#if success}
			<div class="rounded-2xl border border-emerald-400/30 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-100">{success}</div>
		{/if}
	{/if}
</div>
