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

Areas: `kva` (KVA files through the original serializer, selected by `kind`; see the table below),
`render` (GDI+ rasterizations and hit tables of a case KVA), `posture` (generic posture tools parsed and moved by the original), `kinematics` (synthetic
trajectories, see `tests/cases/kinematics/README.md`), `timecode` (every `TimecodeFormat` for a list
of times and frame rates), `units`, `preferences`, `geometry` and `calibration` (seeded inputs; see
the table below).

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

## Areas

| Area | Case inputs | Outputs | Consumed by |
|---|---|---|---|
| `kva` | `kind`: `roundtrip` (default; `kva` path to a sample, optional `context` with `imageSize`, `averageTimeStampsPerFrame`, `averageTimeStampsPerSecond`, `baselineFrameInterval`, `firstTimeStamp`, `timeOrigin`: a different video, so the original remaps every time), `fuzz` (`count`, default 20), `fading`, `migration` (`kva`: a 1.3 or 1.5 file), `capture` (`kva`), `crossload` (`files`: KVA files written by the Rust port) | `roundtrip` (with or without `context`): `roundtrip.kva`, `summary.json` (counts), `drawings.json` (element, id and name of every drawing per container); `fuzz`: `fuzz/NN.kva` and `fuzz/NN.roundtrip.kva`; `fading`: `opacity.csv`; `migration`: `converted.kva` (XSLT output), `roundtrip.kva`, `summary.json`, `drawings.json`, `migration.json` (`relative_trajectories`); `capture`: `capture.kva`, `drawings.kva`, `calibration.kva`, `video.kva`; `crossload`: `summary.json` with one entry per file (`keyframes`, `attached_drawings`, `chronos`, `tracks`, `identical`: the original's save equals the input byte for byte), and `<name>.kva` | `kinovea-metadata` |
| `posture` | `file` (a tool XML in `Tools/DrawingTools/Custom`), `moves` (targets per handle, default 3) | `summary.json` (id, name, display name; counts of points, segments, circles, angles, polylines, handles, hit zones, distances, positions, computed points and options; flags; trackable points, point names, point and computed positions, option keys), `moves.csv` (`handle,options_on,move,target_x,target_y,point,x,y`: the positions after moving each handle to a seeded target, options off then on; circles appear as `circle<i>` with the radius in `x`) | `kinovea-metadata` |
| `render` | `kva` (a case KVA), `frame` (flat frame colour), `scale` (viewport scale), `timestamps`, `defaultFading`, `hitScales`, `hitStep`, `hitTimestamp`, `mad_max`/`outliers_max` (snapshot tolerances, fixed when the case is written) | `<timestamp>.png` (GDI+ rasterization with the player's `FlushOnGraphics`/`FlushDrawingsOnGraphics` settings and z-order), `hits.csv` (`container,index,element,timestamp,scale,x,y,result`, non-miss probes only; `exception` marks a drawing whose original `HitTest` throws) | `kinovea-render` |
| `kinematics` | synthetic trajectory parameters, optional `filtering`/`smoothing` flags | `linear.csv`, `angular.csv`, `filtering.json`, `truth.csv` | `kinovea-kinematics` |
| `timecode` | millisecond list, frame rates, precisions | `timecodes.csv`, `ms_to_timecode.csv` | `kinovea-core` |
| `units` | magnitudes, pixel-size tuples | `abbreviations.csv`, `velocity.csv`, `acceleration.csv`, `length_from_speed.csv`, `angular_velocity.csv`, `angular_acceleration.csv`, `frequency.csv`, `pixel_size.csv` | `kinovea-core` |
| `calibration` | `kind` (`transforms`, `kva`, `distortion`, `pose`), calibrations list, lens parameters, `agisoft.xml` | `transforms/{default,line-h,line-v,plane,plane-offset}.csv`, `kva/{line,plane,plane-distortion}.xml`, `distortion/{points,lines,agisoft}.csv`, `distortion/kinovea.xml`, `pose/cases.csv` | `kinovea-measurement` |
| `geometry` | `kind` (`clipping`, `homography`, `helpers`, `quads`, `svd`), `count`, `points`, `window` | `clipping/lines.csv`, `homography/{cases,points,vectors}.csv`, `helpers/{angles,points}.csv`, `quads/ops.csv`, `svd/cases.csv` | `kinovea-geometry` |
| `preferences` | optional input `Preferences.xml` | `Preferences.xml` as saved by the original (GUIDs normalised, Documents folder replaced by `{MyDocuments}`) | `kinovea-core` |
