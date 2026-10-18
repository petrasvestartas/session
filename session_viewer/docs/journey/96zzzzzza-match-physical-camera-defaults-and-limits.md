# 21.5.1 · Match physical camera defaults and limits

**Chapter 21 · Precise camera and browser coordinates · Section 21.5 · Physical camera, grid and navigation** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

Keep camera defaults and numeric limits in physical units.

## Type

Continue from [Use reversed depth across the frame](96zzzzzz-use-reversed-depth-across-the-frame.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Reuse the existing isometric recipe and convert the three-metre default once. Bound physical camera travel independently of fitted document size. Retain the current physical minimum distance when fitting tiny originals. Keep the orthographic minimum in metres.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6*scale);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
        };
```

</details>

Replace it with:

```rust
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
        };
```

<details>
<summary>Locate this block</summary>

```rust

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.distance = (self.distance / factor as f64).clamp(self.radius * 0.2, self.radius * 50.0);
        }
    }
```

</details>

Replace it with:

```rust

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            let scale=self.unit.to_meters();
            self.distance=(self.distance/factor as f64).clamp(1.0e-6/scale,1.0e15/scale);
        }
    }
```

<details>
<summary>Locate this block</summary>

```rust
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
```

</details>

Replace it with:

```rust
        }
        self.target = centre;
        self.radius = radius;
        self.distance=(distance*1.05).max(1.0e-6/self.unit.to_meters());
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
```

<details>
<summary>Locate this block</summary>

```rust

impl Default for Camera {
    fn default() -> Self {
        Self {
            unit:Unit::Millimeters,
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
            orientation: Quaternion::from_axis_angle(Vector::x_axis(), -std::f64::consts::FRAC_PI_2),
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
            grid: None,grid_plane:None,
            frame:[0.,0.,1.,1.],
        }
    }
}
```

</details>

Replace it with:

```rust

impl Default for Camera {
    fn default() -> Self {
        let unit=Unit::Millimeters;
        let mut camera=Self {
            unit,
            target:[0.;3],distance:3./unit.to_meters(),radius:0.,
            orientation:Quaternion::identity(),aspect:640./480.,
            projection:Projection::Perspective,grid:None,grid_plane:None,
            frame:[0.,0.,1.,1.],
        };
        camera.isometric();camera
    }
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Unit {Millimeters,Meters}
impl Unit {pub fn to_meters(self)->f64 {match self {Self::Millimeters=>0.001,Self::Meters=>1.}}
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}


pub const NEAR_FRACTION:f64=1.0e-4;

const HALF_FOV: f64 = std::f64::consts::FRAC_PI_6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    Perspective,
    Orthographic,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub max_distance: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum View {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Iso,
    IsoBack,
}


pub struct Camera {
    pub unit:Unit,
    pub target: [f64; 3],
    pub distance: f64,
    pub radius: f64,
    pub orientation: Quaternion,
    pub aspect: f64,
    pub projection: Projection,
    pub grid: Option<crate::grid::Settings>,
    pub grid_plane:Option<crate::cplane::CPlane>,
    pub frame:[f64;4],
}

impl Default for Camera {
    fn default() -> Self {
        let unit=Unit::Millimeters;
        let mut camera=Self {
            unit,
            target:[0.;3],distance:3./unit.to_meters(),radius:0.,
            orientation:Quaternion::identity(),aspect:640./480.,
            projection:Projection::Perspective,grid:None,grid_plane:None,
            frame:[0.,0.,1.,1.],
        };
        camera.isometric();camera
    }
}

impl Camera {
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds.min.iter().chain(&bounds.max).any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
        }
        let centre = bounds.centre();
        let radius = bounds.radius();
        if !radius.is_finite() || centre.iter().any(|v| !v.is_finite()) {
            return;
        }
        if radius == 0.0 {
            self.target = centre;
            return;
        }
        let tx = self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0]);
        let ty = HALF_FOV.tan() * (self.frame[3] - self.frame[1]);
        if !tx.is_finite() || tx <= 0.0 || !ty.is_finite() || ty <= 0.0 {
            return;
        }
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let forward = self.orientation.rotate_vector(Vector::y_axis());
        let mut distance: f64 = 0.0;
        for corner in 0..8 {
            let p: [f64; 3] = std::array::from_fn(|i| {
                let coordinate = if corner & (1 << i) == 0 { bounds.min[i] } else { bounds.max[i] };
                coordinate - centre[i]
            });
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance=(distance*1.05).max(1.0e-6/self.unit.to_meters());
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += dx as f64 * right[i] + dy as f64 * up[i];
        }
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            let scale=self.unit.to_meters();
            self.distance=(self.distance/factor as f64).clamp(1.0e-6/scale,1.0e15/scale);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.orbit(radians as f64, 0.0);
    }

    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let turn = Quaternion::from_axis_angle(Vector::z_axis(), yaw);
        let tilt = Quaternion::from_axis_angle(right, pitch);
        self.orientation = (turn * (tilt * self.orientation.duplicate())).normalized();
    }

    pub fn set_view(&mut self, view: View) {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};
        let z = Vector::z_axis();
        let x = Vector::x_axis();
        self.orientation = match view {
            View::Front => Quaternion::from_axis_angle(z, 0.0),
            View::Back => Quaternion::from_axis_angle(z, PI),
            View::Right => Quaternion::from_axis_angle(z, FRAC_PI_2),
            View::Left => Quaternion::from_axis_angle(z, -FRAC_PI_2),
            View::Top => Quaternion::from_axis_angle(x, -FRAC_PI_2),
            View::Bottom => Quaternion::from_axis_angle(x, FRAC_PI_2),
            View::Iso => {
                let yaw_q = Quaternion::from_axis_angle(z, -FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
            View::IsoBack => {
                // the isometric from the opposite corner: turned half round
                let yaw_q = Quaternion::from_axis_angle(z, PI - FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        self.projection = Projection::Orthographic;
    }

    pub fn isometric(&mut self) {
        let turn = Quaternion::from_axis_angle(Vector::z_axis(), -std::f64::consts::FRAC_PI_6);
        let right = turn.rotate_vector(Vector::x_axis());
        let tilt = Quaternion::from_axis_angle(right, -std::f64::consts::FRAC_PI_6);
        self.orientation = (tilt * turn).normalized();
    }

    pub fn view_projection(&self)->Xform {self.view_projection_anchored([0.;3])}

    pub fn view_projection_anchored(&self,anchor:[f64;3])->Xform {
        let scale=self.unit.to_meters();
        let local: [f64;3]=std::array::from_fn(|i|(self.target[i]-anchor[i])*scale);
        let distance=self.distance*scale;
        let forward = self.orientation.rotate_vector(Vector::y_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let target = Point::new(local[0], local[1], local[2]);
        let eye = Point::new(
            local[0] - forward[0] * distance,
            local[1] - forward[1] * distance,
            local[2] - forward[2] * distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = distance * NEAR_FRACTION;
        let reach = self.grid.and_then(|grid| grid.reach_on(self.grid_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.orientation.rotate_vector(Vector::y_axis()))),self.target).ok()).map_or(self.radius, |reach| self.radius.max(reach));
        let reach=reach*scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near),
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
        };
        let shift=self.shift();
        let mut matrix=Xform::translation(shift[0],shift[1],0.) * projection * view * Xform::scale_xyz(scale,scale,scale);
        if self.projection==Projection::Perspective {for value in &mut matrix.m {*value/=scale;}}
        matrix
    }

    pub fn shift(&self)->[f64;2] { [self.frame[0]+self.frame[2]-1.,1.-self.frame[1]-self.frame[3]] }

    pub fn frame(&mut self,frame:[f64;4])->bool {
        if frame.iter().any(|v|!v.is_finite()||*v<0.||*v>1.)||frame[0]>=frame[2]||frame[1]>=frame[3] {return false;}
        if self.frame==frame {return false;}self.frame=frame;true
    }

    pub fn pan_screen(&mut self,delta:[f64;2],size:[f64;2]) {
        if delta.iter().any(|v|!v.is_finite())||size.iter().any(|v|!v.is_finite()||*v<=0.) {return;}
        let scale=self.distance*0.0015;
        let right=self.orientation.rotate_vector(Vector::x_axis());let up=self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {self.target[i]+=scale*(-delta[0]*right[i]+delta[1]*up[i]);}
    }

    pub fn zoom_at(&mut self,factor:f32,screen:[f32;2]) {
        if screen.iter().any(|v|!v.is_finite()||v.abs()>1.) {return;}
        let old=self.distance;self.zoom(factor);let k=self.distance/old;let shift=self.shift();
        let h=old*HALF_FOV.tan();let right=self.orientation.rotate_vector(Vector::x_axis());let up=self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {self.target[i]+=(1.-k)*h*(right[i]*(screen[0] as f64-shift[0])*self.aspect+up[i]*(screen[1] as f64-shift[1]));}
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen.iter().any(|value| !value.is_finite() || value.abs() > 1.0) {
            return None;
        }
        self.ray_unbounded(screen)
    }

    pub fn ray_unbounded(&self,screen:[f32;2])->Option<Ray> {
        if screen.iter().any(|v|!v.is_finite()) {return None;}
        let inverse = self.view_projection_anchored(self.target).inverse()?;
        let origin = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, crate::depth::NEAR));
        let end = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, crate::depth::FAR));
        let delta = &end - &origin;
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        let origin=Point::new(origin[0]+self.target[0],origin[1]+self.target[1],origin[2]+self.target[2]);
        Some(Ray { origin, direction: delta.normalized(), max_distance })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_target_stays_centred_after_pan_zoom_and_turn() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        camera.rotate(0.7);
        camera.orbit(0.3, 0.2);
        let target = Point::new(camera.target[0], camera.target[1], camera.target[2]);
        let screen = camera.view_projection().transform_point(&target);
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }

    #[test]
    fn repeated_orbit_keeps_the_camera_axes_orthonormal() {
        let mut camera = Camera::default();
        for _ in 0..1_000 {
            camera.orbit(0.01, 0.02);
        }
        let right = camera.orientation.rotate_vector(Vector::x_axis());
        let forward = camera.orientation.rotate_vector(Vector::y_axis());
        let up = camera.orientation.rotate_vector(Vector::z_axis());
        for axis in [&right, &forward, &up] {
            assert!((axis.magnitude() - 1.0).abs() < 1.0e-10);
        }
        assert!(right.dot(&forward).abs() < 1.0e-10);
        assert!(right.dot(&up).abs() < 1.0e-10);
        assert!(forward.dot(&up).abs() < 1.0e-10);
    }

    #[test]
    fn a_projected_point_lies_on_its_unprojected_ray() {
        let camera = Camera::default();
        let world = Point::new(0.4, 0.2, 0.75);
        let screen = camera.view_projection().transform_point(&world);
        let ray = camera.ray([screen[0] as f32, screen[1] as f32]).unwrap();
        let offset = &world - &ray.origin;
        assert!(offset.cross(&ray.direction).magnitude() < 1.0e-6);
        assert!(offset.dot(&ray.direction) > 0.0);
        assert!(offset.magnitude() < ray.max_distance);
        assert!(camera.ray([f32::NAN, 0.0]).is_none());
    }
}

#[cfg(test)]
mod near_range_tests{
 use super::*;
 #[test]
 fn large_scene_radius_does_not_cut_geometry_just_ahead_of_the_eye(){
  for distance in [0.001,1.,1000.]{let mut camera=Camera::default();camera.distance=distance;camera.radius=distance*100000.;
   let forward=camera.orientation.rotate_vector(Vector::y_axis());let point=|gap:f64|Point::new(camera.target[0]+forward[0]*(gap-distance),camera.target[1]+forward[1]*(gap-distance),camera.target[2]+forward[2]*(gap-distance));
   let matrix=camera.view_projection();let inside=matrix.transform_point(&point(distance*NEAR_FRACTION*2.));let cut=matrix.transform_point(&point(distance*NEAR_FRACTION*0.5));
   assert!((0.0..1.0).contains(&inside[2]),"near geometry remains in view: {inside:?}");assert!(cut[2]>1.,"the near plane still cuts ahead of the eye");
  }
 }
}

#[cfg(test)]
mod orthographic_range_tests{
 use super::*;
 #[test]
 fn orthographic_depth_retains_four_millimetres_at_a_distant_camera(){
  let mut camera=Camera::default();camera.projection=Projection::Orthographic;camera.distance=1_000_000.;camera.radius=1000.;
  let forward=camera.orientation.rotate_vector(Vector::y_axis());let center=Point::new(camera.target[0],camera.target[1],camera.target[2]);let behind=&center+&(forward*4.);
  let matrix=camera.view_projection();let a=matrix.transform_point(&center);let b=matrix.transform_point(&behind);
  assert!((0.0..1.0).contains(&a[2])&&(0.0..1.0).contains(&b[2]));let front=a[2]as f32;let rear=b[2]as f32;
  assert!((front-rear).abs()>rear.abs()*1.9073486e-6,"four millimetres exceed the ink visibility tolerance");
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-01.rs"
```

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-02.rs"
```

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-03.rs"
```

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/camera_reference_tests.rs`

Supply the exact maintained-camera source and its zoom helper as a native-only independent reference; it is not part of learner browser implementation.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-05.rs"
```

</details>

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/camera_reference_fixture.json`

Supply the dated source and reference hashes for independent camera validation.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-06.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/camera_defaults_tests.rs`

Supply independent default/rotation, reference-integrity, zoom-bound and tiny-fit checks in both units.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/lib.rs`

Register optional native camera comparisons; browser builds omit both modules.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)]
mod diagnostic_shape_tests;
pub mod load_measure;
#[cfg(test)]
mod load_measure_tests;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod diagnostic;
pub mod adapter_info;
pub mod load_phase;
#[cfg(test)]
mod load_phase_tests;
#[cfg(test)]
mod adapter_info_tests;
pub mod suspension;
pub mod startup;
pub mod error_message;
#[cfg(test)]
mod error_message_tests;
#[cfg(test)]
mod startup_tests;
#[cfg(test)]
mod suspension_tests;
#[cfg(test)]
mod close_report_tests;
#[cfg(test)]
mod heartbeat_tests;
#[cfg(test)]
mod diagnostic_tests;
pub mod gpu_fault;
#[cfg(test)]
mod gpu_fault_tests;
pub mod background;
pub mod camera;
mod render_space;
#[cfg(test)] mod scene_origin_tests;
#[cfg(test)] mod camera_space_tests;
pub mod bounds;
pub mod memory;
pub mod stroke_gpu;
pub mod stroke;
pub mod chain;
pub mod marker;
pub mod controls;
#[cfg(test)]
mod control_parent_tests;
pub mod curve;
#[cfg(test)]
mod curve_input_tests;
#[cfg(test)]
mod curve_source_tests;
#[cfg(test)]
mod curve_sample_tests;
#[cfg(test)]
mod curve_layout_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod point_input_tests;
#[cfg(test)]
mod point_document_tests;
#[cfg(test)]
mod marker_tests;
#[cfg(test)]
mod path_input_tests;
#[cfg(test)]
mod path_placement_tests;
#[cfg(test)]
mod path_document_tests;
#[cfg(test)]
mod chain_link_tests;
#[cfg(test)]
mod chain_tests;
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
pub mod mesh;
pub mod normals;
pub mod normal_example;
pub mod grid;
#[cfg(test)]
mod grid_input_tests;
#[cfg(test)]
mod grid_source_tests;
#[cfg(test)]
mod normal_input_tests;

#[cfg(test)]
mod normal_view_tests;

#[cfg(test)]
mod normal_gpu_tests;
#[cfg(test)]
mod normal_matrix_tests;
#[cfg(test)]
mod normal_source_tests;
#[cfg(test)]
mod index_width_tests;
pub mod colour;
#[cfg(test)]
mod colour_input_tests;
#[cfg(test)]
mod colour_tests;
pub mod prepared;
pub mod row_metadata;
pub mod origin;
pub mod reload_url;
pub mod rehydrate;
pub mod reload_job;
pub mod edit_intent;
pub mod edit_replay;
pub mod reload_reply;
#[cfg(test)]
mod reload_reply_tests;
#[cfg(test)]
mod reload_complete_tests;
#[cfg(test)]
mod reload_precision_tests;
#[cfg(test)]
mod reload_failure_tests;
#[cfg(test)]
mod edit_owner_tests;
#[cfg(test)]
mod edit_scope_tests;
#[cfg(test)]
mod edit_move_tests;
#[cfg(test)]
mod edit_delete_tests;
mod edit_source;
#[cfg(test)]
mod origin_tests;
#[cfg(test)]
mod unload_tests;
#[cfg(test)]
mod rehydrate_tests;
pub mod placement;
#[cfg(test)]
mod line_scene_tests;
pub mod scene;
#[cfg(test)]
mod source_visibility_tests;
pub mod picking;
pub mod history;
pub mod recovery;
#[cfg(test)]
mod recovery_tests;
pub mod drawing_document;
pub mod editor;
pub mod offset;
pub mod viewport;
pub mod gesture;
pub mod shortcuts;
#[cfg(test)]
mod shortcuts_tests;

pub mod navigation;

#[cfg(test)]
mod document_phase_tests;
pub mod revision;
pub mod document;
pub mod read_gate;
pub mod specimen;
#[cfg(test)]
mod document_tests;
#[cfg(test)]
mod fit_tests;
#[cfg(test)]
mod move_tests;
#[cfg(test)]
mod prepared_tests;
#[cfg(test)]
mod source_tests;
#[cfg(test)]
mod file_identity_tests;
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod placement_load_tests;
#[cfg(test)]
mod save_roundtrip_tests;
#[cfg(test)]
mod replace_tests;
#[cfg(test)]
mod imported_source_tests;
#[cfg(test)]
mod placement_tests;
#[cfg(test)]
mod selected_fit_tests;
#[cfg(test)]
mod projection_tests;
#[cfg(test)]
mod navigation_tests;
#[cfg(test)]
mod gesture_tests;
pub mod geometry_cache;
pub mod gpu_geometry;
pub mod packed_vertex;
pub mod arena_spans;
mod arena_rows;
pub mod mesh_arena;
pub mod object_record;
#[cfg(test)]
mod geometry_storage_tests;
#[cfg(test)]
mod object_record_tests;
#[cfg(test)]
mod mesh_arena_tests;
#[cfg(test)]
mod arena_rows_tests;
pub mod browser_defaults;
#[cfg(test)]
mod browser_defaults_tests;
#[cfg(test)]
mod display_rgba_tests;
#[cfg(test)]
mod packed_vertex_tests;
mod primitive_image;
pub mod gpu_mesh;
pub mod renderer;
pub mod depth;
#[cfg(target_arch = "wasm32")]
mod browser_reload;
#[cfg(target_arch = "wasm32")]
mod browser_network;
#[cfg(target_arch = "wasm32")]
mod source_fetch;
#[cfg(target_arch = "wasm32")]
mod browser_recovery;
#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
mod file_input;
#[cfg(target_arch = "wasm32")]
mod file_output;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = browser::run().await {
            browser_report::fatal(&format!("Cannot draw: {error:?}"));
            browser::report(&format!("Cannot draw: {error:?}"));
        }
    });
}

#[cfg(any(test, target_arch = "wasm32"))]
mod command_dock;

#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
mod tool_buttons_tests;
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;
pub mod feature_owner;
#[cfg(test)]
mod feature_owner_tests;
#[cfg(test)]
mod projected_fit_tests;
#[cfg(test)]
mod instance_feature_tests;
#[cfg(test)]
mod copy_feature_tests;
#[cfg(test)]
mod feature_pick_tests;
#[cfg(test)]
mod feature_style_tests;

#[cfg(target_arch = "wasm32")]
mod panel;
#[cfg(target_arch = "wasm32")]
mod listeners;
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
#[cfg(target_arch = "wasm32")]
mod browser_report;
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
#[cfg(target_arch = "wasm32")]
mod timer;
#[cfg(target_arch = "wasm32")]
mod browser_rebase;
#[cfg(target_arch = "wasm32")]
mod report_lifecycle;
#[cfg(target_arch = "wasm32")]
mod browser_startup;
#[cfg(target_arch = "wasm32")]
mod browser_errors;
#[cfg(target_arch = "wasm32")]
mod browser_adapter;
#[cfg(target_arch = "wasm32")]
mod browser_upload;
#[cfg(target_arch = "wasm32")]
mod browser_phase;

pub mod pick_window;

#[cfg(test)]
mod pick_window_tests;

pub mod gpu_layout;

pub mod pick_pipeline;

mod pick_image;

pub mod pick_gpu;

mod pick_encode;

#[cfg(target_arch = "wasm32")]
pub mod browser_pick;

#[cfg(test)]
mod pick_input_tests;

pub mod pick_gate;
pub mod pick_view;
#[cfg(test)]
mod pick_view_tests;

pub mod mesh_parts;

#[cfg(test)]
mod mesh_parts_tests;

pub mod sub_id;
pub mod pick_sources;

pub mod selection;

#[cfg(test)]
mod pick_sources_tests;

pub mod face_pick;

pub mod highlight;

#[cfg(test)]
mod selection_tests;

pub mod object_selection;

#[cfg(test)]
mod object_selection_tests;

#[cfg(test)]
mod multi_selection_tests;

pub mod commands;

#[cfg(test)]
mod commands_tests;

pub mod tool_coords;
#[cfg(test)]
mod tool_coords_tests;

pub mod tool;

pub mod tool_preview;

#[cfg(test)]
mod tool_tests;

#[cfg(test)]
mod selection_cycle_tests;

pub mod snap;

pub mod snap_math;

pub mod snap_sources;

pub mod snap_bins;

pub mod snap_index;

#[cfg(test)]
mod snap_tests;

pub mod edit_preview;

#[cfg(test)]
mod edit_preview_tests;

pub mod gizmo;

#[cfg(test)]
mod gizmo_tests;

pub mod widget_mesh;

pub mod widget_layout;

pub mod widget_pipelines;

pub mod widget;

#[cfg(test)]
mod widget_tests;

pub mod part_preview;

pub mod gizmo_edit;

pub mod number_box;

#[cfg(test)]
mod gizmo_edit_tests;
#[cfg(test)]
mod number_box_tests;

pub mod hierarchy;

pub mod tree_panel;

pub mod tree_specimen;

#[cfg(test)]
mod hierarchy_tests;

pub mod view_flags;

pub mod layer_properties;

pub mod layer_edit;

pub mod layer_palette;

#[cfg(test)]
mod view_flags_tests;

#[cfg(test)]
mod layer_edit_tests;

#[cfg(test)]
mod layer_ui_tests;

pub mod rect_select;

#[cfg(test)]
mod rect_select_tests;

pub mod canvas_drag;

pub mod touch;

#[cfg(test)]
mod navigation_complete_tests;

#[cfg(test)]
mod shortcut_focus_tests;

pub mod surface;

#[cfg(test)]
mod surface_tests;

pub mod cad_edges;

pub mod cad_boundary;

#[cfg(test)]
mod cad_boundary_tests;

pub mod cad_samples;

#[cfg(test)]
mod cad_trim_tests;

pub mod cad_seams;

#[cfg(test)]
mod cad_seam_tests;

pub mod cad_orient;

pub mod cad_face;

pub mod cad_display;

#[cfg(test)]
mod cad_display_tests;

pub mod text;

pub mod text_atlas;

pub mod text_example;

#[cfg(test)]
mod text_atlas_tests;

pub mod scene_label;

pub mod scene_label_screen;

