# 11.4 · Original point identity and manifest source records

**Steps:** 6 · **Source:** [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/cloud.rs](../../../../src/engine/gpu/cloud.rs) · [engine/gpu/lod.rs](../../../../src/engine/gpu/lod.rs) · [engine/gpu/splat.rs](../../../../src/engine/gpu/splat.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3×3 Neighborhood: ID_center != ID_neighbor or Δz &gt; τ</text>
  <rect x="86" y="50" width="168" height="104" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="142" y1="50" x2="142" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="198" y1="50" x2="198" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="86" y1="85" x2="254" y2="85" stroke="#111111" stroke-width="1"/>
  <line x1="86" y1="120" x2="254" y2="120" stroke="#111111" stroke-width="1"/>
  <rect x="142" y="85" width="56" height="35" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="170" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">id=4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="114" y="72" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#64748b">id=0</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="226" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">id=4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Point Cloud Quad Splat: r_px = clamp(size / w, r_min, r_max)</text>
  <circle cx="484" cy="96" r="24" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="546" cy="84" r="18" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <circle cx="524" cy="128" r="20" fill="#fef08a" stroke="#d97706" stroke-width="2"/>
  <circle cx="626" cy="104" r="30" fill="#fee2e2" stroke="#dc2626" stroke-width="2"/>
  <line x1="626" y1="104" x2="656" y2="104" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="641" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">r</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Silhouette Outlines + Dense Splat Cloud</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,142 888,64 938,142" fill="#dbeafe" stroke="#dc2626" stroke-width="3.5"/>
  <circle cx="978" cy="84" r="8" fill="#2563eb"/>
  <circle cx="998" cy="76" r="7" fill="#16a34a"/>
  <circle cx="1016" cy="94" r="8" fill="#d97706"/>
  <circle cx="988" cy="108" r="9" fill="#dc2626"/>
  <circle cx="1012" cy="122" r="7" fill="#2563eb"/>
</svg>
</div>

## <a id="checkpoint-65-originals"></a>1. Resolve source point identities and precise positions

### 1. `src/cloud.rs`

**Locate this existing block:**

```rust
pub struct Row {
    pub id: crate::scene::ObjectId,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Row {
    pub controls_visible: bool,
    pub id: crate::scene::ObjectId,
```

### 2. `src/cloud_pick.rs`

Create the file and type:

```rust
use crate::{
    pick_sources::{Part, Target},
    scene::{ObjectId, Scene},
};

// Storage indices address current GPU columns; original IDs survive octree reordering and may use every bit of a u32.
pub fn resolve(scene: &Scene, owner: ObjectId, index: usize) -> Option<Target> {
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none()
        || index >= row.prepared.source.point_count()
    {
        return None;
    }
    let target = Target {
        owner,
        part: Part::CloudPoint(row.prepared.source.point_id(index)),
    };
    Some(target)
}

pub fn position(
    scene: &Scene,
    owner: ObjectId,
    id: u32,
) -> Option<session_rust::Point> {
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index = row.prepared.source.index_of_id(id)?;
    Some(
        row.model
            .transform_point(&row.prepared.source.get_point(index)),
    )
}
```

### 3. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if !target.valid(self) || !crate::placement::valid(&delta.m) {
            return Err("Invalid part edit");
        }
```

Replace it with:

```rust
        if !target.valid(self) || !crate::placement::valid(&delta.m) {
            return Err("Invalid part edit");
        }
        if matches!(target.part, Part::CloudPoint(_)) {
            return Err("Cloud point editing connects with source transforms");
        }
