# 6.3 · Pick original faces, edges and controls

**Goal:** Select and inspect original faces, edges and controls. · **Steps:** 14 · **Source:** [engine/gpu/pick.rs](../../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../../src/app/walk/mesh_topology.rs)

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

## <a id="checkpoint-44-topology"></a>1. Preserve source faces and edges

### 1. `src/mesh_parts.rs`

Retain source keys, triangle ownership and original edge endpoints.

Create the file and type:

```rust
#[derive(Default)]
pub struct Parts {
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(
        source: &session_rust::Mesh,
        count: usize,
    ) -> Result<Self, &'static str> {
        let faces = source.faces();
        let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source
                .get_triangulation()
                .get(&face)
                .filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached {
                cached.clone()
            } else {
                let vertices = source
                    .face_vertices(face)
                    .ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1))
                    .map(|i| [vertices[0], vertices[i], vertices[i + 1]])
                    .collect()
            };
            for tri in indices {
                if tri
                    .iter()
                    .all(|&key| source.vertex_point(key).is_some())
                {
                    triangles.push(slot);
                }
            }
        }
        if triangles.len() != count {
            return Err("Source faces do not match the rendered triangles");
        }
        let edges = source.edges();
        let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source
                .vertex_point(a)
                .ok_or("Missing edge endpoint")?;
            let b = source
                .vertex_point(b)
                .ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self {
            faces,
            triangles,
            edges,
            endpoints,
        })
    }

    pub fn bytes(&self) -> usize {
        self.faces.capacity() * std::mem::size_of::<usize>()
            + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>()
            + self.endpoints.capacity() * 48
    }
}
```

### 2. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod pick_view_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod pick_view_tests;

pub mod mesh_parts;
```

### 3. `src/mesh.rs`

Attach original topology to prepared display meshes and count its capacity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
    }
```

Replace it with:

```rust
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.parts.bytes()
    }
```

**Locate this block:**

```rust
            .iter()
            .map(|vertex| vertex.normal)
            .collect();
        Self::new(vertices, render.indices)?.with_normals(normals)
    }

    pub fn with_normals(
```

Replace it with:

```rust
            .iter()
            .map(|vertex| vertex.normal)
            .collect();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::new(vertices, render.indices)?.with_normals(normals)?;
        display.parts = parts;
        Ok(display)
    }

    pub fn with_normals(
```

**Locate this block:**

```rust
            vertices,
            normals,
            indices,
        })
    }
```

Replace it with:

```rust
            vertices,
            normals,
            indices,
            parts: Default::default(),
        })
    }
```

**Locate this block:**

```rust
    indices: Vec<u32>,
}

impl Mesh {
```

Replace it with:

```rust
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
```

### 4. `src/mesh_parts_tests.rs`

Copy original face and diagonal checks.

Create the file and type:

```rust
use crate::{mesh::Mesh, mesh_parts::Parts};
use session_rust::Point;

#[test]
fn source_polygon_keys_survive_cached_triangulation_and_face_colors() {
    let mut source = session_rust::Mesh::from_vertices_and_faces(
        vec![
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(1., 1., 0.),
            Point::new(0., 1., 0.),
        ],
        vec![vec![0, 1, 2, 3]],
    );
    source.set_face_triangulation(0, vec![[0, 1, 3], [1, 2, 3]]);
    for colors in [false, true] {
        if colors {
            source.set_facecolors(vec![session_rust::Color::new(
                1., 0., 0., 1.,
            )]);
        }
        let display = Mesh::from_kernel(&source).unwrap();
        let parts = Parts::new(&source, display.indices().len() / 3).unwrap();
        assert_eq!(parts.faces, vec![0]);
        assert_eq!(parts.triangles, vec![0, 0]);
        assert_eq!(parts.edges.len(), 4);
        assert!(
            !parts.edges.contains(&(1, 3)),
            "The rendering diagonal is not a source edge"
        );
        assert_eq!(
            parts.endpoints[0],
            [[0., 0., 0.], [1., 0., 0.]]
        );
    }
    assert!(Parts::new(&source, 3).is_err());
}
```

### 5. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
mod pick_view_tests;

pub mod mesh_parts;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod pick_view_tests;

pub mod mesh_parts;

#[cfg(test)]
mod mesh_parts_tests;
```

## <a id="checkpoint-44a-tags"></a>2. Tag the kind of a picked part

### 1. `src/sub_id.rs`

Encode source part kinds without zero or tag collisions.

Create the file and type:

```rust
pub const FACE: u32 = 0x2000_0000;
pub const CONTROL: u32 = 0x4000_0000;
pub const EDGE: u32 = 0x8000_0000;
const MASK: u32 = 0xe000_0000;
pub const LIMIT: u32 = 0x1fff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Face,
    Control,
    Edge,
}

pub fn encode(kind: Kind, address: usize) -> Result<u32, &'static str> {
    let address = u32::try_from(address).map_err(|_| "Pick address overflows")?;
    if address > LIMIT {
        return Err("Too many sub-object addresses");
    }
    let tag = match kind {
        Kind::Face => FACE,
        Kind::Control => CONTROL,
        Kind::Edge => EDGE,
    };
    Ok((tag | address) + 1)
}

pub fn decode(pixel: u32) -> Option<(Kind, usize)> {
    let value = pixel.checked_sub(1)?;
    let kind = match value & MASK {
        FACE => Kind::Face,
        CONTROL => Kind::Control,
        EDGE => Kind::Edge,
        _ => return None,
    };
    Some((kind, (value & !MASK) as usize))
}