pub mod text_gpu_support;
#[cfg(test)]
mod growing_buffer_tests;

pub mod scene_text_gpu;

pub mod text_fonts;

#[cfg(test)]
mod scene_label_tests;

pub mod outline_math;

pub mod outline_gpu;

#[cfg(test)]
mod outline_math_tests;

pub mod cloud;

pub mod cloud_lod;

#[cfg(test)]
mod cloud_tests;

pub mod cloud_normals;

pub mod cloud_gpu;

pub mod cloud_pick;

#[cfg(test)]
mod cloud_pick_tests;

pub mod manifest;

pub mod live_revision;

pub mod source_validate;

pub mod archive;

pub mod geometry_snapshot;

pub mod scene_archive;

pub mod archive_identity;

pub mod archive_document;

pub mod archive_presentation;

pub mod document_source;

pub mod document_geometry;

pub mod scene_document;

pub mod archive_restore_texts;

pub mod archive_restore_flags;

pub mod publication;

pub mod scene_manifest;

pub mod publication_route;

pub mod publication_index;

#[cfg(target_arch = "wasm32")]
pub mod publication_fetch;

#[cfg(target_arch = "wasm32")]
pub mod browser_publication;

pub mod stream;

pub mod range_gate;

pub mod range_rules;

pub mod range_page;

#[cfg(target_arch = "wasm32")]
pub mod range_fetch;

pub mod range_source;

pub mod page_cache;

pub mod stream_generation;

pub mod stream_budget;

pub mod stream_properties;

pub mod range_cloud;

pub mod range_example;

#[cfg(test)]
mod stream_row_tests;

pub mod source_encoding;

#[cfg(target_arch = "wasm32")]
pub mod source_unpack;

#[cfg(target_arch = "wasm32")]
pub mod stream_load;

pub mod range_shape;

pub mod instance_plan;
mod instance_features;

pub mod instance_definition;

pub mod instance_snapshot;

pub mod instance_specimen;

#[cfg(test)]
mod instance_scene_tests;

pub mod instance_members;
mod instance_gpu;

pub mod cad_shape_key;

pub mod cad_shape_cache;

pub mod cad_shape_example;

pub mod nested_placement;

pub mod nested_example;

#[cfg(test)]
mod nested_placement_tests;

pub mod sheet_entity;

pub mod sheet_example;

#[cfg(target_arch="wasm32")]
pub mod sheet_entity_fetch;

pub mod sheet_page;

pub mod sheet_stream;

pub mod sheet_source;

pub mod sheet_range;

pub mod sheet_lookup;

pub mod sheet_pipeline;

pub mod sheet_gpu;

#[cfg(target_arch="wasm32")]
pub mod browser_sheet;

#[cfg(test)]
mod sheet_scene_tests;

pub mod cplane;

pub mod drawing_coords;

pub mod drawing_input;

#[cfg(test)]
mod drawing_tests;

pub mod planar;

#[cfg(test)]
mod planar_tests;

pub mod primitive_options;

pub mod primitive_shape;

pub mod primitive_box;

pub mod primitive_pyramid;

pub mod primitive_block_with_hole;

pub mod primitive_tetrahedron;

pub mod primitive_octahedron;

pub mod primitive_icosahedron;

pub mod primitive_dodecahedron;

pub mod primitive_draft;

pub mod primitive_geometry;

pub mod primitive_registry;

#[cfg(test)]
mod primitive_tests;

pub mod round_cylinder;

pub mod round_cone;

pub mod round_sphere;

pub mod round_torus;

pub mod round_capsule;

pub mod round_quad_sphere;

#[cfg(test)]
mod round_tests;

pub mod freeform_circle;

pub mod freeform_ellipse;

pub mod freeform_arc;

pub mod freeform_parabola;

pub mod freeform_curve;

#[cfg(test)]
mod freeform_tests;

pub mod transform_scale;

pub mod transform_rotate;

pub mod source_controls;
#[cfg(test)]
mod original_control_tests;

pub mod source_deform;

pub mod source_topology_colour;

pub mod transform_draft;

pub mod source_part;

pub mod transform_commands;

#[cfg(test)]
mod transform_tests;

pub mod gather_input;

pub mod surfacing;

pub mod extrude_geometry;

pub mod revolve_geometry;

pub mod gather_draft;

pub mod gather_registry;

#[cfg(test)]
mod gather_tests;

pub mod loft_geometry;

pub mod nurbs_loft_geometry;

pub mod sweep1_geometry;

pub mod sweep2_geometry;

#[cfg(test)]
mod loft_tests;

pub mod network_geometry;

pub mod patch_geometry;

#[cfg(test)]
mod network_tests;

pub mod project_geometry;

pub mod source_cut;

pub mod extend_reach;

pub mod interval_geometry;

pub mod source_edit;

pub mod project_draft;

pub mod extend_draft;

pub mod editing_commands;

#[cfg(test)]
mod editing_tests;

pub mod split_geometry;

pub mod split_draft;

#[cfg(test)]
mod split_tests;

#[cfg(test)]
mod split_face_tests;

pub mod measure_math;

pub mod measure_length;

pub mod measure_area;

pub mod measure_volume;

pub mod bounds_original;

pub mod measure_mark;

pub mod measure_gpu;

#[cfg(test)]
mod measure_tests;

pub mod trim_parts;

pub mod explode_geometry;

pub mod text_geometry;

pub mod curve_heads;

pub mod selection_region;

pub mod source_shape;

pub mod plane_original;

pub mod selection_query;

pub mod explode_edit;

pub mod text_draft;

pub mod guide_gpu;

pub mod trim_draft;

pub mod lasso;

pub mod graph_edit;

pub mod group_plan;

pub mod command_request;

#[cfg(test)]
mod command_inventory_tests;

#[cfg(test)]
mod command_owner_tests;

#[cfg(test)]
mod command_fetch_tests;

#[cfg(test)]
mod lasso_geometry_tests;

pub mod tile_math;

pub mod browser_tiles;

pub mod tile_report;

pub mod tile_pipelines;

pub mod tile_gpu;

pub mod finite_sources;

pub mod finite_ink;

#[cfg(test)]
mod tile_shader_tests;

#[cfg(test)]
mod tile_gpu_tests;

#[cfg(test)]
mod finite_visibility_tests;

pub mod clipping_geometry;

pub mod clipping_rows;

pub mod clipping_uniform;

pub mod clipping_input;

pub mod clipping_tool;

pub mod clipping_gpu;

#[cfg(test)]
mod clipping_source_tests;

#[cfg(test)]
mod clipping_gpu_tests;

#[cfg(test)]
mod clipping_uniform_tests;

pub mod solid_orientation;

pub mod solid_cache;

pub mod cap_bounds;

pub mod cap_source_bounds;

pub mod cap_image;

pub mod cap_geometry;
mod cap_pipeline;

pub mod cap_pick_pipeline;

pub mod cap_pick_gpu;

pub mod caps_gpu;

#[cfg(test)]
mod cap_gpu_tests;

pub mod pyramid_size;

pub mod pyramid_image;

pub mod pyramid_pipeline;

pub mod pyramid_gpu;

pub mod ambient_radius;

pub mod ambient_camera;

pub mod ambient_rows;

pub mod ambient_source_pipeline;

pub mod ambient_source_gpu;

pub mod ambient_image;

pub mod ambient_images;

pub mod ambient_pipeline;

pub mod ambient_groups;

pub mod ambient_pass;

pub mod ambient_gpu;

#[cfg(test)]
mod arctic_tests;

pub mod outline_caps;

pub mod outline_composite;

#[cfg(test)]
mod global_outline_command_tests;

pub mod element_values;

pub mod element_beam;

pub mod element_column;

pub mod element_plate;

pub mod element_support;

pub mod element_beam_variable;

pub mod element_block;

pub mod element_original;

pub mod element_input;

pub mod element_draft;

pub mod element_adopt;

#[cfg(test)]
mod element_tests;

#[cfg(test)]
mod opacity_command_tests;

pub mod comparison;

#[cfg(not(target_arch="wasm32"))]
pub mod reference_frame;

#[cfg(test)]
mod comparison_tests;

pub mod bench_config;

pub mod bench_samples;

pub mod bench_compare;

pub mod tree_witness;

#[cfg(test)]
mod bench_tests;

pub mod source_metadata;

pub mod definition_merge;

pub mod registry_audit;

pub mod source_audit;

#[cfg(test)]
mod metadata_tests;

pub mod feature_plan;

pub mod feature_display;

pub mod interaction_example;

#[cfg(test)]
mod interaction_bridge_tests;

#[cfg(test)]
mod inventory_tests;
#[cfg(test)]
mod ambient_shader_contract;
pub mod projection_lookup;
mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
mod object_rows;
mod primitive_slot_table;
mod slot_space;
#[cfg(test)] mod draw_slots_tests;
#[cfg(test)] mod primitive_slot_table_tests;

#[cfg(test)] mod source_phase_tests;

#[cfg(test)]
mod current_command_tests;

#[cfg(test)]
mod full_source_action_tests;

#[cfg(test)]
mod camera_reverse_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-08.rs" skip=16111
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/camera_space_tests.rs`

Declare the existing Top pose for exact rectangle crossing/window sets; retain distant source, pixel and ownership assertions.

<details>
<summary>Locate the existing block</summary>

```text
use crate::camera::{Camera,Projection,Unit};
use session_rust::Point;
#[test]
fn equivalent_millimetre_and_metre_scenes_share_screen_and_ray_geometry(){
 for unit in [Unit::Millimeters,Unit::Meters]{for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.unit=unit;camera.projection=projection;assert!(camera.view_projection().inverse().is_some());assert!(camera.ray([0.2,-0.1]).is_some());}}
 for projection in [Projection::Perspective,Projection::Orthographic]{
  let mut mm=Camera::default();mm.projection=projection;mm.target=[2e6,-3e6,4e6];mm.distance=5000.;mm.radius=1000.;
  let mut m=Camera::default();m.projection=projection;m.unit=Unit::Meters;m.target=mm.target.map(|x|x*0.001);m.distance=5.;m.radius=1.;
  let a=mm.view_projection_anchored(mm.target);let b=m.view_projection_anchored(m.target);
  let pa=a.transform_point(&Point::new(500.,250.,100.));let pb=b.transform_point(&Point::new(0.5,0.25,0.1));for i in 0..3{assert!((pa[i]-pb[i]).abs()<1e-12);}
  let ra=mm.ray([0.2,-0.1]).unwrap();let rb=m.ray([0.2,-0.1]).unwrap();for i in 0..3{assert!((ra.origin[i]*0.001-rb.origin[i]).abs()<1e-6);assert!((ra.direction[i]-rb.direction[i]).abs()<1e-6);}assert!((ra.max_distance*0.001-rb.max_distance).abs()<1e-5);
 }
}
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin(){
 let mut camera=Camera::default();camera.target=[1e12,-1e12,1e12];camera.distance=1.;camera.radius=1.;
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let local=camera.view_projection_anchored(camera.target).to_f32();let matrix=session_rust::Xform::from_matrix(local.map(f64::from));let a=matrix.transform_point(&Point::new(0.,0.,0.));let b=matrix.transform_point(&Point::new(0.000125,0.,0.));assert!(b[0]-a[0]>0.0001);assert!((0.0..1.0).contains(&a[2]));}
 for projection in [Projection::Perspective,Projection::Orthographic]{camera.projection=projection;let mut local=Camera::default();local.projection=projection;local.distance=1.;local.radius=1.;for screen in [[0.,0.],[0.2,-0.1]]{let a=camera.ray(screen).unwrap();let b=local.ray(screen).unwrap();for i in 0..3{assert!((a.origin[i]-camera.target[i]-b.origin[i]).abs()<0.00025);assert!((a.direction[i]-b.direction[i]).abs()<1e-10);}assert!((a.max_distance-b.max_distance).abs()<1e-8);}}
 assert_eq!(camera.target,[1e12,-1e12,1e12]);
}

#[test]
fn cpu_screen_queries_keep_distant_pixels_and_original_clip_guards(){
 use crate::snap_math::Screen;let anchor=[1e12,-1e12,1e12];for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.projection=projection;camera.distance=2.;let baseline=Screen::camera(&camera,[640.,480.]);camera.target=anchor;let far=Screen::camera(&camera,[640.,480.]);assert_eq!(baseline.matrix,far.matrix);assert!(baseline!=far);for point in [[0.25,0.125,-0.5],[-0.125,-0.25,0.125],[0.,0.,0.]]{let world=std::array::from_fn(|i|anchor[i]+point[i]);assert_eq!(far.point(world),baseline.point(point));assert_eq!(far.origin,anchor);}}
 let mut view=Screen{matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:anchor};assert_eq!(view.point([anchor[0]+0.25,anchor[1]+0.125,anchor[2]+0.5]),Some([125.,43.75]));for point in [[anchor[0],anchor[1],anchor[2]-0.5],[anchor[0],anchor[1],anchor[2]+1.5],[f64::NAN,0.,0.]]{assert_eq!(view.point(point),None);}view.size[0]=0.;assert_eq!(view.point(anchor),None);view.size[0]=200.;view.origin[0]=f64::NAN;assert_eq!(view.point(anchor),None);view.origin=anchor;view.matrix[15]=0.;assert_eq!(view.point(anchor),None);
}

#[test]
fn text_query_polygons_keep_precise_placements_sources_and_unclipped_rules(){
 use crate::{snap_math::Screen,text::TextPlacement};let anchor=[1e12,-1e12,1e12];for projection in [Projection::Perspective,Projection::Orthographic]{let mut camera=Camera::default();camera.projection=projection;camera.distance=3.;let view=Screen::camera(&camera,[800.,600.]);camera.target=anchor;let far_view=Screen::camera(&camera,[800.,600.]);for placement in [TextPlacement::WorldPlane{world:[0.125,0.25,-0.5],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.25},TextPlacement::WorldBillboard{world:[0.125,0.25,-0.5],world_height:0.25},TextPlacement::Anchor{world:[0.125,0.25,-0.5],offset:[3.,5.]},TextPlacement::Nameplate{world:[0.125,0.25,-0.5],padding:[4.,6.],rounded:true}]{let mut scene=crate::scene::Scene::empty();let source=std::rc::Rc::new(crate::scene_label::example(placement));let id=scene.insert_text(crate::scene_label::Prepared::new(source.clone()).unwrap()).unwrap();let baseline=crate::scene_label_screen::polygon_in(&scene.texts()[0],&view).unwrap();let original=serde_json::to_vec(source.as_ref()).unwrap();scene.translate(id,anchor).unwrap();assert_eq!(crate::scene_label_screen::polygon_in(&scene.texts()[0],&far_view).unwrap(),baseline);assert!(std::rc::Rc::ptr_eq(&scene.texts()[0].prepared.source,&source));assert_eq!(serde_json::to_vec(source.as_ref()).unwrap(),original);let bounds=[baseline.iter().map(|p|p[0]).fold(f64::INFINITY,f64::min)-1.,baseline.iter().map(|p|p[1]).fold(f64::INFINITY,f64::min)-1.,baseline.iter().map(|p|p[0]).fold(f64::NEG_INFINITY,f64::max)+1.,baseline.iter().map(|p|p[1]).fold(f64::NEG_INFINITY,f64::max)+1.];let rectangle=crate::rect_select::Rectangle{bounds,crossing:false};assert!(crate::scene_label_screen::matches_in(&scene.texts()[0],&far_view,rectangle));}}
 let mut view=Screen{matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:[0.;3]};assert!(view.unclipped([0.,0.,-1.]).is_some());assert!(view.point([0.,0.,-1.]).is_none());view.matrix[2]=f64::NAN;assert_eq!(view.unclipped([0.25,0.125,0.5]),Some([125.,43.75]));assert!(view.point([0.25,0.125,0.5]).is_none());view.matrix=session_rust::Xform::identity().m;view.matrix[15]=1e-10;assert!(view.unclipped([0.25,0.125,0.5]).is_some());assert!(view.point([0.25,0.125,0.5]).is_none());view.matrix[15]=0.;assert!(view.unclipped([0.;3]).is_none());view.matrix[15]=1.;view.size[0]=0.;assert!(view.unclipped([0.;3]).is_none());
}

#[test]
fn rectangle_queries_keep_distant_originals_and_exact_window_crossing_sets(){
 use crate::{snap_math::Screen,rect_select::{Rectangle,found_in},scene::Scene};use std::rc::Rc;let mut scene=Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.5,0.5,-0.5,0.5,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.5,-0.5,-0.5),session_rust::Point::new(0.,-0.25,-0.5),session_rust::Point::new(0.5,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]))).unwrap()).unwrap();let text_source=Rc::new(crate::scene_label::example(crate::text::TextPlacement::Anchor{world:[-0.5,0.75,-0.5],offset:[0.;2]}));let text=scene.insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.25,-0.5,0.75,0.25,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let point_source=scene.points()[0].prepared.source.clone();let mut camera=Camera::default();camera.projection=Projection::Orthographic;camera.distance=3.;let view=Screen::camera(&camera,[800.,600.]);let wide=Rectangle::new([0.,0.],[800.,600.]).unwrap();let all=found_in(&scene,&view,wide);for id in [mesh,line,path,point,cloud,text,drawing]{assert!(all.contains(&id));}let crossing=Rectangle::new([420.,310.],[380.,290.]).unwrap();let small=Rectangle::new([380.,290.],[420.,310.]).unwrap();let expected_crossing=found_in(&scene,&view,crossing);let expected_small=found_in(&scene,&view,small);assert!(expected_crossing.contains(&mesh));assert!(!expected_small.contains(&mesh));let mut far=scene.clone();let anchor=[1e12,-1e12,1e12];for id in [mesh,line,path,point,cloud,text,drawing]{far.translate(id,anchor).unwrap();}camera.target=anchor;let far_view=Screen::camera(&camera,[800.,600.]);assert_eq!(view.matrix,far_view.matrix);for(rectangle,expected)in[(wide,all),(crossing,expected_crossing),(small,expected_small)]{assert_eq!(found_in(&far,&far_view,rectangle),expected);}assert!(Rc::ptr_eq(&far.points()[0].prepared.source,&point_source));assert!(Rc::ptr_eq(&far.texts()[0].prepared.source,&text_source));assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));Rc::make_mut(&mut far.view).hide(&[line],true);assert!(!found_in(&far,&far_view,wide).contains(&line));Rc::make_mut(&mut far.view).lock(&[point],true);assert!(!found_in(&far,&far_view,wide).contains(&point));assert!(found_in(&Scene::empty(),&far_view,wide).is_empty());
}

