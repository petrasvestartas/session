# 8.4 · Navigation, delayed picks and direct dragging

**Steps:** 18 · **Source:** [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/layers.rs](../../../src/app/layers.rs) · [app/ui/layers.rs](../../../src/app/ui/layers.rs) · [app/gesture/mod.rs](../../../src/app/gesture/mod.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Drag Left-&gt;Right (Window) vs Right-&gt;Left (Crossing)</text>
  <rect x="54" y="54" width="108" height="88" fill="#dbeafe" fill-opacity="0.35" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">dx &gt; 0: Inside Only</text>
  <rect x="180" y="54" width="108" height="88" fill="#dcfce7" fill-opacity="0.35" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="234" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">dx &lt; 0: Any Overlap</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen AABB Containment vs Segment-Rect Intersection</text>
  <rect x="434" y="56" width="112" height="84" fill="none" stroke="#2563eb" stroke-width="2"/>
  <polygon points="458,122 490,74 522,122" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="574" y="56" width="112" height="84" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <polygon points="638,126 676,68 712,126" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Multi-Object Selection Set Updated</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 876,74 914,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <polygon points="944,136 982,74 1020,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

## <a id="checkpoint-54-camera"></a>1. Navigate and fit inside the free canvas

### 1. `src/camera.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
```

Replace it with:

```rust
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
        }
        self.ray_unbounded(screen)
    }

    pub fn ray_unbounded(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen.iter().any(|v| !v.is_finite()) {
            return None;
```

**Locate this block:**

```rust
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        projection * view
    }

    pub fn uniform(&self) -> [f32; 16] {
```

Replace it with:

```rust
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        let shift = self.shift();
        Xform::translation(shift[0], shift[1], 0.) * projection * view
    }

    pub fn shift(&self) -> [f64; 2] {
        [
            self.frame[0] + self.frame[2] - 1.,
            1. - self.frame[1] - self.frame[3],
        ]
    }

    pub fn frame(&mut self, frame: [f64; 4]) -> bool {
        if frame
            .iter()
            .any(|v| !v.is_finite() || *v < 0. || *v > 1.)
            || frame[0] >= frame[2]
            || frame[1] >= frame[3]
        {
            return false;
        }
        if self.frame == frame {
            return false;
        }
        self.frame = frame;
        true
    }

    pub fn pan_screen(&mut self, delta: [f64; 2], size: [f64; 2]) {
        if delta.iter().any(|v| !v.is_finite())
            || size
                .iter()
                .any(|v| !v.is_finite() || *v <= 0.)
        {
            return;
        }
        let scale = self.distance * 0.0015;
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += scale * (-delta[0] * right[i] + delta[1] * up[i]);
        }
    }

    pub fn zoom_at(&mut self, factor: f32, screen: [f32; 2]) {
        if screen
            .iter()
            .any(|v| !v.is_finite() || v.abs() > 1.)
        {
            return;
        }
        let old = self.distance;
        self.zoom(factor);
        let k = self.distance / old;
        let shift = self.shift();
        let h = old * HALF_FOV.tan();
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += (1. - k)
                * h
                * (right[i] * (screen[0] as f64 - shift[0]) * self.aspect
                    + up[i] * (screen[1] as f64 - shift[1]));
        }
    }

    pub fn uniform(&self) -> [f32; 16] {
```

**Locate this block:**

```rust
        self.target = bounds.centre();
        let opening = match self.projection {
            Projection::Perspective => half_x.min(half_y).sin(),
            Projection::Orthographic => half_y.tan() * self.aspect.min(1.0),
        };
        self.distance = self.radius * 1.1 / opening;
    }
```

Replace it with:

```rust
        self.target = bounds.centre();
        let opening = match self.projection {
            Projection::Perspective => half_x.min(half_y).sin(),
            Projection::Orthographic => half_y.tan().min(half_x.tan()),
        };
        self.distance = self.radius * 1.1 / opening;
    }
```

**Locate this block:**

```rust
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
            grid: None,
        }
    }
}

impl Camera {
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = HALF_FOV;
        let half_x = (self.aspect * half_y.tan()).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 {
            radius
```

Replace it with:

```rust
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
            grid: None,
            frame: [0., 0., 1., 1.],
        }
    }
}

impl Camera {
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = (HALF_FOV.tan() * (self.frame[3] - self.frame[1])).atan();
        let half_x =
            (self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0])).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 {
            radius
```

**Locate this block:**

```rust
    pub grid: Option<crate::grid::Settings>,
}

impl Default for Camera {
```

Replace it with:

```rust
    pub grid: Option<crate::grid::Settings>,
    pub frame: [f64; 4],
}

impl Default for Camera {
```

### 2. `src/navigation.rs`

**Locate this existing block:**

```rust
    };
    let pixels = (delta * unit).clamp(-600.0, 600.0);
    Some(Action::Zoom((-0.002 * pixels).exp() as f32))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    let pixels = (delta * unit).clamp(-1_000.0, 1_000.0);
    Some(Action::Zoom(
        0.9_f64.powf(pixels / 100.0) as f32
    ))
}
```

### 3. `src/navigation_tests.rs`

**Locate this existing block:**

```rust
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(600.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-600.0, 0));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}
```

## <a id="checkpoint-54aa-view-actions"></a>2. Route camera changes without consuming document history

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        self.camera = Camera {
                            aspect,
                            grid: self.camera.grid,
                            ..Camera::default()
```

Replace it with:

```rust
                        self.camera = Camera {
                            aspect,
                            grid: self.camera.grid,
                            frame: self.camera.frame,
                            ..Camera::default()
```

**Locate this block:**

```rust
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds() {
                            self.camera.fit(&bounds);
                        }
                    }
```

Replace it with:

```rust
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt { factor, screen } => {
                        self.camera.zoom_at(factor, screen)
                    }
                    Action::PanScreen { delta, size } => {
                        self.camera.pan_screen(delta, size)
                    }
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self
                            .display_scene()
                            .bounds_of(&self.selected_objects())
                            .or_else(|| self.display_scene().bounds())
                        {
                            self.camera.fit(&bounds);
                        }
                    }
```

**Locate this block:**

```rust
                | Action::CancelTool
                | Action::Snap(_)
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
```

Replace it with:

```rust
                | Action::CancelTool
                | Action::Snap(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
```

**Locate this block:**

```rust
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
```

Replace it with:

```rust
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
```

**Locate this block:**

```rust
pub enum Action {
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
```

Replace it with:

```rust
pub enum Action {
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
    },
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
```

## <a id="checkpoint-54a-selection"></a>3. Apply visible selection and modifier precedence

