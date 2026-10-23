# 16.4 · Measurements, temporary marks and retained CAD pieces

**Goal:** Count derived pieces before allocating them. · **Steps:** 37 · **Source:** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs) · [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs) · [app/splitting.rs](../../../../src/app/splitting.rs) · [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs) · [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs) · [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs) · [app/walk/frames.rs](../../../../src/app/walk/frames.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 16.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/measure_volume.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">37 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Measure closed-solid volu…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Rg32Uint [R:owner, G:sub]</text>
  <rect x="624" y="50" width="152" height="92" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="624" y="50" width="50" height="30" fill="#fee2e2" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">R:id=4</text>
  <rect x="674" y="50" width="52" height="30" fill="#dcfce7" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">G:sub=2</text>
  <line x1="674" y1="50" x2="674" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="726" y1="50" x2="726" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="80" x2="776" y2="80" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="111" x2="776" y2="111" stroke="#111111" stroke-width="1"/>
  <rect x="674" y="80" width="52" height="31" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 3]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Measurements, temporary mar…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Explode supported families</text>
</svg>
</div>

## <a id="checkpoint-84-measure-original-curves-surfaces-and-closed-solids"></a>1. Measure closed-solid volume

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_volume.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure or refusal</text>
</svg>
</div>

**What this code does:** Reject open geometry, choose a world point near the mesh and reuse the shared swept-volume helper.

### 1. `src/measure_volume.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-1.rs"
```

### 2. `src/lib.rs`

Register this original measurement owner.

**Locate this existing block:**

```rust

pub mod measure_area;
```

After that block, type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-2.rs" skip=23
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/measure_volume.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```text
use crate::measure_math::compute_swept;
use crate::cad_edges::edge_chains;
use crate::cad_orient::face_signs;
use session_rust::{BRep,Geometry,Mesh,Point,Xform};
use session_rust::element::ElementGeometry;
const OPEN:&str="open";
/// World volume of a closed mesh or solid BRep; Err when open, None when the kind has no volume.
pub fn compute_volume(geometry: &Geometry, place: &Xform) -> Option<Result<f64, &'static str>> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
        Geometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
        Geometry::NurbsSurface(_) => Some(Err(OPEN)),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
            ElementGeometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// Volume of a closed mesh, from its drawn triangles.
fn compute_mesh_volume(mesh: &Mesh, place: &Xform) -> Result<f64, &'static str> {
    if !mesh.is_closed() {
        return Err(OPEN);
    }

    let origin = origin_of(mesh, place);
    Ok(compute_swept(mesh, place, &origin).abs() / 6.0)
}

/// Volume of a solid BRep as the viewer meshes it, each face turned outward first.
fn compute_brep_volume(brep: &BRep, place: &Xform) -> Result<f64, &'static str> {
    if !brep.is_solid() {
        return Err(OPEN);
    }

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    let chains = edge_chains(brep, &meshes);
    let signs = face_signs(brep, &meshes, &chains);
    let Some(origin) = meshes.iter().find(|mesh| !mesh.vertex.is_empty()) else {
        return Err(OPEN);
    };
    let origin = origin_of(origin, place);
    let swept: f64 = meshes
        .iter()
        .zip(&signs)
        .map(|(mesh, sign)| sign * compute_swept(mesh, place, &origin))
        .sum();
    Ok(swept.abs() / 6.0)
}

/// A world point on the mesh, so far-away models keep their precision.
fn origin_of(mesh: &Mesh, place: &Xform) -> Point {
    mesh.faces()
        .first()
        .and_then(|face| mesh.vertex_point(*mesh.face[face].first()?))
        .map_or(Point::new(0.0, 0.0, 0.0), |p| p.transformed(place))
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-3.rs" skip=2289
```

## <a id="checkpoint-84a-validate-an-original-box-and-derive-wire-only-display"></a>2. Validate an box and derive wire-only display

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/bounds_or…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original box frame</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite orthonormal valida…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">twelve display edges</text>
</svg>
</div>

**What this code does:** Validate an original box and derive wire-only display.

### 1. `src/scene.rs`

Retain an independent original geometry owner beside the existing mesh display.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }
```

Replace it with:

```rust
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {original:None,cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }
```

**Locate this block:**

```rust

#[derive(Clone)]
pub struct Object {
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
```

Replace it with:

```rust

#[derive(Clone)]
pub struct Object {
    pub original:Option<session_rust::Geometry>,
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
```

### 2. `src/bounds_original.rs`

Validate original box frames and prepare a wire-only display.

Create the file and type:

```rust
--8<-- "journey/code/84a-validate-an-original-box-and-derive-wire-only-display-03.rs"
```

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust

pub mod measure_volume;
```

After that block, type:

```rust
--8<-- "journey/code/84a-validate-an-original-box-and-derive-wire-only-display-04.rs" skip=25
```

## <a id="checkpoint-84b-adopt-original-boxes-and-retain-precise-placed-bounds"></a>3. Adopt boxes and retain precise placed bounds

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bounds_original.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original corners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stable source row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked replacement</text>
</svg>
</div>

**What this code does:** Adopt original boxes and retain precise placed bounds.

### 1. `src/bounds_original.rs`

Adopt and place original boxes without substituting their display proxy.

**Locate this existing block:**

```rust
    })
}
```

After that block, type:

```rust
--8<-- "journey/code/84b-adopt-original-boxes-and-retain-precise-placed-bounds-01.rs" skip=9
```

### 2. `src/scene.rs`

Use original box bounds and preserve the original family during source replacement.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {
```

Replace it with:

```rust
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        if self.objects.iter().any(|row|row.id==target.owner&&row.original.is_some()){return Err("Part editing for this original family connects in the complete command inventory")}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {
```

**Locate this block:**

```rust
        let original=crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(original.guid());
        match geometry{
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}self.points.iter_mut().find(|r|r.id==id).ok_or("Expected a point source")?.prepared=prepared;},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{let prepared=crate::stroke::PreparedLine::new(source)?;for p in [prepared.source.start(),prepared.source.end()]{let p=model.transform_point(&p);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=prepared;},
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
```

Replace it with:

```rust
        let original=crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(original.guid());
        match geometry{
            Geometry::OBB(source)=>{let prepared=crate::bounds_original::prepared(&source)?;let bounds=crate::bounds_original::placed(&source,&model);if bounds.min.into_iter().chain(bounds.max).any(|v|!(v as f32).is_finite()){return Err("Edited box exceeds display range")}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a box source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::OBB(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}self.points.iter_mut().find(|r|r.id==id).ok_or("Expected a point source")?.prepared=prepared;},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{let prepared=crate::stroke::PreparedLine::new(source)?;for p in [prepared.source.start(),prepared.source.end()]{let p=model.transform_point(&p);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=prepared;},
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
```

**Locate this block:**

```rust
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
```

Replace it with:

```rust
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::OBB(source))=&object.original{return Some(crate::bounds_original::placed(source,&object.model))}
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
```

### 3. `src/source_edit.rs`

Return the original box allocation before considering display geometry.