#[test]
fn lasso_queries_keep_distant_original_sets_and_density(){
 use std::rc::Rc;let mut scene=crate::scene::Scene::empty();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.5,0.5,-0.5,0.5,0.5,-0.5))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.points()[0].prepared.source.clone();let anchor=[1e12,-1e12,1e12];let mut far=scene.clone();for id in [line,point,mesh]{far.translate(id,anchor).unwrap();}for density in [1.,2.]{let size=[800.,600.];let mut lasso=crate::lasso::Lasso::default();lasso.size=size;lasso.density=density;lasso.points=vec![(0.,0.),(800.*density,0.),(800.*density,600.*density),(0.,600.*density),(0.,0.)];let mut camera=Camera::default();camera.projection=Projection::Orthographic;camera.distance=3.;let baseline=lasso.found(&scene,&camera).unwrap();assert_eq!(baseline,vec![line,point,mesh]);camera.target=anchor;assert_eq!(lasso.found(&far,&camera).unwrap(),baseline);assert!(Rc::ptr_eq(&far.points()[0].prepared.source,&source));let mut locked=far.clone();Rc::make_mut(&mut locked.view).lock(&[point],true);assert_eq!(lasso.found(&locked,&camera).unwrap(),vec![line,mesh]);assert!(lasso.found(&crate::scene::Scene::empty(),&camera).is_err());}
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-09.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/drawing_tests.rs`

Keep the plane-click fixture at its original unit-sized Top pose so existing distant source points cannot become accidental near-cursor snaps.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{editor::{Action,Change,Editor},cplane::CPlane,tool::Shape};
use session_rust::Point;

fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,text:&str)->Result<Change,String> {
    let action=if let Some(action)=e.tool_line(text) {action.map_err(str::to_owned)?}else{
        match crate::commands::parse(text)? {crate::commands::Command::Editor(action)=>action,_=>return Err("Expected editor command".into())}
    };e.apply(action).map_err(str::to_owned)
}
fn tilted()->CPlane {CPlane::from_3_points(&Point::new(4.,5.,6.),&Point::new(5.,5.,7.),&Point::new(4.,6.,6.)).unwrap()}
fn near(a:[f64;3],b:[f64;3]) {for i in 0..3 {assert!((a[i]-b[i]).abs()<1e-12,"{a:?} {b:?}");}}

#[test]
fn finished_world_prefixed_drafts_do_not_apply_the_fixed_plane_twice() {
    let mut e=editor();let plane=tilted();e.apply(Action::ConstructionPlane(Some(plane))).unwrap();
    command(&mut e,"Line").unwrap();command(&mut e,"1,0,0").unwrap();
    let a=plane.world([1.,0.,0.]);let b=[9.123456789012345,8.,7.];
    let draft=e.draft.as_ref().unwrap();assert!(draft.command(&[[a[0],a[1],a[2]],b]).starts_with("Line w"));
    e.apply(Action::ToolPoint(b)).unwrap();let source=&e.scene.lines()[0].prepared.source;
    near([source.start()[0],source.start()[1],source.start()[2]],[a[0],a[1],a[2]]);
    near([source.end()[0],source.end()[1],source.end()[2]],b);
    let bytes=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Undo).unwrap();assert!(e.scene.lines().is_empty());e.apply(Action::Redo).unwrap();
    e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&bytes).unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0],b[0]);
}

#[test]
fn direct_and_interactive_coordinates_share_plane_relative_polar_and_world_rules() {
    let mut e=editor();command(&mut e,"CPlane XZ").unwrap();command(&mut e,"Line 1,2 @3,4").unwrap();
    let line=&e.scene.lines()[0].prepared.source;near([line.start()[0],line.start()[1],line.start()[2]],[1.,0.,2.]);near([line.end()[0],line.end()[1],line.end()[2]],[4.,0.,6.]);
    command(&mut e,"Line").unwrap();command(&mut e,"w1,2,3").unwrap();command(&mut e,"5<90").unwrap();
    let line=&e.scene.lines()[1].prepared.source;near([line.end()[0],line.end()[1],line.end()[2]],[1.,2.,8.]);
    command(&mut e,"CPlane View").unwrap();command(&mut e,"Point w1000000.001,0,0").unwrap();assert_eq!(e.scene.points()[0].prepared.source[0],1000000.001);
    assert!(e.construction_plane.is_none());
}

#[test]
fn typed_batches_validate_before_changing_a_draft_or_history() {
    let mut e=editor();command(&mut e,"Line").unwrap();
    for text in ["0,0,0 NaN,1,0","0,0,0 1,0,0 2,0,0","@1,0","5"] {assert!(command(&mut e,text).is_err(),"{text}");assert!(e.draft.as_ref().unwrap().points.is_empty());}
    command(&mut e,"0,0,0").unwrap();assert!(command(&mut e,"0,0,0").is_err());assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    e.apply(Action::ToolHover(Some([0.,4.,0.]))).unwrap();command(&mut e,"2").unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.end()[1],2.);command(&mut e,"Undo").unwrap();assert!(e.scene.lines().is_empty());
    command(&mut e,"Line NaN,0 1,0").unwrap_err();assert!(e.draft.is_none());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.lines().len(),1);
}

#[test]
fn an_open_polyline_commits_once_on_enter_and_preserves_redo_when_cancelled() {
    let mut e=editor();command(&mut e,"Polyline").unwrap();command(&mut e,"0,0").unwrap();
    assert!(command(&mut e,"").is_err());assert!(e.scene.paths().is_empty());
    command(&mut e,"@1,0 @0,1").unwrap();assert!(e.scene.paths().is_empty());
    for x in 0..30 {e.apply(Action::ToolHover(Some([x as f64/30.,1.,0.]))).unwrap();}
    command(&mut e,"").unwrap();assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),vec![[0.,0.,0.],[1.,0.,0.],[1.,1.,0.]]);
    command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Polyline").unwrap();command(&mut e,"0,0 1,1").unwrap();command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),1);
}

#[test]
fn polyline_limits_and_direct_commands_leave_original_geometry_atomic() {
    let mut e=editor();command(&mut e,"Polyline w0,0,0 w1,0,0 w1,1,0").unwrap();assert_eq!(e.scene.paths().len(),1);
    command(&mut e,"Polyline").unwrap();let points:Vec<_>=(0..crate::tool::MAX_POINTS).map(|i|[i as f64,0.,0.]).collect();
    e.draft.as_mut().unwrap().submit_many(&points).unwrap();assert!(e.apply(Action::ToolPoint([4096.,0.,0.])).is_err());assert_eq!(e.draft.as_ref().unwrap().points.len(),4096);
    command(&mut e,"Escape").unwrap();command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());
}

#[test]
fn construction_plane_chooser_and_three_points_do_not_create_geometry_or_undo_entries() {
    let mut e=editor();command(&mut e,"Point w0,0,0").unwrap();command(&mut e,"CPlane").unwrap();assert!(e.plane_menu);
    let buttons=crate::drawing_input::buttons();assert_eq!(buttons.len(),6);command(&mut e,"3 Point").unwrap();
    command(&mut e,"w4,5,6").unwrap();command(&mut e,"w5,5,7").unwrap();assert!(command(&mut e,"w6,5,8").is_err());
    command(&mut e,"w4,6,6").unwrap();assert_eq!(e.construction_plane,Some(tilted()));assert!(e.draft.is_none());assert_eq!(e.scene.points().len(),1);
    command(&mut e,"Undo").unwrap();assert!(e.scene.points().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.points().len(),1);
}

#[test]
fn a_plane_can_change_mid_draft_without_reinterpreting_accepted_world_points() {
    let mut e=editor();command(&mut e,"Line").unwrap();command(&mut e,"w2,3,4").unwrap();command(&mut e,"3 Point").unwrap();
    command(&mut e,"w4,5,6").unwrap();command(&mut e,"w5,5,7").unwrap();command(&mut e,"w4,6,6").unwrap();assert_eq!(e.draft.as_ref().unwrap().points,vec![[2.,3.,4.]]);
    command(&mut e,"1,0,0").unwrap();let expected=tilted().world([1.,0.,0.]);let line=&e.scene.lines()[0].prepared.source;
    near([line.start()[0],line.start()[1],line.start()[2]],[2.,3.,4.]);near([line.end()[0],line.end()[1],line.end()[2]],[expected[0],expected[1],expected[2]]);
}

#[test]
fn plane_clicks_grid_and_editor_coordinate_context_use_current_owners() {
    let mut a=editor();a.apply(Action::ConstructionPlane(Some(tilted()))).unwrap();
    let mut b=editor();command(&mut b,"Point 1,2,3").unwrap();assert_eq!(b.scene.points()[0].prepared.source[0],1.);
    command(&mut a,"Point 1,0,0").unwrap();let expected=tilted().world([1.,0.,0.]);assert_eq!(a.scene.points()[0].prepared.source[0],expected[0]);
    a.apply(Action::ConstructionPlane(Some(CPlane::Xy))).unwrap();a.apply(Action::BeginTool(Shape::Point)).unwrap();
    let hit=a.snap_point([0.,0.],[640.,480.]).unwrap();assert!(hit.point[2].abs()<1e-12);
    a.apply(Action::Close).unwrap();assert!(a.draft.is_none());
}

#[test]
fn closing_a_polyline_is_one_transaction_and_the_last_point_is_original() {
    let mut e=editor();e.apply(Action::ConstructionPlane(Some(tilted()))).unwrap();command(&mut e,"Polyline").unwrap();command(&mut e,"w0,0 w1,0").unwrap();assert!(command(&mut e,"Close").is_err());
    command(&mut e,"w1,1").unwrap();command(&mut e,"Close").unwrap();let points=e.scene.paths()[0].prepared.source.coordinates();assert_eq!(points.len(),4);assert_eq!(points[0],points[3]);
    command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths()[0].prepared.source.coordinates(),points);
}

#[test]
fn edge_on_planes_and_view_depth_keep_a_finite_click_point() {
    let camera=crate::camera::Camera::default();let point=crate::drawing_input::on_plane(&camera,[0.,0.],CPlane::Xz).unwrap();assert_eq!(point[1],0.);assert!(point.iter().all(|v|v.is_finite()));
    let point=crate::drawing_input::on_at(&camera,[0.,0.],CPlane::Xy,Some([0.,0.,0.5])).unwrap();assert!((point[2]-0.5).abs()<1e-12);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-10.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/editor.rs`

