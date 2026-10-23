# 7.5 · Temporary transforms with a fixed baseline

**Goal:** Cancel previews without consuming document history. · **Steps:** 5 · **Source:** [app/command/mod.rs](../../../../src/app/command/mod.rs) · [app/command/tool.rs](../../../../src/app/command/tool.rs) · [app/cplane.rs](../../../../src/app/cplane.rs) · [app/snap.rs](../../../../src/app/snap.rs) · [app/gizmo.rs](../../../../src/app/gizmo.rs) · [engine/gpu/widget.rs](../../../../src/engine/gpu/widget.rs) · [app/deform.rs](../../../../src/app/deform.rs) (`src/app/cplane.rs,src/app/gizmo.rs,src/app/gesture/gizmo.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 7.5 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/edit_preview.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">5 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/gizmo.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Address placements across…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. C-Plane Snap &amp; Preview</text>
  <polygon points="602,128 652,72 798,72 748,128" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="628" y1="50" x2="696" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="700" cy="102" r="5" fill="#f59e0b" stroke="#111111" stroke-width="1.5"/>
  <circle cx="700" cy="102" r="10" fill="none" stroke="#d97706" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Temporary transforms with a…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGB 3D Gumball Gizmo</text>
  <circle cx="956" cy="118" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="956" y1="118" x2="956" y2="52" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="940" y="60" font-family="monospace" font-size="10.5" font-weight="700" fill="#2563eb">Z</text>
  <line x1="956" y1="118" x2="1036" y2="118" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1044" y="122" font-family="monospace" font-size="10.5" font-weight="700" fill="#dc2626">X</text>
  <line x1="956" y1="118" x2="904" y2="144" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="888" y="146" font-family="monospace" font-size="10.5" font-weight="700" fill="#16a34a">Y</text>
  <path d="M 956 80 A 38 38 0 0 1 994 118" fill="none" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Cancel previews without c…</text>
</svg>
</div>

## <a id="checkpoint-50-placements"></a>1. Address placements across geometry lanes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original owner ID</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked replacement</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original geometry</text>
</svg>
</div>

**What this code does:** Address placements across original geometry lanes.

### 1. `src/scene.rs`

Read and replace checked placements across all supported original geometry lanes.

**Locate this existing block:**

```rust
    }

    pub fn translate(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.model.clone())
            .or_else(|| {
                self.lines
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.paths
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.points
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
    }

    pub fn place_owner(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id == id) {
            self.place(id, model)
        } else if self.lines.iter().any(|r| r.id == id) {
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else {
            self.place_point(id, model)
        }
    }

    pub fn translate(
```

## <a id="checkpoint-50a-preview"></a>2. Keep temporary placements outside the document

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/edit_preview.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">eligible owners and basel…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived placement rows</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cancel or one history com…</text>
</svg>
</div>

**What this code does:** Keep temporary placements outside the document.

### 1. `src/edit_preview.rs`

Capture eligible original owners, derive temporary placements and commit their group atomically.

Create the file and type:

```rust
use crate::{
    history::History,
    scene::{ObjectId, Scene},
};
use session_rust::Xform;

// A placement preview shares original geometry, but owns temporary row placements outside the saved document and its history.
pub struct Preview {
    base: Scene,
    pub scene: Scene,
    ids: Vec<ObjectId>,
    transform: Xform,
}

impl Preview {
    pub fn new(scene: &Scene, mut ids: Vec<ObjectId>) -> Result<Self, &'static str> {
        ids.sort_unstable_by_key(|id| id.value());
        ids.dedup();
        if ids.is_empty() {
            return Err("Select objects before Move");
        }
        for id in &ids {
            if scene.pick_id(id.value()) != Some(*id) {
                return Err("Move needs visible unlocked objects");
            }
            if scene
                .objects()
                .iter()
                .any(|r| r.id == *id && r.geometry().is_none())
            {
                return Err("Reload editable sources before Move");
            }
        }
        Ok(Self {
            base: scene.clone(),
            scene: scene.clone(),
            ids,
            transform: Xform::identity(),
        })
    }
    pub fn ids(&self) -> &[ObjectId] {
        &self.ids
    }
    pub fn matrix(&self) -> &[f64; 16] {
        &self.transform.m
    }
    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) {
            return Err("Invalid preview transform");
        }
        let mut scene = self.base.clone();
        for &id in &self.ids {
            let model = &transform
                * &self
                    .base
                    .placement(id)
                    .ok_or("Preview owner missing")?;
            scene.place_owner(id, model)?;
        }
        self.scene = scene;
        self.transform = transform;
        Ok(())
    }
    pub fn commit(
        &self,
        scene: &mut Scene,
        history: &mut History,
    ) -> Result<bool, &'static str> {
        for &id in &self.ids {
            if scene.pick_id(id.value()) != Some(id)
                || scene.placement(id).map(|m| m.m)
                    != self.base.placement(id).map(|m| m.m)
            {
                return Err("Preview targets changed; start Move again");
            }
        }
        if self.transform.m == Xform::identity().m {
            return Ok(false);
        }
        history.try_edit(scene, |scene| {
            for &id in &self.ids {
                scene.place_owner(
                    id,
                    self.scene
                        .placement(id)
                        .ok_or("Preview owner missing")?,
                )?;
            }
            Ok(())
        })?;
        Ok(true)
    }
}
```

### 2. `src/lib.rs`

Register the placement preview and supplied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod snap_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod snap_tests;

pub mod edit_preview;
```