### 1. `src/shortcuts.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Delete,
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
```

### 2. `src/object_selection.rs`

**Locate this existing block:**

```rust
    }

    pub fn retain(&mut self, scene: &Scene) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn remove(&mut self, scene: &Scene, rows: impl IntoIterator<Item = ObjectId>) {
        let rows = rows
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        self.order.retain(|id| !rows.contains(id));
        self.retain(scene);
    }

    pub fn retain(&mut self, scene: &Scene) {
```

### 3. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .collect();
                self.select_objects(ids, false);
            }
```

Replace it with:

```rust
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .collect();
                self.select_objects(ids, false);
                return Ok(Change::View);
            }
```

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
```

Replace it with:

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::SelectRows { ids, add, remove } => {
                self.select_rows(ids, add, remove);
                return Ok(Change::View);
            }
            _ => {}
        }
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
```

**Locate this block:**

```rust
        self.selected = self.objects.primary();
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
```

Replace it with:

```rust
        self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self, ids: Vec<ObjectId>, add: bool, remove: bool) {
        if remove {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
            self.objects.remove(&self.scene, ids);
            self.selected = self.objects.primary();
            self.selected_part = None;
        } else {
            self.select_objects(ids, add);
        }
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene);
```

**Locate this block:**

```rust
pub enum Action {
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
```

Replace it with:

```rust
pub enum Action {
    SelectRows {
        ids: Vec<ObjectId>,
        add: bool,
        remove: bool,
    },
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
```

## <a id="checkpoint-54b-finger"></a>4. Keep touch handles reachable without enlarging them

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
            Action::GizmoPress {
```

Replace it with:

```rust
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
            Action::GizmoTouch {
                pointer,
                screen,
                size,
            } => return self.gizmo_press_radius(pointer, screen, size, false, 18.),
            Action::GizmoPress {
```

**Locate this block:**

```rust
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
```

Replace it with:

```rust
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
```

**Locate this block:**

```rust
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
```

Replace it with:

```rust
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoTouch { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
```

**Locate this block:**

```rust
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit(screen, size) else {
            return Ok(Change::View);
        };
        let origin = self
```

Replace it with:

```rust
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit_radius(screen, size, radius) else {
            return Ok(Change::View);
        };
        let origin = self
```

**Locate this block:**

```rust
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
```

Replace it with:

```rust
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        self.gizmo_press_radius(pointer, screen, size, modified, 8.)
    }

    fn gizmo_press_radius(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
        radius: f64,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
```

**Locate this block:**

```rust
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit(&ray.origin, &ray.direction, scale)
    }

    pub fn number_prompt(
```

Replace it with:

```rust
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
```

**Locate this block:**

```rust
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
```

Replace it with:

```rust
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        self.gizmo_hit_radius(screen, size, 8.)
    }

    pub fn gizmo_hit_radius(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
        radius: f64,
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
```

**Locate this block:**

```rust
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
```

Replace it with:

```rust
    },
    GizmoTouch {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
```

## <a id="checkpoint-54c-rectangle"></a>5. Distinguish window and crossing rectangles

### 1. `src/rect_select.rs`

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub bounds: [f64; 4],
    pub crossing: bool,
}

impl Rectangle {
    pub fn new(from: [f64; 2], to: [f64; 2]) -> Option<Self> {
        if from
            .into_iter()
            .chain(to)
            .any(|value| !value.is_finite())
        {
            return None;
        }
        let bounds = [
            from[0].min(to[0]),
            from[1].min(to[1]),
            from[0].max(to[0]),
            from[1].max(to[1]),
        ];
        (bounds[0] < bounds[2] && bounds[1] < bounds[3]).then_some(Self {
            bounds,
            crossing: to[0] < from[0],
        })
    }

    pub fn contains(&self, point: [f64; 2]) -> bool {
        point.iter().all(|value| value.is_finite())
            && point[0] >= self.bounds[0]
            && point[0] <= self.bounds[2]
            && point[1] >= self.bounds[1]
            && point[1] <= self.bounds[3]
    }

    pub fn crosses(&self, a: [f64; 2], b: [f64; 2]) -> bool {
        if a.into_iter()
            .chain(b)
            .any(|value| !value.is_finite())
        {
            return false;
        }
        let [left, top, right, bottom] = self.bounds;
        let [dx, dy] = [b[0] - a[0], b[1] - a[1]];
        let (mut low, mut high) = (0_f64, 1_f64);
        for (p, q) in [
            (-dx, a[0] - left),
            (dx, right - a[0]),
            (-dy, a[1] - top),
            (dy, bottom - a[1]),
        ] {
            if p == 0. {
                if q < 0. {
                    return false;
                }
                continue;
            }
            let t = q / p;
            if p < 0. {
                low = low.max(t);
            } else {
                high = high.min(t);
            }
        }
        low <= high
    }
}

pub fn screen(matrix: &[f64; 16], point: [f64; 3], size: [f64; 2]) -> Option<[f64; 2]> {
    let clip = |row: usize| {
        point
            .iter()
            .enumerate()
            .map(|(i, value)| matrix[i * 4 + row] * value)
            .sum::<f64>()
            + matrix[12 + row]
    };
    let w = clip(3);
    if !w.is_finite()
        || w <= 0.
        || size
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.)
    {
        return None;
    }
    let point = [
        (clip(0) / w + 1.) * size[0] / 2.,
        (1. - clip(1) / w) * size[1] / 2.,
    ];
    point
        .iter()
        .all(|value| value.is_finite())
        .then_some(point)
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod layer_ui_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod layer_ui_tests;

pub mod rect_select;
```

## <a id="checkpoint-54d-samples"></a>6. Select projected geometry in one source pass

### 1. `src/rect_select.rs`

**Locate this existing block:**

```rust
        .then_some(point)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .then_some(point)
}

fn world(matrix: &session_rust::Xform, point: [f64; 3]) -> [f64; 3] {
    let point = matrix.transform_point(&session_rust::Point::new(
        point[0], point[1], point[2],
    ));
    [point[0], point[1], point[2]]
}

fn matches(
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
    strokes: Vec<Vec<[f64; 3]>>,
) -> bool {
    if strokes.is_empty() {
        return false;
    }
    if !rectangle.crossing {
        return strokes.iter().flatten().all(|point| {
            screen(matrix, *point, size).is_some_and(|point| rectangle.contains(point))
        });
    }
    strokes.iter().any(|stroke| {
        let points = stroke
            .iter()
            .map(|point| screen(matrix, *point, size))
            .collect::<Vec<_>>();
        points
            .iter()
            .flatten()
            .any(|point| rectangle.contains(*point))
            || points.windows(2).any(
                |pair| matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)),
            )
    })
}

pub fn found(
    scene: &Scene,
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
) -> Vec<ObjectId> {
    let eligible = |id: ObjectId| scene.pick_id(id.value()).is_some();
    let mut found = Vec::new();
    for row in scene
        .objects()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut strokes = row
            .mesh
            .parts
            .endpoints
            .iter()
            .map(|ends| {
                ends.iter()
                    .map(|point| world(&row.model, *point))
                    .collect()
            })
            .collect::<Vec<_>>();
        if let Some(source) = row.geometry() {
            strokes.extend(
                source
                    .vertices()
                    .into_iter()
                    .filter_map(|key| source.vertex_point(key))
                    .map(|point| {
                        vec![world(
                            &row.model,
                            [point[0], point[1], point[2]],
                        )]
                    }),
            );
        }
        if matches(matrix, size, rectangle, strokes) {
            found.push(row.id);
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            matrix,
            size,
            rectangle,
            vec![
                row.endpoints()
                    .map(|point| [point[0], point[1], point[2]])
                    .to_vec(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if matches(
            matrix,
            size,
            rectangle,
            vec![
                row.points()
                    .into_iter()
                    .map(|point| [point[0], point[1], point[2]])
                    .collect(),
            ],
        ) {
            found.push(row.id);
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let point = row.point();
        if matches(
            matrix,
            size,
            rectangle,
            vec![vec![[point[0], point[1], point[2]]]],
        ) {
            found.push(row.id);
        }
    }
    found.sort_unstable_by_key(|id| id.value());
    found.dedup();
    found
}
```

### 2. `src/rect_select_tests.rs`

Create the file and type:

```rust
use crate::rect_select::{Rectangle, found, screen};

#[test]
fn direction_and_segment_clipping_distinguish_window_from_crossing() {
    let window = Rectangle::new([40., 40.], [60., 60.]).unwrap();
    let crossing = Rectangle::new([60., 40.], [40., 60.]).unwrap();
    assert!(!window.crossing && crossing.crossing);
    assert_eq!(window.bounds, crossing.bounds);
    assert!(crossing.crosses([0., 50.], [100., 50.]));
    assert!(!crossing.contains([0., 50.]) && !crossing.contains([100., 50.]));
    assert!(!crossing.crosses([0., 20.], [100., 20.]));
    assert!(crossing.crosses([40., 0.], [40., 100.]));
    assert!(Rectangle::new([0., 0.], [0., 3.]).is_none());
    assert!(Rectangle::new([f64::NAN, 0.], [3., 3.]).is_none());
    assert!(!crossing.crosses([f64::INFINITY, 0.], [1., 2.]));
}

#[test]
fn projection_uses_css_sizes_and_rejects_behind_eye_and_invalid_dimensions() {
    let identity = session_rust::Xform::identity().m;
    assert_eq!(
        screen(&identity, [0., 0., 0.], [640., 480.]),
        Some([320., 240.])
    );
    assert_eq!(
        screen(&identity, [0.5, 0.5, 0.], [640., 480.]),
        Some([480., 120.])
    );
    let mut behind = identity;
    behind[15] = -1.;
    assert!(screen(&behind, [0.; 3], [640., 480.]).is_none());
    assert!(screen(&identity, [0.; 3], [0., 480.]).is_none());
    assert!(screen(&identity, [f64::NAN, 0., 0.], [640., 480.]).is_none());
}

#[test]
fn mixed_precise_sources_are_selected_by_projected_geometry_and_current_eligibility() {
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::Close)
        .unwrap();
    let line = e
        .scene
        .insert_line(
            crate::stroke::PreparedLine::new(std::rc::Rc::new(
                session_rust::Line::new(-0.8, 0., 0., 0.8, 0., 0.),
            ))
            .unwrap(),
        )
        .unwrap();
    let point = e
        .scene
        .insert_point(
            crate::marker::PreparedPoint::new(std::rc::Rc::new(
                session_rust::Point::new(0., 0., 0.),
            ))
            .unwrap(),
        )
        .unwrap();
    let identity = session_rust::Xform::identity().m;
    let window = Rectangle::new([90., 90.], [110., 110.]).unwrap();
    let crossing = Rectangle::new([110., 90.], [90., 110.]).unwrap();
    assert_eq!(
        found(&e.scene, &identity, [200., 200.], window),
        vec![point]
    );
    assert_eq!(
        found(&e.scene, &identity, [200., 200.], crossing),
        vec![line, point]
    );
    std::rc::Rc::make_mut(&mut e.scene.view).lock(&[point], true);
    assert_eq!(
        found(&e.scene, &identity, [200., 200.], crossing),
        vec![line]
    );
    std::rc::Rc::make_mut(&mut e.scene.view).hide(&[line], true);
    assert!(found(&e.scene, &identity, [200., 200.], crossing).is_empty());
    assert_eq!(
        e.scene.lines()[0].prepared.source.start()[0],
        -0.8
    );
    assert_eq!(e.scene.points()[0].prepared.source[0], 0.);
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod layer_ui_tests;

pub mod rect_select;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod layer_ui_tests;

pub mod rect_select;

#[cfg(test)]
mod rect_select_tests;
```

## <a id="checkpoint-54e-drag"></a>7. Freeze a released drag until its pick answers

### 1. `src/canvas_drag.rs`

Create the file and type:

```rust
use crate::{
    camera::Camera,
    scene::{ObjectId, Scene},
};
use session_rust::{Point, Vector, Xform};

pub enum Answer {
    Follow,
    Move {
        add: bool,
        remove: bool,
    },
    Rectangle,
    Select {
        add: bool,
        remove: bool,
    },
}
pub struct Drag {
    pub from: [f32; 2],
    pub to: [f32; 2],
    pub size: [f64; 2],
    pub pointer: i32,
    pub released: Option<(bool, bool)>,
    pub modified: bool,
    pub missed: bool,
    pub preview: Option<crate::edit_preview::Preview>,
    plane: Option<([f64; 3], Vector)>,
    start: Option<[f64; 3]>,
}
impl Drag {
    pub fn new(
        from: [f32; 2],
        to: [f32; 2],
        size: [f64; 2],
        modified: bool,
    ) -> Option<Self> {
        if from
            .iter()
            .chain(to.iter())
            .any(|v| !v.is_finite() || v.abs() > 1.)
            || size
                .iter()
                .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        Some(Self {
            from,
            to,
            size,
            pointer: 0,
            released: None,
            modified,
            missed: modified,
            preview: None,
            plane: None,
            start: None,
        })
    }
    pub fn answer(&self, hit: bool) -> Answer {
        match (hit, self.released) {
            (true, None) => Answer::Follow,
            (true, Some((add, remove))) => Answer::Move { add, remove },
            (false, None) => Answer::Rectangle,
            (false, Some((add, remove))) => Answer::Select { add, remove },
        }
    }
    pub fn rectangle(&self) -> Option<crate::rect_select::Rectangle> {
        let css = |p: [f32; 2]| {
            [
                (p[0] as f64 + 1.) * self.size[0] / 2.,
                (1. - p[1] as f64) * self.size[1] / 2.,
            ]
        };
        crate::rect_select::Rectangle::new(css(self.from), css(self.to))
    }
    pub fn grab(
        &mut self,
        scene: &Scene,
        ids: Vec<ObjectId>,
        camera: &Camera,
    ) -> Result<(), &'static str> {
        let origin = scene
            .bounds_of(&ids)
            .ok_or("No drag bounds")?
            .centre();
        let view = camera
            .orientation
            .rotate_vector(Vector::y_axis());
        let normal = if view[2].abs() < 0.34 {
            Vector::new(view[0], view[1], 0.).normalized()
        } else {
            Vector::z_axis()
        };
        self.plane = Some((origin, normal));
        self.start = Some(
            self.point(camera, self.from)
                .ok_or("Drag plane is parallel to the view")?,
        );
        self.preview = Some(crate::edit_preview::Preview::new(
            scene, ids,
        )?);
        self.update(camera)
    }
    fn point(&self, camera: &Camera, screen: [f32; 2]) -> Option<[f64; 3]> {
        let (origin, normal) = self.plane.as_ref()?;
        let ray = camera.ray_unbounded(screen)?;
        let denominator = ray.direction.dot(normal);
        if denominator.abs() < 1e-9 {
            return None;
        }
        let distance = (&Point::new(origin[0], origin[1], origin[2]) - &ray.origin)
            .dot(normal)
            / denominator;
        if !distance.is_finite() || distance < 0. {
            return None;
        }
        Some(std::array::from_fn(|i| {
            ray.origin[i] + distance * ray.direction[i]
        }))
    }
    pub fn update(&mut self, camera: &Camera) -> Result<(), &'static str> {
        let Some(point) = self.point(camera, self.to) else {
            return Ok(());
        };
        let Some(start) = self.start else {
            return Ok(());
        };
        if let Some(preview) = self.preview.as_mut() {
            preview.set(Xform::translation(
                point[0] - start[0],
                point[1] - start[1],
                point[2] - start[2],
            ))?;
        }
        Ok(())
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod rect_select_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod rect_select_tests;

pub mod canvas_drag;
```

## <a id="checkpoint-54f-drag-state"></a>8. Own drag previews outside document history

### 1. `src/editor.rs`

**Locate this existing block:**

```rust
pub enum Action {
    SelectRows {
        ids: Vec<ObjectId>,
        add: bool,
        remove: bool,
    },
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
    },
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
    },
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
    },
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
    },
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    },
    GizmoTouch {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoRelease {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
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
    Hide,
    Show,
    AddBox,
    AddNested,
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
    Preview,
}

pub struct Editor {
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
    gizmo_hover: Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
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
            gizmo_edit: None,
            gizmo_hover: None,
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
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
    pub fn display_scene(&self) -> &Scene {
        self.gizmo_edit.as_ref().map_or_else(
            || {
                self.edit_preview
                    .as_ref()
                    .map_or(&self.scene, |p| &p.scene)
            },
            |g| g.preview.scene(),
        )
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
        if self.draft.is_some() {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
        if let Some(target) = self
            .selection()
            .filter(|_| self.selected_part.is_some())
        {
            return crate::part_preview::origin(&self.scene, target);
        }
        self.scene
            .bounds_of(&self.selected_objects())
            .map(|b| b.centre())
    }

    pub fn gizmo_hit(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        self.gizmo_hit_radius(screen, size, 8.)
    }

    pub fn gizmo_hit_radius(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
        radius: f64,
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::world_per_pixel(
            &self.camera.view_projection(),
            origin,
            size[1],
        )?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let matrix = self.camera.view_projection();
        let scale = crate::widget_layout::world_per_pixel(&matrix, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen {
            matrix: matrix.m,
            size,
        }
        .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
            title: handle.title(),
            unit,
            hint: if unit == "factor" {
                "1"
            } else {
                "0"
            },
            at: at.map(|v| v as f32),
        })
    }

    pub fn gizmo_active(&self) -> f32 {
        crate::gizmo_edit::code(
            self.gizmo_edit
                .as_ref()
                .and_then(|e| e.active())
                .or(self.gizmo_hover),
        )
    }

    pub fn hover_gizmo(&mut self, screen: Option<([f32; 2], [f64; 2])>) {
        self.gizmo_hover = if self.gizmo_edit.is_some() {
            None
        } else {
            screen.and_then(|(point, size)| self.gizmo_hit(point, size))
        };
    }

    fn gizmo_press(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        self.gizmo_press_radius(pointer, screen, size, modified, 8.)
    }

    fn gizmo_press_radius(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
        radius: f64,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
                .gizmo_edit
                .as_ref()
                .is_some_and(|g| g.pointer.is_some())
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit_radius(screen, size, radius) else {
            return Ok(Change::View);
        };
        let origin = self
            .gizmo_origin()
            .ok_or("No editable selection")?;
        let preview = crate::gizmo_edit::Preview::new(
            &self.scene,
            self.selected_objects(),
            self.selection()
                .filter(|_| self.selected_part.is_some()),
        )?;
        let ray = self
            .camera
            .ray(screen)
            .ok_or("Invalid handle ray")?;
        let gizmo = crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ));
        self.gizmo_edit = crate::gizmo_edit::Edit::begin(
            gizmo, handle, &ray, preview, pointer, screen,
        );
        Ok(Change::View)
    }

    fn gizmo_move(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return Err("Invalid handle viewport");
        }
        let Some(ray) = self.camera.ray(screen) else {
            return Err("Invalid handle ray");
        };
        let Some(edit) = self.gizmo_edit.as_mut() else {
            return Ok(Change::View);
        };
        Ok(
            if edit.update(pointer, screen, size, &ray, shift)? {
                Change::Preview
            } else {
                Change::View
            },
        )
    }

    fn gizmo_release(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if self
            .gizmo_edit
            .as_ref()
            .is_none_or(|g| g.pointer != Some(pointer))
        {
            return Ok(Change::View);
        }
        self.gizmo_move(pointer, screen, size, shift)?;
        let edit = self.gizmo_edit.as_mut().unwrap();
        if edit.click() {
            self.tool_status = format!(
                "{}: type a value in {}, Enter applies, Esc closes",
                edit.active().unwrap().title(),
                edit.active().unwrap().labels().2
            );
            return Ok(Change::Preview);
        }
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = "Handle edit completed.".into();
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn gizmo_number(&mut self, text: &str) -> Result<Change, &'static str> {
        let edit = self
            .gizmo_edit
            .as_mut()
            .ok_or("No handle number is being requested")?;
        let handle = edit
            .gizmo
            .typing
            .ok_or("No handle number is being requested")?;
        let value = crate::gizmo::typed_value(handle, text)
            .map_err(|_| "Enter a finite number; scale needs a positive factor")?;
        let Some(value) = value else {
            self.gizmo_edit = None;
            return Ok(Change::Preview);
        };
        edit.preview
            .set(edit.gizmo.typed(handle, value))?;
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = format!(
            "{} {value} {}",
            handle.title(),
            handle.labels().2
        );
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
        let offset = self
            .draft
            .as_ref()
            .and_then(|d| d.points.first().zip(d.cursor))
            .map(|(a, b)| std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]))
            .unwrap_or([0.; 3]);
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
                self.edit_preview
                    .iter()
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.gizmo_edit
                    .iter()
                    .flat_map(|g| g.preview.scenes()),
            )
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
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
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
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let ids = self.scene.with_descendants(&[id]);
        let delta = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    let matrix = &delta
                        * &scene
                            .placement(id)
                            .ok_or("Object not found")?;
                    scene.place_following(id, matrix)?;
                }
                Ok(())
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
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
        let ids = self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
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

    pub fn select_rows(&mut self, ids: Vec<ObjectId>, add: bool, remove: bool) {
        if remove {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
            self.objects.remove(&self.scene, ids);
            self.selected = self.objects.primary();
            self.selected_part = None;
        } else {
            self.select_objects(ids, add);
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub enum Action {
    CanvasStart {
        from: [f32; 2],
        to: [f32; 2],
        size: [f64; 2],
        modified: bool,
        pointer: i32,
    },
    CanvasMove {
        screen: [f32; 2],
        size: [f64; 2],
    },
    CanvasRelease {
        screen: [f32; 2],
        size: [f64; 2],
        add: bool,
        remove: bool,
    },
    CanvasHit(Option<ObjectId>),
    CanvasCancel,
    SelectRows {
        ids: Vec<ObjectId>,
        add: bool,
        remove: bool,
    },
    PanScreen {
        delta: [f64; 2],
        size: [f64; 2],
    },
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    Touch(Vec<Action>),
    LayerEdit {
        key: String,
        edit: crate::layer_edit::Edit,
    },
    LayerProperty {
        key: String,
        property: crate::layer_properties::Property,
    },
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
    },
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    },
    GizmoTouch {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
    },
    GizmoMove {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoRelease {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    },
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
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
    Hide,
    Show,
    AddBox,
    AddNested,
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
    Preview,
}

pub struct Editor {
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
    gizmo_hover: Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
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
            canvas_drag: None,
            gizmo_edit: None,
            gizmo_hover: None,
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
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
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag
            .as_ref()
            .and_then(|d| d.preview.as_ref())
            .map_or_else(
                || {
                    self.gizmo_edit.as_ref().map_or_else(
                        || {
                            self.edit_preview
                                .as_ref()
                                .map_or(&self.scene, |p| &p.scene)
                        },
                        |g| g.preview.scene(),
                    )
                },
                |p| &p.scene,
            )
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
        if self.draft.is_some() || self.canvas_drag.is_some() {
            return None;
        }
        if let Some(edit) = self.gizmo_edit.as_ref() {
            return Some(edit.origin());
        }
        if let Some(target) = self
            .selection()
            .filter(|_| self.selected_part.is_some())
        {
            return crate::part_preview::origin(&self.scene, target);
        }
        self.scene
            .bounds_of(&self.selected_objects())
            .map(|b| b.centre())
    }

    pub fn gizmo_hit(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
    ) -> Option<crate::gizmo::Handle> {
        self.gizmo_hit_radius(screen, size, 8.)
    }

    pub fn gizmo_hit_radius(
        &self,
        screen: [f32; 2],
        size: [f64; 2],
        radius: f64,
    ) -> Option<crate::gizmo::Handle> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let origin = self.gizmo_origin()?;
        let scale = crate::widget_layout::world_per_pixel(
            &self.camera.view_projection(),
            origin,
            size[1],
        )?;
        let ray = self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ))
        .hit_with_radius(&ray.origin, &ray.direction, scale, radius)
    }

    pub fn number_prompt(
        &self,
        size: [f64; 2],
    ) -> Option<crate::number_box::NumberPrompt> {
        let edit = self.gizmo_edit.as_ref()?;
        let handle = edit.gizmo.typing?;
        let origin = edit.origin();
        let matrix = self.camera.view_projection();
        let scale = crate::widget_layout::world_per_pixel(&matrix, origin, size[1])?;
        let point = edit.gizmo.handle_point(handle, scale);
        let at = crate::snap_math::Screen {
            matrix: matrix.m,
            size,
        }
        .point([point[0], point[1], point[2]])?;
        let unit = handle.labels().2;
        Some(crate::number_box::NumberPrompt {
            handle,
            title: handle.title(),
            unit,
            hint: if unit == "factor" {
                "1"
            } else {
                "0"
            },
            at: at.map(|v| v as f32),
        })
    }

    pub fn gizmo_active(&self) -> f32 {
        crate::gizmo_edit::code(
            self.gizmo_edit
                .as_ref()
                .and_then(|e| e.active())
                .or(self.gizmo_hover),
        )
    }

    pub fn hover_gizmo(&mut self, screen: Option<([f32; 2], [f64; 2])>) {
        self.gizmo_hover = if self.gizmo_edit.is_some() {
            None
        } else {
            screen.and_then(|(point, size)| self.gizmo_hit(point, size))
        };
    }

    fn gizmo_press(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    ) -> Result<Change, &'static str> {
        self.gizmo_press_radius(pointer, screen, size, modified, 8.)
    }

    fn gizmo_press_radius(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
        radius: f64,
    ) -> Result<Change, &'static str> {
        if modified
            || self.draft.is_some()
            || self
                .gizmo_edit
                .as_ref()
                .is_some_and(|g| g.pointer.is_some())
        {
            return Ok(Change::View);
        }
        let Some(handle) = self.gizmo_hit_radius(screen, size, radius) else {
            return Ok(Change::View);
        };
        let origin = self
            .gizmo_origin()
            .ok_or("No editable selection")?;
        let preview = crate::gizmo_edit::Preview::new(
            &self.scene,
            self.selected_objects(),
            self.selection()
                .filter(|_| self.selected_part.is_some()),
        )?;
        let ray = self
            .camera
            .ray(screen)
            .ok_or("Invalid handle ray")?;
        let gizmo = crate::gizmo::Gizmo::new(session_rust::Point::new(
            origin[0], origin[1], origin[2],
        ));
        self.gizmo_edit = crate::gizmo_edit::Edit::begin(
            gizmo, handle, &ray, preview, pointer, screen,
        );
        Ok(Change::View)
    }

    fn gizmo_move(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return Err("Invalid handle viewport");
        }
        let Some(ray) = self.camera.ray(screen) else {
            return Err("Invalid handle ray");
        };
        let Some(edit) = self.gizmo_edit.as_mut() else {
            return Ok(Change::View);
        };
        Ok(
            if edit.update(pointer, screen, size, &ray, shift)? {
                Change::Preview
            } else {
                Change::View
            },
        )
    }

    fn gizmo_release(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        shift: bool,
    ) -> Result<Change, &'static str> {
        if self
            .gizmo_edit
            .as_ref()
            .is_none_or(|g| g.pointer != Some(pointer))
        {
            return Ok(Change::View);
        }
        self.gizmo_move(pointer, screen, size, shift)?;
        let edit = self.gizmo_edit.as_mut().unwrap();
        if edit.click() {
            self.tool_status = format!(
                "{}: type a value in {}, Enter applies, Esc closes",
                edit.active().unwrap().title(),
                edit.active().unwrap().labels().2
            );
            return Ok(Change::Preview);
        }
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = "Handle edit completed.".into();
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn gizmo_number(&mut self, text: &str) -> Result<Change, &'static str> {
        let edit = self
            .gizmo_edit
            .as_mut()
            .ok_or("No handle number is being requested")?;
        let handle = edit
            .gizmo
            .typing
            .ok_or("No handle number is being requested")?;
        let value = crate::gizmo::typed_value(handle, text)
            .map_err(|_| "Enter a finite number; scale needs a positive factor")?;
        let Some(value) = value else {
            self.gizmo_edit = None;
            return Ok(Change::Preview);
        };
        edit.preview
            .set(edit.gizmo.typed(handle, value))?;
        let changed = edit
            .preview
            .commit(&mut self.scene, &mut self.history)?;
        self.gizmo_edit = None;
        self.tool_status = format!(
            "{} {value} {}",
            handle.title(),
            handle.labels().2
        );
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
        let offset = self
            .draft
            .as_ref()
            .and_then(|d| d.points.first().zip(d.cursor))
            .map(|(a, b)| std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]))
            .unwrap_or([0.; 3]);
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
                self.canvas_drag
                    .iter()
                    .filter_map(|d| d.preview.as_ref())
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.edit_preview
                    .iter()
                    .flat_map(|p| p.scenes()),
            )
            .chain(
                self.gizmo_edit
                    .iter()
                    .flat_map(|g| g.preview.scenes()),
            )
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
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
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
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let ids = self.scene.with_descendants(&[id]);
        let delta = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    let matrix = &delta
                        * &scene
                            .placement(id)
                            .ok_or("Object not found")?;
                    scene.place_following(id, matrix)?;
                }
                Ok(())
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
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
        let ids = self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) {
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
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

    pub fn select_rows(&mut self, ids: Vec<ObjectId>, add: bool, remove: bool) {
        if remove {
            let old = self.selected_objects();
            self.objects.set(&self.scene, old, false);
            self.objects.remove(&self.scene, ids);
            self.selected = self.objects.primary();
            self.selected_part = None;
        } else {
            self.select_objects(ids, add);
        }
    }

    fn finish_canvas(&mut self) -> Result<Change, &'static str> {
        let Some(mut drag) = self.canvas_drag.take() else {
            return Ok(Change::View);
        };
        let (add, remove) = drag.released.unwrap_or((false, false));
        if let Some(preview) = drag.preview.take() {
            let changed = preview.commit(&mut self.scene, &mut self.history)?;
            if changed {
                self.snap_index = None;
            }
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        if drag.missed {
            if let Some(rectangle) = drag.rectangle() {
                let ids = crate::rect_select::found(
                    &self.scene,
                    &self.camera.view_projection().m,
                    drag.size,
                    rectangle,
                );
                self.select_rows(ids, add, remove);
            }
        }
        Ok(Change::View)
    }
```

## <a id="checkpoint-54g-drag-actions"></a>9. Apply held released and cancelled drag outcomes

### 1. `src/editor.rs`

**Locate this existing block:**

```rust
        match action {
            Action::SelectRows { ids, add, remove } => {
                self.select_rows(ids, add, remove);
                return Ok(Change::View);
            }
            _ => {}
        }
        let keep_gizmo = matches!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match action {
            Action::Touch(actions) => {
                let mut change = Change::View;
                for action in actions {
                    let next = self.apply(action)?;
                    if next != Change::View {
                        change = next;
                    }
                }
                return Ok(change);
            }
            Action::CanvasStart {
                from,
                to,
                size,
                modified,
                pointer,
            } => {
                self.canvas_drag =
                    crate::canvas_drag::Drag::new(from, to, size, modified);
                if let Some(drag) = self.canvas_drag.as_mut() {
                    drag.pointer = pointer;
                }
                return Ok(Change::View);
            }
            Action::CanvasMove { screen, size } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.released.is_none()
                        && drag.size == size
                        && screen.iter().all(|v| v.is_finite())
                    {
                        drag.to = screen;
                        drag.update(&self.camera)?;
                    }
                    if drag.preview.is_some() {
                        return Ok(Change::Preview);
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasRelease {
                screen,
                size,
                add,
                remove,
            } => {
                if let Some(drag) = self.canvas_drag.as_mut() {
                    if drag.size != size {
                        self.canvas_drag = None;
                        return Ok(Change::Preview);
                    }
                    drag.to = screen;
                    drag.released = Some((add, remove));
                    drag.update(&self.camera)?;
                    if drag.preview.is_some() || drag.missed {
                        return self.finish_canvas();
                    }
                }
                return Ok(Change::View);
            }
            Action::CanvasHit(hit) => {
                let Some(drag) = self.canvas_drag.as_ref() else {
                    return Ok(Change::View);
                };
                let ids = hit.filter(|id| {
                    !drag.modified && self.scene.pick_id(id.value()).is_some()
                });
                if let Some(id) = ids {
                    let mut rows = self.selected_objects();
                    if !rows.contains(&id) {
                        self.select_objects([id], false);
                        rows = vec![id];
                    }
                    if let Err(error) = self.canvas_drag.as_mut().unwrap().grab(
                        &self.scene,
                        rows,
                        &self.camera,
                    ) {
                        self.canvas_drag = None;
                        return Err(error);
                    }
                } else {
                    self.canvas_drag.as_mut().unwrap().missed = true;
                }
                if self
                    .canvas_drag
                    .as_ref()
                    .unwrap()
                    .released
                    .is_some()
                {
                    return self.finish_canvas();
                }
                return Ok(Change::Preview);
            }
            Action::CanvasCancel => {
                let preview = self
                    .canvas_drag
                    .take()
                    .is_some_and(|d| d.preview.is_some());
                return Ok(if preview {
                    Change::Preview
                } else {
                    Change::View
                });
            }
            Action::SelectRows { ids, add, remove } => {
                self.select_rows(ids, add, remove);
                return Ok(Change::View);
            }
            _ => {}
        }
        self.canvas_drag = None;
        let keep_gizmo = matches!(
```

## <a id="checkpoint-54h-mouse"></a>10. Capture mouse gestures in CSS pixels

### 1. `src/gesture.rs`

**Locate this existing block:**

```rust
    Orbit([f64; 2]),
}

impl Motion {
    pub fn action(self, rect: [f64; 4]) -> Option<Action> {
        let [left, top, width, height] = rect;
        if rect.iter().any(|n| !n.is_finite()) || width <= 0.0 || height <= 0.0 {
            return None;
        }
        Some(match self {
            Self::Pick([x, y]) => Action::Pick([
                (2.0 * (x - left) / width - 1.0) as f32,
                (1.0 - 2.0 * (y - top) / height) as f32,
            ]),
            Self::Orbit([dx, dy]) => Action::Orbit(
                -std::f64::consts::PI * dx / width,
                -std::f64::consts::PI * dy / height,
            ),
        })
    }
}

struct Drag {
    id: i32,
    button: i16,
    start: [f64; 2],
    last: [f64; 2],
    moved: bool,
}

#[derive(Default)]
pub struct Gesture {
    active: Option<Drag>,
}

impl Gesture {
    pub fn press(&mut self, id: i32, button: i16, position: [f64; 2]) -> bool {
        if self.active.is_some()
            || !matches!(button, 0 | 2)
            || position.iter().any(|n| !n.is_finite())
        {
            return false;
        }
        self.active = Some(Drag {
            id,
            button,
            start: position,
            last: position,
            moved: false,
        });
        true
    }

    pub fn move_to(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        let drag = self
            .active
            .as_mut()
            .filter(|drag| drag.id == id)?;
        if position.iter().any(|n| !n.is_finite()) {
            return None;
        }
        let delta = [
            position[0] - drag.last[0],
            position[1] - drag.last[1],
        ];
        drag.last = position;
        let distance = (position[0] - drag.start[0]).hypot(position[1] - drag.start[1]);
        // Once a press becomes a drag, returning to its start must not turn it into a click.
        drag.moved |= distance > 4.0;
        (drag.button == 2 && delta != [0.0, 0.0]).then_some(Motion::Orbit(delta))
    }

    pub fn release(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        if self.active.as_ref()?.id != id {
            return None;
        }
        let last_motion = self.move_to(id, position);
        let drag = self.active.take()?;
        if drag.button == 2 {
            return last_motion;
        }
        (!drag.moved && position.iter().all(|n| n.is_finite()))
            .then_some(Motion::Pick(position))
    }

    pub fn cancel(&mut self) {
        self.active = None;
    }

    pub fn cancel_pointer(&mut self, id: i32) {
        if self
            .active
            .as_ref()
            .is_some_and(|drag| drag.id == id)
        {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Orbit([f64; 2]),
    Pan([f64; 2]),
    Probe {
        from: [f64; 2],
        to: [f64; 2],
    },
    Drag([f64; 2]),
    Finish([f64; 2]),
}

impl Motion {
    pub fn action(self, rect: [f64; 4]) -> Option<Action> {
        let [left, top, width, height] = rect;
        if rect.iter().any(|v| !v.is_finite()) || width <= 0. || height <= 0. {
            return None;
        }
        let screen = |[x, y]: [f64; 2]| {
            [
                (2. * (x - left) / width - 1.) as f32,
                (1. - 2. * (y - top) / height) as f32,
            ]
        };
        let size = [width, height];
        Some(match self {
            Self::Pick(at) => Action::Pick(screen(at)),
            Self::Orbit([dx, dy]) => Action::Orbit(-0.005 * dx, -0.005 * dy),
            Self::Pan(delta) => Action::PanScreen { delta, size },
            Self::Probe { from, to } => Action::CanvasStart {
                from: screen(from),
                to: screen(to),
                size,
                modified: false,
                pointer: 0,
            },
            Self::Drag(at) => Action::CanvasMove {
                screen: screen(at),
                size,
            },
            Self::Finish(at) => Action::CanvasRelease {
                screen: screen(at),
                size,
                add: false,
                remove: false,
            },
        })
    }
}

struct Drag {
    id: i32,
    button: i16,
    start: [f64; 2],
    last: [f64; 2],
    moved: bool,
    pan: bool,
}
#[derive(Default)]
pub struct Gesture {
    active: Option<Drag>,
    repeat: bool,
}
impl Gesture {
    pub fn press(&mut self, id: i32, button: i16, position: [f64; 2]) -> bool {
        self.press_with(id, button, position, false)
    }
    pub fn press_with(
        &mut self,
        id: i32,
        button: i16,
        position: [f64; 2],
        pan: bool,
    ) -> bool {
        if self.active.is_some()
            || !matches!(button, 0 | 1 | 2)
            || position.iter().any(|v| !v.is_finite())
        {
            return false;
        }
        self.repeat = false;
        self.active = Some(Drag {
            id,
            button,
            start: position,
            last: position,
            moved: false,
            pan: pan || button == 1,
        });
        true
    }
    pub fn move_to(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        let drag = self.active.as_mut().filter(|d| d.id == id)?;
        if position.iter().any(|v| !v.is_finite()) {
            return None;
        }
        let was = drag.moved;
        drag.moved |= (position[0] - drag.start[0])
            .abs()
            .max((position[1] - drag.start[1]).abs())
            > 4.;
        let from = if was {
            drag.last
        } else {
            drag.start
        };
        let delta = std::array::from_fn(|i| position[i] - from[i]);
        drag.last = position;
        if !drag.moved {
            return None;
        }
        if drag.button == 0 {
            Some(if was {
                Motion::Drag(position)
            } else {
                Motion::Probe {
                    from: drag.start,
                    to: position,
                }
            })
        } else if delta == [0.; 2] {
            None
        } else {
            Some(if drag.pan {
                Motion::Pan(delta)
            } else {
                Motion::Orbit(delta)
            })
        }
    }
    pub fn release(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        if self.active.as_ref()?.id != id {
            return None;
        }
        let motion = self.move_to(id, position);
        let drag = self.active.take()?;
        if position.iter().any(|v| !v.is_finite()) {
            return None;
        }
        if drag.button != 0 {
            self.repeat = drag.button == 2 && !drag.moved;
            return motion;
        }
        Some(if drag.moved {
            if matches!(motion, Some(Motion::Probe { .. })) {
                motion.unwrap()
            } else {
                Motion::Finish(position)
            }
        } else {
            Motion::Pick(position)
        })
    }
    pub fn take_repeat(&mut self) -> bool {
        std::mem::take(&mut self.repeat)
    }
    pub fn cancel(&mut self) {
        self.active = None;
        self.repeat = false;
    }
    pub fn cancel_pointer(&mut self, id: i32) {
        if self
            .active
            .as_ref()
            .is_some_and(|d| d.id == id)
        {
```

### 2. `src/gesture_tests.rs`

**Locate this existing block:**

```rust
#[test]
fn a_click_selects_but_a_left_drag_does_not() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(
        input.release(1, [12.0, 20.0]),
        Some(Motion::Pick([12.0, 20.0]))
    );
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.move_to(1, [20.0, 20.0]), None);
    assert_eq!(input.release(1, [10.0, 20.0]), None);
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.release(1, [30.0, 20.0]), None);
}

#[test]
fn only_the_captured_pointer_can_move_or_finish_the_gesture() {
    let mut input = Gesture::default();
    assert!(input.press(1, 2, [0.0, 0.0]));
    assert!(!input.press(2, 0, [0.0, 0.0]));
    assert_eq!(input.move_to(2, [100.0, 50.0]), None);
    assert_eq!(input.release(2, [0.0, 0.0]), None);
    input.cancel_pointer(2);
    assert_eq!(
        input.move_to(1, [10.0, 5.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        input.move_to(1, [30.0, 8.0]),
        Some(Motion::Orbit([20.0, 3.0]))
    );
    assert_eq!(input.release(1, [30.0, 8.0]), None);
    assert_eq!(input.move_to(1, [40.0, 8.0]), None);
    input.press(3, 2, [0.0, 0.0]);
    assert_eq!(
        input.release(3, [5.0, 2.0]),
        Some(Motion::Orbit([5.0, 2.0]))
    );
}

#[test]
fn cancellation_never_selects_and_allows_a_fresh_press() {
    let mut input = Gesture::default();
    input.press(1, 0, [0.0, 0.0]);
    input.cancel_pointer(1);
    assert_eq!(input.release(1, [0.0, 0.0]), None);
    assert!(input.press(2, 2, [0.0, 0.0]));
    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(!input.press(1, 1, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn a_click_selects_but_a_left_drag_starts_a_probe() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(
        input.release(1, [12.0, 20.0]),
        Some(Motion::Pick([12.0, 20.0]))
    );
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(
        input.move_to(1, [20.0, 20.0]),
        Some(Motion::Probe {
            from: [10., 20.],
            to: [20., 20.]
        })
    );
    assert_eq!(
        input.release(1, [10.0, 20.0]),
        Some(Motion::Finish([10., 20.]))
    );
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(
        input.release(1, [30.0, 20.0]),
        Some(Motion::Probe {
            from: [10., 20.],
            to: [30., 20.]
        })
    );
}

#[test]
fn only_the_captured_pointer_can_move_or_finish_the_gesture() {
    let mut input = Gesture::default();
    assert!(input.press(1, 2, [0.0, 0.0]));
    assert!(!input.press(2, 0, [0.0, 0.0]));
    assert_eq!(input.move_to(2, [100.0, 50.0]), None);
    assert_eq!(input.release(2, [0.0, 0.0]), None);
    input.cancel_pointer(2);
    assert_eq!(
        input.move_to(1, [10.0, 5.0]),
        Some(Motion::Orbit([10.0, 5.0]))
    );
    assert_eq!(
        input.move_to(1, [30.0, 8.0]),
        Some(Motion::Orbit([20.0, 3.0]))
    );
    assert_eq!(input.release(1, [30.0, 8.0]), None);
    assert_eq!(input.move_to(1, [40.0, 8.0]), None);
    input.press(3, 2, [0.0, 0.0]);
    assert_eq!(
        input.release(3, [5.0, 2.0]),
        Some(Motion::Orbit([5.0, 2.0]))
    );
}

#[test]
fn cancellation_never_selects_and_allows_a_fresh_press() {
    let mut input = Gesture::default();
    input.press(1, 0, [0.0, 0.0]);
    input.cancel_pointer(1);
    assert_eq!(input.release(1, [0.0, 0.0]), None);
    assert!(input.press(2, 2, [0.0, 0.0]));
    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(input.press(1, 1, [0.0, 0.0]));
    assert_eq!(
        input.move_to(1, [10., 0.]),
        Some(Motion::Pan([10., 0.]))
    );
    input.cancel();
    assert!(!input.press(1, 3, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
```

## <a id="checkpoint-54i-touch"></a>11. Orbit with one finger and pan pinch with two

### 1. `src/touch.rs`

Create the file and type:

```rust
use crate::{editor::Action, gesture::Motion};

struct Finger {
    id: i32,
    from: [f64; 2],
    at: [f64; 2],
    time: f64,
    moved: bool,
}
#[derive(Default)]
pub struct Touch {
    fingers: Vec<Finger>,
    multiple: bool,
    tap: Option<([f64; 2], f64)>,
}
impl Touch {
    pub fn press(&mut self, id: i32, at: [f64; 2], time: f64) -> bool {
        if at.iter().any(|v| !v.is_finite())
            || !time.is_finite()
            || self.fingers.iter().any(|f| f.id == id)
        {
            return false;
        }
        self.multiple |= !self.fingers.is_empty();
        self.fingers.push(Finger {
            id,
            from: at,
            at,
            time,
            moved: false,
        });
        true
    }
    pub fn move_to(&mut self, id: i32, at: [f64; 2], rect: [f64; 4]) -> Vec<Action> {
        if at.iter().any(|v| !v.is_finite()) {
            return Vec::new();
        }
        let Some(index) = self.fingers.iter().position(|f| f.id == id) else {
            return Vec::new();
        };
        let old = self
            .fingers
            .iter()
            .map(|f| f.at)
            .collect::<Vec<_>>();
        let finger = &mut self.fingers[index];
        let was = finger.moved;
        finger.at = at;
        finger.moved |= (at[0] - finger.from[0])
            .abs()
            .max((at[1] - finger.from[1]).abs())
            > 4.;
        if self.fingers.len() == 1 && !self.multiple {
            if !self.fingers[0].moved {
                return Vec::new();
            }
            let from = if was {
                old[0]
            } else {
                self.fingers[0].from
            };
            let delta = std::array::from_fn(|i| at[i] - from[i]);
            return Motion::Orbit(delta)
                .action(rect)
                .into_iter()
                .collect();
        }
        if self.fingers.len() != 2 {
            return Vec::new();
        }
        let centre = |a: [f64; 2], b: [f64; 2]| {
            std::array::from_fn::<_, 2, _>(|i| (a[i] + b[i]) / 2.)
        };
        let before = centre(old[0], old[1]);
        let after = centre(self.fingers[0].at, self.fingers[1].at);
        let distance = |a: [f64; 2], b: [f64; 2]| (a[0] - b[0]).hypot(a[1] - b[1]);
        let first = distance(old[0], old[1]);
        let last = distance(self.fingers[0].at, self.fingers[1].at);
        let mut actions = Motion::Pan(std::array::from_fn(|i| after[i] - before[i]))
            .action(rect)
            .into_iter()
            .collect::<Vec<_>>();
        if first > 1. && last > 1. {
            if let Some(Action::Pick(screen)) = Motion::Pick(after).action(rect) {
                actions.push(Action::ZoomAt {
                    factor: (last / first).clamp(0.1, 10.) as f32,
                    screen,
                });
            }
        }
        actions
    }
    pub fn contains(&self, id: i32) -> bool {
        self.fingers.iter().any(|f| f.id == id)
    }
    pub fn count(&self) -> usize {
        self.fingers.len()
    }
    pub fn cancel(&mut self) {
        self.fingers.clear();
        self.multiple = false;
        self.tap = None;
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod rect_select_tests;

pub mod canvas_drag;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod rect_select_tests;

pub mod canvas_drag;

pub mod touch;
```

## <a id="checkpoint-54j-taps"></a>12. Finish taps double taps and touch cancellation

### 1. `src/touch.rs`

**Locate this existing block:**

```rust
    }
    pub fn contains(&self, id: i32) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn release(
        &mut self,
        id: i32,
        at: [f64; 2],
        time: f64,
        rect: [f64; 4],
    ) -> Vec<Action> {
        let mut actions = self.move_to(id, at, rect);
        let Some(index) = self.fingers.iter().position(|f| f.id == id) else {
            return actions;
        };
        let finger = self.fingers.remove(index);
        if !self.multiple
            && time.is_finite()
            && time >= finger.time
            && time - finger.time <= 300.
            && (at[0] - finger.from[0]).hypot(at[1] - finger.from[1]) <= 12.
            && !finger.moved
        {
            if self.tap.is_some_and(|(last, when)| {
                time >= when
                    && time - when <= 320.
                    && (at[0] - last[0]).hypot(at[1] - last[1]) <= 40.
            }) {
                self.tap = None;
                actions.push(Action::Fit);
            } else {
                self.tap = Some((at, time));
                actions.extend(Motion::Pick(at).action(rect));
            }
        }
        if self.fingers.is_empty() {
            self.multiple = false;
        }
        actions
    }
    pub fn contains(&self, id: i32) -> bool {
```

## <a id="checkpoint-54k-overlay"></a>13. Draw a thin selection rectangle above the canvas

### 1. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            crate::tree_panel::draw(root, &mut self.tree, &mut self.controls);
```

Replace it with:

```rust
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            if let Some(rectangle) = self.rectangle {
                let painter = root.ctx().layer_painter(egui::LayerId::new(
                    egui::Order::Middle,
                    egui::Id::new("selection-rectangle"),
                ));
                let [x0, y0, x1, y1] = rectangle.bounds.map(|v| v as f32);
                let rect =
                    egui::Rect::from_min_max(egui::pos2(x0, y0), egui::pos2(x1, y1));
                painter.rect_filled(
                    rect,
                    0.,
                    egui::Color32::from_rgba_unmultiplied(160, 160, 160, 40),
                );
                let stroke = egui::Stroke::new(1., egui::Color32::from_gray(110));
                let corners = [
                    rect.left_top(),
                    rect.right_top(),
                    rect.right_bottom(),
                    rect.left_bottom(),
                    rect.left_top(),
                ];
                for pair in corners.windows(2) {
                    if rectangle.crossing {
                        let length = pair[0].distance(pair[1]);
                        let direction = (pair[1] - pair[0]) / length;
                        for n in 0..((length / 8.).ceil() as usize).min(16384) {
                            let at = n as f32 * 8.;
                            painter.line_segment(
                                [
                                    pair[0] + direction * at,
                                    pair[0] + direction * (at + 4.).min(length),
                                ],
                                stroke,
                            );
                        }
                    } else {
                        painter.line_segment([pair[0], pair[1]], stroke);
                    }
                }
            }
            crate::tree_panel::draw(root, &mut self.tree, &mut self.controls);
```

**Locate this block:**

```rust
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"tree_rect":self.tree.rect.map(|r|[r.min.x,r.min.y,r.max.x,r.max.y]),"tree_open":self.tree.open,"number":self.number.number,"number_error":self.number.number_error,"number_rect":self.number.number_rect.map(|r|[r.min.x,r.min.y,r.max.x,r.max.y]),"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
```

Replace it with:

```rust
        over
    }

    pub fn free_frame(&self, size: [f64; 2]) -> Option<[f64; 4]> {
        if size
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.)
        {
            return None;
        }
        let right = self
            .tree
            .rect
            .map_or(size[0], |r| r.left() as f64)
            .clamp(size[0] * 0.1, size[0]);
        let bottom = (self.top as f64)
            .min(size[1])
            .max(size[1] * 0.1);
        Some([0., 0., right / size[0], bottom / size[1]])
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"tree_rect":self.tree.rect.map(|r|[r.min.x,r.min.y,r.max.x,r.max.y]),"tree_open":self.tree.open,"number":self.number.number,"number_error":self.number.number_error,"number_rect":self.number.number_rect.map(|r|[r.min.x,r.min.y,r.max.x,r.max.y]),"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
```

**Locate this block:**

```rust
        {
            return None;
        }
        crate::shortcuts::key(
            &event.key(),
            event.ctrl_key() || event.meta_key(),
            event.shift_key(),
            event.alt_key(),
            event.is_composing(),
            self.context.egui_wants_keyboard_input(),
            self.model.command.is_empty(),
        )
    }
```

Replace it with:

```rust
        {
            return None;
        }
        crate::shortcuts::context_key(
            &self.context,
            &self.model.command,
            &event.key(),
            event.ctrl_key() || event.meta_key(),
            event.shift_key(),
            event.alt_key(),
            event.is_composing(),
        )
    }
```

**Locate this block:**

```rust
        });
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