Retain exact Reset/aspect and navigation-independent Undo assertions, with default distances now expressed in metres.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    ElementBegin{kind:crate::element_input::Kind,text:String},ElementText(String),ElementPick(Option<ObjectId>),ElementAttributes(Option<bool>),ElementInteractions(Option<bool>),
    Opacity(f32),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
    SelectLasso,LassoPress{at:[f64;2],size:[f64;2],density:f64,pointer:i32},LassoMove{at:[f64;2],pointer:i32},LassoRelease{at:[f64;2],pointer:i32,add:bool,remove:bool},LassoCancel,
    AddGroup(String),AddEdge,
    StreamRefresh,
    Trim,TrimPick(Option<ObjectId>),TrimScreen{screen:[f32;2],size:[f64;2],commit:bool},CancelTrim,
    TextBegin(String),
    Explode,
    Query(crate::selection_query::Query),Arrowhead(session_rust::Arrowhead),Escape,
    NamedView(crate::camera::View),
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
    Project(Vec<String>),ExtendBegin{distance:Option<f64>,asking:bool},ExtendPick(Option<ObjectId>),Interval(crate::interval_geometry::Interval),
    GatherBegin{recipe:&'static crate::gather_input::Recipe,text:String},GatherPick(Option<ObjectId>),
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),RotateWorld{axis:crate::gizmo::Axis,degrees:f64},
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    EnableControls,
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
    pub element:Option<crate::element_draft::Draft>,
    pub element_attributes:bool,
    pub element_interactions:bool,
    pub opacity:f32,
    pub arctic:bool,
    pub outlines:bool,
    pub clipping_enabled:bool,pub clipping_solid:bool,
    pub interaction_epoch:u64,
    pub lasso:Option<crate::lasso::Lasso>,
    pub trim:Option<crate::trim_draft::Draft>,
    pub last_text_height:f64,
    pub mark:Option<crate::measure_mark::Mark>,
    pub split:Option<crate::split_draft::Draft>,
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
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
            element:None,element_attributes:false,element_interactions:true,opacity:1.,arctic:false,outlines:false,clipping_enabled:true,clipping_solid:true,interaction_epoch:0,lasso:None,trim:None,last_text_height:0.,mark:None,split:None,canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
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
        if let Some(scene)=self.trim.as_ref().and_then(|t|t.preview.as_ref()){return scene}
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.trim.is_some()||self.split.is_some()||self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if self.pick_mode==crate::pick_sources::Mode::Control && self.selected_part.is_none() {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::camera_scale(&self.camera,origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let scale=crate::widget_layout::camera_scale(&self.camera,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen::camera(&self.camera,size).point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        let rows=crate::graph_edit::expand(&self.scene,rows);
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found_in(&self.scene,&crate::snap_math::Screen::camera(&self.camera,drag.size),rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn drawing_plane(&self)->crate::cplane::CPlane{self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())))}
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.element.is_some(){return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape"){Action::CancelTool}else{Action::ElementText(text.into())}))}
        if self.draft.as_ref().is_some_and(|d|d.tool.name()=="Clipping Plane"){if let Some(mode)=crate::clipping_geometry::Mode::parse(text.trim()){return Some(Ok(Action::Clipping(crate::clipping_input::Request::Begin(mode))))}}
        if self.trim.is_some(){return match text.trim(){""=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Trim")=>Some(Ok(Action::Trim)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelTrim)),_=>None}}
        if self.mark.is_some()&&text.trim().eq_ignore_ascii_case("Escape"){return Some(Ok(Action::CancelTool))}
        if self.split.is_some(){return match text.trim(){""=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Split")=>Some(Ok(Action::Split)),v if v.eq_ignore_ascii_case("Escape")||v.eq_ignore_ascii_case("Esc")=>Some(Ok(Action::CancelSplit)),_=>None}}
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if draft.plane_points.is_none(){if let Some(label)=&draft.text{if label.asking||text.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Height")){return Some(Ok(Action::ToolText(text.into())))}}}

        if let Some(choice)=crate::drawing_input::choice(text){return Some(Ok(choice));}
        if draft.extend.is_some()&&draft.plane_points.is_none(){let word=text.trim();if word.is_empty()||word.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Distance")||w.parse::<f64>().is_ok()){return Some(Ok(Action::ToolText(text.into())))}return None}
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none(){if let Some(gather)=&draft.gather{let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.starts_with('w')||word.starts_with('W')||gather.recipe.option_in(&[word]).is_some()||gather.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&crate::primitive_options::compact(word)))||!gather.typed.is_empty()){return Some(Ok(Action::ToolText(text.into())))}return None}}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()&&d.plane_points.is_none()){return self.project_points(&[point])}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return Err("Extend uses curve-end clicks")}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_point(point);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        if let crate::tool::Built::Text(point)=built{return self.commit_text(point)}
        if let crate::tool::Built::Measured(points)=built{return self.distance_mark(points)}
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Text(_)=>unreachable!("Text adopts its original label owner"),
            crate::tool::Built::Measured(_)=>unreachable!("Measurement changes no original geometry"),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        if name=="Clipping Plane"{if let Some(row)=self.scene.objects().last(){self.select_rows(vec![row.id],false,false);}}
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(e)=&self.element{return e.ids()}if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||crate::feature_owner::highlighted(&self.scene,&self.selected_objects()),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn begin_gather(&mut self,recipe:&'static crate::gather_input::Recipe,text:&str)->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let next=crate::gather_draft::Draft::new(recipe,text,&self.scene,self.selected_objects(),frame).map_err(|message|self.primitive_failure(message))?;let made=next.build().map_err(|message|self.primitive_failure(message))?;
        if made.is_some(){return self.gather_adopt(next,made)}
        self.apply(Action::BeginTool(crate::tool::Shape::Gather(recipe)))?;self.select_objects([],false);self.gather_adopt(next,made)
    }
    fn gather_point(&mut self,point:[f64;3])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.point(point).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.text(text,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    fn gather_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.gather.clone()).ok_or("No gathering tool is running")?;next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;let made=next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.gather_adopt(next,made)}
    pub fn editing_boundaries(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.extend.as_ref().is_some_and(|e|!e.curves))}
    fn commit_project(&mut self,project:crate::project_draft::Draft,plane:session_rust::Plane)->Result<Change,&'static str>{let(edits,message)=project.prepare(&self.scene,&plane).map_err(|message|self.primitive_failure(message))?;self.history.try_edit(&mut self.scene,|scene|{for(id,geometry)in edits{scene.replace_original(id,geometry)?}Ok(())})?;self.select_objects(project.before,false);self.draft=None;self.tool_status=message;Ok(Change::Scene)}
    fn begin_project(&mut self,words:&[String])->Result<Change,&'static str>{
        crate::drawing_coords::set_plane(self.construction_plane);let plan=crate::project_geometry::parse("Project To Plane",&words.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|message|self.primitive_failure(message))?;let mut project=crate::project_draft::Draft::new(&self.scene,self.selected_objects())?;
        if plan.plane=="3Point"{let plane=project.accept(&plan.points).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(project,plane)}self.apply(Action::BeginTool(crate::tool::Shape::Project))?;self.draft.as_mut().unwrap().project=Some(project);self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View)}
        let frame=match plan.plane{"XY"=>crate::cplane::CPlane::Xy,"YZ"=>crate::cplane::CPlane::Yz,"ZX"=>crate::cplane::CPlane::Xz,_=>crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis()))};let(x,y)=frame.axes();self.commit_project(project,session_rust::Plane::from_point_normal(session_rust::Point::new(0.,0.,0.),x.cross(&y),None))
    }
    fn project_points(&mut self,added:&[[f64;3]])->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.project.clone()).ok_or("No projection tool is running")?;let plane=next.accept(added).map_err(|message|self.primitive_failure(message))?;if let Some(plane)=plane{return self.commit_project(next,plane)}let draft=self.draft.as_mut().unwrap();draft.points=next.points.clone();draft.project=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)}
    fn begin_extend(&mut self,distance:Option<f64>,asking:bool)->Result<Change,&'static str>{let next=crate::extend_draft::Draft::new(&self.scene,self.selected_objects(),distance,asking).map_err(|message|self.primitive_failure(message))?;self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;self.select_objects([],false);self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_text(&mut self,text:&str)->Result<Change,&'static str>{let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let finished=next.text(&self.scene,text).map_err(|message|self.primitive_failure(message))?;if finished{self.select_objects(next.before,false);self.draft=None;self.tool_status="Extend done".into();return Ok(Change::View)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn extend_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut next=self.draft.as_ref().and_then(|d|d.extend.clone()).ok_or("No extension tool is running")?;let view=crate::snap_math::Screen::camera(&self.camera,size);next.follow(&self.scene,&view,[(screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.]).map_err(|message|self.primitive_failure(message))?;
        if commit{let hover=next.hover.clone().ok_or_else(||self.primitive_failure(if next.error.is_empty(){"Click near a curve end".into()}else{next.error.clone()}))?;hover.owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(hover.owner.id,hover.geometry))?;let added=hover.tail.windows(2).map(|p|(0..3).map(|i|(p[1][i]-p[0][i]).powi(2)).sum::<f64>().sqrt()).sum::<f64>();next.refreshed(&self.scene,hover.owner.id).map_err(|message|self.primitive_failure(message))?;self.tool_status=format!("Extended by {added:.3} · Undo reverts it");self.draft.as_mut().unwrap().extend=Some(next);return Ok(Change::Scene)}self.draft.as_mut().unwrap().extend=Some(next);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)
    }
    fn edit_interval(&mut self,interval:crate::interval_geometry::Interval)->Result<Change,&'static str>{let ids=self.selected_objects();let[id]=ids.as_slice()else{return Err("Select one original curve")};let owner=crate::source_edit::Owner::capture(&self.scene,*id)?;let geometry=crate::interval_geometry::edited(&owner.source,interval).map_err(|message|self.primitive_failure(message))?;owner.valid(&self.scene)?;self.history.try_edit(&mut self.scene,|scene|scene.replace_original(*id,geometry))?;self.draft=None;self.tool_status="Geometry updated".into();Ok(Change::Scene)}
    fn split_command(&mut self)->Result<Change,&'static str>{
        if self.split.as_ref().is_some_and(|s|s.cutters.is_empty()){return Err("Select at least one cutter curve, then press Enter")}
        if let Some(next)=self.split.take(){let(pieces,regions)=next.prepare(&self.scene).map_err(|message|self.primitive_failure(message))?;if regions<2{self.tool_status="No division: cutters must cross the curve or lie on the selected face".into();return Ok(Change::View)}self.history.try_edit(&mut self.scene,|scene|crate::split_draft::adopt(scene,next.target.id,pieces))?;self.select_objects([next.target.id],false);self.tool_status=format!("Split into {regions} regions · cutters retained · Undo restores the original");return Ok(Change::Scene)}
        let id=self.selection().map(|t|t.owner).or(self.selected).ok_or("Select a curve or one surface face, then run Split")?;
        let face=match self.selected_part{Some(crate::pick_sources::Part::Face(face))=>Some(face as usize),_=>None};
        let next=crate::split_draft::Draft::new(&self.scene,id,face).map_err(|message|self.primitive_failure(message))?;
        self.draft=None;self.gizmo_edit=None;self.edit_preview=None;self.transform_preview=None;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)
    }
    fn split_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{let mut next=self.split.clone().ok_or("Start Split first")?;next.pick(&self.scene,id).map_err(|message|self.primitive_failure(message))?;self.tool_status=next.prompt();self.split=Some(next);Ok(Change::View)}
    fn measured_ids(&self)->Vec<ObjectId>{if self.selected_part.is_some(){self.selected.into_iter().filter(|id|self.scene.contains(*id)).collect()}else{self.selected_objects()}}
    fn measure_selection(&mut self,kind:crate::measure_mark::Kind)->Result<Change,&'static str>{let face=match(self.selected,self.selected_part){(Some(id),Some(crate::pick_sources::Part::Face(face)))=>Some((id,face)),_=>None};let(mark,message)=crate::measure_mark::selection(&self.scene,&self.measured_ids(),face,kind,self.camera.unit).map_err(|message|self.primitive_failure(message))?;self.mark=Some(mark);self.tool_status=message;Ok(Change::View)}
    fn distance_mark(&mut self,points:[[f64;3];2])->Result<Change,&'static str>{let(mark,message)=crate::measure_mark::distance(points[0],points[1],self.camera.unit);self.mark=Some(mark);self.tool_status=message;self.draft=None;Ok(Change::View)}
    fn begin_measure_distance(&mut self,text:&str)->Result<Change,&'static str>{crate::drawing_coords::set_plane(self.construction_plane);let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));let points=if text.trim().is_empty(){Vec::new()}else{crate::drawing_input::points(text,self.construction_plane,frame,&[],None)?};if points.len()>2{return Err("Measure Distance takes two points")};self.apply(Action::BeginTool(crate::tool::Shape::Measure))?;self.snapping.objects=true;if !points.is_empty(){let built=self.draft.as_mut().unwrap().submit_many(&points)?;if let Some(crate::tool::Built::Measured(points))=built{return self.distance_mark(points)}}self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn bounding_box(&mut self)->Result<Change,&'static str>{let ids=self.measured_ids();for id in &ids{if self.scene.objects().iter().any(|r|r.id==*id&&r.geometry().is_none()){return Err("Reload original sources before BoundingBox")}}let bounds=crate::measure_mark::selected_bounds(&self.scene,&ids).map_err(|message|self.primitive_failure(message))?;let mut original=crate::bounds_original::box_geometry(&bounds);original.name="bounding box".into();let mut id=None;self.history.try_edit(&mut self.scene,|scene|{id=Some(crate::bounds_original::insert(scene,std::rc::Rc::new(original))?);Ok(())})?;self.select_objects(id,false);self.tool_status="Created a world axis aligned bounding box".into();Ok(Change::Scene)}
    pub fn guide_overlay(&self)->crate::guide_gpu::Overlay {self.guide_overlay_at([0.;3])}
    pub fn guide_overlay_at(&self,anchor:[f64;3])->crate::guide_gpu::Overlay{
        if let Some(trim)=&self.trim{return trim.overlay_at(anchor)}
        let mut overlay=crate::guide_gpu::Overlay::at(anchor);let red=[210./255.,40./255.,40./255.,1.];
        let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref())else{return overlay};
        let label=if let Some(hover)=&extend.hover{
            overlay.wire(&hover.tail,red,true);hover.tail.last().map(|point|{let added=hover.tail.windows(2).map(|p|(p[1][0]-p[0][0]).hypot(p[1][1]-p[0][1]).hypot(p[1][2]-p[0][2])).sum::<f64>();(*point,format!("+{added:.3}"))})
        }else if !extend.error.is_empty(){extend.last.and_then(|(id,end)|{
            let owner=&extend.candidates.iter().find(|c|c.0.id==id)?.0;let (tip,_)=crate::extend_reach::tip(&owner.source,end)?;let p=tip.transformed(&owner.model);Some(([p[0],p[1],p[2]],extend.error.clone()))
        })}else{None};
        if let Some((point,word))=label{overlay.mark(point,6.,red);overlay.label=Some((point,word));}overlay
    }

    pub fn trimming_picks(&self)->bool{self.trim.as_ref().is_some_and(|t|t.picks())}
    pub fn trimming_parts(&self)->bool{self.trim.as_ref().is_some_and(|t|!t.picks())}
    fn trim_command(&mut self)->Result<Change,&'static str>{
        if self.trim.is_none(){if self.selected_part.is_some(){return Err("Trim starts from whole original objects")}
            let next=crate::trim_draft::Draft::new(&self.scene,self.selected_objects()).map_err(|message|self.primitive_failure(message))?;self.apply(Action::CancelTool)?;self.select_objects([],false);self.tool_status=next.prompt();self.trim=Some(next);return Ok(Change::View)}
        if self.trimming_parts(){return self.commit_trim()}
        let mut trim=self.trim.take().unwrap();let result=trim.prepare(&self.scene,&self.camera);self.tool_status=trim.prompt();self.trim=Some(trim);result.map_err(|message|self.primitive_failure(message))?;Ok(Change::View)
    }
    fn commit_trim(&mut self)->Result<Change,&'static str>{
        let trim=self.trim.take().ok_or("Start Trim first")?;let before=trim.before.clone();if !trim.changed(){self.select_objects(before,false);self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)}
        let edits=trim.prepared_edits(&self.scene).map_err(|message|{self.select_objects(before.clone(),false);self.primitive_failure(message)})?;
        let result=self.history.try_edit(&mut self.scene,|scene|{for(id,pieces)in edits{crate::split_draft::adopt(scene,id,pieces)?}Ok(())});self.select_objects([],false);self.snap_index=None;result?;self.tool_status="Trimmed originals · one Undo restores them".into();Ok(Change::Scene)
    }
    fn trim_screen(&mut self,screen:[f32;2],size:[f64;2],commit:bool)->Result<Change,&'static str>{
        let mut trim=self.trim.take().ok_or("Start Trim first")?;let hit=trim.part_at(&self.camera,screen,size,commit);
        if !commit{trim.hover=hit;self.trim=Some(trim);return Ok(Change::View)}
        let result=hit.ok_or_else(||"Click a remaining trim part".to_owned()).and_then(|(cut,part)|trim.remove(cut,part));
        let finish=match result{Ok(finish)=>finish,Err(message)=>{self.trim=Some(trim);return Err(self.primitive_failure(message))}};
        if !finish{if let Err(message)=trim.refresh_preview(&self.scene){self.trim=Some(trim);return Err(self.primitive_failure(message))}}
        self.trim=Some(trim);if finish{self.commit_trim()}else{Ok(Change::Preview)}
    }
    fn explode_selected(&mut self)->Result<Change,&'static str>{if self.selected_part.is_some(){return Err("Explode works on whole objects")}
        let plan=crate::explode_edit::Plan::prepare(&self.scene,&self.selected_objects()).map_err(|message|self.primitive_failure(message))?;let mut made=None;self.history.try_edit(&mut self.scene,|scene|{made=Some(plan.adopt(scene)?);Ok(())})?;self.select_objects(made.unwrap(),false);self.snap_index=None;Ok(Change::Scene)
    }
    fn curve_heads(&mut self,head:session_rust::Arrowhead)->Result<Change,&'static str>{
        let mut edits=Vec::new();let mut curves=0;
        for id in self.measured_ids(){let owner=crate::source_edit::Owner::capture(&self.scene,id)?;if let Some((next,changed))=crate::curve_heads::headed(&owner.source,head){curves+=1;if changed{edits.push((id,next))}}}
        if curves==0{return Err("Arrowhead needs a selected line, polyline or curve")}if edits.is_empty(){return Ok(Change::View)}
        self.history.try_edit(&mut self.scene,|scene|{for(id,source)in edits{scene.replace_original(id,source)?;}Ok(())})?;self.snap_index=None;Ok(Change::Scene)
    }
    fn begin_text(&mut self,words:String)->Result<Change,&'static str>{let text=crate::text_draft::Draft::new(words,self.camera.distance,self.last_text_height)?;self.apply(Action::BeginTool(crate::tool::Shape::Text))?;self.draft.as_mut().unwrap().text=Some(text);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn commit_text(&mut self,point:[f64;3])->Result<Change,&'static str>{let draft=self.draft.as_ref().ok_or("Start Text first")?;let text=draft.text.as_ref().ok_or("Start Text first")?;let prepared=crate::scene_label::Prepared::new(std::rc::Rc::new(text.source(point,draft.frame,&self.camera)))?;let height=text.height;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;self.last_text_height=height;self.draft=None;self.tool_status="Created original scene text. Undo removes it.".into();Ok(Change::Scene)}
    fn clipping_begin(&mut self,mode:crate::clipping_geometry::Mode)->Result<Change,&'static str>{
        if crate::clipping_rows::owners(&self.scene).len()>=6{return Err("At most six clipping planes")}
        self.apply(Action::CancelTool)?;let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.frame=self.drawing_plane();draft.tool=Box::new(crate::clipping_tool::Clipping{mode,half:crate::clipping_rows::half_size(&self.scene)});self.tool_status=draft.prompt();self.draft=Some(draft);self.snap_index=None;Ok(Change::View)
    }
    fn clipping_request(&mut self,request:crate::clipping_input::Request)->Result<Change,&'static str>{
        use crate::clipping_input::Request;
        match request{
            Request::Begin(mode)=>self.clipping_begin(mode),
            Request::Create(mode,points)=>{let tool=crate::clipping_tool::Clipping{mode,half:crate::clipping_rows::half_size(&self.scene)};let built=crate::tool::Tool::build(&tool,&points)?;self.clipping_begin(mode)?;self.commit_tool(built)},
            Request::Switch(on)=>{self.clipping_enabled=on;self.tool_status=format!("Clipping Plane {}",if on{"On"}else{"Off"});Ok(Change::View)},
            Request::Fill(solid)=>{self.clipping_solid=solid.unwrap_or(!self.clipping_solid);self.tool_status=format!("Clipping Plane Fill {}",if self.clipping_solid{"Solid"}else{"Hatch"});Ok(Change::View)},
            Request::Flip=>{let ids=self.selected_objects();let owners=crate::clipping_rows::owners(&self.scene);let ids=ids.into_iter().filter(|id|owners.contains(id)).collect::<Vec<_>>();if ids.is_empty(){return Err("Select a clipping plane to flip")}
                self.history.try_edit(&mut self.scene,|scene|{for id in &ids{let plane=scene.objects().iter().find(|r|r.id==*id).and_then(|r|r.original.as_ref());let Some(session_rust::Geometry::Plane(plane))=plane else{return Err("Missing original clipping plane")};let flipped=crate::clipping_geometry::flipped(plane);scene.replace_original(*id,session_rust::Geometry::Plane(std::rc::Rc::new(flipped)))?;}Ok(())})?;self.tool_status="Flipped clipping planes. Undo restores their original sides.".into();Ok(Change::Scene)}
        }
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if let Some(label)=self.draft.as_mut().filter(|d|d.plane_points.is_none()).and_then(|d|d.text.as_mut()){if label.asking||text.split_whitespace().next().is_some_and(|w|w.eq_ignore_ascii_case("Height")){label.height(text)?;self.tool_status=label.prompt();return Ok(Change::View)}}
        if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_text(text)}
        if self.draft.as_ref().is_some_and(|d|d.gather.is_some()&&d.plane_points.is_none()){return self.gather_text(text);}
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return self.project_points(&added)}
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        if let Some(gather)=self.draft.as_ref()?.gather.as_ref(){if self.draft.as_ref()?.plane_points.is_none(){match gather.current(){Some(crate::gather_input::Step::Curves{..}|crate::gather_input::Step::Number{..})=>return None,Some(crate::gather_input::Step::Distance(_))=>return gather.on_axis(&self.camera,screen).map(|point|crate::snap::Hit{point,kind:crate::snap::Kind::Near,owner:None}),_=>{}}}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen::camera(&self.camera,size);
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen::camera(&self.camera,size);let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    fn element_commit(&mut self,next:crate::element_draft::Draft)->Result<Change,&'static str>{
        let made=next.build(&self.scene).map_err(|m|self.primitive_failure(m))?;let consumed=next.ids();let mut ids=Vec::new();let show=self.element_attributes;let interactions=self.element_interactions;self.history.try_edit(&mut self.scene,|scene|{ids=crate::element_adopt::adopt(scene,made,&consumed)?;let features=crate::element_adopt::attribute_ids(scene);std::rc::Rc::make_mut(&mut scene.view).hide(&features,!show);let ids=crate::element_adopt::interaction_ids(scene);std::rc::Rc::make_mut(&mut scene.view).hide(&ids,!interactions);Ok(())})?;self.element=None;self.select_objects(ids,false);self.snap_index=None;self.tool_status=format!("Created {} from its sources · Undo restores them",next.kind.name());Ok(Change::Scene)
    }
    fn element_begin(&mut self,kind:crate::element_input::Kind,text:&str)->Result<Change,&'static str>{
        let(next,typed)=crate::element_draft::Draft::new(kind,text,&self.scene,self.selected_objects()).map_err(|m|self.primitive_failure(m))?;
        self.apply(Action::CancelTool)?;if typed&&!next.picked.is_empty(){return self.element_commit(next)}self.select_objects([],false);self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    fn element_text(&mut self,text:&str)->Result<Change,&'static str>{
        let mut next=self.element.clone().ok_or("Start an Element tool first")?;next.valid(&self.scene).map_err(|m|self.primitive_failure(m))?;let finish=next.text(text).map_err(|m|self.primitive_failure(m))?;if finish{return self.element_commit(next)}self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    fn element_pick(&mut self,id:Option<ObjectId>)->Result<Change,&'static str>{
        let mut next=self.element.clone().ok_or("Start an Element tool first")?;next.valid(&self.scene).map_err(|m|self.primitive_failure(m))?;next.pick(&self.scene,id).map_err(|m|self.primitive_failure(m))?;self.tool_status=next.prompt();self.element=Some(next);Ok(Change::View)
    }
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let action=match action{Action::Touch(actions)=>{let mut change=Change::View;for action in actions{let next=self.apply(action)?;if next!=Change::View{change=next;}}return Ok(change)},action=>action};
        if let Action::Opacity(value)=&action{if !value.is_finite()||!(0.0..=1.0).contains(value){return Err("Opacity takes a value from0 to1")}self.opacity=*value;self.tool_status=format!("Opacity {value}");return Ok(Change::View)}
        if let Action::ViewOutline(on)=&action{self.outlines=on.unwrap_or(!self.outlines);self.tool_status=format!("View Outline {}",if self.outlines{"On"}else{"Off"});return Ok(Change::View)}
        if let Action::Arctic(on)=&action{self.arctic=on.unwrap_or(!self.arctic);if self.arctic{self.outlines=true;}self.tool_status=format!("Arctic {}",if self.arctic{"On"}else{"Off"});return Ok(Change::View)}
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::CanvasCancel|Action::ToolScreen{commit:false,..}|Action::TrimScreen{commit:false,..}|Action::ToolHover(_)){self.interaction_epoch=self.interaction_epoch.wrapping_add(1);}
        if matches!(&action,Action::Escape){return if self.lasso.is_some(){self.apply(Action::LassoCancel)}else if self.trim.is_some(){self.apply(Action::CancelTrim)}else if self.split.is_some(){self.apply(Action::CancelSplit)}else if self.gizmo_edit.is_some(){self.apply(Action::GizmoCancel)}else if self.draft.is_some()||self.plane_menu{self.apply(Action::CancelTool)}else if self.pick_mode==crate::pick_sources::Mode::Control{self.apply(Action::Controls(false))}else{self.apply(Action::Picked(None))}}
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::CanvasCancel){self.mark=None;}
        match action {
            Action::LassoPress{at,size,density,pointer}=>{if size.iter().any(|v|!v.is_finite()||*v<=0.)||!density.is_finite()||density<=0.||size.iter().any(|v|v*density>32768.)||at.iter().any(|v|!v.is_finite()){return Err("Invalid lasso viewport")}let lasso=self.lasso.as_mut().ok_or("Start Select Lasso first")?;lasso.abandoned();lasso.size=size;lasso.density=density;lasso.pointer=Some(pointer);lasso.spacing=2.;lasso.points.push((at[0]*density,at[1]*density));return Ok(Change::View)},
            Action::LassoMove{at,pointer}=>{if let Some(lasso)=self.lasso.as_mut().filter(|l|l.pointer==Some(pointer)){if at.iter().all(|v|v.is_finite()){let at=[at[0].clamp(0.,lasso.size[0]),at[1].clamp(0.,lasso.size[1])];lasso.follow((at[0]*lasso.density,at[1]*lasso.density));}}return Ok(Change::View)},
            Action::LassoRelease{at,pointer,add,remove}=>{self.apply(Action::LassoMove{at,pointer})?;let Some(lasso)=self.lasso.as_mut().filter(|l|l.pointer==Some(pointer))else{return Ok(Change::View)};lasso.pointer=None;if lasso.points.len()<3{lasso.points.clear();return Ok(Change::View)}let ids=lasso.found(&self.scene,&self.camera);lasso.points.clear();let ids=ids?;self.select_rows(ids,add&&!remove,remove);self.lasso=None;self.tool_status=format!("{} selected inside the lasso",self.selected_objects().len());return Ok(Change::View)},
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::NamedView(_) | Action::Isometric | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.project.is_some()||d.extend.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.gather.is_some()){self.apply_action(Action::CancelTool)?;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {
        if self.lasso.is_some()&&!matches!(&action,Action::SelectLasso|Action::LassoCancel|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::NamedView(_)|Action::Isometric|Action::Projection(_)|Action::Fit|Action::FitSelected|Action::ResetView|Action::Grid(_)|Action::ConstructionPlane(_)){self.lasso=None;}
        if self.trim.is_some()&&!matches!(&action,Action::Trim|Action::TrimPick(_)|Action::TrimScreen{..}|Action::CancelTrim|Action::TreePick{..}|Action::ResetView|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::NamedView(_)|Action::Isometric|Action::Projection(_)|Action::Fit|Action::FitSelected|Action::Grid(_)|Action::NormalView(_)|Action::Background|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){let trim=self.trim.take().unwrap();self.select_objects(trim.before,false);}

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        if self.split.is_some()&&!matches!(&action,Action::Split|Action::SplitPick(_)|Action::CancelSplit|Action::TreePick{..}|Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::NamedView(_)|Action::Isometric|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::ConstructionPlane(_)|Action::PlaneMenu|Action::Plane3){self.split=None;}
        match action {
            Action::ElementBegin{kind,text}=>return self.element_begin(kind,&text),
            Action::ElementText(text)=>return self.element_text(&text),
            Action::ElementPick(id)=>return self.element_pick(id),
            Action::ElementAttributes(on)|Action::ElementInteractions(on)=>{
                let (shown,ids,label)=if matches!(action,Action::ElementInteractions(_)) {
                    (&mut self.element_interactions,crate::element_adopt::interaction_ids(&self.scene),"Interactions")
                }else{(&mut self.element_attributes,crate::element_adopt::attribute_ids(&self.scene),"Attributes")};
                *shown=on.unwrap_or(!*shown);
                std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,!*shown);
                self.tool_status=format!("Element {label} {}",if *shown{"On"}else{"Off"});
                return Ok(Change::Scene)
            },
            Action::Opacity(_)=>unreachable!("Opacity presentation returned before document mutation"),
            Action::ViewOutline(_)=>unreachable!("Outline presentation returned before editor mutation"),
            Action::Arctic(_)=>unreachable!("Arctic presentation returned before editor mutation"),
            Action::Clipping(request)=>{return self.clipping_request(request)},
            Action::SelectLasso=>{self.lasso=Some(Default::default());self.tool_status="Select Lasso · drag a loop · Shift adds · Ctrl removes · Esc cancels".into();return Ok(Change::View)},
            Action::LassoCancel=>{self.lasso=None;self.tool_status="Select Lasso cancelled; selection unchanged".into();return Ok(Change::View)},
            Action::LassoPress{..}|Action::LassoMove{..}|Action::LassoRelease{..}=>unreachable!(),
            Action::AddGroup(name)=>{let ids=self.selected_objects();let mut label=String::new();self.history.try_edit(&mut self.scene,|scene|{label=crate::layer_edit::group(scene,&ids,&name)?;Ok(())})?;self.tool_status=format!("Grouped {} objects as {label}; a click selects all; Undo removes the group",ids.len());return Ok(Change::Scene)},
            Action::AddEdge=>{let ids=self.selected_objects();let mut message=String::new();self.history.try_edit(&mut self.scene,|scene|{message=crate::graph_edit::connect(scene,&ids)?;Ok(())})?;self.tool_status=message;return Ok(Change::Scene)},
            Action::Explode=>return self.explode_selected(),
            Action::TextBegin(words)=>return self.begin_text(words),
            Action::Trim=>return self.trim_command(),
            Action::TrimPick(id)=>{let mut trim=self.trim.take().ok_or("Start Trim first")?;let result=trim.pick(&self.scene,id);self.tool_status=trim.prompt();self.trim=Some(trim);result.map_err(|message|self.primitive_failure(message))?;return Ok(Change::View)},
            Action::TrimScreen{screen,size,commit}=>return self.trim_screen(screen,size,commit),
            Action::CancelTrim=>{if let Some(trim)=self.trim.take(){self.select_objects(trim.before,false)}self.tool_status="Trim cancelled · nothing changed".into();return Ok(Change::Preview)},
            Action::Query(query)=>{let ids=query.found(&self.scene).map_err(|message|self.primitive_failure(message))?;self.select_objects(ids,false);return Ok(Change::View)},
            Action::Arrowhead(head)=>return self.curve_heads(head),
            Action::Escape=>{if self.element.is_some()||self.draft.is_some()||self.split.is_some()||self.gizmo_edit.is_some(){self.apply_action(Action::GizmoCancel)?;return self.apply_action(Action::CancelTool)}self.select_objects([],false);return Ok(Change::View)},
            Action::Measure(kind)=>return self.measure_selection(kind),
            Action::MeasureDistance(text)=>return self.begin_measure_distance(&text),
            Action::BoundingBox=>return self.bounding_box(),
            Action::Split=>return self.split_command(),
            Action::SplitPick(id)=>return self.split_pick(id),
            Action::CancelSplit=>{self.split=None;self.tool_status="Split cancelled".into();return Ok(Change::View)},
            Action::Project(words)=>return self.begin_project(&words),
            Action::ExtendBegin{distance,asking}=>return self.begin_extend(distance,asking),
            Action::ExtendPick(id)=>return self.extend_pick(id),
            Action::Interval(interval)=>return self.edit_interval(interval),
            Action::GatherBegin{recipe,text}=>return self.begin_gather(recipe,&text),
            Action::GatherPick(id)=>return self.gather_pick(id),
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                if let Some(gather)=self.draft.as_mut().and_then(|d|d.gather.as_mut()){gather.frame=frame;}
                self.camera.grid_plane=plane;self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if self.plane_only || draft.plane_points.is_some() || !matches!(draft.tool.name(),"Polyline"|"Curve"){return Err("Close works on a polyline or curve")}
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{if self.draft.as_ref().is_some_and(|d|d.extend.is_some()){return self.extend_text("")}if self.draft.as_ref().is_some_and(|d|d.project.is_some()){return Err("Project 3Point needs three plane points")}if self.draft.as_ref().is_some_and(|d|d.gather.is_some()){return self.gather_text("")}crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);self.apply(Action::FitSelected)?;return Ok(Change::Preview);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                if self.draft.as_ref().is_some_and(|d|d.extend.is_some()&&d.plane_points.is_none()){return self.extend_screen(screen,size,commit)}
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.gather.is_none() && d.project.is_none() && d.extend.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => {self.element=None;if let Some(before)=self.draft.as_ref().and_then(|d|d.project.as_ref()).map(|p|p.before.clone()).or_else(||self.draft.as_ref().and_then(|d|d.extend.as_ref()).map(|e|e.before.clone())){self.select_objects(before,false);}if let Some(before)=self.draft.as_ref().and_then(|d|d.gather.as_ref()).map(|g|g.before.clone()){self.select_objects(before,false);}let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let ready=crate::publication::prepare(&self.scene,&crate::tree_specimen::nested(true),false,None).map_err(|_|"Cannot prepare nested originals")?;self.history.edit(&mut self.scene,|scene|scene.restore(ready));},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            file @ (Action::Replace(_) | Action::Import(_) | Action::ImportAt(_,_) | Action::ReplaceAt(_,_)) => {
                let (bytes,replace,location)=match file {
                    Action::Replace(bytes)=>(bytes,true,None),
                    Action::Import(bytes)=>(bytes,false,None),
                    Action::ImportAt(bytes,location)=>(bytes,false,Some(location)),
                    Action::ReplaceAt(bytes,location)=>(bytes,true,Some(location)),
                    _=>unreachable!(),
                };
                let ready=crate::publication::prepare(&self.scene,&bytes,replace,location).map_err(|_|"Cannot prepare original document")?;
                self.history.edit(&mut self.scene,|scene|scene.restore(ready));
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::EnableControls => {
                let parent=self.selected.ok_or("Select one object before Controls")?;
                if self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.control_markers().iter().any(|m|m.parent==parent) || self.scene.clouds().iter().any(|r|r.id==parent&&r.controls_visible) || self.scene.streams().iter().any(|r|r.id==parent&&r.controls_visible)) {return Ok(Change::View);}
                if self.selected_objects().len()!=1 {return Err("Select one object before Controls");}
                self.scene.owner_controls(parent)?;
                self.pick_mode=crate::pick_sources::Mode::Control;
                self.objects.clear();self.selected_part=None;
            },
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.set_view(crate::camera::View::Iso),
                    Action::NamedView(view)=>self.camera.set_view(view),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,grid_plane:self.camera.grid_plane,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/navigation_complete_tests.rs`

Retain the same cursor-zoom pose and exact target-plane invariance in both projections and panel frames.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{editor::{Action,Change,Editor},camera::{Camera,Projection},gesture::{Gesture,Motion}};

#[test]
fn cursor_zoom_holds_the_target_plane_point_in_both_projections_and_frames() {
    for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut camera=Camera::default();camera.projection=projection;camera.frame([0.,0.,0.7,0.8]);
        let at=[0.2,-0.1];let ray=camera.ray(at).unwrap();let forward=camera.orientation.rotate_vector(session_rust::Vector::y_axis());
        let centre=session_rust::Point::new(camera.target[0],camera.target[1],camera.target[2]);let t=(&centre-&ray.origin).dot(&forward)/ray.direction.dot(&forward);
        let point=session_rust::Point::new(ray.origin[0]+ray.direction[0]*t,ray.origin[1]+ray.direction[1]*t,ray.origin[2]+ray.direction[2]*t);
        let before=camera.view_projection().transform_point(&point);camera.zoom_at(1.4,at);let after=camera.view_projection().transform_point(&point);
        assert!((before[0]-after[0]).abs()<1e-7&&(before[1]-after[1]).abs()<1e-7);assert!((camera.distance-3./1.4_f32 as f64).abs()<1e-9);
        let saved=camera.view_projection().m;camera.zoom_at(2.,[f32::NAN,0.]);assert_eq!(camera.view_projection().m,saved);
    }
}

#[test]
fn slop_pan_and_free_canvas_are_css_based_without_document_history() {
    let mut input=Gesture::default();input.press_with(1,2,[10.,20.],true);assert_eq!(input.move_to(1,[13.,23.]),None);assert_eq!(input.move_to(1,[15.,24.]),Some(Motion::Pan([5.,4.])));
    let mut camera=Camera::default();let old=camera.target;camera.pan_screen([20.,-10.],[640.,480.]);assert_ne!(camera.target,old);camera.pan_screen([-20.,10.],[640.,480.]);assert!(camera.target.into_iter().zip(old).all(|(a,b)|(a-b).abs()<1e-12));
    assert!(camera.frame([0.,0.,0.7,0.8]));let target=session_rust::Point::new(camera.target[0],camera.target[1],camera.target[2]);let projected=camera.view_projection().transform_point(&target);assert!((projected[0]+0.3).abs()<1e-12&&(projected[1]-0.2).abs()<1e-12);
    let frame=camera.frame;assert!(!camera.frame([0.,0.,0.,1.]));assert_eq!(camera.frame,frame);
}

#[test]
fn select_all_and_remove_preserve_sources_and_undo_while_respecting_current_flags() {
    let mut e=Editor::default();let ids=e.scene.objects().iter().map(|r|r.id).collect::<Vec<_>>();let source=e.scene.objects()[0].geometry().unwrap().clone();
    std::rc::Rc::make_mut(&mut e.scene.view).lock(&[ids[1]],true);assert_eq!(e.apply(Action::SelectAll).unwrap(),Change::View);assert_eq!(e.selected_objects(),vec![ids[0]]);
    e.apply(Action::SelectRows {ids:vec![ids[0]],add:true,remove:true}).unwrap();assert!(e.selected_objects().is_empty());assert!(std::rc::Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    assert_eq!(crate::shortcuts::key("a",true,false,false,false,false,false),Some(crate::shortcuts::Shortcut::SelectAll));assert_eq!(crate::shortcuts::key("a",true,false,false,false,true,false),None);
}

fn start(e:&mut Editor,from:[f32;2],to:[f32;2]) {e.apply(Action::CanvasStart {from,to,size:[640.,480.],modified:false,pointer:1}).unwrap();}

#[test]
fn a_released_hit_commits_the_entire_move_once_and_shares_original_source() {
    let mut e=Editor::default();let id=e.scene.objects()[0].id;let original=e.scene.placement(id).unwrap().m;let source=e.scene.objects()[0].geometry().unwrap().clone();
    start(&mut e,[-0.3,0.],[0.1,0.]);assert_eq!(e.apply(Action::CanvasMove {screen:[0.2,0.],size:[640.,480.]}).unwrap(),Change::View);
    e.apply(Action::CanvasRelease {screen:[0.4,0.],size:[640.,480.],add:false,remove:false}).unwrap();let released=e.canvas_drag.as_ref().unwrap().to;
    e.apply(Action::CanvasMove {screen:[0.6,0.],size:[640.,480.]}).unwrap();assert_eq!(e.canvas_drag.as_ref().unwrap().to,released);
    assert_eq!(e.apply(Action::CanvasHit(Some(id))).unwrap(),Change::Scene);let moved=e.scene.placement(id).unwrap().m;assert_ne!(moved,original);assert!(e.canvas_drag.is_none());assert!(std::rc::Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,moved);
}

#[test]
fn a_released_miss_uses_release_modifiers_and_new_press_discards_the_old_drag() {
    let mut e=Editor::default();e.apply(Action::SelectAll).unwrap();let before=e.selected_objects();
    start(&mut e,[-1.,1.],[1.,-1.]);e.apply(Action::CanvasRelease {screen:[1.,-1.],size:[640.,480.],add:true,remove:true}).unwrap();
    assert_eq!(e.apply(Action::CanvasHit(None)).unwrap(),Change::View);assert!(e.selected_objects().is_empty());assert!(e.canvas_drag.is_none());assert!(!before.is_empty());
    start(&mut e,[-0.5,0.],[0.2,0.]);e.apply(Action::CanvasRelease {screen:[0.3,0.],size:[640.,480.],add:false,remove:false}).unwrap();start(&mut e,[0.,0.],[0.1,0.]);assert_eq!(e.canvas_drag.as_ref().unwrap().from,[0.,0.]);
    e.apply(Action::CanvasCancel).unwrap();let placements=e.scene.objects().iter().map(|r|r.model.m).collect::<Vec<_>>();e.apply(Action::CanvasHit(Some(e.scene.objects()[0].id))).unwrap();assert_eq!(e.scene.objects().iter().map(|r|r.model.m).collect::<Vec<_>>(),placements);
}

#[test]
fn touch_pinch_pan_wait_for_all_lifts_and_double_tap_fits() {
    let rect=[0.,0.,640.,480.];let mut touch=crate::touch::Touch::default();assert!(touch.press(1,[200.,200.],0.));assert!(touch.move_to(1,[203.,202.],rect).is_empty());assert!(matches!(touch.move_to(1,[210.,205.],rect).as_slice(),[Action::Orbit(..)]));
    assert!(touch.press(2,[300.,200.],20.));let actions=touch.move_to(2,[340.,220.],rect);assert!(matches!(actions.as_slice(),[Action::PanScreen {..},Action::ZoomAt {..}]));
    touch.release(2,[340.,220.],30.,rect);assert!(touch.move_to(1,[230.,205.],rect).is_empty());assert!(touch.release(1,[230.,205.],40.,rect).is_empty());assert_eq!(touch.count(),0);
    touch.press(3,[200.,200.],100.);assert!(matches!(touch.release(3,[200.,200.],120.,rect).as_slice(),[Action::Pick(_)]));touch.press(4,[210.,200.],150.);assert!(matches!(touch.release(4,[210.,200.],180.,rect).as_slice(),[Action::Fit]));
    touch.press(5,[200.,200.],200.);touch.cancel();assert!(touch.release(5,[200.,200.],220.,rect).is_empty());assert!(!touch.press(6,[f64::NAN,0.],230.));
}

#[test]
fn a_failed_cold_source_drag_releases_its_preview_and_keeps_redo() {
    let mut e=Editor::default();let location=std::rc::Rc::new(crate::reload_url::ReloadUrl::new("test:drag".into()));
    e.apply(Action::ReplaceAt(crate::specimen::bytes(),location)).unwrap();let id=e.scene.objects()[0].id;
    e.apply(Action::AddBox).unwrap();e.apply(Action::Undo).unwrap();e.apply(Action::UnloadSources).unwrap();let before=e.scene.placement(id).unwrap().m;
    start(&mut e,[0.,0.],[0.2,0.]);assert!(e.apply(Action::CanvasHit(Some(id))).unwrap_err().contains("Reload"));assert!(e.canvas_drag.is_none());assert_eq!(e.scene.placement(id).unwrap().m,before);
    let count=e.scene.objects().len();e.apply(Action::Redo).unwrap();assert_eq!(e.scene.objects().len(),count+1);
}

#[test]
fn panel_free_fit_contains_every_corner_in_perspective_and_orthographic() {
    let mut bounds=crate::bounds::Bounds::point([-3.,-2.,-1.]);bounds.include([3.,2.,1.]);
    for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut camera=Camera::default();camera.projection=projection;camera.isometric();camera.frame([0.,0.,0.6,0.75]);camera.fit(&bounds);
        for x in [bounds.min[0],bounds.max[0]] {for y in [bounds.min[1],bounds.max[1]] {for z in [bounds.min[2],bounds.max[2]] {
            let p=camera.view_projection().transform_point(&session_rust::Point::new(x,y,z));assert!(p[0]>=-1.&&p[0]<=0.2&&p[1]>=-0.5&&p[1]<=1.);
        }}}
    }
}

#[test]
fn only_an_unmoved_right_click_repeats_and_cancel_clears_the_request() {
    let mut gesture=Gesture::default();gesture.press(1,2,[0.,0.]);assert_eq!(gesture.release(1,[2.,2.]),None);assert!(gesture.take_repeat());assert!(!gesture.take_repeat());
    gesture.press(1,2,[0.,0.]);gesture.move_to(1,[10.,0.]);gesture.release(1,[0.,0.]);assert!(!gesture.take_repeat());gesture.press(1,2,[0.,0.]);gesture.cancel();assert!(!gesture.take_repeat());
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/navigation_tests.rs`

Declare the original three-unit wheel fixture so its absolute accumulation tolerance and history assertions remain unchanged.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{editor::{Action, Change, Editor}, navigation::wheel};

fn distance(delta: f64, mode: u32) -> f64 {
    let mut editor = Editor::default();
    assert_eq!(editor.apply(wheel(delta, mode, 480.0).unwrap()).unwrap(), Change::View);
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(48.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!((distance(48.0, 0) - distance(0.1, 2)).abs() < 1.0e-6);
    let mut editor = Editor::default();
    for _ in 0..10 {
        editor.apply(wheel(-12.0, 0, 480.0).unwrap()).unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor.apply(wheel(120.0, 0, 480.0).unwrap()).unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 480.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(wheel(-120.0, 0, 480.0).unwrap()).unwrap();
    let zoomed = editor.camera.distance;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.distance, zoomed);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected.unwrap();
    editor.apply(Action::Delete).unwrap();
    assert!(!editor.scene.contains(selected));
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.contains(selected));
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/picking.rs`

Declare the original Top ray before asserting turquoise-before-pink original surface ordering and bounded-ray rejection.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{camera::Ray, scene::{ObjectId, Scene}};
use session_rust::{Point, Vector};

pub fn pick(scene: &Scene, ray: &Ray) -> Option<ObjectId> {
    let mut nearest = ray.max_distance;
    let mut hit = None;
    for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
        let vertices = object.mesh.vertices();
        for triangle in object.mesh.indices().chunks_exact(3) {
            let a = object.world_point(vertices[triangle[0] as usize]);
            let b = object.world_point(vertices[triangle[1] as usize]);
            let c = object.world_point(vertices[triangle[2] as usize]);
            if let Some(distance) = triangle_distance(ray, a, b, c) {
                if distance >= 0.0 && distance < nearest {
                    nearest = distance;
                    hit = Some(object.id);
                }
            }
        }
    }
    hit
}

fn triangle_distance(ray: &Ray, a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<f64> {
    let edge1 = Vector::new(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let edge2 = Vector::new(c[0] - a[0], c[1] - a[1], c[2] - a[2]);
    let p = ray.direction.cross(&edge2);
    let determinant = edge1.dot(&p);
    if determinant.abs() < 1.0e-10 {
        return None;
    }
    let origin = &ray.origin - &Point::new(a[0], a[1], a[2]);
    let u = origin.dot(&p) / determinant;
    let q = origin.cross(&edge1);
    let v = ray.direction.dot(&q) / determinant;
    if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
        Some(edge2.dot(&q) / determinant)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;

    #[test]
    fn perspective_picking_chooses_the_nearest_visible_surface() {
        let mut scene = Scene::demo();
        let camera = Camera::default();
        let ray = camera.ray([0.0, 0.0]).unwrap();
        let pink = scene.objects()[0].id;
        let turquoise = scene.objects()[1].id;
        assert_eq!(pick(&scene, &ray), Some(turquoise));
        scene.remove(turquoise);
        assert_eq!(pick(&scene, &ray), Some(pink));
        let short = Ray { max_distance: 0.1, ..ray };
        assert_eq!(pick(&scene, &short), None);
    }

    #[test]
    fn picking_follows_the_camera_and_ignores_triangles_behind_the_ray() {
        let scene = Scene::demo();
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.rotate(0.7);
        camera.zoom(0.5);
        let screen = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.75));
        let ray = camera.ray([screen[0] as f32, screen[1] as f32]).unwrap();
        assert_eq!(pick(&scene, &ray), Some(scene.objects()[1].id));
        let away = Ray { origin: Point::new(0.0, 0.0, 2.0), direction: Vector::z_axis(), max_distance: 10.0 };
        assert_eq!(pick(&scene, &away), None);
    }
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-14.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/primitive_tests.rs`

Declare the axis-aligned Top view before testing the height-axis refusal and orbit recovery.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{editor::{Action,Editor},primitive_shape::{Answer,Kind},primitive_draft::Draft,cplane::CPlane};
use session_rust::Point;
fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,line:&str)->Result<(),String> {
    let action=if let Some(action)=e.tool_line(line){action.map_err(str::to_owned)?}else{let crate::commands::Command::Editor(action)=crate::commands::parse(line)? else{return Err("Editor command".into())};action};e.apply(action).map_err(|s|if s=="Shape input invalid" {e.tool_status.clone()}else{s.into()})?;Ok(())
}
#[test]
fn direct_maintained_solid_builders_create_original_geometry_as_one_transaction() {
    let mut e=editor();for (line,faces) in [("Box 0,0,0 2 3 4",6),("Pyramid 4,0,0 2 3",5),("Block With Hole 8,0,0 4 3 2 0.4",7),("Tetrahedron 12,0,0 2",4),("Octahedron 16,0,0 2",8),("Icosahedron 20,0,0 2",20),("Dodecahedron 24,0,0 2",12)] {
        let count=e.scene.objects().len();command(&mut e,line).unwrap();assert!(e.draft.is_none());let cad=e.scene.objects().last().unwrap().cad.as_ref().unwrap();assert_eq!(cad.source.face_count(),faces);command(&mut e,"Undo").unwrap();assert_eq!(e.scene.objects().len(),count);command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),count+1);
    }
    let sources=e.scene.objects().iter().map(|row|row.cad.as_ref().unwrap().source.to_proto()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&bytes).unwrap();assert_eq!(e.scene.objects().iter().map(|row|row.cad.as_ref().unwrap().source.to_proto()).collect::<Vec<_>>(),sources);
}
#[test]
fn typed_defaults_and_brep_mesh_choices_preserve_the_current_answers() {
    let mut e=editor();command(&mut e,"Box").unwrap();command(&mut e,"0,0,0").unwrap();assert!(command(&mut e,"").is_err());command(&mut e,"2").unwrap();command(&mut e,"Mesh").unwrap();assert_eq!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.len(),2);command(&mut e,"").unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"").unwrap();assert_eq!(e.scene.objects().len(),1);assert!(e.scene.objects()[0].cad.is_none());
    let bounds=e.scene.bounds().unwrap();assert_eq!(bounds.min,[-1.,-1.,0.]);assert_eq!(bounds.max,[1.,1.,2.]);command(&mut e,"Undo").unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),1);
}
#[test]
fn clicked_corners_and_signed_heights_follow_the_current_fixed_plane() {
    let mut e=editor();command(&mut e,"CPlane XZ").unwrap();command(&mut e,"Box").unwrap();e.apply(Action::ToolPoint([1.,2.,3.])).unwrap();e.apply(Action::ToolPoint([2.,2.,5.])).unwrap();assert!(e.scene.objects().is_empty());command(&mut e,"-3").unwrap();
    let source=&e.scene.objects()[0].cad.as_ref().unwrap().source;let bounds=crate::primitive_shape::tests::bounds(&source.vertex_points());assert_eq!(bounds,[[0.,2.,1.],[2.,5.,5.]]);
}
#[test]
fn invalid_direct_and_batched_shape_input_preserves_draft_points_and_redo() {
    let mut e=editor();command(&mut e,"Box 0,0,0 2 3 4").unwrap();command(&mut e,"Undo").unwrap();command(&mut e,"Line").unwrap();command(&mut e,"w0,0,0").unwrap();
    assert!(command(&mut e,"Box 0,0,0 0 3 4").is_err());assert_eq!(e.draft.as_ref().unwrap().tool.name(),"Line");assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.;3]]);
    command(&mut e,"Escape").unwrap();command(&mut e,"Box").unwrap();for text in ["0,0,0 2 3 4 extra","0,0,0 2 3 4 Mesh","0,0,0 NaN","0,0,0 2 0"] {assert!(command(&mut e,text).is_err(),"{text}");assert!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.is_empty());assert!(e.scene.objects().is_empty());}
    command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),1);
}
#[test]
fn height_clicks_use_the_normal_axis_and_refuse_an_axis_aligned_view() {
    let mut draft=Draft::new(&crate::primitive_box::SHAPE,CPlane::Xy,0).unwrap();draft.accept(Answer::Point(Point::new(0.,0.,0.))).unwrap();draft.accept(Answer::Number(2.)).unwrap();draft.accept(Answer::Number(3.)).unwrap();assert_eq!(draft.next().unwrap().kind,Kind::Height);
    let mut camera=crate::camera::Camera::default();assert!(draft.on_axis(&camera,[0.,0.]).is_none());camera.orbit(0.,std::f64::consts::FRAC_PI_4);let point=draft.on_axis(&camera,[0.,0.2]).unwrap();assert!(point[0].abs()<1e-12 && point[1].abs()<1e-12);assert!(point[2].is_finite());
}
#[test]
fn pending_solid_wires_and_cancel_never_create_saved_geometry() {
    let mut e=editor();command(&mut e,"Box 0,0,0 2 3").unwrap();let draft=e.draft.as_ref().unwrap();assert_eq!(draft.primitive.as_ref().unwrap().answers.len(),3);assert!(e.scene.objects().is_empty());assert_eq!(draft.outlines().len(),1);
    e.apply(Action::ToolHover(Some([0.,0.,4.]))).unwrap();assert_eq!(e.draft.as_ref().unwrap().outlines().len(),6);assert!(e.scene.objects().is_empty());command(&mut e,"Escape").unwrap();assert!(e.draft.is_none());assert!(e.scene.objects().is_empty());
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-15.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/projection_tests.rs`

Keep the close orthographic fixture at its declared three-unit Top pose and preserve behind-eye picking, history and Reset projection assertions.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{bounds::Bounds, camera::{Camera, Projection}, editor::{Action, Change, Editor}};
use session_rust::{Point, Vector};

#[test]
fn switching_projection_preserves_scale_on_the_target_plane() {
    let mut camera = Camera::default();
    camera.pan(0.3, -0.2);
    camera.isometric();
    let right = camera.orientation.rotate_vector(Vector::x_axis());
    let up = camera.orientation.rotate_vector(Vector::z_axis());
    let point = Point::new(
        camera.target[0] + right[0] * 0.4 + up[0] * 0.2,
        camera.target[1] + right[1] * 0.4 + up[1] * 0.2,
        camera.target[2] + right[2] * 0.4 + up[2] * 0.2,
    );
    let before = camera.view_projection().transform_point(&point);
    camera.projection = Projection::Orthographic;
    let after = camera.view_projection().transform_point(&point);
    for i in 0..2 {
        assert!((before[i] - after[i]).abs() < 1.0e-12);
    }
}

#[test]
fn orthographic_size_ignores_depth_and_its_pick_rays_are_parallel() {
    let mut camera = Camera { projection: Projection::Orthographic, ..Camera::default() };
    let a = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.0));
    let b = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.75));
    assert!((a[0] - b[0]).abs() < 1.0e-12);
    let left = camera.ray([-0.5, 0.0]).unwrap();
    let right = camera.ray([0.5, 0.0]).unwrap();
    assert!((&right.origin - &left.origin).magnitude() > 1.0);
    assert!(left.direction.cross(&right.direction).magnitude() < 1.0e-12);
    camera.projection = Projection::Perspective;
    let a = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.0));
    let b = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.75));
    assert!(b[0] > a[0]);
    assert!(camera.ray([-0.5, 0.0]).unwrap().direction
        .cross(&camera.ray([0.5, 0.0]).unwrap().direction).magnitude() > 0.1);
}

#[test]
fn an_orthographic_fit_contains_every_corner_in_tall_and_wide_views() {
    let mut bounds = Bounds::point([-3.0, -1.0, -0.5]);
    bounds.include([2.0, 4.0, 1.0]);
    for aspect in [0.25, 1.0, 4.0] {
        let mut camera = Camera { projection: Projection::Orthographic, aspect, ..Camera::default() };
        camera.isometric();
        camera.fit(&bounds);
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let point = camera.view_projection().transform_point(&Point::new(x, y, z));
                    assert!(point[0].abs() < 1.0 && point[1].abs() < 1.0);
                    assert!((0.0..1.0).contains(&point[2]));
                }
            }
        }
    }
}

#[test]
fn close_orthographic_picking_sees_geometry_behind_the_eye() {
    let mut editor = Editor::default();
    let before = editor.camera.uniform();
    assert_eq!(editor.apply(Action::Projection(Projection::Orthographic)).unwrap(), Change::View);
    editor.apply(Action::Projection(Projection::Perspective)).unwrap();
    assert_eq!(editor.camera.uniform(), before);
    editor.apply(Action::Projection(Projection::Orthographic)).unwrap();
    editor.apply(Action::Zoom(100.0)).unwrap();
    assert!(editor.camera.distance < 0.75);
    editor.apply(Action::Pick([0.0, 0.0])).unwrap();
    assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Projection(Projection::Perspective)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.projection, Projection::Perspective);
    editor.apply(Action::Projection(Projection::Orthographic)).unwrap();
    editor.apply(Action::ResetView).unwrap();
    assert_eq!(editor.camera.projection, Projection::Perspective);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/global_outline_tests.rs`

Declare the existing unit-sized camera/editor poses for GPU fixture scenes; retain exact pixels, primitive/source IDs, released-drag completion, caches, uploads and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.2,-0.25,-0.5],[0.3,-0.25,-0.5],[0.1,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&layout,&scene.objects()[0],false);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let original=gpu.meshes[0].packed();let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload_at(&gpu.device,&layout,&far.objects()[0],false,anchor);assert_eq!(gpu.meshes[0].packed(),original);let geometry=gpu.meshes[0].geometry.clone();let style=crate::gpu_mesh::Style{selected:false,pickable:true,colour:None};gpu.meshes[0]=crate::gpu_mesh::GpuMesh::with_geometry_at(&gpu.device,&layout,&far.objects()[0],style,geometry.clone(),anchor);assert_eq!(gpu.meshes[0].packed(),original);assert!(Rc::ptr_eq(&gpu.meshes[0].geometry,&geometry));gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[0,0]);let selected=crate::gpu_mesh::Style{selected:true,..style};assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],selected),[1,16]);assert_eq!(&gpu.meshes[0].packed()[..64],&original[..64]);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[1,16]);assert_eq!(gpu.meshes[0].packed(),original);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);assert_eq!(gpu.stats()[0],uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_original_highlights_keep_local_pixels_addresses_and_sources_after_rebasing(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.4,0.5,-0.5,0.4,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.4,-0.5,-0.5),session_rust::Point::new(0.,-0.3,-0.5),session_rust::Point::new(0.4,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(path).unwrap();scene.cloud_controls(Some(cloud));let curve=scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(&[session_rust::Point::new(-0.4,0.8,-0.5),session_rust::Point::new(0.,0.65,-0.5),session_rust::Point::new(0.4,0.8,-0.5)]))).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let edge=scene.objects()[0].mesh.parts.edges[0];let control=scene.control_markers().into_iter().find(|c|c.parent==path).unwrap().part;let targets=[Target{owner:mesh,part:Part::Face(key)},Target{owner:mesh,part:Part::MeshEdge(edge.0,edge.1)},Target{owner:line,part:Part::Curve},Target{owner:curve,part:Part::Curve},Target{owner:path,part:Part::PathEdge(0)},Target{owner:path,part:control},Target{owner:cloud,part:Part::CloudPoint(u32::MAX)}];let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let mut pixels=Vec::new();for target in targets{assert!(target.valid(&scene));gpu.set_highlight(&scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(selected!=baseline,"A valid original highlight must change actual pixels: {target:?}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);pixels.push(selected);}gpu.set_highlight(&scene,None).unwrap();gpu.select_objects(&scene,&[line,path,point]).unwrap();let objects=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert_ne!(objects,baseline);gpu.select_objects(&scene,&[]).unwrap();gpu.set_highlight(&scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,transform),baseline);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,curve]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.space.update(anchor,3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&far);gpu.set_clouds(&far,&[]).unwrap();camera.target=anchor;gpu.cloud_camera(&camera);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);
 gpu.mesh_ink.set(&gpu.device,crate::pick_sources::Sources::anchored(&far,anchor).unwrap().mesh_ink).unwrap();for(target,expected)in targets.into_iter().zip(pixels){assert!(target.valid(&far));gpu.set_highlight(&far,Some(target)).unwrap();let actual=crate::cap_gpu_tests::pixels_at(&gpu,local);assert!(actual==expected,"Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",actual.iter().zip(&expected).filter(|(a,b)|a!=b).count());assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);}gpu.set_highlight(&far,None).unwrap();gpu.select_objects(&far,&[line,path,point]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),objects);gpu.select_objects(&far,&[]).unwrap();gpu.set_highlight(&far,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_distant_widgets_keep_local_pixels_scale_picks_and_tile_release(){
 let scene=Scene::empty();let anchor=[1e12,-1e12,1e12];let pivot=[0.125,0.25,-0.5];let mut renderer=renderer(&scene);for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic]{for density in [1u32,2]{let mut camera=crate::camera::Camera::default();camera.isometric();camera.projection=projection;camera.aspect=4./3.;let size=[320*density,240*density];renderer.resize(crate::viewport::Viewport{width:size[0],height:size[1]});renderer.set_density(density as f32);renderer.space.forget();let scale=crate::widget_layout::camera_scale(&camera,pivot,240.).unwrap();renderer.set_gizmo(&camera,Some(pivot),-1.);let usage=renderer.gizmo_usage();let extent=renderer.gizmo_extent();let transform=camera.uniform();let baseline=crate::widget_tests::pixels(&mut renderer,&transform,size);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));renderer.space.update(anchor,3.,0.).unwrap();camera.target=anchor;let distant=std::array::from_fn(|i|anchor[i]+pivot[i]);assert_eq!(crate::widget_layout::camera_scale(&camera,distant,240.).unwrap(),scale);renderer.set_gizmo(&camera,Some(distant),-1.);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);let actual=crate::widget_tests::pixels(&mut renderer,&local,size);assert!(actual==baseline,"Precise distant widget pixels differ at density {density}");assert_eq!(renderer.gizmo_extent(),extent);assert_eq!(renderer.gizmo_usage()[2],usage[2]);renderer.request_pick([0.,0.],local).unwrap();crate::cap_gpu_tests::wait(&renderer);assert!(renderer.take_pick(local).unwrap().unwrap().is_none());renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());}}
}

