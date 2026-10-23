# 6.4 · Ordered multi-selection and atomic edits

**Goal:** Collect multiple objects with Shift-click and commands. · **Steps:** 5 · **Source:** [engine/gpu/pick.rs](../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../src/app/walk/mesh_topology.rs) (`src/engine/gpu/pick.rs,src/app/selecting.rs,src/shaders/scene.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 6.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/object_selection.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">5 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/pick.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/selecting.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Preserve unique selection…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Ordered multi-selection and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Priority &amp; 256B Readback</text>
  <rect x="880" y="54" width="188" height="36" rx="5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Point &gt; Wire &gt; Face</text>
  <rect x="880" y="100" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">256B Aligned Staging</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Collect multiple objects…</text>
</svg>
</div>

## <a id="checkpoint-45-order"></a>1. Preserve unique selections in pick order

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/object_selection.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid source owners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original pick order</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stable primary row</text>
</svg>
</div>

**What this code does:** Preserve unique selections in pick order.

### 1. `src/object_selection.rs`

Keep unique current owners in their original pick order.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};

#[derive(Default)]
pub struct Objects {
    order: Vec<ObjectId>,
}

impl Objects {
    // Membership is unique, while pick order is preserved because later curve and element tools interpret their inputs in the order picked.
    pub fn set(
        &mut self,
        scene: &Scene,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if !additive {
            self.order.clear();
        }
        for id in rows {
            if scene.pick_id(id.value()).is_some() && !self.order.contains(&id) {
                self.order.push(id);
            }
        }
        self.retain(scene);
    }

    pub fn retain(&mut self, scene: &Scene) {
        self.order
            .retain(|id| scene.pick_id(id.value()).is_some());
    }
    pub fn rows(&self) -> &[ObjectId] {
        &self.order
    }
    pub fn primary(&self) -> Option<ObjectId> {
        self.order
            .iter()
            .copied()
            .min_by_key(|id| id.value())
    }
    pub fn clear(&mut self) {
        self.order.clear();
    }
}
```

### 2. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod selection_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod selection_tests;

pub mod object_selection;
```

### 3. `src/scene.rs`

Collect selected bounds and move mixed original row placements.

**Locate this existing block:**

```rust
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids {
            if let Some(next) = self.selected_bounds(*id) {
                if let Some(b) = &mut bounds {
                    b.include(next.min);
                    b.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        bounds
    }

    pub fn translate(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() {
                return Err("Reload editable sources before Move");
            }
            let model = &shift * &row.model;
            return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_line(id, model);
        }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_path(id, model);
        }
        if let Some(row) = self.points().iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
```

### 4. `src/object_selection_tests.rs`

Copy unique order and additive miss checks.

Create the file and type:

```rust
use crate::{object_selection::Objects, scene::Scene};

#[test]
fn repeated_and_blank_additive_picks_keep_unique_original_order() {
    let scene = Scene::demo();
    let a = scene.objects()[0].id;
    let b = scene.objects()[1].id;
    let mut selected = Objects::default();
    selected.set(&scene, [b, a, b], false);
    assert_eq!(selected.rows(), &[b, a]);
    assert_eq!(selected.primary(), Some(a));
    selected.set(&scene, [], true);
    assert_eq!(selected.rows(), &[b, a]);
    selected.set(&scene, [], false);
    assert!(selected.rows().is_empty());
}
```

### 5. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
mod selection_tests;

pub mod object_selection;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod selection_tests;

pub mod object_selection;

#[cfg(test)]
mod object_selection_tests;
```

## <a id="checkpoint-45a-batch"></a>2. Commit mixed selection edits atomically

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/rehydrat…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected original rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked placements</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete commit or rollba…</text>
</svg>
</div>

**What this code does:** Commit mixed selection edits atomically.

### 1. `src/editor.rs`

Own ordered selections and commit Move or Delete as one original-row transaction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.selected = self
```

Replace it with:

```rust
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self
```

**Locate this block:**

```rust
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self
                            .selected
                            .and_then(|id| self.scene.selected_bounds(id))
                        {
                            self.camera.fit(&bounds);
                        }
```

Replace it with:

```rust
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self
                            .scene
                            .bounds_of(&self.selected_objects())
                            .or_else(|| {
                                self.selected_part
                                    .and(self.selected)
                                    .and_then(|id| self.scene.selected_bounds(id))
                            })
                        {
                            self.camera.fit(&bounds);
                        }
```

**Locate this block:**

```rust
                self.selected_part = None;
                self.selected = self.scene.next(self.selected);
            }
            Action::Delete => {
                if self.selected_part.is_some() {
                    return Err("Use Select Object before Delete");
                }
                return match self.selected {
                    Some(id) => self.delete_object(id),
                    None => Ok(Change::View),
                };
            }
            Action::Undo => {
                self.history.undo(&mut self.scene);
            }
```

Replace it with:

```rust
                self.selected_part = None;
                self.selected = self.scene.next(self.selected);
            }
            Action::Delete => return self.delete_selection(),
            Action::Undo => {
                self.history.undo(&mut self.scene);
            }
