# 6.4 · Ordered multi-selection and atomic edits

**Steps:** 5 · **Source:** [engine/gpu/pick.rs](../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../src/app/walk/mesh_topology.rs)

## <a id="checkpoint-45-order"></a>1. Preserve unique selections in pick order

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-6e696b9c" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-6e696b9c" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-6e696b9c" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-6e696b9c" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-6e696b9c" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">45-ORDER · SELECTION ORDER</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Insertion-Ordered Selection Vec (1st, 2nd, ...) &amp; Tab Cycling Through Ranked Overlaps</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Shift+Click toggles membership · Tab cycles rank[(i+1) mod k]</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-6e696b9c)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Insertion-Ordered Selection Vec</text>
  <rect x="44" y="84" width="86" height="64" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="87" y="110" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">#0 (1st)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="87" y="128" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Base A</text>
  <line x1="130" y1="116" x2="148" y2="116" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-6e696b9c)"/>
  <rect x="152" y="84" width="86" height="64" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="195" y="110" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">#1 (2nd)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="195" y="128" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Cutter B</text>
  <line x1="238" y1="116" x2="256" y2="116" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-6e696b9c)"/>
  <rect x="260" y="84" width="72" height="64" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="296" y="110" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">#2 (3rd)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="174" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#111111">Order matters: A \ B != B \ A</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-6e696b9c)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-6e696b9c)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Tab Cycling Ranked Candidates at Same Pixel</text>
  <rect x="414" y="76" width="292" height="104" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Candidates in 5×5 Window:</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="120" text-anchor="middle" font-family="monospace" font-size="9" fill="#dc2626">[0] Vertex #4  &lt;-- Click 1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="140" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">[1] Edge #12   &lt;-- Press Tab</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="160" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">[2] Face #2    &lt;-- Press Tab again</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-6e696b9c)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-6e696b9c)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Ordered CAD Multi-Selection</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="836" y="88" width="76" height="76" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="874" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">1st</text>
  <rect x="948" y="88" width="76" height="76" fill="#dbeafe" stroke="#2563eb" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="986" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">2nd</text>
</svg>
</div>

### 1. `src/object_selection.rs`

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

### 1. `src/editor.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">5×5 Pixel Readback Window Around Cursor (cx, cy)</text>
  <rect x="88" y="48" width="164" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <line x1="121" y1="48" x2="121" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="154" y1="48" x2="154" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="187" y1="48" x2="187" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="220" y1="48" x2="220" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="88" y1="74" x2="252" y2="74" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="88" y1="101" x2="252" y2="101" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="88" y1="128" x2="252" y2="128" stroke="#cbd5e1" stroke-width="1"/>
  <rect x="154" y="74" width="33" height="27" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <rect x="121" y="101" width="33" height="27" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Score = Priority(Point &gt; Edge &gt; Face) * 100 - d_px²</text>
  <circle cx="464" cy="96" r="14" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="464" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">1. Point</text>
  <line x1="532" y1="112" x2="588" y2="80" stroke="#2563eb" stroke-width="5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">2. Edge</text>
  <polygon points="628,116 658,74 688,116" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="658" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">3. Face</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Sub-Object Highlight (Vertex / Edge / Face)</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="856,138 934,60 1012,138" fill="#f1f5f9" stroke="#111111" stroke-width="1.75"/>
  <line x1="856" y1="138" x2="934" y2="60" stroke="#2563eb" stroke-width="4"/>
  <circle cx="934" cy="60" r="7.5" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
</svg>
</div>

### 1. `src/editor.rs`

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

**Locate this existing block:**

