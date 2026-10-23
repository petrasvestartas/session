# 9.3 · Trimmed curved faces with real open holes

**Goal:** Prove trimmed curved faces and open holes. · **Steps:** 2 · **Source:** [app/walk/brep.rs](../../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) (`src/app/walk/brep_shapes.rs,src/app/walk/brep_orient.rs,src/shaders/normals.wgsl`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 9.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cad_samples.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cad_trim_tests.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">2 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_shapes.…</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_orient.…</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep the trimmed face con…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Trimmed B-Rep (u,v)</text>
  <rect x="620" y="52" width="160" height="88" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="700" cy="96" r="24" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Hole</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Trimmed curved faces with r…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Crease Split Normals</text>
  <polyline points="894,136 974,90 1054,136" fill="none" stroke="#111111" stroke-width="2.5"/>
  <circle cx="974" cy="90" r="4.5" fill="#111111"/>
  <line x1="974" y1="90" x2="942" y2="52" stroke="#2563eb" stroke-width="2.25" marker-end="url(#arrB)"/>
  <line x1="974" y1="90" x2="1006" y2="52" stroke="#dc2626" stroke-width="2.25" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="924" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">N1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1012" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">N2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove trimmed curved face…</text>
</svg>
</div>

## <a id="checkpoint-57-parameters"></a>1. Keep the trimmed face connected to its surface

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cad_samples.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">trimmed mesh vertex</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source surface and UV</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/walk/brep_s…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact point and normal</text>
</svg>
</div>

**What this code does:** Keep the trimmed face connected to its surface.

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

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cad_trim_tests.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">outer wire and hole</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained face samples</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/walk/brep_s…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged source</text>
</svg>
</div>

**What this code does:** Prove trimmed curved faces and open holes.

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