Replace the entire file with:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::{Geometry, Xform};
use std::rc::Rc;
pub fn original(scene: &Scene, id: ObjectId) -> Option<Geometry> {
    if let Some(row) = scene.objects().iter().find(|r| r.id == id) {
        if let Some(original)=&row.original{return Some(original.clone())}
        if let Some(cad) = &row.cad {
            return Some(Geometry::BRep(cad.source.clone()));
        }
        if let Some(s) = &row.surface {
            return Some(Geometry::NurbsSurface(s.source.clone()));
        }
        return row.geometry().map(|g| Geometry::Mesh(g.clone()));
    }
    if let Some(row) = scene.points().iter().find(|r| r.id == id) {
        return Some(Geometry::Point(row.prepared.source.clone()));
    }
    crate::gather_draft::Draft::source(scene, id)
}
pub fn same(a: &Geometry, b: &Geometry) -> bool {
    match (a, b) {
        (Geometry::OBB(a), Geometry::OBB(b)) => Rc::ptr_eq(a, b),
        (Geometry::Line(a), Geometry::Line(b)) => Rc::ptr_eq(a, b),
        (Geometry::Polyline(a), Geometry::Polyline(b)) => Rc::ptr_eq(a, b),
        (Geometry::NurbsCurve(a), Geometry::NurbsCurve(b)) => Rc::ptr_eq(a, b),
        (Geometry::Point(a), Geometry::Point(b)) => Rc::ptr_eq(a, b),
        (Geometry::Mesh(a), Geometry::Mesh(b)) => Rc::ptr_eq(a, b),
        (Geometry::BRep(a), Geometry::BRep(b)) => Rc::ptr_eq(a, b),
        (Geometry::NurbsSurface(a), Geometry::NurbsSurface(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}
#[derive(Clone)]
pub struct Owner {
    pub id: ObjectId,
    pub source: Geometry,
    pub model: Xform,
}
impl Owner {
    pub fn capture(scene: &Scene, id: ObjectId) -> Result<Self, &'static str> {
        if scene.pick_id(id.value()) != Some(id) {
            return Err("Pick a visible unlocked original");
        }
        Ok(Self {
            id,
            source: original(scene, id).ok_or("Reload an editable original source")?,
            model: scene.placement(id).ok_or("Source placement is missing")?,
        })
    }
    pub fn valid(&self, scene: &Scene) -> Result<(), &'static str> {
        if scene.pick_id(self.id.value()) != Some(self.id)
            || scene.placement(self.id).is_none_or(|p| p.m != self.model.m)
            || original(scene, self.id).is_none_or(|g| !same(&self.source, &g))
        {
            return Err("Original source changed; start again");
        }
        Ok(())
    }
}
```

### 4. `src/gpu_geometry.rs`

Skip empty triangle draws before binding a wire-only box index buffer.

**Locate this existing block:**

```rust
    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>,count:u32) {
```

After that block, type:

```rust
--8<-- "journey/code/84b-adopt-original-boxes-and-retain-precise-placed-bounds-wire-draw.rs" skip=78
```

### 5. `src/highlight.rs`

Show whole selection on wire-only original edges without creating selected proxy faces.

**Locate this existing block:**

```rust
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }))?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
```

## <a id="checkpoint-84c-save-and-reopen-the-original-box-frame"></a>4. Save and reopen the box frame

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/geometry_snapshot.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original box owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked source import</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original frame</text>
</svg>
</div>

**What this code does:** Save and reopen the original box frame.

### 1. `src/geometry_snapshot.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
    }
    for row in scene.objects().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {
        if let Some(cad)=row.cad.as_ref() {push!(breps,row.id,cad.source.to_proto());}
        else if let Some(surface)=row.surface.as_ref() {push!(nurbssurfaces,row.id,surface.source.to_proto());}
        else {push!(meshes,row.id,row.geometry().ok_or("Reload editable sources before saving")?.to_proto());}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84c-save-and-reopen-the-original-box-frame-01.rs"
```

### 2. `src/document_source.rs`

Retain validated original OBB records through mixed document persistence.

Replace the entire file with:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {return Err("Interactions arrive with element features".into());}
    let objects=source.objects.as_ref().ok_or("Session has no objects")?;
    if !objects.planes.is_empty()||!objects.elements.is_empty()||!objects.components.is_empty() {return Err("This subject cannot open planes, boxes or assemblies yet".into());}
    Ok(())
}
```

### 3. `src/document_geometry.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.lines {add!(source,scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).map_err(str::to_owned)?));}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84c-save-and-reopen-the-original-box-frame-04.rs"
```

### 4. `src/archive_identity.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
pub(crate) fn objects(objects:&mut proto::Objects,names:&HashMap<String,String>) {
    macro_rules! remap {($field:ident)=>{for object in &mut objects.$field {if let Some(guid)=names.get(&object.guid) {object.guid=guid.clone();}}};}
    remap!(meshes);remap!(lines);remap!(points);remap!(polylines);remap!(nurbscurves);remap!(nurbssurfaces);remap!(breps);remap!(pointclouds);remap!(instances);remap!(sheets);
}
pub(crate) fn tree(node:&mut proto::TreeNode,names:&HashMap<String,String>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84c-save-and-reopen-the-original-box-frame-05.rs"
```

### 5. `src/source_validate.rs`

Retain validated original OBB records through mixed document persistence.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return Err(TOO_MANY.into());
        }

        for item in &objects.points {
            point(item)?;
        }
```

Replace it with:

```rust
            return Err(TOO_MANY.into());
        }

        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
        }
```

**Locate this block:**

```rust
/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
```

Replace it with:

```rust
/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.bboxes.len()+objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
```

## <a id="checkpoint-84d-own-temporary-measurement-labels-without-scene-identities"></a>5. Own temporary measurement labels without scene identities

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">numeric value and points</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unused label IDs</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary presentation wo…</text>
</svg>
</div>

**What this code does:** Own temporary measurement labels without scene identities.

### 1. `src/measure_mark.rs`

Own temporary points, unit suffixes and collision-free labels.

Create the file and type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-01.rs"
```

### 2. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust

pub mod bounds_original;
```

After that block, type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-02.rs" skip=26
```

## <a id="checkpoint-84e-report-distance-and-world-bounding-lengths"></a>6. Report distance and world bounding lengths

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected bounds or two po…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">world extents and deltas</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">formatted result</text>
</svg>
</div>

**What this code does:** Report distance and world bounding lengths.

### 1. `src/measure_mark.rs`

Resolve world selected bounds and distance without changing source geometry.

**Locate this existing block:**

```rust
        out
    }
}
```

After that block, type:

```rust
--8<-- "journey/code/84e-report-distance-and-world-bounding-lengths-01.rs" skip=20
```

## <a id="checkpoint-84f-collect-only-supported-finite-original-measurements"></a>7. Collect only supported finite measurements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected original owners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source copies</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">supported totals and refu…</text>
</svg>
</div>

**What this code does:** Collect only supported finite original measurements.

### 1. `src/measure_mark.rs`

Collect original length, area and volume while keeping refusals explicit.

**Locate this existing block:**

```rust
    BoundingLengths,
}
```

After that block, type:

```rust
--8<-- "journey/code/84f-collect-only-supported-finite-original-measurements-01.rs" skip=23
```

## <a id="checkpoint-84g-format-totals-refusals-and-three-axis-box-extents"></a>8. Format totals refusals and three-axis box extents

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite total and refusals</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current value formatting</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary result chip</text>
</svg>
</div>

**What this code does:** Format totals refusals and three-axis box extents.

### 1. `src/measure_mark.rs`

Report current formatted totals and world bounding lengths with bounded errors.

**Locate this existing block:**

```rust
        Ok(result)
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84g-format-totals-refusals-and-three-axis-box-extents-01.rs"
```

## <a id="checkpoint-84h-keep-measurements-outside-geometry-history"></a>9. Keep measurements outside geometry history

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected whole owner or f…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measurement or original b…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged or single histo…</text>
</svg>
</div>

**What this code does:** Keep measurements outside geometry history.

### 1. `src/editor.rs`

Own temporary marks and connect selected-original numerical results and box creation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
```

Replace it with:

```rust
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
```

**Locate this block:**

```rust
}

pub struct Editor {
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
```

Replace it with:

```rust
}

pub struct Editor {
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
```

## <a id="checkpoint-84i-accept-two-distance-points-through-the-drawing-tool"></a>10. Accept two distance points through the drawing tool

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tool.rs, src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">drawing-plane or world po…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing snapping tool</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary measured pair</text>
</svg>
</div>

**What this code does:** Accept two distance points through the drawing tool.

### 1. `src/tool.rs`

Add a two-point measurement result to the existing drawing tool.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
            (Self::Line, [a, b]) => {
                if a.iter().zip(b).all(|(a, b)| (a-b).abs() < 1e-12) { return Err("Line endpoints must differ"); }
```

Replace it with:

```rust
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Measure,[a,b])=>Ok(Built::Measured([*a,*b])),
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
            (Self::Line, [a, b]) => {
                if a.iter().zip(b).all(|(a, b)| (a-b).abs() < 1e-12) { return Err("Line endpoints must differ"); }
```

**Locate this block:**

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Project,Extend, Gather(&'static crate::gather_input::Recipe), Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape), Transform(crate::transform_draft::Kind) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Project=>"Project To Plane",Self::Extend=>"Extend",Self::Gather(recipe)=>recipe.name, Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name,Self::Transform(kind)=>kind.name() } }
    fn count(&self) -> usize { match self { Self::Project|Self::Extend=>0,Self::Gather(_)=>0, Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)|Self::Transform(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

Replace it with:

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Measure, Project,Extend, Gather(&'static crate::gather_input::Recipe), Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape), Transform(crate::transform_draft::Kind) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Measure=>"Measure Distance",Self::Project=>"Project To Plane",Self::Extend=>"Extend",Self::Gather(recipe)=>recipe.name, Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name,Self::Transform(kind)=>kind.name() } }
    fn count(&self) -> usize { match self { Self::Project|Self::Extend=>0,Self::Gather(_)=>0, Self::Point => 1, Self::Line | Self::Move | Self::Measure => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)|Self::Transform(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

**Locate this block:**

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]), Polyline(crate::chain::PreparedChain), Geometry(session_rust::Geometry) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
```

Replace it with:

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Measured([[f64;3];2]), Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]), Polyline(crate::chain::PreparedChain), Geometry(session_rust::Geometry) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
```

### 2. `src/editor.rs`

Complete accepted distance points without opening scene history and expose measurement actions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
```

