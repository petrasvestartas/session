# 7.6 · Handles, numeric edits and source-part deformation

**Goal:** Verify complete handle editing and input. · **Steps:** 34 · **Source:** [app/command/mod.rs](../../../src/app/command/mod.rs) · [app/command/tool.rs](../../../src/app/command/tool.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/snap.rs](../../../src/app/snap.rs)

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

## <a id="checkpoint-51-handles"></a>1. Name move, rotation and scale handles

### 1. `src/gizmo.rs`

Define world-axis handles, labels and finite operation-specific numeric answers.

Create the file and type:

```rust
use session_rust::intersection::{line_line_parameters, line_plane};
use session_rust::{Line, Plane, Point, Vector, Xform};

pub const ARM: f64 = 96.0;

pub const BALL_AT: f64 = ARM * 0.5;

const ARROW_AT: f64 = ARM * 0.9;

const GRAB: f64 = 8.0;

pub const HUB: f64 = 6.0;

const SCALE_SOFTENING: f64 = 0.5;

pub(crate) const MIN_SCALE: f64 = 0.01;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Translate(Axis),
    Rotate(Axis),
    Scale(Axis),
    ScaleUniform,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn unit(self) -> Vector {
        match self {
            Axis::X => Vector::new(1.0, 0.0, 0.0),
            Axis::Y => Vector::new(0.0, 1.0, 0.0),
            Axis::Z => Vector::new(0.0, 0.0, 1.0),
        }
    }

    fn others(self) -> (Vector, Vector) {
        match self {
            Axis::X => (Axis::Y.unit(), Axis::Z.unit()),
            Axis::Y => (Axis::Z.unit(), Axis::X.unit()),
            Axis::Z => (Axis::X.unit(), Axis::Y.unit()),
        }
    }
}

impl Handle {
    pub fn labels(self) -> (&'static str, &'static str, &'static str) {
        let name = |a: Axis| match a {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
        };

        match self {
            Handle::Translate(a) => ("Move", name(a), "mm"),
            Handle::Rotate(a) => ("Rotate", name(a), "deg"),
            Handle::Scale(a) => ("Scale", name(a), "factor"),
            Handle::ScaleUniform => ("Scale", "", "factor"),
        }
    }

    pub fn title(self) -> String {
        let (verb, axis, _) = self.labels();

        if axis.is_empty() {
            verb.to_string()
        } else {
            format!("{verb} {axis}")
        }
    }
}

pub fn typed_value(handle: Handle, text: &str) -> Result<Option<f64>, String> {
    let text = text.trim();

    if text.is_empty() {
        return Ok(None);
    }

    let value = text
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("`{text}` is not a number"))?;

    match handle {
        Handle::Scale(_) | Handle::ScaleUniform if value <= 0.0 => {
            Err("scale wants a factor above zero".into())
        }
        Handle::Scale(_) | Handle::ScaleUniform => Ok((value != 1.0).then_some(value)),
        Handle::Translate(_) | Handle::Rotate(_) => Ok((value != 0.0).then_some(value)),
    }
}
```

### 2. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod edit_preview_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod edit_preview_tests;

pub mod gizmo;
```

## <a id="checkpoint-51a-rays"></a>2. Measure handle rays in world coordinates

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Curve Quadrature ∑ w_i ||C'(t_i)|| &amp; Surface Patch dA = ||Su×Sv||</text>
  <path d="M 56 138 Q 112 52 176 126" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="84" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="116" cy="84" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="148" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <polygon points="196,136 226,74 288,74 258,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="226" y1="105" x2="268" y2="105" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <line x1="226" y1="105" x2="242" y2="74" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Divergence Volume: V = (1/6) ∑ det(v0, v1, v2) &amp; Centroid C_g</text>
  <circle cx="448" cy="138" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="448" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Origin O</text>
  <polygon points="448,138 556,58 656,92" fill="#dbeafe" fill-opacity="0.55" stroke="#2563eb" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="556,58 656,92 596,142" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="576" cy="104" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Exact Length, Area, Volume &amp; Centroid Marker</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,136 894,64 988,64 940,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="917" cy="100" r="6" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1016" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">C_g</text>
</svg>
</div>

Measure handle rays in original world coordinates.

### 1. `src/gizmo.rs`

Resolve checked world-ray intersections, pivot transforms and positive drag scaling.

**Locate this existing block:**

```rust
        Handle::Translate(_) | Handle::Rotate(_) => Ok((value != 0.0).then_some(value)),
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Handle::Translate(_) | Handle::Rotate(_) => Ok((value != 0.0).then_some(value)),
    }
}

fn typed_factor(value: f64) -> f64 {
    if value <= 0.0 {
        return MIN_SCALE;
    }

    value
}

fn softened(ratio: f64) -> f64 {
    if !ratio.is_finite() || ratio <= 0.0 {
        return MIN_SCALE;
    }

    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

fn nonzero(v: f64) -> f64 {
    if v.abs() < 1e-9 {
        1e-9_f64.copysign(if v < 0.0 {
            -1.0
        } else {
            1.0
        })
    } else {
        v
    }
}

fn facing(dir: &Vector) -> Vector {
    let (x, y, z) = (dir[0].abs(), dir[1].abs(), dir[2].abs());

    if x >= y && x >= z {
        Axis::X.unit()
    } else if y >= z {
        Axis::Y.unit()
    } else {
        Axis::Z.unit()
    }
}

fn end_on(dir: &Vector, axis: Axis) -> bool {
    dir.dot(&axis.unit()).abs() > 0.97
}

fn closest_on_axis(
    from: &Point,
    dir: &Vector,
    origin: &Point,
    axis: &Vector,
) -> Option<Point> {
    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let line = Line::from_point_direction_length(origin, axis, 1.0);
    let (_, t) = line_line_parameters(&ray, &line, 0.0, false, false)?;

    Some(origin + &(axis * t))
}

fn plane_hit(
    from: &Point,
    dir: &Vector,
    origin: &Point,
    normal: &Vector,
) -> Option<Point> {
    if dir.dot(normal).abs() < 1e-9 {
        return None;
    }

    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let plane = Plane::from_point_normal(origin.clone(), normal.clone(), Some(false));
    let hit = line_plane(&ray, &plane, false)?;
    let t = (&hit - from).dot(dir);

    (t > 0.0 && t.is_finite()).then_some(hit)
}

fn within(from: &Point, dir: &Vector, at: &Point, radius: f64) -> bool {
    let t = (at - from).dot(dir);

    if t < 0.0 {
        return false;
    }

    let closest = from + &(dir * t);
    (&closest - at).magnitude() <= radius
}

fn angle_in_plane(p: &Point, origin: &Point, axis: Axis) -> f64 {
    let (u, v) = axis.others();
    let d = p - origin;
    d.dot(&v).atan2(d.dot(&u))
}

fn rotation(axis: Axis, radians: f64) -> Xform {
    match axis {
        Axis::X => Xform::rotation_x(radians, false),
        Axis::Y => Xform::rotation_y(radians, false),
        Axis::Z => Xform::rotation_z(radians, false),
    }
}

fn about(pivot: &Point, m: Xform) -> Xform {
    let to = Xform::translation(pivot[0], pivot[1], pivot[2]);
    let back = Xform::translation(-pivot[0], -pivot[1], -pivot[2]);

    &(&to * &m) * &back
}
```

## <a id="checkpoint-51b-pivot"></a>3. Keep handle state at one original-world pivot

### 1. `src/gizmo.rs`

Own transient handle state and compute each handle’s projected anchor.

**Locate this existing block:**

```rust
}

fn typed_factor(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[derive(Clone, Debug)]
pub struct Drag {
    pub handle: Handle,
    grabbed: Point,
    angle: f64,
    reach: f64,
    plane: Vector,
}

pub struct Gizmo {
    pub origin: Point,
    pub hovered: Option<Handle>,
    pub drag: Option<Drag>,
    pub typing: Option<Handle>,
}

impl Gizmo {
    pub fn new(origin: Point) -> Self {
        Self {
            origin,
            hovered: None,
            drag: None,
            typing: None,
        }
    }

    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
        self.hovered = None;
        self.drag = None;
        self.typing = None;
    }

    pub fn handle_point(&self, handle: Handle, world_per_px: f64) -> Point {
        let s = world_per_px;

        match handle {
            Handle::Translate(axis) => &self.origin + &(&axis.unit() * (ARROW_AT * s)),
            Handle::Scale(axis) => &self.origin + &(&axis.unit() * (BALL_AT * s)),
            Handle::ScaleUniform => self.origin.clone(),
            Handle::Rotate(axis) => {
                let (u, v) = axis.others();
                &self.origin - &(&(&u + &v) * (ARM * s / 2.0_f64.sqrt()))
            }
        }
    }
}

fn typed_factor(value: f64) -> f64 {
```

## <a id="checkpoint-51c-hit"></a>4. Pick handles in a stable priority order

### 1. `src/gizmo.rs`

Resolve stable handle priority and the same CSS aperture used by drawing.

**Locate this existing block:**

```rust
        }
    }
}

fn typed_factor(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }

    pub fn hit(&self, from: &Point, dir: &Vector, world_per_px: f64) -> Option<Handle> {
        self.hit_with_radius(from, dir, world_per_px, GRAB)
    }

    pub fn hit_with_radius(
        &self,
        from: &Point,
        dir: &Vector,
        world_per_px: f64,
        radius: f64,
    ) -> Option<Handle> {
        let s = world_per_px;
        let grab = radius.max(GRAB);

        if within(from, dir, &self.origin, HUB * s) {
            return Some(Handle::ScaleUniform);
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            let at = &self.origin + &(&axis.unit() * (BALL_AT * s));

            if within(from, dir, &at, grab * s) {
                return Some(Handle::Scale(axis));
            }
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            if let Some(p) = closest_on_axis(from, dir, &self.origin, &axis.unit()) {
                let t = (&p - &self.origin).dot(&axis.unit());

                if (HUB * s..=ARM * s).contains(&t) && within(from, dir, &p, grab * s) {
                    return Some(Handle::Translate(axis));
                }
            }
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = plane_hit(from, dir, &self.origin, &axis.unit()) {
                let d = &p - &self.origin;
                let (u, v) = axis.others();

                if d.dot(&u) < 0.0
                    && d.dot(&v) < 0.0
                    && (d.magnitude() - ARM * s).abs() < grab * s
                {
                    return Some(Handle::Rotate(axis));
                }
            }
        }

        None
    }
}

fn typed_factor(value: f64) -> f64 {
```

## <a id="checkpoint-51d-grab"></a>5. Capture the handle’s grab

Capture the handle’s original grab.

### 1. `src/gizmo.rs`

Capture original world grab measurements before producing a transform.

**Locate this existing block:**

```rust
        None
    }
}

fn typed_factor(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        None
    }

    pub fn begin(
        &mut self,
        handle: Handle,
        from: &Point,
        dir: &Vector,
    ) -> Option<Drag> {
        let drag = match handle {
            Handle::Translate(axis) => Drag {
                handle,
                grabbed: closest_on_axis(from, dir, &self.origin, &axis.unit())?,
                angle: 0.0,
                reach: 1.0,
                plane: axis.unit(),
            },
            Handle::Rotate(axis) => Drag {
                handle,
                grabbed: self.origin.clone(),
                angle: angle_in_plane(
                    &plane_hit(from, dir, &self.origin, &axis.unit())?,
                    &self.origin,
                    axis,
                ),
                reach: 1.0,
                plane: axis.unit(),
            },
            Handle::Scale(axis) => {
                let p = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = (&p - &self.origin).dot(&axis.unit());
                Drag {
                    handle,
                    grabbed: p,
                    angle: 0.0,
                    reach: nonzero(reach),
                    plane: axis.unit(),
                }
            }
            Handle::ScaleUniform => {
                let normal = facing(dir);
                let p = plane_hit(from, dir, &self.origin, &normal)?;
                let reach = (&p - &self.origin).magnitude();
                Drag {
                    handle,
                    grabbed: p,
                    angle: 0.0,
                    reach: nonzero(reach),
                    plane: normal,
                }
            }
        };
        self.drag = Some(drag.clone());
        Some(drag)
    }
}

fn typed_factor(value: f64) -> f64 {
```

## <a id="checkpoint-51e-transform"></a>6. Derive handle transforms and quarter-turn snapping

### 1. `src/gizmo.rs`

Derive world transforms from original grabs and implement current90-degree Shift rotation snapping.

**Locate this existing block:**

```rust
        Some(drag)
    }
}

fn typed_factor(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Some(drag)
    }

    pub fn update(
        &self,
        drag: &Drag,
        from: &Point,
        dir: &Vector,
        snap: bool,
    ) -> Option<Xform> {
        match drag.handle {
            Handle::Translate(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let d = &now - &drag.grabbed;
                Some(Xform::translation(d[0], d[1], d[2]))
            }
            Handle::Rotate(axis) => {
                let now = plane_hit(from, dir, &self.origin, &axis.unit())?;
                let turned = angle_in_plane(&now, &self.origin, axis) - drag.angle;
                let turned = if snap {
                    (turned / std::f64::consts::FRAC_PI_2).round()
                        * std::f64::consts::FRAC_PI_2
                } else {
                    turned
                };
                Some(about(&self.origin, rotation(axis, turned)))
            }
            Handle::Scale(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = (&now - &self.origin).dot(&axis.unit());
                let k = softened(reach / drag.reach);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                Some(about(
                    &self.origin,
                    Xform::scale_xyz(x, y, z),
                ))
            }
            Handle::ScaleUniform => {
                let now = plane_hit(from, dir, &self.origin, &drag.plane)?;
                let k = softened((&now - &self.origin).magnitude() / drag.reach);
                Some(about(
                    &self.origin,
                    Xform::scale_xyz(k, k, k),
                ))
            }
        }
    }

    pub fn typed(&self, handle: Handle, value: f64) -> Xform {
        match handle {
            Handle::Translate(axis) => {
                let u = axis.unit();
                Xform::translation(u[0] * value, u[1] * value, u[2] * value)
            }
            Handle::Rotate(axis) => about(
                &self.origin,
                rotation(axis, value.to_radians()),
            ),
            Handle::Scale(axis) => {
                let k = typed_factor(value);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                about(&self.origin, Xform::scale_xyz(x, y, z))
            }
            Handle::ScaleUniform => {
                let k = typed_factor(value);
                about(&self.origin, Xform::scale_xyz(k, k, k))
            }
        }
    }
}