```

**Locate this block:**

```rust
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => {
                let id = self
                    .whole_selection()
                    .ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
            Action::ToggleExtra => self
                .history
                .edit(&mut self.scene, Scene::toggle_extra),
```

Replace it with:

```rust
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self
                .history
                .edit(&mut self.scene, Scene::toggle_extra),
```

**Locate this block:**

```rust
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
```

Replace it with:

```rust
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
```

**Locate this block:**

```rust
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.selected == Some(id) {
            self.selected = None;
            self.selected_part = None;
        }
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
```

Replace it with:

```rust
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            return Vec::new();
        }
        if self.objects.primary() == self.selected {
            self.objects.rows().to_vec()
        } else {
            self.selected
                .into_iter()
                .filter(|id| self.scene.pick_id(id.value()).is_some())
                .collect()
        }
    }

    pub fn select_objects(
        &mut self,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if additive {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
        }
        self.selected_part = None;
        self.objects.set(&self.scene, rows, additive);
        self.selected = self.objects.primary();
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
            self.selected = self.objects.primary();
        } else {
            self.objects.clear();
        }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Err("Select an object before Move");
        }
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    scene.translate(id, offset)?;
                }
                Ok(())
            })?;
        Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() {
            return Err("Use Select Object before Delete");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Ok(Change::View);
        }
        if self
            .scene
            .objects()
            .iter()
            .any(|r| ids.contains(&r.id) && r.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        self.objects.clear();
        self.selected = None;
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
```

**Locate this block:**

```rust
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
```

Replace it with:

```rust
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
```

**Locate this block:**

```rust
    pub pick_mode: crate::pick_sources::Mode,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
```

Replace it with:

```rust
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
```

### 2. `src/rehydrate_tests.rs`

Copy the existing hydration precision proof using its explicit row fixture placement, without selecting a locked row through input.

**Locate this existing block:**

```rust
        .unwrap();
    editor.apply(Action::SelectNext).unwrap();
    editor
        .apply(Action::Translate([0.25, 0.0, 0.0]))
        .unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .unwrap();
    let id = editor.scene.objects()[0].id;
    editor
        .move_object(id, [0.25, 0.0, 0.0])
        .unwrap();
```

## <a id="checkpoint-45b-membership"></a>3. Connect multi-hit state and highlight every selected row

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/highligh…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">plain or additive target</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exclusive part mode</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared geometry with sele…</text>
</svg>
</div>

**What this code does:** Connect multi-hit state and highlight every selected row.

### 1. `src/editor.rs`

Adopt unique multi-hit membership and repair it after source changes.

**Locate this existing block:**

```rust
    PickedTarget(Option<crate::pick_sources::Target>),
    PickMode(crate::pick_sources::Mode),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    AddBox,
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
}