## <a id="checkpoint-50b-display"></a>3. Present temporary edits through a separate scene view

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/snap_index.rs, src/edit…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">saved scene or preview ro…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">display scene</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared geometry buffers a…</text>
</svg>
</div>

**What this code does:** Present temporary edits through a separate scene view.

### 1. `src/snap_index.rs`

Exclude the moving owners when querying destination snap targets.

**Locate this existing block:**

```rust
    }
    pub fn query(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn excluding(mut self, ids: &[crate::scene::ObjectId]) -> Self {
        self.targets.retain(|target| match target {
            Target::Point(hit) => hit.owner.is_none_or(|id| !ids.contains(&id)),
            Target::Wire { owner, .. } => !ids.contains(owner),
        });
        self
    }
    pub fn query(
```

### 2. `src/editor.rs`

Own temporary edit presentation and derive displayed rows without modifying the saved scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let index = self
                .snap_index
                .get_or_insert_with(|| crate::snap_index::Index::new(&self.scene));
            let viewport = crate::snap_math::Screen {
```

Replace it with:

```rust
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude = self
                .edit_preview
                .as_ref()
                .filter(|_| from.is_some())
                .map_or_else(Vec::new, |p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| {
                crate::snap_index::Index::new(&self.scene).excluding(&exclude)
            });
            let viewport = crate::snap_math::Screen {
```

**Locate this block:**

```rust
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }
```

Replace it with:

```rust
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.edit_preview
            .as_ref()
            .map_or(&self.scene, |p| &p.scene)
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
        let offset = self
            .draft
            .as_ref()
            .and_then(|d| d.points.first().zip(d.cursor))
            .map(|(a, b)| std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]))
            .unwrap_or([0.; 3]);
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            snapping: Default::default(),
            snap_index: None,
            draft: None,
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
```

**Locate this block:**

```rust
    Scene,
}

