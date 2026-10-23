# 13.1 · Shared original definitions and independently placed instances

**Goal:** Check sharing precision history and release. · **Steps:** 9 · **Source:** [app/scene_instances.rs](../../../src/app/scene_instances.rs) · [engine/gpu/instanced.rs](../../../src/engine/gpu/instanced.rs) · [engine/gpu/slots.rs](../../../src/engine/gpu/slots.rs) · [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs) · [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/sheet_query.rs](../../../src/app/sheet_query.rs) (`src/app/instances.rs,src/app/sheets.rs,src/engine/gpu/instances.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">1 Prototype Mesh Def -&gt; N Instance Matrices M_0..M_k</text>
  <polygon points="64,132 104,64 144,132" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="104" y="150" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Def #0</text>
  <line x1="150" y1="98" x2="214" y2="68" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arrB)"/>
  <line x1="150" y1="98" x2="214" y2="104" stroke="#16a34a" stroke-width="1.75" marker-end="url(#arrG)"/>
  <line x1="150" y1="98" x2="214" y2="140" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="72" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">M_0</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">M_1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="144" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">M_2</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D World Space vs 2D Sheet Paper Space (mm)</text>
  <polygon points="434,138 468,78 502,138" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <polygon points="494,138 528,78 562,138" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <rect x="588" y="54" width="112" height="96" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="598" y="64" width="92" height="56" fill="#f8fafc" stroke="#64748b" stroke-width="1.2"/>
  <rect x="646" y="126" width="44" height="16" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Instanced Scene + 2D Detail Sheet Viewport</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="832,138 860,82 888,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="888,138 916,82 944,138" fill="#bbf7d0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="944,138 972,82 1000,138" fill="#fee2e2" stroke="#111111" stroke-width="1.75"/>
  <rect x="1008" y="56" width="42" height="92" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
</svg>
</div>

## <a id="checkpoint-69-validate-original-definition-references"></a>1. Validate definition references

**What this code does:** Validate original definition references.

### 1. `src/instance_plan.rs`

Validate original definition references and retain shared owners.

Create the file and type:

```rust
use session_rust::{Geometry, InstanceRef, Session, proto};
use std::{collections::HashSet, rc::Rc};

pub fn validate(message: &proto::Session) -> Result<(), String> {
    let instances = message
        .objects
        .as_ref()
        .map_or(&[][..], |objects| {
            objects.instances.as_slice()
        });
    let Some(definitions) = message.definitions.as_ref() else {
        return if instances.is_empty() {
            Ok(())
        } else {
            Err("Instances have no original definitions".into())
        };
    };
    crate::source_validate::definitions(definitions, instances.len())?;
    if !definitions.planes.is_empty()
        || !definitions.bboxes.is_empty()
        || !definitions.elements.is_empty()
        || !definitions.components.is_empty()
        || !definitions.sheets.is_empty()
        || !definitions.instances.is_empty()
    {
        return Err("Unsupported original definition family".into());
    }
    let mut guids = HashSet::new();
    macro_rules! identity {
        ($field:ident) => {
            for object in &definitions.$field {
                if object.guid.is_empty() || !guids.insert(object.guid.as_str()) {
                    return Err("Definition GUIDs must be distinct".into());
                }
            }
        };
    }
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    let mut refs = HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()
            || !refs.insert(instance.guid.as_str())
            || !guids.contains(instance.definition_guid.as_str())
        {
            return Err("Invalid original instance reference".into());
        }
        if instance.flags & InstanceRef::FLAG_COLOR != 0 {
            let color = instance
                .color
                .as_ref()
                .ok_or("Instance colour flag has no original colour")?;
            if [color.r, color.g, color.b, color.a]
                .iter()
                .any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))
            {
                return Err("Invalid original instance colour".into());
            }
        }
        if !instance.features.is_empty() {
            return Err("Instance features connect with element features".into());
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct Owner {
    pub document: Rc<Session>,
    pub instance: Rc<InstanceRef>,
    pub definition: Geometry,
}
impl Owner {
    pub fn new(
        document: Rc<Session>,
        instance: Rc<InstanceRef>,
    ) -> Result<Self, &'static str> {
        let definition = document
            .definition_lookup
            .get(&instance.definition_guid)
            .ok_or("Missing original instance definition")?
            .clone();
        Ok(Self {
            document,
            instance,
            definition,
        })
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_shape;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_shape;

pub mod instance_plan;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/instance_plan.rs`

Copy source identity, invalid-reference and release fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn message() -> proto::Session {
        let mesh = crate::prepared::PreparedMesh::triangle(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            [0.1, 0.3, 0.8],
        )
        .unwrap()
        .geometry;
        proto::Session {
            objects: Some(proto::Objects {
                instances: vec![
                    proto::InstanceRef {
                        guid: "first".into(),
                        name: "First".into(),
                        definition_guid: mesh.guid().into(),
                        ..Default::default()
                    },
                    proto::InstanceRef {
                        guid: "second".into(),
                        name: "Second".into(),
                        definition_guid: mesh.guid().into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            definitions: Some(proto::Objects {
                meshes: vec![mesh.to_proto()],
                ..Default::default()
            }),
            ..Default::default()
        }
    }
    #[test]
    fn checked_original_references_share_definition_identity_without_copying_geometry()
    {
        let message = message();
        validate(&message).unwrap();
        let document = Rc::new(Session::from_proto(message).unwrap());
        let first = Owner::new(
            document.clone(),
            document.objects.instances[0].clone(),
        )
        .unwrap();
        let second = Owner::new(
            document.clone(),
            document.objects.instances[1].clone(),
        )
        .unwrap();
        let (Geometry::Mesh(a), Geometry::Mesh(b)) =
            (&first.definition, &second.definition)
        else {
            panic!("Original mesh definition")
        };
        assert!(Rc::ptr_eq(a, b));
        assert_eq!(first.instance.guid(), "first");
        assert_eq!(second.instance.guid(), "second");
        let weak = Rc::downgrade(a);
        drop(document);
        drop(first);
        assert!(weak.upgrade().is_some());
        drop(second);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn missing_duplicate_and_unsupported_original_definitions_are_refused() {
        let mut source = message();
        source.definitions = None;
        assert!(validate(&source).is_err());
        let mut source = message();
        source.objects.as_mut().unwrap().instances[0].definition_guid =
            "missing".into();
        assert!(validate(&source).is_err());
        let mut source = message();
        let duplicate = source.definitions.as_ref().unwrap().meshes[0].clone();
        source
            .definitions
            .as_mut()
            .unwrap()
            .meshes
            .push(duplicate);
        assert!(validate(&source).is_err());
        let mut source = message();
        source
            .definitions
            .as_mut()
            .unwrap()
            .instances
            .push(Default::default());
        assert!(validate(&source).is_err());
    }
}
```

## <a id="checkpoint-69b-prepare-each-used-definition-once"></a>2. Prepare each used definition once

**What this code does:** Prepare each used definition once.

### 1. `src/instance_definition.rs`

Prepare each referenced original definition once.

Create the file and type:

```rust
use crate::{instance_plan::Owner, scene::Scene};
use session_rust::Geometry;
use std::{collections::HashMap, rc::Rc};

pub fn prepare(owner: &Owner) -> Result<Scene, &'static str> {
    let mut scene = Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source) => {
            scene.insert(crate::prepared::PreparedMesh::from_shared(
                source.clone(),
            )?)?;
        }
        Geometry::BRep(source) => {
            scene.insert_cad(source.clone())?;
        }
        Geometry::NurbsSurface(source) => {
            scene.insert_surface(source.clone())?;
        }
        Geometry::NurbsCurve(source) => {
            scene.insert_path(crate::chain::PreparedChain::curve(
                source.clone(),
            )?)?;
        }
        Geometry::Polyline(source) => {
            scene.insert_path(crate::chain::PreparedChain::polyline(
                source.clone(),
            )?)?;
        }
        Geometry::Line(source) => {
            scene.insert_line(crate::stroke::PreparedLine::new(
                source.clone(),
            )?)?;
        }
        Geometry::Point(source) => {
            scene.insert_point(crate::marker::PreparedPoint::new(
                source.clone(),
            )?)?;
        }
        Geometry::PointCloud(source) => {
            scene.insert_cloud(crate::cloud::Prepared::new(source.clone())?)?;
        }
        _ => return Err("This definition family connects in a later subject"),
    }
    Ok(scene)
}

pub fn templates(
    document: Rc<session_rust::Session>,
) -> Result<HashMap<String, Scene>, &'static str> {
    let mut templates = HashMap::new();
    for instance in &document.objects.instances {
        if !templates.contains_key(&instance.definition_guid) {
            let owner = Owner::new(document.clone(), instance.clone())?;
            templates.insert(
                instance.definition_guid.clone(),
                prepare(&owner)?,
            );
        }
    }
    Ok(templates)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod instance_plan;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_plan;

pub mod instance_definition;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-69c-keep-instance-bindings-through-scene-history"></a>3. Keep instance bindings through scene history

**What this code does:** Keep instance bindings through scene history.

### 1. `src/scene.rs`

Preserve instance bindings through row identity and scene lifetime operations.

**Locate this existing block:**

```rust
pub struct Scene {
    pub(crate) created: Option<u32>,
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
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
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
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
        let object = self
            .objects
            .iter()
            .find(|object| object.id == id)?;
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
                    control,
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
        let (prepared, mesh) = crate::cad_display::prepare(source)?;
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
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.texts.push(row);
        }
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
        for old in &ids {
            let id = remap[old];
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
            if source.created == Some(old_key) && candidate.created.is_none() {
                candidate.created = Some(doc.key);
            }
            candidate.forest.push(doc);
        }
        self.restore(candidate);
        Ok(())
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
        self.created = None;
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.streamed = Vec::new();
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
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id);
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        self.clouds.retain(|row| row.id != id);
        self.streamed.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
        existed
    }

    pub fn contains(&self, id: ObjectId) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Scene {
    pub(crate) instances:
        std::collections::HashMap<ObjectId, crate::instance_plan::Owner>,
    pub(crate) created: Option<u32>,
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
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
            instances: Default::default(),
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
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
        let object = self
            .objects
            .iter()
            .find(|object| object.id == id)?;
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
                    control,
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
        let (prepared, mesh) = crate::cad_display::prepare(source)?;
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
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.texts.push(row);
        }
        if let Some(owner) = self.instances.get(&id).cloned() {
            self.instances.insert(copy, owner);
        }
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
            if source.created == Some(old_key) && candidate.created.is_none() {
                candidate.created = Some(doc.key);
            }
            candidate.forest.push(doc);
        }
        self.restore(candidate);
        Ok(())
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
        self.instances.clear();
        self.created = None;
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.streamed = Vec::new();
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
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id);
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        self.clouds.retain(|row| row.id != id);
        self.streamed.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
        existed
    }

    pub(crate) fn all_ids(&self) -> std::collections::HashSet<ObjectId> {
        (*self.ids).clone()
    }
    pub fn contains(&self, id: ObjectId) -> bool {
```

## <a id="checkpoint-69d-append-independently-placed-shared-rows"></a>4. Append independently placed shared rows

**What this code does:** Append independently placed shared rows.

### 1. `src/scene.rs`

Append checked independent instance rows without cloning the growing scene.

**Locate this existing block:**

```rust
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

## <a id="checkpoint-69e-import-canonical-original-instance-placements"></a>5. Import canonical instance placements

**What this code does:** Import canonical original instance placements.

### 1. `src/document_source.rs`

Import original definitions and apply canonical instance placement once.

**Locate this existing block:**

```rust
    identity!(pointclouds);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    Ok(Source {
        document,
        origin,
        tree,
    })
}

fn supported(source: &proto::Session) -> Result<(), String> {
    if source.definitions.is_some() || !source.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later subjects".into());
    }
    let objects = source
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.planes.is_empty()
        || !objects.bboxes.is_empty()
        || !objects.elements.is_empty()
        || !objects.components.is_empty()
        || !objects.sheets.is_empty()
        || !objects.instances.is_empty()
    {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    identity!(pointclouds);
    identity!(instances);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    Ok(Source {
        document,
        origin,
        tree,
    })
}

fn supported(source: &proto::Session) -> Result<(), String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {
        return Err("Interactions arrive with element features".into());
    }
    let objects = source
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.planes.is_empty()
        || !objects.bboxes.is_empty()
        || !objects.elements.is_empty()
        || !objects.components.is_empty()
        || !objects.sheets.is_empty()
    {
```

### 2. `src/document_geometry.rs`

Import original definitions and apply canonical instance placement once.

**Locate this existing block:**

```rust
    }
    Ok(owners)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    let templates = crate::instance_definition::templates(loaded.document.clone())
        .map_err(str::to_owned)?;
    let mut used = scene.used_guids();
    for instance in &loaded.document.objects.instances {
        let template = &templates[&instance.definition_guid];
        let mut guid = instance.guid().to_owned();
        while !used.insert(guid.clone()) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        // The kernel folds legacy instance placement into Session.xforms; combine that canonical placement with tree ancestry once, without another instance transform.
        let model = place
            * &crate::hierarchy::world(
                loaded.tree.as_ref(),
                &loaded.document.xforms,
                instance.guid(),
            );
        let owner =
            crate::instance_plan::Owner::new(loaded.document.clone(), instance.clone())
                .map_err(str::to_owned)?;
        let id = scene
            .append_instance(template, owner, model, guid, point_px)
            .map_err(str::to_owned)?;
        owners.push((
            instance.guid().to_owned(),
            id,
            instance.name.clone(),
        ));
    }
    Ok(owners)
```

### 3. `src/scene_document.rs`

Copy the revised unsupported nested-definition fixture.

**Locate this existing block:**

```rust
            .clear();
        source.definitions = Some(proto::Objects::default());
        assert!(
            import(
                &mut scene,
                &source.encode_to_vec(),
                "unsupported",
                &Xform::identity(),
                0.
            )
            .unwrap_err()
            .contains("Definitions")
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .clear();
        source.definitions = Some(proto::Objects {
            instances: vec![Default::default()],
            ..Default::default()
        });
        assert!(
            import(
                &mut scene,
                &source.encode_to_vec(),
                "unsupported",
                &Xform::identity(),
                0.
            )
            .unwrap_err()
            .contains("definition")
        );
```

## <a id="checkpoint-69f-protect-shared-originals-during-part-edits"></a>6. Protect shared originals during part edits

**What this code does:** Protect shared originals during part edits.

### 1. `src/part_preview.rs`

Protect original definitions from unsupported source-part edits.

**Locate this existing block:**

```rust
pub fn points(scene: &Scene, target: Target) -> Result<Vec<Point>, &'static str> {
    if !target.valid(scene) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn points(scene: &Scene, target: Target) -> Result<Vec<Point>, &'static str> {
    if scene.instances.contains_key(&target.owner) {
        return Err("Detach the shared definition before editing its source geometry");
    }
    if !target.valid(scene) {
```

## <a id="checkpoint-69g-collect-shared-definitions-without-flattening"></a>7. Collect shared definitions without flattening

**What this code does:** Collect shared definitions without flattening.

### 1. `src/instance_snapshot.rs`

Collect original definitions once and reject semantic identity conflicts.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::{Geometry, proto};
use std::collections::{HashMap, HashSet};

pub fn definitions(
    scene: &Scene,
    ids: &HashSet<ObjectId>,
) -> Result<Option<proto::Objects>, String> {
    let mut out = proto::Objects::default();
    let mut records = HashMap::new();
    macro_rules! keep {
        ($field:ident,$source:expr) => {{
            let shared = $source;
            let key = shared.guid().to_owned();
            let pointer = std::rc::Rc::as_ptr(shared) as usize;
            if let Some((kind, index, previous)) = records.get(&key) {
                if *kind != stringify!($field) {
                    return Err(
                        "Different definition families reuse a saved GUID".into(),
                    );
                }
                if *previous != pointer
                    && out
                        .$field
                        .get(*index)
                        .is_none_or(|saved| saved != &shared.to_proto())
                {
                    return Err(
                        "Different originals reuse a saved definition GUID".into(),
                    );
                }
            } else {
                let source = shared.to_proto();
                records.insert(
                    key,
                    (stringify!($field), out.$field.len(), pointer),
                );
                out.$field.push(source);
            }
        }};
    }

    for (_, owner) in scene
        .instances
        .iter()
        .filter(|(id, _)| ids.contains(id))
    {
        match &owner.definition {
            Geometry::Mesh(source) => keep!(meshes, source),
            Geometry::BRep(source) => keep!(breps, source),
            Geometry::NurbsSurface(source) => keep!(nurbssurfaces, source),
            Geometry::NurbsCurve(source) => keep!(nurbscurves, source),
            Geometry::Polyline(source) => keep!(polylines, source),
            Geometry::Line(source) => keep!(lines, source),
            Geometry::Point(source) => keep!(points, source),
            Geometry::PointCloud(source) => keep!(pointclouds, source),
            _ => return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {
        ($field:ident) => {
            out.$field
                .sort_by(|a, b| a.guid.cmp(&b.guid));
        };
    }
    order!(meshes);
    order!(breps);
    order!(nurbssurfaces);
    order!(nurbscurves);
    order!(polylines);
    order!(lines);
    order!(points);
    order!(pointclouds);
    Ok((!records.is_empty()).then_some(out))
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod instance_definition;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_definition;

pub mod instance_snapshot;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-69h-save-original-instance-references-and-namespaces"></a>8. Save instance references and namespaces

**What this code does:** Save original instance references and namespaces.

### 1. `src/geometry_snapshot.rs`

Save original references and definitions without flattening source geometry.

**Locate this existing block:**

```rust
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        if let Some(cad) = row.cad.as_ref() {
            push!(breps, row.id, cad.source.to_proto());
        } else if let Some(surface) = row.surface.as_ref() {
            push!(
                nurbssurfaces,
                row.id,
                surface.source.to_proto()
            );
        } else {
            push!(
                meshes,
                row.id,
                row.geometry()
                    .ok_or("Reload editable sources before saving")?
                    .to_proto()
            );
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(lines, row.id, row.prepared.source.to_proto());
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        match &row.prepared.source {
            crate::chain::Source::Line(source) => {
                push!(lines, row.id, source.to_proto())
            }
            crate::chain::Source::Polyline(source) => {
                push!(polylines, row.id, source.to_proto())
            }
            crate::chain::Source::Curve(source, _) => {
                push!(nurbscurves, row.id, source.to_proto())
            }
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(
            points,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(
            pointclouds,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    Ok(out)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .iter()
        .filter(|row| ids.contains(&row.id) && !scene.instances.contains_key(&row.id))
    {
        if let Some(cad) = row.cad.as_ref() {
            push!(breps, row.id, cad.source.to_proto());
        } else if let Some(surface) = row.surface.as_ref() {
            push!(
                nurbssurfaces,
                row.id,
                surface.source.to_proto()
            );
        } else {
            push!(
                meshes,
                row.id,
                row.geometry()
                    .ok_or("Reload editable sources before saving")?
                    .to_proto()
            );
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| ids.contains(&row.id) && !scene.instances.contains_key(&row.id))
    {
        push!(lines, row.id, row.prepared.source.to_proto());
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| ids.contains(&row.id) && !scene.instances.contains_key(&row.id))
    {
        match &row.prepared.source {
            crate::chain::Source::Line(source) => {
                push!(lines, row.id, source.to_proto())
            }
            crate::chain::Source::Polyline(source) => {
                push!(polylines, row.id, source.to_proto())
            }
            crate::chain::Source::Curve(source, _) => {
                push!(nurbscurves, row.id, source.to_proto())
            }
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| ids.contains(&row.id) && !scene.instances.contains_key(&row.id))
    {
        push!(
            points,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| ids.contains(&row.id) && !scene.instances.contains_key(&row.id))
    {
        push!(
            pointclouds,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    for (id, owner) in scene
        .instances
        .iter()
        .filter(|(id, _)| ids.contains(id))
    {
        let mut source = owner.instance.to_proto();
        source.guid = scene
            .row_guid(*id)
            .ok_or("Original instance owner is missing")?
            .into();
        source.flags = (source.flags
            & !(session_rust::InstanceRef::FLAG_HIDDEN
                | session_rust::InstanceRef::FLAG_LOCKED))
            | if scene.visible(*id) {
                0
            } else {
                session_rust::InstanceRef::FLAG_HIDDEN
            }
            | if scene.view.locked.contains(id) {
                session_rust::InstanceRef::FLAG_LOCKED
            } else {
                0
            };
        out.instances.push(source);
    }
    out.instances
        .sort_by(|a, b| a.guid.cmp(&b.guid));
    Ok(out)
```

### 2. `src/archive_identity.rs`

Save original references and definitions without flattening source geometry.

**Locate this existing block:**

```rust
    remap!(pointclouds);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    remap!(pointclouds);
    remap!(instances);
}
```

### 3. `src/archive_document.rs`

Save original references and definitions without flattening source geometry.

**Locate this existing block:**

```rust
    };
    source.objects = Some(objects);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    source.definitions = crate::instance_snapshot::definitions(scene, &namespace.ids)?;
    source.objects = Some(objects);
```

## <a id="checkpoint-69i-check-sharing-precision-history-and-release"></a>9. Check sharing precision, history, and release

**What this code does:** Check sharing precision history and release.

### 1. `src/memory.rs`

Count original instance documents and definitions across shared scene snapshots.

**Locate this existing block:**

```rust
                }
            }
        }
    }
    usage[1] = sources.len();
    usage
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
            }
        }
    }
    usage[1] = sources.len();
    usage
}

pub fn instances<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut documents = HashSet::new();
    let mut definitions = HashSet::new();
    let mut usage = [0; 3];
    for scene in scenes {
        usage[0] += scene.instances.len();
        for owner in scene.instances.values() {
            let document = Rc::as_ptr(&owner.document) as usize;
            documents.insert(document);
            definitions.insert((document, owner.definition.guid().to_owned()));
        }
    }
    usage[1] = documents.len();
    usage[2] = definitions.len();
    usage
}
```

### 2. `src/instance_specimen.rs`

Copy precise original and visible shared-definition examples.

Create the file and type:

```rust
use prost::Message;
use session_rust::proto;

pub fn message() -> proto::Session {
    let mesh = crate::prepared::PreparedMesh::triangle(
        [
            [-0.4, -0.4, 0.],
            [0.4, -0.4, 0.],
            [0., 0.4, 0.],
        ],
        [0.1, 0.3, 0.8],
    )
    .unwrap()
    .geometry;
    proto::Session {
        name: "Shared original definitions".into(),
        guid: "definition-document".into(),
        objects: Some(proto::Objects {
            instances: vec![
                proto::InstanceRef {
                    guid: "first".into(),
                    name: "First".into(),
                    definition_guid: mesh.guid().into(),
                    xform: Some(
                        session_rust::Xform::translation(1000., 0., 0.).to_proto(),
                    ),
                    ..Default::default()
                },
                proto::InstanceRef {
                    guid: "second".into(),
                    name: "Second".into(),
                    definition_guid: mesh.guid().into(),
                    flags: session_rust::InstanceRef::FLAG_LOCKED,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
        definitions: Some(proto::Objects {
            meshes: vec![mesh.to_proto()],
            ..Default::default()
        }),
        tree: Some(proto::Tree {
            root: Some(proto::TreeNode {
                name: "root".into(),
                guid: "root-node".into(),
                children: vec![
                    proto::TreeNode {
                        name: "first".into(),
                        guid: "first-node".into(),
                        ..Default::default()
                    },
                    proto::TreeNode {
                        name: "second".into(),
                        guid: "second-node".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        }),
        xforms: vec![
            proto::XformEntry {
                guid: "root".into(),
                xform: Some(
                    session_rust::Xform::translation(1_000_000_000.125, 0., 0.)
                        .to_proto(),
                ),
            },
            proto::XformEntry {
                guid: "first".into(),
                xform: Some(session_rust::Xform::translation(-1., 0., 0.).to_proto()),
            },
            proto::XformEntry {
                guid: "second".into(),
                xform: Some(session_rust::Xform::translation(1., 0., 0.).to_proto()),
            },
        ],
        ..Default::default()
    }
}
pub fn bytes() -> Vec<u8> {
    message().encode_to_vec()
}

// A small shared-definition example keeps placements near the camera while the precision fixture exercises distant original coordinates.
pub fn display_bytes() -> Vec<u8> {
    let mut source = message();
    source.objects.as_mut().unwrap().instances[0].xform = None;
    source.objects.as_mut().unwrap().instances[1].flags = 0;
    source.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(session_rust::Xform::translation(-0.6, 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(session_rust::Xform::translation(0.6, 0., 0.).to_proto()),
        },
    ];
    source.encode_to_vec()
}
```

### 3. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod instance_snapshot;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_snapshot;

pub mod instance_specimen;

#[cfg(target_arch = "wasm32")]
```

### 4. `src/instance_scene_tests.rs`

Copy original reference, family, identity, placement, history and release regressions.

Create the file and type:

```rust
use crate::{archive::Bundle, instance_specimen, scene::Scene, scene_document};
use prost::Message;
use session_rust::{Xform, proto};
use std::rc::Rc;

#[test]
fn original_instances_share_geometry_apply_session_placement_once_and_roundtrip_references()
 {
    let source = instance_specimen::message();
    let original = source.definitions.as_ref().unwrap().meshes[0].clone();
    let mut scene = Scene::empty();
    scene_document::import(
        &mut scene,
        &source.encode_to_vec(),
        "",
        &Xform::translation(0.25, 0., 0.),
        0.,
    )
    .unwrap();
    let a = &scene.objects()[0];
    let b = &scene.objects()[1];
    assert!(Rc::ptr_eq(&a.mesh, &b.mesh));
    assert!(Rc::ptr_eq(
        a.geometry().unwrap(),
        b.geometry().unwrap()
    ));
    assert_ne!(a.id, b.id);
    assert_ne!(a.guid, b.guid);
    assert_eq!(
        a.model.m[12],
        1_000_000_000.125 - 1. + 1000. + 0.25
    );
    assert_eq!(b.model.m[12], 1_000_000_000.125 + 1. + 0.25);
    assert!(scene.pick_id(a.id.value()).is_some());
    assert!(scene.pick_id(b.id.value()).is_none());
    assert_eq!(a.geometry().unwrap().to_proto(), original);
    assert!(
        scene.instances[&a.id]
            .instance
            .xform
            .is_identity()
    );
    let saved = Bundle::decode(&crate::scene_archive::save(&scene).unwrap()).unwrap();
    let document = proto::Session::decode(saved.documents[0].as_slice()).unwrap();
    assert!(
        document
            .objects
            .as_ref()
            .unwrap()
            .meshes
            .is_empty()
    );
    assert_eq!(
        document
            .objects
            .as_ref()
            .unwrap()
            .instances
            .len(),
        2
    );
    assert_eq!(
        document.definitions.as_ref().unwrap().meshes,
        [original.clone()]
    );
    let mut restored = Scene::empty();
    scene_document::open(
        &mut restored,
        &crate::scene_archive::save(&scene).unwrap(),
    )
    .unwrap();
    assert_eq!(restored.objects().len(), 2);
    assert!(Rc::ptr_eq(
        &restored.objects()[0].mesh,
        &restored.objects()[1].mesh
    ));
    assert_eq!(restored.objects()[0].model.m, a.model.m);
    assert_eq!(restored.objects()[1].model.m, b.model.m);
    assert_eq!(
        restored.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}
#[test]
fn copied_and_deleted_instance_rows_keep_definitions_and_release_closed_history() {
    let mut scene = Scene::empty();
    scene_document::import(
        &mut scene,
        &instance_specimen::bytes(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let first = scene.objects()[0].id;
    let second = scene.objects()[1].id;
    let source = Rc::downgrade(scene.objects()[0].geometry().unwrap());
    let document = Rc::downgrade(&scene.instances[&first].document);
    let copy = scene.copy_row(first, false).unwrap();
    scene
        .translate(copy, [0.25, 0., 0.])
        .unwrap();
    assert!(Rc::ptr_eq(
        scene.objects()[0].geometry().unwrap(),
        scene.objects()[2].geometry().unwrap()
    ));
    assert!(scene.remove(second));
    let saved = Bundle::decode(&crate::scene_archive::save(&scene).unwrap()).unwrap();
    let refs = saved
        .documents
        .iter()
        .map(|raw| {
            proto::Session::decode(raw.as_slice())
                .unwrap()
                .objects
                .unwrap()
                .instances
                .len()
        })
        .sum::<usize>();
    assert_eq!(refs, 2);
    let bytes = crate::scene_archive::save(&scene).unwrap();
    let mut restored = Scene::empty();
    scene_document::open(&mut restored, &bytes).unwrap();
    assert_eq!(restored.objects().len(), 2);
    assert!(
        restored
            .objects()
            .iter()
            .all(|row| restored.instances.contains_key(&row.id))
    );
    scene.clear();
    assert!(document.upgrade().is_none());
    assert!(source.upgrade().is_none());
}
#[test]
fn invalid_definition_references_and_part_edits_leave_existing_sources_unchanged() {
    let mut scene = Scene::demo();
    let before = scene.objects()[0].id;
    let mut invalid = instance_specimen::message();
    invalid.objects.as_mut().unwrap().instances[0].definition_guid = "missing".into();
    assert!(
        scene_document::import(
            &mut scene,
            &invalid.encode_to_vec(),
            "",
            &Xform::identity(),
            0.
        )
        .is_err()
    );
    assert_eq!(scene.objects()[0].id, before);
    scene.clear();
    scene_document::import(
        &mut scene,
        &instance_specimen::bytes(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let owner = scene.objects()[0].id;
    let target = crate::pick_sources::Target {
        owner,
        part: crate::pick_sources::Part::Face(0),
    };
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    assert!(crate::part_preview::Preview::new(&scene, target).is_err());
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}

#[test]
fn shared_instance_snapshots_count_original_owners_once_and_conflicting_definitions_cannot_merge()
 {
    let mut scene = Scene::empty();
    scene_document::import(
        &mut scene,
        &instance_specimen::bytes(),
        "first",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let snapshot = scene.clone();
    assert_eq!(
        crate::memory::instances([&scene, &snapshot]),
        [4, 1, 1]
    );
    let mut other = instance_specimen::message();
    let guid = scene
        .instances
        .values()
        .next()
        .unwrap()
        .definition
        .guid()
        .to_owned();
    other.definitions.as_mut().unwrap().meshes[0].guid = guid.clone();
    other.definitions.as_mut().unwrap().meshes[0].name = "Different original".into();
    for instance in &mut other.objects.as_mut().unwrap().instances {
        instance.definition_guid = guid.clone();
    }
    scene_document::import(
        &mut scene,
        &other.encode_to_vec(),
        "second",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(crate::memory::instances([&scene]), [4, 2, 2]);
    assert!(crate::scene_archive::save(&scene).is_ok());
    let second = scene.objects()[2].id;
    let first = scene.forest.documents[0].clone();
    let key = format!(
        "{}/{}",
        first.key, first.nodes[0].source.guid
    );
    crate::layer_edit::apply(
        &mut scene,
        &key,
        crate::layer_edit::Edit::Move(vec![second]),
    )
    .unwrap();
    assert!(
        crate::scene_archive::save(&scene)
            .unwrap_err()
            .contains("Different originals")
    );
    scene.clear();
    assert_eq!(crate::memory::instances([&scene]), [0, 0, 0]);
}

#[test]
fn instance_colour_overrides_one_placement_without_mutating_definition_and_history_is_reversible()
 {
    use crate::editor::{Action, Editor};
    let mut original =
        proto::Session::decode(instance_specimen::display_bytes().as_slice()).unwrap();
    let definition = original.definitions.as_ref().unwrap().meshes[0].clone();
    let first = &mut original.objects.as_mut().unwrap().instances[0];
    first.flags |= session_rust::InstanceRef::FLAG_COLOR;
    first.color = Some(proto::Color {
        r: 0.23,
        g: 0.75,
        b: 0.11,
        a: 0.5,
        ..Default::default()
    });
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    let mut ready = Scene::empty();
    scene_document::import(
        &mut ready,
        &original.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    editor
        .apply(Action::Adopt {
            scene: Box::new(ready),
            replace: false,
        })
        .unwrap();
    let id = editor.scene.objects()[0].id;
    assert_eq!(
        editor.scene.face_rgba(id),
        Some([0.23, 0.75, 0.11, 0.5])
    );
    assert_eq!(
        editor
            .scene
            .face_rgba(editor.scene.objects()[1].id),
        None
    );
    assert_eq!(
        editor.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        definition
    );
    editor
        .apply(Action::Picked(Some(id)))
        .unwrap();
    editor.apply(Action::Delete).unwrap();
    assert_eq!(editor.scene.instances.len(), 1);
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.instances.contains_key(&id));
    editor.apply(Action::Redo).unwrap();
    assert!(!editor.scene.instances.contains_key(&id));
    editor.apply(Action::Undo).unwrap();
    let bytes = crate::scene_archive::save(&editor.scene).unwrap();
    let mut restored = Scene::empty();
    scene_document::open(&mut restored, &bytes).unwrap();
    assert_eq!(
        restored.face_rgba(restored.objects()[0].id),
        Some([0.23, 0.75, 0.11, 0.5])
    );
    editor.apply(Action::Close).unwrap();
    assert_eq!(
        crate::memory::instances(editor.scenes()),
        [0, 0, 0]
    );
}

#[test]
fn every_basic_definition_family_roundtrips_as_original_references() {
    use crate::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    for action in [
        Action::AddBox,
        Action::AddCad,
        Action::AddSurface,
        Action::AddCurve,
        Action::AddPolyline,
        Action::AddLine,
        Action::AddPoint,
        Action::AddCloud,
    ] {
        editor.apply(action).unwrap();
    }
    let mut definitions =
        crate::geometry_snapshot::objects(&editor.scene, &editor.scene.all_ids())
            .unwrap();
    definitions.instances.clear();
    let mut refs = Vec::new();
    let mut index = 0;
    macro_rules! reference {
        ($field:ident) => {
            for definition in &definitions.$field {
                for n in 0..2 {
                    refs.push(proto::InstanceRef {
                        guid: format!("original-{index}-{n}"),
                        name: format!("Placement {index}-{n}"),
                        definition_guid: definition.guid.clone(),
                        ..Default::default()
                    });
                }
                index += 1;
            }
        };
    }
    reference!(meshes);
    reference!(breps);
    reference!(nurbssurfaces);
    reference!(nurbscurves);
    reference!(polylines);
    reference!(lines);
    reference!(points);
    reference!(pointclouds);
    assert_eq!(index, 8);
    let original = proto::Session {
        objects: Some(proto::Objects {
            instances: refs,
            ..Default::default()
        }),
        definitions: Some(definitions.clone()),
        ..Default::default()
    };
    let mut scene = Scene::empty();
    scene_document::import(
        &mut scene,
        &original.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(scene.instances.len(), 16);
    let bytes = crate::scene_archive::save(&scene).unwrap();
    let bundle = Bundle::decode(&bytes).unwrap();
    let saved = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
    assert_eq!(saved.objects.unwrap().instances.len(), 16);
    let saved_defs = saved.definitions.unwrap();
    macro_rules! equal {
        ($field:ident) => {
            assert_eq!(saved_defs.$field, definitions.$field);
        };
    }
    equal!(meshes);
    equal!(breps);
    equal!(nurbssurfaces);
    equal!(nurbscurves);
    equal!(polylines);
    equal!(lines);
    equal!(points);
    equal!(pointclouds);
    let mut restored = Scene::empty();
    scene_document::open(&mut restored, &bytes).unwrap();
    assert_eq!(restored.instances.len(), 16);
    assert_eq!(
        crate::memory::instances([&restored]),
        [16, 1, 8]
    );
}
```

### 5. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod instance_specimen;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_specimen;

#[cfg(test)]
mod instance_scene_tests;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 13.1 (69i-check-sharing-precision-history-and-release)](../../screenshots/journey/69i-check-sharing-precision-history-and-release.png)

```sh
npm --prefix ../session_tests run course -- dependencies 13.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 13.1
npm --prefix ../session_tests run course -- save 13.1
```

Continue with [13.2 · Independent presentation and reusable CAD preparation](69j-preserve-per-instance-original-colours.md).