Replace it with:

```rust
        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
```

**Locate this block:**

```rust
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
```

Replace it with:

```rust
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
```

**Locate this block:**

```rust
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
```

Replace it with:

```rust
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Measured(_)=>unreachable!("Measurement changes no original geometry"),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
```

**Locate this block:**

```rust
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
```

Replace it with:

```rust
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        if let crate::tool::Built::Measured(points)=built{return self.distance_mark(points)}
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
```

**Locate this block:**

```rust

pub enum Action {
    StreamRefresh,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
```

Replace it with:

```rust

pub enum Action {
    StreamRefresh,
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
```

## <a id="checkpoint-84j-register-measurements-and-end-temporary-marks-predictably"></a>11. Register measurements and end temporary marks predictably

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/commands…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current command vocabulary</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked measurement action</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit mark lifecycle</text>
</svg>
</div>

**What this code does:** Register measurements and end temporary marks predictably.

### 1. `src/editor.rs`

Connect transient mark cleanup and completed-mark Escape.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
```

Replace it with:

```rust
    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Isometric|Action::Background|Action::NormalView(_)|Action::Grid(_)){self.mark=None;}
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
```

**Locate this block:**

```rust
    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
```

Replace it with:

```rust
    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
```

### 2. `src/commands.rs`

Register original BoundingBox and current numerical measurement commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name=="Split"{return if words.len()==count{Ok(Command::Editor(Action::Split))}else{Err("Split takes no arguments".into())};}
    if let Some(action)=crate::editing_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
```

Replace it with:

```rust
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name=="Measure Distance"{if words.len()>count+2{return Err("Measure Distance takes two points".into())}return Ok(Command::Editor(Action::MeasureDistance(words[count..].join(" "))))}
    if name=="Split"{return if words.len()==count{Ok(Command::Editor(Action::Split))}else{Err("Split takes no arguments".into())};}
    if let Some(action)=crate::editing_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
```

**Locate this block:**

```rust
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Curve",||Action::BeginTool(crate::tool::Shape::Curve)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
```

Replace it with:

```rust
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("BoundingBox",||Action::BoundingBox),
    ("Length BoundingBox",||Action::Measure(crate::measure_mark::Kind::BoundingLengths)),
    ("Length",||Action::Measure(crate::measure_mark::Kind::Length)),
    ("Area",||Action::Measure(crate::measure_mark::Kind::Area)),
    ("Volume",||Action::Measure(crate::measure_mark::Kind::Volume)),
    ("Measure Distance",||Action::MeasureDistance(String::new())),
    ("Curve",||Action::BeginTool(crate::tool::Shape::Curve)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
```

## <a id="checkpoint-84k-draw-cached-unpickable-measurement-strokes"></a>12. Draw cached unpickable measurement strokes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/meas…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary point pair</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached CSS-width stroke</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">overlay colour pass</text>
</svg>
</div>

**What this code does:** Draw cached unpickable measurement strokes.

### 1. `src/stroke_gpu.rs`

Add a colour-only overlay lane with explicit depth behaviour.

**Locate this existing block:**

```rust
    }

    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_depth(device, format, crate::stroke::SHADER, 44, 6,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84k-draw-cached-unpickable-measurement-strokes-01.rs"
```

### 2. `src/measure_gpu.rs`

Cache unchanged marks and account for their actual stroke allocation.

Create the file and type:

```rust
--8<-- "journey/code/84k-draw-cached-unpickable-measurement-strokes-02.rs"
```

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust

pub mod measure_mark;
```

After that block, type:

```rust
--8<-- "journey/code/84k-draw-cached-unpickable-measurement-strokes-03.rs" skip=23
```

## <a id="checkpoint-84l-merge-measurement-chips-with-cached-scene-text"></a>13. Merge measurement chips with cached scene text

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged mark guard</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stroke overlay</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">value chip</text>
</svg>
</div>

**What this code does:** Merge measurement chips with cached scene text.

### 1. `src/renderer.rs`

Draw cached measurement strokes and merge temporary chips with the existing text owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
```

Replace it with:

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);}
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
```

**Locate this block:**

```rust
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
```

Replace it with:

```rust
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
```

**Locate this block:**

```rust
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(merged)?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(merged)?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
```

Replace it with:

```rust
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
```

**Locate this block:**

```rust
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }
```

Replace it with:

```rust
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }
```

**Locate this block:**

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let mut renderer = Self {sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

Replace it with:

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);
        let mut renderer = Self {measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

**Locate this block:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
```

Replace it with:

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
```

## <a id="checkpoint-84m-exercise-original-boxes-and-temporary-distance-annotations"></a>14. Exercise boxes and temporary distance annotations

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs, src/measure…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original solid</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reversible wire box</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary distance line a…</text>
</svg>
</div>

**What this code does:** Exercise original boxes and temporary distance annotations.

### 1. `src/browser.rs`

Route current measurement input, physical Escape and truthful annotation diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
```

Replace it with:

```rust
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-measurement",&editor.mark.as_ref().map_or_else(||"null".into(),|m|serde_json::json!({"points":m.points,"label":m.label}).to_string()))?;
    canvas.set_attribute("data-measurement-gpu-bytes",&renderer.measurement_bytes().to_string())?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
```

**Locate this block:**

```rust
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
```

Replace it with:

```rust
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
```

**Locate this block:**

```rust
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
```

Replace it with:

```rust
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
```

**Locate this block:**

```rust
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
```

Replace it with:

```rust
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            editor.mark=None;
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
```

**Locate this block:**

```rust

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };
```

Replace it with:

```rust

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.mark.is_some() || editor.split.is_some() || editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/measure_tests.rs`

Copy original box, exact archive, numerical, selected-face, history, distance and label-collision fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/84m-exercise-original-boxes-and-temporary-distance-annotations-06.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```text

pub mod measure_gpu;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/84m-exercise-original-boxes-and-temporary-distance-annotations-07.rs" skip=22
```

## <a id="checkpoint-84zza-describe-removable-cad-parts"></a>15. Describe removable CAD parts

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs, src/lib.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Represent curve intervals, BRep face groups and mesh sides without replacing the original CAD geometry.

### 1. `src/trim_parts.rs`

Type the original CAD part representation and seam-aware curve helpers.

Create the file and type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-1.rs"
```

### 2. `src/lib.rs`

Register the shared part owner.

**Locate this existing block:**

```rust
#[cfg(test)]
mod measure_tests;
```

After that block, type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-2.rs" skip=32
```

## <a id="checkpoint-84zzb-read-trim-part-previews"></a>16. Read trim part previews

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Expose part counts, sampled curve outlines, section edges and meshes through one Parts owner.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
/// One cutter and, for a straight line, the plane it stands for off the object.
pub struct Blade {
    pub cutter: Cutter,       // in the target's frame
    pub fence: Option<Plane>, // a line's vertical plane, in the target's frame
}
```

After that block, type:

```rust
impl Parts {
    /// How many parts there are.
    pub fn count(&self) -> usize {
        match self {
            Parts::Curve { curve, cuts } if curve.is_closed() => cuts.len(),
            Parts::Curve { cuts, .. } => cuts.len() + 1,
            Parts::Faces { groups, .. } => groups.len(),
            Parts::Mesh { sides, .. } => sides.len(),
        }
    }

    /// Part `index` of a curve as points, for drawing and clicking.
    pub fn outline(&self, index: usize) -> Vec<Point> {
        let Parts::Curve { curve, cuts } = self else {
            return Vec::new();
        };
        let (a, b) = interval(curve, cuts, index);
        piece(curve, a, b).map_or_else(Vec::new, |part| samples(&part, 48))
    }

