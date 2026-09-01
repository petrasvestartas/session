# 100 Edit points — drag the curve itself

> **Big picture.** *Phase 13.* CVs (85) are honest but unintuitive — they float *off* the curve and
> pull like magnets. Users expect to grab **the curve itself**. Edit points do that: points *on* the
> curve at the **Greville abscissae** (each CV's natural parameter), draggable — and the kernel
> solves the "which CVs must move so the curve passes through the dragged point" problem as a small
> linear refit. Same UI as 78, one linear solve deeper.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="control points float off the curve and pull; edit points lie on the curve at greville parameters and dragging one refits the cvs so the curve passes through it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(20,10)">
    <path d="M 10,80 C 70,20 150,100 210,40" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <g fill="none" stroke="#888"><circle cx="10" cy="80" r="4"/><circle cx="70" cy="35" r="4"/><circle cx="150" cy="85" r="4"/><circle cx="210" cy="40" r="4"/></g>
    <line x1="10" y1="80" x2="70" y2="35" stroke="#3a3a3a" stroke-dasharray="3 3"/><line x1="70" y1="35" x2="150" y2="85" stroke="#3a3a3a" stroke-dasharray="3 3"/><line x1="150" y1="85" x2="210" y2="40" stroke="#3a3a3a" stroke-dasharray="3 3"/>
    <text x="110" y="118" fill="#888" text-anchor="middle">CVs: off the curve (85)</text>
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
new curve are `P' = R⁻¹ · E'`. R is small (cv_count × cv_count), banded, and *usually* well-behaved
for clamped curves — but a degenerate knot vector (duplicate Greville abscissae) makes it singular,
so Step 2's `invert` refuses a near-zero pivot instead of NaN-ing the curve. One LU solve per drag
*release* (the live drag can reuse the factorization). Weights:
work on the **projected** 3-D points and write back through `set_cv_4d` with each CV's original `w` —
85's homogeneous rule, unchanged.

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

In edit-point mode these render as glyphs (amber, to distinguish from 85's CVs) and pick with the
same screen radius. (`greville_points` evaluates in the curve's LOCAL frame — the glyphs ride the
row's placed frame like every vertex glyph, so nothing converts here; the *drag*, which comes back
in world coordinates, converts in Step 2.) Surfaces get the tensor version — Greville in u ×
Greville in v — same idea, a grid of on-surface handles; start with curves, the surface loop is the
same code twice.

## Step 2 — the refit: `src/app/scene.rs`

The cache type first — `R` depends only on knots/degree, so one inversion serves every drag. The
matrix builds **numerically**: displace CV `j` by a unit (homogeneous, weight untouched),
re-evaluate at every Greville parameter — column `j` falls out, no basis-function internals needed,
and it matches the analytic matrix to ~1e-14. The cost is n² curve evaluations plus an O(n³) dense
inverse; if the kernel ever exposes a basis-function evaluator, build R analytically instead
(O(n²·degree), no probe) — the rest of the lesson is unchanged. The probe is `nc.duplicate()`, and
note the kernel
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
    /// None = R is singular for this curve — the refit must REFUSE, not NaN the CVs.
    pub fn r_inverse(&mut self, guid: &str, nc: &NurbsCurve) -> Option<&Vec<Vec<f64>>> {
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
            self.map.insert(guid.to_string(), invert(r)?);
        }
        self.map.get(guid)
    }
    pub fn invalidate(&mut self, guid: &str) { self.map.remove(guid); }
}

