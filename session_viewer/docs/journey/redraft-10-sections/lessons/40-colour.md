# 5.6 · Original mesh colours and large index buffers

**Goal:** Preserve kernel u32 indices through display and GPU drawing, with truthful payload and allocation accounting. · **Steps:** 3 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">6 Duplicate Vertices (2 Triangles)</text>
  <polygon points="74,60 174,60 74,142" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <polygon points="196,68 196,150 96,150" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <circle cx="74" cy="60" r="4" fill="#111111"/>
  <circle cx="174" cy="60" r="4" fill="#111111"/>
  <circle cx="74" cy="142" r="4" fill="#111111"/>
  <circle cx="196" cy="68" r="4" fill="#111111"/>
  <circle cx="196" cy="150" r="4" fill="#111111"/>
  <circle cx="96" cy="150" r="4" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="268" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">6 verts</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="268" y="114" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#dc2626">2 shared</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4 Unique Vertices + Index [0,1,2, 0,2,3]</text>
  <polygon points="560,48 648,104 560,160 472,104" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="560" y1="48" x2="560" y2="160" stroke="#111111" stroke-width="1.75" stroke-dasharray="4 3"/>
  <circle cx="560" cy="48" r="4.5" fill="#dc2626"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="580" y="52" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0</text>
  <circle cx="648" cy="104" r="4.5" fill="#16a34a"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="664" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1</text>
  <circle cx="560" cy="160" r="4.5" fill="#2563eb"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="580" y="164" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2</text>
  <circle cx="472" cy="104" r="4.5" fill="#d97706"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="452" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v3</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Output (draw_indexed 0..6)</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,54 1004,103 934,152 864,103" fill="#ed5c87" stroke="#111111" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-40-colour"></a>1. Carry the mesh colour mode into display data

Preserve original object, point and face colour modes through the kernel render adapter, including shared face boundaries.

### 1. `src/colour.rs`

Build original colour-mode specimens while retaining shared kernel topology.

Create the file and type:

```rust
use session_rust::{Color, Mesh, Point};

#[derive(Clone, Copy)]
pub enum Mode {
    Object,
    Points,
    Faces,
}

pub fn specimen(mode: Mode) -> Mesh {
    let points = [-0.7, 0.0, 0.7]
        .into_iter()
        .map(|x| Point::new(x, -0.5, 0.0))
        .chain(
            [-0.7, 0.0, 0.7]
                .into_iter()
                .map(|x| Point::new(x, 0.5, 0.0)),
        )
        .collect();
    let mut mesh = Mesh::from_vertices_and_faces(
        points,
        vec![vec![0, 1, 4, 3], vec![1, 2, 5, 4]],
    );
    mesh.set_objectcolor(Color::new(0.8, 0.6, 0.1, 1.0));
    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);
    match mode {
        Mode::Object => {}
        Mode::Points => mesh.set_pointcolors(vec![
            red.clone(),
            green.clone(),
            blue.clone(),
            red,
            green,
            blue,
        ]),
        Mode::Faces => mesh.set_facecolors(vec![red, blue]),
    }
    mesh
}
```

### 2. `src/lib.rs`

Register original colour specimens and extraction checks.

**Locate this existing block:**

```rust
mod close_report_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod close_report_tests;
pub mod colour;
#[cfg(test)]
mod colour_tests;
#[cfg(test)]
```

### 3. `src/colour_tests.rs`

Copy active mode, exact topology/arrays, shared interpolation, face duplication and fallback checks.

Create the file and type:

```rust
use crate::{
    colour::{self, Mode},
    mesh::Mesh,
};
use session_rust::{Color, mesh::ColorMode};

#[test]
fn source_modes_preserve_topology_and_resolve_complete_colour_arrays() {
    let faces = colour::specimen(Mode::Faces);
    let original = faces.to_proto();
    let display = Mesh::from_kernel(&faces).unwrap();
    assert_eq!(faces.number_of_vertices(), 6);
    assert_eq!(display.vertices().len(), 12);
    assert_eq!(display.indices().len(), 12);
    assert!(
        display.vertices()[..6]
            .iter()
            .all(|p| p[3..] == [1.0, 0.0, 0.0])
    );
    assert!(
        display.vertices()[6..]
            .iter()
            .all(|p| p[3..] == [0.0, 0.0, 1.0])
    );
    assert_eq!(faces.to_proto(), original);
    let points = colour::specimen(Mode::Points);
    let display = Mesh::from_kernel(&points).unwrap();
    assert_eq!(display.vertices().len(), 6);
    assert_eq!(display.vertices()[1][3..], [0.0, 1.0, 0.0]);
    assert_eq!(points.get_pointcolors().len(), 6);
    assert_eq!(points.number_of_faces(), 2);
    let mut object = colour::specimen(Mode::Points);
    object.color_mode = ColorMode::OBJECTCOLOR;
    let display = Mesh::from_kernel(&object).unwrap();
    assert!(
        display
            .vertices()
            .iter()
            .all(|p| p[3..] == [0.8, 0.6, 0.1])
    );
    assert_eq!(object.get_pointcolors().len(), 6);
}

#[test]
fn incomplete_active_arrays_use_object_colour_without_source_mutation() {
    for points in [true, false] {
        let mut source = colour::specimen(Mode::Object);
        if points {
            source.set_pointcolors(vec![Color::new(1.0, 0.0, 0.0, 1.0)]);
        } else {
            source.set_facecolors(vec![Color::new(1.0, 0.0, 0.0, 1.0)]);
        }
        let display = Mesh::from_kernel(&source).unwrap();
        assert!(
            display
                .vertices()
                .iter()
                .all(|p| p[3..] == [0.8, 0.6, 0.1])
        );
        assert_eq!(
            if points {
                source.get_pointcolors().len()
            } else {
                source.get_facecolors().len()
            },
            1
        );
    }
}
```