    /// The points where a curve was cut.
    pub fn cut_points(&self) -> Vec<Point> {
        match self {
            Parts::Curve { curve, cuts } => cuts.iter().map(|t| curve.point_at(*t)).collect(),
            _ => Vec::new(),
        }
    }

    /// The cut edges as polylines.
    pub fn section(&self) -> Vec<Vec<Point>> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { section, .. } => section.clone(),
            Parts::Mesh { section, .. } => section.iter().map(|pair| pair.to_vec()).collect(),
        }
    }

    /// The meshes a click can hit, with the part each belongs to.
    pub fn hit_meshes(&self) -> Vec<(usize, &Mesh)> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { groups, meshes, .. } => groups
                .iter()
                .enumerate()
                .flat_map(|(part, faces)| {
                    faces
                        .iter()
                        .filter_map(move |f| Some((part, meshes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}
```

## <a id="checkpoint-84zzc-collect-curve-crossings"></a>17. Collect curve crossings

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Collect kernel curve intersections, filter open endpoints and merge a closed seam.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 { d0 } else { cuts[index - 1] };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near).then(|| trimmed(a, d1)).flatten();
    let head = (b > d0 + near).then(|| trimmed(d0, b)).flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzc-collect-curve-crossings-1.rs"
```

## <a id="checkpoint-84zzd-preserve-trimmed-source-types"></a>18. Preserve trimmed source types

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Rebuild kept curves in their original source family with their presentation.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}
```

After that block, type:

```rust
/// The run `a..b` of `curve` as the source's own type, with its name and pen.
fn rebuilt(source: &Geometry, curve: &NurbsCurve, a: f64, b: f64) -> Option<Geometry> {
    let part = piece(curve, a, b)?;

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&part.point_at_start(), &part.point_at_end());
            next.name = line.name.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.linecolor = line.linecolor.clone();
            Some(Geometry::Line(Rc::new(next)))
        }
        Geometry::Polyline(polyline) => {
            let mut next = Polyline::new(samples(&part, 1));
            next.name = polyline.name.clone();
            next.width = polyline.width;
            next.dash = polyline.dash.clone();
            next.linecolor = polyline.linecolor.clone();
            Some(Geometry::Polyline(Rc::new(next)))
        }
        Geometry::NurbsCurve(original) => {
            let mut next = part;
            next.refresh_guid();
            next.name = original.name.clone();
            next.width = original.width;

            // per-control colours only fit the control count they were made for
            if next.pointcolors.len() != next.m_cv_count {
                next.pointcolors.clear();
            }

            if next.linecolors.len() + 1 != next.m_cv_count {
                next.linecolors.clear();
            }

            Some(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => None,
    }
}

/// A surface as a one-face BRep, cut by nothing yet.
fn face_of(surface: &NurbsSurface) -> Result<BRep, String> {
    let corner = surface
        .get_cv(0, 0)
        .ok_or("Surface has no control points")?;
    let size = planar_size(surface);
    // a cutter far off the surface cuts nothing, so the kernel only wraps it
    let far = &corner + &Vector::new(size * 10.0, size * 10.0, size * 10.0);
    let away = NurbsCurve::create(
        false,
        1,
        &[far.clone(), &far + &Vector::new(size, 0.0, 0.0)],
    );
    let mut brep = split::split_surface_by_curves(surface, &[away], TOLERANCE)?;
    brep.name = surface.name.clone();
    Ok(brep)
}

/// The largest control point coordinate of a surface, at least 1.
fn planar_size(surface: &NurbsSurface) -> f64 {
    let mut size: f64 = 1.0;

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}
```

## <a id="checkpoint-84zze-group-faces-by-cutting-planes"></a>19. Group faces by cutting planes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Extend a plane-plane intersection past surface control points.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}
```

After that block, type:

```rust
/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
fn sides(
    brep: &BRep,
    meshes: &[Mesh],
    planes: &[Plane],
    regions: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut groups: Vec<(Vec<bool>, Option<usize>, Vec<usize>)> = Vec::new();

    for face in 0..brep.face_count() {
        let center = meshes
            .get(face)
            .map_or_else(|| brep.point_at(face, 0.5, 0.5), |mesh| mesh.centroid());
        let key: Vec<bool> = planes
            .iter()
            .map(|plane| (&center - &plane.origin()).dot(&plane.z_axis()) >= 0.0)
            .collect();
        let region = regions
            .iter()
            .any(|faces| faces.contains(&face))
            .then_some(face);

        match groups
            .iter_mut()
            .find(|(signs, own, _)| *signs == key && *own == region && region.is_none())
        {
            Some((_, _, faces)) => faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups.into_iter().map(|(_, _, faces)| faces).collect()
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
fn across(line: &Line, surface: &NurbsSurface) -> NurbsCurve {
    let start = line.start();
    let mut direction = line.to_vector();
    direction.normalize_self();
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                let t = (&p - &start).dot(&direction);
                low = low.min(t);
                high = high.max(t);
            }
        }
    }

    let margin = (high - low).max(1.0) * 0.1;
    NurbsCurve::create(
        false,
        1,
        &[
            &start + &(&direction * (low - margin)),
            &start + &(&direction * (high + margin)),
        ],
    )
}
```

## <a id="checkpoint-84zzf-cut-planar-brep-faces"></a>20. Cut planar BRep faces

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Find crossed planar faces, split them through the kernel and collect section edges.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
            Some((_, _, faces)) => faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups.into_iter().map(|(_, _, faces)| faces).collect()
}
```

After that block, type:

```rust
/// Split every face `plane` passes through; returns the new edges on the plane.
fn cut_by_plane(brep: &mut BRep, plane: &Plane) -> Result<Vec<Vec<Point>>, String> {
    let normal = plane.z_axis();
    let origin = plane.origin();
    let side = |p: &Point| (p - &origin).dot(&normal);
    let size = brep
        .m_vertices
        .iter()
        .map(|v| v.point[0].abs().max(v.point[1].abs()).max(v.point[2].abs()))
        .fold(1.0, f64::max);
    let near = TOLERANCE * size;
    let mut crossed = false;

    for face in 0..brep.face_count() {
        let surface = &brep.m_surfaces[brep.m_faces[face].surface_index as usize];
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

        for i in 0..surface.m_cv_count[0] {
            for j in 0..surface.m_cv_count[1] {
                let d = surface.get_cv(i, j).map_or(0.0, |p| side(&p));
                low = low.min(d);
                high = high.max(d);
            }
        }

        // all on one side: nothing to cut
        if low > -near || high < near {
            continue;
        }

        // curved sections are left out: the kernel's surface-plane tracer costs 15 KB of wasm
        let own = planar(surface).ok_or(
            "A plane cuts flat faces only; cut a curved face with a curve that lies on it",
        )?;
        let Some(line) = intersection::plane_plane(&own, plane) else {
            continue;
        };
        let before = brep.face_count();
        let next = split::split_brep_face_by_curves(
            brep,
            face,
            &[across(&line, surface)],
            TOLERANCE,
        )?;
        crossed |= next.face_count() > before;
        *brep = next;
    }

    if !crossed {
        return Err("not crossed".into());
    }

    // edges lying in the plane are the cut
    let mut section = Vec::new();

    for edge in &brep.m_edges {
        let Some(curve) = brep
            .m_curves_3d
            .get(edge.curve_3d_index.max(0) as usize)
            .filter(|_| edge.curve_3d_index >= 0)
        else {
            continue;
        };
        let points = samples(curve, 8);

        if points.iter().all(|p| side(p).abs() <= near * 10.0) {
            section.push(points);
        }
    }

    Ok(section)
}
```

## <a id="checkpoint-84zzg-assemble-removable-face-groups"></a>21. Assemble removable face groups

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Combine curve regions, fallback fence planes and crossed plane sections into one Parts value.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}
```

After that block, type:

```rust
/// Split the faces of `brep` by the blades: curves on its faces make regions, planes make sides.
fn faces(brep: &BRep, blades: &[Blade]) -> Result<Parts, String> {
    let mut brep = brep.clone();
    let mut regions: Vec<Vec<usize>> = Vec::new(); // faces a curve split, one region each
    let mut planes: Vec<Plane> = Vec::new();
    let mut section: Vec<Vec<Point>> = Vec::new();
    let mut refused = None;

    for blade in blades {
        let curve = match &blade.cutter {
            Cutter::Plane(plane) => {
                planes.push(plane.clone());
                continue;
            }
            Cutter::Curve(curve) => curve,
        };
        let mut hit = false;

        for face in 0..brep.face_count() {
            let before = brep.face_count();

            match split::split_brep_face_by_curves(
                &brep,
                face,
                std::slice::from_ref(curve),
                TOLERANCE,
            ) {
                Ok(next) if next.face_count() > before => {
                    regions.push(
                        [face]
                            .into_iter()
                            .chain(before..next.face_count())
                            .collect(),
                    );
                    brep = next;
                    hit = true;
                }
                Ok(_) => {}
                Err(error) => refused = Some(error),
            }
        }

        match (&blade.fence, hit) {
            (_, true) => section.push(samples(curve, 48)),
            (Some(fence), false) => planes.push(fence.clone()),
            (None, false) => {}
        }
    }

    // a plane that misses this object is some other target's
    let mut crossing = Vec::new();

    for plane in planes {
        match cut_by_plane(&mut brep, &plane) {
            Ok(split) => {
                section.extend(split);
                crossing.push(plane);
            }
            Err(error) if error == "not crossed" => {}
            Err(error) => return Err(error),
        }
    }

    let planes = crossing;

    if regions.is_empty() && section.is_empty() {
        return Err(refused.unwrap_or_else(|| {
            "Curves cut a BRep only where they lie on its faces; use a line, a Plane or a planar surface to cut through it".into()
        }));
    }

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    let groups = if planes.is_empty() {
        // each region of a split face is a part; unsplit faces are no part
        regions
            .into_iter()
            .flatten()
            .map(|face| vec![face])
            .collect()
    } else {
        sides(&brep, &meshes, &planes, &regions)
    };

    if groups.len() < 2 {
        return Err("not crossed".into());
    }

    Ok(Parts::Faces {
        brep,
        groups,
        meshes,
        section,
    })
}
```

## <a id="checkpoint-84zzh-mark-retained-brep-topology"></a>22. Mark retained BRep topology

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Walk retained faces through wires and edges to mark their surfaces, vertices and curve tables.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
        .map(|&kept| {
            if !kept {
                return -1;
            }

            next += 1;
            next - 1
        })
        .collect()
}
```

After that block, type:

```rust
/// `brep` without `removed` faces; every table is compacted and a solid left open is dropped.
pub fn without_faces(brep: &BRep, removed: &[usize]) -> Result<BRep, String> {
    if removed.is_empty() {
        return Ok(brep.clone());
    }

    let keep_face: Vec<bool> = (0..brep.m_faces.len())
        .map(|face| !removed.contains(&face))
        .collect();

    if !keep_face.contains(&true) {
        return Err("The last part stays; use Delete".into());
    }

    let mut keep_wire = vec![false; brep.m_wires.len()];
    let mut keep_surface = vec![false; brep.m_surfaces.len()];

    for (face, kept) in brep.m_faces.iter().zip(&keep_face) {
        if !kept {
            continue;
        }

        keep_surface[face.surface_index as usize] = true;

        for wire in &face.wires {
            keep_wire[wire.index as usize] = true;
        }
    }

    let mut keep_edge = vec![false; brep.m_edges.len()];

    for (wire, kept) in brep.m_wires.iter().zip(&keep_wire) {
        if !kept {
            continue;
        }

        for edge in &wire.edges {
            keep_edge[edge.index as usize] = true;
        }
    }

    let mut keep_vertex = vec![false; brep.m_vertices.len()];
    let mut keep_curve = vec![false; brep.m_curves_3d.len()];
    let mut keep_pcurve = vec![false; brep.m_curves_2d.len()];

    for (edge, kept) in brep.m_edges.iter().zip(&keep_edge) {
        if !kept {
            continue;
        }

        for vertex in [edge.start_vertex, edge.end_vertex] {
            if vertex >= 0 {
                keep_vertex[vertex as usize] = true;
            }
        }

        if edge.curve_3d_index >= 0 {
            keep_curve[edge.curve_3d_index as usize] = true;
        }

        for pcurve in edge
            .pcurves
            .iter()
            .filter(|p| keep_surface[p.surface_index as usize])
        {
            keep_pcurve[pcurve.curve_2d_index as usize] = true;

            if pcurve.curve_2d_index_2 >= 0 {
                keep_pcurve[pcurve.curve_2d_index_2 as usize] = true;
            }
        }
    }

    Err("BRep table reconstruction is not connected yet".into())
}
```

## <a id="checkpoint-84zzi-rebuild-retained-brep-tables"></a>23. Rebuild retained BRep tables

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Compact marked BRep tables and rewrite every reference.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
    Err("BRep table reconstruction is not connected yet".into())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzi-rebuild-retained-brep-tables-1.rs"
```

