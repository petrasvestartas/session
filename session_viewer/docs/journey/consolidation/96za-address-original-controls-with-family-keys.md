# 19.4 · Family-aware original controls and source previews

**Steps:** 7 · **Source:** [app/session_io.rs](../../../src/app/session_io.rs) · [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/scene.rs](../../../src/app/scene.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Retained Plane Triad: O + X_axis (R) + Y_axis (G) + Z_axis (B)</text>
  <polygon points="68,136 132,76 268,76 204,136" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <circle cx="168" cy="106" r="4.5" fill="#111111"/>
  <line x1="168" y1="106" x2="236" y2="106" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <line x1="168" y1="106" x2="202" y2="74" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="168" y1="106" x2="168" y2="48" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Family Bitmask: Mesh | BRep | Curve | Point | Plane | Cloud</text>
  <rect x="430" y="56" width="78" height="36" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="469" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Mesh ✓</text>
  <rect x="520" y="56" width="78" height="36" rx="4" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">BRep ✓</text>
  <rect x="610" y="56" width="78" height="36" rx="4" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Plane ✓</text>
  <rect x="475" y="104" width="168" height="34" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="125" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">w_pen = w_base * scale</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Filtered Geometry Families + Explicit Plane Frame</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 888,68 938,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="948,132 978,94 1036,94 1006,132" fill="#fef9c3" stroke="#111111" stroke-width="1.5"/>
  <line x1="992" y1="113" x2="1028" y2="113" stroke="#dc2626" stroke-width="2"/>
  <line x1="992" y1="113" x2="992" y2="74" stroke="#2563eb" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-96za-address-original-controls-with-family-keys"></a>1. Address controls with family keys

### 1. `src/pick_sources.rs`

**Locate this existing block:**

```rust
    Control(usize),
    CloudPoint(u32),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Control(usize),
    SourceControl(crate::source_controls::ControlId),
    CloudPoint(u32),
```

### 2. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    control,
                    display,
```

Replace it with:

```rust
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::Control(control.index),
                    display,
```

**Locate this block:**

```rust
    pub parent: ObjectId,
    pub control: crate::controls::Control,
    pub display: crate::marker::Marker,
}
```

Replace it with:

```rust
    pub parent: ObjectId,
    pub part: crate::pick_sources::Part,
    pub display: crate::marker::Marker,
}
```

### 3. `src/pick_sources.rs`

**Locate this existing block:**

```rust
                    owner: row.parent,
                    part: Part::Control(row.control.index),
                },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    owner: row.parent,
                    part: row.part,
                },