fn typed_factor(value: f64) -> f64 {
```

### 2. `src/gizmo_tests.rs`

Copy fourteen maintained handle-math regression checks.

Create the file and type:

```rust
use crate::gizmo::*;
use session_rust::{Point, Vector};

/// A ray a few pixels off the arm still grabs it.
#[test]
fn arm_grab_accepts_a_ray_within_the_screen_aperture() {
    let gizmo = Gizmo::new(Point::new(5.0, 45.0, 0.0));
    let from = Point::new(25.0, 44.5, 100.0);
    assert_eq!(
        gizmo.hit(&from, &Vector::new(0.0, 0.0, -1.0), 0.25),
        Some(Handle::Translate(Axis::X))
    );
}

const SCALE: f64 = 1.0; // one world unit per pixel

/// A gizmo at the origin.
fn at_origin() -> Gizmo {
    Gizmo::new(Point::new(0.0, 0.0, 0.0))
}

/// A ray straight down through (x, y).
fn down(x: f64, y: f64) -> (Point, Vector) {
    (
        Point::new(x, y, 500.0),
        Vector::new(0.0, 0.0, -1.0),
    )
}

/// Hub, ball, arm and arc each answer at their own place.
#[test]
fn the_handles_do_not_shadow_each_other() {
    let g = at_origin();
    let (f, d) = down(0.0, 0.0);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::ScaleUniform)
    );
    let (f, d) = down(BALL_AT, 0.0);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::Scale(Axis::X))
    );
    let (f, d) = down(ARM * 0.8, 0.0);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::Translate(Axis::X))
    );
    // the arcs live in the quadrant the arms do not
    let r = ARM / 2.0_f64.sqrt();
    let (f, d) = down(-r, -r);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::Rotate(Axis::Z))
    );
    let (f, d) = down(ARM, 0.0);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::Translate(Axis::X)),
        "the arm tip is not an arc"
    );
    let (f, d) = down(ARM * 3.0, ARM * 3.0);
    assert_eq!(g.hit(&f, &d, SCALE), None);
}

/// An axis pointing at the eye cannot be grabbed.
#[test]
fn an_axis_seen_end_on_is_not_grabbable() {
    let g = at_origin();
    // just past the hub, where only the Z ball could answer
    let (f, d) = down(-7.0, 0.0);
    assert_eq!(g.hit(&f, &d, SCALE), None);
    let (f, d) = down(BALL_AT, 0.0);
    assert_eq!(
        g.hit(&f, &d, SCALE),
        Some(Handle::Scale(Axis::X))
    );
}

/// Handles keep their pixel size at any zoom.
#[test]
fn the_widget_is_screen_constant() {
    let g = at_origin();
    let (f, d) = down(BALL_AT * 2.0, 0.0);
    assert_eq!(
        g.hit(&f, &d, 2.0),
        Some(Handle::Scale(Axis::X))
    );
    assert_ne!(
        g.hit(&f, &d, 1.0),
        Some(Handle::Scale(Axis::X))
    );
}

/// A translate drag moves only along its axis.
#[test]
fn a_translate_drag_moves_along_its_axis_only() {
    let mut g = at_origin();
    let (f0, d0) = down(30.0, 0.0);
    let drag = g
        .begin(Handle::Translate(Axis::X), &f0, &d0)
        .expect("grabbed");
    let (f1, d1) = down(42.0, 0.0);
    let m = g
        .update(&drag, &f1, &d1, false)
        .expect("a transform");
    assert!((m.m[12] - 12.0).abs() < 1e-9);
    assert!(m.m[13].abs() < 1e-9 && m.m[14].abs() < 1e-9);
}

/// A drag along the viewing direction is refused.
#[test]
fn a_drag_down_its_own_axis_refuses() {
    let mut g = at_origin();
    let from = Point::new(-500.0, 0.0, 0.0);
    let dir = Vector::new(1.0, 0.0, 0.0);
    assert!(
        g.begin(Handle::Translate(Axis::X), &from, &dir)
            .is_none()
    );
}

/// A quarter turn about Z takes the x axis onto the y axis.
#[test]
fn a_rotate_drag_turns_by_the_angle_swept() {
    let mut g = at_origin();
    let (f0, d0) = down(ARM, 0.0);
    let drag = g
        .begin(Handle::Rotate(Axis::Z), &f0, &d0)
        .expect("grabbed");
    let (f1, d1) = down(0.0, ARM);
    let m = g
        .update(&drag, &f1, &d1, false)
        .expect("a transform");
    // column 0 is the image of the x axis
    assert!((m.m[0]).abs() < 1e-9, "x.x");
    assert!((m.m[1] - 1.0).abs() < 1e-9, "x.y");
}

/// With Shift a turn of 60 degrees snaps to the nearest quarter turn, 90.
#[test]
fn a_snapped_rotate_turns_by_quarter_turns() {
    let mut g = at_origin();
    let (f0, d0) = down(ARM, 0.0);
    let drag = g
        .begin(Handle::Rotate(Axis::Z), &f0, &d0)
        .expect("grabbed");
    let (f1, d1) = down(ARM * 0.5, ARM * 0.866);
    let free = g
        .update(&drag, &f1, &d1, false)
        .expect("a transform");
    let snapped = g
        .update(&drag, &f1, &d1, true)
        .expect("a transform");
    assert!(
        (free.m[1] - 0.866).abs() < 1e-3,
        "60 degrees free"
    );
    assert!(
        snapped.m[0].abs() < 1e-9 && (snapped.m[1] - 1.0).abs() < 1e-9,
        "90 degrees snapped"
    );
}

/// A scale starts at 1 and never flips.
#[test]
fn a_scale_is_relative_to_the_grab_and_never_collapses() {
    let mut g = at_origin();
    let (f0, d0) = down(BALL_AT, 0.0);
    let drag = g
        .begin(Handle::Scale(Axis::X), &f0, &d0)
        .expect("grabbed");
    let m = g
        .update(&drag, &f0, &d0, false)
        .expect("a transform");
    assert!(
        (m.m[0] - 1.0).abs() < 1e-9,
        "no movement is no change"
    );

    let (f1, d1) = down(BALL_AT * 4.0, 0.0);
    let grown = g
        .update(&drag, &f1, &d1, false)
        .expect("a transform");
    assert!(
        grown.m[0] > 1.0 && grown.m[5] == 1.0 && grown.m[10] == 1.0,
        "one axis only"
    );

    let (f2, d2) = down(-BALL_AT * 4.0, 0.0);
    let flipped = g
        .update(&drag, &f2, &d2, false)
        .expect("a transform");
    assert!(flipped.m[0] >= MIN_SCALE, "never mirrors");
}

/// A point at the gizmo centre does not move.
#[test]
fn transforms_act_about_the_gumball() {
    let g = Gizmo::new(Point::new(100.0, 200.0, 300.0));

    for m in [
        g.typed(Handle::Rotate(Axis::Z), 90.0),
        g.typed(Handle::ScaleUniform, 3.0),
    ] {
        let p = Point::new(100.0, 200.0, 300.0);
        let moved = m.transform_point(&p);

        for i in 0..3 {
            assert!((moved[i] - p[i]).abs() < 1e-9, "axis {i}");
        }
    }
}

/// Typed values give the same transforms as drags.
#[test]
fn typed_values_match_the_drag_and_are_clamped() {
    let g = at_origin();
    let m = g.typed(Handle::Translate(Axis::Y), 12.5);
    assert_eq!([m.m[12], m.m[13], m.m[14]], [0.0, 12.5, 0.0]);
    let z = g.typed(Handle::ScaleUniform, 0.0);
    assert!(z.m[0] >= MIN_SCALE && z.m[5] >= MIN_SCALE && z.m[10] >= MIN_SCALE);
    let fine = g.typed(Handle::Scale(Axis::X), 0.001);
    assert!(
        (fine.m[0] - 0.001).abs() < 1e-15,
        "a typed factor below the drag minimum"
    );
}

/// Every number box is pinned on its own handle.
#[test]
fn every_handle_anchor_lies_on_its_handle() {
    let g = at_origin();
    let top = Vector::new(0.0, 0.0, -1.0);
    let front = Vector::new(0.0, 1.0, 0.0);
    let side = Vector::new(1.0, 0.0, 0.0);
    let views = [
        (Handle::Translate(Axis::X), top.clone()),
        (Handle::Translate(Axis::Y), top.clone()),
        (Handle::Translate(Axis::Z), front.clone()),
        (Handle::Scale(Axis::X), top.clone()),
        (Handle::Scale(Axis::Y), top.clone()),
        (Handle::Scale(Axis::Z), front.clone()),
        (Handle::ScaleUniform, top.clone()),
        (Handle::Rotate(Axis::X), side),
        (Handle::Rotate(Axis::Y), front),
        (Handle::Rotate(Axis::Z), top),
    ];

    for (handle, dir) in views {
        // a ray through the anchor, from 500 units back
        let from = &g.handle_point(handle, SCALE) - &(&dir * 500.0);
        assert_eq!(
            g.hit(&from, &dir, SCALE),
            Some(handle),
            "{handle:?}"
        );
    }
}

/// Typed text becomes a value, nothing, or a reason.
#[test]
fn typed_values_parse_per_handle() {
    let arm = Handle::Translate(Axis::X);
    let ball = Handle::Scale(Axis::Y);
    assert_eq!(typed_value(arm, " 250 "), Ok(Some(250.0)));
    assert_eq!(
        typed_value(Handle::Rotate(Axis::Z), "-90"),
        Ok(Some(-90.0))
    );
    assert_eq!(typed_value(arm, ""), Ok(None));
    assert_eq!(typed_value(arm, "0"), Ok(None), "no move");
    assert_eq!(typed_value(ball, "1"), Ok(None), "no scale");

    for text in ["abc", "1e999", "NaN"] {
        assert_eq!(
            typed_value(arm, text),
            Err(format!("`{text}` is not a number"))
        );
    }

    for (handle, text) in [(ball, "0"), (Handle::ScaleUniform, "-2")] {
        assert_eq!(
            typed_value(handle, text),
            Err("scale wants a factor above zero".into())
        );
    }
}

/// Moving the gumball closes its number box.
#[test]
fn a_moved_gizmo_closes_its_number_box() {
    let mut g = at_origin();
    assert_eq!(g.typing, None);
    g.typing = Some(Handle::Rotate(Axis::Z));
    g.set_origin(Point::new(1.0, 2.0, 3.0));
    assert_eq!(g.typing, None);
    assert_eq!(Handle::ScaleUniform.title(), "Scale");
    assert_eq!(Handle::Translate(Axis::X).title(), "Move X");
}
```

### 3. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
mod edit_preview_tests;

pub mod gizmo;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod edit_preview_tests;

pub mod gizmo;

#[cfg(test)]
mod gizmo_tests;
```

## <a id="checkpoint-51ez-pack-and-sample-local-handle-vertices"></a>7. Pack and sample local handle vertices

Write the twenty-byte widget vertex format, explicit byte packing and one shared rectangular surface sampler.

### 1. `src/widget_mesh.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use crate::gizmo::{ARM, BALL_AT, HUB};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: u32,
    pub(crate) handle: u32,
}

const COLORS: [u32; 3] = [0xff2424e8, 0xff30b820, 0xffef6628];

const SIDES: usize = 24;

const SHAFT: f32 = 2.2;

const TIP: f32 = 14.0;

fn orient(p: [f32; 3], axis: usize) -> [f32; 3] {
    match axis {
        0 => p,
        1 => [p[2], p[0], p[1]],
        _ => [p[1], p[2], p[0]],
    }
}

fn surface(
    out: &mut Vec<Vertex>,
    rows: usize,
    columns: usize,
    color: u32,
    handle: u32,
    sample: impl Fn(f32, f32) -> [f32; 3],
) {
    for row in 0..rows {
        for column in 0..columns {
            for (i, j) in [
                (0, 0),
                (1, 0),
                (1, 1),
                (0, 0),
                (1, 1),
                (0, 1),
            ] {
                let position = sample(
                    (row + i) as f32 / rows as f32,
                    (column + j) as f32 / columns as f32,
                );
                out.push(Vertex {
                    position,
                    color,
                    handle,
                });
            }
        }
    }
}

impl Vertex {
    pub fn bytes(self) -> [u8; 20] {
        let mut bytes = [0; 20];
        for (i, value) in self.position.into_iter().enumerate() {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&value.to_ne_bytes());
        }
        bytes[12..16].copy_from_slice(&self.color.to_ne_bytes());
        bytes[16..20].copy_from_slice(&self.handle.to_ne_bytes());
        bytes
    }
}
```

### 2. `src/lib.rs`

Register the local widget mesh helpers.

**Locate this existing block:**

```rust
#[cfg(test)]
mod gizmo_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod gizmo_tests;

pub mod widget_mesh;
```

## <a id="checkpoint-51f-mesh"></a>8. Build local arrows balls and rotation arcs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Corner Tangents t0, t1 &amp; Miter Vector m = (n0+n1)/dot</text>
  <polyline points="58,136 164,66 270,136" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <polygon points="48,122 164,44 280,122 260,150 164,88 68,150" fill="#dbeafe" fill-opacity="0.7" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="164" y1="66" x2="164" y2="44" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="192" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">m</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Round Cap SDF (||uv|| &lt;= 1) + Arc-Length Dash Modulo</text>
  <circle cx="468" cy="102" r="28" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="468" y1="102" x2="496" y2="102" stroke="#dc2626" stroke-width="2"/>
  <rect x="526" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="592" y="88" width="24" height="28" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="636" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Clean Mitered Corners &amp; Dashed CAD Strokes</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polyline points="834,136 904,68 974,136" fill="none" stroke="#111111" stroke-width="6" stroke-linejoin="miter" stroke-linecap="round"/>
  <line x1="964" y1="74" x2="1044" y2="136" stroke="#2563eb" stroke-width="4" stroke-dasharray="10 6" stroke-linecap="round"/>
</svg>
</div>

Reuse the surface sampler for the original lathed shafts, cones, balls, rotation arcs and central hub.

### 1. `src/widget_mesh.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
const TIP: f32 = 14.0;

