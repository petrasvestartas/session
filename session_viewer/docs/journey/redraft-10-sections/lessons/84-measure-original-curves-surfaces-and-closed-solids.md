# 16.4 · Measurements, temporary marks and retained CAD pieces

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Checkpoints:** `84-measure-original-curves-surfaces-and-closed-solids` → `84zod-explode-supported-original-families` (37 steps) · **Production `session_viewer/src` target:** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs) · [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs) · [app/splitting.rs](../../../../src/app/splitting.rs) · [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs) · [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs) · [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs) · [app/walk/frames.rs](../../../../src/app/walk/frames.rs) (`session_viewer/src/lib.rs`) · **Goal:** Count derived pieces before allocating them.

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 16.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/measure_volume.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">84-measure-original-curves-surfaces-and-closed-solids -&gt; 84zod-explode-supported-original-families</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Measure original closed-s…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Rg32Uint [R:owner, G:sub]</text>
  <rect x="624" y="50" width="152" height="92" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="624" y="50" width="50" height="30" fill="#fee2e2" stroke="#111111" stroke-width="1"/>
  <text x="649" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">R:id=4</text>
  <rect x="674" y="50" width="52" height="30" fill="#dcfce7" stroke="#111111" stroke-width="1"/>
  <text x="700" y="69" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">G:sub=2</text>
  <line x1="674" y1="50" x2="674" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="726" y1="50" x2="726" y2="142" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="80" x2="776" y2="80" stroke="#111111" stroke-width="1"/>
  <line x1="624" y1="111" x2="776" y2="111" stroke="#111111" stroke-width="1"/>
  <rect x="674" y="80" width="52" height="31" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 3]</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Measurements, temporary mar…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Explode supported origina…</text>
</svg>
</div>

## <a id="checkpoint-84-measure-original-curves-surfaces-and-closed-solids"></a>Step 1 of 37 · Measure original closed-solid volume (`84-measure-original-curves-surfaces-and-closed-solids`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/37: 84-measure-origin…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_volume.rs, src/…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">measure or refusal</text>
</svg>
</div>

Reject open geometry, choose a world point near the mesh and reuse the shared swept-volume helper. Orient solid BRep face meshes before adding their signed volumes.

> **Check:** *Why choose an origin on the mesh?* — Subtracting a nearby origin keeps large world translations out of the triple products and preserves volume precision.

### 1. `src/measure_volume.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-1.rs"
```

### 2. `src/lib.rs`

Register this original measurement owner.

After that block, type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-2.rs" skip=23
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/measure_volume.rs`

Supply only the unchanged original validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-3.rs" skip=2289
```

**Result after `84-measure-original-curves-surfaces-and-closed-solids`:** The original split face and its cutter remain visible while measurement connects.

## <a id="checkpoint-84a-validate-an-original-box-and-derive-wire-only-display"></a>Step 2 of 37 · Validate an original box and derive wire-only display (`84a-validate-an-original-box-and-derive-wire-only-display`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/37: 84a-validate-an-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/bounds_or…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original box frame</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite orthonormal valida…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">twelve display edges</text>
</svg>
</div>

Keep the original box frame separate from its twelve-edge display proxy. Reject nonfinite extents and nonorthogonal axes before allocation.

> **Check:** *Does an original bounding box need opaque display faces?* — No. Its eight corners and twelve edges show the bounds while leaving the enclosed solid visible.

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

**Complete updated block reference:**

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
    pub original:Option<session_rust::Geometry>,
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn visible(&self) -> bool { self.metadata.is_visible }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable { crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry), _ => None }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable { crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(), _ => None }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref().map(|source| &source.origin),
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable { crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch), _ => None }
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else { return false; };
        let Some(geometry) = self.geometry() else { return false; };
        source.origin.location.is_some() && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible && self.metadata.is_locked == geometry.is_locked
            && source.document.objects.meshes.iter().any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct LineObject {
    pub guid: String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}

#[derive(Clone)]
pub struct PathObject { pub controls_visible: bool, pub id: ObjectId, pub guid: String, pub prepared: crate::chain::PreparedChain, pub model: session_rust::Xform }

impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.bounds_coordinates().into_iter().map(|p|
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

}

pub struct ControlMarker { pub parent: ObjectId, pub control: crate::controls::Control, pub display: crate::marker::Marker }

#[derive(Clone)]
pub struct PointObject { pub id: ObjectId, pub guid: String, pub prepared: crate::marker::PreparedPoint, pub model: session_rust::Xform }

impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
pub struct Scene {
    pub cad_walks:[usize;2],
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    sheets:Vec<crate::sheet_source::Row>,
    streamed:Vec<crate::range_cloud::Row>,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }

    pub fn empty()->Self {
        Self {cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),sheets:Vec::new(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self,id:ObjectId)->bool {self.contains(id)&&!self.view.is_hidden(&id)}

    pub(crate) fn restore_view(&mut self,mut saved:Self) {saved.view=Rc::clone(&self.view);self.restore(saved);Rc::make_mut(&mut self.view).refresh(&self.forest);}

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self.objects.iter().filter(|row| self.visible(row.id)) {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self.lines.iter().filter(|line| self.visible(line.id)) {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self.paths.iter().filter(|row| self.visible(row.id)) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds { Some(bounds) => bounds.include(point), None => bounds = Some(crate::bounds::Bounds::point(point)) }
            }
        }
        for row in self.points.iter().filter(|row| self.visible(row.id)) {
            let p = row.point(); let p = [p[0], p[1], p[2]];
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.sheets.iter().filter(|r|self.visible(r.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.streamed.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids { if let Some(next) = self.selected_bounds(*id) {
            if let Some(b) = &mut bounds { b.include(next.min); b.include(next.max); } else { bounds = Some(next); }
        } } bounds
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects.iter().find(|r| r.id==id).map(|r| r.model.clone())
            .or_else(|| self.lines.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.streamed.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.sheets.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }

    pub fn with_descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {self.forest.descendants(ids).into_iter().filter(|id|self.contains(*id)).collect()}

    pub fn place_owner(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_following(id,model)
    }

    pub(crate) fn place_following(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id==id) { self.place_mesh(id,model) }
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.streamed.iter().any(|r|r.id==id) {self.place_stream(id,model)}
        else if self.sheets.iter().any(|r|r.id==id) {self.place_sheet(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }

    pub fn translate(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() { return Err("Reload editable sources before Move"); }
            let model = &shift * &row.model; return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.streamed.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_stream(id,model);}
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.sheets.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_sheet(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let points=if let Some(row)=self.lines.iter().find(|row|row.id==id){Some(row.endpoints().map(|p|[p[0],p[1],p[2]]).to_vec())}
            else if let Some(row)=self.points.iter().find(|row|row.id==id){let p=row.model.transform_point(&row.prepared.source);Some(vec![[p[0],p[1],p[2]]])}
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn sheets(&self)->&[crate::sheet_source::Row] {&self.sheets}
    pub fn insert_sheet(&mut self,prepared:Rc<crate::sheet_source::Prepared>)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.original.guid.clone();let used=self.used_guids();while used.contains(&guid) {guid=uuid::Uuid::new_v4().to_string();}let name=prepared.original.name.clone();let url=prepared.streamed.as_ref().map(|stream|stream.source.url.clone());let row=crate::sheet_source::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity(),url};self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);self.sheets.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_sheet(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid sheet placement");}let row=self.sheets.iter_mut().find(|r|r.id==id).ok_or("Sheet is gone")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Sheet placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn sheet_import_url(&mut self,url:&str,previous:&std::collections::HashSet<ObjectId>) {for row in &mut self.sheets {if !previous.contains(&row.id) {row.url=Some(url.into());}}}
    pub fn sheet_url(&mut self,url:&str) {for row in &mut self.sheets {row.url=Some(url.into());}}
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
        if !crate::placement::valid(&model.m)||!point_px.is_finite()||point_px<0. {return Err("Invalid streamed source placement or point size");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.streamed.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let row=crate::range_cloud::Row {id,guid:guid.clone(),prepared,model,point_px,controls_visible:false};
        if row.bounds().min.into_iter().chain(row.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}
        let name=row.prepared.source.name.clone();let properties=row.prepared.source.properties;
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,properties.visible,properties.locked);self.streamed.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_stream(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid streamed source placement");}
        let row=self.streamed.iter_mut().find(|row|row.id==id).ok_or("Streamed source is gone")?;let mut candidate=row.clone();candidate.model=model;
        if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {point_px:0.,controls_visible:false,id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid cloud placement");}let row=self.clouds.iter_mut().find(|r|r.id==id).ok_or("Cloud object not found")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!(v as f32).is_finite()) {return Err("Cloud placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn texts(&self)->&[crate::scene_label::Row] {&self.texts}
    pub(crate) fn texts_mut(&mut self)->&mut [crate::scene_label::Row] {&mut self.texts}
    pub fn insert_text(&mut self,prepared:crate::scene_label::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.texts.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();let row=crate::scene_label::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};row.label(false,true)?;if row.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,row.prepared.source.visible,row.prepared.source.locked);self.texts.push(row);crate::layer_edit::attach_generated(self,id,guid,label)?;Ok(id)
    }
    pub fn place_text(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid text placement");}let row=self.texts.iter_mut().find(|row|row.id==id).ok_or("Text object not found")?;let mut candidate=row.clone();candidate.model=model;candidate.label(false,true)?;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}*row=candidate;Ok(())
    }
    pub fn text_labels(&self,selected:&[ObjectId])->Result<Vec<crate::text::TextLabel>,&'static str> {self.texts.iter().filter(|row|self.visible(row.id)).map(|row|row.label(selected.contains(&row.id),self.pick_id(row.id.value()).is_some())).collect()}
    pub fn points(&self) -> &[PointObject] { &self.points }

    pub fn point_capacity_bytes(&self) -> usize { self.points.capacity() * std::mem::size_of::<PointObject>() }

    pub fn insert_point(&mut self, prepared: crate::marker::PreparedPoint) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.points.iter().any(|row| row.guid == guid) || self.paths.iter().any(|row| row.guid == guid)
            || self.objects.iter().any(|row| row.guid == guid) || self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let label=prepared.source.name.clone();let node_guid=guid.clone();self.points.push(PointObject { id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_point(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid point placement"); }
        let row = self.points.iter_mut().find(|row| row.id == id).ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) { return Err("Point placement exceeds the display range"); }
        row.model = model; Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker { center: p.to_f32(), ..control.marker().expect("Prepared control marker") };
                markers.push(ControlMarker { parent: row.id, control, display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }

    pub fn path_capacity_bytes(&self) -> usize { self.paths.capacity() * std::mem::size_of::<PathObject>() }

    pub fn insert_path(&mut self, prepared: crate::chain::PreparedChain) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid) || self.objects.iter().any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid) { guid = uuid::Uuid::new_v4().to_string(); }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.visible(),prepared.source.locked());
        let label=prepared.source.name().to_owned();let node_guid=guid.clone();self.paths.push(PathObject { controls_visible: false, id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_path(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid path placement"); }
        let row = self.paths.iter_mut().find(|row| row.id == id).ok_or("Path not found")?;
        for p in row.prepared.source.coordinates().into_iter().chain(row.prepared.source.bounds_coordinates()) {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) { return Err("Path placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn lines(&self) -> &[LineObject] { &self.lines }

    pub fn insert_line(&mut self, prepared: crate::stroke::PreparedLine) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let mut guid=prepared.source.guid().to_owned();while self.lines.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();
        self.lines.push(LineObject { guid:guid.clone(), id, prepared, model: session_rust::Xform::identity() });
        crate::layer_edit::attach_generated(self,id,guid,label)?;
        Ok(id)
    }

    pub fn place_line(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid line placement"); }
        let row = self.lines.iter_mut().find(|row| row.id == id).ok_or("Line not found")?;
        for point in [row.prepared.source.start(), row.prepared.source.end()] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) { return Err("Line placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] { &mut self.objects }

    pub(crate) fn unload(&mut self, origins: &std::collections::HashSet<uuid::Uuid>, epoch: u64) {
        for row in &mut self.objects {
            if let Some(origin) = row.origin().filter(|origin| origins.contains(&origin.id)).cloned() {
                row.editable = crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) { continue; }
            let Some(geometry) = prepared.geometry(&row.metadata.source_guid).cloned() else { continue; };
            let source = crate::document::Source { document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(), origin: Rc::clone(&prepared.key.origin) };
            row.editable = crate::edit_source::EditSource::Loaded { geometry, source: Some(source) };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) { return Err("Placement must be finite, affine and fit the display range"); }
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.geometry.is_visible,prepared.geometry.is_locked);
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {original:None,cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(&mut self,(prepared,mesh):(crate::cad_display::Prepared,crate::prepared::PreparedMesh))->Result<ObjectId,&'static str> {
        let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let copy=ObjectId(self.next_id);let guid=uuid::Uuid::new_v4().to_string();
        if let Some(row)=self.objects.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.objects.push(row);}
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.streamed.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.streamed.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.sheets.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.sheets.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
        Ok(copy)
    }

    pub(crate) fn row_guid(&self,id:ObjectId)->Option<&str> {
        self.objects.iter().find(|row|row.id==id).map(|row|row.guid.as_str())
            .or_else(||self.paths.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.sheets.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }

    pub(crate) fn append_document(&mut self,source:&Scene,name:&str,place:&session_rust::Xform,point_px:f32)->Result<(),&'static str> {
        if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size");}
        let mut candidate=self.clone();let mut ids=source.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());
        let mut remap=std::collections::HashMap::new();let mut names=std::collections::HashMap::new();let mut used=candidate.ids.iter().filter_map(|id|candidate.row_guid(*id).map(str::to_owned)).collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id=ObjectId(candidate.reserve_document_key()?);remap.insert(*old,id);Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(streamed);rows!(texts);rows!(sheets);
        for old in &ids {
            let id=remap[old];if let Some(owner)=source.instances.get(old) {candidate.instances.insert(id,owner.clone());}candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
            if source.streamed.iter().any(|row|row.id==*old)&&point_px>0. {candidate.streamed.iter_mut().find(|row|row.id==id).unwrap().point_px=point_px;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
            if !name.is_empty() {doc.label=name.to_owned();}
            let guids=doc.nodes.iter().map(|node|(node.source.guid.clone(),uuid::Uuid::new_v4().to_string())).collect::<std::collections::HashMap<_,_>>();
            for node in &mut doc.nodes {node.object=node.object.map(|id|remap[&id]);node.source.guid=guids[&node.source.guid].clone();node.portable_guid=node.source.guid.clone();node.source.parent_guid=guids.get(&node.source.parent_guid).cloned().unwrap_or_default();}
            if source.created==Some(old_key)&&candidate.created.is_none() {candidate.created=Some(doc.key);}
            candidate.forest.push(doc);
        }
        self.restore(candidate);Ok(())
    }

    pub fn face_rgba(&self,id:ObjectId)->Option<[f32;4]> {
        if let Some(rgb)=self.view.faces.get(&id) {return Some([rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);}
        let owner=self.instances.get(&id)?;
        (owner.instance.flags&session_rust::InstanceRef::FLAG_COLOR!=0).then(|| {let c=&owner.instance.color;[c.r as f32,c.g as f32,c.b as f32,c.a as f32]})
    }

    pub(crate) fn used_guids(&self)->std::collections::HashSet<String> {
        self.objects.iter().map(|r|r.guid.clone()).chain(self.lines.iter().map(|r|r.guid.clone())).chain(self.paths.iter().map(|r|r.guid.clone())).chain(self.points.iter().map(|r|r.guid.clone())).chain(self.clouds.iter().map(|r|r.guid.clone())).chain(self.streamed.iter().map(|r|r.guid.clone())).chain(self.texts.iter().map(|r|r.guid.clone())).chain(self.sheets.iter().map(|r|r.guid.clone())).collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(&mut self,template:&Scene,owner:crate::instance_plan::Owner,model:session_rust::Xform,guid:String,point_px:f32)->Result<ObjectId,&'static str> {
        let mut placed=template.clone();let original=placed.ids.iter().copied().next().ok_or("Missing definition display owner")?;
        if placed.ids.len()!=1 {return Err("One definition must create one drawable owner");}
        placed.place_following(original,model)?;if !placed.clouds.is_empty()&&point_px>0. {placed.cloud_point_size(original,point_px)?;}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);
        let visible=owner.instance.flags&session_rust::InstanceRef::FLAG_HIDDEN==0;let locked=owner.instance.flags&session_rust::InstanceRef::FLAG_LOCKED!=0;
        if let Some(row)=placed.objects.first_mut() {row.metadata=Rc::new(crate::row_metadata::RowMetadata {source_guid:owner.instance.guid().into(),name:owner.instance.name.clone(),is_visible:visible,is_locked:locked});}
        macro_rules! append {($field:ident)=>{if let Some(mut row)=placed.$field.pop() {row.id=id;row.guid=guid.clone();self.$field.push(row);}};}
        append!(objects);append!(lines);append!(paths);append!(points);append!(clouds);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.instances.insert(id,owner);Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self)->Result<u32,&'static str> {
        let key=self.next_id;self.next_id=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;Ok(key)
    }

    pub(crate) fn cloud_point_size(&mut self,id:ObjectId,px:f32)->Result<(),&'static str> {
        if !px.is_finite() {return Err("Non-finite cloud point size");}
        self.clouds.iter_mut().find(|row|row.id==id).ok_or("Cloud object not found")?.point_px=px;Ok(())
    }

    pub fn clear(&mut self) {
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();self.streamed=Vec::new();self.sheets=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        let mut objects=Vec::new();let key=self.next_id;
        for prepared in loaded.meshes {
            let guid=prepared.geometry.guid().to_owned();let name=prepared.geometry.name.clone();
            let id=self.insert(prepared)?;objects.push((guid,id,name));
        }
        self.forest.add(key,loaded.document.name.clone(),loaded.tree,&objects);if let Some(origin)=objects.first().and_then(|(_,id,_)|self.objects.iter().find(|row|row.id==*id)).and_then(|row|row.origin()).cloned() {Rc::make_mut(self.forest.documents.last_mut().unwrap()).header=Some(origin);}let colours=self.forest.colours(key);Rc::make_mut(&mut self.view).faces.extend(colours);Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_mesh(id,model)
    }

    fn place_mesh(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);self.sheets.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }

    pub(crate) fn all_ids(&self)->std::collections::HashSet<ObjectId> {(*self.ids).clone()}
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).chain(self.streamed.iter().map(|r|r.id)).chain(self.sheets.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo(); assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene.replace(crate::document::load(&bytes).unwrap()).unwrap(); assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:policy".into()));
        scene.replace(crate::document::load_at(&bytes, Some(url)).unwrap()).unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true; assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned(); let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source }; assert!(!row.can_unload());
    }

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(object.geometry().unwrap(), old.geometry().unwrap()));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn replace_original(&mut self,id:ObjectId,mut geometry:session_rust::Geometry)->Result<(),&'static str>{
        use session_rust::Geometry;
        if self.pick_id(id.value())!=Some(id){return Err("Edit a visible unlocked original")}
        let original=crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(original.guid());
        match geometry{
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}self.points.iter_mut().find(|r|r.id==id).ok_or("Expected a point source")?.prepared=prepared;},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{let prepared=crate::stroke::PreparedLine::new(source)?;for p in [prepared.source.start(),prepared.source.end()]{let p=model.transform_point(&p);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=prepared;},
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
                let prepared=match geometry{Geometry::Line(source)=>crate::chain::PreparedChain::line(source)?,Geometry::Polyline(source)=>crate::chain::PreparedChain::polyline(source)?,Geometry::NurbsCurve(source)=>crate::chain::PreparedChain::curve(source)?,_=>unreachable!()};
                for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()){let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited path exceeds display range")}}self.paths.iter_mut().find(|r|r.id==id).ok_or("Expected a path source")?.prepared=prepared;
            },
            Geometry::Mesh(_)|Geometry::BRep(_)|Geometry::NurbsSurface(_)=>{
                let(cad,surface,prepared)=match geometry{Geometry::Mesh(source)=>(None,None,crate::prepared::PreparedMesh::new((*source).clone())?),Geometry::BRep(source)=>{let(cad,mesh)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,mesh)},Geometry::NurbsSurface(source)=>{let s=Rc::new(crate::surface::Prepared::new(source)?);let mesh=s.mesh()?;(None,Some(s),mesh)},_=>unreachable!()};
                for v in prepared.display.vertices(){let p=model.transform_point(&session_rust::Point::new(v[0]as f64,v[1]as f64,v[2]as f64));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited object exceeds display range")}}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected an object source")?;row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};
            },
            _=>return Err("This original source cannot be replaced"),
        }Ok(())
    }
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {
            let original=crate::source_part::geometry(self,target.owner)?;let edited=crate::source_deform::transform(&original,crate::source_part::target(target.part)?,&local).map_err(|_|"CAD edit would invalidate the original source or adjacent trims")?;
            let (cad,surface,prepared)=match edited {
                session_rust::Geometry::BRep(source)=>{let(cad,prepared)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,prepared)},
                session_rust::Geometry::NurbsSurface(source)=>{let surface=Rc::new(crate::surface::Prepared::new(source)?);let prepared=surface.mesh()?;(None,Some(surface),prepared)},_=>return Err("Unexpected edited CAD source"),
            };
            for vertex in prepared.display.vertices(){let point=model.transform_point(&session_rust::Point::new(vertex[0]as f64,vertex[1]as f64,vertex[2]as f64));if(0..3).any(|i|!(point[i]as f32).is_finite()){return Err("Part edit exceeds the display range")}}
            let row=self.objects.iter_mut().find(|row|row.id==target.owner).unwrap();row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};return Ok(());
        }
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
                source.vertex.get_mut(&key).ok_or("Missing source vertex")?.set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared=crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {let point=model.transform_point(&session_rust::Point::new(vertex[0] as f64,vertex[1] as f64,vertex[2] as f64));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}}
            row.mesh=Rc::new(prepared.display); row.editable=crate::edit_source::EditSource::Loaded {geometry:prepared.geometry,source:None}; return Ok(());
        }
        let row=self.paths.iter_mut().find(|r|r.id==target.owner).ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        let mut source=(**curve).clone(); let point=local.transform_point(&source.get_cv(index).ok_or("Missing curve control")?);
        let weight=source.get_cv_4d(index).ok_or("Missing curve control")?.3;
        let set=if source.m_is_rat {source.set_cv_4d(index,point[0]*weight,point[1]*weight,point[2]*weight,weight)} else {source.set_cv_point(index,&point)};
        if !set {return Err("Cannot set curve control");}
        let prepared=crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()) {
            let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}
        }
        row.prepared=prepared;Ok(())
    }
}
```

