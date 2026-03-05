#!/usr/bin/env bash
# Guardian ICP — Frontend Testnet Deployment Script
# Builds the SvelteKit static frontend and deploys it to an ICP asset canister.
#
# Usage:
#   ./scripts/deploy-frontend-testnet.sh [--network ic|testnet] [--dry-run]
#
# Prerequisites:
#   - dfx CLI installed and on PATH
#   - Identity with sufficient cycles (>= 2T cycles for asset canister creation)
#   - Backend canisters already deployed (guardian_config, guardian_engine)
#   - Node.js 18+
#
# To fund identity:
#   dfx ledger transfer <wallet-address> --amount 1.0 --network ic
#   dfx cycles convert --amount 0.5 --network ic

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FRONTEND_DIR="$PROJECT_ROOT/frontend"

# ─── Defaults ─────────────────────────────────────────────────────────────────
NETWORK="${DEPLOY_NETWORK:-ic}"
DRY_RUN=false
DFX_WARNING_FLAGS="DFX_WARNING=-mainnet_plaintext_identity"

# ─── Arg parsing ──────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case $1 in
    --network) NETWORK="$2"; shift 2 ;;
    --dry-run) DRY_RUN=true; shift ;;
    *) echo "Unknown argument: $1"; exit 1 ;;
  esac
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Guardian ICP — Frontend Deploy"
echo "  Network: $NETWORK"
echo "  Dry run: $DRY_RUN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# ─── Step 1: Get canister IDs ──────────────────────────────────────────────────
cd "$PROJECT_ROOT"
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"

echo ""
echo "📋 Step 1: Fetching canister IDs from dfx..."

ENGINE_ID=$(env $DFX_WARNING_FLAGS dfx canister id guardian_engine --network "$NETWORK" 2>/dev/null || echo "UNKNOWN")
CONFIG_ID=$(env $DFX_WARNING_FLAGS dfx canister id guardian_config --network "$NETWORK" 2>/dev/null || echo "UNKNOWN")

echo "  guardian_engine: $ENGINE_ID"
echo "  guardian_config: $CONFIG_ID"

if [[ "$ENGINE_ID" == "UNKNOWN" || "$CONFIG_ID" == "UNKNOWN" ]]; then
  echo ""
  echo "⚠️  WARNING: Backend canisters not found on $NETWORK."
  echo "   Deploy them first with: dfx deploy --network $NETWORK"
  echo "   Continuing with placeholder IDs for build..."
fi

# ─── Step 2: Build frontend ────────────────────────────────────────────────────
echo ""
echo "📦 Step 2: Building frontend..."
cd "$FRONTEND_DIR"

# Write a temp .env for the build
cat > .env.deploy <<EOF
VITE_CANISTER_NETWORK=$NETWORK
VITE_CANISTER_IDS={"guardian_engine":"$ENGINE_ID","guardian_config":"$CONFIG_ID"}
EOF

npm install --silent
npm run build -- --mode production

# Check bundle size
BUILD_SIZE=$(du -sh build/ 2>/dev/null | cut -f1)
echo "  Build size: $BUILD_SIZE"

# Cleanup temp env
rm -f .env.deploy

# ─── Step 3: Ensure asset canister in dfx.json ─────────────────────────────────
echo ""
echo "📌 Step 3: Checking dfx.json for guardian_frontend asset canister..."
cd "$PROJECT_ROOT"

if ! grep -q "guardian_frontend" dfx.json; then
  echo "  Adding guardian_frontend asset canister to dfx.json..."
  # Use node to update dfx.json safely
  node -e "
    const fs = require('fs');
    const config = JSON.parse(fs.readFileSync('dfx.json', 'utf8'));
    config.canisters.guardian_frontend = {
      type: 'assets',
      source: ['frontend/build']
    };
    fs.writeFileSync('dfx.json', JSON.stringify(config, null, 2) + '\n');
    console.log('  dfx.json updated.');
  "
else
  echo "  guardian_frontend already in dfx.json — OK."
fi

# ─── Step 4: Deploy ────────────────────────────────────────────────────────────
if [[ "$DRY_RUN" == "true" ]]; then
  echo ""
  echo "🔍 DRY RUN — would execute:"
  echo "  env $DFX_WARNING_FLAGS dfx deploy guardian_frontend --network $NETWORK"
  echo ""
  echo "✅ Dry run complete. Build is at frontend/build/"
  exit 0
fi

echo ""
echo "🚀 Step 4: Deploying asset canister to $NETWORK..."
env $DFX_WARNING_FLAGS dfx deploy guardian_frontend --network "$NETWORK"

# ─── Step 5: Output result ────────────────────────────────────────────────────
echo ""
FRONTEND_ID=$(env $DFX_WARNING_FLAGS dfx canister id guardian_frontend --network "$NETWORK" 2>/dev/null || echo "UNKNOWN")
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✅ Deployment complete!"
echo "  guardian_frontend: $FRONTEND_ID"
echo ""
echo "  Live at: https://$FRONTEND_ID.icp0.io"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