pub fn surface(pixel: u32) -> bool {
    pixel == 0 || matches!(decode(pixel), Some((Kind::Face, _)))
}
```

### 2. `src/pick_sources.rs`

Define typed targets, modes and bounded source tables.

Create the file and type:

```rust
use crate::{
    scene::{ObjectId, Scene},
    sub_id::{self, Kind},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Object,
    Face(usize),
    MeshEdge(usize, usize),
    PathEdge(usize),
    Curve,
    Control(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target {
    pub owner: ObjectId,
    pub part: Part,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode {
    #[default]
    Object,
    Face,
    Edge,
    Control,
}

#[derive(Default)]
pub struct Sources {
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(
    rows: &mut Vec<Target>,
    kind: Kind,
    target: Target,
) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?;
    rows.push(target);
    Ok([target.owner.value(), tag])
}
```

### 3. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod mesh_parts_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod mesh_parts_tests;

pub mod pick_sources;
pub mod sub_id;
```

### 4. `src/mesh_parts_tests.rs`

Copy tag boundaries and collision checks.

**Locate this existing block:**

```rust
    assert!(Parts::new(&source, 3).is_err());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(Parts::new(&source, 3).is_err());
}

#[test]
fn tagged_zero_addresses_and_largest_addresses_roundtrip_without_kind_collisions() {
    for kind in [
        crate::sub_id::Kind::Face,
        crate::sub_id::Kind::Control,
        crate::sub_id::Kind::Edge,
    ] {
        for address in [0, crate::sub_id::LIMIT as usize] {
            let encoded = crate::sub_id::encode(kind, address).unwrap();
            assert_ne!(encoded, 0);
            assert_eq!(
                crate::sub_id::decode(encoded),
                Some((kind, address))
            );
            assert_eq!(
                crate::sub_id::surface(encoded),
                kind == crate::sub_id::Kind::Face
            );
        }
        assert!(
            crate::sub_id::encode(kind, crate::sub_id::LIMIT as usize + 1).is_err()
        );
    }
    assert!(crate::sub_id::decode(0).is_none());
    assert!(crate::sub_id::decode(1).is_none());
}
```

## <a id="checkpoint-44b-sources"></a>3. Map visible ink back to sources

Map visible ink back to original sources.

### 1. `src/pick_sources.rs`

Build ordered face, edge, control and GPU owner streams.

**Locate this existing block:**

```rust
    Ok([target.owner.value(), tag])
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene: &Scene) -> Result<Self, &'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| {
            [
                scene
                    .pick_id(owner.value())
                    .map_or(0, ObjectId::value),
                tag,
            ]
        };
        for row in scene
            .objects()
            .iter()
            .filter(|row| row.visible())
        {
            sources.mesh_bases.push((
                row.id,
                u32::try_from(sources.faces.len())
                    .map_err(|_| "Too many source faces")?,
            ));
            for &key in &row.mesh.parts.faces {
                append(
                    &mut sources.faces,
                    Kind::Face,
                    Target {
                        owner: row.id,
                        part: Part::Face(key),
                    },
                )?;
            }
            for (&(a, b), ends) in row
                .mesh
                .parts
                .edges
                .iter()
                .zip(&row.mesh.parts.endpoints)
            {
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part: Part::MeshEdge(a, b),
                    },
                )?;
                let ends = ends.map(|p| {
                    row.model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
                        .to_f32()
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
                    end: ends[1],
                    colour: [0., 0., 0., 1.],
                    width: 1.,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene
            .lines()
            .iter()
            .filter(|row| row.visible())
        {
            let ids = append(
                &mut sources.edges,
                Kind::Edge,
                Target {
                    owner: row.id,
                    part: Part::Curve,
                },
            )?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene
            .paths()
            .iter()
            .filter(|row| row.visible())
        {
            let points: Vec<_> = row
                .points()
                .iter()
                .map(session_rust::Point::to_f32)
                .collect();
            for (index, _) in points
                .windows(2)
                .enumerate()
                .filter(|(_, pair)| pair[0] != pair[1])
            {
                let part = if matches!(
                    row.prepared.source,
                    crate::chain::Source::Polyline(_)
                ) {
                    Part::PathEdge(index)
                } else {
                    Part::Curve
                };
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part,
                    },
                )?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene
            .points()
            .iter()
            .filter(|row| row.visible())
        {
            sources.marker_ids.push(id(row.id, 1));
        }
        for row in scene.control_markers() {
            let ids = append(
                &mut sources.controls,
                Kind::Control,
                Target {
                    owner: row.parent,
                    part: Part::Control(row.control.index),
                },
            )?;
            sources
                .marker_ids
                .push(id(row.parent, ids[1]));
        }
        Ok(sources)
    }
}
```

## <a id="checkpoint-44c-resolve"></a>4. Validate a part against the current source

### 1. `src/selection.rs`

Check original part ownership and visibility.

Create the file and type:

```rust
use crate::{
    pick_sources::{Part, Target},
    scene::Scene,
};

impl Target {
    pub fn valid(self, scene: &Scene) -> bool {
        if scene.pick_id(self.owner.value()).is_none() {
            return false;
        }
        match self.part {
            Part::Object => true,
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }
}
```

### 2. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
pub mod pick_sources;
pub mod sub_id;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod pick_sources;
pub mod sub_id;

pub mod selection;
```

### 3. `src/pick_sources.rs`

Resolve typed addresses and count source table capacities.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity())
            * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity()
                + self.line_ids.capacity()
                + self.path_ids.capacity()
                + self.marker_ids.capacity())
                * 8
    }

    pub fn resolve(
        &self,
        scene: &Scene,
        pixel: [u32; 2],
        mode: Mode,
    ) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object {
            return Some(Target {
                owner,
                part: Part::Object,
            });
        }
        let (kind, address) = sub_id::decode(pixel[1])?;
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address),
            _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

### 4. `src/pick_sources_tests.rs`

Copy current-owner, wrong-kind and original segment checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Sources},
    sub_id::{self, Kind},
};

#[test]
fn source_addresses_require_the_correct_kind_owner_and_current_row() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::AddCurve).unwrap();
    editor.apply(Action::Controls(true)).unwrap();
    let sources = Sources::new(&editor.scene).unwrap();
    assert_eq!(sources.faces.len(), 6);
    assert_eq!(sources.mesh_ink.len(), 12);
    let face = sources.faces[0];
    let pixel = [
        face.owner.value(),
        sub_id::encode(Kind::Face, 0).unwrap(),
    ];
    assert_eq!(
        sources.resolve(&editor.scene, pixel, Mode::Face),
        Some(face)
    );
    assert_eq!(face.part, Part::Face(0));
    assert!(
        sources
            .resolve(&editor.scene, pixel, Mode::Edge)
            .is_none()
    );
    assert_eq!(
        sources
            .resolve(&editor.scene, pixel, Mode::Object)
            .unwrap()
            .part,
        Part::Object
    );
    let control = sources.controls[1];
    let pixel = [
        control.owner.value(),
        sub_id::encode(Kind::Control, 1).unwrap(),
    ];
    assert_eq!(
        sources.resolve(&editor.scene, pixel, Mode::Control),
        Some(control)
    );
    assert_eq!(control.part, Part::Control(1));
    assert!(
        sources
            .resolve(
                &editor.scene,
                [face.owner.value(), pixel[1]],
                Mode::Control
            )
            .is_none()
    );
    assert_eq!(
        sources.path_ids.len(),
        editor.scene.paths()[0].segments().len()
    );
    editor.apply(Action::Close).unwrap();
    assert!(
        sources
            .resolve(&editor.scene, pixel, Mode::Control)
            .is_none()
    );
}

#[test]
fn degenerate_polyline_segments_preserve_the_original_nonzero_segment_index() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    let line = session_rust::Polyline::new(vec![
        session_rust::Point::new(0., 0., 0.),
        session_rust::Point::new(0., 0., 0.),
        session_rust::Point::new(1., 0., 0.),
    ]);
    let id = editor
        .scene
        .insert_path(
            crate::chain::PreparedChain::polyline(std::rc::Rc::new(line)).unwrap(),
        )
        .unwrap();
    let sources = Sources::new(&editor.scene).unwrap();
    assert_eq!(sources.path_ids.len(), 1);
    let target = sources
        .resolve(
            &editor.scene,
            sources.path_ids[0],
            Mode::Edge,
        )
        .unwrap();
    assert_eq!(target.owner, id);
    assert_eq!(target.part, Part::PathEdge(1));
}
```

### 5. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
pub mod sub_id;

pub mod selection;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod sub_id;

pub mod selection;

#[cfg(test)]
mod pick_sources_tests;
```

## <a id="checkpoint-44d-pulling"></a>5. Reuse visible geometry for face identities

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Camera / View Uniform (16-Byte Aligned)</text>
  <rect x="52" y="48" width="268" height="106" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="66" y="58" width="240" height="38" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="81" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">view_proj: [[f32; 4]; 4] (64B)</text>
  <rect x="66" y="104" width="240" height="36" fill="#fef08a" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye_pos / viewport_px (16B)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_buffer -&gt; wgpu::BindGroup (@group(0))</text>
  <rect x="424" y="50" width="272" height="102" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="442" y="64" width="236" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">BufferUsages::UNIFORM | COPY_DST</text>
  <rect x="442" y="106" width="236" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">pass.set_bind_group(0, &amp;bg, &amp;[])</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Shader Uniform Read: var&lt;uniform&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@group(0) @binding(0)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;uniform&gt; u: Uniforms;</text>
  <rect x="828" y="114" width="212" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">u.view_proj * vec4(p, 1.0)</text>
</svg>
</div>

### 1. `src/pick_crop.wgsl`

Define the shared mode crop and original-face shader.

Create the file and type:

```wgsl
struct Crop { transform: mat4x4<f32>, mode: u32, _a: u32, _b: u32, _c: u32 }
fn wanted(pixel: u32, mode: u32) -> bool {
    if (mode == 0u) { return true; }
    if (pixel == 0u) { return false; }
    let kind = (pixel - 1u) & 0xe0000000u;
    return (mode == 2u && kind == 0x80000000u) || (mode == 3u && kind == 0x40000000u);
}
```

### 2. `src/face_pick.wgsl`

Define the shared mode crop and original-face shader.

Create the file and type:

```wgsl
struct PackedVertex { x: f32, y: f32, z: f32, r: f32, g: f32, b: f32 }
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(vec3<f32>(vertex.x, vertex.y, vertex.z), vec3<f32>(vertex.r, vertex.g, vertex.b), vec3<f32>(0.0));
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    return vec2<u32>(object.selection.y, (0x20000000u | input.face) + 1u);
}
```

### 3. `src/gpu_geometry.rs`

Expose existing geometry to source-face drawing without another position buffer.

**Locate this existing block:**

```rust
    pub source: Rc<Mesh>,
    vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh normals"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        let bytes: Vec<u8> = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh normals"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
        });
