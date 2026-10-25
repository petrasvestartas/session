# 8.3 · Layers, visibility and source-preserving properties

**Steps:** 23 · **Source:** [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/layers.rs](../../../src/app/layers.rs) · [app/ui/layers.rs](../../../src/app/ui/layers.rs) · [app/gesture/mod.rs](../../../src/app/gesture/mod.rs)

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

## <a id="checkpoint-53-flags"></a>1. Separate layer presentation from sources

### 1. `src/view_flags.rs`

Create the file and type:

```rust
use crate::scene::ObjectId;
use std::collections::{HashMap, HashSet};

// Presentation stores plain identities and colours, so hiding or recolouring geometry never edits or retains its original source records.
#[derive(Clone, Default)]
pub struct Flags {
    pub current: Option<String>,
    pub hidden: HashSet<ObjectId>,
    pub lamps: HashSet<String>,
    pub held: HashSet<ObjectId>,
    pub locked: HashSet<ObjectId>,
    pub faces: HashMap<ObjectId, [u8; 3]>,
    pub edges: HashMap<ObjectId, [u8; 3]>,
}
impl Flags {
    // A parent lamp masks drawing without overwriting the independent visibility of its children.
    pub fn is_hidden(&self, id: &ObjectId) -> bool {
        self.hidden.contains(id) || self.held.contains(id)
    }
    pub fn show_all(&mut self) {
        self.hidden.clear();
        self.lamps.clear();
        self.held.clear();
    }
    pub fn refresh(&mut self, forest: &crate::hierarchy::Forest) {
        self.held.clear();
        for doc in &forest.documents {
            for (index, node) in doc.nodes.iter().enumerate() {
                if self
                    .lamps
                    .contains(&format!("{}/{}", doc.key, node.source.guid))
                {
                    self.held.extend(
                        doc.nodes[index..node.end]
                            .iter()
                            .filter_map(|node| node.object),
                    );
                }
            }
        }
    }

    pub fn initial(&mut self, id: ObjectId, visible: bool, locked: bool) {
        if !visible {
            self.hidden.insert(id);
        }
        if locked {
            self.locked.insert(id);
        }
    }
    pub fn hide(&mut self, ids: &[ObjectId], on: bool) {
        for id in ids {
            if on {
                self.hidden.insert(*id);
            } else {
                self.hidden.remove(id);
            }
        }
    }
    pub fn lock(&mut self, ids: &[ObjectId], on: bool) {
        for id in ids {
            if on {
                self.locked.insert(*id);
            } else {
                self.locked.remove(id);
            }
        }
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod hierarchy_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod hierarchy_tests;

pub mod view_flags;
```

## <a id="checkpoint-53a-ids"></a>2. Index live identities and initialise view flags

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.objects
            .iter()
            .map(|r| r.id)
            .chain(self.lines.iter().map(|r| r.id))
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .any(|row| row == id)
    }
```

Replace it with:

```rust
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.ids.contains(&id)
    }
```

**Locate this block:**

```rust
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = self.contains(id);
        self.objects.retain(|row| row.id != id);
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
```

Replace it with:

```rust
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id);
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
```

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.forest = Default::default();
        self.points = Vec::new();
        self.paths = Vec::new();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.points = Vec::new();
        self.paths = Vec::new();
```

**Locate this block:**

```rust
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
```

Replace it with:

```rust
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        self.lines.push(LineObject {
            id,
            prepared,
```

**Locate this block:**

```rust
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.paths.push(PathObject {
            controls_visible: false,
            id,
```

Replace it with:

```rust
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.visible(),
            prepared.source.locked(),
        );
        self.paths.push(PathObject {
            controls_visible: false,
            id,
```

**Locate this block:**

```rust
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.points.push(PointObject {
            id,
            guid,
```

Replace it with:

```rust
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        self.points.push(PointObject {
            id,
            guid,
```

**Locate this block:**

```rust
        for object in self
            .objects
            .iter()
            .filter(|row| row.visible())
        {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
```

Replace it with:

```rust
        for object in self
            .objects
            .iter()
            .filter(|row| self.visible(row.id))
        {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
```

**Locate this block:**

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            forest: Default::default(),
            objects: Vec::new(),
            points: Vec::new(),
```

Replace it with:

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            objects: Vec::new(),
            points: Vec::new(),
```

**Locate this block:**

```rust
pub struct Scene {
    pub forest: crate::hierarchy::Forest,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
```

Replace it with:

```rust
pub struct Scene {
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
```

### 2. `src/hierarchy.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.documents
            .iter()
            .flat_map(|doc| &doc.nodes)
            .any(|n| n.object == Some(id) && n.readonly)
    }
```

Replace it with:

```rust
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.readonly_ids.contains(&id)
    }
```

**Locate this block:**

```rust
                )
            }));
        }
        self.documents.push(Rc::new(Document {
            key,
            label,
```

Replace it with:

```rust
                )
            }));
        }
        Rc::make_mut(&mut self.readonly_ids).extend(
            nodes
                .iter()
                .filter(|node| node.readonly)
                .filter_map(|node| node.object),
        );
        self.documents.push(Rc::new(Document {
            key,
            label,
```

**Locate this block:**

```rust
    pub documents: Vec<Rc<Document>>,
}

impl Forest {
```

Replace it with:

```rust
    pub documents: Vec<Rc<Document>>,
    readonly_ids: Rc<HashSet<ObjectId>>,
}

impl Forest {
```

## <a id="checkpoint-53b-uniform"></a>3. Update surface colour and pickability in small GPU records

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

### 1. `src/gpu_mesh.rs`

**Locate this existing block:**

```rust
    model: wgpu::Buffer,
    previous: [u8; 144],
}

fn settings(object: &Object, selected: bool) -> [u8; 144] {
    let mut bytes = [0; 144];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(selected).to_ne_bytes());
    let id = if object.metadata.is_locked {
        0
    } else {
        object.id.value()
    };
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
        .expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry(device, layout, object, selected, geometry)
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            geometry,
            model_group,
            id: object.id,
            model,
            previous: bytes,
        }
    }

    pub fn settings_bytes(&self) -> u64 {
        self.model.size()
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        object: &Object,
        selected: bool,
    ) -> [usize; 2] {
        let next = settings(object, selected);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144)] {
            if next[start..end] != self.previous[start..end] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    model: wgpu::Buffer,
    previous: [u8; 160],
}

#[derive(Clone, Copy)]
pub struct Style {
    pub selected: bool,
    pub pickable: bool,
    pub colour: Option<[u8; 3]>,
}

fn settings(object: &Object, style: Style) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = if style.pickable {
        object.id.value()
    } else {
        0
    };
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
        .expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour) = style.colour {
        for (index, value) in colour
            .into_iter()
            .map(|v| v as f32 / 255.)
            .chain([1.])
            .enumerate()
        {
            bytes[144 + index * 4..148 + index * 4]
                .copy_from_slice(&value.to_ne_bytes());
        }
    }
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry(
            device,
            layout,
            object,
            Style {
                selected,
                pickable: !object.metadata.is_locked,
                colour: None,
            },
            geometry,
        )
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            geometry,
            model_group,
            id: object.id,
            model,
            previous: bytes,
        }
    }

    pub fn settings_bytes(&self) -> u64 {
        self.model.size()
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let next = settings(object, style);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144, 160)] {
            if next[start..end] != self.previous[start..end] {
```

### 2. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;
```

### 3. `src/face_pick.rs`

**Locate this existing block:**

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 144);
        let crop = crate::gpu_layout::uniform(device, 80);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let crop = crate::gpu_layout::uniform(device, 80);
```

### 4. `src/renderer.rs`

**Locate this existing block:**

```rust
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 144);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
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
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            widget,
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
        self.preview
            .set(&self.device, None)
            .expect("Empty tool preview");
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

    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        self.preview.set(&self.device, draft)
    }
    pub fn preview_bytes(&self) -> u64 {
        self.preview.bytes()
    }

    pub fn set_gizmo(
        &mut self,
        camera: &crate::camera::Camera,
        origin: Option<[f64; 3]>,
        active: f32,
    ) {
        let matrix = camera.view_projection();
        let layout = origin.and_then(|p| {
            crate::widget_layout::world_per_pixel(
                &matrix,
                p,
                self.size[1] as f64 / self.density as f64,
            )
            .and_then(|scale| {
                crate::widget_layout::layout(&matrix, p, scale, self.size, active)
            })
        });
        self.widget
            .prepare(&self.device, &self.queue, layout);
    }
    pub fn gizmo_usage(&self) -> [u64; 4] {
        self.widget.usage()
    }
    pub fn gizmo_extent(&self) -> Option<[u32; 2]> {
        self.widget.extent()
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
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
        self.highlight.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.preview.view(
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
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder, view);
        self.queue.submit([encoder.finish()]);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
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
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            widget,
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
        self.preview
            .set(&self.device, None)
            .expect("Empty tool preview");
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
                    style(scene, object.id, selected == Some(object.id)),
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
                    style(scene, object.id, selected == Some(object.id)),
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
            + self.settings_allocations as u64 * 160
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
                let [calls, bytes] = mesh.update(
                    &self.queue,
                    row,
                    style(scene, row.id, ids.contains(&row.id)),
                );
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

    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        self.preview.set(&self.device, draft)
    }
    pub fn preview_bytes(&self) -> u64 {
        self.preview.bytes()
    }

    pub fn set_gizmo(
        &mut self,
        camera: &crate::camera::Camera,
        origin: Option<[f64; 3]>,
        active: f32,
    ) {
        let matrix = camera.view_projection();
        let layout = origin.and_then(|p| {
            crate::widget_layout::world_per_pixel(
                &matrix,
                p,
                self.size[1] as f64 / self.density as f64,
            )
            .and_then(|scale| {
                crate::widget_layout::layout(&matrix, p, scale, self.size, active)
            })
        });
        self.widget
            .prepare(&self.device, &self.queue, layout);
    }
    pub fn gizmo_usage(&self) -> [u64; 4] {
        self.widget.usage()
    }
    pub fn gizmo_extent(&self) -> Option<[u32; 2]> {
        self.widget.extent()
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
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
        self.highlight.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.preview.view(
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
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder, view);
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene: &Scene, id: ObjectId, selected: bool) -> crate::gpu_mesh::Style {
    crate::gpu_mesh::Style {
        selected,
        pickable: scene.pick_id(id.value()).is_some(),
        colour: scene.view.faces.get(&id).copied(),
    }
}

fn rgba(rgb: [u8; 3]) -> [f32; 4] {
    [
        rgb[0] as f32 / 255.,
        rgb[1] as f32 / 255.,
        rgb[2] as f32 / 255.,
        1.,
    ]
}
```

## <a id="checkpoint-53bb-eligibility"></a>4. Resolve live visibility and selection eligibility

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

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
        ids.retain(|id| !self.forest.readonly(*id));
        ids.sort_unstable_by_key(|id| id.value());
```

Replace it with:

```rust
    }

    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        let mut ids = self
            .objects
            .iter()
            .map(|r| r.id)
            .chain(self.lines.iter().map(|r| r.id))
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
            .collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
```

**Locate this block:**

```rust
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && row.visible())
        {
            for control in row.prepared.source.controls() {
                let p = control
```

Replace it with:

```rust
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && self.visible(row.id))
        {
            for control in row.prepared.source.controls() {
                let p = control
```

**Locate this block:**

```rust
        for row in self
            .points
            .iter()
            .filter(|row| row.visible())
        {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
```

Replace it with:

```rust
        for row in self
            .points
            .iter()
            .filter(|row| self.visible(row.id))
        {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
```

**Locate this block:**

```rust
                }
            }
        }
        for row in self.paths.iter().filter(|row| row.visible()) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
        for line in self
            .lines
            .iter()
            .filter(|line| line.visible())
        {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
```

Replace it with:

```rust
        for line in self
            .lines
            .iter()
            .filter(|line| self.visible(line.id))
        {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
```

**Locate this block:**

```rust
    }

    pub fn visible(&self, id: ObjectId) -> bool {
        if let Some(row) = self.objects.iter().find(|row| row.id == id) {
            return row.metadata.is_visible;
        }
        if let Some(row) = self.lines.iter().find(|row| row.id == id) {
            return row.prepared.source.is_visible;
        }
        if let Some(row) = self.paths.iter().find(|row| row.id == id) {
            return row.prepared.source.visible();
        }
        self.points
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.prepared.source.is_visible)
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        self.objects
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
            .find(|id| id.value() == value && !self.forest.readonly(*id))
    }

    pub fn demo() -> Self {
```

Replace it with:

```rust
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        let id = ObjectId(value);
        (self.visible(id)
            && !self.view.locked.contains(&id)
            && !self.forest.readonly(id))
        .then_some(id)
    }

    pub fn demo() -> Self {
```