#[test]
fn actual_origin_only_pick_views_discard_old_hits_and_restore_owners(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);let transform=matrix();let uploads=gpu.stats()[0];gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,gpu.space.anchor());}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert!(gpu.take_pick(transform).is_none(),"Identical matrix in a new origin must not accept the old owner");assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none(),"The automatic current-frame retry is empty");gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(transform).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources(){
 use crate::editor::{Editor,Action,Change};let mut editor=Editor::default();editor.scene.clear();let owner=editor.scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();editor.scene.translate(owner,[2000.,0.,0.]).unwrap();editor.camera.target=[2000.,0.,0.];let original=editor.scene.placement(owner).unwrap().m;let source=editor.scene.objects()[0].geometry().unwrap().clone();let mut gpu=renderer(&editor.scene);let uploads=gpu.stats()[0];let transform=editor.camera.uniform();let size=[320.,240.];editor.apply(Action::CanvasStart{from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();gpu.request_pick([0.,0.],transform).unwrap();editor.apply(Action::CanvasRelease{screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);gpu.space.update(editor.camera.target,editor.camera.distance,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,editor.camera.target);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let local=editor.camera.view_projection_anchored(gpu.anchor()).to_f32();assert_ne!(local,transform);wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);assert_eq!(editor.scene.placement(owner).unwrap().m,original);wait(&gpu);let hit=gpu.take_pick(local).unwrap().unwrap().and_then(|pixel|gpu.resolve_pick(&editor.scene,pixel));assert_eq!(hit.unwrap().owner,owner);assert_eq!(editor.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(editor.scene.placement(owner).unwrap().m,original);assert!(Rc::ptr_eq(&source,editor.scene.objects()[0].geometry().unwrap()));gpu.set_scene(&editor.scene,editor.selected);assert_eq!(gpu.stats()[0],uploads);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);
}
#[test]
fn actual_rebased_pending_cursor_stays_newest_and_cancellation_never_revives_it(){
 let mut scene=Scene::empty();let make=||PreparedMesh::triangle([[-0.18,-0.18,-0.5],[0.18,-0.18,-0.5],[0.,0.18,-0.5]],[0.2,0.4,0.8]).unwrap();let a=scene.insert(make()).unwrap();let b=scene.insert(make()).unwrap();scene.translate(a,[1999.5,0.,0.]).unwrap();scene.translate(b,[2000.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;let local=matrix();gpu.request_pick([-0.5,0.],old).unwrap();gpu.request_pick([0.5,0.],old).unwrap();assert_eq!(&gpu.pick_usage()[..2],&[1,1]);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],b.value());assert_eq!(gpu.pick_usage()[3],2);assert!(gpu.take_pick(local).is_none());gpu.request_pick([-0.5,0.],local).unwrap();gpu.cancel_pick();gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(old).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);gpu.request_pick([-0.5,0.],old).unwrap();gpu.set_scene(&Scene::empty(),None);gpu.space.update([2000.,0.,0.],3.,600.).unwrap();wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_origin_refresh_reuses_selected_face_geometry_and_original_ink(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();scene.translate(owner,[2000.,0.,0.]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let target=Some(Target{owner,part:Part::Face(key)});let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;gpu.set_highlight(&scene,target).unwrap();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,old);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let geometry=gpu.highlight.geometry_for_test().unwrap().clone();let uploads=gpu.stats()[0];gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&scene);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));let actual=crate::cap_gpu_tests::pixels_at(&gpu,matrix());assert!(actual==baseline,"Rebased original face and ink pixels must match exactly");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);drop(geometry);gpu.set_highlight(&scene,None).unwrap();assert!(gpu.highlight.geometry_for_test().is_none());assert!(!gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_shared_origin_keeps_all_original_lanes_and_pending_picks_coherent(){
 let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.5,0.5,-0.5,0.5,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.5,-0.5,-0.5),session_rust::Point::new(0.,-0.25,-0.5),session_rust::Point::new(0.5,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![255,0,0,255],vec![]))).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.25,-0.5,0.75,0.25,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let text_source=Rc::new(crate::scene_label::example(crate::text::TextPlacement::Anchor{world:[-0.5,0.75,-0.5],offset:[0.;2]}));let text=scene.insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap()).unwrap();let ids=[line,path,point,cloud,drawing,text];let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;let mut gpu=renderer(&scene);gpu.select_objects(&scene,&ids).unwrap();let settings=Some(crate::grid::Settings{spacing:0.125,half_cells:4});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();assert!(!gpu.prepare_origin(&camera,&scene,None,&ids,0.).unwrap().moved);let transform=gpu.camera_matrix(&camera);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));let ground=gpu.ambient_rows.ground_at([0.;3]);let uploads=gpu.stats()[0];let anchor=[2000.,-2000.,2000.];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,drawing,text]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);camera.target=anchor;gpu.set_grid_on(settings,plane([anchor[0],anchor[1],anchor[2]-0.75])).unwrap();let frame=gpu.prepare_origin(&camera,&far,None,&ids,300.).unwrap();assert!(frame.moved&&!frame.pending);assert_eq!(gpu.camera_matrix(&camera),transform);let actual=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(actual==baseline,"Shared original lanes must retain exact local/distant pixels");assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);let writes=gpu.stats();assert!(!gpu.prepare_origin(&camera,&far,None,&ids,301.).unwrap().moved);assert_eq!(gpu.stats(),writes);gpu.request_pick([0.,0.],transform).unwrap();camera.target[0]+=2000.;let pending=gpu.prepare_origin(&camera,&far,None,&ids,350.).unwrap();assert!(!pending.moved&&pending.pending);assert_eq!(gpu.rebase_delay(350.),Some(151));assert_eq!(&gpu.pick_usage()[..2],&[1,0]);let moved=gpu.prepare_origin(&camera,&far,None,&ids,501.).unwrap();assert!(moved.moved&&!moved.pending);assert_eq!(gpu.camera_matrix(&camera),transform);assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());camera.target=anchor;assert!(gpu.prepare_origin(&camera,&far,None,&ids,802.).unwrap().moved);let restored=crate::cap_gpu_tests::pixels_at(&gpu,gpu.camera_matrix(&camera));assert!(restored==baseline,"Restored shared origin must restore exact pixels");assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),original);assert!(Rc::ptr_eq(&far.texts()[0].prepared.source,&text_source));assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-17.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/pick_gpu_tests.rs`

Declare the existing unit-sized camera/editor poses for GPU fixture scenes; retain exact pixels, primitive/source IDs, released-drag completion, caches, uploads and release.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use crate::{renderer::Renderer, viewport::Viewport};

fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, flags: Default::default(), memory_budget_thresholds: Default::default(), backend_options: Default::default(), display: None });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).expect("Actual GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 }); renderer
}

fn finish(renderer: &mut Renderer) -> Option<[u32; 2]> {
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    renderer.take_pick(identity()).expect("Map callback completed").unwrap()
}

fn identity() -> [f32; 16] { crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|n| n as f32)) }
fn triangle(z: f64) -> PreparedMesh {
    PreparedMesh::triangle([[-0.8, -0.8, z], [0.8, -0.8, z], [0.0, 0.8, z]], [0.8, 0.2, 0.1]).unwrap()
}


#[test]
fn actual_projected_feature_pixel_selects_original_element_not_display_path() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature pick");
    element.set_geometry(session_rust::Mesh::create_box(0.2,0.2,0.2));
    let outline = session_rust::Polyline::new(vec![session_rust::Point::new(-0.8,0.4,0.2),session_rust::Point::new(0.8,0.4,0.2)]);
    element.add_feature(session_rust::element::ElementFeature::new("cut", 0, vec![outline], "pick line"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene,owner,&session_rust::Session::new("test"),&original).unwrap();
    assert_eq!(scene.paths().len(),1); let path=scene.paths()[0].id;
    let mut renderer=renderer(&scene); renderer.request_pick([0.,0.4],identity()).unwrap();
    let pixel=finish(&mut renderer).unwrap();assert_eq!(pixel,[owner.value(),0]);assert_ne!(pixel[0],path.value());
    assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target {owner,part:crate::pick_sources::Part::Object}));
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.,0.4],identity()).unwrap();assert_eq!(finish(&mut renderer),None);
}

#[test]
fn actual_feature_point_disc_uses_original_owner_and_css_diameter() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature dot");
    element.set_geometry(session_rust::Mesh::create_box(0.2, 0.2, 0.2));
    let mut dot = session_rust::Polyline::new(vec![session_rust::Point::new(0.5, 0.4, 0.2), session_rust::Point::new(0.5, 0.4, 0.2)]);
    dot.linecolor = session_rust::Color::new(0.9, 0.1, 0.1, 1.);
    element.add_feature(session_rust::element::ElementFeature::new("drill", -1, vec![dot], "dot"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene, owner, &session_rust::Session::new("dot"), &original).unwrap();
    assert_eq!(scene.points().len(), 1);
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.resize(Viewport {width: 320, height: 240});
        renderer.set_density(density);
        let image = colour_pixels(&mut renderer);
        let pixels: Vec<_> = image.chunks_exact(4).enumerate().filter_map(|(index, pixel)| {
            let (x, y) = (index % 320, index / 320);
            (x > 210 && (40..105).contains(&y) && u16::from(pixel[0]) > 2 * u16::from(pixel[1])
                && u16::from(pixel[0]) > 2 * u16::from(pixel[2])).then_some((x, y))
        }).collect();
        assert!(!pixels.is_empty());
        for axis in [0, 1] {
            let values: Vec<_> = pixels.iter().map(|&(x, y)| if axis == 0 { x } else { y }).collect();
            let span = values.iter().max().unwrap() - values.iter().min().unwrap() + 1;
            assert!((span as f32 - 12. * density).abs() <= 1., "DPR {density}, axis {axis}, physical span {span}");
        }
        // Picking searches the already-taught six-CSS-pixel neighbourhood around the disc.
        for (offset, expected) in [(0., Some([owner.value(), 0])), (5., Some([owner.value(), 0])), (13., None)] {
            renderer.request_pick([0.5 + 2. * offset * density / 320., 0.4], identity()).unwrap();
            assert_eq!(finish(&mut renderer), expected, "DPR {density}, CSS offset {offset}");
        }
    }
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
    let mut scene = Scene::demo(); scene.clear(); scene.next_id = 16_777_217;
    let back = scene.insert(triangle(0.6)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    let front = scene.insert(triangle(0.2)).unwrap(); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([front.value(), 0]));
    Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
    assert_eq!(scene.pick_id(front.value()), None);
    Rc::make_mut(&mut scene.view).hide(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    renderer.request_pick([0.95, 0.95], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
}

#[test]
fn actual_ink_pick_uses_coverage_depth_density_and_bounded_coalescing() {
    let mut scene = Scene::demo(); scene.clear();
    let mesh = scene.insert(triangle(0.5)).unwrap();
    let mut point = session_rust::Point::new(0.0, 0.0, 0.2); point.width = 12.0;
    let point = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap()).unwrap();
    let mut line = session_rust::Line::new(-0.4, 0.4, 0.2, 0.4, 0.4, 0.2); line.width = 5.0;
    let line = scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let mut path = session_rust::Polyline::new(vec![session_rust::Point::new(-0.4, -0.4, 0.2), session_rust::Point::new(0.4, -0.4, 0.2)]);
    path.width = 5.0;
    let path = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(path)).unwrap()).unwrap();
    let mut renderer = renderer(&scene); let geometry = renderer.stats();
    for density in [1.0, 2.0] {
        renderer.set_density(density); renderer.resize(Viewport { width: (320.0 * density) as u32, height: (240.0 * density) as u32 });
        for (cursor, expected) in [([0.0, 0.0], point), ([0.0, 0.4], line), ([0.0, -0.4], path), ([0.0, 0.65], mesh)] {
            renderer.request_pick(cursor, identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], expected.value());
        }
    }
    assert_eq!(renderer.stats(), geometry, "Picking and resize reuse visible geometry");
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    for _ in 0..20 { renderer.request_pick([0.95, 0.95], identity()).unwrap(); }
    assert_eq!(&renderer.pick_usage()[..2], &[1, 1]); assert_eq!(renderer.pick_usage()[4], 19);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "A newer cursor supersedes the first result");
    assert_eq!(finish(&mut renderer), None);
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    assert!(renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 16 + 192 + 3 * 52, "Readback does not grow with the full drawing");
}

#[test]
fn cancelled_readback_cannot_select_a_new_document_or_keep_its_source_alive() {
    let mut scene = Scene::demo(); scene.clear(); let old = scene.insert(triangle(0.2)).unwrap();
    let weak = Rc::downgrade(&scene.objects[0].mesh); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); scene.clear(); renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none()); assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 160]);
    let new = scene.insert(triangle(0.3)).unwrap(); assert_ne!(old, new); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(renderer.take_pick(identity()).is_none(), "The cancelled map has no result for the replacement");
    let uploads = renderer.stats(); renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.cancel_pick();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none()); assert_eq!(renderer.stats(), uploads);
}

#[test]
fn a_real_readback_from_an_old_camera_cannot_select_in_the_current_view() {
    let mut scene = Scene::demo(); scene.clear(); scene.insert(triangle(0.2)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    let mut moved = identity(); moved[12] = 0.2;
    assert!(renderer.take_pick(moved).is_none(), "Same scene, different camera: no accepted hit");
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    let submitted = renderer.pick_usage()[3]; let mut invalid = moved; invalid[0] = f32::NAN;
    assert!(renderer.request_pick([0.0, 0.0], invalid).is_err()); assert_eq!(renderer.pick_usage()[3], submitted);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert!(finish(&mut renderer).is_some());
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.set_density(2.0);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "Density changed while the map was pending");
}

#[test]
fn actual_face_and_edge_ids_preserve_sparse_source_keys_and_exclude_a_rendering_diagonal() {
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.5, -0.5, 0.5]), (20, [0.5, -0.5, 0.5]), (30, [0.5, 0.5, 0.5]), (40, [-0.5, 0.5, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.set_face_triangulation(37, vec![[10, 20, 40], [20, 30, 40]]);
    source.set_facecolors(vec![session_rust::Color::new(0.8, 0.2, 0.1, 1.)]);
    let mut scene = Scene::demo(); scene.clear(); let id = scene.insert(PreparedMesh::new(source).unwrap()).unwrap(); let mut renderer = renderer(&scene);
    let before = renderer.stats(); renderer.set_pick_mode(crate::pick_sources::Mode::Face);
    renderer.request_pick([0., 0.], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Face(37) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., -0.5], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::MeshEdge(10, 20) }));
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "The triangulation diagonal is not an editable edge");
    renderer.set_pick_mode(crate::pick_sources::Mode::Object); renderer.request_pick([0., 0.], identity()).unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); assert_eq!(renderer.stats(), before);
}

#[test]
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let id = editor.scene.paths()[0].id; let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Control(1) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., 0.1], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Curve }));
    editor.scene.show_controls(false); renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap(); assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {colour_pixels_at(renderer,identity())}
fn colour_pixels_at(renderer:&mut Renderer,transform:[f32;16])->Vec<u8> {colour_pixels_display(renderer,transform,false)}
fn colour_pixels_display(renderer:&mut Renderer,transform:[f32;16],normals:bool)->Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &transform, normals);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.8, -0.6, 0.5]), (20, [0., -0.6, 0.5]), (30, [0., 0.6, 0.5]), (40, [-0.8, 0.6, 0.5]),
                     (50, [0.8, -0.6, 0.5]), (60, [0.8, 0.6, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo(); scene.clear(); let owner = scene.insert(PreparedMesh::new(source).unwrap()).unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh); let mut renderer = renderer(&scene); let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::Face(37) })).unwrap();
    let face = colour_pixels(&mut renderer); assert!(renderer.highlight_bytes() > 0); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(face.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((31..=160).contains(&x) && (47..=192).contains(&y)); changed += 1;
    } }
    assert!(changed > 10000, "The original face, rather than the whole object, is visibly selected");
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::MeshEdge(10, 20) })).unwrap();
    let edge = colour_pixels(&mut renderer); let mut changed = 0; let mut black = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(edge.chunks_exact(4)).enumerate() {
        let x = index % 320; let y = index / 320;
        if a != b { assert!((30..=161).contains(&x) && (190..=193).contains(&y)); changed += 1; }
        if (35..150).contains(&x) && (191..=192).contains(&y) && b[0] < 160 && b[1] < 140 && b[2] < 80 { black += 1; }
    }
    assert!(changed > 80 && black > 80, "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}");
    assert_eq!(renderer.stats(), stats, "Highlights reuse primary geometry");
    renderer.set_highlight(&scene, None).unwrap(); assert_eq!(renderer.highlight_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear(); renderer.set_scene(&scene, None); assert!(weak.upgrade().is_none()); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner = editor.scene.paths()[0].id; let original = editor.scene.paths()[0].points(); let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer); renderer.set_highlight(&editor.scene, Some(crate::pick_sources::Target {
        owner, part: crate::pick_sources::Part::Control(1) })).unwrap(); let after = colour_pixels(&mut renderer); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(after.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((151..=168).contains(&x) && (39..=56).contains(&y)); changed += 1;
    } }
    assert!(changed > 20, "The halo belongs to the original control rather than a curve sample");
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor.apply(crate::editor::Action::Close).unwrap(); renderer.set_scene(&editor.scene, None); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo(); scene.clear();
    let panel = |x: f64| PreparedMesh::triangle([[x - 0.25, -0.4, 0.5], [x + 0.25, -0.4, 0.5], [x, 0.4, 0.5]], [0.5, 0.5, 0.5]).unwrap();
    let a = scene.insert(panel(-0.4)).unwrap(); let b = scene.insert(panel(0.4)).unwrap(); let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer); let geometry = renderer.stats();
    renderer.select_objects(&scene, &[a, b]).unwrap(); let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec();
    for x in [96, 224] { let p = pixel(&selected, x); assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0); }
    renderer.select_objects(&scene, &[b]).unwrap(); let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96)); assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(&renderer.stats()[..2], &geometry[..2], "Only selected settings change, not primary geometry");
    for (x, id) in [(-0.4, a), (0.4, b)] { renderer.request_pick([x, 0.], identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo(); scene.clear(); let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer); let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap(); draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap(); assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(a,b)| a!=b).count();
    assert!(changed > 100, "The actual unfinished rubber band and cursor render");
    let p = &after[(120*320+160)*4..(120*320+160)*4+3]; assert_eq!(p, &[255,255,0], "The preview uses current selection yellow: {p:?}");
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "Drafts have no saved-object pick owner");
    assert_eq!(renderer.stats(), stats); assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap(); assert_eq!(renderer.preview_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap(); scene.clear(); renderer.set_scene(&scene, None); assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.5)).unwrap();let source=Rc::clone(scene.objects()[0].geometry().unwrap());let original=source.to_proto();
    let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let allocations=renderer.stats();
    Rc::make_mut(&mut scene.view).faces.insert(id,[0,0,255]);renderer.set_scene(&scene,None);let blue=colour_pixels(&mut renderer);
    assert_ne!(blue,before);let centre=(120*320+160)*4;assert!(blue[centre+2]>blue[centre].saturating_add(100)&&blue[centre+2]>blue[centre+1].saturating_add(100));
    assert_eq!(&renderer.stats()[..2],&allocations[..2],"Layer colour reuses primary geometry and its settings allocation");assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    Rc::make_mut(&mut scene.view).edges.insert(id,[0,255,0]);renderer.set_scene(&scene,None);let green_edges=colour_pixels(&mut renderer);let count=green_edges.chunks_exact(4).filter(|p|p[1]>p[0].saturating_add(100)&&p[1]>p[2].saturating_add(100)).count();assert!(count>100,"Actual source boundary colour is independent: {count}");
    Rc::make_mut(&mut scene.view).faces.remove(&id);renderer.set_scene(&scene,None);let restored_faces=colour_pixels(&mut renderer);assert_eq!(&restored_faces[centre..centre+4],&before[centre..centre+4]);assert_ne!(restored_faces,before,"Restoring Faces leaves the independent edge override");
    Rc::make_mut(&mut scene.view).edges.remove(&id);renderer.set_scene(&scene,None);assert_eq!(colour_pixels(&mut renderer),before);assert!(Rc::ptr_eq(&source,scene.objects()[0].geometry().unwrap()));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Editor,Action,Change};
    let mut e=Editor::default();e.scene.clear();let id=e.scene.insert(triangle(0.2)).unwrap();let original=e.scene.placement(id).unwrap().m;let source=e.scene.objects()[0].geometry().unwrap().clone();
    let mut renderer=renderer(&e.scene);let transform=e.camera.uniform();let gpu=renderer.stats();let size=[320.,240.];
    e.apply(Action::CanvasStart {from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();renderer.request_pick([0.,0.],transform).unwrap();
    e.apply(Action::CanvasRelease {screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
    renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();let hit=renderer.take_pick(transform).unwrap().unwrap().and_then(|pixel|renderer.resolve_pick(&e.scene,pixel));assert_eq!(hit.unwrap().owner,id);
    assert_eq!(e.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(e.scene.placement(id).unwrap().m,original);assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    renderer.set_scene(&e.scene,e.selected);assert_eq!(&renderer.stats()[..2],&gpu[..2]);e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let source=Rc::new(crate::surface::example());let _=source.guid();let before=source.to_proto();let id=scene.insert_surface(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap().part,Part::Face(0));
    renderer.set_pick_mode(Mode::Edge);for (point,edge) in [([-1.,0.,0.],0),([1.,0.,0.],1),([0.,-1.,0.],2),([0.,1.,0.],3)] {
        let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));renderer.request_pick([point[0] as f32,point[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let mut original=session_rust::BRep::create_block_with_hole(1.6,1.2,0.24,0.26);original.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);let source=Rc::new(original);let _=source.guid();let before=source.to_proto();let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert!(finish_at(&mut renderer).is_none(),"The actual shaded hole stays empty");
    let p=camera.view_projection().transform_point(&session_rust::Point::new(0.5,0.,0.12));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::Face(6),"The visible top cap retains its original CAD identity");renderer.set_pick_mode(Mode::Edge);let mut seen=0;
    for border in &scene.objects()[0].cad.as_ref().unwrap().edges {
        let points=&border.points;if !points.iter().all(|p|(p[2]-0.12).abs()<1e-10) {continue;}
        let middle=points.windows(2).nth((points.len()-1)/2).unwrap();let p=std::array::from_fn::<_,3,_>(|i|(middle[0][i]+middle[1][i])*0.5);let p=camera.view_projection().transform_point(&session_rust::Point::new(p[0],p[1],p[2]));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap_or_else(||panic!("Missing original CAD edge {} at {:?}",border.edge,p)).part,Part::BRepEdge(border.edge));seen+=1;
    }
    assert_eq!(seen,5,"All four outer edges and the original hole rim resolve");assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.2)).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();renderer.text_demo().unwrap();let with_text=colour_pixels(&mut renderer);assert_ne!(with_text,before);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.stats(),gpu);assert_eq!(scene.objects().len(),1);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([-0.85,0.8],identity()).unwrap();assert!(finish(&mut renderer).is_none());assert_eq!(renderer.text_usage()[0],1);renderer.set_text(Vec::new()).unwrap();assert_eq!(renderer.text_usage(),[0;6]);assert_eq!(colour_pixels(&mut renderer),before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();
    let mut source=example(TextPlacement::WorldPlane {world:[-0.1,0.05,0.6],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.08});source.label.text="Hi".into();let source=Rc::new(source);let original=serde_json::to_string(&*source).unwrap();let weak=Rc::downgrade(&source);let id=scene.insert_text(Prepared::new(source.clone()).unwrap()).unwrap();drop(source);renderer.set_scene(&scene,None);
    assert_eq!(colour_pixels(&mut renderer),before,"Actual fixed text behind the opaque solid is hidden");renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.place_text(id,session_rust::Xform::translation(0.,0.,-0.45)).unwrap();renderer.set_scene(&scene,None);let front=colour_pixels(&mut renderer);assert_ne!(front,before);assert_eq!(renderer.text_error(),None);assert!(front.chunks_exact(4).filter(|p|p[0]<10&&p[1]<10&&p[2]<10).count()>20);let raster=renderer.text_scene_usage()[2];assert_eq!(raster,1,"Placement reuses already rasterized original words");assert_eq!(renderer.text_usage()[0],1);
    renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap(),[id.value(),0]);renderer.select_objects(&scene,&[id]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).filter(|p|p[0]>200&&p[1]>200&&p[2]<40).count()>20);assert_eq!(renderer.text_scene_usage()[2],raster);assert_eq!(renderer.text_usage()[0],1);assert_eq!(&renderer.stats()[..2],&gpu[..2]);assert_eq!(serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),original);
    Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,-0.008],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"The locked plate remains an occluder without a selectable id");Rc::make_mut(&mut scene.view).hide(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.clear();renderer.set_scene(&scene,None);assert!(weak.upgrade().is_none());assert_eq!(renderer.text_scene_usage(),[0;4]);assert_eq!(renderer.text_usage(),[0;6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    for (placement,overlay) in [(TextPlacement::Anchor {world:[0.,0.,0.6],offset:[0.;2]},false),(TextPlacement::Nameplate {world:[0.,0.,0.6],padding:[17.,4.],rounded:true},true),(TextPlacement::WorldBillboard {world:[0.,0.,0.6],world_height:0.08},false)] {
        let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let mut source=example(placement);source.label.text="Hi".into();let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();renderer.set_scene(&scene,None);let after=colour_pixels(&mut renderer);assert_eq!(after!=before,overlay,"Only the nameplate overlays its own solid");renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],if overlay {id.value()} else {mesh.value()});assert_eq!(renderer.text_error(),None);
        let fonts=vec![crate::text::FONT_BYTES.to_vec(),crate::text::FALLBACK_BYTES.to_vec(),crate::text::SYMBOL_BYTES.to_vec()];assert!(renderer.replace_text_fonts(vec![vec![0;8]]).is_err());assert_eq!(colour_pixels(&mut renderer),after);renderer.replace_text_fonts(fonts).unwrap();assert_eq!(colour_pixels(&mut renderer),after);assert_eq!(renderer.text_usage()[5],1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mut source=example(TextPlacement::WorldPlane {world:[-0.2,0.1,0.3],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.2});source.label.text="Hi".into();source.label.clip=Some([140.,100.,160.,140.]);let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);assert_eq!(renderer.text_error(),None);
    let mut clean=Scene::demo();clean.clear();let mut no_text=super::pick_gpu_tests::renderer(&clean);let baseline=colour_pixels(&mut no_text);let mut changed=0;
    for (i,(p,b)) in before.chunks_exact(4).zip(baseline.chunks_exact(4)).enumerate() {let (x,y)=(i%320,i/320);if x<140||x>=160||y<100||y>=140 {assert_eq!(p,b,"Clipped text does not affect pixels outside the CSS clip");} else {changed+=usize::from(p!=b);}}
    assert!(changed>50);renderer.request_pick([-0.0625,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([0.125,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks() {
    let mut scene=Scene::demo();scene.clear();let mesh=|x:f64|{let mut source=session_rust::Mesh::create_box(0.6,0.8,0.1);source.set_objectcolor(session_rust::Color::new(0.5,0.5,0.5,1.));let mut prepared=PreparedMesh::new(source).unwrap();prepared.model=session_rust::Xform::translation(x,0.,0.5);prepared};let a=scene.insert(mesh(-0.3)).unwrap();let b=scene.insert(mesh(0.3)).unwrap();let sources=scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>();let mut renderer=renderer(&scene);let geometry=renderer.stats();renderer.select_objects(&scene,&[a,b]).unwrap();let first=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert_eq!(alpha.len(),320*240);assert!(alpha.iter().filter(|v|**v>0).count()>300);for y in 90..150 {for x in 90..230 {assert_eq!(alpha[y*320+x],0,"The combined coverage has no outline along the shared object join or tessellation diagonals");}}
    let stats=renderer.outline_usage();assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.outline_alpha(),alpha);assert_eq!(renderer.outline_usage()[1],stats[1]);assert!(renderer.outline_usage()[2]>stats[2]);assert_eq!(&renderer.stats()[..2],&geometry[..2]);assert_eq!(scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>(),sources);renderer.request_pick([-0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],a.value());renderer.request_pick([0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],b.value());
    renderer.select_objects(&scene,&[]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().is_empty());assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha() {
    use crate::pick_sources::{Target,Part};
    let mut scene=Scene::demo();scene.clear();let back=scene.insert(triangle(0.6)).unwrap();let front=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[back]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().iter().all(|v|*v==0),"A selected surface wholly behind the current opaque depth adds no silhouette");renderer.select_objects(&scene,&[front]).unwrap();let plain=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v>128));let edge=scene.objects().iter().find(|row|row.id==front).unwrap().mesh.parts.edges[0];renderer.set_highlight(&scene,Some(Target {owner:front,part:Part::MeshEdge(edge.0,edge.1)})).unwrap();let inked=colour_pixels(&mut renderer);assert_eq!(renderer.outline_alpha(),alpha,"Source edge highlighting cannot change the actual outline width");let mut opaque=0;for ((a,b),coverage) in plain.chunks_exact(4).zip(inked.chunks_exact(4)).zip(&alpha) {if *coverage>=250&&a[..3].iter().all(|c|*c<8) {opaque+=1;assert!(b[..3].iter().all(|c|*c<12),"Yellow source edge ink cannot paint over black outline pixels");}}assert!(opaque>100);scene.clear();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[id]).unwrap();let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source=Rc::new(session_rust::BRep::create_block_with_hole(1.2,1.2,0.4,0.25));
    let _=source.guid();let original=source.to_proto();let mut scene=Scene::demo();scene.clear();
    let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);
    renderer.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut renderer);
    let alpha=renderer.outline_alpha();
    for y in 112..128 {for x in 152..168 {assert_eq!(alpha[y*320+x],0,"The bore center stays open");}}
    let mut inner=0;
    for y in 80..160 {for x in 110..210 {
        let i=y*320+x;
        if alpha[i]>=250 {inner+=1;assert!(pixels[i*4..i*4+3].iter().all(|c|*c<12),"The actual inner silhouette is black");}
    }}
    assert!(inner>100,"The real bore boundary has an inner outline: {inner}");
    renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    renderer.request_pick([0.4,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());
    assert_eq!(source.to_proto(),original,"Coverage cannot change original topology");
}

#[test]
fn actual_cloud_pixels_and_whole_ids_keep_cached_source_uploads_density_and_locked_occlusion() {
    let mut source=session_rust::PointCloud::from_coords(vec![0.,0.,0.2],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let _=source.guid();let before=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.6)).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    for density in [1.,2.] {renderer.set_density(density);let first=colour_pixels(&mut renderer);let center=&first[(120*320+160)*4..(120*320+160)*4+4];assert!(center[0]>240&&center[1]<10&&center[2]<10,"Original cloud color covers the solid behind it");let usage=renderer.cloud_usage();assert_eq!(usage[0],1);assert_eq!(usage[2],1);assert!(usage[1]>320*240*16);assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.cloud_usage()[3],usage[3]);assert!(renderer.cloud_usage()[4]>usage[4]);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([cloud.value(),0]));}
    renderer.set_density(1.);Rc::make_mut(&mut scene.view).lock(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),None,"The locked cloud writes zero while occluding the mesh");
    Rc::make_mut(&mut scene.view).hide(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([mesh.value(),0]));assert_eq!(source.to_proto(),before);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}
#[test]
fn actual_cloud_copy_and_normal_shading_preserve_one_source_and_release_all_derived_owners() {
    let source=Rc::new(session_rust::PointCloud::from_coords(vec![-0.3,0.,0.2,0.3,0.,0.2],vec![0,255,0,255,0,255,0,255],vec![0.,0.,1.,0.,0.,1.]));let weak=Rc::downgrade(&source);let mut scene=Scene::demo();scene.clear();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[1]>160&&p[0]<10&&p[2]<10));assert_eq!(renderer.cloud_usage()[2],1);
    let copy=scene.copy_row(cloud,false).unwrap();scene.translate(copy,[0.,0.3,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],1,"Copies share one original upload");assert_eq!(renderer.cloud_usage()[0],4);renderer.request_pick([-0.3,0.3],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],copy.value());drop(source);scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

fn camera_pixels(renderer: &mut Renderer,camera:&crate::camera::Camera) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.cloud_camera(camera);renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &camera.uniform(), false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload() {
    let source=Rc::new(crate::cloud::large_example());assert_eq!(source.len(),2_000_064);let mut scene=Scene::demo();scene.clear();scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    let coarse=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];let prepared=renderer.cloud_usage()[3];assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[3],prepared);
    camera.distance=0.7/std::f64::consts::FRAC_PI_6.tan();let fine=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);assert_ne!(fine,coarse);assert_eq!(renderer.cloud_usage()[2],uploads);assert_eq!(source.point_id(64),64);assert_eq!(source.point_id(source.len()-1),2_000_063);
    camera.distance=100./std::f64::consts::FRAC_PI_6.tan();assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=session_rust::PointCloud::from_coords(vec![-0.4,0.,0.2,0.4,0.,0.2],vec![],vec![]).to_proto();
    proto.point_size=16.;proto.point_ids=vec![u32::MAX,16_777_217];let source=Rc::new(session_rust::PointCloud::from_proto(proto));let _=source.guid();let original=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();scene.cloud_controls(Some(owner));let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    let uploaded=renderer.cloud_usage()[2];let geometry=renderer.stats();
    for density in [1.,2.] {
        renderer.set_density(density);renderer.resize(Viewport {width:(320.*density) as u32,height:(240.*density) as u32});
        for (x,id) in [(-0.4,u32::MAX),(0.4,16_777_217)] {
            renderer.request_pick([x,0.],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();assert_eq!(target.part,Part::CloudPoint(id));assert_eq!(target.owner,owner);
            renderer.set_highlight(&scene,Some(target)).unwrap();assert!(renderer.highlight_bytes()>0);assert_eq!(renderer.stats(),geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2],uploaded);assert_eq!(source.to_proto(),original);
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_density(1.);renderer.resize(Viewport {width:320,height:240});renderer.set_scene(&scene,None);
    renderer.request_pick([-0.4,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    scene.clear();renderer.set_scene(&scene,None);renderer.set_highlight(&scene,None).unwrap();assert_eq!(renderer.cloud_usage(),[0;5]);assert_eq!(renderer.highlight_bytes(),0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=crate::cloud::large_example().to_proto();for xyz in proto.coords[..64*3].chunks_exact_mut(3) {xyz[2]=0.;}
    let source=Rc::new(session_rust::PointCloud::from_proto(proto));let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Control);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);
    let transform=camera.view_projection().to_f32();let p=camera.view_projection().transform_point(&session_rust::Point::new(0.25,0.25,0.2));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();
    let pixel=renderer.take_pick(transform).unwrap().unwrap().unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();let Part::CloudPoint(id)=target.part else {panic!("Original cloud point identity");};assert!(id>=64);assert!(target.valid(&scene));assert_eq!(renderer.cloud_usage()[2],uploads);
    renderer.request_pick([0.,0.],transform).unwrap();renderer.cancel_pick();assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode,Part};
    let prepared=crate::range_example::prepared();let weak=Rc::downgrade(&prepared.source);
    let mut scene=Scene::empty();let owner=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),12.).unwrap();scene.cloud_controls(Some(owner));
    let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    for (cursor,id) in [([-0.5,0.5],u32::MAX),([0.5,0.5],16_777_217),([-0.5,-0.5],10)] {
        renderer.request_pick(cursor,identity()).unwrap();let hit=finish(&mut renderer).expect("Actual streamed point coverage");let target=renderer.resolve_pick(&scene,hit).unwrap();assert_eq!(target.owner,owner);assert_eq!(target.part,Part::CloudPoint(id));
    }
    let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[2]>160&&p[0]<10));assert!(pixels.chunks_exact(4).any(|p|p[0]>160&&p[2]<10));
    let uploads=renderer.cloud_usage()[2];let copy=scene.copy_row(owner,false).unwrap();scene.translate(copy,[0.2,0.,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],uploads,"Placed owners share original page uploads");
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],false);let held=Rc::downgrade(&prepared.page(2).unwrap());prepared.pages.borrow_mut().clear();assert!(held.upgrade().is_some(),"GPU page owner retains precise pick identities");renderer.set_scene(&scene,None);assert!(held.upgrade().is_none());renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"Evicted detailed points are absent");
    renderer.request_pick([-0.5,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit).unwrap().part,Part::CloudPoint(10));
    scene.clear();renderer.set_scene(&scene,None);drop(prepared);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_definition_instances_batch_one_geometry_keep_integer_picks_and_release_records() {
    let mut original=crate::instance_specimen::message();original.objects.as_mut().unwrap().instances[0].xform=None;original.objects.as_mut().unwrap().instances[1].flags=0;
    original.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.5,0.,0.3).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.5,0.,0.3).to_proto())}];
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&prost::Message::encode_to_vec(&original),"",&session_rust::Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[1].id;let weak=Rc::downgrade(scene.objects()[0].geometry().unwrap());let mut renderer=renderer(&scene);assert_eq!(&renderer.instance_usage()[..3],&[1,2,16760]);let source_uploads=renderer.stats()[0];let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
    for (cursor,id) in [([-0.5,-0.1],a),([0.5,-0.1],b)] {renderer.request_pick(cursor,identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());}
    let mut coloured=original.clone();let first=&mut coloured.objects.as_mut().unwrap().instances[0];first.flags|=session_rust::InstanceRef::FLAG_COLOR;first.color=Some(session_rust::proto::Color {r:0.9,g:0.08,b:0.05,a:0.5,..Default::default()});let mut colour_scene=Scene::empty();crate::scene_document::import(&mut colour_scene,&prost::Message::encode_to_vec(&coloured),"",&session_rust::Xform::identity(),0.).unwrap();let mut colour_renderer=super::pick_gpu_tests::renderer(&colour_scene);let colours=colour_pixels(&mut colour_renderer);assert!(colours.chunks_exact(4).filter(|p|p[0]>p[1].saturating_add(50)&&p[0]>p[2].saturating_add(50)).count()>500);assert!(colours.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)).count()>500);
    let writes=renderer.instance_usage()[3];colour_pixels(&mut renderer);assert_eq!(renderer.instance_usage()[3],writes);scene.translate(a,[0.,0.2,0.]).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.stats()[0],source_uploads);assert!(renderer.instance_usage()[3]>writes);renderer.select_objects(&scene,&[a]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).any(|p|p[0]>140&&p[1]>90&&p[2]<40));
    Rc::make_mut(&mut scene.view).hide(&[b],true);renderer.set_scene(&scene,None);assert_eq!(renderer.instance_usage()[1],1);scene.clear();renderer.set_scene(&scene,None);assert_eq!(&renderer.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_congruent_cad_replay_matches_independent_faces_ink_and_owner_picks() {
    let first=Rc::new(session_rust::BRep::create_block_with_hole(0.7,0.6,0.2,0.12).transformed(&session_rust::Xform::translation(-0.35,-0.3,0.3)));let mut cache=crate::cad_shape_cache::Cache::default();cache.prepare(first.clone()).unwrap();let motion=&session_rust::Xform::translation(0.1,-0.1,0.)*&session_rust::Xform::rotation_z(90.,true);let copy=Rc::new(first.transformed(&motion));let mut shared=Scene::empty();let id=shared.insert_prepared_cad(cache.prepare(copy.clone()).unwrap()).unwrap();assert_eq!(cache.replays,1);let mut alone=Scene::empty();alone.insert_cad(copy.clone()).unwrap();let mut shared_renderer=renderer(&shared);let mut independent_renderer=renderer(&alone);let a=colour_pixels(&mut shared_renderer);let b=colour_pixels(&mut independent_renderer);let changed=a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(a,b)|a[..3].iter().zip(&b[..3]).any(|(a,b)|a.abs_diff(*b)>3)).count();assert!(changed<100,"Congruent faces and source ink differed at {changed} pixels");assert!(a.chunks_exact(4).filter(|p|p[0]<180&&p[1]<180&&p[2]<180).count()>500);
    let mut hits=0;for point in [[-0.1,-0.05],[0.25,-0.05],[0.1,0.15]] {shared_renderer.request_pick(point,identity()).unwrap();independent_renderer.request_pick(point,identity()).unwrap();let actual=finish(&mut shared_renderer);let independent=finish(&mut independent_renderer);assert_eq!(actual,independent);if let Some(actual)=actual {assert_eq!(actual[0],id.value());hits+=1;}}assert!(hits>0,"Actual original CAD ownership must be picked");
    let weak=Rc::downgrade(&copy);drop(copy);shared.clear();alone.clear();shared_renderer.set_scene(&shared,None);independent_renderer.set_scene(&alone,None);drop(cache);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_nested_rotated_and_mirrored_definition_placements_keep_independent_integer_owners() {
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);assert_eq!(&gpu.instance_usage()[..3],&[1,2,16760]);for row in scene.objects() {let point=row.model.transform_point(&session_rust::Point::new((0.123456789012345+0.8)/3.,0.8/3.,0.));gpu.request_pick([point[0] as f32,point[1] as f32],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],row.id.value());}
    let id=scene.objects()[1].id;gpu.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut gpu);assert!(pixels.chunks_exact(4).filter(|p|p[0]>140&&p[1]>90&&p[2]<40).count()>500);let bytes=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::empty();crate::scene_document::open(&mut restored,&bytes).unwrap();for a in scene.objects() {let guid=scene.instances[&a.id].instance.guid();let b=restored.objects().iter().find(|row|restored.instances[&row.id].instance.guid()==guid).unwrap();for (a,b) in a.model.m.iter().zip(&b.model.m) {assert!((a-b).abs()<1e-12,"Canonical placement for {guid}: {a} {b}");}}
}

#[test]
fn actual_sheet_batches_share_source_uploads_keep_world_pens_full_entity_ids_and_locked_occlusion() {
    use crate::pick_sources::{Part,Mode,Target};
    let mut source=crate::sheet_example::source();source.entity_count=u32::MAX;source.source_ids=vec![u32::MAX-1;5];let mut scene=Scene::empty();scene.next_id=16_777_217;let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let copy=scene.copy_row(id,false).unwrap();scene.place_sheet(copy,session_rust::Xform::translation(0.,0.1,0.)).unwrap();let weak=Rc::downgrade(&scene.sheets()[0].prepared);let mut renderer=renderer(&scene);assert_eq!(renderer.sheet_usage()[..2],[2,1]);let uploads=renderer.sheet_usage()[3];renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    renderer.set_pick_mode(Mode::Edge);renderer.request_pick([0.,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:id,part:Part::SheetEntity(u32::MAX-1)}));renderer.request_pick([0.,-0.4],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:copy,part:Part::SheetEntity(u32::MAX-1)}));assert!(hit[1]<u32::MAX-1);
    let pixels=colour_pixels(&mut renderer);let black=(0..240).filter(|y| {let p=&pixels[(*y*320+160)*4..][..4];p[0]<30&&p[1]<30&&p[2]<30}).count();assert!(black>=8&&black<20,"Physical .025mm pens draw separate three-pixel boundaries: {black}");scene.place_sheet(copy,session_rust::Xform::translation(0.,0.2,0.)).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    let front=scene.insert(triangle(0.1)).unwrap();renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);renderer.request_pick([0.,-0.5],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],front.value());Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());Rc::make_mut(&mut scene.view).hide(&[front],true);Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[..2],[0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_evicted_sheet_pages_remain_charged_until_renderer_refresh_cancels_old_picks() {
    let budget=crate::page_cache::Budget::new(256);let source=Rc::new(crate::sheet_range::Source {guid:crate::sheet_example::source().guid,name:"paged drawing".into(),url:"plan.pb".into(),fields:crate::stream::SheetFields {count:2,entities:u32::MAX,..Default::default()}});let stream=crate::sheet_stream::Stream::new(source,budget.clone());let raw=[-0.5f64,0.,0.3,0.5,0.,0.3].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let page=crate::sheet_page::Page::decode(0..1,2,u32::MAX,&raw,None,None,Some(&(u32::MAX-1).to_le_bytes())).unwrap();let held=stream.adopt(page,budget.reserve(192).unwrap()).unwrap();let weak=Rc::downgrade(&held);let mut scene=Scene::empty();let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::streamed(stream.clone()))).unwrap();let mut renderer=renderer(&scene);drop(held);renderer.set_pick_mode(crate::pick_sources::Mode::Edge);renderer.request_pick([0.,0.],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(hit[0],id.value());assert!(renderer.resolve_pick(&scene,hit).is_some());assert_eq!(budget.used(),84);stream.pages.borrow_mut().clear();assert_eq!(budget.used(),84);assert!(budget.reserve(192).is_err());renderer.request_pick([0.,0.],identity()).unwrap();renderer.set_scene(&scene,None);assert!(renderer.take_pick(identity()).is_none());assert_eq!(budget.used(),0);assert!(weak.upgrade().is_none());assert!(renderer.resolve_pick(&scene,hit).is_none());renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_sheet_pen_projection_uses_camera_lens_independently_of_panel_frame_shift() {
    let source=session_rust::proto::Sheet {guid:crate::sheet_example::source().guid,coords:vec![-0.6,0.,0.3,0.6,0.,0.3],widths:vec![0.35],segment_count:1,..Default::default()};let mut scene=Scene::empty();scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let mut renderer=renderer(&scene);let mut camera=crate::camera::Camera::default();let pen_rows=|pixels:Vec<u8>|pixels.chunks_exact(320*4).filter(|row|row[160*4..][..3].iter().all(|v|*v<30)).count();renderer.sheet_camera(&camera);let first=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert!(first>20);camera.frame=[0.,0.,1.,0.2];renderer.sheet_camera(&camera);let shifted=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert_eq!(shifted,first,"A panel shifts the frame without changing the original world pen");
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![10,20,30],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    let source=crate::source_edit::original(&scene,owner).unwrap();let model=session_rust::Xform::translation(0.125,0.25,0.);scene.place(owner,model).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.375,-0.25],identity()).unwrap();
        let pixel=finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))}));
        scene.show_controls(false);renderer.set_scene(&scene,None);renderer.request_pick([-0.375,-0.25],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.show_controls(true);
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.1,0.1,0.25]),(30,[0.8,0.1,0.25]),(40,[0.1,0.8,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![20,30,40],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    assert!(scene.control_markers().iter().all(|m|m.display.diameter==7.&&m.display.colour==[0.15,0.35,0.9,1.]));
    let source=crate::source_edit::original(&scene,owner).unwrap();let target=crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);
        let pixels=colour_pixels(&mut renderer);let center=(180*320+80)*4;
        for (actual,expected) in pixels[center..center+4].iter().zip([38_i16,89,230,255]){assert!((*actual as i16-expected).abs()<=2,"Actual blue control pixel {:?}",&pixels[center..center+4]);}
        let coverage:f32=(60..100).map(|x|(255.-pixels[(180*320+x)*4]as f32)/217.).sum();
        assert!((coverage/density-7.).abs()<0.6,"Seven CSS pixel row coverage at density {density}: {coverage}");
        renderer.set_highlight(&scene,Some(target)).unwrap();let selected=colour_pixels(&mut renderer);
        assert_eq!(&selected[center..center+4],&[255,255,0,255]);
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor=crate::editor::Editor::default();editor.apply(crate::editor::Action::Close).unwrap();editor.apply(crate::editor::Action::AddCurve).unwrap();editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner=editor.scene.paths()[0].id;let nets=editor.scene.control_nets();assert_eq!(nets.len(),2);let markers=editor.scene.control_markers();
    for(parent,line)in &nets{assert_eq!(*parent,owner);assert!(markers.iter().any(|m|m.display.center==line.start));assert!(markers.iter().any(|m|m.display.center==line.end));assert_eq!(line.width,1.);}
    let source=crate::source_edit::original(&editor.scene,owner).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&editor.scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(crate::pick_sources::Mode::Object);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([owner.value(),0]));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}

#[test]
fn actual_edited_element_controls_move_their_integer_pixels_and_undo() {
    use crate::pick_sources::{Mode,Part,Target};
    let mut mesh=session_rust::Mesh::new();for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut element=session_rust::Element::new("edited control pixel");element.set_geometry(mesh);let mut base=Scene::empty();let owner=crate::element_original::insert(&mut base,std::rc::Rc::new(element)).unwrap();base.owner_controls(owner).unwrap();let original=crate::source_edit::original(&base,owner).unwrap();let target=Target{owner,part:Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut scene=base.clone();let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(session_rust::Xform::translation(0.25,0.,0.)).unwrap();renderer.set_scene(&preview.scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&preview.scene,pixel),Some(target));renderer.request_pick([-0.5,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
        let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());renderer.set_scene(&scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        assert!(history.undo(&mut scene));renderer.set_scene(&scene,None);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));assert!(crate::source_edit::same(&original,&crate::source_edit::original(&scene,owner).unwrap()));
    }
}

#[test]
fn actual_pulled_source_normals_keep_shared_reflected_placements_and_original_picks() {
    let mut source=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.2,-0.4,0.3]),(20,[0.2,-0.4,0.3]),(30,[0.,0.4,0.3])]{source.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));for(name,value)in[("nx",4.),("ny",0.),("nz",3.)]{source.set_vertex_attribute(key,name,value);}}
    source.add_face(vec![30,10,20],Some(37));let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let b=scene.copy_row(a,false).unwrap();
    scene.place(a,session_rust::Xform::translation(-0.5,0.,0.)).unwrap();let mut reflected=session_rust::Xform::identity();reflected.m[0]=-1.;reflected.m[12]=0.5;scene.place(b,reflected).unwrap();assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),scene.objects()[1].geometry().unwrap()));
    let mut gpu=renderer(&scene);let usage=gpu.stats();
    for density in [1.,2.] {gpu.set_density(density);let pixels=colour_pixels_display(&mut gpu,identity(),true);
        for(x,expected,owner,position)in[(80,[230u8,128,204],a,-0.5),(240,[230u8,128,51],b,0.5)]{let pixel=&pixels[(132*320+x)*4..][..3];for(actual,expected)in pixel.iter().zip(expected){assert!(actual.abs_diff(expected)<=2,"Actual placed source normal: {pixel:?}");}gpu.request_pick([position,-0.1],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],owner.value());}
        assert_eq!(&gpu.stats()[..2],&usage[..2]);
    }
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(scene.objects()[1].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_copied_originals_share_instance_records_keep_original_picks_and_return_to_ordinary_drawing() {
 let mut scene=Scene::empty();scene.next_id=16_777_217;let a=scene.insert(PreparedMesh::triangle([[-0.18,-0.2,0.4],[0.18,-0.2,0.4],[0.,0.2,0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(a,[-0.45,0.,0.]).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(b,[0.9,0.,0.]).unwrap();assert!(scene.instances.is_empty());assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));let original=scene.objects()[0].geometry().unwrap().to_proto();let weak=Rc::downgrade(&scene.objects()[0].mesh);
 {let classify=|limit,declared|crate::instance_gpu::batch_rows(scene.objects().iter().map(|row|(row.id,Rc::as_ptr(&row.mesh)as usize,declared==Some(row.id))),limit);assert!(classify(1,None).is_empty());assert_eq!(classify(2,None).len(),2);assert_eq!(classify(1,Some(b)),[b].into_iter().collect());}
 let mut gpu=renderer(&scene);gpu.set_opacity(1.);assert_eq!(&gpu.instance_usage()[..3],&[1,2,16760]);let uploads=gpu.stats()[0];let plain=colour_pixels(&mut gpu);let mut blue=[0;2];for line in plain.chunks_exact(320*4){for(x,p)in line.chunks_exact(4).enumerate(){if p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10){blue[x/160]+=1;}}}assert!(blue.into_iter().all(|n|n>100));
 for(id,x)in[(a,-0.45),(b,0.45)]{gpu.request_pick([x,0.],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],id.value());}
 gpu.select_objects(&scene,&[b]).unwrap();let selected=colour_pixels(&mut gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>140&&p[1]>90&&p[2]<40));gpu.select_objects(&scene,&[]).unwrap();assert_eq!(colour_pixels(&mut gpu),plain);
 Rc::make_mut(&mut scene.view).hide(&[b],true);gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,16760]);gpu.request_pick([-0.45,0.],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],a.value());
 Rc::make_mut(&mut scene.view).hide(&[b],false);gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[1,2,16760]);assert_eq!(colour_pixels(&mut gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert!(scene.instances.is_empty());for row in scene.objects(){assert_eq!(row.geometry().unwrap().to_proto(),original);}
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-18.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/snap_tests.rs`