```

**Locate this block:**

```rust
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.clouds.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
```

Replace it with:

```rust
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.clouds.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
```

**Locate this block:**

```rust
        let visible = prepared.source.is_visible;
        let locked = prepared.source.is_locked;
        let row = crate::cloud::Row {
            id,
            guid: guid.clone(),
            prepared,
```

Replace it with:

```rust
        let visible = prepared.source.is_visible;
        let locked = prepared.source.is_locked;
        let row = crate::cloud::Row {
            controls_visible: false,
            id,
            guid: guid.clone(),
            prepared,
```

**Locate this block:**

```rust
    }

    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
```

Replace it with:

```rust
    }

    pub fn cloud_controls(&mut self, parent: Option<ObjectId>) {
        let parent = parent.filter(|id| self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
```

### 4. `src/pick_sources.rs`

**Locate this existing block:**

```rust
    Control(usize),
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Control(usize),
    CloudPoint(u32),
}
```

### 5. `src/selection.rs`

**Locate this existing block:**

```rust
        match self.part {
            Part::Object => true,
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::BRepEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.cad.as_ref().is_some_and(|cad|cad.edges.iter().any(|border|border.edge==edge))),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match self.part {
            Part::CloudPoint(id)=>crate::cloud_pick::position(scene,self.owner,id).is_some(),
            Part::Object => true,
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::BRepEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.cad.as_ref().is_some_and(|cad|cad.edges.iter().any(|border|border.edge==edge))),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::CloudPoint(id) => format!(
                "Object {} · original point {id}",
                self.owner.value()
            ),
            Part::Object => format!("Object {}", self.owner.value()),
```

### 6. `src/highlight.rs`

**Locate this existing block:**

```rust
                }
                Part::Object => {}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                Part::CloudPoint(id) => {
                    let p = crate::cloud_pick::position(scene, target.owner, id)
                        .ok_or("Cloud point is missing")?;
                    markers.push(crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 11.,
                        colour: yellow,
                    });
                }
                Part::Object => {}
```

### 7. `src/part_preview.rs`

**Locate this existing block:**

```rust
    }
    let model = scene
        .placement(target.owner)
        .ok_or("Part owner is missing")?;
    let points = if let Some(row) = scene
        .objects()
        .iter()
        .find(|r| r.id == target.owner)
    {
        if row.surface.is_some() || row.cad.is_some() {
            return Err(
                "CAD surface part editing connects after source parameter mapping",
            );
        }
        let mesh = row
            .geometry()
            .ok_or("Reload editable sources before editing a part")?;
        keys(mesh, target.part)?
            .into_iter()
            .map(|key| {
                mesh.vertex_point(key)
                    .ok_or("Missing source vertex")
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let row = scene
            .paths()
            .iter()
            .find(|r| r.id == target.owner)
            .ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve, _), Part::Control(index)) =
            (&row.prepared.source, target.part)
        else {
            return Err("Select a curve control point");
        };
        vec![
            curve
                .get_cv(index)
                .ok_or("Missing curve control")?,
        ]
    };
    Ok(points
        .into_iter()
        .map(|p| model.transform_point(&p))
        .collect())
}

pub fn origin(scene: &Scene, target: Target) -> Option<[f64; 3]> {
    let points = points(scene, target).ok()?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    if matches!(target.part, Part::CloudPoint(_)) {
        return Err("Cloud point editing connects with source transforms");
    }
    let model = scene
        .placement(target.owner)
        .ok_or("Part owner is missing")?;
    let points = if let Some(row) = scene
        .objects()
        .iter()
        .find(|r| r.id == target.owner)
    {
        if row.surface.is_some() || row.cad.is_some() {
            return Err(
                "CAD surface part editing connects after source parameter mapping",
            );
        }
        let mesh = row
            .geometry()
            .ok_or("Reload editable sources before editing a part")?;
        keys(mesh, target.part)?
            .into_iter()
            .map(|key| {
                mesh.vertex_point(key)
                    .ok_or("Missing source vertex")
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let row = scene
            .paths()
            .iter()
            .find(|r| r.id == target.owner)
            .ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve, _), Part::Control(index)) =
            (&row.prepared.source, target.part)
        else {
            return Err("Select a curve control point");
        };
        vec![
            curve
                .get_cv(index)
                .ok_or("Missing curve control")?,
        ]
    };
    Ok(points
        .into_iter()
        .map(|p| model.transform_point(&p))
        .collect())
}