## <a id="checkpoint-40a-input"></a>2. Compare colour modes through actual input

Insert original object/point/face colour specimens through real input and preserve their modes through Save/reopen.

### 1. `src/editor.rs`

Expose original mesh colour specimens as editor actions. Insert one original colour-mode mesh in a single history transaction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddCurve => {
```

Replace it with:

```rust
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddColours(mode) => {
                let prepared =
                    crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddCurve => {
```

**Locate this block:**

```rust
    AddCurve,
    Controls(bool),
    AddPolyline,
    AddArrow,
```

Replace it with:

```rust
    AddCurve,
    AddColours(crate::colour::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
```

### 2. `src/browser.rs`

Expose original colour comparisons in dock completion and Help. Dispatch actual colour input through the existing editor.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example curve" => Action::AddCurve,
                        "controls on" => Action::Controls(true),
```

Replace it with:

```rust
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example curve" => Action::AddCurve,
                        "example object color" => {
                            Action::AddColours(crate::colour::Mode::Object)
                        }
                        "example point colors" => {
                            Action::AddColours(crate::colour::Mode::Points)
                        }
                        "example face colors" => {
                            Action::AddColours(crate::colour::Mode::Faces)
                        }
                        "controls on" => Action::Controls(true),
```

**Locate this block:**

```rust
            "Example Curve",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

Replace it with:

```rust
            "Example Curve",
            "Example Object Color",
            "Example Point Colors",
            "Example Face Colors",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

### 3. `src/lib.rs`

Register original colour history and editable roundtrip checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod colour_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod colour_input_tests;
#[cfg(test)]
mod colour_tests;
```

### 4. `src/colour_input_tests.rs`

Copy original mode/array roundtrip, stable row identity, one-step Undo/Redo and Close checks.

Create the file and type:

```rust
use crate::{
    colour::Mode,
    document,
    editor::{Action, Editor},
};
use session_rust::mesh::ColorMode;

#[test]
fn colour_commands_keep_original_modes_arrays_and_one_step_history() {
    for (mode, expected) in [
        (Mode::Object, ColorMode::OBJECTCOLOR),
        (Mode::Points, ColorMode::POINTCOLORS),
        (Mode::Faces, ColorMode::FACECOLORS),
    ] {
        let mut editor = Editor::default();
        editor.apply(Action::Close).unwrap();
        editor
            .apply(Action::AddColours(mode))
            .unwrap();
        let row = &editor.scene.objects()[0];
        let id = row.id;
        let source = row.geometry().unwrap();
        let weak = std::rc::Rc::downgrade(source);
        assert!(source.color_mode == expected);
        assert_eq!(source.number_of_vertices(), 6);
        let original = source.to_proto();
        let bytes = document::snapshot(&editor.scene).unwrap();
        let reopened = document::load(&bytes).unwrap();
        assert_eq!(
            reopened.meshes[0].geometry.to_proto(),
            original
        );
        drop(reopened);
        editor.apply(Action::Undo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert!(weak.upgrade().is_some());
        editor.apply(Action::Redo).unwrap();
        assert_eq!(editor.scene.objects()[0].id, id);
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
    }
}

#[test]
fn packed_colour_validation_refuses_partial_nonfinite_and_unknown_records() {
    use prost::Message;
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
        .apply(Action::AddColours(Mode::Faces))
        .unwrap();
    let valid = document::snapshot(&editor.scene).unwrap();
    for case in 0..3 {
        let mut message =
            session_rust::proto::Session::decode(valid.as_slice()).unwrap();
        let mesh = &mut message.objects.as_mut().unwrap().meshes[0];
        match case {
            0 => mesh.pointcolors_rgba = vec![1.0, 0.0, 0.0],
            1 => mesh.facecolors_rgba[0] = f32::NAN,
            _ => mesh.color_mode = 4,
        }
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
}
```

### 5. `src/document.rs`

Accept known source colour modes and preserve only complete finite packed colour records.

**Locate this existing block:**

```rust
    }
    if mesh.color_mode != 0 {
        return Err("Per-face and per-vertex colours arrive later");
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    if !(0..=3).contains(&mesh.color_mode) {
        return Err("Unknown mesh colour mode");
    }
    for colours in [
        &mesh.pointcolors_rgba,
        &mesh.facecolors_rgba,
        &mesh.linecolors_rgba,
    ] {
        if colours.len() % 4 != 0
            || colours.len() > 160_000
            || colours.iter().any(|v| !v.is_finite())
        {
            return Err("Mesh colours need bounded complete finite RGBA records");
        }
    }
    if mesh.objectcolor.as_ref().is_some_and(|c| {
        [c.r, c.g, c.b, c.a]
            .iter()
            .any(|v| !v.is_finite())
    }) {
        return Err("Mesh object colour must be finite");
    }
```

### 6. `src/document_tests.rs`

Copy unknown-mode refusal while accepting the newly implemented colour modes.

**Locate this existing block:**

```rust
            3 => {
                meshes[0].color_mode = 1;
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            3 => {
                meshes[0].color_mode = 4;
            }
```

### 7. `src/document_phase_tests.rs`

Copy the invalid-mode phase fixture with an actually unsupported mode.

**Locate this existing block:**

```rust
        session_rust::proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    message.objects.as_mut().unwrap().meshes[0].color_mode = 1;
    let (result, phases) = observed(&message.encode_to_vec());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        session_rust::proto::Session::decode(specimen::bytes().as_slice()).unwrap();
    message.objects.as_mut().unwrap().meshes[0].color_mode = 4;
    let (result, phases) = observed(&message.encode_to_vec());
```

## <a id="checkpoint-40b-indices"></a>3. Draw meshes beyond the sixteen-bit index limit

### 1. `src/mesh.rs`

Keep kernel index precision in retained display topology. Accept complete thirty-two-bit triangle references. Retain the authoritative kernel indices without narrowing them. Count actual retained index capacity. Expose the actual display index type to drawing and picking.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                [x, y, z, r, g, b]
            })
            .collect();
        let indices: Result<Vec<u16>, _> = render
            .indices
            .into_iter()
            .map(u16::try_from)
            .collect();
        Self::new(
            vertices,
            indices.map_err(|_| "Mesh exceeds this lesson's 16-bit index capacity")?,
        )
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u16>()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
```

Replace it with:

```rust
                [x, y, z, r, g, b]
            })
            .collect();
        Self::new(vertices, render.indices)
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
```

**Locate this block:**

```rust
    vertices: Vec<[f32; 6]>,
    indices: Vec<u16>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u16>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
```

Replace it with:

```rust
    vertices: Vec<[f32; 6]>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
```

### 2. `src/gpu_geometry.rs`

Interpret uploaded index bytes with their exact GPU format.

**Locate this existing block:**

```rust
            self.indices.slice(..),
            wgpu::IndexFormat::Uint16,
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            self.indices.slice(..),
            wgpu::IndexFormat::Uint32,
        );
```

### 3. `src/geometry_cache.rs`

Count initialized index payload bytes independently of driver allocation.

**Locate this existing block:**

```rust
        self.uploaded_bytes +=
            (source.vertices().len() * 24 + source.indices().len() * 2) as u64;
        geometry
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.uploaded_bytes +=
            (source.vertices().len() * 24 + source.indices().len() * 4) as u64;
        geometry
```

### 4. `src/memory.rs`

Copy the updated spare-capacity oracle for the real index layout.

**Locate this existing block:**

```rust
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(retained[4], baseline[4] + 16 * 24 + 16 * 2);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(retained[4], baseline[4] + 16 * 24 + 16 * 4);
    }
```

### 5. `src/lib.rs`

Register actual original high-index checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod line_scene_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod index_width_tests;
#[cfg(test)]
mod line_scene_tests;
```

### 6. `src/index_width_tests.rs`

Copy original high-index topology, correct displayed corners and missing-vertex refusal checks.

Create the file and type:

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use session_rust::Point;

#[test]
fn original_high_vertex_references_survive_checked_display_preparation() {
    let mut points = vec![Point::new(0.0, 0.0, 0.0); 65_537];
    points[0] = Point::new(-0.5, -0.5, 0.0);
    points[65_535] = Point::new(0.5, -0.5, 0.0);
    points[65_536] = Point::new(0.0, 0.5, 0.0);
    let source = session_rust::Mesh::from_vertices_and_faces(
        points,
        vec![vec![0, 65_535, 65_536]],
    );
    let prepared = PreparedMesh::new(source).unwrap();
    assert_eq!(
        prepared.geometry.number_of_vertices(),
        65_537
    );
    assert_eq!(
        prepared.display.indices(),
        &[0, 65_535, 65_536]
    );
    assert_eq!(prepared.display.vertices().len(), 65_537);
    assert_eq!(
        prepared.display.vertices()[65_536][..3],
        [0.0, 0.5, 0.0]
    );
    assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, u32::MAX]).is_err());
}
```

## Build & Run

![Recorded visual output for Lesson 5.6 (40b-indices)](../../../screenshots/journey/40b-indices-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.6
npm --prefix ../session_tests run course -- save 5.6
```

Continue with [5.7 · Normals, placement and a depth-correct construction grid](../../consolidation/41-normals.md).
