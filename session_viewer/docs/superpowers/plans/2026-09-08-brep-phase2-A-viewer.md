# BRep Phase 2, Part A: Smooth Faces and Edges on Facets (Viewer) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** BReps shade smooth (per-face vertices with the kernel's analytic normals, no weld) and every BRep edge is inked as pipes taken from its owning face's own tessellation grid, so the ink lies on facet boundaries today, with a facing cull fed by both adjacent faces.

**Architecture:** `walk_brep` stops welding. Each face mesh from `face_meshes_q` is uploaded as its own vertices through `to_render()` (which already carries `nx/ny/nz`), all under one object row with `FLAG_SMOOTH`. A new `brep_edges.rs` finds, for every non-degenerated BRep edge, the chain of face-mesh vertices along the edge's iso-parametric line on the first grid-meshed face that uses it (the kernel's grid mesher puts every boundary on an iso line and tags vertices with exact `u`/`v` attributes), and pushes one pipe per chain segment with `facing` packed from the owning face's vertex normals and the other adjacent face's nearest vertex normal. Edges no grid face owns fall back to today's curve-sampled ribbons. The kernel is not touched; part B replaces the chain finder with `edge_polygons_q`.

**Tech Stack:** Rust 2024, wgpu 29.0.4 (no shader or engine change in this part), `session_rust` kernel at a59c4124, Python 3 for the probe scripts.

**Spec:** `docs/superpowers/specs/2026-09-08-phase2-brep-edges-shading-design.md` sections 2.2, 2.3 and 4; `docs/superpowers/specs/2026-09-07-ink-visibility-design.md` section 6 for context. Section 2.1 (kernel) is part B and is NOT in this plan.

## Global Constraints

- Working directory for every command: `/home/petras/code/code_cpp/session_worktrees/brep-phase2/session_viewer` (a git worktree of the `session` repo on branch `brep-phase2`; the submodule `../session_rust` is checked out at a59c4124). Never run cargo in `~/code/code_cpp/wood_research/session` - another agent works there.
- Every cargo command: `export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target CARGO_BUILD_JOBS=4`. `cargo check` is wasm32 by default; native tests are `cargo xtest`; native examples take `--target x86_64-unknown-linux-gnu`. Never more than four build jobs and never two heavy cargo commands at once (a 32-job build has run this machine out of memory).
- Scratch directory: `/tmp/claude-1000/-home-petras-code-code-cpp-wood-research/c5a928d8-1da0-46e3-b421-6515e06f7d31/scratchpad`, referred to below as `$S`. The twelve benchmark scene files are already in `$S/bench/pb/` with their manifests `$S/bench/view_mixed.yaml`, `view_meshes.yaml`, `view_lines.yaml`. Never use `/tmp` directly.
- Read `../.claude/skills/session-viewer-wgpu/SKILL.md` is NOT required by this part: no `.wgsl` and no `engine/` file changes. If a task seems to need one, stop and report; the design says the shader already reads per-vertex normals.
- At most four parameters per function; grouped inputs become a named struct. No closures unless they are the fastest way (`sort_by`, `find`, `retain`, `map` on an iterator are fine). A docstring on every function. Comments say WHY.
- Every number in a comment, a docstring, a commit message or a doc is measured on this machine on the day it is written, or absent.
- Clippy must be clean: `cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings` and the wasm check `cargo check -q --target wasm32-unknown-unknown` must pass at every commit.
- Commit messages start with `viewer:`. Never add Claude or any AI as author, co-author or trailer. Do not push.
- Never edit `docs/0*.md` (lesson docs).
- The ink rule in `src/shaders/ink_visibility.wgsl` is depth-buffer-only. Do not reintroduce face tokens or any face identity.
- `Mesh::orient_faces` / `unify_winding` in the kernel are nondeterministic (HashMap-seeded BFS): do not call them. This plan removes the one call the viewer had.

---

## File map

| file | after part A |
|---|---|
| `src/app/walk/brep.rs` | `walk_brep`: per-face upload with normals, `FLAG_SMOOTH`, `FLAG_OPEN` when `!is_solid()`, edges through `brep_edges`; `walk_surface` unchanged; `QUALITY` unchanged |
| `src/app/walk/brep_edges.rs` | new: `EdgeUse`, `iso_chain`, `edge_chains`, `push_edge_pipes`, the curve-ribbon fallback; unit tests |
| `src/app/walk/mod.rs` | `pub mod brep_edges;` |
| `src/app/walk/mesh.rs` | `mesh_spacing` becomes `pub(super)` |
| `examples/mk_shade_probe.rs` | new: one BRep (sphere, torus or block-with-hole) or the dome NURBS surface, chosen by argument, alone in a file |
| `examples/mk_cylinder_hidden_probe.rs` | new: a magenta polyline behind a BRep cylinder (design section 4) |
| `examples/mk_teapot.rs` | new: the Utah teapot as one BRep of 32 bicubic patches, the hard case |
| `examples/mk_mixed_solids.rs` | two rows instead of one 10 m line (user request) |
| `docs/_shade_scanline.py` | new: the sphere scanline's maximum second difference and the back-face pixel count of a frame |
| `docs/_ink_suite.sh` | the shade probe, the cylinder hidden-line probe and the mixed scene's determinism join the suite |
| `ARCHITECTURE.md` | section 0 file list, section 6 smooth-tessellation bullet, section 11 check count |
| `docs/_PERF.md` | the twelve legs re-measured, before and after, same day |
| `examples/zz_inspect.rs` | scratch file present in the worktree today: deleted in Task 1, never committed |

## What the kernel's face meshes contain (measured 2026-09-08 with the scratch inspector, quality (5.0, 0.001))