pub fn origin(scene: &Scene, target: Target) -> Option<[f64; 3]> {
    if let Part::CloudPoint(id) = target.part {
        let p = crate::cloud_pick::position(scene, target.owner, id)?;
        return Some([p[0], p[1], p[2]]);
    }
    let points = points(scene, target).ok()?;
```

### 8. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod cloud_gpu;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cloud_gpu;

pub mod cloud_pick;
#[cfg(test)]
```

## <a id="checkpoint-65a-addresses"></a>2. Recover IDs from integer cloud pixels

### 1. `src/pick_sources.rs`

**Locate this existing block:**

```rust
        let (kind, address) = sub_id::decode(pixel[1])?;
        let target = match (mode, kind) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode == Mode::Control
            && kind == Kind::Control
            && scene.clouds().iter().any(|r| r.id == owner)
        {
            return crate::cloud_pick::resolve(scene, owner, address);
        }
        let target = match (mode, kind) {
```

### 2. `src/cloud_gpu.rs`

**Locate this existing block:**

```rust
                height_px: size[1],
                lod_px: view.lod_px,
                nodes: &row.prepared.nodes,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                height_px: size[1],
                lod_px: if row.controls_visible {
                    0.
                } else {
                    view.lod_px
                },
                nodes: &row.prepared.nodes,
```

### 3. `src/cloud_resolve.wgsl`

**Locate this existing block:**

```wgsl
    let ids=textureLoad(sids,q,0).xy;
    return Pick(select(vec2<u32>(0u),vec2<u32>(ids.x,0u),window.mode==0u),d);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let ids=textureLoad(sids,q,0).xy;
    var hit=vec2<u32>(0u);
    if (window.mode==0u) {hit=vec2<u32>(ids.x,0u);}
    if (window.mode==3u && ids.x!=0u && ids.y!=0u) {hit=vec2<u32>(ids.x,(0x40000000u|(ids.y-1u))+1u);}
    return Pick(hit,d);
}
```

## <a id="checkpoint-65b-controls"></a>3. Enter source controls with the current command or F10

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
```

Replace it with:

```rust
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => {
                let parent = if visible {
                    self.selected.filter(|id| {
                        self.scene
                            .clouds()
                            .iter()
                            .any(|r| r.id == *id)
                    })
                } else {
                    None
                };
                if parent.is_some()
                    && self.pick_mode == crate::pick_sources::Mode::Control
                    && self
                        .scene
                        .clouds()
                        .iter()
                        .any(|r| Some(r.id) == parent && r.controls_visible)
                {
                    return Ok(Change::View);
                }
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {
                    self.pick_mode = crate::pick_sources::Mode::Control;
                    self.objects.clear();
                    self.selected_part = None;
                } else if !visible
                    && self.pick_mode == crate::pick_sources::Mode::Control
                {
                    self.pick_mode = crate::pick_sources::Mode::Object;
                    self.selected_part = None;
                    self.select_objects(self.selected, false);
                }
            }
            Action::AddNormals(mode) => {
```

**Locate this block:**

```rust
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
```

Replace it with:

```rust
                    .filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => {
                if mode != crate::pick_sources::Mode::Control {
                    self.scene.cloud_controls(None);
                }
                self.pick_mode = mode;
                self.selected = None;
                self.selected_part = None;
```

**Locate this block:**

```rust
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
```

Replace it with:

```rust
        }
        if self
            .scene
            .clouds()
            .iter()
            .any(|r| r.controls_visible)
        {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
```

### 2. `src/commands.rs`

**Locate this existing block:**

```rust
    }),
    ("Controls On", || Action::Controls(true)),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }),
    ("Controls", || Action::Controls(true)),
    ("Controls On", || Action::Controls(true)),