```

### 4. `src/gpu_mesh.rs`

Expose existing geometry to source-face drawing without another position buffer.

**Locate this existing block:**

```rust
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

### 5. `src/triangle.wgsl`

Expose existing geometry to source-face drawing without another position buffer.

**Locate this existing block:**

```wgsl
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), 1.0); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, 1.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), 1.0); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, 1.0);
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput { return geometry(position, colour, normal); }
```

## <a id="checkpoint-44e-pipeline"></a>6. Create the integer source-face pipeline

### 1. `src/face_pick.rs`

Create the compatible source-face layout and pipeline.

Create the file and type:

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row {
    group: wgpu::BindGroup,
    bytes: u64,
}

pub struct Faces {
    pipeline: wgpu::RenderPipeline,
    rows: Vec<Option<Row>>,
}

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 {
        entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: if binding == 3 {
                    wgpu::BufferBindingType::Uniform
                } else {
                    wgpu::BufferBindingType::Storage { read_only: true }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
    }
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("source face metadata"),
        entries: &entries,
    })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let storage = layout(device);
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 144);
        let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("source face IDs"),
            bind_group_layouts: &[
                Some(&view),
                Some(&model),
                Some(&storage),
                Some(&crop),
            ],
            immediate_size: 0,
        });
        let source = concat!(
            include_str!("triangle.wgsl"),
            "\n",
            include_str!("pick_crop.wgsl"),
            "\n",
            include_str!("face_pick.wgsl")
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("source faces"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("source face IDs"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("face_vertex"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("face_fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            rows: Vec::new(),
        }
    }
}
```

### 2. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod pick_sources_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod pick_sources_tests;

pub mod face_pick;
```

## <a id="checkpoint-44f-metadata"></a>7. Upload only original-face metadata

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU RAM: #[repr(C)] Structs -&gt; bytemuck::cast_slice</text>
  <rect x="46" y="54" width="280" height="44" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="52" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="84" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0: 24B</text>
  <rect x="122" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="154" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1: 24B</text>
  <rect x="192" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2: 24B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="290" y="80" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">...</text>
  <rect x="66" y="112" width="240" height="34" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="133" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">&amp;[u8] Contiguous Byte Slice</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU DMA: queue.write_buffer(buf, 0, bytes)</text>
  <rect x="424" y="54" width="272" height="92" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="84" x2="672" y2="84" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">create_buffer_init / write_buffer</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="112" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">VERTEX | INDEX | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">array_stride = size_of::&lt;Vertex&gt;()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU VRAM Buffer -&gt; @vertex Input Locations</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="56" width="224" height="40" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
  <rect x="822" y="106" width="224" height="40" rx="4" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(1) color: vec3&lt;f32&gt;</text>
</svg>
</div>

### 1. `src/face_pick.rs`

Upload bounded face metadata and draw from existing geometry.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn set(
        &mut self,
        device: &wgpu::Device,
        meshes: &[GpuMesh],
        bases: &[(crate::scene::ObjectId, u32)],
    ) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner);
            let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() {
                self.rows.push(None);
                continue;
            }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry
                .source
                .parts
                .triangles
                .iter()
                .flat_map(|n| n.to_ne_bytes())
                .collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("source triangle faces"),
                contents: &bytes,
                usage: wgpu::BufferUsages::STORAGE,
            });
            let data: Vec<u8> = [base, 0, 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect();
            let settings =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("source face base"),
                    contents: &data,
                    usage: wgpu::BufferUsages::UNIFORM,
                });
            let buffers = [
                &geometry.vertices,
                &geometry.indices,
                &faces,
                &settings,
            ];
            let entries: Vec<_> = buffers
                .iter()
                .enumerate()
                .map(|(binding, buffer)| wgpu::BindGroupEntry {
                    binding: binding as u32,
                    resource: buffer.as_entire_binding(),
                })
                .collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("source face metadata"),
                layout: &self.pipeline.get_bind_group_layout(2),
                entries: &entries,
            });
            self.rows.push(Some(Row {
                group,
                bytes: faces.size() + settings.size(),
            }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        meshes: &[GpuMesh],
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) {
            if let Some(row) = row {
                mesh.bind_model(pass);
                pass.set_bind_group(2, &row.group, &[]);
                pass.draw(0..mesh.geometry.index_count, 0..1);
            }
        }
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }
    pub fn ready(&self, count: usize) -> bool {
        self.rows.len() == count
    }

    pub fn bytes(&self) -> u64 {
        self.rows
            .iter()
            .flatten()
            .map(|row| row.bytes)
            .sum()
    }
}
```

## <a id="checkpoint-44g-edge-ink"></a>8. Draw boundary edges consistently

Draw original boundary edges consistently.

### 1. `src/renderer.rs`

Own source tables, visible original-edge ink and mode state.

**Locate this existing block:**

```rust
    picker: crate::pick_gpu::Picker,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 144);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
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
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            settings_writes: 0,
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
            strokes,
            paths,
            markers,
            grid,
            grid_settings: None,
            size: [640, 480],
            density: 1.0,
            picker,
        };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
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
        if self.size != [size.width, size.height] {
            self.picker.cancel();
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.picker.cancel();
            self.density = density;
        }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [
            count,
            bytes,
            self.paths.uploads,
            self.paths.uploaded_bytes,
        ]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [
            count,
            bytes,
            self.markers.uploads,
            self.markers.uploaded_bytes,
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| row.visible())
                    .map(crate::scene::PointObject::marker)
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| row.visible())
                    .flat_map(crate::scene::PathObject::segments),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| row.visible())
                    .map(crate::scene::LineObject::stroke),
            )
            .expect("Prepared line placement fits the display range");
        let controls = scene.control_markers();
        self.markers
            .set_ids(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|r| r.visible())
                    .map(|r| {
                        [
                            if r.prepared.source.is_locked {
                                0
                            } else {
                                r.id.value()
                            },
                            1,
                        ]
                    })
                    .chain(controls.iter().map(|r| {
                        [
                            scene
                                .pick_id(r.parent.value())
                                .map_or(0, ObjectId::value),
                            1,
                        ]
                    })),
            )
            .expect("Marker IDs match markers");
        self.paths
            .set_ids(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|r| r.visible())
                    .flat_map(|r| {
                        vec![
                            [
                                if r.prepared.source.locked() {
                                    0
                                } else {
                                    r.id.value()
                                },
                                1
                            ];
                            r.segments().len()
                        ]
                    }),
            )
            .expect("Path IDs match segments");
        self.strokes
            .set_ids(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|r| r.visible())
                    .map(|r| {
                        [
                            if r.prepared.source.is_locked {
                                0
                            } else {
                                r.id.value()
                            },
                            1,
                        ]
                    }),
            )
            .expect("Line IDs match lines");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| row.visible())
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    selected == Some(object.id),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    selected == Some(object.id),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
    }

    pub fn request_pick(
        &mut self,
        screen: [f32; 2],
        transform: [f32; 16],
    ) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density)
            .ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request {
            window,
            transform,
            density: self.density,
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[&self.strokes, &self.paths, &self.markers],
            );
        }
    }

    pub fn take_pick(
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density)
        {
            Ok(view) => view,
            Err(error) => {
                self.picker.cancel();
                return Some(Err(error));
            }
        };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => {
                self.pump_pick(transform);
                None
            }
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }

    pub fn cancel_pick(&mut self) {
        self.picker.cancel();
    }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [
            active,
            pending,
            bytes
                + self.strokes.pick_bytes()
                + self.paths.pick_bytes()
                + self.markers.pick_bytes(),
            self.picker.submitted,
            self.picker.coalesced,
        ]
    }

    pub fn geometry_owners(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] {
        crate::memory::gpu(&self.meshes)
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 144
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
            + self.grid.uploaded_bytes
    }

    pub fn set_grid(
        &mut self,
        settings: Option<crate::grid::Settings>,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings {
            return Ok(());
        }
        let strokes = settings
            .map(crate::grid::Settings::strokes)
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage();
        [
            count,
            bytes,
            self.grid.uploads,
            self.grid.uploaded_bytes,
        ]
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
            self.settings_allocations,
            self.settings_writes,
            self.settings_bytes,
        ]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) {
        self.draw_mode(view, background, transform, false);
    }

    pub fn draw_mode(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
        normals: bool,
    ) {
        let mut bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let display = [
            if normals {
                1.0f32
            } else {
                0.0
            },
            0.0,
            0.0,
            0.0,
        ];
        bytes.extend(
            display
                .iter()
                .flat_map(|value| value.to_ne_bytes()),
        );
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.paths.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.markers.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.grid.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.grid.draw(&mut pass);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 144);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
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
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            settings_writes: 0,
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
            strokes,
            paths,
            markers,
            grid,
            grid_settings: None,
            size: [640, 480],
            density: 1.0,
            picker,
            faces,
            mesh_ink,
            sources: Default::default(),
            pick_mode: Default::default(),
        };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
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
        if self.size != [size.width, size.height] {
            self.picker.cancel();
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.picker.cancel();
            self.density = density;
        }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [
            count,
            bytes,
            self.paths.uploads,
            self.paths.uploaded_bytes,
        ]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [
            count,
            bytes,
            self.markers.uploads,
            self.markers.uploaded_bytes,
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
            .set(
                &self.device,
                self.sources.mesh_ink.iter().copied(),
            )
            .expect("Prepared source edges");
        self.mesh_ink
            .set_ids(
                &self.device,
                self.sources.mesh_ids.iter().copied(),
            )
            .expect("Source edge IDs match");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| row.visible())
                    .map(crate::scene::PointObject::marker)
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
            .expect("Prepared point placement fits the display range");
        self.paths
            .set_chains(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|row| row.visible())
                    .flat_map(crate::scene::PathObject::segments),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| row.visible())
                    .map(crate::scene::LineObject::stroke),
            )
            .expect("Prepared line placement fits the display range");
        self.markers
            .set_ids(
                &self.device,
                self.sources.marker_ids.iter().copied(),
            )
            .expect("Marker owners match");
        self.paths
            .set_ids(
                &self.device,
                self.sources.path_ids.iter().copied(),
            )
            .expect("Path owners match");
        self.strokes
            .set_ids(
                &self.device,
                self.sources.line_ids.iter().copied(),
            )
            .expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| row.visible())
        {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    selected == Some(object.id),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    selected == Some(object.id),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
    }

    pub fn request_pick(
        &mut self,
        screen: [f32; 2],
        transform: [f32; 16],
    ) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density)
            .ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request {
            window,
            transform,
            density: self.density,
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
        if self.pick_mode == crate::pick_sources::Mode::Face
            && !self.faces.ready(self.meshes.len())
        {
            self.faces.set(
                &self.device,
                &self.meshes,
                &self.sources.mesh_bases,
            );
        }
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[
                    &self.mesh_ink,
                    &self.strokes,
                    &self.paths,
                    &self.markers,
                ],
            );
        }
    }

    pub fn take_pick(
        &mut self,
        transform: [f32; 16],
    ) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density)
        {
            Ok(view) => view,
            Err(error) => {
                self.picker.cancel();
                return Some(Err(error));
            }
        };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => {
                self.pump_pick(transform);
                None
            }
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode {
            self.picker.cancel();
            self.pick_mode = mode;
        }
    }

    pub fn resolve_pick(
        &self,
        scene: &Scene,
        pixel: [u32; 2],
    ) -> Option<crate::pick_sources::Target> {
        self.sources
            .resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.mesh_ink.usage();
        [
            count,
            bytes,
            self.faces.bytes(),
            self.sources.bytes() as u64,
        ]
    }

    pub fn cancel_pick(&mut self) {
        self.picker.cancel();
    }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [
            active,
            pending,
            bytes
                + self.strokes.pick_bytes()
                + self.paths.pick_bytes()
                + self.markers.pick_bytes()
                + self.mesh_ink.pick_bytes()
                + self.mesh_ink.usage()[1]
                + self.faces.bytes(),
            self.picker.submitted,
            self.picker.coalesced,
        ]
    }

    pub fn geometry_owners(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] {
        crate::memory::gpu(&self.meshes)
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 144
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
            + self.grid.uploaded_bytes
    }

    pub fn set_grid(
        &mut self,
        settings: Option<crate::grid::Settings>,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings {
            return Ok(());
        }
        let strokes = settings
            .map(crate::grid::Settings::strokes)
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage();
        [
            count,
            bytes,
            self.grid.uploads,
            self.grid.uploaded_bytes,
        ]
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
            self.settings_allocations,
            self.settings_writes,
            self.settings_bytes,
        ]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) {
        self.draw_mode(view, background, transform, false);
    }

    pub fn draw_mode(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
        normals: bool,
    ) {
        let mut bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let display = [
            if normals {
                1.0f32
            } else {
                0.0
            },
            0.0,
            0.0,
            0.0,
        ];
        bytes.extend(
            display
                .iter()
                .flat_map(|value| value.to_ne_bytes()),
        );
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.paths.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.markers.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.grid.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.mesh_ink.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
```

### 2. `src/pick_window.rs`

Rank source-face pixels as surfaces and original edge pixels as ink.

**Locate this existing block:**

```rust
                }
                let key = (sub == 0, distance, row, sub);
                if best.is_none_or(|old| key < old) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                let key = (
                    crate::sub_id::surface(sub),
                    distance,
                    row,
                    sub,
                );
                if best.is_none_or(|old| key < old) {
```

## <a id="checkpoint-44h-mode-gpu"></a>9. Pick source parts with visible coverage

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU CommandEncoder Records GPU Commands</text>
  <rect x="54" y="50" width="264" height="102" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::CommandEncoder</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">1. begin_render_pass (3D Scene)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">2. begin_render_pass (Overlay/UI)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">3. encoder.finish()</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pass 1 (LoadOp::Clear) -&gt; Pass 2 (LoadOp::Load)</text>
  <rect x="416" y="52" width="128" height="96" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="480" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Clear</text>
  <polygon points="452,132 480,88 508,132" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="52" width="128" height="96" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Load</text>
  <polygon points="612,126 640,86 668,126" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <rect x="590" y="130" width="100" height="12" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.submit([cmd_buf]) -&gt; GPU Execution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,132 934,58 984,132" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="136" width="224" height="18" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="149" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Composited Frame Ready</text>
</svg>
</div>

### 1. `src/pick_gpu.rs`

Use mode-aware coverage and compatible eighty-byte crop settings.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    + u64::from(image.window.draw[2])
                        * u64::from(image.window.draw[3])
                        * 12
            }) + 144,
        ]
```

Replace it with:

```rust
                    + u64::from(image.window.draw[2])
                        * u64::from(image.window.draw[3])
                        * 12
            }) + 160,
        ]
```

**Locate this block:**

```rust
            &self.crop_group,
            meshes,
            ink,
        )]);
        let ready = Arc::new(AtomicU8::new(0));
        self.active = Some(Flight {
```

Replace it with:

```rust
            &self.crop_group,
            meshes,
            ink,
            faces,
            request.mode,
        )]);
        let ready = Arc::new(AtomicU8::new(0));
        self.active = Some(Flight {
```

**Locate this block:**

```rust
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
```

Replace it with:

```rust
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let mut bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.extend((request.mode as u32).to_ne_bytes());
        bytes.resize(80, 0);
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
```

**Locate this block:**

```rust
        queued: Queued,
        meshes: &[GpuMesh],
        ink: &[&Lane],
    ) {
        assert!(
            self.active.is_none(),
```

Replace it with:

```rust
        queued: Queued,
        meshes: &[GpuMesh],
        ink: &[&Lane],
        faces: &crate::face_pick::Faces,
    ) {
        assert!(
            self.active.is_none(),
```

**Locate this block:**

```rust
            })
        };
        let view = buffer(80);
        let crop = buffer(64);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self {
```

Replace it with:

```rust
            })
        };
        let view = buffer(80);
        let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self {
```

**Locate this block:**

```rust
        view_layout: &wgpu::BindGroupLayout,
        model_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let pipeline = crate::pick_pipeline::mesh(
            device,
            view_layout,
```

Replace it with:

```rust
        view_layout: &wgpu::BindGroupLayout,
        model_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(
            device,
            view_layout,
```

**Locate this block:**

```rust
    pub density: f32,
}

pub(crate) struct Queued {
```

Replace it with:

```rust
    pub density: f32,
    pub mode: crate::pick_sources::Mode,
}

pub(crate) struct Queued {
```

### 2. `src/pick_encode.rs`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```rust
    ink: &[&Lane],
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ink: &[&Lane],
    faces: &crate::face_pick::Faces,
    mode: crate::pick_sources::Mode,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
        if mode == crate::pick_sources::Mode::Face {
            faces.draw(&mut pass, meshes, view_group, crop_group);
        }
```

### 3. `src/pick_pipeline.rs`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```rust
                "\n",
                include_str!("triangle_pick.wgsl")
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                "\n",
                include_str!("pick_crop.wgsl"),
                "\n",
                include_str!("triangle_pick.wgsl")
```

### 4. `src/stroke_gpu.rs`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```rust
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
            bind_group_layouts: &[Some(&view_layout)],
            immediate_size: 0,
        });
        let pick_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ink pick settings"),
                bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)],
                immediate_size: 0,
            });
        let pick_source = match stride {
            44 => include_str!("stroke_pick.wgsl"),
            72 => include_str!("chain_pick.wgsl"),
            32 => include_str!("marker_pick.wgsl"),
            _ => unreachable!(),
        };
        let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{pick_source}").into()),
        });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
            bind_group_layouts: &[Some(&view_layout)],
            immediate_size: 0,
        });
        let pick_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ink pick settings"),
                bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)],
                immediate_size: 0,
            });
        let pick_source = match stride {
            44 => include_str!("stroke_pick.wgsl"),
            72 => include_str!("chain_pick.wgsl"),
            32 => include_str!("marker_pick.wgsl"),
            _ => unreachable!(),
        };
        let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "{source}\n{}\n{pick_source}",
                    include_str!("pick_crop.wgsl")
                )
                .into(),
            ),
        });