```

### 4. `src/selection.rs`

**Locate this existing block:**

```rust
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
            Part::SheetEntity(id) => format!(
                "Object {} · original sheet entity {id}",
                self.owner.value()
            ),
            Part::CloudPoint(id) => format!(
                "Object {} · original point {id}",
                self.owner.value()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Part::CloudPoint(id)=>crate::cloud_pick::position(scene,self.owner,id).is_some(),
            Part::SourceControl(key) => scene.control_markers().iter().any(|m| m.parent == self.owner && m.part == Part::SourceControl(key)),
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
            Part::SheetEntity(id) => format!(
                "Object {} · original sheet entity {id}",
                self.owner.value()
            ),
            Part::CloudPoint(id) => format!(
                "Object {} · original point {id}",
                self.owner.value()
            ),
            Part::SourceControl(key) => format!(
                "Object {} · original control {key:?}",
                self.owner.value()
```

### 5. `src/highlight.rs`

**Locate this existing block:**

```rust
                }
                Part::Control(index) => {
                    let marker = scene
                        .control_markers()
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.control.index == index)
                        .unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                Part::Control(_) | Part::SourceControl(_) => {
                    let marker = scene
                        .control_markers()
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.part == target.part)
                        .unwrap();
```

### 6. `src/control_parent_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(markers[3].parent, second);
    assert_eq!(markers[1].control.index, 1);
    assert_eq!(markers[4].control.index, 1);
    assert!(Rc::ptr_eq(
        &markers[1].control.source,
        &markers[4].control.source
    ));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(markers[3].parent, second);
    assert_eq!(
        markers[1].part,
        crate::pick_sources::Part::Control(1)
    );
    assert_eq!(
        markers[4].part,
        crate::pick_sources::Part::Control(1)
    );
    assert!(crate::source_edit::same(
        &crate::source_edit::original(&scene, first).unwrap(),
        &crate::source_edit::original(&scene, second).unwrap()
    ));
```

## <a id="checkpoint-96zb-draw-and-pick-original-cad-control-points"></a>2. Draw and pick CAD control points

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
pub struct Scene {
    pub groups: Rc<std::collections::HashSet<(u32, String)>>,
    pub graphs: Rc<std::collections::HashMap<u32, Rc<session_rust::Graph>>>,
    pub cad_walks: [usize; 2],
    pub(crate) instances:
        std::collections::HashMap<ObjectId, crate::instance_plan::Owner>,
    pub(crate) created: Option<u32>,
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
    sheets: Vec<crate::sheet_source::Row>,
    streamed: Vec<crate::range_cloud::Row>,
    clouds: Vec<crate::cloud::Row>,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        let id = ObjectId(value);
        (self.visible(id)
            && !self.view.locked.contains(&id)
            && !self.forest.readonly(id))
        .then_some(id)
    }

    pub fn empty() -> Self {
        Self {
            groups: Default::default(),
            graphs: Default::default(),
            cad_walks: [0; 2],
            instances: Default::default(),
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            sheets: Vec::new(),
            streamed: Vec::new(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        }
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle(
            [
                [-0.7, -0.6, 0.25],
                [0.5, -0.6, 0.25],
                [-0.1, 0.6, 0.25],
            ],
            [0.9, 0.25, 0.45],
        )
        .expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle(
            [
                [-0.4, -0.2, 0.75],
                [0.8, -0.2, 0.75],
                [0.2, 0.8, 0.75],
            ],
            [0.05, 0.7, 0.7],
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene
            .insert(turquoise)
            .expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self, id: ObjectId) -> bool {
        self.contains(id) && !self.view.is_hidden(&id)
    }

    pub(crate) fn restore_view(&mut self, mut saved: Self) {
        saved.view = Rc::clone(&self.view);
        self.restore(saved);
        Rc::make_mut(&mut self.view).refresh(&self.forest);
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self
            .objects
            .iter()
            .filter(|row| self.visible(row.id))
        {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self
            .lines
            .iter()
            .filter(|line| self.visible(line.id))
        {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self
            .paths
            .iter()
            .filter(|row| self.visible(row.id))
        {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self
            .points
            .iter()
            .filter(|row| self.visible(row.id))
        {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
            match &mut bounds {
                Some(bounds) => bounds.include(p),
                None => bounds = Some(crate::bounds::Bounds::point(p)),
            }
        }
        for row in self
            .texts
            .iter()
            .filter(|row| self.visible(row.id))
        {
            if let Some(next) = row.bounds() {
                if let Some(bounds) = &mut bounds {
                    bounds.include(next.min);
                    bounds.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        for row in self
            .sheets
            .iter()
            .filter(|r| self.visible(r.id))
        {
            if let Some(next) = row.bounds() {
                if let Some(bounds) = &mut bounds {
                    bounds.include(next.min);
                    bounds.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        for row in self
            .streamed
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        for row in self
            .clouds
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        bounds
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
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.sheets
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
    }

    pub fn with_descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
        self.forest
            .descendants(ids)
            .into_iter()
            .filter(|id| self.contains(*id))
            .collect()
    }

    pub fn place_owner(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_following(id, model)
    }

    pub(crate) fn place_following(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id == id) {
            self.place_mesh(id, model)
        } else if self.lines.iter().any(|r| r.id == id) {
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else if self.clouds.iter().any(|r| r.id == id) {
            self.place_cloud(id, model)
        } else if self.streamed.iter().any(|r| r.id == id) {
            self.place_stream(id, model)
        } else if self.sheets.iter().any(|r| r.id == id) {
            self.place_sheet(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
            self.place_point(id, model)
        }
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
        if let Some(row) = self.streamed.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_stream(id, model);
        }
        if let Some(row) = self.clouds.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_cloud(id, model);
        }
        if let Some(row) = self.sheets.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_sheet(id, model);
        }
        if let Some(row) = self.texts.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_text(id, model);
        }
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) {
            return None;
        }
        if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
        if let Some(row) = self.sheets.iter().find(|row| row.id == id) {
            return row.bounds();
        }
        let points =
            if let Some(row) = self.lines.iter().find(|row| row.id == id) {
                Some(
                    row.endpoints()
                        .map(|p| [p[0], p[1], p[2]])
                        .to_vec(),
                )
            } else if let Some(row) = self.points.iter().find(|row| row.id == id) {
                let p = row
                    .model
                    .transform_point(&row.prepared.source);
                Some(vec![[p[0], p[1], p[2]]])
            } else {
                self.paths
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| {
                        row.prepared
                            .source
                            .bounds_coordinates()
                            .into_iter()
                            .map(|p| {
                                let p = row.model.transform_point(
                                    &session_rust::Point::new(p[0], p[1], p[2]),
                                );
                                [p[0], p[1], p[2]]
                            })
                            .collect::<Vec<_>>()
                    })
            };
        if let Some(points) = points {
            let mut bounds = crate::bounds::Bounds::point(*points.first()?);
            for p in points {
                bounds.include(p);
            }
            return Some(bounds);
        }
        let object = self
            .objects
            .iter()
            .find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::Plane(source)) = &object.original {
            return crate::plane_original::placed(source, &object.model);
        }
        if let Some(session_rust::Geometry::OBB(source)) = &object.original {
            return Some(crate::bounds_original::placed(
                source,
                &object.model,
            ));
        }
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize {
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn cloud_controls(&mut self, parent: Option<ObjectId>) {
        let parent = parent.filter(|id| self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {
            row.controls_visible = Some(row.id) == parent;
        }
        for row in &mut self.streamed {
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn sheets(&self) -> &[crate::sheet_source::Row] {
        &self.sheets
    }
    pub fn insert_sheet(
        &mut self,
        prepared: Rc<crate::sheet_source::Prepared>,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.original.guid.clone();
        let used = self.used_guids();
        while used.contains(&guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let name = prepared.original.name.clone();
        let url = prepared
            .streamed
            .as_ref()
            .map(|stream| stream.source.url.clone());
        let row = crate::sheet_source::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
            url,
        };
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        self.sheets.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_sheet(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid sheet placement");
        }
        let row = self
            .sheets
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Sheet is gone")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Sheet placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn sheet_import_url(
        &mut self,
        url: &str,
        previous: &std::collections::HashSet<ObjectId>,
    ) {
        for row in &mut self.sheets {
            if !previous.contains(&row.id) {
                row.url = Some(url.into());
            }
        }
    }
    pub fn sheet_url(&mut self, url: &str) {
        for row in &mut self.sheets {
            row.url = Some(url.into());
        }
    }
    pub fn streams(&self) -> &[crate::range_cloud::Row] {
        &self.streamed
    }
    pub fn insert_stream(
        &mut self,
        prepared: Rc<crate::range_cloud::Prepared>,
        model: session_rust::Xform,
        point_px: f32,
    ) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&model.m) || !point_px.is_finite() || point_px < 0.
        {
            return Err("Invalid streamed source placement or point size");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self
            .streamed
            .iter()
            .any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let row = crate::range_cloud::Row {
            id,
            guid: guid.clone(),
            prepared,
            model,
            point_px,
            controls_visible: false,
        };
        if row
            .bounds()
            .min
            .into_iter()
            .chain(row.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        let name = row.prepared.source.name.clone();
        let properties = row.prepared.source.properties;
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, properties.visible, properties.locked);
        self.streamed.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_stream(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid streamed source placement");
        }
        let row = self
            .streamed
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Streamed source is gone")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
    pub fn insert_cloud(
        &mut self,
        prepared: crate::cloud::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid().to_string();
        while self.clouds.iter().any(|r| r.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let name = prepared.source.name.clone();
        let visible = prepared.source.is_visible;
        let locked = prepared.source.is_locked;
        let row = crate::cloud::Row {
            point_px: 0.,
            controls_visible: false,
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, visible, locked);
        self.clouds.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_cloud(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid cloud placement");
        }
        let row = self
            .clouds
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Cloud object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !(v as f32).is_finite())
        {
            return Err("Cloud placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn texts(&self) -> &[crate::scene_label::Row] {
        &self.texts
    }
    pub(crate) fn texts_mut(&mut self) -> &mut [crate::scene_label::Row] {
        &mut self.texts
    }
    pub fn insert_text(
        &mut self,
        prepared: crate::scene_label::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self.texts.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        let row = crate::scene_label::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        row.label(false, true)?;
        if row.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            row.prepared.source.visible,
            row.prepared.source.locked,
        );
        self.texts.push(row);
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }
    pub fn place_text(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid text placement");
        }
        let row = self
            .texts
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Text object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        candidate.label(false, true)?;
        if candidate.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn text_labels(
        &self,
        selected: &[ObjectId],
    ) -> Result<Vec<crate::text::TextLabel>, &'static str> {
        self.texts
            .iter()
            .filter(|row| self.visible(row.id))
            .map(|row| {
                row.label(
                    selected.contains(&row.id),
                    self.pick_id(row.id.value()).is_some(),
                )
            })
            .collect()
    }
    pub fn points(&self) -> &[PointObject] {
        &self.points
    }

    pub fn point_capacity_bytes(&self) -> usize {
        self.points.capacity() * std::mem::size_of::<PointObject>()
    }

    pub fn insert_point(
        &mut self,
        prepared: crate::marker::PreparedPoint,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self
            .points
            .iter()
            .any(|row| row.guid == guid)
            || self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let label = prepared.source.name.clone();
        let node_guid = guid.clone();
        self.points.push(PointObject {
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        Ok(id)
    }

    pub fn place_point(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid point placement");
        }
        let row = self
            .points
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) {
            return Err("Point placement exceeds the display range");
        }
        row.model = model;
        Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths {
            row.controls_visible = visible
                && matches!(
                    row.prepared.source,
                    crate::chain::Source::Curve(..)
                );
        }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && self.visible(row.id))
        {
            for control in row.prepared.source.controls() {
                let p = control
                    .position()
                    .expect("Validated original control");
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker {
                    center: p.to_f32(),
                    ..control
                        .marker()
                        .expect("Prepared control marker")
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::Control(control.index),
                    display,
                });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }

    pub fn path_capacity_bytes(&self) -> usize {
        self.paths.capacity() * std::mem::size_of::<PathObject>()
    }

    pub fn insert_path(
        &mut self,
        prepared: crate::chain::PreparedChain,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.visible(),
            prepared.source.locked(),
        );
        let label = prepared.source.name().to_owned();
        let node_guid = guid.clone();
        self.paths.push(PathObject {
            controls_visible: false,
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        Ok(id)
    }

    pub fn place_path(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid path placement");
        }
        let row = self
            .paths
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Path not found")?;
        for p in row
            .prepared
            .source
            .coordinates()
            .into_iter()
            .chain(row.prepared.source.bounds_coordinates())
        {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                return Err("Path placement exceeds the display range");
            }
        }
        row.model = model;
        Ok(())
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        if prepared.source.arrowhead != session_rust::Arrowhead::NONE {
            return self.insert_path(crate::chain::PreparedChain::line(
                prepared.source,
            )?);
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let mut guid = prepared.source.guid().to_owned();
        while self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        self.lines.push(LineObject {
            guid: guid.clone(),
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        for point in [
            row.prepared.source.start(),
            row.prepared.source.end(),
        ] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) {
                return Err("Line placement exceeds the display range");
            }
        }
        row.model = model;
        Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] {
        &mut self.objects
    }

    pub(crate) fn unload(
        &mut self,
        origins: &std::collections::HashSet<uuid::Uuid>,
        epoch: u64,
    ) {
        for row in &mut self.objects {
            if let Some(origin) = row
                .origin()
                .filter(|origin| origins.contains(&origin.id))
                .cloned()
            {
                row.editable =
                    crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) {
                continue;
            }
            let Some(geometry) = prepared
                .geometry(&row.metadata.source_guid)
                .cloned()
            else {
                continue;
            };
            let source = crate::document::Source {
                document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(),
                origin: Rc::clone(&prepared.key.origin),
            };
            row.editable = crate::edit_source::EditSource::Loaded {
                geometry,
                source: Some(source),
            };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self
            .objects
            .iter()
            .any(|object| object.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.geometry.is_visible,
            prepared.geometry.is_locked,
        );
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        let generated = prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded {
            geometry: prepared.geometry,
            source: prepared.source,
        };
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            original: None,
            cad: None,
            surface: None,
            id,
            guid,
            metadata,
            mesh: Rc::new(prepared.display),
            model: prepared.model,
            editable,
        });
        if generated {
            crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        }
        Ok(id)
    }

    pub fn insert_cad(
        &mut self,
        source: Rc<session_rust::BRep>,
    ) -> Result<ObjectId, &'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(
        &mut self,
        (prepared, mesh): (
            crate::cad_display::Prepared,
            crate::prepared::PreparedMesh,
        ),
    ) -> Result<ObjectId, &'static str> {
        let prepared = Rc::new(prepared);
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .cad = Some(prepared);
        Ok(id)
    }

    pub fn insert_surface(
        &mut self,
        source: Rc<session_rust::NurbsSurface>,
    ) -> Result<ObjectId, &'static str> {
        let prepared = Rc::new(crate::surface::Prepared::new(source)?);
        let mesh = prepared.mesh()?;
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .surface = Some(prepared);
        Ok(id)
    }

    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|row| row.id == id)
        {
            let element = !instance
                && matches!(
                    row.original,
                    Some(session_rust::Geometry::Element(_))
                );
            if element {
                if let Some(session_rust::Geometry::Element(source)) =
                    row.original.as_mut()
                {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element {
                Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone();
            }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
        all: bool,
    ) -> Result<ObjectId, &'static str> {
        if self
            .objects
            .iter()
            .any(|row| row.id == id && row.geometry().is_none())
        {
            return Err("Reload editable sources before copying");
        }
        if !self.contains(id) {
            return Err("Object is gone");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let copy = ObjectId(self.next_id);
        let guid = uuid::Uuid::new_v4().to_string();
        if let Some(row) = self.objects.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.objects.push(row);
        } else if let Some(row) = self.lines.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.lines.push(row);
        } else if let Some(row) = self.paths.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.paths.push(row);
        } else if let Some(row) = self.points.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.streamed.push(row);
        } else if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.clouds.push(row);
        } else if let Some(row) = self.sheets.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.sheets.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.texts.push(row);
        }
        if let Some(owner) = self.instances.get(&id).cloned() {
            self.instances.insert(copy, owner);
        }
        self.detach_copy_identity(copy);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(copy);
        let view = Rc::make_mut(&mut self.view);
        if all {
            view.hide(&[copy], view.is_hidden(&id));
            view.lock(&[copy], view.locked.contains(&id));
        }
        for channel in [&mut view.faces, &mut view.edges] {
            if let Some(rgb) = channel.get(&id).copied() {
                channel.insert(copy, rgb);
            }
        }
        Ok(copy)
    }

    pub(crate) fn row_guid(&self, id: ObjectId) -> Option<&str> {
        self.objects
            .iter()
            .find(|row| row.id == id)
            .map(|row| row.guid.as_str())
            .or_else(|| {
                self.paths
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.points
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.lines
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.sheets
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
    }

    pub(crate) fn append_document(
        &mut self,
        source: &Scene,
        name: &str,
        place: &session_rust::Xform,
        point_px: f32,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&place.m) || !point_px.is_finite() {
            return Err("Invalid document placement or point size");
        }
        let mut candidate = self.clone();
        let mut ids = source
            .ids
            .iter()
            .copied()
            .collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
        let mut remap = std::collections::HashMap::new();
        let mut names = std::collections::HashMap::new();
        let mut used = candidate
            .ids
            .iter()
            .filter_map(|id| candidate.row_guid(*id).map(str::to_owned))
            .collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id = ObjectId(candidate.reserve_document_key()?);
            remap.insert(*old, id);
            Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid = source
                .row_guid(*old)
                .ok_or("Original source owner is missing")?
                .to_owned();
            while !used.insert(guid.clone()) {
                guid = uuid::Uuid::new_v4().to_string();
            }
            names.insert(*old, guid);
        }
        macro_rules! rows {
            ($field:ident) => {
                for row in &source.$field {
                    let mut row = row.clone();
                    let old = row.id;
                    row.id = remap[&old];
                    row.guid = names[&old].clone();
                    candidate.$field.push(row);
                }
            };
        }
        rows!(objects);
        rows!(lines);
        rows!(paths);
        rows!(points);
        rows!(clouds);
        rows!(streamed);
        rows!(texts);
        rows!(sheets);
        for old in &ids {
            let id = remap[old];
            if let Some(owner) = source.instances.get(old) {
                candidate.instances.insert(id, owner.clone());
            }
            candidate.place_following(
                id,
                place
                    * &source
                        .placement(*old)
                        .ok_or("Original placement is missing")?,
            )?;
            let view = Rc::make_mut(&mut candidate.view);
            view.initial(
                id,
                source.visible(*old),
                source.view.locked.contains(old),
            );
            if let Some(rgb) = source.view.faces.get(old) {
                view.faces.insert(id, *rgb);
            }
            if let Some(rgb) = source.view.edges.get(old) {
                view.edges.insert(id, *rgb);
            }
            if source
                .clouds
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate.cloud_point_size(id, point_px)?;
            }
            if source
                .streamed
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate
                    .streamed
                    .iter_mut()
                    .find(|row| row.id == id)
                    .unwrap()
                    .point_px = point_px;
            }
        }
        for doc in &source.forest.documents {
            let mut doc = (**doc).clone();
            let old_key = doc.key;
            doc.key = candidate.reserve_document_key()?;
            if !name.is_empty() {
                doc.label = name.to_owned();
            }
            let guids = doc
                .nodes
                .iter()
                .map(|node| {
                    (
                        node.source.guid.clone(),
                        uuid::Uuid::new_v4().to_string(),
                    )
                })
                .collect::<std::collections::HashMap<_, _>>();
            for node in &mut doc.nodes {
                node.object = node.object.map(|id| remap[&id]);
                node.source.guid = guids[&node.source.guid].clone();
                node.portable_guid = node.source.guid.clone();
                node.source.parent_guid = guids
                    .get(&node.source.parent_guid)
                    .cloned()
                    .unwrap_or_default();
            }
            for (key, guid) in source
                .groups
                .iter()
                .filter(|(key, _)| *key == old_key)
            {
                let _ = key;
                if let Some(guid) = guids.get(guid) {
                    Rc::make_mut(&mut candidate.groups).insert((doc.key, guid.clone()));
                }
            }
            let graph_names = ids
                .iter()
                .filter_map(|old| {
                    Some((
                        source.row_guid(*old)?.to_owned(),
                        names[old].clone(),
                    ))
                })
                .collect::<std::collections::HashMap<_, _>>();
            if source.graphs.contains_key(&old_key)
                || doc
                    .header
                    .as_ref()
                    .and_then(|h| h.header.graph.as_ref())
                    .is_some()
            {
                let graph = crate::graph_edit::transferred(
                    source,
                    source
                        .forest
                        .documents
                        .iter()
                        .find(|d| d.key == old_key)
                        .unwrap(),
                    &graph_names,
                );
                Rc::make_mut(&mut candidate.graphs).insert(
                    doc.key,
                    Rc::new(session_rust::Graph::from_proto(graph)),
                );
            }
            if source.created == Some(old_key) && candidate.created.is_none() {
                candidate.created = Some(doc.key);
            }
            candidate.forest.push(doc);
        }
        self.restore(candidate);
        Ok(())
    }

    pub fn face_rgba(&self, id: ObjectId) -> Option<[f32; 4]> {
        if let Some(rgb) = self.view.faces.get(&id) {
            return Some([
                rgb[0] as f32 / 255.,
                rgb[1] as f32 / 255.,
                rgb[2] as f32 / 255.,
                1.,
            ]);
        }
        let owner = self.instances.get(&id)?;
        (owner.instance.flags & session_rust::InstanceRef::FLAG_COLOR != 0).then(|| {
            let c = &owner.instance.color;
            [
                c.r as f32, c.g as f32, c.b as f32, c.a as f32,
            ]
        })
    }

    pub(crate) fn used_guids(&self) -> std::collections::HashSet<String> {
        self.objects
            .iter()
            .map(|r| r.guid.clone())
            .chain(self.lines.iter().map(|r| r.guid.clone()))
            .chain(self.paths.iter().map(|r| r.guid.clone()))
            .chain(self.points.iter().map(|r| r.guid.clone()))
            .chain(self.clouds.iter().map(|r| r.guid.clone()))
            .chain(self.streamed.iter().map(|r| r.guid.clone()))
            .chain(self.texts.iter().map(|r| r.guid.clone()))
            .chain(self.sheets.iter().map(|r| r.guid.clone()))
            .collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(
        &mut self,
        template: &Scene,
        owner: crate::instance_plan::Owner,
        model: session_rust::Xform,
        guid: String,
        point_px: f32,
    ) -> Result<ObjectId, &'static str> {
        let mut placed = template.clone();
        let original = placed
            .ids
            .iter()
            .copied()
            .next()
            .ok_or("Missing definition display owner")?;
        if placed.ids.len() != 1 {
            return Err("One definition must create one drawable owner");
        }
        placed.place_following(original, model)?;
        if !placed.clouds.is_empty() && point_px > 0. {
            placed.cloud_point_size(original, point_px)?;
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let visible =
            owner.instance.flags & session_rust::InstanceRef::FLAG_HIDDEN == 0;
        let locked = owner.instance.flags & session_rust::InstanceRef::FLAG_LOCKED != 0;
        if let Some(row) = placed.objects.first_mut() {
            row.metadata = Rc::new(crate::row_metadata::RowMetadata {
                source_guid: owner.instance.guid().into(),
                name: owner.instance.name.clone(),
                is_visible: visible,
                is_locked: locked,
            });
        }
        macro_rules! append {
            ($field:ident) => {
                if let Some(mut row) = placed.$field.pop() {
                    row.id = id;
                    row.guid = guid.clone();
                    self.$field.push(row);
                }
            };
        }
        append!(objects);
        append!(lines);
        append!(paths);
        append!(points);
        append!(clouds);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, visible, locked);
        self.instances.insert(id, owner);
        Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
        let key = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        Ok(key)
    }

    pub(crate) fn cloud_point_size(
        &mut self,
        id: ObjectId,
        px: f32,
    ) -> Result<(), &'static str> {
        if !px.is_finite() {
            return Err("Non-finite cloud point size");
        }
        self.clouds
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Cloud object not found")?
            .point_px = px;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.groups = Default::default();
        self.graphs = Default::default();
        self.cad_walks = [0; 2];
        self.instances.clear();
        self.created = None;
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.streamed = Vec::new();
        self.sheets = Vec::new();
        self.texts = Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        let mut objects = Vec::new();
        let key = self.next_id;
        for prepared in loaded.meshes {
            let guid = prepared.geometry.guid().to_owned();
            let name = prepared.geometry.name.clone();
            let id = self.insert(prepared)?;
            objects.push((guid, id, name));
        }
        self.forest.add(
            key,
            loaded.document.name.clone(),
            loaded.tree,
            &objects,
        );
        if let Some(origin) = objects
            .first()
            .and_then(|(_, id, _)| self.objects.iter().find(|row| row.id == *id))
            .and_then(|row| row.origin())
            .cloned()
        {
            Rc::make_mut(self.forest.documents.last_mut().unwrap()).header =
                Some(origin);
        }
        let colours = self.forest.colours(key);
        Rc::make_mut(&mut self.view)
            .faces
            .extend(colours);
        Ok(())
    }

    pub fn place(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_mesh(id, model)
    }

    fn place_mesh(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self
            .objects
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or("Object not found")?;
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        crate::graph_edit::remove(self, id);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Scene {
    pub(crate) control_owners: Rc<std::collections::HashSet<ObjectId>>,
    pub groups: Rc<std::collections::HashSet<(u32, String)>>,
    pub graphs: Rc<std::collections::HashMap<u32, Rc<session_rust::Graph>>>,
    pub cad_walks: [usize; 2],
    pub(crate) instances:
        std::collections::HashMap<ObjectId, crate::instance_plan::Owner>,
    pub(crate) created: Option<u32>,
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
    sheets: Vec<crate::sheet_source::Row>,
    streamed: Vec<crate::range_cloud::Row>,
    clouds: Vec<crate::cloud::Row>,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        let id = ObjectId(value);
        (self.visible(id)
            && !self.view.locked.contains(&id)
            && !self.forest.readonly(id))
        .then_some(id)
    }

    pub fn empty() -> Self {
        Self {
            control_owners: Default::default(),
            groups: Default::default(),
            graphs: Default::default(),
            cad_walks: [0; 2],
            instances: Default::default(),
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            sheets: Vec::new(),
            streamed: Vec::new(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        }
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle(
            [
                [-0.7, -0.6, 0.25],
                [0.5, -0.6, 0.25],
                [-0.1, 0.6, 0.25],
            ],
            [0.9, 0.25, 0.45],
        )
        .expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle(
            [
                [-0.4, -0.2, 0.75],
                [0.8, -0.2, 0.75],
                [0.2, 0.8, 0.75],
            ],
            [0.05, 0.7, 0.7],
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene
            .insert(turquoise)
            .expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self, id: ObjectId) -> bool {
        self.contains(id) && !self.view.is_hidden(&id)
    }

    pub(crate) fn restore_view(&mut self, mut saved: Self) {
        saved.view = Rc::clone(&self.view);
        self.restore(saved);
        Rc::make_mut(&mut self.view).refresh(&self.forest);
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self
            .objects
            .iter()
            .filter(|row| self.visible(row.id))
        {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self
            .lines
            .iter()
            .filter(|line| self.visible(line.id))
        {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self
            .paths
            .iter()
            .filter(|row| self.visible(row.id))
        {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self
            .points
            .iter()
            .filter(|row| self.visible(row.id))
        {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
            match &mut bounds {
                Some(bounds) => bounds.include(p),
                None => bounds = Some(crate::bounds::Bounds::point(p)),
            }
        }
        for row in self
            .texts
            .iter()
            .filter(|row| self.visible(row.id))
        {
            if let Some(next) = row.bounds() {
                if let Some(bounds) = &mut bounds {
                    bounds.include(next.min);
                    bounds.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        for row in self
            .sheets
            .iter()
            .filter(|r| self.visible(r.id))
        {
            if let Some(next) = row.bounds() {
                if let Some(bounds) = &mut bounds {
                    bounds.include(next.min);
                    bounds.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        for row in self
            .streamed
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        for row in self
            .clouds
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        bounds
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
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.sheets
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
    }

    pub fn with_descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
        self.forest
            .descendants(ids)
            .into_iter()
            .filter(|id| self.contains(*id))
            .collect()
    }

    pub fn place_owner(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_following(id, model)
    }

    pub(crate) fn place_following(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id == id) {
            self.place_mesh(id, model)
        } else if self.lines.iter().any(|r| r.id == id) {
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else if self.clouds.iter().any(|r| r.id == id) {
            self.place_cloud(id, model)
        } else if self.streamed.iter().any(|r| r.id == id) {
            self.place_stream(id, model)
        } else if self.sheets.iter().any(|r| r.id == id) {
            self.place_sheet(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
            self.place_point(id, model)
        }
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
        if let Some(row) = self.streamed.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_stream(id, model);
        }
        if let Some(row) = self.clouds.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_cloud(id, model);
        }
        if let Some(row) = self.sheets.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_sheet(id, model);
        }
        if let Some(row) = self.texts.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_text(id, model);
        }
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) {
            return None;
        }
        if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
        if let Some(row) = self.sheets.iter().find(|row| row.id == id) {
            return row.bounds();
        }
        let points =
            if let Some(row) = self.lines.iter().find(|row| row.id == id) {
                Some(
                    row.endpoints()
                        .map(|p| [p[0], p[1], p[2]])
                        .to_vec(),
                )
            } else if let Some(row) = self.points.iter().find(|row| row.id == id) {
                let p = row
                    .model
                    .transform_point(&row.prepared.source);
                Some(vec![[p[0], p[1], p[2]]])
            } else {
                self.paths
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| {
                        row.prepared
                            .source
                            .bounds_coordinates()
                            .into_iter()
                            .map(|p| {
                                let p = row.model.transform_point(
                                    &session_rust::Point::new(p[0], p[1], p[2]),
                                );
                                [p[0], p[1], p[2]]
                            })
                            .collect::<Vec<_>>()
                    })
            };
        if let Some(points) = points {
            let mut bounds = crate::bounds::Bounds::point(*points.first()?);
            for p in points {
                bounds.include(p);
            }
            return Some(bounds);
        }
        let object = self
            .objects
            .iter()
            .find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::Plane(source)) = &object.original {
            return crate::plane_original::placed(source, &object.model);
        }
        if let Some(session_rust::Geometry::OBB(source)) = &object.original {
            return Some(crate::bounds_original::placed(
                source,
                &object.model,
            ));
        }
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize {
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn cloud_controls(&mut self, parent: Option<ObjectId>) {
        let parent = parent.filter(|id| self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {
            row.controls_visible = Some(row.id) == parent;
        }
        for row in &mut self.streamed {
            row.controls_visible = Some(row.id) == parent;
        }
    }
    pub fn sheets(&self) -> &[crate::sheet_source::Row] {
        &self.sheets
    }
    pub fn insert_sheet(
        &mut self,
        prepared: Rc<crate::sheet_source::Prepared>,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.original.guid.clone();
        let used = self.used_guids();
        while used.contains(&guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let name = prepared.original.name.clone();
        let url = prepared
            .streamed
            .as_ref()
            .map(|stream| stream.source.url.clone());
        let row = crate::sheet_source::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
            url,
        };
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        self.sheets.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_sheet(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid sheet placement");
        }
        let row = self
            .sheets
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Sheet is gone")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Sheet placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn sheet_import_url(
        &mut self,
        url: &str,
        previous: &std::collections::HashSet<ObjectId>,
    ) {
        for row in &mut self.sheets {
            if !previous.contains(&row.id) {
                row.url = Some(url.into());
            }
        }
    }
    pub fn sheet_url(&mut self, url: &str) {
        for row in &mut self.sheets {
            row.url = Some(url.into());
        }
    }
    pub fn streams(&self) -> &[crate::range_cloud::Row] {
        &self.streamed
    }
    pub fn insert_stream(
        &mut self,
        prepared: Rc<crate::range_cloud::Prepared>,
        model: session_rust::Xform,
        point_px: f32,
    ) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&model.m) || !point_px.is_finite() || point_px < 0.
        {
            return Err("Invalid streamed source placement or point size");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self
            .streamed
            .iter()
            .any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let row = crate::range_cloud::Row {
            id,
            guid: guid.clone(),
            prepared,
            model,
            point_px,
            controls_visible: false,
        };
        if row
            .bounds()
            .min
            .into_iter()
            .chain(row.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        let name = row.prepared.source.name.clone();
        let properties = row.prepared.source.properties;
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, properties.visible, properties.locked);
        self.streamed.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_stream(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid streamed source placement");
        }
        let row = self
            .streamed
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Streamed source is gone")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !v.is_finite() || !(v as f32).is_finite())
        {
            return Err("Streamed source placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
    pub fn insert_cloud(
        &mut self,
        prepared: crate::cloud::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid().to_string();
        while self.clouds.iter().any(|r| r.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let name = prepared.source.name.clone();
        let visible = prepared.source.is_visible;
        let locked = prepared.source.is_locked;
        let row = crate::cloud::Row {
            point_px: 0.,
            controls_visible: false,
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, visible, locked);
        self.clouds.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_cloud(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid cloud placement");
        }
        let row = self
            .clouds
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Cloud object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !(v as f32).is_finite())
        {
            return Err("Cloud placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn texts(&self) -> &[crate::scene_label::Row] {
        &self.texts
    }
    pub(crate) fn texts_mut(&mut self) -> &mut [crate::scene_label::Row] {
        &mut self.texts
    }
    pub fn insert_text(
        &mut self,
        prepared: crate::scene_label::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self.texts.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        let row = crate::scene_label::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        row.label(false, true)?;
        if row.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            row.prepared.source.visible,
            row.prepared.source.locked,
        );
        self.texts.push(row);
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }
    pub fn place_text(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid text placement");
        }
        let row = self
            .texts
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Text object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        candidate.label(false, true)?;
        if candidate.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn text_labels(
        &self,
        selected: &[ObjectId],
    ) -> Result<Vec<crate::text::TextLabel>, &'static str> {
        self.texts
            .iter()
            .filter(|row| self.visible(row.id))
            .map(|row| {
                row.label(
                    selected.contains(&row.id),
                    self.pick_id(row.id.value()).is_some(),
                )
            })
            .collect()
    }
    pub fn points(&self) -> &[PointObject] {
        &self.points
    }

    pub fn point_capacity_bytes(&self) -> usize {
        self.points.capacity() * std::mem::size_of::<PointObject>()
    }

    pub fn insert_point(
        &mut self,
        prepared: crate::marker::PreparedPoint,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self
            .points
            .iter()
            .any(|row| row.guid == guid)
            || self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let label = prepared.source.name.clone();
        let node_guid = guid.clone();
        self.points.push(PointObject {
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        Ok(id)
    }

    pub fn place_point(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid point placement");
        }
        let row = self
            .points
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) {
            return Err("Point placement exceeds the display range");
        }
        row.model = model;
        Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        let owners = Rc::make_mut(&mut self.control_owners);
        owners.clear();
        if visible {
            owners.extend(self.objects.iter().map(|row| row.id));
        }
        for row in &mut self.paths {
            row.controls_visible = visible
                && matches!(
                    row.prepared.source,
                    crate::chain::Source::Curve(..)
                );
        }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && self.visible(row.id))
        {
            for control in row.prepared.source.controls() {
                let p = control
                    .position()
                    .expect("Validated original control");
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker {
                    center: p.to_f32(),
                    ..control
                        .marker()
                        .expect("Prepared control marker")
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::Control(control.index),
                    display,
                });
            }
        }
        for row in self
            .objects
            .iter()
            .filter(|row| self.control_owners.contains(&row.id) && self.visible(row.id))
        {
            let Some(source) = crate::source_edit::original(self, row.id) else {
                continue;
            };
            for control in
                crate::source_controls::Controls::from_geometry(&source).points
            {
                let local = session_rust::Point::new(
                    control.position[0],
                    control.position[1],
                    control.position[2],
                );
                let center = row.model.transform_point(&local).to_f32();
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
                let display = crate::marker::Marker {
                    center,
                    diameter: 6.,
                    colour: [0., 0., 0., 1.],
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::SourceControl(control.id),
                    display,
                });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }

    pub fn path_capacity_bytes(&self) -> usize {
        self.paths.capacity() * std::mem::size_of::<PathObject>()
    }

    pub fn insert_path(
        &mut self,
        prepared: crate::chain::PreparedChain,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.visible(),
            prepared.source.locked(),
        );
        let label = prepared.source.name().to_owned();
        let node_guid = guid.clone();
        self.paths.push(PathObject {
            controls_visible: false,
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        Ok(id)
    }

    pub fn place_path(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid path placement");
        }
        let row = self
            .paths
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Path not found")?;
        for p in row
            .prepared
            .source
            .coordinates()
            .into_iter()
            .chain(row.prepared.source.bounds_coordinates())
        {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                return Err("Path placement exceeds the display range");
            }
        }
        row.model = model;
        Ok(())
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        if prepared.source.arrowhead != session_rust::Arrowhead::NONE {
            return self.insert_path(crate::chain::PreparedChain::line(
                prepared.source,
            )?);
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let mut guid = prepared.source.guid().to_owned();
        while self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        self.lines.push(LineObject {
            guid: guid.clone(),
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        for point in [
            row.prepared.source.start(),
            row.prepared.source.end(),
        ] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) {
                return Err("Line placement exceeds the display range");
            }
        }
        row.model = model;
        Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] {
        &mut self.objects
    }

    pub(crate) fn unload(
        &mut self,
        origins: &std::collections::HashSet<uuid::Uuid>,
        epoch: u64,
    ) {
        for row in &mut self.objects {
            if let Some(origin) = row
                .origin()
                .filter(|origin| origins.contains(&origin.id))
                .cloned()
            {
                row.editable =
                    crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) {
                continue;
            }
            let Some(geometry) = prepared
                .geometry(&row.metadata.source_guid)
                .cloned()
            else {
                continue;
            };
            let source = crate::document::Source {
                document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(),
                origin: Rc::clone(&prepared.key.origin),
            };
            row.editable = crate::edit_source::EditSource::Loaded {
                geometry,
                source: Some(source),
            };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self
            .objects
            .iter()
            .any(|object| object.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.geometry.is_visible,
            prepared.geometry.is_locked,
        );
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        let generated = prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded {
            geometry: prepared.geometry,
            source: prepared.source,
        };
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            original: None,
            cad: None,
            surface: None,
            id,
            guid,
            metadata,
            mesh: Rc::new(prepared.display),
            model: prepared.model,
            editable,
        });
        if generated {
            crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        }
        Ok(id)
    }

    pub fn insert_cad(
        &mut self,
        source: Rc<session_rust::BRep>,
    ) -> Result<ObjectId, &'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(
        &mut self,
        (prepared, mesh): (
            crate::cad_display::Prepared,
            crate::prepared::PreparedMesh,
        ),
    ) -> Result<ObjectId, &'static str> {
        let prepared = Rc::new(prepared);
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .cad = Some(prepared);
        Ok(id)
    }

    pub fn insert_surface(
        &mut self,
        source: Rc<session_rust::NurbsSurface>,
    ) -> Result<ObjectId, &'static str> {
        let prepared = Rc::new(crate::surface::Prepared::new(source)?);
        let mesh = prepared.mesh()?;
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .surface = Some(prepared);
        Ok(id)
    }

    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|row| row.id == id)
        {
            let element = !instance
                && matches!(
                    row.original,
                    Some(session_rust::Geometry::Element(_))
                );
            if element {
                if let Some(session_rust::Geometry::Element(source)) =
                    row.original.as_mut()
                {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element {
                Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone();
            }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
        all: bool,
    ) -> Result<ObjectId, &'static str> {
        if self
            .objects
            .iter()
            .any(|row| row.id == id && row.geometry().is_none())
        {
            return Err("Reload editable sources before copying");
        }
        if !self.contains(id) {
            return Err("Object is gone");
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let copy = ObjectId(self.next_id);
        let guid = uuid::Uuid::new_v4().to_string();
        if let Some(row) = self.objects.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.objects.push(row);
        } else if let Some(row) = self.lines.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.lines.push(row);
        } else if let Some(row) = self.paths.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.paths.push(row);
        } else if let Some(row) = self.points.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.streamed.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.streamed.push(row);
        } else if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            row.controls_visible = false;
            self.clouds.push(row);
        } else if let Some(row) = self.sheets.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.sheets.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.texts.push(row);
        }
        if let Some(owner) = self.instances.get(&id).cloned() {
            self.instances.insert(copy, owner);
        }
        self.detach_copy_identity(copy);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(copy);
        let view = Rc::make_mut(&mut self.view);
        if all {
            view.hide(&[copy], view.is_hidden(&id));
            view.lock(&[copy], view.locked.contains(&id));
        }
        for channel in [&mut view.faces, &mut view.edges] {
            if let Some(rgb) = channel.get(&id).copied() {
                channel.insert(copy, rgb);
            }
        }
        Ok(copy)
    }

    pub(crate) fn row_guid(&self, id: ObjectId) -> Option<&str> {
        self.objects
            .iter()
            .find(|row| row.id == id)
            .map(|row| row.guid.as_str())
            .or_else(|| {
                self.paths
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.points
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.lines
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.streamed
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.sheets
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
    }

    pub(crate) fn append_document(
        &mut self,
        source: &Scene,
        name: &str,
        place: &session_rust::Xform,
        point_px: f32,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&place.m) || !point_px.is_finite() {
            return Err("Invalid document placement or point size");
        }
        let mut candidate = self.clone();
        let mut ids = source
            .ids
            .iter()
            .copied()
            .collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
        let mut remap = std::collections::HashMap::new();
        let mut names = std::collections::HashMap::new();
        let mut used = candidate
            .ids
            .iter()
            .filter_map(|id| candidate.row_guid(*id).map(str::to_owned))
            .collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id = ObjectId(candidate.reserve_document_key()?);
            remap.insert(*old, id);
            Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid = source
                .row_guid(*old)
                .ok_or("Original source owner is missing")?
                .to_owned();
            while !used.insert(guid.clone()) {
                guid = uuid::Uuid::new_v4().to_string();
            }
            names.insert(*old, guid);
        }
        macro_rules! rows {
            ($field:ident) => {
                for row in &source.$field {
                    let mut row = row.clone();
                    let old = row.id;
                    row.id = remap[&old];
                    row.guid = names[&old].clone();
                    candidate.$field.push(row);
                }
            };
        }
        rows!(objects);
        rows!(lines);
        rows!(paths);
        rows!(points);
        rows!(clouds);
        rows!(streamed);
        rows!(texts);
        rows!(sheets);
        for old in &ids {
            let id = remap[old];
            if let Some(owner) = source.instances.get(old) {
                candidate.instances.insert(id, owner.clone());
            }
            candidate.place_following(
                id,
                place
                    * &source
                        .placement(*old)
                        .ok_or("Original placement is missing")?,
            )?;
            let view = Rc::make_mut(&mut candidate.view);
            view.initial(
                id,
                source.visible(*old),
                source.view.locked.contains(old),
            );
            if let Some(rgb) = source.view.faces.get(old) {
                view.faces.insert(id, *rgb);
            }
            if let Some(rgb) = source.view.edges.get(old) {
                view.edges.insert(id, *rgb);
            }
            if source
                .clouds
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate.cloud_point_size(id, point_px)?;
            }
            if source
                .streamed
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate
                    .streamed
                    .iter_mut()
                    .find(|row| row.id == id)
                    .unwrap()
                    .point_px = point_px;
            }
        }
        for doc in &source.forest.documents {
            let mut doc = (**doc).clone();
            let old_key = doc.key;
            doc.key = candidate.reserve_document_key()?;
            if !name.is_empty() {
                doc.label = name.to_owned();
            }
            let guids = doc
                .nodes
                .iter()
                .map(|node| {
                    (
                        node.source.guid.clone(),
                        uuid::Uuid::new_v4().to_string(),
                    )
                })
                .collect::<std::collections::HashMap<_, _>>();
            for node in &mut doc.nodes {
                node.object = node.object.map(|id| remap[&id]);
                node.source.guid = guids[&node.source.guid].clone();
                node.portable_guid = node.source.guid.clone();
                node.source.parent_guid = guids
                    .get(&node.source.parent_guid)
                    .cloned()
                    .unwrap_or_default();
            }
            for (key, guid) in source
                .groups
                .iter()
                .filter(|(key, _)| *key == old_key)
            {
                let _ = key;
                if let Some(guid) = guids.get(guid) {
                    Rc::make_mut(&mut candidate.groups).insert((doc.key, guid.clone()));
                }
            }
            let graph_names = ids
                .iter()
                .filter_map(|old| {
                    Some((
                        source.row_guid(*old)?.to_owned(),
                        names[old].clone(),
                    ))
                })
                .collect::<std::collections::HashMap<_, _>>();
            if source.graphs.contains_key(&old_key)
                || doc
                    .header
                    .as_ref()
                    .and_then(|h| h.header.graph.as_ref())
                    .is_some()
            {
                let graph = crate::graph_edit::transferred(
                    source,
                    source
                        .forest
                        .documents
                        .iter()
                        .find(|d| d.key == old_key)
                        .unwrap(),
                    &graph_names,
                );
                Rc::make_mut(&mut candidate.graphs).insert(
                    doc.key,
                    Rc::new(session_rust::Graph::from_proto(graph)),
                );
            }
            if source.created == Some(old_key) && candidate.created.is_none() {
                candidate.created = Some(doc.key);
            }
            candidate.forest.push(doc);
        }
        self.restore(candidate);
        Ok(())
    }

    pub fn face_rgba(&self, id: ObjectId) -> Option<[f32; 4]> {
        if let Some(rgb) = self.view.faces.get(&id) {
            return Some([
                rgb[0] as f32 / 255.,
                rgb[1] as f32 / 255.,
                rgb[2] as f32 / 255.,
                1.,
            ]);
        }
        let owner = self.instances.get(&id)?;
        (owner.instance.flags & session_rust::InstanceRef::FLAG_COLOR != 0).then(|| {
            let c = &owner.instance.color;
            [
                c.r as f32, c.g as f32, c.b as f32, c.a as f32,
            ]
        })
    }

    pub(crate) fn used_guids(&self) -> std::collections::HashSet<String> {
        self.objects
            .iter()
            .map(|r| r.guid.clone())
            .chain(self.lines.iter().map(|r| r.guid.clone()))
            .chain(self.paths.iter().map(|r| r.guid.clone()))
            .chain(self.points.iter().map(|r| r.guid.clone()))
            .chain(self.clouds.iter().map(|r| r.guid.clone()))
            .chain(self.streamed.iter().map(|r| r.guid.clone()))
            .chain(self.texts.iter().map(|r| r.guid.clone()))
            .chain(self.sheets.iter().map(|r| r.guid.clone()))
            .collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(
        &mut self,
        template: &Scene,
        owner: crate::instance_plan::Owner,
        model: session_rust::Xform,
        guid: String,
        point_px: f32,
    ) -> Result<ObjectId, &'static str> {
        let mut placed = template.clone();
        let original = placed
            .ids
            .iter()
            .copied()
            .next()
            .ok_or("Missing definition display owner")?;
        if placed.ids.len() != 1 {
            return Err("One definition must create one drawable owner");
        }
        placed.place_following(original, model)?;
        if !placed.clouds.is_empty() && point_px > 0. {
            placed.cloud_point_size(original, point_px)?;
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let visible =
            owner.instance.flags & session_rust::InstanceRef::FLAG_HIDDEN == 0;
        let locked = owner.instance.flags & session_rust::InstanceRef::FLAG_LOCKED != 0;
        if let Some(row) = placed.objects.first_mut() {
            row.metadata = Rc::new(crate::row_metadata::RowMetadata {
                source_guid: owner.instance.guid().into(),
                name: owner.instance.name.clone(),
                is_visible: visible,
                is_locked: locked,
            });
        }
        macro_rules! append {
            ($field:ident) => {
                if let Some(mut row) = placed.$field.pop() {
                    row.id = id;
                    row.guid = guid.clone();
                    self.$field.push(row);
                }
            };
        }
        append!(objects);
        append!(lines);
        append!(paths);
        append!(points);
        append!(clouds);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, visible, locked);
        self.instances.insert(id, owner);
        Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
        let key = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        Ok(key)
    }

    pub(crate) fn cloud_point_size(
        &mut self,
        id: ObjectId,
        px: f32,
    ) -> Result<(), &'static str> {
        if !px.is_finite() {
            return Err("Non-finite cloud point size");
        }
        self.clouds
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Cloud object not found")?
            .point_px = px;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.control_owners = Default::default();
        self.groups = Default::default();
        self.graphs = Default::default();
        self.cad_walks = [0; 2];
        self.instances.clear();
        self.created = None;
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.streamed = Vec::new();
        self.sheets = Vec::new();
        self.texts = Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        let mut objects = Vec::new();
        let key = self.next_id;
        for prepared in loaded.meshes {
            let guid = prepared.geometry.guid().to_owned();
            let name = prepared.geometry.name.clone();
            let id = self.insert(prepared)?;
            objects.push((guid, id, name));
        }
        self.forest.add(
            key,
            loaded.document.name.clone(),
            loaded.tree,
            &objects,
        );
        if let Some(origin) = objects
            .first()
            .and_then(|(_, id, _)| self.objects.iter().find(|row| row.id == *id))
            .and_then(|row| row.origin())
            .cloned()
        {
            Rc::make_mut(self.forest.documents.last_mut().unwrap()).header =
                Some(origin);
        }
        let colours = self.forest.colours(key);
        Rc::make_mut(&mut self.view)
            .faces
            .extend(colours);
        Ok(())
    }

    pub fn place(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_mesh(id, model)
    }

    fn place_mesh(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self
            .objects
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or("Object not found")?;
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        Rc::make_mut(&mut self.control_owners).remove(&id);
        crate::graph_edit::remove(self, id);
```

### 2. `src/original_control_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Part, Sources},
    source_controls::ControlId,
};