pub struct Editor {
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

Replace it with:

```rust
    Scene,
    Preview,
}

pub struct Editor {
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

### 3. `src/browser.rs`

Present temporary row settings and keep saved source inspection separate.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
```

Replace it with:

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
```

**Locate this block:**

```rust
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) = renderer
                    .set_highlight(&editor.scene, editor.selection())
                    .and_then(|()| {
                        renderer
                            .select_objects(&editor.scene, &editor.selected_objects())
                    })
                {
                    report(error);
```

Replace it with:

```rust
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) = renderer
                    .set_highlight(editor.display_scene(), editor.selection())
                    .and_then(|()| {
                        renderer.select_objects(
                            editor.display_scene(),
                            &editor.selected_objects(),
                        )
                    })
                {
                    report(error);
```

**Locate this block:**

```rust
                    }
                    Ok(Some(Change::View)) | Ok(None) => {}
                    Err(error) => {
                        report(&error);
```

Replace it with:

```rust
                    }
                    Ok(Some(Change::Preview)) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(
                            &mut renderer,
                            editor.display_scene(),
                            editor.whole_selection(),
                            active_fault.clone(),
                        );
                    }
                    Ok(Some(Change::View)) | Ok(None) => {}
                    Err(error) => {
                        report(&error);
```

## <a id="checkpoint-50c-move"></a>4. Preview Move from a base point to a target

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tool.rs, src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected original owners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">snapped target and tempor…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one Move commit</text>
</svg>
</div>

**What this code does:** Preview Move from a base point to a target.

### 1. `src/tool.rs`

Preview a selected-owner Move, exclude its own snap targets and commit the chosen target once.

**Locate this existing block:**

```rust
    Line(PreparedLine),
}

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn buttons(&self) -> Vec<(String, String)> {
        vec![("Cancel".into(), "Escape".into())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Point,
    Line,
}

impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Point => "Point",
            Self::Line => "Line",
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Point => 1,
            Self::Line => 2,
        }
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(
                session_rust::Point::new(p[0], p[1], p[2]),
            ))?)),
            (Self::Line, [a, b]) => {
                if a.iter()
                    .zip(b)
                    .all(|(a, b)| (a - b).abs() < 1e-12)
                {
                    return Err("Line endpoints must differ");
                }
                Ok(Built::Line(PreparedLine::new(Rc::new(
                    session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2]),
                ))?))
            }
            _ => Err("The tool needs more points"),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Line(PreparedLine),
    Move([f64; 3]),
}

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn buttons(&self) -> Vec<(String, String)> {
        vec![("Cancel".into(), "Escape".into())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Point,
    Line,
    Move,
}

impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Point => "Point",
            Self::Line => "Line",
            Self::Move => "Move",
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Point => 1,
            Self::Line | Self::Move => 2,
        }
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(
                session_rust::Point::new(p[0], p[1], p[2]),
            ))?)),
            (Self::Line, [a, b]) => {
                if a.iter()
                    .zip(b)
                    .all(|(a, b)| (a - b).abs() < 1e-12)
                {
                    return Err("Line endpoints must differ");
                }
                Ok(Built::Line(PreparedLine::new(Rc::new(
                    session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2]),
                ))?))
            }
            (Self::Move, [a, b]) => Ok(Built::Move(std::array::from_fn(|i| {
                b[i] - a[i]
            }))),
            _ => Err("The tool needs more points"),
```

### 2. `src/editor.rs`

Preview a selected-owner Move, exclude its own snap targets and commit the chosen target once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    draft.snapped = None;
                    draft.hover(point)?;
                }
                return Ok(Change::View);
            }
            Action::CancelTool => {
                self.draft = None;
                self.tool_status = "Tool cancelled.".into();
                return Ok(Change::View);
            }
```

Replace it with:

```rust
                    draft.snapped = None;
                    draft.hover(point)?;
                }
                return self.update_edit_preview();
            }
            Action::CancelTool => {
                let changed = self.edit_preview.take().is_some();
                self.draft = None;
                self.tool_status = "Tool cancelled.".into();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
```

**Locate this block:**

```rust
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(hit.map(|h| h.point))?;
                }
                return Ok(Change::View);
            }
            Action::BeginTool(shape) => {
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
                self.tool_status = self.draft.as_ref().unwrap().prompt();
                return Ok(Change::View);
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
```

Replace it with:

```rust
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(hit.map(|h| h.point))?;
                }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview = if matches!(shape, crate::tool::Shape::Move) {
                    if self.selected_part.is_some() {
                        return Err("Use Select Object before Move");
                    }
                    Some(crate::edit_preview::Preview::new(
                        &self.scene,
                        self.selected_objects(),
                    )?)
                } else {
                    None
                };
                let changed = preview.is_some() || self.edit_preview.is_some();
                self.edit_preview = preview;
                self.snap_index = None;
                self.draft = Some(crate::tool::Draft::new(shape));
                self.tool_status = self.draft.as_ref().unwrap().prompt();
                return Ok(if changed {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
```

**Locate this block:**

```rust
                    draft.cursor = None;
                    draft.snapped = None;
                }
                return Ok(Change::View);
            }
            Action::ToolScreen {
                screen,
```

Replace it with:

```rust
                    draft.cursor = None;
                    draft.snapped = None;
                }
                return self.update_edit_preview();
            }
            Action::ToolScreen {
                screen,
```

**Locate this block:**

```rust
        let change = self.apply_action(action)?;
        if change == Change::Scene {
            self.snap_index = None;
        }
        Ok(change)
    }
```

Replace it with:

