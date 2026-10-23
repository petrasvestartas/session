# 3.5 · Save, replace and asynchronous transaction results

**Goal:** Test partial replacement failure, preserved selection/Redo and placement upload limits. · **Steps:** 11 · **Source:** `src/app/decode.rs,src/app/loader.rs,src/state.rs`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Hierarchy: M_world = M_parent * M_child</text>
  <g transform="translate(88,122)">
    <line x1="0" y1="0" x2="48" y2="0" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
    <line x1="0" y1="0" x2="0" y2="-44" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
    <circle cx="0" cy="0" r="4" fill="#111111"/>
    <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="-10" y="18" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Parent</text>
  </g>
  <path d="M 116 106 Q 168 68 214 86" fill="none" stroke="#111111" stroke-width="1.75" stroke-dasharray="4 3" marker-end="url(#arr)"/>
  <g transform="translate(230,94) rotate(-22)">
    <rect x="-18" y="-14" width="36" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
    <line x1="0" y1="0" x2="34" y2="0" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
    <line x1="0" y1="0" x2="0" y2="-30" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  </g>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Window (Inside) vs Crossing (Intersect) Rect</text>
  <rect x="424" y="54" width="118" height="88" fill="#dbeafe" fill-opacity="0.35" stroke="#2563eb" stroke-width="2"/>
  <polygon points="454,122 484,76 516,122" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="572" y="54" width="118" height="88" fill="#dcfce7" fill-opacity="0.35" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <polygon points="642,124 678,72 714,124" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Selected Subtree &amp; Isolated Visibility</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,138 882,84 918,138" fill="#e2e8f0" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <polygon points="942,138 986,68 1030,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
</svg>
</div>

## <a id="checkpoint-29-identity"></a>1. Give each saved object a stable identity

**What this code does:** Distinguish a scene object from the original geometry it shares.

### 1. `Cargo.toml`

Declare the UUID generator explicitly; browser randomness uses its js feature.

**Locate this existing block:**

```toml
prost = "=0.14.4"
js-sys = "=0.3.105"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
prost = "=0.14.4"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
```

### 2. `src/scene.rs`

Store the inserted object identity separately from kernel source identity and the local counter. Keep the source GUID when available; assign and check a fresh stored GUID for a duplicate import.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object {
            id,
            mesh: Rc::new(prepared.display),
```

Replace it with:

```rust
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
        self.objects.push(Object {
            id,
            guid,
            mesh: Rc::new(prepared.display),
```

**Locate this block:**

```rust
    pub id: ObjectId,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

Replace it with:

```rust
    pub id: ObjectId,
    pub guid: String,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

### 3. `src/file_identity_tests.rs`

Optional authoring fixture. Exercise duplicate imports, row removal and history without mutating original source GUIDs.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    specimen,
};
use std::collections::HashSet;

#[test]
fn duplicate_imports_keep_distinct_stored_identity_through_history() {
    let mut editor = Editor::default();
    let bytes = specimen::bytes();
    editor
        .apply(Action::Import(bytes.clone()))
        .unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let objects = editor.scene.objects();
    assert_eq!(objects.len(), 8);
    let distinct: HashSet<_> = objects
        .iter()
        .map(|object| object.guid.as_str())
        .collect();
    assert_eq!(distinct.len(), objects.len());
    assert_eq!(objects[2].guid, objects[2].geometry.guid());
    assert_eq!(
        objects[2].geometry.guid(),
        objects[5].geometry.guid()
    );
    assert_eq!(
        objects[2].source.as_ref().unwrap().guid,
        objects[5].source.as_ref().unwrap().guid
    );
    assert_ne!(objects[2].guid, objects[5].guid);
    assert!(uuid::Uuid::parse_str(&objects[5].guid).is_ok());
    let copy = objects[5].clone();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::Redo).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    assert_eq!(current.geometry.guid(), copy.geometry.guid());
}
```

### 4. `src/lib.rs`

Compile the file-identity regression check.

**Locate this existing block:**

```rust
mod document_tests;
#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod document_tests;
#[cfg(test)]
mod file_identity_tests;
#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-29a-snapshot"></a>2. Write a snapshot from the editable sources

**What this code does:** Serialize live source meshes and their placements without changing the editor.

### 1. `src/document.rs`

Copy live sources into one bounded flat document, write placements separately and return bytes without editor mutation.

**Locate this existing block:**

```rust
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
    let mut xforms = Vec::new();
    for object in scene.objects() {
        let mut mesh = object.geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        children.push(proto::TreeNode {
            name: object.guid.clone(),
            ..Default::default()
        });
        if object.model.m != session_rust::Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: object.guid.clone(),
                xform: Some(object.model.to_proto()),
            });
        }
    }
    let message = proto::Session {
        name: "Viewer document".into(),
        objects: Some(objects),
        tree: Some(proto::Tree {
            root: Some(proto::TreeNode {
                name: "Objects".into(),
                children,
                ..Default::default()
            }),
            ..Default::default()
        }),
        xforms,
        ..Default::default()
    };
    // Source validation; placements come next.
    let mut geometry = message.clone();
    geometry.xforms.clear();
    validate(&geometry)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES {
        return Err("The saved document exceeds 4 MiB");
    }
    Ok(bytes)
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
```

### 2. `src/snapshot_tests.rs`

Optional authoring fixture. Check live row membership, source topology/attributes, separate placement and untouched history.

Create the file and type:

```rust
use crate::{
    document,
    editor::{Action, Editor},
};
use prost::Message;
use session_rust::proto;