#[test]
fn original_families_reuse_control_addresses_and_keep_the_source_unchanged() {
    for action in [
        Action::AddBox,
        Action::AddCad,
        Action::AddSurface,
    ] {
        let mut editor = Editor::default();
        editor.apply(Action::Close).unwrap();
        editor.apply(action).unwrap();
        let owner = editor.scene.objects()[0].id;
        let source = crate::source_edit::original(&editor.scene, owner).unwrap();
        editor.apply(Action::Controls(true)).unwrap();
        let markers = editor.scene.control_markers();
        assert!(!markers.is_empty());
        let sources = Sources::new(&editor.scene).unwrap();
        assert_eq!(sources.controls.len(), markers.len());
        for (index, target) in sources.controls.iter().enumerate() {
            assert_eq!(target.owner, owner);
            assert!(matches!(target.part, Part::SourceControl(_)));
            assert!(target.valid(&editor.scene));
            let pixel = [
                owner.value(),
                crate::sub_id::encode(crate::sub_id::Kind::Control, index).unwrap(),
            ];
            assert_eq!(
                sources.resolve(
                    &editor.scene,
                    pixel,
                    crate::pick_sources::Mode::Control
                ),
                Some(*target)
            );
        }
        assert!(crate::source_edit::same(
            &source,
            &crate::source_edit::original(&editor.scene, owner).unwrap()
        ));
        let old = sources.controls[0];
        editor.scene.show_controls(false);
        assert!(!old.valid(&editor.scene));
        assert!(editor.scene.control_markers().is_empty());
        editor.scene.show_controls(true);
        std::rc::Rc::make_mut(&mut editor.scene.view).hide(&[owner], true);
        assert!(editor.scene.control_markers().is_empty());
        assert!(!old.valid(&editor.scene));
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.control_owners.is_empty());
    }
}