```

### 3. `src/shortcuts.rs`

**Locate this existing block:**

```rust
    SelectAll,
}

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Fit => "Fit",
            Self::Hide => "Hide",
            Self::Show => "Show",
            Self::Delete => "Delete",
            Self::SelectAll => "Select All",
        }
    }
}

pub fn key(
    text: &str,
    command: bool,
    shift: bool,
    alt: bool,
    composing: bool,
    focused: bool,
    text_empty: bool,
) -> Option<Shortcut> {
    if alt || composing {
        return None;
    }
    if command {
        if focused && !text_empty {
            return None;
        }
        match text.to_ascii_lowercase().as_str() {
            "a" => Some(Shortcut::SelectAll),
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
            "y" => Some(Shortcut::Redo),
            _ => None,
        }
    } else if !focused {
        match text.to_ascii_lowercase().as_str() {
            "f" => Some(Shortcut::Fit),
            "h" => Some(Shortcut::Hide),
            "s" => Some(Shortcut::Show),
            "delete" => Some(Shortcut::Delete),
            _ => None,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    SelectAll,
    Controls,
}

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Fit => "Fit",
            Self::Hide => "Hide",
            Self::Show => "Show",
            Self::Delete => "Delete",
            Self::SelectAll => "Select All",
            Self::Controls => "Controls",
        }
    }
}