### 2. `src/bounds_original.rs`

Validate original box frames and prepare a wire-only display.

Create the file and type:

```rust
--8<-- "journey/code/84a-validate-an-original-box-and-derive-wire-only-display-03.rs"
```

### 3. `src/lib.rs`

Register the measurement component.

After that block, type:

```rust
--8<-- "journey/code/84a-validate-an-original-box-and-derive-wire-only-display-04.rs" skip=25
```

## <a id="checkpoint-84b-adopt-original-boxes-and-retain-precise-placed-bounds"></a>Step 3 of 37 · Adopt original boxes and retain precise placed bounds (`84b-adopt-original-boxes-and-retain-precise-placed-bounds`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/37: 84b-adopt-origina…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bounds_original.rs, src…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original corners</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stable source row</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">checked replacement</text>
</svg>
</div>

Compute bounds from original f64 corners, insert an original box owner and reject edits that would deform only its mesh proxy.

> **Check:** *Can distant original box extents survive Save independently of display floats?* — Yes. Original f64 coordinates remain the source. The current f32 display still requires the later large-world rendering audit.

### 1. `src/bounds_original.rs`

Adopt and place original boxes without substituting their display proxy.

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

**Complete updated block reference:**

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
    pub original:Option<session_rust::Geometry>,
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn visible(&self) -> bool { self.metadata.is_visible }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable { crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry), _ => None }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable { crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(), _ => None }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref().map(|source| &source.origin),
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable { crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch), _ => None }
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else { return false; };
        let Some(geometry) = self.geometry() else { return false; };
        source.origin.location.is_some() && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible && self.metadata.is_locked == geometry.is_locked
            && source.document.objects.meshes.iter().any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct LineObject {
    pub guid: String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}

#[derive(Clone)]
pub struct PathObject { pub controls_visible: bool, pub id: ObjectId, pub guid: String, pub prepared: crate::chain::PreparedChain, pub model: session_rust::Xform }

impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.bounds_coordinates().into_iter().map(|p|
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

}

pub struct ControlMarker { pub parent: ObjectId, pub control: crate::controls::Control, pub display: crate::marker::Marker }

#[derive(Clone)]
pub struct PointObject { pub id: ObjectId, pub guid: String, pub prepared: crate::marker::PreparedPoint, pub model: session_rust::Xform }

impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
pub struct Scene {
    pub cad_walks:[usize;2],
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    sheets:Vec<crate::sheet_source::Row>,
    streamed:Vec<crate::range_cloud::Row>,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }

    pub fn empty()->Self {
        Self {cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),sheets:Vec::new(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self,id:ObjectId)->bool {self.contains(id)&&!self.view.is_hidden(&id)}

    pub(crate) fn restore_view(&mut self,mut saved:Self) {saved.view=Rc::clone(&self.view);self.restore(saved);Rc::make_mut(&mut self.view).refresh(&self.forest);}

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self.objects.iter().filter(|row| self.visible(row.id)) {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self.lines.iter().filter(|line| self.visible(line.id)) {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self.paths.iter().filter(|row| self.visible(row.id)) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds { Some(bounds) => bounds.include(point), None => bounds = Some(crate::bounds::Bounds::point(point)) }
            }
        }
        for row in self.points.iter().filter(|row| self.visible(row.id)) {
            let p = row.point(); let p = [p[0], p[1], p[2]];
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.sheets.iter().filter(|r|self.visible(r.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.streamed.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids { if let Some(next) = self.selected_bounds(*id) {
            if let Some(b) = &mut bounds { b.include(next.min); b.include(next.max); } else { bounds = Some(next); }
        } } bounds
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects.iter().find(|r| r.id==id).map(|r| r.model.clone())
            .or_else(|| self.lines.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.streamed.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.sheets.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }

    pub fn with_descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {self.forest.descendants(ids).into_iter().filter(|id|self.contains(*id)).collect()}

    pub fn place_owner(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_following(id,model)
    }

    pub(crate) fn place_following(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id==id) { self.place_mesh(id,model) }
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.streamed.iter().any(|r|r.id==id) {self.place_stream(id,model)}
        else if self.sheets.iter().any(|r|r.id==id) {self.place_sheet(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }

    pub fn translate(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() { return Err("Reload editable sources before Move"); }
            let model = &shift * &row.model; return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.streamed.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_stream(id,model);}
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.sheets.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_sheet(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let points=if let Some(row)=self.lines.iter().find(|row|row.id==id){Some(row.endpoints().map(|p|[p[0],p[1],p[2]]).to_vec())}
            else if let Some(row)=self.points.iter().find(|row|row.id==id){let p=row.model.transform_point(&row.prepared.source);Some(vec![[p[0],p[1],p[2]]])}
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::OBB(source))=&object.original{return Some(crate::bounds_original::placed(source,&object.model))}
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn sheets(&self)->&[crate::sheet_source::Row] {&self.sheets}
    pub fn insert_sheet(&mut self,prepared:Rc<crate::sheet_source::Prepared>)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.original.guid.clone();let used=self.used_guids();while used.contains(&guid) {guid=uuid::Uuid::new_v4().to_string();}let name=prepared.original.name.clone();let url=prepared.streamed.as_ref().map(|stream|stream.source.url.clone());let row=crate::sheet_source::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity(),url};self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);self.sheets.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_sheet(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid sheet placement");}let row=self.sheets.iter_mut().find(|r|r.id==id).ok_or("Sheet is gone")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Sheet placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn sheet_import_url(&mut self,url:&str,previous:&std::collections::HashSet<ObjectId>) {for row in &mut self.sheets {if !previous.contains(&row.id) {row.url=Some(url.into());}}}
    pub fn sheet_url(&mut self,url:&str) {for row in &mut self.sheets {row.url=Some(url.into());}}
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
        if !crate::placement::valid(&model.m)||!point_px.is_finite()||point_px<0. {return Err("Invalid streamed source placement or point size");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.streamed.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let row=crate::range_cloud::Row {id,guid:guid.clone(),prepared,model,point_px,controls_visible:false};
        if row.bounds().min.into_iter().chain(row.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}
        let name=row.prepared.source.name.clone();let properties=row.prepared.source.properties;
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,properties.visible,properties.locked);self.streamed.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_stream(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid streamed source placement");}
        let row=self.streamed.iter_mut().find(|row|row.id==id).ok_or("Streamed source is gone")?;let mut candidate=row.clone();candidate.model=model;
        if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {point_px:0.,controls_visible:false,id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid cloud placement");}let row=self.clouds.iter_mut().find(|r|r.id==id).ok_or("Cloud object not found")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!(v as f32).is_finite()) {return Err("Cloud placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn texts(&self)->&[crate::scene_label::Row] {&self.texts}
    pub(crate) fn texts_mut(&mut self)->&mut [crate::scene_label::Row] {&mut self.texts}
    pub fn insert_text(&mut self,prepared:crate::scene_label::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.texts.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();let row=crate::scene_label::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};row.label(false,true)?;if row.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,row.prepared.source.visible,row.prepared.source.locked);self.texts.push(row);crate::layer_edit::attach_generated(self,id,guid,label)?;Ok(id)
    }
    pub fn place_text(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid text placement");}let row=self.texts.iter_mut().find(|row|row.id==id).ok_or("Text object not found")?;let mut candidate=row.clone();candidate.model=model;candidate.label(false,true)?;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}*row=candidate;Ok(())
    }
    pub fn text_labels(&self,selected:&[ObjectId])->Result<Vec<crate::text::TextLabel>,&'static str> {self.texts.iter().filter(|row|self.visible(row.id)).map(|row|row.label(selected.contains(&row.id),self.pick_id(row.id.value()).is_some())).collect()}
    pub fn points(&self) -> &[PointObject] { &self.points }

    pub fn point_capacity_bytes(&self) -> usize { self.points.capacity() * std::mem::size_of::<PointObject>() }

    pub fn insert_point(&mut self, prepared: crate::marker::PreparedPoint) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.points.iter().any(|row| row.guid == guid) || self.paths.iter().any(|row| row.guid == guid)
            || self.objects.iter().any(|row| row.guid == guid) || self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let label=prepared.source.name.clone();let node_guid=guid.clone();self.points.push(PointObject { id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_point(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid point placement"); }
        let row = self.points.iter_mut().find(|row| row.id == id).ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) { return Err("Point placement exceeds the display range"); }
        row.model = model; Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker { center: p.to_f32(), ..control.marker().expect("Prepared control marker") };
                markers.push(ControlMarker { parent: row.id, control, display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }

    pub fn path_capacity_bytes(&self) -> usize { self.paths.capacity() * std::mem::size_of::<PathObject>() }

    pub fn insert_path(&mut self, prepared: crate::chain::PreparedChain) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid) || self.objects.iter().any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid) { guid = uuid::Uuid::new_v4().to_string(); }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.visible(),prepared.source.locked());
        let label=prepared.source.name().to_owned();let node_guid=guid.clone();self.paths.push(PathObject { controls_visible: false, id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_path(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid path placement"); }
        let row = self.paths.iter_mut().find(|row| row.id == id).ok_or("Path not found")?;
        for p in row.prepared.source.coordinates().into_iter().chain(row.prepared.source.bounds_coordinates()) {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) { return Err("Path placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn lines(&self) -> &[LineObject] { &self.lines }

    pub fn insert_line(&mut self, prepared: crate::stroke::PreparedLine) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let mut guid=prepared.source.guid().to_owned();while self.lines.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();
        self.lines.push(LineObject { guid:guid.clone(), id, prepared, model: session_rust::Xform::identity() });
        crate::layer_edit::attach_generated(self,id,guid,label)?;
        Ok(id)
    }

    pub fn place_line(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid line placement"); }
        let row = self.lines.iter_mut().find(|row| row.id == id).ok_or("Line not found")?;
        for point in [row.prepared.source.start(), row.prepared.source.end()] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) { return Err("Line placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] { &mut self.objects }

    pub(crate) fn unload(&mut self, origins: &std::collections::HashSet<uuid::Uuid>, epoch: u64) {
        for row in &mut self.objects {
            if let Some(origin) = row.origin().filter(|origin| origins.contains(&origin.id)).cloned() {
                row.editable = crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) { continue; }
            let Some(geometry) = prepared.geometry(&row.metadata.source_guid).cloned() else { continue; };
            let source = crate::document::Source { document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(), origin: Rc::clone(&prepared.key.origin) };
            row.editable = crate::edit_source::EditSource::Loaded { geometry, source: Some(source) };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) { return Err("Placement must be finite, affine and fit the display range"); }
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.geometry.is_visible,prepared.geometry.is_locked);
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {original:None,cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(&mut self,(prepared,mesh):(crate::cad_display::Prepared,crate::prepared::PreparedMesh))->Result<ObjectId,&'static str> {
        let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let copy=ObjectId(self.next_id);let guid=uuid::Uuid::new_v4().to_string();
        if let Some(row)=self.objects.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.objects.push(row);}
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.streamed.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.streamed.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.sheets.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.sheets.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
        Ok(copy)
    }

    pub(crate) fn row_guid(&self,id:ObjectId)->Option<&str> {
        self.objects.iter().find(|row|row.id==id).map(|row|row.guid.as_str())
            .or_else(||self.paths.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.sheets.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }

    pub(crate) fn append_document(&mut self,source:&Scene,name:&str,place:&session_rust::Xform,point_px:f32)->Result<(),&'static str> {
        if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size");}
        let mut candidate=self.clone();let mut ids=source.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());
        let mut remap=std::collections::HashMap::new();let mut names=std::collections::HashMap::new();let mut used=candidate.ids.iter().filter_map(|id|candidate.row_guid(*id).map(str::to_owned)).collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id=ObjectId(candidate.reserve_document_key()?);remap.insert(*old,id);Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(streamed);rows!(texts);rows!(sheets);
        for old in &ids {
            let id=remap[old];if let Some(owner)=source.instances.get(old) {candidate.instances.insert(id,owner.clone());}candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
            if source.streamed.iter().any(|row|row.id==*old)&&point_px>0. {candidate.streamed.iter_mut().find(|row|row.id==id).unwrap().point_px=point_px;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
            if !name.is_empty() {doc.label=name.to_owned();}
            let guids=doc.nodes.iter().map(|node|(node.source.guid.clone(),uuid::Uuid::new_v4().to_string())).collect::<std::collections::HashMap<_,_>>();
            for node in &mut doc.nodes {node.object=node.object.map(|id|remap[&id]);node.source.guid=guids[&node.source.guid].clone();node.portable_guid=node.source.guid.clone();node.source.parent_guid=guids.get(&node.source.parent_guid).cloned().unwrap_or_default();}
            if source.created==Some(old_key)&&candidate.created.is_none() {candidate.created=Some(doc.key);}
            candidate.forest.push(doc);
        }
        self.restore(candidate);Ok(())
    }

    pub fn face_rgba(&self,id:ObjectId)->Option<[f32;4]> {
        if let Some(rgb)=self.view.faces.get(&id) {return Some([rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);}
        let owner=self.instances.get(&id)?;
        (owner.instance.flags&session_rust::InstanceRef::FLAG_COLOR!=0).then(|| {let c=&owner.instance.color;[c.r as f32,c.g as f32,c.b as f32,c.a as f32]})
    }

    pub(crate) fn used_guids(&self)->std::collections::HashSet<String> {
        self.objects.iter().map(|r|r.guid.clone()).chain(self.lines.iter().map(|r|r.guid.clone())).chain(self.paths.iter().map(|r|r.guid.clone())).chain(self.points.iter().map(|r|r.guid.clone())).chain(self.clouds.iter().map(|r|r.guid.clone())).chain(self.streamed.iter().map(|r|r.guid.clone())).chain(self.texts.iter().map(|r|r.guid.clone())).chain(self.sheets.iter().map(|r|r.guid.clone())).collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(&mut self,template:&Scene,owner:crate::instance_plan::Owner,model:session_rust::Xform,guid:String,point_px:f32)->Result<ObjectId,&'static str> {
        let mut placed=template.clone();let original=placed.ids.iter().copied().next().ok_or("Missing definition display owner")?;
        if placed.ids.len()!=1 {return Err("One definition must create one drawable owner");}
        placed.place_following(original,model)?;if !placed.clouds.is_empty()&&point_px>0. {placed.cloud_point_size(original,point_px)?;}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);
        let visible=owner.instance.flags&session_rust::InstanceRef::FLAG_HIDDEN==0;let locked=owner.instance.flags&session_rust::InstanceRef::FLAG_LOCKED!=0;
        if let Some(row)=placed.objects.first_mut() {row.metadata=Rc::new(crate::row_metadata::RowMetadata {source_guid:owner.instance.guid().into(),name:owner.instance.name.clone(),is_visible:visible,is_locked:locked});}
        macro_rules! append {($field:ident)=>{if let Some(mut row)=placed.$field.pop() {row.id=id;row.guid=guid.clone();self.$field.push(row);}};}
        append!(objects);append!(lines);append!(paths);append!(points);append!(clouds);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.instances.insert(id,owner);Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self)->Result<u32,&'static str> {
        let key=self.next_id;self.next_id=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;Ok(key)
    }

    pub(crate) fn cloud_point_size(&mut self,id:ObjectId,px:f32)->Result<(),&'static str> {
        if !px.is_finite() {return Err("Non-finite cloud point size");}
        self.clouds.iter_mut().find(|row|row.id==id).ok_or("Cloud object not found")?.point_px=px;Ok(())
    }

    pub fn clear(&mut self) {
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();self.streamed=Vec::new();self.sheets=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        let mut objects=Vec::new();let key=self.next_id;
        for prepared in loaded.meshes {
            let guid=prepared.geometry.guid().to_owned();let name=prepared.geometry.name.clone();
            let id=self.insert(prepared)?;objects.push((guid,id,name));
        }
        self.forest.add(key,loaded.document.name.clone(),loaded.tree,&objects);if let Some(origin)=objects.first().and_then(|(_,id,_)|self.objects.iter().find(|row|row.id==*id)).and_then(|row|row.origin()).cloned() {Rc::make_mut(self.forest.documents.last_mut().unwrap()).header=Some(origin);}let colours=self.forest.colours(key);Rc::make_mut(&mut self.view).faces.extend(colours);Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_mesh(id,model)
    }

    fn place_mesh(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);self.sheets.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }

    pub(crate) fn all_ids(&self)->std::collections::HashSet<ObjectId> {(*self.ids).clone()}
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).chain(self.streamed.iter().map(|r|r.id)).chain(self.sheets.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo(); assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene.replace(crate::document::load(&bytes).unwrap()).unwrap(); assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:policy".into()));
        scene.replace(crate::document::load_at(&bytes, Some(url)).unwrap()).unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true; assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned(); let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source }; assert!(!row.can_unload());
    }

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(object.geometry().unwrap(), old.geometry().unwrap()));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn replace_original(&mut self,id:ObjectId,mut geometry:session_rust::Geometry)->Result<(),&'static str>{
        use session_rust::Geometry;
        if self.pick_id(id.value())!=Some(id){return Err("Edit a visible unlocked original")}
        let original=crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(original.guid());
        match geometry{
            Geometry::OBB(source)=>{let prepared=crate::bounds_original::prepared(&source)?;let bounds=crate::bounds_original::placed(&source,&model);if bounds.min.into_iter().chain(bounds.max).any(|v|!(v as f32).is_finite()){return Err("Edited box exceeds display range")}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a box source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::OBB(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}self.points.iter_mut().find(|r|r.id==id).ok_or("Expected a point source")?.prepared=prepared;},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{let prepared=crate::stroke::PreparedLine::new(source)?;for p in [prepared.source.start(),prepared.source.end()]{let p=model.transform_point(&p);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=prepared;},
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
                let prepared=match geometry{Geometry::Line(source)=>crate::chain::PreparedChain::line(source)?,Geometry::Polyline(source)=>crate::chain::PreparedChain::polyline(source)?,Geometry::NurbsCurve(source)=>crate::chain::PreparedChain::curve(source)?,_=>unreachable!()};
                for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()){let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited path exceeds display range")}}self.paths.iter_mut().find(|r|r.id==id).ok_or("Expected a path source")?.prepared=prepared;
            },
            Geometry::Mesh(_)|Geometry::BRep(_)|Geometry::NurbsSurface(_)=>{
                let(cad,surface,prepared)=match geometry{Geometry::Mesh(source)=>(None,None,crate::prepared::PreparedMesh::new((*source).clone())?),Geometry::BRep(source)=>{let(cad,mesh)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,mesh)},Geometry::NurbsSurface(source)=>{let s=Rc::new(crate::surface::Prepared::new(source)?);let mesh=s.mesh()?;(None,Some(s),mesh)},_=>unreachable!()};
                for v in prepared.display.vertices(){let p=model.transform_point(&session_rust::Point::new(v[0]as f64,v[1]as f64,v[2]as f64));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited object exceeds display range")}}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected an object source")?;row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};
            },
            _=>return Err("This original source cannot be replaced"),
        }Ok(())
    }
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        if self.objects.iter().any(|row|row.id==target.owner&&row.original.is_some()){return Err("Part editing for this original family connects in the complete command inventory")}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {
            let original=crate::source_part::geometry(self,target.owner)?;let edited=crate::source_deform::transform(&original,crate::source_part::target(target.part)?,&local).map_err(|_|"CAD edit would invalidate the original source or adjacent trims")?;
            let (cad,surface,prepared)=match edited {
                session_rust::Geometry::BRep(source)=>{let(cad,prepared)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,prepared)},
                session_rust::Geometry::NurbsSurface(source)=>{let surface=Rc::new(crate::surface::Prepared::new(source)?);let prepared=surface.mesh()?;(None,Some(surface),prepared)},_=>return Err("Unexpected edited CAD source"),
            };
            for vertex in prepared.display.vertices(){let point=model.transform_point(&session_rust::Point::new(vertex[0]as f64,vertex[1]as f64,vertex[2]as f64));if(0..3).any(|i|!(point[i]as f32).is_finite()){return Err("Part edit exceeds the display range")}}
            let row=self.objects.iter_mut().find(|row|row.id==target.owner).unwrap();row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};return Ok(());
        }
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
                source.vertex.get_mut(&key).ok_or("Missing source vertex")?.set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared=crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {let point=model.transform_point(&session_rust::Point::new(vertex[0] as f64,vertex[1] as f64,vertex[2] as f64));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}}
            row.mesh=Rc::new(prepared.display); row.editable=crate::edit_source::EditSource::Loaded {geometry:prepared.geometry,source:None}; return Ok(());
        }
        let row=self.paths.iter_mut().find(|r|r.id==target.owner).ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        let mut source=(**curve).clone(); let point=local.transform_point(&source.get_cv(index).ok_or("Missing curve control")?);
        let weight=source.get_cv_4d(index).ok_or("Missing curve control")?.3;
        let set=if source.m_is_rat {source.set_cv_4d(index,point[0]*weight,point[1]*weight,point[2]*weight,weight)} else {source.set_cv_point(index,&point)};
        if !set {return Err("Cannot set curve control");}
        let prepared=crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()) {
            let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}
        }
        row.prepared=prepared;Ok(())
    }
}
```

### 3. `src/source_edit.rs`

Return the original box allocation before considering display geometry.

Edit the existing block to match this code; keep unchanged lines:

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

After that block, type:

```rust
--8<-- "journey/code/84b-adopt-original-boxes-and-retain-precise-placed-bounds-wire-draw.rs" skip=78
```

### 5. `src/highlight.rs`

Show whole selection on wire-only original edges without creating selected proxy faces.

Edit the existing block to match this code; keep unchanged lines:

```rust
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
```

## <a id="checkpoint-84c-save-and-reopen-the-original-box-frame"></a>Step 4 of 37 · Save and reopen the original box frame (`84c-save-and-reopen-the-original-box-frame`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/37: 84c-save-and-reop…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/geometry_snapshot.rs, s…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original box owner</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked source import</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">same original frame</text>
</svg>
</div>

Include original boxes in mixed document identity, validation, snapshot and import paths. Preserve their family and precise coordinates.

> **Check:** *Should a box proxy be serialized as a mesh?* — No. Save writes the original OBB record. The proxy exists only for drawing and ownership addresses.

### 1. `src/geometry_snapshot.rs`

Retain validated original OBB records through mixed document persistence.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84c-save-and-reopen-the-original-box-frame-01.rs"
```

### 2. `src/document_source.rs`

Retain validated original OBB records through mixed document persistence.

Edit the existing block to match this code; keep unchanged lines:

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

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84c-save-and-reopen-the-original-box-frame-04.rs"
```

### 4. `src/archive_identity.rs`

Retain validated original OBB records through mixed document persistence.

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

**Complete updated block reference:**

```rust
use prost::Message;
use session_rust::proto;

/// Source objects a scene may hold.
pub const MAX_OBJECTS: usize = 2_000_000;

/// Why a scene with more objects is refused.
pub const TOO_MANY: &str = "scene exceeds two million source objects";

/// Deepest tree level a scene may have.
pub const MAX_DEPTH: usize = 64;

/// Why a scene with a deeper tree is refused.
pub const TOO_DEEP: &str = "scene hierarchy exceeds 64 levels";

/// Every value must be finite.
fn finite(values: &[f64], label: &str) -> Result<(), String> {
    for value in values {
        if !value.is_finite() {
            return Err(format!("{label}: non-finite value"));
        }
    }

    Ok(())
}

/// A finite array of whole xyz triples.
fn triples(values: &[f64], label: &str) -> Result<(), String> {
    if !values.len().is_multiple_of(3) {
        return Err(format!("{label}: incomplete coordinate triple"));
    }

    finite(values, label)
}

/// A protobuf decode error as a message.
fn decode_error(error: prost::DecodeError) -> String {
    format!("invalid element geometry: {error}")
}

/// The varint at `i` and its byte length.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;

    loop {
        let byte = *b.get(i)?;

        if shift == 63 && byte > 1 {
            return None;
        }

        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;

        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }

        shift += 7;

        if shift > 63 {
            return None;
        }
    }
}

/// The highest control index a surface reads, None on overflow.
fn addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -> Option<usize> {
    (rows - 1)
        .checked_mul(stride_u)?
        .checked_add((columns - 1).checked_mul(stride_v)?)?
        .checked_add(size)
}

/// Check order, count and knots; returns the knot count.
fn axis(order: i32, count: i32, knots: &[f64]) -> Result<usize, String> {
    if !(2..=64).contains(&order) || count < order || count > 1_000_000 {
        return Err("invalid or oversized NURBS order/control count".into());
    }

    if knots.len() != (order + count - 2) as usize {
        return Err("NURBS knot count does not match its order and controls".into());
    }

    finite(knots, "NURBS knots")?;

    for pair in knots.windows(2) {
        if pair[0] > pair[1] {
            return Err("NURBS knots are not monotonic".into());
        }
    }

    if knots[(order - 2) as usize] >= knots[(count - 1) as usize] {
        return Err("NURBS parameter domain is empty".into());
    }

    Ok(count as usize)
}

/// A JSON parse error as a message.
fn json_error(error: serde_json::Error) -> String {
    format!("invalid session JSON: {error}")
}

/// A count field, defaulted and capped.
fn json_count(
    source: &serde_json::Value,
    key: &str,
    default: u64,
    maximum: u64,
) -> Result<usize, String> {
    let count = match source.get(key) {
        Some(value) => match value.as_u64() {
            Some(value) => value,
            None => return Err(format!("invalid JSON {key}")),
        },
        None => default,
    };

    if count > maximum {
        return Err(format!("JSON {key} exceeds its supported count"));
    }

    Ok(count as usize)
}

/// Check one JSON knot vector.
fn json_axis(
    source: &serde_json::Value,
    order_key: &str,
    count: usize,
    knots_key: &str,
) -> Result<(), String> {
    let order = json_count(source, order_key, 4, 64)?;
    let values = source[knots_key]
        .as_array()
        .ok_or("missing JSON NURBS knots")?;
    let mut knots = Vec::with_capacity(values.len());

    for value in values {
        knots.push(value.as_f64().ok_or("invalid JSON NURBS knot")?);
    }

    axis(order as i32, count as i32, &knots)?;
    Ok(())
}

/// Check one JSON curve or surface's controls against its counts.
fn json_controls(source: &serde_json::Value) -> Result<(), String> {
    let controls = source["control_points"]
        .as_array()
        .ok_or("JSON controls must be an array")?;
    let dimension = json_count(source, "dimension", 3, 3)?;

    if dimension < 2 {
        return Err("invalid JSON control dimension".into());
    }

    let size = dimension + usize::from(source["is_rational"].as_bool().unwrap_or(false));

    if source.get("cv_count_u").is_some() {
        let rows = json_count(source, "cv_count_u", 0, 1_000_000)?;
        let columns = json_count(source, "cv_count_v", 0, 1_000_000)?;
        let total = rows as u64 * columns as u64;

        if total > 4_000_000 || controls.len() as u64 != total * size as u64 {
            return Err("JSON surface controls disagree with their declared counts".into());
        }

        json_axis(source, "order_u", rows, "nurbsknots_u")?;
        json_axis(source, "order_v", columns, "nurbsknots_v")?;
    } else {
        let count = json_count(source, "cv_count", 0, 1_000_000)?;
        let stride = json_count(source, "cv_stride", size as u64, 4)?;

        if count != controls.len() || stride != size {
            return Err("JSON curve controls disagree with their declared count or stride".into());
        }

        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }

        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}

