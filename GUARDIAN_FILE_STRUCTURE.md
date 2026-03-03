# Guardian Phase 1 — File Organization

**Date:** Mar 2, 2026  
**Status:** Files organized and ready for development

---

## Directory Structure

```
/home/ranch/.openclaw/workspace/
├── guardian-icp/                    # ← MAIN CODEBASE (where Guardian-Dev works)
│   ├── src/
│   │   ├── lib.rs                   # Guardian Config Canister
│   │   ├── guardian.did             # Candid interface
│   │   └── tests/                   # Unit tests (to be created)
│   ├── Cargo.toml                   # Rust dependencies + build config
│   ├── Cargo.lock
│   ├── dfx.json                     # ICP network + build config
│   ├── QUICKSTART.md                # Day-1 setup guide
│   ├── README.md                    # (original, minimal)
│   ├── .git/                        # Git repo (linked to GitHub)
│   ├── .gitignore
│   └── target/                      # Build artifacts
│
├── guardian-dev/                    # ← PLANNING & DOCUMENTATION
│   ├── DEV_PLAN.md                  # Full 4-week Phase 1 plan
│   ├── DEV_LOG.md                   # Running changelog (Guardian-Dev updates)
│   ├── CANISTER_MANIFEST.md         # Canister IDs (devnet/mainnet)
│   ├── OISY_GUARDIAN_SPEC.md        # Full spec (40KB, reference)
│   └── (other Guardian planning files)
│
├── GUARDIAN_LAUNCH.md               # Status report (Mar 2)
├── GUARDIAN_FILE_STRUCTURE.md       # This file
│
├── agents_state.json                # ✅ Updated (guardian-dev → guardian-icp)
├── mission_control_dashboard.md     # ✅ Updated (references Guardian)
│
├── memory/                          # Long-term memory
│   ├── 2026-02-28.md
│   ├── projects.md
│   ├── security.md
│   ├── cost-optimization.md
│   └── ...
│
├── prospector/                      # Lead gen agent
├── creator/                         # Content agent
├── scripts/                         # Helper scripts
└── ... (other workspace files)
```

---

## GitHub Integration

**Repository:** https://github.com/ranchersurfer/icp-oisy-guardian

**Status:**
- ✅ Remote added: `origin` → GitHub repo
- ✅ Branch: `master` (clean)
- ⏳ Ready for guardian-dev commits

**Daily workflow:**
```bash
cd /home/ranch/.openclaw/workspace/guardian-icp
git add -A
git commit -m "feat: Phase 1a - implement rate limiting"
git push origin master
```

---

## File Locations Reference

| Document | Location | Purpose |
|----------|----------|---------|
| **Development Plan** | `guardian-dev/DEV_PLAN.md` | 4-week Phase 1 breakdown |
| **Quick Start** | `guardian-icp/QUICKSTART.md` | Day-1 setup + daily workflow |
| **Full Spec** | `guardian-dev/OISY_GUARDIAN_SPEC.md` | Architecture, rules, testing (40KB) |
| **Changelog** | `guardian-dev/DEV_LOG.md` | Weekly status (updated by Guardian-Dev) |
| **Launch Status** | `GUARDIAN_LAUNCH.md` | Phase 1 readiness report (Mar 2) |
| **Code** | `guardian-icp/src/lib.rs` | Guardian Config Canister (main focus) |
| **Build Config** | `guardian-icp/dfx.json` | ICP network + canister build settings |
| **Dependencies** | `guardian-icp/Cargo.toml` | Rust crate dependencies |

---

## Key Paths (For Guardian-Dev)

**Working directory:**
```bash
cd /home/ranch/.openclaw/workspace/guardian-icp
```

**Environment setup:**
```bash
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"
```

**Daily commands:**
```bash
# Build
cargo build --target wasm32-unknown-unknown --release

# Test
cargo test

# Deploy locally
dfx start --clean  # in Terminal 1
dfx deploy guardian_config --network local  # in Terminal 2

# Push to GitHub
git add -A
git commit -m "feat: ..."
git push origin master
```

---

## agents_state.json Update (Mar 2)

**What changed:**
- `guardian-dev.workspace` → `/home/ranch/.openclaw/workspace/guardian-icp`
- Added `github_repo` field
- Updated `task_status` → `active`
- Added Phase 1 milestones (1a–1e deadlines)
- Added dfx/Rust status

**Current state:**
```json
{
  "id": "guardian-dev",
  "workspace": "/home/ranch/.openclaw/workspace/guardian-icp",
  "github_repo": "https://github.com/ranchersurfer/icp-oisy-guardian",
  "task_status": "active",
  "start_date": "2026-03-02",
  "phase_deadline": "2026-04-01",
  "1a_deadline": "2026-03-08",
  "1b_deadline": "2026-03-15",
  "1c_deadline": "2026-03-22",
  "1d_deadline": "2026-03-29",
  "1e_deadline": "2026-04-01"
}
```

---

## Next Steps (Today, Mar 2)

1. ✅ Files organized
2. ✅ GitHub remote added
3. ✅ agents_state.json updated
4. ⏳ Guardian-Dev begins Phase 1a
5. ⏳ First commit to GitHub by Mar 3

---

## Verification

Check that everything is in place:

```bash
# Verify guardian-icp exists
ls -la /home/ranch/.openclaw/workspace/guardian-icp/

# Verify GitHub remote
cd /home/ranch/.openclaw/workspace/guardian-icp
git remote -v

# Verify dfx available
export PATH="/home/ranch/.local/share/dfx/bin:$PATH"
dfx --version

# Verify Rust available
cargo --version

# Verify agents_state.json updated
grep -A5 '"id": "guardian-dev"' /home/ranch/.openclaw/workspace/agents_state.json
```

---

**Status:** ✅ Files organized, GitHub linked, agents_state.json updated  
**Ready for:** Guardian-Dev Phase 1a launch (Mar 2)
