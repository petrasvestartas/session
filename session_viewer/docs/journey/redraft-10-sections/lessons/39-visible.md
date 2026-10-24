# 5.5 · One visibility policy and shared surface depth

**Goal:** Use one explicit depth format and policy for opaque surfaces and screen-space ink, then prove actual occlusion. · **Steps:** 2 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Overlapping 3D Triangles along View Ray</text>
  <circle cx="58" cy="104" r="5" fill="#111111"/>
  <line x1="64" y1="104" x2="324" y2="104" stroke="#111111" stroke-width="1.75" stroke-dasharray="4 3" marker-end="url(#arr)"/>
  <polygon points="148,56 176,150 126,142" fill="#fdba74" stroke="#ea580c" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="152" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ea580c">z = 0.25 (Near)</text>
  <polygon points="248,50 284,154 220,148" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="254" y="44" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">z = 0.75 (Far)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Depth32Float Test: z_frag &lt; z_buf</text>
  <rect x="438" y="50" width="244" height="106" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="462" y="66" width="88" height="56" fill="#fdba74" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="506" y="98" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">0.25 PASS</text>
  <rect x="570" y="66" width="88" height="56" fill="#e2e8f0" stroke="#64748b" stroke-width="1.5" stroke-dasharray="3 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="614" y="98" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">0.75 REJECT</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">CompareFunction::Less</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Output (Near Occludes Far)</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="944,54 1018,144 892,144" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <polygon points="914,62 978,150 850,150" fill="#fdba74" stroke="#111111" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-39-visible"></a>1. Resolve visibility for every drawing kind

Respect original source visibility consistently in drawing, Fit, control markers and mesh picking while retaining hidden sources.

### 1. `src/chain.rs`

Read visibility from the original shared path source.

**Locate this existing block:**

```rust
impl Source {
    pub fn guid(&self) -> &str {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Source {
    pub fn visible(&self) -> bool {
        match self {
            Self::Line(line) => line.is_visible,
            Self::Polyline(line) => line.is_visible,
            Self::Curve(curve, _) => curve.is_visible,
        }
    }

    pub fn guid(&self) -> &str {
```

### 2. `src/scene.rs`

Resolve original visibility for every current document kind without removing ownership. Exclude invisible surfaces from visible Fit bounds. Exclude invisible original line endpoints from Fit. Exclude hidden polylines and original curve hulls from Fit. Keep hidden standalone points out of visible bounds. Hide original controls with their invisible curve parent. Prevent hidden selected surfaces from influencing Fit Selected.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible)
        {
```

Replace it with:

```rust
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible && row.visible())
        {
```

**Locate this block:**

```rust
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        let object = self
            .objects
            .iter()
```

Replace it with:

```rust
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) {
            return None;
        }
        let object = self
            .objects
            .iter()
```

**Locate this block:**

```rust
                }
            }
        }
        for row in &self.points {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
            match &mut bounds {
```

Replace it with:

```rust
                }
            }
        }
        for row in self
            .points
            .iter()
            .filter(|row| row.visible())
        {
            let p = row.point();
            let p = [p[0], p[1], p[2]];
            match &mut bounds {
```

**Locate this block:**

```rust
                }
            }
        }
        for row in &self.paths {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
```

Replace it with:

```rust
                }
            }
        }
        for row in self.paths.iter().filter(|row| row.visible()) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