#[test]
fn snapshot_uses_live_local_sources_and_separate_placements() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    editor.apply(Action::AddBox).unwrap();
    let object = editor
        .scene
        .objects()
        .last()
        .unwrap()
        .clone();
    let source = object.geometry.to_proto();
    let message = proto::Session::decode(
        document::snapshot(&editor.scene)
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    let meshes = &message.objects.as_ref().unwrap().meshes;
    assert_eq!(meshes.len(), 2);
    let saved = meshes
        .iter()
        .find(|mesh| mesh.guid == object.guid)
        .unwrap();
    assert_eq!(saved.vertices, source.vertices);
    assert_eq!(saved.faces, source.faces);
    assert_eq!(saved.objectcolor, source.objectcolor);
    assert_eq!(
        (saved.is_visible, saved.is_locked),
        (source.is_visible, source.is_locked)
    );
    let placement = message
        .xforms
        .iter()
        .find(|entry| entry.guid == object.guid)
        .unwrap();
    assert_eq!(
        placement.xform.as_ref().unwrap().matrix,
        object.model.m.to_vec()
    );
    assert_eq!(
        message
            .tree
            .unwrap()
            .root
            .unwrap()
            .children
            .len(),
        meshes.len()
    );
    assert_eq!(object.geometry.guid(), source.guid);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(
        editor.scene.objects().len(),
        1,
        "Saving must not create a history entry"
    );
}

#[test]
fn an_empty_scene_has_no_saveable_mesh_document_yet() {
    let mut editor = Editor::default();
    let ids: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|object| object.id)
        .collect();
    for id in ids {
        editor.scene.remove(id);
    }
    assert!(document::snapshot(&editor.scene).is_err());
}
```

### 3. `src/lib.rs`

Compile the read-only snapshot checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod source_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod source_tests;
```

## <a id="checkpoint-29b-placements"></a>3. Reopen source geometry with its placement

**What this code does:** Validate stored placements before reconstructing source-backed objects.

### 1. `src/placement.rs`

Use one affine validation rule for decoded matrices and direct edits.

Create the file and type:

```rust
pub fn valid(matrix: &[f64]) -> bool {
    matrix.len() == 16
        && matrix.iter().all(|value| value.is_finite())
        && [matrix[3], matrix[7], matrix[11], matrix[15]] == [0.0, 0.0, 0.0, 1.0]
}
```

### 2. `src/lib.rs`

Expose the shared matrix validator.

**Locate this existing block:**

```rust
pub mod picking;
pub mod prepared;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod picking;
pub mod placement;
pub mod prepared;
```

### 3. `src/scene.rs`

Make direct placement edits use the same rule as imported files.

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        if !model.m.iter().all(|v| v.is_finite())
            || [
                model.m[3],
                model.m[7],
                model.m[11],
                model.m[15],
            ] != [0.0, 0.0, 0.0, 1.0]
        {
            return Err("Placement must be finite and affine");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite and affine");
```

### 4. `src/prepared.rs`

Carry the validated object placement beside local source geometry. Generated geometry begins with identity placement.

**Locate this existing block:**

```rust
    pub source: Option<Source>,
}

impl PreparedMesh {
    pub fn triangle(
        points: [[f64; 3]; 3],
        colour: [f32; 3],
    ) -> Result<Self, &'static str> {
        let vertices = points
            .into_iter()
            .map(|p| session_rust::Point::new(p[0], p[1], p[2]))
            .collect();
        let mut geometry =
            session_rust::Mesh::from_vertices_and_faces(vertices, vec![vec![0, 1, 2]]);
        geometry.set_objectcolor(session_rust::Color::new(
            colour[0], colour[1], colour[2], 1.0,
        ));
        Self::new(geometry)
    }

    pub fn new(geometry: session_rust::Mesh) -> Result<Self, &'static str> {
        Self::from_shared(Rc::new(geometry))
    }

    pub fn from_shared(geometry: Rc<session_rust::Mesh>) -> Result<Self, &'static str> {
        // The kernel creates a GUID lazily; establish identity before sharing.
        let _ = geometry.guid();
        let display = Mesh::from_kernel(&geometry)?;
        Ok(Self {
            geometry,
            display,
            source: None,
        })
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub source: Option<Source>,
    pub model: session_rust::Xform,
}

impl PreparedMesh {
    pub fn triangle(
        points: [[f64; 3]; 3],
        colour: [f32; 3],
    ) -> Result<Self, &'static str> {
        let vertices = points
            .into_iter()
            .map(|p| session_rust::Point::new(p[0], p[1], p[2]))
            .collect();
        let mut geometry =
            session_rust::Mesh::from_vertices_and_faces(vertices, vec![vec![0, 1, 2]]);
        geometry.set_objectcolor(session_rust::Color::new(
            colour[0], colour[1], colour[2], 1.0,
        ));
        Self::new(geometry)
    }

    pub fn new(geometry: session_rust::Mesh) -> Result<Self, &'static str> {
        Self::from_shared(Rc::new(geometry))
    }

    pub fn from_shared(geometry: Rc<session_rust::Mesh>) -> Result<Self, &'static str> {
        // The kernel creates a GUID lazily; establish identity before sharing.
        let _ = geometry.guid();
        let display = Mesh::from_kernel(&geometry)?;
        Ok(Self {
            geometry,
            display,
            source: None,
            model: session_rust::Xform::identity(),
        })
