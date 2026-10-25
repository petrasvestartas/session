# 21.1 · Units and one precise render origin

**Steps:** 11 · **Source:** [camera.rs](../../../../src/camera.rs) · [state.rs](../../../../src/state.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Large World Coordinates (f64): P_world = 10^7 m</text>
  <circle cx="64" cy="138" r="4" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="64" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,0,0)</text>
  <line x1="64" y1="138" x2="236" y2="74" stroke="#94a3b8" stroke-width="1.75" stroke-dasharray="5 3" marker-end="url(#arr)"/>
  <circle cx="244" cy="70" r="5" fill="#dc2626"/>
  <polygon points="244,70 278,54 272,92" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="218" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor_f64</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU f64 Delta: P_rel_f32 = (P_world_f64 - Anchor_f64) as f32</text>
  <circle cx="486" cy="112" r="5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="486" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor = (0,0,0)_rel</text>
  <line x1="486" y1="112" x2="574" y2="82" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <polygon points="574,82 634,58 622,118" fill="#bbf7d0" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="606" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">||P_rel|| &lt; 10 m (Zero Jitter)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Jitter-Free Far-Origin Orbit + Crisp Depth Precision</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,142 934,58 1002,142" fill="#dbeafe" stroke="#111111" stroke-width="2.2"/>
  <line x1="934" y1="58" x2="934" y2="142" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-96zzzzy-prepare-unit-aware-anchored-camera-space"></a>1. Prepare unit-aware anchored camera space

### 1. `src/camera.rs`

Move the existing measurement Unit and suffix method to the camera; this body was already typed.

**Locate this existing block:**

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

pub const NEAR_FRACTION: f64 = 1.0e-4;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}

pub const NEAR_FRACTION: f64 = 1.0e-4;
```

### 2. `src/measure_mark.rs`

Keep the existing measurement name as an alias to the shared unit.

**Locate this existing block:**

```rust
use crate::{
    scene::{ObjectId, Scene},
    text::{TextLabel, TextPlacement},
};
use session_rust::{Geometry, Point};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}
#[derive(Clone, Debug, PartialEq)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub use crate::camera::Unit;
use crate::{
    scene::{ObjectId, Scene},
    text::{TextLabel, TextPlacement},
};
use session_rust::{Geometry, Point};
#[derive(Clone, Debug, PartialEq)]
```

### 3. `src/camera.rs`

Add conversion to metres to the moved unit while retaining its existing suffix method. Retain the file unit with camera state; source geometry stays in scene units. Start with the maintained viewer’s millimetre file unit. Build one unit-aware matrix, subtracting the anchor before narrowing any coordinate. Look at the target relative to the shared scene anchor in metres. Keep eye and target in the same physical coordinate space. Scale the near plane with the physical eye distance. Convert the shared scene/grid reach once. Express the orthographic image size in metres. Retain the existing scene-unit minimum reach during this coordinate preparation. Convert incoming anchor-relative scene vertices to metres in the same matrix.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .ok()
            })
            .map_or(self.radius, |reach| self.radius.max(reach));
        let far = (self.distance * 10.0).max(self.distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, near, far)
            }
            Projection::Orthographic => {
                let h = self.distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (self.distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        let shift = self.shift();
        Xform::translation(shift[0], shift[1], 0.) * projection * view
    }
```

Replace it with:

```rust
                .ok()
            })
            .map_or(self.radius, |reach| self.radius.max(reach));
        let reach = reach * scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, near, far)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6 * scale);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
        };
        let shift = self.shift();
        let mut matrix = Xform::translation(shift[0], shift[1], 0.)
            * projection
            * view
            * Xform::scale_xyz(scale, scale, scale);
        if self.projection == Projection::Perspective {
            for value in &mut matrix.m {
                *value /= scale;
            }
        }
        matrix
    }
```

**Locate this block:**

```rust
    }

    pub fn view_projection(&self) -> Xform {
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let target = Point::new(
            self.target[0],
            self.target[1],
            self.target[2],
        );
        let eye = Point::new(
            self.target[0] - forward[0] * self.distance,
            self.target[1] - forward[1] * self.distance,
            self.target[2] - forward[2] * self.distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = self.distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| {
```

Replace it with:

```rust
    }

    pub fn view_projection(&self) -> Xform {
        self.view_projection_anchored([0.; 3])
    }

    pub fn view_projection_anchored(&self, anchor: [f64; 3]) -> Xform {
        let scale = self.unit.to_meters();
        let local: [f64; 3] =
            std::array::from_fn(|i| (self.target[i] - anchor[i]) * scale);
        let distance = self.distance * scale;
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let target = Point::new(local[0], local[1], local[2]);
        let eye = Point::new(
            local[0] - forward[0] * distance,
            local[1] - forward[1] * distance,
            local[2] - forward[2] * distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| {
```

**Locate this block:**

```rust
impl Default for Camera {
    fn default() -> Self {
        Self {
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
```

Replace it with:

```rust
impl Default for Camera {
    fn default() -> Self {
        Self {
            unit: Unit::Millimeters,
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
```

**Locate this block:**

```rust
}

pub struct Camera {
    pub target: [f64; 3],
    pub distance: f64,
    pub radius: f64,
```

Replace it with:

```rust
}

pub struct Camera {
    pub unit: Unit,
    pub target: [f64; 3],
    pub distance: f64,
    pub radius: f64,
```

**Locate this block:**

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
```

Replace it with:

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn to_meters(self) -> f64 {
        match self {
            Self::Millimeters => 0.001,
            Self::Meters => 1.,
        }
    }
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
```

### 4. `src/lib.rs`

Keep the optional coordinate fixtures outside required coding.

**Locate this existing block:**

```rust
pub mod camera;
pub mod chain;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
#[cfg(test)]
mod camera_space_tests;
pub mod chain;
```

### 5. `src/camera_space_tests.rs`

Supply equivalent physical mm/metre projection/rays and distant submillimetre detail after f32 camera upload.

Create the file and type:

```rust
use crate::camera::{Camera, Projection, Unit};
use session_rust::Point;
#[test]
fn equivalent_millimetre_and_metre_scenes_share_screen_and_ray_geometry() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut camera = Camera::default();
            camera.unit = unit;
            camera.projection = projection;
            assert!(camera.view_projection().inverse().is_some());
            assert!(camera.ray([0.2, -0.1]).is_some());
        }
    }
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut mm = Camera::default();
        mm.projection = projection;
        mm.target = [2e6, -3e6, 4e6];
        mm.distance = 5000.;
        mm.radius = 1000.;
        let mut m = Camera::default();
        m.projection = projection;
        m.unit = Unit::Meters;
        m.target = mm.target.map(|x| x * 0.001);
        m.distance = 5.;
        m.radius = 1.;
        let a = mm.view_projection_anchored(mm.target);
        let b = m.view_projection_anchored(m.target);
        let pa = a.transform_point(&Point::new(500., 250., 100.));
        let pb = b.transform_point(&Point::new(0.5, 0.25, 0.1));
        for i in 0..3 {
            assert!((pa[i] - pb[i]).abs() < 1e-12);
        }
        let ra = mm.ray([0.2, -0.1]).unwrap();
        let rb = m.ray([0.2, -0.1]).unwrap();
        for i in 0..3 {
            assert!((ra.origin[i] * 0.001 - rb.origin[i]).abs() < 1e-6);
            assert!((ra.direction[i] - rb.direction[i]).abs() < 1e-6);
        }
        assert!((ra.max_distance * 0.001 - rb.max_distance).abs() < 1e-5);
    }
}
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin() {
    let mut camera = Camera::default();
    camera.target = [1e12, -1e12, 1e12];
    camera.distance = 1.;
    camera.radius = 1.;
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        camera.projection = projection;
        let local = camera
            .view_projection_anchored(camera.target)
            .to_f32();
        let matrix = session_rust::Xform::from_matrix(local.map(f64::from));
        let a = matrix.transform_point(&Point::new(0., 0., 0.));
        let b = matrix.transform_point(&Point::new(0.000125, 0., 0.));
        assert!(b[0] - a[0] > 0.0001);
        assert!((0.0..1.0).contains(&a[2]));
    }
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        camera.projection = projection;
        let mut local = Camera::default();
        local.projection = projection;
        local.distance = 1.;
        local.radius = 1.;
        for screen in [[0., 0.], [0.2, -0.1]] {
            let a = camera.ray(screen).unwrap();
            let b = local.ray(screen).unwrap();
            for i in 0..3 {
                assert!((a.origin[i] - camera.target[i] - b.origin[i]).abs() < 0.00025);
                assert!((a.direction[i] - b.direction[i]).abs() < 1e-10);
            }
            assert!((a.max_distance - b.max_distance).abs() < 1e-8);
        }
    }
    assert_eq!(camera.target, [1e12, -1e12, 1e12]);
}
```

### 6. `src/camera.rs`

Invert around the precise camera target so large absolute translations cannot spoil ray construction. Compute ray direction and extent locally, then restore the precise scene-space origin.

**Locate this existing block:**

```rust
        }
        let inverse = self.view_projection().inverse()?;
        let origin = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            0.0,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            1.0,
        ));
        let delta = &end - &origin;
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        Some(Ray {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let inverse = self
            .view_projection_anchored(self.target)
            .inverse()?;
        let origin = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            0.0,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            1.0,
        ));
        let delta = &end - &origin;
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        let origin = Point::new(
            origin[0] + self.target[0],
            origin[1] + self.target[1],
            origin[2] + self.target[2],
        );
        Some(Ray {
```

### 7. `src/editor.rs`

Remove the independent editor unit value. Use the camera’s existing default scene unit. Measure selected originals using the same scene unit as projection. Give distance annotations that same unit interpretation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    fn distance_mark(&mut self, points: [[f64; 3]; 2]) -> Result<Change, &'static str> {
        let (mark, message) =
            crate::measure_mark::distance(points[0], points[1], self.unit);
        self.mark = Some(mark);
```

Replace it with:

```rust
    }
    fn distance_mark(&mut self, points: [[f64; 3]; 2]) -> Result<Change, &'static str> {
        let (mark, message) =
            crate::measure_mark::distance(points[0], points[1], self.camera.unit);
        self.mark = Some(mark);
```

**Locate this block:**

```rust
            &self.measured_ids(),
            face,
            kind,
            self.unit,
        )
        .map_err(|message| self.primitive_failure(message))?;
        self.mark = Some(mark);
```

Replace it with:

```rust
            &self.measured_ids(),
            face,
            kind,
            self.camera.unit,
        )
        .map_err(|message| self.primitive_failure(message))?;
        self.mark = Some(mark);
```

**Locate this block:**

```rust
            trim: None,
            last_text_height: 0.,
            mark: None,
            unit: crate::measure_mark::Unit::Millimeters,
            split: None,
            canvas_drag: None,
            gizmo_edit: None,
```

Replace it with:

```rust
            trim: None,
            last_text_height: 0.,
            mark: None,
            split: None,
            canvas_drag: None,
            gizmo_edit: None,
```

**Locate this block:**

```rust
    pub mark: Option<crate::measure_mark::Mark>,
    pub unit: crate::measure_mark::Unit,
    pub split: Option<crate::split_draft::Draft>,
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
```

Replace it with:

```rust
    pub mark: Option<crate::measure_mark::Mark>,
    pub split: Option<crate::split_draft::Draft>,
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
```

### 8. `src/measure_tests.rs`

Retain all existing area, volume, original-face and open-shape assertions under the shared unit owner.

**Locate this existing block:**

```rust
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 mm³");
    e.unit = Unit::Meters;
    command(&mut e, "Volume").unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 mm³");
    e.camera.unit = Unit::Meters;
    command(&mut e, "Volume").unwrap();
