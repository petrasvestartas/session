# 2.6 · Perspective picking and orbit navigation

**Goal:** Orbit and tilt a perspective camera while keeping its target in place. · **Steps:** 2 · **Source:** `src/camera.rs,src/engine/gpu/render.rs,src/shaders/scene.wgsl`

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Perspective Frustum (w = -z_eye) vs Ortho Box (w = 1)</text>
  <circle cx="58" cy="102" r="4.5" fill="#111111"/>
  <polygon points="58,102 158,52 158,152" fill="#fef9c3" fill-opacity="0.45" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <rect x="184" y="56" width="102" height="92" fill="#dbeafe" fill-opacity="0.45" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="184" y1="78" x2="286" y2="78" stroke="#2563eb" stroke-width="1.2" marker-end="url(#arrB)"/>
  <line x1="184" y1="126" x2="286" y2="126" stroke="#2563eb" stroke-width="1.2" marker-end="url(#arrB)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Foreshortening x_ndc = x/(z*tan(fov/2)) vs Parallel x/s</text>
  <polygon points="432,140 468,62 544,82 518,140" fill="#fce7f3" stroke="#db2777" stroke-width="1.75"/>
  <rect x="582" y="66" width="96" height="72" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="582" y1="66" x2="678" y2="66" stroke="#16a34a" stroke-width="2"/>
  <line x1="582" y1="138" x2="678" y2="138" stroke="#16a34a" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">True-Scale Parallel CAD Elevation View</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="844" y="62" width="82" height="82" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="844" y1="103" x2="926" y2="103" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="992" y="106" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Top / Front</text>
</svg>
</div>

## <a id="checkpoint-15-perspective"></a>1. Pick the nearest surface through the view

**What this code does:** View the scene in perspective and select surfaces with a ray that agrees with the camera.

### 1. `src/picking.rs`

Replace the flat coverage query with ray–triangle intersections and update the visibility tests for the new camera.

**Locate this existing block:**

```rust
use crate::scene::{ObjectId, Scene};

pub fn pick(scene: &Scene, point: [f32; 2]) -> Option<ObjectId> {
    let mut nearest = 1.0;
    let mut hit = None;
    for object in scene.objects() {
        let vertices = object.mesh.vertices();
        for triangle in object.mesh.indices().chunks_exact(3) {
            let a = vertices[triangle[0] as usize];
            let b = vertices[triangle[1] as usize];
            let c = vertices[triangle[2] as usize];
            if let Some(depth) = triangle_depth(point, a, b, c) {
                if depth >= 0.0 && depth < nearest {
                    nearest = depth;
                    hit = Some(object.id);
                }
            }
        }
    }
    hit
}

fn triangle_depth(p: [f32; 2], a: [f32; 6], b: [f32; 6], c: [f32; 6]) -> Option<f32> {
    let cross = |u: [f32; 2], v: [f32; 2]| u[0] * v[1] - u[1] * v[0];
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ac = [c[0] - a[0], c[1] - a[1]];
    let ap = [p[0] - a[0], p[1] - a[1]];
    let area = cross(ab, ac);
    if area.abs() < 1.0e-8 {
        return None;
    }
    let u = cross(ap, ac) / area;
    let v = cross(ab, ap) / area;
    if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
        Some((1.0 - u - v) * a[2] + u * b[2] + v * c[2])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;

    #[test]
    fn overlap_chooses_the_visible_object_and_deletion_reveals_the_next() {
        let mut scene = Scene::demo();
        let near = scene.objects()[0].id;
        let far = scene.objects()[1].id;
        assert_eq!(pick(&scene, [0.0, 0.0]), Some(near));
        assert_eq!(pick(&scene, [0.4, 0.0]), Some(far));
        scene.remove(near);
        assert_eq!(pick(&scene, [0.0, 0.0]), Some(far));
        assert_eq!(pick(&scene, [-0.9, -0.9]), None);
    }

    #[test]
    fn picking_follows_a_panned_zoomed_and_rotated_camera() {
        let scene = Scene::demo();
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.zoom(0.5);
        camera.rotate(0.7);
        let world = [0.4, 0.0];
        let m = camera.uniform();
        let screen = [
            m[0] * world[0] + m[4] * world[1] + m[12],
            m[1] * world[0] + m[5] * world[1] + m[13],
        ];
        let restored = camera.world_from_screen(screen);
        assert!((restored[0] - world[0]).abs() < 1.0e-6);
        assert!((restored[1] - world[1]).abs() < 1.0e-6);
        assert_eq!(
            pick(&scene, restored),
            Some(scene.objects()[1].id)
        );
    }

    #[test]
    fn an_edge_on_or_invalid_triangle_cannot_be_picked() {
        assert_eq!(
            triangle_depth([0.0, 0.0], [0.0; 6], [0.0; 6], [0.0; 6]),
            None
        );
        assert_eq!(pick(&Scene::demo(), [f32::NAN, 0.0]), None);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{
    camera::Ray,
    scene::{ObjectId, Scene},
};
use session_rust::{Point, Vector};

pub fn pick(scene: &Scene, ray: &Ray) -> Option<ObjectId> {
    let mut nearest = ray.max_distance;
    let mut hit = None;
    for object in scene.objects() {
        let vertices = object.mesh.vertices();
        for triangle in object.mesh.indices().chunks_exact(3) {
            let a = vertices[triangle[0] as usize];
            let b = vertices[triangle[1] as usize];
            let c = vertices[triangle[2] as usize];
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

fn triangle_distance(ray: &Ray, a: [f32; 6], b: [f32; 6], c: [f32; 6]) -> Option<f64> {
    let edge1 = Vector::new(
        (b[0] - a[0]) as f64,
        (b[1] - a[1]) as f64,
        (b[2] - a[2]) as f64,
    );
    let edge2 = Vector::new(
        (c[0] - a[0]) as f64,
        (c[1] - a[1]) as f64,
        (c[2] - a[2]) as f64,
    );
    let p = ray.direction.cross(&edge2);
    let determinant = edge1.dot(&p);
    if determinant.abs() < 1.0e-10 {
        return None;
    }
    let origin = &ray.origin - &Point::new(a[0] as f64, a[1] as f64, a[2] as f64);
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
        let short = Ray {
            max_distance: 0.1,
            ..ray
        };
        assert_eq!(pick(&scene, &short), None);
    }

    #[test]
    fn picking_follows_the_camera_and_ignores_triangles_behind_the_ray() {
        let scene = Scene::demo();
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.rotate(0.7);
        camera.zoom(0.5);
        let screen = camera
            .view_projection()
            .transform_point(&Point::new(0.4, 0.0, 0.75));
        let ray = camera
            .ray([screen[0] as f32, screen[1] as f32])
            .unwrap();
        assert_eq!(
            pick(&scene, &ray),
            Some(scene.objects()[1].id)
        );
        let away = Ray {
            origin: Point::new(0.0, 0.0, 2.0),
            direction: Vector::z_axis(),
            max_distance: 10.0,
        };
        assert_eq!(pick(&scene, &away), None);
    }
```