/// Check every NURBS record in session JSON before it is parsed.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];

    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }

        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count") || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }

                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Check one protobuf point.
pub fn point(source: &proto::Point) -> Result<(), String> {
    finite(&[source.x, source.y, source.z, source.width], "point")
}

/// Check one protobuf line.
pub fn line(source: &proto::Line) -> Result<(), String> {
    if source.coords.len() != 6 {
        return Err("line needs exactly two endpoint triples".into());
    }

    finite(&source.coords, "line coordinates")?;
    finite(&source.dash, "line dashes")
}

/// Check one protobuf polyline.
pub fn polyline(source: &proto::Polyline) -> Result<(), String> {
    triples(&source.coords, "polyline coordinates")
}

/// Check one object placement.
pub fn xform(entry: &proto::XformEntry) -> Result<(), String> {
    let Some(transform) = &entry.xform else {
        return Err("missing object transform".into());
    };
    finite(&transform.matrix, "object transform")?;

    if transform.matrix.len() != 16
        || transform.matrix[3] != 0.0
        || transform.matrix[7] != 0.0
        || transform.matrix[11] != 0.0
        || transform.matrix[15] != 1.0
    {
        return Err("object placement must be a sixteen-value affine matrix".into());
    }

    Ok(())
}

/// Nothing to check.
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}

/// Every index must name a vertex.
fn mesh_indices(source: &proto::Mesh, indices: &[u64]) -> Result<(), String> {
    for index in indices {
        if !source.vertices.contains_key(index) {
            return Err(format!("mesh references missing vertex {index}"));
        }
    }

    Ok(())
}

/// Check one protobuf mesh.
pub fn mesh(source: &proto::Mesh) -> Result<(), String> {
    for (key, vertex) in &source.vertices {
        if *key > u32::MAX as u64 {
            return Err("mesh vertex key exceeds the browser index range".into());
        }

        finite(&[vertex.x, vertex.y, vertex.z], "mesh vertex")?;

        for value in vertex.attributes.values() {
            if !value.is_finite() {
                return Err("mesh vertex has a non-finite attribute".into());
            }
        }
    }

    for face in source.faces.values() {
        if face.vertices.len() < 3 {
            return Err("mesh face has fewer than three vertices".into());
        }

        mesh_indices(source, &face.vertices)?;

        for hole in &face.holes {
            mesh_indices(source, &hole.vertices)?;
        }
    }

    for (face, triangles) in &source.triangulation {
        if !source.faces.contains_key(face) || !triangles.vertices.len().is_multiple_of(3) {
            return Err("invalid mesh triangulation record".into());
        }

        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}

/// Check one protobuf curve.
pub fn curve(source: &proto::NurbsCurve) -> Result<(), String> {
    if !(2..=3).contains(&source.dimension) {
        return Err("NURBS curve dimension must be 2 or 3".into());
    }

    let count = axis(source.order, source.cv_count, &source.nurbsknots)?;
    let size = source.dimension as usize + usize::from(source.is_rational);

    if source.cvs.len() != count * size {
        return Err("NURBS curve control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS curve controls")
}

/// Check one protobuf surface.
pub fn surface(source: &proto::NurbsSurface) -> Result<(), String> {
    if source.dimension != 3 {
        return Err("NURBS surface dimension must be 3".into());
    }

    let rows = axis(source.order_u, source.cv_count_u, &source.nurbsknots_u)?;
    let columns = axis(source.order_v, source.cv_count_v, &source.nurbsknots_v)?;

    if rows.saturating_mul(columns) > 4_000_000 {
        return Err("surface exceeds four million controls".into());
    }

    let size = 3 + usize::from(source.is_rational);
    let stride_u = if source.cv_stride_u > 0 {
        source.cv_stride_u as usize
    } else {
        columns * size
    };
    let stride_v = if source.cv_stride_v > 0 {
        source.cv_stride_v as usize
    } else {
        size
    };
    let addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);

    if !matches!(addressed, Some(length) if length <= source.cvs.len()) {
        return Err("NURBS surface control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS surface controls")?;

    if let Some(cached) = &source.cached_mesh {
        mesh(cached)?;
    }

    Ok(())
}

/// Check one protobuf point cloud.
pub fn cloud(source: &proto::PointCloud) -> Result<(), String> {
    triples(&source.coords, "point cloud")?;
    finite(&source.normals, "point normals")?;
    let points = source.coords.len() / 3;

    if (!source.normals.is_empty() && source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() && source.colors.len() != points * 4)
        || (!source.point_ids.is_empty() && source.point_ids.len() != points)
    {
        return Err("point-cloud attribute counts disagree".into());
    }

    let nodes = source.lod_size.len();

    if nodes > 0 {
        if source.lod_min.len() != nodes * 3
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * 8
        {
            return Err("point-cloud octree arrays disagree".into());
        }

        finite(&source.lod_min, "octree bounds")?;
        finite(&source.lod_size, "octree sizes")?;

        for node in 0..nodes {
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64 > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }

        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }

    Ok(())
}

/// Check one protobuf BRep.
pub fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }

    for item in &source.curves_3d {
        curve(item)?;
    }

    for item in &source.surfaces {
        surface(item)?;
    }

    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }

    Ok(())
}