fn orient(p: [f32; 3], axis: usize) -> [f32; 3] {
    match axis {
        0 => p,
        1 => [p[2], p[0], p[1]],
        _ => [p[1], p[2], p[0]],
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
const TIP: f32 = 14.0;

pub fn vertices() -> Vec<Vertex> {
    let mut out = Vec::with_capacity(18_576);

    for (axis, color) in COLORS.into_iter().enumerate() {
        let arm = ARM as f32;
        let shaft = [(HUB as f32, SHAFT), (arm - TIP, SHAFT)];
        lathe(&mut out, axis, color, axis as u32, &shaft);
        let cone = [
            (arm - TIP, 0.0),
            (arm - TIP, 5.5),
            (arm, 0.0),
        ];
        lathe(&mut out, axis, color, axis as u32, &cone);
        sphere(
            &mut out,
            axis,
            BALL_AT as f32,
            5.0,
            color,
            axis as u32 + 6,
        );
        surface(
            &mut out,
            48,
            12,
            color,
            axis as u32 + 3,
            |s, t| {
                let angle = s * FRAC_PI_2;
                let tube = t * TAU;
                let radius = arm + 1.8 * tube.cos();
                let position = [
                    1.8 * tube.sin(),
                    -angle.cos() * radius,
                    -angle.sin() * radius,
                ];
                orient(position, axis)
            },
        );
    }

    sphere(&mut out, 0, 0.0, HUB as f32, 0xffd8d8d8, 9);
    out
}

fn orient(p: [f32; 3], axis: usize) -> [f32; 3] {
    match axis {
        0 => p,
        1 => [p[2], p[0], p[1]],
        _ => [p[1], p[2], p[0]],
    }
}

fn lathe(
    out: &mut Vec<Vertex>,
    axis: usize,
    color: u32,
    handle: u32,
    profile: &[(f32, f32)],
) {
    for pair in profile.windows(2) {
        let [(x0, r0), (x1, r1)] = [pair[0], pair[1]];
        surface(out, 1, SIDES, color, handle, |s, t| {
            let (sin, cos) = (t * TAU).sin_cos();
            let r = r0 + (r1 - r0) * s;
            let position = [x0 + (x1 - x0) * s, r * cos, r * sin];
            orient(position, axis)
        });
    }
}

fn sphere(
    out: &mut Vec<Vertex>,
    axis: usize,
    at: f32,
    radius: f32,
    color: u32,
    handle: u32,
) {
    surface(out, 12, SIDES, color, handle, |s, t| {
        let (sin, cos) = (s * PI).sin_cos();
        let (v, u) = (t * TAU).sin_cos();
        let position = [
            at + radius * cos,
            radius * sin * u,
            radius * sin * v,
        ];
        orient(position, axis)
    });
}
```

## <a id="checkpoint-51g-layout"></a>9. Keep handles constant in CSS pixels

### 1. `src/widget_layout.rs`

Convert CSS handle size through the pivot’s current depth and bound its projected tile.

Create the file and type:

```rust
use session_rust::Xform;

pub struct Layout {
    pub uniform: [f32; 24],
    pub extent: [u32; 2],
    pub rect: [f64; 4],
}

// Measuring the projected pivot keeps a 96-pixel handle the same size while the camera zooms or changes projection.
pub fn world_per_pixel(matrix: &Xform, origin: [f64; 3], height: f64) -> Option<f64> {
    if !height.is_finite() || height <= 0. || origin.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let m = &matrix.m;
    let w = m[3] * origin[0] + m[7] * origin[1] + m[11] * origin[2] + m[15];
    let vertical = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
    let value = 2. * w / (height * vertical);
    (w > 0. && value.is_finite() && value > 0.).then_some(value)
}

pub fn layout(
    matrix: &Xform,
    origin: [f64; 3],
    scale: f64,
    size: [u32; 2],
    active: f32,
) -> Option<Layout> {
    if size.contains(&0) || !scale.is_finite() || scale <= 0. || !active.is_finite() {
        return None;
    }
    let model = &Xform::translation(origin[0], origin[1], origin[2])
        * &Xform::scale_xyz(scale, scale, scale);
    let matrix = matrix * &model;
    let rect = bounds(&matrix.m, size)?;
    let extent = std::array::from_fn(|i| {
        ((rect[i + 2] * 2.).ceil() as u32)
            .div_ceil(64)
            .clamp(1, 16)
            * 64
    });
    let [x, y, width, height] = rect;
    let sx = size[0] as f64 / width;
    let sy = size[1] as f64 / height;
    let tx = (size[0] as f64 - 2. * x - width) / width;
    let ty = (2. * y + height - size[1] as f64) / height;
    let mut uniform = [0f32; 24];
    for column in 0..4 {
        let at = column * 4;
        uniform[at] = (sx * matrix.m[at] + tx * matrix.m[at + 3]) as f32;
        uniform[at + 1] = (sy * matrix.m[at + 1] + ty * matrix.m[at + 3]) as f32;
        uniform[at + 2] = matrix.m[at + 2] as f32;
        uniform[at + 3] = matrix.m[at + 3] as f32;
    }
    uniform[16] = (2. * x / size[0] as f64 - 1.) as f32;
    uniform[17] = (1. - 2. * y / size[1] as f64) as f32;
    uniform[18] = (2. * width / size[0] as f64) as f32;
    uniform[19] = (-2. * height / size[1] as f64) as f32;
    uniform[20] = active;
    uniform
        .iter()
        .all(|v| v.is_finite())
        .then_some(Layout {
            uniform,
            extent,
            rect,
        })
}

fn bounds(m: &[f64; 16], size: [u32; 2]) -> Option<[f64; 4]> {
    let radius = crate::gizmo::ARM + 2.;
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for corner in 0..8 {
        let p = std::array::from_fn::<_, 3, _>(|i| {
            if corner & (1 << i) == 0 {
                -radius
            } else {
                radius
            }
        });
        let c = std::array::from_fn::<_, 4, _>(|i| {
            m[i] * p[0] + m[4 + i] * p[1] + m[8 + i] * p[2] + m[12 + i]
        });
        if c.iter().any(|v| !v.is_finite()) || c[3] <= 0. {
            return None;
        }
        let screen = [
            (c[0] / c[3] * 0.5 + 0.5) * size[0] as f64,
            (0.5 - c[1] / c[3] * 0.5) * size[1] as f64,
        ];
        for i in 0..2 {
            min[i] = min[i].min(screen[i]);
            max[i] = max[i].max(screen[i]);
        }
    }
    let x = (min[0] - 2.).floor().max(0.);
    let y = (min[1] - 2.).floor().max(0.);
    let width = (max[0] + 2.).ceil().min(size[0] as f64) - x;
    let height = (max[1] + 2.).ceil().min(size[1] as f64) - y;
    (width > 0. && height > 0.).then_some([x, y, width, height])
}
```

### 2. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
mod gizmo_tests;

pub mod widget_mesh;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod gizmo_tests;

pub mod widget_mesh;

pub mod widget_layout;
```

## <a id="checkpoint-51h-shader"></a>10. Draw a local handle tile and its canvas rectangle

### 1. `src/widget.wgsl`

Draw packed handle colors, active orange and the resolved tile composite.

Create the file and type:

```wgsl
// Gumball uniform, 96 bytes; matches the Rust array.
struct Widget {
    mvp: mat4x4<f32>, // gumball to tile clip space
    rect: vec4<f32>, // where the tile lands on the canvas: x, y, w, h
    settings: vec4<f32>, // x: active handle, -1 = none
};

@group(0) @binding(0) var<uniform> widget: Widget; // placement
@group(1) @binding(0) var picture: texture_2d<f32>; // the drawn gumball tile
@group(1) @binding(1) var picture_sampler: sampler; // linear sampler

// What the mesh vertex shader hands the fragment shader.
struct Varying {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) color: vec3<f32>, // rgb
};

@vertex
// Place a mesh vertex; the active handle turns orange.
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: u32,
           @location(2) handle: u32) -> Varying {
    var out: Varying;
    out.position = widget.mvp * vec4<f32>(position, 1.0);
    out.color = unpack4x8unorm(color).rgb;

    if f32(handle) == widget.settings.x {
        out.color = vec3<f32>(0.95, 0.65, 0.08);
    }

    return out;
}

@fragment
// Flat color.
fn fs_main(in: Varying) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// What the composite vertex shader hands the fragment shader.
struct Composite {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // texture coordinate
};

@vertex
// One corner of the tile's quad on the canvas.
fn vs_composite(@builtin(vertex_index) index: u32) -> Composite {
    let corners = array<vec2<f32>, 6>(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.),
                                     vec2(0., 0.), vec2(1., 1.), vec2(0., 1.));
    var out: Composite;
    out.uv = corners[index];
    out.position = vec4<f32>(widget.rect.xy + out.uv * widget.rect.zw, 0., 1.);
    return out;
}

@fragment
// Blend the tile over the frame.
fn fs_composite(in: Composite) -> @location(0) vec4<f32> {
    let pixel = textureSample(picture, picture_sampler, in.uv);
    return vec4<f32>(pixel.rgb / max(pixel.a, 0.00001), pixel.a);
}
```

## <a id="checkpoint-51i-image"></a>11. Bind the widget’s own sampled image

### 1. `src/widget_pipelines.rs`

Declare the explicit sampled tile and filtering sampler bindings.

Create the file and type:

```rust
pub fn texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("handle tile"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}
```

### 2. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
pub mod widget_mesh;

pub mod widget_layout;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod widget_mesh;

pub mod widget_layout;

pub mod widget_pipelines;
```

## <a id="checkpoint-51j-pipelines"></a>12. Separate handle depth from scene depth

### 1. `src/widget_pipelines.rs`

Build separate4x depth-tested mesh and depth-free alpha-composite pipelines.

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}

pub fn mesh(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("handles"),
        source: wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into()),
    });
    let mesh_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("handle mesh"),
        bind_group_layouts: &[Some(&layout)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("handle mesh"),
        layout: Some(&mesh_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 20,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 4,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}

pub fn composite(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("handles"),
        source: wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into()),
    });
    let composite_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("handle composite"),
            bind_group_layouts: &[Some(&layout), Some(&texture_layout)],
            immediate_size: 0,
        });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("handle composite"),
        layout: Some(&composite_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_composite"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_composite"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

## <a id="checkpoint-51k-tile"></a>13. Own color, depth and resolve textures together

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

### 1. `src/widget.rs`

Own the widget’s three matching texture attachments and sampled image group.

Create the file and type:

```rust
struct Tile {
    size: [u32; 2],
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    resolved: wgpu::TextureView,
    group: wgpu::BindGroup,
}

impl Tile {
    fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        size: [u32; 2],
    ) -> Self {
        let texture = |format, sample_count, usage| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("handle tile"),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = texture(
            wgpu::TextureFormat::Rgba8Unorm,
            4,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let depth = texture(
            wgpu::TextureFormat::Depth32Float,
            4,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let resolved = texture(
            wgpu::TextureFormat::Rgba8Unorm,
            1,
            wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("handle tile"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolved),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        Self {
            size,
            color,
            depth,
            resolved,
            group,
        }
    }
}
```

### 2. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
pub mod widget_layout;

pub mod widget_pipelines;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod widget_layout;

pub mod widget_pipelines;

pub mod widget;
```

## <a id="checkpoint-51l-upload"></a>14. Upload handle geometry once

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

### 1. `src/widget.rs`

Upload one static handle mesh, construct pipelines and expose explicit allocation accounting.

**Locate this existing block:**

```rust
struct Tile {
    size: [u32; 2],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;

pub struct Widget {
    vertices: wgpu::Buffer,
    count: u32,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    composite: wgpu::RenderPipeline,
    texture_layout: wgpu::BindGroupLayout,
    tile: Option<Tile>,
    visible: bool,
    pub allocations: u64,
    pub writes: u64,
}

impl Widget {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let mesh = crate::widget_mesh::vertices();
        let bytes: Vec<u8> = mesh
            .iter()
            .copied()
            .flat_map(crate::widget_mesh::Vertex::bytes)
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("handle mesh"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let layout = crate::gpu_layout::uniform(device, 96);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("handle placement"),
            size: 96,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = crate::gpu_layout::group(device, &layout, &uniform);
        let texture_layout = crate::widget_pipelines::texture_layout(device);
        let pipeline = crate::widget_pipelines::mesh(device, &layout);
        let composite = crate::widget_pipelines::composite(
            device,
            &layout,
            &texture_layout,
            format,
        );
        Self {
            vertices,
            count: mesh.len() as u32,
            uniform,
            group,
            pipeline,
            composite,
            texture_layout,
            tile: None,
            visible: false,
            allocations: 0,
            writes: 0,
        }
    }

    pub fn clear(&mut self) {
        self.tile = None;
        self.visible = false;
    }
    pub fn usage(&self) -> [u64; 4] {
        [
            self.vertices.size() + self.uniform.size(),
            self.tile.as_ref().map_or(0, |t| {
                u64::from(t.size[0]) * u64::from(t.size[1]) * 36
            }),
            self.allocations,
            self.writes,
        ]
    }
    pub fn extent(&self) -> Option<[u32; 2]> {
        self.tile.as_ref().map(|t| t.size)
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        self.vertices.destroy();
        self.uniform.destroy();
    }
}

struct Tile {
    size: [u32; 2],
```

## <a id="checkpoint-51m-prepare"></a>15. Write only current handle placement

### 1. `src/widget.rs`

Reuse a matching tile, release absent widgets and update the small current-frame uniform.

**Locate this existing block:**

```rust
        self.tile.as_ref().map(|t| t.size)
    }
}

impl Drop for Widget {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.tile.as_ref().map(|t| t.size)
    }

    // A small independent depth buffer lets handles overlap correctly while remaining visible over the entire scene.
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: Option<crate::widget_layout::Layout>,
    ) {
        self.visible = false;
        let Some(layout) = layout else {
            self.clear();
            return;
        };
        if self
            .tile
            .as_ref()
            .is_none_or(|tile| tile.size != layout.extent)
        {
            self.tile = Some(Tile::new(
                device,
                &self.texture_layout,
                layout.extent,
            ));
            self.allocations += 1;
        }
        let bytes: Vec<u8> = layout
            .uniform
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
        self.writes += 1;
        self.visible = true;
    }
}

impl Drop for Widget {
```

## <a id="checkpoint-51n-draw"></a>16. Composite handles after source geometry

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

### 1. `src/widget.rs`

Render the local4x tile, resolve it and blend its rectangle over the existing canvas.

**Locate this existing block:**

```rust
        self.visible = true;
    }
}

impl Drop for Widget {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.visible = true;
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> u32 {
        let Some(tile) = self.tile.as_ref().filter(|_| self.visible) else {
            return 0;
        };
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("handle tile"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &tile.color,
                    resolve_target: Some(&tile.resolved),
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &tile.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    },
                ),
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.group, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..self.count, 0..1);
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("handle overlay"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.composite);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, &tile.group, &[]);
        pass.draw(0..6, 0..1);
        2
    }
}

impl Drop for Widget {
```

## <a id="checkpoint-51o-renderer"></a>17. Register the widget outside source picking

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Offscreen Rg32Uint Texture (owner_id, sub_id)</text>
  <rect x="74" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="146" y="76" width="80" height="50" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="105" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">5×5 px Window</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">COPY_SRC Attachment</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">copy_texture_to_buffer (256B Row Pitch) -&gt; Staging</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer (MAP_READ | COPY_DST)</text>
  <rect x="446" y="90" width="228" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">bytes_per_row = 256 (WebGPU alignment)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">slice.map_async(MapMode::Read, ...)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Callback Reads Mapped [u32; 2] Winner</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_mapped_range() -&gt; (id, sub)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="94" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">staging.unmap()</text>
  <rect x="846" y="114" width="176" height="32" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Selected Owner Updated</text>
</svg>
</div>

### 1. `src/renderer.rs`

Own and draw the bounded widget independently of primary source geometry and integer picking.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.queue.submit([encoder.finish()]);
```

Replace it with:

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder, view);
        self.queue.submit([encoder.finish()]);
```

**Locate this block:**

```rust
        self.preview.bytes()
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
            highlight_pipeline,
            highlight,
            preview,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

Replace it with:

```rust
            highlight_pipeline,
            highlight,
            preview,
            widget,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
```

**Locate this block:**

```rust
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

Replace it with:

```rust
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
```

**Locate this block:**

```rust
    preview: crate::tool_preview::Preview,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

Replace it with:

```rust
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
```

### 2. `src/widget_tests.rs`

Copy real GPU pixels, bounded layout, constant CSS size, hover reuse and texture-release checks.

Create the file and type:

```rust
use crate::{renderer::Renderer, scene::Scene, viewport::Viewport};
fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default()))
        .expect("Actual GPU adapter");
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default()))
            .expect("Actual GPU device");
    let mut renderer = Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        scene,
    );
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    renderer
}