pub struct Editor {
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
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            return Vec::new();
        }
        if self.objects.primary() == self.selected {
            self.objects.rows().to_vec()
        } else {
            self.selected
                .into_iter()
                .filter(|id| self.scene.pick_id(id.value()).is_some())
                .collect()
        }
    }

    pub fn select_objects(
        &mut self,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if additive {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
        }
        self.selected_part = None;
        self.objects.set(&self.scene, rows, additive);
        self.selected = self.objects.primary();
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
            self.selected = self.objects.primary();
        } else {
            self.objects.clear();
        }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Err("Select an object before Move");
        }
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    scene.translate(id, offset)?;
                }
                Ok(())
            })?;
        Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() {
            return Err("Use Select Object before Delete");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Ok(Change::View);
        }
        if self
            .scene
            .objects()
            .iter()
            .any(|r| ids.contains(&r.id) && r.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        self.objects.clear();
        self.selected = None;
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
                owner,
                part: self
                    .selected_part
                    .unwrap_or(crate::pick_sources::Part::Object),
            })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> {
        self.selected
            .filter(|_| self.selected_part.is_none())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Picked(id) => {
                self.selected =
                    id.filter(|id| self.scene.pick_id(id.value()).is_some());
                self.selected_part = None;
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.selected = target.map(|target| target.owner);
                self.selected_part = target
                    .map(|target| target.part)
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
            }
            Action::Pick(screen) => {
                self.selected_part = None;
                self.selected = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ]);
                    line.width = 6.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line =
                        session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(
                    crate::normal_example::specimen(mode),
                )?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddColours(mode) => {
                let prepared =
                    crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(
                    false,
                    2,
                    &[
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.6, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ],
                );
                curve.width = 5.0;
                curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared =
                    crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0;
                point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared =
                    crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_point(prepared).map(|_| ())
                    })?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0;
                line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared =
                    crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_line(prepared).map(|_| ())
                    })?;
            }
            Action::AddBox => {
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self
                .history
                .edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => {
                self.selected_part = None;
                self.selected = self.scene.next(self.selected);
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany {
        target: Option<crate::pick_sources::Target>,
        additive: bool,
    },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    AddBox,
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
}

pub struct Editor {
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
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            return Vec::new();
        }
        if self.objects.primary() == self.selected {
            self.objects.rows().to_vec()
        } else {
            self.selected
                .into_iter()
                .filter(|id| self.scene.pick_id(id.value()).is_some())
                .collect()
        }
    }

    pub fn select_objects(
        &mut self,
        rows: impl IntoIterator<Item = ObjectId>,
        additive: bool,
    ) {
        if additive {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
        }
        self.selected_part = None;
        self.objects.set(&self.scene, rows, additive);
        self.selected = self.objects.primary();
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
            self.selected = self.objects.primary();
        } else {
            self.objects.clear();
        }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Err("Select an object before Move");
        }
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    scene.translate(id, offset)?;
                }
                Ok(())
            })?;
        Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() {
            return Err("Use Select Object before Delete");
        }
        let ids = self.selected_objects();
        if ids.is_empty() {
            return Ok(Change::View);
        }
        if self
            .scene
            .objects()
            .iter()
            .any(|r| ids.contains(&r.id) && r.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        self.objects.clear();
        self.selected = None;
        Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected
            .map(|owner| crate::pick_sources::Target {
                owner,
                part: self
                    .selected_part
                    .unwrap_or(crate::pick_sources::Part::Object),
            })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> {
        self.selected
            .filter(|_| self.selected_part.is_none())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object {
                    return self.apply(Action::PickedTarget(target));
                }
                self.select_objects(
                    target
                        .filter(|t| {
                            t.part == crate::pick_sources::Part::Object
                                && t.valid(&self.scene)
                        })
                        .map(|t| t.owner),
                    additive,
                );
            }
            Action::SelectAll => {
                let ids: Vec<_> = self
                    .scene
                    .objects()
                    .iter()
                    .map(|r| r.id)
                    .chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id))
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .collect();
                self.select_objects(ids, false);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) =
                    target.filter(|t| t.part == crate::pick_sources::Part::Object)
                {
                    self.objects
                        .set(&self.scene, [target.owner], false);
                }
                self.selected_part = target
                    .map(|target| target.part)
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
                self.objects.clear();
            }
            Action::Pick(screen) => {
                let id = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ]);
                    line.width = 6.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line =
                        session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0;
                    line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(
                    crate::normal_example::specimen(mode),
                )?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddColours(mode) => {
                let prepared =
                    crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(
                    false,
                    2,
                    &[
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.6, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ],
                );
                curve.width = 5.0;
                curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared =
                    crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0;
                point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared =
                    crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_point(prepared).map(|_| ())
                    })?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0;
                line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared =
                    crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_line(prepared).map(|_| ())
                    })?;
            }
            Action::AddBox => {
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self
                .history
                .edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => {
                let id = self.scene.next(self.selected);
                self.select_objects(id, false);
            }
```

### 2. `src/highlight.rs`

Present every selected original row while sharing primary geometry.

**Locate this existing block:**

```rust
    }

    pub fn view(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn objects(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let yellow = [1., 0.75, 0.05, 1.];
        self.strokes.set(
            device,
            scene
                .lines()
                .iter()
                .filter(|r| ids.contains(&r.id) && r.visible())
                .map(|r| crate::stroke::Stroke {
                    colour: yellow,
                    width: r.prepared.display.width + 2.,
                    ..r.stroke()
                }),
        )?;
        self.paths.set_chains(
            device,
            scene
                .paths()
                .iter()
                .filter(|r| ids.contains(&r.id) && r.visible())
                .flat_map(|r| {
                    r.segments().into_iter().map(|mut s| {
                        s.stroke.colour = yellow;
                        s.stroke.width += 2.;
                        s
                    })
                }),
        )?;
        self.markers.set_points(
            device,
            scene
                .points()
                .iter()
                .filter(|r| ids.contains(&r.id) && r.visible())
                .map(|r| crate::marker::Marker {
                    colour: yellow,
                    diameter: r.prepared.display.diameter + 4.,
                    ..r.marker()
                }),
        )?;
        Ok(())
    }

    pub fn view(
```

### 3. `src/renderer.rs`

Present every selected original row while sharing primary geometry.

**Locate this existing block:**

```rust
    }

    pub fn highlight_bytes(&self) -> u64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn select_objects(
        &mut self,
        scene: &Scene,
        ids: &[ObjectId],
    ) -> Result<(), &'static str> {
        for mesh in &mut self.meshes {
            if let Some(row) = scene
                .objects()
                .iter()
                .find(|r| r.id == mesh.id)
            {
                let [calls, bytes] =
                    mesh.update(&self.queue, row, ids.contains(&row.id));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
            }
        }
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
        Ok(())
    }

    pub fn highlight_bytes(&self) -> u64 {
```

### 4. `src/multi_selection_tests.rs`

Copy ordering, exclusive parts, mixed transactions, rollback, eligibility and release checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Target},
};
use std::rc::Rc;

#[test]
fn additive_hits_preserve_pick_order_and_exclusive_parts_cannot_mix_with_objects() {
    let mut editor = Editor::default();
    let ids: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|r| r.id)
        .collect();
    let hit = |id| {
        Some(Target {
            owner: id,
            part: Part::Object,
        })
    };
    editor
        .apply(Action::PickedMany {
            target: hit(ids[1]),
            additive: false,
        })
        .unwrap();
    editor
        .apply(Action::PickedMany {
            target: hit(ids[0]),
            additive: true,
        })
        .unwrap();
    editor
        .apply(Action::PickedMany {
            target: hit(ids[1]),
            additive: true,
        })
        .unwrap();
    editor
        .apply(Action::PickedMany {
            target: None,
            additive: true,
        })
        .unwrap();
    assert_eq!(
        editor.selected_objects(),
        vec![ids[1], ids[0]]
    );
    assert_eq!(editor.selected, Some(ids[0]));
    editor.apply(Action::FitSelected).unwrap();
    let bounds = editor.scene.bounds_of(&ids).unwrap();
    assert_eq!(editor.camera.target, bounds.centre());
    editor
        .apply(Action::PickMode(Mode::Face))
        .unwrap();
    let face = Target {
        owner: ids[1],
        part: Part::Face(0),
    };
    editor
        .apply(Action::PickedMany {
            target: Some(face),
            additive: true,
        })
        .unwrap();
    assert_eq!(editor.selection(), Some(face));
    assert!(editor.selected_objects().is_empty());
    editor
        .apply(Action::PickMode(Mode::Object))
        .unwrap();
    editor
        .apply(Action::PickedMany {
            target: None,
            additive: false,
        })
        .unwrap();
    assert!(editor.selected_objects().is_empty());
}

#[test]
fn one_move_and_delete_transaction_cover_mixed_original_rows_without_copying_sources() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    for action in [
        Action::AddBox,
        Action::AddLine,
        Action::AddPoint,
        Action::AddPolyline,
    ] {
        editor.apply(action).unwrap();
    }
    let mesh = Rc::clone(editor.scene.objects()[0].geometry().unwrap());
    let line = Rc::clone(&editor.scene.lines()[0].prepared.source);
    let point = Rc::clone(&editor.scene.points()[0].prepared.source);
    editor.apply(Action::SelectAll).unwrap();
    let ids = editor.selected_objects();
    assert_eq!(ids.len(), 4);
    let bounds = editor.scene.bounds_of(&ids).unwrap();
    editor
        .apply(Action::Translate([2., 3., 4.]))
        .unwrap();
    let moved = editor.scene.bounds_of(&ids).unwrap();
    for i in 0..3 {
        assert!((moved.min[i] - bounds.min[i] - [2., 3., 4.][i]).abs() < 1e-12);
    }
    assert!(Rc::ptr_eq(
        &mesh,
        editor.scene.objects()[0].geometry().unwrap()
    ));
    assert!(Rc::ptr_eq(
        &line,
        &editor.scene.lines()[0].prepared.source
    ));
    assert!(Rc::ptr_eq(
        &point,
        &editor.scene.points()[0].prepared.source
    ));
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.bounds_of(&ids).unwrap().min,
        bounds.min
    );
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor.scene.bounds_of(&ids).unwrap().min,
        moved.min
    );
    editor.apply(Action::Delete).unwrap();
    assert!(editor.scene.bounds().is_none());
    assert!(editor.selected_objects().is_empty());
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.bounds_of(&ids).unwrap().min,
        moved.min
    );
    editor.apply(Action::Redo).unwrap();
    assert!(editor.scene.bounds().is_none());
}

#[test]
fn an_invalid_batch_rolls_back_earlier_moves_and_preserves_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddBox).unwrap();
    let hot = editor.scene.objects()[0].id;
    let old = editor.scene.objects()[0].model.m;
    editor
        .apply(Action::ImportAt(
            crate::specimen::bytes(),
            Rc::new(crate::reload_url::ReloadUrl::new(
                "test:multi".into(),
            )),
        ))
        .unwrap();
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::Redo).unwrap();
    let cold = editor.scene.objects()[1].id;
    editor.apply(Action::UnloadSources).unwrap();
    editor.select_objects([hot, cold], false);
    assert!(
        editor
            .apply(Action::Translate([1., 0., 0.]))
            .is_err()
    );
    assert_eq!(editor.scene.objects()[0].model.m, old);
    assert_eq!(editor.selected_objects(), vec![hot, cold]);
    assert!(editor.apply(Action::Delete).is_err());
    assert_eq!(editor.scene.objects().len(), 4);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.objects().len(),
        1,
        "Refused batch adds no history entry"
    );
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 4);
}

#[test]
fn select_all_filters_locked_and_hidden_rows_and_close_retains_only_ids() {
    let mut editor = Editor::default();
    editor.apply(Action::AddPoint).unwrap();
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    Rc::make_mut(&mut editor.scene.objects_mut()[0].metadata).is_locked = true;
    Rc::make_mut(&mut editor.scene.objects_mut()[1].metadata).is_visible = false;
    editor.apply(Action::SelectAll).unwrap();
    assert_eq!(
        editor.selected_objects(),
        vec![editor.scene.points()[0].id]
    );
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert!(editor.selected_objects().is_empty());
}
```

### 5. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod object_selection_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod object_selection_tests;

#[cfg(test)]
mod multi_selection_tests;
```

### 6. `src/pick_gpu_tests.rs`

Copy actual multiple-mesh pixels, shared geometry and preserved pick ownership.

**Locate this existing block:**

```rust
    renderer.set_scene(&editor.scene, None);
    assert_eq!(renderer.highlight_bytes(), 0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    renderer.set_scene(&editor.scene, None);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo();
    scene.clear();
    let panel = |x: f64| {
        PreparedMesh::triangle(
            [
                [x - 0.25, -0.4, 0.5],
                [x + 0.25, -0.4, 0.5],
                [x, 0.4, 0.5],
            ],
            [0.5, 0.5, 0.5],
        )
        .unwrap()
    };
    let a = scene.insert(panel(-0.4)).unwrap();
    let b = scene.insert(panel(0.4)).unwrap();
    let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| {
        bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec()
    };
    for x in [96, 224] {
        let p = pixel(&selected, x);
        assert!(p[0] > p[1] && p[1] > p[2]);
    }
    renderer
        .select_objects(&scene, &[b])
        .unwrap();
    let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96));
    assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(
        &renderer.stats()[..2],
        &geometry[..2],
        "Only selected settings change, not primary geometry"
    );
    for (x, id) in [(-0.4, a), (0.4, b)] {
        renderer
            .request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
}
```

### 7. `src/browser.rs`

Refresh cached selection settings after Select All, even when the editor returns a view-only change.

**Locate this existing block:**

```rust
            if let Some(action) = action {
                if matches!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            if let Some(action) = action {
                highlight_changed |= matches!(&action, Action::SelectAll);
                if matches!(
```

## <a id="checkpoint-45bb-cycle"></a>4. Cycle every eligible geometry owner

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/selection…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">all original geometry lan…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stable ID order</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">next or wrap</text>
</svg>
</div>

**What this code does:** Cycle visible unlocked mesh, line, path and point rows in stable owner order.

### 1. `src/scene.rs`

Cycle all original owners after applying visibility and locking.

**Locate this existing block:**

```rust
    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        let start = self
            .objects
            .iter()
            .position(|object| Some(object.id) == current)
            .map_or(0, |index| index + 1);
        (0..self.objects.len())
            .map(|offset| &self.objects[(start + offset) % self.objects.len()])
            .find(|row| row.visible())
            .map(|row| row.id)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        let mut ids: Vec<_> = self
            .objects
            .iter()
            .filter(|r| r.visible() && !r.metadata.is_locked)
            .map(|r| r.id)
            .chain(
                self.lines
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.is_locked)
                    .map(|r| r.id),
            )
            .chain(
                self.paths
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.locked())
                    .map(|r| r.id),
            )
            .chain(
                self.points
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.is_locked)
                    .map(|r| r.id),
            )
            .collect();
        ids.sort_unstable_by_key(|id| id.value());
        if ids.is_empty() {
            return None;
        }
        let start = ids
            .iter()
            .position(|id| Some(*id) == current)
            .map_or(0, |i| i + 1);
        Some(ids[start % ids.len()])
    }