/// Check one protobuf element's mesh or BRep.
pub fn element(source: &proto::Element) -> Result<(), String> {
    match source.geometry_type.as_str() {
        "Mesh" => {
            mesh(&proto::Mesh::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        "BRep" => {
            brep(&proto::BRep::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        _ => Ok(()),
    }
}

/// Check every definition record, and the definitions with `instances` within the cap.
pub fn definitions(d: &proto::Objects, instances: usize) -> Result<(), String> {
    let count = d.points.len()
        + d.lines.len()
        + d.polylines.len()
        + d.meshes.len()
        + d.pointclouds.len()
        + d.nurbscurves.len()
        + d.nurbssurfaces.len()
        + d.breps.len()
        + d.elements.len();

    if count + instances > MAX_OBJECTS {
        return Err(TOO_MANY.into());
    }

    for item in &d.points {
        point(item)?;
    }

    for item in &d.lines {
        line(item)?;
    }

    for item in &d.polylines {
        polyline(item)?;
    }

    for item in &d.meshes {
        mesh(item)?;
    }

    for item in &d.pointclouds {
        cloud(item)?;
    }

    for item in &d.nurbscurves {
        curve(item)?;
    }

    for item in &d.nurbssurfaces {
        surface(item)?;
    }

    for item in &d.breps {
        brep(item)?;
    }

    for item in &d.elements {
        element(item)?;
    }

    Ok(())
}

/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.bboxes.len()+objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
        }

        for item in &objects.lines {
            line(item)?;
        }

        for item in &objects.polylines {
            polyline(item)?;
        }

        for item in &objects.meshes {
            mesh(item)?;
        }

        for item in &objects.pointclouds {
            cloud(item)?;
        }

        for item in &objects.nurbscurves {
            curve(item)?;
        }

        for item in &objects.nurbssurfaces {
            surface(item)?;
        }

        for item in &objects.breps {
            brep(item)?;
        }

        for item in &objects.elements {
            element(item)?;
        }
    }

    for entry in &source.xforms {
        xform(entry)?;
    }

    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];

        while let Some((node, depth)) = pending.pop() {
            if depth > MAX_DEPTH {
                return Err(TOO_DEEP.into());
            }

            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}

/// Check every geometry of a loaded session.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Huge or wrong counts are refused.
    #[test]
    fn hostile_counts_do_not_reach_kernel_allocations() {
        let source = proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&source).is_err());
        let source = proto::NurbsSurface {
            dimension: 3,
            order_u: 2,
            order_v: 2,
            cv_count_u: -1,
            cv_count_v: 2,
            ..Default::default()
        };
        assert!(surface(&source).is_err());
    }

    /// Bad indices and short cloud arrays are refused.
    #[test]
    fn missing_indices_and_partial_cloud_attributes_are_errors() {
        let mut source = proto::Mesh::default();
        source.faces.insert(
            0,
            proto::FaceData {
                vertices: vec![0, 1, 2],
                ..Default::default()
            },
        );
        assert!(mesh(&source).is_err());
        let cloud_source = proto::PointCloud {
            coords: vec![0., 0., 0.],
            point_ids: vec![7, 8],
            ..Default::default()
        };
        assert!(cloud(&cloud_source).is_err());
    }

    /// JSON counts must match the controls given.
    #[test]
    fn json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(r#"{"control_points":[],"cv_count":18446744073709551615,"dimension":3}"#).is_err()
        );
        assert!(
            json(r#"{"control_points":[],"cv_count_u":1000000,"cv_count_v":1000000}"#).is_err()
        );
        assert!(json(r#"{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}"#).is_ok());
    }
}
```

## <a id="checkpoint-84d-own-temporary-measurement-labels-without-scene-identities"></a>Step 5 of 37 · Own temporary measurement labels without scene identities (`84d-own-temporary-measurement-labels-without-scene-identities`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/37: 84d-own-temporary…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs, src/li…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">numeric value and points</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unused label IDs</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">temporary presentation wo…</text>
</svg>
</div>

Assign unused presentation IDs to a value chip and endpoint squares. Keep these annotations outside original scene rows and integer picking.

> **Check:** *Can a measurement chip reuse an existing label ID?* — No. Allocate from unused presentation IDs while checking existing labels. Each mark carries no original object owner.

### 1. `src/measure_mark.rs`

Own temporary points, unit suffixes and collision-free labels.

Create the file and type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-01.rs"
```

### 2. `src/lib.rs`

Register the measurement component.

After that block, type:

```rust
--8<-- "journey/code/84d-own-temporary-measurement-labels-without-scene-identities-02.rs" skip=26
```

## <a id="checkpoint-84e-report-distance-and-world-bounding-lengths"></a>Step 6 of 37 · Report distance and world bounding lengths (`84e-report-distance-and-world-bounding-lengths`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6/37: 84e-report-distan…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected bounds or two po…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">world extents and deltas</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">formatted result</text>
</svg>
</div>

Union precise selected bounds and calculate two-point distance with signed world-coordinate deltas. Preserve model coordinates when labeling units.

> **Check:** *Does changing a unit suffix rescale geometry?* — No. Units describe the existing model coordinates. Numerical conversion requires a separate geometry operation.

### 1. `src/measure_mark.rs`

Resolve world selected bounds and distance without changing source geometry.

After that block, type:

```rust
--8<-- "journey/code/84e-report-distance-and-world-bounding-lengths-01.rs" skip=20
```

## <a id="checkpoint-84f-collect-only-supported-finite-original-measurements"></a>Step 7 of 37 · Collect only supported finite original measurements (`84f-collect-only-supported-finite-original-measurements`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7/37: 84f-collect-only-…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected original owners</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source copies</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">supported totals and refu…</text>
</svg>
</div>

Transform temporary original copies and aggregate supported results. Track unsupported families and open solids separately.

> **Check:** *What happens when selection mixes curves with unsupported objects?* — Supported originals contribute to the total; the result reports skipped objects. Released editable sources require Reload.

### 1. `src/measure_mark.rs`

Collect original length, area and volume while keeping refusals explicit.

After that block, type:

```rust
--8<-- "journey/code/84f-collect-only-supported-finite-original-measurements-01.rs" skip=23
```

## <a id="checkpoint-84g-format-totals-refusals-and-three-axis-box-extents"></a>Step 8 of 37 · Format totals refusals and three-axis box extents (`84g-format-totals-refusals-and-three-axis-box-extents`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8/37: 84g-format-totals…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_mark.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite total and refusals</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current value formatting</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">temporary result chip</text>
</svg>
</div>

Format values with current rounding and unit powers. Bound open-object names and attach one temporary value chip to the selected bounds.

> **Check:** *Should one open object erase a valid closed-object volume?* — No. Report the measured closed total and separately name the refused open objects; an entirely unsupported selection returns an error.

### 1. `src/measure_mark.rs`

Report current formatted totals and world bounding lengths with bounded errors.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84g-format-totals-refusals-and-three-axis-box-extents-01.rs"
```

## <a id="checkpoint-84h-keep-measurements-outside-geometry-history"></a>Step 9 of 37 · Keep measurements outside geometry history (`84h-keep-measurements-outside-geometry-history`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9/37: 84h-keep-measurem…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected whole owner or f…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measurement or original b…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unchanged or single histo…</text>
</svg>
</div>

Connect selected-parent measurement and checked box creation. Distance results change presentation; original bounding boxes use one scene transaction.

> **Check:** *Does measuring a selected face use whole-object selection?* — No. Its selected parent supplies original geometry while the face index limits Area. A whole-object measure uses normal eligible selection.

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

**Complete updated block reference:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),RotateWorld{axis:crate::gizmo::Axis,degrees:f64},
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.split.is_some()||self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()&&d.plane_points.is_none()){return self.project_points(&[point])}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return Err("Extend uses curve-end clicks")}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_point(point);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn begin_gather(&mut self,recipe:&'static crate::gather_input::Recipe,text:&str)->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let next=crate::gather_draft::Draft::new(recipe,text,&self.scene,self.selected_objects(),frame).map_err(|message|self.primitive_failure(message))?;let made=next.build().map_err(|message|self.primitive_failure(message))?;
        if made.is_some(){return self.gather_adopt(next,made)}
        self.apply(Action::BeginTool(crate::tool::Shape::Gather(recipe)))?;self.select_objects([],false);self.gather_adopt(next,made)
    }
    fn gather_point(&mut self,point:[f64;3])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.point(point).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.text(text,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    pub fn editing_boundaries(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.extend.as_ref().is_some_and(|e|!e.curves))}
    fn commit_project(&mut self,project:crate::project_draft::Draft,plane:session_rust::Plane)->Result<Change,&'static str>{let(edits,message)=project.prepare(&self.scene,&plane).map_err(|message|self.primitive_failure(message))?;self.history.try_edit(&mut self.scene,|scene|{for(id,geometry)in edits{scene.replace_original(id,geometry)?}Ok(())})?;self.select_objects(project.before,false);self.draft=None;self.tool_status=message;Ok(Change::Scene)}
    fn begin_project(&mut self,words:&[String])->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let plan=crate::project_geometry::parse("Project To Plane",&words.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|message|self.primitive_failure(message))?;let mut project=crate::project_draft::Draft::new(&self.scene,self.selected_objects())?;
        if plan.plane=="3Point"{let plane=project.accept(&plan.points).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(project,plane)}self.apply(Action::BeginTool(crate::tool::Shape::Project))?;self.draft.as_mut().unwrap().project=Some(project);self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View)}
        let frame=match plan.plane{"XY"=>crate::cplane::CPlane::Xy,"YZ"=>crate::cplane::CPlane::Yz,"ZX"=>crate::cplane::CPlane::Xz,_=>crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis()))};let(x,y)=frame.axes();self.commit_project(project,session_rust::Plane::from_point_normal(session_rust::Point::new(0.,0.,0.),x.cross(&y),None))
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn begin_extend(&mut self,distance:Option<f64>,asking:bool)->Result<Change,&'static str>{let next=crate::extend_draft::Draft::new(&self.scene,self.selected_objects(),distance,asking).map_err(|message|self.primitive_failure(message))?;self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;self.select_objects([],false);self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let finished=next.text(&self.scene,text).map_err(|message|self.primitive_failure(message))?;if finished{self.select_objects(next.before,false);self.draft=None;self.tool_status="Extend done".into();return Ok(Change::View)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let view=crate::snap_math::Screen{matrix:self.camera.view_projection().m,size};next.follow(&self.scene,&view,[(screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.]).map_err(|message|self.primitive_failure(message))?;
        if commit{let hover=next.hover.clone().ok_or_else(||self.primitive_failure(if next.error.is_empty(){"Click near a curve end".into()}else{next.error.clone()}))?;hover.owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(hover.owner.id,hover.geometry))?;let added=hover.tail.windows(2).map(|p|(0..3).map(|i|(p[1][i]-p[0][i]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();next.refreshed(&self.scene,hover.owner.id).map_err(|message|self.primitive_failure(message))?;self.tool_status=format!("Extended by {added:.3} · Undo reverts it");self.draft.as_mut().unwrap().extend=Some(next);return Ok(Change::Scene)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)
    }
    fn edit_interval(&mut self,interval:crate::interval_geometry::Interval)->Result<Change,&'static str>{let ids=self.selected_objects();let[id]=ids.as_slice()else{return Err("Select one original curve")};let owner=crate::source_edit::Owner::capture(&self.scene,*id)?;let geometry=crate::interval_geometry::edited(&owner.source,interval).map_err(|message|self.primitive_failure(message))?;owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(*id,geometry))?;self.draft=None;self.tool_status="Geometry updated".into();Ok(Change::Scene)}
    fn split_command(&mut self)->Result<Change,&'static str>{
        if self.split.as_ref().is_some_and(|s|s.cutters.is_empty()){return Err("Select at least one cutter curve, then press Enter")}
        if let Some(next)=self.split.take(){let(pieces,regions)=next.prepare(&self.scene).map_err(|message|self.primitive_failure(message))?;if regions<2{self.tool_status="No division: cutters must cross the curve or lie on the selected face".into();return Ok(Change::View)}self.history.try_edit(&mut self.scene,|scene|crate::split_draft::adopt(scene,next.target.id,pieces))?;self.select_objects([next.target.id],false);self.tool_status=format!("Split into {regions} regions · cutters retained · Undo restores the original");return Ok(Change::Scene)}
        let id=self.selection().map(|t|t.owner).or(self.selected).ok_or("Select a curve or one surface face, then run Split")?;
        let face=match self.selected_part{Some(crate::pick_sources::Part::Face(face))=>Some(face as usize),_=>None};
        let next=crate::split_draft::Draft::new(&self.scene,id,face).map_err(|message|self.primitive_failure(message))?;
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
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return self.project_points(&added)}
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        if let Some(gather)=self.draft.as_ref()?.gather.as_ref(){if self.draft.as_ref()?.plane_points.is_none(){match gather.current(){Some(crate::gather_input::Step::Curves{..}|crate::gather_input::Step::Number{..})=>return None,Some(crate::gather_input::Step::Distance(_))=>return gather.on_axis(&self.camera,screen).map(|point|crate::snap::Hit{point,kind:crate::snap::Kind::Near,owner:None}),_=>{}}}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.project.is_some()||d.extend.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.gather.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
            Action::Project(words)=>return self.begin_project(&words),
            Action::ExtendBegin{distance,asking}=>return self.begin_extend(distance,asking),
            Action::ExtendPick(id)=>return self.extend_pick(id),
            Action::Interval(interval)=>return self.edit_interval(interval),
            Action::GatherBegin{recipe,text}=>return self.begin_gather(recipe,&text),
            Action::GatherPick(id)=>return self.gather_pick(id),
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                if let Some(gather)=self.draft.as_mut().and_then(|d|d.gather.as_mut()){gather.frame=frame;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.gather.is_some()||draft.project.is_some()||draft.extend.is_some(){return Err("Close works on a polyline or curve")}
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{if self.draft.as_ref().is_some_and(|d|d.extend.is_some()){return self.extend_text("")}if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return Err("Project 3Point needs three plane points")}if self.draft.as_ref().is_some_and(|d|d.gather.is_some()){return self.gather_text("")}crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_screen(screen,size,commit)}
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

## <a id="checkpoint-84i-accept-two-distance-points-through-the-drawing-tool"></a>Step 10 of 37 · Accept two distance points through the drawing tool (`84i-accept-two-distance-points-through-the-drawing-tool`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10/37: 84i-accept-two-di…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tool.rs, src/editor.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">drawing-plane or world po…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing snapping tool</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">temporary measured pair</text>
</svg>
</div>

Use the existing point parser and snapping owner for distance input. Validate initial arguments before replacing a pending tool and finish without inserting geometry.

> **Check:** *Should invalid initial distance input discard an accepted drawing point?* — No. Validate it before switching owners. Once started, distance enables object snapping and accepts exactly two points.

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

**Complete updated block reference:**

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Measured([[f64;3];2]), Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]), Polyline(crate::chain::PreparedChain), Geometry(session_rust::Geometry) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Result<Built,&'static str> {self.build(points)}
    fn outline(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Vec<[f64;3]> {points.to_vec()}
    fn sides(&self)->Option<usize> {None}
    fn set_sides(&mut self,_sides:usize)->Result<(),&'static str> {Err("Sides works while drawing a polygon")}
    fn buttons(&self) -> Vec<(String, String)> { vec![("Cancel".into(), "Escape".into())] }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Measure, Project,Extend, Gather(&'static crate::gather_input::Recipe), Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape), Transform(crate::transform_draft::Kind) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Measure=>"Measure Distance",Self::Project=>"Project To Plane",Self::Extend=>"Extend",Self::Gather(recipe)=>recipe.name, Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name,Self::Transform(kind)=>kind.name() } }
    fn count(&self) -> usize { match self { Self::Project|Self::Extend=>0,Self::Gather(_)=>0, Self::Point => 1, Self::Line | Self::Move | Self::Measure => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)|Self::Transform(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
        let Self::Polygon {sides}=self else {return Err("Sides works while drawing a polygon")};*sides=value;Ok(())
    }
    fn build_on_plane(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Result<Built,&'static str> {
        let (kind,sides)=match self {Self::Rectangle=>("rectangle",4),Self::Polygon {sides}=>("polygon",*sides),_=>return self.build(points)};
        let points=crate::planar::points(kind,plane,sides,points)?;Self::Polyline.build(&points)
    }
    fn outline(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Vec<[f64;3]> {
        match self {Self::Rectangle if points.len()==2=>crate::planar::points("rectangle",plane,4,points).unwrap_or_default(),
            Self::Polygon {sides} if points.len()==2=>crate::planar::points("polygon",plane,*sides,points).unwrap_or_default(),_=>points.to_vec()}
    }
    fn buttons(&self)->Vec<(String,String)> {
        if matches!(self,Self::Project){return vec![("Cancel".into(),"Escape".into())]}
        if matches!(self,Self::Extend){return vec![("Distance".into(),"Distance".into()),("Next / Done".into(),"".into()),("Cancel".into(),"Escape".into())]}
        if let Self::Gather(recipe)=self{return recipe.chips.iter().map(|(label,line)|((*label).into(),(*line).into())).collect()}
        if let Self::Transform(kind)=self {return crate::transform_draft::Draft::new(*kind,crate::cplane::CPlane::Xy).buttons();}
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
        let mut buttons=vec![("Cancel".into(),"Escape".into())];
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline|Self::Curve) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Measure,[a,b])=>Ok(Built::Measured([*a,*b])),
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
            (Self::Line, [a, b]) => {
                if a.iter().zip(b).all(|(a, b)| (a-b).abs() < 1e-12) { return Err("Line endpoints must differ"); }
                Ok(Built::Line(PreparedLine::new(Rc::new(session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2])))?))
            }
            (Self::Move, [a,b]) => Ok(Built::Move(std::array::from_fn(|i| b[i]-a[i]))),
            (Self::Polyline, points) if (2..=MAX_POINTS).contains(&points.len()) => {
                if !points.windows(2).any(|p| p[0] != p[1]) { return Err("Polyline points must differ"); }
                let points=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                Ok(Built::Polyline(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points)))?))
            }
            (Self::Curve,points) if (2..=MAX_POINTS).contains(&points.len())=>{
                if !points.windows(2).any(|p|p[0]!=p[1]) {return Err("Curve controls must differ");}
                let controls=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect::<Vec<_>>();
                let source=crate::freeform_curve::curve(&controls);
                Ok(Built::Geometry(session_rust::Geometry::NurbsCurve(Rc::new(source))))
            },
            (Self::Rectangle|Self::Polygon {..},_)=>self.build_on_plane(points,crate::cplane::CPlane::Xy),
            _ => Err("The tool needs more points"),
        }
    }
}

pub const MAX_POINTS: usize = 4096;

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft>, pub transform:Option<crate::transform_draft::Draft>,pub gather:Option<crate::gather_draft::Draft>,pub project:Option<crate::project_draft::Draft>,pub extend:Option<crate::extend_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { project:None,extend:None,gather:None,tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None},transform:if let Shape::Transform(kind)=shape {Some(crate::transform_draft::Draft::new(kind,crate::cplane::CPlane::Xy))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none(){if let Some(p)=&self.project{return p.prompt()}if let Some(e)=&self.extend{return e.prompt()}}
        if self.plane_points.is_none(){if let Some(gather)=&self.gather{return gather.prompt()}}
        if self.plane_points.is_none() {if let Some(transform)=&self.transform {return transform.prompt();}}
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
            else if self.tool.count()==0 {format!("{} point {} · Enter finishes · Esc cancels",self.tool.name(),self.points.len()+1)}
            else {format!("{} point {} of {} · Esc cancels", self.tool.name(), self.points.len()+1, self.tool.count())};
        if let Some(sides)=self.tool.sides() {prompt.push_str(&format!(" · Sides {sides}"));}
        if let Some(kind)=self.snapped { prompt.push_str(&format!(" · Snap {kind:?}")); }
        prompt
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> { self.submit_many(&[point]) }
    pub fn submit_many(&mut self, added: &[[f64;3]]) -> Result<Option<Built>, &'static str> {
        let mut points=self.points.clone();
        for &point in added { points.push(crate::tool_coords::checked(point)?); }
        let count=self.tool.count(); let limit=if count==0 {MAX_POINTS} else {count};
        if points.len()>limit {return Err("The tool has enough points");}
        let built=if count>0 && points.len()==count {Some(self.build_world(&points)?)} else {None};
        self.points=points;self.cursor=None;self.snapped=None;Ok(built)
    }
    pub fn finish(&self) -> Result<Built,&'static str> { self.build_world(&self.points) }
    // Finished coordinates already belong to the world; their prefix prevents applying a fixed plane a second time during replay.
    pub fn command(&self, points:&[[f64;3]]) -> String {
        let mut line=self.tool.name().to_owned();
        for p in points {line.push_str(&format!(" w{},{},{}",p[0],p[1],p[2]));}line
    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=&command[self.tool.name().len()..];
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build_on_plane(&world,self.frame)
    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(points)=&self.plane_points{let mut points=points.clone();if let Some(cursor)=self.cursor{points.push(cursor)}return vec![points]}
        if let Some(extend)=&self.extend{return extend.hover.as_ref().map_or_else(Vec::new,|h|vec![h.tail.clone()])}
        if let Some(gather)=&self.gather {gather.outlines(self.cursor)}else if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
    pub fn outline(&self)->Vec<[f64;3]> {
        let mut points=self.points.clone();if let Some(cursor)=self.cursor {points.push(cursor);}
        self.tool.outline(&points,self.frame)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
        self.cursor = point.map(crate::tool_coords::checked).transpose()?; Ok(())
    }
}
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

**Complete updated block reference:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),RotateWorld{axis:crate::gizmo::Axis,degrees:f64},
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.split.is_some()||self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()&&d.plane_points.is_none()){return self.project_points(&[point])}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return Err("Extend uses curve-end clicks")}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_point(point);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        if let crate::tool::Built::Measured(points)=built{return self.distance_mark(points)}
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Measured(_)=>unreachable!("Measurement changes no original geometry"),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn begin_gather(&mut self,recipe:&'static crate::gather_input::Recipe,text:&str)->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let next=crate::gather_draft::Draft::new(recipe,text,&self.scene,self.selected_objects(),frame).map_err(|message|self.primitive_failure(message))?;let made=next.build().map_err(|message|self.primitive_failure(message))?;
        if made.is_some(){return self.gather_adopt(next,made)}
        self.apply(Action::BeginTool(crate::tool::Shape::Gather(recipe)))?;self.select_objects([],false);self.gather_adopt(next,made)
    }
    fn gather_point(&mut self,point:[f64;3])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.point(point).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.text(text,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    pub fn editing_boundaries(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.extend.as_ref().is_some_and(|e|!e.curves))}
    fn commit_project(&mut self,project:crate::project_draft::Draft,plane:session_rust::Plane)->Result<Change,&'static str>{let(edits,message)=project.prepare(&self.scene,&plane).map_err(|message|self.primitive_failure(message))?;self.history.try_edit(&mut self.scene,|scene|{for(id,geometry)in edits{scene.replace_original(id,geometry)?}Ok(())})?;self.select_objects(project.before,false);self.draft=None;self.tool_status=message;Ok(Change::Scene)}
    fn begin_project(&mut self,words:&[String])->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let plan=crate::project_geometry::parse("Project To Plane",&words.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|message|self.primitive_failure(message))?;let mut project=crate::project_draft::Draft::new(&self.scene,self.selected_objects())?;
        if plan.plane=="3Point"{let plane=project.accept(&plan.points).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(project,plane)}self.apply(Action::BeginTool(crate::tool::Shape::Project))?;self.draft.as_mut().unwrap().project=Some(project);self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View)}
        let frame=match plan.plane{"XY"=>crate::cplane::CPlane::Xy,"YZ"=>crate::cplane::CPlane::Yz,"ZX"=>crate::cplane::CPlane::Xz,_=>crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis()))};let(x,y)=frame.axes();self.commit_project(project,session_rust::Plane::from_point_normal(session_rust::Point::new(0.,0.,0.),x.cross(&y),None))
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn begin_extend(&mut self,distance:Option<f64>,asking:bool)->Result<Change,&'static str>{let next=crate::extend_draft::Draft::new(&self.scene,self.selected_objects(),distance,asking).map_err(|message|self.primitive_failure(message))?;self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;self.select_objects([],false);self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let finished=next.text(&self.scene,text).map_err(|message|self.primitive_failure(message))?;if finished{self.select_objects(next.before,false);self.draft=None;self.tool_status="Extend done".into();return Ok(Change::View)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let view=crate::snap_math::Screen{matrix:self.camera.view_projection().m,size};next.follow(&self.scene,&view,[(screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.]).map_err(|message|self.primitive_failure(message))?;
        if commit{let hover=next.hover.clone().ok_or_else(||self.primitive_failure(if next.error.is_empty(){"Click near a curve end".into()}else{next.error.clone()}))?;hover.owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(hover.owner.id,hover.geometry))?;let added=hover.tail.windows(2).map(|p|(0..3).map(|i|(p[1][i]-p[0][i]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();next.refreshed(&self.scene,hover.owner.id).map_err(|message|self.primitive_failure(message))?;self.tool_status=format!("Extended by {added:.3} · Undo reverts it");self.draft.as_mut().unwrap().extend=Some(next);return Ok(Change::Scene)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)
    }
    fn edit_interval(&mut self,interval:crate::interval_geometry::Interval)->Result<Change,&'static str>{let ids=self.selected_objects();let[id]=ids.as_slice()else{return Err("Select one original curve")};let owner=crate::source_edit::Owner::capture(&self.scene,*id)?;let geometry=crate::interval_geometry::edited(&owner.source,interval).map_err(|message|self.primitive_failure(message))?;owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(*id,geometry))?;self.draft=None;self.tool_status="Geometry updated".into();Ok(Change::Scene)}
    fn split_command(&mut self)->Result<Change,&'static str>{
        if self.split.as_ref().is_some_and(|s|s.cutters.is_empty()){return Err("Select at least one cutter curve, then press Enter")}
        if let Some(next)=self.split.take(){let(pieces,regions)=next.prepare(&self.scene).map_err(|message|self.primitive_failure(message))?;if regions<2{self.tool_status="No division: cutters must cross the curve or lie on the selected face".into();return Ok(Change::View)}self.history.try_edit(&mut self.scene,|scene|crate::split_draft::adopt(scene,next.target.id,pieces))?;self.select_objects([next.target.id],false);self.tool_status=format!("Split into {regions} regions · cutters retained · Undo restores the original");return Ok(Change::Scene)}
        let id=self.selection().map(|t|t.owner).or(self.selected).ok_or("Select a curve or one surface face, then run Split")?;
        let face=match self.selected_part{Some(crate::pick_sources::Part::Face(face))=>Some(face as usize),_=>None};
        let next=crate::split_draft::Draft::new(&self.scene,id,face).map_err(|message|self.primitive_failure(message))?;
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return self.project_points(&added)}
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        if let Some(gather)=self.draft.as_ref()?.gather.as_ref(){if self.draft.as_ref()?.plane_points.is_none(){match gather.current(){Some(crate::gather_input::Step::Curves{..}|crate::gather_input::Step::Number{..})=>return None,Some(crate::gather_input::Step::Distance(_))=>return gather.on_axis(&self.camera,screen).map(|point|crate::snap::Hit{point,kind:crate::snap::Kind::Near,owner:None}),_=>{}}}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.project.is_some()||d.extend.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.gather.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
            Action::Project(words)=>return self.begin_project(&words),
            Action::ExtendBegin{distance,asking}=>return self.begin_extend(distance,asking),
            Action::ExtendPick(id)=>return self.extend_pick(id),
            Action::Interval(interval)=>return self.edit_interval(interval),
            Action::GatherBegin{recipe,text}=>return self.begin_gather(recipe,&text),
            Action::GatherPick(id)=>return self.gather_pick(id),
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                if let Some(gather)=self.draft.as_mut().and_then(|d|d.gather.as_mut()){gather.frame=frame;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.gather.is_some()||draft.project.is_some()||draft.extend.is_some(){return Err("Close works on a polyline or curve")}
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{if self.draft.as_ref().is_some_and(|d|d.extend.is_some()){return self.extend_text("")}if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return Err("Project 3Point needs three plane points")}if self.draft.as_ref().is_some_and(|d|d.gather.is_some()){return self.gather_text("")}crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_screen(screen,size,commit)}
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

## <a id="checkpoint-84j-register-measurements-and-end-temporary-marks-predictably"></a>Step 11 of 37 · Register measurements and end temporary marks predictably (`84j-register-measurements-and-end-temporary-marks-predictably`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11/37: 84j-register-meas…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/commands…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current command vocabulary</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked measurement action</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">explicit mark lifecycle</text>
</svg>
</div>

Register current measurement verbs and box commands. Preserve marks during pointer views; clear them on the next typed command or Escape.

> **Check:** *Can orbiting the scene remove a completed distance annotation?* — No. Pointer view controls retain it. Another typed command or Escape removes the temporary annotation without editing sources.

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

**Complete updated block reference:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),RotateWorld{axis:crate::gizmo::Axis,degrees:f64},
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub mark:Option<crate::measure_mark::Mark>,pub unit:crate::measure_mark::Unit,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mark:None,unit:crate::measure_mark::Unit::Millimeters,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.split.is_some()||self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()&&d.plane_points.is_none()){return self.project_points(&[point])}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return Err("Extend uses curve-end clicks")}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_point(point);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        if let crate::tool::Built::Measured(points)=built{return self.distance_mark(points)}
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Measured(_)=>unreachable!("Measurement changes no original geometry"),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn begin_gather(&mut self,recipe:&'static crate::gather_input::Recipe,text:&str)->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let next=crate::gather_draft::Draft::new(recipe,text,&self.scene,self.selected_objects(),frame).map_err(|message|self.primitive_failure(message))?;let made=next.build().map_err(|message|self.primitive_failure(message))?;
        if made.is_some(){return self.gather_adopt(next,made)}
        self.apply(Action::BeginTool(crate::tool::Shape::Gather(recipe)))?;self.select_objects([],false);self.gather_adopt(next,made)
    }
    fn gather_point(&mut self,point:[f64;3])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.point(point).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.text(text,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    pub fn editing_boundaries(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.extend.as_ref().is_some_and(|e|!e.curves))}
    fn commit_project(&mut self,project:crate::project_draft::Draft,plane:session_rust::Plane)->Result<Change,&'static str>{let(edits,message)=project.prepare(&self.scene,&plane).map_err(|message|self.primitive_failure(message))?;self.history.try_edit(&mut self.scene,|scene|{for(id,geometry)in edits{scene.replace_original(id,geometry)?}Ok(())})?;self.select_objects(project.before,false);self.draft=None;self.tool_status=message;Ok(Change::Scene)}
    fn begin_project(&mut self,words:&[String])->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let plan=crate::project_geometry::parse("Project To Plane",&words.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|message|self.primitive_failure(message))?;let mut project=crate::project_draft::Draft::new(&self.scene,self.selected_objects())?;
        if plan.plane=="3Point"{let plane=project.accept(&plan.points).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(project,plane)}self.apply(Action::BeginTool(crate::tool::Shape::Project))?;self.draft.as_mut().unwrap().project=Some(project);self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View)}
        let frame=match plan.plane{"XY"=>crate::cplane::CPlane::Xy,"YZ"=>crate::cplane::CPlane::Yz,"ZX"=>crate::cplane::CPlane::Xz,_=>crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis()))};let(x,y)=frame.axes();self.commit_project(project,session_rust::Plane::from_point_normal(session_rust::Point::new(0.,0.,0.),x.cross(&y),None))
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn begin_extend(&mut self,distance:Option<f64>,asking:bool)->Result<Change,&'static str>{let next=crate::extend_draft::Draft::new(&self.scene,self.selected_objects(),distance,asking).map_err(|message|self.primitive_failure(message))?;self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;self.select_objects([],false);self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let finished=next.text(&self.scene,text).map_err(|message|self.primitive_failure(message))?;if finished{self.select_objects(next.before,false);self.draft=None;self.tool_status="Extend done".into();return Ok(Change::View)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let view=crate::snap_math::Screen{matrix:self.camera.view_projection().m,size};next.follow(&self.scene,&view,[(screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.]).map_err(|message|self.primitive_failure(message))?;
        if commit{let hover=next.hover.clone().ok_or_else(||self.primitive_failure(if next.error.is_empty(){"Click near a curve end".into()}else{next.error.clone()}))?;hover.owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(hover.owner.id,hover.geometry))?;let added=hover.tail.windows(2).map(|p|(0..3).map(|i|(p[1][i]-p[0][i]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();next.refreshed(&self.scene,hover.owner.id).map_err(|message|self.primitive_failure(message))?;self.tool_status=format!("Extended by {added:.3} · Undo reverts it");self.draft.as_mut().unwrap().extend=Some(next);return Ok(Change::Scene)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)
    }
    fn edit_interval(&mut self,interval:crate::interval_geometry::Interval)->Result<Change,&'static str>{let ids=self.selected_objects();let[id]=ids.as_slice()else{return Err("Select one original curve")};let owner=crate::source_edit::Owner::capture(&self.scene,*id)?;let geometry=crate::interval_geometry::edited(&owner.source,interval).map_err(|message|self.primitive_failure(message))?;owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(*id,geometry))?;self.draft=None;self.tool_status="Geometry updated".into();Ok(Change::Scene)}
    fn split_command(&mut self)->Result<Change,&'static str>{
        if self.split.as_ref().is_some_and(|s|s.cutters.is_empty()){return Err("Select at least one cutter curve, then press Enter")}
        if let Some(next)=self.split.take(){let(pieces,regions)=next.prepare(&self.scene).map_err(|message|self.primitive_failure(message))?;if regions<2{self.tool_status="No division: cutters must cross the curve or lie on the selected face".into();return Ok(Change::View)}self.history.try_edit(&mut self.scene,|scene|crate::split_draft::adopt(scene,next.target.id,pieces))?;self.select_objects([next.target.id],false);self.tool_status=format!("Split into {regions} regions · cutters retained · Undo restores the original");return Ok(Change::Scene)}
        let id=self.selection().map(|t|t.owner).or(self.selected).ok_or("Select a curve or one surface face, then run Split")?;
        let face=match self.selected_part{Some(crate::pick_sources::Part::Face(face))=>Some(face as usize),_=>None};
        let next=crate::split_draft::Draft::new(&self.scene,id,face).map_err(|message|self.primitive_failure(message))?;
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return self.project_points(&added)}
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        if let Some(gather)=self.draft.as_ref()?.gather.as_ref(){if self.draft.as_ref()?.plane_points.is_none(){match gather.current(){Some(crate::gather_input::Step::Curves{..}|crate::gather_input::Step::Number{..})=>return None,Some(crate::gather_input::Step::Distance(_))=>return gather.on_axis(&self.camera,screen).map(|point|crate::snap::Hit{point,kind:crate::snap::Kind::Near,owner:None}),_=>{}}}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Isometric|Action::Background|Action::NormalView(_)|Action::Grid(_)){self.mark=None;}
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.project.is_some()||d.extend.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.gather.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
            Action::Project(words)=>return self.begin_project(&words),
            Action::ExtendBegin{distance,asking}=>return self.begin_extend(distance,asking),
            Action::ExtendPick(id)=>return self.extend_pick(id),
            Action::Interval(interval)=>return self.edit_interval(interval),
            Action::GatherBegin{recipe,text}=>return self.begin_gather(recipe,&text),
            Action::GatherPick(id)=>return self.gather_pick(id),
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                if let Some(gather)=self.draft.as_mut().and_then(|d|d.gather.as_mut()){gather.frame=frame;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.gather.is_some()||draft.project.is_some()||draft.extend.is_some(){return Err("Close works on a polyline or curve")}
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{if self.draft.as_ref().is_some_and(|d|d.extend.is_some()){return self.extend_text("")}if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return Err("Project 3Point needs three plane points")}if self.draft.as_ref().is_some_and(|d|d.gather.is_some()){return self.gather_text("")}crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_screen(screen,size,commit)}
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
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

**Complete updated block reference:**

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { TextDemo, Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Snap", || Action::Snap(crate::snap::Setting::Objects(None))),
    ("Snap On", || Action::Snap(crate::snap::Setting::Objects(Some(true)))),
    ("Snap Off", || Action::Snap(crate::snap::Setting::Objects(Some(false)))),
    ("Grid Snap", || Action::Snap(crate::snap::Setting::Grid(None))),
    ("Grid Snap On", || Action::Snap(crate::snap::Setting::Grid(Some(true)))),
    ("Grid Snap Off", || Action::Snap(crate::snap::Setting::Grid(Some(false)))),
    ("Snap End", || Action::Snap(crate::snap::Setting::Mode(crate::snap::END))),
    ("Snap Near", || Action::Snap(crate::snap::Setting::Mode(crate::snap::NEAR))),
    ("Snap Mid", || Action::Snap(crate::snap::Setting::Mode(crate::snap::MID))),
    ("Snap Center", || Action::Snap(crate::snap::Setting::Mode(crate::snap::CENTER))),
    ("Snap Perp", || Action::Snap(crate::snap::Setting::Mode(crate::snap::PERP))),
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
    ("Polyline Rectangle",||Action::BeginTool(crate::tool::Shape::Rectangle)),
    ("Polyline Polygon",||Action::BeginTool(crate::tool::Shape::Polygon {sides:6})),
    ("CPlane",||Action::PlaneMenu),
    ("Hide",||Action::Hide),
    ("Show",||Action::Show),
    ("Example Box", || Action::AddBox),
    ("Example Cloud LOD",||Action::AddCloudLod),
    ("Example Cloud",||Action::AddCloud),
    ("Example CAD",||Action::AddCad),
    ("Example Instances",||Action::AddInstances),
    ("Example Shape Reuse",||Action::AddShapeReuse),
    ("Example Nested Instances",||Action::AddNestedInstances),
    ("Example Sheet",||Action::AddSheet),
    ("Example Scene Text",||Action::AddText(crate::text::TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})),
    ("Example Billboard",||Action::AddText(crate::text::TextPlacement::WorldBillboard {world:[-1.5,0.,1.],world_height:0.22})),
    ("Example Anchor",||Action::AddText(crate::text::TextPlacement::Anchor {world:[0.,0.,1.],offset:[8.,-12.]})),
    ("Example Nameplate",||Action::AddText(crate::text::TextPlacement::Nameplate {world:[0.,0.,1.],padding:[17.,4.],rounded:true})),
    ("Example Surface",||Action::AddSurface),
    ("Example Nested",||Action::AddNested),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
    ("Example Curve", || Action::AddCurve),
    ("Example Object Color", || Action::AddColours(crate::colour::Mode::Object)),
    ("Example Point Colors", || Action::AddColours(crate::colour::Mode::Points)),
    ("Example Face Colors", || Action::AddColours(crate::colour::Mode::Faces)),
    ("View Normals On", || Action::NormalView(true)),
    ("View Normals Off", || Action::NormalView(false)),
    ("View Grid On", || Action::Grid(Some(crate::grid::Settings::default()))),
    ("View Grid Off", || Action::Grid(None)),
    ("Example Sharp", || Action::AddNormals(crate::normal_example::Mode::Sharp)),
    ("Example Smooth", || Action::AddNormals(crate::normal_example::Mode::Smooth)),
    ("Example Winding", || Action::AddNormals(crate::normal_example::Mode::Winding)),
    ("Controls", || Action::Controls(true)),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
    ("Example Polyline", || Action::AddPolyline),
    ("Example Arrow", || Action::AddArrow),
    ("Example Triangle", || Action::ToggleExtra),
    ("Select Next", || Action::SelectNext),
    ("Select All", || Action::SelectAll),
    ("Select None", || Action::Picked(None)),
    ("Select Object", || Action::PickMode(crate::pick_sources::Mode::Object)),
    ("Select Face", || Action::PickMode(crate::pick_sources::Mode::Face)),
    ("Select Edge", || Action::PickMode(crate::pick_sources::Mode::Edge)),
    ("Select Control", || Action::PickMode(crate::pick_sources::Mode::Control)),
    ("Delete", || Action::Delete),
    ("Undo", || Action::Undo),
    ("Redo", || Action::Redo),
    ("Background", || Action::Background),
    ("Zoom In", || Action::Zoom(2.0)),
    ("Zoom Out", || Action::Zoom(0.5)),
    ("Pan Left", || Action::Pan(-0.25, 0.0)),
    ("Pan Right", || Action::Pan(0.25, 0.0)),
    ("Orbit Right", || Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)),
    ("Orbit Up", || Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)),
    ("View Isometric", || Action::Isometric),
    ("Fit", || Action::Fit),
    ("Fit Selected", || Action::FitSelected),
    ("View Reset", || Action::ResetView),
    ("View Perspective", || Action::Projection(crate::camera::Projection::Perspective)),
    ("View Orthographic", || Action::Projection(crate::camera::Projection::Orthographic)),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
];

pub const HOSTS: &[(&str, Host)] = &[
    ("Layers On",Host::LayersOn),("Layers Off",Host::LayersOff),("Layers All",Host::LayersAll),
    ("Help", Host::Help),
    ("Open", Host::Open),
    ("Open Replace", Host::Replace),
    ("Cancel Open", Host::CancelOpen),
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text",Host::TextDemo),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).chain(crate::transform_commands::NAMES.iter().copied()).chain(crate::editing_commands::NAMES.iter().copied()).chain(["Split"]).chain(crate::gather_registry::ENTRIES.iter().map(|(name,_)|*name)).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names().into_iter().filter_map(|name| {
        let compact = name.split_whitespace().collect::<String>().to_ascii_lowercase();
        let mut rest = compact.as_str();
        for (index, word) in words.iter().enumerate() {
            let word = word.to_ascii_lowercase();
            rest = rest.strip_prefix(&word)?;
            if rest.is_empty() { return Some((name, index + 1, compact.len())); }
        }
        None
    }).max_by_key(|(_, _, letters)| *letters).map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
    })
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name=="Measure Distance"{if words.len()>count+2{return Err("Measure Distance takes two points".into())}return Ok(Command::Editor(Action::MeasureDistance(words[count..].join(" "))))}
    if name=="Split"{return if words.len()==count{Ok(Command::Editor(Action::Split))}else{Err("Split takes no arguments".into())};}
    if let Some(action)=crate::editing_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
        let shape=match name {"Point"=>crate::tool::Shape::Point,"Curve"=>crate::tool::Shape::Curve,"Line"=>crate::tool::Shape::Line,"Polyline Rectangle"=>crate::tool::Shape::Rectangle,"Polyline Polygon"=>crate::tool::Shape::Polygon {sides:6},_=>crate::tool::Shape::Polyline};
        return Ok(Command::Editor(Action::Draw {shape,text:words[count..].join(" ")}));
    }
    if name == "Move" && words.len()>count {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
            .map(|offset| Command::Editor(Action::Translate(offset))).map_err(str::to_owned);
    }
    if name == "Grid Snap" && words.len() > count {
        if words.len() != count+1 { return Err("Grid Snap (On Off) or a spacing".into()); }
        let step = words[count].parse::<f64>().map_err(|_| "Grid Snap (On Off) or a spacing")?;
        if !step.is_finite() || step<=0. { return Err("The grid spacing must be above zero · Example: Grid Snap 50".into()); }
        return Ok(Command::Editor(Action::Snap(crate::snap::Setting::Spacing(step))));
    }
    if words.len() != count { return Err(format!("{name} takes no arguments")); }
    if let Some((_, make)) = ACTIONS.iter().find(|(key, _)| *key == name) { return Ok(Command::Editor(make())); }
    let (_, host) = HOSTS.iter().find(|(key, _)| *key == name).ok_or("Command registry is incomplete")?;
    Ok(Command::Host(*host))
}
```

## <a id="checkpoint-84k-draw-cached-unpickable-measurement-strokes"></a>Step 12 of 37 · Draw cached unpickable measurement strokes (`84k-draw-cached-unpickable-measurement-strokes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12/37: 84k-draw-cached-u…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/meas…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary point pair</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached CSS-width stroke</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">overlay colour pass</text>
</svg>
</div>

Create a depth-independent overlay lane for temporary measurement segments. Reuse unchanged stroke allocation and keep it outside integer picks.

> **Check:** *Does the measurement line acquire a geometry pick address?* — No. This overlay draws colour only. Its endpoints and value chip remain presentation data.

### 1. `src/stroke_gpu.rs`

Add a colour-only overlay lane with explicit depth behaviour.

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

After that block, type:

```rust
--8<-- "journey/code/84k-draw-cached-unpickable-measurement-strokes-03.rs" skip=23
```

## <a id="checkpoint-84l-merge-measurement-chips-with-cached-scene-text"></a>Step 13 of 37 · Merge measurement chips with cached scene text (`84l-merge-measurement-chips-with-cached-scene-text`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13/37: 84l-merge-measure…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged mark guard</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stroke overlay</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">value chip</text>
</svg>
</div>

Merge collision-free measurement words into the existing text lane and draw temporary strokes before text. Skip unchanged marks during camera motion.

> **Check:** *Should rotating reshape the same measurement words every frame?* — No. Keep the mark and text document unchanged while updating only their projected frame.

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

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, include_str!("cloud_points.wgsl").into(), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout)], immediate_size: 0 });
        let mesh_pipeline = |depth| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }, wgpu::VertexBufferLayout {
                    array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);
        let mut renderer = Self {measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.instance_rows=scene.instances.keys().copied().collect();
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) { self.picker.submit(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self, settings: Option<crate::grid::Settings>) -> Result<(), &'static str> {
        if settings == self.grid_settings { return Ok(()); }
        let strokes = settings.map(crate::grid::Settings::strokes).transpose()?.unwrap_or_default();
        self.grid.set(&self.device, strokes)?; self.grid_settings = settings; Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:[[0.;4];4]}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
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
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, 0.0, 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)) {mesh.draw(&mut pass);}
            self.instances.draw(&mut pass);
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);}
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8> {
  let owner=self.outline.borrow();let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}