#[test]
fn element_mesh_controls_keep_the_original_element_owner() {
    let mut scene = crate::scene::Scene::empty();
    let mut element = session_rust::Element::new("controls");
    element.set_geometry(session_rust::Mesh::create_box(1., 1., 1.));
    let source = std::rc::Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, source.clone()).unwrap();
    scene.show_controls(true);
    let markers = scene.control_markers();
    assert!(!markers.is_empty());
    assert!(markers.iter().all(|m| m.parent == owner
        && matches!(
            m.part,
            Part::SourceControl(ControlId::Vertex(_))
        )));
    assert_eq!(scene.objects().len(), 1);
    assert!(
        matches!(&scene.objects()[0].original,Some(session_rust::Geometry::Element(current)) if std::rc::Rc::ptr_eq(current,&source))
    );
    scene.remove(owner);
    assert!(scene.control_owners.is_empty());
    assert!(scene.control_markers().is_empty());
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod transform_rotate;

pub mod source_controls;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod transform_rotate;

#[cfg(test)]
mod original_control_tests;
pub mod source_controls;
```

### 4. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
        "A panel shifts the frame without changing the original world pen"
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "A panel shifts the frame without changing the original world pen"
    );
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.5, -0.5, 0.25]),
        (30, [0., 0.5, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let model = session_rust::Xform::translation(0.125, 0.25, 0.);
    scene.place(owner, model).unwrap();
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        let pixel = finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::SourceControl(
                    crate::source_controls::ControlId::Vertex(10)
                )
            })
        );
        scene.show_controls(false);
        renderer.set_scene(&scene, None);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        assert!(finish(&mut renderer).is_none());
        scene.show_controls(true);
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}
```