Every face mesh carries `nx/ny/nz` on every vertex (grid faces: the analytic surface normal, poles face-averaged; CDT faces: the trimmed mesher's normals). Grid-meshed faces carry exact `u`/`v` attributes taken from the mesher's sample arrays; CDT faces carry none. The grid mesher WELDS a closed direction (its parameter array drops the domain end), so a sphere's or torus's face mesh has zero border edges and the seam is an interior grid line at the domain start; poles are one vertex each carrying `u = us[0]`. Counts:

| solid | faces | edges | face meshes (verts / triangles / border loops) |
|---|---|---|---|
| box | 6 | 12 | 6 grid faces, 4 / 2 / 1 loop each |
| cylinder | 3 | 3 | side grid 146 / 146 / 2 loops of 73; two CDT caps 72 / 70 / 1 loop |
| cone | 2 | 3 (one degenerated) | side grid 70 / 69 / 1 loop of 69; CDT base 68 / 66 |
| sphere | 1 | 3 (two degenerated) | grid 1717 / 3430 / 0 loops |
| torus | 1 | 2 | grid 6675 / 13350 / 0 loops |
| block with hole | 7 | 15 | 4 grid box sides, 1 grid hole side 146 / 146 / 2 loops, 2 CDT faces 76 / 76 / loops of 72 and 4 |
| pyramid | 5 | 12 (four degenerated) | 5 grid faces |

Every non-degenerated edge of these seven solids has a grid face as one of its uses in `edge_faces`, so the iso-chain path covers the whole mixed scene; the ribbon fallback is for BReps this scene does not contain.

---

### Task 1: Baseline numbers, the shade probe and its scanline script

**Files:**
- Delete: `examples/zz_inspect.rs`
- Create: `examples/mk_shade_probe.rs`, `docs/_shade_scanline.py`

**Interfaces:**
- Produces: `mk_shade_probe <out.pb> [sphere|torus|hole|dome]` writes one object; `_shade_scanline.py <ppm> [--max-second-diff N] [--max-backface N]` prints `second_diff <max>` and `backface <count>` and exits 1 when a floor is exceeded.

- [ ] **Step 1: Remove the scratch inspector**

```bash
rm examples/zz_inspect.rs
git status --short   # must show nothing under examples/
```

- [ ] **Step 2: Build the baseline binaries and measure the twelve perf legs BEFORE any code change**

```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target CARGO_BUILD_JOBS=4
cargo build -q --release --target x86_64-unknown-linux-gnu --examples
B=$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/examples
S=/tmp/claude-1000/-home-petras-code-code-cpp-wood-research/c5a928d8-1da0-46e3-b421-6515e06f7d31/scratchpad
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq   # must be equal, else the numbers are void
for gpu in Intel NVIDIA; do for scene in view_mixed view_meshes view_lines; do
  echo "== $scene $gpu"; env VIEWER_ADAPTER=$gpu VIEWER_W=1400 VIEWER_H=900 BENCH_FRAMES=60 "$B/bench_frame" "$S/bench/$scene.yaml"
done; done 2>&1 | tee "$S/bench/before.txt"
```

Expected: six blocks, each with a `still:` and a `moving:` line. Keep `$S/bench/before.txt`; Task 9 measures `after.txt` with the same command and both go into `docs/_PERF.md`. Confirm the adapter from the `adapter:` log line of a `selftest` run under each `VIEWER_ADAPTER` (`env VIEWER_ADAPTER=Intel "$B/selftest" "$S/a.ppm" assets/view_local.yaml 2>&1 | grep -i adapter`).

- [ ] **Step 3: Write the shade probe example**

`examples/mk_shade_probe.rs`:

```rust
// Shade probe: ONE smooth object alone in a file, so the harness's fit puts it across the whole
// frame and a scanline through it measures the shading. The argument picks the object: the
// sphere (default) for the scanline check, the torus and the block with hole for the seam and
// inner-wire loops, the dome for the NURBS surface path. Sizes and colours are the mixed
// scene's (`mk_mixed_solids`), so what is measured here is what that scene shows.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_shade_probe -- <out.pb> [sphere|torus|hole|dome]
use session_rust::brep::BRep;
use session_rust::{Color, NurbsSurface, Point, Session, Xform};

/// Half-width of the dome patch, as in the mixed scene.
const HALF: f64 = 300.0;

/// The 4x4 control grid of the mixed scene's dome: z = 300 + 200 * cos(r / 250).
fn dome_points() -> Vec<Point> {
    let coord = [-HALF, -HALF / 3.0, HALF / 3.0, HALF];
    let mut points = Vec::with_capacity(16);
    for &x in &coord {
        for &y in &coord {
            let r = (x * x + y * y).sqrt();
            points.push(Point::new(x, y, 300.0 + 200.0 * (r / 250.0).cos()));
        }
    }
    points
}

/// The BRep named by `which`, lifted so its lowest point sits on z = 0, in the mixed scene's colour.
fn solid(which: &str) -> BRep {
    let (mut b, up, color) = match which {
        "torus" => (BRep::create_torus(220.0, 70.0), 70.0, Color::new(0.91, 0.83, 0.80, 1.0)),
        "hole" => (BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0), 100.0, Color::grey()),
        _ => (BRep::create_sphere(180.0), 180.0, Color::new(0.82, 0.85, 0.91, 1.0)),
    };
    b.transform(&Xform::translation(0.0, 0.0, up));
    b.surfacecolor = color;
    b
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/shade_probe.pb".into());
    let which = std::env::args().nth(2).unwrap_or_else(|| "sphere".into());
    let mut s = Session::new("shade_probe");
    if which == "dome" {
        let mut d = NurbsSurface::create(false, false, 3, 3, 4, 4, &dome_points()).expect("dome patch");
        d.facecolors = vec![Color::grey()];
        d.name = "dome".to_string();
        s.add_nurbssurface(d, None);
    } else {
        s.add_brep(solid(&which), None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

- [ ] **Step 4: Write the scanline script**

`docs/_shade_scanline.py`:

```python
#!/usr/bin/env python3
"""_shade_scanline.py <ppm> [--max-second-diff N] [--max-backface N]

Two facts about one frame of the shade probe:

  second_diff <max>   the largest absolute second difference of luminance along the scanline
                      through the centre of the frame's non-white bounding box, taken over the
                      first run of non-white pixels on that row with 4 px trimmed at each end
                      (the antialiased silhouette). A flat-shaded sphere steps at every facet
                      boundary; a smooth one is a gradient whose 8-bit quantisation gives
                      second differences of 1 or 2.
  backface <count>    pixels within 30 of the shader's BACKFACE_COLOR (204, 13, 13): a face
                      wound inside out.

Floors are measured, never assumed: the numbers in the ink suite are the ones this script
printed on the day they were written. Exits 1 when a floor is exceeded."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

TRIM = 4
BACKFACE = (204, 13, 13)


def luminance(px, k):
    """Rec. 709 luma of pixel k of a packed RGB byte string."""
    return 0.2126 * px[k] + 0.7152 * px[k + 1] + 0.0722 * px[k + 2]


def non_white(px, k):
    """The white clear is (255, 255, 255); anything darker in any channel is the object."""
    return px[k] < 250 or px[k + 1] < 250 or px[k + 2] < 250


def bbox(w, h, px):
    """Rows and columns that hold a non-white pixel: (y0, y1, x0, x1), inclusive."""
    y0, y1, x0, x1 = h, -1, w, -1
    for y in range(h):
        for x in range(w):
            if non_white(px, 3 * (y * w + x)):
                y0, y1, x0, x1 = min(y0, y), max(y1, y), min(x0, x), max(x1, x)
    return y0, y1, x0, x1


def scanline(w, px, y):
    """Luma along the first non-white run of row y, trimmed by TRIM px at both ends."""
    xs = [x for x in range(w) if non_white(px, 3 * (y * w + x))]
    if not xs:
        return []
    run = [xs[0]]
    for x in xs[1:]:
        if x != run[-1] + 1:
            break
        run.append(x)
    run = run[TRIM:len(run) - TRIM]
    return [luminance(px, 3 * (y * w + x)) for x in run]


def second_diff(lum):
    """The largest |l[i-1] - 2 l[i] + l[i+1]| along the line, 0 for fewer than three samples."""
    best = 0.0
    for i in range(1, len(lum) - 1):
        best = max(best, abs(lum[i - 1] - 2.0 * lum[i] + lum[i + 1]))
    return best


def backface(px):
    """Pixels within 30 of BACKFACE_COLOR in every channel."""
    n = 0
    for k in range(0, len(px), 3):
        if all(abs(px[k + c] - BACKFACE[c]) <= 30 for c in range(3)):
            n += 1
    return n


def main():
    args = sys.argv[1:]
    path = args[0]
    max_sd = float(args[args.index("--max-second-diff") + 1]) if "--max-second-diff" in args else None
    max_bf = int(args[args.index("--max-backface") + 1]) if "--max-backface" in args else None
    w, h, px = read_ppm(path)
    y0, y1, _, _ = bbox(w, h, px)
    if y1 < 0:
        print("empty frame")
        return 1
    lum = scanline(w, px, (y0 + y1) // 2)
    sd = second_diff(lum)
    bf = backface(px)
    print(f"second_diff {sd:.2f} over {len(lum)} px")
    print(f"backface {bf}")
    bad = (max_sd is not None and sd > max_sd) or (max_bf is not None and bf > max_bf)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
```

- [ ] **Step 5: Build, render the BEFORE frame, record the flat numbers**

```bash
cargo build -q --release --target x86_64-unknown-linux-gnu --example mk_shade_probe --example selftest
"$B/mk_shade_probe" "$S/sphere.pb" sphere
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_NO_EDGES=1 "$B/selftest" "$S/sphere_before.ppm" "$S/sphere.pb"
python3 docs/_shade_scanline.py "$S/sphere_before.ppm"
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$S/mixed_top_before.ppm" "$S/bench/pb/view_mixed_solids.pb"
```

Expected: `second_diff` well above 2 (the facets), `backface 0`. Write both numbers down; they go into the suite's comment and the commit message of Task 8 as the "before" values. Keep `sphere_before.ppm` for the user.

- [ ] **Step 6: Clippy, then commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
git add examples/mk_shade_probe.rs docs/_shade_scanline.py
git commit -m "viewer: a shade probe and its scanline measure

One smooth object alone in a file, and a script that reports the largest
second difference of luminance across it and the count of back-face
pixels. Measured before the shading change: second_diff <N> on the
sphere (flat facets), backface 0."
```

Replace `<N>` with the measured value.

---

### Task 2: `brep_edges.rs` - the iso-chain of one edge use

**Files:**
- Create: `src/app/walk/brep_edges.rs`
- Modify: `src/app/walk/mod.rs` (add `pub mod brep_edges;` in alphabetical order after `pub mod brep;`)

**Interfaces:**
- Produces:
  - `pub struct EdgeUse { pub edge: usize, pub face: usize, pub orientation: BRepOrientation }`
  - `pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>>` - the face-mesh vertex KEYS along the edge on this face, ordered along the varying parameter, the first key repeated at the end when the edge is closed (`start_vertex == end_vertex`); `None` when the face mesh has no `u`/`v` attributes (a CDT face), the use has no pcurve, or fewer than two vertices lie on the line.
- Consumes: `BRep::pcurve_index`, `BRep::m_curves_2d`, `BRep::m_edges`, `BRep::m_surfaces`, `BRep::m_faces`, `NurbsSurface::domain`, `NurbsSurface::is_closed`, `NurbsCurve::get_cv`, `NurbsCurve::cv_count`, `Mesh.vertex[key].attributes`.

- [ ] **Step 1: Write the failing tests**

Create `src/app/walk/brep_edges.rs` with the module doc, the struct, a stub and the tests:

```rust
//! A BRep's edges as ink, taken from the tessellation itself. The kernel's grid mesher puts
//! every boundary of a grid-meshed face on an iso-parametric line and tags each vertex with
//! the exact `u`/`v` it was sampled at, so the chain of vertices along an edge IS the facet
//! boundary - no resampling, no tolerance. One chain per BRep edge, from the first face that
//! can supply one; the other adjacent face lends the facing cull its normal.

use session_rust::brep::{BRep, BRepOrientation};
use session_rust::Mesh;

/// One use of an edge by a face: which edge, which face, and the orientation of that use
/// (`BRep::edge_faces` composes it), which selects the pcurve on a seam.
pub struct EdgeUse {
    pub edge: usize,
    pub face: usize,
    pub orientation: BRepOrientation,
}

/// The face-mesh vertex keys along edge use `eu`, ordered along the parameter that varies,
/// closed (first key repeated last) when the edge starts and ends at the same vertex.
pub fn iso_chain(_b: &BRep, _fm: &Mesh, _eu: &EdgeUse) -> Option<Vec<usize>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::brep::QUALITY;

    /// The first use of every edge of `b`, with its face mesh index.
    fn first_uses(b: &BRep) -> Vec<EdgeUse> {
        let mut out = Vec::new();
        for ei in 0..b.m_edges.len() {
            let uses = b.edge_faces(ei);
            let Some(u) = uses.first() else { continue };
            out.push(EdgeUse { edge: ei, face: u.index as usize, orientation: u.orientation });
        }
        out
    }

    /// Chain `keys` of face mesh `fm` starts (and, when `closed`, ends) on BRep vertex `vi`
    /// bit for bit: the mesher sampled the corner, not a point near it.
    fn ends_on(b: &BRep, fm: &Mesh, keys: &[usize], vi: usize) -> bool {
        let p = fm.vertex[&keys[0]].position();
        let v = &b.m_vertices[vi].point;
        p[0] == v[0] && p[1] == v[1] && p[2] == v[2]
    }

    /// A cylinder's three edges: two closed circles of the same length that start on their own
    /// vertex, and an open seam of two keys (the side grid has two rows) from vertex 0 to 1.
    #[test]
    fn cylinder_chains_follow_the_grid() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let uses = first_uses(&b);
        let chains: Vec<Vec<usize>> = uses.iter().map(|u| iso_chain(&b, &fms[u.face], u).expect("grid use")).collect();
        assert_eq!(chains.len(), 3);
        for k in 0..2 {
            assert_eq!(chains[k].first(), chains[k].last());
            assert!(chains[k].len() > 4);
            assert!(ends_on(&b, &fms[uses[k].face], &chains[k], b.m_edges[k].start_vertex as usize));
        }
        assert_eq!(chains[0].len(), chains[1].len());
        assert_eq!(chains[2].len(), 2);
        assert!(ends_on(&b, &fms[uses[2].face], &chains[2], 0));
        let top = fms[uses[2].face].vertex[chains[2].last().unwrap()].position();
        assert_eq!([top[0], top[1], top[2]], [b.m_vertices[1].point[0], b.m_vertices[1].point[1], b.m_vertices[1].point[2]]);
    }

    /// A sphere's seam runs pole to pole through the welded grid: one open chain whose ends
    /// are the two BRep vertices; its two degenerated edges yield nothing.
    #[test]
    fn sphere_seam_reaches_both_poles() {
        let b = BRep::create_sphere(180.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let uses = first_uses(&b);
        let seam = iso_chain(&b, &fms[0], &uses[0]).expect("seam");
        assert!(seam.len() > 10);
        assert_ne!(seam.first(), seam.last());
        assert!(ends_on(&b, &fms[0], &seam, 0));
        let north = fms[0].vertex[seam.last().unwrap()].position();
        assert_eq!(north[2], b.m_vertices[1].point[2]);
        assert!(iso_chain(&b, &fms[0], &uses[1]).is_none());
        assert!(iso_chain(&b, &fms[0], &uses[2]).is_none());
    }

    /// A torus has two closed seams on one welded face, one along each parameter, both
    /// through the single BRep vertex; a CDT face (the cylinder's cap) has no iso chain.
    #[test]
    fn torus_seams_close_and_cdt_faces_decline() {
        let b = BRep::create_torus(220.0, 70.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        for u in first_uses(&b) {
            let c = iso_chain(&b, &fms[u.face], &u).expect("seam");
            assert_eq!(c.first(), c.last());
            assert!(ends_on(&b, &fms[u.face], &c, 0));
        }
        let cyl = BRep::create_cylinder(150.0, 400.0);
        let cfm = cyl.face_meshes_q(Some(QUALITY));
        let cap = cyl.edge_faces(0)[1].clone();
        let eu = EdgeUse { edge: 0, face: cap.index as usize, orientation: cap.orientation };
        assert!(iso_chain(&cyl, &cfm[eu.face], &eu).is_none());
    }
}
```

Make `QUALITY` public in `src/app/walk/brep.rs`: change `const QUALITY` to `pub const QUALITY`.

Add to `src/app/walk/mod.rs` after `pub mod brep;`:

```rust
pub mod brep_edges;
```

- [ ] **Step 2: Run the tests to verify they fail**

```bash
cargo xtest -q brep_edges 2>&1 | tail -20
```

Expected: three failures on `expect("grid use")` / `expect("seam")` (the stub returns `None`).

- [ ] **Step 3: Implement `iso_chain`**

Replace the stub with:

```rust
/// A pcurve's two ends in UV, from its first and last control point. The primitives' pcurves
/// are straight iso lines, which is also the condition under which the kernel grid-meshes a
/// face; a curved pcurve never reaches here because its face has no `u`/`v` attributes.
fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);
    if ci < 0 {
        return None;
    }
    let c = &b.m_curves_2d[ci as usize];
    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

/// The distinct values of attribute `name` over the face mesh, sorted: the mesher's own
/// sample array, recovered exactly (every vertex carries one of its entries).
fn sample_values(fm: &Mesh, name: &str) -> Vec<f64> {
    let mut vals: Vec<f64> = fm.vertex.values().filter_map(|vd| vd.attributes.get(name).copied()).collect();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals.dedup();
    vals
}

/// Which sample value the pcurve's constant parameter `target` names. A closed direction has
/// no sample at its domain end (the mesher welds the wrap), so a pcurve sitting at the end -
/// the reversed use of a seam - is measured against the start as well; the nearest wins,
/// which needs no tolerance.
fn nearest_sample(vals: &[f64], target: f64, wrap: Option<(f64, f64)>) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;
    for &v in vals {
        let mut d = (v - target).abs();
        if let Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }
        if best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, v));
        }
    }
    best.map(|(_, v)| v)
}

/// The face-mesh vertex keys along edge use `eu`, ordered along the parameter that varies,
/// closed (first key repeated last) when the edge starts and ends at the same vertex.
pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let (p0, p1) = pcurve_ends(b, eu)?;
    // The constant parameter is the one that moves least between the pcurve's ends: u for a
    // meridian, v for a circle of latitude.
    let fixed = if (p1[0] - p0[0]).abs() <= (p1[1] - p0[1]).abs() { 0 } else { 1 };
    let (fixed_name, free_name) = if fixed == 0 { ("u", "v") } else { ("v", "u") };
    let vals = sample_values(fm, fixed_name);
    if vals.is_empty() {
        return None;
    }
    let srf = &b.m_surfaces[b.m_faces[eu.face].surface_index as usize];
    let wrap = if srf.is_closed(fixed) { srf.domain(fixed) } else { None };
    let at = nearest_sample(&vals, p0[fixed], wrap)?;

    let mut on_line: Vec<(f64, usize)> = Vec::new();
    for (&key, vd) in fm.vertex.iter() {
        let (Some(&f), Some(&t)) = (vd.attributes.get(fixed_name), vd.attributes.get(free_name)) else { continue };
        if f == at {
            on_line.push((t, key));
        }
    }
    if on_line.len() < 2 {
        return None;
    }
    // By parameter, then by key: the map's order must never reach the chain.
    on_line.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
    let mut keys: Vec<usize> = on_line.into_iter().map(|(_, k)| k).collect();
    let e = &b.m_edges[eu.edge];
    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }
    Some(keys)
}
```

Check that `Option::is_none_or` compiles on this toolchain (`rustc --version`; it is stable since 1.82). If not, write `best.map_or(true, |(bd, _)| d < bd)`.

- [ ] **Step 4: Run the tests to verify they pass**

```bash
cargo xtest -q brep_edges 2>&1 | tail -20
```

Expected: `test result: ok. 3 passed`. If `sphere_seam_reaches_both_poles` fails on the north pole's z, print the chain's last key's attributes: the north pole must carry `u = us[0]`; if the seam's pcurve sits at the domain END and the wrap rule did not fire, `srf.is_closed(0)` returned false for the sphere - report that rather than adding a tolerance.

- [ ] **Step 5: Clippy, wasm check, commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
cargo check -q --target wasm32-unknown-unknown
git add src/app/walk/brep_edges.rs src/app/walk/mod.rs src/app/walk/brep.rs
git commit -m "viewer: the chain of grid vertices along one BRep edge use

The kernel's grid mesher samples every boundary on an iso line and tags
each vertex with the exact parameter, so the vertices along an edge are
recovered by equality, ordered by the free parameter, and closed when the
edge is. A CDT face has no such tags and declines."
```

---

### Task 3: `brep_edges.rs` - chains for every edge, pipes with two-face facing

**Files:**
- Modify: `src/app/walk/brep_edges.rs`

**Interfaces:**
- Produces:
  - `pub struct EdgeChain { pub face: usize, pub keys: Vec<usize>, pub other: Option<usize> }` - the owning face mesh index, the keys along it, and the index of the other adjacent face mesh (None for a seam or a free edge).
  - `pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>>` - one entry per `b.m_edges` index; None for a degenerated edge or one with no grid use.
  - `pub struct EdgePen<'a> { pub fms: &'a [Mesh], pub pen: Pen }` - the face meshes and the pen (row, radius, colour) the pipes are written with.
  - `pub fn push_edge_pipes(seg: &mut SegRows, chain: &EdgeChain, ep: &EdgePen, bounds: &mut Aabb) -> usize` - one pipe per chain segment into `seg.pipes`, returns the count.
- Consumes: `iso_chain` (Task 2); `pack_facing`, `Pen`, `BLACK` from `encode.rs`; `CylinderSegment`, `SegRows`; `Aabb`.

- [ ] **Step 1: Write the failing tests**

Append inside `mod tests`:

```rust
    /// Every non-degenerated edge of the mixed scene's seven solids gets a chain, and every
    /// chain's other face is the second use's face, or None on a seam (both uses on one face).
    #[test]
    fn every_solid_edge_has_a_chain() {
        let solids = [
            BRep::create_box(400.0, 300.0, 250.0),
            BRep::create_cylinder(150.0, 400.0),
            BRep::create_cone(150.0, 400.0),
            BRep::create_sphere(180.0),
            BRep::create_torus(220.0, 70.0),
            BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0),
            BRep::create_pyramid(400.0, 350.0),
        ];
        for b in &solids {
            let fms = b.face_meshes_q(Some(QUALITY));
            let chains = edge_chains(b, &fms);
            assert_eq!(chains.len(), b.m_edges.len());
            for (ei, e) in b.m_edges.iter().enumerate() {
                let uses = b.edge_faces(ei);
                match &chains[ei] {
                    None => assert!(e.degenerated, "{} edge {ei}", b.name),
                    Some(c) => {
                        assert_eq!(c.face, uses[0].index as usize);
                        let second = uses.get(1).map(|u| u.index as usize);
                        assert_eq!(c.other, second.filter(|f| *f != c.face));
                    }
                }
            }
        }
    }

    /// A circle edge of the cylinder packs the side's normal and the cap's normal - two
    /// different codes - into every pipe; the seam packs the side's normal twice.
    #[test]
    fn pipes_face_both_adjacent_faces() {
        use crate::app::walk::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};
        use crate::engine::gpu::segments::SegRows;
        use crate::math::Aabb;
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let ep = EdgePen { fms: &fms, pen: Pen { row: 3, radius: encode_width(1.0), color: pack_rgba([0.0, 0.0, 0.0, 1.0]) } };
        let mut seg = SegRows::default();
        let mut bounds = Aabb::empty();
        let circle = push_edge_pipes(&mut seg, chains[0].as_ref().unwrap(), &ep, &mut bounds);
        assert_eq!(circle, chains[0].as_ref().unwrap().keys.len() - 1);
        for p in &seg.pipes {
            assert_ne!(p.facing, FACING_UNKNOWN);
            assert_ne!(p.facing & 0xffff, p.facing >> 16);
            assert_eq!(p.instance_id, 3);
        }
        let before = seg.pipes.len();
        let seam = push_edge_pipes(&mut seg, chains[2].as_ref().unwrap(), &ep, &mut bounds);
        assert_eq!(seam, 1);
        let p = &seg.pipes[before];
        assert_eq!(p.facing & 0xffff, p.facing >> 16);
        assert!(seg.ribbons.is_empty());
    }
```

- [ ] **Step 2: Run to verify they fail to compile**

```bash
cargo xtest -q brep_edges 2>&1 | grep -E "^error" | head
```

Expected: `cannot find function edge_chains`, `cannot find struct EdgePen` and friends.

- [ ] **Step 3: Implement**

Add the imports at the top of `brep_edges.rs`:

```rust
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use crate::math::Aabb;
use super::encode::{pack_facing, Pen};
```

Add after `iso_chain`:

```rust
/// One edge's ink source: the face mesh it is read from, the keys along it, and the other
/// face that meets it (None on a seam, where both uses are the same face, or on a free edge).
pub struct EdgeChain {
    pub face: usize,
    pub keys: Vec<usize>,
    pub other: Option<usize>,
}

/// A chain for every edge of `b`, from the first use that is a grid face; a degenerated edge,
/// or one whose every use is a CDT face, gets None and falls back to the curve.
pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>> {
    let mut out = Vec::with_capacity(b.m_edges.len());
    for (ei, e) in b.m_edges.iter().enumerate() {
        if e.degenerated {
            out.push(None);
            continue;
        }
        let uses = b.edge_faces(ei);
        let mut found: Option<EdgeChain> = None;
        for (k, u) in uses.iter().enumerate() {
            let eu = EdgeUse { edge: ei, face: u.index as usize, orientation: u.orientation };
            let Some(keys) = iso_chain(b, &fms[eu.face], &eu) else { continue };
            // The other face is any use on a different face - a seam's second use is the
            // same face and lends nothing new.
            let other = uses.iter().enumerate().find(|(j, o)| *j != k && o.index as usize != eu.face).map(|(_, o)| o.index as usize);
            found = Some(EdgeChain { face: eu.face, keys, other });
            break;
        }
        out.push(found);
    }
    out
}

/// The unit sum of two vertex normals: the surface direction along one chain segment.
fn mean_normal(a: Option<[f64; 3]>, b: Option<[f64; 3]>) -> Option<[f64; 3]> {
    let (a, b) = (a?, b?);
    let s = [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
    let l = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
    if l > 0.0 { Some([s[0] / l, s[1] / l, s[2] / l]) } else { None }
}

/// The normal of the face-mesh vertex nearest to `p`: the other face's surface direction at
/// the edge, without that face having sampled the edge the same way. A minimum, not a
/// threshold, so no tolerance enters.
fn nearest_normal(fm: &Mesh, p: [f64; 3]) -> Option<[f64; 3]> {
    let mut best: Option<(f64, [f64; 3])> = None;
    for vd in fm.vertex.values() {
        let d = (vd.x - p[0]).powi(2) + (vd.y - p[1]).powi(2) + (vd.z - p[2]).powi(2);
        if best.is_none_or(|(bd, _)| d < bd) && let Some(n) = vd.normal() {
            best = Some((d, n));
        }
    }
    best.map(|(_, n)| n)
}

/// What the pipe loop reads: every face mesh (for the other face's normals) and the pen.
pub struct EdgePen<'a> {
    pub fms: &'a [Mesh],
    pub pen: Pen,
}

/// One pipe per chain segment. `facing` carries the owning face's normal along the segment
/// and the other face's normal nearest its midpoint, so the vertex-stage cull drops the edge
/// only when BOTH faces turn away - a cap's rim stays inked from above while the side below
/// it faces away.
pub fn push_edge_pipes(seg: &mut SegRows, chain: &EdgeChain, ep: &EdgePen, bounds: &mut Aabb) -> usize {
    let fm = &ep.fms[chain.face];
    let other = chain.other.map(|f| &ep.fms[f]);
    seg.pipes.reserve(chain.keys.len().saturating_sub(1));
    let mut count = 0;
    for w in chain.keys.windows(2) {
        let (a, b) = (&fm.vertex[&w[0]], &fm.vertex[&w[1]]);
        let p0 = [a.x, a.y, a.z];
        let p1 = [b.x, b.y, b.z];
        let n0 = mean_normal(a.normal(), b.normal());
        let mid = [(p0[0] + p1[0]) * 0.5, (p0[1] + p1[1]) * 0.5, (p0[2] + p1[2]) * 0.5];
        let n1 = match other {
            Some(o) => nearest_normal(o, mid),
            None => n0,
        };
        let p0f = p0.map(|v| v as f32);
        let p1f = p1.map(|v| v as f32);
        bounds.grow(p0f);
        bounds.grow(p1f);
        seg.pipes.push(CylinderSegment {
            p0: p0f,
            radius: ep.pen.radius,
            p1: p1f,
            instance_id: ep.pen.row,
            color: ep.pen.color,
            facing: pack_facing(n0.as_ref(), n1.as_ref()),
        });
        count += 1;
    }
    count
}
```

`VertexData` exposes `x`, `y`, `z` as public fields (`session_rust/src/mesh.rs`, used as `point.x` in `walk_mesh`); `vd.normal()` returns `Option<[f64; 3]>`. `let ... && let` chains are already used in this crate (`mesh_ink.rs`).

- [ ] **Step 4: Run the tests**

```bash
cargo xtest -q brep_edges 2>&1 | tail -20
```

Expected: `5 passed`.

- [ ] **Step 5: Clippy, wasm check, commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
cargo check -q --target wasm32-unknown-unknown
git add src/app/walk/brep_edges.rs
git commit -m "viewer: one chain per BRep edge, pipes facing both adjacent faces

The first grid face that uses an edge supplies the chain; the other
adjacent face lends the normal nearest each segment's midpoint, so the
cull keeps a cap's rim from above while the side under it faces away."
```

---

### Task 4: `walk_brep` - faces with their own vertices and normals, edges as pipes

**Files:**
- Modify: `src/app/walk/brep.rs` (rewrite `walk_brep` and `walk_brep_edges`; keep `walk_surface` and `QUALITY`)
- Modify: `src/app/walk/mesh.rs:37` (`fn mesh_spacing` -> `pub(super) fn mesh_spacing`)

**Interfaces:**
- Consumes: `edge_chains`, `push_edge_pipes`, `EdgePen` (Task 3); `Mesh::to_render`, `RenderMesh`; `mesh_thickness` (`bounds.rs`); `mesh_spacing`; `Instance::FLAG_SMOOTH`, `FLAG_OPEN`; `knobs::no_edges`; `sample_nurbscurve`, `push_polyline` (`curves.rs`) for the fallback.
- Produces: `pub fn walk_brep(arena, ink, b, cx) -> Row` with the same signature as today.

- [ ] **Step 1: Write the failing test**

Append to `src/app/walk/brep.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::glyphs::GlyphRows;
    use crate::engine::gpu::segments::SegRows;
    use crate::engine::gpu::Instance;
    use crate::app::walk::brep_edges::edge_chains;

    /// The walked tables of one BRep at row 5: arena rows, pipes, ribbons, markers, flags.
    fn walked(b: &BRep) -> (ArenaRows, SegRows, GlyphRows, Row) {
        let mut arena = ArenaRows::default();
        let mut seg = SegRows::default();
        let mut glyph = GlyphRows::default();
        let cx = WalkCx { vert_base: 100, cloud_px: 0.0, row: 5 };
        let row = {
            let mut ink = Ink { seg: &mut seg, glyph: &mut glyph };
            walk_brep(&mut arena, &mut ink, b, &cx)
        };
        (arena, seg, glyph, row)
    }

    /// A cylinder uploads every face mesh's own vertices (no weld: the side's 146 plus the two
    /// caps' 72 each, read from the kernel, not assumed), each with a unit normal, its
    /// indices based on the file's vertex base, one pipe per chain segment and no ribbons,
    /// no markers, FLAG_SMOOTH and not FLAG_OPEN.
    #[test]
    fn cylinder_walks_unwelded_with_normals() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let verts: usize = fms.iter().map(|m| m.vertex.len()).sum();
        let tris: usize = fms.iter().map(|m| m.to_render().indices.len()).sum();
        let segments: usize = edge_chains(&b, &fms).iter().flatten().map(|c| c.keys.len() - 1).sum();
        let (arena, seg, glyph, row) = walked(&b);
        assert_eq!(arena.verts.len(), verts);
        assert_eq!(arena.vids.len(), verts);
        assert_eq!(arena.idx.len(), tris);
        assert!(arena.idx.iter().all(|&i| i >= 100 && i < 100 + verts as u32));
        for v in &arena.verts {
            let n = v.normal;
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((l - 1.0).abs() < 1e-3, "normal {n:?}");
        }
        assert_eq!(seg.pipes.len(), segments);
        assert!(seg.ribbons.is_empty());
        assert!(glyph.spheres.is_empty());
        assert_ne!(row.flags & Instance::FLAG_SMOOTH, 0);
        assert_eq!(row.flags & Instance::FLAG_OPEN, 0);
        assert!(row.faces);
        assert!(row.bounds.diagonal() > 400.0);
    }

    /// A single face pulled out of a solid is not solid: it is walked FLAG_OPEN so the facing
    /// cull, whose premise is a closed surface, is skipped.
    #[test]
    fn open_brep_is_flagged_open() {
        let mut b = BRep::create_box(400.0, 300.0, 250.0);
        b.m_solids.clear();
        let (_, _, _, row) = walked(&b);
        assert_ne!(row.flags & Instance::FLAG_OPEN, 0);
    }
}
```

Check `b.m_solids` is a public field (`grep -n "pub m_solids" ../session_rust/src/brep.rs`); if it is not, build the open case with `BRep::new()` plus one face from the box's surfaces via the public `add_*` methods, or assert on `is_solid()` of a BRep with an empty solid list obtained another public way - do not add a kernel API.

- [ ] **Step 2: Run to verify it fails**

```bash
cargo xtest -q walk::brep 2>&1 | tail -20
```

Expected: `cylinder_walks_unwelded_with_normals` fails (today's weld gives one welded vertex count and ribbons, not pipes).

- [ ] **Step 3: Rewrite `walk_brep`**

Replace the whole file `src/app/walk/brep.rs` above the `#[cfg(test)]` block with:

```rust
//! A BRep or a NURBS surface into the tables. A BRep's faces are uploaded one by one with the
//! kernel's own vertices and analytic normals - no weld across faces, which is what let the
//! shader fall back to flat derivative normals - and its edges are pipes read off the face
//! tessellations (`brep_edges`). No sheet lanes; `FLAG_OPEN` only when the BRep is not a
//! solid, from its own topology rather than from a welded mesh.

use session_rust::{BRep, Color, Mesh, NurbsSurface, RenderMesh};
use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::Instance;
use super::{Row, WalkCx};
use super::bounds::mesh_thickness;
use super::brep_edges::{edge_chains, push_edge_pipes, EdgePen};
use super::mesh::{mesh_spacing, walk_mesh, MeshCx, MeshOpts};
use super::mesh_ink::Ink;
use super::curves::{push_polyline, sample_nurbscurve};
use super::encode::{encode_width, pack_rgba, Pen};
use crate::math::Aabb;

/// How finely the VIEWER wants a surface tessellated: the normal may turn 5 degrees between
/// samples and a chord may sag a thousandth of the object. The kernel's own default is 20
/// degrees and 0.005, which turns a cylinder into an 18-sided prism with a visibly polygonal
/// silhouette - tessellation quality is a display decision, so the display makes it.
/// Measured on 25 extruded circles: 1650 -> 4050 faces, 1.6 -> 1.8 ms a frame.
pub const QUALITY: (f64, f64) = (5.0, 0.001);

/// The positions and the file-local triangle indices of every face uploaded so far: what the
/// thickness measure reads once all faces are in.
struct Solid {
    pos: Vec<[f32; 3]>,
    tris: Vec<u32>,
    bounds: Aabb,
}

/// One face mesh into the arena under `cx.row`: its own vertices, its normals as the kernel
/// evaluated them, its triangles based on the file's vertex base.
fn push_face(arena: &mut ArenaRows, rm: &RenderMesh, cx: &WalkCx, solid: &mut Solid) {
    let base = cx.vert_base + arena.verts.len() as u32;
    let local = solid.pos.len() as u32;
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices {
        solid.bounds.grow(v.position);
        solid.pos.push(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }
    arena.idx.reserve(rm.indices.len());
    for &i in &rm.indices {
        arena.idx.push(base + i);
        solid.tris.push(local + i);
    }
}

/// Tessellate a BRep face by face, upload each with its surface colour and normals, then ink
/// its edges. The row is one object; `FLAG_SMOOTH` tells the marker lane the vertices are
/// samples; `FLAG_OPEN` is the BRep's own `is_solid`, since an open shell shows its inside.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut fms = b.face_meshes_q(Some(QUALITY));
    let mut solid = Solid { pos: Vec::new(), tris: Vec::new(), bounds: Aabb::empty() };
    let mut verts = 0;
    for fm in fms.iter_mut() {
        fm.set_objectcolor(b.surfacecolor.clone());
        verts += fm.vertex.len();
        push_face(arena, &fm.to_render(), cx, &mut solid);
    }
    let mut flags = Instance::FLAG_SMOOTH;
    if !b.is_solid() {
        flags |= Instance::FLAG_OPEN;
    }
    let thickness = mesh_thickness(&solid.pos, &solid.tris);
    let mut row = Row { bounds: solid.bounds, spacing: mesh_spacing(&solid.bounds, verts), flags, faces: true, thickness };
    if !knobs::no_edges() {
        walk_brep_edges(ink, b, &fms, &mut row.bounds, cx.row);
    }
    row
}

/// The solid's own edges, one chain per BRep edge off the tessellation (pipes, culled by the
/// two adjacent faces); an edge no grid face owns is sampled off its 3D curve as a ribbon,
/// today's path, until the kernel supplies every edge's polygon.
fn walk_brep_edges(ink: &mut Ink, b: &BRep, fms: &[Mesh], bounds: &mut Aabb, row: u32) {
    let pen = Pen { row, radius: encode_width(b.width), color: pack_rgba(Color::black().to_f32()) };
    let ep = EdgePen { fms, pen };
    for (ei, chain) in edge_chains(b, fms).iter().enumerate() {
        match chain {
            Some(c) => {
                push_edge_pipes(ink.seg, c, &ep, bounds);
            }
            None => push_curve_ribbon(ink, b, ei, (&ep.pen, bounds)),
        }
    }
}

/// The fallback for an edge with no grid face: the 3D curve sampled by turning angle, drawn as
/// a ribbon with no facing (nothing exact is known about its neighbours).
fn push_curve_ribbon(ink: &mut Ink, b: &BRep, ei: usize, out: (&Pen, &mut Aabb)) {
    let edge = &b.m_edges[ei];
    if edge.degenerated || edge.curve_3d_index < 0 {
        return;
    }
    let points: Vec<[f32; 3]> = sample_nurbscurve(&b.m_curves_3d[edge.curve_3d_index as usize])
        .into_iter()
        .map(|p| p.map(|v| v as f32))
        .collect();
    if points.len() < 2 {
        return;
    }
    push_polyline(ink.seg, &points, out.0, out.1);
}

/// Tessellate a surface with its first face colour and walk it. A planar surface has no
/// curvature to follow, so the kernel's own corner quad is already exact.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = if s.is_planar(1e-6) {
        s.mesh()
    } else {
        RemeshNurbsSurfaceGrid::from_u_v_q(s.clone(), 0, 0, QUALITY.0, QUALITY.1)
    };
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}
```

The `walk_brep_edges` signature has five inputs; it is written with four parameters plus `cx.row` folded in by passing `(ink, b, fms, bounds, row)` - that is five. Fix it to four: make `walk_brep_edges(ink: &mut Ink, b: &BRep, ep: &EdgePen, bounds: &mut Aabb)` and build `ep` in `walk_brep`:

```rust
    if !knobs::no_edges() {
        let pen = Pen { row: cx.row, radius: encode_width(b.width), color: pack_rgba(Color::black().to_f32()) };
        walk_brep_edges(ink, b, &EdgePen { fms: &fms, pen }, &mut row.bounds);
    }
```

and

```rust
/// The solid's own edges, one chain per BRep edge off the tessellation (pipes, culled by the
/// two adjacent faces); an edge no grid face owns is sampled off its 3D curve as a ribbon,
/// today's path, until the kernel supplies every edge's polygon.
fn walk_brep_edges(ink: &mut Ink, b: &BRep, ep: &EdgePen, bounds: &mut Aabb) {
    for (ei, chain) in edge_chains(b, ep.fms).iter().enumerate() {
        match chain {
            Some(c) => {
                push_edge_pipes(ink.seg, c, ep, bounds);
            }
            None => push_curve_ribbon(ink, b, ei, (&ep.pen, bounds)),
        }
    }
}
```

`RenderMesh` must be re-exported by `session_rust` (`grep -n "pub use render_mesh" ../session_rust/src/lib.rs`); if only `RenderVertex` is, import `session_rust::render_mesh::RenderMesh`.

In `src/app/walk/mesh.rs` change:

```rust
fn mesh_spacing(bounds: &Aabb, verts: usize) -> f32 {
```

to

```rust
pub(super) fn mesh_spacing(bounds: &Aabb, verts: usize) -> f32 {
```

- [ ] **Step 4: Run the tests**

```bash
cargo xtest -q 2>&1 | tail -20
```

Expected: all pass, including `walk::brep` (2), `brep_edges` (5) and the existing `mesh_ink` tests.

- [ ] **Step 5: Clippy, wasm check, render the mixed scene, commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
cargo check -q --target wasm32-unknown-unknown
cargo build -q --release --target x86_64-unknown-linux-gnu --example selftest --example mk_shade_probe
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_NO_EDGES=1 "$B/selftest" "$S/sphere_after.ppm" "$S/sphere.pb"
python3 docs/_shade_scanline.py "$S/sphere_after.ppm"
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 "$B/selftest" "$S/mixed_iso.ppm" "$S/bench/pb/view_mixed_solids.pb"
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$S/mixed_top.ppm" "$S/bench/pb/view_mixed_solids.pb"
python3 docs/_shade_scanline.py "$S/mixed_top.ppm"
```

Expected: `second_diff` on the sphere at most 2 (8-bit quantisation of a gradient), `backface 0` on both frames. Look at `mixed_iso.ppm` (convert with `python3 -c` or view the PPM) and confirm: smooth spheres and tori, every solid's edges present, no doubled seam. If the sphere's shading still steps, check that `arena.verts` normals are non-zero in the test above; if `backface` is non-zero, the kernel's flip of a Reversed face and the `to_render` winding disagree - report which solid, do not call `orient_faces`.

```bash
git add src/app/walk/brep.rs src/app/walk/mesh.rs
git commit -m "viewer: BRep faces keep their own vertices and normals, edges are pipes

No weld: each face mesh is uploaded as the kernel evaluated it, so the
headlight shades the analytic normal and a sphere reads as a gradient
(scanline second difference <before> -> <after>). Edges come off the
owning face's grid as pipes with both faces' normals, and FLAG_OPEN is
the BRep's own is_solid rather than a welded mesh's border count."
```

Fill `<before>` and `<after>` with the measured second differences.

---

### Task 5: The mixed-scene and probe renders, the pick, determinism

**Files:** none modified; this task produces evidence and reports.

- [ ] **Step 1: Close-ups of the three smooth objects and the hole block**

```bash
for w in sphere torus hole dome; do
  "$B/mk_shade_probe" "$S/$w.pb" $w
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 "$B/selftest" "$S/${w}_iso.ppm" "$S/$w.pb"
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$S/${w}_top.ppm" "$S/$w.pb"
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_ORBIT=0,60 "$B/selftest" "$S/${w}_front.ppm" "$S/$w.pb"
  python3 docs/_shade_scanline.py "$S/${w}_top.ppm"
done
```

Expected on every frame: `backface 0`. Inspect each PPM: the sphere shows one meridian seam, not two and not broken; the torus two closed seam rings; the hole block its box edges and both circles; the dome its four border curves and nothing across its interior. Convert the PPMs to PNG for the user (`python3 -c "import sys; from PIL import Image; Image.open(sys.argv[1]).save(sys.argv[2])" in.ppm out.png` if PIL is present; otherwise leave PPM and say so).

- [ ] **Step 2: Pick a sphere**

```bash
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_PICK=700,450 "$B/selftest" "$S/sphere_pick.ppm" "$S/sphere.pb" | grep pick
```

Expected: `pick: (700,450) doc='...' guid=... row=0`. Then on the mixed scene: find a sphere pixel (its lit tint is near (209, 217, 232); pick the centroid of pixels within 12 of it):

```bash
python3 - "$S/mixed_iso.ppm" <<'EOF'
import sys; sys.path.insert(0, "docs")
from _count_colors import read_ppm
w, h, px = read_ppm(sys.argv[1])
xs = []; ys = []
for y in range(h):
    for x in range(w):
        k = 3 * (y * w + x)
        if abs(px[k] - 209) <= 12 and abs(px[k+1] - 217) <= 12 and abs(px[k+2] - 232) <= 12:
            xs.append(x); ys.append(y)
print(len(xs), sum(xs) // max(1, len(xs)), sum(ys) // max(1, len(ys)))
EOF
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_PICK=<x>,<y> "$B/selftest" "$S/mixed_pick.ppm" "$S/bench/pb/view_mixed_solids.pb" | grep pick
```

Expected: a hit on row 3 (the sphere is the fourth object added). Record the exact line.

- [ ] **Step 3: Determinism of the new walk**

```bash
cargo build -q --release --target x86_64-unknown-linux-gnu --example check_determinism
"$B/check_determinism" "$S/bench/pb/view_mixed_solids.pb" "$S/torus.pb" "$S/hole.pb"
```

Expected: green (two loads produce identical tables). A difference means a HashMap order reached the rows: the only maps touched are `fm.vertex` in `sample_values` (sorted and deduped), `iso_chain` (sorted by parameter then key) and `nearest_normal` (a minimum; a tie between two vertices at the same distance with different normals would be order-dependent - if that happens, break the tie by the smaller key).

- [ ] **Step 4: Report**

No commit. Hand back: the scanline numbers before and after, `backface` counts, the pick lines, the determinism result, and the paths of the PNG/PPM renders.

---

### Task 6: The suite grows the shade probe and the mixed scene

**Files:**
- Modify: `docs/_ink_suite.sh`

- [ ] **Step 1: Add the checks**

After the `check determinism ...` line, add:

```bash
check mixed_scene "$B/mk_mixed_solids" "$OUT/mixed.pb"
# Determinism of the unwelded walk on a torus and a block with hole. NOT the sphere and NOT the
# mixed scene: the kernel's grid mesher averages a pole's normal over its fan in HashMap order,
# so a sphere's two pole normals differ in their last bits between loads (measured 2026-09-08,
# five loads of sphere.pb, `verts` flaky every time; torus and hole deterministic in three
# loads each). The weld used to drop those normals; the analytic normals now reach the GPU.
# The fix is a sorted accumulation in the kernel (phase 2 part B), after which the sphere and
# the mixed scene join this line.
check probe_torus "$B/mk_shade_probe" "$OUT/torus.pb" torus
check probe_hole "$B/mk_shade_probe" "$OUT/hole.pb" hole
check probe_determinism "$B/check_determinism" "$OUT/torus.pb" "$OUT/hole.pb"

# Smooth shading: a sphere alone, headlight on, no ink. The largest second difference of luma
# across its centre scanline measured <after> with per-vertex normals against <before> with the
# welded flat facets (1400x900, MSAA 4, Intel iGPU, 2026-09-08); the floor is 2.5, the
# quantisation stair of an 8-bit gradient with room for a driver's rounding. Zero back-face
# pixels from above: a face wound inside out shows the shader's red.
check shade_probe "$B/mk_shade_probe" "$OUT/sphere.pb" sphere
check render_shade env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_NO_EDGES=1 "$B/selftest" "$OUT/sphere.ppm" "$OUT/sphere.pb"
check shade_scanline python3 docs/_shade_scanline.py "$OUT/sphere.ppm" --max-second-diff 2.5 --max-backface 0
check render_mixed_top env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$OUT/mixed_top.ppm" "$OUT/mixed.pb"
check mixed_backface python3 docs/_shade_scanline.py "$OUT/mixed_top.ppm" --max-backface 0
```

Replace `<before>` and `<after>` with the numbers measured in Tasks 1 and 4. If the measured `after` is above 2.5, the floor is the measured value rounded up to the next 0.5 and the comment says so; never set a floor that was not measured.

- [ ] **Step 2: Run the whole suite**

```bash
docs/_ink_suite.sh 2>&1 | tee "$S/ink_suite.log"
```

Expected: every line `PASS`, ending `ink suite OK`. The BRep orbit checks (`orbit_ok`, `orbit_flipped`) are the ones this change can move: they compare each frame with its neighbours (12% of the series mean) and the ok/flipped masks pixel for pixel. If `orbit_flipped` fails on the mask diff, the flipped cylinder's edge pipes carry different `facing` than the ok one (their faces are reversed, so the kernel flipped their normals) and are culled differently: the ink then DOES depend on orientation. Report it with the frame numbers and the pixel count; a fix is to be discussed, not improvised. If `floor_census` fails, the floor model has no BReps and the failure is unrelated to this change - report it.

- [ ] **Step 3: Commit**

```bash
git add docs/_ink_suite.sh
git commit -m "viewer: the ink suite measures the sphere's shading and the mixed scene

The scanline floor and the back-face count join the run, and the
mixed-solids file is built and checked for determinism in the suite."
```

---

### Task 7: The hidden line behind the BRep cylinder (design section 4)

**Files:**
- Create: `examples/mk_cylinder_hidden_probe.rs`
- Modify: `docs/_ink_suite.sh`

- [ ] **Step 1: Write the probe**

```rust
// Hidden line behind a BRep cylinder: the case the ink suite never covered. A magenta polyline
// stands behind the cylinder (seen from the front camera, VIEWER_ORBIT=0,60) so that the
// cylinder's curved side must hide its middle; its ends stick out on both sides so a run that
// hides everything is caught too. Zero magenta pixels in the frame at distance 1 and 4 is the
// acceptance, read from the colour frame the way the joint probe reads it.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_cylinder_hidden_probe -- <out.pb>
use session_rust::brep::BRep;
use session_rust::{Color, Point, Polyline, Session};

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/cylinder_hidden.pb".into());
    let mut s = Session::new("cylinder_hidden");
    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.surfacecolor = Color::grey();
    s.add_brep(cylinder, None);
    // 60 mm behind the axis, at half height, 100 mm short of each silhouette: hidden entirely.
    let mut line = Polyline::new(vec![Point::new(-50.0, 60.0, 200.0), Point::new(50.0, 60.0, 200.0)]);
    line.linecolor = Color::magenta();
    line.name = "hidden".to_string();
    s.add_polyline(line, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

Check `Polyline::new` takes `Vec<Point>` and `linecolor` is a public field (both used in `mk_mixed_solids.rs` and `mk_joint_probe.rs`). The front camera looks along +y from -y (confirm from `report_camera`'s eye line in the harness log: the eye's y must be negative; if it is positive, put the line at y = -60).

- [ ] **Step 2: Render and count**

```bash
cargo build -q --release --target x86_64-unknown-linux-gnu --example mk_cylinder_hidden_probe
"$B/mk_cylinder_hidden_probe" "$S/cyl_hidden.pb"
for d in 1 4; do
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_ORBIT=0,60 VIEWER_DISTANCE_SCALE=$d "$B/selftest" "$S/cyl_hidden_$d.ppm" "$S/cyl_hidden.pb" 2>&1 | grep -i "camera:"
  python3 docs/_count_colors.py "$S/cyl_hidden_$d.ppm"
done
```

Expected: `magenta 0` at both distances. If magenta shows, look at the frame: a line poking past the silhouette means the camera is not where the comment says (fix the probe's y or the orbit), while magenta INSIDE the cylinder's disc is a real leak of the ink rule through the curved tessellation and must be reported with the pixel count, not hidden.

- [ ] **Step 3: Add to the suite**

After the `mixed_backface` check:

```bash
# A hidden line behind a curved BRep: zero magenta at distance 1 and 4, read from the whole
# colour frame.
check cylinder_hidden "$B/mk_cylinder_hidden_probe" "$OUT/cyl_hidden.pb"
for d in 1 4; do
    check "render_cylinder_hidden_$d" env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_ORBIT=0,60 VIEWER_DISTANCE_SCALE=$d "$B/selftest" "$OUT/cyl_hidden_$d.ppm" "$OUT/cyl_hidden.pb"
    check "cylinder_hidden_$d" sh -c "read -r _ _ _ m < <(python3 docs/_count_colors.py '$OUT/cyl_hidden_$d.ppm'); [ \"\$m\" -eq 0 ]"
done
```

`sh` has no process substitution; write the count check as:

```bash
    check "cylinder_hidden_$d" bash -c "set -- \$(python3 docs/_count_colors.py '$OUT/cyl_hidden_$d.ppm'); [ \"\$4\" -eq 0 ]"
```

- [ ] **Step 4: Run the suite, commit**

```bash
docs/_ink_suite.sh 2>&1 | tail -12
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
git add examples/mk_cylinder_hidden_probe.rs docs/_ink_suite.sh
git commit -m "viewer: a hidden line behind a BRep cylinder joins the suite

Zero magenta at distance 1 and 4 through the curved tessellation."
```

---

### Task 8: The Utah teapot as one BRep, the hard case

**Files:**
- Create: `examples/mk_teapot.rs`
- Modify: `docs/_ink_suite.sh` (after the cylinder checks)

**Interfaces:**
- Produces: `mk_teapot <out.pb>` writes one BRep named `teapot`: 32 bicubic patches, shared boundaries as single edges, degenerate rows as degenerated edges, one shell per connected patch group; prints `teapot: faces F edges E vertices V shells S chained C degenerated D unchained U`.
- Consumes: `session_viewer::app::walk::brep_edges::edge_chains` and `session_viewer::app::walk::brep::QUALITY` (Tasks 2-3) for the printed chain census; the kernel's public builder `BRep::add_vertex / add_curve_3d / add_curve_2d / add_edge / add_pcurve / add_wire / add_face / add_shell`, `NurbsSurface::create`, `NurbsSurface::iso_curve(dir, c)` (the curve varies along `dir` at the other parameter fixed to `c`), `NurbsSurface::domain`, `BRep::face_meshes`.

Why: the seven primitives share every edge between a grid face and at most one other face and every seam is sampled once. The teapot is 32 grid faces whose neighbours sample the shared boundary each with their own span count, two exactly degenerate rows (the bottom's pole and the lid's top), a nearly degenerate row (the knob's 0.002 loop), and open shells - what a model from a CAD exchange looks like, and where any edge that is not read off the tessellation drifts visibly.

- [ ] **Step 1: Write the example**

The data is Newell's teapot as the GLUT sources carry it (ten input patches; rim, body, lid and bottom are turned through the four quadrants, handle and spout are mirrored in y). Copy the two tables from `$S/fg_teapot_data.h` (lines 957 to 1039): the 10 x 16 index table as `const PATCHES: [[usize; 16]; 10]` and the 129 points as `const POINTS: [[f64; 3]; 129]`, values verbatim with the `f` suffix dropped.

```rust
// The Utah teapot as a BRep: Newell's 32 bicubic Bezier patches from the ten GLUT input
// patches - rim, body, lid and bottom turned through the four quadrants, handle and spout
// mirrored in y - with every shared boundary one edge, every exactly degenerate row a
// degenerated edge, and one shell per connected patch group. The hard case for the BRep walk:
// 32 grid faces whose seams are sampled by both sides, a pole, a tip, a 0.002 loop at the knob,
// and open shells. Scaled by SCALE so it sits with the mixed solids; the original unit is a
// few teapot-widths.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_teapot -- <out.pb>
use session_rust::brep::{BRep, BRepOrientation, BRepRef};
use session_rust::{Color, NurbsCurve, NurbsSurface, Point, Session};
use session_viewer::app::walk::brep::QUALITY;
use session_viewer::app::walk::brep_edges::edge_chains;

/// Teapot units to millimetres: the body is 4 units across, so 100 makes it 400 mm, the
/// size of the mixed scene's box.
const SCALE: f64 = 100.0;

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;

const PATCHES: [[usize; 16]; 10] = [ /* the ten rows of patchdata_teapot, verbatim */ ];

const POINTS: [[f64; 3]; 129] = [ /* the 129 rows of cpdata_teapot, verbatim */ ];

/// A point of the data turned `q` quarter turns about z and scaled: (x, y) -> (-y, x) is
/// exact in floating point, so patches that share a boundary share its coordinates bit for bit.
fn turned(p: [f64; 3], q: usize) -> Point {
    let (mut x, mut y) = (p[0], p[1]);
    for _ in 0..q {
        (x, y) = (-y, x);
    }
    Point::new(x * SCALE, y * SCALE, p[2] * SCALE)
}

/// The 32 patches as 4 x 4 control grids, u slowest. A mirrored copy (y negated) would turn
/// its normal inward, so it is transposed as well, which turns the normal back out.
fn patches() -> Vec<[Point; 16]> {
    let mut out = Vec::with_capacity(32);
    for (k, idx) in PATCHES.iter().enumerate() {
        let grid: Vec<[f64; 3]> = idx.iter().map(|&i| POINTS[i]).collect();
        if k < 6 {
            for q in 0..4 {
                out.push(std::array::from_fn(|n| turned(grid[n], q)));
            }
        } else {
            out.push(std::array::from_fn(|n| turned(grid[n], 0)));
            out.push(std::array::from_fn(|n| {
                let (iu, iv) = (n / 4, n % 4);
                let p = grid[iv * 4 + iu];
                Point::new(p[0] * SCALE, -p[1] * SCALE, p[2] * SCALE)
            }));
        }
    }
    out
}

/// A point's coordinates as bits: the matching key, exact by construction.
fn key(p: &Point) -> [u64; 3] {
    [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()]
}

/// One side of a patch: its four control points in the +parameter direction, which
/// `iso_curve` argument draws it, and the corner parameters its pcurve runs between.
struct Side {
    cvs: [Point; 4],
    iso: (usize, f64),
    from: (f64, f64),
    to: (f64, f64),
}

/// The four sides of a 4 x 4 grid in wire order: v = v0 along +u, u = u1 along +v, v = v1
/// along +u (the wire walks it backwards), u = u0 along +v (backwards too).
fn sides(g: &[Point; 16], dom: ((f64, f64), (f64, f64))) -> [Side; 4] {
    let ((u0, u1), (v0, v1)) = dom;
    let at = |iu: usize, iv: usize| g[iu * 4 + iv].clone();
    [
        Side { cvs: [at(0, 0), at(1, 0), at(2, 0), at(3, 0)], iso: (0, v0), from: (u0, v0), to: (u1, v0) },
        Side { cvs: [at(3, 0), at(3, 1), at(3, 2), at(3, 3)], iso: (1, u1), from: (u1, v0), to: (u1, v1) },
        Side { cvs: [at(0, 3), at(1, 3), at(2, 3), at(3, 3)], iso: (0, v1), from: (u0, v1), to: (u1, v1) },
        Side { cvs: [at(0, 0), at(0, 1), at(0, 2), at(0, 3)], iso: (1, u0), from: (u0, v0), to: (u0, v1) },
    ]
}

/// A straight pcurve between two corners of the domain.
fn uv_line(a: (f64, f64), b: (f64, f64)) -> NurbsCurve {
    NurbsCurve::create(false, 1, &[Point::new(a.0, a.1, 0.0), Point::new(b.0, b.1, 0.0)])
}

/// The builder's memory across patches: corner points to vertex indices and boundary rows
/// to the edge that already carries them.
struct Shared {
    vertices: Vec<([u64; 3], usize)>,
    edges: Vec<([[u64; 3]; 4], usize)>,
}

impl Shared {
    /// The vertex at `p`, made on first sight.
    fn vertex(&mut self, b: &mut BRep, p: &Point) -> usize {
        let k = key(p);
        if let Some((_, v)) = self.vertices.iter().find(|(kk, _)| *kk == k) {
            return *v;
        }
        let v = b.add_vertex(p, 0.0);
        self.vertices.push((k, v));
        v
    }

    /// The edge carrying these four control points, and whether it runs the same way: +1
    /// when the stored row reads forwards, -1 when it reads backwards, None when new.
    fn edge(&self, cvs: &[Point; 4]) -> Option<(usize, i32)> {
        let fwd: [[u64; 3]; 4] = std::array::from_fn(|i| key(&cvs[i]));
        let rev: [[u64; 3]; 4] = std::array::from_fn(|i| key(&cvs[3 - i]));
        for (k, e) in &self.edges {
            if *k == fwd {
                return Some((*e, 1));
            }
            if *k == rev {
                return Some((*e, -1));
            }
        }
        None
    }
}

/// One side into the BRep for surface `si`: the edge (shared, new, or degenerated), its
/// pcurve on this face in the edge's own direction, and the wire use. `walk` is +1 when the
/// wire walks this side along +parameter, -1 backwards.
fn add_side(b: &mut BRep, sh: &mut Shared, s: &Side, ctx: (usize, i32)) -> BRepRef {
    let (si, walk) = ctx;
    let degenerate = s.cvs.iter().all(|p| key(p) == key(&s.cvs[0]));
    if degenerate {
        let v = sh.vertex(b, &s.cvs[0]) as i32;
        let e = b.add_edge(-1, v, v);
        let c = b.add_curve_2d(&uv_line(s.from, s.to)) as i32;
        b.add_pcurve(e, si, c, -1);
        return BRepRef::new(e as i32, if walk > 0 { F } else { R });
    }
    let (e, dir) = match sh.edge(&s.cvs) {
        Some(found) => found,
        None => {
            let v0 = sh.vertex(b, &s.cvs[0]) as i32;
            let v1 = sh.vertex(b, &s.cvs[3]) as i32;
            let crv = b.m_surfaces[si].iso_curve(s.iso.0, s.iso.1).expect("iso curve");
            let c3 = b.add_curve_3d(&crv) as i32;
            let e = b.add_edge(c3, v0, v1);
            sh.edges.push((std::array::from_fn(|i| key(&s.cvs[i])), e));
            (e, 1)
        }
    };
    // SameParameter: the pcurve follows the edge's 3D direction, not the side's.
    let c = if dir > 0 { uv_line(s.from, s.to) } else { uv_line(s.to, s.from) };
    let ci = b.add_curve_2d(&c) as i32;
    b.add_pcurve(e, si, ci, -1);
    BRepRef::new(e as i32, if walk * dir > 0 { F } else { R })
}

/// Faces sharing a non-degenerated edge belong to one shell: union-find over the faces.
fn components(b: &BRep) -> Vec<Vec<usize>> {
    let nf = b.m_faces.len();
    let mut parent: Vec<usize> = (0..nf).collect();
    fn root(parent: &mut [usize], i: usize) -> usize {
        let mut i = i;
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    for ei in 0..b.m_edges.len() {
        if b.m_edges[ei].degenerated {
            continue;
        }
        let uses = b.edge_faces(ei);
        for w in uses.windows(2) {
            let (a, c) = (root(&mut parent, w[0].index as usize), root(&mut parent, w[1].index as usize));
            parent[a] = c;
        }
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    for f in 0..nf {
        let r = root(&mut parent, f);
        match roots.iter().position(|&x| x == r) {
            Some(i) => groups[i].push(f),
            None => {
                roots.push(r);
                groups.push(vec![f]);
            }
        }
    }
    groups
}

/// Six times the signed volume the faces of one group enclose, summed over their triangles
/// about the origin: negative means the group's normals point inward and the shell is added
/// reversed. An open group still carries the sign of the side it mostly faces.
fn signed_volume(meshes: &[session_rust::Mesh], group: &[usize]) -> f64 {
    let mut six_v = 0.0;
    for &fi in group {
        let m = &meshes[fi];
        for verts in m.face.values() {
            if verts.len() < 3 {
                continue;
            }
            let p = |k: usize| m.vertex[&verts[k]].position();
            let (a, b, c) = (p(0), p(1), p(2));
            six_v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0]);
        }
    }
    six_v
}

/// The teapot BRep: patches to faces with shared edges, then one shell per group, reversed
/// when its volume comes out negative, and a solid for every closed shell.
fn teapot() -> BRep {
    let mut b = BRep::new();
    b.name = "teapot".to_string();
    let mut sh = Shared { vertices: Vec::new(), edges: Vec::new() };
    for g in patches() {
        let srf = NurbsSurface::create(false, false, 3, 3, 4, 4, &g).expect("bicubic patch");
        let dom = (srf.domain(0).expect("u domain"), srf.domain(1).expect("v domain"));
        let si = b.add_surface(&srf);
        let walks = [1, 1, -1, -1];
        let mut uses = Vec::with_capacity(4);
        for (s, walk) in sides(&g, dom).iter().zip(walks) {
            uses.push(add_side(&mut b, &mut sh, s, (si, walk)));
        }
        let wi = b.add_wire(&uses);
        b.add_face(si as i32, &[BRepRef::new(wi as i32, F)], 0.0);
    }
    let meshes = b.face_meshes();
    for group in components(&b) {
        let o = if signed_volume(&meshes, &group) < 0.0 { R } else { F };
        let refs: Vec<BRepRef> = group.iter().map(|&f| BRepRef::new(f as i32, o)).collect();
        let s = b.add_shell(&refs);
        if b.is_closed(s) {
            b.add_solid(&[BRepRef::new(s as i32, F)]);
        }
    }
    b.surfacecolor = Color::new(0.93, 0.90, 0.82, 1.0);
    b
}

/// What the walk will find: how many edges have a chain, how many are degenerated, and how
/// many would fall back to the curve.
fn census(b: &BRep) -> (usize, usize, usize) {
    let fms = b.face_meshes_q(Some(QUALITY));
    let chains = edge_chains(b, &fms);
    let chained = chains.iter().filter(|c| c.is_some()).count();
    let degenerated = b.m_edges.iter().filter(|e| e.degenerated).count();
    (chained, degenerated, b.m_edges.len() - chained - degenerated)
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/teapot.pb".into());
    let b = teapot();
    let (chained, degenerated, unchained) = census(&b);
    println!(
        "teapot: faces {} edges {} vertices {} shells {} chained {chained} degenerated {degenerated} unchained {unchained}",
        b.m_faces.len(), b.m_edges.len(), b.m_vertices.len(), b.m_shells.len()
    );
    let mut s = Session::new("teapot");
    s.add_brep(b, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

Check before building: `BRep.m_shells` is a public field (`grep -n "pub m_shells" ../session_rust/src/brep.rs`); `BRepRef::new(i32, BRepOrientation)`; `BRep::is_closed(shell_index)`; `Point` implements `Clone` and `Index<usize>`; `NurbsSurface::create` returns `Result`. If `session_viewer::app::walk::brep_edges` is not reachable from an example (it is `pub mod`, and `app` is `pub mod` in `lib.rs`), say so and print the census from the kernel side only.

- [ ] **Step 2: Build, run, read the census**

```bash
cargo build -q --release --target x86_64-unknown-linux-gnu --example mk_teapot
"$B/mk_teapot" "$S/teapot.pb"
```

Expected: `faces 32`, `unchained 0`, `degenerated` equal to the number of exactly degenerate patch sides (the bottom's pole row in four quadrants and the lid's tip row in four: 8), `shells` at least 4 (body with bottom, lid, handle, spout - the rim's inner ring shares nothing with the lid). If `unchained` is not 0, print which edges and on which faces (their `edge_faces`), and report: it means an iso line of a grid face carries fewer than two sample vertices, the pyramid case again, and the ribbon fallback covers it - not a failure of this task, but the numbers go in the report.

- [ ] **Step 3: Render and inspect**

```bash
for v in iso top front; do
  case $v in iso) cam=() ;; top) cam=(VIEWER_VIEW=top) ;; front) cam=(VIEWER_ORBIT=0,60) ;; esac
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 "${cam[@]}" "$B/selftest" "$S/teapot_$v.ppm" "$S/teapot.pb"
  python3 docs/_shade_scanline.py "$S/teapot_$v.ppm"
  python3 "$S/ppm2png.py" "$S/teapot_$v.ppm" "$S/teapot_$v.png"
done
"$B/check_determinism" "$S/teapot.pb"   # expected FLAKY on verts (pole normals), record it
```

Look at every PNG. Expected: `backface 0` on all three (a red patch means a shell's sign came out wrong: report which part), smooth shading over the body, spout and handle, every patch boundary inked once, the pole and the tip carrying no stray line, no line floating off the surface. Use `python3 "$S/crop.py" in.ppm out.png x0 y0 x1 y1 3` on the spout tip, the knob and a body seam and describe what you see in the report, honestly: a boundary that visibly leaves its neighbour's facets is the sampling mismatch part B fixes and must be named, not hidden.

- [ ] **Step 4: Join the suite**

After the cylinder checks in `docs/_ink_suite.sh`:

```bash
# The teapot: 32 bicubic patches, open shells, a pole and a tip. Every edge chained off the
# tessellation, no back-face pixel from above, and two loads of the file agree.
check teapot "$B/mk_teapot" "$OUT/teapot.pb"
check teapot_census bash -c "'$B/mk_teapot' '$OUT/teapot.pb' | grep -q ' unchained 0'"
check render_teapot_top env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$OUT/teapot_top.ppm" "$OUT/teapot.pb"
check teapot_backface python3 docs/_shade_scanline.py "$OUT/teapot_top.ppm" --max-backface 0
```

No determinism line for the teapot: its pole and tip carry the kernel's HashMap-ordered pole normal (see the comment above the `probe_determinism` check), so `check_determinism` on it is expected FLAKY until part B; run it once in Step 3 and record the verdict in the report.

If the census printed `unchained` above 0 in Step 2, the `teapot_census` line greps for the measured number instead and the comment says which edges and why.

- [ ] **Step 5: Clippy, suite, commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
cargo check -q --target wasm32-unknown-unknown
docs/_ink_suite.sh 2>&1 | tail -8
git add examples/mk_teapot.rs docs/_ink_suite.sh
git commit -m "viewer: the Utah teapot as a BRep joins the suite

Thirty-two bicubic patches with shared edges, a pole, a tip and open
shells: <E> edges, <C> chained off the tessellation, <D> degenerated."
```

Fill the numbers from the census.

---

### Task 9: Perf after, the ledger, the architecture map

**Files:**
- Modify: `docs/_PERF.md`, `ARCHITECTURE.md`

- [ ] **Step 1: Measure the twelve legs AFTER**

Same command as Task 1 step 2, into `$S/bench/after.txt`, on an otherwise idle desktop, CPU frequency confirmed unthrottled. Then:

```bash
paste <(grep -E "^==|still|moving" "$S/bench/before.txt") <(grep -E "still|moving" "$S/bench/after.txt")
```

Every after leg must be at most its before leg. `view_mixed` is the only scene with BReps (seven solids in one file of 13 objects), so it is the one that may move; `view_meshes` and `view_lines` have none and should be within noise. If a leg is slower, rerun both once; if it stays slower, report the numbers and stop - do not tune.

- [ ] **Step 2: Rewrite the ledger**

Replace the table and its paragraph in `docs/_PERF.md` with the new "before" (commit of Task 1, the weld) and "after" (commit of Task 8) columns, the date, both adapters as confirmed from the `adapter:` log line, and one sentence on the observed change in `view_mixed`. Keep the rules paragraph at the end and the ink fragment cost paragraph (unchanged by this work). Delete the previous table: a number that was not re-measured today is deleted, not carried over (`docs/_PERF.md`'s own rule).

- [ ] **Step 3: Update the architecture map**

In `ARCHITECTURE.md`:

- Section 0 table row `files` for `app/walk/`: add `brep_edges.rs` after `brep.rs`.
- Section 1 tree is by directory and needs no change.
- Section 6, replace the bullet beginning `Smooth tessellations (BRep and NURBS fills) ink border and crease edges only` with:

```markdown
- A BRep is uploaded face by face with the kernel's own vertices and analytic normals - no
  weld - so the headlight shades a sphere as a gradient (`src/app/walk/brep.rs`). Its edges
  are pipes read off the owning face's tessellation grid, one chain per BRep edge
  (`src/app/walk/brep_edges.rs`): the grid mesher samples every boundary on an iso line and
  tags each vertex with its exact parameter, so the chain IS the facet boundary and needs no
  tolerance; `facing` packs the owning face's normal and the other adjacent face's nearest
  vertex normal. An edge no grid face owns is sampled off its 3D curve as a ribbon until the
  kernel's `edge_polygons_q` lands (phase 2 part B). `FLAG_OPEN` on a BRep is its own
  `is_solid`. NURBS surface fills ink border and crease edges only, decided on the CPU by the
  walk (`mesh_ink.rs`, `CREASE_COS` in `mesh.rs`); there is no view-dependent silhouette
  term, so nothing flips as the camera turns. `FLAG_SMOOTH` only suppresses vertex markers,
  since a tessellation's vertices are sample positions, not corners; the vertex-stage facing
  cull (both adjacent faces away) and `FLAG_INSIDE`/`FLAG_OPEN` are as before.
```

- Section 11: update the check count in `docs/_ink_suite.sh: ... one PASS/FAIL line each (24 checks)` to the count the suite now prints (count the `PASS` lines of the Task 8 run).

- [ ] **Step 4: Full verification, then commit**

```bash
cargo xtest -q 2>&1 | tail -3
cargo check -q --target wasm32-unknown-unknown
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
docs/_ink_suite.sh 2>&1 | tail -3
git add docs/_PERF.md ARCHITECTURE.md
git commit -m "viewer: the ledger and the map after unwelded BRep faces

Twelve legs re-measured on both adapters; view_mixed <before> -> <after>
ms still on the Intel iGPU."
```

- [ ] **Step 5: Report to the user**

Hand back: the before/after scanline numbers, the perf table, the suite's last lines, the pick lines, the paths of `sphere_before`, `sphere_after`, `mixed_iso`, `mixed_top`, `torus_iso`, `hole_iso`, `dome_iso` renders, and the branch state (`git log --oneline main..brep-phase2`). Do not merge and do not push: the user looks at the renders first.

---

### Task 10: A compact mixed-solids scene

**Files:**
- Modify: `examples/mk_mixed_solids.rs`

Why: the user's request of 2026-09-08 - the thirteen objects stand on one 800 mm grid in a single row, 10 m wide for objects 200 to 600 mm across, so the whole scene is a thin line and every object is a few pixels tall when the scene is fitted. Two rows, 700 mm apart, make it 4.2 by 1.5 m without moving any object off its slot number.

- [ ] **Step 1: Two rows**

In `examples/mk_mixed_solids.rs`:

Find:
```rust
// polyline, in one row along x on an 800 mm grid, every solid standing on z = 0. The point is
```
Replace with:
```rust
// polyline, in two rows 700 mm apart - the seven BReps along y = 0, the curves, surfaces and
// the polyline along y = ROW - every solid standing on z = 0. The point is
```

Find:
```rust
/// Centre-to-centre spacing of the row: every item is narrower than this, so nothing overlaps.
const STEP: f64 = 800.0;
```
Replace with:
```rust
/// Centre-to-centre spacing along a row: the widest item (the torus, 580 mm) leaves 120 mm.
const STEP: f64 = 700.0;

/// Distance between the two rows: the deepest items (the patches, 600 mm) leave 300 mm.
const ROW: f64 = 900.0;

/// Items per row: the seven BReps fill the first row, the six others the second.
const PER_ROW: usize = 7;
```

Find:
```rust
/// The x of slot `i` of the row.
fn slot(i: usize) -> f64 {
    i as f64 * STEP
}
```
Replace with:
```rust
/// The x of slot `i`: slots wrap after PER_ROW.
fn slot(i: usize) -> f64 {
    (i % PER_ROW) as f64 * STEP
}

/// The y of slot `i`: the first PER_ROW slots on y = 0, the rest one ROW behind.
fn row(i: usize) -> f64 {
    (i / PER_ROW) as f64 * ROW
}
```

Then every helper that builds points at `slot(i)` gets the row's y added. Find:
```rust
    b.transform(&Xform::translation(slot(i), 0.0, up));
```
Replace with:
```rust
    b.transform(&Xform::translation(slot(i), row(i), up));
```
Find (helix):
```rust
        points.push(Point::new(slot(i) + 200.0 * a.cos(), 200.0 * a.sin(), 300.0 + 600.0 * t));
```
Replace with:
```rust
        points.push(Point::new(slot(i) + 200.0 * a.cos(), row(i) + 200.0 * a.sin(), 300.0 + 600.0 * t));
```
Find (S curve):
```rust
        points.push(Point::new(slot(i) + 250.0 * a.sin(), 0.0, 300.0 + 600.0 * t));
```
Replace with:
```rust
        points.push(Point::new(slot(i) + 250.0 * a.sin(), row(i), 300.0 + 600.0 * t));
```
Find (ring):
```rust
        points.push(Point::new(slot(i) + 250.0 * a.cos(), 250.0 * a.sin(), 600.0 + 120.0 * a.sin()));
```
Replace with:
```rust
        points.push(Point::new(slot(i) + 250.0 * a.cos(), row(i) + 250.0 * a.sin(), 600.0 + 120.0 * a.sin()));
```
Find (box top, stays on the box in slot 0, whose row is 0):
```rust
        points.push(Point::new(slot(i) - 150.0 + 300.0 * t, 100.0 * (t * 6.0).sin(), z));
```
Replace with:
```rust
        points.push(Point::new(slot(i) - 150.0 + 300.0 * t, row(i) + 100.0 * (t * 6.0).sin(), z));
```
Find (dome):
```rust
            points.push(Point::new(slot(i) + x, y, 300.0 + 200.0 * (r / 250.0).cos()));
```
Replace with:
```rust
            points.push(Point::new(slot(i) + x, row(i) + y, 300.0 + 200.0 * (r / 250.0).cos()));
```
Find (saddle):
```rust
            points.push(Point::new(slot(i) + x, y, 250.0 + 0.0006 * (x * x - y * y)));
```
Replace with:
```rust
            points.push(Point::new(slot(i) + x, row(i) + y, 250.0 + 0.0006 * (x * x - y * y)));
```
Find (cube polyline):
```rust
        points.push(Point::new(slot(i) + c[0], c[1], c[2]));
```
Replace with:
```rust
        points.push(Point::new(slot(i) + c[0], row(i) + c[1], c[2]));
```

Check with `grep -n "slot(" examples/mk_mixed_solids.rs` that no point builder still passes a bare `0.0` or a bare `c[1]`-style y where the slot's row belongs; the sphere/torus/... `place` calls carry it through `row(i)`.

- [ ] **Step 2: Build, render, look**

```bash
cargo build -q --release --target x86_64-unknown-linux-gnu --example mk_mixed_solids --example selftest
"$B/mk_mixed_solids" "$S/mixed_compact.pb"
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 "$B/selftest" "$S/mixed_compact_iso.ppm" "$S/mixed_compact.pb"
env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$S/mixed_compact_top.ppm" "$S/mixed_compact.pb"
python3 docs/_shade_scanline.py "$S/mixed_compact_top.ppm"
python3 "$S/ppm2png.py" "$S/mixed_compact_iso.ppm" "$S/mixed_compact_iso.png"
python3 "$S/ppm2png.py" "$S/mixed_compact_top.ppm" "$S/mixed_compact_top.png"
```

Look at both PNGs: two rows, no object touching another, the red curve still on the box top, `backface 0`. The bench legs keep using the bucket's `view_mixed_solids.pb` (a before/after comparison must be on the same file); the compact file replaces it in the bucket when the user uploads it.

- [ ] **Step 3: Clippy, suite's mixed checks, commit**

```bash
cargo clippy -q --release --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
git add examples/mk_mixed_solids.rs
git commit -m "viewer: the mixed solids stand in two rows, 700 mm apart

Thirteen objects 200 to 600 mm across on one 800 mm line made a 10 m
scene; two rows make it 4.2 by 1.5 m and every object legible when the
scene is fitted."
```

---

## Self-review

- **Spec coverage.** 2.2 "walk_brep no longer welds ... own vertices, analytic normals, FLAG_SMOOTH": Task 4. "BRep edges become pipes ... facing from the two adjacent facets' normals ... no FACING_UNKNOWN ribbons for BReps": Tasks 3 and 4 (the ribbon fallback exists for BReps outside the mixed scene and is named as temporary). "NURBS surfaces: walk_surface keeps the grid path's normals; its border edges come from the same attributes": deferred to part C by the user's order of work; `walk_surface` is unchanged. "The facing cull's closed test uses is_solid": Task 4. 2.3 smoothness with the 5-degree quality: Task 1/4 measure it. Section 4: orbit series before/after through the suite (Task 6), the hidden line behind the cylinder (Task 7), the scanline (Tasks 1, 4, 6). The teapot hard case (user request of 2026-09-08): Task 8. Perf and the ledger: Tasks 1 and 9. Docs: Task 9.
- **Placeholders.** The `<N>`, `<before>`, `<after>` markers are measurement slots the executor fills from its own runs; every other step carries its code.
- **Type consistency.** `EdgeUse { edge, face, orientation }`, `iso_chain(&BRep, &Mesh, &EdgeUse) -> Option<Vec<usize>>`, `EdgeChain { face, keys, other }`, `edge_chains(&BRep, &[Mesh]) -> Vec<Option<EdgeChain>>`, `EdgePen { fms, pen }`, `push_edge_pipes(&mut SegRows, &EdgeChain, &EdgePen, &mut Aabb) -> usize` are used with these exact shapes in Tasks 2, 3 and 4; `QUALITY` is `pub` from Task 2 on; `mesh_spacing` is `pub(super)` from Task 4.