```

### 2. `src/selection_cycle_tests.rs`

Copy mixed-owner, visibility, lock and wraparound checks.

Create the file and type:

```rust
use crate::{prepared::PreparedMesh, scene::Scene};
use std::rc::Rc;

#[test]
fn cycling_skips_locked_and_hidden_rows_and_wraps_across_geometry_lanes() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
    mesh.is_locked = true;
    let locked = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    let mesh = scene
        .insert(PreparedMesh::new(session_rust::Mesh::create_box(1., 1., 1.)).unwrap())
        .unwrap();
    let mut line = session_rust::Line::new(0., 0., 0., 1., 0., 0.);
    line.is_locked = true;
    scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0., 0., 0.,
            )))
            .unwrap(),
        )
        .unwrap();
    let path = scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(vec![
                    session_rust::Point::new(0., 0., 0.),
                    session_rust::Point::new(1., 0., 0.),
                ]),
            ))
            .unwrap(),
        )
        .unwrap();
    let mut hidden = session_rust::Point::new(0., 0., 0.);
    hidden.is_visible = false;
    scene
        .insert_point(crate::marker::PreparedPoint::new(Rc::new(hidden)).unwrap())
        .unwrap();
    assert_eq!(scene.next(None), Some(mesh));
    assert_eq!(scene.next(Some(locked)), Some(mesh));
    assert_eq!(scene.next(Some(mesh)), Some(point));
    assert_eq!(scene.next(Some(point)), Some(path));
    assert_eq!(scene.next(Some(path)), Some(mesh));
    scene.clear();
    assert_eq!(scene.next(None), None);
}
```

### 3. `src/lib.rs`

Register the copied original-owner cycling check.

**Locate this existing block:**

```rust
#[cfg(test)]
mod multi_selection_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod multi_selection_tests;