### 2. `src/multi_selection_tests.rs`

**Locate this existing block:**

```rust
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    Rc::make_mut(&mut editor.scene.objects_mut()[0].metadata).is_locked = true;
    Rc::make_mut(&mut editor.scene.objects_mut()[1].metadata).is_visible = false;
    editor.apply(Action::SelectAll).unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let locked = editor.scene.objects()[0].id;
    let hidden = editor.scene.objects()[1].id;
    Rc::make_mut(&mut editor.scene.view).lock(&[locked], true);
    Rc::make_mut(&mut editor.scene.view).hide(&[hidden], true);
    editor.apply(Action::SelectAll).unwrap();
```

### 3. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    );
    let mut locked = scene.objects[1].metadata.as_ref().clone();
    locked.is_locked = true;
    scene.objects[1].metadata = Rc::new(locked);
    renderer.set_scene(&scene, None);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    Rc::make_mut(&mut scene.view).lock(&[front], true);
    renderer.set_scene(&scene, None);
```

## <a id="checkpoint-53c-presentation"></a>5. Use current visibility and colours in every derived pass

### 1. `src/renderer.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.view.faces.get(&row.id) {
                            marker.colour = rgba(*colour);
                        }
                        marker
                    })
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
                    .filter(|row| scene.visible(row.id))
                    .flat_map(|row| {
                        let mut segments = row.segments();
                        if let Some(colour) = scene.view.faces.get(&row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = rgba(*colour);
                            }
                        }
                        segments
                    }),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut stroke = row.stroke();
                        if let Some(colour) = scene.view.faces.get(&row.id) {
                            stroke.colour = rgba(*colour);
                        }
                        stroke
                    }),
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
            .filter(|row| scene.visible(row.id))
        {
```

### 2. `src/highlight.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .iter()
                .filter(|r| ids.contains(&r.id) && scene.visible(r.id))
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
                .filter(|r| ids.contains(&r.id) && scene.visible(r.id))
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
                .filter(|r| ids.contains(&r.id) && scene.visible(r.id))
                .map(|r| crate::marker::Marker {
```

### 3. `src/pick_sources.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for row in scene
            .points()
            .iter()
            .filter(|row| row.visible())
        {
```

Replace it with:

```rust
        for row in scene
            .points()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
```

**Locate this block:**

```rust
        for row in scene
            .paths()
            .iter()
            .filter(|row| row.visible())
        {
            let points: Vec<_> = row
                .points()
```

Replace it with:

```rust
        for row in scene
            .paths()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let points: Vec<_> = row
                .points()
```

**Locate this block:**

```rust
        for row in scene
            .lines()
            .iter()
            .filter(|row| row.visible())
        {
            let ids = append(
                &mut sources.edges,
```

Replace it with:

```rust
        for row in scene
            .lines()
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            let ids = append(
                &mut sources.edges,
```

**Locate this block:**

```rust
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
                    end: ends[1],
                    colour: [0., 0., 0., 1.],
                    width: 1.,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
```

Replace it with:

```rust
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
                    end: ends[1],
                    colour: scene.view.edges.get(&row.id).map_or(
                        [0., 0., 0., 1.],
                        |rgb| {
                            [
                                rgb[0] as f32 / 255.,
                                rgb[1] as f32 / 255.,
                                rgb[2] as f32 / 255.,
                                1.,
                            ]
                        },
                    ),
                    width: 1.,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
```

**Locate this block:**

```rust
            .iter()
            .filter(|row| row.visible())
        {
            sources.mesh_bases.push((
                row.id,
```

Replace it with:

```rust
            .iter()
            .filter(|row| scene.visible(row.id))
        {
            sources.mesh_bases.push((
                row.id,
```

### 4. `src/snap_sources.rs`

**Locate this existing block:**

```rust
    let mut out = Vec::new();
    for row in scene.points().iter().filter(|r| r.visible()) {
        let p = row.point();
        point(
            &mut out,
            row.id,
            [p[0], p[1], p[2]],
            Kind::End,
        );
    }
    for row in scene.lines().iter().filter(|r| r.visible()) {
        let ends = row.endpoints().map(|p| [p[0], p[1], p[2]]);
        polyline(&mut out, row.id, &ends);
    }
    for row in scene.paths().iter().filter(|r| r.visible()) {
        let points: Vec<_> = row
            .points()
            .iter()
            .map(|p| [p[0], p[1], p[2]])
            .collect();
        if let crate::chain::Source::Curve(curve, _) = &row.prepared.source {
            let (a, b) = curve.domain();
            for (t, kind) in [
                (a, Kind::End),
                (b, Kind::End),
                ((a + b) * 0.5, Kind::Mid),
            ] {
                let p = row.model.transform_point(&curve.point_at(t));
                point(&mut out, row.id, [p[0], p[1], p[2]], kind);
            }
            wires(&mut out, row.id, &points);
        } else {
            polyline(&mut out, row.id, &points);
        }
        if points.len() >= 4
            && (0..3)
                .map(|i| (points[0][i] - points[points.len() - 1][i]).powi(2))
                .sum::<f64>()
                .sqrt()
                <= 1e-9
        {
            let corners = &points[..points.len() - 1];
            let center = std::array::from_fn(|i| {
                corners.iter().map(|p| p[i]).sum::<f64>() / corners.len() as f64
            });
            point(&mut out, row.id, center, Kind::Center);
        }
    }
    for row in scene
        .objects()
        .iter()
        .filter(|r| r.visible())
    {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut out = Vec::new();
    for row in scene
        .points()
        .iter()
        .filter(|r| scene.visible(r.id))
    {
        let p = row.point();
        point(
            &mut out,
            row.id,
            [p[0], p[1], p[2]],
            Kind::End,
        );
    }
    for row in scene
        .lines()
        .iter()
        .filter(|r| scene.visible(r.id))
    {
        let ends = row.endpoints().map(|p| [p[0], p[1], p[2]]);
        polyline(&mut out, row.id, &ends);
    }
    for row in scene
        .paths()
        .iter()
        .filter(|r| scene.visible(r.id))
    {
        let points: Vec<_> = row
            .points()
            .iter()
            .map(|p| [p[0], p[1], p[2]])
            .collect();
        if let crate::chain::Source::Curve(curve, _) = &row.prepared.source {
            let (a, b) = curve.domain();
            for (t, kind) in [
                (a, Kind::End),
                (b, Kind::End),
                ((a + b) * 0.5, Kind::Mid),
            ] {
                let p = row.model.transform_point(&curve.point_at(t));
                point(&mut out, row.id, [p[0], p[1], p[2]], kind);
            }
            wires(&mut out, row.id, &points);
        } else {
            polyline(&mut out, row.id, &points);
        }
        if points.len() >= 4
            && (0..3)
                .map(|i| (points[0][i] - points[points.len() - 1][i]).powi(2))
                .sum::<f64>()
                .sqrt()
                <= 1e-9
        {
            let corners = &points[..points.len() - 1];
            let center = std::array::from_fn(|i| {
                corners.iter().map(|p| p[i]).sum::<f64>() / corners.len() as f64
            });
            point(&mut out, row.id, center, Kind::Center);
        }
    }
    for row in scene
        .objects()
        .iter()
        .filter(|r| scene.visible(r.id))
    {
```

### 5. `src/picking.rs`

**Locate this existing block:**

```rust
        .iter()
        .filter(|row| row.visible())
    {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .iter()
        .filter(|row| scene.visible(row.id))
    {
```

### 6. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(scene.pick_id(front.value()), None);
    let mut hidden = scene.objects[1].metadata.as_ref().clone();
    hidden.is_visible = false;
    scene.objects[1].metadata = Rc::new(hidden);
    renderer.set_scene(&scene, None);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(scene.pick_id(front.value()), None);
    Rc::make_mut(&mut scene.view).hide(&[front], true);
    renderer.set_scene(&scene, None);
```

## <a id="checkpoint-53d-hide-show"></a>6. Connect Hide Show and canvas Delete shortcuts

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::TreePick { ids, additive } => {
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::Hide => {
                let ids = self
                    .scene
                    .with_descendants(&self.selected_objects());
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids, true);
                self.select_objects([], false);
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Show => {
                std::rc::Rc::make_mut(&mut self.scene.view).show_all();
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::TreePick { ids, additive } => {
```

**Locate this block:**

```rust
    UnloadSources,
    AddBox,
    AddNested,
    AddLine,
```

Replace it with:

```rust
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddLine,
```

### 2. `src/history.rs`

**Locate this existing block:**

```rust
        to.push(scene.clone());
        scene.restore(saved);
        true
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        to.push(scene.clone());
        scene.restore_view(saved);
        true
```

### 3. `src/commands.rs`

**Locate this existing block:**

```rust
    }),
    ("Example Box", || Action::AddBox),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }),
    ("Hide", || Action::Hide),
    ("Show", || Action::Show),
    ("Example Box", || Action::AddBox),
```

### 4. `src/shortcuts.rs`

**Locate this existing block:**

```rust
    Fit,
}

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Fit => "Fit",
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
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
            "y" => Some(Shortcut::Redo),
            _ => None,
        }
    } else if !focused && text.eq_ignore_ascii_case("f") {
        Some(Shortcut::Fit)
    } else {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Fit,
    Hide,
    Show,
    Delete,
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
        }
    } else {
```

### 5. `src/shortcuts_tests.rs`

**Locate this existing block:**

```rust
        key("h", false, false, false, false, false, true),
        None
    );
    assert_eq!(
        key("s", false, false, false, false, false, true),
        None
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        key("h", false, false, false, false, false, true),
        Some(Shortcut::Hide)
    );
    assert_eq!(
        key("s", false, false, false, false, false, true),
        Some(Shortcut::Show)
    );
}

#[test]
fn canvas_delete_hide_show_keep_focused_text_and_modifiers_owned_by_the_field() {
    for (text, shortcut) in [
        ("Delete", Shortcut::Delete),
        ("H", Shortcut::Hide),
        ("S", Shortcut::Show),
    ] {
        assert_eq!(
            key(text, false, false, false, false, false, true),
            Some(shortcut)
        );
        for (command, alt, composing, focused) in [
            (true, false, false, false),
            (false, true, false, false),
            (false, false, true, false),
            (false, false, false, true),
        ] {
            assert_eq!(
                key(
                    text, command, false, alt, composing, focused, true
                ),
                None
            );
        }
    }
}
```

## <a id="checkpoint-53e-properties"></a>7. Apply eye lock and independent colour properties

### 1. `src/layer_properties.rs`

Create the file and type:

```rust
use crate::{hierarchy, scene::Scene};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Property {
    Hidden(bool),
    Locked(bool),
    Colour {
        edge: bool,
        rgb: Option<[u8; 3]>,
    },
}
pub fn apply(
    scene: &mut Scene,
    key: &str,
    property: Property,
) -> Result<(), &'static str> {
    let rows = hierarchy::rows(scene);
    let index = rows
        .iter()
        .position(|row| row.key == key)
        .ok_or("This layer row is gone")?;
    let row = &rows[index];
    if matches!(property, Property::Hidden(true))
        && scene
            .view
            .current
            .as_ref()
            .is_some_and(|key| {
                rows[index..row.end]
                    .iter()
                    .any(|row| &row.key == key)
            })
    {
        return Err("The current layer cannot be hidden");
    }
    let view = Rc::make_mut(&mut scene.view);
    match property {
        Property::Hidden(on) => {
            let group = scene.forest.documents.iter().any(|doc| {
                doc.nodes.iter().any(|node| {
                    node.object.is_none()
                        && format!("{}/{}", doc.key, node.source.guid) == key
                })
            });
            if group {
                if on {
                    view.lamps.insert(key.to_owned());
                } else {
                    view.lamps.remove(key);
                }
            } else {
                view.hide(&row.ids, on);
            }
            view.refresh(&scene.forest);
        }
        Property::Locked(on) => view.lock(&row.ids, on),
        Property::Colour { edge, rgb } => {
            let colours = if edge {
                &mut view.edges
            } else {
                &mut view.faces
            };
            for id in &row.ids {
                if let Some(rgb) = rgb {
                    colours.insert(*id, rgb);
                } else {
                    colours.remove(id);
                }
            }
        }
    }
    Ok(())
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod hierarchy_tests;

pub mod view_flags;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod hierarchy_tests;

pub mod view_flags;

pub mod layer_properties;
```

### 3. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::Hide => {
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::LayerProperty { key, property } => {
                crate::layer_properties::apply(&mut self.scene, &key, property)?;
                self.repair_selection();
                if self
                    .selected
                    .is_some_and(|id| self.scene.pick_id(id.value()).is_none())
                {
                    self.selected = None;
                    self.selected_part = None;
                }
                self.snap_index = None;
                return Ok(Change::Preview);
            }
            Action::Hide => {
```

**Locate this block:**

```rust
pub enum Action {
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
```

Replace it with:

```rust
pub enum Action {
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
    },
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
```

### 4. `src/hierarchy.rs`

**Locate this existing block:**

```rust
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn colours(&self, key: u32) -> HashMap<ObjectId, [u8; 3]> {
        let mut out = HashMap::new();
        let Some(doc) = self
            .documents
            .iter()
            .find(|doc| doc.key == key)
        else {
            return out;
        };
        let mut inherited = vec![None; doc.nodes.len()];
        for (index, node) in doc.nodes.iter().enumerate() {
            let colour = node
                .source
                .color
                .as_ref()
                .map(|colour| {
                    [colour.r, colour.g, colour.b]
                        .map(|value| (value.clamp(0., 1.) * 255.).round() as u8)
                })
                .or_else(|| {
                    node.parent
                        .and_then(|parent| inherited[parent])
                });
            inherited[index] = colour;
            if let (Some(id), Some(colour)) = (node.object, colour) {
                out.insert(id, colour);
            }
        }
        out
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
```

### 5. `src/scene.rs`

**Locate this existing block:**

```rust
        );
        Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
        let colours = self.forest.colours(key);
        Rc::make_mut(&mut self.view)
            .faces
            .extend(colours);
        Ok(())
```

## <a id="checkpoint-53f-portable"></a>8. Keep tree identities stable across edits and saves

### 1. `src/hierarchy.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
) -> Result<Option<proto::TreeNode>, &'static str> {
    let original = &doc.nodes[index];
    let mut node = original.source.clone();
    node.guid = uuid::Uuid::new_v4().to_string();
    node.parent_guid = parent_guid.to_owned();
```

Replace it with:

```rust
) -> Result<Option<proto::TreeNode>, &'static str> {
    let original = &doc.nodes[index];
    let mut node = original.source.clone();
    node.guid = original.portable_guid.clone();
    node.parent_guid = parent_guid.to_owned();
```

**Locate this block:**

```rust
        out
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.readonly_ids.contains(&id)
    }
```

Replace it with:

```rust
        out
    }

    pub(crate) fn replace(&mut self, index: usize, document: Document) {
        self.documents[index] = Rc::new(document);
        self.readonly_ids = Rc::new(
            self.documents
                .iter()
                .flat_map(|doc| doc.nodes.iter())
                .filter(|node| node.readonly)
                .filter_map(|node| node.object)
                .collect(),
        );
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.readonly_ids.contains(&id)
    }
```

**Locate this block:**

```rust
                children: Vec::new(),
            };
            nodes.push(Node {
                source,
                object: object.map(|v| v.0),
                label: object.map_or_else(|| node.name.clone(), |v| v.1.to_owned()),
```

Replace it with:

```rust
                children: Vec::new(),
            };
            nodes.push(Node {
                portable_guid: uuid::Uuid::new_v4().to_string(),
                source,
                object: object.map(|v| v.0),
                label: object.map_or_else(|| node.name.clone(), |v| v.1.to_owned()),
```

**Locate this block:**

```rust
    pub end: usize,
    pub readonly: bool,
}
pub struct Document {
    pub key: u32,
    pub label: String,
```

Replace it with:

```rust
    pub end: usize,
    pub readonly: bool,
}
#[derive(Clone)]
pub struct Document {
    pub key: u32,
    pub label: String,
```

**Locate this block:**

```rust
}

pub struct Node {
    pub source: proto::TreeNode,
    pub object: Option<ObjectId>,
    pub label: String,
```

Replace it with:

```rust
}

#[derive(Clone)]
pub struct Node {
    pub portable_guid: String,
    pub source: proto::TreeNode,
    pub object: Option<ObjectId>,
    pub label: String,
```

## <a id="checkpoint-53g-branches"></a>9. Represent group edits with temporary branches

### 1. `src/layer_edit.rs`

Create the file and type:

```rust
use crate::{
    hierarchy::{Document, Node},
    scene::Scene,
};
use std::rc::Rc;

pub enum Edit {
    Current,
    New { sublayer: bool },
    Rename(String),
    Delete,
    Duplicate,
    Move(Vec<crate::scene::ObjectId>),
    Copy(Vec<crate::scene::ObjectId>),
}

// Temporary branches make subtree edits atomic while the saved scene continues sharing its original source and display allocations.
#[derive(Clone)]
struct Branch {
    node: Node,
    children: Vec<Branch>,
}

fn branch(doc: &Document, index: usize) -> Branch {
    Branch {
        node: doc.nodes[index].clone(),
        children: (index + 1..doc.nodes[index].end)
            .filter(|child| doc.nodes[*child].parent == Some(index))
            .map(|child| branch(doc, child))
            .collect(),
    }
}

fn find<'a>(root: &'a mut Branch, guid: &str) -> Option<&'a mut Branch> {
    if root.node.source.guid == guid {
        return Some(root);
    }
    root.children
        .iter_mut()
        .find_map(|child| find(child, guid))
}

fn flatten(root: Branch, parent: Option<usize>, depth: usize, nodes: &mut Vec<Node>) {
    let index = nodes.len();
    let mut node = root.node;
    node.parent = parent;
    node.depth = depth;
    node.source.parent_guid = parent.map_or_else(String::new, |parent| {
        nodes[parent].source.guid.clone()
    });
    nodes.push(node);
    for child in root.children {
        flatten(child, Some(index), depth + 1, nodes);
    }
    nodes[index].end = nodes.len();
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod view_flags;

pub mod layer_properties;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod view_flags;

pub mod layer_properties;

pub mod layer_edit;
```

## <a id="checkpoint-53h-layer-targets"></a>10. Validate current layers and protected ancestors

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.ids = Default::default();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.created = None;
        self.ids = Default::default();
```

**Locate this block:**

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
```

Replace it with:

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
```

**Locate this block:**

```rust
pub struct Scene {
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
```

Replace it with:

```rust
pub struct Scene {
    pub(crate) created: Option<u32>,
    ids: Rc<std::collections::HashSet<ObjectId>>,
    pub view: Rc<crate::view_flags::Flags>,
    pub forest: crate::hierarchy::Forest,
```

### 2. `src/layer_edit.rs`

**Locate this existing block:**

```rust
    nodes[index].end = nodes.len();
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    nodes[index].end = nodes.len();
}

pub fn locate(scene: &Scene, key: &str) -> Result<(usize, usize), &'static str> {
    locate_group(scene, key, true)
}

fn locate_group(
    scene: &Scene,
    key: &str,
    editable: bool,
) -> Result<(usize, usize), &'static str> {
    for (owner, doc) in scene.forest.documents.iter().enumerate() {
        for (index, node) in doc.nodes.iter().enumerate() {
            if format!("{}/{}", doc.key, node.source.guid) != key {
                continue;
            }
            let mut ancestor = Some(index);
            while let Some(index) = ancestor {
                if doc.nodes[index].object.is_some() {
                    return Err("Objects and their attributes are not layers");
                }
                ancestor = doc.nodes[index].parent;
            }
            if editable
                && doc
                    .nodes
                    .iter()
                    .filter_map(|node| node.object)
                    .any(|id| {
                        scene
                            .objects()
                            .iter()
                            .any(|row| row.id == id && row.geometry().is_none())
                    })
            {
                return Err("Reload editable sources before editing layers");
            }
            return Ok((owner, index));
        }
    }
    Err("This layer is gone")
}

pub fn holds_current(scene: &Scene, key: &str) -> bool {
    let Some(current) = current_key(scene) else {
        return false;
    };
    let rows = crate::hierarchy::rows(scene);
    rows.iter()
        .position(|row| row.key == key)
        .is_some_and(|index| {
            rows[index..rows[index].end]
                .iter()
                .any(|row| row.key == current)
        })
}

pub fn current_key(scene: &Scene) -> Option<String> {
    if let Some(key) = scene.view.current.as_ref()
        && locate_group(scene, key, false).is_ok()
    {
        return Some(key.clone());
    }
    let doc = scene
        .forest
        .documents
        .iter()
        .find(|doc| Some(doc.key) == scene.created)?;
    Some(format!(
        "{}/{}",
        doc.key, doc.nodes[0].source.guid
    ))
}

fn replace(scene: &mut Scene, owner: usize, tree: Branch) {
    let old = &scene.forest.documents[owner];
    let mut document = Document {
        key: old.key,
        label: old.label.clone(),
        nodes: Vec::new(),
    };
    flatten(tree, None, 0, &mut document.nodes);
    scene.forest.replace(owner, document);
    Rc::make_mut(&mut scene.view).refresh(&scene.forest);
}
```

### 3. `src/layer_properties.rs`

**Locate this existing block:**

```rust
    if matches!(property, Property::Hidden(true))
        && scene
            .view
            .current
            .as_ref()
            .is_some_and(|key| {
                rows[index..row.end]
                    .iter()
                    .any(|row| &row.key == key)
            })
    {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if matches!(property, Property::Hidden(true))
        && crate::layer_edit::holds_current(scene, key)
    {
```

## <a id="checkpoint-53i-copy-records"></a>11. Copy row identities while sharing source allocations

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
    }

    pub fn clear(&mut self) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
                    .map(|row| row.prepared.source.guid())
            })
    }

    pub fn clear(&mut self) {
```

## <a id="checkpoint-53j-transfers"></a>12. Move and copy objects between layer trees

### 1. `src/layer_edit.rs`

**Locate this existing block:**

```rust
    Rc::make_mut(&mut scene.view).refresh(&scene.forest);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Rc::make_mut(&mut scene.view).refresh(&scene.forest);
}

fn take(root: &mut Branch, id: crate::scene::ObjectId) -> Option<Branch> {
    if let Some(index) = root
        .children
        .iter()
        .position(|child| child.node.object == Some(id))
    {
        return Some(root.children.remove(index));
    }
    root.children
        .iter_mut()
        .find_map(|child| take(child, id))
}

fn copy_branch(
    scene: &mut Scene,
    mut tree: Branch,
    all: bool,
) -> Result<Branch, &'static str> {
    tree.node.source.guid = uuid::Uuid::new_v4().to_string();
    tree.node.portable_guid = tree.node.source.guid.clone();
    if let Some(id) = tree.node.object {
        let copy = scene.copy_row(id, all)?;
        tree.node.object = Some(copy);
        tree.node.source.name = scene.row_guid(copy).unwrap().to_owned();
    }
    tree.children = tree
        .children
        .into_iter()
        .map(|child| copy_branch(scene, child, all))
        .collect::<Result<_, _>>()?;
    Ok(tree)
}

fn transfer(
    scene: &mut Scene,
    key: &str,
    ids: Vec<crate::scene::ObjectId>,
    copy: bool,
) -> Result<(), &'static str> {
    let (owner, index) = locate(scene, key)?;
    let target = scene.forest.documents[owner].nodes[index]
        .source
        .guid
        .clone();
    if ids.is_empty() {
        return Err("Select objects first");
    }
    if ids
        .iter()
        .any(|id| scene.pick_id(id.value()) != Some(*id))
    {
        return Err("Choose visible unlocked editable objects");
    }
    let chosen = ids
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    let mut trees = scene
        .forest
        .documents
        .iter()
        .map(|doc| branch(doc, 0))
        .collect::<Vec<_>>();
    let mut moved = Vec::new();
    for (doc_index, doc) in scene.forest.documents.iter().enumerate() {
        for (index, node) in doc.nodes.iter().enumerate() {
            let Some(id) = node.object.filter(|id| chosen.contains(id)) else {
                continue;
            };
            let mut parent = node.parent;
            let mut follows = false;
            while let Some(index) = parent {
                follows |= doc.nodes[index]
                    .object
                    .is_some_and(|id| chosen.contains(&id));
                parent = doc.nodes[index].parent;
            }
            if follows && !copy {
                continue;
            }
            if !copy
                && doc_index == owner
                && node
                    .parent
                    .is_some_and(|parent| doc.nodes[parent].source.guid == target)
            {
                continue;
            }
            let mut tree = if copy {
                branch(doc, index)
            } else {
                take(&mut trees[doc_index], id).ok_or("Object tree is gone")?
            };
            if copy {
                tree.children.clear();
            }
            moved.push(tree);
        }
    }
    if moved.is_empty() {
        return Err("The objects are already on this layer");
    }
    let mut target_nodes = Vec::new();
    flatten(
        trees[owner].clone(),
        None,
        0,
        &mut target_nodes,
    );
    let count = target_nodes.len()
        + moved
            .iter()
            .map(|tree| {
                let mut nodes = Vec::new();
                flatten(tree.clone(), None, 0, &mut nodes);
                nodes.len()
            })
            .sum::<usize>();
    if count > crate::hierarchy::MAX_NODES {
        return Err("Target layer exceeds its node limit");
    }
    for tree in moved {
        let tree = if copy {
            copy_branch(scene, tree, false)?
        } else {
            tree
        };
        find(&mut trees[owner], &target)
            .unwrap()
            .children
            .push(tree);
    }
    for (owner, tree) in trees.into_iter().enumerate() {
        let mut nodes = Vec::new();
        flatten(tree.clone(), None, 0, &mut nodes);
        if nodes
            .iter()
            .any(|node| node.depth > crate::hierarchy::MAX_DEPTH)
        {
            return Err("Target layer exceeds its depth limit");
        }
        replace(scene, owner, tree);
    }
    Ok(())
}
```

## <a id="checkpoint-53k-new-rename"></a>13. Create and rename uniquely named layers

### 1. `src/layer_edit.rs`

**Locate this existing block:**

```rust
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(())
}

fn new_layer(scene: &mut Scene, key: &str, sublayer: bool) -> Result<(), &'static str> {
    let (owner, index) = locate(scene, key)?;
    let doc = &scene.forest.documents[owner];
    let node = &doc.nodes[index];
    if doc.nodes.len() >= crate::hierarchy::MAX_NODES {
        return Err("Tree exceeds its node limit");
    }
    let parent = if sublayer {
        index
    } else {
        node.parent.unwrap_or(index)
    };
    if doc.nodes[parent].depth >= crate::hierarchy::MAX_DEPTH {
        return Err("Tree exceeds its depth limit");
    }
    let label = (1..)
        .map(|i| format!("Layer {i:02}"))
        .find(|name| {
            !doc.nodes
                .iter()
                .any(|node| node.label == *name || node.source.name == *name)
        })
        .unwrap();
    let source = session_rust::proto::TreeNode {
        guid: uuid::Uuid::new_v4().to_string(),
        name: label.clone(),
        ..Default::default()
    };
    let child = Branch {
        node: Node {
            portable_guid: source.guid.clone(),
            source,
            object: None,
            label,
            parent: None,
            depth: 0,
            end: 1,
            readonly: false,
        },
        children: Vec::new(),
    };
    let mut tree = branch(doc, 0);
    find(&mut tree, &doc.nodes[parent].source.guid)
        .unwrap()
        .children
        .push(child);
    replace(scene, owner, tree);
    Ok(())
}

fn rename(scene: &mut Scene, key: &str, label: &str) -> Result<(), &'static str> {
    let (owner, index) = locate(scene, key)?;
    let doc = &scene.forest.documents[owner];
    let node = &doc.nodes[index];
    if node.parent.is_none() {
        return Err("The root layer cannot be renamed");
    }
    let label = label.trim();
    if label.is_empty() || label.contains('/') {
        return Err("A layer name must be non-empty and without /");
    }
    if label == node.label {
        return Err("The layer already has that name");
    }
    if doc
        .nodes
        .iter()
        .any(|other| other.label == label || other.source.name == label)
    {
        return Err("This name is already used in the document");
    }
    let mut tree = branch(doc, 0);
    let node = &mut find(&mut tree, &node.source.guid)
        .unwrap()
        .node;
    node.label = label.to_owned();
    node.source.name = label.to_owned();
    replace(scene, owner, tree);
    Ok(())
}
```

## <a id="checkpoint-53l-delete-duplicate"></a>14. Delete and duplicate complete layer contents

### 1. `src/layer_edit.rs`

**Locate this existing block:**

```rust
    node.source.name = label.to_owned();
    replace(scene, owner, tree);
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    node.source.name = label.to_owned();
    replace(scene, owner, tree);
    Ok(())
}