```rust
        if math_view.uploaded {let view=math_view.scene(&editor.scene);let _=renderer.set_highlight(&view,editor.selection());}
        let _=canvas.set_attribute("data-math-display",&serde_json::json!({"pending":pending,"bytes":math_view.bytes,"generations":math_view.generations,"limited":math_view.limited,"qualities":math_view.qualities()}).to_string());
        if pending {math_view.schedule(&window,&canvas);}
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
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if event.type_()=="viewer-math" {math_view.event();}
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
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
                        math_view.upload(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
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
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "selection" {
                panel.result(&editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()));
                None
            } else if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "report" || line == "report previous" {
                let output = if line.ends_with(" previous") { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example line" => Action::AddLine,
                    "example point" => Action::AddPoint,
                    "example curve" => Action::AddCurve,
                    "example object color" => Action::AddColours(crate::colour::Mode::Object),
                    "example point colors" => Action::AddColours(crate::colour::Mode::Points),
                    "example face colors" => Action::AddColours(crate::colour::Mode::Faces),
                    "view normals on" => Action::NormalView(true),
                    "view normals off" => Action::NormalView(false),
                    "view grid on" => Action::Grid(Some(crate::grid::Settings::default())),
                    "view grid off" => Action::Grid(None),
                    "example sharp" => Action::AddNormals(crate::normal_example::Mode::Sharp),
                    "example smooth" => Action::AddNormals(crate::normal_example::Mode::Smooth),
                    "example winding" => Action::AddNormals(crate::normal_example::Mode::Winding),


                    "controls on" => Action::Controls(true),
                    "controls off" => Action::Controls(false),
                    "example polyline" => Action::AddPolyline,
                    "example arrow" => Action::AddArrow,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "select object" => Action::PickMode(crate::pick_sources::Mode::Object),
                    "select face" => Action::PickMode(crate::pick_sources::Mode::Face),
                    "select edge" => Action::PickMode(crate::pick_sources::Mode::Edge),
                    "select control" => Action::PickMode(crate::pick_sources::Mode::Control),
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        let action = if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => Some(Action::PickedTarget(hit.and_then(|hit| renderer.resolve_pick(&editor.scene, hit)))),
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) => {
                if let Err(error) = renderer.request_pick(screen, editor.camera.uniform()) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.whole_selection())
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    math_view.upload(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        renderer.set_pick_mode(editor.pick_mode);
        if highlight_changed { if let Err(error) = renderer.set_highlight(&editor.scene, editor.selection()) { report(error); return; } }
        if let Err(error) = renderer.set_grid(editor.camera.grid) {
            report(error); return;
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
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
            let pending=if false {math_view.invalidate(&editor.scene);false} else {math_view.refresh(&mut renderer,&editor.scene,&editor.camera,[config.width,config.height],editor.whole_selection())};
        if math_view.uploaded {let view=math_view.scene(&editor.scene);let _=renderer.set_highlight(&view,editor.selection());}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if math_view.uploaded {let view=math_view.scene(&editor.scene);let _=renderer.set_highlight(&view,editor.selection());let _=renderer.select_objects(&view,&editor.selected_objects());}
        let _=canvas.set_attribute("data-math-display",&serde_json::json!({"pending":pending,"bytes":math_view.bytes,"generations":math_view.generations,"limited":math_view.limited,"qualities":math_view.qualities()}).to_string());
        if pending {math_view.schedule(&window,&canvas);}
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
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if event.type_()=="viewer-math" {math_view.event();}
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
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
                        math_view.upload(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
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
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "selection" {
                panel.result(&editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()));
                None
            } else if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "report" || line == "report previous" {
                let output = if line.ends_with(" previous") { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example line" => Action::AddLine,
                    "example point" => Action::AddPoint,
                    "example curve" => Action::AddCurve,
                    "example object color" => Action::AddColours(crate::colour::Mode::Object),
                    "example point colors" => Action::AddColours(crate::colour::Mode::Points),
                    "example face colors" => Action::AddColours(crate::colour::Mode::Faces),
                    "view normals on" => Action::NormalView(true),
                    "view normals off" => Action::NormalView(false),
                    "view grid on" => Action::Grid(Some(crate::grid::Settings::default())),
                    "view grid off" => Action::Grid(None),
                    "example sharp" => Action::AddNormals(crate::normal_example::Mode::Sharp),
                    "example smooth" => Action::AddNormals(crate::normal_example::Mode::Smooth),
                    "example winding" => Action::AddNormals(crate::normal_example::Mode::Winding),


                    "controls on" => Action::Controls(true),
                    "controls off" => Action::Controls(false),
                    "example polyline" => Action::AddPolyline,
                    "example arrow" => Action::AddArrow,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "select object" => Action::PickMode(crate::pick_sources::Mode::Object),
                    "select face" => Action::PickMode(crate::pick_sources::Mode::Face),
                    "select edge" => Action::PickMode(crate::pick_sources::Mode::Edge),
                    "select control" => Action::PickMode(crate::pick_sources::Mode::Control),
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        let action = if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => Some(Action::PickedTarget(hit.and_then(|hit| renderer.resolve_pick(&editor.scene, hit)))),
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) => {
                if let Err(error) = renderer.request_pick(screen, editor.camera.uniform()) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.whole_selection())
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    math_view.upload(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        renderer.set_pick_mode(editor.pick_mode);
        if highlight_changed { if let Err(error) = renderer.set_highlight(&editor.scene, editor.selection()) { report(error); return; } }
        if let Err(error) = renderer.set_grid(editor.camera.grid) {
            report(error); return;
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
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
            let pending=if false {math_view.invalidate(&editor.scene);false} else {math_view.refresh(&mut renderer,&editor.scene,&editor.camera,[config.width,config.height],editor.whole_selection())};
        if math_view.uploaded {let view=math_view.scene(&editor.scene);let _=renderer.set_highlight(&view,editor.selection());let _=renderer.select_objects(&view,&editor.selected_objects());}
```

## <a id="checkpoint-45bb-cycle"></a>4. Cycle every eligible geometry owner

### 1. `src/scene.rs`

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

### 1. `src/browser.rs`

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