```

### 5. `src/document.rs`

Retain the validated stored placement without baking it into source vertices.

**Locate this existing block:**

```rust
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.source = Some(Source {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
```

### 6. `src/scene.rs`

Insert each prepared model exactly once.

**Locate this existing block:**

```rust
            geometry: prepared.geometry,
            model: session_rust::Xform::identity(),
            source: prepared.source,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            geometry: prepared.geometry,
            model: prepared.model,
            source: prepared.source,
```

### 7. `src/document.rs`

Remove the temporary source-only validation comment. Validate the complete saved message now that the loader accepts placements. Accept placements while keeping the other unsupported source families explicit. Reject unknown, duplicate, incomplete, non-finite or projective placement records before kernel construction.

**Locate this existing block:**

```rust
    };
    // Source validation; placements come next.
    let mut geometry = message.clone();
    geometry.xforms.clear();
    validate(&geometry)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES {
        return Err("The saved document exceeds 4 MiB");
    }
    Ok(bytes)
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if !message.xforms.is_empty()
        || message.definitions.is_some()
        || !message.interactions.is_empty()
    {
        return Err(
            "Placements, definitions and interactions arrive in later checkpoints",
        );
    }
    let other = objects.points.len()
        + objects.lines.len()
        + objects.planes.len()
        + objects.bboxes.len()
        + objects.polylines.len()
        + objects.pointclouds.len()
        + objects.nurbscurves.len()
        + objects.nurbssurfaces.len()
        + objects.breps.len()
        + objects.elements.len()
        + objects.components.len()
        + objects.sheets.len()
        + objects.instances.len();
    if other != 0 || objects.meshes.is_empty() || objects.meshes.len() > 64 {
        return Err("This checkpoint accepts 1–64 meshes and no other geometry types");
    }
    let mut guids = HashSet::new();
    for mesh in &objects.meshes {
        if mesh.guid.is_empty() || !guids.insert(mesh.guid.as_str()) {
            return Err("Every mesh needs a distinct source GUID");
        }
        validate_mesh(mesh)?;
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    validate(&message)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES {
        return Err("The saved document exceeds 4 MiB");
    }
    Ok(bytes)
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later checkpoints");
    }
    let other = objects.points.len()
        + objects.lines.len()
        + objects.planes.len()
        + objects.bboxes.len()
        + objects.polylines.len()
        + objects.pointclouds.len()
        + objects.nurbscurves.len()
        + objects.nurbssurfaces.len()
        + objects.breps.len()
        + objects.elements.len()
        + objects.components.len()
        + objects.sheets.len()
        + objects.instances.len();
    if other != 0 || objects.meshes.is_empty() || objects.meshes.len() > 64 {
        return Err("This checkpoint accepts 1–64 meshes and no other geometry types");
    }
    let mut guids = HashSet::new();
    for mesh in &objects.meshes {
        if mesh.guid.is_empty() || !guids.insert(mesh.guid.as_str()) {
            return Err("Every mesh needs a distinct source GUID");
        }
        validate_mesh(mesh)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        let matrix = &entry
            .xform
            .as_ref()
            .ok_or("Missing placement matrix")?
            .matrix;
        if !guids.contains(entry.guid.as_str())
            || !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(matrix)
        {
            return Err("Invalid mesh placement");
        }
    }
```

### 8. `src/placement_load_tests.rs`

Optional authoring fixture. Reject all malformed placement families using actual encoded messages.

Create the file and type:

```rust
use crate::{document, specimen};
use prost::Message;
use session_rust::{Xform, proto};

#[test]
fn malformed_placements_are_rejected_before_construction() {
    let base = proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    let guid = base.objects.as_ref().unwrap().meshes[0]
        .guid
        .clone();
    let entry = proto::XformEntry {
        guid,
        xform: Some(Xform::identity().to_proto()),
    };
    let mut nonfinite = Xform::identity().m.to_vec();
    nonfinite[0] = f64::NAN;
    let mut projective = Xform::identity().m.to_vec();
    projective[3] = 0.5;
    for matrix in [
        Vec::new(),
        vec![0.0; 15],
        nonfinite,
        projective,
    ] {
        let mut message = base.clone();
        let mut invalid = entry.clone();
        invalid.xform.as_mut().unwrap().matrix = matrix;
        message.xforms = vec![invalid];
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
    let mut missing = entry.clone();
    missing.xform = None;
    let mut orphan = entry.clone();
    orphan.guid = "not-a-mesh".into();
    for entries in [
        vec![missing],
        vec![orphan],
        vec![entry.clone(), entry.clone()],
    ] {
        let mut message = base.clone();
        message.xforms = entries;
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
}
```

### 9. `src/lib.rs`

Compile the malformed-input checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod placement_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod placement_load_tests;
#[cfg(test)]
mod placement_tests;
```

## <a id="checkpoint-29c-roundtrip"></a>4. Prove the saved document reopens faithfully

**What this code does:** Check exact local source data, attributes, identities and placement through saving and loading.

### 1. `src/save_roundtrip_tests.rs`

Optional authoring fixture. Compare exact source values, stored identities and object matrices after encoding and decoding.

Create the file and type:

```rust
use crate::{
    document,
    editor::{Action, Editor},
    prepared::PreparedMesh,
    specimen,
};
use session_rust::{Mesh, Point, Xform};

#[test]
fn source_precision_attributes_and_placement_reopen_exactly() {
    let mut editor = Editor::default();
    let ids: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|object| object.id)
        .collect();
    for id in ids {
        editor.scene.remove(id);
    }
    let x = 100000000.125;
    let mut source = Mesh::from_vertices_and_faces(
        vec![
            Point::new(x, 0.0, 0.0),
            Point::new(x + 16.0, 0.0, 0.0),
            Point::new(x, 16.0, 0.0),
        ],
        vec![vec![0, 1, 2]],
    );
    source.name = "Exact source".into();
    source.is_visible = false;
    source.is_locked = true;
    let id = editor
        .scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let model = Xform::translation(1.25, -2.5, 3.75);
    editor
        .scene
        .place(id, model.clone())
        .unwrap();
    let bytes = document::snapshot(&editor.scene).unwrap();
    let reopened = document::load(&bytes).unwrap();
    let mesh = &reopened.meshes[0];
    assert_eq!(mesh.geometry.vertex_point(0).unwrap()[0], x);
    assert_eq!(mesh.geometry.name, "Exact source");
    assert!(!mesh.geometry.is_visible && mesh.geometry.is_locked);
    assert_eq!(
        mesh.geometry.guid(),
        editor.scene.objects()[0].guid
    );
    assert_eq!(mesh.model.m, model.m);
    assert_ne!(x as f32 as f64, x);
}

#[test]
fn both_imported_copies_reopen_with_their_stored_identity() {
    let mut editor = Editor::default();
    let bytes = specimen::bytes();
    editor
        .apply(Action::Import(bytes.clone()))
        .unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let saved = document::snapshot(&editor.scene).unwrap();
    let reopened = document::load(&saved).unwrap();
    assert_eq!(
        reopened.meshes.len(),
        editor.scene.objects().len()
    );
    for object in editor.scene.objects() {
        let prepared = reopened
            .meshes
            .iter()
            .find(|mesh| mesh.geometry.guid() == object.guid)
            .unwrap();
        assert_eq!(prepared.model.m, object.model.m);
        assert_eq!(prepared.geometry.name, object.geometry.name);
    }
    assert_eq!(
        editor.scene.objects()[2].geometry.guid(),
        editor.scene.objects()[5].geometry.guid()
    );
    assert_ne!(
        editor.scene.objects()[2].guid,
        editor.scene.objects()[5].guid
    );
}
```

### 2. `src/lib.rs`

Compile exact document round-trip checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod selected_fit_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod save_roundtrip_tests;
#[cfg(test)]
mod selected_fit_tests;
```

## <a id="checkpoint-29d-save"></a>5. Download the editable document from the command line

**What this code does:** Connect Save to a browser download and reopen the actual downloaded document.

### 1. `Cargo.toml`

Enable the browser object URL and download anchor APIs. These features do not change dependency versions.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/file_output.rs`

Create a temporary file owner, request a download and schedule one-shot URL cleanup.

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let body = document.body().ok_or("No document body")?;
    let anchor: web_sys::HtmlAnchorElement =
        document.create_element("a")?.dyn_into()?;
    anchor.set_hidden(true);
    anchor.set_download("viewer.session");
    body.append_child(&anchor)?;
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(bytes));
    let blob = match web_sys::Blob::new_with_u8_array_sequence(&parts) {
        Ok(blob) => blob,
        Err(error) => {
            anchor.remove();
            return Err(error);
        }
    };
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(url) => url,
        Err(error) => {
            anchor.remove();
            return Err(error);
        }
    };
    anchor.set_href(&url);
    anchor.click();
    anchor.remove();
    let cleanup: js_sys::Function = Closure::once_into_js(move || {
        let _ = web_sys::Url::revoke_object_url(&url);
    })
    .unchecked_into();
    if let Err(error) =
        window.set_timeout_with_callback_and_timeout_and_arguments_0(&cleanup, 10_000)
    {
        let _ = cleanup.call0(&JsValue::NULL);
        return Err(error);
    }
    Ok(())
}
```

### 3. `src/lib.rs`

Expose the browser file-output adapter.

**Locate this existing block:**

```rust
mod file_input;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod file_input;
#[cfg(target_arch = "wasm32")]
mod file_output;
#[cfg(test)]
```

### 4. `src/browser.rs`

Offer Save through command completion in the actual dock. Read the scene for Save without creating a document action or consuming history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    {
                        input.click();
                    }
```