fn pixels(renderer: &mut Renderer, transform: &[f32; 16], size: [u32; 2]) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
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
        transform,
        false,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: (u64::from(size[0]) * 4).div_ceil(256) * 256 * u64::from(size[1]),
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
                bytes_per_row: Some((size[0] * 4).div_ceil(256) * 256),
                rows_per_image: Some(size[1]),
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
    let mapped = bytes.slice(..).get_mapped_range();
    let pitch = (size[0] * 4).div_ceil(256) * 256;
    let result = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..size[0] as usize * 4].iter().copied())
        .collect();
    drop(mapped);
    bytes.unmap();
    result
}

#[test]
fn all_handles_have_finite_twenty_byte_vertices_and_layouts_keep_css_size() {
    let vertices = crate::widget_mesh::vertices();
    assert!(vertices.len() < 20_000);
    let mut handles = [false; 10];
    for vertex in vertices {
        handles[vertex.handle as usize] = true;
        assert_eq!(vertex.bytes().len(), 20);
        assert!(
            vertex
                .position
                .iter()
                .all(|v| v.is_finite() && v.abs() <= 98.)
        );
    }
    assert!(handles.into_iter().all(|v| v));
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    for projection in [
        crate::camera::Projection::Perspective,
        crate::camera::Projection::Orthographic,
    ] {
        camera.projection = projection;
        for distance in [3., 9., 30.] {
            camera.distance = distance;
            let matrix = camera.view_projection();
            let scale =
                crate::widget_layout::world_per_pixel(&matrix, [0.; 3], 480.).unwrap();
            let centre = matrix.transform_point(&session_rust::Point::new(0., 0., 0.));
            let world = camera
                .orientation
                .rotate_vector(session_rust::Vector::z_axis())
                * (96. * scale);
            let tip = matrix.transform_point(&session_rust::Point::new(
                world[0], world[1], world[2],
            ));
            assert!(((tip[1] - centre[1]) * 240. - 96.).abs() < 1e-9);
            for density in [1u32, 2] {
                let layout = crate::widget_layout::layout(
                    &matrix,
                    [0.; 3],
                    scale,
                    [640 * density, 480 * density],
                    -1.,
                )
                .unwrap();
                assert!(
                    layout
                        .extent
                        .into_iter()
                        .all(|v| v <= 1024 && v % 64 == 0)
                );
            }
        }
    }
    assert!(
        crate::widget_layout::world_per_pixel(
            &session_rust::Xform::identity(),
            [0.; 3],
            0.
        )
        .is_none()
    );
    let mut invalid = session_rust::Xform::identity();
    invalid.m[15] = -1.;
    assert!(
        crate::widget_layout::layout(&invalid, [0.; 3], 1., [640, 480], -1.).is_none()
    );
}

#[test]
fn actual_widget_gpu_draws_over_geometry_reuses_tile_and_never_creates_pick_owners() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    camera.aspect = 320. / 240.;
    for projection in [
        crate::camera::Projection::Perspective,
        crate::camera::Projection::Orthographic,
    ] {
        camera.projection = projection;
        let inverse = camera.view_projection().inverse().unwrap();
        let points = [
            [-1., -1., 0.001],
            [3., -1., 0.001],
            [-1., 3., 0.001],
        ]
        .map(|p| {
            let p =
                inverse.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            [p[0], p[1], p[2]]
        });
        scene.clear();
        scene
            .insert(crate::prepared::PreparedMesh::triangle(points, [0.5; 3]).unwrap())
            .unwrap();
        renderer.set_scene(&scene, None);
        let stats = renderer.stats();
        let pick = renderer.pick_usage();
        for density in [1u32, 2] {
            let size = [320 * density, 240 * density];
            renderer.resize(Viewport {
                width: size[0],
                height: size[1],
            });
            renderer.set_density(density as f32);
            renderer.set_gizmo(&camera, Some([0.; 3]), -1.);
            let usage = renderer.gizmo_usage();
            assert!(usage[0] < 400_100 && usage[1] <= 1024 * 1024 * 36 && usage[1] > 0);
            let image = pixels(&mut renderer, &camera.uniform(), size);
            let mut colours = [0; 3];
            for p in image.chunks_exact(4) {
                for i in 0..3 {
                    if p[i] > 140 && p[(i + 1) % 3] < 130 && p[(i + 2) % 3] < 130 {
                        colours[i] += 1;
                    }
                }
            }
            assert!(
                colours.into_iter().all(|n| n > 100),
                "Three visible handle colours: {colours:?}"
            );
            renderer.set_gizmo(&camera, Some([0.; 3]), 0.);
            assert_eq!(
                renderer.gizmo_usage()[2],
                usage[2],
                "Hover reuses the tile"
            );
            let active = pixels(&mut renderer, &camera.uniform(), size);
            assert_ne!(active, image);
            let orange = active
                .chunks_exact(4)
                .filter(|p| p[0] > 180 && p[1] > 100 && p[1] < 210 && p[2] < 90)
                .count();
            assert!(orange > 100);
            assert_eq!(renderer.stats(), stats);
            assert_eq!(renderer.pick_usage(), pick);
            renderer.set_gizmo(&camera, None, -1.);
            assert_eq!(renderer.gizmo_usage()[1], 0);
            assert!(renderer.gizmo_extent().is_none());
        }
    }
}
```

### 3. `src/lib.rs`

Register the handle component and supplied checks.

**Locate this existing block:**

```rust
pub mod widget_pipelines;

pub mod widget;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod widget_pipelines;

pub mod widget;

#[cfg(test)]
mod widget_tests;
```

## <a id="checkpoint-51p-part-points"></a>18. Locate mesh parts and curve controls

Locate original mesh parts and curve controls.

### 1. `src/part_preview.rs`

Resolve original part keys and precise placed source points.

Create the file and type:

```rust
use crate::{
    pick_sources::{Part, Target},
    scene::{ObjectId, Scene},
};
use session_rust::{Point, Xform};
use std::rc::Rc;

pub fn points(scene: &Scene, target: Target) -> Result<Vec<Point>, &'static str> {
    if !target.valid(scene) {
        return Err("Selected part is no longer editable");
    }
    let model = scene
        .placement(target.owner)
        .ok_or("Part owner is missing")?;
    let points = if let Some(row) = scene
        .objects()
        .iter()
        .find(|r| r.id == target.owner)
    {
        let mesh = row
            .geometry()
            .ok_or("Reload editable sources before editing a part")?;
        keys(mesh, target.part)?
            .into_iter()
            .map(|key| {
                mesh.vertex_point(key)
                    .ok_or("Missing source vertex")
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let row = scene
            .paths()
            .iter()
            .find(|r| r.id == target.owner)
            .ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve, _), Part::Control(index)) =
            (&row.prepared.source, target.part)
        else {
            return Err("Select a curve control point");
        };
        vec![
            curve
                .get_cv(index)
                .ok_or("Missing curve control")?,
        ]
    };
    Ok(points
        .into_iter()
        .map(|p| model.transform_point(&p))
        .collect())
}

pub fn origin(scene: &Scene, target: Target) -> Option<[f64; 3]> {
    let points = points(scene, target).ok()?;
    if points.is_empty() {
        return None;
    }
    Some(std::array::from_fn(|i| {
        points
            .iter()
            .map(|p| p[i] / points.len() as f64)
            .sum()
    }))
}

pub fn keys(mesh: &session_rust::Mesh, part: Part) -> Result<Vec<usize>, &'static str> {
    let mut keys = match part {
        Part::Face(key) => {
            let mut keys = mesh
                .face
                .get(&key)
                .ok_or("Unknown mesh face")?
                .clone();
            if let Some(holes) = mesh.face_holes.get(&key) {
                keys.extend(holes.iter().flatten());
            }
            keys
        }
        Part::MeshEdge(a, b) if mesh.edges().contains(&(a, b)) => vec![a, b],
        _ => return Err("Select a mesh face or edge"),
    };
    keys.sort_unstable();
    keys.dedup();
    if keys
        .iter()
        .any(|key| mesh.vertex_point(*key).is_none())
    {
        return Err("Missing source vertex");
    }
    Ok(keys)
}
```

### 2. `src/lib.rs`

Register the edit component and supplied checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod widget_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod widget_tests;

pub mod part_preview;
```

## <a id="checkpoint-51q-deform"></a>19. Deform selected points through their placement

Deform selected original points through their placement.

### 1. `src/scene.rs`

Transform original selected part points through local placement and preserve homogeneous curve weights.

**Locate this existing block:**

```rust
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn deform_part(
        &mut self,
        target: crate::pick_sources::Target,
        delta: &session_rust::Xform,
    ) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {
            return Err("Invalid part edit");
        }
        let model = self
            .placement(target.owner)
            .ok_or("Part owner is missing")?;
        let local = &(&model
            .inverse()
            .ok_or("Part placement cannot be inverted")?
            * delta)
            * &model;
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            let mut source = (**row
                .geometry()
                .ok_or("Reload editable sources before editing a part")?)
            .clone();
            for key in crate::part_preview::keys(&source, target.part)? {
                let point = local.transform_point(
                    &source
                        .vertex_point(key)
                        .ok_or("Missing source vertex")?,
                );
                source
                    .vertex
                    .get_mut(&key)
                    .ok_or("Missing source vertex")?
                    .set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared = crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {
                let point = model.transform_point(&session_rust::Point::new(
                    vertex[0] as f64,
                    vertex[1] as f64,
                    vertex[2] as f64,
                ));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {
                    return Err("Part edit exceeds the display range");
                }
            }
            row.mesh = Rc::new(prepared.display);
            row.editable = crate::edit_source::EditSource::Loaded {
                geometry: prepared.geometry,
                source: None,
            };
            return Ok(());
        }
        let row = self
            .paths
            .iter_mut()
            .find(|r| r.id == target.owner)
            .ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve, _), Part::Control(index)) =
            (&row.prepared.source, target.part)
        else {
            return Err("Select a curve control point");
        };
        let mut source = (**curve).clone();
        let point = local.transform_point(
            &source
                .get_cv(index)
                .ok_or("Missing curve control")?,
        );
        let weight = source
            .get_cv_4d(index)
            .ok_or("Missing curve control")?
            .3;
        let set = if source.m_is_rat {
            source.set_cv_4d(
                index,
                point[0] * weight,
                point[1] * weight,
                point[2] * weight,
                weight,
            )
        } else {
            source.set_cv_point(index, &point)
        };
        if !set {
            return Err("Cannot set curve control");
        }
        let prepared = crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared
            .source
            .coordinates()
            .into_iter()
            .chain(prepared.source.bounds_coordinates())
        {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                return Err("Part edit exceeds the display range");
            }
        }
        row.prepared = prepared;
        Ok(())
    }
}
```

## <a id="checkpoint-51r-part-preview"></a>20. Keep part deformation outside saved geometry

### 1. `src/part_preview.rs`

Own temporary source deformation and refuse stale part commits.

**Locate this existing block:**

```rust
    Ok(keys)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(keys)
}

// Part previews deform a cloned original source; the saved source and all other owners remain shared until one commit.
pub struct Preview {
    base: Scene,
    pub scene: Scene,
    target: Target,
    transform: Xform,
}

impl Preview {
    pub fn new(scene: &Scene, target: Target) -> Result<Self, &'static str> {
        points(scene, target)?;
        scene
            .placement(target.owner)
            .and_then(|m| m.inverse())
            .ok_or("Part placement cannot be inverted")?;
        Ok(Self {
            base: scene.clone(),
            scene: scene.clone(),
            target,
            transform: Xform::identity(),
        })
    }
    pub fn scenes(&self) -> [&Scene; 2] {
        [&self.base, &self.scene]
    }
    pub fn matrix(&self) -> &[f64; 16] {
        &self.transform.m
    }
    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) {
            return Err("Invalid part transform");
        }
        let mut scene = self.base.clone();
        if transform.m != Xform::identity().m {
            scene.deform_part(self.target, &transform)?;
        }
        self.scene = scene;
        self.transform = transform;
        Ok(())
    }
    pub fn commit(
        &self,
        scene: &mut Scene,
        history: &mut crate::history::History,
    ) -> Result<bool, &'static str> {
        if !self.target.valid(scene)
            || scene
                .placement(self.target.owner)
                .map(|m| m.m)
                != self
                    .base
                    .placement(self.target.owner)
                    .map(|m| m.m)
            || !same_source(scene, &self.base, self.target.owner)
        {
            return Err("Part source changed; start editing again");
        }
        if self.transform.m == Xform::identity().m {
            return Ok(false);
        }
        history.try_edit(scene, |scene| {
            scene.deform_part(self.target, &self.transform)
        })?;
        Ok(true)
    }
}

fn same_source(a: &Scene, b: &Scene, id: ObjectId) -> bool {
    if let Some(row) = a.objects().iter().find(|r| r.id == id) {
        return b
            .objects()
            .iter()
            .find(|r| r.id == id)
            .and_then(|r| r.geometry())
            .zip(row.geometry())
            .is_some_and(|(a, b)| Rc::ptr_eq(a, b));
    }
    a.paths()
        .iter()
        .find(|r| r.id == id)
        .zip(b.paths().iter().find(|r| r.id == id))
        .is_some_and(|(a, b)| a.prepared.source.owner() == b.prepared.source.owner())
}
```