/// Dense Gauss–Jordan inverse with partial pivoting — cv_count is dozens, not thousands.
/// None on a singular matrix: the basis values R is built from are O(1), so a pivot under 1e-12
/// is genuine degeneracy (duplicate abscissae), not a small-but-legitimate value.
fn invert(mut a: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
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
        if d.abs() < 1e-12 { return None; }   // pivot ≈ 0 → singular: refuse, don't NaN
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
    Some(inv)
}
```

`greville_cache: GrevilleCache` is a **new `Scene` field** — add it to `struct Scene` and
initialize `greville_cache: GrevilleCache::default()` in `Scene::new` (a struct literal, so a
missing field is **E0063**), like any other cache. Then the refit itself, in `impl Scene`:

```rust
    /// Move edit point k of curve `guid` to `target_world`: P' = R⁻¹ · E' per coordinate,
    /// CVs written back HOMOGENEOUS with their original weights — 85's rule, unchanged. And 85's
    /// two contracts hold here too: the drag target arrives WORLD (CVs are LOCAL — convert through
    /// the row's inverse placed frame, or edit points leap by the manifest translation on placed
    /// sheets), and the write goes through `lookup.get_mut` + COW (lookup wins — a collection-side
    /// edit would be invisible to lookup readers and DISCARDED on save).
    pub fn refit_through(&mut self, guid: &str, k: usize, target_world: &Point) {
        let Some(&row) = self.guid_to_row.get(guid) else { return };
        let Some(inv) = self.placed_frame(row).inverse() else { return };
        let target = inv.transform_point(target_world);         // world → the object's frame
        let d = self.doc_of_row(row);
        let Some(Geometry::NurbsCurve(rc)) = self.docs[d].session.lookup.get_mut(guid)
            else { return };
        let ts = rc.get_greville_abcissae();
        let mut e: Vec<Point> = ts.iter().map(|&t| rc.point_at(t)).collect();
        if k >= e.len() { return; }
        e[k] = target;
        let Some(rinv) = self.greville_cache.r_inverse(guid, rc) else { return };  // singular R
        let n = rc.cv_count();
        let nc = std::rc::Rc::make_mut(rc);                     // COW — only the write pays
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
changes rebuild it — a CV *drag* doesn't, since `R` depends only on knots); and the cache needs
**eviction rules**, not just invalidation — each entry is cv_count² f64s keyed by guid, so call
`greville_cache.invalidate(guid)` everywhere 85's release bookkeeping calls `tess_cache.remove`,
and drop entries on delete/undo (64) and reconcile (46), or dead guids accumulate for the whole
session. Finally, dragging an **end** edit point of a clamped curve degenerates to dragging its
end CV (`R`'s first/last rows are unit vectors) — a nice built-in sanity check.

## Step 3 — mode switch + the same skeleton: `src/state.rs`

`F10` cycles: off → CVs → edit points → off (or F10/Shift+F10 — pick a convention and label it in
the HUD). The drag path is 85's verbatim with one substitution: per move, `refit_through` instead of
`move_cv`; the live resample + partial upload + release Command are shared, because the *effect* is
identical — CVs changed, curve re-evaluates.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Edit-point mode on 43's curve: amber points sit **on** the line. Drag one — the curve follows so
  the dragged point stays *pinned under the cursor* (compare 78: the curve lags behind a dragged CV).
  Neighboring regions flex smoothly; far spans barely move (R⁻¹'s locality). Drag a CV on a
  *placed* sheet — the point still tracks the cursor (85's world→local conversion runs here too).
- Drag an **endpoint** → behaves exactly like dragging the end CV (the degeneracy check).
- A weighted curve keeps its character through edit-point drags (the `w`-preserving write-back —
  same test as 85's).
- Kernel cross-check (`#[cfg(test)]`): refit a curve through a moved Greville point, then evaluate at
  that abscissa — equals the target to ~1e-12 (the memory's benchmark: the kernel refit matches
  Rhino's to 1e-14).

## Recap

```
Ch 78: CVs — honest handles, off the curve.
Ch 79: EDIT POINTS — handles ON the curve at the Greville abscissae (kernel: get_greville_abcissae →
       point_at). Drag = a linear refit: E = R·P with R[i][j] = basis_j(greville_i); solve
       P' = R⁻¹·E' (three RHS, one cached factorization; R depends on knots only, so drags reuse it;
       numeric column-building matches analytic to 1e-14). Weights preserved via 85's homogeneous
       set_cv_4d write-back. Ends degenerate to end-CV drags (unit rows) — built-in sanity.
       Same drag skeleton, live partial upload, and release Command as 78 — the effect is just
       "CVs moved".
```

Edited: `app/scene.rs` (`greville_points`, `refit_through`, greville cache), `state.rs` (mode cycle,
drag substitution).

## Next

`101-work-plane.md` — until now every tool draws on `z = 0`. The construction plane makes "the ground"
a *choice*: set it by three points or to a face, and the draw tools, grid, and snapping all follow.