Replace it with:

```rust
        });
    }

    pub fn repeat_line(&self) -> Option<String> {
        if !self.model.command.trim().is_empty() {
            return Some(self.model.command.clone());
        }
        self.model
            .history
            .iter()
            .rev()
            .find_map(|line| {
                line.strip_prefix("> ")
                    .and_then(|line| line.lines().next())
                    .map(str::to_owned)
            })
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }
```

**Locate this block:**

```rust
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            tree: Default::default(),
            number: Default::default(),
            commands: Commands(commands),
```

Replace it with:

```rust
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            rectangle: None,
            tree: Default::default(),
            number: Default::default(),
            commands: Commands(commands),
```

**Locate this block:**

```rust
pub struct Panel {
    pub tree: crate::tree_panel::TreePanel,
    number: crate::number_box::NumberBox,
    commands: Commands,
```

Replace it with:

```rust
pub struct Panel {
    pub rectangle: Option<crate::rect_select::Rectangle>,
    pub tree: crate::tree_panel::TreePanel,
    number: crate::number_box::NumberBox,
    commands: Commands,
```

## <a id="checkpoint-54l-navigation"></a>14. Route mouse touch and wheel after panel input

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([
        rect.left(),
        rect.top(),
        rect.width(),
        rect.height(),
    ])
}
```

Replace it with:

```rust
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    let mut action = motion.action([
        rect.left(),
        rect.top(),
        rect.width(),
        rect.height(),
    ])?;
    match &mut action {
        Action::CanvasStart {
            modified,
            pointer: id,
            ..
        } => {
            *id = pointer.pointer_id();
            *modified = pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key();
        }
        Action::CanvasRelease { add, remove, .. } => {
            *add = pointer.shift_key();
            *remove = pointer.ctrl_key() || pointer.meta_key();
        }
        _ => {}
    }
    Some(action)
}
```

**Locate this block:**

```rust
    ];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