### 2. `src/edit_preview.rs`

Expose both retained placement-preview scenes for later memory accounting.

**Locate this existing block:**

```rust
    }
    pub fn matrix(&self) -> &[f64; 16] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn scenes(&self) -> [&Scene; 2] {
        [&self.base, &self.scene]
    }
    pub fn matrix(&self) -> &[f64; 16] {
```

## <a id="checkpoint-51s-edit-owner"></a>21. Share one handle interface across whole and part edits

### 1. `src/gizmo_edit.rs`

Unify whole-placement and original-part previews without merging their source ownership.

Create the file and type:

```rust
use crate::{
    gizmo::{Gizmo, Handle},
    pick_sources::Target,
    scene::{ObjectId, Scene},
};
use session_rust::Xform;

pub enum Preview {
    Whole(crate::edit_preview::Preview),
    Part(crate::part_preview::Preview),
}
impl Preview {
    pub fn new(
        scene: &Scene,
        ids: Vec<ObjectId>,
        part: Option<Target>,
    ) -> Result<Self, &'static str> {
        match part {
            Some(target) => {
                crate::part_preview::Preview::new(scene, target).map(Self::Part)
            }
            None => crate::edit_preview::Preview::new(scene, ids).map(Self::Whole),
        }
    }
    pub fn scene(&self) -> &Scene {
        match self {
            Self::Whole(p) => &p.scene,
            Self::Part(p) => &p.scene,
        }
    }
    pub fn scenes(&self) -> [&Scene; 2] {
        match self {
            Self::Whole(p) => p.scenes(),
            Self::Part(p) => p.scenes(),
        }
    }
    pub fn matrix(&self) -> &[f64; 16] {
        match self {
            Self::Whole(p) => p.matrix(),
            Self::Part(p) => p.matrix(),
        }
    }
    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        match self {
            Self::Whole(p) => p.set(transform),
            Self::Part(p) => p.set(transform),
        }
    }
    pub fn commit(
        &self,
        scene: &mut Scene,
        history: &mut crate::history::History,
    ) -> Result<bool, &'static str> {
        match self {
            Self::Whole(p) => p.commit(scene, history),
            Self::Part(p) => p.commit(scene, history),
        }
    }
}
```

### 2. `src/lib.rs`

Register the edit component and supplied checks.

**Locate this existing block:**

```rust
mod widget_tests;

pub mod part_preview;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod widget_tests;

pub mod part_preview;

pub mod gizmo_edit;
```

## <a id="checkpoint-51t-capture"></a>22. Keep pointer ownership through click or drag

### 1. `src/gizmo_edit.rs`

Capture one pointer, distinguish clicks from drags and track the displayed active handle.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub struct Edit {
    pub gizmo: Gizmo,
    pub preview: Preview,
    pub pointer: Option<i32>,
    start: [f32; 2],
    moved: bool,
}
impl Edit {
    pub fn begin(
        mut gizmo: Gizmo,
        handle: Handle,
        ray: &crate::camera::Ray,
        preview: Preview,
        pointer: i32,
        screen: [f32; 2],
    ) -> Option<Self> {
        gizmo.begin(handle, &ray.origin, &ray.direction)?;
        Some(Self {
            gizmo,
            preview,
            pointer: Some(pointer),
            start: screen,
            moved: false,
        })
    }
    pub fn update(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        ray: &crate::camera::Ray,
        shift: bool,
    ) -> Result<bool, &'static str> {
        if self.pointer != Some(pointer) {
            return Ok(false);
        }
        let distance = ((screen[0] - self.start[0]) as f64 * size[0] / 2.)
            .hypot((screen[1] - self.start[1]) as f64 * size[1] / 2.);
        self.moved |= distance > 4.;
        if !self.moved {
            return Ok(true);
        }
        let drag = self
            .gizmo
            .drag
            .as_ref()
            .ok_or("No handle drag is running")?;
        if let Some(transform) =
            self.gizmo
                .update(drag, &ray.origin, &ray.direction, shift)
        {
            self.preview.set(transform)?;
        }
        Ok(true)
    }
    pub fn click(&mut self) -> bool {
        if self.moved {
            return false;
        }
        self.gizmo.typing = self.gizmo.drag.take().map(|d| d.handle);
        self.pointer = None;
        true
    }
    pub fn active(&self) -> Option<Handle> {
        self.gizmo
            .drag
            .as_ref()
            .map(|d| d.handle)
            .or(self.gizmo.typing)
    }
    pub fn origin(&self) -> [f64; 3] {
        let origin = if matches!(self.active(), Some(Handle::Translate(_))) {
            {
                let mut matrix = Xform::identity();
                matrix.m = *self.preview.matrix();
                matrix.transform_point(&self.gizmo.origin)
            }
        } else {
            self.gizmo.origin.clone()
        };
        [origin[0], origin[1], origin[2]]
    }
}

