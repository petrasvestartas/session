# 9.4 · Periodic seams and face-edge direction

**Goal:** Match shared BRep edges across independently keyed face meshes. · **Steps:** 3 · **Source:** [app/walk/brep.rs](../../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">(u,v) Outer Loop + Inner Hole Trim Loop</text>
  <rect x="64" y="48" width="212" height="106" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="170" cy="101" r="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="64" y1="48" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="48" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="64" y1="154" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="154" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crease Angle &gt; θ: Split Normals N1, N2 vs Smooth N</text>
  <line x1="426" y1="138" x2="496" y2="88" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="566" y2="138" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="464" y2="44" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="496" y1="88" x2="528" y2="44" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arrB)"/>
  <path d="M 596 138 Q 646 72 696 138" fill="none" stroke="#111111" stroke-width="2.5"/>
  <line x1="646" y1="105" x2="646" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crisp Crease Shading + Boundary Wire Edges</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,128 934,62 934,146" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="934,62 1022,128 934,146" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <ellipse cx="904" cy="112" rx="12" ry="8" fill="#ffffff" stroke="#dc2626" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-58-uses"></a>1. Keep both parameter-space uses of a seam

### 1. `src/cad_seams.rs`

Retain both original face uses and pcurve identities at a closed seam.

Create the file and type:

```rust
use session_rust::brep::BRepOrientation;
pub struct EdgeUse {
    pub face: usize,
    pub orientation: BRepOrientation,
    pub pcurve: usize,
    pub keys: Option<Vec<usize>>,
}

pub fn uses(
    prepared: &crate::cad_boundary::Prepared,
    edge: usize,
) -> Result<Vec<EdgeUse>, &'static str> {
    if edge >= prepared.source.m_edges.len() {
        return Err("Missing original CAD edge");
    }
    prepared
        .source
        .edge_faces(edge)
        .into_iter()
        .map(|use_| {
            let face =
                usize::try_from(use_.index).map_err(|_| "Invalid incident face")?;
            let mesh = prepared
                .faces
                .get(face)
                .ok_or("Missing incident face mesh")?;
            let pcurve = usize::try_from(prepared.source.pcurve_index(
                edge,
                face,
                use_.orientation,
            ))
            .map_err(|_| "Missing edge pcurve")?;
            let keys = crate::cad_edges::iso_chain(
                &prepared.source,
                mesh,
                &crate::cad_edges::EdgeUse {
                    edge,
                    face,
                    orientation: use_.orientation,
                },
            );
            Ok(EdgeUse {
                face,
                orientation: use_.orientation,
                pcurve,
                keys,
            })
        })
        .collect()
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod cad_trim_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod cad_trim_tests;

pub mod cad_seams;
```

## <a id="checkpoint-58a-periodic"></a>2. Prove seam continuity and suppress false open borders

### 1. `src/cad_seam_tests.rs`

Copy actual same-face seam use and periodic-grid continuity checks.

Create the file and type:

```rust
use session_rust::{BRep, NurbsSurface, Point};
use std::rc::Rc;

#[test]
fn seam_keeps_both_original_pcurves_on_the_same_face() {
    let source = Rc::new(BRep::create_cylinder(2., 3.));
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = crate::cad_boundary::Prepared::new(source.clone()).unwrap();
    let uses = crate::cad_seams::uses(&prepared, 2).unwrap();
    assert_eq!(uses.len(), 2);
    assert_eq!(uses[0].face, uses[1].face);
    assert_ne!(uses[0].pcurve, uses[1].pcurve);
    assert_ne!(uses[0].orientation, uses[1].orientation);
    let mesh = &prepared.faces[uses[0].face];
    let a = uses[0].keys.as_ref().unwrap();
    let b = uses[1].keys.as_ref().unwrap();
    assert!(a.len() >= 2 && b.len() >= 2);
    for key in a {
        let v = &mesh.vertex[key];
        assert!(b.iter().any(|key| {
            let p = &mesh.vertex[key];
            (v.x - p.x).abs() < 1e-9
                && (v.y - p.y).abs() < 1e-9
                && (v.z - p.z).abs() < 1e-9
        }));
    }
    assert_eq!(source.to_proto(), before);
}

#[test]
fn periodic_surface_grid_keeps_closed_borders_without_drawing_a_seam() {
    let points = (0..10)
        .flat_map(|i| {
            let angle = std::f64::consts::TAU * (i % 8) as f64 / 8.;
            (0..2).map(move |j| Point::new(angle.cos(), angle.sin(), j as f64))
        })
        .collect::<Vec<_>>();
    let surface = NurbsSurface::create(true, false, 2, 1, 10, 2, &points).unwrap();
    assert!(surface.is_periodic(0));
    let before = surface.to_proto();
    let grid = crate::surface::Grid::new(&surface).unwrap();
    assert_eq!(
        grid.boundaries
            .iter()
            .map(|b| b.edge)
            .collect::<Vec<_>>(),
        vec![2, 3]
    );
    let [nu, nv] = grid.steps;
    for j in 0..=nv {
        let a = &grid.samples[j];
        let b = &grid.samples[nu * (nv + 1) + j];
        assert!(
            a.point
                .iter()
                .zip(b.point)
                .all(|(a, b)| (a - b).abs() < 1e-10)
        );
        assert!(
            a.normal
                .iter()
                .zip(b.normal)
                .all(|(a, b)| (a - b).abs() < 1e-9)
        );
        assert_ne!(a.uv[0], b.uv[0]);
    }
    assert_eq!(surface.to_proto(), before);
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod cad_trim_tests;

pub mod cad_seams;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cad_trim_tests;

pub mod cad_seams;

#[cfg(test)]
mod cad_seam_tests;
```