```

Replace it with:

```rust
    ];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary()
                && gesture.press_with(
                    id,
                    pointer.button(),
                    position,
                    pointer.ctrl_key() || pointer.meta_key(),
                )
            {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
```

**Locate this block:**

```rust
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
```

Replace it with:

```rust
            }
            return None;
        }
        _ => {
            if let Some(pointer) = event
                .dyn_ref::<web_sys::PointerEvent>()
                .filter(|p| p.pointer_type() == "touch")
            {
                event.prevent_default();
                let r = canvas.get_bounding_client_rect();
                let rect = [r.left(), r.top(), r.width(), r.height()];
                let at = [
                    pointer.client_x() as f64,
                    pointer.client_y() as f64,
                ];
                let id = pointer.pointer_id();
                let actions = match event.type_().as_str() {
                    "pointerdown" => {
                        gesture.cancel();
                        let multiple = touch.count() > 0;
                        if touch.press(id, at, event.time_stamp()) {
                            if canvas.set_pointer_capture(id).is_err() {
                                touch.cancel();
                            }
                        }
                        if multiple {
                            vec![Action::GizmoCancel, Action::CanvasCancel]
                        } else {
                            Vec::new()
                        }
                    }
                    "pointermove" => touch.move_to(id, at, rect),
                    "pointerup" => {
                        let actions = touch.release(id, at, event.time_stamp(), rect);
                        let _ = canvas.release_pointer_capture(id);
                        actions
                    }
                    "pointercancel" => {
                        touch.cancel();
                        Vec::new()
                    }
                    "lostpointercapture" => {
                        if touch.contains(id) {
                            touch.cancel();
                        }
                        Vec::new()
                    }
                    _ => Vec::new(),
                };
                return (!actions.is_empty()).then_some(Action::Touch(actions));
            }
            return pointer_action(event, canvas, gesture);
        }
    }?;
    event.prevent_default();
    Some(action)