Replace it with:

```rust
                    {
                        input.click();
                    }
                    None
                } else if line == "save" {
                    match crate::document::snapshot(&editor.scene) {
                        Ok(bytes) => match crate::file_output::download(&bytes) {
                            Ok(()) => panel.result(&format!(
                                "Saved editable document ({} bytes)",
                                bytes.len()
                            )),
                            Err(error) => {
                                panel.result(&format!("Save failed: {error:?}"))
                            }
                        },
                        Err(error) => panel.result(error),
                    }
```

**Locate this block:**

```rust
            "Open",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

Replace it with:

```rust
            "Open",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

### 5. `index.html`

Let the native file picker offer both original specimens and downloaded session files.

**Locate this existing block:**

```html
  <canvas id="canvas" tabindex="0" width="640" height="480" aria-label="Viewer drawing"></canvas>
  <input id="open" type="file" accept=".pb" hidden>
</body>
```

Edit the existing block to match this code; keep unchanged lines:

```html
  <canvas id="canvas" tabindex="0" width="640" height="480" aria-label="Viewer drawing"></canvas>
  <input id="open" type="file" accept=".pb,.session" hidden>
</body>
```

## <a id="checkpoint-30-feedback"></a>6. Report the result that actually committed

**What this code does:** Keep failed reads and imports visible in command history without claiming success.