## <a id="checkpoint-96zc-share-the-current-control-point-style"></a>3. Share the current control-point style

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
                let display = crate::marker::Marker {
                    center,
                    diameter: 6.,
                    colour: [0., 0., 0., 1.],
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::SourceControl(control.id),
                    display,
                });
            }
```

Replace it with:

```rust
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
                markers.push(ControlMarker::new(
                    row.id,
                    crate::pick_sources::Part::SourceControl(control.id),
                    center,
                ));
            }
```

**Locate this block:**

```rust
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker {
                    center: p.to_f32(),
                    ..control
                        .marker()
                        .expect("Prepared control marker")
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    part: crate::pick_sources::Part::Control(control.index),
                    display,
                });
            }
        }
        for row in self
```

Replace it with:

```rust
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                markers.push(ControlMarker::new(
                    row.id,
                    crate::pick_sources::Part::Control(control.index),
                    p.to_f32(),
                ));
            }
        }
        for row in self
```

**Locate this block:**

```rust
}

#[derive(Clone)]
pub struct PointObject {
```

Replace it with:

```rust
}
impl ControlMarker {
    fn new(
        parent: ObjectId,
        part: crate::pick_sources::Part,
        center: [f32; 3],
    ) -> Self {
        Self {
            parent,
            part,
            display: crate::marker::Marker {
                center,
                diameter: 7.,
                colour: [0.15, 0.35, 0.9, 1.],
            },
        }
    }
}