```

**Locate this block:**

```rust
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
```

Replace it with:

```rust
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
    touch: &mut crate::touch::Touch,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            let rect = canvas.get_bounding_client_rect();
            let Action::Zoom(factor) =
                crate::navigation::wheel(wheel.delta_y(), mode, rect.height())?
            else {
                return None;
            };
            let screen = [
                (2. * (wheel.client_x() as f64 - rect.left()) / rect.width() - 1.)
                    as f32,
                (1. - 2. * (wheel.client_y() as f64 - rect.top()) / rect.height())
                    as f32,
            ];
            Some(Action::ZoomAt { factor, screen })
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
```

**Locate this block:**

```rust
                    }
                }
            } else if !panel.consumed {
                navigation_action(&event, &pointer_canvas, &mut gesture)
            } else {
                None
            };
```

Replace it with:

```rust
                    }
                }
            } else if !panel.consumed {
                navigation
            } else {
                None
            };
```

**Locate this block:**

```rust
                }
            };

            let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some())
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
```

Replace it with:

```rust
                }
            };

            let navigation = if line.is_none() && !panel.consumed {
                navigation_action(
                    &event,
                    &pointer_canvas,
                    &mut gesture,
                    &mut touch,
                )
            } else {
                None
            };
            let line = if gesture.take_repeat() {
                event.prevent_default();
                if editor.draft.is_some() || editor.gizmo_edit.is_some() {
                    Some(String::new())
                } else {
                    panel.repeat_line()
                }
            } else {
                line
            };
            let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some())
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
```

**Locate this block:**

```rust
    let mut gesture = Gesture::default();
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
```

Replace it with:

```rust
    let mut gesture = Gesture::default();
    let mut touch = crate::touch::Touch::default();
    let pick_remove = std::cell::Cell::new(false);
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(
        crate::read_gate::ReadGate::default(),
```

## <a id="checkpoint-54m-pick-release"></a>15. Retain the press pick through a quick release

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::PickedMany {
                        target: hit
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit)),
                        additive: pick_additive.get(),
                    }),
                    Some(Err(error)) => {
                        report(error);
                        None
                    }
                    None => None,
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) if editor.draft.is_some() => {
                        Some(Action::ToolScreen {
                            screen,
                            size: [
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .width(),
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .height(),
                            ],
                            commit: true,
                        })
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
                                Action::ToolScreen {
                                    screen,
                                    size: [r.width(), r.height()],
                                    commit: false,
                                }
                            })
                    }
                    Some(Action::Pick(screen)) => {
                        pick_additive.set(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => {
                        let target = hit
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit));
                        if editor.canvas_drag.is_some() {
                            Some(Action::CanvasHit(target.map(|t| t.owner)))
                        } else if pick_remove.get() {
                            Some(Action::SelectRows {
                                ids: target
                                    .filter(|t| {
                                        t.part == crate::pick_sources::Part::Object
                                    })
                                    .map(|t| t.owner)
                                    .into_iter()
                                    .collect(),
                                add: false,
                                remove: true,
                            })
                        } else {
                            Some(Action::PickedMany {
                                target,
                                additive: pick_additive.get(),
                            })
                        }
                    }
                    Some(Err(error)) => {
                        report(error);
                        None
                    }
                    None => None,
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) if editor.draft.is_some() => {
                        Some(Action::ToolScreen {
                            screen,
                            size: [
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .width(),
                                pointer_canvas
                                    .get_bounding_client_rect()
                                    .height(),
                            ],
                            commit: true,
                        })
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
                                Action::ToolScreen {
                                    screen,
                                    size: [r.width(), r.height()],
                                    commit: false,
                                }
                            })
                    }
                    Some(Action::Touch(actions)) => {
                        let mut applied = Vec::new();
                        for action in actions {
                            match action {
                                Action::Pick(screen) => {
                                    if editor.draft.is_some() {
                                        let r =
                                            pointer_canvas.get_bounding_client_rect();
                                        applied.push(Action::ToolScreen {
                                            screen,
                                            size: [r.width(), r.height()],
                                            commit: true,
                                        });
                                    } else {
                                        pick_additive.set(false);
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
                                            screen,
                                            editor.camera.uniform(),
                                        ) {
                                            report(error);
                                        }
                                    }
                                }
                                action => {
                                    renderer.cancel_pick();
                                    applied.push(action);
                                }
                            }
                        }
                        Some(Action::Touch(applied))
                    }
                    Some(Action::CanvasStart {
                        from,
                        to,
                        size,
                        modified,
                        pointer,
                    }) if editor.draft.is_none() => {
                        renderer.cancel_pick();
                        let modified = modified
                            || editor.pick_mode != crate::pick_sources::Mode::Object;
                        let start = Action::CanvasStart {
                            from,
                            to,
                            size,
                            modified,
                            pointer,
                        };
                        let release = event
                            .dyn_ref::<web_sys::PointerEvent>()
                            .filter(|_| event.type_() == "pointerup")
                            .map(|pointer| Action::CanvasRelease {
                                screen: to,
                                size,
                                add: pointer.shift_key(),
                                remove: pointer.ctrl_key() || pointer.meta_key(),
                            });
                        if !modified {
                            if let Err(error) =
                                renderer.request_pick(from, editor.camera.uniform())
                            {
                                report(error);
                                return;
                            }
                        }
                        Some(Action::Touch(
                            std::iter::once(start)
                                .chain(release)
                                .collect(),
                        ))
                    }
                    Some(
                        Action::CanvasStart { .. }
                        | Action::CanvasMove { .. }
                        | Action::CanvasRelease { .. },
                    ) if editor.draft.is_some() => None,
                    Some(
                        action @ (Action::CanvasMove { .. }
                        | Action::CanvasRelease { .. }),
                    ) => Some(action),
                    Some(Action::Pick(screen)) => {
                        pick_remove.set(
                            event
                                .dyn_ref::<web_sys::PointerEvent>()
                                .is_some_and(|p| p.ctrl_key() || p.meta_key()),
                        );
                        pick_additive.set(
```

## <a id="checkpoint-54n-cancellation"></a>16. Cancel owned gestures and preserve normal capture release

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {
        return None;
    }
    if editor.gizmo_hit(screen, size).is_some() {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {
            return Some(Action::GizmoCancel);
        }
        Some(Action::GizmoPress {
            pointer: id,
            screen,
            size,
            modified: false,
        })
```

Replace it with:

```rust
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {
        return None;
    }
    let finger = pointer.pointer_type() == "touch";
    if editor
        .gizmo_hit_radius(
            screen,
            size,
            if finger {
                18.
            } else {
                8.
            },
        )
        .is_some()
    {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {
            return Some(Action::GizmoCancel);
        }
        Some(if finger {
            Action::GizmoTouch {
                pointer: id,
                screen,
                size,
            }
        } else {
            Action::GizmoPress {
                pointer: id,
                screen,
                size,
                modified: false,
            }
        })
```

**Locate this block:**

```rust
                });
                return;
            }
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