fn delete(scene: &mut Scene, key: &str) -> Result<(), &'static str> {
    let (owner, index) = locate(scene, key)?;
    let doc = &scene.forest.documents[owner];
    let node = &doc.nodes[index];
    let parent = node
        .parent
        .ok_or("The root layer cannot be deleted")?;
    if holds_current(scene, key) {
        return Err("The current layer cannot be deleted");
    }
    let ids = doc.nodes[index..node.end]
        .iter()
        .filter_map(|node| node.object)
        .collect::<Vec<_>>();
    let mut tree = branch(doc, 0);
    find(&mut tree, &doc.nodes[parent].source.guid)
        .unwrap()
        .children
        .retain(|child| child.node.source.guid != node.source.guid);
    for id in ids {
        scene.remove(id);
    }
    replace(scene, owner, tree);
    Ok(())
}

fn duplicate(scene: &mut Scene, key: &str) -> Result<(), &'static str> {
    let (owner, index) = locate(scene, key)?;
    let doc = &scene.forest.documents[owner];
    let node = &doc.nodes[index];
    let parent = node
        .parent
        .ok_or("The root layer cannot be duplicated")?;
    if doc.nodes.len() + node.end - index > crate::hierarchy::MAX_NODES {
        return Err("Tree exceeds its node limit");
    }
    let label = (1..)
        .map(|i| format!("{} copy {i:02}", node.label))
        .find(|name| {
            !doc.nodes
                .iter()
                .any(|node| node.label == *name || node.source.name == *name)
        })
        .unwrap();
    let parent = doc.nodes[parent].source.guid.clone();
    let source = branch(doc, index);
    let mut tree = branch(doc, 0);
    let mut copy = copy_branch(scene, source, true)?;
    copy.node.label = label.clone();
    copy.node.source.name = label;
    find(&mut tree, &parent)
        .unwrap()
        .children
        .push(copy);
    replace(scene, owner, tree);
    Ok(())
}
```

## <a id="checkpoint-53m-actions"></a>15. Commit group edits through one history boundary

### 1. `src/layer_edit.rs`

**Locate this existing block:**

```rust
        .push(copy);
    replace(scene, owner, tree);
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .push(copy);
    replace(scene, owner, tree);
    Ok(())
}