Keep the original Top pose for exact grid-rounding and unrounded typed-coordinate assertions.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{snap::{Settings, Setting, Kind}, snap_index::Index, snap_math::Screen, scene::Scene};
use std::rc::Rc;

fn screen() -> Screen { Screen {matrix:session_rust::Xform::identity().m,size:[200.,100.],origin:[0.;3]} }
fn ray(p: [f64; 2]) -> crate::camera::Ray { crate::camera::Ray { origin:session_rust::Point::new(p[0],p[1],-1.),direction:session_rust::Vector::z_axis(),max_distance:2. } }
fn empty() -> Scene { let mut scene=Scene::demo(); scene.clear(); scene }
fn line(scene: &mut Scene, a: [f64; 3], b: [f64; 3]) -> crate::scene::ObjectId {
    scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(a[0],a[1],a[2],b[0],b[1],b[2]))).unwrap()).unwrap()
}

#[test]
fn current_grid_default_and_independent_switches_validate_before_mutation() {
    let mut settings=Settings::default(); assert!(settings.objects && settings.grid); assert_eq!(settings.step,1000.);
    assert_eq!(settings.modes,crate::snap::END|crate::snap::NEAR|crate::snap::MID|crate::snap::CENTER);
    settings.apply(Setting::Objects(Some(false))).unwrap(); assert!(settings.grid && !settings.objects);
    settings.apply(Setting::Grid(Some(false))).unwrap(); settings.apply(Setting::Spacing(50.)).unwrap();
    assert!(settings.grid && !settings.objects); assert_eq!(settings.step,50.); let before=settings;
    for value in [0.,-1.,f64::NAN,f64::INFINITY] { assert!(settings.apply(Setting::Spacing(value)).is_err()); assert_eq!(settings,before); }
    assert!(settings.apply(Setting::Mode(32)).is_err()); assert_eq!(settings,before);
    settings.apply(Setting::Mode(crate::snap::PERP)).unwrap(); assert_ne!(settings.modes & crate::snap::PERP,0);
}

