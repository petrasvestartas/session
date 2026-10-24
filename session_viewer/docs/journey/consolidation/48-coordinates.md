# 7.3 · Construction-plane drafts and one final commit

**Goal:** Draw points and lines through the shared command dock. · **Steps:** 5 · **Source:** [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/command/tool.rs](../../../src/app/command/tool.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/snap.rs](../../../src/app/snap.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Ray ∩ C-Plane: t = -dot(ro - O, n) / dot(rd, n)</text>
  <polygon points="48,144 128,84 316,84 236,144" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="88" y1="114" x2="276" y2="114" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="182" y1="84" x2="102" y2="144" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="74" y1="48" x2="182" y2="114" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#arr)"/>
  <circle cx="182" cy="114" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="200" y="110" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">P_snap(u,v)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Gumball Axes &amp; Arcs (Move / Rotate / Scale)</text>
  <circle cx="560" cy="116" r="46" fill="none" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <line x1="560" y1="116" x2="646" y2="116" stroke="#dc2626" stroke-width="3" marker-end="url(#arrR)"/>
  <line x1="560" y1="116" x2="560" y2="44" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <line x1="560" y1="116" x2="504" y2="154" stroke="#2563eb" stroke-width="3" marker-end="url(#arrB)"/>
  <rect x="618" y="84" width="10" height="10" fill="#dc2626" stroke="#111111" stroke-width="1.2"/>
  <circle cx="560" cy="116" r="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Transformed Geometry: M_delta * P_local</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="842,136 882,88 932,136" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="918,126 958,78 1008,126" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="882" y1="112" x2="952" y2="102" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
</svg>
</div>

## <a id="checkpoint-48-coordinates"></a>1. Resolve drawing points on the construction plane

### 1. `src/tool_coords.rs`

Validate world coordinates and intersect the current ray with the XY plane.

Create the file and type:

```rust
pub fn checked(point: [f64; 3]) -> Result<[f64; 3], &'static str> {
    if point
        .iter()
        .any(|v| !v.is_finite() || v.abs() > 1e12)
    {
        return Err("Coordinates must be finite and within ±1e12");
    }
    Ok(point)
}

pub fn parse(text: &str) -> Result<[f64; 3], &'static str> {
    let parts: Vec<_> = text.split(',').map(str::trim).collect();
    if !(2..=3).contains(&parts.len()) {
        return Err("Use world coordinates x,y,z or x,y");
    }
    let mut point = [0.; 3];
    for (value, text) in point.iter_mut().zip(parts) {
        *value = text
            .parse()
            .map_err(|_| "Coordinates must be numbers")?;
    }
    checked(point)
}

// A screen ray intersects the construction plane; parallel or clipped rays cannot supply a drawing point.
pub fn on_xy(camera: &crate::camera::Camera, screen: [f32; 2]) -> Option<[f64; 3]> {
    let ray = camera.ray(screen)?;
    if ray.direction[2].abs() < 1e-10 {
        return None;
    }
    let distance = -ray.origin[2] / ray.direction[2];
    if distance < 0. || distance > ray.max_distance {
        return None;
    }
    checked(std::array::from_fn(|i| {
        ray.origin[i] + distance * ray.direction[i]
    }))
    .ok()
}
```

### 2. `src/tool_coords_tests.rs`

Copy finite-coordinate, camera round-trip and invalid-ray checks.

Create the file and type:

```rust
#[test]
fn absolute_points_require_valid_finite_coordinates() {
    assert_eq!(
        crate::tool_coords::parse("1, 2").unwrap(),
        [1., 2., 0.]
    );
    assert_eq!(
        crate::tool_coords::parse("1e-12, -2, 3").unwrap(),
        [1e-12, -2., 3.]
    );
    for text in [
        "1", "1,,2", "1,2,3,4", "NaN,2,3", "inf,2,3", "1e13,2,3",
    ] {
        assert!(
            crate::tool_coords::parse(text).is_err(),
            "{text}"
        );
    }
}

#[test]
fn plane_points_use_the_current_camera_and_reject_parallel_or_clipped_rays() {
    for projection in [
        crate::camera::Projection::Perspective,
        crate::camera::Projection::Orthographic,
    ] {
        let mut camera = crate::camera::Camera::default();
        camera.projection = projection;
        camera.pan(0.25, -0.5);
        let p = [0.125, -0.25, 0.];
        let screen = camera
            .view_projection()
            .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        let actual =
            crate::tool_coords::on_xy(&camera, [screen[0] as f32, screen[1] as f32])
                .unwrap();
        for i in 0..3 {
            assert!((actual[i] - p[i]).abs() < 1e-6);
        }
        assert!(crate::tool_coords::on_xy(&camera, [f32::NAN, 0.]).is_none());
        assert!(crate::tool_coords::on_xy(&camera, [2., 0.]).is_none());
        camera.orientation = session_rust::Quaternion::identity();
        assert!(crate::tool_coords::on_xy(&camera, [0., 0.]).is_none());
    }
}
```

### 3. `src/lib.rs`

Register the tool component and copied checks.

**Locate this existing block:**

```rust
mod commands_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod commands_tests;

pub mod tool_coords;
#[cfg(test)]
mod tool_coords_tests;

#[cfg(test)]
```

## <a id="checkpoint-48a-draft"></a>2. Keep unfinished point tools outside the document

### 1. `src/tool.rs`

Define Point and Line tools with temporary accepted points and a cursor.

Create the file and type:

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built {
    Point(PreparedPoint),
    Line(PreparedLine),
}

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn buttons(&self) -> Vec<(String, String)> {
        vec![("Cancel".into(), "Escape".into())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Point,
    Line,
}

impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Point => "Point",
            Self::Line => "Line",
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Point => 1,
            Self::Line => 2,
        }
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(
                session_rust::Point::new(p[0], p[1], p[2]),
            ))?)),
            (Self::Line, [a, b]) => {
                if a.iter()
                    .zip(b)
                    .all(|(a, b)| (a - b).abs() < 1e-12)
                {
                    return Err("Line endpoints must differ");
                }
                Ok(Built::Line(PreparedLine::new(Rc::new(
                    session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2]),
                ))?))
            }
            _ => Err("The tool needs more points"),
        }
    }
}

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft {
    pub tool: Box<dyn Tool>,
    pub points: Vec<[f64; 3]>,
    pub cursor: Option<[f64; 3]>,
}