Replace it with:

```rust
                });
                return;
            }
            if matches!(
                event.type_().as_str(),
                "blur" | "resize" | "pointercancel"
            ) || event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .is_some_and(|key| key.key() == "Escape" && !key.is_composing())
            {
                gesture.cancel();
                touch.cancel();
                renderer.cancel_pick();
                if editor.apply(Action::CanvasCancel) == Ok(Change::Preview) {
                    crate::browser_upload::prepare(
                        &mut renderer,
                        &editor.scene,
                        editor.whole_selection(),
                        active_fault.clone(),
                    );
                }
            }
            if event.type_() == "lostpointercapture"
                && event
                    .dyn_ref::<web_sys::PointerEvent>()
                    .is_some_and(|p| {
                        editor.canvas_drag.as_ref().is_some_and(|d| {
                            d.pointer == p.pointer_id() && d.released.is_none()
                        })
                    })
            {
                gesture.cancel();
                renderer.cancel_pick();
                if editor.apply(Action::CanvasCancel) == Ok(Change::Preview) {
                    crate::browser_upload::prepare(
                        &mut renderer,
                        &editor.scene,
                        editor.whole_selection(),
                        active_fault.clone(),
                    );
                }
            }
            if event.type_() == "pointerdown"
                && event
                    .dyn_ref::<web_sys::PointerEvent>()
                    .is_some_and(|p| p.is_primary())
                && editor
                    .canvas_drag
                    .as_ref()
                    .is_none_or(|d| d.released.is_some())
            {
                renderer.cancel_pick();
                let _ = editor.apply(Action::CanvasCancel);
            }
            let shortcut = event
                .dyn_ref::<web_sys::KeyboardEvent>()
                .and_then(|key| panel.shortcut(key));
```