```

## <a id="checkpoint-96zzzzz-prepare-precise-anchor-relative-object-records"></a>2. Prepare precise anchor-relative object records

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

### 1. `src/render_space.rs`

Share double-precision anchor subtraction before GPU narrowing.

Create the file and type:

```rust
pub fn relative(translation: [f64; 3], anchor: [f64; 3]) -> [f32; 3] {
    std::array::from_fn(|i| (translation[i] - anchor[i]) as f32)
}
#[derive(Default)]
pub struct Origin {
    anchor: Option<[f64; 3]>,
    last_ms: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rebase {
    pub anchor: [f64; 3],
    pub moved: bool,
    pub pending: bool,
}
impl Origin {
    pub fn forget(&mut self) {
        self.anchor = None;
    }
    pub fn update(
        &mut self,
        target: [f64; 3],
        distance: f64,
        now: f64,
    ) -> Option<Rebase> {
        if target.iter().any(|v| !v.is_finite())
            || !distance.is_finite()
            || distance <= 0.
            || !now.is_finite()
        {
            return None;
        }
        let threshold = (distance * 0.25).clamp(1000., 100000.);
        let need = self.anchor.is_none_or(|a| {
            (0..3)
                .map(|i| (a[i] - target[i]).powi(2))
                .sum::<f64>()
                .sqrt()
                > threshold
        });
        let moved = need && (self.anchor.is_none() || now - self.last_ms > 200.);
        if moved {
            self.anchor = Some(target);
            self.last_ms = now;
        }
        Some(Rebase {
            anchor: self.anchor.unwrap(),
            moved,
            pending: need && !moved,
        })
    }
}
```

### 2. `src/lib.rs`

Keep anchor-relative GPU conversion under one coordinate owner.

**Locate this existing block:**

```rust
mod point_input_tests;
pub mod report_recency;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod point_input_tests;
mod render_space;
pub mod report_recency;
```

### 3. `src/gpu_mesh.rs`

Retain precise placement translation and the anchor used by its GPU mirror. Make the placement anchor explicit at the one record-packing boundary. Narrow translation only after subtracting the precise anchor; retain the original linear transform. New isolated GPU geometry begins in the ordinary zero-anchor frame. Retain source placement independently of its narrowed uniform and shared table. Rebase only the three translation values; an unchanged relative translation writes nothing. Refresh the precise retained translation for ordinary object edits. Keep selection, styles and placement edits in the current anchor frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.cap_token,
            self.primitive_base,
            self.geometry.index_start,
        );
```

Replace it with:

```rust
            self.cap_token,
            self.primitive_base,
            self.geometry.index_start,
            self.anchor,
        );
```

**Locate this block:**

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let mut next = settings(
            object,
            style,
```

Replace it with:

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        self.translation = object.model.m[12..15].try_into().unwrap();
        let mut next = settings(
            object,
            style,
```

**Locate this block:**

```rust
        }
    }

    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
```

Replace it with:

```rust
        }
    }

    pub fn rebase(&mut self, queue: &wgpu::Queue, anchor: [f64; 3]) -> [usize; 2] {
        assert!(anchor.iter().all(|v| v.is_finite()));
        self.anchor = anchor;
        let bytes = crate::render_space::relative(self.translation, anchor)
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        if self.previous[48..60] == bytes {
            return [0, 0];
        }
        self.previous[48..60].copy_from_slice(&bytes);
        queue.write_buffer(&self.model, 48, &bytes);
        [1, 12]
    }
    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
```

**Locate this block:**

```rust
            }],
        });
        Self {
            geometry,
            model_group,
            id: object.id,
```

Replace it with:

```rust
            }],
        });
        Self {
            translation: object.model.m[12..15].try_into().unwrap(),
            anchor: [0.; 3],
            geometry,
            model_group,
            id: object.id,
```

**Locate this block:**

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style, 0, 0, 0, geometry.index_start);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

Replace it with:

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(
            object,
            style,
            0,
            0,
            0,
            geometry.index_start,
            [0.; 3],
        );
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

**Locate this block:**

```rust
    cap_token: u32,
    primitive_base: u32,
    index_start: u32,
) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
```

Replace it with:

```rust
    cap_token: u32,
    primitive_base: u32,
    index_start: u32,
    anchor: [f64; 3],
) -> Record {
    let mut bytes = [0; BYTES];
    let mut model = object.model.to_f32();
    model[12..15].copy_from_slice(&crate::render_space::relative(
        object.model.m[12..15].try_into().unwrap(),
        anchor,
    ));
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
```

**Locate this block:**

```rust
    model: wgpu::Buffer,
    previous: Record,
    primitive_base: u32,
    xray: bool,
```

Replace it with:

```rust
    model: wgpu::Buffer,
    translation: [f64; 3],
    anchor: [f64; 3],
    previous: Record,
    primitive_base: u32,
    xray: bool,
```

### 4. `src/global_outline_tests.rs`

Supply actual distant/local pixel equality, exact mapped object records, unchanged geometry uploads, original picks and selection, repeated-anchor zero writes and precise placement edits. Derived ink is omitted explicitly in this isolated surface fixture.

**Locate this existing block:**

```rust
    assert!(history.scenes().count() > 0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(history.scenes().count() > 0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners() {
    let mut local = Scene::empty();
    let owner = local
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.3, -0.4],
                    [0.2, -0.3, -0.4],
                    [0., 0.3, -0.4],
                ],
                [0.1, 0.3, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    local
        .translate(owner, [0.125, -0.25, 0.])
        .unwrap();
    let mut gpu = renderer(&local);
    gpu.set_opacity(1.);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        baseline
            .chunks_exact(4)
            .filter(|p| p[2] > p[0].saturating_add(50))
            .count()
            > 100
    );
    let source = local.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let uploads = gpu.stats()[0];
    let anchor = [1e12, -1e12, 1e12];
    let mut far = local.clone();
    far.translate(owner, anchor).unwrap();
    gpu.set_scene(&far, None);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    assert_eq!(
        gpu.meshes[0].rebase(&gpu.queue, anchor),
        [1, 12]
    );
    assert_eq!(
        gpu.meshes[0].rebase(&gpu.queue, anchor),
        [0, 0]
    );
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(gpu.stats()[0], uploads);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let bytes = crate::growing_buffer_tests::read(
        &ctx,
        gpu.instances.record_buffer(),
        crate::object_record::BYTES as u64,
    );
    assert_eq!(
        &bytes[48..60],
        &[0.125f32, -0.25, 0.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        &bytes[..crate::object_record::BYTES],
        &gpu.meshes[0].packed()
    );
    gpu.select_objects(&far, &[owner]).unwrap();
    assert_ne!(pixels(&gpu), baseline);
    gpu.request_pick([0.125, -0.25], matrix())
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(matrix())
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    gpu.select_objects(&far, &[]).unwrap();
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(
        gpu.meshes[0].rebase(&gpu.queue, anchor),
        [0, 0]
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    far.translate(owner, [0.001, 0., 0.])
        .unwrap();
    gpu.set_scene(&far, None);
    let expected = crate::render_space::relative(
        far.objects()[0].model.m[12..15]
            .try_into()
            .unwrap(),
        anchor,
    );
    assert_eq!(
        &gpu.meshes[0].packed()[48..60],
        &expected
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>()
    );
    assert!(expected[0] > 0.125);
    assert_eq!(gpu.stats()[0], uploads);
}
```

### 5. `src/render_space.rs`

Supply exact threshold, strict throttle boundary, pending/settled state, upper clamp, invalid-input atomicity and reset coverage for the prepared origin policy.

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
    #[test]
    fn origin_threshold_throttle_reset_and_invalid_inputs_keep_one_anchor() {
        let mut origin = Origin::default();
        assert_eq!(
            origin.update([0.; 3], 4000., 0.),
            Some(Rebase {
                anchor: [0.; 3],
                moved: true,
                pending: false
            })
        );
        assert_eq!(
            origin.update([1000., 0., 0.], 4000., 1.),
            Some(Rebase {
                anchor: [0.; 3],
                moved: false,
                pending: false
            })
        );
        assert_eq!(
            origin.update([1000.25, 0., 0.], 4000., 200.),
            Some(Rebase {
                anchor: [0.; 3],
                moved: false,
                pending: true
            })
        );
        assert_eq!(
            origin.update([1000.25, 0., 0.], 4000., 200.01),
            Some(Rebase {
                anchor: [1000.25, 0., 0.],
                moved: true,
                pending: false
            })
        );
        assert!(
            !origin
                .update([99000., 0., 0.], 1e15, 300.)
                .unwrap()
                .moved
        );
        assert!(
            origin
                .update([200000., 0., 0.], 1e15, 500.)
                .unwrap()
                .moved
        );
        for (target, distance, now) in [
            ([f64::NAN, 0., 0.], 1., 501.),
            ([0.; 3], 0., 501.),
            ([0.; 3], f64::INFINITY, 501.),
            ([0.; 3], 1., f64::NAN),
        ] {
            assert!(
                origin
                    .update(target, distance, now)
                    .is_none()
            );
        }
        assert_eq!(
            origin
                .update([200000., 0., 0.], 1., 501.)
                .unwrap()
                .anchor,
            [200000., 0., 0.]
        );
        origin.forget();
        assert!(
            origin
                .update([1e12, -1e12, 1e12], 1., 501.)
                .unwrap()
                .moved
        );
    }
}
```

## <a id="checkpoint-96zzzzza-prepare-anchor-relative-original-ink"></a>3. Prepare anchor-relative ink

### 1. `src/scene.rs`

Retain the existing line API while making its packing origin explicit. Subtract the anchor from placed precise endpoints before conversion to GPU floats. Reuse one path-neighbour algorithm for zero and explicit origins. Filter local GPU duplicates after the retained precise sample and original placement. Read the original point and placement before subtracting the anchor; retain its current pixel style. Share the existing original control collector under an explicit packing origin. Retain precise collected control positions through the GPU boundary. Keep original parent-only net ownership while packing its links in the chosen frame. Use the same precise conversion for both ends of a control net.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                else {
                    continue;
                };
                let start = a.position.map(|v| v as f32);
                let end = b.position.map(|v| v as f32);
                if start
```

Replace it with:

```rust
                else {
                    continue;
                };
                let start = crate::render_space::relative(a.position, anchor);
                let end = crate::render_space::relative(b.position, anchor);
                if start
```

**Locate this block:**

```rust
    }

    pub fn control_nets(&self) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
```

Replace it with:

```rust
    }

    pub fn control_nets(&self) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        self.control_nets_at([0.; 3])
    }
    pub fn control_nets_at(
        &self,
        anchor: [f64; 3],
    ) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
```

**Locate this block:**

```rust
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = control.position.map(|v| v as f32);
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
```

Replace it with:

```rust
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        self.control_markers_at([0.; 3])
    }
    pub fn control_markers_at(&self, anchor: [f64; 3]) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = crate::render_space::relative(control.position, anchor);
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
```

**Locate this block:**

```rust
            .transform_point(&self.prepared.source)
    }
    pub fn marker(&self) -> crate::marker::Marker {
        crate::marker::Marker {
            center: self.point().to_f32(),
            ..self.prepared.display
        }
    }
```

Replace it with:

```rust
            .transform_point(&self.prepared.source)
    }
    pub fn marker(&self) -> crate::marker::Marker {
        self.marker_at([0.; 3])
    }
    pub fn marker_at(&self, anchor: [f64; 3]) -> crate::marker::Marker {
        crate::marker::Marker {
            center: crate::render_space::relative(
                crate::clipping_geometry::array(&self.point()),
                anchor,
            ),
            ..self.prepared.display
        }
    }
```

**Locate this block:**

```rust
    }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) {
                points.push(p);
            }
```

Replace it with:

```rust
    }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        self.segments_at([0.; 3])
    }
    pub fn segments_at(&self, anchor: [f64; 3]) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = crate::render_space::relative(
                crate::clipping_geometry::array(&p),
                anchor,
            );
            if points.last() != Some(&p) {
                points.push(p);
            }
```

**Locate this block:**

```rust
    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke {
            start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32),
            ..self.prepared.display
        }
    }
```

Replace it with:

```rust
    pub fn stroke(&self) -> crate::stroke::Stroke {
        self.stroke_at([0.; 3])
    }
    pub fn stroke_at(&self, anchor: [f64; 3]) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke {
            start: crate::render_space::relative(
                crate::clipping_geometry::array(&start),
                anchor,
            ),
            end: crate::render_space::relative(
                crate::clipping_geometry::array(&end),
                anchor,
            ),
            ..self.prepared.display
        }
    }
```

### 2. `src/lib.rs`

Keep optional precise source-packing fixtures outside required learner coding.

**Locate this existing block:**

```rust
mod report_store_tests;
pub mod startup;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod report_store_tests;
#[cfg(test)]
mod scene_origin_tests;
pub mod startup;
```

### 3. `src/scene_origin_tests.rs`

Supply actual original line/path/point placement, fractional bytes, chain neighbours, parent/control IDs, net styles and retained original/cache pointers at a large coordinate.

Create the file and type:

```rust
use std::rc::Rc;
#[test]
fn placed_original_ink_keeps_fractional_positions_controls_and_sources() {
    let mut scene = crate::scene::Scene::empty();
    let line = Rc::new(session_rust::Line::new(
        -0.25, 0.125, 0., 0.25, 0.125, 0.,
    ));
    let line_id = scene
        .insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap())
        .unwrap();
    let polyline = Rc::new(session_rust::Polyline::new(vec![
        session_rust::Point::new(0., -0.25, 0.),
        session_rust::Point::new(0.125, 0.25, 0.),
        session_rust::Point::new(0.375, -0.25, 0.),
    ]));
    let path_id = scene
        .insert_path(crate::chain::PreparedChain::polyline(polyline.clone()).unwrap())
        .unwrap();
    let point = Rc::new(session_rust::Point::new(0.25, 0.375, 0.));
    let point_id = scene
        .insert_point(crate::marker::PreparedPoint::new(point.clone()).unwrap())
        .unwrap();
    scene.owner_controls(path_id).unwrap();
    let line_bytes = scene.lines()[0].stroke().bytes();
    let path_bytes = scene.paths()[0]
        .segments()
        .iter()
        .flat_map(|s| s.bytes())
        .collect::<Vec<_>>();
    let marker = scene.points()[0].marker().values();
    let controls = scene
        .control_markers()
        .iter()
        .map(|c| (c.parent, c.part, c.display.values()))
        .collect::<Vec<_>>();
    let nets = scene.control_nets();
    assert!(!controls.is_empty());
    assert!(!nets.is_empty());
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in [line_id, path_id, point_id] {
        far.translate(id, anchor).unwrap();
    }
    let collapsed = far.lines()[0].stroke();
    assert_eq!(collapsed.start, collapsed.end);
    assert_eq!(
        far.lines()[0].stroke_at(anchor).bytes(),
        line_bytes
    );
    assert_eq!(
        far.paths()[0]
            .segments_at(anchor)
            .iter()
            .flat_map(|s| s.bytes())
            .collect::<Vec<_>>(),
        path_bytes
    );
    assert_eq!(
        far.points()[0].marker_at(anchor).values(),
        marker
    );
    assert_eq!(
        far.control_markers_at(anchor)
            .iter()
            .map(|c| (c.parent, c.part, c.display.values()))
            .collect::<Vec<_>>(),
        controls
    );
    assert_eq!(far.control_nets_at(anchor), nets);
    assert!(Rc::ptr_eq(
        &far.lines()[0].prepared.source,
        &line
    ));
    assert!(
        matches!(&far.paths()[0].prepared.source,crate::chain::Source::Polyline(source) if Rc::ptr_eq(source,&polyline))
    );
    assert!(Rc::ptr_eq(
        &far.points()[0].prepared.source,
        &point
    ));
    assert!(Rc::ptr_eq(
        &far.paths()[0].prepared.points,
        &scene.paths()[0].prepared.points
    ));
    assert_eq!(line.start()[0], -0.25);
    assert_eq!(polyline.coords[3], 0.125);
    assert_eq!(point[0], 0.25);
}

#[test]
fn anchored_source_tables_keep_original_edge_and_control_addresses() {
    let mut scene = crate::scene::Scene::empty();
    let mut ids = Vec::new();
    ids.push(
        scene
            .insert(
                crate::prepared::PreparedMesh::new(session_rust::Mesh::create_box(
                    0.5, 0.5, 0.5,
                ))
                .unwrap(),
            )
            .unwrap(),
    );
    ids.push(
        scene
            .insert_line(
                crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                    0., 0., 0., 0.25, 0., 0.,
                )))
                .unwrap(),
            )
            .unwrap(),
    );
    ids.push(
        scene
            .insert_path(
                crate::chain::PreparedChain::polyline(Rc::new(
                    session_rust::Polyline::new(vec![
                        session_rust::Point::new(0., 0., 0.),
                        session_rust::Point::new(0.125, 0.25, 0.),
                        session_rust::Point::new(0.25, 0., 0.),
                    ]),
                ))
                .unwrap(),
            )
            .unwrap(),
    );
    ids.push(
        scene
            .insert_point(
                crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                    0.125, 0.25, 0.,
                )))
                .unwrap(),
            )
            .unwrap(),
    );
    scene.owner_controls(ids[0]).unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let baseline = crate::pick_sources::Sources::new(&scene).unwrap();
    assert!(!baseline.faces.is_empty());
    assert!(!baseline.controls.is_empty());
    assert_eq!(baseline.path_ids.len(), 2);
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in ids {
        far.translate(id, anchor).unwrap();
    }
    assert!(
        crate::pick_sources::Sources::new(&far)
            .unwrap()
            .path_ids
            .len()
            < baseline.path_ids.len()
    );
    let current = crate::pick_sources::Sources::anchored(&far, anchor).unwrap();
    assert_eq!(current.faces, baseline.faces);
    assert_eq!(current.edges, baseline.edges);
    assert_eq!(current.controls, baseline.controls);
    assert_eq!(current.mesh_bases, baseline.mesh_bases);
    assert_eq!(current.mesh_ink, baseline.mesh_ink);
    assert_eq!(current.mesh_ids, baseline.mesh_ids);
    assert_eq!(current.line_ids, baseline.line_ids);
    assert_eq!(current.path_ids, baseline.path_ids);
    assert_eq!(current.marker_ids, baseline.marker_ids);
    assert_eq!(current.sheet_bases, baseline.sheet_bases);
    assert_eq!(
        current.path_ids.len(),
        far.paths()
            .iter()
            .flat_map(|row| row.segments_at(anchor))
            .count()
    );
    assert_eq!(
        current.marker_ids.len(),
        far.points().len() + far.control_markers_at(anchor).len()
    );
    assert_eq!(
        current.mesh_ids.len(),
        current.mesh_ink.len()
    );
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}
```