```

**Locate this block:**

```rust
                }
            }
        }
        for line in &self.lines {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
```

Replace it with:

```rust
                }
            }
        }
        for line in self
            .lines
            .iter()
            .filter(|line| line.visible())
        {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
```

**Locate this block:**

```rust
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in &self.objects {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
```

Replace it with:

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
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self
            .objects
            .iter()
            .filter(|row| row.visible())
        {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
```

### 3. `src/renderer.rs`

Draw only visible original point rows. Keep invisible connected path data out of GPU instance allocation. Keep hidden original lines out of the stroke batch. Allocate surface GPU rows only for visible document objects.

**Locate this existing block:**

```rust
                    .iter()
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
                    .flat_map(crate::scene::PathObject::segments),
            )
            .expect("Prepared path placement fits the display range");
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .map(crate::scene::LineObject::stroke),
            )
            .expect("Prepared line placement fits the display range");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
            let matching = old.iter().position(|row| {
```

Edit the existing block to match this code; keep unchanged lines:

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
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
            .iter()
            .filter(|row| row.visible())
        {
            let matching = old.iter().position(|row| {
```

### 4. `src/picking.rs`

Use the same visibility predicate for ray picking and drawing.

**Locate this existing block:**

```rust
    let mut hit = None;
    for object in scene.objects() {
        let vertices = object.mesh.vertices();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut hit = None;
    for object in scene
        .objects()
        .iter()
        .filter(|row| row.visible())
    {
        let vertices = object.mesh.vertices();
```

### 5. `src/lib.rs`

Register hidden ownership and visible picking checks.

**Locate this existing block:**

```rust
mod shortcuts_tests;
pub mod startup;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shortcuts_tests;
#[cfg(test)]
mod source_visibility_tests;
pub mod startup;
```

### 6. `src/source_visibility_tests.rs`

Copy retained hidden ownership, empty visible bounds and invisible picking refusal checks.

Create the file and type:

```rust
use crate::{
    camera::Camera, chain::PreparedChain, marker::PreparedPoint,
    prepared::PreparedMesh, scene::Scene, stroke::PreparedLine,
};
use session_rust::{Line, Mesh, NurbsCurve, Point, Polyline};
use std::rc::Rc;

pub fn hidden_scene() -> Scene {
    let mut scene = Scene::demo();
    scene.clear();
    let mut mesh = Mesh::create_box(1.0, 1.0, 1.0);
    mesh.is_visible = false;
    scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    let mut line = Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
    line.is_visible = false;
    scene
        .insert_line(PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let points = vec![
        Point::new(-0.6, -0.4, 0.0),
        Point::new(0.0, 0.6, 0.0),
        Point::new(0.6, -0.4, 0.0),
    ];
    let mut polyline = Polyline::new(points.clone());
    polyline.is_visible = false;
    scene
        .insert_path(PreparedChain::polyline(Rc::new(polyline)).unwrap())
        .unwrap();
    let mut curve = NurbsCurve::create(false, 2, &points);
    curve.is_visible = false;
    scene
        .insert_path(PreparedChain::curve(Rc::new(curve)).unwrap())
        .unwrap();
    let mut point = Point::new(0.0, 0.0, 0.0);
    point.is_visible = false;
    scene
        .insert_point(PreparedPoint::new(Rc::new(point)).unwrap())
        .unwrap();
    scene.show_controls(true);
    scene
}

#[test]
fn invisible_original_sources_remain_owned_but_cannot_fit_or_pick() {
    let scene = hidden_scene();
    assert_eq!(scene.objects().len(), 1);
    assert_eq!(scene.lines().len(), 1);
    assert_eq!(scene.paths().len(), 2);
    assert_eq!(scene.points().len(), 1);
    assert!(scene.bounds().is_none());
    assert!(scene.next(None).is_none());
    assert!(scene.control_markers().is_empty());
    assert!(
        scene
            .selected_bounds(scene.objects()[0].id)
            .is_none()
    );
    let ray = Camera::default().ray([0.0, 0.0]).unwrap();
    assert!(crate::picking::pick(&scene, &ray).is_none());
    assert_eq!(crate::memory::curves([&scene])[1], 1);
    assert_eq!(crate::memory::points([&scene])[1], 1);
}

#[test]
fn selection_cycles_past_hidden_sources_without_changing_their_rows() {
    let mut scene = hidden_scene();
    let hidden = scene.objects()[0].id;
    let visible = scene
        .insert(PreparedMesh::new(Mesh::create_box(1.0, 1.0, 1.0)).unwrap())
        .unwrap();
    assert_eq!(scene.next(None), Some(visible));
    assert_eq!(scene.next(Some(hidden)), Some(visible));
    assert_eq!(scene.next(Some(visible)), Some(visible));
    let ray = Camera::default().ray([0.0, 0.0]).unwrap();
    assert_eq!(
        crate::picking::pick(&scene, &ray),
        Some(visible)
    );
}
```

### 7. `src/scene.rs`

Read retained source visibility directly without scanning all document IDs. Cycle only visible mesh rows without allocating a second object list.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        if self.objects.is_empty() {
            return None;
        }
        let index = self
            .objects
            .iter()
            .position(|object| Some(object.id) == current)
            .map_or(0, |index| index + 1);
        Some(self.objects[index % self.objects.len()].id)
    }
```

Replace it with:

```rust
    }

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

**Locate this block:**

```rust
}

impl PointObject {
    pub fn point(&self) -> session_rust::Point {
        self.model
            .transform_point(&self.prepared.source)
```

Replace it with:

```rust
}

impl PointObject {
    pub fn visible(&self) -> bool {
        self.prepared.source.is_visible
    }

    pub fn point(&self) -> session_rust::Point {
        self.model
            .transform_point(&self.prepared.source)
```

**Locate this block:**

```rust
}

impl PathObject {
    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
```

Replace it with:

```rust
}

impl PathObject {
    pub fn visible(&self) -> bool {
        self.prepared.source.visible()
    }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
```

**Locate this block:**

```rust
}

impl LineObject {
    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [
```

Replace it with:

```rust
}

impl LineObject {
    pub fn visible(&self) -> bool {
        self.prepared.source.is_visible
    }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [
```

**Locate this block:**

```rust
impl Object {
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry),
```

Replace it with:

```rust
impl Object {
    pub fn visible(&self) -> bool {
        self.metadata.is_visible
    }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry),
```

## <a id="checkpoint-39a-depth"></a>2. Share surface depth across all ink lanes

### 1. `src/depth.rs`

Name one depth format and explicit surface/ink comparison policies.

Create the file and type:

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub fn surface() -> wgpu::DepthStencilState {
    state(true, wgpu::CompareFunction::Less)
}
pub fn ink() -> wgpu::DepthStencilState {
    state(false, wgpu::CompareFunction::LessEqual)
}

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: FORMAT,
        depth_write_enabled: Some(write),
        depth_compare: Some(compare),
        stencil: Default::default(),
        bias: Default::default(),
    }
}
```

### 2. `src/lib.rs`

Register the shared depth contract.

**Locate this existing block:**

```rust
mod browser_reload;
pub mod document;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod browser_reload;
pub mod depth;
pub mod document;
```

### 3. `src/renderer.rs`

Use the shared writable nearer-surface policy. Allocate the exact depth format used by every drawing pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
```

Replace it with:

```rust
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
```

**Locate this block:**

```rust
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
```

Replace it with:

```rust
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
```

### 4. `src/stroke_gpu.rs`

Keep all analytic ink lanes read-only against the same surface depth.

**Locate this existing block:**

```rust
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(),
```

## Build & Run

![Recorded visual output for Lesson 5.5 (39a-depth)](../../../screenshots/journey/39a-depth-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.5
npm --prefix ../session_tests run course -- save 5.5
```

Continue with [5.6 · Original mesh colours and large index buffers](../../consolidation/40-colour.md).