#[test]
fn grid_rounding_preserves_height_and_never_creates_nonfinite_points() {
    assert_eq!(crate::snap_math::grid([149.,51.,33.3],[0.;3],[[1.,0.,0.],[0.,1.,0.]],100.).unwrap(),[100.,100.,33.3]);
    assert_eq!(crate::snap_math::grid([149.,51.,33.3],[0.;3],[[0.,1.,0.],[0.,0.,1.]],10.).unwrap(),[149.,50.,30.]);
    assert_eq!(crate::snap_math::grid([-150.,150.,0.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],100.).unwrap(),[-200.,200.,0.]);
    assert!(crate::snap_math::grid([1.,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],0.).is_none());
    assert!(crate::snap_math::grid([f64::NAN,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],50.).is_none());
    assert_eq!(crate::snap_math::grid([1e12,2.,3.],[0.;3],[[1.,0.,0.],[0.,1.,0.]],1e-320).unwrap(),[1e12,2.,3.]);
}

#[test]
fn priority_wins_inside_twelve_css_pixels_then_distance_breaks_same_kind_ties() {
    let mut scene=empty(); let owner=line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]); let mut index=Index::new(&scene);
    let hit=index.query(screen(),&ray([0.49,0.]),[149.,50.],crate::snap::END|crate::snap::NEAR,None).unwrap();
    assert_eq!(hit.kind,Kind::End); assert_eq!(hit.point,[0.5,0.,0.2]); assert_eq!(hit.owner,Some(owner));
    let near=index.query(screen(),&ray([0.49,0.]),[149.,50.],crate::snap::NEAR,None).unwrap(); assert_eq!(near.kind,Kind::Near); assert!((near.point[0]-0.49).abs()<1e-12);
    let mid=index.query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::MID,None).unwrap(); assert_eq!(mid.point,[0.,0.,0.2]);
    assert!(index.query(screen(),&ray([0.7,0.]),[170.,50.],crate::snap::END|crate::snap::NEAR,None).is_none());
}