```

## <a id="checkpoint-84m-exercise-original-boxes-and-temporary-distance-annotations"></a>Step 14 of 37 · Exercise original boxes and temporary distance annotations (`84m-exercise-original-boxes-and-temporary-distance-annotations`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14/37: 84m-exercise-orig…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs, src/measure…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original solid</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reversible wire box</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">temporary distance line a…</text>
</svg>
</div>

Copy original/history/coordinate fixtures and observe actual measurement state in the browser. Display a selected wire box around its original solid.

> **Check:** *What distinguishes BoundingBox from measuring its lengths?* — BoundingBox creates an original OBB and one Undo step. Length BoundingBox only reports world extents; it adds no geometry or history.

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

**Complete updated block reference:**

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            if crate::browser_recovery::request(&message) { return; }
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> { run_with(crate::drawing_document::new()).await }

pub async fn run_with(shared: crate::drawing_document::Shared) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let recovery = crate::browser_recovery::adopt()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter_started = crate::browser_phase::now();
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() { return Ok(()); }
    crate::browser_phase::finish("adapter request", adapter_started, 0, "GPU");
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    crate::browser_phase::finish("device request", device_started, 0, "GPU");
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let text_fonts=std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        crate::commands::names(),
    );
    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    if let Some(message) = &recovery { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer.queue.on_submitted_work_done(move || {
            if crate::browser_runtime::owns(&completing) && completing.message().is_none() {
                crate::browser_phase::finish("first frame complete", frame_started, 0, "GPU");
                let _ = crate::browser_report::observe("milestone", "geometry on screen");
            }
        });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let mut touch=crate::touch::Touch::default();
    let pick_remove=std::cell::Cell::new(false);
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let publication=crate::browser_publication::Publication::new(std::rc::Rc::clone(&shared),fault.clone())?;
    let publication_control=publication.control();let start_publication=publication_control.clone();
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let sheet_query=crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();touch.cancel();let _=editor.apply(Action::CanvasCancel); let _=editor.apply(Action::GizmoCancel);
            crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());renderer.set_gizmo(&editor.camera,None,-1.);
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        if matches!(event.type_().as_str(),"blur"|"resize"|"pointercancel")||event.dyn_ref::<web_sys::KeyboardEvent>().is_some_and(|key|key.key()=="Escape"&&!key.is_composing()) {
            gesture.cancel();touch.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="lostpointercapture"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|editor.canvas_drag.as_ref().is_some_and(|d|d.pointer==p.pointer_id()&&d.released.is_none())) {
            gesture.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="pointerdown"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.is_primary())&&editor.canvas_drag.as_ref().is_none_or(|d|d.released.is_some()) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.mark.is_some() || editor.split.is_some() || editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        let mut highlight_changed = false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(&mut renderer, editor.display_scene(), editor.whole_selection(), active_fault.clone());
                        crate::browser_phase::revision(&editor.scene);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-stream-page" {
            renderer.cancel_pick();Some(Action::StreamRefresh)
        } else if event.type_() == "viewer-publication" {
            match publication_control.take() {Ok(action)=>action,Err(error)=>{let message=format!("Cannot load scene: {error}");report(&message);panel.result(&message);None}}
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery, &editor.scene) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot open file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            editor.mark=None;
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { crate::stream::reset_range_gate();publication_control.cancel();request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::TextDemo=>{if let Err(error)=renderer.text_demo() {panel.result(&error.to_string());}None},
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
                panel.result(&if rows.len() > 1 { format!("Objects {:?}", rows.iter().map(|id| id.value()).collect::<Vec<_>>()) }
                    else { editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()) });
                None
            }
            Host::Reload => {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            }
            Host::CancelReload => {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            }
            Host::CancelOpen => {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            }
            Host::Open | Host::Replace => {
                publication_control.cancel();
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if host == Host::Replace { reload.borrow_mut().cancel(); }
                read_mode.set(if host == Host::Replace { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            }
            Host::Report | Host::PreviousReport => {
                let output = if host == Host::PreviousReport { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            }
            Host::LayersOn=>{panel.tree.open=true;None},
            Host::LayersOff=>{panel.tree.open=false;None},
            Host::LayersAll=>{panel.tree.all();None},
            Host::Save => {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::scene_archive::save(&editor.scene)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            }
            }
            }
            }
        } else if !panel.consumed {
            navigation
        } else {
            None
        };
        let handle_action=gizmo_action(&event,&pointer_canvas,&editor,panel.consumed);
        let action = if let Some((key,edit))=panel.tree.edit.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerEdit {key,edit})
        } else if let Some((key,property))=panel.tree.property.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerProperty {key,property})
        } else if let Some((ids,additive))=panel.tree.action.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::TreePick {ids,additive})
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
                    else if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
                },
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
            None if editor.draft.is_some() && matches!(event.type_().as_str(), "blur" | "resize" | "pointercancel" | "lostpointercapture") => Some(Action::ToolHover(None)),
            None if editor.draft.is_some() && !panel.consumed && event.type_() == "pointermove" => {
                event.dyn_ref::<web_sys::PointerEvent>().filter(|p| p.buttons() == 0).map(|p| {
                    let r = pointer_canvas.get_bounding_client_rect();
                    let screen = [(2. * (p.client_x() as f64-r.left())/r.width()-1.) as f32, (1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32];
                    Action::ToolScreen { screen,size:[r.width(),r.height()],commit:false }
                })
            }
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
            Some(Action::CanvasStart {from,to,size,modified,pointer}) if editor.draft.is_none()=> {
                renderer.cancel_pick();
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,editor.camera.uniform()) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
            Some(action @ (Action::CanvasMove {..}|Action::CanvasRelease {..}))=>Some(action),
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, editor.camera.uniform()) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            if closing_action(&action) {publication_control.cancel();}
            let replacement = matches!(&action, Action::Adopt {replace:true,..} | Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = if editor.selected_objects().len() > 1 { Ok(None) }
                else { crate::edit_intent::Intent::capture(&action, editor.whole_selection()) }
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            if rename_action {panel.tree.rename_result(change.as_ref().err().map(String::as_str));}
            if tool_action && change.is_ok() {highlight_changed=true;panel.result(&editor.tool_status); }
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    crate::browser_upload::prepare(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-publication" {
                        crate::browser_phase::revision(&editor.scene);let _=crate::browser_report::observe("live-reload","source bytes changed");report("URL scene replaced.");panel.result("URL scene replaced.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::Preview)) => {
                    highlight_changed=true;
                    crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    let error=if error=="Shape input invalid" {editor.tool_status.as_str()}else{error.as_str()};
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if editor.plane_menu {panel.tool(editor.tool_status.clone(),crate::drawing_input::buttons());}
        else if let Some(split)=&editor.split{panel.tool(split.prompt(),vec![("Split".into(),"Split".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(draft) = editor.draft.as_ref() {let mut buttons=draft.tool.buttons();buttons.extend(crate::drawing_input::buttons());panel.tool(draft.prompt(),buttons);}
        else { panel.tool(String::new(), Vec::new()); }
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
            if editor.apply(Action::GizmoCancel).is_ok() {highlight_changed=true;crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());}
        }
        panel.rectangle=editor.canvas_drag.as_ref().filter(|d|d.missed).and_then(|d|d.rectangle());
        panel.number(number);
        panel.tree.refresh(&editor.scene,editor.selected_objects());
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
        }
        if let Err(error) = renderer.set_grid(editor.camera.grid) {
            report(error); return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Some(frame)=panel.free_frame([rect.width(),rect.height()]) {if editor.camera.frame(frame) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}}
            let hover=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|event.type_()=="pointermove" && p.buttons()==0 && !panel.consumed).map(|p| {
                let r=pointer_canvas.get_bounding_client_rect();
                ([(2.*(p.client_x() as f64-r.left())/r.width()-1.) as f32,(1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32],[r.width(),r.height()])
            });
            editor.hover_gizmo(hover);
            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            renderer.cloud_camera(&editor.camera);
            if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                editor.normal_view,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault, owned_editor, publication);
    report(&recovery.unwrap_or_else(|| "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.".into()));
    start_publication.begin();
    Ok(())
}

fn closing_action(action:&Action)->bool {matches!(action,Action::Close|Action::Adopt {..}|Action::Replace(_)|Action::ReplaceAt(_, _))}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id)).chain(editor.scene.texts().iter().map(|row|row.id)).chain(editor.scene.clouds().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    let local=editor.scene.forest.documents.iter().flat_map(|doc|doc.nodes.iter().map(move |node|serde_json::json!({"document":doc.key,"name":node.source.name,"object":node.object.map(|id|id.value()),"local":node.placement.local.m,"original_world":node.placement.world.m}))).collect::<Vec<_>>();
    canvas.set_attribute("data-original-tree-placements",&serde_json::to_string(&local).unwrap())?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-cpu",&serde_json::to_string(&crate::memory::clouds(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-cloud-gpu",&serde_json::to_string(&renderer.cloud_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-error",&renderer.cloud_error().unwrap_or_default())?;
    canvas.set_attribute("data-cloud-sources",&serde_json::json!(editor.scene.clouds().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid(),"point_size":row.prepared.source.point_size,"point_px":row.point_px,"points":row.prepared.source.len(),"nodes":row.prepared.nodes.len(),"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-surface-outline",&serde_json::to_string(&renderer.outline_usage()).unwrap())?;
    canvas.set_attribute("data-scene-text-stats",&serde_json::to_string(&renderer.text_stats()).unwrap())?;
    canvas.set_attribute("data-scene-text-cpu",&serde_json::to_string(&crate::memory::texts(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-scene-text-gpu",&serde_json::to_string(&renderer.text_scene_usage()).unwrap())?;
    canvas.set_attribute("data-scene-texts",&serde_json::json!(editor.scene.texts().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source":row.prepared.source,"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-walks",&serde_json::json!({"scope":"last imported original document","walks":editor.scene.cad_walks[0],"replays":editor.scene.cad_walks[1]}).to_string())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
    canvas.set_attribute("data-surface-cpu",&serde_json::to_string(&crate::memory::surfaces(editor.presentation_scenes())).unwrap())?;
    let surfaces=editor.scene.objects().iter().filter_map(|row|row.surface.as_ref().map(|s|serde_json::json!({"id":row.id.value(),"source":s.source.guid(),"controls":s.source.m_cv_count,"steps":s.grid.steps,"samples":s.grid.samples.len(),"faces":row.mesh.parts.faces,"edges":s.grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-surfaces",&serde_json::to_string(&surfaces).unwrap())?;
    canvas.set_attribute("data-curve-cpu", &serde_json::to_string(&crate::memory::curves(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-control-count", &editor.scene.control_markers().len().to_string())?;
    canvas.set_attribute("data-point-cpu", &serde_json::to_string(&crate::memory::points(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-path-gpu", &serde_json::to_string(&renderer.path_usage()).unwrap())?;
    canvas.set_attribute("data-path-cpu", &serde_json::to_string(&crate::memory::paths(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-stroke-usage", &serde_json::to_string(&renderer.stroke_usage()).unwrap())?;
    canvas.set_attribute("data-line-usage", &serde_json::to_string(&crate::memory::lines(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    let originals=serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-measurement",&editor.mark.as_ref().map_or_else(||"null".into(),|m|serde_json::json!({"points":m.points,"label":m.label}).to_string()))?;
    canvas.set_attribute("data-measurement-gpu-bytes",&renderer.measurement_bytes().to_string())?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
    canvas.set_attribute("data-gather",&editor.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||"null".into(),|g|serde_json::json!({"name":g.recipe.name,"step":g.progress.step,"rows":g.progress.rows,"answered":g.progress.answered,"choice":g.choice}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
    canvas.set_attribute("data-snap-settings", &serde_json::json!({"objects":editor.snapping.objects,"modes":editor.snapping.modes,"grid":editor.snapping.grid,"step":editor.snapping.step}).to_string())?;
    canvas.set_attribute("data-snap-usage", &serde_json::to_string(&editor.snap_usage()).unwrap())?;
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    let sheets=editor.scene.sheets().iter().map(|row|serde_json::json!({"row":row.id.value(),"guid":row.guid,"source":row.prepared.original.guid,"model":row.model.m,"segments":row.prepared.original.segment_count,"entities":row.prepared.original.entity_count,"meta":row.prepared.original.meta,"streamed":row.prepared.streamed.is_some(),"resident":row.prepared.pages().iter().map(|page|page.rows.end-page.rows.start).sum::<u32>()})).collect::<Vec<_>>();canvas.set_attribute("data-sheets",&serde_json::to_string(&sheets).unwrap())?;canvas.set_attribute("data-sheet-cpu",&serde_json::to_string(&crate::memory::sheets(editor.scenes())).unwrap())?;canvas.set_attribute("data-sheet-gpu",&serde_json::to_string(&renderer.sheet_usage()).unwrap())?;
    canvas.set_attribute("data-instance-cpu",&serde_json::to_string(&crate::memory::instances(editor.scenes())).unwrap())?;
    canvas.set_attribute("data-instance-gpu",&serde_json::to_string(&renderer.instance_usage()).unwrap())?;
    let instances=editor.scene.instances.iter().map(|(id,owner)|serde_json::json!({"id":id.value(),"guid":editor.scene.row_guid(*id),"source_guid":owner.instance.guid(),"definition_guid":owner.definition.guid(),"document_guid":owner.document.guid(),"model":editor.scene.placement(*id).map(|m|m.m),"flags":owner.instance.flags,"colour":editor.scene.face_rgba(*id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-instances",&serde_json::to_string(&instances).unwrap())?;
    let stream_usage=crate::memory::streams(editor.scenes());
    canvas.set_attribute("data-range-sources",&serde_json::to_string(&stream_usage).unwrap())?;
    let streamed=editor.scene.streams().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid,"name":row.prepared.source.name,"url":row.prepared.source.url,"revision":row.prepared.source.fields.revision,"total":row.prepared.source.fields.count,"model":row.model.m,"point_px":row.point_px,"controls":row.controls_visible})).collect::<Vec<_>>();
    canvas.set_attribute("data-range-documents",&serde_json::to_string(&streamed).unwrap())?;

    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-grid-enabled", if editor.camera.grid.is_some() { "true" } else { "false" })?;
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
    touch:&mut crate::touch::Touch,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            let rect=canvas.get_bounding_client_rect();
            let Action::Zoom(factor)=crate::navigation::wheel(wheel.delta_y(),mode,rect.height())? else {return None;};
            let screen=[(2.*(wheel.client_x() as f64-rect.left())/rect.width()-1.) as f32,(1.-2.*(wheel.client_y() as f64-rect.top())/rect.height()) as f32];
            Some(Action::ZoomAt {factor,screen})
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _=>{
            if let Some(pointer)=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|p.pointer_type()=="touch") {
                event.prevent_default();let r=canvas.get_bounding_client_rect();let rect=[r.left(),r.top(),r.width(),r.height()];let at=[pointer.client_x() as f64,pointer.client_y() as f64];let id=pointer.pointer_id();
                let actions=match event.type_().as_str() {
                    "pointerdown"=>{gesture.cancel();let multiple=touch.count()>0;if touch.press(id,at,event.time_stamp()) {if canvas.set_pointer_capture(id).is_err() {touch.cancel();}}if multiple {vec![Action::GizmoCancel,Action::CanvasCancel]} else {Vec::new()}},
                    "pointermove"=>touch.move_to(id,at,rect),
                    "pointerup"=>{let actions=touch.release(id,at,event.time_stamp(),rect);let _=canvas.release_pointer_capture(id);actions},
                    "pointercancel"=>{touch.cancel();Vec::new()},"lostpointercapture"=>{if touch.contains(id) {touch.cancel();}Vec::new()},_=>Vec::new(),
                };return (!actions.is_empty()).then_some(Action::Touch(actions));
            }
            return pointer_action(event,canvas,gesture);
        },
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press_at(id, pointer.button(), position,pointer.ctrl_key()||pointer.meta_key(),pointer.time_stamp()) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_at(id, position,pointer.time_stamp()),
        "pointerup" => {
            let motion = gesture.release_at(id, position,pointer.time_stamp());
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    let mut action=motion.action([rect.left(),rect.top(),rect.width(),rect.height()])?;
    match &mut action {
        Action::CanvasStart {modified,pointer:id,..}=>{*id=pointer.pointer_id();*modified=pointer.shift_key()||pointer.ctrl_key()||pointer.meta_key();},
        Action::CanvasRelease {add,remove,..}=>{*add=pointer.shift_key();*remove=pointer.ctrl_key()||pointer.meta_key();},_=>{}
    }Some(action)
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        density,
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    normals: bool,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    if let Some(error)=renderer.cloud_error() {panel.result(&format!("Cloud: {error}"));}
    if let Some(error)=renderer.text_error() {panel.result(&format!("Text: {error}"));}
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}

fn gizmo_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action> {
    if matches!(event.type_().as_str(),"blur"|"resize") && editor.gizmo_edit.is_some() {return Some(Action::GizmoCancel);}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;
    if matches!(event.type_().as_str(),"pointercancel"|"lostpointercapture") {
        return editor.gizmo_edit.as_ref().filter(|e|e.pointer==Some(pointer.pointer_id())).map(|_|Action::GizmoCancel);
    }
    let rect=canvas.get_bounding_client_rect();let size=[rect.width(),rect.height()];
    if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
    let screen=[(2.*(pointer.client_x() as f64-rect.left())/size[0]-1.) as f32,(1.-2.*(pointer.client_y() as f64-rect.top())/size[1]) as f32];
    let id=pointer.pointer_id();
    if editor.gizmo_edit.as_ref().is_some_and(|e|e.pointer==Some(id)) {
        event.prevent_default();
        return match event.type_().as_str() {
            "pointermove"=>Some(Action::GizmoMove {pointer:id,screen,size,shift:pointer.shift_key()}),
            "pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::GizmoRelease {pointer:id,screen,size,shift:pointer.shift_key()})},
            _=>None,
        };
    }
    if consumed || event.type_()!="pointerdown" || !pointer.is_primary() || pointer.button()!=0 {return None;}
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {return None;}
    let finger=pointer.pointer_type()=="touch";
    if editor.gizmo_hit_radius(screen,size,if finger {18.} else {8.}).is_some() {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {return Some(Action::GizmoCancel);}
        Some(if finger {Action::GizmoTouch {pointer:id,screen,size}} else {Action::GizmoPress {pointer:id,screen,size,modified:false}})
    } else if editor.gizmo_edit.is_some() {Some(Action::GizmoCancel)} else {None}
}
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

After that block, append this supplied code:

```text
--8<-- "journey/code/84m-exercise-original-boxes-and-temporary-distance-annotations-07.rs" skip=22
```

**Result after `84m-exercise-original-boxes-and-temporary-distance-annotations`:** A selected original wire bounding box surrounds the solid. A temporary black distance line, endpoint squares and a 2 mm value chip remain outside geometry history.

## <a id="checkpoint-84zza-describe-removable-cad-parts"></a>Step 15 of 37 · Describe removable CAD parts (`84zza-describe-removable-cad-parts`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15/37: 84zza-describe-re…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs, src/lib.…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Represent curve intervals, BRep face groups and mesh sides without replacing the original CAD geometry. Write seam-aware curve intervals and pieces.

> **Check:** *Why can the last interval end before it starts?* — A closed curve wraps through its parameter seam; join its tail and head.

### 1. `src/trim_parts.rs`

Type the original CAD part representation and seam-aware curve helpers.

Create the file and type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-1.rs"
```