#[cfg(test)]
mod selection_cycle_tests;
```

## <a id="checkpoint-45c-input"></a>5. Collect multiple objects with Shift-click and commands

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">accepted pointer modifiers</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current original hit</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/pick…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one batch command</text>
</svg>
</div>

**What this code does:** Collect multiple objects with Shift-click and commands.

### 1. `src/browser.rs`

Capture additive pointer ownership and route ordered selection, batch edits and inspection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-part-usage",
        &serde_json::to_string(&renderer.part_usage()).unwrap(),
    )?;
```

Replace it with:

```rust
        "data-part-usage",
        &serde_json::to_string(&renderer.part_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-selected-ids",
        &serde_json::to_string(
            &editor
                .selected_objects()
                .iter()
                .map(|id| id.value())
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    )?;
```

**Locate this block:**

```rust
            }
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) =
                    renderer.set_highlight(&editor.scene, editor.selection())
                {
                    report(error);
                    return;
```

Replace it with:

```rust
            }
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
                    return;
```

**Locate this block:**

```rust
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                let waiting = crate::edit_intent::Intent::capture(
                    &action,
                    editor.whole_selection(),
                )
                .map_err(str::to_owned)
                .and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(
```

Replace it with:

```rust
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                let waiting = if editor.selected_objects().len() > 1 {
                    Ok(None)
                } else {
                    crate::edit_intent::Intent::capture(
                        &action,
                        editor.whole_selection(),
                    )
                }
                .map_err(str::to_owned)
                .and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(
```

