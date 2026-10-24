# 9.3 · Trimmed curved faces with real open holes

**Goal:** Prove trimmed curved faces and open holes. · **Steps:** 2 · **Source:** [app/walk/brep.rs](../../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs)

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

## <a id="checkpoint-57-parameters"></a>1. Keep the trimmed face connected to its surface

### 1. `src/cad_samples.rs`

Retain original surface addresses and exact UV samples on a trimmed face.

Create the file and type:

```rust
pub struct Sample {
    pub key: usize,
    pub surface: usize,
    pub uv: [f64; 2],
    pub point: [f64; 3],
    pub normal: [f64; 3],
}

pub fn face(
    prepared: &crate::cad_boundary::Prepared,
    index: usize,
) -> Result<Vec<Sample>, &'static str> {
    let original = prepared
        .source
        .m_faces
        .get(index)
        .ok_or("Missing original CAD face")?;
    let surface = prepared
        .source
        .m_surfaces
        .get(original.surface_index as usize)
        .ok_or("Missing original face surface")?;
    let mesh = prepared
        .faces
        .get(index)
        .ok_or("Missing prepared face mesh")?;
    let mut rows = mesh.vertex.iter().collect::<Vec<_>>();
    rows.sort_unstable_by_key(|(key, _)| **key);
    let mut out = Vec::new();
    for (&key, vertex) in rows {
        let (Some(&u), Some(&v)) = (
            vertex.attributes.get("u"),
            vertex.attributes.get("v"),
        ) else {
            continue;
        };
        let point = surface
            .point_at(u, v)
            .ok_or("Face UV sample cannot be evaluated")?;
        let normal = surface.normal_at(u, v);
        let point = [point[0], point[1], point[2]];
        let normal = [normal[0], normal[1], normal[2]];
        if point
            .iter()
            .chain(&normal)
            .any(|v| !v.is_finite())
        {
            return Err("Face samples must be finite");
        }
        out.push(Sample {
            key,
            surface: original.surface_index as usize,
            uv: [u, v],
            point,
            normal,
        });
    }
    Ok(out)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod cad_boundary_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod cad_boundary_tests;

pub mod cad_samples;
```

## <a id="checkpoint-57a-trims"></a>2. Prove trimmed curved faces and open holes

### 1. `src/cad_trim_tests.rs`

Copy curved-trim source/UV/normal checks and the hole-region triangle regression.

Create the file and type:

```rust
use crate::cad_boundary::Prepared;
use session_rust::brep::{BRepOrientation, BRepRef};
use session_rust::{BRep, NurbsCurve, Point, Primitives};
use std::rc::Rc;

fn trimmed_wave() -> BRep {
    let surface = Primitives::wave_surface(1., 0.5);
    let mut source = BRep::new();
    let si = source.add_surface(&surface);
    let corners = [
        [0.1, 0.1],
        [0.9, 0.1],
        [0.9, 0.9],
        [0.1, 0.9],
    ];
    for uv in corners {
        source.add_vertex(&surface.point_at(uv[0], uv[1]).unwrap(), 0.);
    }
    let mut edges = Vec::new();
    for side in 0..4 {
        let a = corners[side];
        let b = corners[(side + 1) % 4];
        let dir = usize::from(a[0] == b[0]);
        let mut curve = surface.iso_curve(dir, a[1 - dir]).unwrap();
        assert!(curve.trim(0.1, 0.9));
        if a[dir] > b[dir] {
            curve.reverse();
        }
        let ci = source.add_curve_3d(&curve);
        let edge = source.add_edge(
            ci as i32,
            side as i32,
            ((side + 1) % 4) as i32,
        );
        let pc = NurbsCurve::create(
            false,
            1,
            &[
                Point::new(a[0], a[1], 0.),
                Point::new(b[0], b[1], 0.),
            ],
        );
        let ci = source.add_curve_2d(&pc);
        source.add_pcurve(edge, si, ci as i32, -1);
        edges.push(BRepRef::new(
            edge as i32,
            BRepOrientation::Forward,
        ));
    }
    let wi = source.add_wire(&edges);
    source.add_face(
        si as i32,
        &[BRepRef::new(
            wi as i32,
            BRepOrientation::Forward,
        )],
        0.,
    );
    source
}

#[test]
fn trimmed_curved_face_retains_uvs_analytic_normals_and_original_source() {
    let source = Rc::new(trimmed_wave());
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    let samples = crate::cad_samples::face(&prepared, 0).unwrap();
    assert_eq!(samples.len(), prepared.faces[0].vertex.len());
    assert!(samples.len() > 16);
    for sample in samples {
        assert!(
            sample
                .uv
                .iter()
                .all(|v| *v >= 0.1 - 1e-10 && *v <= 0.9 + 1e-10)
        );
        assert_eq!(sample.surface, 0);
        let v = &prepared.faces[0].vertex[&sample.key];
        assert_eq!(sample.point, [v.x, v.y, v.z]);
        assert!(
            (sample
                .normal
                .iter()
                .map(|v| v * v)
                .sum::<f64>()
                - 1.)
                .abs()
                < 1e-10
        );
    }
    assert!(
        prepared
            .chains
            .iter()
            .all(|c| c.as_ref().is_some_and(|c| c.keys.len() > 4))
    );
    assert_eq!(source.to_proto(), before);
}

#[test]
fn hole_face_keeps_both_wires_and_never_fills_its_interior() {
    let mut source = BRep::create_block_with_hole(8., 6., 2., 1.);
    source.m_faces = vec![source.m_faces[5].clone()];
    source.m_shells.clear();
    source.m_solids.clear();
    let source = Rc::new(source);
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    assert_eq!(source.m_faces[0].wires.len(), 2);
    let mesh = &prepared.faces[0];
    assert!(!mesh.face.is_empty());
    for tri in mesh.face.values() {
        let p = tri
            .iter()
            .map(|key| &mesh.vertex[key])
            .collect::<Vec<_>>();
        let x = p.iter().map(|v| v.x).sum::<f64>() / p.len() as f64;
        let y = p.iter().map(|v| v.y).sum::<f64>() / p.len() as f64;
        assert!(
            x * x + y * y > 0.99,
            "A triangle fills the hole at {x},{y}"
        );
    }
    assert_eq!(prepared.chains.iter().flatten().count(), 5);
    assert_eq!(source.to_proto(), before);
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod cad_boundary_tests;

pub mod cad_samples;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cad_boundary_tests;

pub mod cad_samples;

#[cfg(test)]
mod cad_trim_tests;
```

## Build & Run

![Recorded visual output for Lesson 9.3 (57a-trims)](../../../screenshots/journey/57a-trims.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.3
npm --prefix ../session_tests run course -- save 9.3
```

Continue with [9.4 · Periodic seams and face-edge direction](../../consolidation/58-uses.md).