```

### 5. `src/triangle_pick.wgsl`

Use mode-aware coverage and compatible eighty-byte crop settings.

Replace the entire file with:

```wgsl
@group(2) @binding(0) var<uniform> crop: Crop;
@vertex
fn pick_vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> @builtin(position) vec4<f32> {
    return crop.transform * geometry(position, colour, normal).position;
}
@fragment
fn pick_fragment() -> @location(0) vec2<u32> { return vec2<u32>(select(0u, object.selection.y, crop.mode == 0u), 0u); }
```

### 6. `src/stroke_pick.wgsl`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width);
    if (colour(source).a <= 0.0) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
```

### 7. `src/marker_pick.wgsl`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius);
    if (colour(source).a <= 0.0) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
```

### 8. `src/chain_pick.wgsl`

Use mode-aware coverage and compatible eighty-byte crop settings.

**Locate this existing block:**

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, previous, next, heads, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.pixel = source.pixel;
    output.origin = source.origin;
    output.direction = source.direction;
    output.half_width = source.half_width;
    output.part = source.part;
    output.head_length = source.head_length;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.pixel, input.origin, input.direction, input.half_width, input.part, input.head_length);
    if (colour(source).a <= 0.0) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, previous, next, heads, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.pixel = source.pixel;
    output.origin = source.origin;
    output.direction = source.direction;
    output.half_width = source.half_width;
    output.part = source.part;
    output.head_length = source.head_length;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.pixel, input.origin, input.direction, input.half_width, input.part, input.head_length);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
```

