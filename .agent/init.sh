#!/usr/bin/env bash
# Idempotent bootstrap for agents working on Kinovea. Safe to re-run.
# Does NOT start Docker/databases and contains no secrets.
set -euo pipefail
cd "$(dirname "$0")/.."

say() { printf '\033[1m[init]\033[0m %s\n' "$*"; }
warn() { printf '\033[33m[init] WARN:\033[0m %s\n' "$*"; }

# 1. Required binaries (harness only; the .NET build is Windows-only).
for bin in git node npm npx; do
  command -v "$bin" >/dev/null || { echo "[init] missing required binary: $bin" >&2; exit 1; }
done
say "git $(git --version | cut -d' ' -f3) · node $(node --version) · npm $(npm --version)"

# 2. Install harness dependencies only when package-lock.json changed.
HASH_FILE=.agent/.deps-hash.local
NEW_HASH=$(sha256sum package-lock.json | cut -d' ' -f1)
if [ ! -d node_modules ] || [ "$(cat "$HASH_FILE" 2>/dev/null || true)" != "$NEW_HASH" ]; then
  say "installing npm devDependencies (open-harness linters)"
  npm ci --no-audit --no-fund
  echo "$NEW_HASH" > "$HASH_FILE"
else
  say "npm deps up to date"
fi

# 3. Optional tooling.
command -v openspec >/dev/null && say "openspec $(openspec --version)" || warn "openspec CLI not found: npm install -g @fission-ai/openspec@latest"
if command -v lefthook >/dev/null; then
  lefthook install >/dev/null && say "lefthook hooks installed"
else
  warn "lefthook not found: pre-commit linters will not run locally"
fi

# 4. Build / type check. Kinovea is .NET Framework 4.8 + C++/CLI: MSBuild on Windows only.
if command -v msbuild >/dev/null; then
  say "msbuild found. Build with: msbuild Kinovea.VS2019.sln /p:Platform=x64 /p:Configuration=Release"
else
  warn "msbuild not available on this host; build is verified by GitHub Actions (.github/workflows/build.yml)"
fi

# 5. Quick checks (<60s): the four open-harness linters, report only.
say "running open-harness linters (report only; --fail is enforced by lefthook)"
for tool in linelens dupelens secretlens testlens; do
  npx "$tool" check --no-color 2>/dev/null | tail -1 | sed "s/^/  $tool: /"
done

say "done. Read AGENTS.md and .agent/progress.md before starting work."