### 2. `src/lib.rs`

Register the shared part owner.

After that block, type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-2.rs" skip=32
```

## <a id="checkpoint-84zzb-read-trim-part-previews"></a>Step 16 of 37 · Read trim part previews (`84zzb-read-trim-part-previews`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16/37: 84zzb-read-trim-p…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Expose part counts, sampled curve outlines, section edges and meshes through one Parts owner. These helpers prepare later pointer picking.

> **Check:** *Why return face meshes with their part index?* — Several faces can belong to one removable part; the click must recover that group.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzc-collect-curve-crossings"></a>Step 17 of 37 · Collect curve crossings (`84zzc-collect-curve-crossings`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17/37: 84zzc-collect-cur…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Collect kernel curve intersections, filter open endpoints and merge a closed seam. Add shared compact-table helpers for later BRep reconstruction.

> **Check:** *Why remove open endpoints from the cut list?* — An endpoint creates no additional removable interval.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzc-collect-curve-crossings-1.rs"
```

## <a id="checkpoint-84zzd-preserve-trimmed-source-types"></a>Step 18 of 37 · Preserve trimmed source types (`84zzd-preserve-trimmed-source-types`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18/37: 84zzd-preserve-tr…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Rebuild kept curves in their original source family with their presentation. Wrap a surface as a single-face BRep using the existing kernel split API.