### 1. `src/browser.rs`

Keep a startup failure visible when the GPU dock could not be created; file errors stay in command history.

**Locate this existing block:**

```rust
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") {
                let _ = status.remove_attribute("hidden");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") {
                let _ = status.remove_attribute("hidden");
```

### 2. `src/file_input.rs`

Route the size refusal through the panel’s main event closure. Deliver a read error only after the existing latest-read check. Pass asynchronous errors through a small custom event rather than borrowing the panel.

**Locate this existing block:**

```rust
    if file.size() > crate::document::MAX_BYTES as f64 {
        super::browser::report("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id {
            return;
        }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            super::browser::report(&format!("Cannot read file: {error:?}"));
        }
    });
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if file.size() > crate::document::MAX_BYTES as f64 {
        failure("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id {
            return;
        }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result =
        web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
            .and_then(|event| {
                web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window"))?
                    .dispatch_event(&event)
            });
    if result.is_err() {
        super::browser::report(message);
    }
}
```

### 3. `src/browser.rs`

Receive read failures in the same closure that owns command history. A successful transaction is the only branch that may announce an imported file. Remove the unconditional success that overwrote decoding errors. Keep the error event connected for the viewer lifetime.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    window.add_event_listener_with_callback(
        "viewer-file",
        update.as_ref().unchecked_ref(),
```

Replace it with:

```rust
    window.add_event_listener_with_callback(
        "viewer-file",
        update.as_ref().unchecked_ref(),
    )?;
    window.add_event_listener_with_callback(
        "viewer-file-error",
        update.as_ref().unchecked_ref(),
```

**Locate this block:**

```rust
                ));
                return;
            }
            if event.type_() == "viewer-file" {
                report("File imported. Undo removes the entire import.");
            }
            if resize(
                &browser_window,
                &pointer_canvas,
```

Replace it with:

```rust
                ));
                return;
            }
            if resize(
                &browser_window,
                &pointer_canvas,
```

**Locate this block:**

```rust
            if let Some(action) = action {
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected)
                    }
                    Ok(Change::View) => {}
                    Err(error) => {
```

Replace it with:

```rust
            if let Some(action) = action {
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            report("File imported. Undo removes the entire import.");
                        }
                    }
                    Ok(Change::View) => {}
                    Err(error) => {
```

**Locate this block:**

```rust
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::bytes(&event).map(Action::Import)
            } else if let Some(line) = line {
```

Replace it with:

```rust
                None
            } else if event.type_() == "viewer-file-error" {
                if let Some(message) = event
                    .dyn_ref::<web_sys::CustomEvent>()
                    .and_then(|event| event.detail().as_string())
                {
                    report(&message);
                    panel.result(&message);
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::bytes(&event).map(Action::Import)
            } else if let Some(line) = line {
```

### 4. `src/file_input.rs`

Move the size refusal out of the currently borrowed input callback. Check size in the queued task before reading; error delivery can now enter the main callback safely.

**Locate this existing block:**

```rust
    request.set(id);
    if file.size() > crate::document::MAX_BYTES as f64 {
        failure("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    request.set(id);
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.get() != id {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            failure("This checkpoint accepts files up to 4 MiB");
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
```

## <a id="checkpoint-30a-tickets"></a>7. Give a pending read an explicit ticket

**What this code does:** Describe latest-read ownership, cancellation and one-shot completion in ordinary Rust.

### 1. `src/read_gate.rs`

Own one pending ticket and test stale, duplicate, cancelled and exhausted requests independently of browser APIs.

Create the file and type:

```rust
#[derive(Default)]
pub struct ReadGate {
    issued: u64,
    pending: Option<u64>,
}

impl ReadGate {
    pub fn begin(&mut self) -> Result<u64, &'static str> {
        self.pending = None;
        let ticket = self
            .issued
            .checked_add(1)
            .ok_or("Read tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(ticket);
        Ok(ticket)
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
    }

    pub fn finish(&mut self, ticket: u64) -> bool {
        if self.pending != Some(ticket) {
            return false;
        }
        self.pending = None;
        true
    }

    pub fn cancel(&mut self) {
        self.pending = None;
    }
}

#[cfg(test)]
mod tests {
    use super::ReadGate;

    #[test]
    fn stale_and_duplicate_completions_cannot_consume_current_work() {
        let mut gate = ReadGate::default();
        let first = gate.begin().unwrap();
        let second = gate.begin().unwrap();
        assert!(second > first);
        assert!(!gate.finish(first));
        assert_eq!(gate.pending(), Some(second));
        assert!(gate.finish(second));
        assert_eq!(gate.pending(), None);
        assert!(!gate.finish(second));
    }

    #[test]
    fn cancellation_revokes_the_pending_ticket() {
        let mut gate = ReadGate::default();
        let ticket = gate.begin().unwrap();
        gate.cancel();
        assert!(!gate.finish(ticket));
        assert!(gate.begin().unwrap() > ticket);
    }

    #[test]
    fn exhausted_ids_do_not_wrap_or_leave_old_work_pending() {
        let mut gate = ReadGate {
            issued: u64::MAX,
            pending: Some(u64::MAX),
        };
        assert!(gate.begin().is_err());
        assert_eq!(gate.pending(), None);
    }
}
```

### 2. `src/lib.rs`

Expose the native request model without changing the browser adapter yet.

**Locate this existing block:**

```rust
mod projection_tests;
pub mod renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod projection_tests;
pub mod read_gate;
pub mod renderer;
```

## <a id="checkpoint-30b-cancel"></a>8. Cancel reads without accepting their late result

**What this code does:** Adopt one-shot tickets in the browser and give asynchronous results their own history entry.

### 1. `src/file_input.rs`

Share the native gate through short interior mutable borrows. Receive the same pending-read owner as the command callback. Issue a bounded ticket; even an exhaustion error must be delivered after the input callback returns. Avoid work for a ticket already superseded or cancelled before its queued task starts. Consume the current ticket before reporting a size refusal, releasing the RefCell borrow before dispatch. Only a current one-shot completion may deliver bytes or a read error.

**Locate this existing block:**

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        return;
    };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.get() != id {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            failure("This checkpoint accepts files up to 4 MiB");
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id {
            return;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(
    event: &web_sys::Event,
    request: Rc<RefCell<crate::read_gate::ReadGate>>,
) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        return;
    };
    input.set_value("");
    let id = match request.borrow_mut().begin() {
        Ok(id) => id,
        Err(error) => {
            wasm_bindgen_futures::spawn_local(async move {
                failure(error);
            });
            return;
        }
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
```

### 2. `src/browser.rs`

Expose the initial row count through hidden diagnostics. Offer read cancellation as a command rather than a keyboard feature shortcut. Let the input callback and queued tasks share one native gate. An asynchronous read error gets its own history entry instead of rewriting a later command. Revoke pending delivery without touching the scene or edit history. Record asynchronous success separately from subsequent typed commands. Distinguish a file result from a synchronous error on the current command. Update hidden diagnostics after every handled event. Keep scene count and projection in one diagnostic boundary. Report rows independently of overlapping drawing pixels.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute(
        "data-projection",
        &format!("{:?}", camera.projection),
    )
```

Replace it with:

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute(
        "data-object-count",
        &editor.scene.objects().len().to_string(),
    )?;
    canvas.set_attribute(
        "data-projection",
        &format!("{:?}", editor.camera.projection),
    )
```

**Locate this block:**

```rust
    Ok(())
}

fn show_projection(camera: &crate::camera::Camera) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
```

Replace it with:

```rust
    Ok(())
}

fn inspect(editor: &Editor) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
```

**Locate this block:**

```rust
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            report("File imported. Undo removes the entire import.");
                        }
                    }
                    Ok(Change::View) => {}
                    Err(error) => {
                        report(error);
                        panel.result(error);
                    }
                }
            }
            if let Err(error) = show_projection(&editor.camera) {
                report(&format!(
                    "Cannot report projection: {error:?}"
                ));
```

Replace it with:

```rust
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            report("File imported. Undo removes the entire import.");
                            panel.answer(
                                "Open",
                                "File imported. Undo removes the entire import.",
                            );
                        }
                    }
                    Ok(Change::View) => {}
                    Err(error) => {
                        report(error);
                        if event.type_() == "viewer-file" {
                            panel.answer("Open", error);
                        } else {
                            panel.result(error);
                        }
                    }
                }
            }
            if let Err(error) = inspect(&editor) {
                report(&format!(
                    "Cannot report projection: {error:?}"
                ));
```

**Locate this block:**

```rust
                    .and_then(|event| event.detail().as_string())
                {
                    report(&message);
                    panel.result(&message);
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::bytes(&event).map(Action::Import)
            } else if let Some(line) = line {
                if line == "open" {
                    if let Some(input) = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id("open"))
```

Replace it with:

```rust
                    .and_then(|event| event.detail().as_string())
                {
                    report(&message);
                    panel.answer("Open", &message);
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::bytes(&event).map(Action::Import)
            } else if let Some(line) = line {
                if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
                    None
                } else if line == "open" {
                    if let Some(input) = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id("open"))
```

**Locate this block:**

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::Cell::new(0));
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let shortcut = event
```

Replace it with:

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let shortcut = event
```

**Locate this block:**

```rust
        &[
            "Help",
            "Open",
            "Save",
            "Example Box",
            "Example Triangle",
```

Replace it with:

```rust
        &[
            "Help",
            "Open",
            "Cancel Open",
            "Save",
            "Example Box",
            "Example Triangle",
```

**Locate this block:**

```rust
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
```

Replace it with:

```rust
    let mut editor = Editor::default();
    inspect(&editor)?;
    let mut renderer = Renderer::new(
        device,
        queue,
```

## <a id="checkpoint-30c-replace"></a>9. Replace a document as one reversible change

**What this code does:** Prepare a whole replacement before changing live rows, then commit it through existing history.

### 1. `src/scene.rs`

Replace rows while preserving the issued local-ID counter; the caller supplies the transaction.

**Locate this existing block:**

```rust
    }

    pub fn import(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn replace(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        self.objects.clear();
        self.extra = None;
        self.import(loaded)
    }

    pub fn import(
```

### 2. `src/editor.rs`

Name replacement explicitly instead of giving Import a hidden destructive mode. Validate every source first, then replace as one undoable transaction.

**Locate this existing block:**

```rust
    Import(Vec<u8>),
    AddBox,
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
    pub camera: Camera,
    pub background: Background,
    history: History,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
        }
    }
}

impl Editor {
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Import(Vec<u8>),
    Replace(Vec<u8>),
    AddBox,
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
    pub camera: Camera,
    pub background: Background,
    history: History,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
        }
    }
}

impl Editor {
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
```

### 3. `src/replace_tests.rs`

Optional authoring fixture. Prove replacement/Undo/Redo preserve owners, camera state and non-reused local IDs.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    specimen,
};
use std::rc::Rc;

#[test]
fn replacement_history_retains_camera_and_distinct_local_identity() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    let old = editor.scene.objects()[0].clone();
    editor.camera.distance = 7.0;
    editor.camera.aspect = 2.0;
    editor
        .apply(Action::Replace(specimen::bytes()))
        .unwrap();
    let ids: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|object| object.id)
        .collect();
    assert_eq!(ids.len(), 3);
    assert!(!editor.scene.contains(old.id));
    assert!(editor.selected.is_none());
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.scene.objects()[0].id, old.id);
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].geometry,
        &old.geometry
    ));
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].mesh,
        &old.mesh
    ));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor
            .scene
            .objects()
            .iter()
            .map(|object| object.id)
            .collect::<Vec<_>>(),
        ids
    );
    assert_eq!(
        (editor.camera.distance, editor.camera.aspect),
        (7.0, 2.0)
    );
}
```

### 4. `src/lib.rs`

Compile the replacement contract.

**Locate this existing block:**

```rust
#[cfg(test)]
mod save_roundtrip_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod replace_tests;
#[cfg(test)]
mod save_roundtrip_tests;
```

## <a id="checkpoint-30d-bridge"></a>10. Choose append or replace before opening the picker

**What this code does:** Carry the selected Open operation through asynchronous file reading and cancel older work immediately.

### 1. `src/file_input.rs`

Capture a typed operation value with the selected file instead of consulting mutable state after reading. A choice with no file revokes pending ownership without editing the scene. Use the ticket already issued by the Open command. Keep the captured mode beside the accepted bytes. Include the selected operation in the private byte event. Use an explicit replace flag and byte array at the browser bridge. Translate checked browser payloads into ordinary editor actions.

**Locate this existing block:**

```rust
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(
    event: &web_sys::Event,
    request: Rc<RefCell<crate::read_gate::ReadGate>>,
) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        return;
    };
    input.set_value("");
    let id = match request.borrow_mut().begin() {
        Ok(id) => id,
        Err(error) => {
            wasm_bindgen_futures::spawn_local(async move {
                failure(error);
            });
            return;
        }
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result =
        web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
            .and_then(|event| {
                web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window"))?
                    .dispatch_event(&event)
            });
    if result.is_err() {
        super::browser::report(message);
    }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window()
        .ok_or("No browser window")?
        .dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event
        .detail()
        .dyn_into::<js_sys::Uint8Array>()
        .ok()?;
    if array.length() as usize > crate::document::MAX_BYTES {
        return None;
    }
    Some(array.to_vec())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Append,
    Replace,
}