### 2. `src/lib.rs`

Register the camera-ray query now that its input is available.

**Locate this existing block:**

```rust
pub mod mesh;
pub mod renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh;
pub mod picking;
pub mod renderer;
```

### 3. `src/browser.rs`

Convert a canvas click to a camera ray and highlight its nearest valid object. Report that perspective picking is ready.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective drawing uses the current camera.");
    Ok(())
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective and picking use the same camera.");
    Ok(())
```

**Locate this block:**

```rust
                match line.as_str() {
                    "canvas" => return,
                    "example triangle" => {
                        history.edit(&mut scene, Scene::toggle_extra);
                        selected = selected.filter(|id| scene.contains(*id));
```

Replace it with:

```rust
                match line.as_str() {
                    "canvas" => {
                        let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                            return;
                        };
                        let rect = input_canvas.get_bounding_client_rect();
                        if rect.width() <= 0.0 || rect.height() <= 0.0 {
                            return;
                        }
                        let screen = [
                            (2.0 * (event.client_x() as f64 - rect.left())
                                / rect.width()
                                - 1.0) as f32,
                            (1.0 - 2.0 * (event.client_y() as f64 - rect.top())
                                / rect.height()) as f32,
                        ];
                        selected = camera
                            .ray(screen)
                            .and_then(|ray| crate::picking::pick(&scene, &ray));
                        renderer.set_scene(&scene, selected);
                    }
                    "example triangle" => {
                        history.edit(&mut scene, Scene::toggle_extra);
                        selected = selected.filter(|id| scene.contains(*id));
```

## <a id="checkpoint-16-orbit"></a>2. Walk around the model

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D World Space (x, y, z, 1)</text>
  <line x1="110" y1="136" x2="250" y2="136" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="140" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">+X</text>
  <line x1="110" y1="136" x2="110" y2="50" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="110" y="44" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">+Z</text>
  <line x1="110" y1="136" x2="62" y2="164" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <polygon points="156,118 216,96 184,58" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <circle cx="80" cy="76" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <line x1="85" y1="78" x2="176" y2="94" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3" marker-end="url(#arr)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="80" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: clip = M_proj * M_view * p</text>
  <polygon points="444,82 516,50 516,158 444,126" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="444" y1="82" x2="660" y2="46" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="444" y1="126" x2="660" y2="162" stroke="#64748b" stroke-width="1.25" stroke-dasharray="3 3"/>
  <polygon points="600,56 668,46 668,162 600,148" fill="none" stroke="#111111" stroke-width="1.5"/>
  <polygon points="536,84 584,72 562,130" fill="#bfdbfe" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="164" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">ndc = clip.xyz / clip.w</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Perspective / Orbit Frame</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 858 124 A 78 28 0 1 1 1010 124" fill="none" stroke="#2563eb" stroke-width="1.75" stroke-dasharray="4 3" marker-end="url(#arrB)"/>
  <polygon points="934,60 886,128 986,136" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="934" cy="112" r="4" fill="#dc2626"/>
</svg>
</div>

**What this code does:** Orbit and tilt a perspective camera while keeping its target in place.

### 1. `src/camera.rs`

Use the kernel’s rotation representation alongside its points, vectors and matrices. Store a 3D target and orientation instead of a flat centre and one angle. Keep the same initial eye and up directions as the perspective lesson. Move the target in the camera’s current right/up plane, including its world-z component. Add world-up orbit and a named pose. Keep rotate as the existing Turn view action. Calculate eye and up from the orientation, then reuse the existing view and projection construction. Extend the target-centering test to include a 3D orbit. Check the rotation invariant after many small updates, not just one convenient angle.

**Locate this existing block:**

```rust
use session_rust::{Point, Vector, Xform};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub max_distance: f64,
}

pub struct Camera {
    pub center: [f64; 2],
    pub distance: f64,
    pub angle: f64,
    pub aspect: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            distance: 3.0,
            angle: 0.0,
            aspect: 640.0 / 480.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (sin, cos) = self.angle.sin_cos();
        self.center[0] += dx as f64 * cos - dy as f64 * sin;
        self.center[1] += dx as f64 * sin + dy as f64 * cos;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.distance = (self.distance / factor as f64).clamp(0.2, 50.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians as f64;
    }

    pub fn view_projection(&self) -> Xform {
        let eye = Point::new(self.center[0], self.center[1], self.distance);
        let target = Point::new(self.center[0], self.center[1], 0.0);
        let (sin, cos) = self.angle.sin_cos();
        let up = Vector::new(-sin, cos, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let projection = Xform::perspective(
            60.0_f64.to_radians(),
            self.aspect,
            0.01,
            100.0,
        );
        projection * view
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
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
            origin,
            direction: delta.normalized(),
            max_distance,
        })
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
        let target = Point::new(camera.center[0], camera.center[1], 0.0);
        let screen = camera
            .view_projection()
            .transform_point(&target);
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub max_distance: f64,
}

pub struct Camera {
    pub target: [f64; 3],
    pub distance: f64,
    pub orientation: Quaternion,
    pub aspect: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: [0.0; 3],
            distance: 3.0,
            orientation: Quaternion::from_axis_angle(
                Vector::x_axis(),
                -std::f64::consts::FRAC_PI_2,
            ),
            aspect: 640.0 / 480.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += dx as f64 * right[i] + dy as f64 * up[i];
        }
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.distance = (self.distance / factor as f64).clamp(0.2, 50.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.orbit(radians as f64, 0.0);
    }

    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let turn = Quaternion::from_axis_angle(Vector::z_axis(), yaw);
        let tilt = Quaternion::from_axis_angle(right, pitch);
        self.orientation = (turn * (tilt * self.orientation.duplicate())).normalized();
    }

    pub fn isometric(&mut self) {
        let turn = Quaternion::from_axis_angle(
            Vector::z_axis(),
            -std::f64::consts::FRAC_PI_6,
        );
        let right = turn.rotate_vector(Vector::x_axis());
        let tilt = Quaternion::from_axis_angle(right, -std::f64::consts::FRAC_PI_6);
        self.orientation = (tilt * turn).normalized();
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
        let projection = Xform::perspective(
            60.0_f64.to_radians(),
            self.aspect,
            0.01,
            100.0,
        );
        projection * view
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
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
            origin,
            direction: delta.normalized(),
            max_distance,
        })
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
        let target = Point::new(
            camera.target[0],
            camera.target[1],
            camera.target[2],
        );
        let screen = camera
            .view_projection()
            .transform_point(&target);
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }

    #[test]
    fn repeated_orbit_keeps_the_camera_axes_orthonormal() {
        let mut camera = Camera::default();
        for _ in 0..1_000 {
            camera.orbit(0.01, 0.02);
        }
        let right = camera
            .orientation
            .rotate_vector(Vector::x_axis());
        let forward = camera
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = camera
            .orientation
            .rotate_vector(Vector::z_axis());
        for axis in [&right, &forward, &up] {
            assert!((axis.magnitude() - 1.0).abs() < 1.0e-10);
        }
        assert!(right.dot(&forward).abs() < 1.0e-10);
        assert!(right.dot(&up).abs() < 1.0e-10);
        assert!(forward.dot(&up).abs() < 1.0e-10);
    }
```

### 2. `src/browser.rs`

Add vertical orbit and isometric commands. Apply vertical orbit or the isometric orientation. Report that orbit preserves the target.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective and picking use the same camera.");
    Ok(())
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Orbit moves the eye around a fixed target.");
    Ok(())
```

**Locate this block:**

```rust
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                    "view reset" => {
                        camera = Camera {
                            aspect: camera.aspect,
```

Replace it with:

```rust
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                    "orbit up" => camera.orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => camera.isometric(),
                    "view reset" => {
                        camera = Camera {
                            aspect: camera.aspect,
```

**Locate this block:**

```rust
            "Orbit Right",
            "View Reset",
        ],
    );
```

Replace it with:

```rust
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Reset",
        ],
    );
```

## Build & Run

![Recorded visual output for Lesson 2.6 (16-orbit)](../../screenshots/journey/16-orbit-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.6
npm --prefix ../session_tests run course -- save 2.6
```

Continue with [3.1 · Kernel geometry and a single editor owner](17-solid.md).