#[derive(Clone)]
pub struct PointObject {
```

### 2. `src/highlight.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.paths.draw(pass);
        self.markers.draw(pass);
```

Replace it with:

```rust
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.paths.draw(pass);
    }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.markers.draw(pass);
```

**Locate this block:**

```rust
                    markers.push(crate::marker::Marker {
                        diameter: marker.display.diameter + 4.,
                        colour: yellow,
                        ..marker.display
                    });
                }
```

Replace it with:

```rust
                    markers.push(crate::marker::Marker {
                        colour: [1., 1., 0., 1.],
                        ..marker.display
                    });
                }
```

### 3. `src/renderer.rs`

**Locate this existing block:**

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
```

### 4. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.1, 0.1, 0.25]),
        (30, [0.8, 0.1, 0.25]),
        (40, [0.1, 0.8, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![20, 30, 40], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    assert!(
        scene
            .control_markers()
            .iter()
            .all(|m| m.display.diameter == 7.
                && m.display.colour == [0.15, 0.35, 0.9, 1.])
    );
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let target = crate::pick_sources::Target {
        owner,
        part: crate::pick_sources::Part::SourceControl(
            crate::source_controls::ControlId::Vertex(10),
        ),
    };
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        let pixels = colour_pixels(&mut renderer);
        let center = (180 * 320 + 80) * 4;
        for (actual, expected) in pixels[center..center + 4]
            .iter()
            .zip([38_i16, 89, 230, 255])
        {
            assert!(
                (*actual as i16 - expected).abs() <= 2,
                "Actual blue control pixel {:?}",
                &pixels[center..center + 4]
            );
        }
        let coverage: f32 = (60..100)
            .map(|x| (255. - pixels[(180 * 320 + x) * 4] as f32) / 217.)
            .sum();
        assert!(
            (coverage / density - 7.).abs() < 0.6,
            "Seven CSS pixel row coverage at density {density}: {coverage}"
        );
        renderer
            .set_highlight(&scene, Some(target))
            .unwrap();
        let selected = colour_pixels(&mut renderer);
        assert_eq!(
            &selected[center..center + 4],
            &[255, 255, 0, 255]
        );
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}
```

## <a id="checkpoint-96zd-reuse-world-controls-for-their-display-nets"></a>4. Reuse world controls for their display nets

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && self.visible(row.id))
        {
            for control in row.prepared.source.controls() {
                let p = control
                    .position()
                    .expect("Validated original control");
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                markers.push(ControlMarker::new(
                    row.id,
                    crate::pick_sources::Part::Control(control.index),
                    p.to_f32(),
                ));
            }
        }
        for row in self
            .objects
            .iter()
            .filter(|row| self.control_owners.contains(&row.id) && self.visible(row.id))
        {
            let Some(source) = crate::source_edit::original(self, row.id) else {
                continue;
            };
            for control in
                crate::source_controls::Controls::from_geometry(&source).points
            {
                let local = session_rust::Point::new(
                    control.position[0],
                    control.position[1],
                    control.position[2],
                );
                let center = row.model.transform_point(&local).to_f32();
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
                markers.push(ControlMarker::new(
                    row.id,
                    crate::pick_sources::Part::SourceControl(control.id),
                    center,
                ));
            }
        }
        markers
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    fn control_sources(
        &self,
    ) -> Vec<(
        ObjectId,
        bool,
        crate::source_controls::Controls,
    )> {
        let curves = self
            .paths
            .iter()
            .filter(|row| row.controls_visible)
            .map(|row| (row.id, true));
        let originals = self
            .objects
            .iter()
            .filter(|row| self.control_owners.contains(&row.id))
            .map(|row| (row.id, false));
        curves
            .chain(originals)
            .filter(|(id, _)| self.visible(*id))
            .filter_map(|(id, legacy)| {
                let source = crate::source_edit::original(self, id)?;
                let model = self.placement(id)?;
                let mut controls =
                    crate::source_controls::Controls::from_geometry(&source);
                for control in &mut controls.points {
                    let p = model.transform_point(&session_rust::Point::new(
                        control.position[0],
                        control.position[1],
                        control.position[2],
                    ));
                    control.position = [p[0], p[1], p[2]];
                }
                Some((id, legacy, controls))
            })
            .collect()
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = control.position.map(|v| v as f32);
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => {
                        crate::pick_sources::Part::Control(point)
                    }
                    (_, id) => crate::pick_sources::Part::SourceControl(id),
                };
                markers.push(ControlMarker::new(owner, part, center));
            }
        }
        markers
    }

    pub fn control_nets(&self) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls
                    .points
                    .get(a)
                    .zip(controls.points.get(b))
                else {
                    continue;
                };
                let start = a.position.map(|v| v as f32);
                let end = b.position.map(|v| v as f32);
                if start
                    .iter()
                    .chain(&end)
                    .any(|v| !v.is_finite())
                {
                    continue;
                }
                nets.push((
                    owner,
                    crate::stroke::Stroke {
                        start,
                        end,
                        colour: [102. / 255., 136. / 255., 204. / 255., 1.],
                        width: 1.,
                    },
                ));
            }
        }
        nets
    }
```

### 2. `src/pick_sources.rs`

**Locate this existing block:**

```rust
        }
        for row in scene.control_markers() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        for (owner, stroke) in scene.control_nets() {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers() {
```

### 3. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor = crate::editor::Editor::default();
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    editor
        .apply(crate::editor::Action::AddCurve)
        .unwrap();
    editor
        .apply(crate::editor::Action::Controls(true))
        .unwrap();
    let owner = editor.scene.paths()[0].id;
    let nets = editor.scene.control_nets();
    assert_eq!(nets.len(), 2);
    let markers = editor.scene.control_markers();
    for (parent, line) in &nets {
        assert_eq!(*parent, owner);
        assert!(
            markers
                .iter()
                .any(|m| m.display.center == line.start)
        );
        assert!(
            markers
                .iter()
                .any(|m| m.display.center == line.end)
        );
        assert_eq!(line.width, 1.);
    }
    let source = crate::source_edit::original(&editor.scene, owner).unwrap();
    for density in [1., 2.] {
        let mut renderer = renderer(&editor.scene);
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        renderer.set_pick_mode(crate::pick_sources::Mode::Object);
        renderer
            .request_pick([-0.3, 0.1], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer),
            Some([owner.value(), 0])
        );
        for mode in [
            crate::pick_sources::Mode::Edge,
            crate::pick_sources::Mode::Control,
        ] {
            renderer.set_pick_mode(mode);
            renderer
                .request_pick([-0.3, 0.1], identity())
                .unwrap();
            assert!(finish(&mut renderer).is_none());
        }
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&editor.scene, owner).unwrap()
    ));
    editor.scene.show_controls(false);
    assert!(editor.scene.control_nets().is_empty());
}
```

### 4. `src/pick_sources_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(sources.faces.len(), 6);
    assert_eq!(sources.mesh_ink.len(), 12);
    let face = sources.faces[0];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(sources.faces.len(), 6);
    assert_eq!(
        sources.edges.len(),
        12 + sources.path_ids.len()
    );
    assert_eq!(sources.mesh_ink.len(), 14);
    assert!(
        sources.mesh_ids[12..]
            .iter()
            .all(|pixel| pixel[1] == 0)
    );
    let face = sources.faces[0];