pub fn code(handle: Option<Handle>) -> f32 {
    use crate::gizmo::Axis;
    let axis = |a| match a {
        Axis::X => 0.,
        Axis::Y => 1.,
        Axis::Z => 2.,
    };
    match handle {
        Some(Handle::Translate(a)) => axis(a),
        Some(Handle::Rotate(a)) => 3. + axis(a),
        Some(Handle::Scale(a)) => 6. + axis(a),
        Some(Handle::ScaleUniform) => 9.,
        None => -1.,
    }
}
```

## <a id="checkpoint-51u-selection"></a>23. Place handles on the current editable selection

### 1. `src/editor.rs`

Own a handle preview, derive selection pivots and resolve hits through current camera rays.

**Locate this existing block:**

```rust
pub struct Editor {
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
        self.edit_preview
            .as_ref()
            .map_or(&self.scene, |p| &p.scene)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        .hit(&ray.origin, &ray.direction, scale)
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
```

## <a id="checkpoint-51v-press"></a>24. Capture a valid handle without editing the document

### 1. `src/editor.rs`

Capture checked handle presses and cancel older captures only after successful document commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::Snap(setting) => {
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::GizmoPress {
                pointer,
                screen,
                size,
                modified,
            } => return self.gizmo_press(pointer, screen, size, modified),
            Action::Snap(setting) => {
```

**Locate this block:**

```rust
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
```

Replace it with:

```rust
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
            change = Change::Preview;
        }
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
```

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let keep_edit = matches!(
            &action,
            Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

Replace it with:

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
                | Action::Snap(_)
        );
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

**Locate this block:**

```rust
        };
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
```

Replace it with:

```rust
        };
    }

    fn gizmo_press(
        &mut self,
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
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
        let Some(handle) = self.gizmo_hit(screen, size) else {
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

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
```

**Locate this block:**

```rust
pub enum Action {
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

Replace it with:

```rust
pub enum Action {
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
        size: [f64; 2],
        modified: bool,
    },
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

## <a id="checkpoint-51w-release"></a>25. Preview captured motion and commit one release

### 1. `src/editor.rs`

Update and release one captured handle, preserve click input and commit the current snapped transform once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                size,
                modified,
            } => return self.gizmo_press(pointer, screen, size, modified),
            Action::Snap(setting) => {
```

Replace it with:

```rust
                size,
                modified,
            } => return self.gizmo_press(pointer, screen, size, modified),
            Action::GizmoMove {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_move(pointer, screen, size, shift),
            Action::GizmoRelease {
                pointer,
                screen,
                size,
                shift,
            } => return self.gizmo_release(pointer, screen, size, shift),
            Action::GizmoCancel => {
                self.gizmo_edit = None;
                self.tool_status = "Handle edit cancelled.".into();
                return Ok(Change::Preview);
            }
            Action::Snap(setting) => {
```

**Locate this block:**

```rust
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

Replace it with:

```rust
        let keep_edit = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

**Locate this block:**

```rust
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
```

Replace it with:

```rust
        let keep_gizmo = matches!(
            &action,
            Action::GizmoPress { .. }
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
```

**Locate this block:**

```rust
        Ok(Change::View)
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
```

Replace it with:

```rust
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

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
```

**Locate this block:**

```rust
    },
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

Replace it with:

```rust
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
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

## <a id="checkpoint-51x-number-model"></a>26. Own one handle’s number-field state

### 1. `src/number_box.rs`

Own one projected handle-number prompt and its one-time submitted answer.

Create the file and type:

```rust
#[derive(Clone)]
pub struct NumberPrompt {
    pub handle: crate::gizmo::Handle,
    pub title: String,
    pub unit: &'static str,
    pub hint: &'static str,
    pub at: [f32; 2],
}

#[derive(Default)]
pub struct NumberBox {
    pub number_prompt: Option<NumberPrompt>,
    number_handle: Option<crate::gizmo::Handle>,
    pub number: String,
    pub number_error: String,
    pub number_rect: Option<egui::Rect>,
    typed: Option<String>,
    closed: bool,
}
impl NumberBox {
    pub fn take(&mut self) -> Option<String> {
        if std::mem::take(&mut self.closed) {
            Some("Escape".into())
        } else {
            self.typed.take()
        }
    }
    pub fn set(&mut self, prompt: Option<NumberPrompt>) {
        self.number_prompt = prompt;
        if self.number_prompt.is_none() {
            self.number_handle = None;
            self.number_rect = None;
        }
    }
}
```

### 2. `src/lib.rs`

Register the edit component and supplied checks.

**Locate this existing block:**

```rust
pub mod part_preview;

pub mod gizmo_edit;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod part_preview;

pub mod gizmo_edit;

pub mod number_box;
```

## <a id="checkpoint-51y-number-field"></a>27. Draw the number field beside its handle

### 1. `src/number_box.rs`

Draw the foreground handle-number field with measured bounds and exclusive Enter/Escape ownership.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn draw(
    root: &mut egui::Ui,   // the panel area
    model: &mut NumberBox, // the panel state
    controls: &mut Option<Vec<crate::command_dock::Control>>, // placed controls to draw
) {
    let Some(prompt) = model.number_prompt.as_ref() else {
        model.number_handle = None;
        model.number_rect = None;
        return;
    };
    let id = egui::Id::new("number-input");
    let opened = model.number_handle != Some(prompt.handle);

    if opened {
        model.number_handle = Some(prompt.handle);
        model.number.clear();
        model.number_error.clear();
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let focused = root.memory(|memory| memory.focused());

    if root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        || focused.is_some_and(|other| other != id)
    {
        model.closed = true;
        return;
    }

    if focused.is_none() {
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let enter =
        root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let area = egui::Area::new(egui::Id::new("number-box"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(
            prompt.at[0] + 12.0,
            prompt.at[1] - 12.0,
        ))
        .show(root.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_gray(215),
                ))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.style_mut().override_font_id =
                        Some(egui::FontId::proportional(14.0));
                    ui.horizontal(|ui| {
                        ui.label(&prompt.title);
                        let edit = ui.add_sized(
                            [72.0, 22.0],
                            egui::TextEdit::singleline(&mut model.number)
                                .id(id)
                                .hint_text(prompt.hint)
                                .char_limit(64),
                        );
                        crate::command_dock::record(
                            controls,
                            "number/input",
                            &prompt.title,
                            &edit,
                        );
                        ui.label(prompt.unit);
                    });

                    if !model.number_error.is_empty() {
                        ui.colored_label(
                            egui::Color32::from_rgb(170, 30, 30),
                            &model.number_error,
                        );
                    }
                });
        });
    model.number_rect = Some(area.response.rect);

    if enter {
        model.typed = Some(model.number.clone());
    }
}
```

## <a id="checkpoint-51z-number-submit"></a>28. Apply precise typed handle values once

### 1. `src/editor.rs`

Project the current number request and validate precise, neutral or invalid typed answers before committing. Route Escape to cancellation for an active handle drag as well as numeric entry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.tool_status = "Handle edit cancelled.".into();
                return Ok(Change::Preview);
            }
            Action::Snap(setting) => {
```

Replace it with:

```rust
                self.tool_status = "Handle edit cancelled.".into();
                return Ok(Change::Preview);
            }
            Action::GizmoNumber(text) => return self.gizmo_number(&text),
            Action::Snap(setting) => {
```

**Locate this block:**

```rust
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

Replace it with:

```rust
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::BeginTool(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

**Locate this block:**

```rust
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
```

Replace it with:

```rust
                | Action::GizmoMove { .. }
                | Action::GizmoRelease { .. }
                | Action::GizmoCancel
                | Action::GizmoNumber(_)
                | Action::Zoom(_)
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
```

**Locate this block:**

```rust
    }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
```

Replace it with:

```rust
    }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e| {
            e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")
        }) {
            return Some(Ok(
                if text.trim().eq_ignore_ascii_case("Escape") {
                    Action::GizmoCancel
                } else {
                    Action::GizmoNumber(text.to_owned())
                },
            ));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
```

**Locate this block:**

```rust
        })
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
        .hit(&ray.origin, &ray.direction, scale)
    }

    pub fn gizmo_active(&self) -> f32 {
        crate::gizmo_edit::code(
            self.gizmo_edit
```

Replace it with:

```rust
        .hit(&ray.origin, &ray.direction, scale)
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
```

**Locate this block:**

```rust
    GizmoCancel,
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

Replace it with:

```rust
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen {
        screen: [f32; 2],
```

## <a id="checkpoint-51za-panel"></a>29. Route field input through the shared canvas panel

### 1. `src/panel.rs`

Present and inspect the handle-number model with exclusive pointer, text and keyboard ownership.

**Locate this existing block:**

```rust
pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: Vec<&'static str>,
    ) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            consumed: false,
            pointer_owned: false,
            top: f32::INFINITY,
            controls: None,
            context,
            painter,
            events: Vec::new(),
            output: None,
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [640, 480],
                pixels_per_point: 1.0,
            },
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
        }
    }

    pub fn tool(&mut self, prompt: String, options: Vec<(String, String)>) {
        self.model.drawing_prompt = prompt;
        self.model.options = options;
        if self.model.options.is_empty() {
            self.model.options_rect = None;
        }
        if !self.model.options.is_empty() {
            self.model.completion_visible = false;
            self.model.inline_suffix = false;
        }
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

    pub fn history(&self) -> &std::collections::VecDeque<String> {
        &self.model.history
    }

    pub fn shortcut(
        &self,
        event: &web_sys::KeyboardEvent,
    ) -> Option<crate::shortcuts::Shortcut> {
        if event.type_() != "keydown" || event.repeat() {
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

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() {
            return false;
        }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(),
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            mac_cmd: event.meta_key(),
            command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown"
            && event.key().chars().count() == 1
            && !modifiers.command
            && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() {
                self.model.focus_command = true;
            }
            self.context.memory_mut(|memory| {
                memory.request_focus(egui::Id::new("command-input"))
            });
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() {
            return false;
        }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: event.type_() == "keydown",
                repeat: event.repeat(),
                modifiers,
            });
        }
        if text {
            self.events
                .push(egui::Event::Text(event.key()));
        }
        event.prevent_default();
        true
    }

    pub fn pointer(
        &mut self,
        event: &web_sys::PointerEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| {
                    memory.surrender_focus(egui::Id::new("command-input"))
                });
            }
        }
        self.events
            .push(egui::Event::PointerMoved(pos));
        if matches!(
            event.type_().as_str(),
            "pointercancel" | "lostpointercapture"
        ) {
            for button in [
                egui::PointerButton::Primary,
                egui::PointerButton::Middle,
                egui::PointerButton::Secondary,
            ] {
                self.events.push(egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: false,
                    modifiers: Default::default(),
                });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(
            event.type_().as_str(),
            "pointerdown" | "pointerup"
        ) {
            let button = match event.button() {
                0 => egui::PointerButton::Primary,
                1 => egui::PointerButton::Middle,
                _ => egui::PointerButton::Secondary,
            };
            self.events.push(egui::Event::PointerButton {
                pos,
                button,
                pressed: event.type_() == "pointerdown",
                modifiers: Default::default(),
            });
        }
        if matches!(
            event.type_().as_str(),
            "pointerup" | "pointercancel" | "lostpointercapture"
        ) {
            self.pointer_owned = false;
        }
        if owned {
            event.prevent_default();
        }
        owned
    }

    pub fn wheel(
        &mut self,
        event: &web_sys::WheelEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        self.events
            .push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() {
                1 => egui::MouseWheelUnit::Line,
                2 => egui::MouseWheelUnit::Page,
                _ => egui::MouseWheelUnit::Point,
            },
            phase: egui::TouchPhase::Move,
            delta: egui::vec2(
                -event.delta_x() as f32,
                -event.delta_y() as f32,
            ),
            modifiers: Default::default(),
        });
        let over = pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        if over {
            event.prevent_default();
        }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
            "options": self.model.options, "options_rect": self.model.options_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        self.model
            .remember(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn result(&mut self, message: &str) {
        self.model.status = message.into();
        if let Some(last) = self.model.history.back_mut()
            && let Some(at) = last.find('\n')
        {
            last.truncate(at + 1);
            last.push_str(message);
        }
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let rect = canvas.get_bounding_client_rect();
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / rect.width() as f32;
        self.consumed = false;
        if let Some(event) = event {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                self.consumed = self.key(key);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                self.consumed = self.wheel(wheel, canvas);
            }
        }
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else {
            return Ok(None);
        };
        if !self.model.options.is_empty() {
            self.answer(&line, "Tool input submitted.");
            return Ok(Some(line));
        }
        if line == "Escape" {
            return Ok(None);
        }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line))
    }

    fn prepare(&mut self) -> Option<String> {
        let size = self.screen.size_in_pixels;
        let mut input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32)
                    / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .native_pixels_per_point = Some(self.screen.pixels_per_point);
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(
                root,
                &mut self.model,
                &mut self.controls,
                &mut line,
                &self.commands,
                true,
            );
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Panel {
    number: crate::number_box::NumberBox,
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: Vec<&'static str>,
    ) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            number: Default::default(),
            commands: Commands(commands),
            consumed: false,
            pointer_owned: false,
            top: f32::INFINITY,
            controls: None,
            context,
            painter,
            events: Vec::new(),
            output: None,
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [640, 480],
                pixels_per_point: 1.0,
            },
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
        }
    }

    pub fn number(&mut self, prompt: Option<crate::number_box::NumberPrompt>) {
        if prompt.is_some() {
            self.model.focus_command = false;
        } else if self.number.number_prompt.is_some() {
            self.context
                .memory_mut(|m| m.surrender_focus(egui::Id::new("number-input")));
        }
        self.number.set(prompt);
    }

    pub fn tool(&mut self, prompt: String, options: Vec<(String, String)>) {
        self.model.drawing_prompt = prompt;
        self.model.options = options;
        if self.model.options.is_empty() {
            self.model.options_rect = None;
        }
        if !self.model.options.is_empty() {
            self.model.completion_visible = false;
            self.model.inline_suffix = false;
        }
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

    pub fn history(&self) -> &std::collections::VecDeque<String> {
        &self.model.history
    }

    pub fn shortcut(
        &self,
        event: &web_sys::KeyboardEvent,
    ) -> Option<crate::shortcuts::Shortcut> {
        if self.number.number_prompt.is_some()
            || event.type_() != "keydown"
            || event.repeat()
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

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() {
            return false;
        }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(),
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            mac_cmd: event.meta_key(),
            command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown"
            && event.key().chars().count() == 1
            && !modifiers.command
            && !modifiers.alt;
        if text {
            let number = self.number.number_prompt.is_some();
            if !self.context.egui_wants_keyboard_input() && !number {
                self.model.focus_command = true;
            }
            self.context.memory_mut(|memory| {
                memory.request_focus(egui::Id::new(if number {
                    "number-input"
                } else {
                    "command-input"
                }))
            });
            if !number {
                self.model.command_open = true;
            }
        }
        if !self.context.egui_wants_keyboard_input() {
            return false;
        }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: event.type_() == "keydown",
                repeat: event.repeat(),
                modifiers,
            });
        }
        if text {
            self.events
                .push(egui::Event::Text(event.key()));
        }
        event.prevent_default();
        true
    }

    pub fn pointer(
        &mut self,
        event: &web_sys::PointerEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = self
            .number
            .number_rect
            .is_some_and(|r| r.contains(pos))
            || pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| {
                    memory.surrender_focus(egui::Id::new("command-input"))
                });
            }
        }
        self.events
            .push(egui::Event::PointerMoved(pos));
        if matches!(
            event.type_().as_str(),
            "pointercancel" | "lostpointercapture"
        ) {
            for button in [
                egui::PointerButton::Primary,
                egui::PointerButton::Middle,
                egui::PointerButton::Secondary,
            ] {
                self.events.push(egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: false,
                    modifiers: Default::default(),
                });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(
            event.type_().as_str(),
            "pointerdown" | "pointerup"
        ) {
            let button = match event.button() {
                0 => egui::PointerButton::Primary,
                1 => egui::PointerButton::Middle,
                _ => egui::PointerButton::Secondary,
            };
            self.events.push(egui::Event::PointerButton {
                pos,
                button,
                pressed: event.type_() == "pointerdown",
                modifiers: Default::default(),
            });
        }
        if matches!(
            event.type_().as_str(),
            "pointerup" | "pointercancel" | "lostpointercapture"
        ) {
            self.pointer_owned = false;
        }
        if owned {
            event.prevent_default();
        }
        owned
    }

    pub fn wheel(
        &mut self,
        event: &web_sys::WheelEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        self.events
            .push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() {
                1 => egui::MouseWheelUnit::Line,
                2 => egui::MouseWheelUnit::Page,
                _ => egui::MouseWheelUnit::Point,
            },
            phase: egui::TouchPhase::Move,
            delta: egui::vec2(
                -event.delta_x() as f32,
                -event.delta_y() as f32,
            ),
            modifiers: Default::default(),
        });
        let over = self
            .number
            .number_rect
            .is_some_and(|r| r.contains(pos))
            || pos.y >= self.top
            || (self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos))
                || self
                    .model
                    .options_rect
                    .is_some_and(|r| r.contains(pos)));
        if over {
            event.prevent_default();
        }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"number":self.number.number,"number_error":self.number.number_error,"number_rect":self.number.number_rect.map(|r|[r.min.x,r.min.y,r.max.x,r.max.y]),"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
            "options": self.model.options, "options_rect": self.model.options_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        self.model
            .remember(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn result(&mut self, message: &str) {
        self.model.status = message.into();
        if self.number.number_prompt.is_some() {
            self.number.number_error = message.into();
        }
        if let Some(last) = self.model.history.back_mut()
            && let Some(at) = last.find('\n')
        {
            last.truncate(at + 1);
            last.push_str(message);
        }
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let rect = canvas.get_bounding_client_rect();
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / rect.width() as f32;
        self.consumed = false;
        if let Some(event) = event {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                self.consumed = self.key(key);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                self.consumed = self.wheel(wheel, canvas);
            }
        }
        let number_open = self.number.number_prompt.is_some();
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else {
            return Ok(None);
        };
        if number_open {
            return Ok(Some(line));
        }
        if !self.model.options.is_empty() {
            self.answer(&line, "Tool input submitted.");
            return Ok(Some(line));
        }
        if line == "Escape" {
            return Ok(None);
        }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line))
    }

    fn prepare(&mut self) -> Option<String> {
        let size = self.screen.size_in_pixels;
        let mut input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32)
                    / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .native_pixels_per_point = Some(self.screen.pixels_per_point);
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            crate::number_box::draw(root, &mut self.number, &mut self.controls);
            command_dock::draw(
                root,
                &mut self.model,
                &mut self.controls,
                &mut line,
                &self.commands,
                true,
            );
        });
        if let Some(number) = self.number.take() {
            line = Some(number);
        }
        if let Some(control) = self.controls.as_ref().and_then(|items| {
```

## <a id="checkpoint-51zb-presentation"></a>30. Update selected handles with the actual camera

### 1. `src/browser.rs`

Prepare the selected widget from the latest resized camera and expose its own presentation ledger. Remove diagnostics before viewport and widget preparation. Highlight the current idle handle and report the prepared widget after resizing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .iter()
        .map(|row| (format!("{:?}", row.id), row.model.m))
        .collect();
    canvas.set_attribute(
```

Replace it with:

```rust
        .iter()
        .map(|row| (format!("{:?}", row.id), row.model.m))
        .collect();
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
                    ));
                    return;
                }
                if let Err(error) = present(
                    &surface,
                    &renderer,
```

Replace it with:

```rust
                    ));
                    return;
                }
                let hover = event
                    .dyn_ref::<web_sys::PointerEvent>()
                    .filter(|p| {
                        event.type_() == "pointermove"
                            && p.buttons() == 0
                            && !panel.consumed
                    })
                    .map(|p| {
                        let r = pointer_canvas.get_bounding_client_rect();
                        (
                            [
                                (2. * (p.client_x() as f64 - r.left()) / r.width() - 1.)
                                    as f32,
                                (1. - 2. * (p.client_y() as f64 - r.top()) / r.height())
                                    as f32,
                            ],
                            [r.width(), r.height()],
                        )
                    });
                editor.hover_gizmo(hover);
                renderer.set_gizmo(
                    &editor.camera,
                    editor.gizmo_origin(),
                    editor.gizmo_active(),
                );
                if let Err(error) = inspect(&editor, &renderer) {
                    report(&format!(
                        "Cannot report projection: {error:?}"
                    ));
                    return;
                }
                if let Err(error) = present(
                    &surface,
                    &renderer,
```

**Locate this block:**

```rust
            }
            if let Err(error) = inspect(&editor, &renderer) {
                report(&format!(
                    "Cannot report projection: {error:?}"
                ));
                return;
            }
            if resize(
                &browser_window,
                &pointer_canvas,
```

Replace it with:

```rust
            }
            if resize(
                &browser_window,
                &pointer_canvas,
```

## <a id="checkpoint-51zc-pointer"></a>31. Capture actual browser handle drags

### 1. `src/browser.rs`

Capture one actual handle pointer and route its matching motion and release before source picking.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    frame.present();
    Ok(())
}
```

Replace it with:

```rust
    frame.present();
    Ok(())
}

fn gizmo_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    editor: &Editor,
    consumed: bool,
) -> Option<Action> {
    if matches!(event.type_().as_str(), "blur" | "resize")
        && editor.gizmo_edit.is_some()
    {
        return Some(Action::GizmoCancel);
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    if matches!(
        event.type_().as_str(),
        "pointercancel" | "lostpointercapture"
    ) {
        return editor
            .gizmo_edit
            .as_ref()
            .filter(|e| e.pointer == Some(pointer.pointer_id()))
            .map(|_| Action::GizmoCancel);
    }
    let rect = canvas.get_bounding_client_rect();
    let size = [rect.width(), rect.height()];
    if size
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.)
    {
        return None;
    }
    let screen = [
        (2. * (pointer.client_x() as f64 - rect.left()) / size[0] - 1.) as f32,
        (1. - 2. * (pointer.client_y() as f64 - rect.top()) / size[1]) as f32,
    ];
    let id = pointer.pointer_id();
    if editor
        .gizmo_edit
        .as_ref()
        .is_some_and(|e| e.pointer == Some(id))
    {
        event.prevent_default();
        return match event.type_().as_str() {
            "pointermove" => Some(Action::GizmoMove {
                pointer: id,
                screen,
                size,
                shift: pointer.shift_key(),
            }),
            "pointerup" => {
                let _ = canvas.release_pointer_capture(id);
                Some(Action::GizmoRelease {
                    pointer: id,
                    screen,
                    size,
                    shift: pointer.shift_key(),
                })
            }
            _ => None,
        };
    }
    if consumed
        || event.type_() != "pointerdown"
        || !pointer.is_primary()
        || pointer.button() != 0
    {
        return None;
    }
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
    } else if editor.gizmo_edit.is_some() {
        Some(Action::GizmoCancel)
    } else {
        None
    }
}
```

**Locate this block:**

```rust
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::PickedMany {
                        target: hit
```

Replace it with:

```rust
            };
            let handle_action = gizmo_action(
                &event,
                &pointer_canvas,
                &editor,
                panel.consumed,
            );
            let action = if let Some(handle_action) = handle_action {
                renderer.cancel_pick();
                gesture.cancel();
                Some(handle_action)
            } else if event.type_() == "viewer-pick" {
                match renderer.take_pick(editor.camera.uniform()) {
                    Some(Ok(hit)) => Some(Action::PickedMany {
                        target: hit
```

## <a id="checkpoint-51zd-interruption"></a>32. Restore previews when input ownership ends

### 1. `src/browser.rs`

Synchronize number fields and restore temporary source presentation on real input interruption.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &format!("{:?}", renderer.stats()),
    )?;
    let cpu = crate::memory::cpu(
        editor.scenes(),
        renderer
```

Replace it with:

```rust
        &format!("{:?}", renderer.stats()),
    )?;
    let cpu = crate::memory::cpu(
        editor.presentation_scenes(),
        renderer
```

**Locate this block:**

```rust
    )?;
    canvas.set_attribute(
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
```

Replace it with:

```rust
    )?;
    canvas.set_attribute(
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
```

**Locate this block:**

```rust
    )?;
    canvas.set_attribute(
        "data-path-cpu",
        &serde_json::to_string(&crate::memory::paths(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-stroke-usage",
```

Replace it with:

```rust
    )?;
    canvas.set_attribute(
        "data-path-cpu",
        &serde_json::to_string(&crate::memory::paths(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-stroke-usage",
```

**Locate this block:**

```rust
    )?;
    canvas.set_attribute(
        "data-point-cpu",
        &serde_json::to_string(&crate::memory::points(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-path-gpu",
```

Replace it with:

```rust
    )?;
    canvas.set_attribute(
        "data-point-cpu",
        &serde_json::to_string(&crate::memory::points(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-path-gpu",
```

**Locate this block:**

```rust
    )?;
    canvas.set_attribute(
        "data-curve-cpu",
        &serde_json::to_string(&crate::memory::curves(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-control-count",
```

Replace it with:

```rust
    )?;
    canvas.set_attribute(
        "data-curve-cpu",
        &serde_json::to_string(&crate::memory::curves(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-control-count",
```

**Locate this block:**

```rust
            } else {
                panel.tool(String::new(), Vec::new());
            }
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
                panel.result(error);
            }
```

Replace it with:

```rust
            } else {
                panel.tool(String::new(), Vec::new());
            }
            let rect = pointer_canvas.get_bounding_client_rect();
            let number = editor.number_prompt([rect.width(), rect.height()]);
            if editor
                .gizmo_edit
                .as_ref()
                .is_some_and(|e| e.gizmo.typing.is_some())
                && number.is_none()
            {
                if editor.apply(Action::GizmoCancel).is_ok() {
                    highlight_changed = true;
                    crate::browser_upload::prepare(
                        &mut renderer,
                        editor.display_scene(),
                        editor.whole_selection(),
                        active_fault.clone(),
                    );
                }
            }
            panel.number(number);
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
                panel.result(error);
            }
```

**Locate this block:**

```rust
                        | Action::ToolPoint(_)
                        | Action::CancelTool
                        | Action::Snap(_)
                        | Action::ToolScreen { commit: true, .. }
                );
                if matches!(&action, Action::CancelTool) {
```

Replace it with:

```rust
                        | Action::ToolPoint(_)
                        | Action::CancelTool
                        | Action::Snap(_)
                        | Action::GizmoRelease { .. }
                        | Action::GizmoCancel
                        | Action::GizmoNumber(_)
                        | Action::ToolScreen { commit: true, .. }
                );
                if matches!(&action, Action::CancelTool) {
```

**Locate this block:**

```rust
                            highlight_changed = true;
                            crate::browser_upload::prepare(
                                &mut renderer,
                                &editor.scene,
                                editor.whole_selection(),
                                active_fault.clone(),
                            );
```

Replace it with:

```rust
                            highlight_changed = true;
                            crate::browser_upload::prepare(
                                &mut renderer,
                                editor.display_scene(),
                                editor.whole_selection(),
                                active_fault.clone(),
                            );
```

**Locate this block:**

```rust
                }
            };

            let line = if editor.draft.is_some()
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .is_some_and(|key| {
```

Replace it with:

```rust
                }
            };

            let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some())
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .is_some_and(|key| {
```

**Locate this block:**

```rust
                gesture.cancel();
                if event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted())
```

Replace it with:

```rust
                gesture.cancel();
                let _ = editor.apply(Action::GizmoCancel);
                crate::browser_upload::prepare(
                    &mut renderer,
                    editor.display_scene(),
                    editor.whole_selection(),
                    active_fault.clone(),
                );
                renderer.set_gizmo(&editor.camera, None, -1.);
                if event
                    .dyn_ref::<web_sys::PageTransitionEvent>()
                    .is_some_and(|event| event.persisted())
```

### 2. `src/editor.rs`

Expose saved and captured preview scenes before the browser reports their source allocations.

**Locate this existing block:**

```rust
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

## <a id="checkpoint-51ze-ledger"></a>33. Count temporary source owners without saving them

### 1. `src/editor.rs`

Count both retained preview scenes and revoke captures after successful direct source mutations. Clear idle highlighting after successful commands change its source or camera.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
```

Replace it with:

```rust
                | Action::Grid(_)
        );
        let mut change = self.apply_action(action)?;
        self.gizmo_hover = None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
```

**Locate this block:**

```rust
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
            self.selected = self.objects.primary();
            self.selected_part = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
        if self.edit_preview.take().is_some() {
            self.draft = None;
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
        }
        self.snap_index = None;
        Ok(true)
    }
```

Replace it with:

```rust
        }
        self.gizmo_edit = None;
        self.snap_index = None;
        Ok(true)
    }
```

## <a id="checkpoint-51zf-handles"></a>34. Verify complete handle editing and input

### 1. `src/gizmo_edit_tests.rs`

Copy original-source, pointer-ID, stale-edit, rational-weight, one-Undo and preview-ledger regression checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Change, Editor},
    gizmo::{Axis, Gizmo, Handle},
    pick_sources::{Part, Target},
};
use session_rust::{Point, Xform};
use std::rc::Rc;
const SIZE: [f64; 2] = [640., 480.];

fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::AddLine).unwrap();
    e.apply(Action::AddPoint).unwrap();
    e.apply(Action::SelectAll).unwrap();
    e.apply(Action::Fit).unwrap();
    e
}
fn screen(e: &Editor, p: Point) -> [f32; 2] {
    let p = e
        .camera
        .view_projection()
        .transform_point(&p);
    [p[0] as f32, p[1] as f32]
}
fn handle(e: &Editor, handle: Handle) -> [f32; 2] {
    let p = e.gizmo_origin().unwrap();
    let scale =
        crate::widget_layout::world_per_pixel(&e.camera.view_projection(), p, SIZE[1])
            .unwrap();
    screen(
        e,
        Gizmo::new(Point::new(p[0], p[1], p[2])).handle_point(handle, scale),
    )
}
fn press(e: &mut Editor, handle_: Handle) -> [f32; 2] {
    let p = handle(e, handle_);
    assert_eq!(e.gizmo_hit(p, SIZE), Some(handle_));
    e.apply(Action::GizmoPress {
        pointer: 7,
        screen: p,
        size: SIZE,
        modified: false,
    })
    .unwrap();
    assert_eq!(
        e.gizmo_edit.as_ref().unwrap().active(),
        Some(handle_)
    );
    p
}
fn click(e: &mut Editor, h: Handle) {
    let p = press(e, h);
    e.apply(Action::GizmoRelease {
        pointer: 7,
        screen: p,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    assert_eq!(
        e.gizmo_edit.as_ref().unwrap().gizmo.typing,
        Some(h)
    );
}
fn models(e: &Editor) -> Vec<[f64; 16]> {
    e.scene
        .objects()
        .iter()
        .map(|r| r.id)
        .chain(e.scene.lines().iter().map(|r| r.id))
        .chain(e.scene.points().iter().map(|r| r.id))
        .map(|id| e.scene.placement(id).unwrap().m)
        .collect()
}

#[test]
fn handle_drag_shares_original_sources_keeps_save_unchanged_and_commits_once() {
    let mut e = editor();
    let before = models(&e);
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let display = Rc::clone(&e.scene.objects()[0].mesh);
    let p = press(&mut e, Handle::Translate(Axis::X));
    let moved = [p[0] + 0.1, p[1]];
    e.apply(Action::GizmoMove {
        pointer: 8,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    assert_eq!(
        e.gizmo_edit
            .as_ref()
            .unwrap()
            .preview
            .matrix(),
        &Xform::identity().m
    );
    assert_eq!(
        e.apply(Action::GizmoMove {
            pointer: 7,
            screen: moved,
            size: SIZE,
            shift: false
        })
        .unwrap(),
        Change::Preview
    );
    assert_eq!(models(&e), before);
    assert!(Rc::ptr_eq(
        &source,
        e.display_scene().objects()[0]
            .geometry()
            .unwrap()
    ));
    assert!(Rc::ptr_eq(
        &display,
        &e.display_scene().objects()[0].mesh
    ));
    assert!(e.display_scene().objects()[0].model.m[12] != before[0][12]);
    e.apply(Action::GizmoRelease {
        pointer: 8,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    assert!(e.gizmo_edit.is_some());
    e.apply(Action::GizmoRelease {
        pointer: 7,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    let after = models(&e);
    assert_ne!(after, before);
    assert!(e.gizmo_edit.is_none());
    e.apply(Action::Undo).unwrap();
    assert_eq!(models(&e), before);
    e.apply(Action::Redo).unwrap();
    assert_eq!(models(&e), after);
    e.apply(Action::Undo).unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.points().is_empty(),
        "The whole handle drag owns exactly one history entry"
    );
}

#[test]
fn numeric_handles_keep_precise_values_and_invalid_answers_preserve_selection_and_redo()
{
    let mut e = editor();
    let before = models(&e);
    click(&mut e, Handle::Translate(Axis::X));
    for text in ["NaN", "1e999", "nonsense"] {
        assert!(
            e.apply(Action::GizmoNumber(text.into()))
                .is_err()
        );
        assert!(e.gizmo_edit.is_some());
        assert_eq!(models(&e), before);
    }
    let value = 0.123456789012345;
    let action = e
        .tool_line(&value.to_string())
        .unwrap()
        .unwrap();
    e.apply(action).unwrap();
    let after = models(&e);
    assert!((after[0][12] - before[0][12] - value).abs() < 1e-15);
    e.apply(Action::Undo).unwrap();
    click(&mut e, Handle::ScaleUniform);
    assert!(
        e.apply(Action::GizmoNumber("0".into()))
            .is_err()
    );
    assert!(e.gizmo_edit.is_some());
    e.apply(Action::GizmoCancel).unwrap();
    click(&mut e, Handle::ScaleUniform);
    e.apply(Action::GizmoNumber("1".into()))
        .unwrap();
    e.apply(Action::Redo).unwrap();
    assert_eq!(models(&e), after);
}

#[test]
fn modified_presses_and_point_drafts_keep_handle_capture_out_of_selection_and_drawing()
{
    let mut e = editor();
    let p = handle(&e, Handle::Translate(Axis::X));
    e.apply(Action::GizmoPress {
        pointer: 7,
        screen: p,
        size: SIZE,
        modified: true,
    })
    .unwrap();
    assert!(e.gizmo_edit.is_none());
    e.apply(Action::BeginTool(crate::tool::Shape::Line))
        .unwrap();
    assert!(e.gizmo_origin().is_none());
    e.apply(Action::GizmoPress {
        pointer: 7,
        screen: p,
        size: SIZE,
        modified: false,
    })
    .unwrap();
    assert!(e.gizmo_edit.is_none());
    e.apply(Action::CancelTool).unwrap();
    e.apply(Action::BeginTool(crate::tool::Shape::Move))
        .unwrap();
    e.apply(Action::ToolPoint([0.; 3])).unwrap();
    e.apply(Action::GizmoPress {
        pointer: 7,
        screen: p,
        size: SIZE,
        modified: false,
    })
    .unwrap();
    assert!(e.edit_preview.is_some());
    assert_eq!(
        e.draft.as_ref().unwrap().points,
        vec![[0.; 3]]
    );
    e.apply(Action::CancelTool).unwrap();
    press(&mut e, Handle::Translate(Axis::X));
    e.apply(Action::Close).unwrap();
    assert!(e.gizmo_edit.is_none());
    assert!(e.scene.bounds().is_none());
}

#[test]
fn stale_documents_and_invalid_drag_answers_cannot_publish_a_handle_preview() {
    let mut e = editor();
    let p = press(&mut e, Handle::Translate(Axis::X));
    let moved = [p[0] + 0.08, p[1]];
    e.apply(Action::GizmoMove {
        pointer: 7,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    let preview = *e
        .gizmo_edit
        .as_ref()
        .unwrap()
        .preview
        .matrix();
    let before = models(&e);
    assert!(
        e.apply(Action::GizmoMove {
            pointer: 7,
            screen: [f32::NAN, 0.],
            size: SIZE,
            shift: false
        })
        .is_err()
    );
    assert_eq!(
        *e.gizmo_edit
            .as_ref()
            .unwrap()
            .preview
            .matrix(),
        preview
    );
    assert_eq!(models(&e), before);
    let id = e.scene.objects()[0].id;
    e.scene.translate(id, [0.2, 0., 0.]).unwrap();
    let changed = models(&e);
    assert!(
        e.apply(Action::GizmoRelease {
            pointer: 7,
            screen: moved,
            size: SIZE,
            shift: false
        })
        .is_err()
    );
    assert_eq!(models(&e), changed);
    assert!(e.gizmo_edit.is_some());
    e.apply(Action::GizmoCancel).unwrap();
}

#[test]
fn face_and_edge_previews_use_original_keys_and_world_to_local_placement() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    let mut model = Xform::scale_xyz(2., 3., 4.);
    model.m[4] = 0.7;
    model.m[12] = 12.3456789012345;
    e.scene.place(id, model.clone()).unwrap();
    let source = Rc::clone(e.scene.objects()[0].geometry().unwrap());
    let face = source.faces()[0];
    let target = Target {
        owner: id,
        part: Part::Face(face),
    };
    let keys = crate::part_preview::keys(&source, target.part).unwrap();
    let mut p = crate::part_preview::Preview::new(&e.scene, target).unwrap();
    p.set(Xform::translation(
        0.123456789012345,
        0.25,
        -0.5,
    ))
    .unwrap();
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    let edited = p.scene.objects()[0].geometry().unwrap();
    assert_eq!(edited.guid(), source.guid());
    assert_eq!(edited.face, source.face);
    assert_eq!(edited.face_holes, source.face_holes);
    for key in source.vertices() {
        let a = source.vertex_point(key).unwrap();
        let b = edited.vertex_point(key).unwrap();
        if keys.contains(&key) {
            let a = model.transform_point(&a);
            let b = model.transform_point(&b);
            for i in 0..3 {
                assert!(
                    (b[i] - a[i] - [0.123456789012345, 0.25, -0.5][i]).abs() < 1e-12
                );
            }
        } else {
            for i in 0..3 {
                assert_eq!(a[i], b[i]);
            }
        }
    }
    let mut history = crate::history::History::default();
    assert!(p.commit(&mut e.scene, &mut history).unwrap());
    assert!(history.undo(&mut e.scene));
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    assert!(history.redo(&mut e.scene));
    let (a, b) = source.edges()[0];
    assert_eq!(
        crate::part_preview::keys(&source, Part::MeshEdge(a, b)).unwrap(),
        vec![a, b]
    );
}

#[test]
fn face_hole_keys_deduplicate_and_curve_control_edits_preserve_weights_and_knots() {
    let mut source = session_rust::Mesh::new();
    for key in 0..8 {
        source.add_vertex(Point::new(key as f64, 0., 0.), Some(key));
    }
    source.add_face(vec![0, 1, 2, 3], Some(37));
    source.set_face_holes(37, vec![vec![4, 5, 6, 7, 4]]);
    assert_eq!(
        crate::part_preview::keys(&source, Part::Face(37)).unwrap(),
        (0..8).collect::<Vec<_>>()
    );
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    let mut curve = session_rust::NurbsCurve::create(
        true,
        2,
        &[
            Point::new(-0.6, -0.4, 0.),
            Point::new(0., 0.6, 0.),
            Point::new(0.6, -0.4, 0.),
        ],
    );
    assert!(curve.set_cv_4d(1, 0., 1.2, 0., 2.));
    let prepared = crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap();
    e.scene.insert_path(prepared).unwrap();
    e.apply(Action::Controls(true)).unwrap();
    let id = e.scene.paths()[0].id;
    let crate::chain::Source::Curve(source, _) =
        e.scene.paths()[0].prepared.source.clone()
    else {
        panic!("Curve");
    };
    let target = Target {
        owner: id,
        part: Part::Control(1),
    };
    let mut p = crate::part_preview::Preview::new(&e.scene, target).unwrap();
    p.set(Xform::translation(
        0.123456789012345,
        0.25,
        0.,
    ))
    .unwrap();
    let crate::chain::Source::Curve(edited, _) = &p.scene.paths()[0].prepared.source
    else {
        panic!("Curve");
    };
    assert_eq!(edited.guid(), source.guid());
    assert_eq!(edited.m_nurbsknot, source.m_nurbsknot);
    assert_eq!(edited.m_is_rat, source.m_is_rat);
    assert!(edited.m_is_rat);
    for index in 0..source.m_cv_count {
        let weight = index * source.m_cv_stride + source.m_dim;
        assert_eq!(edited.m_cv[weight], source.m_cv[weight]);
    }
    for index in 0..source.m_cv_count {
        let a = source.get_cv(index).unwrap();
        let b = edited.get_cv(index).unwrap();
        for i in 0..3 {
            assert!(
                (b[i]
                    - a[i]
                    - if index == 1 {
                        [0.123456789012345, 0.25, 0.][i]
                    } else {
                        0.
                    })
                .abs()
                    < 1e-14
            );
        }
    }
    let mut history = crate::history::History::default();
    assert!(p.commit(&mut e.scene, &mut history).unwrap());
    assert!(history.undo(&mut e.scene));
    let crate::chain::Source::Curve(restored, _) = &e.scene.paths()[0].prepared.source
    else {
        panic!("Curve");
    };
    assert!(Rc::ptr_eq(restored, &source));
}

#[test]
fn changed_part_source_is_refused_and_cancelled_preview_releases_derived_geometry() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    let target = Target {
        owner: id,
        part: Part::Face(
            e.scene.objects()[0]
                .geometry()
                .unwrap()
                .faces()[0],
        ),
    };
    let mut p = crate::part_preview::Preview::new(&e.scene, target).unwrap();
    p.set(Xform::translation(0.1, 0., 0.))
        .unwrap();
    let derived = Rc::downgrade(p.scene.objects()[0].geometry().unwrap());
    e.scene
        .deform_part(target, &Xform::translation(0.2, 0., 0.))
        .unwrap();
    assert!(
        p.commit(
            &mut e.scene,
            &mut crate::history::History::default()
        )
        .is_err()
    );
    drop(p);
    assert!(derived.upgrade().is_none());
}

#[test]
fn part_selection_places_its_handle_at_the_average_original_world_points() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    let face = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .faces()[0];
    let target = Target {
        owner: id,
        part: Part::Face(face),
    };
    let model = e.scene.placement(id).unwrap().m;
    e.apply(Action::PickedTarget(Some(target)))
        .unwrap();
    assert_eq!(
        e.gizmo_origin(),
        crate::part_preview::origin(&e.scene, target)
    );
    click(&mut e, Handle::Translate(Axis::X));
    e.apply(Action::GizmoNumber("0.25".into()))
        .unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, model);
    e.apply(Action::Undo).unwrap();
    assert!(e.gizmo_edit.is_none());
}

#[test]
fn release_uses_current_shift_state_for_quarter_turn_commit() {
    let mut e = editor();
    let origin = e.gizmo_origin().unwrap();
    let scale = crate::widget_layout::world_per_pixel(
        &e.camera.view_projection(),
        origin,
        SIZE[1],
    )
    .unwrap();
    let start = press(&mut e, Handle::Rotate(Axis::Z));
    let angle = (-75f64).to_radians();
    let p = screen(
        &e,
        Point::new(
            origin[0] + crate::gizmo::ARM * scale * angle.cos(),
            origin[1] + crate::gizmo::ARM * scale * angle.sin(),
            origin[2],
        ),
    );
    e.apply(Action::GizmoMove {
        pointer: 7,
        screen: p,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    let free = *e
        .gizmo_edit
        .as_ref()
        .unwrap()
        .preview
        .matrix();
    assert!((free[0] - 0.5).abs() < 1e-5);
    e.apply(Action::GizmoRelease {
        pointer: 7,
        screen: p,
        size: SIZE,
        shift: true,
    })
    .unwrap();
    let first = e.scene.objects()[0].model.clone();
    assert!(first.m[0].abs() < 1e-12 && (first.m[1] - 1.).abs() < 1e-12);
    e.apply(Action::Undo).unwrap();
    assert!(e.gizmo_edit.is_none());
    assert_ne!(start, p);
}

#[test]
fn memory_counts_part_preview_sources_and_releases_them_on_cancel() {
    let mut e = editor();
    let before = crate::memory::cpu(e.presentation_scenes(), std::iter::empty());
    let id = e.scene.objects()[0].id;
    let target = Target {
        owner: id,
        part: Part::Face(
            e.scene.objects()[0]
                .geometry()
                .unwrap()
                .faces()[0],
        ),
    };
    e.apply(Action::PickedTarget(Some(target)))
        .unwrap();
    let start = press(&mut e, Handle::Translate(Axis::X));
    e.apply(Action::GizmoMove {
        pointer: 7,
        screen: [start[0] + 0.1, start[1]],
        size: SIZE,
        shift: false,
    })
    .unwrap();
    let during = crate::memory::cpu(e.presentation_scenes(), std::iter::empty());
    assert_eq!(during[1], before[1] + 1);
    assert_eq!(during[3], before[3] + 1);
    assert_eq!(during[5], before[5] + 2);
    e.apply(Action::GizmoCancel).unwrap();
    assert_eq!(
        crate::memory::cpu(e.presentation_scenes(), std::iter::empty()),
        before
    );
}

#[test]
fn idle_handle_hover_changes_only_highlight_and_releases_on_commands_and_drafts() {
    let mut e = editor();
    let before = models(&e);
    let p = handle(&e, Handle::Translate(Axis::X));
    e.hover_gizmo(Some((p, SIZE)));
    assert_eq!(e.gizmo_active(), 0.);
    assert!(e.gizmo_edit.is_none());
    assert_eq!(models(&e), before);
    e.hover_gizmo(None);
    assert_eq!(e.gizmo_active(), -1.);
    e.hover_gizmo(Some((p, SIZE)));
    e.apply(Action::Zoom(1.)).unwrap();
    assert_eq!(e.gizmo_active(), -1.);
    assert_eq!(models(&e), before);
    e.apply(Action::BeginTool(crate::tool::Shape::Line))
        .unwrap();
    e.hover_gizmo(Some((p, SIZE)));
    assert_eq!(e.gizmo_active(), -1.);
    assert!(e.gizmo_edit.is_none());
    e.apply(Action::CancelTool).unwrap();
    e.apply(Action::Undo).unwrap();
    assert!(
        e.scene.points().is_empty(),
        "Hover created no history entry"
    );
}
```

### 2. `src/number_box_tests.rs`

Copy actual egui field-focus, text, Enter/Escape, rectangle and handle-switch regression checks.

Create the file and type:

```rust
use crate::{
    command_dock::*,
    gizmo::{Axis, Handle},
    number_box::{NumberBox, NumberPrompt},
};

struct Vocabulary;
impl Commands for Vocabulary {
    fn canonical(&self, line: &str) -> String {
        line.into()
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (line.into(), true)
    }
    fn completions(&self, _: &str) -> Vec<&'static str> {
        vec!["Help"]
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}
fn context() -> egui::Context {
    let context = egui::Context::default();
    context.options_mut(|o| o.max_passes = 1.try_into().unwrap());
    context.set_fonts(theme::fonts([
        include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
    ]));
    context
}
fn prompt(handle: Handle) -> NumberPrompt {
    NumberPrompt {
        handle,
        title: handle.title(),
        unit: handle.labels().2,
        hint: if handle.labels().2 == "factor" {
            "1"
        } else {
            "0"
        },
        at: [300., 200.],
    }
}
fn frame(
    context: &egui::Context,
    number: &mut NumberBox,
    model: &mut CommandLine,
    events: Vec<egui::Event>,
) -> (Option<String>, Option<String>, Vec<Control>) {
    let mut line = None;
    let mut controls = Some(Vec::new());
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640., 480.),
            )),
            focused: true,
            events,
            ..Default::default()
        },
        |root| {
            crate::number_box::draw(root, number, &mut controls);
            draw(
                root,
                model,
                &mut controls,
                &mut line,
                &Vocabulary,
                true,
            );
        },
    );
    (number.take(), line, controls.unwrap())
}
fn key(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Default::default(),
    }
}

#[test]
fn real_handle_number_field_owns_text_enter_and_escape_without_submitting_a_command() {
    let context = context();
    let mut number = NumberBox::default();
    number.set(Some(prompt(Handle::Translate(Axis::X))));
    let mut model = CommandLine::default();
    for _ in 0..3 {
        frame(&context, &mut number, &mut model, vec![]);
    }
    assert_eq!(
        context.memory(|m| m.focused()),
        Some(egui::Id::new("number-input"))
    );
    let (_, _, controls) = frame(
        &context,
        &mut number,
        &mut model,
        vec![egui::Event::Text("12.3456789012345".into())],
    );
    let field = controls
        .iter()
        .find(|c| c.key == "number/input")
        .unwrap();
    assert!(
        number
            .number_rect
            .unwrap()
            .contains(egui::pos2(field.rect[0], field.rect[1]))
    );
    assert_eq!(number.number, "12.3456789012345");
    assert!(model.command.is_empty());
    let (answer, command, _) = frame(
        &context,
        &mut number,
        &mut model,
        vec![key(egui::Key::Enter)],
    );
    assert_eq!(answer.as_deref(), Some("12.3456789012345"));
    assert!(command.is_none());
    number.number_error = "A finite number is required".into();
    let (_, _, controls) = frame(&context, &mut number, &mut model, vec![]);
    assert!(
        controls
            .iter()
            .any(|c| c.key == "number/input")
    );
    assert_eq!(number.number, "12.3456789012345");
    let (answer, command, _) = frame(
        &context,
        &mut number,
        &mut model,
        vec![key(egui::Key::Escape)],
    );
    assert_eq!(answer.as_deref(), Some("Escape"));
    assert!(command.is_none());
}

#[test]
fn switching_handle_clears_old_text_and_error_and_closed_box_releases_its_rectangle() {
    let context = context();
    let mut number = NumberBox::default();
    let mut model = CommandLine::default();
    number.set(Some(prompt(Handle::Translate(Axis::X))));
    frame(&context, &mut number, &mut model, vec![]);
    frame(
        &context,
        &mut number,
        &mut model,
        vec![egui::Event::Text("bad".into())],
    );
    number.number_error = "Invalid answer".into();
    number.set(Some(prompt(Handle::ScaleUniform)));
    frame(&context, &mut number, &mut model, vec![]);
    assert!(number.number.is_empty() && number.number_error.is_empty());
    let (answer, _, _) = frame(
        &context,
        &mut number,
        &mut model,
        vec![key(egui::Key::Enter)],
    );
    assert_eq!(answer.as_deref(), Some(""));
    number.set(None);
    frame(&context, &mut number, &mut model, vec![]);
    assert!(number.number_rect.is_none());
}
```

### 3. `src/lib.rs`

Register the edit component and supplied checks.

**Locate this existing block:**

```rust
pub mod gizmo_edit;

pub mod number_box;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod gizmo_edit;

pub mod number_box;

#[cfg(test)]
mod gizmo_edit_tests;
#[cfg(test)]
mod number_box_tests;
```

### 4. `src/gizmo_edit_tests.rs`

Supply a regression for Escape during a held pointer drag, its later release and the preserved redo branch.

**Locate this existing block:**

```rust
        "Hover created no history entry"
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "Hover created no history entry"
    );
}

#[test]
fn escape_during_handle_drag_cancels_release_and_preserves_redo() {
    let mut e = editor();
    let before = models(&e);
    click(&mut e, Handle::Translate(Axis::X));
    e.apply(Action::GizmoNumber("0.25".into()))
        .unwrap();
    let committed = models(&e);
    e.apply(Action::Undo).unwrap();
    assert_eq!(models(&e), before);
    let p = press(&mut e, Handle::Translate(Axis::X));
    let moved = [p[0] + 0.1, p[1]];
    e.apply(Action::GizmoMove {
        pointer: 7,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    assert!(
        e.gizmo_edit
            .as_ref()
            .unwrap()
            .gizmo
            .typing
            .is_none()
    );
    assert_ne!(
        e.gizmo_edit
            .as_ref()
            .unwrap()
            .preview
            .matrix(),
        &Xform::identity().m
    );
    let cancel = e.tool_line(" Escape ").unwrap().unwrap();
    assert!(matches!(cancel, Action::GizmoCancel));
    e.apply(cancel).unwrap();
    assert!(e.gizmo_edit.is_none());
    assert_eq!(models(&e), before);
    e.apply(Action::GizmoRelease {
        pointer: 7,
        screen: moved,
        size: SIZE,
        shift: false,
    })
    .unwrap();
    assert_eq!(models(&e), before);
    e.apply(Action::Redo).unwrap();
    assert_eq!(models(&e), committed);
}
```

## Build & Run

![Recorded visual output for Lesson 7.6 (51zf-handles)](../../screenshots/journey/51zf-handles.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 7.6
npm --prefix ../session_tests run course -- save 7.6
```

Continue with [8.1 · Nested original sources and exact parent edits](52-tree.md).