pub fn choose(
    event: &web_sys::Event,
    request: Rc<RefCell<crate::read_gate::ReadGate>>,
    mode: Mode,
) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        request.borrow_mut().cancel();
        return;
    };
    input.set_value("");
    let Some(id) = request.borrow().pending() else {
        return;
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let result = result.and_then(|buffer| deliver(buffer, mode));
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result =
        web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
            .and_then(|event| {
                web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window"))?
                    .dispatch_event(&event)
            });
    if result.is_err() {
        super::browser::report(message);
    }
}

fn deliver(buffer: JsValue, mode: Mode) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    let payload = js_sys::Array::new();
    payload.push(&JsValue::from_bool(mode == Mode::Replace));
    payload.push(&js_sys::Uint8Array::new(&buffer));
    detail.set_detail(&payload);
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window()
        .ok_or("No browser window")?
        .dispatch_event(&event)?;
    Ok(())
}

pub fn action(event: &web_sys::Event) -> Option<crate::editor::Action> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let payload = event
        .detail()
        .dyn_into::<js_sys::Array>()
        .ok()?;
    if payload.length() != 2 {
        return None;
    }
    let replace = payload.get(0).as_bool()?;
    let array = payload
        .get(1)
        .dyn_into::<js_sys::Uint8Array>()
        .ok()?;
    if array.length() as usize > crate::document::MAX_BYTES {
        return None;
    }
    Some(if replace {
        crate::editor::Action::Replace(array.to_vec())
    } else {
        crate::editor::Action::Import(array.to_vec())
    })
}
```

### 2. `src/browser.rs`

Offer replacement as a typed command. Hold the picker operation in the event closure; each task copies it at selection. Revoke native picker cancellation and capture the operation when a file is selected. Keep browser values outside the editor transaction. A newer request revokes older work at picker opening, not only after file selection. Remember which successful file action was committed before consuming it. Report the committed operation without overwriting another command. Receive the native file-input cancellation event for the viewer lifetime.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document
        .add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback(
```