pub fn apply(scene: &mut Scene, key: &str, edit: Edit) -> Result<(), &'static str> {
    match edit {
        Edit::Current => {
            locate(scene, key)?;
            Rc::make_mut(&mut scene.view).current = Some(key.to_owned());
            Ok(())
        }
        Edit::New { sublayer } => new_layer(scene, key, sublayer),
        Edit::Rename(label) => rename(scene, key, &label),
        Edit::Delete => delete(scene, key),
        Edit::Duplicate => duplicate(scene, key),
        Edit::Move(ids) => transfer(scene, key, ids, false),
        Edit::Copy(ids) => transfer(scene, key, ids, true),
    }
}
```

### 2. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::LayerProperty { key, property } => {
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::LayerEdit { key, edit } => {
                if matches!(edit, crate::layer_edit::Edit::Current) {
                    crate::layer_edit::apply(&mut self.scene, &key, edit)?;
                    return Ok(Change::View);
                }
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        crate::layer_edit::apply(scene, &key, edit)
                    })?;
                self.repair_selection();
            }
            Action::LayerProperty { key, property } => {
```

**Locate this block:**

```rust
pub enum Action {
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
```

Replace it with:

```rust
pub enum Action {
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
    },
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
```

## <a id="checkpoint-53n-routing"></a>16. Create geometry on the current layer or Created

### 1. `src/layer_edit.rs`

**Locate this existing block:**

```rust
}

fn take(root: &mut Branch, id: crate::scene::ObjectId) -> Option<Branch> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn attach_generated(
    scene: &mut Scene,
    id: crate::scene::ObjectId,
    guid: String,
    label: String,
) -> Result<(), &'static str> {
    if current_key(scene).is_none() {
        scene
            .forest
            .add(id.value(), "Created".into(), None, &[]);
        scene.created = Some(id.value());
    }
    let key = current_key(scene).unwrap();
    let (owner, index) = locate(scene, &key)?;
    let doc = &scene.forest.documents[owner];
    if doc.nodes.len() >= crate::hierarchy::MAX_NODES
        || doc.nodes[index].depth >= crate::hierarchy::MAX_DEPTH
    {
        return Err("Current layer exceeds its tree limit");
    }
    let parent = doc.nodes[index].source.guid.clone();
    let mut tree = branch(doc, 0);
    let source = session_rust::proto::TreeNode {
        guid: uuid::Uuid::new_v4().to_string(),
        name: guid,
        ..Default::default()
    };
    find(&mut tree, &parent)
        .unwrap()
        .children
        .push(Branch {
            node: Node {
                portable_guid: source.guid.clone(),
                source,
                object: Some(id),
                label,
                parent: None,
                depth: 0,
                end: 1,
                readonly: false,
            },
            children: Vec::new(),
        });
    replace(scene, owner, tree);
    Ok(())
}

fn take(root: &mut Branch, id: crate::scene::ObjectId) -> Option<Branch> {
```

### 2. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            model: prepared.model,
            editable,
        });
        Ok(id)
```

Replace it with:

```rust
            model: prepared.model,
            editable,
        });
        if generated {
            crate::layer_edit::attach_generated(self, id, node_guid, label)?;
        }
        Ok(id)