## <a id="checkpoint-84zzj-split-mesh-faces-at-a-plane"></a>24. Split mesh faces at a plane

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 24 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Classify mesh vertices, reuse one cut vertex per original edge and build both clipped face rings.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
fn kept_of<T: Clone>(list: &[T], used: &[bool]) -> Vec<T> {
    list.iter()
        .zip(used)
        .filter(|(_, kept)| **kept)
        .map(|(item, _)| item.clone())
        .collect()
}
```

After that block, type:

```rust
/// A mesh cut by `plane` into its two sides, sharing the cut vertices; the cut as segments.
pub fn mesh_sides(mesh: &Mesh, plane: &Plane) -> Result<(Vec<Mesh>, Vec<[Point; 2]>), String> {
    if !mesh.get_face_holes().is_empty() {
        return Err("Trim cannot cut a mesh with holed faces".into());
    }

    let (mut points, faces) = mesh.to_vertices_and_faces();
    let normal = plane.z_axis();
    let origin = plane.origin();
    let size = points
        .iter()
        .map(|p| p[0].abs().max(p[1].abs()).max(p[2].abs()))
        .fold(1.0, f64::max);
    let near = 1e-9 * size;
    let side: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = (p - &origin).dot(&normal);
            if d.abs() <= near { 0.0 } else { d }
        })
        .collect();

    if side.iter().all(|d| *d >= 0.0) || side.iter().all(|d| *d <= 0.0) {
        return Err("not crossed".into());
    }

    let mut crossings: HashMap<(usize, usize), usize> = HashMap::new(); // edge to its cut vertex
    let mut halves: [Vec<Vec<usize>>; 2] = [Vec::new(), Vec::new()];
    let mut section = Vec::new();

    for face in &faces {
        if face.iter().all(|v| side[*v] >= 0.0) {
            halves[0].push(face.clone());
            continue;
        }

        if face.iter().all(|v| side[*v] <= 0.0) {
            halves[1].push(face.clone());
            continue;
        }

        let mut above = Vec::new();
        let mut below = Vec::new();
        let mut on = Vec::new(); // this face's points on the plane

        for (i, &a) in face.iter().enumerate() {
            let b = face[(i + 1) % face.len()];

            if side[a] >= 0.0 {
                above.push(a);
            }

            if side[a] <= 0.0 {
                below.push(a);
            }

            if side[a] == 0.0 {
                on.push(a);
            }

            if side[a] * side[b] < 0.0 {
                let key = (a.min(b), a.max(b));
                let at = *crossings.entry(key).or_insert_with(|| {
                    let t = side[a] / (side[a] - side[b]);
                    let (p, q) = (&points[a], &points[b]);
                    points.push(Point::new(
                        p[0] + t * (q[0] - p[0]),
                        p[1] + t * (q[1] - p[1]),
                        p[2] + t * (q[2] - p[2]),
                    ));
                    points.len() - 1
                });
                above.push(at);
                below.push(at);
                on.push(at);
            }
        }

        if let [a, b] = on.as_slice() {
            section.push([points[*a].clone(), points[*b].clone()]);
        }

        for (half, ring) in [(0, above), (1, below)] {
            if ring.len() >= 3 {
                halves[half].push(ring);
            }
        }
    }

    Err("Mesh table reconstruction is not connected yet".into())
}
```

## <a id="checkpoint-84zzk-compact-both-trimmed-mesh-sides"></a>25. Compact both trimmed mesh sides

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 25 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Compact each clipped mesh side to only its used vertices, rewrite its faces and preserve source name and color.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
    Err("Mesh table reconstruction is not connected yet".into())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzk-compact-both-trimmed-mesh-sides-1.rs"
```

