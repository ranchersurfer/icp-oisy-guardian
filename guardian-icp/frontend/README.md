# Guardian ICP Admin Dashboard

A lightweight SvelteKit admin dashboard for the Guardian ICP canister system. Provides read-only visibility into canister health, user configurations, alert history, and system statistics.

## Pages

| Page | Route | Description |
|------|-------|-------------|
| Health Status | `/` | Real-time engine health: cycles, timer state, watermark count, alert queue |
| Configuration | `/config` | Active users, alert channels, detection rules (read-only) |
| Alert History | `/alerts` | Caller-scoped personal alert history from `guardian_engine.get_my_alerts(limit)` |
| System Stats | `/stats` | Delivery rates, chain/severity breakdowns, uptime |

## Architecture

```
frontend/
├── src/
│   ├── lib/
│   │   ├── types.ts            # TypeScript types mirroring guardian_engine Candid types
│   │   ├── canister.ts         # Real @dfinity/agent integration (Phase 4)
│   │   ├── mock.ts             # Mock data layer (fallback when canister unreachable)
│   │   ├── utils.ts            # Formatters, color helpers
│   │   └── idl/
│   │       ├── guardian_engine.idl.ts  # IDL factory for guardian_engine
│   │       └── guardian_config.idl.ts  # IDL factory for guardian_config
│   └── routes/
│       ├── +layout.svelte      # Navigation shell (live/mock indicator)
│       ├── +page.svelte        # Health page
│       ├── config/             # Config page
│       ├── alerts/             # Alert history page
│       └── stats/              # System stats page
├── .env.example                # Environment variable template
├── .env.local                  # Local dev config (gitignored)
├── svelte.config.js            # adapter-static (canister asset canister compatible)
└── vite.config.ts              # Tailwind CSS v4 via @tailwindcss/vite
```

### Data Flow

`canister.ts` wraps real `@dfinity/agent` calls with graceful fallback to mock data:

```
fetchHealth()  → guardian_engine.get_health()         [live]
fetchUsers()   → guardian_config.get_config()          [live, anonymous principal]
fetchAlerts()  → guardian_engine.get_my_alerts(limit) [live, caller-scoped, no mock fallback]
fetchStats()   → derived from health + alert counts    [mixed]
```

Private alert history does **not** fall back to mock data. In normal consumer mode, `/alerts` only renders caller-scoped live data from the engine canister.

## Environment Setup

Copy `.env.example` to `.env.local` and configure:

```bash
cp .env.example .env.local
```

Key variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_CANISTER_NETWORK` | `local` | `local` \| `testnet` \| `ic` |
| `VITE_CANISTER_IDS` | local IDs | JSON: `{"guardian_engine":"...","guardian_config":"..."}` |
| `VITE_ENGINE_CANISTER_ID` | — | Alternative to VITE_CANISTER_IDS |
| `VITE_CONFIG_CANISTER_ID` | — | Alternative to VITE_CANISTER_IDS |
| `VITE_IC_HOST` | inferred | Override IC host URL |
| `VITE_USE_MOCK` | `false` | Force mock data mode |

### Local Development (with dfx replica)

```bash
# 1. Start local replica
cd /home/ranch/.openclaw/workspace/guardian-icp
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"
dfx start --background

# 2. Deploy canisters
dfx deploy

# 3. Start frontend dev server
cd frontend
cp .env.example .env.local  # edit to set local canister IDs
npm install
npm run dev
# → http://localhost:5173
```

Get local canister IDs with:
```bash
dfx canister id guardian_engine   # → u6s2n-gx777-77774-qaaba-cai (local)
dfx canister id guardian_config   # → uxrrr-q7777-77774-qaaaq-cai (local)
```

### Mock Mode (no canister needed)

```bash
cd frontend
echo "VITE_USE_MOCK=true" > .env.local
npm run dev
```

## Build

```bash
npm run build
# Output: frontend/build/ (~412KB, well under 2MB canister asset limit)
```

## Testnet Deployment

Use the provided deployment script:

```bash
# Deploy to IC mainnet
./scripts/deploy-frontend-testnet.sh --network ic

# Deploy to testnet
./scripts/deploy-frontend-testnet.sh --network testnet

# Dry run (build only, don't deploy)
./scripts/deploy-frontend-testnet.sh --dry-run
```

The script:
1. Reads canister IDs from `dfx canister id` for the target network
2. Builds the frontend with correct canister IDs baked in
3. Adds `guardian_frontend` asset canister to `dfx.json` if missing
4. Deploys via `dfx deploy guardian_frontend --network <network>`

After deployment, the dashboard is live at:
```
https://<guardian_frontend-canister-id>.icp0.io
```

### Manual Deployment Steps

```bash
# 1. Add frontend canister to dfx.json:
#    "guardian_frontend": { "type": "assets", "source": ["frontend/build"] }

# 2. Build with production env
VITE_CANISTER_NETWORK=ic \
VITE_CANISTER_IDS='{"guardian_engine":"<id>","guardian_config":"<id>"}' \
npm run build

# 3. Deploy
DFX_WARNING=-mainnet_plaintext_identity dfx deploy guardian_frontend --network ic

# 4. View
dfx canister id guardian_frontend --network ic
# → https://<id>.icp0.io
```

### Prerequisites for Testnet/IC Deployment

```bash
# Fund identity (requires ICP in wallet)
dfx ledger transfer <wallet-address> --amount 1.0 --network ic
dfx cycles convert --amount 0.5 --network ic

# Check balance
dfx cycles balance --network ic
# Need: ~2T cycles for asset canister creation
```

## Security Notes

- **Read-only UI**: No mutations are possible from the dashboard
- **No hardcoded secrets**: All canister IDs loaded from environment at build time
- **Mock mode**: `VITE_USE_MOCK=true` uses mock data (default in dev via env.local)
- Alert channel targets shown as `**REDACTED**` in mock data
- Anonymous identity used for queries (admin auth deferred to Phase 5)

## Tech Stack

- [SvelteKit](https://svelte.dev/docs/kit) — framework
- [Tailwind CSS v4](https://tailwindcss.com) — styling (dark theme)
- [@dfinity/agent](https://www.npmjs.com/package/@dfinity/agent) — ICP canister calls
- [@sveltejs/adapter-static](https://www.npmjs.com/package/@sveltejs/adapter-static) — static export for asset canisters
- TypeScript — type safety

## Phase History

| Phase | Description |
|-------|-------------|
| Phase 3 | SvelteKit dashboard built with mock data |
| **Phase 4** | **Real @dfinity/agent integration, env config, testnet deploy script** |
