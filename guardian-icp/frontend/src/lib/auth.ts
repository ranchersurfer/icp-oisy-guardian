import { writable } from 'svelte/store';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent, Actor } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { idlFactory as configIdlFactory } from './idl/guardian_config.idl.js';
import { idlFactory as engineIdlFactory } from './idl/guardian_engine.idl.js';

function getEnv(key: string): string | undefined {
	return (import.meta.env as Record<string, string | undefined>)[key];
}

const NETWORK = getEnv('VITE_CANISTER_NETWORK') ?? 'local';
const HOST_MAP: Record<string, string> = {
	local: 'http://127.0.0.1:4943',
	testnet: 'https://icp0.io',
	ic: 'https://icp0.io',
	mainnet: 'https://icp0.io'
};
const IC_HOST = getEnv('VITE_IC_HOST') ?? HOST_MAP[NETWORK] ?? 'https://icp0.io';
const LOCAL_ENGINE_ID = 'u6s2n-gx777-77774-qaaba-cai';
const LOCAL_CONFIG_ID = 'uxrrr-q7777-77774-qaaaq-cai';

function getCanisterIds(): { engine: string; config: string } {
	const raw = getEnv('VITE_CANISTER_IDS');
	if (raw) {
		try {
			const parsed = JSON.parse(raw) as Record<string, string>;
			return {
				engine: parsed.guardian_engine ?? LOCAL_ENGINE_ID,
				config: parsed.guardian_config ?? LOCAL_CONFIG_ID
			};
		} catch {
			// noop
		}
	}
	return {
		engine: getEnv('VITE_ENGINE_CANISTER_ID') ?? LOCAL_ENGINE_ID,
		config: getEnv('VITE_CONFIG_CANISTER_ID') ?? LOCAL_CONFIG_ID
	};
}

export const CANISTER_IDS = getCanisterIds();
const IS_LOCAL = NETWORK === 'local';
const DEV_II_URL = 'http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943';
const PROD_II_URL = 'https://identity.ic0.app';

export interface AuthState {
	ready: boolean;
	connecting: boolean;
	isAuthenticated: boolean;
	principal: string | null;
	error: string | null;
}

const defaultState: AuthState = {
	ready: false,
	connecting: false,
	isAuthenticated: false,
	principal: null,
	error: null
};

export const authState = writable<AuthState>(defaultState);

let authClient: AuthClient | null = null;
let currentIdentity: Identity | null = null;
let currentAgent: HttpAgent | null = null;

async function createAgent(identity?: Identity): Promise<HttpAgent> {
	const agent = new HttpAgent({ host: IC_HOST, identity });
	if (IS_LOCAL) {
		await agent.fetchRootKey().catch(() => undefined);
	}
	return agent;
}

export async function initAuth(): Promise<void> {
	if (typeof window === 'undefined') return;
	if (!authClient) {
		authClient = await AuthClient.create();
	}

	const isAuthenticated = await authClient.isAuthenticated();
	currentIdentity = isAuthenticated ? authClient.getIdentity() : null;
	currentAgent = await createAgent(currentIdentity ?? undefined);

	authState.set({
		ready: true,
		connecting: false,
		isAuthenticated,
		principal: isAuthenticated ? currentIdentity?.getPrincipal().toText() ?? null : null,
		error: null
	});
}

export async function login(): Promise<void> {
	if (typeof window === 'undefined') return;
	if (!authClient) authClient = await AuthClient.create();
	authState.update((state) => ({ ...state, connecting: true, error: null }));

	await new Promise<void>((resolve, reject) => {
		authClient!.login({
			identityProvider: IS_LOCAL ? DEV_II_URL : PROD_II_URL,
			onSuccess: () => resolve(),
			onError: (error) => reject(error),
			maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1_000_000_000)
		});
	});

	await initAuth();
}

export async function logout(): Promise<void> {
	if (!authClient) return;
	await authClient.logout();
	currentIdentity = null;
	currentAgent = await createAgent();
	authState.set({ ...defaultState, ready: true });
}

export async function getAgent(): Promise<HttpAgent> {
	if (!currentAgent) {
		currentAgent = await createAgent(currentIdentity ?? undefined);
	}
	return currentAgent;
}

export async function createConfigActor() {
	const agent = await getAgent();
	return Actor.createActor(configIdlFactory as never, {
		agent,
		canisterId: CANISTER_IDS.config
	}) as any;
}

export async function createEngineActor() {
	const agent = await getAgent();
	return Actor.createActor(engineIdlFactory as never, {
		agent,
		canisterId: CANISTER_IDS.engine
	}) as any;
}