#[test]
fn placed_original_endpoints_curve_domain_midpoints_and_hidden_sources_are_distinct() {
    let mut scene=empty(); let id=line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]);
    let shift=0.123456789012345; scene.place_line(id,session_rust::Xform::translation(shift,0.,0.)).unwrap();
    let p=[0.5+shift,0.,0.2]; let pixel=screen().point(p).unwrap(); let mut index=Index::new(&scene);
    assert_eq!(index.query(screen(),&ray([p[0],p[1]]),pixel,crate::snap::END,None).unwrap().point,p);
    let mut hidden=session_rust::Point::new(0.,0.8,0.2); hidden.is_visible=false; scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(hidden)).unwrap()).unwrap();
    let mut index=Index::new(&scene); assert!(index.query(screen(),&ray([0.,0.8]),[100.,10.],crate::snap::END,None).is_none());
    let mut curve=session_rust::NurbsCurve::create(false,2,&[session_rust::Point::new(-0.5,0.,0.2),session_rust::Point::new(0.,0.6,0.2),session_rust::Point::new(0.5,0.,0.2)]);
    curve.is_locked=true; scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap()).unwrap();
    let mut index=Index::new(&scene); let mid=index.query(screen(),&ray([0.,0.3]),[100.,35.],crate::snap::MID,None).unwrap();
    assert_eq!(mid.kind,Kind::Mid); assert!((mid.point[1]-0.3).abs()<1e-12); assert!((mid.point[1]-0.6).abs()>0.1);
    assert!(scene.pick_id(mid.owner.unwrap().value()).is_none(),"Read-only geometry can supply a snap without becoming an editable selection");
}

#[test]
fn perpendicular_uses_the_previous_point_and_excludes_feet_outside_the_segment() {
    let mut scene=empty(); line(&mut scene,[-0.5,0.,0.2],[0.5,0.,0.2]); let mut index=Index::new(&scene);
    let hit=index.query(screen(),&ray([0.,0.01]),[100.,49.5],crate::snap::PERP|crate::snap::NEAR,Some([0.,0.5,0.2])).unwrap();
    assert_eq!(hit.kind,Kind::Perp); assert_eq!(hit.point,[0.,0.,0.2]);
    assert!(crate::snap_math::perpendicular([-0.5,0.,0.2],[0.5,0.,0.2],[1.,0.5,0.2]).is_none());
    assert!(crate::snap_math::perpendicular([0.;3],[0.;3],[1.;3]).is_none());
}

#[test]
fn screen_cells_keep_queries_local_and_rebuild_only_for_a_changed_view() {
    let mut scene=empty();
    for i in 0..2000 { scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(i as f64*2.,0.,0.2))).unwrap()).unwrap(); }
    let weak=Rc::downgrade(&scene.points()[0].prepared.source); let mut index=Index::new(&scene);
    index.query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,1); assert!(index.visits<5);
    index.query(screen(),&ray([0.01,0.]),[101.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,1);
    let mut changed=screen(); changed.size=[300.,100.]; index.query(changed,&ray([0.,0.]),[150.,50.],crate::snap::END,None).unwrap(); assert_eq!(index.builds,2);
    scene.clear(); assert!(weak.upgrade().is_none(),"Derived snap coordinates do not retain original source owners");
    let mut bins=crate::snap_bins::Bins::default(); bins.insert(1,Some([[0.,0.],[128.,64.]])); bins.insert(2,Some([[-1e50,-1e50],[1e50,1e50]]));
    assert_eq!(bins.near([127.,63.],12.),vec![1,2]); assert_eq!(bins.near([1e6,1e6],12.),vec![2]);
}

#[test]
fn snapping_settings_preserve_a_draft_and_typed_coordinates_bypass_grid_rounding() {
    use crate::editor::{Action,Editor}; let mut editor=Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap(); editor.apply(Action::ToolPoint([0.123456789012345,0.25,0.])).unwrap();
    let first=editor.draft.as_ref().unwrap().points.clone(); editor.apply(Action::Snap(Setting::Objects(Some(false)))).unwrap();
    editor.apply(Action::ToolScreen { screen:[0.25,0.25],size:[400.,300.],commit:false }).unwrap();
    assert_eq!(editor.draft.as_ref().unwrap().cursor,Some([0.,0.,0.])); assert_eq!(editor.draft.as_ref().unwrap().snapped,Some(Kind::Grid));
    editor.apply(Action::Snap(Setting::Spacing(50.))).unwrap(); assert_eq!(editor.draft.as_ref().unwrap().points,first);
    editor.apply(Action::Snap(Setting::Grid(Some(false)))).unwrap();
    editor.apply(Action::ToolScreen { screen:[0.25,0.25],size:[400.,300.],commit:false }).unwrap();
    assert_ne!(editor.draft.as_ref().unwrap().cursor,Some([0.,0.,0.])); assert_eq!(editor.draft.as_ref().unwrap().snapped,None);
    editor.apply(Action::ToolPoint([0.987654321012345,0.5,0.])).unwrap(); assert_eq!(editor.scene.lines()[0].prepared.source.start()[0],first[0][0]);
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.lines().is_empty());
}

#[test]
fn original_transactions_invalidate_snap_positions_without_creating_snap_history() {
    use crate::editor::{Action,Editor}; let mut editor=Editor::default(); editor.apply(Action::Close).unwrap();
    editor.camera.distance=4.; editor.camera.aspect=2.; editor.apply(Action::AddLine).unwrap(); editor.apply(Action::SelectAll).unwrap();
    editor.apply(Action::BeginTool(crate::tool::Shape::Point)).unwrap(); editor.apply(Action::Snap(Setting::Grid(Some(false)))).unwrap(); editor.snapping.modes=crate::snap::END;
    let target=|editor:&Editor,p:[f64;3]| { let p=editor.camera.view_projection().transform_point(&session_rust::Point::new(p[0],p[1],p[2])); [p[0] as f32,p[1] as f32] };
    let old=target(&editor,[0.6,0.,0.]); assert_eq!(editor.snap_point(old,[400.,200.]).unwrap().point,[0.6,0.,0.]);
    editor.apply(Action::Translate([0.5,0.,0.])).unwrap(); let current=target(&editor,[1.1,0.,0.]);
    assert_eq!(editor.snap_point(current,[400.,200.]).unwrap().point,[1.1,0.,0.]); assert!(editor.snap_point(old,[400.,200.]).unwrap().owner.is_none());
    editor.apply(Action::CancelTool).unwrap(); editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.lines()[0].endpoints()[1][0],0.6);
    for text in ["Grid Snap 0","Grid Snap NaN","Grid Snap -1","Grid Snap 1 2","Snap Maybe"] { assert!(crate::commands::parse(text).is_err(),"{text}"); }
    assert!(matches!(crate::commands::parse("GridSnap 50").unwrap(),crate::commands::Command::Editor(Action::Snap(Setting::Spacing(50.)))));
}

#[test]
fn a_closed_source_loop_offers_its_center_with_a_small_closure_tolerance() {
    let mut scene=empty(); let points=vec![session_rust::Point::new(-0.5,-0.5,0.2),session_rust::Point::new(0.5,-0.5,0.2),session_rust::Point::new(0.5,0.5,0.2),session_rust::Point::new(-0.5,0.5,0.2),session_rust::Point::new(-0.5+1e-10,-0.5,0.2)];
    scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points))).unwrap()).unwrap();
    let hit=Index::new(&scene).query(screen(),&ray([0.,0.]),[100.,50.],crate::snap::CENTER,None).unwrap();
    assert_eq!(hit.kind,Kind::Center); assert_eq!(hit.point,[0.,0.,0.2]);
}

#[test]
fn a_tilted_grid_counts_from_its_origin_and_preserves_local_height() {
    let q=2.0_f64.sqrt().recip(); let origin=[10.,0.,0.]; let axes=[[0.,q,q],[-1.,0.,0.]];
    let world=|u:f64,v:f64,w:f64| [10.-v, q*(u-w),q*(u+w)];
    let point=crate::snap_math::grid(world(1490.,520.,7.),origin,axes,1000.).unwrap();
    let expected=world(1000.,1000.,7.);
    assert!((0..3).all(|i| (point[i]-expected[i]).abs()<1e-9));
    assert!(crate::snap_math::grid(world(1.,2.,3.),origin,[[1.,0.,0.],[1.,0.,0.]],10.).is_none());
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-19.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 17. `src/widget_tests.rs`

Keep the original three-unit isometric widget fixture, exact hover pixels, physical tile reuse and no-pick-owner checks.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,renderer::Renderer,viewport::Viewport};
fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, flags: Default::default(), memory_budget_thresholds: Default::default(), backend_options: Default::default(), display: None });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).expect("Actual GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 }); renderer
}

pub(crate) fn pixels(renderer: &mut Renderer, transform:&[f32;16], size:[u32;2]) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), transform, false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: (u64::from(size[0])*4).div_ceil(256)*256*u64::from(size[1]),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some((size[0]*4).div_ceil(256)*256), rows_per_image: Some(size[1]) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let mapped=bytes.slice(..).get_mapped_range(); let pitch=(size[0]*4).div_ceil(256)*256; let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size[0] as usize*4].iter().copied()).collect();drop(mapped); bytes.unmap(); result
}

#[test]
fn all_handles_have_finite_twenty_byte_vertices_and_layouts_keep_css_size() {
    let vertices=crate::widget_mesh::vertices();assert!(vertices.len()<20_000);let mut handles=[false;10];
    for vertex in vertices {handles[vertex.handle as usize]=true;assert_eq!(vertex.bytes().len(),20);assert!(vertex.position.iter().all(|v|v.is_finite() && v.abs()<=98.));}assert!(handles.into_iter().all(|v|v));
    let mut camera=crate::camera::Camera::default();camera.isometric();
    for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic] {camera.projection=projection;
        for distance in [3.,9.,30.] {camera.distance=distance;let matrix=camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,[0.;3],480.).unwrap();
            let centre=matrix.transform_point(&session_rust::Point::new(0.,0.,0.));let world=camera.orientation.rotate_vector(session_rust::Vector::z_axis())*(96.*scale);let tip=matrix.transform_point(&session_rust::Point::new(world[0],world[1],world[2]));assert!(((tip[1]-centre[1])*240.-96.).abs()<1e-9);
            for density in [1u32,2] {let layout=crate::widget_layout::layout(&matrix,[0.;3],scale,[640*density,480*density],-1.).unwrap();assert!(layout.extent.into_iter().all(|v|v<=1024 && v%64==0));}
        }
    }
    assert!(crate::widget_layout::world_per_pixel(&session_rust::Xform::identity(),[0.;3],0.).is_none());
    let mut invalid=session_rust::Xform::identity();invalid.m[15]=-1.;assert!(crate::widget_layout::layout(&invalid,[0.;3],1.,[640,480],-1.).is_none());
}

#[test]
fn actual_widget_gpu_draws_over_geometry_reuses_tile_and_never_creates_pick_owners() {
    let mut scene=Scene::demo();scene.clear();let mut renderer=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=320./240.;
    for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic] {camera.projection=projection;
        let inverse=camera.view_projection().inverse().unwrap();
        let points=[[-1.,-1.,0.001],[3.,-1.,0.001],[-1.,3.,0.001]].map(|p| {let p=inverse.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]});
        scene.clear();scene.insert(crate::prepared::PreparedMesh::triangle(points,[0.5;3]).unwrap()).unwrap();renderer.set_scene(&scene,None);
        let stats=renderer.stats();let pick=renderer.pick_usage();
        for density in [1u32,2] {let size=[320*density,240*density];renderer.resize(Viewport {width:size[0],height:size[1]});renderer.set_density(density as f32);
            renderer.set_gizmo(&camera,Some([0.;3]),-1.);let usage=renderer.gizmo_usage();assert!(usage[0]<400_100 && usage[1]<=1024*1024*36 && usage[1]>0);
            let image=pixels(&mut renderer,&camera.uniform(),size);let mut colours=[0;3];
            for p in image.chunks_exact(4) {for i in 0..3 {if p[i]>140 && p[(i+1)%3]<130 && p[(i+2)%3]<130 {colours[i]+=1;}}}
            assert!(colours.into_iter().all(|n|n>100),"Three visible handle colours: {colours:?}");
            renderer.set_gizmo(&camera,Some([0.;3]),0.);assert_eq!(renderer.gizmo_usage()[2],usage[2],"Hover reuses the tile");let active=pixels(&mut renderer,&camera.uniform(),size);assert_ne!(active,image);
            let orange=active.chunks_exact(4).filter(|p|p[0]>180 && p[1]>100 && p[1]<210 && p[2]<90).count();assert!(orange>100);
            assert_eq!(renderer.stats(),stats);assert_eq!(renderer.pick_usage(),pick);
            renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());
        }
    }
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzza-match-physical-camera-defaults-and-limits-20.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 21.5.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

View Reset uses the current physical isometric pose. Zoom and tiny-object Fit retain unit-independent physical limits.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Match physical camera defaults and limits; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzzza-match-physical-camera-defaults-and-limits-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

Reuse the isometric orientation already taught for named views. The maintained default eye distance is three metres, so convert it to the file unit instead of using three scene units. A fresh camera has no scene extent. Zoom and tiny-object Fit use physical distance bounds rather than bounds tied to the last fitted object. The generic zoom factor API stays the same; command and wheel step mapping remain separate input work.

File unit → physical default and limits → existing camera and named-view owners.



Why must zoom limits be independent of the last fitted radius?

A fitted object describes the document, not the permitted camera travel. Radius-based limits can stop navigation before the current viewer would, and change when another object is fitted.

Study estimate, including typing and experiments: 0.3–0.6 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create a Box with 1000-unit sides and use View Reset. Compare its isometric view with Fit. Zoom repeatedly in and out, then Fit a tiny original. Navigation must not stop merely because the earlier fitted object was small.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 21.5.1
npm --prefix ../session_tests run course -- save 21.5.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Physical defaults and limits are connected. The independent maintained-camera snapshot is optional native authoring evidence and is absent from the browser application. Grid scale/range, complete camera parity and later completion items remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 21.5.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