**Locate this block:**

```rust
            } else {
                match action {
                    Some(Action::Pick(screen)) => {
                        if let Err(error) =
                            renderer.request_pick(screen, editor.camera.uniform())
                        {
```

Replace it with:

```rust
            } else {
                match action {
                    Some(Action::Pick(screen)) => {
                        pick_additive.set(
                            event
                                .dyn_ref::<web_sys::PointerEvent>()
                                .is_some_and(|p| {
                                    p.shift_key() && !p.ctrl_key() && !p.meta_key()
                                }),
                        );
                        if let Err(error) =
                            renderer.request_pick(screen, editor.camera.uniform())
                        {
```

**Locate this block:**

```rust
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => {
                        Some(Action::PickedTarget(hit.and_then(|hit| {
                            renderer.resolve_pick(&editor.scene, hit)
                        })))
                    }
                    Some(Err(error)) => {
                        report(error);
                        None
```

Replace it with:

```rust
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::PickedMany {
                        target: hit
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit)),
                        additive: pick_additive.get(),
                    }),
                    Some(Err(error)) => {
                        report(error);
                        None
```

**Locate this block:**

```rust
                        "example arrow" => Action::AddArrow,
                        "example triangle" => Action::ToggleExtra,
                        "select next" => Action::SelectNext,
                        "select object" => {
                            Action::PickMode(crate::pick_sources::Mode::Object)
                        }
```