## <a id="checkpoint-84zzl-rebuild-kept-cad-parts"></a>26. Rebuild kept CAD parts

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 26 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Merge adjacent kept curve intervals, remove BRep face groups and return the retained mesh half.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}
```

After that block, type:

```rust
/// What is left of `source` once `removed` parts are gone: one geometry per unbroken run.
pub fn kept(source: &Geometry, parts: &Parts, removed: &[usize]) -> Result<Vec<Geometry>, String> {
    let count = parts.count();

    if removed.len() >= count {
        return Err("The last part stays; use Delete".into());
    }

    match parts {
        Parts::Curve { curve, cuts } => {
            let closed = curve.is_closed();
            // start after a removed part, so a run never breaks at a closed curve's seam
            let first = if closed {
                (0..count)
                    .find(|i| removed.contains(i))
                    .map_or(0, |i| (i + 1) % count)
            } else {
                0
            };
            let mut runs: Vec<(f64, f64)> = Vec::new();
            let mut open = false; // the last run can still grow

            for step in 0..count {
                let index = (first + step) % count;

                if removed.contains(&index) {
                    open = false;
                    continue;
                }

                let (a, b) = interval(curve, cuts, index);

                match runs.last_mut() {
                    Some(run) if open => run.1 = b,
                    _ => runs.push((a, b)),
                }

                open = true;
            }

            runs.iter()
                .map(|(a, b)| {
                    rebuilt(source, curve, *a, *b)
                        .ok_or_else(|| "Cannot cut this curve".to_string())
                })
                .collect()
        }
        Parts::Faces { brep, groups, .. } => {
            let faces: Vec<usize> = removed
                .iter()
                .flat_map(|part| groups[*part].iter().copied())
                .collect();
            Ok(vec![Geometry::BRep(Rc::new(without_faces(brep, &faces)?))])
        }
        Parts::Mesh { sides, .. } => {
            let side = (0..sides.len())
                .find(|i| !removed.contains(i))
                .ok_or("The last part stays")?;
            Ok(vec![Geometry::Mesh(Rc::new(sides[side].clone()))])
        }
    }
}
```

## <a id="checkpoint-84zzm-connect-original-trim-recipes"></a>27. Connect trim recipes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 27 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained source geometry</text>
</svg>
</div>

**What this code does:** Route original curves, surfaces, BReps and meshes through the shared part builders.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
                        .filter_map(move |f| Some((part, meshes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}
```

After that block, type:

```rust
/// Cut `target` (in its own frame) by `blades`; an error when nothing is crossed.
pub fn cut(target: &Geometry, blades: &[Blade]) -> Result<Parts, String> {
    match target {
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => {
            let curve = crate::source_cut::as_curve(target)
                .ok_or("This curve has no points")?;
            let cuts = curve_cuts(&curve, blades)?;
            let parts = Parts::Curve { curve, cuts };

            if parts.count() < 2 {
                return Err("not crossed".into());
            }

            Ok(parts)
        }
        Geometry::BRep(brep) => faces(brep, blades),
        Geometry::NurbsSurface(surface) => {
            let brep = face_of(surface)?;
            faces(&brep, blades)
        }
        Geometry::Mesh(mesh) => {
            let mut planes = blades
                .iter()
                .filter_map(|blade| match &blade.cutter {
                    Cutter::Plane(plane) => Some(plane),
                    Cutter::Curve(_) => blade.fence.as_ref(),
                })
                .peekable();

            if planes.peek().is_none() {
                return Err(
                    "A mesh is cut by a plane: a Plane, a planar surface or a straight line".into(),
                );
            }

            // the first plane that passes through it
            let mut answer = Err("not crossed".to_string());

            for plane in planes {
                answer = mesh_sides(mesh, plane);

                if answer.as_ref().is_ok()
                    || answer.as_ref().is_err_and(|error| error != "not crossed")
                {
                    break;
                }
            }

            let (sides, section) = answer?;
            Ok(Parts::Mesh { sides, section })
        }
        _ => Err("Trim cuts lines, polylines, curves, surfaces, BReps and meshes".into()),
    }
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/trim_parts.rs`

Supply the original trim validation fixtures.

**Locate this existing block:**

```text
/// A mesh cut by `plane` into its two sides, sharing the cut vertices; the cut as segments.
pub fn mesh_sides(mesh: &Mesh, plane: &Plane) -> Result<(Vec<Mesh>, Vec<[Point; 2]>), String> {
    if !mesh.get_face_holes().is_empty() {
        return Err("Trim cannot cut a mesh with holed faces".into());
    }

    let (mut points, faces) = mesh.to_vertices_and_faces();
    let normal = plane.z_axis();
    let origin = plane.origin();
    let size = points
        .iter()
        .map(|p| p[0].abs().max(p[1].abs()).max(p[2].abs()))
        .fold(1.0, f64::max);
    let near = 1e-9 * size;
    let side: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = (p - &origin).dot(&normal);
            if d.abs() <= near { 0.0 } else { d }
        })
        .collect();

    if side.iter().all(|d| *d >= 0.0) || side.iter().all(|d| *d <= 0.0) {
        return Err("not crossed".into());
    }

    let mut crossings: HashMap<(usize, usize), usize> = HashMap::new(); // edge to its cut vertex
    let mut halves: [Vec<Vec<usize>>; 2] = [Vec::new(), Vec::new()];
    let mut section = Vec::new();

    for face in &faces {
        if face.iter().all(|v| side[*v] >= 0.0) {
            halves[0].push(face.clone());
            continue;
        }

        if face.iter().all(|v| side[*v] <= 0.0) {
            halves[1].push(face.clone());
            continue;
        }

        let mut above = Vec::new();
        let mut below = Vec::new();
        let mut on = Vec::new(); // this face's points on the plane

        for (i, &a) in face.iter().enumerate() {
            let b = face[(i + 1) % face.len()];

            if side[a] >= 0.0 {
                above.push(a);
            }

            if side[a] <= 0.0 {
                below.push(a);
            }

            if side[a] == 0.0 {
                on.push(a);
            }

            if side[a] * side[b] < 0.0 {
                let key = (a.min(b), a.max(b));
                let at = *crossings.entry(key).or_insert_with(|| {
                    let t = side[a] / (side[a] - side[b]);
                    let (p, q) = (&points[a], &points[b]);
                    points.push(Point::new(
                        p[0] + t * (q[0] - p[0]),
                        p[1] + t * (q[1] - p[1]),
                        p[2] + t * (q[2] - p[2]),
                    ));
                    points.len() - 1
                });
                above.push(at);
                below.push(at);
                on.push(at);
            }
        }

        if let [a, b] = on.as_slice() {
            section.push([points[*a].clone(), points[*b].clone()]);
        }

        for (half, ring) in [(0, above), (1, below)] {
            if ring.len() >= 3 {
                halves[half].push(ring);
            }
        }
    }

    let sides = halves
        .into_iter()
        .map(|faces| {
            // only the vertices this side uses
            let mut index = vec![usize::MAX; points.len()];
            let mut used = Vec::new();

            for face in &faces {
                for &v in face {
                    if index[v] == usize::MAX {
                        index[v] = used.len();
                        used.push(points[v].clone());
                    }
                }
            }

            let faces = faces
                .iter()
                .map(|face| face.iter().map(|v| index[*v]).collect())
                .collect();
            let mut side = Mesh::from_vertices_and_faces(used, faces);
            side.name = mesh.name.clone();
            side.set_objectcolor(mesh.get_objectcolor().clone());
            side
        })
        .collect();
    Ok((sides, section))
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/84zzm-connect-original-trim-recipes-2.rs" skip=3698
```

## <a id="checkpoint-84zna-index-selection-loops-by-pixel-row"></a>28. Index selection loops by pixel row

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 28 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Build a row index for the even-odd point test.

### 1. `src/selection_region.rs`

Type the shared selection region and row index.

Create the file and type:

```rust
--8<-- "journey/code/84zna-index-selection-loops-by-pixel-row-1.rs"
```

### 2. `src/lib.rs`

Register the selection owner.

**Locate this existing block:**

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;
```

After that block, type:

```rust