### 4. `src/pick_sources.rs`

Reuse the original source-table owner in the same coordinate frame as its packed ink. Pack placed precise mesh boundaries relative to the same anchor without changing their original edge keys. Build compact path addresses from the same local duplicate predicate as the drawable chain while retaining each raw original edge index. Keep control-net positions and original parent addresses in one frame. Keep control marker addresses aligned with their anchor-relative original positions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        {
            sources.marker_ids.push(id(row.id, 1));
        }
        for (owner, stroke) in scene.control_nets() {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers() {
            let ids = append(
```

Replace it with:

```rust
        {
            sources.marker_ids.push(id(row.id, 1));
        }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(
```

**Locate this block:**

```rust
            let points: Vec<_> = row
                .points()
                .iter()
                .map(session_rust::Point::to_f32)
                .collect();
            for (index, _) in points
                .windows(2)
```

Replace it with:

```rust
            let points: Vec<_> = row
                .points()
                .iter()
                .map(|p| {
                    crate::render_space::relative(
                        crate::clipping_geometry::array(p),
                        anchor,
                    )
                })
                .collect();
            for (index, _) in points
                .windows(2)
```

**Locate this block:**

```rust
                    },
                )?;
                let ends = ends.map(|p| {
                    row.model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
                        .to_f32()
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
```

Replace it with:

```rust
                    },
                )?;
                let ends = ends.map(|p| {
                    let p = row
                        .model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                    crate::render_space::relative(
                        crate::clipping_geometry::array(&p),
                        anchor,
                    )
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
```

**Locate this block:**

```rust
    pub fn new(scene: &Scene) -> Result<Self, &'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| {
            [
```

Replace it with:

```rust
    pub fn new(scene: &Scene) -> Result<Self, &'static str> {
        Self::anchored(scene, [0.; 3])
    }
    pub fn anchored(scene: &Scene, anchor: [f64; 3]) -> Result<Self, &'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| {
            [
```

## <a id="checkpoint-96zzzzzb-prepare-shared-origin-frame-inputs"></a>4. Prepare shared-origin frame inputs

### 1. `src/render_space.rs`

Expose the current origin once, using the existing zero frame before its first update.

**Locate this existing block:**

```rust
impl Origin {
    pub fn forget(&mut self) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Origin {
    pub fn anchor(&self) -> [f64; 3] {
        self.anchor.unwrap_or([0.; 3])
    }
    pub fn forget(&mut self) {
```

### 2. `src/renderer.rs`

Keep one origin policy under the renderer owner. Begin in the existing zero-origin frame. Let every derived consumer read the same precise origin.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        renderer.set_scene(scene, None);
        renderer
    }
```

Replace it with:

```rust
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self) -> [f64; 3] {
        self.space.anchor()
    }
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
            ambient: Default::default(),
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
            ambient: Default::default(),
```

**Locate this block:**

```rust
pub struct Renderer {
    opacity: std::cell::Cell<f32>,
    ambient_enabled: std::cell::Cell<bool>,
    ambient: std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
```

Replace it with:

```rust
pub struct Renderer {
    space: crate::render_space::Origin,
    opacity: std::cell::Cell<f32>,
    ambient_enabled: std::cell::Cell<bool>,
    ambient: std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
```

### 3. `src/clipping_gpu.rs`

Reuse one clipping conversion with an explicit anchor while retaining zero-origin callers. Use the existing precise plane/hatch conversion relative to the declared frame origin.

**Locate this existing block:**

```rust
    ) {
        let matrix = session_rust::Xform::from_matrix(matrix.map(f64::from));
        let planes = if self.enabled {
            self.rows
                .iter()
                .map(|r| r.plane)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let uniform = clip_uniform(
            &planes,
            u32::from(self.solid),
            &ClipView {
                view_proj: &matrix,
                anchor: [0.; 3],
                height,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) {
        self.frame_anchored(queue, matrix, [0.; 3], height, density)
    }
    pub fn frame_anchored(
        &mut self,
        queue: &wgpu::Queue,
        matrix: [f32; 16],
        anchor: [f64; 3],
        height: u32,
        density: f32,
    ) {
        let matrix = session_rust::Xform::from_matrix(matrix.map(f64::from));
        let planes = if self.enabled {
            self.rows
                .iter()
                .map(|r| r.plane)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let uniform = clip_uniform(
            &planes,
            u32::from(self.solid),
            &ClipView {
                view_proj: &matrix,
                anchor,
                height,
```

### 4. `src/renderer.rs`

Prepare pick clipping in the renderer’s current frame. Keep the first-pick crossing preparation on the same origin. Use the same origin for visible clipping and cap inputs. Provide the precise text origin; retain world planes for the text engine’s pre-subtraction CPU visibility test.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ) {
        let (clip_group, clip_revision) = {
            let mut clip = self.clipping.borrow_mut();
            clip.frame(
                &self.queue,
                *transform,
                self.size[1],
```

Replace it with:

```rust
    ) {
        let (clip_group, clip_revision) = {
            let mut clip = self.clipping.borrow_mut();
            clip.frame_anchored(
                &self.queue,
                *transform,
                self.anchor(),
                self.size[1],
```

**Locate this block:**

```rust
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: [0.; 3],
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
```

Replace it with:

```rust
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: self.anchor(),
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
```

**Locate this block:**

```rust
        }
        self.clipping.borrow_mut().frame(
            &self.queue,
            transform,
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame(
                &self.queue,
                transform,
                self.size[1],
                self.density,
            );
```

Replace it with:

```rust
        }
        self.clipping.borrow_mut().frame_anchored(
            &self.queue,
            transform,
            self.anchor(),
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame_anchored(
                &self.queue,
                transform,
                self.anchor(),
                self.size[1],
                self.density,
            );
```

### 5. `src/ambient_rows.rs`

Retain precise world floor height with the ambient source owner. Subtract the frame origin before narrowing ambient floor height; radius remains a distance. Avoid losing a small floor displacement during source preparation.

**Locate this existing block:**

```rust
    pub specs: Vec<(u32, f32)>,
    pub ground: [f32; 2],
}
impl Rows {
    pub fn set(&mut self, scene: &crate::scene::Scene) {
        self.specs.clear();
        let mut floor = f64::INFINITY;
        let mut radius = 0f32;
        for object in scene
            .objects()
            .iter()
            .filter(|r| scene.visible(r.id) && !r.mesh.indices().is_empty())
        {
            if let Some(bounds) = self
                .bounds
                .placed(&object.mesh, &object.model)
            {
                let r = crate::ambient_radius::ambient_radius(&bounds);
                self.specs.push((object.id.value(), r));
                floor = floor.min(bounds.cz - bounds.hz);
                radius = radius.max(r);
            }
        }
        self.ground = [
            if floor.is_finite() {
                floor as f32
            } else {
                0.
            },
            radius,
        ];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub specs: Vec<(u32, f32)>,
    pub ground: [f64; 2],
}
impl Rows {
    pub fn ground_at(&self, anchor: [f64; 3]) -> [f32; 2] {
        [
            (self.ground[0] - anchor[2]) as f32,
            self.ground[1] as f32,
        ]
    }
    pub fn set(&mut self, scene: &crate::scene::Scene) {
        self.specs.clear();
        let mut floor = f64::INFINITY;
        let mut radius = 0f32;
        for object in scene
            .objects()
            .iter()
            .filter(|r| scene.visible(r.id) && !r.mesh.indices().is_empty())
        {
            if let Some(bounds) = self
                .bounds
                .placed(&object.mesh, &object.model)
            {
                let r = crate::ambient_radius::ambient_radius(&bounds);
                self.specs.push((object.id.value(), r));
                floor = floor.min(bounds.cz - bounds.hz);
                radius = radius.max(r);
            }
        }
        self.ground = [
            if floor.is_finite() {
                floor
            } else {
                0.
            },
            f64::from(radius),
        ];
```

### 6. `src/renderer.rs`

Feed ambient preparation the same anchor-relative ground height.

**Locate this existing block:**

```rust
                    self.density,
                    self.ambient_rows.ground,
                    [self.pyramid_revision, clip_revision],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    self.density,
                    self.ambient_rows.ground_at(self.anchor()),
                    [self.pyramid_revision, clip_revision],
```

### 7. `src/global_outline_tests.rs`

Supply actual local/distant clipped cap pixels, shared object records, world-space text predicates, exact relative ground height, unchanged source uploads and repeated-uniform reuse.

**Locate this existing block:**

```rust
    assert_eq!(gpu.stats()[0], uploads);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(gpu.stats()[0], uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground() {
    let mut scene = Scene::empty();
    let mut prepared =
        PreparedMesh::new(session_rust::Mesh::create_box(0.5, 0.5, 0.5)).unwrap();
    prepared.model = session_rust::Xform::translation(0., 0., -0.5);
    let owner = scene.insert(prepared).unwrap();
    let plane = crate::clipping_geometry::plane_from(
        crate::clipping_geometry::Mode::Xy,
        &[[0., 0., -0.5]],
        1.,
    )
    .unwrap();
    let plane_id = crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        crate::cap_gpu_tests::at(&baseline, 160, 120)[..3]
            .iter()
            .all(|v| (150..=156).contains(v))
    );
    let ground = gpu.ambient_rows.ground_at([0.; 3]);
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let uploads = gpu.stats()[0];
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in [owner, plane_id] {
        far.translate(id, anchor).unwrap();
    }
    gpu.set_scene(&far, None);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    assert!(
        gpu.space
            .update(anchor, 1., 0.)
            .unwrap()
            .moved
    );
    for mesh in &mut gpu.meshes {
        mesh.rebase(&gpu.queue, anchor);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    assert_eq!(gpu.anchor(), anchor);
    assert_eq!(gpu.ambient_rows.ground_at(anchor), ground);
    assert_eq!(gpu.ambient_rows.ground[0] - anchor[2], -0.75);
    let frame = gpu.text_frame(matrix());
    assert_eq!(frame.origin, anchor);
    assert!(!frame.cut([anchor[0], anchor[1], anchor[2] - 0.75]));
    assert!(frame.cut([anchor[0], anchor[1], anchor[2] - 0.25]));
    assert_eq!(pixels(&gpu), baseline);
    let writes = gpu.clipping.borrow().writes;
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(gpu.clipping.borrow().writes, writes);
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
}
```

## <a id="checkpoint-96zzzzzc-prepare-cloud-and-sheet-frame-placements"></a>5. Prepare cloud and sheet frame placements

### 1. `src/render_space.rs`

Subtract the origin in double precision before GPU narrowing or matrix composition.

**Locate this existing block:**

```rust
}
#[derive(Default)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub fn placed(place: &session_rust::Xform, anchor: [f64; 3]) -> session_rust::Xform {
    let mut local = place.clone();
    for i in 0..3 {
        local.m[12 + i] -= anchor[i];
    }
    local
}
#[derive(Default)]
```

### 2. `src/gpu_mesh.rs`

Reuse the same precise model conversion for existing object records.

**Locate this existing block:**

```rust
    let mut bytes = [0; BYTES];
    let mut model = object.model.to_f32();
    model[12..15].copy_from_slice(&crate::render_space::relative(
        object.model.m[12..15].try_into().unwrap(),
        anchor,
    ));
    for (index, value) in model.iter().enumerate() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut bytes = [0; BYTES];
    let model = crate::render_space::placed(&object.model, anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
```

### 3. `src/cloud_gpu.rs`

Carry the declared origin with cloud view inputs. Retain the earlier default cloud frame. Keep zero-origin callers while preparing one explicit camera-relative cloud view. Retain the exact origin alongside the narrowed eye. Subtract the anchor before the eye offset and narrowing, preserving local camera detail. Include origin identity independently of matrix and eye values. Invalidate retained cloud records when only the frame origin changes. Select cloud detail with eye and source placement in the same local frame. Prepare one precise local placement for cloud records and normal/position inputs. Compose projection after subtracting the precise source placement, then narrow once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
    let row = &draw.row;
    let model = row.model.to_f32();
    let scale = row.model.uniform_scale();
    let mut total = 0u32;
    let m = (&session_rust::Xform::from_matrix(transform.map(f64::from)) * &row.model)
        .to_f32();
    let rot = [
```

Replace it with:

```rust
    ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
    let row = &draw.row;
    let local = crate::render_space::placed(&row.model, view.anchor);
    let model = local.to_f32();
    let scale = row.model.uniform_scale();
    let mut total = 0u32;
    let m =
        (&session_rust::Xform::from_matrix(transform.map(f64::from)) * &local).to_f32();
    let rot = [
```

**Locate this block:**

```rust
        view: View,
    ) -> Result<(), &'static str> {
        let key = Key {
            transform,
            size,
            density: density.to_bits(),
```

Replace it with:

```rust
        view: View,
    ) -> Result<(), &'static str> {
        let key = Key {
            anchor: view.anchor.map(f64::to_bits),
            transform,
            size,
            density: density.to_bits(),
```

**Locate this block:**

```rust
                continue;
            }
            let row = &draw.row;
            let model = row.model.to_f32();
            let projection = Projection {
                eye: view.eye,
                ortho_h: view.ortho_h,
```

Replace it with:

```rust
                continue;
            }
            let row = &draw.row;
            let model = crate::render_space::placed(&row.model, view.anchor).to_f32();
            let projection = Projection {
                eye: view.eye,
                ortho_h: view.ortho_h,
```

**Locate this block:**

```rust
}
#[derive(PartialEq)]
struct Key {
    transform: [f32; 16],
    size: [u32; 2],
    density: u32,
```

Replace it with:

```rust
}
#[derive(PartialEq)]
struct Key {
    anchor: [u64; 3],
    transform: [f32; 16],
    size: [u32; 2],
    density: u32,
```

**Locate this block:**

```rust
}
impl View {
    pub fn camera(camera: &crate::camera::Camera) -> Self {
        let forward = camera
            .orientation
            .rotate_vector(session_rust::Vector::y_axis());
        Self {
            eye: std::array::from_fn(|i| {
                (camera.target[i] - forward[i] * camera.distance) as f32
            }),
            ortho_h: if camera.projection == crate::camera::Projection::Orthographic {
                (camera.distance * std::f64::consts::FRAC_PI_6.tan()) as f32
```

Replace it with:

```rust
}
impl View {
    pub fn camera(camera: &crate::camera::Camera) -> Self {
        Self::anchored(camera, [0.; 3])
    }
    pub fn anchored(camera: &crate::camera::Camera, anchor: [f64; 3]) -> Self {
        let forward = camera
            .orientation
            .rotate_vector(session_rust::Vector::y_axis());
        Self {
            anchor,
            eye: std::array::from_fn(|i| {
                (camera.target[i] - anchor[i] - forward[i] * camera.distance) as f32
            }),
            ortho_h: if camera.projection == crate::camera::Projection::Orthographic {
                (camera.distance * std::f64::consts::FRAC_PI_6.tan()) as f32
```

**Locate this block:**

```rust
impl Default for View {
    fn default() -> Self {
        Self {
            eye: [0., 0., 3.],
            ortho_h: 0.,
            lod_px: 2.,
```

Replace it with:

```rust
impl Default for View {
    fn default() -> Self {
        Self {
            anchor: [0.; 3],
            eye: [0., 0., 3.],
            ortho_h: 0.,
            lod_px: 2.,
```

**Locate this block:**

```rust
pub struct View {
    pub eye: [f32; 3],
    pub ortho_h: f32,
    pub lod_px: f32,
```

Replace it with:

```rust
pub struct View {
    pub anchor: [f64; 3],
    pub eye: [f32; 3],
    pub ortho_h: f32,
    pub lod_px: f32,
```

### 4. `src/sheet_gpu.rs`

Reuse sheet geometry/cache ownership with an explicit placement origin. Build each original sheet placement in its declared frame. Make precise sheet placement conversion explicit without changing original entity addresses. Reuse the same double-precision local model before packing sheet uniforms.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    row: &crate::sheet_source::Row,
    base: u32,
    selected: bool,
) -> Vec<u8> {
    let mut bytes = row
        .model
        .m
```

Replace it with:

```rust
    row: &crate::sheet_source::Row,
    base: u32,
    selected: bool,
    anchor: [f64; 3],
) -> Vec<u8> {
    let mut bytes = crate::render_space::placed(&row.model, anchor)
        .m
```

**Locate this block:**

```rust
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(scene, row, base, selected.contains(&row.id));
                    let draw = if let Some(i) = old.iter().position(|d| {
                        d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry)
                    }) {
```

Replace it with:

```rust
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(
                        scene,
                        row,
                        base,
                        selected.contains(&row.id),
                        anchor,
                    );
                    let draw = if let Some(i) = old.iter().position(|d| {
                        d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry)
                    }) {
```

**Locate this block:**

```rust
    ) -> Result<(), &'static str> {
        let mut old = std::mem::take(&mut self.draws);
        for row in scene.sheets().iter().filter(|row| {
            scene.visible(row.id) && row.prepared.original.segment_count > 0
```

Replace it with:

```rust
    ) -> Result<(), &'static str> {
        self.set_anchored(
            device, queue, scene, selected, bases, [0.; 3],
        )
    }
    pub fn set_anchored(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        let mut old = std::mem::take(&mut self.draws);
        for row in scene.sheets().iter().filter(|row| {
            scene.visible(row.id) && row.prepared.original.segment_count > 0
```

### 5. `src/renderer.rs`

Capture the renderer origin before mutable lane updates. Keep sheet source adoption in the renderer’s current frame. Keep selection updates in the retained frame. Preserve original sheet selection without reverting its local placement. Supply cloud eye, origin and record-cache inputs from one renderer frame.

**Locate this existing block:**

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);
        if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
            if self.ambient_enabled.get() {
                ambient.sources.set(
                    &self.device,
                    &self.queue,
                    &self.ambient_rows.specs,
                );
            }
        }
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
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
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
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
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
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
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
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
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
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
                &self.clipping.borrow().buffer,
            );
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            if let Err(error) = lane.prepare(
                &self.text_context(),
                &self.text_frame(transform),
            ) {
                *self.text_error.borrow_mut() = Some(error.to_string());
                return;
            }
        }
        self.clipping.borrow_mut().frame_anchored(
            &self.queue,
            transform,
            self.anchor(),
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame_anchored(
                &self.queue,
                transform,
                self.anchor(),
                self.size[1],
                self.density,
            );
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
                let mut tiles = self.tiles.borrow_mut();
                tiles.prepare(&self.device, &self.queue, self.size);
                let mut encoder = self
                    .device
                    .create_command_encoder(&Default::default());
                tiles.encode(
                    &self.device,
                    &mut encoder,
                    &self.view_group,
                    transform,
                );
                self.queue.submit([encoder.finish()]);
                tiles.map_report();
                crate::finite_ink::Tables {
                    projected: tiles.projected.clone(),
                    tiles: tiles.buffer.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
            self.picker.submit_finite(
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
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
                Some(&tables),
                &clipping.group,
                &clipping.buffer,
                Some(&mut self.caps.borrow_mut()),
                Some(&self.instances),
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
        let mut usage = crate::memory::gpu(&self.meshes);
        usage[1] += self.geometry.allocated_bytes();
        usage
    }
    pub fn arena_cpu_bytes(&self) -> usize {
        self.geometry.cpu_bytes()
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * crate::object_record::BYTES as u64
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
        self.set_grid_on(settings, crate::cplane::CPlane::Xy)
    }
    pub fn set_grid_on(
        &mut self,
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings && plane == self.grid_plane {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on(plane))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
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
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        self.instances.set(
            &self.device,
            &self.queue,
            &mut self.meshes,
            &self.instance_rows,
            false,
        )?;
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        self.set_clouds(scene, ids)?;
        self.sheets.set(
            &self.device,
            &self.queue,
            scene,
            ids,
            &self.sources.sheet_bases,
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
        self.set_scene_text(scene, ids)
            .map_err(|_| "Cannot prepare scene labels")?;
        Ok(())
    }

    pub fn set_guides(
        &mut self,
        overlay: &crate::guide_gpu::Overlay,
    ) -> Result<(), &'static str> {
        if self.guides.set(&self.device, overlay)? {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare editing guide label")?
        }
        Ok(())
    }
    pub fn guide_bytes(&self) -> u64 {
        self.guides.bytes()
    }
    pub fn set_measurement(
        &mut self,
        mark: Option<&crate::measure_mark::Mark>,
    ) -> Result<(), &'static str> {
        if self.measurement.mark.as_ref() == mark {
            return Ok(());
        }
        self.measurement.set(&self.device, mark)?;
        self.set_scene_measure_text()
            .map_err(|_| "Cannot prepare measurement annotation")
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
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

    fn text_context(&self) -> crate::text_gpu_support::GpuCtx {
        crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue)
    }
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: self.anchor(),
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
                self.size[1] as f64 / self.density as f64,
            ],
            ortho_half_height: 0.,
            clip: {
                let clip = self.clipping.borrow();
                let mut planes = [[0.; 4]; 6];
                if clip.enabled {
                    for (p, row) in planes.iter_mut().zip(&clip.rows) {
                        *p = [
                            row.plane.normal[0],
                            row.plane.normal[1],
                            row.plane.normal[2],
                            row.plane.offset,
                        ];
                    }
                }
                planes
            },
        }
    }
    fn adopt_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            self.text_error.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        if lane.is_none() {
            let mut new = crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            );
            if let Some(fonts) = self.full_text_fonts.borrow().as_ref() {
                new.document.replace_fonts(fonts.clone())?;
            }
            *lane = Some(new);
        }
        lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> anyhow::Result<()> {
        for (i, label) in labels.iter_mut().enumerate() {
            label.id = u32::MAX - i as u32;
            label.object = None;
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
    fn set_scene_text(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> anyhow::Result<()> {
        let labels = scene
            .text_labels(selected)
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
                    .document
                    .runs
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
                                [
                                    w.max(line.line_w as f64),
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
    pub fn text_stats(&self) -> Option<crate::scene_text_gpu::TextStats> {
        self.text
            .borrow()
            .as_ref()
            .map(|lane| lane.stats.clone())
    }
    pub fn text_scene_usage(&self) -> [u64; 4] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 4], |lane| {
                [
                    lane.allocated_bytes(),
                    lane.texture_bytes(),
                    lane.stats.world_plane_rasterizations,
                    lane.document.runs.len() as u64,
                ]
            })
    }
    pub fn replace_text_fonts(&self, fonts: Vec<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.document.replace_fonts(fonts.clone())?;
        } else {
            let mut document = crate::text::TextDocument::new();
            document.replace_fonts(fonts.clone())?;
        }
        *self.full_text_fonts.borrow_mut() = Some(fonts);
        Ok(())
    }
    pub fn clear_text_fonts(&self) {
        self.full_text_fonts.borrow_mut().take();
    }

    pub fn text_error(&self) -> Option<String> {
        self.text_error.borrow().clone()
    }
    pub fn text_usage(&self) -> [u64; 6] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                [
                    lane.document.shape_count,
                    lane.stats.preparations,
                    lane.stats.skipped_preparations,
                    lane.stats.raster_images as u64,
                    lane.stats.raster_image_capacity_bytes as u64,
                    lane.stats.atlas_resets,
                ]
            })
    }
    pub fn text_demo(&self) -> anyhow::Result<()> {
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn cloud_camera(&self, camera: &crate::camera::Camera) {
        self.cloud_view
            .set(crate::cloud_gpu::View::camera(camera));
        self.sheets.camera(camera);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor = self.anchor();
        self.ambient_rows.set(scene);
        if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
            if self.ambient_enabled.get() {
                ambient.sources.set(
                    &self.device,
                    &self.queue,
                    &self.ambient_rows.specs,
                );
            }
        }
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
            .set_points(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker();
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
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
                        if let Some(colour) = scene.face_rgba(row.id) {
                            for segment in &mut segments {
                                segment.stroke.colour = colour;
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
                        if let Some(colour) = scene.face_rgba(row.id) {
                            stroke.colour = colour;
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
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
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
        self.caps.borrow_mut().set_scene(
            &self.queue,
            scene,
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
            .set_anchored(
                &self.device,
                &self.queue,
                scene,
                selected.as_slice(),
                &self.sources.sheet_bases,
                anchor,
            )
            .expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_clouds(scene, selected.as_slice()) {
            *self.cloud_error.borrow_mut() = Some(error.to_string());
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
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
                &self.clipping.borrow().buffer,
            );
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            if let Err(error) = lane.prepare(
                &self.text_context(),
                &self.text_frame(transform),
            ) {
                *self.text_error.borrow_mut() = Some(error.to_string());
                return;
            }
        }
        self.clipping.borrow_mut().frame_anchored(
            &self.queue,
            transform,
            self.anchor(),
            self.size[1],
            self.density,
        );
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping = self.clipping.borrow_mut();
            clipping.frame_anchored(
                &self.queue,
                transform,
                self.anchor(),
                self.size[1],
                self.density,
            );
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
                let mut tiles = self.tiles.borrow_mut();
                tiles.prepare(&self.device, &self.queue, self.size);
                let mut encoder = self
                    .device
                    .create_command_encoder(&Default::default());
                tiles.encode(
                    &self.device,
                    &mut encoder,
                    &self.view_group,
                    transform,
                );
                self.queue.submit([encoder.finish()]);
                tiles.map_report();
                crate::finite_ink::Tables {
                    projected: tiles.projected.clone(),
                    tiles: tiles.buffer.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
            self.picker.submit_finite(
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
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
                Some(&tables),
                &clipping.group,
                &clipping.buffer,
                Some(&mut self.caps.borrow_mut()),
                Some(&self.instances),
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
        let mut usage = crate::memory::gpu(&self.meshes);
        usage[1] += self.geometry.allocated_bytes();
        usage
    }
    pub fn arena_cpu_bytes(&self) -> usize {
        self.geometry.cpu_bytes()
    }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * crate::object_record::BYTES as u64
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
        self.set_grid_on(settings, crate::cplane::CPlane::Xy)
    }
    pub fn set_grid_on(
        &mut self,
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings && plane == self.grid_plane {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on(plane))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
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
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        let anchor = self.anchor();
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        if let Some(outline) = self.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
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
        self.instances.set(
            &self.device,
            &self.queue,
            &mut self.meshes,
            &self.instance_rows,
            false,
        )?;
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        self.set_clouds(scene, ids)?;
        self.sheets.set_anchored(
            &self.device,
            &self.queue,
            scene,
            ids,
            &self.sources.sheet_bases,
            anchor,
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
        self.set_scene_text(scene, ids)
            .map_err(|_| "Cannot prepare scene labels")?;
        Ok(())
    }

    pub fn set_guides(
        &mut self,
        overlay: &crate::guide_gpu::Overlay,
    ) -> Result<(), &'static str> {
        if self.guides.set(&self.device, overlay)? {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare editing guide label")?
        }
        Ok(())
    }
    pub fn guide_bytes(&self) -> u64 {
        self.guides.bytes()
    }
    pub fn set_measurement(
        &mut self,
        mark: Option<&crate::measure_mark::Mark>,
    ) -> Result<(), &'static str> {
        if self.measurement.mark.as_ref() == mark {
            return Ok(());
        }
        self.measurement.set(&self.device, mark)?;
        self.set_scene_measure_text()
            .map_err(|_| "Cannot prepare measurement annotation")
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
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

    fn text_context(&self) -> crate::text_gpu_support::GpuCtx {
        crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue)
    }
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: self.anchor(),
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
                self.size[1] as f64 / self.density as f64,
            ],
            ortho_half_height: 0.,
            clip: {
                let clip = self.clipping.borrow();
                let mut planes = [[0.; 4]; 6];
                if clip.enabled {
                    for (p, row) in planes.iter_mut().zip(&clip.rows) {
                        *p = [
                            row.plane.normal[0],
                            row.plane.normal[1],
                            row.plane.normal[2],
                            row.plane.offset,
                        ];
                    }
                }
                planes
            },
        }
    }
    fn adopt_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            self.text_error.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        if lane.is_none() {
            let mut new = crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            );
            if let Some(fonts) = self.full_text_fonts.borrow().as_ref() {
                new.document.replace_fonts(fonts.clone())?;
            }
            *lane = Some(new);
        }
        lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> anyhow::Result<()> {
        for (i, label) in labels.iter_mut().enumerate() {
            label.id = u32::MAX - i as u32;
            label.object = None;
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
    fn set_scene_text(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> anyhow::Result<()> {
        let labels = scene
            .text_labels(selected)
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
                    .document
                    .runs
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
                                [
                                    w.max(line.line_w as f64),
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
    pub fn text_stats(&self) -> Option<crate::scene_text_gpu::TextStats> {
        self.text
            .borrow()
            .as_ref()
            .map(|lane| lane.stats.clone())
    }
    pub fn text_scene_usage(&self) -> [u64; 4] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 4], |lane| {
                [
                    lane.allocated_bytes(),
                    lane.texture_bytes(),
                    lane.stats.world_plane_rasterizations,
                    lane.document.runs.len() as u64,
                ]
            })
    }
    pub fn replace_text_fonts(&self, fonts: Vec<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.document.replace_fonts(fonts.clone())?;
        } else {
            let mut document = crate::text::TextDocument::new();
            document.replace_fonts(fonts.clone())?;
        }
        *self.full_text_fonts.borrow_mut() = Some(fonts);
        Ok(())
    }
    pub fn clear_text_fonts(&self) {
        self.full_text_fonts.borrow_mut().take();
    }

    pub fn text_error(&self) -> Option<String> {
        self.text_error.borrow().clone()
    }
    pub fn text_usage(&self) -> [u64; 6] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                [
                    lane.document.shape_count,
                    lane.stats.preparations,
                    lane.stats.skipped_preparations,
                    lane.stats.raster_images as u64,
                    lane.stats.raster_image_capacity_bytes as u64,
                    lane.stats.atlas_resets,
                ]
            })
    }
    pub fn text_demo(&self) -> anyhow::Result<()> {
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn cloud_camera(&self, camera: &crate::camera::Camera) {
        self.cloud_view
            .set(crate::cloud_gpu::View::anchored(
                camera,
                self.anchor(),
            ));
        self.sheets.camera(camera);
```

### 6. `src/cap_gpu_tests.rs`

Reuse the existing optional GPU readback fixture with an explicit matrix; retain every earlier cap pixel assertion.

**Locate this existing block:**

```rust
pub(crate) fn pixels(r: &Renderer) -> Vec<u8> {
    let texture = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("original cap pixel fixture"),
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
    r.draw(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &matrix(),
    );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn pixels(r: &Renderer) -> Vec<u8> {
    pixels_at(r, matrix())
}
pub(crate) fn pixels_at(r: &Renderer, transform: [f32; 16]) -> Vec<u8> {
    let texture = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("original cap pixel fixture"),
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
    r.draw(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &transform,
    );
```

### 7. `src/global_outline_tests.rs`

Supply actual local/distant cloud and sheet pixels, unchanged uploads, equal-matrix camera pan/rebase cache invalidation, exact restoration, original picks and empty release while sources remain retained.

**Locate this existing block:**

```rust
        source
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        source
    );
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views() {
    let mut scene = Scene::empty();
    let mut source = session_rust::PointCloud::from_coords(
        vec![-0.5, 0., -0.5],
        vec![255, 0, 0, 255],
        vec![],
    );
    source.point_size = 16.;
    let source = Rc::new(source);
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let sheet = Rc::new(session_rust::proto::Sheet {
        guid: crate::sheet_example::source().guid,
        coords: vec![0.25, 0., -0.5, 0.75, 0., -0.5],
        widths: vec![0.0625],
        colors: vec![0xff00_0000],
        segment_count: 1,
        entity_count: 1,
        source_ids: vec![0],
        ..Default::default()
    });
    let drawing = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(sheet.clone()).unwrap(),
        ))
        .unwrap();
    let original = source.to_proto();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    camera.projection = crate::camera::Projection::Orthographic;
    gpu.cloud_camera(&camera);
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    assert!(
        baseline
            .chunks_exact(4)
            .any(|p| p[0] > 200 && p[1] < 20 && p[2] < 20)
    );
    assert!(
        baseline
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 20
    );
    let uploads = (gpu.cloud_usage()[2], gpu.sheet_usage()[3]);
    assert_eq!(uploads, (1, 1));
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in [cloud, drawing] {
        far.translate(id, anchor).unwrap();
    }
    gpu.set_scene(&far, None);
    assert!(
        gpu.space
            .update(anchor, 2., 0.)
            .unwrap()
            .moved
    );
    let mut far_camera = crate::camera::Camera::default();
    far_camera.distance = camera.distance;
    far_camera.aspect = camera.aspect;
    far_camera.projection = camera.projection;
    far_camera.target = anchor;
    gpu.cloud_camera(&far_camera);
    gpu.sheets
        .set_anchored(
            &gpu.device,
            &gpu.queue,
            &far,
            &[],
            &gpu.sources.sheet_bases,
            anchor,
        )
        .unwrap();
    let local = far_camera
        .view_projection_anchored(anchor)
        .to_f32();
    assert_eq!(local, transform);
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert_eq!(
        (gpu.cloud_usage()[2], gpu.sheet_usage()[3]),
        uploads
    );
    let prepared = gpu.cloud_usage()[3];
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert_eq!(gpu.cloud_usage()[3], prepared);
    let moved = [anchor[0] + 2000., anchor[1], anchor[2]];
    far_camera.target = moved;
    assert!(
        gpu.space
            .update(moved, 2., 300.)
            .unwrap()
            .moved
    );
    gpu.cloud_camera(&far_camera);
    gpu.sheets
        .set_anchored(
            &gpu.device,
            &gpu.queue,
            &far,
            &[],
            &gpu.sources.sheet_bases,
            moved,
        )
        .unwrap();
    let same_matrix = far_camera
        .view_projection_anchored(moved)
        .to_f32();
    assert_eq!(same_matrix, local);
    let panned = crate::cap_gpu_tests::pixels_at(&gpu, same_matrix);
    assert!(
        panned
            .chunks_exact(4)
            .all(|p| p == &[255; 4])
    );
    assert!(
        gpu.cloud_usage()[3] > prepared,
        "Origin alone invalidates the equal-matrix cloud view"
    );
    assert_eq!(
        (gpu.cloud_usage()[2], gpu.sheet_usage()[3]),
        uploads
    );
    far_camera.target = anchor;
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
            .unwrap()
            .moved
    );
    gpu.cloud_camera(&far_camera);
    gpu.sheets
        .set_anchored(
            &gpu.device,
            &gpu.queue,
            &far,
            &[],
            &gpu.sources.sheet_bases,
            anchor,
        )
        .unwrap();
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    for (point, id) in [
        ([-0.5, 0., -0.5], cloud),
        ([0.5, 0., -0.5], drawing),
    ] {
        let point =
            camera
                .view_projection()
                .transform_point(&session_rust::Point::new(
                    point[0], point[1], point[2],
                ));
        gpu.request_pick([point[0] as f32, point[1] as f32], local)
            .unwrap();
        wait(&gpu);
        assert_eq!(
            gpu.take_pick(local)
                .unwrap()
                .unwrap()
                .unwrap()[0],
            id.value()
        );
    }
    assert_eq!(source.to_proto(), original);
    assert!(Rc::ptr_eq(
        &far.sheets()[0].prepared.original,
        &sheet
    ));
    assert_eq!(
        (gpu.cloud_usage()[2], gpu.sheet_usage()[3]),
        uploads
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.cloud_usage(), [0; 5]);
    assert_eq!(&gpu.sheet_usage()[..2], &[0, 0]);
}
```

## <a id="checkpoint-96zzzzzd-prepare-local-temporary-overlays"></a>6. Prepare local temporary overlays

### 1. `src/tool_preview.rs`

Reuse the draft preview collector with an explicit frame origin. Retain precise draft outline positions until the local GPU boundary. Pack the precise cursor marker in the same frame.

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        let yellow = crate::colour::selection();
        let mut strokes = Vec::new();
        let mut markers = Vec::new();
        if let Some(draft) = draft {
            for wire in if draft.extend.is_some() {
                Vec::new()
            } else {
                draft.outlines()
            } {
                for pair in wire.windows(2) {
                    if pair[0] != pair[1] {
                        strokes.push(crate::stroke::Stroke {
                            start: pair[0].map(|v| v as f32),
                            end: pair[1].map(|v| v as f32),
                            width: 2.,
                            colour: yellow,
                        });
                    }
                }
            }
            if let Some(cursor) = draft.cursor {
                markers.push(crate::marker::Marker {
                    center: cursor.map(|v| v as f32),
                    diameter: 8.,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        self.set_at(device, draft, [0.; 3])
    }
    pub fn set_at(
        &mut self,
        device: &wgpu::Device,
        draft: Option<&Draft>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        let yellow = crate::colour::selection();
        let mut strokes = Vec::new();
        let mut markers = Vec::new();
        if let Some(draft) = draft {
            for wire in if draft.extend.is_some() {
                Vec::new()
            } else {
                draft.outlines()
            } {
                for pair in wire.windows(2) {
                    if pair[0] != pair[1] {
                        strokes.push(crate::stroke::Stroke {
                            start: crate::render_space::relative(pair[0], anchor),
                            end: crate::render_space::relative(pair[1], anchor),
                            width: 2.,
                            colour: yellow,
                        });
                    }
                }
            }
            if let Some(cursor) = draft.cursor {
                markers.push(crate::marker::Marker {
                    center: crate::render_space::relative(cursor, anchor),
                    diameter: 8.,
```

### 2. `src/guide_gpu.rs`

Retain the collector origin as part of guide cache identity. Share precise marker packing across trim and extension guides. Narrow guide wires only after subtracting their collector origin.

**Locate this existing block:**

```rust
pub struct Overlay {
    pub segments: Vec<Segment>,
    pub marks: Vec<Marker>,
    pub label: Option<([f64; 3], String)>,
}
impl Overlay {
    pub fn wire(&mut self, points: &[[f64; 3]], colour: [f32; 4], dashed: bool) {
        for p in points.windows(2) {
            if p[0] != p[1] {
                self.segments.push(Segment {
                    stroke: Stroke {
                        start: p[0].map(|v| v as f32),
                        end: p[1].map(|v| v as f32),
                        colour,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Overlay {
    pub anchor: [f64; 3],
    pub segments: Vec<Segment>,
    pub marks: Vec<Marker>,
    pub label: Option<([f64; 3], String)>,
}
impl Overlay {
    pub fn at(anchor: [f64; 3]) -> Self {
        Self {
            anchor,
            ..Default::default()
        }
    }
    pub fn mark(&mut self, point: [f64; 3], diameter: f32, colour: [f32; 4]) {
        self.marks.push(Marker {
            center: crate::render_space::relative(point, self.anchor),
            diameter,
            colour,
        });
    }
    pub fn wire(&mut self, points: &[[f64; 3]], colour: [f32; 4], dashed: bool) {
        for p in points.windows(2) {
            if p[0] != p[1] {
                self.segments.push(Segment {
                    stroke: Stroke {
                        start: crate::render_space::relative(p[0], self.anchor),
                        end: crate::render_space::relative(p[1], self.anchor),
                        colour,
```

### 3. `src/editor.rs`

Keep the legacy collector and expose the shared frame to guide producers. Route trim guidance through the same origin-aware collector. Collect extension wires under the declared frame. Keep the precise guide point and world label before their separate GPU boundaries.

**Locate this existing block:**

```rust
    pub fn guide_overlay(&self) -> crate::guide_gpu::Overlay {
        if let Some(trim) = &self.trim {
            return trim.overlay();
        }
        let mut overlay = crate::guide_gpu::Overlay::default();
        let red = [210. / 255., 40. / 255., 40. / 255., 1.];
        let Some(extend) = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.as_ref())
        else {
            return overlay;
        };
        let label = if let Some(hover) = &extend.hover {
            overlay.wire(&hover.tail, red, true);
            hover.tail.last().map(|point| {
                let added = hover
                    .tail
                    .windows(2)
                    .map(|p| {
                        (p[1][0] - p[0][0])
                            .hypot(p[1][1] - p[0][1])
                            .hypot(p[1][2] - p[0][2])
                    })
                    .sum::<f64>();
                (*point, format!("+{added:.3}"))
            })
        } else if !extend.error.is_empty() {
            extend.last.and_then(|(id, end)| {
                let owner = &extend
                    .candidates
                    .iter()
                    .find(|c| c.0.id == id)?
                    .0;
                let (tip, _) = crate::extend_reach::tip(&owner.source, end)?;
                let p = tip.transformed(&owner.model);
                Some(([p[0], p[1], p[2]], extend.error.clone()))
            })
        } else {
            None
        };
        if let Some((point, word)) = label {
            overlay.marks.push(crate::marker::Marker {
                center: point.map(|v| v as f32),
                diameter: 6.,
                colour: red,
            });
            overlay.label = Some((point, word));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn guide_overlay(&self) -> crate::guide_gpu::Overlay {
        self.guide_overlay_at([0.; 3])
    }
    pub fn guide_overlay_at(&self, anchor: [f64; 3]) -> crate::guide_gpu::Overlay {
        if let Some(trim) = &self.trim {
            return trim.overlay_at(anchor);
        }
        let mut overlay = crate::guide_gpu::Overlay::at(anchor);
        let red = [210. / 255., 40. / 255., 40. / 255., 1.];
        let Some(extend) = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.as_ref())
        else {
            return overlay;
        };
        let label = if let Some(hover) = &extend.hover {
            overlay.wire(&hover.tail, red, true);
            hover.tail.last().map(|point| {
                let added = hover
                    .tail
                    .windows(2)
                    .map(|p| {
                        (p[1][0] - p[0][0])
                            .hypot(p[1][1] - p[0][1])
                            .hypot(p[1][2] - p[0][2])
                    })
                    .sum::<f64>();
                (*point, format!("+{added:.3}"))
            })
        } else if !extend.error.is_empty() {
            extend.last.and_then(|(id, end)| {
                let owner = &extend
                    .candidates
                    .iter()
                    .find(|c| c.0.id == id)?
                    .0;
                let (tip, _) = crate::extend_reach::tip(&owner.source, end)?;
                let p = tip.transformed(&owner.model);
                Some(([p[0], p[1], p[2]], extend.error.clone()))
            })
        } else {
            None
        };
        if let Some((point, word)) = label {
            overlay.mark(point, 6., red);
            overlay.label = Some((point, word));
```

### 4. `src/trim_draft.rs`

Preserve one trim overlay algorithm while making its local origin explicit. Retain precise trim intersection points until guide marker packing.

**Locate this existing block:**

```rust
    pub fn overlay(&self) -> crate::guide_gpu::Overlay {
        let mut overlay = crate::guide_gpu::Overlay::default();
        if self.phase != Phase::Parts {
            return overlay;
        }
        let red = [210. / 255., 40. / 255., 40. / 255., 1.];
        let blue = [30. / 255., 110. / 255., 170. / 255., 1.];
        for (i, cut) in self.cuts.iter().enumerate() {
            for (part, line) in cut.outlines.iter().enumerate() {
                if cut.removed.contains(&part) {
                    continue;
                }
                let hovered = self.hover == Some((i, part));
                let points = line
                    .iter()
                    .map(|p| [p[0], p[1], p[2]])
                    .collect::<Vec<_>>();
                overlay.wire(
                    &points,
                    if hovered {
                        red
                    } else {
                        blue
                    },
                    hovered,
                );
                if hovered {
                    if let Some(point) = points.get(points.len() / 2) {
                        overlay.label = Some((*point, "remove".into()))
                    }
                }
            }
            for line in &cut.section {
                overlay.wire(
                    &line
                        .iter()
                        .map(|p| [p[0], p[1], p[2]])
                        .collect::<Vec<_>>(),
                    red,
                    false,
                )
            }
            overlay.marks.extend(
                cut.marks
                    .iter()
                    .map(|p| crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 6.,
                        colour: blue,
                    }),
            );
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn overlay(&self) -> crate::guide_gpu::Overlay {
        self.overlay_at([0.; 3])
    }
    pub fn overlay_at(&self, anchor: [f64; 3]) -> crate::guide_gpu::Overlay {
        let mut overlay = crate::guide_gpu::Overlay::at(anchor);
        if self.phase != Phase::Parts {
            return overlay;
        }
        let red = [210. / 255., 40. / 255., 40. / 255., 1.];
        let blue = [30. / 255., 110. / 255., 170. / 255., 1.];
        for (i, cut) in self.cuts.iter().enumerate() {
            for (part, line) in cut.outlines.iter().enumerate() {
                if cut.removed.contains(&part) {
                    continue;
                }
                let hovered = self.hover == Some((i, part));
                let points = line
                    .iter()
                    .map(|p| [p[0], p[1], p[2]])
                    .collect::<Vec<_>>();
                overlay.wire(
                    &points,
                    if hovered {
                        red
                    } else {
                        blue
                    },
                    hovered,
                );
                if hovered {
                    if let Some(point) = points.get(points.len() / 2) {
                        overlay.label = Some((*point, "remove".into()))
                    }
                }
            }
            for line in &cut.section {
                overlay.wire(
                    &line
                        .iter()
                        .map(|p| [p[0], p[1], p[2]])
                        .collect::<Vec<_>>(),
                    red,
                    false,
                )
            }
            for p in &cut.marks {
                overlay.mark(crate::clipping_geometry::array(p), 6., blue);
            }
        }
```

### 5. `src/measure_gpu.rs`

Keep the packing origin beside the retained precise measurement. Begin with the earlier zero-origin measurement cache. Report actual measurement changes while retaining existing zero-origin callers. Observe origin changes even when precise measurement points are unchanged. Pack both precise measurement endpoints in the declared frame. Publish the cache witness only after successful derived-ink adoption.

**Locate this existing block:**

```rust
    pub mark: Option<Mark>,
    strokes: Lane,
}
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mark: None,
            strokes: Lane::overlay(device, format),
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        mark: Option<&Mark>,
    ) -> Result<(), &'static str> {
        if self.mark.as_ref() == mark {
            return Ok(());
        }
        let mut strokes = Vec::new();
        if let Some(mark) = mark {
            for p in mark.points.windows(2) {
                if p[0] != p[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: p[0].map(|v| v as f32),
                        end: p[1].map(|v| v as f32),
                        width: 1.5,
                        colour: [20. / 255., 20. / 255., 20. / 255., 1.],
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.mark = mark.cloned();
        Ok(())
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub mark: Option<Mark>,
    anchor: [f64; 3],
    strokes: Lane,
}
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mark: None,
            anchor: [0.; 3],
            strokes: Lane::overlay(device, format),
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        mark: Option<&Mark>,
    ) -> Result<(), &'static str> {
        self.set_at(device, mark, [0.; 3])
            .map(|_| ())
    }
    pub fn set_at(
        &mut self,
        device: &wgpu::Device,
        mark: Option<&Mark>,
        anchor: [f64; 3],
    ) -> Result<bool, &'static str> {
        if self.mark.as_ref() == mark && self.anchor == anchor {
            return Ok(false);
        }
        let mut strokes = Vec::new();
        if let Some(mark) = mark {
            for p in mark.points.windows(2) {
                if p[0] != p[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: crate::render_space::relative(p[0], anchor),
                        end: crate::render_space::relative(p[1], anchor),
                        width: 1.5,
                        colour: [20. / 255., 20. / 255., 20. / 255., 1.],
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.mark = mark.cloned();
        self.anchor = anchor;
        Ok(true)
    }
```

### 6. `src/renderer.rs`

Use the renderer origin for temporary draft geometry. Refresh measurement labels only after source or frame changes.

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        if self.measurement.mark.as_ref() == mark {
            return Ok(());
        }
        self.measurement.set(&self.device, mark)?;
        self.set_scene_measure_text()
            .map_err(|_| "Cannot prepare measurement annotation")
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
    }
    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        self.preview.set(&self.device, draft)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if self
            .measurement
            .set_at(&self.device, mark, anchor)?
        {
            self.set_scene_measure_text()
                .map_err(|_| "Cannot prepare measurement annotation")?;
        }
        Ok(())
    }
    pub fn measurement_bytes(&self) -> u64 {
        self.measurement.bytes()
    }
    fn merge_measurement(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> Vec<crate::text::TextLabel> {
        if let Some(mark) = &self.measurement.mark {
            labels.extend(mark.labels(&labels));
        }
        labels.extend(self.guides.overlay.labels(&labels));
        labels
    }
    fn set_scene_measure_text(&self) -> anyhow::Result<()> {
        let mut labels = self.scene_text.borrow().clone();
        labels.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(labels))
    }
    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        self.preview
            .set_at(&self.device, draft, anchor)
    }
```

### 7. `src/browser.rs`

Collect browser guide coordinates using the renderer’s current origin.

**Locate this existing block:**

```rust
            }
            if let Err(error) = renderer.set_guides(&editor.guide_overlay()) {
                panel.result(error);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            }
            let guides = editor.guide_overlay_at(renderer.anchor());
            if let Err(error) = renderer.set_guides(&guides) {
                panel.result(error);
```

### 8. `src/global_outline_tests.rs`

Supply actual local/distant temporary ink and glyph pixels, origin-only invalidation of unchanged measurements, exact restoration, no invented pick ownership and complete temporary GPU release.

**Locate this existing block:**

```rust
    assert_eq!(&gpu.sheet_usage()[..2], &[0, 0]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(&gpu.sheet_usage()[..2], &[0, 0]);
}

fn temporary_inputs(gpu: &mut super::Renderer, world: [f64; 3]) {
    let point = |p: [f64; 3]| std::array::from_fn(|i| p[i] + world[i]);
    let mut guides = crate::guide_gpu::Overlay::at(gpu.anchor());
    guides.wire(
        &[
            point([-0.75, 0.25, -0.5]),
            point([0.75, 0.25, -0.5]),
        ],
        [210. / 255., 40. / 255., 40. / 255., 1.],
        true,
    );
    guides.mark(
        point([0.5, 0.25, -0.5]),
        6.,
        [0., 0., 1., 1.],
    );
    guides.label = Some((point([0.5, 0.25, -0.5]), "guide".into()));
    gpu.set_guides(&guides).unwrap();
    let mark = crate::measure_mark::Mark {
        points: vec![
            point([-0.5, -0.25, -0.5]),
            point([0.5, -0.25, -0.5]),
        ],
        label: "distance".into(),
    };
    gpu.set_measurement(Some(&mark)).unwrap();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit(point([0., 0., -0.5])).unwrap();
    draft
        .hover(Some(point([0.5, 0., -0.5])))
        .unwrap();
    gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release() {
    let scene = Scene::empty();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    let transform = camera.uniform();
    temporary_inputs(&mut gpu, [0.; 3]);
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    assert!(
        baseline
            .chunks_exact(4)
            .any(|p| p[0] > 200 && p[1] > 200 && p[2] < 30)
    );
    assert!(
        baseline
            .chunks_exact(4)
            .any(|p| p[0] > 150 && p[1] < 80 && p[2] < 80)
    );
    assert!(
        baseline
            .chunks_exact(4)
            .filter(|p| p[..3] != [255; 3])
            .count()
            > 200
    );
    let bytes = (
        gpu.guide_bytes(),
        gpu.preview_bytes(),
        gpu.measurement.bytes(),
    );
    assert!(bytes.0 > 0 && bytes.1 > 0 && bytes.2 > 0);
    assert_eq!(gpu.text_error(), None);
    let anchor = [1e9, -1e9, 1e9];
    assert!(
        gpu.space
            .update(anchor, 2., 0.)
            .unwrap()
            .moved
    );
    camera.target = anchor;
    temporary_inputs(&mut gpu, anchor);
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
    assert_eq!(local, transform);
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert_eq!(
        (
            gpu.guide_bytes(),
            gpu.preview_bytes(),
            gpu.measurement.bytes()
        ),
        bytes
    );
    let moved = [anchor[0] + 2000., anchor[1], anchor[2]];
    camera.target = moved;
    assert!(
        gpu.space
            .update(moved, 2., 300.)
            .unwrap()
            .moved
    );
    temporary_inputs(&mut gpu, anchor);
    assert_eq!(
        camera
            .view_projection_anchored(moved)
            .to_f32(),
        local
    );
    assert!(
        crate::cap_gpu_tests::pixels_at(&gpu, local)
            .chunks_exact(4)
            .all(|p| p == &[255; 4])
    );
    camera.target = anchor;
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
            .unwrap()
            .moved
    );
    temporary_inputs(&mut gpu, anchor);
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert!(scene.all_ids().is_empty());
    gpu.request_pick([0., 0.], local).unwrap();
    wait(&gpu);
    assert_eq!(gpu.take_pick(local).unwrap().unwrap(), None);
    gpu.set_preview(None).unwrap();
    gpu.set_measurement(None).unwrap();
    gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor))
        .unwrap();
    crate::cap_gpu_tests::pixels_at(&gpu, local);
    assert_eq!(gpu.preview_bytes(), 0);
    assert_eq!(gpu.guide_bytes(), 0);
    assert_eq!(gpu.measurement.bytes(), 0);
    assert_eq!(gpu.text_usage()[0], 0);
}
```

## <a id="checkpoint-96zzzzze-preserve-local-precision-before-source-placement"></a>7. Preserve local precision before source placement

### 1. `src/scene.rs`

Retain world endpoints and expose one precise local endpoint path. Subtract the placement origin before adding original local endpoint coordinates. Narrow already-local precise endpoints once. Use one precise path position owner before GPU duplicate filtering. Reuse retained precise samples through an anchor-relative placement instead of a rounded world intermediate. Keep world point callers while sharing a precise local point transform. Narrow original point markers after their local transform. Make the collected control coordinate frame explicit. Transform controls through the precise local placement. Collect original markers in the declared local frame. Keep original net links in the same local frame. Collected control positions are already local; narrow without subtracting twice. Pack local control links without a second origin subtraction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                else {
                    continue;
                };
                let start = crate::render_space::relative(a.position, anchor);
                let end = crate::render_space::relative(b.position, anchor);
                if start
```

Replace it with:

```rust
                else {
                    continue;
                };
                let start = a.position.map(|v| v as f32);
                let end = b.position.map(|v| v as f32);
                if start
```

**Locate this block:**

```rust
        anchor: [f64; 3],
    ) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls
                    .points
```

Replace it with:

```rust
        anchor: [f64; 3],
    ) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources_at(anchor) {
            for [a, b] in controls.links {
                let Some((a, b)) = controls
                    .points
```

**Locate this block:**

```rust
    }
    pub fn control_markers_at(&self, anchor: [f64; 3]) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = crate::render_space::relative(control.position, anchor);
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
```

Replace it with:

```rust
    }
    pub fn control_markers_at(&self, anchor: [f64; 3]) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources_at(anchor) {
            for control in controls.points {
                let center = control.position.map(|v| v as f32);
                if center.iter().any(|v| !v.is_finite()) {
                    continue;
                }
```

**Locate this block:**

```rust
            .filter(|(id, _)| self.visible(*id))
            .filter_map(|(id, legacy)| {
                let source = crate::source_edit::original(self, id)?;
                let model = self.placement(id)?;
                let mut controls =
                    crate::source_controls::Controls::from_geometry(&source);
                for control in &mut controls.points {
```

Replace it with:

```rust
            .filter(|(id, _)| self.visible(*id))
            .filter_map(|(id, legacy)| {
                let source = crate::source_edit::original(self, id)?;
                let model = crate::render_space::placed(&self.placement(id)?, anchor);
                let mut controls =
                    crate::source_controls::Controls::from_geometry(&source);
                for control in &mut controls.points {
```

**Locate this block:**

```rust
        }
    }

    fn control_sources(
        &self,
    ) -> Vec<(
        ObjectId,
        bool,
```

Replace it with:

```rust
        }
    }

    fn control_sources_at(
        &self,
        anchor: [f64; 3],
    ) -> Vec<(
        ObjectId,
        bool,
```

**Locate this block:**

```rust
    }
    pub fn marker_at(&self, anchor: [f64; 3]) -> crate::marker::Marker {
        crate::marker::Marker {
            center: crate::render_space::relative(
                crate::clipping_geometry::array(&self.point()),
                anchor,
            ),
            ..self.prepared.display
        }
    }
```

Replace it with:

```rust
    }
    pub fn marker_at(&self, anchor: [f64; 3]) -> crate::marker::Marker {
        crate::marker::Marker {
            center: self.point_at(anchor).to_f32(),
            ..self.prepared.display
        }
    }
```

**Locate this block:**

```rust
    }

    pub fn point(&self) -> session_rust::Point {
        self.model
            .transform_point(&self.prepared.source)
    }
    pub fn marker(&self) -> crate::marker::Marker {
```

Replace it with:

```rust
    }

    pub fn point(&self) -> session_rust::Point {
        self.point_at([0.; 3])
    }
    pub fn point_at(&self, anchor: [f64; 3]) -> session_rust::Point {
        crate::render_space::placed(&self.model, anchor)
            .transform_point(&self.prepared.source)
    }
    pub fn marker(&self) -> crate::marker::Marker {
```

**Locate this block:**

```rust
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared
            .source
            .coordinates()
            .into_iter()
            .map(|p| {
                self.model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
            })
            .collect()
    }
}
```

Replace it with:

```rust
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.points_at([0.; 3])
    }
    pub fn points_at(&self, anchor: [f64; 3]) -> Vec<session_rust::Point> {
        let model = crate::render_space::placed(&self.model, anchor);
        self.prepared
            .source
            .coordinates()
            .into_iter()
            .map(|p| model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])))
            .collect()
    }
}
```

**Locate this block:**

```rust
    }
    pub fn segments_at(&self, anchor: [f64; 3]) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = crate::render_space::relative(
                crate::clipping_geometry::array(&p),
                anchor,
            );
            if points.last() != Some(&p) {
                points.push(p);
            }
```

Replace it with:

```rust
    }
    pub fn segments_at(&self, anchor: [f64; 3]) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points_at(anchor) {
            let p = p.to_f32();
            if points.last() != Some(&p) {
                points.push(p);
            }
```

**Locate this block:**

```rust
        self.stroke_at([0.; 3])
    }
    pub fn stroke_at(&self, anchor: [f64; 3]) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke {
            start: crate::render_space::relative(
                crate::clipping_geometry::array(&start),
                anchor,
            ),
            end: crate::render_space::relative(
                crate::clipping_geometry::array(&end),
                anchor,
            ),
            ..self.prepared.display
        }
    }
```

Replace it with:

```rust
        self.stroke_at([0.; 3])
    }
    pub fn stroke_at(&self, anchor: [f64; 3]) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints_at(anchor);
        crate::stroke::Stroke {
            start: start.to_f32(),
            end: end.to_f32(),
            ..self.prepared.display
        }
    }
```

**Locate this block:**

```rust
    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [
            self.model.transform_point(&line.start()),
            self.model.transform_point(&line.end()),
        ]
    }
```

Replace it with:

```rust
    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        self.endpoints_at([0.; 3])
    }
    pub fn endpoints_at(&self, anchor: [f64; 3]) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        let model = crate::render_space::placed(&self.model, anchor);
        [
            model.transform_point(&line.start()),
            model.transform_point(&line.end()),
        ]
    }
```

### 2. `src/pick_sources.rs`

Use one precise local placement for each original boundary owner. Avoid a large intermediate world coordinate for original mesh/CAD/surface pick ink. Keep compact pick addresses on the exact same local sample path as drawn ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .filter(|row| scene.visible(row.id))
        {
            let points: Vec<_> = row
                .points()
                .iter()
                .map(|p| {
                    crate::render_space::relative(
                        crate::clipping_geometry::array(p),
                        anchor,
                    )
                })
                .collect();
```

Replace it with:

```rust
            .filter(|row| scene.visible(row.id))
        {
            let points: Vec<_> = row
                .points_at(anchor)
                .iter()
                .map(session_rust::Point::to_f32)
                .collect();
```

**Locate this block:**

```rust
                    },
                )?;
                let ends = ends.map(|p| {
                    let p = row
                        .model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                    crate::render_space::relative(
                        crate::clipping_geometry::array(&p),
                        anchor,
                    )
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
```

Replace it with:

```rust
                    },
                )?;
                let ends = ends.map(|p| {
                    model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
                        .to_f32()
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
```

**Locate this block:**

```rust
        {
            sources.mesh_bases.push((
                row.id,
                u32::try_from(sources.faces.len())
```

Replace it with:

```rust
        {
            let model = crate::render_space::placed(&row.model, anchor);
            sources.mesh_bases.push((
                row.id,
                u32::try_from(sources.faces.len())
```

### 3. `src/cloud_pick.rs`

Retain original world control positions and prepare an explicit local lookup. Apply the local placement for streamed original cloud IDs. Use the same placement for resident original cloud IDs.

**Locate this existing block:**

```rust
) -> Option<session_rust::Point> {
    if let Some(row) = scene
        .streams()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)
    {
        scene.pick_id(owner.value())?;
        let p = row.prepared.position(id)?;
        return Some(
            row.model
                .transform_point(&session_rust::Point::new(p[0], p[1], p[2])),
        );
    }
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index = row.prepared.source.index_of_id(id)?;
    Some(
        row.model
            .transform_point(&row.prepared.source.get_point(index)),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Option<session_rust::Point> {
    position_at(scene, owner, id, [0.; 3])
}
pub fn position_at(
    scene: &Scene,
    owner: ObjectId,
    id: u32,
    anchor: [f64; 3],
) -> Option<session_rust::Point> {
    if let Some(row) = scene
        .streams()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)
    {
        scene.pick_id(owner.value())?;
        let p = row.prepared.position(id)?;
        return Some(
            crate::render_space::placed(&row.model, anchor)
                .transform_point(&session_rust::Point::new(p[0], p[1], p[2])),
        );
    }
    let row = scene
        .clouds()
        .iter()
        .find(|r| r.id == owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index = row.prepared.source.index_of_id(id)?;
    Some(
        crate::render_space::placed(&row.model, anchor)
            .transform_point(&row.prepared.source.get_point(index)),
```

### 4. `src/scene_origin_tests.rs`

Supply exact nonbinary local line/path/point/control/cloud and pick-table data through large original placements without changing source IDs.

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
fn nonbinary_local_sources_and_pick_tables_survive_large_placements() {
    let mut scene = crate::scene::Scene::empty();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [0.1, 0.2, -0.5],
                    [0.3, 0.2, -0.5],
                    [0.2, 0.4, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let line = scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                0.1, 0.2, -0.5, 0.3, 0.4, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let path = scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(vec![
                    session_rust::Point::new(0.1, 0.2, -0.5),
                    session_rust::Point::new(0.3, 0.4, -0.5),
                    session_rust::Point::new(0.5, 0.2, -0.5),
                ]),
            ))
            .unwrap(),
        )
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0.1, 0.2, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let mut proto =
        session_rust::PointCloud::from_coords(vec![0.1, 0.2, -0.5], vec![], vec![])
            .to_proto();
    proto.point_ids = vec![u32::MAX];
    let cloud = scene
        .insert_cloud(
            crate::cloud::Prepared::new(Rc::new(
                session_rust::PointCloud::from_proto(proto),
            ))
            .unwrap(),
        )
        .unwrap();
    scene.owner_controls(mesh).unwrap();
    scene.cloud_controls(Some(cloud));
    let baseline = crate::pick_sources::Sources::new(&scene).unwrap();
    let controls = scene
        .control_markers()
        .iter()
        .map(|c| (c.parent, c.part, c.display.values()))
        .collect::<Vec<_>>();
    let nets = scene.control_nets();
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in [mesh, line, path, point, cloud] {
        far.translate(id, anchor).unwrap();
    }
    assert_eq!(
        far.lines()[0].stroke_at(anchor).bytes(),
        scene.lines()[0].stroke().bytes()
    );
    assert_eq!(
        far.paths()[0]
            .segments_at(anchor)
            .iter()
            .flat_map(|s| s.bytes())
            .collect::<Vec<_>>(),
        scene.paths()[0]
            .segments()
            .iter()
            .flat_map(|s| s.bytes())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        far.points()[0].marker_at(anchor).values(),
        scene.points()[0].marker().values()
    );
    assert_eq!(
        far.control_markers_at(anchor)
            .iter()
            .map(|c| (c.parent, c.part, c.display.values()))
            .collect::<Vec<_>>(),
        controls
    );
    assert_eq!(far.control_nets_at(anchor), nets);
    let current = crate::pick_sources::Sources::anchored(&far, anchor).unwrap();
    assert_eq!(current.mesh_ink, baseline.mesh_ink);
    assert_eq!(current.mesh_ids, baseline.mesh_ids);
    assert_eq!(current.path_ids, baseline.path_ids);
    assert_eq!(current.controls, baseline.controls);
    assert_eq!(
        crate::cloud_pick::position_at(&far, cloud, u32::MAX, anchor)
            .unwrap()
            .to_f32(),
        [0.1f32, 0.2, -0.5]
    );
}
```

## <a id="checkpoint-96zzzzzf-prepare-precise-construction-grid-origins"></a>8. Prepare precise construction-grid origins

### 1. `src/cplane.rs`

Retain one construction-plane matrix recipe in double precision; existing GPU callers narrow at their boundary.

**Locate this existing block:**

```rust
    pub fn matrix(self) -> [f32; 16] {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        [
            x[0] as f32,
            x[1] as f32,
            x[2] as f32,
            0.0,
            y[0] as f32,
            y[1] as f32,
            y[2] as f32,
            0.0,
            z[0] as f32,
            z[1] as f32,
            z[2] as f32,
            0.0,
            o[0] as f32,
            o[1] as f32,
            o[2] as f32,
            1.0,
        ]
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn matrix(self) -> [f32; 16] {
        self.matrix64().to_f32()
    }
    pub fn matrix64(self) -> session_rust::Xform {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        session_rust::Xform::from_matrix([
            x[0], x[1], x[2], 0.0, y[0], y[1], y[2], 0.0, z[0], z[1], z[2], 0.0, o[0],
            o[1], o[2], 1.0,
        ])
    }
```

### 2. `src/grid.rs`

Reuse grid generation in a precise declared frame. Apply local grid positions through the origin-relative precise plane matrix before narrowing.

**Locate this existing block:**

```rust
    ) -> Result<Vec<Stroke>, &'static str> {
        let mut strokes = self.strokes()?;
        for stroke in &mut strokes {
            for p in [&mut stroke.start, &mut stroke.end] {
                let world = plane.world(p.map(f64::from));
                let values = [
                    world[0] as f32,
                    world[1] as f32,
                    world[2] as f32,
                ];
                if values.iter().any(|v| !v.is_finite()) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<Vec<Stroke>, &'static str> {
        self.strokes_on_at(plane, [0.; 3])
    }
    pub fn strokes_on_at(
        self,
        plane: crate::cplane::CPlane,
        anchor: [f64; 3],
    ) -> Result<Vec<Stroke>, &'static str> {
        let mut strokes = self.strokes()?;
        let model = crate::render_space::placed(&plane.matrix64(), anchor);
        for stroke in &mut strokes {
            for p in [&mut stroke.start, &mut stroke.end] {
                let values = model
                    .transform_point(&session_rust::Point::new(
                        f64::from(p[0]),
                        f64::from(p[1]),
                        f64::from(p[2]),
                    ))
                    .to_f32();
                if values.iter().any(|v| !v.is_finite()) {
```

### 3. `src/renderer.rs`

Retain the origin used by grid vertices. Begin with the existing grid frame. Include origin changes when deciding whether cached grid data is current. Pack the grid through the current renderer origin. Publish the complete grid witness after successful adoption.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings && plane == self.grid_plane {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on(plane))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
        Ok(())
```

Replace it with:

```rust
        settings: Option<crate::grid::Settings>,
        plane: crate::cplane::CPlane,
    ) -> Result<(), &'static str> {
        let anchor = self.anchor();
        if settings == self.grid_settings
            && plane == self.grid_plane
            && anchor == self.grid_anchor
        {
            return Ok(());
        }
        let strokes = settings
            .map(|s| s.strokes_on_at(plane, anchor))
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        self.grid_plane = plane;
        self.grid_anchor = anchor;
        Ok(())
```

**Locate this block:**

```rust
            grid,
            grid_settings: None,
            grid_plane: crate::cplane::CPlane::Xy,
            size: [640, 480],
            density: 1.0,
            picker,
```

Replace it with:

```rust
            grid,
            grid_settings: None,
            grid_plane: crate::cplane::CPlane::Xy,
            grid_anchor: [0.; 3],
            size: [640, 480],
            density: 1.0,
            picker,
```

**Locate this block:**

```rust
    grid_plane: crate::cplane::CPlane,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
```

Replace it with:

```rust
    grid_plane: crate::cplane::CPlane,
    grid_anchor: [f64; 3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
```

### 4. `src/scene_origin_tests.rs`

Supply an exact nonbinary grid comparison under a large independently placed construction plane.

**Locate this existing block:**

```rust
        [0.1f32, 0.2, -0.5]
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        [0.1f32, 0.2, -0.5]
    );
}

#[test]
fn nonbinary_construction_grid_uses_local_plane_before_narrowing() {
    let settings = crate::grid::Settings {
        spacing: 0.1,
        half_cells: 3,
    };
    let anchor = [1e9, -1e9, 1e9];
    let axes = |origin| crate::cplane::CPlane::Frame {
        origin,
        x: [0., 1., 0.],
        y: [-1., 0., 0.],
    };
    let local = settings.strokes_on(axes([0.; 3])).unwrap();
    assert_eq!(
        settings
            .strokes_on_at(axes(anchor), anchor)
            .unwrap(),
        local
    );
    assert_eq!(
        axes(anchor).matrix64().to_f32(),
        axes(anchor).matrix()
    );
    assert!(
        local
            .iter()
            .flat_map(|s| [s.start, s.end])
            .flatten()
            .any(|v| v.abs() == 0.1f32)
    );
}
```

### 5. `src/global_outline_tests.rs`

Supply actual grid/surface pixels, unchanged source geometry uploads, origin-only cache invalidation under an identical local matrix, exact restoration, original picks and empty release.

**Locate this existing block:**

```rust
    assert_eq!(gpu.text_usage()[0], 0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(gpu.text_usage()[0], 0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut gpu = renderer(&scene);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let settings = Some(crate::grid::Settings {
        spacing: 0.1,
        half_cells: 5,
    });
    let plane = |origin| crate::cplane::CPlane::Frame {
        origin,
        x: [1., 0., 0.],
        y: [0., 1., 0.],
    };
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    let uploads = gpu.stats()[0];
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    gpu.set_scene(&far, None);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.space.update(anchor, 2., 0.).unwrap();
    for mesh in &mut gpu.meshes {
        mesh.rebase(&gpu.queue, anchor);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    let far_plane = plane([anchor[0], anchor[1], anchor[2] - 0.75]);
    gpu.set_grid_on(settings, far_plane).unwrap();
    camera.target = anchor;
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
    assert_eq!(local, transform);
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    let grid_uploads = gpu.grid_usage()[2];
    gpu.set_grid_on(settings, far_plane).unwrap();
    assert_eq!(gpu.grid_usage()[2], grid_uploads);
    let moved = [anchor[0] + 2000., anchor[1], anchor[2]];
    gpu.space.update(moved, 2., 300.).unwrap();
    camera.target = moved;
    for mesh in &mut gpu.meshes {
        mesh.rebase(&gpu.queue, moved);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.set_grid_on(settings, far_plane).unwrap();
    assert!(gpu.grid_usage()[2] > grid_uploads);
    assert_eq!(
        camera
            .view_projection_anchored(moved)
            .to_f32(),
        local
    );
    assert!(
        crate::cap_gpu_tests::pixels_at(&gpu, local)
            .chunks_exact(4)
            .all(|p| p == &[255; 4])
    );
    gpu.space.update(anchor, 2., 600.).unwrap();
    for mesh in &mut gpu.meshes {
        mesh.rebase(&gpu.queue, anchor);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.set_grid_on(settings, far_plane).unwrap();
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    gpu.request_pick([0., 0.], local).unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(local)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_grid(None).unwrap();
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(&gpu.grid_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzzzzg-initialize-object-records-in-the-local-frame"></a>9. Initialize object records in the local frame

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

Initialize derived geometry records in their actual frame; retain zero-origin callers. Keep a single record initialization path for both shared geometry and derived faces. Retain the frame used by the initialized record.

**Locate this existing block:**

```rust
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
        let bytes = settings(
            object,
            style,
            0,
            0,
            0,
            geometry.index_start,
            [0.; 3],
        );
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
            translation: object.model.m[12..15].try_into().unwrap(),
            anchor: [0.; 3],
            geometry,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        Self::upload_at(device, layout, object, selected, [0.; 3])
    }
    pub fn upload_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        anchor: [f64; 3],
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry_at(
            device,
            layout,
            object,
            Style {
                selected,
                pickable: !object.metadata.is_locked,
                colour: None,
            },
            geometry,
            anchor,
        )
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        Self::with_geometry_at(
            device, layout, object, style, geometry, [0.; 3],
        )
    }
    pub fn with_geometry_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
        anchor: [f64; 3],
    ) -> Self {
        let bytes = settings(
            object,
            style,
            0,
            0,
            0,
            geometry.index_start,
            anchor,
        );
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
            translation: object.model.m[12..15].try_into().unwrap(),
            anchor,
            geometry,
```

### 2. `src/global_outline_tests.rs`

Supply actual initial local/distant records and pixels, subsequent exact settings writes, shared geometry identity and unchanged sources.

**Locate this existing block:**

```rust
    assert_eq!(&gpu.grid_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(&gpu.grid_usage()[..2], &[0, 0]);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.25, -0.5],
                    [0.3, -0.25, -0.5],
                    [0.1, 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut gpu = renderer(&scene);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let layout = gpu.pipeline.get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload(
        &gpu.device,
        &layout,
        &scene.objects()[0],
        false,
    );
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    let original = gpu.meshes[0].packed();
    let uploads = gpu.stats()[0];
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    let layout = gpu.pipeline.get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload_at(
        &gpu.device,
        &layout,
        &far.objects()[0],
        false,
        anchor,
    );
    assert_eq!(gpu.meshes[0].packed(), original);
    let geometry = gpu.meshes[0].geometry.clone();
    let style = crate::gpu_mesh::Style {
        selected: false,
        pickable: true,
        colour: None,
    };
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::with_geometry_at(
        &gpu.device,
        &layout,
        &far.objects()[0],
        style,
        geometry.clone(),
        anchor,
    );
    assert_eq!(gpu.meshes[0].packed(), original);
    assert!(Rc::ptr_eq(
        &gpu.meshes[0].geometry,
        &geometry
    ));
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    camera.target = anchor;
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
    assert_eq!(local, transform);
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert_eq!(
        gpu.meshes[0].update(&gpu.queue, &far.objects()[0], style),
        [0, 0]
    );
    let selected = crate::gpu_mesh::Style {
        selected: true,
        ..style
    };
    assert_eq!(
        gpu.meshes[0].update(&gpu.queue, &far.objects()[0], selected),
        [1, 16]
    );
    assert_eq!(
        &gpu.meshes[0].packed()[..64],
        &original[..64]
    );
    assert_eq!(
        gpu.meshes[0].update(&gpu.queue, &far.objects()[0], style),
        [1, 16]
    );
    assert_eq!(gpu.meshes[0].packed(), original);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    assert_eq!(gpu.stats()[0], uploads);
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzzzzh-prepare-anchor-relative-original-highlights"></a>10. Prepare highlights in the shared local frame

### 1. `src/highlight.rs`

Expose the origin once at highlight preparation. Place derived face records precisely before their first draw. Apply the existing precise local placement before transforming original boundary coordinates. Reuse the already taught precise local source accessor for highlight presentation. Share the declared origin with whole-object ink selection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .map(|r| crate::marker::Marker {
                    colour: yellow,
                    diameter: r.prepared.display.diameter + 4.,
                    ..r.marker()
                }),
```

Replace it with:

```rust
                .map(|r| crate::marker::Marker {
                    colour: yellow,
                    diameter: r.prepared.display.diameter + 4.,
                    ..r.marker_at(anchor)
                }),
```

**Locate this block:**

```rust
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
```

Replace it with:

```rust
                .iter()
                .filter(|r| ids.contains(&r.id) && scene.visible(r.id))
                .flat_map(|r| {
                    r.segments_at(anchor)
                        .into_iter()
                        .map(|mut s| {
                            s.stroke.colour = yellow;
                            s.stroke.width += 2.;
                            s
                        })
                }),
        )?;
        self.markers.set_points(
```

**Locate this block:**

```rust
                                .iter()
                                .map(move |pair| {
                                    let ends = pair.map(|p| {
                                        r.model
                                            .transform_point(&session_rust::Point::new(
                                                p[0], p[1], p[2],
                                            ))
```

Replace it with:

```rust
                                .iter()
                                .map(move |pair| {
                                    let ends = pair.map(|p| {
                                        crate::render_space::placed(&r.model, anchor)
                                            .transform_point(&session_rust::Point::new(
                                                p[0], p[1], p[2],
                                            ))
```

**Locate this block:**

```rust
                .map(|r| crate::stroke::Stroke {
                    colour: yellow,
                    width: r.prepared.display.width + 2.,
                    ..r.stroke()
                })
                .chain(
                    scene
```

Replace it with:

```rust
                .map(|r| crate::stroke::Stroke {
                    colour: yellow,
                    width: r.prepared.display.width + 2.,
                    ..r.stroke_at(anchor)
                })
                .chain(
                    scene
```

**Locate this block:**

```rust
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let yellow = crate::colour::selection();
        self.strokes.set(
```

Replace it with:

```rust
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.objects_at(device, scene, ids, [0.; 3])
    }
    pub fn objects_at(
        &mut self,
        device: &wgpu::Device,
        scene: &Scene,
        ids: &[crate::scene::ObjectId],
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let yellow = crate::colour::selection();
        self.strokes.set(
```

**Locate this block:**

```rust
                    });
                }
                Part::CloudPoint(id) => {
                    let p = crate::cloud_pick::position(scene, target.owner, id)
                        .ok_or("Cloud point is missing")?;
                    markers.push(crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 11.,
```

Replace it with:

```rust
                    });
                }
                Part::CloudPoint(id) => {
                    let p =
                        crate::cloud_pick::position_at(scene, target.owner, id, anchor)
                            .ok_or("Cloud point is missing")?;
                    markers.push(crate::marker::Marker {
                        center: p.to_f32(),
                        diameter: 11.,
```

**Locate this block:**

```rust
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
                Part::Control(_) | Part::SourceControl(_) => {
                    let marker = scene
                        .control_markers()
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.part == target.part)
                        .unwrap();
```

Replace it with:

```rust
                        strokes.push(crate::stroke::Stroke {
                            colour: yellow,
                            width: row.prepared.display.width + 2.,
                            ..row.stroke_at(anchor)
                        });
                    } else if let Some(row) = scene
                        .paths()
                        .iter()
                        .find(|r| r.id == target.owner)
                    {
                        paths.extend(row.segments_at(anchor).into_iter().map(
                            |mut segment| {
                                segment.stroke.colour = yellow;
                                segment.stroke.width += 2.;
                                segment
                            },
                        ));
                    }
                }
                Part::Control(_) | Part::SourceControl(_) => {
                    let marker = scene
                        .control_markers_at(anchor)
                        .into_iter()
                        .find(|m| m.parent == target.owner && m.part == target.part)
                        .unwrap();
```

**Locate this block:**

```rust
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = row.points();
                    strokes.push(crate::stroke::Stroke {
                        start: points[index].to_f32(),
                        end: points[index + 1].to_f32(),
```

Replace it with:

```rust
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = row.points_at(anchor);
                    strokes.push(crate::stroke::Stroke {
                        start: points[index].to_f32(),
                        end: points[index + 1].to_f32(),
```

**Locate this block:**

```rust
                        .position(|&edge| edge == (a, b))
                        .unwrap();
                    let ends = row.mesh.parts.endpoints[index].map(|p| {
                        row.model
                            .transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ))
```

Replace it with:

```rust
                        .position(|&edge| edge == (a, b))
                        .unwrap();
                    let ends = row.mesh.parts.endpoints[index].map(|p| {
                        crate::render_space::placed(&row.model, anchor)
                            .transform_point(&session_rust::Point::new(
                                p[0], p[1], p[2],
                            ))
```

**Locate this block:**

```rust
                    for ends in &boundary.segments {
                        let ends = ends.map(|index| {
                            let p = surface.grid.samples[index as usize].point;
                            row.model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
```

Replace it with:

```rust
                    for ends in &boundary.segments {
                        let ends = ends.map(|index| {
                            let p = surface.grid.samples[index as usize].point;
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
```

**Locate this block:**

```rust
                        .points;
                    for pair in points.windows(2) {
                        let ends = [pair[0], pair[1]].map(|p| {
                            row.model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
```

Replace it with:

```rust
                        .points;
                    for pair in points.windows(2) {
                        let ends = [pair[0], pair[1]].map(|p| {
                            crate::render_space::placed(&row.model, anchor)
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
```

**Locate this block:**

```rust
                        .with_normals(normals)?;
                    let mut object = row.clone();
                    object.mesh = Rc::new(display);
                    self.mesh = Some(GpuMesh::upload(
                        device, layout, &object, true,
                    ));
                }
                Part::BRepEdge(edge) => {
```

Replace it with:

```rust
                        .with_normals(normals)?;
                    let mut object = row.clone();
                    object.mesh = Rc::new(display);
                    self.mesh = Some(GpuMesh::upload_at(
                        device, layout, &object, true, anchor,
                    ));
                }
                Part::BRepEdge(edge) => {
```

**Locate this block:**

```rust
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let mut strokes = Vec::new();
        let mut paths = Vec::new();
```

Replace it with:

```rust
    ) -> Result<(), &'static str> {
        self.set_at(device, layout, scene, target, [0.; 3])
    }
    pub fn set_at(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        scene: &Scene,
        target: Option<Target>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let mut strokes = Vec::new();
        let mut paths = Vec::new();
```

### 2. `src/renderer.rs`

Prepare original part selection in the renderer frame. Prepare whole-object ink selection in that same frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
```

Replace it with:

```rust
        )?;
        if !ids.is_empty() {
            self.highlight
                .objects_at(&self.device, scene, ids, anchor)?;
        }
```

**Locate this block:**

```rust
        }
        self.highlight.set(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
        )
    }
```

Replace it with:

```rust
        }
        let anchor = self.anchor();
        self.highlight.set_at(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
            anchor,
        )
    }
```

### 3. `src/global_outline_tests.rs`

Supply actual local/distant face/edge/curve/control/high-bit cloud and object-highlight pixel comparisons with exact clearing and unchanged sources.

**Locate this existing block:**

```rust
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn actual_original_highlights_keep_local_pixels_addresses_and_sources_after_rebasing() {
    use crate::pick_sources::{Part, Target};
    let mut scene = Scene::empty();
    let mesh = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.25, -0.25, -0.5],
                    [0.25, -0.25, -0.5],
                    [0., 0.25, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let line = scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                -0.4, 0.5, -0.5, 0.4, 0.5, -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let path = scene
        .insert_path(
            crate::chain::PreparedChain::polyline(Rc::new(
                session_rust::Polyline::new(vec![
                    session_rust::Point::new(-0.4, -0.5, -0.5),
                    session_rust::Point::new(0., -0.3, -0.5),
                    session_rust::Point::new(0.4, -0.5, -0.5),
                ]),
            ))
            .unwrap(),
        )
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                0.75, 0., -0.5,
            )))
            .unwrap(),
        )
        .unwrap();
    let mut proto =
        session_rust::PointCloud::from_coords(vec![-0.75, 0., -0.5], vec![], vec![])
            .to_proto();
    proto.point_ids = vec![u32::MAX];
    let cloud = scene
        .insert_cloud(
            crate::cloud::Prepared::new(Rc::new(
                session_rust::PointCloud::from_proto(proto),
            ))
            .unwrap(),
        )
        .unwrap();
    scene.owner_controls(path).unwrap();
    scene.cloud_controls(Some(cloud));
    let curve = scene
        .insert_path(
            crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(
                &[
                    session_rust::Point::new(-0.4, 0.8, -0.5),
                    session_rust::Point::new(0., 0.65, -0.5),
                    session_rust::Point::new(0.4, 0.8, -0.5),
                ],
            )))
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let key = *scene.objects()[0]
        .geometry()
        .unwrap()
        .face
        .keys()
        .next()
        .unwrap();
    let edge = scene.objects()[0].mesh.parts.edges[0];
    let control = scene
        .control_markers()
        .into_iter()
        .find(|c| c.parent == path)
        .unwrap()
        .part;
    let targets = [
        Target {
            owner: mesh,
            part: Part::Face(key),
        },
        Target {
            owner: mesh,
            part: Part::MeshEdge(edge.0, edge.1),
        },
        Target {
            owner: line,
            part: Part::Curve,
        },
        Target {
            owner: curve,
            part: Part::Curve,
        },
        Target {
            owner: path,
            part: Part::PathEdge(0),
        },
        Target {
            owner: path,
            part: control,
        },
        Target {
            owner: cloud,
            part: Part::CloudPoint(u32::MAX),
        },
    ];
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
    gpu.cloud_camera(&camera);
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    let uploads = gpu.stats()[0];
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    let mut pixels = Vec::new();
    for target in targets {
        assert!(target.valid(&scene));
        gpu.set_highlight(&scene, Some(target))
            .unwrap();
        let selected = crate::cap_gpu_tests::pixels_at(&gpu, transform);
        assert!(
            selected != baseline,
            "A valid original highlight must change actual pixels: {target:?}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        pixels.push(selected);
    }
    gpu.set_highlight(&scene, None).unwrap();
    gpu.select_objects(&scene, &[line, path, point])
        .unwrap();
    let objects = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    assert_ne!(objects, baseline);
    gpu.select_objects(&scene, &[]).unwrap();
    gpu.set_highlight(&scene, None).unwrap();
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, transform),
        baseline
    );
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    for id in [mesh, line, path, point, cloud, curve] {
        far.translate(id, anchor).unwrap();
    }
    gpu.set_scene(&far, None);
    gpu.space.update(anchor, 3., 0.).unwrap();
    for row in &mut gpu.meshes {
        row.rebase(&gpu.queue, anchor);
    }
    gpu.instances
        .set(
            &gpu.device,
            &gpu.queue,
            &mut gpu.meshes,
            &gpu.instance_rows,
            false,
        )
        .unwrap();
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    gpu.sources = crate::pick_sources::Sources::anchored(&far, anchor).unwrap();
    gpu.mesh_ink
        .set(
            &gpu.device,
            gpu.sources.mesh_ink.iter().copied(),
        )
        .unwrap();
    gpu.strokes
        .set(
            &gpu.device,
            scene.lines().iter().map(|r| r.stroke()),
        )
        .unwrap();
    gpu.paths
        .set_chains(
            &gpu.device,
            scene
                .paths()
                .iter()
                .flat_map(|r| r.segments()),
        )
        .unwrap();
    gpu.markers
        .set_points(
            &gpu.device,
            scene
                .points()
                .iter()
                .map(|r| r.marker())
                .chain(
                    scene
                        .control_markers()
                        .into_iter()
                        .map(|r| r.display),
                ),
        )
        .unwrap();
    gpu.strokes
        .set_ids(
            &gpu.device,
            gpu.sources.line_ids.iter().copied(),
        )
        .unwrap();
    gpu.paths
        .set_ids(
            &gpu.device,
            gpu.sources.path_ids.iter().copied(),
        )
        .unwrap();
    gpu.markers
        .set_ids(
            &gpu.device,
            gpu.sources.marker_ids.iter().copied(),
        )
        .unwrap();
    gpu.set_clouds(&far, &[]).unwrap();
    camera.target = anchor;
    gpu.cloud_camera(&camera);
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
    assert_eq!(local, transform);
    gpu.mesh_ink
        .set(
            &gpu.device,
            crate::pick_sources::Sources::anchored(&far, anchor)
                .unwrap()
                .mesh_ink,
        )
        .unwrap();
    for (target, expected) in targets.into_iter().zip(pixels) {
        assert!(target.valid(&far));
        gpu.set_highlight(&far, Some(target))
            .unwrap();
        let actual = crate::cap_gpu_tests::pixels_at(&gpu, local);
        assert!(
            actual == expected,
            "Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",
            actual
                .iter()
                .zip(&expected)
                .filter(|(a, b)| a != b)
                .count()
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
    }
    gpu.set_highlight(&far, None).unwrap();
    gpu.select_objects(&far, &[line, path, point])
        .unwrap();
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        objects
    );
    gpu.select_objects(&far, &[]).unwrap();
    gpu.set_highlight(&far, None).unwrap();
    assert_eq!(
        crate::cap_gpu_tests::pixels_at(&gpu, local),
        baseline
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        far.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzzzzi-share-one-camera-and-measurement-unit"></a>11. Verify the shared unit through history

### 1. `src/measure_tests.rs`

Supply one unit/measurement/camera-inverse/original-source and exact Undo/Redo fixture while retaining earlier measurement assertions.

**Locate this existing block:**

```rust
    assert_eq!(labels[1].text, "■");
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(labels[1].text, "■");
}

#[test]
fn one_camera_unit_drives_measurements_without_editing_originals_or_history() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w3,4,0").unwrap();
    let owner = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    let original = source.to_proto();
    e.select_objects([owner], false);
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    e.camera.unit = crate::camera::Unit::Meters;
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 m");
    assert_eq!(e.camera.unit, Unit::Meters);
    assert_eq!(e.camera.unit.suffix(2), "m²");
    assert_eq!(e.camera.unit.to_meters(), 1.);
    assert!(
        e.camera
            .view_projection()
            .inverse()
            .is_some()
    );
    assert!(e.camera.ray([0., 0.]).is_some());
    assert!(Rc::ptr_eq(
        &e.scene.lines()[0].prepared.source,
        &source
    ));
    assert_eq!(source.to_proto(), original);
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.lines().is_empty());
    assert_eq!(e.camera.unit, Unit::Meters);
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.lines()[0].id, owner);
    assert_eq!(
        e.scene.lines()[0].prepared.source.to_proto(),
        original
    );
    e.select_objects([owner], false);
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 m");
}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.1
npm --prefix ../session_tests run course -- save 21.1
```

Continue with [21.2 · Precise CPU queries and origin-stamped picks](../../consolidation/96zzzzzj-project-cpu-queries-around-the-precise-target.md).