### 9. `src/renderer.rs`

Submit source faces and original edge ink in the current mode.

**Locate this existing block:**

```rust
            density: self.density,
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
        if self.pick_mode == crate::pick_sources::Mode::Face
            && !self.faces.ready(self.meshes.len())
        {
            self.faces.set(
                &self.device,
                &self.meshes,
                &self.sources.mesh_bases,
            );
        }
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[
                    &self.mesh_ink,
                    &self.strokes,
                    &self.paths,
                    &self.markers,
                ],
            );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            density: self.density,
            mode: self.pick_mode,
        })?;
        self.pump_pick(transform);
        Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density)
        else {
            self.picker.cancel();
            return;
        };
        if self.pick_mode == crate::pick_sources::Mode::Face
            && !self.faces.ready(self.meshes.len())
        {
            self.faces.set(
                &self.device,
                &self.meshes,
                &self.sources.mesh_bases,
            );
        }
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[
                    &self.mesh_ink,
                    &self.strokes,
                    &self.paths,
                    &self.markers,
                ],
                &self.faces,
            );
```

### 10. `src/pick_gpu_tests.rs`

Copy actual sparse-key, diagonal and original-control GPU checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "Density changed while the map was pending"
    );
}
```

Replace it with:

```rust
        "Density changed while the map was pending"
    );
}

