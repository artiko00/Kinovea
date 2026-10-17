#!/usr/bin/env bash
# Drives the parity oracle from Linux: build the C# solution on the Windows host,
# run Kinovea.FixtureGen there and pull the fixtures into tests/fixtures/.
#
#   tools/oracle/fixtures.sh            generate and pull fixtures for HEAD
#   tools/oracle/fixtures.sh --verify   run the generator twice on the host and diff; pull nothing
#   ORACLE_HOST=windows-ts tools/oracle/fixtures.sh   use another ssh alias
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

HOST=${ORACLE_HOST:-windows}
REMOTE_DIR='C:\oracle'
VERIFY=0
[ "${1:-}" = "--verify" ] && VERIFY=1
SSH=(ssh -o BatchMode=yes -o ConnectTimeout=10 "$HOST")

say() { printf '\033[1m[oracle]\033[0m %s\n' "$*"; }
die() { printf '\033[31m[oracle] ERROR:\033[0m %s\n' "$*" >&2; exit 1; }

"${SSH[@]}" 'echo ok' >/dev/null 2>&1 || die "host '$HOST' unreachable over ssh (set ORACLE_HOST to override)"

commit=$(git rev-parse HEAD)
[ -z "$(git status --porcelain -- tests/cases Kinovea.FixtureGen 2>/dev/null)" ] || die "uncommitted changes in tests/cases or Kinovea.FixtureGen; commit them first"

say "copying oracle scripts"
"${SSH[@]}" "if not exist $REMOTE_DIR mkdir $REMOTE_DIR" >/dev/null
scp -q tools/oracle/build.ps1 tools/oracle/provision.ps1 "$HOST:${REMOTE_DIR//\\//}/"

bundle_arg=""
if ! git merge-base --is-ancestor "$commit" origin/master 2>/dev/null; then
  say "HEAD not on origin/master; shipping a bundle"
  base=$(git merge-base HEAD origin/master)
  git bundle create /tmp/kinovea-oracle.bundle "$base..HEAD" >/dev/null 2>&1
  scp -q /tmp/kinovea-oracle.bundle "$HOST:${REMOTE_DIR//\\//}/kinovea.bundle"
  bundle_arg="-Bundle $REMOTE_DIR\\kinovea.bundle"
fi

say "building $commit on $HOST"
"${SSH[@]}" "powershell -NoProfile -ExecutionPolicy Bypass -File $REMOTE_DIR\\build.ps1 -Commit $commit $bundle_arg" || die "remote build failed"

gen='C:\oracle\Kinovea\Kinovea.FixtureGen\bin\x64\Release\Kinovea.FixtureGen.exe'
cases='C:\oracle\Kinovea\tests\cases'
out='C:\oracle\out'
"${SSH[@]}" "if exist $out rmdir /s /q $out" >/dev/null
if [ "$VERIFY" = 1 ]; then
  say "verify mode: generating twice and comparing on the host"
  "${SSH[@]}" "$gen --cases $cases --out $out --verify" || die "fixture generation is not deterministic"
  say "deterministic: OK"
  exit 0
fi

say "generating fixtures"
"${SSH[@]}" "$gen --cases $cases --out $out" || die "fixture generation failed"

say "pulling fixtures into tests/fixtures/"
mkdir -p tests/fixtures
scp -q -r "$HOST:C:/oracle/out/." tests/fixtures/
say "done: $(find tests/fixtures -name manifest.json | wc -l) fixture(s)"