Replace it with:

```rust
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document
        .add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document
        .add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback(
```

**Locate this block:**

```rust
                    report(&format!("Cannot redraw: {error:?}"));
                }
            }
        });
    for name in [
        "pointerdown",
        "pointermove",
```

Replace it with:

```rust
                    report(&format!("Cannot redraw: {error:?}"));
                }
            }
        },
    );
    for name in [
        "pointerdown",
        "pointermove",
```

**Locate this block:**

```rust
                None
            };
            if let Some(action) = action {
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            report("File imported. Undo removes the entire import.");
                            panel.answer(
                                "Open",
                                "File imported. Undo removes the entire import.",
                            );
                        }
                    }
```

Replace it with:

```rust
                None
            };
            if let Some(action) = action {
                let replacement = matches!(&action, Action::Replace(_));
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            let message = if replacement {
                                "Document replaced. Undo restores the previous document."
                            } else {
                                "File imported. Undo removes the entire import."
                            };
                            report(message);
                            panel.answer(
                                if replacement {
                                    "Open Replace"
                                } else {
                                    "Open"
                                },
                                message,
                            );
                        }
                    }
```

**Locate this block:**

```rust
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::bytes(&event).map(Action::Import)
            } else if let Some(line) = line {
                if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
                    None
                } else if line == "open" {
                    if let Some(input) = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id("open"))
```