```

## <a id="checkpoint-96ze-scope-controls-to-the-selected-original"></a>5. Scope controls to the selected original

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
    }

    pub fn show_controls(&mut self, visible: bool) {
        let owners = Rc::make_mut(&mut self.control_owners);
        owners.clear();
        if visible {
            owners.extend(self.objects.iter().map(|row| row.id));
        }
        for row in &mut self.paths {
            row.controls_visible = visible
                && matches!(
                    row.prepared.source,
                    crate::chain::Source::Curve(..)
                );
        }
    }

    fn control_sources(
        &self,
    ) -> Vec<(
        ObjectId,
        bool,
        crate::source_controls::Controls,
    )> {
        let curves = self
            .paths
            .iter()
            .filter(|row| row.controls_visible)
            .map(|row| (row.id, true));
        let originals = self
            .objects
            .iter()
            .filter(|row| self.control_owners.contains(&row.id))
            .map(|row| (row.id, false));
        curves
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn owner_controls(&mut self, parent: ObjectId) -> Result<(), &'static str> {
        let cloud = self.clouds.iter().any(|r| r.id == parent)
            || self.streamed.iter().any(|r| r.id == parent);
        if !cloud {
            let source = crate::source_edit::original(self, parent)
                .ok_or("Source controls are unavailable")?;
            if crate::source_controls::Controls::from_geometry(&source)
                .points
                .is_empty()
            {
                return Err("This object has no source controls");
            }
        }
        self.show_controls(false);
        self.cloud_controls(cloud.then_some(parent));
        if !cloud {
            Rc::make_mut(&mut self.control_owners).insert(parent);
            for row in &mut self.paths {
                row.controls_visible = row.id == parent;
            }
        }
        Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        let owners = Rc::make_mut(&mut self.control_owners);
        owners.clear();
        if visible {
            owners.extend(self.objects.iter().map(|row| row.id));
        }
        for row in &mut self.paths {
            row.controls_visible = visible
                && matches!(
                    row.prepared.source,
                    crate::chain::Source::Curve(..)
                );
        }
    }

    fn control_sources(
        &self,
    ) -> Vec<(
        ObjectId,
        bool,
        crate::source_controls::Controls,
    )> {
        let curves = self
            .paths
            .iter()
            .filter(|row| row.controls_visible)
            .map(|row| (row.id, true));
        let originals = self
            .control_owners
            .iter()
            .copied()
            .filter(|id| {
                !self
                    .paths
                    .iter()
                    .any(|r| r.id == *id && r.controls_visible)
            })
            .map(|id| (id, false));
        curves
```

### 2. `src/editor.rs`

**Locate this existing block:**

```rust
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    AddNormals(crate::normal_example::Mode),
    EnableControls,
    Controls(bool),
```

### 3. `src/commands.rs`

**Locate this existing block:**

```rust
    }),
    ("Controls", || Action::Controls(true)),
    ("Controls On", || Action::Controls(true)),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }),
    ("Controls", || Action::EnableControls),
    ("Controls On", || Action::Controls(true)),
```

### 4. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
```

Replace it with:

```rust
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::EnableControls => {
                let parent = self
                    .selected
                    .ok_or("Select one object before Controls")?;
                if self.pick_mode == crate::pick_sources::Mode::Control
                    && (self
                        .scene
                        .control_markers()
                        .iter()
                        .any(|m| m.parent == parent)
                        || self
                            .scene
                            .clouds()
                            .iter()
                            .any(|r| r.id == parent && r.controls_visible)
                        || self
                            .scene
                            .streams()
                            .iter()
                            .any(|r| r.id == parent && r.controls_visible))
                {
                    return Ok(Change::View);
                }
                if self.selected_objects().len() != 1 {
                    return Err("Select one object before Controls");
                }
                self.scene.owner_controls(parent)?;
                self.pick_mode = crate::pick_sources::Mode::Control;
                self.objects.clear();
                self.selected_part = None;
            }
```

**Locate this block:**

```rust
                self.apply(Action::GizmoCancel)
            } else if self.draft.is_some() || self.plane_menu {
                self.apply(Action::CancelTool)
            } else {
                self.apply(Action::Picked(None))
            };
```

Replace it with:

```rust
                self.apply(Action::GizmoCancel)
            } else if self.draft.is_some() || self.plane_menu {
                self.apply(Action::CancelTool)
            } else if self.pick_mode == crate::pick_sources::Mode::Control {
                self.apply(Action::Controls(false))
            } else {
                self.apply(Action::Picked(None))
            };
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
        if self.pick_mode == crate::pick_sources::Mode::Control
            && self.selected_part.is_none()
        {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
```

### 5. `src/original_control_tests.rs`

**Locate this existing block:**

```rust
    assert!(scene.control_markers().is_empty());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(scene.control_markers().is_empty());
}

#[test]
fn controls_scope_and_escape_preserve_original_ownership_and_history() {
    use crate::{
        editor::{Action, Editor},
        pick_sources::{Mode, Part, Target},
    };
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    assert!(editor.apply(Action::EnableControls).is_err());
    editor.apply(Action::AddCurve).unwrap();
    editor.apply(Action::AddCurve).unwrap();
    let first = editor.scene.paths()[0].id;
    let second = editor.scene.paths()[1].id;
    let source = crate::source_edit::original(&editor.scene, first).unwrap();
    editor.select_objects([first], false);
    editor.apply(Action::EnableControls).unwrap();
    assert_eq!(editor.pick_mode, Mode::Control);
    assert!(editor.gizmo_origin().is_none());
    assert!(
        editor
            .scene
            .control_markers()
            .iter()
            .all(|m| m.parent == first)
    );
    assert!(
        editor
            .scene
            .control_nets()
            .iter()
            .all(|(id, _)| *id == first)
    );
    editor
        .apply(Action::PickedTarget(Some(Target {
            owner: first,
            part: Part::Control(1),
        })))
        .unwrap();
    editor.apply(Action::EnableControls).unwrap();
    assert_eq!(editor.selected_part, Some(Part::Control(1)));
    editor.apply(Action::Escape).unwrap();
    assert_eq!(editor.pick_mode, Mode::Object);
    assert_eq!(editor.selected, Some(first));
    assert!(editor.selected_part.is_none());
    assert!(editor.scene.control_markers().is_empty());
    assert!(editor.scene.control_nets().is_empty());
    editor.select_objects([second], false);
    editor.apply(Action::EnableControls).unwrap();
    assert!(
        editor
            .scene
            .control_markers()
            .iter()
            .all(|m| m.parent == second)
    );
    for (action, count) in [
        (Action::AddPoint, 1),
        (Action::AddLine, 2),
        (Action::AddPolyline, 3),
    ] {
        let mut e = Editor::default();
        e.apply(Action::Close).unwrap();
        e.apply(action).unwrap();
        e.apply(Action::SelectAll).unwrap();
        let owner = e.selected.unwrap();
        e.apply(Action::EnableControls).unwrap();
        let rows = e.scene.control_markers();
        assert_eq!(rows.len(), count);
        assert!(rows.iter().all(|r| r.parent == owner));
        assert!(
            rows.iter()
                .all(|r| matches!(r.part, Part::SourceControl(_)))
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&editor.scene, first).unwrap()
    ));
    editor.apply(Action::Escape).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.paths().len(),
        1,
        "Controls and Escape add no source history"
    );
}
```

## <a id="checkpoint-96zf-edit-original-controls-through-the-source-preview"></a>6. Edit controls through the source preview

### 1. `src/source_part.rs`

**Locate this existing block:**

```rust
use session_rust::Geometry;
use std::rc::Rc;
pub fn geometry(scene: &Scene, id: ObjectId) -> Result<Geometry, &'static str> {
    let row = scene
        .objects()
        .iter()
        .find(|row| row.id == id)
        .ok_or("Source object is missing")?;
    if let Some(original) = &row.original {
        return Ok(original.clone());
    }
    if let Some(cad) = &row.cad {
        return Ok(Geometry::BRep(Rc::clone(&cad.source)));
    }
    if let Some(surface) = &row.surface {
        return Ok(Geometry::NurbsSurface(Rc::clone(
            &surface.source,
        )));
    }
    Err("Select an original CAD source")
}
pub fn target(part: Part) -> Result<crate::source_deform::Target, &'static str> {
    match part {
        Part::Face(face) => Ok(crate::source_deform::Target::Face(face)),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::Geometry;
pub fn geometry(scene: &Scene, id: ObjectId) -> Result<Geometry, &'static str> {
    crate::source_edit::original(scene, id).ok_or("Reload an editable original source")
}
pub fn target(part: Part) -> Result<crate::source_deform::Target, &'static str> {
    match part {
        Part::SourceControl(id) => Ok(crate::source_deform::Target::Control(id)),
        Part::Face(face) => Ok(crate::source_deform::Target::Face(face)),
```

### 2. `src/part_preview.rs`

**Locate this existing block:**

```rust
        .ok_or("Part owner is missing")?;
    let points = if let Some(row) = scene
        .objects()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .ok_or("Part owner is missing")?;
    let points = if matches!(target.part, Part::SourceControl(_)) {
        crate::source_part::points(scene, target)
            .map_err(|_| "Cannot resolve original control")?
    } else if let Some(row) = scene
        .objects()
```

### 3. `src/scene.rs`

**Locate this existing block:**

```rust
        }
        if self
            .objects
            .iter()
            .any(|row| row.id == target.owner && row.original.is_some())
        {
            return Err(
                "Part editing for this original family connects in the complete command inventory",
            );
        }
        let model = self
            .placement(target.owner)
            .ok_or("Part owner is missing")?;
        let local = &(&model
            .inverse()
            .ok_or("Part placement cannot be inverted")?
            * delta)
            * &model;
        if self.objects.iter().any(|row| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        if !matches!(target.part, Part::SourceControl(_))
            && self
                .objects
                .iter()
                .any(|row| row.id == target.owner && row.original.is_some())
        {
            return Err(
                "Part editing for this original family connects in the complete command inventory",
            );
        }
        let model = self
            .placement(target.owner)
            .ok_or("Part owner is missing")?;
        let local = &(&model
            .inverse()
            .ok_or("Part placement cannot be inverted")?
            * delta)
            * &model;
        if matches!(target.part, Part::SourceControl(_)) {
            let original = crate::source_part::geometry(self, target.owner)?;
            let edited = crate::source_deform::transform(
                &original,
                crate::source_part::target(target.part)?,
                &local,
            )
            .map_err(|_| "Source control edit would invalidate geometry")?;
            return self.replace_original(target.owner, edited);
        }
        if self.objects.iter().any(|row| {
```

### 4. `src/part_preview.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
}

fn same_source(a: &Scene, b: &Scene, id: ObjectId) -> bool {
    if let Some(row) = a.objects().iter().find(|r| r.id == id) {
        return b
            .objects()
            .iter()
            .find(|r| r.id == id)
            .is_some_and(|other| {
                match (
                    &row.cad,
                    &other.cad,
                    &row.surface,
                    &other.surface,
                ) {
                    (Some(a), Some(b), _, _) => Rc::ptr_eq(&a.source, &b.source),
                    (_, _, Some(a), Some(b)) => Rc::ptr_eq(&a.source, &b.source),
                    (None, None, None, None) => other
                        .geometry()
                        .zip(row.geometry())
                        .is_some_and(|(a, b)| Rc::ptr_eq(a, b)),
                    _ => false,
                }
            });
    }
    a.paths()
        .iter()
        .find(|r| r.id == id)
        .zip(b.paths().iter().find(|r| r.id == id))
        .is_some_and(|(a, b)| a.prepared.source.owner() == b.prepared.source.owner())
}
```

Replace it with:

```rust
}

fn same_source(a: &Scene, b: &Scene, id: ObjectId) -> bool {
    crate::source_edit::original(a, id)
        .zip(crate::source_edit::original(b, id))
        .is_some_and(|(a, b)| crate::source_edit::same(&a, &b))
}
```

**Locate this block:**

```rust
use session_rust::{Point, Xform};
use std::rc::Rc;

pub fn points(scene: &Scene, target: Target) -> Result<Vec<Point>, &'static str> {
    if scene.instances.contains_key(&target.owner) {
```

Replace it with:

```rust
use session_rust::{Point, Xform};

pub fn points(scene: &Scene, target: Target) -> Result<Vec<Point>, &'static str> {
    if scene.instances.contains_key(&target.owner) {
```

### 5. `src/original_control_tests.rs`

**Locate this existing block:**

```rust
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
}

#[test]
fn original_sparse_control_preview_conjugates_placement_and_restores_source_history() {
    use crate::{
        pick_sources::{Part, Target},
        prepared::PreparedMesh,
        scene::Scene,
        source_controls::ControlId,
    };
    use session_rust::{Geometry, Mesh, Point, Xform};
    let mut mesh = Mesh::new();
    for (key, p) in [
        (10, [0., 0., 0.]),
        (20, [1., 0., 0.]),
        (30, [0., 1., 0.]),
    ] {
        mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut scene = Scene::empty();
    let first = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    let other = scene.copy_row(first, false).unwrap();
    scene
        .place(
            first,
            &Xform::translation(2., 3., 0.)
                * &Xform::scale_uniform(&Point::new(0., 0., 0.), 2.),
        )
        .unwrap();
    scene.owner_controls(first).unwrap();
    let source = crate::source_edit::original(&scene, first).unwrap();
    let shared = crate::source_edit::original(&scene, other).unwrap();
    assert!(crate::source_edit::same(&source, &shared));
    let target = Target {
        owner: first,
        part: Part::SourceControl(ControlId::Vertex(10)),
    };
    assert_eq!(
        crate::part_preview::origin(&scene, target),
        Some([2., 3., 0.])
    );
    let mut preview = crate::part_preview::Preview::new(&scene, target).unwrap();
    preview
        .set(Xform::translation(0.125, 0.25, 0.))
        .unwrap();
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, first).unwrap()
    ));
    let Geometry::Mesh(edited) =
        crate::source_edit::original(&preview.scene, first).unwrap()
    else {
        panic!("Original mesh")
    };
    assert_eq!(
        edited.vertex_point(10).unwrap(),
        Point::new(0.0625, 0.125, 0.)
    );
    assert_eq!(
        edited.vertex_point(20).unwrap(),
        Point::new(1., 0., 0.)
    );
    let mut history = crate::history::History::default();
    assert!(
        preview
            .commit(&mut scene, &mut history)
            .unwrap()
    );
    assert!(crate::source_edit::same(
        &shared,
        &crate::source_edit::original(&scene, other).unwrap()
    ));
    assert!(history.undo(&mut scene));
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, first).unwrap()
    ));
    assert!(history.redo(&mut scene));
    assert!(!crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, first).unwrap()
    ));
    assert!(
        preview
            .commit(&mut scene, &mut history)
            .is_err(),
        "A stale source preview cannot overwrite the edit"
    );
}
```

## <a id="checkpoint-96zg-rebuild-an-edited-element-without-replacing-its-owner"></a>7. Rebuild an edited Element without replacing its owner

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
        match geometry {
            Geometry::Plane(source) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match geometry {
            Geometry::Element(source) => {
                let mut staging = Self::empty();
                let prepared = crate::element_original::insert(&mut staging, source)?;
                let mut replacement = staging
                    .objects()
                    .iter()
                    .find(|row| row.id == prepared)
                    .ok_or("Prepared Element is missing")?
                    .clone();
                for vertex in replacement.mesh.vertices() {
                    let point = model.transform_point(&session_rust::Point::new(
                        vertex[0] as f64,
                        vertex[1] as f64,
                        vertex[2] as f64,
                    ));
                    if (0..3).any(|i| !(point[i] as f32).is_finite()) {
                        return Err("Edited Element exceeds display range");
                    }
                }
                let row = self
                    .objects
                    .iter_mut()
                    .find(|row| row.id == id)
                    .ok_or("Element owner is missing")?;
                replacement.id = row.id;
                replacement.guid = row.guid.clone();
                replacement.metadata = row.metadata.clone();
                replacement.model = model;
                *row = replacement;
            }
            Geometry::Plane(source) => {
```