**Locate this block:**

```rust
                gesture.cancel();
                let _ = editor.apply(Action::GizmoCancel);
                crate::browser_upload::prepare(
                    &mut renderer,
```

Replace it with:

```rust
                gesture.cancel();
                touch.cancel();
                let _ = editor.apply(Action::CanvasCancel);
                let _ = editor.apply(Action::GizmoCancel);
                crate::browser_upload::prepare(
                    &mut renderer,
```

## <a id="checkpoint-54o-frame"></a>17. Expose drag ownership and centre the available drawing

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .map(|row| row.id),
        );
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
```

Replace it with:

```rust
                .map(|row| row.id),
        );
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute(
        "data-camera-frame",
        &serde_json::to_string(&editor.camera.frame).unwrap(),
    )?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
```

**Locate this block:**

```rust
                    ));
                    return;
                }
                let hover = event
                    .dyn_ref::<web_sys::PointerEvent>()
                    .filter(|p| {
```

Replace it with:

```rust
                    ));
                    return;
                }
                if let Some(frame) = panel.free_frame([rect.width(), rect.height()]) {
                    if editor.camera.frame(frame) {
                        renderer.cancel_pick();
                        let _ = editor.apply(Action::CanvasCancel);
                    }
                }
                let hover = event
                    .dyn_ref::<web_sys::PointerEvent>()
                    .filter(|p| {
```

**Locate this block:**

```rust
            }
            panel.number(number);
            panel
                .tree
```

Replace it with:

```rust
            }
            panel.rectangle = editor
                .canvas_drag
                .as_ref()
                .filter(|d| d.missed)
                .and_then(|d| d.rectangle());
            panel.number(number);
            panel
                .tree