pub fn key(
    text: &str,
    command: bool,
    shift: bool,
    alt: bool,
    composing: bool,
    focused: bool,
    text_empty: bool,
) -> Option<Shortcut> {
    if alt || composing {
        return None;
    }
    if command {
        if focused && !text_empty {
            return None;
        }
        match text.to_ascii_lowercase().as_str() {
            "a" => Some(Shortcut::SelectAll),
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
            "y" => Some(Shortcut::Redo),
            _ => None,
        }
    } else if !focused {
        match text.to_ascii_lowercase().as_str() {
            "f" => Some(Shortcut::Fit),
            "h" => Some(Shortcut::Hide),
            "s" => Some(Shortcut::Show),
            "delete" => Some(Shortcut::Delete),
            "f10" => Some(Shortcut::Controls),
            _ => None,
```

### 4. `src/shortcuts_tests.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[test]
fn f10_controls_only_when_the_canvas_owns_the_key() {
    use crate::shortcuts::{Shortcut, key};
    assert_eq!(
        key(
            "F10", false, false, false, false, false, true
        ),
        Some(Shortcut::Controls)
    );
    for (command, alt, composing, focused) in [
        (true, false, false, false),
        (false, true, false, false),
        (false, false, true, false),
        (false, false, false, true),
    ] {
        assert_eq!(
            key(
                "F10", command, false, alt, composing, focused, true
            ),
            None
        );
    }
    assert_eq!(Shortcut::Controls.command(), "Controls");
}
```

## <a id="checkpoint-65c-cloud-point"></a>4. Pick points and prove source query reuse

With `Controls Off` and `Select None`, the same `Example Cloud` shows its stored RGB colors without the yellow selection overlay. These 1,681 points form a synthetic wave sampled on a 41 by 41 grid; this small color fixture does not represent a dense photographic scan.

![Unselected RGB colors on the original 1,681-point synthetic cloud](../../release/cloud65c-unselected-rgb.png)

### 1. `src/cloud_pick_tests.rs`

Create the file and type:

```rust
use crate::{
    cloud::Prepared,
    editor::{Action, Editor},
    pick_sources::{Part, Target},
    scene::Scene,
};
use session_rust::{Point, PointCloud, Xform};
use std::rc::Rc;

fn source() -> Rc<PointCloud> {
    let mut source = PointCloud::new(
        vec![
            Point::new(-0.4, 0., 0.2),
            Point::new(0.4, 0., 0.2),
        ],
        vec![],
        vec![],
    );
    source.point_size = 16.;
    let mut proto = source.to_proto();
    proto.point_ids = vec![u32::MAX, 16_777_217];
    Rc::new(PointCloud::from_proto(proto))
}

#[test]
fn cloud_point_identity_uses_original_u32_ids_and_f64_instance_positions() {
    let source = source();
    let _ = source.guid();
    let before = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(Prepared::new(source.clone()).unwrap())
        .unwrap();
    scene.cloud_controls(Some(owner));
    let target = crate::cloud_pick::resolve(&scene, owner, 0).unwrap();
    assert_eq!(
        target,
        Target {
            owner,
            part: Part::CloudPoint(u32::MAX)
        }
    );
    assert!(target.valid(&scene));
    assert!(target.describe().contains("4294967295"));
    assert!(crate::cloud_pick::resolve(&scene, owner, 2).is_none());
    scene
        .place_cloud(
            owner,
            Xform::translation(1_000_000_000.125, 0., 0.),
        )
        .unwrap();
    assert_eq!(
        crate::cloud_pick::position(&scene, owner, u32::MAX).unwrap()[0],
        1_000_000_000.125 - 0.4
    );
    assert_eq!(
        crate::part_preview::origin(&scene, target).unwrap()[0],
        1_000_000_000.125 - 0.4
    );
    assert!(crate::part_preview::Preview::new(&scene, target).is_err());
    assert_eq!(source.to_proto(), before);
    Rc::make_mut(&mut scene.view).lock(&[owner], true);
    assert!(!target.valid(&scene));
    Rc::make_mut(&mut scene.view).lock(&[owner], false);
    Rc::make_mut(&mut scene.view).hide(&[owner], true);
    assert!(!target.valid(&scene));
    Rc::make_mut(&mut scene.view).hide(&[owner], false);
    scene.cloud_controls(None);
    assert!(!target.valid(&scene));
}

#[test]
fn cloud_controls_bind_one_selected_parent_without_a_per_point_cpu_target_table() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    let source = source();
    let owner = editor
        .scene
        .insert_cloud(Prepared::new(source.clone()).unwrap())
        .unwrap();
    let copy = editor.scene.copy_row(owner, false).unwrap();
    editor
        .apply(Action::Picked(Some(owner)))
        .unwrap();
    let history = editor.scenes().count();
    editor.apply(Action::Controls(true)).unwrap();
    assert!(editor.scene.clouds()[0].controls_visible);
    assert!(!editor.scene.clouds()[1].controls_visible);
    let targets = crate::pick_sources::Sources::new(&editor.scene).unwrap();
    assert!(targets.controls.is_empty());
    let tag = crate::sub_id::encode(crate::sub_id::Kind::Control, 1).unwrap();
    let target = targets
        .resolve(
            &editor.scene,
            [owner.value(), tag],
            crate::pick_sources::Mode::Control,
        )
        .unwrap();
    assert_eq!(target.part, Part::CloudPoint(16_777_217));
    assert!(
        targets
            .resolve(
                &editor.scene,
                [copy.value(), tag],
                crate::pick_sources::Mode::Control
            )
            .is_none()
    );
    editor
        .apply(Action::PickedTarget(Some(target)))
        .unwrap();
    assert_eq!(editor.selection(), Some(target));
    assert!(editor.gizmo_origin().is_none());
    editor.apply(Action::Controls(true)).unwrap();
    assert_eq!(editor.selection(), Some(target));
    assert_eq!(editor.scenes().count(), history);
    editor
        .apply(Action::Controls(false))
        .unwrap();
    assert!(!target.valid(&editor.scene));
    assert!(
        editor
            .scene
            .clouds()
            .iter()
            .all(|r| !r.controls_visible)
    );
    editor.apply(Action::Close).unwrap();
    assert!(editor.scene.clouds().is_empty());
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod cloud_pick;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cloud_pick;

#[cfg(test)]
mod cloud_pick_tests;
#[cfg(test)]
```

### 3. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode, Part};
    let mut proto = session_rust::PointCloud::from_coords(
        vec![-0.4, 0., 0.2, 0.4, 0., 0.2],
        vec![],
        vec![],
    )
    .to_proto();
    proto.point_size = 16.;
    proto.point_ids = vec![u32::MAX, 16_777_217];
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    let uploaded = renderer.cloud_usage()[2];
    let geometry = renderer.stats();
    for density in [1., 2.] {
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        for (x, id) in [(-0.4, u32::MAX), (0.4, 16_777_217)] {
            renderer
                .request_pick([x, 0.], identity())
                .unwrap();
            let pixel = finish(&mut renderer).unwrap();
            let target = renderer.resolve_pick(&scene, pixel).unwrap();
            assert_eq!(target.part, Part::CloudPoint(id));
            assert_eq!(target.owner, owner);
            renderer
                .set_highlight(&scene, Some(target))
                .unwrap();
            assert!(renderer.highlight_bytes() > 0);
            assert_eq!(renderer.stats(), geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2], uploaded);
    assert_eq!(source.to_proto(), original);
    Rc::make_mut(&mut scene.view).lock(&[owner], true);
    renderer.set_density(1.);
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.4, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    scene.clear();
    renderer.set_scene(&scene, None);
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset()
 {
    use crate::pick_sources::{Mode, Part};
    let mut proto = crate::cloud::large_example().to_proto();
    for xyz in proto.coords[..64 * 3].chunks_exact_mut(3) {
        xyz[2] = 0.;
    }
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.aspect = 320. / 240.;
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    let uploads = renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Control);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 2_000_064);
    let transform = camera.view_projection().to_f32();
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.25, 0.25, 0.2));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let pixel = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .unwrap();
    let target = renderer.resolve_pick(&scene, pixel).unwrap();
    let Part::CloudPoint(id) = target.part else {
        panic!("Original cloud point identity");
    };
    assert!(id >= 64);
    assert!(target.valid(&scene));
    assert_eq!(renderer.cloud_usage()[2], uploads);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    renderer.cancel_pick();
    assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Object);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}
```

## <a id="checkpoint-65za-read-original-manifest-records-and-placements"></a>5. Read manifest records and placements

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

[dev-dependencies]
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["float_roundtrip"] }

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

serde_yaml_ng = "=0.10.0"
toml = { version = "=0.8.23", optional = true }

[features]
json-sessions = []
toml-manifests = ["dep:toml"]

[dev-dependencies]
```

### 2. `src/manifest.rs`

Create the file and type:

```rust
use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file and its placement.
#[derive(Clone, Deserialize)]
pub struct Item {
    pub file: String, // path like `pb/box.pb`, relative to the scene
    #[serde(default)]
    pub name: String, // display name, empty = the file's own
    #[serde(default)]
    pub at: Option<[f64; 3]>, // translation in world units
    #[serde(default)]
    pub xform: Option<[f64; 16]>, // full matrix, wins over `at`
    #[serde(default)]
    pub point_size: f64, // cloud point size in px, 0 = the file's own
    #[serde(default)]
    pub display_only: bool, // old flag, no longer changes anything
    #[serde(default)]
    pub size: u64, // decoded bytes, 0 = unknown
    #[serde(default)]
    pub encoding: String, // stored Content-Encoding such as `gzip`, empty = identity
}

/// One text placed in the world.
#[derive(Clone, Debug, Deserialize)]
pub struct TextItem {
    pub text: String, // the text
    #[serde(default)]
    pub at: [f64; 3], // world origin of the text
    #[serde(default = "text_right")]
    pub right: [f64; 3], // unit direction of the text line
    #[serde(default = "text_up")]
    pub up: [f64; 3], // unit direction up the text
    pub height: f64,  // letter height in world units
    #[serde(default)]
    pub camera_facing: bool, // always face the camera
}

/// The parsed scene file.
#[derive(Clone, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String, // scene name
    pub items: Vec<Item>, // geometry files
    #[serde(default)]
    pub texts: Vec<TextItem>, // world texts
}

impl Item {
    /// The decoded size of a file stored encoded: it is never range-read.
    pub fn encoded_size(&self) -> Option<u64> {
        (!self.encoding.is_empty()).then_some(self.size)
    }

    /// The item's placement, None for the auto grid.
    pub fn placement(&self) -> Option<Xform> {
        if let Some(m) = self.xform {
            let mut x = Xform::identity();
            x.m = m;
            return Some(x);
        }

        self.at.map(translation)
    }
}

fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation(
        (index % cols) as f64 * cell[0],
        (index / cols) as f64 * cell[1],
        0.0,
    )
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod cloud_pick_tests;
#[cfg(test)]
mod cloud_shader_contract;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cloud_pick_tests;

#[cfg(test)]
mod cloud_shader_contract;
pub mod manifest;
mod shader_check;
```

## <a id="checkpoint-65zb-validate-original-manifest-text-and-source-keys"></a>6. Validate manifest text and source keys

### 1. `src/manifest.rs`

**Locate this existing block:**

```rust
}

fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Manifest {
    /// Item `i`'s placement, or its auto grid slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        match self.items[i].placement() {
            Some(placement) => placement,
            None => auto_grid(i, self.items.len(), cell),
        }
    }

    /// Item `i`'s name, or `fallback`.
    pub fn name_of(&self, i: usize, fallback: &str) -> String {
        let n = &self.items[i].name;

        if n.is_empty() {
            fallback.to_string()
        } else {
            n.clone()
        }
    }
}

impl TextItem {
    /// Check the text, height and plane axes.
    fn validate(&self, index: usize) -> Result<(), String> {
        if self.text.trim().is_empty() {
            return Err(format!("text {index}: missing text content"));
        }

        if !self.height.is_finite() || self.height <= 0.0 {
            return Err(format!(
                "text {index}: height must be positive and finite"
            ));
        }

        if nonfinite_transform(self.at) {
            return Err(format!("text {index}: non-finite position"));
        }

        for axis in [self.right, self.up] {
            if nonfinite_transform(axis) {
                return Err(format!(
                    "text {index}: non-finite plane axis"
                ));
            }

            let length =
                (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();

            if (length - 1.0).abs() > 1e-6 {
                return Err(format!(
                    "text {index}: plane axes must be unit length within 1e-6"
                ));
            }
        }

        let dot = self.right[0] * self.up[0]
            + self.right[1] * self.up[1]
            + self.right[2] * self.up[2];

        if dot.abs() > 1e-6 {
            return Err(format!(
                "text {index}: plane axes must be orthogonal within 1e-6"
            ));
        }

        Ok(())
    }
}

/// World +X.
fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
fn nonfinite_transform<const N: usize>(values: [f64; N]) -> bool {
    !values.into_iter().all(f64::is_finite)
}

/// A TOML manifest.
#[cfg(feature = "toml-manifests")]
fn toml_form(text: &str) -> Result<Manifest, String> {
    toml::from_str(text).map_err(|error| error.to_string())
}

/// TOML is not read without the `toml-manifests` feature.
#[cfg(not(feature = "toml-manifests"))]
fn toml_form(_text: &str) -> Result<Manifest, String> {
    Err("not read by this build (feature toml-manifests)".to_string())
}

/// The line and column of a YAML error, if known.
pub fn immutable_key(url: &str) -> bool {
    let name = url.rsplit('/').next().unwrap_or(url);
    let stem = name.split('.').next().unwrap_or(name);
    let hash = stem.rsplit('-').next().unwrap_or(stem);
    hash.len() >= 16 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Grid slot `index` of `count`, `cell` apart.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
```

## Build & Run

![Recorded visual output for Lesson 11.4 (65zb-validate-original-manifest-text-and-source-keys)](../../../screenshots/journey/65zb-validate-original-manifest-text-and-source-keys.png)

```sh
npm --prefix ../session_tests run course -- dependencies 11.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 11.4
npm --prefix ../session_tests run course -- save 11.4
```

Continue with [12.1 · Complete viewer files and published scene sources](../../consolidation/66-read-scene-manifests.md).