```

**Locate this block:**

```rust
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        let editable = crate::edit_source::EditSource::Loaded {
            geometry: prepared.geometry,
            source: prepared.source,
        };
        self.objects.push(Object {
            id,
            guid,
```

Replace it with:

```rust
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
            id,
            guid,
```

**Locate this block:**

```rust
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }
```

Replace it with:

```rust
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let guid = prepared.source.guid().to_owned();
        let label = prepared.source.name.clone();
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }
```

**Locate this block:**

```rust
            prepared.source.visible(),
            prepared.source.locked(),
        );
        self.paths.push(PathObject {
            controls_visible: false,
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_path(
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
        );
        self.points.push(PointObject {
            id,
            guid,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }
```

Replace it with:

```rust
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
```

### 3. `src/hierarchy.rs`

**Locate this existing block:**

```rust
        );
    for (id, label) in other {
        let index = rows.len();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
    for (id, label) in other.filter(|(id, _)| !placed.contains(id)) {
        let index = rows.len();
```

### 4. `src/snapshot_tests.rs`

**Locate this existing block:**

```rust
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
    assert_eq!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    let root = message
        .tree
        .as_ref()
        .unwrap()
        .root
        .as_ref()
        .unwrap();
    assert_eq!(root.children.len(), 1);
    let created = &root.children[0];
    assert_eq!(created.name, "Created");
    assert_eq!(created.children.len(), meshes.len());
    assert!(created.children.iter().all(|node| {
        meshes
            .iter()
            .any(|mesh| mesh.guid == node.name)
    }));
    assert_eq!(
```

## <a id="checkpoint-53o-palette"></a>17. Supply nine colour swatches and RGB sliders

### 1. `src/layer_palette.rs`

Create the file and type:

```rust
#[cfg(any(test, target_arch = "wasm32"))]
pub fn draw(
    ui: &mut egui::Ui,
    key: &str,
    edge: bool,
    colour: Option<[u8; 3]>,
    controls: &mut Option<Vec<crate::command_dock::Control>>,
) -> Option<[u8; 3]> {
    use crate::command_dock::record;
    let mut answer = None;
    ui.separator();
    for colours in [
        [
            ("Red", [230, 65, 55]),
            ("Orange", [240, 145, 45]),
            ("Yellow", [240, 210, 60]),
        ],
        [
            ("Green", [60, 170, 100]),
            ("Blue", [65, 130, 225]),
            ("Violet", [160, 85, 210]),
        ],
        [
            ("White", [245, 245, 245]),
            ("Gray", [150, 150, 150]),
            ("Black", [35, 35, 35]),
        ],
    ] {
        ui.horizontal(|ui| {
            for (name, rgb) in colours {
                let button = ui
                    .add_sized(
                        [48., 28.],
                        egui::Button::new(egui::RichText::new("■").color(
                            egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]),
                        )),
                    )
                    .on_hover_text(name);
                record(
                    controls,
                    &format!("layer/colour/{key}/{edge}/{name}"),
                    name,
                    &button,
                );
                if button.clicked() {
                    answer = Some(rgb);
                    ui.close();
                }
            }
        });
    }
    ui.separator();
    let mut rgb = colour.unwrap_or([180; 3]);
    let mut changed = false;
    for (name, value) in ["R", "G", "B"]
        .into_iter()
        .zip(rgb.iter_mut())
    {
        let slider = ui.add(egui::Slider::new(value, 0..=255).text(name));
        record(
            controls,
            &format!("layer/rgb/{key}/{edge}/{name}"),
            name,
            &slider,
        );
        changed |= slider.changed();
    }
    if changed {
        answer = Some(rgb);
    }
    answer
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod layer_properties;

pub mod layer_edit;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod layer_properties;

pub mod layer_edit;

pub mod layer_palette;
```

## <a id="checkpoint-53p-panel-state"></a>18. Resolve layer presentation for real tree controls

### 1. `src/tree_panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    });
    model.rect = model.open.then_some(response.response.rect);
}
```

Replace it with:

```rust
    });
    model.rect = model.open.then_some(response.response.rect);
}
#[derive(Clone, Copy, Default)]
struct RowState {
    hidden: bool,
    locked: bool,
    selectable: bool,
    current: bool,
    has_faces: bool,
    colours: [Option<[u8; 3]>; 2],
    group: bool,
    root: bool,
    holds_current: bool,
}
```

**Locate this block:**

```rust
    pub fn refresh(&mut self, scene: &crate::scene::Scene, selected: Vec<ObjectId>) {
        self.rows = crate::hierarchy::rows(scene);
        self.selected = selected;
    }
    pub fn all(&mut self) {
        self.open = true;
        self.expanded.extend(
```

Replace it with:

```rust
    pub fn refresh(&mut self, scene: &crate::scene::Scene, selected: Vec<ObjectId>) {
        self.rows = crate::hierarchy::rows(scene);
        self.selected = selected;
        self.states = self
            .rows
            .iter()
            .map(|row| {
                (
                    row.key.clone(),
                    RowState {
                        hidden: scene.view.lamps.contains(&row.key)
                            || !row.ids.is_empty()
                                && row
                                    .ids
                                    .iter()
                                    .all(|id| scene.view.hidden.contains(id)),
                        locked: !row.ids.is_empty()
                            && row
                                .ids
                                .iter()
                                .all(|id| scene.view.locked.contains(id)),
                        selectable: row
                            .ids
                            .iter()
                            .any(|id| scene.pick_id(id.value()).is_some()),
                        current: crate::layer_edit::current_key(scene).as_ref()
                            == Some(&row.key),
                        has_faces: row.ids.iter().any(|id| {
                            scene
                                .objects()
                                .iter()
                                .any(|mesh| mesh.id == *id)
                        }),
                        colours: [
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.faces.get(id).copied()),
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.edges.get(id).copied()),
                        ],
                        group: crate::layer_edit::locate(scene, &row.key).is_ok(),
                        root: row.depth == 0,
                        holds_current: crate::layer_edit::holds_current(
                            scene, &row.key,
                        ),
                    },
                )
            })
            .collect();
    }
    pub fn rename_result(&mut self, error: Option<&str>) {
        if let Some(error) = error {
            self.rename_error = error.into();
            self.rename_focus = true;
        } else {
            self.rename = None;
            self.rename_error.clear();
        }
    }
    pub fn all(&mut self) {
        self.open = true;
        self.expanded.extend(
```

**Locate this block:**

```rust
pub struct TreePanel {
    pub open: bool,
    pub rows: Vec<Row>,
    pub selected: Vec<ObjectId>,
```

Replace it with:

```rust
pub struct TreePanel {
    pub edit: Option<(String, crate::layer_edit::Edit)>,
    pub rename: Option<(String, String)>,
    pub rename_error: String,
    rename_focus: bool,
    pub confirm: Option<(String, usize)>,
    pub property: Option<(String, crate::layer_properties::Property)>,
    pub menu_rects: Vec<egui::Rect>,
    states: std::collections::HashMap<String, RowState>,
    pub open: bool,
    pub rows: Vec<Row>,
    pub selected: Vec<ObjectId>,
```

## <a id="checkpoint-53q-properties-ui"></a>19. Draw eye lock and independent colour menus

### 1. `src/tree_panel.rs`

**Locate this existing block:**

```rust
    holds_current: bool,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    holds_current: bool,
}

#[cfg(any(test, target_arch = "wasm32"))]
fn properties(
    ui: &mut egui::Ui,
    key: &str,
    state: RowState,
    action: &mut Option<(String, crate::layer_properties::Property)>,
    menus: &mut Vec<egui::Rect>,
    controls: &mut Option<Vec<Control>>,
) {
    use crate::layer_properties::Property;
    for (name, label, property, enabled) in [
        (
            "hide",
            if state.current {
                "✓"
            } else if state.hidden {
                "○"
            } else {
                "●"
            },
            Property::Hidden(!state.hidden),
            !state.current,
        ),
        (
            "lock",
            if state.locked {
                "🔒"
            } else {
                "🔓"
            },
            Property::Locked(!state.locked),
            true,
        ),
    ] {
        let button = ui
            .add_enabled(enabled, egui::Button::new(label))
            .on_hover_text(if name == "hide" {
                "Switch this row’s lamp; preserve the children’s own lamps"
            } else {
                "Lock or unlock selection of this row and children"
            });
        record(
            controls,
            &format!("layer/{name}/{key}"),
            name,
            &button,
        );
        if button.clicked() {
            *action = Some((key.to_owned(), property));
        }
    }
    let button = ui
        .button("■")
        .on_hover_text("Object colours; Faces and Edges are independent");
    record(
        controls,
        &format!("layer/colour/{key}"),
        "Colour",
        &button,
    );
    if let Some(menu) = egui::Popup::menu(&button)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            let channel_id = egui::Id::new(("layer-colour-channel", key));
            let mut edge = ui.ctx().data_mut(|data| {
                data.get_temp::<bool>(channel_id)
                    .unwrap_or(false)
            }) && state.has_faces;
            if state.has_faces {
                ui.horizontal(|ui| {
                    for (name, value) in [("Faces", false), ("Edges", true)] {
                        let button = ui.selectable_label(edge == value, name);
                        record(
                            controls,
                            &format!("layer/channel/{key}/{name}"),
                            name,
                            &button,
                        );
                        if button.clicked() {
                            edge = value;
                        }
                    }
                });
            } else {
                ui.label("Object and child colours");
            }
            ui.ctx()
                .data_mut(|data| data.insert_temp(channel_id, edge));
            let original = ui.button("Original");
            record(
                controls,
                &format!("layer/original/{key}/{edge}"),
                "Original",
                &original,
            );
            if original.clicked() {
                *action = Some((
                    key.to_owned(),
                    Property::Colour { edge, rgb: None },
                ));
                ui.close();
            }
            if let Some(rgb) = crate::layer_palette::draw(
                ui,
                key,
                edge,
                state.colours[usize::from(edge)],
                controls,
            ) {
                *action = Some((
                    key.to_owned(),
                    Property::Colour {
                        edge,
                        rgb: Some(rgb),
                    },
                ));
            }
        })
    {
        menus.push(menu.response.rect);
    }
}
```

## <a id="checkpoint-53r-name"></a>20. Own rename text and populated deletion confirmation

### 1. `src/tree_panel.rs`

**Locate this existing block:**

```rust
        menus.push(menu.response.rect);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        menus.push(menu.response.rect);
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn dialogs(
    ui: &mut egui::Ui,
    model: &mut TreePanel,
    controls: &mut Option<Vec<Control>>,
) {
    if let Some((key, text)) = model.rename.as_mut() {
        let id = egui::Id::new("layer-name-input");
        if std::mem::take(&mut model.rename_focus) {
            ui.memory_mut(|memory| memory.request_focus(id));
        }
        let cancel = ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
        });
        let mut submit = ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
        });
        ui.horizontal(|ui| {
            let field = ui.add(
                egui::TextEdit::singleline(text)
                    .id(id)
                    .char_limit(128)
                    .desired_width(130.),
            );
            record(controls, "layer/name", "Layer name", &field);
            let apply = ui.button("Rename");
            record(
                controls,
                "layer/name/apply",
                "Rename",
                &apply,
            );
            submit |= apply.clicked();
        });
        if !model.rename_error.is_empty() {
            ui.colored_label(
                egui::Color32::from_rgb(170, 30, 30),
                &model.rename_error,
            );
        }
        if submit {
            model.edit = Some((
                key.clone(),
                crate::layer_edit::Edit::Rename(text.clone()),
            ));
        }
        if cancel {
            model.rename = None;
            model.rename_error.clear();
            ui.memory_mut(|memory| memory.surrender_focus(id));
        }
    }
    if let Some((key, count)) = model.confirm.clone() {
        ui.label(format!(
            "Delete this layer and {count} objects?"
        ));
        ui.horizontal(|ui| {
            for (name, yes) in [("Delete", true), ("Cancel", false)] {
                let button = ui.button(name);
                record(
                    controls,
                    &format!("layer/delete/{name}"),
                    name,
                    &button,
                );
                if button.clicked() {
                    if yes {
                        model.edit =
                            Some((key.clone(), crate::layer_edit::Edit::Delete));
                    }
                    model.confirm = None;
                }
            }
        });
    }
}
```

## <a id="checkpoint-53s-tree-controls"></a>21. Connect group menus to the wider tree panel

### 1. `src/tree_panel.rs`

**Locate this existing block:**

```rust
    model.rect = None;
    if !model.open {
        return;
    }
    let visible = model.visible();
    let pages = visible.len().div_ceil(128).max(1);
    model.page = model.page.min(pages - 1);
    let response=egui::Area::new("session-tree".into()).anchor(egui::Align2::RIGHT_TOP,egui::vec2(-8.,8.)).order(egui::Order::Foreground).show(root.ctx(),|ui| {
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(248,248,248)).show(ui,|ui| {
            ui.set_width(240.);ui.horizontal(|ui| {ui.strong("Layers");let close=ui.button("×");record(controls,"tree/close","Close Layers",&close);if close.clicked() {model.open=false;}});
            if pages>1 {ui.horizontal(|ui| {if ui.button("Previous").clicked() {model.page=model.page.saturating_sub(1);}ui.label(format!("{} / {pages}",model.page+1));if ui.button("Next").clicked() {model.page=(model.page+1).min(pages-1);}});}
            let height=(root.max_rect().height()-100.).max(60.);
            egui::ScrollArea::vertical().max_height(height).show(ui,|ui| {
                for &index in visible.iter().skip(model.page*128).take(128) {
                    let row=&model.rows[index];ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=row.ids.iter().any(|id|model.selected.contains(id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let button=ui.add_enabled(!row.readonly && !row.ids.is_empty(),egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected)).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() {model.action=Some((row.ids.clone(),ui.input(|input|input.modifiers.shift)));}
                    });
                }
            });
        });
    });
    model.rect = model.open.then_some(response.response.rect);
}
#[derive(Clone, Copy, Default)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    model.rect = None;
    model.menu_rects.clear();
    if !model.open {
        return;
    }
    let visible = model.visible();
    let pages = visible.len().div_ceil(128).max(1);
    model.page = model.page.min(pages - 1);
    let response=egui::Area::new("session-tree".into()).anchor(egui::Align2::RIGHT_TOP,egui::vec2(-8.,8.)).order(egui::Order::Foreground).show(root.ctx(),|ui| {
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(248,248,248)).show(ui,|ui| {
            ui.set_width((root.available_width()/3.).clamp(240.,413.33334));ui.set_height((root.max_rect().height()-32.).max(60.));ui.horizontal(|ui| {ui.strong("Layers");let close=ui.button("×");record(controls,"tree/close","Close Layers",&close);if close.clicked() {model.open=false;}});
            dialogs(ui,model,controls);
            if pages>1 {ui.horizontal(|ui| {if ui.button("Previous").clicked() {model.page=model.page.saturating_sub(1);}ui.label(format!("{} / {pages}",model.page+1));if ui.button("Next").clicked() {model.page=(model.page+1).min(pages-1);}});}
            let height=(root.max_rect().height()-100.).max(60.);
            egui::ScrollArea::vertical().max_height(height).show(ui,|ui| {
                for &index in visible.iter().skip(model.page*128).take(128) {
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=row.ids.iter().any(|id|model.selected.contains(id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((row.ids.clone(),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
                                let button=ui.add_enabled(enabled,egui::Button::new(name));record(controls,&format!("layer/menu/{}/{name}",row.key),name,&button);
                                if button.clicked() {
                                    if name=="Rename" {model.rename=Some((row.key.clone(),row.label.clone()));model.rename_error.clear();model.rename_focus=true;}
                                    else if name=="Delete"&&!row.ids.is_empty() {model.confirm=Some((row.key.clone(),row.ids.len()));}
                                    else {model.edit=Some((row.key.clone(),edit.unwrap()));}
                                    ui.close();
                                }
                            }
                        }) {model.menu_rects.push(menu.response.rect);}}
                        properties(ui,&row.key,state,&mut model.property,&mut model.menu_rects,controls);
                    }));
                }
            });
        });
    });
    model.rect = model.open.then_some(response.response.rect);
}

#[derive(Clone, Copy, Default)]
```

## <a id="checkpoint-53t-input"></a>22. Route layer controls before handles and canvas input

### 1. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            ),
            modifiers: Default::default(),
        });
        let over = self
            .tree
            .rect
            .is_some_and(|r| r.contains(pos))
            || self
```

Replace it with:

```rust
            ),
            modifiers: Default::default(),
        });
        let over = (self
            .tree
            .rect
            .is_some_and(|r| r.contains(pos))
            || self
                .tree
                .menu_rects
                .iter()
                .any(|r| r.contains(pos)))
            || self
```

