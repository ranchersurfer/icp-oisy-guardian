# Guardian ICP Admin Dashboard

A lightweight SvelteKit admin dashboard for the Guardian ICP canister system. Provides read-only visibility into canister health, user configurations, alert history, and system statistics.

## Pages

| Page | Route | Description |
|------|-------|-------------|
| Health Status | `/` | Real-time engine health: cycles, timer state, watermark count, alert queue |
| Configuration | `/config` | Active users, alert channels, detection rules (read-only) |
| Alert History | `/alerts` | Last 100 alerts, filterable + sortable + paginated |
| System Stats | `/stats` | Delivery rates, chain/severity breakdowns, uptime |

## Architecture

```
frontend/
├── src/
│   ├── lib/
│   │   ├── types.ts       # TypeScript types mirroring guardian_engine Candid types
│   │   ├── mock.ts        # Mock data layer (replace with @dfinity/agent calls)
│   │   └── utils.ts       # Formatters, color helpers
│   └── routes/
│       ├── +layout.svelte # Navigation shell
│       ├── +page.svelte   # Health page
│       ├── config/        # Config page
│       ├── alerts/        # Alert history page
│       └── stats/         # System stats page
├── svelte.config.js       # adapter-static (canister asset canister compatible)
└── vite.config.ts         # Tailwind CSS v4 via @tailwindcss/vite
```

### Data Flow (Production)

Replace mock functions in `src/lib/mock.ts` with real `@dfinity/agent` calls:

```ts
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from './guardian_engine.did.js';

const agent = new HttpAgent({ host: 'https://ic0.app' });
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId: GUARDIAN_ENGINE_CANISTER_ID
});

// Example: replace fetchHealth()
export async function fetchHealth(): Promise<CanisterHealth> {
  const health = await actor.get_health();
  const queueLen = await actor.get_alert_queue_len();
  return { engine: health, alert_queue_len: queueLen, config_canister_id: null };
}
```

### Canister Query Endpoints Used

| Endpoint | Canister | Returns |
|----------|----------|---------|
| `get_health()` | guardian_engine | `EngineHealthStatus` |
| `get_alert_queue_len()` | guardian_engine | `nat64` |
| `get_config_for_user(principal)` | guardian_config | `ApiResult<GuardianConfig>` |

## Local Development

```bash
cd frontend/
npm install
npm run dev
# → http://localhost:5173
```

## Build

```bash
npm run build
# Output: frontend/build/ (~200KB, well under 2MB canister asset limit)
```

## Deploy to ICP Asset Canister

1. Add a `frontend` canister to `dfx.json`:
   ```json
   "guardian_frontend": {
     "type": "assets",
     "source": ["frontend/build"]
   }
   ```
2. Build the frontend: `npm run build`
3. Deploy: `dfx deploy guardian_frontend --network ic`
4. Access at: `https://<canister-id>.icp0.io`

## Security Notes

- **Read-only UI**: No mutations are possible from the dashboard
- **No hardcoded secrets**: All canister IDs are loaded from environment or passed at runtime
- **Mock mode**: `VITE_USE_MOCK=true` uses mock data (default in dev)
- All sensitive targets in alert channels are shown as `**REDACTED**` in mock data

## Tech Stack

- [SvelteKit](https://svelte.dev/docs/kit) — framework
- [Tailwind CSS v4](https://tailwindcss.com) — styling (dark theme)
- [@sveltejs/adapter-static](https://www.npmjs.com/package/@sveltejs/adapter-static) — static export for asset canisters
- TypeScript — type safety

## Phase

This dashboard was built as **Phase 3** of Guardian ICP development.
See `/guardian-dev/DEV_LOG.md` for full changelog.