```

## <a id="checkpoint-54p-navigation"></a>18. Exercise current navigation selection and delayed answers

### 1. `src/navigation_complete_tests.rs`

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection},
    editor::{Action, Change, Editor},
    gesture::{Gesture, Motion},
};

#[test]
fn cursor_zoom_holds_the_target_plane_point_in_both_projections_and_frames() {
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.frame([0., 0., 0.7, 0.8]);
        let at = [0.2, -0.1];
        let ray = camera.ray(at).unwrap();
        let forward = camera
            .orientation
            .rotate_vector(session_rust::Vector::y_axis());
        let centre = session_rust::Point::new(
            camera.target[0],
            camera.target[1],
            camera.target[2],
        );
        let t = (&centre - &ray.origin).dot(&forward) / ray.direction.dot(&forward);
        let point = session_rust::Point::new(
            ray.origin[0] + ray.direction[0] * t,
            ray.origin[1] + ray.direction[1] * t,
            ray.origin[2] + ray.direction[2] * t,
        );
        let before = camera
            .view_projection()
            .transform_point(&point);
        camera.zoom_at(1.4, at);
        let after = camera
            .view_projection()
            .transform_point(&point);
        assert!(
            (before[0] - after[0]).abs() < 1e-7 && (before[1] - after[1]).abs() < 1e-7
        );
        assert!((camera.distance - 3. / 1.4_f32 as f64).abs() < 1e-9);
        let saved = camera.view_projection().m;
        camera.zoom_at(2., [f32::NAN, 0.]);
        assert_eq!(camera.view_projection().m, saved);
    }
}

#[test]
fn slop_pan_and_free_canvas_are_css_based_without_document_history() {
    let mut input = Gesture::default();
    input.press_with(1, 2, [10., 20.], true);
    assert_eq!(input.move_to(1, [13., 23.]), None);
    assert_eq!(
        input.move_to(1, [15., 24.]),
        Some(Motion::Pan([5., 4.]))
    );
    let mut camera = Camera::default();
    let old = camera.target;
    camera.pan_screen([20., -10.], [640., 480.]);
    assert_ne!(camera.target, old);
    camera.pan_screen([-20., 10.], [640., 480.]);
    assert!(
        camera
            .target
            .into_iter()
            .zip(old)
            .all(|(a, b)| (a - b).abs() < 1e-12)
    );
    assert!(camera.frame([0., 0., 0.7, 0.8]));
    let target = session_rust::Point::new(
        camera.target[0],
        camera.target[1],
        camera.target[2],
    );
    let projected = camera
        .view_projection()
        .transform_point(&target);
    assert!((projected[0] + 0.3).abs() < 1e-12 && (projected[1] - 0.2).abs() < 1e-12);
    let frame = camera.frame;
    assert!(!camera.frame([0., 0., 0., 1.]));
    assert_eq!(camera.frame, frame);
}

#[test]
fn select_all_and_remove_preserve_sources_and_undo_while_respecting_current_flags() {
    let mut e = Editor::default();
    let ids = e
        .scene
        .objects()
        .iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    std::rc::Rc::make_mut(&mut e.scene.view).lock(&[ids[1]], true);
    assert_eq!(
        e.apply(Action::SelectAll).unwrap(),
        Change::View
    );
    assert_eq!(e.selected_objects(), vec![ids[0]]);
    e.apply(Action::SelectRows {
        ids: vec![ids[0]],
        add: true,
        remove: true,
    })
    .unwrap();
    assert!(e.selected_objects().is_empty());
    assert!(std::rc::Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    assert_eq!(
        crate::shortcuts::key("a", true, false, false, false, false, false),
        Some(crate::shortcuts::Shortcut::SelectAll)
    );
    assert_eq!(
        crate::shortcuts::key("a", true, false, false, false, true, false),
        None
    );
}

fn start(e: &mut Editor, from: [f32; 2], to: [f32; 2]) {
    e.apply(Action::CanvasStart {
        from,
        to,
        size: [640., 480.],
        modified: false,
        pointer: 1,
    })
    .unwrap();
}

#[test]
fn a_released_hit_commits_the_entire_move_once_and_shares_original_source() {
    let mut e = Editor::default();
    let id = e.scene.objects()[0].id;
    let original = e.scene.placement(id).unwrap().m;
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    start(&mut e, [-0.3, 0.], [0.1, 0.]);
    assert_eq!(
        e.apply(Action::CanvasMove {
            screen: [0.2, 0.],
            size: [640., 480.]
        })
        .unwrap(),
        Change::View
    );
    e.apply(Action::CanvasRelease {
        screen: [0.4, 0.],
        size: [640., 480.],
        add: false,
        remove: false,
    })
    .unwrap();
    let released = e.canvas_drag.as_ref().unwrap().to;
    e.apply(Action::CanvasMove {
        screen: [0.6, 0.],
        size: [640., 480.],
    })
    .unwrap();
    assert_eq!(e.canvas_drag.as_ref().unwrap().to, released);
    assert_eq!(
        e.apply(Action::CanvasHit(Some(id))).unwrap(),
        Change::Scene
    );
    let moved = e.scene.placement(id).unwrap().m;
    assert_ne!(moved, original);
    assert!(e.canvas_drag.is_none());
    assert!(std::rc::Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, moved);
}

#[test]
fn a_released_miss_uses_release_modifiers_and_new_press_discards_the_old_drag() {
    let mut e = Editor::default();
    e.apply(Action::SelectAll).unwrap();
    let before = e.selected_objects();
    start(&mut e, [-1., 1.], [1., -1.]);
    e.apply(Action::CanvasRelease {
        screen: [1., -1.],
        size: [640., 480.],
        add: true,
        remove: true,
    })
    .unwrap();
    assert_eq!(
        e.apply(Action::CanvasHit(None)).unwrap(),
        Change::View
    );
    assert!(e.selected_objects().is_empty());
    assert!(e.canvas_drag.is_none());
    assert!(!before.is_empty());
    start(&mut e, [-0.5, 0.], [0.2, 0.]);
    e.apply(Action::CanvasRelease {
        screen: [0.3, 0.],
        size: [640., 480.],
        add: false,
        remove: false,
    })
    .unwrap();
    start(&mut e, [0., 0.], [0.1, 0.]);
    assert_eq!(
        e.canvas_drag.as_ref().unwrap().from,
        [0., 0.]
    );
    e.apply(Action::CanvasCancel).unwrap();
    let placements = e
        .scene
        .objects()
        .iter()
        .map(|r| r.model.m)
        .collect::<Vec<_>>();
    e.apply(Action::CanvasHit(Some(
        e.scene.objects()[0].id,
    )))
    .unwrap();
    assert_eq!(
        e.scene
            .objects()
            .iter()
            .map(|r| r.model.m)
            .collect::<Vec<_>>(),
        placements
    );
}

#[test]
fn touch_pinch_pan_wait_for_all_lifts_and_double_tap_fits() {
    let rect = [0., 0., 640., 480.];
    let mut touch = crate::touch::Touch::default();
    assert!(touch.press(1, [200., 200.], 0.));
    assert!(
        touch
            .move_to(1, [203., 202.], rect)
            .is_empty()
    );
    assert!(matches!(
        touch
            .move_to(1, [210., 205.], rect)
            .as_slice(),
        [Action::Orbit(..)]
    ));
    assert!(touch.press(2, [300., 200.], 20.));
    let actions = touch.move_to(2, [340., 220.], rect);
    assert!(matches!(
        actions.as_slice(),
        [
            Action::PanScreen { .. },
            Action::ZoomAt { .. }
        ]
    ));
    touch.release(2, [340., 220.], 30., rect);
    assert!(
        touch
            .move_to(1, [230., 205.], rect)
            .is_empty()
    );
    assert!(
        touch
            .release(1, [230., 205.], 40., rect)
            .is_empty()
    );
    assert_eq!(touch.count(), 0);
    touch.press(3, [200., 200.], 100.);
    assert!(matches!(
        touch
            .release(3, [200., 200.], 120., rect)
            .as_slice(),
        [Action::Pick(_)]
    ));
    touch.press(4, [210., 200.], 150.);
    assert!(matches!(
        touch
            .release(4, [210., 200.], 180., rect)
            .as_slice(),
        [Action::Fit]
    ));
    touch.press(5, [200., 200.], 200.);
    touch.cancel();
    assert!(
        touch
            .release(5, [200., 200.], 220., rect)
            .is_empty()
    );
    assert!(!touch.press(6, [f64::NAN, 0.], 230.));
}

#[test]
fn a_failed_cold_source_drag_releases_its_preview_and_keeps_redo() {
    let mut e = Editor::default();
    let location = std::rc::Rc::new(crate::reload_url::ReloadUrl::new(
        "test:drag".into(),
    ));
    e.apply(Action::ReplaceAt(
        crate::specimen::bytes(),
        location,
    ))
    .unwrap();
    let id = e.scene.objects()[0].id;
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::Undo).unwrap();
    e.apply(Action::UnloadSources).unwrap();
    let before = e.scene.placement(id).unwrap().m;
    start(&mut e, [0., 0.], [0.2, 0.]);
    assert!(
        e.apply(Action::CanvasHit(Some(id)))
            .unwrap_err()
            .contains("Reload")
    );
    assert!(e.canvas_drag.is_none());
    assert_eq!(e.scene.placement(id).unwrap().m, before);
    let count = e.scene.objects().len();
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.objects().len(), count + 1);
}

#[test]
fn panel_free_fit_contains_every_corner_in_perspective_and_orthographic() {
    let mut bounds = crate::bounds::Bounds::point([-3., -2., -1.]);
    bounds.include([3., 2., 1.]);
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.isometric();
        camera.frame([0., 0., 0.6, 0.75]);
        camera.fit(&bounds);
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let p = camera
                        .view_projection()
                        .transform_point(&session_rust::Point::new(x, y, z));
                    assert!(p[0] >= -1. && p[0] <= 0.2 && p[1] >= -0.5 && p[1] <= 1.);
                }
            }
        }
    }
}

#[test]
fn only_an_unmoved_right_click_repeats_and_cancel_clears_the_request() {
    let mut gesture = Gesture::default();
    gesture.press(1, 2, [0., 0.]);
    assert_eq!(gesture.release(1, [2., 2.]), None);
    assert!(gesture.take_repeat());
    assert!(!gesture.take_repeat());
    gesture.press(1, 2, [0., 0.]);
    gesture.move_to(1, [10., 0.]);
    gesture.release(1, [0., 0.]);
    assert!(!gesture.take_repeat());
    gesture.press(1, 2, [0., 0.]);
    gesture.cancel();
    assert!(!gesture.take_repeat());
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod canvas_drag;

pub mod touch;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod canvas_drag;

pub mod touch;

#[cfg(test)]
mod navigation_complete_tests;
```

### 3. `src/shortcut_focus_tests.rs`

Create the file and type:

```rust
use crate::shortcuts::{Shortcut, context_key};

#[test]
fn actual_command_name_and_popup_focus_own_shortcuts_like_current_panels() {
    let context = egui::Context::default();
    context.set_fonts(crate::command_dock::theme::fonts([
        include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
    ]));
    let mut command = String::new();
    let mut name = "Posts".to_owned();
    let mut frame = |id: &str| {
        context.memory_mut(|memory| memory.request_focus(egui::Id::new(id)));
        let _ = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640., 480.),
                )),
                ..Default::default()
            },
            |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut command)
                        .id(egui::Id::new("command-input")),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut name)
                        .id(egui::Id::new("layer-name-input")),
                );
            },
        );
    };
    frame("command-input");
    assert_eq!(
        context.memory(|memory| memory.focused()),
        Some(egui::Id::new("command-input"))
    );
    assert_eq!(
        context_key(&context, "", "a", true, false, false, false),
        Some(Shortcut::SelectAll)
    );
    assert_eq!(
        context_key(
            &context, "Move", "a", true, false, false, false
        ),
        None
    );
    assert_eq!(
        context_key(&context, "", "z", true, false, false, false),
        Some(Shortcut::Undo)
    );
    frame("layer-name-input");
    assert_eq!(
        context.memory(|memory| memory.focused()),
        Some(egui::Id::new("layer-name-input"))
    );
    assert_eq!(
        context_key(&context, "", "a", true, false, false, false),
        None
    );
    assert_eq!(
        context_key(&context, "", "z", true, false, false, false),
        None
    );
    egui::Popup::open_id(&context, egui::Id::new("layer-menu"));
    assert_eq!(
        context_key(&context, "", "a", true, false, false, false),
        None
    );
    assert_eq!(
        context_key(&context, "", "h", false, false, false, false),
        None
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod navigation_complete_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod navigation_complete_tests;

#[cfg(test)]
mod shortcut_focus_tests;
```

### 5. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    ));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Action, Change, Editor};
    let mut e = Editor::default();
    e.scene.clear();
    let id = e.scene.insert(triangle(0.2)).unwrap();
    let original = e.scene.placement(id).unwrap().m;
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let mut renderer = renderer(&e.scene);
    let transform = e.camera.uniform();
    let gpu = renderer.stats();
    let size = [320., 240.];
    e.apply(Action::CanvasStart {
        from: [0., 0.],
        to: [0.2, 0.],
        size,
        modified: false,
        pointer: 1,
    })
    .unwrap();
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    e.apply(Action::CanvasRelease {
        screen: [0.4, 0.],
        size,
        add: false,
        remove: false,
    })
    .unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let hit = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .and_then(|pixel| renderer.resolve_pick(&e.scene, pixel));
    assert_eq!(hit.unwrap().owner, id);
    assert_eq!(
        e.apply(Action::CanvasHit(
            hit.map(|target| target.owner)
        ))
        .unwrap(),
        Change::Scene
    );
    assert_ne!(e.scene.placement(id).unwrap().m, original);
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    renderer.set_scene(&e.scene, e.selected);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
}
```

## Build & Run

![Recorded visual output for Lesson 8.4 (54p-navigation)](../../screenshots/journey/54p-navigation.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 8.4
npm --prefix ../session_tests run course -- save 8.4
```

Continue with [9.1 · Original surfaces, analytic normals and trim boundaries](55-validation.md).