**Locate this block:**

```rust
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = self
            .tree
            .rect
            .is_some_and(|r| r.contains(pos))
            || self
                .number
                .number_rect
                .is_some_and(|r| r.contains(pos))
```

Replace it with:

```rust
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = (self
            .tree
            .rect
            .is_some_and(|r| r.contains(pos))
            || self
                .tree
                .menu_rects
                .iter()
                .any(|r| r.contains(pos)))
            || self
                .number
                .number_rect
                .is_some_and(|r| r.contains(pos))
```

**Locate this block:**

```rust
            && !modifiers.alt;
        if text {
            let number = self.number.number_prompt.is_some();
            if !self.context.egui_wants_keyboard_input() && !number {
                self.model.focus_command = true;
```

Replace it with:

```rust
            && !modifiers.alt;
        if text
            && (!self.context.egui_wants_keyboard_input()
                || self.number.number_prompt.is_some())
        {
            let number = self.number.number_prompt.is_some();
            if !self.context.egui_wants_keyboard_input() && !number {
                self.model.focus_command = true;
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .iter()
        .map(|row| (format!("{:?}", row.id), row.model.m))
        .collect();
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
```

Replace it with:

```rust
        .iter()
        .map(|row| (format!("{:?}", row.id), row.model.m))
        .collect();
    let view_ids = editor
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .chain(editor.scene.lines().iter().map(|row| row.id))
        .chain(editor.scene.paths().iter().map(|row| row.id))
        .chain(
            editor
                .scene
                .points()
                .iter()
                .map(|row| row.id),
        );
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
```

**Locate this block:**

```rust
                            .map_err(str::to_owned)
                    }
                });
                if tool_action && change.is_ok() {
                    panel.result(&editor.tool_status);
                }
```

Replace it with:

```rust
                            .map_err(str::to_owned)
                    }
                });
                if rename_action {
                    panel
                        .tree
                        .rename_result(change.as_ref().err().map(String::as_str));
                }
                if tool_action && change.is_ok() {
                    panel.result(&editor.tool_status);
                }
```

**Locate this block:**

```rust
            };
            if let Some(action) = action {
                highlight_changed |= matches!(&action, Action::SelectAll);
                let tool_action = matches!(
                    &action,
                    Action::BeginTool(_)
```

Replace it with:

```rust
            };
            if let Some(action) = action {
                highlight_changed |= matches!(&action, Action::SelectAll);
                let rename_action = matches!(
                    &action,
                    Action::LayerEdit {
                        edit: crate::layer_edit::Edit::Rename(_),
                        ..
                    }
                );
                let tool_action = matches!(
                    &action,
                    Action::BeginTool(_)
```

**Locate this block:**

```rust
            );
            let action = if let Some((ids, additive)) = panel.tree.action.take() {
                renderer.cancel_pick();
                gesture.cancel();
                Some(Action::TreePick { ids, additive })
```

Replace it with:

```rust
            );
            let action = if let Some((key, edit)) = panel.tree.edit.take() {
                renderer.cancel_pick();
                gesture.cancel();
                Some(Action::LayerEdit { key, edit })
            } else if let Some((key, property)) = panel.tree.property.take() {
                renderer.cancel_pick();
                gesture.cancel();
                Some(Action::LayerProperty { key, property })
            } else if let Some((ids, additive)) = panel.tree.action.take() {
                renderer.cancel_pick();
                gesture.cancel();
                Some(Action::TreePick { ids, additive })
```

## <a id="checkpoint-53u-layers"></a>23. Exercise layers shortcuts colours and exact source ownership

### 1. `src/shortcuts.rs`

**Locate this existing block:**

```rust
        None
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        None
    }
}

pub fn context_key(
    context: &egui::Context,
    command_text: &str,
    text: &str,
    command: bool,
    shift: bool,
    alt: bool,
    composing: bool,
) -> Option<Shortcut> {
    if egui::Popup::is_any_open(context) {
        return None;
    }
    let empty_command = context.memory(|memory| memory.focused())
        == Some(egui::Id::new("command-input"))
        && command_text.is_empty();
    key(
        text,
        command,
        shift,
        alt,
        composing,
        context.egui_wants_keyboard_input(),
        empty_command,
    )
}
```

### 2. `src/command_dock/mod.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused
```

Replace it with:

```rust
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = response.has_focus() || response.lost_focus();
    if focused
```

**Locate this block:**

```rust
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    // a running command takes values, not other commands
    if !focused
        || !model.completion_visible
```

Replace it with:

```rust
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = response.has_focus() || response.lost_focus();
    // a running command takes values, not other commands
    if !focused
        || !model.completion_visible
```

**Locate this block:**

```rust
) -> Keys {
    if model.focus_command || model.command_open {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
```

Replace it with:

```rust
) -> Keys {
    if model.focus_command {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
```

### 3. `src/view_flags_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    view_flags::Flags,
};
use std::rc::Rc;

#[test]
fn hide_show_and_undo_keep_original_sources_and_presentation_separate() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    let id = e.scene.objects()[0].id;
    let child = e.scene.objects()[1].id;
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let original = source.to_proto();
    e.select_objects([id], false);
    e.apply(Action::Hide).unwrap();
    assert!(!e.scene.visible(id) && !e.scene.visible(child));
    assert!(e.selected_objects().is_empty());
    assert_eq!(
        e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(!e.scene.visible(id) && !e.scene.visible(child));
    e.apply(Action::Show).unwrap();
    assert!(e.scene.visible(id) && e.scene.visible(child));
    e.apply(Action::Redo).unwrap();
    assert!(e.scene.visible(id));
    assert_eq!(e.scene.objects().len(), 4);
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    e.apply(Action::Undo).unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.objects().is_empty(),
        "Hide and Show created no document transaction"
    );
}

#[test]
fn show_reveals_initially_hidden_originals_without_changing_saved_flags() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
    mesh.is_visible = false;
    let id = e
        .scene
        .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
        .unwrap();
    assert!(!e.scene.visible(id));
    e.apply(Action::Show).unwrap();
    assert!(e.scene.visible(id));
    assert!(
        !e.scene.objects()[0]
            .geometry()
            .unwrap()
            .is_visible
    );
    assert_eq!(e.scene.pick_id(id.value()), Some(id));
    assert!(e.scene.bounds().is_some());
}

#[test]
fn presentation_flags_clone_without_pinning_sources_or_mutating_other_snapshots() {
    let mut e = Editor::default();
    let id = e.scene.objects()[0].id;
    let saved = e.scene.clone();
    let source = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    assert!(Rc::ptr_eq(&saved.view, &e.scene.view));
    Rc::make_mut(&mut e.scene.view).lock(&[id], true);
    assert!(!saved.view.locked.contains(&id));
    assert_eq!(e.scene.pick_id(id.value()), None);
    let flags = Rc::clone(&e.scene.view);
    e.apply(Action::Close).unwrap();
    drop(saved);
    assert!(source.upgrade().is_none());
    assert!(flags.locked.contains(&id));
    assert!(e.scene.view.locked.is_empty());
}

#[test]
fn view_flags_hold_plain_ids_and_rgb_channels_and_restore_originals_independently() {
    let e = Editor::default();
    let id = e.scene.objects()[0].id;
    let mut flags = Flags::default();
    flags.initial(id, false, true);
    assert!(flags.hidden.contains(&id) && flags.locked.contains(&id));
    flags.hide(&[id], false);
    assert!(!flags.hidden.contains(&id));
    flags.lock(&[id], false);
    assert!(!flags.locked.contains(&id));
    flags.faces.insert(id, [255, 0, 0]);
    flags.edges.insert(id, [0, 0, 255]);
    flags.faces.remove(&id);
    assert_eq!(flags.edges[&id], [0, 0, 255]);
}

#[test]
fn layer_row_properties_follow_descendants_and_protect_the_current_layer() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    let rows = crate::hierarchy::rows(&e.scene);
    let group = rows
        .iter()
        .find(|r| r.label == "Posts")
        .unwrap();
    let key = group.key.clone();
    let ids = group.ids.clone();
    Rc::make_mut(&mut e.scene.view).current = Some(key.clone());
    assert!(
        e.apply(Action::LayerProperty {
            key: key.clone(),
            property: crate::layer_properties::Property::Hidden(true)
        })
        .is_err()
    );
    assert!(ids.iter().all(|id| e.scene.visible(*id)));
    Rc::make_mut(&mut e.scene.view).current = None;
    e.apply(Action::LayerProperty {
        key: key.clone(),
        property: crate::layer_properties::Property::Hidden(true),
    })
    .unwrap();
    assert!(ids.iter().all(|id| !e.scene.visible(*id)));
    e.apply(Action::Show).unwrap();
    e.apply(Action::LayerProperty {
        key: key.clone(),
        property: crate::layer_properties::Property::Locked(true),
    })
    .unwrap();
    assert!(
        ids.iter()
            .all(|id| e.scene.pick_id(id.value()).is_none())
    );
    for edge in [false, true] {
        e.apply(Action::LayerProperty {
            key: key.clone(),
            property: crate::layer_properties::Property::Colour {
                edge,
                rgb: Some([65, 130, 225]),
            },
        })
        .unwrap();
    }
    assert!(
        ids.iter()
            .all(|id| e.scene.view.faces[id] == [65, 130, 225]
                && e.scene.view.edges[id] == [65, 130, 225])
    );
    e.apply(Action::LayerProperty {
        key,
        property: crate::layer_properties::Property::Colour {
            edge: false,
            rgb: None,
        },
    })
    .unwrap();
    assert!(
        ids.iter()
            .all(|id| !e.scene.view.faces.contains_key(id)
                && e.scene.view.edges.contains_key(id))
    );
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.objects().is_empty(),
        "Presentation properties created no document history"
    );
}

#[test]
fn nearest_source_node_colours_seed_faces_and_original_clears_only_the_view_override() {
    use prost::Message;
    let mut source = session_rust::proto::Session::decode(
        crate::tree_specimen::nested(true).as_slice(),
    )
    .unwrap();
    let root = source
        .tree
        .as_mut()
        .unwrap()
        .root
        .as_mut()
        .unwrap();
    let posts = &mut root.children[0];
    posts.color = Some(session_rust::Color::red().to_proto());
    posts.children[0].children[0].children[0].color =
        Some(session_rust::Color::blue().to_proto());
    let original = source
        .objects
        .as_ref()
        .unwrap()
        .meshes
        .clone();
    let mut e = Editor::default();
    e.apply(Action::Replace(source.encode_to_vec()))
        .unwrap();
    let id = e
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    assert_eq!(e.scene.view.faces[&id[0]], [255, 0, 0]);
    assert_eq!(e.scene.view.faces[&id[1]], [0, 0, 255]);
    assert!(!e.scene.view.faces.contains_key(&id[2]));
    assert!(e.scene.view.edges.is_empty());
    let key = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| row.label == "Posts")
        .unwrap()
        .key;
    e.apply(Action::LayerProperty {
        key,
        property: crate::layer_properties::Property::Colour {
            edge: false,
            rgb: None,
        },
    })
    .unwrap();
    assert!(e.scene.view.faces.is_empty());
    for (row, mesh) in e.scene.objects().iter().zip(original) {
        assert_eq!(row.geometry().unwrap().to_proto(), mesh);
    }
    let group = e.scene.forest.documents[0]
        .nodes
        .iter()
        .find(|node| node.label == "Posts")
        .unwrap();
    assert_eq!(group.source.color.as_ref().unwrap().r, 1.);
}

#[test]
fn parent_lamp_preserves_child_visibility_and_child_lamps() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    let rows = crate::hierarchy::rows(&e.scene);
    let posts = rows
        .iter()
        .find(|row| row.label == "Posts")
        .unwrap();
    let root = rows[0].key.clone();
    let first = posts.ids[0];
    Rc::make_mut(&mut e.scene.view).hide(&[first], true);
    let set = |e: &mut Editor, key: String, on| {
        e.apply(Action::LayerProperty {
            key,
            property: crate::layer_properties::Property::Hidden(on),
        })
        .unwrap()
    };
    set(&mut e, posts.key.clone(), true);
    set(&mut e, root.clone(), true);
    set(&mut e, root, false);
    assert!(e.scene.view.lamps.contains(&posts.key));
    assert!(
        posts
            .ids
            .iter()
            .all(|id| !e.scene.visible(*id))
    );
    set(&mut e, posts.key.clone(), false);
    assert!(!e.scene.visible(first));
    assert!(
        posts
            .ids
            .iter()
            .filter(|id| **id != first)
            .all(|id| e.scene.visible(*id))
    );
    e.apply(Action::Show).unwrap();
    assert!(e.scene.view.lamps.is_empty() && e.scene.view.held.is_empty());
    assert!(
        posts
            .ids
            .iter()
            .all(|id| e.scene.visible(*id))
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod layer_edit;

pub mod layer_palette;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod layer_edit;

pub mod layer_palette;

#[cfg(test)]
mod view_flags_tests;
```

### 5. `src/layer_edit_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    layer_edit::{self, Edit},
};
use std::rc::Rc;

fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    e
}
fn key(e: &Editor, label: &str) -> String {
    crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| row.label == label)
        .unwrap()
        .key
}
fn edit(
    e: &mut Editor,
    key: &str,
    edit: Edit,
) -> Result<crate::editor::Change, &'static str> {
    e.apply(Action::LayerEdit {
        key: key.to_owned(),
        edit,
    })
}

#[test]
fn new_and_renamed_layers_keep_unrelated_keys_sources_and_one_undo() {
    let mut e = editor();
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    let original = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .map(|row| row.key)
        .collect::<Vec<_>>();
    let sources = e
        .scene
        .objects()
        .iter()
        .map(|row| Rc::clone(row.geometry().unwrap()))
        .collect::<Vec<_>>();
    edit(&mut e, &root, Edit::New { sublayer: false }).unwrap();
    let layer = key(&e, "Layer 01");
    edit(&mut e, &layer, Edit::Current).unwrap();
    edit(
        &mut e,
        &layer,
        Edit::Rename(" Timber ".into()),
    )
    .unwrap();
    assert_eq!(key(&e, "Timber"), layer);
    assert_eq!(e.scene.view.current.as_ref(), Some(&layer));
    assert!(original.iter().all(|key| {
        crate::hierarchy::rows(&e.scene)
            .iter()
            .any(|row| &row.key == key)
    }));
    assert!(
        e.scene
            .objects()
            .iter()
            .zip(sources)
            .all(|(row, source)| Rc::ptr_eq(row.geometry().unwrap(), &source))
    );
    e.apply(Action::Undo).unwrap();
    assert_eq!(key(&e, "Layer 01"), layer);
    e.apply(Action::Undo).unwrap();
    assert!(layer_edit::locate(&e.scene, &layer).is_err());
    e.apply(Action::Redo).unwrap();
    assert_eq!(key(&e, "Layer 01"), layer);
}

#[test]
fn layer_errors_preserve_redo_and_current_ancestors_cannot_be_hidden_or_deleted() {
    let mut e = editor();
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    edit(&mut e, &root, Edit::New { sublayer: true }).unwrap();
    let parent = key(&e, "Layer 01");
    edit(&mut e, &parent, Edit::New { sublayer: true }).unwrap();
    let child = key(&e, "Layer 02");
    edit(&mut e, &child, Edit::Current).unwrap();
    for target in [&parent, &child] {
        assert!(edit(&mut e, target, Edit::Delete).is_err());
        assert!(
            e.apply(Action::LayerProperty {
                key: target.clone(),
                property: crate::layer_properties::Property::Hidden(true)
            })
            .is_err()
        );
    }
    assert!(edit(&mut e, &root, Edit::Rename("Root".into())).is_err());
    assert!(edit(&mut e, &root, Edit::Delete).is_err());
    e.apply(Action::Undo).unwrap();
    assert!(edit(&mut e, &parent, Edit::Rename(" / ".into())).is_err());
    assert!(edit(&mut e, &parent, Edit::Rename("Posts".into())).is_err());
    e.apply(Action::Redo).unwrap();
    assert_eq!(key(&e, "Layer 02"), child);
    let doc = &e.scene.forest.documents[0];
    let node = doc
        .nodes
        .iter()
        .find(|node| node.object == Some(e.scene.objects()[0].id))
        .unwrap();
    let object = format!("{}/{}", doc.key, node.source.guid);
    assert!(edit(&mut e, &object, Edit::Current).is_err());
}

#[test]
fn deleting_a_populated_group_restores_descendants_and_readonly_ownership_on_undo() {
    let mut e = editor();
    let group = key(&e, "Posts");
    let rows = crate::hierarchy::rows(&e.scene);
    let ids = rows
        .iter()
        .find(|row| row.key == group)
        .unwrap()
        .ids
        .clone();
    let child = e.scene.objects()[1].id;
    assert!(e.scene.forest.readonly(child));
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    edit(&mut e, &group, Edit::Delete).unwrap();
    assert!(ids.iter().all(|id| !e.scene.contains(*id)));
    assert!(!e.scene.forest.readonly(child));
    e.apply(Action::Undo).unwrap();
    assert_eq!(key(&e, "Posts"), group);
    assert!(ids.iter().all(|id| e.scene.contains(*id)));
    assert!(e.scene.forest.readonly(child));
    assert_eq!(e.scene.pick_id(child.value()), None);
    assert!(Rc::ptr_eq(
        e.scene.objects()[0].geometry().unwrap(),
        &source
    ));
    e.apply(Action::Redo).unwrap();
    assert!(ids.iter().all(|id| !e.scene.contains(*id)));
}

#[test]
fn created_and_current_layers_receive_every_generated_geometry_lane_and_survive_undo() {
    let mut e = editor();
    e.apply(Action::AddBox).unwrap();
    let created = key(&e, "Created");
    assert_eq!(
        layer_edit::current_key(&e.scene),
        Some(created.clone())
    );
    assert!(edit(&mut e, &created, Edit::Delete).is_err());
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    edit(&mut e, &root, Edit::New { sublayer: true }).unwrap();
    let layer = key(&e, "Layer 01");
    edit(&mut e, &layer, Edit::Current).unwrap();
    let count = e.scene.objects().len();
    for action in [
        Action::AddBox,
        Action::AddLine,
        Action::AddPoint,
        Action::AddCurve,
    ] {
        e.apply(action).unwrap();
    }
    let row = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| row.key == layer)
        .unwrap();
    assert_eq!(row.ids.len(), 4);
    assert_eq!(e.scene.objects().len(), count + 1);
    for _ in 0..4 {
        e.apply(Action::Undo).unwrap();
    }
    assert!(
        crate::hierarchy::rows(&e.scene)
            .iter()
            .find(|row| row.key == layer)
            .unwrap()
            .ids
            .is_empty()
    );
    assert_eq!(
        layer_edit::current_key(&e.scene),
        Some(layer)
    );
    e.apply(Action::Close).unwrap();
    assert!(layer_edit::current_key(&e.scene).is_none());
    assert!(e.scene.view.current.is_none());
}

#[test]
fn changing_layers_across_documents_keeps_world_placement_and_attached_attributes_exact()
 {
    let mut e = editor();
    let ids = e
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let models = e
        .scene
        .objects()
        .iter()
        .map(|row| row.model.m)
        .collect::<Vec<_>>();
    e.apply(Action::Import(crate::tree_specimen::nested(
        false,
    )))
    .unwrap();
    let target_doc = &e.scene.forest.documents[1];
    let target = format!(
        "{}/{}",
        target_doc.key, target_doc.nodes[0].source.guid
    );
    edit(&mut e, &target, Edit::Move(vec![ids[0]])).unwrap();
    assert!(
        e.scene.forest.documents[0]
            .nodes
            .iter()
            .all(|node| node.object != Some(ids[0]) && node.object != Some(ids[1]))
    );
    assert!(
        e.scene.forest.documents[1]
            .nodes
            .iter()
            .any(|node| node.object == Some(ids[1]) && node.readonly)
    );
    assert!(Rc::ptr_eq(
        e.scene.objects()[0].geometry().unwrap(),
        &source
    ));
    assert_eq!(e.scene.objects()[0].model.m, models[0]);
    assert_eq!(e.scene.objects()[1].model.m, models[1]);
    let saved = crate::document::snapshot(&e.scene).unwrap();
    let mut reopened = Editor::default();
    reopened
        .apply(Action::Replace(saved))
        .unwrap();
    assert_eq!(
        reopened.scene.objects()[0].model.m,
        models[0]
    );
    assert_eq!(
        reopened.scene.objects()[1].model.m,
        models[1]
    );
    assert!(
        reopened
            .scene
            .forest
            .readonly(reopened.scene.objects()[1].id)
    );
    assert!(edit(&mut e, &target, Edit::Move(vec![ids[0]])).is_err());
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.forest.documents[0]
            .nodes
            .iter()
            .any(|node| node.object == Some(ids[0]))
    );
    e.apply(Action::Redo).unwrap();
    assert!(
        e.scene.forest.documents[1]
            .nodes
            .iter()
            .any(|node| node.object == Some(ids[0]))
    );
}

#[test]
fn duplicate_layers_copy_descendants_and_view_flags_but_share_precise_source_allocations()
 {
    let mut e = editor();
    let group = key(&e, "Posts");
    let parent = e.scene.objects()[0].clone();
    let child = e.scene.objects()[1].clone();
    e.apply(Action::LayerProperty {
        key: group.clone(),
        property: crate::layer_properties::Property::Hidden(true),
    })
    .unwrap();
    e.apply(Action::LayerProperty {
        key: group.clone(),
        property: crate::layer_properties::Property::Locked(true),
    })
    .unwrap();
    e.apply(Action::LayerProperty {
        key: group.clone(),
        property: crate::layer_properties::Property::Colour {
            edge: true,
            rgb: Some([17, 29, 231]),
        },
    })
    .unwrap();
    edit(&mut e, &group, Edit::Duplicate).unwrap();
    let copy = key(&e, "Posts copy 01");
    let copied = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| row.key == copy)
        .unwrap()
        .ids;
    assert_eq!(copied.len(), 2);
    for (id, original) in copied.iter().zip([parent, child]) {
        let row = e
            .scene
            .objects()
            .iter()
            .find(|row| row.id == *id)
            .unwrap();
        assert_ne!(row.guid, original.guid);
        assert!(Rc::ptr_eq(
            row.geometry().unwrap(),
            original.geometry().unwrap()
        ));
        assert!(Rc::ptr_eq(&row.mesh, &original.mesh));
        assert_eq!(row.model.m, original.model.m);
        assert!(!e.scene.visible(*id));
        assert!(e.scene.view.locked.contains(id));
        assert_eq!(e.scene.view.edges[id], [17, 29, 231]);
    }
    assert!(e.scene.forest.readonly(copied[1]));
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.objects().len(), 3);
    e.apply(Action::Redo).unwrap();
    assert_eq!(key(&e, "Posts copy 01"), copy);
}

#[test]
fn copy_object_layer_keeps_originals_and_rejects_readonly_or_released_sources_without_history()
 {
    let mut e = editor();
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    let parent = e.scene.objects()[0].clone();
    let child = e.scene.objects()[1].id;
    assert!(edit(&mut e, &root, Edit::Copy(vec![child])).is_err());
    edit(&mut e, &root, Edit::Copy(vec![parent.id])).unwrap();
    let copy = e.scene.objects().last().unwrap();
    assert_ne!(copy.id, parent.id);
    assert!(Rc::ptr_eq(
        copy.geometry().unwrap(),
        parent.geometry().unwrap()
    ));
    assert_eq!(copy.model.m, parent.model.m);
    assert!(!e.scene.forest.readonly(copy.id));
    assert_eq!(e.scene.objects().len(), 4);
    e.apply(Action::Undo).unwrap();
    let origins = e
        .scene
        .objects()
        .iter()
        .filter_map(|row| row.origin().map(|origin| origin.id))
        .collect();
    e.scene.unload(&origins, 7);
    assert!(edit(&mut e, &root, Edit::Current).is_err());
    assert!(edit(&mut e, &root, Edit::New { sublayer: true }).is_err());
    assert!(edit(&mut e, &root, Edit::Copy(vec![parent.id])).is_err());
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.objects().len(), 4);
}

#[test]
fn object_copy_keeps_each_explicit_selection_while_parent_move_deduplicates_followers()
{
    let mut e = Editor::default();
    e.apply(Action::Replace(
        crate::tree_specimen::nested(false),
    ))
    .unwrap();
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    let ids = e
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    edit(
        &mut e,
        &root,
        Edit::Copy(vec![ids[0], ids[1], ids[1]]),
    )
    .unwrap();
    assert_eq!(e.scene.objects().len(), 5);
    assert_eq!(
        e.scene.objects()[3].model.m,
        e.scene.objects()[0].model.m
    );
    assert_eq!(
        e.scene.objects()[4].model.m,
        e.scene.objects()[1].model.m
    );
    e.apply(Action::Undo).unwrap();
    edit(&mut e, &root, Edit::New { sublayer: true }).unwrap();
    let target = key(&e, "Layer 01");
    let models = e
        .scene
        .objects()
        .iter()
        .map(|row| row.model.m)
        .collect::<Vec<_>>();
    edit(
        &mut e,
        &target,
        Edit::Move(vec![ids[0], ids[1]]),
    )
    .unwrap();
    let row = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| row.key == target)
        .unwrap();
    assert_eq!(row.ids, ids[..2]);
    assert_eq!(e.scene.objects().len(), 3);
    assert_eq!(
        e.scene
            .objects()
            .iter()
            .map(|row| row.model.m)
            .collect::<Vec<_>>(),
        models
    );
}