pub mod selection_region;
```

## <a id="checkpoint-84znb-test-window-membership-and-segment-crossings"></a>29. Test window membership and segment crossings

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 29 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Complete Region with point membership and segment clipping against the rectangle.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
        Self {
            box_: Some([left, top, right, bottom]),
            ..Self::new(&[(left, top), (right, top), (right, bottom), (left, bottom)])
        }
    }
```

After that block, type:

```rust
    /// True when the segment from `a` to `b` meets the rectangle; false for a loop that is no rectangle.
    fn crosses(&self, a: (f64, f64), b: (f64, f64)) -> bool {
        let Some([left, top, right, bottom]) = self.box_ else {
            return false;
        };
        // clip the segment's parameter range to each side in turn (Liang-Barsky)
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let (mut t0, mut t1) = (0.0_f64, 1.0_f64);

        for (p, q) in [
            (-dx, a.0 - left),
            (dx, right - a.0),
            (-dy, a.1 - top),
            (dy, bottom - a.1),
        ] {
            if p == 0.0 {
                if q < 0.0 {
                    return false;
                }
                continue;
            }

            let t = q / p;

            if p < 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }

        t0 <= t1
    }

    /// True when `at` is inside by the even-odd rule, judged on its row's centre line.
    pub(crate) fn contains(&self, at: (f64, f64)) -> bool {
        let row = (at.1 - self.top).floor();

        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
```

## <a id="checkpoint-84znc-sample-original-cad-families-for-selection"></a>30. Sample CAD families for selection

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 30 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Reuse original curve corners, curved domains, mesh vertices and BRep references as bounded selection samples.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
}
```

After that block, type:

```rust
/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
fn every(
    samples: impl Iterator<Item = [f64; 3]>,
    inside: &dyn Fn([f64; 3]) -> bool,
) -> Option<bool> {
    let mut any = false;

    for sample in samples {
        if !inside(sample) {
            return Some(false);
        }

        any = true;
    }

    any.then_some(true)
}

/// A point as three numbers.
fn xyz(point: &Point) -> [f64; 3] {
    [point[0], point[1], point[2]]
}

/// A straight curve's corners, else `steps` even steps over its domain.
pub(crate) fn curve_samples(
    curve: &NurbsCurve,
    steps: usize,
) -> impl Iterator<Item = [f64; 3]> + '_ {
    let straight = curve.degree() == 1;
    let (t0, t1) = curve.domain();
    let corners = (0..if straight { curve.cv_count() } else { 0 }).filter_map(|i| curve.get_cv(i));
    let even = (0..if straight { 0 } else { steps + 1 })
        .map(move |i| curve.point_at(t0 + (t1 - t0) * i as f64 / steps as f64));
    corners.chain(even).map(|p| xyz(&p))
}

/// A grid of `steps` by `steps` even steps over the surface's domain.
fn surface_samples(surface: &NurbsSurface, steps: usize) -> impl Iterator<Item = [f64; 3]> + '_ {
    let domains = surface.domain(0).zip(surface.domain(1));
    let ((u0, u1), (v0, v1)) = domains.unwrap_or_default();
    let side = if domains.is_some() { steps + 1 } else { 0 };
    (0..side * side)
        .filter_map(move |k| {
            let u = u0 + (u1 - u0) * (k / side) as f64 / steps as f64;
            let v = v0 + (v1 - v0) * (k % side) as f64 / steps as f64;
            surface.point_at(u, v)
        })
        .map(|p| xyz(&p))
}

/// Every vertex of a mesh.
fn mesh_samples(mesh: &Mesh) -> impl Iterator<Item = [f64; 3]> + '_ {
    mesh.vertex.values().map(|v| [v.x, v.y, v.z])
}

/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep.m_vertices.iter().map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
```

## <a id="checkpoint-84znd-collect-original-boundary-strokes"></a>31. Collect boundary strokes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 31 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Build surface grids and closed mesh outlines, then combine BRep vertices, edges and curved faces.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
}
```

After that block, type:

```rust
/// The surface's grid lines both ways, `steps` steps each.
fn surface_strokes(surface: &NurbsSurface, steps: usize) -> Vec<Vec<[f64; 3]>> {
    let Some(((u0, u1), (v0, v1))) = surface.domain(0).zip(surface.domain(1)) else {
        return Vec::new();
    };
    let at = |i: usize, j: usize| {
        let u = u0 + (u1 - u0) * i as f64 / steps as f64;
        let v = v0 + (v1 - v0) * j as f64 / steps as f64;
        surface.point_at(u, v).map(|p| xyz(&p))
    };
    let rows = (0..=steps).map(|i| (0..=steps).filter_map(|j| at(i, j)).collect());
    let columns = (0..=steps).map(|j| (0..=steps).filter_map(|i| at(i, j)).collect());
    rows.chain(columns).collect()
}

/// Every face of a mesh as its closed outline.
fn mesh_strokes(mesh: &Mesh) -> Vec<Vec<[f64; 3]>> {
    mesh.face
        .values()
        .map(|face| {
            let mut loop_: Vec<[f64; 3]> = face
                .iter()
                .filter_map(|key| mesh.vertex.get(key).map(|v| [v.x, v.y, v.z]))
                .collect();
            loop_.extend(loop_.first().copied());
            loop_
        })
        .collect()
}

/// Vertices, edges and the grid lines of curved faces.
fn brep_strokes(brep: &BRep) -> Vec<Vec<[f64; 3]>> {
    let vertices = brep
        .m_vertices
        .iter()
        .map(|vertex| vec![xyz(&vertex.point)]);
    let edges = brep
        .m_curves_3d
        .iter()
        .map(|curve| curve_samples(curve, EDGE_STEPS).collect());
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}
```

## <a id="checkpoint-84zne-test-crossing-selection-on-original-strokes"></a>32. Test crossing selection on strokes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 32 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Dispatch original geometry families to their shared stroke builders.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
}
```

After that block, type:

```rust
/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
    project: &dyn Fn([f64; 3]) -> Option<(f64, f64)>,
    region: &Region,
) -> Option<bool> {
    let strokes = strokes(geometry)?;
    let mut any = false;

    for stroke in &strokes {
        let points: Vec<Option<(f64, f64)>> = stroke.iter().map(|p| project(*p)).collect();

        if points.iter().flatten().any(|at| region.contains(*at)) {
            return Some(true);
        }

        for pair in points.windows(2) {
            if let [Some(a), Some(b)] = pair
                && region.crosses(*a, *b)
            {
                return Some(true);
            }
        }

        any |= !points.is_empty();
    }

    any.then_some(false)
}

/// The geometry as strokes in its own frame, each a run of points joined in order; None for a plane or a frame.
fn strokes(geometry: &Geometry) -> Option<Vec<Vec<[f64; 3]>>> {
    let strokes = match geometry {
        Geometry::Point(point) => vec![vec![xyz(point)]],
        Geometry::Line(line) => vec![vec![xyz(&line.start()), xyz(&line.end())]],
        Geometry::Polyline(polyline) => vec![
            polyline
                .coords
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect(),
        ],
        Geometry::NurbsCurve(curve) => vec![curve_samples(curve, CURVE_STEPS).collect()],
        Geometry::NurbsSurface(surface) => surface_strokes(surface, SURFACE_STEPS),
        Geometry::Mesh(mesh) => mesh_strokes(mesh),
        Geometry::BRep(brep) => brep_strokes(brep),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => mesh_strokes(mesh),
            ElementGeometry::BRep(brep) => brep_strokes(brep),
            ElementGeometry::None => return None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3).div_ceil(CLOUD_SAMPLES).max(1);
            coords
                .chunks_exact(3)
                .step_by(step)
                .map(|c| vec![[c[0], c[1], c[2]]])
                .collect()
        }
        Geometry::Plane(_) | Geometry::OBB(_) => return None,
    };

    Some(strokes)
}
```

## <a id="checkpoint-84znf-connect-window-and-bounds-selection-tests"></a>33. Connect window and bounds selection tests

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 33 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">window or crossing result</text>
</svg>
</div>

**What this code does:** Add projection, conservative screen bounds and complete-enclosure dispatch.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
```

Replace it with:

```rust
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}

/// Whether every sample of `geometry`, in its own frame, passes `inside`; None when only its box can tell.
pub(crate) fn enclosed(geometry: &Geometry, inside: &dyn Fn([f64; 3]) -> bool) -> Option<bool> {
    match geometry {
        Geometry::Point(point) => every(std::iter::once(xyz(point)), inside),
        Geometry::Line(line) => every([xyz(&line.start()), xyz(&line.end())].into_iter(), inside),
        Geometry::Polyline(polyline) => every(
            polyline.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]),
            inside,
        ),
        Geometry::NurbsCurve(curve) => every(curve_samples(curve, CURVE_STEPS), inside),
        Geometry::NurbsSurface(surface) => every(surface_samples(surface, SURFACE_STEPS), inside),
        Geometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
        Geometry::BRep(brep) => every(brep_samples(brep), inside),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
            ElementGeometry::BRep(brep) => every(brep_samples(brep), inside),
            ElementGeometry::None => None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3).div_ceil(CLOUD_SAMPLES).max(1);
            every(
                coords
                    .chunks_exact(3)
                    .step_by(step)
                    .map(|c| [c[0], c[1], c[2]]),
                inside,
            )
        }
        Geometry::Plane(_) | Geometry::OBB(_) => None,
    }
```

**Locate this block:**

```rust
/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

Replace it with:

```rust
/// Device pixels of `p` under the clip matrix `m`; None behind the eye.
fn screen(m: &[f64; 16], p: [f64; 3], size: (f64, f64)) -> Option<(f64, f64)> {
    let clip = |r: usize| m[r] * p[0] + m[r + 4] * p[1] + m[r + 8] * p[2] + m[r + 12];
    let w = clip(3);
    (w > 0.0).then(|| {
        (
            (clip(0) / w * 0.5 + 0.5) * size.0,
            (0.5 - clip(1) / w * 0.5) * size.1,
        )
    })
}

/// Whether the screen box around projected corners overlaps the region's rectangle.
fn box_touches(corners: &[Option<(f64, f64)>], region: &Region) -> bool {
    let (Some([left, top, right, bottom]), Some(first)) =
        (region.box_, corners.iter().flatten().next())
    else {
        return false;
    };
    let mut lo = *first;
    let mut hi = *first;

    for at in corners.iter().flatten() {
        lo = (lo.0.min(at.0), lo.1.min(at.1));
        hi = (hi.0.max(at.0), hi.1.max(at.1));
    }

    lo.0 <= right && hi.0 >= left && lo.1 <= bottom && hi.1 >= top
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/selection_region.rs`

Supply the original region-selection validation fixtures.

**Locate this existing block:**

```text
/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep.m_vertices.iter().map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/84znf-connect-window-and-bounds-selection-tests-2.rs" skip=559
```

## <a id="checkpoint-84zoa-classify-originals-and-isolate-one-mesh-face"></a>34. Classify originals and isolate one mesh face

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 34 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_shape.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Name each original geometry kind once.

### 1. `src/source_shape.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-1.rs"
```

### 2. `src/lib.rs`

Compile the shared original geometry classifier.

**Locate this existing block:**

```rust

pub mod selection_region;
```

After that block, type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-2.rs" skip=27
```

### 3. `src/explode_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-3.rs"
```

### 4. `src/lib.rs`

Register this original command owner.

**Locate this existing block:**

```rust

pub mod trim_parts;
```

After that block, type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-4.rs" skip=21
```

## <a id="checkpoint-84zob-copy-original-brep-edges-and-pcurves"></a>35. Copy BRep edges and pcurves

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 35 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Copy an edge, its endpoint vertices and its surface pcurves into a new BRep.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}
```

After that block, type:

```rust
/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep.m_edges.get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32,
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends.iter_mut().zip([edge.start_vertex, edge.end_vertex]) {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep.m_vertices.get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}
```

## <a id="checkpoint-84zoc-build-one-original-brep-face"></a>36. Build one BRep face

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 36 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Reuse the copied-edge helper to rebuild one face surface, ordered wire references and one oriented shell.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}
```

After that block, type:

```rust
/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(brep.m_surfaces.get(usize::try_from(surface).ok()?)?);
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep.m_wires.get(usize::try_from(wire.index).ok()?)?.edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(brep, &mut part, used.index, surface, &mut vertices)?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(part.add_wire(&refs) as i32, wire.orientation));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}
```

## <a id="checkpoint-84zod-explode-supported-original-families"></a>37. Explode supported families

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 37 of 37</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Count derived pieces before allocating them.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
use session_rust::{Geometry,Line,Mesh,BRep,BRepRef};
use std::{rc::Rc,collections::HashMap};
use crate::interval_geometry::MAX_POINTS;
```

After that block, type:

```rust
--8<-- "journey/code/84zod-explode-supported-original-families-1.rs" skip=135
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/explode_geometry.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```text
use session_rust::{Geometry,Line,Mesh,BRep,BRepRef};
use std::{rc::Rc,collections::HashMap};
use crate::interval_geometry::MAX_POINTS;
pub fn pieces(geometry: &Geometry) -> Result<Vec<Geometry>, String> {
    let count = match geometry {
        Geometry::Polyline(line) => line.point_count().saturating_sub(1),
        Geometry::BRep(brep) => brep.face_count(),
        Geometry::Mesh(mesh) => mesh.number_of_faces(),
        Geometry::PointCloud(cloud) => cloud.point_count(),
        _ => 0,
    };

    // counted before anything is made
    if count > MAX_POINTS {
        return Err(format!(
            "Explode makes at most {MAX_POINTS} objects at once"
        ));
    }

    let parts: Vec<Geometry> = match geometry {
        Geometry::Polyline(line) => line
            .get_points()
            .windows(2)
            .filter(|pair| pair[0].distance(&pair[1], None) > 1e-12)
            .map(|pair| {
                let mut part = Line::from_points(&pair[0], &pair[1]);
                part.name = line.name.clone();
                part.width = line.width;
                part.dash = line.dash.clone();
                part.linecolor = line.linecolor.clone();
                Geometry::Line(Rc::new(part))
            })
            .collect(),
        Geometry::BRep(brep) if brep.face_count() > 1 => (0..brep.face_count())
            .map(|face| face_brep(brep, face).map(|part| Geometry::BRep(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This BRep's tables do not resolve")?,
        Geometry::Mesh(mesh) if mesh.number_of_faces() > 1 => mesh
            .faces()
            .into_iter()
            .map(|face| face_mesh(mesh, face).map(|part| Geometry::Mesh(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This mesh's faces do not resolve")?,
        Geometry::PointCloud(cloud) => (0..cloud.point_count())
            .map(|index| {
                let mut point = cloud.get_point(index);
                point.name = cloud.name.clone();
                point.width = cloud.point_size;

                if index < cloud.color_count() {
                    point.pointcolor = cloud.get_color(index);
                }

                Geometry::Point(Rc::new(point))
            })
            .collect(),
        _ => Vec::new(),
    };

    if parts.len() < 2 {
        return Err(format!(
            "{} cannot be exploded",
            crate::source_shape::Shape::of(geometry).label()
        ));
    }

    Ok(parts)
}

/// One face of a mesh as a mesh of its own, holes included.
fn face_mesh(mesh: &Mesh, face: usize) -> Option<Mesh> {
    let outer = mesh.face_vertices(face)?;
    let holes = mesh.face_holes.get(&face);
    let mut index = HashMap::new(); // old vertex key to new
    let mut points = Vec::new();

    for &key in outer.iter().chain(holes.into_iter().flatten().flatten()) {
        if !index.contains_key(&key) {
            index.insert(key, points.len());
            points.push(mesh.vertex_point(key)?);
        }
    }

    let ring = outer.iter().map(|key| index[key]).collect();
    let mut part = Mesh::from_vertices_and_faces(points, vec![ring]);

    if let Some(holes) = holes {
        let rings = holes
            .iter()
            .map(|hole| hole.iter().map(|key| index[key]).collect())
            .collect();
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(brep.m_surfaces.get(usize::try_from(surface).ok()?)?);
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep.m_wires.get(usize::try_from(wire.index).ok()?)?.edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(brep, &mut part, used.index, surface, &mut vertices)?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(part.add_wire(&refs) as i32, wire.orientation));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep.m_edges.get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32,
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends.iter_mut().zip([edge.start_vertex, edge.end_vertex]) {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep.m_vertices.get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/84zod-explode-supported-original-families-2.rs" skip=6850
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 16.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 16.4
npm --prefix ../session_tests run course -- save 16.4
```

Continue with [16.5 · Text, Plane, Trim and integrated editing commands](../../consolidation/85-copy-current-original-command-recipes.md).
