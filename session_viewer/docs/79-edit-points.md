# 79 Edit points — drag the curve itself

> **Big picture.** *Phase 13.* CVs (78) are honest but unintuitive — they float *off* the curve and
> pull like magnets. Users expect to grab **the curve itself**. Edit points do that: points *on* the
> curve at the **Greville abscissae** (each CV's natural parameter), draggable — and the kernel
> solves the "which CVs must move so the curve passes through the dragged point" problem as a small
> linear refit. Same UI as 73, one linear solve deeper.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="control points float off the curve and pull; edit points lie on the curve at greville parameters and dragging one refits the cvs so the curve passes through it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(20,10)">
    <path d="M 10,80 C 70,20 150,100 210,40" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <g fill="none" stroke="#888"><circle cx="10" cy="80" r="4"/><circle cx="70" cy="35" r="4"/><circle cx="150" cy="85" r="4"/><circle cx="210" cy="40" r="4"/></g>
    <line x1="10" y1="80" x2="70" y2="35" stroke="#3a3a3a" stroke-dasharray="3 3"/><line x1="70" y1="35" x2="150" y2="85" stroke="#3a3a3a" stroke-dasharray="3 3"/><line x1="150" y1="85" x2="210" y2="40" stroke="#3a3a3a" stroke-dasharray="3 3"/>
    <text x="110" y="118" fill="#888" text-anchor="middle">CVs: off the curve (78)</text>
  </g>
  <g transform="translate(330,10)">
    <path d="M 10,80 C 70,20 150,100 210,40" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <g fill="#e0b040"><circle cx="10" cy="80" r="4"/><circle cx="63" cy="52" r="4"/><circle cx="148" cy="70" r="4"/><circle cx="210" cy="40" r="4"/></g>
    <text x="110" y="118" fill="#888" text-anchor="middle">edit points: ON the curve — Greville</text>
  </g>
  <text x="620" y="60" fill="#666" font-size="10" text-anchor="middle">drag one →</text>
  <text x="620" y="74" fill="#666" font-size="10" text-anchor="middle">refit CVs</text>
</svg>

## The math, in one breath

The curve evaluated at every Greville abscissa gives the edit points: `E = R · P`, where `P` are the
CVs and `R` is the square matrix of basis functions sampled at those parameters (`R[i][j] =
N_j(greville_i)`). Dragging edit point `k` changes `E` by a known delta — the CVs that produce the
new curve are `P' = R⁻¹ · E'`. R is small (cv_count × cv_count), banded, well-conditioned for
clamped curves; one LU solve per drag *release* (the live drag can reuse the factorization). Weights:
work on the **projected** 3-D points and write back through `set_cv_4d` with each CV's original `w` —
78's homogeneous rule, unchanged.

## Files we touch

```
src/app/scene.rs   # greville_points; refit_through (build R once per curve, solve on drag)
src/state.rs       # F10 + modifier switches CV-mode ↔ edit-point-mode; same drag skeleton
```

## Step 1 — the points: `src/app/scene.rs`

The kernel hands us the parameters directly:

```rust
    /// The curve's edit points: evaluate at each Greville abscissa. These lie ON the curve.
    pub fn greville_points(nc: &NurbsCurve) -> Vec<(f64, Point)> {
        nc.get_greville_abcissae().into_iter()
            .map(|t| (t, nc.point_at(t)))
            .collect()
    }
```

In edit-point mode these render as glyphs (amber, to distinguish from 78's CVs) and pick with the
same screen radius. (`greville_points` evaluates in the curve's LOCAL frame — the glyphs ride the
row's placed frame like every vertex glyph, so nothing converts here; the *drag*, which comes back
in world coordinates, converts in Step 2.) Surfaces get the tensor version — Greville in u ×
Greville in v — same idea, a grid of on-surface handles; start with curves, the surface loop is the
same code twice.

## Step 2 — the refit: `src/app/scene.rs`

The cache type first — `R` depends only on knots/degree, so one inversion serves every drag. The
matrix builds **numerically**: displace CV `j` by a unit (homogeneous, weight untouched),
re-evaluate at every Greville parameter — column `j` falls out, no basis-function internals needed,
and it matches the analytic matrix to ~1e-14. The probe is `nc.duplicate()`, and note the kernel
contract there: `duplicate()` **mints a fresh guid** (it clears the guid slot; a new one generates
lazily on first read). For a throwaway probe that is exactly right — it can never collide with the
real object — and the `GrevilleCache` stays keyed by the ORIGINAL `guid` parameter, which the
probe's fresh guid never touches. Add to `app/scene.rs`:

```rust
/// Per-curve R⁻¹ cache for edit-point drags (R[i][j] = basis_j(greville_i)).
#[derive(Default)]
pub struct GrevilleCache {
    map: std::collections::HashMap<String, Vec<Vec<f64>>>,   // guid → R⁻¹, dense cv_count²
}

impl GrevilleCache {
    /// R⁻¹ for `nc`, built on first use (numeric column probe, then a dense inverse).
    pub fn r_inverse(&mut self, guid: &str, nc: &NurbsCurve) -> &Vec<Vec<f64>> {
        if !self.map.contains_key(guid) {
            let n = nc.cv_count();
            let ts = nc.get_greville_abcissae();
            let base: Vec<Point> = ts.iter().map(|&t| nc.point_at(t)).collect();
            let mut r = vec![vec![0.0; n]; n];
            let mut probe = nc.duplicate();
            for j in 0..n {
                let Some((x, y, z, w)) = probe.get_cv_4d(j) else { continue };
                probe.set_cv_4d(j, x + w, y, z, w);           // +1 in euclidean x (homogeneous!)
                for i in 0..n {
                    r[i][j] = probe.point_at(ts[i])[0] - base[i][0];
                }
                probe.set_cv_4d(j, x, y, z, w);               // restore
            }
            self.map.insert(guid.to_string(), invert(r));
        }
        &self.map[guid]
    }
    pub fn invalidate(&mut self, guid: &str) { self.map.remove(guid); }
}

/// Dense Gauss–Jordan inverse with partial pivoting — cv_count is dozens, not thousands.
fn invert(mut a: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut inv: Vec<Vec<f64>> = (0..n)
        .map(|i| { let mut row = vec![0.0; n]; row[i] = 1.0; row })
        .collect();
    for col in 0..n {
        let piv = (col..n).max_by(|&r1, &r2|
            a[r1][col].abs().partial_cmp(&a[r2][col].abs()).unwrap()).unwrap();
        a.swap(col, piv);
        inv.swap(col, piv);
        let d = a[col][col];
        for k in 0..n { a[col][k] /= d; inv[col][k] /= d; }
        for row in 0..n {
            if row == col { continue; }
            let f = a[row][col];
            if f == 0.0 { continue; }
            for k in 0..n {
                a[row][k] -= f * a[col][k];
                inv[row][k] -= f * inv[col][k];
            }
        }
    }
    inv
}
```

`greville_cache: GrevilleCache` is a **new `Scene` field** — add it to `struct Scene` and
initialize `greville_cache: GrevilleCache::default()` in `Scene::new` (a struct literal, so a
missing field is **E0063**), like any other cache. Then the refit itself, in `impl Scene`:

```rust
    /// Move edit point k of curve `guid` to `target`: P' = R⁻¹ · E' per coordinate,
    /// CVs written back HOMOGENEOUS with their original weights — 78's rule, unchanged.
    pub fn refit_through(&mut self, guid: &str, k: usize, target: &Point) {
        let Some(nc) = self.session.objects.nurbscurves.iter_mut()
            .find(|c| c.guid() == guid) else { return };
        let ts = nc.get_greville_abcissae();
        let mut e: Vec<Point> = ts.iter().map(|&t| nc.point_at(t)).collect();
        if k >= e.len() { return; }
        e[k] = target.clone();
        let rinv = self.greville_cache.r_inverse(guid, nc);
        let n = nc.cv_count();
        for j in 0..n {
            let Some((_, _, _, w)) = nc.get_cv_4d(j) else { continue };
            let (mut px, mut py, mut pz) = (0.0, 0.0, 0.0);
            for i in 0..n {
                px += rinv[j][i] * e[i][0];
                py += rinv[j][i] * e[i][1];
                pz += rinv[j][i] * e[i][2];
            }
            nc.set_cv_4d(j, px * w, py * w, pz * w, w);
        }
    }
```

Notes that keep this honest: the solve is per-**coordinate** (three right-hand sides, one factored
matrix); `R` and its factorization cache per curve and invalidate with the tess cache (knots/degree
changes rebuild it — a CV *drag* doesn't, since `R` depends only on knots); and dragging an **end**
edit point of a clamped curve degenerates to dragging its end CV (`R`'s first/last rows are unit
vectors) — a nice built-in sanity check.

## Step 3 — mode switch + the same skeleton: `src/state.rs`

`F10` cycles: off → CVs → edit points → off (or F10/Shift+F10 — pick a convention and label it in
the HUD). The drag path is 78's verbatim with one substitution: per move, `refit_through` instead of
`move_cv`; the live resample + partial upload + release Command are shared, because the *effect* is
identical — CVs changed, curve re-evaluates.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Edit-point mode on 65's curve: amber points sit **on** the line. Drag one — the curve follows so
  the dragged point stays *pinned under the cursor* (compare 73: the curve lags behind a dragged CV).
  Neighboring regions flex smoothly; far spans barely move (R⁻¹'s locality).
- Drag an **endpoint** → behaves exactly like dragging the end CV (the degeneracy check).
- A weighted curve keeps its character through edit-point drags (the `w`-preserving write-back —
  same test as 78's).
- Kernel cross-check (`#[cfg(test)]`): refit a curve through a moved Greville point, then evaluate at
  that abscissa — equals the target to ~1e-12 (the memory's benchmark: the kernel refit matches
  Rhino's to 1e-14).

## Recap

```
Ch 78: CVs — honest handles, off the curve.
Ch 79: EDIT POINTS — handles ON the curve at the Greville abscissae (kernel: get_greville_abcissae →
       point_at). Drag = a linear refit: E = R·P with R[i][j] = basis_j(greville_i); solve
       P' = R⁻¹·E' (three RHS, one cached factorization; R depends on knots only, so drags reuse it;
       numeric column-building matches analytic to 1e-14). Weights preserved via 78's homogeneous
       set_cv_4d write-back. Ends degenerate to end-CV drags (unit rows) — built-in sanity.
       Same drag skeleton, live partial upload, and release Command as 73 — the effect is just
       "CVs moved".
```

Edited: `app/scene.rs` (`greville_points`, `refit_through`, greville cache), `state.rs` (mode cycle,
drag substitution).

## Next

`80-work-plane.md` — until now every tool draws on `z = 0`. The construction plane makes "the ground"
a *choice*: set it by three points or to a face, and the draw tools, grid, and snapping all follow.
