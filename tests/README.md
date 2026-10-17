# Parity oracle: cases and fixtures

The Rust port is tested against outputs of the original C# Kinovea. Those outputs are not
written by hand: `Kinovea.FixtureGen` (a C# console in the solution) runs the original code over
the cases in this directory on the Windows oracle host, and the results are committed here.

```
tests/
  cases/<area>/<name>.json        input case (hand-written, small, reviewable)
  fixtures/<area>/<name>/         generated outputs for that case + manifest.json
```

## Generate or refresh

```bash
tools/oracle/fixtures.sh            # build on the host, generate, pull into tests/fixtures/
tools/oracle/fixtures.sh --verify   # generate twice on the host and fail if the trees differ
ORACLE_HOST=windows-ts tools/oracle/fixtures.sh   # another ssh alias (default: windows)
```

Requirements: `ssh windows` works, the host was provisioned with `tools/oracle/provision.ps1 -Install`,
and `tests/cases/` plus `Kinovea.FixtureGen/` are committed (the host builds the exact commit).
Never edit `tests/fixtures/` by hand: change a case or the generator and regenerate.

## Case format

```json
{ "area": "kinematics", "seed": 42, "inputs": { "kind": "linear", "fps": 100 } }
```

Areas: `kva` (round-trip of a KVA file through the original serializer, see `tests/cases/kva`),
`kinematics` (synthetic trajectories, see `tests/cases/kinematics/README.md`), `timecode`
(every `TimecodeFormat` for a list of times and frame rates).

## Manifest

Every fixture directory has `manifest.json`:

```json
{ "case_sha256": "<sha256 of the case file>", "commit": "<generator commit>", "ffmpeg_version": "8.1.2" }
```

`kinovea-fixtures` (Rust) asserts that all manifests share one commit and that each checksum matches
its case, so stale fixtures fail `cargo test` loudly.

## Determinism and tolerances

The generator forces the invariant culture, fixed seeds, UTF-8 without BOM and LF endings, and
writes doubles with .NET round-trip (`R`) formatting. In `kva` fixtures every non-empty Guid is
replaced, in order of first appearance, by `00000000-0000-4000-8000-<n>`: the original importer mints
fresh ids for elements that lack one, so raw output would differ on every run. Rust tests compare with per-area tolerances
(`kinovea_fixtures::approx_eq`): exact for text and integers, declared epsilon for floating point
series. The KVA `Producer` element is fixed to `Kinovea.0.0` so it never churns.