#[test]
fn portable_tree_identity_is_stable_across_repeated_saves_and_distinct_across_imports()
{
    let mut e = editor();
    e.apply(Action::Import(crate::tree_specimen::nested(
        true,
    )))
    .unwrap();
    let first = crate::document::snapshot(&e.scene).unwrap();
    use prost::Message;
    let decoded = |bytes: &[u8]| session_rust::proto::Session::decode(bytes).unwrap();
    assert_eq!(
        decoded(&first),
        decoded(&crate::document::snapshot(&e.scene).unwrap())
    );
    let tree = decoded(&first).tree.unwrap().root.unwrap();
    let mut nodes = vec![&tree];
    let mut guids = std::collections::HashSet::new();
    while let Some(node) = nodes.pop() {
        if !node.guid.is_empty() {
            assert!(uuid::Uuid::parse_str(&node.guid).is_ok());
            assert!(guids.insert(&node.guid));
        }
        nodes.extend(node.children.iter());
    }
    let mut reopened = Editor::default();
    reopened
        .apply(Action::Replace(first))
        .unwrap();
    assert_eq!(reopened.scene.objects().len(), 6);
}
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod view_flags_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod view_flags_tests;

#[cfg(test)]
mod layer_edit_tests;
```

### 7. `src/layer_ui_tests.rs`

Create the file and type:

```rust
use crate::{
    command_dock::Control,
    editor::{Action, Editor},
    layer_edit::Edit,
    tree_panel::{self, TreePanel},
};

fn setup() -> (egui::Context, TreePanel, String) {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddNested).unwrap();
    let root = crate::hierarchy::rows(&e.scene)[0]
        .key
        .clone();
    e.apply(Action::LayerEdit {
        key: root,
        edit: Edit::New { sublayer: true },
    })
    .unwrap();
    let mut model = TreePanel::default();
    model.refresh(&e.scene, vec![]);
    model.all();
    let key = model
        .rows
        .iter()
        .find(|row| row.label == "Layer 01")
        .unwrap()
        .key
        .clone();
    let context = egui::Context::default();
    context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
    (context, model, key)
}

fn frame(
    context: &egui::Context,
    model: &mut TreePanel,
    events: Vec<egui::Event>,
) -> Vec<Control> {
    let mut controls = Some(Vec::new());
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1000., 800.),
        )),
        events,
        ..Default::default()
    };
    let _ = context.run_ui(input, |ui| {
        tree_panel::draw(ui, model, &mut controls)
    });
    controls.unwrap()
}

fn center(controls: &[Control], key: &str) -> egui::Pos2 {
    let rect = controls
        .iter()
        .find(|control| control.key == key)
        .unwrap_or_else(|| panic!("Missing actual control {key}"))
        .rect;
    egui::pos2(
        (rect[0] + rect[2]) / 2.,
        (rect[1] + rect[3]) / 2.,
    )
}
fn click(
    context: &egui::Context,
    model: &mut TreePanel,
    at: egui::Pos2,
    button: egui::PointerButton,
) -> Vec<Control> {
    for pressed in [true, false] {
        frame(
            context,
            model,
            vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button,
                    pressed,
                    modifiers: Default::default(),
                },
            ],
        );
    }
    frame(context, model, vec![])
}

#[test]
fn empty_group_context_menu_renames_with_real_text_focus_and_escape_owns_the_key() {
    let (context, mut model, key) = setup();
    frame(&context, &mut model, vec![]);
    let controls = frame(&context, &mut model, vec![]);
    assert!(model.rect.unwrap().width() >= 330.);
    let controls = click(
        &context,
        &mut model,
        center(&controls, &format!("tree/select/{key}")),
        egui::PointerButton::Secondary,
    );
    assert!(!model.menu_rects.is_empty());
    let controls = click(
        &context,
        &mut model,
        center(
            &controls,
            &format!("layer/menu/{key}/Rename"),
        ),
        egui::PointerButton::Primary,
    );
    assert!(model.rename.is_some());
    assert!(
        controls
            .iter()
            .any(|control| control.key == "layer/name")
    );
    assert_eq!(
        context.memory(|memory| memory.focused()),
        Some(egui::Id::new("layer-name-input"))
    );
    frame(
        &context,
        &mut model,
        vec![egui::Event::Text(" HS".into())],
    );
    assert_eq!(
        model.rename.as_ref().unwrap().1,
        "Layer 01 HS"
    );
    assert!(model.edit.is_none());
    frame(
        &context,
        &mut model,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
    assert!(
        matches!(model.edit.take(),Some((_,Edit::Rename(text))) if text=="Layer 01 HS")
    );
    model.rename_result(Some("This name is already used"));
    frame(&context, &mut model, vec![]);
    assert_eq!(
        model.rename.as_ref().unwrap().1,
        "Layer 01 HS"
    );
    assert!(!model.rename_error.is_empty());
    frame(
        &context,
        &mut model,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
    assert!(model.rename.is_none());
    assert!(model.edit.is_none());
    assert_eq!(
        context.memory(|memory| memory.focused()),
        None
    );
}

#[test]
fn actual_colour_popup_has_nine_swatches_rgb_and_independent_face_edge_channels() {
    let (context, mut model, _) = setup();
    let group = model
        .rows
        .iter()
        .find(|row| row.label == "Posts")
        .unwrap()
        .key
        .clone();
    frame(&context, &mut model, vec![]);
    let controls = frame(&context, &mut model, vec![]);
    let controls = click(
        &context,
        &mut model,
        center(&controls, &format!("layer/colour/{group}")),
        egui::PointerButton::Primary,
    );
    assert!(!model.menu_rects.is_empty());
    for name in [
        "Red", "Orange", "Yellow", "Green", "Blue", "Violet", "White", "Gray", "Black",
    ] {
        assert!(
            controls
                .iter()
                .any(|control| control.key
                    == format!("layer/colour/{group}/false/{name}"))
        );
    }
    for name in ["R", "G", "B"] {
        assert!(controls.iter().any(|control|control.key==format!("layer/rgb/{group}/false/{name}")));
    }
    let controls = click(
        &context,
        &mut model,
        center(
            &controls,
            &format!("layer/channel/{group}/Edges"),
        ),
        egui::PointerButton::Primary,
    );
    assert!(
        controls
            .iter()
            .any(|control| control.key == format!("layer/original/{group}/true"))
    );
    click(
        &context,
        &mut model,
        center(
            &controls,
            &format!("layer/colour/{group}/true/Blue"),
        ),
        egui::PointerButton::Primary,
    );
    assert!(matches!(
        model.property,
        Some((
            _,
            crate::layer_properties::Property::Colour {
                edge: true,
                rgb: Some([65, 130, 225])
            }
        ))
    ));
}

#[test]
fn populated_layer_deletion_requires_the_actual_confirmation_button() {
    let (context, mut model, _) = setup();
    let group = model
        .rows
        .iter()
        .find(|row| row.label == "Posts")
        .unwrap()
        .key
        .clone();
    frame(&context, &mut model, vec![]);
    let controls = frame(&context, &mut model, vec![]);
    let controls = click(
        &context,
        &mut model,
        center(&controls, &format!("tree/select/{group}")),
        egui::PointerButton::Secondary,
    );
    let controls = click(
        &context,
        &mut model,
        center(
            &controls,
            &format!("layer/menu/{group}/Delete"),
        ),
        egui::PointerButton::Primary,
    );
    assert_eq!(model.confirm.as_ref(), Some(&(group, 2)));
    assert!(model.edit.is_none());
    click(
        &context,
        &mut model,
        center(&controls, "layer/delete/Cancel"),
        egui::PointerButton::Primary,
    );
    assert!(model.confirm.is_none());
    assert!(model.edit.is_none());
}

#[test]
fn long_source_labels_keep_the_wider_panel_within_the_canvas() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
    mesh.name = "A very long source name ".repeat(100);
    e.scene
        .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
        .unwrap();
    let context = egui::Context::default();
    let mut model = TreePanel::default();
    model.refresh(&e.scene, vec![]);
    model.all();
    frame(&context, &mut model, vec![]);
    let controls = frame(&context, &mut model, vec![]);
    let panel = model.rect.unwrap();
    assert!(panel.min.x >= 0. && panel.max.x <= 1000.);
    assert!(panel.width() < 380.);
    assert!(controls.iter().all(
        |control| control.rect[0] >= panel.min.x && control.rect[2] <= panel.max.x
    ));
    assert!(
        model
            .rows
            .iter()
            .any(|row| row.label.len() > 1000),
        "Clipping the label does not shorten original source metadata"
    );
}

struct Vocabulary;
impl crate::command_dock::Commands for Vocabulary {
    fn canonical(&self, text: &str) -> String {
        text.into()
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, text: &str) -> (String, bool) {
        (text.into(), true)
    }
    fn completions(&self, _: &str) -> Vec<&'static str> {
        vec![]
    }
    fn browse(&self, _: &str) -> Vec<&'static str> {
        vec![]
    }
}

#[test]
fn an_open_command_dock_does_not_steal_layer_editing_focus() {
    let (context, mut tree, key) = setup();
    let mut dock = crate::command_dock::CommandLine::default();
    dock.command_open = true;
    let run =
        |tree: &mut TreePanel, dock: &mut crate::command_dock::CommandLine, events| {
            let mut command = None;
            let mut controls = Some(Vec::new());
            let _ = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000., 800.),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    tree_panel::draw(ui, tree, &mut controls);
                    crate::command_dock::draw(
                        ui,
                        dock,
                        &mut controls,
                        &mut command,
                        &Vocabulary,
                        true,
                    );
                },
            );
            assert!(command.is_none());
        };
    run(&mut tree, &mut dock, vec![]);
    run(&mut tree, &mut dock, vec![]);
    tree.rename = Some((key, "Layer 01".into()));
    tree.rename_result(Some("Retry this name"));
    run(&mut tree, &mut dock, vec![]);
    run(
        &mut tree,
        &mut dock,
        vec![egui::Event::Text(" HS".into())],
    );
    assert_eq!(
        context.memory(|m| m.focused()),
        Some(egui::Id::new("layer-name-input"))
    );
    assert_eq!(
        tree.rename.as_ref().unwrap().1,
        "Layer 01 HS"
    );
    assert!(dock.command.is_empty());
    assert_eq!(
        crate::shortcuts::context_key(
            &context,
            &dock.command,
            "a",
            true,
            false,
            false,
            false
        ),
        None
    );
}
```

### 8. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod layer_edit_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod layer_edit_tests;

#[cfg(test)]
mod layer_ui_tests;
```

### 9. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(renderer.preview_bytes(), 0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.5)).unwrap();
    let source = Rc::clone(scene.objects()[0].geometry().unwrap());
    let original = source.to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let allocations = renderer.stats();
    Rc::make_mut(&mut scene.view)
        .faces
        .insert(id, [0, 0, 255]);
    renderer.set_scene(&scene, None);
    let blue = colour_pixels(&mut renderer);
    assert_ne!(blue, before);
    let centre = (120 * 320 + 160) * 4;
    assert!(
        blue[centre + 2] > blue[centre].saturating_add(100)
            && blue[centre + 2] > blue[centre + 1].saturating_add(100)
    );
    assert_eq!(
        &renderer.stats()[..2],
        &allocations[..2],
        "Layer colour reuses primary geometry and its settings allocation"
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .insert(id, [0, 255, 0]);
    renderer.set_scene(&scene, None);
    let green_edges = colour_pixels(&mut renderer);
    let count = green_edges
        .chunks_exact(4)
        .filter(|p| p[1] > p[0].saturating_add(100) && p[1] > p[2].saturating_add(100))
        .count();
    assert!(
        count > 100,
        "Actual source boundary colour is independent: {count}"
    );
    Rc::make_mut(&mut scene.view)
        .faces
        .remove(&id);
    renderer.set_scene(&scene, None);
    let restored_faces = colour_pixels(&mut renderer);
    assert_eq!(
        &restored_faces[centre..centre + 4],
        &before[centre..centre + 4]
    );
    assert_ne!(
        restored_faces, before,
        "Restoring Faces leaves the independent edge override"
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .remove(&id);
    renderer.set_scene(&scene, None);
    assert_eq!(colour_pixels(&mut renderer), before);
    assert!(Rc::ptr_eq(
        &source,
        scene.objects()[0].geometry().unwrap()
    ));
}
```

## Build & Run

![Recorded visual output for Lesson 8.3 (53u-layers)](../../screenshots/journey/53u-layers.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 8.3
npm --prefix ../session_tests run course -- save 8.3
```

Continue with [8.4 · Navigation, delayed picks and direct dragging](54-camera.md).