Replace it with:

```rust
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::action(&event)
            } else if let Some(line) = line {
                if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
                    None
                } else if line == "open" || line == "open replace" {
                    let ticket = request.borrow_mut().begin();
                    if let Err(error) = ticket {
                        report(error);
                        panel.result(error);
                        return;
                    }
                    read_mode.set(if line == "open replace" {
                        crate::file_input::Mode::Replace
                    } else {
                        crate::file_input::Mode::Append
                    });
                    if let Some(input) = web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| document.get_element_by_id("open"))
```

**Locate this block:**

```rust
                }
            };

            let action = if event.type_() == "change" {
                crate::file_input::choose(&event, std::rc::Rc::clone(&request));
                None
            } else if event.type_() == "viewer-file-error" {
                if let Some(message) = event
```

Replace it with:

```rust
                }
            };

            let action = if event.type_() == "cancel"
                && event
                    .target()
                    .and_then(|target| {
                        target
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .ok()
                    })
                    .is_some_and(|input| input.id() == "open")
            {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.answer("Open", "Open cancelled.");
                None
            } else if event.type_() == "change" {
                crate::file_input::choose(
                    &event,
                    std::rc::Rc::clone(&request),
                    read_mode.get(),
                );
                None
            } else if event.type_() == "viewer-file-error" {
                if let Some(message) = event
```

**Locate this block:**

```rust
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
    let update =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

Replace it with:

```rust
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
    ));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

**Locate this block:**

```rust
            "Open",
            "Cancel Open",
            "Save",
            "Example Box",
```

Replace it with:

```rust
            "Open",
            "Open Replace",
            "Cancel Open",
            "Save",
            "Example Box",
```

## <a id="checkpoint-30e-recovery"></a>11. Prove recovery at the transaction and display boundaries

**What this code does:** Test partial replacement failure, preserved selection/Redo and placement upload limits.

### 1. `src/placement.rs`

Reject a finite double coefficient that cannot enter the current float uniform.

**Locate this existing block:**

```rust
    matrix.len() == 16
        && matrix.iter().all(|value| value.is_finite())
        && [matrix[3], matrix[7], matrix[11], matrix[15]] == [0.0, 0.0, 0.0, 1.0]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    matrix.len() == 16
        && matrix
            .iter()
            .all(|value| value.is_finite() && (*value as f32).is_finite())
        && [matrix[3], matrix[7], matrix[11], matrix[15]] == [0.0, 0.0, 0.0, 1.0]
```

### 2. `src/scene.rs`

State the full placement boundary in its error. Fail during the second insertion, then prove rollback restores the old allocations without recycling issued IDs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    use super::*;

    #[test]
```

Replace it with:

```rust
    use super::*;

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(
            history
                .try_edit(&mut scene, |scene| scene.replace(loaded))
                .is_err()
        );
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(&object.geometry, &old.geometry));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
```

**Locate this block:**

```rust
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite and affine");
        }
        let object = self
            .objects
```

Replace it with:

```rust
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self
            .objects
```

### 3. `src/replace_tests.rs`

Optional authoring fixture. Test refusal before mutation, selection/Redo retention and the double-to-float upload boundary.

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
fn refused_replacement_preserves_selection_source_owners_and_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    let old = editor.scene.objects()[0].clone();
    assert!(
        editor
            .apply(Action::Replace(vec![255]))
            .is_err()
    );
    assert_eq!(editor.selected, selected);
    assert_eq!(editor.scene.objects().len(), 2);
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].geometry,
        &old.geometry
    ));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
}

#[test]
fn a_finite_double_matrix_can_exceed_the_current_float_upload() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    let before = editor.scene.objects()[0].model.m;
    let mut model = session_rust::Xform::identity();
    model.m[0] = f64::MAX;
    assert!(model.m.iter().all(|value| value.is_finite()));
    assert!(!(model.m[0] as f32).is_finite());
    assert!(
        editor
            .scene
            .place(id, model.clone())
            .is_err()
    );
    assert_eq!(editor.scene.objects()[0].model.m, before);
    use prost::Message;
    let bytes = crate::document::snapshot(&editor.scene).unwrap();
    let mut message = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    message
        .xforms
        .push(session_rust::proto::XformEntry {
            guid: editor.scene.objects()[0].guid.clone(),
            xform: Some(model.to_proto()),
        });
    assert!(crate::document::load(&message.encode_to_vec()).is_err());
}
```

## Build & Run

![Recorded visual output for Lesson 3.5 (30e-recovery)](../../screenshots/journey/30e-recovery-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 3.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 3.5
npm --prefix ../session_tests run course -- save 3.5
```

Continue with [3.6 · Shared GPU geometry and releasable source ownership](31-settings.md).
