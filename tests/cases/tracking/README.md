# Tracking cases

Each case renders its frames by an integer formula (`Kinovea.FixtureGen/Areas/TrackingArea.Frames.cs`), so the
Rust port can build the same pixels:

- Background, per channel c (B, G, R): `clamp(background[c] + (x * gx + y * gy) / 16, 0, 255)` (integer division).
- Object: a square of side `size` or a disc of radius `size`, centered at `path[frame]` given in quarter pixels.
- Each pixel is sampled on a 4x4 grid at eighth-pixel offsets 1, 3, 5, 7. With the center `(cx, cy) = 2 * path[frame]`
  in eighths, a sample `(sx, sy) = (8x + i, 8y + j)` is inside the square when `|sx - cx| < 4 size` and
  `|sy - cy| < 4 size`, inside the disc when `(sx - cx)^2 + (sy - cy)^2 < (8 size)^2`. With `k` samples inside,
  the channel value is `(bg * (16 - k) + color[c] * k) / 16`; alpha is 255. Frames are BGRA, `Format32bppPArgb`.

The reference point is the first path point divided by 4 (integer pixels) at time 0; each later frame runs
`TrackStep` at `t = frame * tsPerFrame`. Outputs: `track.csv` (`frame,t,x,y,r,score,matched`; score is the
similarity of the last template, -1 for circles), `truth.csv` (`frame,x,y`) and, with `useMask`, `mask.csv`
(the elliptic mask the original paints with GDI+).