#[test]
fn actual_face_and_edge_ids_preserve_sparse_source_keys_and_exclude_a_rendering_diagonal()
 {
    let mut source = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.5]),
        (20, [0.5, -0.5, 0.5]),
        (30, [0.5, 0.5, 0.5]),
        (40, [-0.5, 0.5, 0.5]),
    ] {
        source.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    source.add_face(vec![10, 20, 30, 40], Some(37));
    source.set_face_triangulation(37, vec![[10, 20, 40], [20, 30, 40]]);
    source.set_facecolors(vec![session_rust::Color::new(
        0.8, 0.2, 0.1, 1.,
    )]);
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let before = renderer.stats();
    renderer.set_pick_mode(crate::pick_sources::Mode::Face);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Face(37)
        })
    );
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::MeshEdge(10, 20)
        })
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "The triangulation diagonal is not an editable edge"
    );
    renderer.set_pick_mode(crate::pick_sources::Mode::Object);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    assert_eq!(renderer.stats(), before);
}

#[test]
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
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
    let id = editor.scene.paths()[0].id;
    let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Control(1)
        })
    );
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., 0.1], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Curve
        })
    );
    editor.scene.show_controls(false);
    renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}
```

**Locate this block:**

```rust
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 144]);
    let new = scene.insert(triangle(0.3)).unwrap();
    assert_ne!(old, new);
    renderer.set_scene(&scene, None);
```

Replace it with:

```rust
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 160]);
    let new = scene.insert(triangle(0.3)).unwrap();
    assert_ne!(old, new);
    renderer.set_scene(&scene, None);
```

**Locate this block:**

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 12 + 176,
        "Readback does not grow with the full drawing"
    );
}
```

Replace it with:

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 12 + 192 + 3 * 52,
        "Readback does not grow with the full drawing"
    );
}
```

## <a id="checkpoint-44i-ink-highlight"></a>10. Highlight edges and controls

Highlight original edges and controls.

### 1. `src/highlight.rs`

Own derived overlay lanes, drawing and GPU byte accounting.

Create the file and type:

```rust
use crate::{
    gpu_mesh::GpuMesh,
    pick_sources::{Part, Target},
    scene::Scene,
    stroke_gpu::Lane,
};
use std::rc::Rc;

pub struct Highlight {
    mesh: Option<GpuMesh>,
    strokes: Lane,
    paths: Lane,
    markers: Lane,
}

impl Highlight {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mesh: None,
            strokes: Lane::new(device, format),
            paths: Lane::joined(device, format),
            markers: Lane::markers(device, format),
        }
    }

    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.view(queue, transform, size, density);
        }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(mesh) = &self.mesh {
            mesh.draw(pass);
        }
    }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.paths.draw(pass);
        self.markers.draw(pass);
    }
    pub fn bytes(&self) -> u64 {
        self.mesh.as_ref().map_or(0, |m| {
            m.geometry.allocated_bytes() + m.settings_bytes()
        }) + self.strokes.usage()[1]
            + self.paths.usage()[1]
            + self.markers.usage()[1]
    }
}
```

### 2. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
mod pick_sources_tests;

pub mod face_pick;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod pick_sources_tests;

pub mod face_pick;

pub mod highlight;
```

## <a id="checkpoint-44ia-original-ink"></a>11. Build highlights from ink targets

Build highlights from original ink targets.

### 1. `src/highlight.rs`

Build original edge, curve and control overlays.

**Locate this existing block:**

```rust
    }

    pub fn view(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn set(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let mut strokes = Vec::new();
        let mut paths = Vec::new();
        let mut markers = Vec::new();
        let yellow = [1., 0.75, 0.05, 1.];
        if let Some(target) = target.filter(|t| t.valid(scene)) {
            match target.part {
                Part::MeshEdge(a, b) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let index = row
                        .mesh
                        .parts
                        .edges
                        .iter()
                        .position(|&edge| edge == (a, b))
                        .unwrap();
                    let ends = row.mesh.parts.endpoints[index].map(|p| {
                        row.model
                            .transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ))
                            .to_f32()
                    });
                    strokes.push(crate::stroke::Stroke {
                        start: ends[0],
                        end: ends[1],
                        colour: yellow,
                        width: 3.,
                    });
                }
                Part::PathEdge(index) => {
                    let row = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = row.points();
                    strokes.push(crate::stroke::Stroke {
                        start: points[index].to_f32(),
                        end: points[index + 1].to_f32(),
                        colour: yellow,
                        width: row.prepared.width + 2.,
                    });
                }
                Part::Curve => {
                    if let Some(row) = scene
                        .lines()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        strokes.push(crate::stroke::Stroke {
                            colour: yellow,
                            width: row.prepared.display.width + 2.,
                            ..row.stroke()
                        });
                    } else if let Some(row) = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        paths.extend(
                            row.segments()
                                .into_iter()
                                .map(|mut segment| {
                                    segment.stroke.colour = yellow;
                                    segment.stroke.width += 2.;
                                    segment
                                }),
                        );
                    }
                }
                Part::Control(index) => {
                    let marker = scene
                        .control_markers()
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.control.index == index)
                        .unwrap();
                    markers.push(crate::marker::Marker {
                        diameter: marker.display.diameter + 4.,
                        colour: yellow,
                        ..marker.display
                    });
                }
                Part::Face(_) => {
                    return Err("Face overlays are added in the next checkpoint");
                }
                Part::Object => {}
            }
        }
        self.strokes.set(device, strokes)?;
        self.paths.set_chains(device, paths)?;
        self.markers.set_points(device, markers)?;
        Ok(())
    }

    pub fn view(
```

## <a id="checkpoint-44j-face-highlight"></a>12. Highlight one face without recolouring its parent

Highlight one original face without recolouring its parent.

### 1. `src/highlight.rs`

Build a local original-face overlay.

**Locate this existing block:**