> **Check:** *Why discard incompatible per-control colors?* — A trimmed curve can have a different control count; stale color arrays no longer describe it.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zze-group-faces-by-cutting-planes"></a>Step 19 of 37 · Group faces by cutting planes (`84zze-group-faces-by-cutting-planes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19/37: 84zze-group-faces…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Extend a plane-plane intersection past surface control points. Group faces by plane-side signs while retaining independently cut face regions.

> **Check:** *Why keep curve-cut regions separate?* — Faces split by a curve are individually removable even when they share the same plane-side signs.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzf-cut-planar-brep-faces"></a>Step 20 of 37 · Cut planar BRep faces (`84zzf-cut-planar-brep-faces`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20/37: 84zzf-cut-planar-…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Find crossed planar faces, split them through the kernel and collect section edges. Report unsupported curved sections explicitly.

> **Check:** *Why scale the plane tolerance with the geometry?* — The local coordinate magnitude controls how close a point must be to the plane.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzg-assemble-removable-face-groups"></a>Step 21 of 37 · Assemble removable face groups (`84zzg-assemble-removable-face-groups`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21/37: 84zzg-assemble-re…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Combine curve regions, fallback fence planes and crossed plane sections into one Parts value. Keep the source BRep unchanged by working on its clone.

> **Check:** *Why ignore a plane that misses one target?* — The same cutter list can cross another selected target.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzh-mark-retained-brep-topology"></a>Step 22 of 37 · Mark retained BRep topology (`84zzh-mark-retained-brep-topology`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22/37: 84zzh-mark-retain…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Walk retained faces through wires and edges to mark their surfaces, vertices and curve tables. This preparatory function returns an error until the next lesson compacts those tables.

> **Check:** *Why follow references before deleting table entries?* — Deleting first would shift indices before the surviving references can be mapped.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzi-rebuild-retained-brep-tables"></a>Step 23 of 37 · Rebuild retained BRep tables (`84zzi-rebuild-retained-brep-tables`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23/37: 84zzi-rebuild-ret…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Compact marked BRep tables and rewrite every reference. Keep shell orientation, drop empty shells and retain solids only when their shells remain closed.

> **Check:** *Why can removing a face turn a solid into an open shell?* — A solid requires closed shells; the remaining faces still form useful surface geometry.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzi-rebuild-retained-brep-tables-1.rs"
```

## <a id="checkpoint-84zzj-split-mesh-faces-at-a-plane"></a>Step 24 of 37 · Split mesh faces at a plane (`84zzj-split-mesh-faces-at-a-plane`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 24/37: 84zzj-split-mesh-…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Classify mesh vertices, reuse one cut vertex per original edge and build both clipped face rings. This preparatory function returns an error until its output tables are compacted next.

> **Check:** *Why sort the edge endpoints in the crossing key?* — Adjacent faces can traverse the same edge in opposite directions; both must reuse one cut vertex.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzk-compact-both-trimmed-mesh-sides"></a>Step 25 of 37 · Compact both trimmed mesh sides (`84zzk-compact-both-trimmed-mesh-sides`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 25/37: 84zzk-compact-bot…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Compact each clipped mesh side to only its used vertices, rewrite its faces and preserve source name and color. Return both sides and their cut segments.

> **Check:** *Why build a separate vertex map for each side?* — Each output mesh owns only the vertices its own faces reference.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzk-compact-both-trimmed-mesh-sides-1.rs"
```

## <a id="checkpoint-84zzl-rebuild-kept-cad-parts"></a>Step 26 of 37 · Rebuild kept CAD parts (`84zzl-rebuild-kept-cad-parts`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 26/37: 84zzl-rebuild-kep…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Merge adjacent kept curve intervals, remove BRep face groups and return the retained mesh half. Preserve the last part so complete removal remains the Delete command.

> **Check:** *Why start a closed curve after a removed part?* — A kept run that crosses the parameter seam should remain one geometry.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

## <a id="checkpoint-84zzm-connect-original-trim-recipes"></a>Step 27 of 37 · Connect original trim recipes (`84zzm-connect-original-trim-recipes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 27/37: 84zzm-connect-ori…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/trim_parts.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD geometry</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">removable parts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">retained source geometry</text>
</svg>
</div>

Route original curves, surfaces, BReps and meshes through the shared part builders. Keep the original validation fixtures supplied separately.

> **Check:** *Why try successive planes when trimming a mesh?* — The first plane can miss this target; stop at the first real split or substantive error.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

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

After that block, append this supplied code:

```text
--8<-- "journey/code/84zzm-connect-original-trim-recipes-2.rs" skip=3698
```

## <a id="checkpoint-84zna-index-selection-loops-by-pixel-row"></a>Step 28 of 37 · Index selection loops by pixel row (`84zna-index-selection-loops-by-pixel-row`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 28/37: 84zna-index-selec…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs, sr…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Build a row index for the even-odd point test. Keep rectangles in the same Region owner so later crossing selection can reuse their bounds.

> **Check:** *Why exclude the upper endpoint of each crossing edge?* — Counting a shared vertex twice would change whether its row is inside.

### 1. `src/selection_region.rs`

Type the shared selection region and row index.

Create the file and type:

```rust
--8<-- "journey/code/84zna-index-selection-loops-by-pixel-row-1.rs"
```

### 2. `src/lib.rs`

Register the selection owner.

After that block, type:

```rust

pub mod selection_region;
```

## <a id="checkpoint-84znb-test-window-membership-and-segment-crossings"></a>Step 29 of 37 · Test window membership and segment crossings (`84znb-test-window-membership-and-segment-crossings`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 29/37: 84znb-test-window…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Complete Region with point membership and segment clipping against the rectangle. Clip the segment parameter interval rather than sampling its length.

> **Check:** *Why keep a parameter interval for a crossing stroke?* — A long stroke can cross the window even when both endpoints are outside.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

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

## <a id="checkpoint-84znc-sample-original-cad-families-for-selection"></a>Step 30 of 37 · Sample original CAD families for selection (`84znc-sample-original-cad-families-for-selection`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 30/37: 84znc-sample-orig…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Reuse original curve corners, curved domains, mesh vertices and BRep references as bounded selection samples. Distinguish no samples from a failed inside test.

> **Check:** *Why preserve None for an empty sample iterator?* — An empty geometry cannot establish complete enclosure; its bounds may supply the fallback.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

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

## <a id="checkpoint-84znd-collect-original-boundary-strokes"></a>Step 31 of 37 · Collect original boundary strokes (`84znd-collect-original-boundary-strokes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 31/37: 84znd-collect-ori…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Build surface grids and closed mesh outlines, then combine BRep vertices, edges and curved faces. Keep each original boundary as an ordered stroke.

> **Check:** *Why close a mesh face outline explicitly?* — The last vertex and first vertex form an edge that crossing selection must also test.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

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

## <a id="checkpoint-84zne-test-crossing-selection-on-original-strokes"></a>Step 32 of 37 · Test crossing selection on original strokes (`84zne-test-crossing-selection-on-original-strokes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 32/37: 84zne-test-crossi…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Dispatch original geometry families to their shared stroke builders. Crossing selection accepts an inside sample or a rectangle-crossing segment; cloud sampling remains bounded.

> **Check:** *Why project each stroke before testing adjacent points?* — Membership and crossing use screen pixels, while original geometry and its placements stay in world coordinates.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

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

## <a id="checkpoint-84znf-connect-window-and-bounds-selection-tests"></a>Step 33 of 37 · Connect window and bounds selection tests (`84znf-connect-window-and-bounds-selection-tests`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 33/37: 84znf-connect-win…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/selection_region.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD samples</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">screen region</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">window or crossing result</text>
</svg>
</div>

Add projection, conservative screen bounds and complete-enclosure dispatch. Supply the original validation fixtures separately; the existing selection command connects this owner in the next lesson.

> **Check:** *Why keep the bounds fallback separate from sampled geometry?* — Planes and empty original families cannot establish membership from samples, but their projected bounds can still guide selection.

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

**Complete updated block reference:**

```rust
--8<-- "journey/code/84znf-connect-window-and-bounds-selection-tests-1.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/selection_region.rs`

Supply the original region-selection validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/84znf-connect-window-and-bounds-selection-tests-2.rs" skip=559
```

## <a id="checkpoint-84zoa-classify-originals-and-isolate-one-mesh-face"></a>Step 34 of 37 · Classify originals and isolate one mesh face (`84zoa-classify-originals-and-isolate-one-mesh-face`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 34/37: 84zoa-classify-or…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_shape.rs, src/li…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">result or refusal</text>
</svg>
</div>

Name each original geometry kind once. Build one mesh face with compact vertex keys, preserve its hole rings and retain its display attributes.

> **Check:** *Why remap vertices for a separated face?* — The new mesh owns a compact vertex table. All outer and hole rings must refer to that new table.

### 1. `src/source_shape.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-1.rs"
```

### 2. `src/lib.rs`

Compile the shared original geometry classifier.

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

After that block, type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-4.rs" skip=21
```

**Result after `84zoa-classify-originals-and-isolate-one-mesh-face`:** Original-family classification and single mesh-face extraction are prepared.

## <a id="checkpoint-84zob-copy-original-brep-edges-and-pcurves"></a>Step 35 of 37 · Copy original BRep edges and pcurves (`84zob-copy-original-brep-edges-and-pcurves`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 35/37: 84zob-copy-origin…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">result or refusal</text>
</svg>
</div>

Copy an edge, its endpoint vertices and its surface pcurves into a new BRep. Reuse vertex mappings and preserve seam curves, tolerances and degenerated edges.

> **Check:** *Why keep both pcurves on a seam?* — A periodic surface uses the same spatial edge on two parameter boundaries. Both uses must survive the separated face.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

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

**Result after `84zob-copy-original-brep-edges-and-pcurves`:** BRep face extraction can preserve the required original edge and pcurve records.

## <a id="checkpoint-84zoc-build-one-original-brep-face"></a>Step 36 of 37 · Build one original BRep face (`84zoc-build-one-original-brep-face`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 36/37: 84zoc-build-one-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">result or refusal</text>
</svg>
</div>

Reuse the copied-edge helper to rebuild one face surface, ordered wire references and one oriented shell. Keep shared edges unique within that face.

> **Check:** *Why preserve reference orientation?* — An edge table entry stores geometry. Each wire and shell reference also carries the direction in which that geometry is used.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

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

**Result after `84zoc-build-one-original-brep-face`:** One original BRep face can be rebuilt from its retained boundary data.

## <a id="checkpoint-84zod-explode-supported-original-families"></a>Step 37 of 37 · Explode supported original families (`84zod-explode-supported-original-families`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 37/37: 84zod-explode-sup…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/explode_geometry.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">result or refusal</text>
</svg>
</div>

Count derived pieces before allocating them. Split polylines, BRep faces, mesh faces and point clouds while preserving their original display attributes. Reject unsupported or single-piece inputs.

> **Check:** *Why check the count before making pieces?* — The bounded command must refuse excessive output before constructing thousands of objects.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

After that block, type:

```rust
--8<-- "journey/code/84zod-explode-supported-original-families-1.rs" skip=135
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/explode_geometry.rs`

Supply only the unchanged original validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/84zod-explode-supported-original-families-2.rs" skip=6850
```

**Result after `84zod-explode-supported-original-families`:** Supported original families have an explode dispatcher; browser acceptance follows with the connected command.

## Build, Verify & Visual Result

```sh
npm --prefix ../session_tests run course -- dependencies 16.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 16.4
npm --prefix ../session_tests run course -- save 16.4
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [16.5 · Text, Plane, Trim and integrated editing commands](../../consolidation/85-copy-current-original-command-recipes.md).