## <a id="checkpoint-58za-match-original-face-edge-directions"></a>3. Match face edge directions

### 1. `src/cad_orient.rs`

Type the complete shared face-edge helpers.

Create the file and type:

```rust
use crate::cad_edges::EdgeChain;
use session_rust::{BRep, Mesh};

fn walks(fm: &Mesh, s: usize, n: usize) -> Option<bool> {
    let fwd = occupied_halfedge(fm, s, n);
    let back = occupied_halfedge(fm, n, s);

    match (fwd, back) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

fn occupied_halfedge(fm: &Mesh, from: usize, to: usize) -> bool {
    let Some(neighbours) = fm.halfedge.get(&from) else {
        return false;
    };
    matches!(neighbours.get(&to), Some(Some(_)))
}

fn at(fm: &Mesh, k: usize) -> [f64; 3] {
    let v = &fm.vertex[&k];
    [v.x, v.y, v.z]
}

fn neighbour_along(fm: &Mesh, s: usize, dir: [f64; 3]) -> Option<usize> {
    let p = at(fm, s);
    let mut best: Option<(f64, usize)> = None;

    for &w in fm.halfedge.get(&s)?.keys() {
        let q = at(fm, w);
        let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();

        if l == 0.0 {
            continue;
        }

        let c = (d[0] * dir[0] + d[1] * dir[1] + d[2] * dir[2]) / l;

        if match best {
            Some((bc, bw)) => c > bc || (c == bc && w < bw),
            None => true,
        } {
            best = Some((c, w));
        }
    }

    Some(best?.1)
}

fn vertex_at(fm: &Mesh, p: [f64; 3]) -> Option<usize> {
    let mut best: Option<(f64, usize)> = None;

    for (&k, v) in fm.vertex.iter() {
        let d = (v.x - p[0]).powi(2) + (v.y - p[1]).powi(2) + (v.z - p[2]).powi(2);

        if match best {
            Some((bd, bk)) => d < bd || (d == bd && k < bk),
            None => true,
        } {
            best = Some((d, k));
        }
    }

    Some(best?.1)
}

fn opposed(fms: &[Mesh], c: &EdgeChain) -> Option<bool> {
    let other = c.other?;
    let (fa, fb) = (&fms[c.face], &fms[other]);
    let (s, n) = (c.keys[0], c.keys[1]);
    let away_a = walks(fa, s, n)?;
    let (ps, pn) = (at(fa, s), at(fa, n));
    let dir = [pn[0] - ps[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?;
    let nb = neighbour_along(fb, sb, dir)?;
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}
```

### 2. `src/lib.rs`

Compile the one original face-orientation owner.

**Locate this existing block:**

```rust
#[cfg(test)]
mod cad_seam_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod cad_seam_tests;

pub mod cad_orient;
```

## Build & Run

![Recorded visual output for Lesson 9.4 (58za-match-original-face-edge-directions)](../../../screenshots/journey/58za-match-original-face-edge-directions.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.4
npm --prefix ../session_tests run course -- save 9.4
```

Continue with [9.5 · Oriented CAD solids, original picks and empty bores](../../consolidation/59-orientation.md).