```rust
            match target.part {
                Part::MeshEdge(a, b) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            match target.part {
                Part::Face(key) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let face = row
                        .mesh
                        .parts
                        .faces
                        .iter()
                        .position(|&f| f == key)
                        .unwrap() as u32;
                    // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                    let mut vertices = Vec::new();
                    let mut normals = Vec::new();
                    let mut indices = Vec::new();
                    for (tri, &source) in row
                        .mesh
                        .indices()
                        .chunks_exact(3)
                        .zip(&row.mesh.parts.triangles)
                    {
                        if source == face {
                            for &index in tri {
                                indices.push(vertices.len() as u32);
                                vertices.push(row.mesh.vertices()[index as usize]);
                                normals.push(row.mesh.normals()[index as usize]);
                            }
                        }
                    }
                    let display = crate::mesh::Mesh::new(vertices, indices)?
                        .with_normals(normals)?;
                    let mut object = row.clone();
                    object.mesh = Rc::new(display);
                    self.mesh = Some(GpuMesh::upload(
                        device, layout, &object, true,
                    ));
                }
                Part::MeshEdge(a, b) => {
```

### 2. `src/renderer.rs`

Draw read-only highlights before original black ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.mesh_ink.draw(&mut pass);
```

Replace it with:

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
```

**Locate this block:**

```rust
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
            self.size,
            self.density,
        );
        self.highlight.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

**Locate this block:**

```rust
        ]
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
```

Replace it with:

```rust
        ]
    }

    pub fn set_highlight(
        &mut self,
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
    ) -> Result<(), &'static str> {
        self.highlight.set(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
        )
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
```

**Locate this block:**

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
```

Replace it with:

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.highlight
            .set(
                &self.device,
                &self.pipeline.get_bind_group_layout(1),
                scene,
                None,
            )
            .expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene)
            .expect("Prepared source topology is bounded");
        self.mesh_ink
```

**Locate this block:**

```rust
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

Replace it with:

```rust
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

**Locate this block:**

```rust
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

Replace it with:

```rust
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

**Locate this block:**

```rust
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
```

Replace it with:

```rust
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

### 3. `src/pick_gpu_tests.rs`

Copy actual face, edge and control highlight pixels and release checks.

**Locate this existing block:**

```rust
    assert!(finish(&mut renderer).is_none());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.draw_mode(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &identity(),
        false,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &bytes,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    bytes
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec();
    bytes.unmap();
    result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.8, -0.6, 0.5]),
        (20, [0., -0.6, 0.5]),
        (30, [0., 0.6, 0.5]),
        (40, [-0.8, 0.6, 0.5]),
        (50, [0.8, -0.6, 0.5]),
        (60, [0.8, 0.6, 0.5]),
    ] {
        source.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    source.add_face(vec![10, 20, 30, 40], Some(37));
    source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh);
    let mut renderer = renderer(&scene);
    let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::Face(37),
            }),
        )
        .unwrap();
    let face = colour_pixels(&mut renderer);
    assert!(renderer.highlight_bytes() > 0);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(face.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((31..=160).contains(&x) && (47..=192).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 10000,
        "The original face, rather than the whole object, is visibly selected"
    );
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::MeshEdge(10, 20),
            }),
        )
        .unwrap();
    let edge = colour_pixels(&mut renderer);
    let mut changed = 0;
    let mut black = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(edge.chunks_exact(4))
        .enumerate()
    {
        let x = index % 320;
        let y = index / 320;
        if a != b {
            assert!((30..=161).contains(&x) && (190..=193).contains(&y));
            changed += 1;
        }
        if (35..150).contains(&x)
            && (191..=192).contains(&y)
            && b[0] < 160
            && b[1] < 140
            && b[2] < 80
        {
            black += 1;
        }
    }
    assert!(
        changed > 80 && black > 80,
        "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}"
    );
    assert_eq!(
        renderer.stats(),
        stats,
        "Highlights reuse primary geometry"
    );
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.highlight_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision()
 {
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
    let original = editor.scene.paths()[0].points();
    let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &editor.scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Control(1),
            }),
        )
        .unwrap();
    let after = colour_pixels(&mut renderer);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((151..=168).contains(&x) && (39..=56).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 20,
        "The halo belongs to the original control rather than a curve sample"
    );
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    renderer.set_scene(&editor.scene, None);
    assert_eq!(renderer.highlight_bytes(), 0);
}
```

### 4. `src/highlight.rs`

Remove the preparation-only face refusal now that original-face overlays are implemented.

**Locate this existing block:**

```rust
                }
                Part::Face(_) => {
                    return Err("Face overlays are added in the next checkpoint");
                }
                Part::Object => {}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                Part::Object => {}
```

## <a id="checkpoint-44k-selection"></a>13. Keep part selection outside document history

### 1. `src/selection.rs`

Describe the original selected source part.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!(
                "Object {} · source face {key}",
                self.owner.value()
            ),
            Part::MeshEdge(a, b) => format!(
                "Object {} · source edge {a}–{b}",
                self.owner.value()
            ),
            Part::PathEdge(index) => format!(
                "Object {} · source segment {index}",
                self.owner.value()
            ),
            Part::Curve => format!(
                "Object {} · original curve",
                self.owner.value()
            ),
            Part::Control(index) => format!(
                "Object {} · original control {index}",
                self.owner.value()
            ),
        }
    }
}
```

### 2. `src/editor.rs`

Validate part selection and guard commands that require a whole object.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.selected = self
            .selected
            .filter(|id| self.scene.contains(*id));
        Ok(Change::Scene)
```

Replace it with:

```rust
        self.selected = self
            .selected
            .filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) {
                self.selected = None;
            }
        }
        if self.selected.is_none() {
            self.selected_part = None;
        }
        Ok(Change::Scene)
```

**Locate this block:**

```rust
            }
            Action::Translate(offset) => {
                let id = self
                    .selected
                    .ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
            Action::ToggleExtra => self
                .history
                .edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                return match self.selected {
                    Some(id) => self.delete_object(id),
                    None => Ok(Change::View),
```

Replace it with:

```rust
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
            Action::SelectNext => {
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
```

**Locate this block:**

```rust
        });
        if self.selected == Some(id) {
            self.selected = None;
        }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Picked(id) => {
                self.selected = id.filter(|id| self.scene.pick_id(id.value()).is_some())
            }
            Action::Pick(screen) => {
                self.selected = self
                    .camera
                    .ray(screen)
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
```

Replace it with:

```rust
        Self {
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
            pick_mode: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
```

**Locate this block:**

```rust
pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
```

Replace it with:

```rust
pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
```

**Locate this block:**

```rust
    Picked(Option<ObjectId>),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
```

Replace it with:

```rust
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickMode(crate::pick_sources::Mode),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
```

### 3. `src/selection_tests.rs`

Copy selection, command refusal, history and invalid-target checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Target},
};

#[test]
fn selecting_a_part_does_not_edit_history_or_implicitly_delete_its_parent() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddBox).unwrap();
    let owner = editor.scene.objects()[0].id;
    let source = std::rc::Rc::clone(editor.scene.objects()[0].geometry().unwrap());
    let target = Target {
        owner,
        part: Part::Face(0),
    };
    editor
        .apply(Action::PickMode(Mode::Face))
        .unwrap();
    editor
        .apply(Action::PickedTarget(Some(target)))
        .unwrap();
    assert_eq!(editor.selection(), Some(target));
    assert_eq!(editor.whole_selection(), None);
    assert!(
        editor
            .apply(Action::Translate([1., 0., 0.]))
            .is_err()
    );
    assert!(editor.apply(Action::Delete).is_err());
    assert_eq!(editor.scene.objects().len(), 1);
    assert!(std::rc::Rc::ptr_eq(
        &source,
        &editor.scene.objects()[0].geometry().unwrap()
    ));
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.objects().is_empty());
    assert_eq!(editor.selection(), None);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, owner);
    editor
        .apply(Action::PickedTarget(Some(target)))
        .unwrap();
    editor
        .apply(Action::PickMode(Mode::Object))
        .unwrap();
    assert_eq!(editor.selection(), None);
    editor
        .apply(Action::Picked(Some(owner)))
        .unwrap();
    editor.apply(Action::Delete).unwrap();
    assert!(editor.scene.objects().is_empty());
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, owner);
}