Replace it with:

```rust
                        "example arrow" => Action::AddArrow,
                        "example triangle" => Action::ToggleExtra,
                        "select next" => Action::SelectNext,
                        "select all" => Action::SelectAll,
                        "select none" => Action::Picked(None),
                        "select object" => {
                            Action::PickMode(crate::pick_sources::Mode::Object)
                        }
```

**Locate this block:**

```rust
                }
            } else if let Some(line) = line {
                if line == "selection" {
                    panel.result(&editor.selection().map_or_else(
                        || "Nothing selected".to_owned(),
                        |target| target.describe(),
                    ));
                    None
                } else if line == "reload sources" {
                    let result = crate::browser_reload::start(
```

Replace it with:

```rust
                }
            } else if let Some(line) = line {
                if line == "selection" {
                    let rows = editor.selected_objects();
                    panel.result(&if rows.len() > 1 {
                        format!(
                            "Objects {:?}",
                            rows.iter()
                                .map(|id| id.value())
                                .collect::<Vec<_>>()
                        )
                    } else {
                        editor.selection().map_or_else(
                            || "Nothing selected".to_owned(),
                            |target| target.describe(),
                        )
                    });
                    None
                } else if line == "reload sources" {
                    let result = crate::browser_reload::start(
```

**Locate this block:**

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
```

Replace it with:

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
```

**Locate this block:**

```rust
            "Select Next",
            "Select Object",
            "Select Face",
            "Select Edge",
```

Replace it with:

```rust
            "Select Next",
            "Select All",
            "Select None",
            "Select Object",
            "Select Face",
            "Select Edge",
```

## Build & Run

![Recorded visual output for Lesson 6.4 (45c-input)](../../screenshots/journey/45c-input.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 6.4
npm --prefix ../session_tests run course -- save 6.4
```

Continue with [7.1 · Active tools own their command input](46-tool-panel.md).