### 2. `src/original_control_tests.rs`

**Locate this existing block:**

```rust
        "A stale source preview cannot overwrite the edit"
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "A stale source preview cannot overwrite the edit"
    );
}

#[test]
fn element_control_replacement_keeps_original_features_tree_placement_and_history() {
    use crate::{
        pick_sources::{Part, Target},
        scene::Scene,
        source_controls::ControlId,
    };
    use session_rust::{Element, Geometry, Mesh, Point, Polyline, Session, Xform};
    use std::rc::Rc;
    let mut mesh = Mesh::new();
    for (key, p) in [
        (10, [0., 0., 0.]),
        (20, [1., 0., 0.]),
        (30, [0., 1., 0.]),
    ] {
        mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut element = Element::new("original body");
    element.set_geometry(mesh);
    element.add_feature(session_rust::element::ElementFeature::new(
        "axis",
        -1,
        vec![Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(0., 1., 0.),
        ])],
        "kept attribute",
    ));
    let source = Rc::new(element);
    let mut scene = Scene::empty();
    let owner = crate::element_original::insert(&mut scene, source.clone()).unwrap();
    scene
        .place(owner, Xform::translation(2., 3., 0.))
        .unwrap();
    crate::feature_display::append(
        &mut scene,
        owner,
        &Session::new("features"),
        &source,
    )
    .unwrap();
    scene.owner_controls(owner).unwrap();
    let guid = scene.row_guid(owner).unwrap().to_owned();
    let document = scene.forest.documents[0].clone();
    let feature = scene.paths()[0].id;
    let outline = scene.paths()[0].prepared.source.clone();
    let model = scene.placement(owner).unwrap().m;
    let target = Target {
        owner,
        part: Part::SourceControl(ControlId::Vertex(10)),
    };
    let mut preview = crate::part_preview::Preview::new(&scene, target).unwrap();
    preview
        .set(Xform::translation(0.125, 0.25, 0.))
        .unwrap();
    assert!(
        matches!(crate::source_edit::original(&scene,owner),Some(Geometry::Element(old))if Rc::ptr_eq(&old,&source))
    );
    let mut history = crate::history::History::default();
    assert!(
        preview
            .commit(&mut scene, &mut history)
            .unwrap()
    );
    let Geometry::Element(edited) =
        crate::source_edit::original(&scene, owner).unwrap()
    else {
        panic!("Original Element")
    };
    let session_rust::element::ElementGeometry::Mesh(body) = edited.geometry() else {
        panic!("Original body mesh")
    };
    assert_eq!(
        body.vertex_point(10).unwrap(),
        Point::new(0.125, 0.25, 0.)
    );
    assert_eq!(
        edited.features[0].to_proto(),
        source.features[0].to_proto()
    );
    assert_eq!(scene.row_guid(owner), Some(guid.as_str()));
    assert_eq!(scene.placement(owner).unwrap().m, model);
    assert!(Rc::ptr_eq(
        &scene.forest.documents[0],
        &document
    ));
    assert_eq!(scene.paths()[0].id, feature);
    assert_eq!(
        scene.paths()[0].prepared.source.owner(),
        outline.owner()
    );
    assert!(scene.forest.derived(feature));
    assert!(history.undo(&mut scene));
    assert!(
        matches!(crate::source_edit::original(&scene,owner),Some(Geometry::Element(old))if Rc::ptr_eq(&old,&source))
    );
    assert!(history.redo(&mut scene));
    assert!(
        preview
            .commit(&mut scene, &mut history)
            .is_err()
    );
}
```

### 3. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert!(editor.scene.control_nets().is_empty());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(editor.scene.control_nets().is_empty());
}

#[test]
fn actual_edited_element_controls_move_their_integer_pixels_and_undo() {
    use crate::pick_sources::{Mode, Part, Target};
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.5, -0.5, 0.25]),
        (30, [0., 0.5, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut element = session_rust::Element::new("edited control pixel");
    element.set_geometry(mesh);
    let mut base = Scene::empty();
    let owner =
        crate::element_original::insert(&mut base, std::rc::Rc::new(element)).unwrap();
    base.owner_controls(owner).unwrap();
    let original = crate::source_edit::original(&base, owner).unwrap();
    let target = Target {
        owner,
        part: Part::SourceControl(crate::source_controls::ControlId::Vertex(10)),
    };
    for density in [1., 2.] {
        let mut scene = base.clone();
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        renderer.set_pick_mode(Mode::Control);
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
        let mut preview = crate::part_preview::Preview::new(&scene, target).unwrap();
        preview
            .set(session_rust::Xform::translation(
                0.25, 0., 0.,
            ))
            .unwrap();
        renderer.set_scene(&preview.scene, None);
        renderer
            .request_pick([-0.25, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&preview.scene, pixel),
            Some(target)
        );
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        assert!(finish(&mut renderer).is_none());
        let mut history = crate::history::History::default();
        assert!(
            preview
                .commit(&mut scene, &mut history)
                .unwrap()
        );
        renderer.set_scene(&scene, None);
        renderer
            .request_pick([-0.25, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
        assert!(history.undo(&mut scene));
        renderer.set_scene(&scene, None);
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
        assert!(crate::source_edit::same(
            &original,
            &crate::source_edit::original(&scene, owner).unwrap()
        ));
    }
}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 19.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 19.4
npm --prefix ../session_tests run course -- save 19.4
```

Continue with [20.1 · Shared surface geometry and finite visibility addresses](96zh-bind-the-shared-geometry-as-storage.md).