impl Draft {
    pub fn new(shape: Shape) -> Self {
        Self {
            tool: Box::new(shape),
            points: Vec::new(),
            cursor: None,
        }
    }
    pub fn prompt(&self) -> String {
        format!(
            "{} point {} of {} · Esc cancels",
            self.tool.name(),
            self.points.len() + 1,
            self.tool.count()
        )
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> {
        let point = crate::tool_coords::checked(point)?;
        let mut points = self.points.clone();
        points.push(point);
        if points.len() == self.tool.count() {
            return self.tool.build(&points).map(Some);
        }
        if points.len() > self.tool.count() {
            return Err("The tool has enough points");
        }
        self.points = points;
        self.cursor = None;
        Ok(None)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
        self.cursor = point
            .map(crate::tool_coords::checked)
            .transpose()?;
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Register the tool component and copied checks.

**Locate this existing block:**

```rust
mod tool_coords_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod tool_coords_tests;

pub mod tool;

#[cfg(test)]
```

## <a id="checkpoint-48b-preview"></a>3. Draw a temporary rubber band and cursor

### 1. `src/tool_preview.rs`

Draw and release a bounded temporary band and cursor.

Create the file and type:

```rust
use crate::{stroke_gpu::Lane, tool::Draft};

pub struct Preview {
    strokes: Lane,
    markers: Lane,
}

impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            strokes: Lane::new(device, format),
            markers: Lane::markers(device, format),
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        draft: Option<&Draft>,
    ) -> Result<(), &'static str> {
        let yellow = [1., 0.75, 0.05, 1.];
        let mut strokes = Vec::new();
        let mut markers = Vec::new();
        if let Some(draft) = draft {
            if let Some(cursor) = draft.cursor {
                markers.push(crate::marker::Marker {
                    center: cursor.map(|v| v as f32),
                    diameter: 8.,
                    colour: yellow,
                });
                if let Some(&start) = draft.points.last().filter(|&&p| p != cursor) {
                    strokes.push(crate::stroke::Stroke {
                        start: start.map(|v| v as f32),
                        end: cursor.map(|v| v as f32),
                        width: 2.,
                        colour: yellow,
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.markers.set_points(device, markers)?;
        Ok(())
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        self.strokes
            .view(queue, transform, size, density);
        self.markers
            .view(queue, transform, size, density);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.markers.draw(pass);
    }
    pub fn bytes(&self) -> u64 {
        self.strokes.usage()[1] + self.markers.usage()[1]
    }
}
```

### 2. `src/lib.rs`

Register the tool component and copied checks.

**Locate this existing block:**

```rust
pub mod tool;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod tool;

pub mod tool_preview;

#[cfg(test)]
```

### 3. `src/renderer.rs`

Keep tool preview presentation separate from primary geometry and integer picking.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
        }
```

Replace it with:

```rust
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
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
        self.preview.view(
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
        Ok(())
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

Replace it with:

```rust
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

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

**Locate this block:**

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.highlight
            .set(
                &self.device,
```

Replace it with:

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.preview
            .set(&self.device, None)
            .expect("Empty tool preview");
        self.highlight
            .set(
                &self.device,
```

**Locate this block:**

```rust
            pipeline,
            highlight_pipeline,
            highlight,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

Replace it with:

```rust
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

**Locate this block:**

```rust
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

Replace it with:

```rust
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

**Locate this block:**

```rust
    highlight: crate::highlight::Highlight,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

Replace it with:

```rust
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

### 4. `src/pick_gpu_tests.rs`

Copy actual preview pixels, bounded bytes, no pick ownership and resource release checks.

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
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap();
    draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap();
    assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "The actual unfinished rubber band and cursor render"
    );
    let p = &after[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 3];
    assert!(
        p[0] > p[1] && p[1] > p[2],
        "The preview is yellow: {p:?}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Drafts have no saved-object pick owner"
    );
    assert_eq!(renderer.stats(), stats);
    assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap();
    assert_eq!(renderer.preview_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap();
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.preview_bytes(), 0);
}
```

## <a id="checkpoint-48c-commit"></a>4. Commit completed tools in one Undo transaction

### 1. `src/editor.rs`

Own tool input, keep previews out of history and insert a completed source atomically.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .filter(|_| self.selected_part.is_none())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Picked(id) => self.select_objects(id, false),
```

Replace it with:

```rust
            .filter(|_| self.selected_part.is_none())
    }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
        }
        text.contains(',')
            .then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
            .ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else {
            self.tool_status = draft.prompt();
            return Ok(Change::View);
        };
        self.history
            .try_edit(&mut self.scene, |scene| match built {
                crate::tool::Built::Point(point) => {
                    scene.insert_point(point).map(|_| ())
                }
                crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
            })?;
        self.draft = None;
        self.tool_status = format!("{name} created.");
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if matches!(
            &action,
            Action::Close
                | Action::Undo
                | Action::Redo
                | Action::Import(_)
                | Action::Replace(_)
                | Action::ImportAt(_, _)
                | Action::ReplaceAt(_, _)
        ) {
            self.draft = None;
        }
        match action {
            Action::BeginTool(shape) => {
                self.draft = Some(crate::tool::Draft::new(shape));
                self.tool_status = self.draft.as_ref().unwrap().prompt();
                return Ok(Change::View);
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => {
                if let Some(draft) = self.draft.as_mut() {
                    draft.hover(point)?;
                }
                return Ok(Change::View);
            }
            Action::CancelTool => {
                self.draft = None;
                self.tool_status = "Tool cancelled.".into();
                return Ok(Change::View);
            }
            Action::Picked(id) => self.select_objects(id, false),
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            draft: None,
            tool_status: String::new(),
            scene: Scene::demo(),
            selected: None,
            selected_part: None,
```

**Locate this block:**

```rust
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
```

Replace it with:

```rust
}

pub struct Editor {
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
```

**Locate this block:**

```rust
pub enum Action {
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
```

Replace it with:

```rust
pub enum Action {
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
```

### 2. `src/tool_tests.rs`

Copy source precision, one-Undo completion, invalid draft input and cancellation/history checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Change, Editor},
    tool::Shape,
};

#[test]
fn preview_and_cancellation_preserve_original_geometry_and_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    editor
        .apply(Action::Translate([1., 0., 0.]))
        .unwrap();
    editor.apply(Action::Undo).unwrap();
    let source = editor.scene.objects()[0].mesh.clone();
    let before = editor.scene.objects()[0].model.m;
    assert_eq!(
        editor
            .apply(Action::BeginTool(Shape::Line))
            .unwrap(),
        Change::View
    );
    assert_eq!(
        editor
            .apply(Action::ToolPoint([0., 0., 0.]))
            .unwrap(),
        Change::View
    );
    for x in 0..100 {
        editor
            .apply(Action::ToolHover(Some([
                x as f64 / 100.,
                0.,
                0.,
            ])))
            .unwrap();
    }
    assert!(editor.scene.lines().is_empty());
    assert_eq!(
        editor.draft.as_ref().unwrap().points.len(),
        1
    );
    assert!(
        editor
            .apply(Action::ToolPoint([0., 0., 0.]))
            .is_err()
    );
    assert!(
        editor
            .apply(Action::ToolPoint([f64::NAN, 0., 0.]))
            .is_err()
    );
    assert_eq!(
        editor.draft.as_ref().unwrap().points.len(),
        1
    );
    editor
        .apply(editor.tool_line("Escape").unwrap().unwrap())
        .unwrap();
    assert!(editor.draft.is_none());
    assert_eq!(editor.scene.objects()[0].model.m, before);
    assert!(std::rc::Rc::ptr_eq(
        &source,
        &editor.scene.objects()[0].mesh
    ));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor.scene.objects()[0].model.m[12],
        before[12] + 1.
    );
}

#[test]
fn completing_a_line_commits_once_and_preserves_double_coordinates() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
        .apply(Action::BeginTool(Shape::Line))
        .unwrap();
    let a = [0.123456789012345, -0.25, 0.];
    let b = [1.23456789012345, 0.5, 0.];
    editor.apply(Action::ToolPoint(a)).unwrap();
    assert_eq!(
        editor.apply(Action::ToolPoint(b)).unwrap(),
        Change::Scene
    );
    assert!(editor.draft.is_none());
    assert_eq!(editor.scene.lines().len(), 1);
    let line = &editor.scene.lines()[0].prepared.source;
    for i in 0..3 {
        assert_eq!(line.start()[i], a[i]);
        assert_eq!(line.end()[i], b[i]);
    }
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.lines().len(), 1);
    editor
        .apply(Action::BeginTool(Shape::Point))
        .unwrap();
    editor
        .apply(Action::ToolPoint([3., 2., 1.]))
        .unwrap();
    assert_eq!(editor.scene.points().len(), 1);
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.points().is_empty());
    assert_eq!(editor.scene.lines().len(), 1);
}

#[test]
fn absolute_points_validate_before_draft_changes_and_empty_enter_cancels() {
    let mut editor = Editor::default();
    editor
        .apply(Action::BeginTool(Shape::Line))
        .unwrap();
    assert!(editor.tool_line("Fit").is_none());
    assert!(editor.tool_line("NaN,2,3").unwrap().is_err());
    editor
        .apply(editor.tool_line("1,2,3").unwrap().unwrap())
        .unwrap();
    editor
        .apply(editor.tool_line("").unwrap().unwrap())
        .unwrap();
    assert!(editor.draft.is_none());
    editor
        .apply(Action::BeginTool(Shape::Line))
        .unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(editor.draft.is_none());
    editor
        .apply(Action::BeginTool(Shape::Line))
        .unwrap();
    editor.apply(Action::Close).unwrap();
    assert!(editor.draft.is_none());
}
```

### 3. `src/lib.rs`

Register the tool component and copied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod selection_cycle_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod tool_tests;

#[cfg(test)]
mod selection_cycle_tests;
```

## <a id="checkpoint-48d-input"></a>5. Draw points and lines through the shared command dock

### 1. `src/panel.rs`

Route tool coordinates and canvas gestures, own cancellation and show one command answer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return Ok(None);
        };
        if !self.model.options.is_empty() {
            return Ok(Some(line));
```

Replace it with:

```rust
            return Ok(None);
        };
        if !self.model.options.is_empty() {
            self.answer(&line, "Tool input submitted.");
            return Ok(Some(line));
```

**Locate this block:**

```rust
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

Replace it with:

```rust
    }

    pub fn cancel_input(&mut self) {
        self.model.command.clear();
        self.model.completion_visible = false;
        self.model.inline_suffix = false;
        self.model.command_open = false;
        self.model.focus_command = false;
        self.context.memory_mut(|memory| {
            memory.surrender_focus(egui::Id::new("command-input"))
        });
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

### 2. `src/commands.rs`

Route tool coordinates and canvas gestures, own cancellation and show one command answer.

**Locate this existing block:**

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Example Box", || Action::AddBox),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Point", || {
        Action::BeginTool(crate::tool::Shape::Point)
    }),
    ("Line", || {
        Action::BeginTool(crate::tool::Shape::Line)
    }),
    ("Example Box", || Action::AddBox),
```

### 3. `src/browser.rs`

Route tool coordinates and canvas gestures, own cancellation and show one command answer.

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
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor}).to_string()))?;
    canvas.set_attribute(
        "data-preview-bytes",
        &renderer.preview_bytes().to_string(),
    )?;
```

**Locate this block:**

```rust
                    }
                }
            }
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) = renderer
```

Replace it with:

```rust
                    }
                }
            }
            if let Some(draft) = editor.draft.as_ref() {
                panel.tool(draft.prompt(), draft.tool.buttons());
            } else {
                panel.tool(String::new(), Vec::new());
            }
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
                panel.result(error);
            }
            renderer.set_pick_mode(editor.pick_mode);
            if highlight_changed {
                if let Err(error) = renderer
```

**Locate this block:**

```rust
                            .map_err(str::to_owned)
                    }
                });
                match change {
                    Ok(Some(Change::Scene)) => {
                        highlight_changed = true;
```

Replace it with:

```rust
                            .map_err(str::to_owned)
                    }
                });
                if tool_action && change.is_ok() {
                    panel.result(&editor.tool_status);
                }
                match change {
                    Ok(Some(Change::Scene)) => {
                        highlight_changed = true;
```

**Locate this block:**

```rust
            };
            if let Some(action) = action {
                highlight_changed |= matches!(&action, Action::SelectAll);
                if matches!(
                    &action,
                    Action::Close
```

Replace it with:

```rust
            };
            if let Some(action) = action {
                highlight_changed |= matches!(&action, Action::SelectAll);
                let tool_action = matches!(
                    &action,
                    Action::BeginTool(_) | Action::ToolPoint(_) | Action::CancelTool
                );
                if matches!(&action, Action::CancelTool) {
                    panel.cancel_input();
                }
                if matches!(
                    &action,
                    Action::Close
```

**Locate this block:**

```rust
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) => {
                        pick_additive.set(
                            event
```

Replace it with:

```rust
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) if editor.draft.is_some() => {
                        crate::tool_coords::on_xy(&editor.camera, screen)
                            .map(Action::ToolPoint)
                    }
                    None if editor.draft.is_some()
                        && matches!(
                            event.type_().as_str(),
                            "blur" | "resize" | "pointercancel" | "lostpointercapture"
                        ) =>
                    {
                        Some(Action::ToolHover(None))
                    }
                    None if editor.draft.is_some()
                        && !panel.consumed
                        && event.type_() == "pointermove" =>
                    {
                        event
                            .dyn_ref::<web_sys::PointerEvent>()
                            .filter(|p| p.buttons() == 0)
                            .map(|p| {
                                let r = pointer_canvas.get_bounding_client_rect();
                                let screen = [
                                    (2. * (p.client_x() as f64 - r.left()) / r.width()
                                        - 1.)
                                        as f32,
                                    (1. - 2. * (p.client_y() as f64 - r.top())
                                        / r.height())
                                        as f32,
                                ];
                                Action::ToolHover(crate::tool_coords::on_xy(
                                    &editor.camera,
                                    screen,
                                ))
                            })
                    }
                    Some(Action::Pick(screen)) => {
                        pick_additive.set(
                            event
```

**Locate this block:**

```rust
                    }
                }
            } else if let Some(line) = line {
                match crate::commands::parse(&line) {
                    Err(error) => {
                        panel.result(&error);
                        None
```

Replace it with:

```rust
                    }
                }
            } else if let Some(line) = line {
                let command = editor
                    .tool_line(&line)
                    .map(|result| {
                        result
                            .map(crate::commands::Command::Editor)
                            .map_err(str::to_owned)
                    })
                    .unwrap_or_else(|| crate::commands::parse(&line));
                match command {
                    Err(error) => {
                        panel.result(&error);
                        None
```

**Locate this block:**

```rust
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
```

Replace it with:

```rust
            };

            let line = if editor.draft.is_some()
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .is_some_and(|key| {
                        event.type_() == "keydown"
                            && key.key() == "Escape"
                            && !key.is_composing()
                    }) {
                event.prevent_default();
                gesture.cancel();
                Some("Escape".into())
            } else {
                line
            };

            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
```

### 4. `src/panel.rs`

Release the old option rectangle when a tool completes or cancels.

**Locate this existing block:**

```rust
        self.model.options = options;
        if !self.model.options.is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.model.options = options;
        if self.model.options.is_empty() {
            self.model.options_rect = None;
        }
        if !self.model.options.is_empty() {
```

## Build & Run

![Recorded visual output for Lesson 7.3 (48d-input)](../../screenshots/journey/48d-input.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 7.3
npm --prefix ../session_tests run course -- save 7.3
```

Continue with [7.4 · Precise snapping with derived search caches](49-settings.md).