#[test]
fn hidden_controls_closed_owners_and_invalid_source_addresses_are_rejected() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddCurve).unwrap();
    editor.apply(Action::Controls(true)).unwrap();
    let owner = editor.scene.paths()[0].id;
    let control = Target {
        owner,
        part: Part::Control(1),
    };
    editor
        .apply(Action::PickedTarget(Some(control)))
        .unwrap();
    assert_eq!(editor.selection(), Some(control));
    editor
        .apply(Action::Controls(false))
        .unwrap();
    assert_eq!(editor.selection(), None);
    editor
        .apply(Action::PickedTarget(Some(control)))
        .unwrap();
    assert_eq!(editor.selection(), None);
    editor.apply(Action::AddPolyline).unwrap();
    let owner = editor.scene.paths()[1].id;
    for part in [
        Part::PathEdge(usize::MAX),
        Part::Face(usize::MAX),
        Part::Control(usize::MAX),
    ] {
        editor
            .apply(Action::PickedTarget(Some(Target {
                owner,
                part,
            })))
            .unwrap();
        assert_eq!(editor.selection(), None);
    }
    editor.apply(Action::Close).unwrap();
    assert!(!control.valid(&editor.scene));
}
```

### 4. `src/lib.rs`

Register the component and copied checks.

**Locate this existing block:**

```rust
pub mod face_pick;

pub mod highlight;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod face_pick;

pub mod highlight;

#[cfg(test)]
mod selection_tests;
```

## <a id="checkpoint-44l-input"></a>14. Select and inspect faces, edges and controls

### 1. `src/browser.rs`

Connect typed mode commands, current hits, local overlays and source inspection. Capture source reload intents only for a whole-object selection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
```

Replace it with:

```rust
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-highlight-bytes",
        &renderer.highlight_bytes().to_string(),
    )?;
    canvas.set_attribute(
        "data-selection-part",
        &editor
            .selection()
            .map_or_else(String::new, |target| target.describe()),
    )?;
    canvas.set_attribute(
        "data-pick-mode",
        &format!("{:?}", editor.pick_mode),
    )?;
    canvas.set_attribute(
        "data-part-usage",
        &serde_json::to_string(&renderer.part_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
                    }
                }
            }
            if let Err(error) = renderer.set_grid(editor.camera.grid) {
                report(error);
                return;
```

Replace it with:

```rust
                    }
                }
            }
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) =
                    renderer.set_highlight(&editor.scene, editor.selection())
                {
                    report(error);
                    return;
                }
            }
            if let Err(error) = renderer.set_grid(editor.camera.grid) {
                report(error);
                return;
```

**Locate this block:**

```rust
                });
                match change {
                    Ok(Some(Change::Scene)) => {
                        crate::browser_upload::prepare(
                            &mut renderer,
                            &editor.scene,
                            editor.selected,
                            active_fault.clone(),
                        );
                        if unloading {
```

Replace it with:

```rust
                });
                match change {
                    Ok(Some(Change::Scene)) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(
                            &mut renderer,
                            &editor.scene,
                            editor.whole_selection(),
                            active_fault.clone(),
                        );
                        if unloading {
```

**Locate this block:**

```rust
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                let waiting =
                    crate::edit_intent::Intent::capture(&action, editor.selected)
                        .map_err(str::to_owned)
                        .and_then(|intent| match intent {
                            Some(intent) => crate::browser_reload::restore_for(
                                intent,
                                &editor,
                                std::rc::Rc::clone(&reload),
                                std::rc::Rc::clone(&reload_delivery),
                            ),
                            None => Ok(false),
                        });
                let change = waiting.and_then(|waiting| {
                    if waiting {
                        panel.result("Reloading editable sources…");
```

Replace it with:

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
                        intent,
                        &editor,
                        std::rc::Rc::clone(&reload),
                        std::rc::Rc::clone(&reload_delivery),
                    ),
                    None => Ok(false),
                });
                let change = waiting.and_then(|waiting| {
                    if waiting {
                        panel.result("Reloading editable sources…");
```

**Locate this block:**

```rust
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::Picked(
                        hit.and_then(|hit| editor.scene.pick_id(hit[0])),
                    )),
                    Some(Err(error)) => {
                        report(error);
                        None
```

Replace it with:

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

**Locate this block:**

```rust
                        "example arrow" => Action::AddArrow,
                        "example triangle" => Action::ToggleExtra,
                        "select next" => Action::SelectNext,
                        "delete" => Action::Delete,
                        "undo" => Action::Undo,
                        "redo" => Action::Redo,
```

Replace it with:

```rust
                        "example arrow" => Action::AddArrow,
                        "example triangle" => Action::ToggleExtra,
                        "select next" => Action::SelectNext,
                        "select object" => {
                            Action::PickMode(crate::pick_sources::Mode::Object)
                        }
                        "select face" => {
                            Action::PickMode(crate::pick_sources::Mode::Face)
                        }
                        "select edge" => {
                            Action::PickMode(crate::pick_sources::Mode::Edge)
                        }
                        "select control" => {
                            Action::PickMode(crate::pick_sources::Mode::Control)
                        }
                        "delete" => Action::Delete,
                        "undo" => Action::Undo,
                        "redo" => Action::Redo,
```

**Locate this block:**

```rust
                    }
                }
            } else if let Some(line) = line {
                if line == "reload sources" {
                    let result = crate::browser_reload::start(
                        std::rc::Rc::clone(&reload),
                        editor.reload_keys(),
```

Replace it with:

```rust
                    }
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
                        std::rc::Rc::clone(&reload),
                        editor.reload_keys(),
```

**Locate this block:**

```rust
                        .map_or("Reload Sources", |intent| intent.command());
                    match reply.complete(&mut editor) {
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.selected,
                                active_fault.clone(),
                            );
                            crate::browser_phase::revision(&editor.scene);
```

Replace it with:

```rust
                        .map_or("Reload Sources", |intent| intent.command());
                    match reply.complete(&mut editor) {
                        Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                            highlight_changed = true;
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.whole_selection(),
                                active_fault.clone(),
                            );
                            crate::browser_phase::revision(&editor.scene);
```

**Locate this block:**

```rust
                }
            };

            let action = if event.type_() == "cancel"
                && event
                    .target()
```

Replace it with:

```rust
                }
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
                    .target()
```

**Locate this block:**

```rust
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
```

Replace it with:

```rust
            "Select Next",
            "Select Object",
            "Select Face",
            "Select Edge",
            "Select Control",
            "Selection",
            "Delete",
            "Undo",
            "Redo",
```

## Build & Run

![Recorded visual output for Lesson 6.3 (44l-input)](../../../screenshots/journey/44l-input.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 6.3
npm --prefix ../session_tests run course -- save 6.3
```

Continue with [6.4 · Ordered multi-selection and atomic edits](../../consolidation/45-order.md).