```rust
        let change = self.apply_action(action)?;
        if change == Change::Scene {
            self.snap_index = None;
            if self.edit_preview.take().is_some() {
                self.draft = None;
            }
        }
        Ok(change)
    }
```

**Locate this block:**

```rust
            self.tool_status = draft.prompt();
            return Ok(Change::View);
        };
        self.history
            .try_edit(&mut self.scene, |scene| match built {
                crate::tool::Built::Point(point) => {
                    scene.insert_point(point).map(|_| ())
                }
                crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
            })?;
        self.draft = None;
        self.tool_status = format!("{name} created.");
```

Replace it with:

```rust
            self.tool_status = draft.prompt();
            self.snap_index = None;
            return self.update_edit_preview();
        };
        if let crate::tool::Built::Move(offset) = built {
            let preview = self
                .edit_preview
                .as_mut()
                .ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(
                offset[0], offset[1], offset[2],
            ))?;
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if self.edit_preview.take().is_some() {
                self.draft = None;
            }
            self.tool_status = "Move completed.".into();
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        self.history
            .try_edit(&mut self.scene, |scene| match built {
                crate::tool::Built::Point(point) => {
                    scene.insert_point(point).map(|_| ())
                }
                crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
                crate::tool::Built::Move(_) => {
                    unreachable!("Move uses its placement preview")
                }
            })?;
        self.draft = None;
        self.tool_status = format!("{name} created.");
```

### 3. `src/commands.rs`

Preview a selected-owner Move, exclude its own snap targets and commit the chosen target once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Move" {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
```

Replace it with:

```rust
pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Move" && words.len() > count {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
```

**Locate this block:**

```rust
        .iter()
        .map(|(name, _)| *name)
        .chain(ACTIONS.iter().map(|(name, _)| *name))
        .chain(["Move"])
        .collect()
}
```

Replace it with:

```rust
        .iter()
        .map(|(name, _)| *name)
        .chain(ACTIONS.iter().map(|(name, _)| *name))
        .collect()
}
```

**Locate this block:**

```rust
    }),
    ("Point", || {
        Action::BeginTool(crate::tool::Shape::Point)
    }),
```

Replace it with:

```rust
    }),
    ("Move", || {
        Action::BeginTool(crate::tool::Shape::Move)
    }),
    ("Point", || {
        Action::BeginTool(crate::tool::Shape::Point)
    }),
```

### 4. `src/commands_tests.rs`

Copy updated parser checks: bare Move starts a tool while invalid positional forms still fail.

**Locate this existing block:**

```rust
        "Fit Selected extra",
        "Move",
        "Move 1 2",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "Fit Selected extra",
        "Move 1 2",
```

## <a id="checkpoint-50d-ownership"></a>5. Cancel previews without consuming document history

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/edit_pre…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary presentation</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original saved scene</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">preserved or one changed…</text>
</svg>
</div>

**What this code does:** Cancel previews without consuming document history.

### 1. `src/editor.rs`

Clear previews after successful document changes, preserve them on errors and fit the displayed scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.scene.bounds() {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self
                            .scene
                            .bounds_of(&self.selected_objects())
```

Replace it with:

```rust
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds() {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self
                            .display_scene()
                            .bounds_of(&self.selected_objects())
```

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let change = self.apply_action(action)?;
        if change == Change::Scene {
            self.snap_index = None;
            if self.edit_preview.take().is_some() {
                self.draft = None;
            }
        }
        Ok(change)
    }
```

Replace it with:

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let keep_edit = matches!(
            &action,
            Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
                | Action::CancelTool
                | Action::Snap(_)
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
                change = Change::Preview;
            }
        }
        if change == Change::Scene {
            self.snap_index = None;
        }
        Ok(change)
    }
```

**Locate this block:**

```rust
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
```

Replace it with:

```rust
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
```

**Locate this block:**

```rust
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
        }
        self.snap_index = None;
        Ok(true)
    }
```

Replace it with:

```rust
        }
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.snap_index = None;
        Ok(true)
    }
```

### 2. `src/edit_preview_tests.rs`

Copy mixed-owner Undo, original-source sharing, saved protobuf, invalid input, zero-move and cancellation checks.

Create the file and type:

```rust
use crate::{
    edit_preview::Preview,
    editor::{Action, Change, Editor},
    tool::Shape,
};
use session_rust::Xform;
use std::rc::Rc;

fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    for action in [
        Action::AddBox,
        Action::AddLine,
        Action::AddPolyline,
        Action::AddPoint,
        Action::SelectAll,
    ] {
        e.apply(action).unwrap();
    }
    e
}
fn state(scene: &crate::scene::Scene) -> Vec<(u32, [f64; 16])> {
    scene
        .objects()
        .iter()
        .map(|r| r.id)
        .chain(scene.lines().iter().map(|r| r.id))
        .chain(scene.paths().iter().map(|r| r.id))
        .chain(scene.points().iter().map(|r| r.id))
        .map(|id| (id.value(), scene.placement(id).unwrap().m))
        .collect()
}

fn begin(e: &mut Editor) {
    e.apply(Action::BeginTool(Shape::Move))
        .unwrap();
    e.apply(Action::ToolPoint([0.; 3])).unwrap();
}

#[test]
fn preview_changes_derived_placements_and_keeps_original_snapshot_and_sources() {
    let mut e = editor();
    let bytes = state(&e.scene);
    let mesh = Rc::clone(&e.scene.objects()[0].mesh);
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let models: Vec<_> = e
        .selected_objects()
        .iter()
        .map(|id| e.scene.placement(*id).unwrap().m)
        .collect();
    begin(&mut e);
    for x in [0.1, 0.2, 0.3] {
        assert_eq!(
            e.apply(Action::ToolHover(Some([x, 0., 0.])))
                .unwrap(),
            Change::Preview
        );
    }
    assert_eq!(state(&e.scene), bytes);
    for (id, model) in e.selected_objects().iter().zip(models) {
        assert_eq!(e.scene.placement(*id).unwrap().m, model);
        assert_eq!(
            e.display_scene().placement(*id).unwrap().m[12],
            model[12] + 0.3
        );
    }
    assert!(Rc::ptr_eq(
        &mesh,
        &e.display_scene().objects()[0].mesh
    ));
    assert!(Rc::ptr_eq(
        &source,
        e.display_scene().objects()[0]
            .geometry()
            .unwrap()
    ));
    e.apply(Action::CancelTool).unwrap();
    assert!(e.edit_preview.is_none());
    assert_eq!(state(&e.scene), bytes);
}

#[test]
fn committing_mixed_owners_is_one_undo_and_cancel_or_zero_move_preserves_redo() {
    let mut e = editor();
    let bytes = state(&e.scene);
    begin(&mut e);
    e.apply(Action::ToolPoint([
        0.123456789012345,
        0.25,
        -0.5,
    ]))
    .unwrap();
    let moved = state(&e.scene);
    assert_ne!(moved, bytes);
    assert!(e.draft.is_none() && e.edit_preview.is_none());
    e.apply(Action::Undo).unwrap();
    assert_eq!(state(&e.scene), bytes);
    begin(&mut e);
    e.apply(Action::ToolHover(Some([0.7, 0., 0.])))
        .unwrap();
    e.apply(Action::CancelTool).unwrap();
    begin(&mut e);
    e.apply(Action::ToolPoint([0.; 3])).unwrap();
    e.apply(Action::Redo).unwrap();
    assert_eq!(state(&e.scene), moved);
    e.apply(Action::Undo).unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.points().is_empty(),
        "Only one Undo entry belongs to the complete Move"
    );
}

#[test]
fn invalid_updates_and_commands_preserve_the_last_valid_preview() {
    let mut e = editor();
    begin(&mut e);
    e.apply(Action::ToolHover(Some([0.25, 0., 0.])))
        .unwrap();
    let matrix = *e.edit_preview.as_ref().unwrap().matrix();
    let bytes = state(&e.scene);
    assert!(
        e.apply(Action::ToolHover(Some([f64::NAN, 0., 0.])))
            .is_err()
    );
    assert!(
        e.apply(Action::Translate([f64::INFINITY, 0., 0.]))
            .is_err()
    );
    assert_eq!(
        *e.edit_preview.as_ref().unwrap().matrix(),
        matrix
    );
    assert_eq!(state(&e.scene), bytes);
    let preview = e.edit_preview.as_mut().unwrap();
    let mut invalid = Xform::identity();
    invalid.m[15] = 0.;
    assert!(preview.set(invalid).is_err());
    assert_eq!(*preview.matrix(), matrix);
}

#[test]
fn snap_switches_restore_hover_but_keep_accepted_base_and_cancel_restores_display() {
    let mut e = editor();
    begin(&mut e);
    e.apply(Action::ToolHover(Some([0.25, 0., 0.])))
        .unwrap();
    e.apply(Action::Snap(crate::snap::Setting::Grid(
        Some(false),
    )))
    .unwrap();
    assert_eq!(
        e.draft.as_ref().unwrap().points,
        vec![[0.; 3]]
    );
    assert_eq!(
        e.edit_preview.as_ref().unwrap().matrix(),
        &Xform::identity().m
    );
    e.apply(Action::ToolHover(Some([0.5, 0., 0.])))
        .unwrap();
    e.apply(Action::ToolHover(None)).unwrap();
    assert_eq!(
        e.edit_preview.as_ref().unwrap().matrix(),
        &Xform::identity().m
    );
    assert_eq!(
        e.apply(Action::CancelTool).unwrap(),
        Change::Preview
    );
    assert!(e.edit_preview.is_none());
}

#[test]
fn preview_rejects_parts_locked_or_cold_sources_and_stale_targets() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene, vec![]).is_err());
    let mut preview = Preview::new(&e.scene, vec![id, id]).unwrap();
    assert_eq!(preview.ids(), &[id]);
    preview
        .set(Xform::translation(1., 0., 0.))
        .unwrap();
    e.scene.translate(id, [2., 0., 0.]).unwrap();
    assert!(
        preview
            .commit(
                &mut e.scene,
                &mut crate::history::History::default()
            )
            .is_err()
    );
    e.apply(Action::SelectAll).unwrap();
    e.selected_part = Some(crate::pick_sources::Part::Face(0));
    assert!(
        e.apply(Action::BeginTool(Shape::Move))
            .is_err()
    );
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
    mesh.is_locked = true;
    let locked = e
        .scene
        .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
        .unwrap();
    assert!(Preview::new(&e.scene, vec![locked]).is_err());
    let bytes = crate::specimen::bytes();
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:preview-cold".into(),
    ));
    e.apply(Action::ReplaceAt(bytes, url))
        .unwrap();
    e.apply(Action::UnloadSources).unwrap();
    e.apply(Action::SelectAll).unwrap();
    assert!(
        e.apply(Action::BeginTool(Shape::Move))
            .is_err()
    );
}

#[test]
fn successful_document_commands_cancel_preview_and_bare_move_uses_the_tool() {
    let mut e = editor();
    begin(&mut e);
    e.apply(Action::ToolHover(Some([0.5, 0., 0.])))
        .unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(e.edit_preview.is_none() && e.draft.is_none());
    assert!(matches!(
        crate::commands::parse("Move").unwrap(),
        crate::commands::Command::Editor(Action::BeginTool(Shape::Move))
    ));
    assert!(matches!(
        crate::commands::parse("Move 1,2,3").unwrap(),
        crate::commands::Command::Editor(Action::Translate([1., 2., 3.]))
    ));
}

#[test]
fn saving_during_a_mesh_preview_uses_original_document_placements() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::SelectAll).unwrap();
    use prost::Message;
    let saved = |scene: &crate::scene::Scene| {
        session_rust::proto::Session::decode(
            crate::document::snapshot(scene)
                .unwrap()
                .as_slice(),
        )
        .unwrap()
    };
    let message = saved(&e.scene);
    begin(&mut e);
    e.apply(Action::ToolHover(Some([0.25, 0., 0.])))
        .unwrap();
    assert_eq!(saved(&e.scene), message);
    assert_ne!(saved(e.display_scene()), message);
    e.apply(Action::CancelTool).unwrap();
    assert_eq!(saved(&e.scene), message);
}
```

### 3. `src/lib.rs`

Register the placement preview and supplied checks.

**Locate this existing block:**

```rust
mod snap_tests;

pub mod edit_preview;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod snap_tests;

pub mod edit_preview;

#[cfg(test)]
mod edit_preview_tests;
```

## Build & Run

![Recorded visual output for Lesson 7.5 (50d-ownership)](../../../screenshots/journey/50d-ownership.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 7.5
npm --prefix ../session_tests run course -- save 7.5
```

Continue with [7.6 · Handles, numeric edits and source-part deformation](../../consolidation/51-handles.md).
