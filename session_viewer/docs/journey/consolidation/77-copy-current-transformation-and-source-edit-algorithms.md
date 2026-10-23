# 15.1 · Original transforms, part edits and solid recipes

**Goal:** Turn original curves into exact kernel surfaces around a chosen axis. · **Steps:** 49 · **Source:** [app/deform.rs](../../../src/app/deform.rs) · [state/edit.rs](../../../src/state/edit.rs) · [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs) · [app/command/tool/surfacing.rs](../../../src/app/command/tool/surfacing.rs) · [app/walk/mesh.rs](../../../src/app/walk/mesh.rs) · [app/walk/brep.rs](../../../src/app/walk/brep.rs) (`src/app/transforms.rs,src/app/surfacing.rs,src/app/nurbs_eval.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Profile Curve C(u) + Extrude Vector d / Revolve Axis A</text>
  <path d="M 64 138 Q 114 82 164 138" fill="none" stroke="#dc2626" stroke-width="3"/>
  <line x1="114" y1="110" x2="114" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="234" y1="46" x2="234" y2="156" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <path d="M 254 136 Q 282 100 254 64" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">S_ext(u,v) = C(u) + v*d  |  S_rev(u,v) = Rot_A(v) * C(u)</text>
  <polygon points="426,138 476,92 546,92 496,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="426,96 476,50 546,50 496,96" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="426" y1="138" x2="426" y2="96" stroke="#16a34a" stroke-width="2"/>
  <ellipse cx="638" cy="72" rx="42" ry="14" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="638" cy="132" rx="42" ry="14" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="596" y1="72" x2="596" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="680" y1="72" x2="680" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Extruded Prism &amp; Revolved Surface of Revolution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="832,138 866,76 924,76 890,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="992" cy="74" rx="36" ry="12" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <path d="M 956 74 L 956 134 A 36 12 0 0 0 1028 134 L 1028 74" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

Transform or copy original geometry with precise placements, coherent topology and one reversible source edit.

## Ownership and data flow

Capture selected original identities and question answers before previewing. Whole-object transforms compose placement; exclusive part edits resolve original sparse mesh keys, controls, faces and shared BRep boundaries. Preserve rational weights and hole winding, rebuild derived display privately, and adopt only a valid complete candidate. Copy creates fresh row identity beside the original parent while immutable source owners remain shared when eligible. Subsequent curve-gathering, extrusion/cap and revolution helpers prepare exact kernel recipes; preview samples never replace mathematical sources.

## The maintained implementation

app/deform.rs resolves Target and source control/vertex edits; state/edit.rs and app/command/tool.rs own transient transforms and final adoption. app/command/tool/surfacing.rs and verbs/extrude.rs and nurbs_surface_revolve.rs build original surfaces/solids. app/walk/mesh.rs, mesh_topology.rs and brep.rs derive final display, original-face tables and boundary ink after source changes.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/deform.rs](../../../src/app/deform.rs), [state/edit.rs](../../../src/state/edit.rs), [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs), [app/command/tool/surfacing.rs](../../../src/app/command/tool/surfacing.rs), [app/walk/mesh.rs](../../../src/app/walk/mesh.rs), [app/walk/brep.rs](../../../src/app/walk/brep.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-77-copy-current-transformation-and-source-edit-algorithms"></a>1. Build directional scale transforms

**What this code does:** Build directional scale transforms.

### 1. `src/transform_scale.rs`

Build directional scale transforms.

Create the file and type:

```rust
use session_rust::{Plane, Point, Vector, Xform};
/// How far `p` reaches from `origin`: in space, in the plane, or along origin→`from` for 1D.
pub fn reach(
    mode: u8,
    origin: &Point,
    from: &Point,
    p: &Point,
    plane: &Plane,
) -> Option<f64> {
    let d = p - origin;

    match mode {
        1 => {
            let along = from - origin;
            let length = along.magnitude();
            (length > 1e-12).then(|| d.dot(&along) / length)
        }
        2 => Some(
            d.dot(&plane.x_axis())
                .hypot(d.dot(&plane.y_axis())),
        ),
        _ => Some(d.magnitude()),
    }
}

/// The factor that takes the first reference point's reach to `to`'s.
pub fn factor(
    mode: u8,
    origin: &Point,
    from: &Point,
    to: &Point,
    plane: &Plane,
) -> Option<f64> {
    let base = reach(mode, origin, from, from, plane)?;
    let k = reach(mode, origin, from, to, plane)? / base;
    (base > 1e-12 && k.is_finite() && k > 1e-12).then_some(k)
}

/// A scale by `k` about `origin`: uniform, in the plane, or along origin→`from`.
pub fn scaling(mode: u8, origin: &Point, from: &Point, k: f64, plane: &Plane) -> Xform {
    match mode {
        1 => stretch(origin, &[(from - origin).normalized()], k),
        2 => stretch(origin, &[plane.x_axis(), plane.y_axis()], k),
        _ => Xform::scale_uniform(origin, k),
    }
}

/// Scale by `k` along the orthonormal `axes` through `origin`, keeping every other direction.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3)
            .map(|col| xform.m[col * 4 + row] * origin[col])
            .sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
mod freeform_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod freeform_tests;

pub mod transform_scale;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/transform_scale.rs`

Supply independent original-source and fixed-origin regressions.

**Locate this existing block:**

```rust
    xform
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    xform
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The construction plane of the Top view.
    fn top() -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    /// Where the transform puts a point.
    fn moved(xform: &Xform, p: [f64; 3]) -> [f64; 3] {
        let q = xform.transform_point(&Point::new(p[0], p[1], p[2]));
        [q[0], q[1], q[2]]
    }

    /// Close to within 1e-9.
    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        (0..3).all(|i| (a[i] - b[i]).abs() < 1e-9)
    }

    /// 3D grows every direction, 2D keeps the plane normal, 1D only its direction.
    #[test]
    fn each_mode_scales_its_own_directions() {
        let origin = Point::new(1.0, 1.0, 1.0);
        assert!(near(
            moved(
                &scaling(3, &origin, &origin, 2.0, &top()),
                [2.0, 1.0, 1.0]
            ),
            [3.0, 1.0, 1.0]
        ));
        let flat = scaling(2, &origin, &origin, 3.0, &top());
        assert!(near(
            moved(&flat, [2.0, 2.0, 5.0]),
            [4.0, 4.0, 5.0]
        ));
        let zero = Point::new(0.0, 0.0, 0.0);
        let along = scaling(
            1,
            &zero,
            &Point::new(1.0, 1.0, 0.0),
            4.0,
            &top(),
        );
        assert!(near(
            moved(&along, [1.0, 1.0, 0.0]),
            [4.0, 4.0, 0.0]
        ));
        assert!(near(
            moved(&along, [1.0, -1.0, 0.0]),
            [1.0, -1.0, 0.0]
        ));
        assert!(near(
            moved(&along, [0.0, 0.0, 7.0]),
            [0.0, 0.0, 7.0]
        ));
    }

    /// The factor is the second reach over the first, in each mode's measure.
    #[test]
    fn the_reference_points_give_the_factor() {
        let zero = Point::new(0.0, 0.0, 0.0);
        let from = Point::new(10.0, 0.0, 0.0);
        let k = |mode, to: [f64; 3]| {
            factor(
                mode,
                &zero,
                &from,
                &Point::new(to[0], to[1], to[2]),
                &top(),
            )
        };
        assert!((k(3, [5.0, 0.0, 0.0]).unwrap() - 0.5).abs() < 1e-12);
        assert!((k(3, [0.0, 0.0, 30.0]).unwrap() - 3.0).abs() < 1e-12);
        assert!(
            (k(2, [0.0, 20.0, 99.0]).unwrap() - 2.0).abs() < 1e-12,
            "height does not count in 2D"
        );
        assert!(
            (k(1, [30.0, 50.0, 0.0]).unwrap() - 3.0).abs() < 1e-12,
            "only along the direction in 1D"
        );
        assert!(
            k(1, [-5.0, 0.0, 0.0]).is_none(),
            "a mirror is no scale"
        );
        assert!(k(3, [0.0, 0.0, 0.0]).is_none());
        assert!(
            factor(3, &zero, &zero, &from, &top()).is_none(),
            "the first reference is on the origin"
        );
        let preview = scaling(
            1,
            &zero,
            &from,
            factor(
                1,
                &zero,
                &from,
                &Point::new(30.0, 0.0, 0.0),
                &top(),
            )
            .unwrap(),
            &top(),
        );
        assert!(near(
            moved(&preview, [1.0, 1.0, 1.0]),
            [3.0, 1.0, 1.0]
        ));
    }
}
```

## <a id="checkpoint-77sa-compute-signed-construction-plane-rotation"></a>2. Compute signed construction-plane rotation

**What this code does:** Compute signed construction-plane rotation.

### 1. `src/transform_rotate.rs`

Compute signed construction-plane rotation.

Create the file and type:

```rust
use session_rust::{Plane, Point, Xform};
/// The signed angle, radians, from `from` to `to` about `center` in the plane; None when either lies on the center.
pub fn angle(center: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let (x, y) = (plane.x_axis(), plane.y_axis());
    let (a, b) = (from - center, to - center);
    let (ax, ay, bx, by) = (a.dot(&x), a.dot(&y), b.dot(&x), b.dot(&y));

    if ax.hypot(ay) <= 1e-12 || bx.hypot(by) <= 1e-12 {
        return None;
    }

    Some((ax * by - ay * bx).atan2(ax * bx + ay * by))
}

/// A turn of `radians` about the plane normal through `center`.
pub fn rotation(center: &Point, radians: f64, plane: &Plane) -> Xform {
    let to = Xform::translation(center[0], center[1], center[2]);
    let back = Xform::translation(-center[0], -center[1], -center[2]);
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod transform_scale;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod transform_scale;

pub mod transform_rotate;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/transform_rotate.rs`

Supply independent original-source and fixed-origin regressions.

**Locate this existing block:**

```rust
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;

    /// A construction plane from its two axes.
    fn plane(x: [f64; 3], y: [f64; 3]) -> Plane {
        Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(x[0], x[1], x[2]),
            Vector::new(y[0], y[1], y[2]),
        )
    }

    /// Where the transform puts a point.
    fn moved(xform: &Xform, p: [f64; 3]) -> [f64; 3] {
        let q = xform.transform_point(&Point::new(p[0], p[1], p[2]));
        [q[0], q[1], q[2]]
    }

    /// Close to within 1e-9.
    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        (0..3).all(|i| (a[i] - b[i]).abs() < 1e-9)
    }

    /// Counter-clockwise is positive in Top and in Front.
    #[test]
    fn a_turn_is_counter_clockwise_in_the_construction_plane() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let top = plane([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 10.0, 0.0),
            &top,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        let front = plane([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let turn = angle(
            &origin,
            &Point::new(10.0, 0.0, 0.0),
            &Point::new(0.0, 0.0, 10.0),
            &front,
        )
        .unwrap();
        assert!((turn.to_degrees() - 90.0).abs() < 1e-9);
        assert!(near(
            moved(
                &rotation(&origin, turn, &front),
                [1.0, 0.0, 0.0]
            ),
            [0.0, 0.0, 1.0]
        ));
        let quarter = rotation(
            &Point::new(5.0, 5.0, 0.0),
            90f64.to_radians(),
            &top,
        );
        assert!(near(
            moved(&quarter, [6.0, 5.0, 0.0]),
            [5.0, 6.0, 0.0]
        ));
        assert!(
            angle(
                &origin,
                &origin,
                &Point::new(1.0, 0.0, 0.0),
                &top
            )
            .is_none()
        );
        assert!(
            angle(
                &origin,
                &Point::new(0.0, 0.0, 3.0),
                &Point::new(1.0, 0.0, 0.0),
                &top
            )
            .is_none(),
            "straight above the center"
        );
    }
}
```

## <a id="checkpoint-77sb-address-original-controls-and-exclusive-part-selections"></a>3. Address controls and exclusive part selections

**What this code does:** Address original controls and exclusive part selections.

### 1. `src/source_controls.rs`

Address original controls and exclusive part selections.

Create the file and type:

```rust
use session_rust::element::ElementGeometry;
use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point};

/// One control point of a geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum ControlId {
    Vertex(usize), // mesh, polyline or BRep vertex
    Curve {
        curve: usize,
        point: usize,
    }, // curve control point
    Surface {
        surface: usize,
        u: usize,
        v: usize,
    }, // surface control point
    Point(u32),    // cloud point
}

/// What is selected inside one object.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum SelectionMode {
    #[default]
    Object, // whole objects only
    Edge {
        parent: u32, // object row
        edge: u32,   // edge index
    },
    Face {
        parent: u32, // object row
        face: usize, // face key
    },
    Controls {
        parent: u32,                 // object row
        selected: Option<ControlId>, // the picked control
        cloud: bool,                 // parent is a point cloud
    },
}

impl SelectionMode {
    /// The object row of a sub-selection.
    pub fn parent(&self) -> Option<u32> {
        match self {
            Self::Object => None,
            Self::Edge { parent, .. }
            | Self::Face { parent, .. }
            | Self::Controls { parent, .. } => Some(*parent),
        }
    }

    /// Select one edge.
    pub fn select_edge(&mut self, parent: u32, edge: u32) {
        *self = Self::Edge { parent, edge };
    }

    /// Show controls of `parent`; false when nothing changed.
    pub fn enable_controls(&mut self, parent: Option<u32>, cloud: bool) -> bool {
        let Some(parent) = parent else { return false };

        if matches!(self, Self::Controls { parent: active, .. } if *active == parent) {
            return false;
        }

        *self = Self::Controls {
            parent,
            selected: None,
            cloud,
        };
        true
    }

    /// Back to object selection; returns the parent.
    pub fn escape(&mut self) -> Option<u32> {
        let parent = self.parent();
        *self = Self::Object;
        parent
    }
}

/// What a click picks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionTool {
    #[default]
    Object, // whole objects
    Edge, // edges
    Face, // faces
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod transform_rotate;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod transform_rotate;

pub mod source_controls;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77sc-collect-finite-original-controls-and-mesh-vertices"></a>4. Collect finite controls and mesh vertices

**What this code does:** Collect finite original controls and mesh vertices.

### 1. `src/source_controls.rs`

Collect finite original controls and mesh vertices.

**Locate this existing block:**

```rust
    Face, // faces
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Face, // faces
}

/// One control point and where it is.
#[derive(Clone, Debug)]
pub struct Control {
    pub id: ControlId,      // which control
    pub position: [f64; 3], // world position
}

/// Every control of one object and the lines between them.
#[derive(Default)]
pub struct Controls {
    pub points: Vec<Control>,   // the controls
    pub links: Vec<[usize; 2]>, // control net lines, by index
    pub cloud: bool,            // a point cloud, controls stay on the GPU
}

impl Controls {
    /// The controls of a point cloud: its points stay on the GPU.
    pub fn cloud() -> Self {
        Self {
            cloud: true,
            ..Self::default()
        }
    }

    /// Add one control; None when its position is not finite.
    fn push(&mut self, id: ControlId, point: &Point) -> Option<usize> {
        let position = [point[0], point[1], point[2]];

        if !position.into_iter().all(f64::is_finite) {
            return None;
        }

        let index = self.points.len();
        self.points.push(Control { id, position });
        Some(index)
    }

    /// Every mesh vertex.
    fn mesh(&mut self, mesh: &session_rust::Mesh) {
        for key in mesh.vertices() {
            if let Some(vertex) = mesh.vertex.get(&key) {
                self.push(
                    ControlId::Vertex(key),
                    &Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }
}
```

## <a id="checkpoint-77sd-connect-curve-polygons-and-surface-control-nets"></a>5. Connect curve polygons and surface control nets

**What this code does:** Connect curve polygons and surface control nets.

### 1. `src/source_controls.rs`

Connect curve polygons and surface control nets.

**Locate this existing block:**

```rust
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }
}

impl Controls {
    /// A curve's control polygon.
    fn curve(&mut self, curve: &NurbsCurve, index: usize) {
        let mut previous = None;

        for point in 0..curve.cv_count() {
            let current = match curve.get_cv(point) {
                Some(position) => self.push(
                    ControlId::Curve {
                        curve: index,
                        point,
                    },
                    &position,
                ),
                None => None,
            };

            if let (Some(start), Some(end)) = (previous, current) {
                self.links.push([start, end]);
            }

            previous = current;
        }
    }

    /// A surface's control net.
    fn surface(&mut self, surface: &NurbsSurface, index: usize) {
        let [width, height] = surface.m_cv_count;
        let mut previous = vec![None; height];

        for u in 0..width {
            let mut last = None;

            for (v, above) in previous.iter_mut().enumerate() {
                let current = match surface.get_cv(u, v) {
                    Some(position) => self.push(
                        ControlId::Surface {
                            surface: index,
                            u,
                            v,
                        },
                        &position,
                    ),
                    None => None,
                };

                if let (Some(start), Some(end)) = (*above, current) {
                    self.links.push([start, end]);
                }

                if let (Some(start), Some(end)) = (last, current) {
                    self.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}
```

## <a id="checkpoint-77se-reuse-control-collectors-across-original-geometry-families"></a>6. Reuse control collectors across geometry families

**What this code does:** Reuse control collectors across original geometry families.

### 1. `src/source_controls.rs`

Reuse control collectors across original geometry families. Supply independent original-source and fixed-origin regressions.

**Locate this existing block:**

```rust
                last = current;
            }
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                last = current;
            }
        }
    }
}

impl Controls {
    /// BRep vertices, then curve and surface nets.
    fn brep(&mut self, brep: &session_rust::BRep) {
        for (index, vertex) in brep.m_vertices.iter().enumerate() {
            self.push(ControlId::Vertex(index), &vertex.point);
        }

        for (index, curve) in brep.m_curves_3d.iter().enumerate() {
            self.curve(curve, index);
        }

        for (index, surface) in brep.m_surfaces.iter().enumerate() {
            self.surface(surface, index);
        }
    }

    /// The controls of one geometry.
    pub fn from_geometry(geometry: &Geometry) -> Self {
        let mut controls = Self::default();
        controls.append_geometry(geometry);
        controls
    }

    /// Add the controls of one geometry.
    fn append_geometry(&mut self, geometry: &Geometry) {
        match geometry {
            Geometry::Mesh(mesh) => self.mesh(mesh),
            Geometry::Line(line) => {
                self.push(ControlId::Vertex(0), &line.start());
                self.push(ControlId::Vertex(1), &line.end());

                if self.points.len() == 2 {
                    self.links.push([0, 1]);
                }
            }
            Geometry::Polyline(polyline) => {
                let mut previous = None;

                for (index, coords) in polyline.coords.chunks_exact(3).enumerate() {
                    let current = self.push(
                        ControlId::Vertex(index),
                        &Point::new(coords[0], coords[1], coords[2]),
                    );

                    if let (Some(start), Some(end)) = (previous, current) {
                        self.links.push([start, end]);
                    }

                    previous = current;
                }
            }
            Geometry::NurbsCurve(curve) => self.curve(curve, 0),
            Geometry::NurbsSurface(surface) => self.surface(surface, 0),
            Geometry::BRep(brep) => self.brep(brep),
            Geometry::PointCloud(_) => self.cloud = true,
            Geometry::Point(point) => {
                self.push(ControlId::Vertex(0), point);
            }
            Geometry::Element(element) => match element.geometry() {
                ElementGeometry::Mesh(mesh) => self.mesh(mesh),
                ElementGeometry::BRep(brep) => self.brep(brep),
                ElementGeometry::None => {}
            },
            Geometry::Plane(_) | Geometry::OBB(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A new parent replaces the old selection; escape keeps the parent.
    #[test]
    fn switching_parent_replaces_specialized_selection_and_escape_retains_parent() {
        let mut mode = SelectionMode::default();
        mode.select_edge(4, 17);
        assert!(mode.enable_controls(Some(4), false));
        assert!(!mode.enable_controls(Some(4), false));
        mode.select_edge(9, 3);
        assert_eq!(
            mode,
            SelectionMode::Edge {
                parent: 9,
                edge: 3
            }
        );
        assert_eq!(mode.escape(), Some(9));
        assert_eq!(mode, SelectionMode::Object);
        assert!(!mode.enable_controls(None, false));
    }

    /// A line's controls are its two ends.
    #[test]
    fn line_controls_use_original_endpoints() {
        let geometry = Geometry::Line(
            session_rust::Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).into(),
        );
        let controls = Controls::from_geometry(&geometry);
        assert_eq!(controls.points.len(), 2);
        assert_eq!(controls.points[1].position, [4.0, 5.0, 6.0]);
        assert_eq!(controls.points[1].id, ControlId::Vertex(1));
        assert_eq!(controls.links, [[0, 1]]);
    }
}
```

## <a id="checkpoint-77sf-represent-original-geometry-edit-targets"></a>7. Represent geometry edit targets

**What this code does:** Represent original geometry edit targets.

### 1. `src/source_deform.rs`

Represent original geometry edit targets.

Create the file and type:

```rust
use crate::source_controls::{ControlId, Controls, SelectionMode};
use session_rust::{Geometry, Mesh, NurbsSurface, Point, Xform};
use std::collections::HashSet;
use std::rc::Rc;

/// The part of a geometry an edit applies to.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Control(ControlId), // one control point or vertex
    Edge(u32),          // one edge by index
    Face(usize),        // one face by key
}

impl Target {
    /// The target the current selection names, if any.
    pub fn selected(mode: &SelectionMode) -> Option<Self> {
        match *mode {
            SelectionMode::Controls {
                selected: Some(id),
                ..
            } => Some(Self::Control(id)),
            SelectionMode::Edge { edge, .. } => Some(Self::Edge(edge)),
            SelectionMode::Face { face, .. } => Some(Self::Face(face)),
            _ => None,
        }
    }
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod source_controls;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_controls;

pub mod source_deform;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77sg-resolve-original-mesh-vertices-for-part-edits"></a>8. Resolve mesh vertices for part edits

**What this code does:** Resolve original mesh vertices for part edits.

### 1. `src/source_deform.rs`

Resolve original mesh vertices for part edits.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

/// The mesh vertex keys a target covers.
pub(crate) fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
    match target {
        Target::Control(ControlId::Vertex(key)) if mesh.vertex.contains_key(&key) => {
            Ok(vec![key])
        }
        Target::Face(key) => {
            let mut keys = mesh
                .face
                .get(&key)
                .ok_or("Unknown mesh face")?
                .clone();

            if let Some(holes) = mesh.face_holes.get(&key) {
                keys.extend(holes.iter().flatten()); // hole rims move with the face
            }

            keys.sort_unstable();
            keys.dedup();
            Ok(keys)
        }
        Target::Edge(index) => {
            // edges are numbered in order of first appearance
            let mut seen = HashSet::new();
            let mut at = 0;

            for face in mesh.faces() {
                let keys = &mesh.face[&face];

                for i in 0..keys.len() {
                    let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}
```

## <a id="checkpoint-77sh-resolve-surface-grids-and-individual-source-controls"></a>9. Resolve surface grids and individual source controls

**What this code does:** Resolve surface grids and individual source controls.

### 1. `src/source_deform.rs`

Resolve surface grids and individual source controls.

**Locate this existing block:**

```rust
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}

/// The surface control (u, v) pairs a target covers.
fn surface_keys(
    surface: &NurbsSurface,
    target: Target,
) -> Result<Vec<(usize, usize)>, String> {
    let [nu, nv] = surface.m_cv_count; // control grid size
    let all = || {
        (0..nu)
            .flat_map(|u| (0..nv).map(move |v| (u, v)))
            .collect()
    };

    match target {
        Target::Control(ControlId::Surface { surface: 0, u, v })
            if u < nu && v < nv =>
        {
            Ok(vec![(u, v)])
        }
        Target::Face(0) => Ok(all()),
        // edge 0..3: the four sides of the control grid
        Target::Edge(edge) if edge < 4 => Ok((0..nu)
            .flat_map(|u| (0..nv).map(move |v| (u, v)))
            .filter(|&(u, v)| match edge {
                0 => u == 0,
                1 => u + 1 == nu,
                2 => v == 0,
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}
```

## <a id="checkpoint-77si-collect-affected-points-across-cad-geometry-families"></a>10. Collect affected points across CAD geometry families

**What this code does:** Collect affected points across CAD geometry families.

### 1. `src/source_deform.rs`

Collect affected points across CAD geometry families.

**Locate this existing block:**

```rust
        .ok_or("Unknown source control".into())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .ok_or("Unknown source control".into())
}

/// The world points a target covers.
pub fn points(geometry: &Geometry, target: Target) -> Result<Vec<Point>, String> {
    match geometry {
        Geometry::Mesh(mesh) => mesh_keys(mesh, target)?
            .into_iter()
            .map(|k| {
                mesh.vertex_point(k)
                    .ok_or("Missing mesh vertex".into())
            })
            .collect(),
        Geometry::NurbsSurface(surface) => surface_keys(surface, target)?
            .into_iter()
            .map(|(u, v)| {
                surface
                    .get_cv(u, v)
                    .ok_or("Missing surface control".into())
            })
            .collect(),
        Geometry::BRep(brep) => match target {
            Target::Face(face) => {
                let face = brep
                    .m_faces
                    .get(face)
                    .ok_or("Unknown BRep face")?;
                let surface = brep
                    .m_surfaces
                    .get(face.surface_index as usize)
                    .ok_or("Missing face surface")?;
                Ok((0..surface.m_cv_count[0])
                    .flat_map(|u| {
                        (0..surface.m_cv_count[1])
                            .filter_map(move |v| surface.get_cv(u, v))
                    })
                    .collect())
            }
            Target::Edge(edge) => {
                let edge = brep
                    .m_edges
                    .get(edge as usize)
                    .ok_or("Unknown BRep edge")?;
                let curve = brep
                    .m_curves_3d
                    .get(edge.curve_3d_index as usize)
                    .ok_or("Degenerate edge has no movable curve")?;
                Ok((0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect())
            }
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
        },
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => points(
                &Geometry::Mesh(Rc::new(mesh.clone())),
                target,
            ),
            session_rust::element::ElementGeometry::BRep(brep) => points(
                &Geometry::BRep(Rc::new(brep.clone())),
                target,
            ),
            _ => Err("Element has no source geometry".into()),
        },
        _ => match target {
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
            _ => Err("This source has no editable faces or edges".into()),
        },
    }
}
```

## <a id="checkpoint-77sj-preserve-rational-weights-and-validate-brep-boundaries"></a>11. Preserve rational weights and validate BRep boundaries

**What this code does:** Preserve rational weights and validate BRep boundaries.

### 1. `src/source_deform.rs`

Preserve rational weights and validate BRep boundaries.

**Locate this existing block:**

```rust
        },
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        },
    }
}

fn set_curve_control(
    curve: &mut session_rust::NurbsCurve,
    index: usize,
    point: &Point,
) -> bool {
    if !curve.m_is_rat {
        return curve.set_cv_point(index, point);
    }
    let Some((_, _, _, weight)) = curve.get_cv_4d(index) else {
        return false;
    };
    curve.set_cv_4d(
        index,
        point[0] * weight,
        point[1] * weight,
        point[2] * weight,
        weight,
    )
}

/// Refuse an edit whose edges no longer lie on their surfaces.
fn validate_boundaries(
    brep: &session_rust::BRep,
    changed_curves: &HashSet<usize>,
    changed_surfaces: &HashSet<usize>,
) -> Result<(), String> {
    if !brep.is_valid() {
        return Err("Edit would invalidate BRep topology".into());
    }

    for edge in &brep.m_edges {
        if edge.degenerated
            || (!changed_curves.contains(&(edge.curve_3d_index as usize))
                && !edge
                    .pcurves
                    .iter()
                    .any(|pc| changed_surfaces.contains(&(pc.surface_index as usize))))
        {
            continue;
        }

        let curve = &brep.m_curves_3d[edge.curve_3d_index as usize];
        let (a, b) = curve.domain();

        // sample the 3D edge against each surface's 2D curve
        for pc in &edge.pcurves {
            let surface = &brep.m_surfaces[pc.surface_index as usize];

            for ci in [pc.curve_2d_index, pc.curve_2d_index_2] {
                if ci < 0 {
                    continue;
                }

                let uv = &brep.m_curves_2d[ci as usize];
                let (u0, u1) = uv.domain();

                for sample in 0..=16 {
                    let t = sample as f64 / 16.0;
                    let p = curve.point_at(a + (b - a) * t);
                    let q = uv.point_at(u0 + (u1 - u0) * t);
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}
```

## <a id="checkpoint-77sk-transform-original-mesh-and-surface-controls"></a>12. Transform mesh and surface controls

**What this code does:** Transform original mesh and surface controls.

### 1. `src/source_deform.rs`

Transform original mesh and surface controls.

**Locate this existing block:**

```rust
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(())
}

fn transform_mesh(
    source: &Rc<session_rust::Mesh>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let mut mesh = (**source).clone();

    for key in mesh_keys(&mesh, target)? {
        let point = mesh
            .vertex_point(key)
            .ok_or("Missing vertex")?
            .transformed(delta);
        mesh.vertex
            .get_mut(&key)
            .ok_or("Missing vertex")?
            .set_position(point);
    }

    mesh.clear_triangle_bvh(); // stale after moving vertices
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

fn transform_nurbssurface(
    source: &Rc<session_rust::NurbsSurface>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let mut surface = (**source).clone();

    for (u, v) in surface_keys(&surface, target)? {
        let p = surface
            .get_cv(u, v)
            .ok_or("Missing surface control")?
            .transformed(delta);

        if !surface.set_cv(u, v, &p) {
            return Err("Cannot set surface control".into());
        }
    }

    surface.m_mesh = None; // drop the cached mesh
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

## <a id="checkpoint-77sl-transform-original-line-polyline-point-and-curve-controls"></a>13. Transform line polyline point and curve controls

**What this code does:** Transform original line polyline point and curve controls.

### 1. `src/source_deform.rs`

Transform original line polyline point and curve controls.

**Locate this existing block:**

```rust
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

fn transform_polyline(
    source: &Rc<session_rust::Polyline>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i)) = target else {
        return Err("Select a polyline vertex".into());
    };
    let p = source
        .get_point(i)
        .ok_or("Unknown polyline vertex")?
        .transformed(delta);
    let mut next = (**source).clone();
    next.set_point(i, &p);
    Ok(Geometry::Polyline(Rc::new(next)))
}

fn transform_nurbscurve(
    source: &Rc<session_rust::NurbsCurve>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let Target::Control(ControlId::Curve { curve: 0, point }) = target else {
        return Err("Select a curve control point".into());
    };
    let p = source
        .get_cv(point)
        .ok_or("Unknown curve control")?
        .transformed(delta);
    let mut next = (**source).clone();

    if !set_curve_control(&mut next, point, &p) {
        return Err("Cannot set curve control".into());
    }

    Ok(Geometry::NurbsCurve(Rc::new(next)))
}

fn transform_line(
    source: &Rc<session_rust::Line>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i @ 0..=1)) = target else {
        return Err("Select a line endpoint".into());
    };
    let mut ends = [source.start(), source.end()];
    ends[i] = ends[i].transformed(delta);
    let mut next = session_rust::Line::from_points(&ends[0], &ends[1]);
    next.set_guid(source.guid().to_string());
    next.name = source.name.clone();
    next.width = source.width;
    next.dash = source.dash.clone();
    next.linecolor = source.linecolor.clone();
    next.arrowhead = source.arrowhead;
    Ok(Geometry::Line(Rc::new(next)))
}

fn transform_point(
    source: &Rc<session_rust::Point>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(0)) = target else {
        return Err("Unknown point".into());
    };
    Ok(Geometry::Point(Rc::new(
        source.transformed(delta),
    )))
}
```

## <a id="checkpoint-77sm-transform-shared-brep-boundary-positions-coherently"></a>14. Transform shared BRep boundary positions coherently

**What this code does:** Transform shared BRep boundary positions coherently.

### 1. `src/source_deform.rs`

Transform shared BRep boundary positions coherently.

**Locate this existing block:**

```rust
    )))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    )))
}

fn transform_brep(
    geometry: &Geometry,
    source: &Rc<session_rust::BRep>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let selected = points(geometry, target)?;
    let mut next = (**source).clone();
    // every vertex, curve or surface point at a selected position moves
    let matches = |p: &Point| {
        selected
            .iter()
            .any(|s| (0..3).all(|i| (s[i] - p[i]).abs() <= 1e-8))
    };

    for vertex in &mut next.m_vertices {
        if matches(&vertex.point) {
            vertex.point = vertex.point.transformed(delta);
        }
    }

    let mut changed_curves = HashSet::new(); // curves touched
    let mut changed_surfaces = HashSet::new(); // surfaces touched

    for (curve_index, curve) in next.m_curves_3d.iter_mut().enumerate() {
        for i in 0..curve.cv_count() {
            if let Some(p) = curve.get_cv(i)
                && matches(&p)
            {
                if !set_curve_control(curve, i, &p.transformed(delta)) {
                    return Err("Cannot set curve control".into());
                }
                changed_curves.insert(curve_index);
            }
        }
    }

    for (surface_index, surface) in next.m_surfaces.iter_mut().enumerate() {
        for u in 0..surface.m_cv_count[0] {
            for v in 0..surface.m_cv_count[1] {
                if let Some(p) = surface.get_cv(u, v)
                    && matches(&p)
                {
                    surface.set_cv(u, v, &p.transformed(delta));
                    changed_surfaces.insert(surface_index);
                }
            }
        }

        if changed_surfaces.contains(&surface_index) {
            surface.m_mesh = None; // drop the cached mesh
        }
    }

    validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
    Ok(Geometry::BRep(Rc::new(next)))
}
```

## <a id="checkpoint-77sn-dispatch-original-part-edits-and-retain-element-ownership"></a>15. Dispatch part edits and retain Element ownership

**What this code does:** Dispatch original part edits and retain Element ownership.

### 1. `src/source_deform.rs`

Dispatch original part edits and retain Element ownership.

**Locate this existing block:**

```rust
    Ok(Geometry::BRep(Rc::new(next)))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(Geometry::BRep(Rc::new(next)))
}

fn transform_element(
    source: &Rc<session_rust::Element>,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    let mut next = (**source).clone();

    match source.geometry() {
        session_rust::element::ElementGeometry::Mesh(mesh) => {
            let Geometry::Mesh(mesh) = transform(
                &Geometry::Mesh(Rc::new(mesh.clone())),
                target,
                delta,
            )?
            else {
                unreachable!()
            };
            next.set_geometry((*mesh).clone());
        }
        session_rust::element::ElementGeometry::BRep(brep) => {
            let Geometry::BRep(brep) = transform(
                &Geometry::BRep(Rc::new(brep.clone())),
                target,
                delta,
            )?
            else {
                unreachable!()
            };
            next.set_brep_geometry((*brep).clone());
        }
        _ => return Err("Element has no source geometry".into()),
    }

    Ok(Geometry::Element(Rc::new(next)))
}

/// A copy of the geometry with the target moved by `delta`.
pub fn transform(
    geometry: &Geometry,
    target: Target,
    delta: &Xform,
) -> Result<Geometry, String> {
    if !delta.m.iter().all(|v| v.is_finite()) {
        return Err("Transform must be finite".into());
    }
    match geometry {
        Geometry::Mesh(source) => transform_mesh(source, target, delta),
        Geometry::NurbsSurface(source) => transform_nurbssurface(source, target, delta),
        Geometry::Polyline(source) => transform_polyline(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}
```

## <a id="checkpoint-77so-map-sparse-original-vertex-keys-to-display-slots"></a>16. Map sparse vertex keys to display slots

**What this code does:** Map sparse original vertex keys to display slots.

### 1. `src/source_topology_colour.rs`

Pack original edge pen colours into the existing RGBA word convention.

Create the file and type:

```rust
/// One colour channel to a byte, rounded.
fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

/// Four colour bytes in one word, red lowest.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

pub const BLACK: u32 = 0xff00_0000;
```

### 2. `src/source_mesh_topology.rs`

Map sparse original vertex keys to display slots.

Create the file and type:

```rust
use crate::source_topology_colour::{BLACK, pack_rgba};
use session_rust::{Mesh, Tolerance};

/// Vertex key to its position in the sorted key list.
pub struct SlotMap {
    dense: Vec<u32>,                               // key indexes directly
    sparse: std::collections::HashMap<usize, u32>, // used for sparse keys
}

impl SlotMap {
    /// Build from the sorted vertex keys.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);

        // dense when keys are not too spread out
        if max_key < 4 * keys.len().max(1) {
            let mut dense = vec![u32::MAX; max_key + 1];

            for (s, &k) in keys.iter().enumerate() {
                dense[k] = s as u32;
            }

            return Self {
                dense,
                sparse: std::collections::HashMap::new(),
            };
        }

        let mut sparse = std::collections::HashMap::with_capacity(keys.len());

        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }

        Self {
            dense: Vec::new(),
            sparse,
        }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}
```

### 3. `src/lib.rs`

Register the shared source colour helper and original edge topology owner.

**Locate this existing block:**

```rust
pub mod source_deform;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_deform;

pub mod source_mesh_topology;
pub mod source_topology_colour;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77sp-compute-polygon-normals-from-original-boundaries"></a>17. Compute polygon normals from boundaries

**What this code does:** Compute polygon normals from original boundaries.

### 1. `src/source_mesh_topology.rs`

Compute polygon normals from original boundaries.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

/// Edges, faces and normals of one mesh.
pub struct MeshTopo {
    pub edges: Vec<(usize, usize, u32)>, // (low key, high key, pen colour)
    pub edge_faces: Vec<[u32; 2]>,       // two faces per edge, u32::MAX = none
    pub opposed: Vec<bool>,              // per edge: faces wind opposite ways
    pub normals: Vec<Option<[f64; 3]>>,  // per face, None when degenerate
    pub closed: bool,                    // every edge has two faces
}

/// Unit normal of one face, None when degenerate.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }

    // Newell normal: sums over every edge
    let mut n = [0.0f64; 3];

    for i in 0..vs.len() {
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}
```

## <a id="checkpoint-77sq-build-one-original-edge-table-and-record-incident-faces"></a>18. Build one edge table and record incident faces

**What this code does:** Build one original edge table and record incident faces.

### 1. `src/source_mesh_topology.rs`

Build one original edge table and record incident faces.

**Locate this existing block:**

```rust
    face.0
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    face.0
}

struct EdgeTable {
    edges: Vec<(usize, usize, u32)>,
    edge_faces: Vec<[u32; 2]>,
    dir0: Vec<u8>,
    opposed: Vec<bool>,
    head: Vec<u32>,
    next: Vec<u32>,
}

impl EdgeTable {
    fn new(slots: usize) -> Self {
        Self {
            edges: Vec::new(),
            edge_faces: Vec::new(),
            dir0: Vec::new(),
            opposed: Vec::new(),
            head: vec![u32::MAX; slots],
            next: Vec::new(),
        }
    }

    fn add(
        &mut self,
        low_slot: usize,
        lo: usize,
        hi: usize,
        dir: u8,
        face: usize,
        cols: &[session_rust::Color],
    ) {
        let ls = low_slot;
        let mut ei = self.head[ls];

        // look for the edge among those at `lo`
        while ei != u32::MAX && self.edges[ei as usize].1 != hi {
            ei = self.next[ei as usize];
        }

        // new edge
        if ei == u32::MAX {
            ei = self.edges.len() as u32;
            let pen = match cols.get(self.edges.len()) {
                Some(color) => pack_rgba(color.to_f32()),
                None => BLACK,
            };
            self.edges.push((lo, hi, pen));
            self.edge_faces.push([u32::MAX; 2]);
            self.dir0.push(dir);
            self.opposed.push(true);
            self.next.push(self.head[ls]);
            self.head[ls] = ei;
        }

        // record this face on the edge, first free slot
        let ef = &mut self.edge_faces[ei as usize];

        if ef[0] == u32::MAX {
            ef[0] = face as u32;
            self.dir0[ei as usize] = dir;
        } else if ef[1] == u32::MAX && ef[0] != face as u32 {
            ef[1] = face as u32;
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}
```

## <a id="checkpoint-77sr-preserve-hole-winding-and-complete-original-mesh-topology"></a>19. Preserve hole winding and complete mesh topology

**What this code does:** Preserve hole winding and complete original mesh topology.

### 1. `src/source_mesh_topology.rs`

Preserve hole winding and complete original mesh topology. Supply independent original-source and fixed-origin regressions.

**Locate this existing block:**

```rust
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}

/// Collect unique edges, their faces and the face normals.
pub fn mesh_topology(
    m: &Mesh,
    keys: &[usize],
    vpos: &[[f64; 3]],
    slots: &SlotMap,
) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());

    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }

    faces.sort_unstable_by_key(face_key);
    let cols = m.get_linecolors();

    let normals: Vec<_> = faces
        .iter()
        .map(|(_, vs)| face_normal(vs, vpos, slots))
        .collect();
    let mut table = EdgeTable::new(keys.len());

    // Keep the kernel's outer-edge ordering for colors, widths and picking ids.
    for holes in [false, true] {
        for (fs, (key, vs)) in faces.iter().enumerate() {
            let rings: &[Vec<usize>] = if holes {
                m.face_holes
                    .get(key)
                    .map_or(&[], Vec::as_slice)
            } else {
                std::slice::from_ref(*vs)
            };
            for ring in rings {
                // Hole rings may be stored in either direction; a face walks them opposite its border.
                let reverse = holes
                    && normals[fs]
                        .zip(face_normal(ring, vpos, slots))
                        .is_some_and(|(a, b)| {
                            a.iter()
                                .zip(b)
                                .map(|(x, y)| x * y)
                                .sum::<f64>()
                                > 0.0
                        });
                let n = ring.len();

                for i in 0..n {
                    let (u, v) = (ring[i], ring[(i + 1) % n]);
                    let (lo, hi, dir) = if u < v {
                        (u, v, 0)
                    } else {
                        (v, u, 1)
                    }; // edge key
                    let dir = dir ^ u8::from(reverse);
                    table.add(slots.slot(lo), lo, hi, dir, fs, cols);
                }
            }
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in table.edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges: table.edges,
        edge_faces: table.edge_faces,
        opposed: table.opposed,
        normals,
        closed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Point, Polyline};

    fn topology(mesh: &Mesh) -> MeshTopo {
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|key| {
                let p = &mesh.vertex[key];
                [p.x, p.y, p.z]
            })
            .collect();
        mesh_topology(mesh, &keys, &positions, &SlotMap::new(&keys))
    }

    #[test]
    fn hole_rims_carry_both_faces_without_reordering_source_edges() {
        let loops = |z| {
            [10.0, 3.0].map(|r| {
                Polyline::new(vec![
                    Point::new(-r, -r, z),
                    Point::new(r, -r, z),
                    Point::new(r, r, z),
                    Point::new(-r, r, z),
                    Point::new(-r, -r, z),
                ])
            })
        };
        let mesh = Mesh::loft(&loops(0.0), &loops(2.0), true, false);
        assert_eq!(mesh.face_holes.len(), 2);
        let topo = topology(&mesh);
        assert!(topo.closed);
        assert_eq!(topo.edges.len(), 24);
        assert!(
            topo.edge_faces
                .iter()
                .all(|faces| !faces.contains(&u32::MAX))
        );
        assert!(topo.opposed.iter().all(|opposed| *opposed));
        let original = mesh.edges_with_colors();
        for (i, (a, b, _)) in original.iter().enumerate() {
            assert_eq!((topo.edges[i].0, topo.edges[i].1), (*a, *b));
        }
        for (face, holes) in &mesh.face_holes {
            let face_slot = mesh
                .faces()
                .iter()
                .position(|key| key == face)
                .unwrap() as u32;
            for ring in holes {
                for i in 0..ring.len() {
                    let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                    let edge = topo
                        .edges
                        .iter()
                        .position(|(u, v, _)| (*u, *v) == (a.min(b), a.max(b)))
                        .unwrap();
                    assert!(topo.edge_faces[edge].contains(&face_slot));
                }
            }
        }
    }

    #[test]
    fn open_face_draws_its_hole_boundary() {
        let points = vec![
            Point::new(-10.0, -10.0, 0.0),
            Point::new(10.0, -10.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(-10.0, 10.0, 0.0),
            Point::new(-3.0, -3.0, 0.0),
            Point::new(3.0, -3.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(-3.0, 3.0, 0.0),
        ];
        let mut mesh = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        mesh.set_face_holes(0, vec![vec![4, 5, 6, 7]]);
        let topo = topology(&mesh);
        assert_eq!(topo.edges.len(), 8);
        assert!(!topo.closed);
    }
}
```

### 2. `src/source_deform.rs`

Supply independent original-source and fixed-origin regressions.

**Locate this existing block:**

```rust
        _ => Err("This source control cannot be transformed".into()),
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        _ => Err("This source control cannot be transformed".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A quad and a triangle sharing an edge.
    fn mesh() -> Geometry {
        let mut mesh = Mesh::new();

        for (key, p) in [
            (10, [0., 0., 0.]),
            (20, [10., 0., 0.]),
            (30, [10., 10., 0.]),
            (40, [0., 10., 0.]),
            (90, [0., 0., 10.]),
        ] {
            mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
        }

        mesh.add_face(vec![10, 20, 30, 40], Some(7));
        mesh.add_face(vec![10, 90, 20], Some(19));
        Geometry::Mesh(Rc::new(mesh))
    }

    /// Moving a face moves its vertices only.
    #[test]
    fn rational_curve_control_moves_without_changing_weights_or_other_controls() {
        let mut curve = session_rust::NurbsCurve::create(
            true,
            2,
            &[
                Point::new(-1.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        );
        assert!(curve.set_cv_4d(1, 0.0, 2.0, 0.0, 2.0));
        let _ = curve.guid();
        let geometry = Geometry::NurbsCurve(Rc::new(curve));
        let delta = Xform::translation(0.123456789012345, 0.25, -0.5);
        let moved = transform(
            &geometry,
            Target::Control(ControlId::Curve {
                curve: 0,
                point: 1,
            }),
            &delta,
        )
        .unwrap();
        let (Geometry::NurbsCurve(original), Geometry::NurbsCurve(edited)) =
            (&geometry, moved)
        else {
            panic!("Curve");
        };
        assert_eq!(edited.guid(), original.guid());
        assert_eq!(edited.m_nurbsknot, original.m_nurbsknot);
        for index in 0..original.cv_count() {
            assert_eq!(
                edited.get_cv_4d(index).unwrap().3,
                original.get_cv_4d(index).unwrap().3
            );
            let before = original.get_cv(index).unwrap();
            let expected = if index == 1 {
                before.transformed(&delta)
            } else {
                before
            };
            let actual = edited.get_cv(index).unwrap();
            for axis in 0..3 {
                assert!((actual[axis] - expected[axis]).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn mesh_face_moves_shared_source_vertices_not_unrelated_vertices() {
        let source = mesh();
        let Geometry::Mesh(next) = transform(
            &source,
            Target::Face(7),
            &Xform::translation(0., 0., 3.),
        )
        .unwrap() else {
            panic!()
        };

        for key in [10, 20, 30, 40] {
            assert_eq!(next.vertex[&key].z, 3.);
        }

        assert_eq!(next.vertex[&90].z, 10.);
        assert_eq!(next.face[&19], vec![10, 90, 20]);
        let Geometry::Mesh(original) = source else {
            panic!()
        };
        assert_eq!(original.vertex[&10].z, 0.);
    }

    /// Edge numbering matches the display edges.
    #[test]
    fn edge_ids_match_the_display_producer_even_with_sparse_keys() {
        let Geometry::Mesh(mesh) = mesh() else {
            panic!()
        };
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|k| {
                let p = mesh.vertex_point(*k).unwrap();
                [p[0], p[1], p[2]]
            })
            .collect();
        let slots = crate::source_mesh_topology::SlotMap::new(&keys);
        let topo = crate::source_mesh_topology::mesh_topology(
            &mesh, &keys, &positions, &slots,
        );

        for (i, &(a, b, _)) in topo.edges.iter().enumerate() {
            assert_eq!(
                mesh_keys(&mesh, Target::Edge(i as u32)).unwrap(),
                vec![a, b]
            );
        }
    }

    /// Moving one surface edge keeps weights and the other edge.
    #[test]
    fn surface_boundary_preserves_weights_and_the_opposite_boundary() {
        let mut surface = NurbsSurface::create(
            false,
            false,
            1,
            1,
            2,
            2,
            &[
                Point::new(0., 0., 0.),
                Point::new(1., 0., 0.),
                Point::new(0., 1., 0.),
                Point::new(1., 1., 0.),
            ],
        )
        .unwrap();
        assert!(surface.to_rational());

        for u in 0..2 {
            for v in 0..2 {
                surface.set_cv_4d(u, v, u as f64 * 2., v as f64 * 2., 0., 2.);
            }
        }

        let Geometry::NurbsSurface(next) = transform(
            &Geometry::NurbsSurface(Rc::new(surface)),
            Target::Edge(0),
            &Xform::translation(0., 0., 5.),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(next.get_cv(0, 0).unwrap()[2], 5.);
        assert_eq!(next.get_cv(1, 0).unwrap()[2], 0.);
        assert!(next.m_is_rat);
        assert_eq!(next.weight(0, 0), 2.);
        assert_eq!(next.weight(1, 1), 2.);
    }

    /// Moving a box face keeps the solid valid.
    #[test]
    fn moving_box_face_keeps_edges_on_incident_surfaces() {
        let source = Geometry::BRep(Rc::new(session_rust::BRep::create_box(
            10., 20., 30.,
        )));
        let Geometry::BRep(next) = transform(
            &source,
            Target::Face(0),
            &Xform::translation(0., 0., 2.),
        )
        .unwrap() else {
            panic!()
        };
        assert!(next.is_valid());
        assert!(next.is_closed(0));
        validate_boundaries(
            &next,
            &(0..next.m_curves_3d.len()).collect(),
            &(0..next.m_surfaces.len()).collect(),
        )
        .unwrap();
    }
}
```

## <a id="checkpoint-77a-own-transformation-questions-and-the-selected-original-targets"></a>20. Own transformation questions and the selected targets

**What this code does:** Own transformation questions and the selected original targets.

### 1. `src/transform_draft.rs`

Own current transformation questions, modes, accepted points and original targets.

Create the file and type:

```rust
use crate::{cplane::CPlane, primitive_shape::Answer};
use session_rust::{Plane, Point, Xform};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Copy,
    Move,
    Scale,
    Rotate,
}
impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Copy => "Copy",
            Self::Move => "Move",
            Self::Scale => "Scale",
            Self::Rotate => "Rotate",
        }
    }
}
#[derive(Clone)]
pub struct Draft {
    pub kind: Kind,
    pub mode: u8,
    pub points: Vec<[f64; 3]>,
    pub frame: CPlane,
    pub owners: Vec<crate::scene::ObjectId>,
    pub target: Option<crate::pick_sources::Target>,
}
pub struct Outcome {
    pub transforms: Vec<Xform>,
    pub finished: bool,
}
impl Draft {
    pub fn new(kind: Kind, frame: CPlane) -> Self {
        Self {
            kind,
            mode: 3,
            points: Vec::new(),
            frame,
            owners: Vec::new(),
            target: None,
        }
    }
    fn plane(&self) -> Plane {
        let (x, y) = self.frame.axes();
        Plane::new(self.frame.origin(), x, y)
    }
    fn p(point: [f64; 3]) -> Point {
        Point::new(point[0], point[1], point[2])
    }
    pub fn prompt(&self) -> String {
        let question = match (self.kind, self.points.len(), self.mode) {
            (Kind::Copy | Kind::Move, 0, _) => "Base point",
            (Kind::Copy, _, _) => "Target point · Enter finishes",
            (Kind::Move, _, _) => "Target point or distance along the band",
            (Kind::Scale, 0, _) => "Origin point",
            (Kind::Scale, 1, 1) => "First reference point sets the direction",
            (Kind::Scale, 1, _) => "Scale factor or first reference point",
            (Kind::Scale, _, _) => "Second reference point or new length",
            (Kind::Rotate, 0, _) => "Center of rotation",
            (Kind::Rotate, 1, _) => "Angle or first reference point",
            (Kind::Rotate, _, _) => "Second reference point or angle",
        };
        format!(
            "{} · {question} · Esc cancels{}",
            self.kind.name(),
            if self.kind == Kind::Scale {
                format!(" · {}D", self.mode)
            } else {
                String::new()
            }
        )
    }
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod source_deform;

pub mod source_mesh_topology;
pub mod source_topology_colour;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_deform;

pub mod source_topology_colour;

pub mod source_mesh_topology;
pub mod transform_draft;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77b-validate-point-and-number-transformation-answers"></a>21. Validate point and number transformation answers

**What this code does:** Validate point and number transformation answers.

### 1. `src/transform_draft.rs`

Validate current transform points and numbers without adopting an invalid reference.

**Locate this existing block:**

```rust
        )
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
    }
    pub fn accept(&mut self, answer: Answer) -> Result<Option<Xform>, String> {
        match answer {
            Answer::Number(value) => {
                if !value.is_finite() || value.abs() > 1e12 {
                    return Err(
                        "Transform value must be finite and within ±1e12".into(),
                    );
                }
                let Some(&origin) = self.points.first() else {
                    return Err("Pick the origin first".into());
                };
                let origin = Self::p(origin);
                let plane = self.plane();
                match self.kind {
                    Kind::Rotate => Ok(Some(crate::transform_rotate::rotation(
                        &origin,
                        value.to_radians(),
                        &plane,
                    ))),
                    Kind::Scale => {
                        if value <= 0. {
                            return Err("Scale wants a value above zero".into());
                        }
                        let (from, factor) = match self.points.as_slice() {
                            [_] if self.mode == 1 => {
                                return Err(
                                    "Scale 1D needs a reference direction".into()
                                );
                            }
                            [_] => (origin.clone(), value),
                            [_, from] => {
                                let from = Self::p(*from);
                                let base = crate::transform_scale::reach(
                                    self.mode, &origin, &from, &from, &plane,
                                )
                                .filter(|v| *v > 1e-12)
                                .ok_or("Reference point is on the origin")?;
                                (from, value / base)
                            }
                            _ => return Err("Too many reference points".into()),
                        };
                        Ok(Some(crate::transform_scale::scaling(
                            self.mode, &origin, &from, factor, &plane,
                        )))
                    }
                    _ => Err(
                        "Supply a target point or a distance along the cursor band"
                            .into(),
                    ),
                }
            }
            Answer::Point(point) => {
                let point = crate::tool_coords::checked([point[0], point[1], point[2]])
                    .map_err(str::to_owned)?;
                let mut points = self.points.clone();
                points.push(point);
                let plane = self.plane();
                let made = match (self.kind, points.as_slice()) {
                    (Kind::Copy | Kind::Move, [from, to]) => Some(Xform::translation(
                        to[0] - from[0],
                        to[1] - from[1],
                        to[2] - from[2],
                    )),
                    (Kind::Scale, [origin, from]) => {
                        let (o, f) = (Self::p(*origin), Self::p(*from));
                        if crate::transform_scale::reach(self.mode, &o, &f, &f, &plane)
                            .is_none_or(|v| v <= 1e-12)
                        {
                            return Err("Reference point is on the origin".into());
                        }
                        None
                    }
                    (Kind::Scale, [origin, from, to]) => {
                        let (o, f, t) = (
                            Self::p(*origin),
                            Self::p(*from),
                            Self::p(*to),
                        );
                        let factor = crate::transform_scale::factor(
                            self.mode, &o, &f, &t, &plane,
                        )
                        .ok_or("Scale factor must be above zero")?;
                        Some(crate::transform_scale::scaling(
                            self.mode, &o, &f, factor, &plane,
                        ))
                    }
                    (Kind::Rotate, [origin, from]) => {
                        let (o, f) = (Self::p(*origin), Self::p(*from));
                        crate::transform_rotate::angle(&o, &f, &f, &plane)
                            .ok_or("Reference point is on the center")?;
                        None
                    }
                    (Kind::Rotate, [origin, from, to]) => {
                        let (o, f, t) = (
                            Self::p(*origin),
                            Self::p(*from),
                            Self::p(*to),
                        );
                        let angle = crate::transform_rotate::angle(&o, &f, &t, &plane)
                            .ok_or("Second point is on the center")?;
                        Some(crate::transform_rotate::rotation(
                            &o, angle, &plane,
                        ))
                    }
                    (_, [_]) => None,
                    _ => return Err("The transform already has its points".into()),
                };
                self.points = if made.is_some() && self.kind == Kind::Copy {
                    vec![points[0]]
                } else {
                    points
                };
                Ok(made)
            }
        }
    }
}
```

## <a id="checkpoint-77c-resolve-typed-batches-and-temporary-transformation-previews"></a>22. Resolve typed batches and temporary transformation previews

**What this code does:** Resolve typed batches and temporary transformation previews.

### 1. `src/transform_draft.rs`

Resolve current options and atomic batches, temporary preview matrices and tool buttons.

**Locate this existing block:**

```rust
            }
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            }
        }
    }
    pub fn text(
        &mut self,
        text: &str,
        cursor: Option<[f64; 3]>,
        fixed: Option<CPlane>,
    ) -> Result<Outcome, String> {
        let mut next = self.clone();
        let mut result = Outcome {
            transforms: Vec::new(),
            finished: false,
        };
        let words: Vec<_> = text.split_whitespace().collect();
        if words.is_empty() {
            if self.kind != Kind::Copy {
                return Err("The transform needs its next answer".into());
            }
            result.finished = true;
            return Ok(result);
        }
        for word in words {
            if result.finished {
                return Err("Extra input after the completed transform".into());
            }
            if next.kind == Kind::Scale {
                if let Some(mode) = ["1D", "2D", "3D"]
                    .iter()
                    .position(|mode| word.eq_ignore_ascii_case(mode))
                {
                    next.mode = mode as u8 + 1;
                    continue;
                }
            }
            let answer = if let Ok(value) = word.parse::<f64>() {
                if matches!(next.kind, Kind::Copy | Kind::Move) {
                    let points = crate::drawing_input::points(
                        word,
                        fixed,
                        next.frame,
                        &next.points,
                        cursor,
                    )
                    .map_err(str::to_owned)?;
                    Answer::Point(Self::p(points[0]))
                } else {
                    Answer::Number(value)
                }
            } else {
                let points = crate::drawing_input::points(
                    word,
                    fixed,
                    next.frame,
                    &next.points,
                    cursor,
                )
                .map_err(str::to_owned)?;
                Answer::Point(Self::p(points[0]))
            };
            if let Some(transform) = next.accept(answer)? {
                result.transforms.push(transform);
                result.finished = next.kind != Kind::Copy;
            }
        }
        *self = next;
        Ok(result)
    }
    pub fn preview(&self, cursor: Option<[f64; 3]>) -> Option<Xform> {
        if self.kind == Kind::Copy {
            return None;
        }
        let point = cursor?;
        let mut draft = self.clone();
        draft
            .accept(Answer::Point(Self::p(point)))
            .ok()
            .flatten()
    }
    pub fn buttons(&self) -> Vec<(String, String)> {
        let mut b = vec![("Cancel".into(), "Escape".into())];
        if self.kind == Kind::Copy {
            b.insert(0, ("Finish".into(), String::new()));
        }
        if self.kind == Kind::Scale {
            for mode in ["1D", "2D", "3D"] {
                b.push((mode.into(), mode.into()));
            }
        }
        b
    }
}
```

## <a id="checkpoint-77d-resolve-original-cad-parts-and-complete-selected-bounds"></a>23. Resolve CAD parts and complete selected bounds

**What this code does:** Resolve original CAD parts and complete selected bounds.

### 1. `src/source_part.rs`

Resolve existing CAD face and boundary targets against original kernel geometry.

Create the file and type:

```rust
use crate::{
    pick_sources::{Part, Target},
    scene::{ObjectId, Scene},
};
use session_rust::Geometry;
use std::rc::Rc;
pub fn geometry(scene: &Scene, id: ObjectId) -> Result<Geometry, &'static str> {
    let row = scene
        .objects()
        .iter()
        .find(|row| row.id == id)
        .ok_or("Source object is missing")?;
    if let Some(cad) = &row.cad {
        return Ok(Geometry::BRep(Rc::clone(&cad.source)));
    }
    if let Some(surface) = &row.surface {
        return Ok(Geometry::NurbsSurface(Rc::clone(
            &surface.source,
        )));
    }
    Err("Select an original CAD source")
}
pub fn target(part: Part) -> Result<crate::source_deform::Target, &'static str> {
    match part {
        Part::Face(face) => Ok(crate::source_deform::Target::Face(face)),
        Part::BRepEdge(edge) => Ok(crate::source_deform::Target::Edge(
            edge as u32,
        )),
        Part::SurfaceEdge(edge) => Ok(crate::source_deform::Target::Edge(
            u32::try_from(edge).map_err(|_| "Surface edge is out of range")?,
        )),
        _ => Err("Select a CAD face or boundary"),
    }
}
pub fn points(
    scene: &Scene,
    selection: Target,
) -> Result<Vec<session_rust::Point>, String> {
    crate::source_deform::points(
        &geometry(scene, selection.owner).map_err(str::to_owned)?,
        target(selection.part).map_err(str::to_owned)?,
    )
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod source_topology_colour;

pub mod source_mesh_topology;
pub mod transform_draft;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_topology_colour;

pub mod transform_draft;

pub mod source_mesh_topology;
pub mod source_part;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/scene.rs`

Connect current transformation behavior to the existing input and history owner.

**Locate this existing block:**

```rust
            return row.bounds();
        }
        let object = self
            .objects
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            return row.bounds();
        }
        let points =
            if let Some(row) = self.lines.iter().find(|row| row.id == id) {
                Some(
                    row.endpoints()
                        .map(|p| [p[0], p[1], p[2]])
                        .to_vec(),
                )
            } else if let Some(row) = self.points.iter().find(|row| row.id == id) {
                let p = row
                    .model
                    .transform_point(&row.prepared.source);
                Some(vec![[p[0], p[1], p[2]]])
            } else {
                self.paths
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| {
                        row.prepared
                            .source
                            .bounds_coordinates()
                            .into_iter()
                            .map(|p| {
                                let p = row.model.transform_point(
                                    &session_rust::Point::new(p[0], p[1], p[2]),
                                );
                                [p[0], p[1], p[2]]
                            })
                            .collect::<Vec<_>>()
                    })
            };
        if let Some(points) = points {
            let mut bounds = crate::bounds::Bounds::point(*points.first()?);
            for p in points {
                bounds.include(p);
            }
            return Some(bounds);
        }
        let object = self
            .objects
```

## <a id="checkpoint-77e-rebuild-an-edited-original-cad-source-before-adoption"></a>24. Rebuild an edited CAD source before adoption

**What this code does:** Rebuild an edited original CAD source before adoption.

### 1. `src/scene.rs`

Connect current transformation behavior to the existing input and history owner.

**Locate this existing block:**

```rust
            * &model;
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            if row.surface.is_some() || row.cad.is_some() {
                return Err(
                    "CAD surface part editing connects after source parameter mapping",
                );
            }
            let mut source = (**row
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            * &model;
        if self.objects.iter().any(|row| {
            row.id == target.owner && (row.surface.is_some() || row.cad.is_some())
        }) {
            let original = crate::source_part::geometry(self, target.owner)?;
            let edited = crate::source_deform::transform(
                &original,
                crate::source_part::target(target.part)?,
                &local,
            )
            .map_err(
                |_| "CAD edit would invalidate the original source or adjacent trims",
            )?;
            let (cad, surface, prepared) = match edited {
                session_rust::Geometry::BRep(source) => {
                    let (cad, prepared) = crate::cad_display::prepare(source)?;
                    (Some(Rc::new(cad)), None, prepared)
                }
                session_rust::Geometry::NurbsSurface(source) => {
                    let surface = Rc::new(crate::surface::Prepared::new(source)?);
                    let prepared = surface.mesh()?;
                    (None, Some(surface), prepared)
                }
                _ => return Err("Unexpected edited CAD source"),
            };
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
            let row = self
                .objects
                .iter_mut()
                .find(|row| row.id == target.owner)
                .unwrap();
            row.mesh = Rc::new(prepared.display);
            row.cad = cad;
            row.surface = surface;
            row.editable = crate::edit_source::EditSource::Loaded {
                geometry: prepared.geometry,
                source: None,
            };
            return Ok(());
        }
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            let mut source = (**row
```

### 2. `src/part_preview.rs`

Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .objects()
            .iter()
            .find(|r| r.id == id)
            .and_then(|r| r.geometry())
            .zip(row.geometry())
            .is_some_and(|(a, b)| Rc::ptr_eq(a, b));
    }
```

Replace it with:

```rust
            .objects()
            .iter()
            .find(|r| r.id == id)
            .is_some_and(|other| {
                match (
                    &row.cad,
                    &other.cad,
                    &row.surface,
                    &other.surface,
                ) {
                    (Some(a), Some(b), _, _) => Rc::ptr_eq(&a.source, &b.source),
                    (_, _, Some(a), Some(b)) => Rc::ptr_eq(&a.source, &b.source),
                    (None, None, None, None) => other
                        .geometry()
                        .zip(row.geometry())
                        .is_some_and(|(a, b)| Rc::ptr_eq(a, b)),
                    _ => false,
                }
            });
    }
```

**Locate this block:**

```rust
        if row.surface.is_some() || row.cad.is_some() {
            return Err(
                "CAD surface part editing connects after source parameter mapping",
            );
        }
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
```

Replace it with:

```rust
        if row.surface.is_some() || row.cad.is_some() {
            crate::source_part::points(scene, target)
                .map_err(|_| "Cannot resolve original CAD part")?
        } else {
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
        }
    } else {
        let row = scene
            .paths()
```

## <a id="checkpoint-77f-copy-original-rows-beside-their-existing-parents"></a>25. Copy rows beside their existing parents

**What this code does:** Copy original rows beside their existing parents.

### 1. `src/layer_edit.rs`

Copy checked original rows under their existing parent with current attributes, new identities and placed source sharing.

**Locate this existing block:**

```rust
        Edit::Copy(ids) => transfer(scene, key, ids, true),
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Edit::Copy(ids) => transfer(scene, key, ids, true),
    }
}

pub(crate) fn copy_objects(
    scene: &mut Scene,
    ids: &[crate::scene::ObjectId],
    delta: &session_rust::Xform,
) -> Result<Vec<crate::scene::ObjectId>, &'static str> {
    if ids.is_empty() {
        return Err("Select objects before Copy");
    }
    if !crate::placement::valid(&delta.m) {
        return Err("Copy transform is invalid");
    }
    let mut chosen = ids.to_vec();
    chosen.sort_unstable_by_key(|id| id.value());
    chosen.dedup();
    if chosen
        .iter()
        .any(|id| scene.pick_id(id.value()) != Some(*id))
    {
        return Err("Copy needs visible unlocked editable objects");
    }
    let mut trees = scene
        .forest
        .documents
        .iter()
        .map(|doc| branch(doc, 0))
        .collect::<Vec<_>>();
    let mut sources = Vec::new();
    for (owner, doc) in scene.forest.documents.iter().enumerate() {
        for (index, node) in doc.nodes.iter().enumerate() {
            let Some(id) = node.object.filter(|id| chosen.contains(id)) else {
                continue;
            };
            let parent = node.parent.ok_or("Object has no parent")?;
            let mut tree = branch(doc, index);
            tree.children.retain(|child| {
                child.node.object.is_none() && child.node.source.name == "attributes"
            });
            sources.push((
                owner,
                doc.nodes[parent].source.guid.clone(),
                tree,
                scene
                    .placement(id)
                    .ok_or("Copy placement is missing")?,
            ));
        }
    }
    if sources.len() != chosen.len() {
        return Err("Copy source tree is missing");
    }
    for (owner, doc) in scene.forest.documents.iter().enumerate() {
        let extra = sources
            .iter()
            .filter(|source| source.0 == owner)
            .map(|source| {
                let mut nodes = Vec::new();
                flatten(source.2.clone(), None, 0, &mut nodes);
                nodes.len()
            })
            .sum::<usize>();
        if doc.nodes.len() + extra > crate::hierarchy::MAX_NODES {
            return Err("Copy exceeds the tree limit");
        }
    }
    let mut copies = Vec::new();
    fn place_copy(
        scene: &mut Scene,
        tree: &Branch,
        delta: &session_rust::Xform,
    ) -> Result<(), &'static str> {
        if let Some(id) = tree.node.object {
            let model = scene
                .placement(id)
                .ok_or("Copy placement is missing")?;
            scene.place_following(id, delta * &model)?;
        }
        for child in &tree.children {
            place_copy(scene, child, delta)?;
        }
        Ok(())
    }
    for (owner, parent, tree, _model) in sources {
        let copy = copy_branch(scene, tree, false)?;
        let id = copy
            .node
            .object
            .ok_or("Copy owner is missing")?;
        place_copy(scene, &copy, delta)?;
        copies.push(id);
        find(&mut trees[owner], &parent)
            .ok_or("Copy parent is missing")?
            .children
            .push(copy);
    }
    for (owner, tree) in trees.into_iter().enumerate() {
        replace(scene, owner, tree);
    }
    Ok(copies)
}
```

## <a id="checkpoint-77g-carry-transformation-state-through-the-existing-drawing-owner"></a>26. Carry transformation state through the existing drawing owner

**What this code does:** Carry transformation state through the existing drawing owner.

### 1. `src/tool.rs`

Carry current transform question owners and buttons through the existing Draft.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            } else {
                None
            },
        }
    }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {
```

Replace it with:

```rust
            } else {
                None
            },
            transform: if let Shape::Transform(kind) = shape {
                Some(crate::transform_draft::Draft::new(
                    kind,
                    crate::cplane::CPlane::Xy,
                ))
            } else {
                None
            },
        }
    }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {
            if let Some(transform) = &self.transform {
                return transform.prompt();
            }
        }
        if self.plane_points.is_none() {
```

**Locate this block:**

```rust
    pub frame: crate::cplane::CPlane,
    pub plane_points: Option<Vec<[f64; 3]>>,
    pub primitive: Option<crate::primitive_draft::Draft>,
}

impl Draft {
```

Replace it with:

```rust
    pub frame: crate::cplane::CPlane,
    pub plane_points: Option<Vec<[f64; 3]>>,
    pub primitive: Option<crate::primitive_draft::Draft>,
    pub transform: Option<crate::transform_draft::Draft>,
}

impl Draft {
```

**Locate this block:**

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if let Self::Primitive(shape) = self {
            return shape
                .options
```

Replace it with:

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if let Self::Transform(kind) = self {
            return crate::transform_draft::Draft::new(
                *kind,
                crate::cplane::CPlane::Xy,
            )
            .buttons();
        }
        if let Self::Primitive(shape) = self {
            return shape
                .options
```

**Locate this block:**

```rust
            Self::Line | Self::Move => 2,
            Self::Polyline | Self::Curve => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) => 0,
        }
    }
    fn sides(&self) -> Option<usize> {
```

Replace it with:

```rust
            Self::Line | Self::Move => 2,
            Self::Polyline | Self::Curve => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) | Self::Transform(_) => 0,
        }
    }
    fn sides(&self) -> Option<usize> {
```

**Locate this block:**

```rust
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
        }
    }
    fn count(&self) -> usize {
```

Replace it with:

```rust
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
            Self::Transform(kind) => kind.name(),
        }
    }
    fn count(&self) -> usize {
```

**Locate this block:**

```rust
    Primitive(&'static crate::primitive_shape::Shape),
}

impl Tool for Shape {
```

Replace it with:

```rust
    Primitive(&'static crate::primitive_shape::Shape),
    Transform(crate::transform_draft::Kind),
}

impl Tool for Shape {
```

### 2. `src/editor.rs`

Own temporary original-part and whole-placement transformation previews. Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
```

Replace it with:

```rust
    pub fn presentation_scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes()
            .chain(
                self.transform_preview
                    .iter()
                    .flat_map(|p| p.scenes()),
            )
            .chain(
```

**Locate this block:**

```rust
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
```

Replace it with:

```rust
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform) = self
            .draft
            .as_ref()
            .and_then(|draft| draft.transform.as_ref())
        {
            let matrix = transform
                .preview(
                    self.draft
                        .as_ref()
                        .and_then(|draft| draft.cursor),
                )
                .unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview) = self.transform_preview.as_mut() {
                preview.set(matrix)?;
                return Ok(Change::Preview);
            }
            return Ok(Change::View);
        }
        let Some(preview) = self.edit_preview.as_mut() else {
            return Ok(Change::View);
        };
```

**Locate this block:**

```rust
                || {
                    self.gizmo_edit.as_ref().map_or_else(
                        || {
                            self.edit_preview
                                .as_ref()
                                .map_or(&self.scene, |p| &p.scene)
                        },
                        |g| g.preview.scene(),
                    )
```

Replace it with:

```rust
                || {
                    self.gizmo_edit.as_ref().map_or_else(
                        || {
                            self.transform_preview.as_ref().map_or_else(
                                || {
                                    self.edit_preview
                                        .as_ref()
                                        .map_or(&self.scene, |p| &p.scene)
                                },
                                |p| p.scene(),
                            )
                        },
                        |g| g.preview.scene(),
                    )
```

**Locate this block:**

```rust
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            draft: None,
            tool_status: String::new(),
            construction_plane: None,
```

Replace it with:

```rust
            edit_preview: None,
            snapping: Default::default(),
            snap_index: None,
            transform_preview: None,
            draft: None,
            tool_status: String::new(),
            construction_plane: None,
```

**Locate this block:**

```rust
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

Replace it with:

```rust
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview: Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

## <a id="checkpoint-77h-validate-selected-originals-before-beginning-or-committing-a-transform"></a>27. Validate selected originals before beginning or committing a transform

**What this code does:** Validate selected originals before beginning or committing a transform.

### 1. `src/editor.rs`

Connect checked original transformations to the current Editor transaction boundary.

**Locate this existing block:**

```rust
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    fn apply_transform(
        &mut self,
        kind: crate::transform_draft::Kind,
        delta: &session_rust::Xform,
        ids: &[ObjectId],
        target: Option<crate::pick_sources::Target>,
    ) -> Result<Change, &'static str> {
        if !crate::placement::valid(&delta.m) {
            return Err("Transform must be finite, affine and invertible");
        }
        if kind == crate::transform_draft::Kind::Copy {
            if target.is_some() {
                return Err("Copy works on whole objects");
            }
            self.history
                .try_edit(&mut self.scene, |scene| {
                    crate::layer_edit::copy_objects(scene, ids, delta).map(|_| ())
                })?;
            return Ok(Change::Scene);
        }
        let mut preview =
            crate::gizmo_edit::Preview::new(&self.scene, ids.to_vec(), target)?;
        preview.set(delta.clone())?;
        let changed = preview.commit(&mut self.scene, &mut self.history)?;
        Ok(if changed {
            Change::Scene
        } else {
            Change::Preview
        })
    }
    fn begin_transform(
        &mut self,
        kind: crate::transform_draft::Kind,
    ) -> Result<Change, &'static str> {
        let owners = self.selected_objects();
        let target = self
            .selection()
            .filter(|_| self.selected_part.is_some());
        let preview = if kind == crate::transform_draft::Kind::Copy {
            if target.is_some() {
                return Err("Copy works on whole objects");
            }
            let mut candidate = self.scene.clone();
            crate::layer_edit::copy_objects(
                &mut candidate,
                &owners,
                &session_rust::Xform::identity(),
            )?;
            None
        } else {
            Some(crate::gizmo_edit::Preview::new(
                &self.scene,
                owners.clone(),
                target,
            )?)
        };
        let frame = self.construction_plane.unwrap_or_else(|| {
            crate::cplane::CPlane::view(
                &self
                    .camera
                    .orientation
                    .rotate_vector(session_rust::Vector::y_axis()),
            )
        });
        let mut draft = crate::tool::Draft::new(crate::tool::Shape::Transform(kind));
        draft.frame = frame;
        let transform = draft.transform.as_mut().unwrap();
        transform.frame = frame;
        transform.owners = owners;
        transform.target = target;
        self.edit_preview = None;
        self.transform_preview = preview;
        self.draft = Some(draft);
        self.snap_index = None;
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::Preview)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

## <a id="checkpoint-77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions"></a>28. Adopt validated outcomes and preserve repeat copy transactions

**What this code does:** Adopt validated outcomes and preserve repeat copy transactions.

### 1. `src/editor.rs`

Connect checked original transformations to the current Editor transaction boundary.

**Locate this existing block:**

```rust
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    fn transform_outcome(
        &mut self,
        next: crate::transform_draft::Draft,
        outcome: crate::transform_draft::Outcome,
    ) -> Result<Change, &'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate = self.scene.clone();
        for delta in &outcome.transforms {
            if next.kind == crate::transform_draft::Kind::Copy {
                crate::layer_edit::copy_objects(&mut candidate, &next.owners, delta)?;
            } else {
                let mut preview = crate::gizmo_edit::Preview::new(
                    &candidate,
                    next.owners.clone(),
                    next.target,
                )?;
                preview.set(delta.clone())?;
                let mut history = crate::history::History::default();
                preview.commit(&mut candidate, &mut history)?;
            }
        }
        for delta in &outcome.transforms {
            self.apply_transform(next.kind, delta, &next.owners, next.target)?;
        }
        let changed = !outcome.transforms.is_empty();
        if outcome.finished {
            self.draft = None;
            self.transform_preview = None;
            self.tool_status = format!("{} completed.", next.kind.name());
            return Ok(if changed {
                Change::Scene
            } else {
                Change::Preview
            });
        }
        let draft = self
            .draft
            .as_mut()
            .ok_or("No transform tool is running")?;
        draft.points = next.points.clone();
        draft.transform = Some(next);
        draft.cursor = None;
        draft.snapped = None;
        self.tool_status = draft.prompt();
        if changed {
            self.snap_index = None;
            return Ok(Change::Scene);
        }
        self.update_edit_preview()
    }
    fn transform_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let mut next = self
            .draft
            .as_ref()
            .and_then(|draft| draft.transform.clone())
            .ok_or("No transform tool is running")?;
        let transform = next
            .accept(crate::primitive_shape::Answer::Point(
                session_rust::Point::new(point[0], point[1], point[2]),
            ))
            .map_err(|message| self.primitive_failure(message))?;
        let finished =
            transform.is_some() && next.kind != crate::transform_draft::Kind::Copy;
        self.transform_outcome(
            next,
            crate::transform_draft::Outcome {
                transforms: transform.into_iter().collect(),
                finished,
            },
        )
    }
    fn transform_text(&mut self, text: &str) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_ref()
            .ok_or("No transform tool is running")?;
        let mut next = draft
            .transform
            .clone()
            .ok_or("No transform tool is running")?;
        let outcome = next
            .text(text, draft.cursor, self.construction_plane)
            .map_err(|message| self.primitive_failure(message))?;
        self.transform_outcome(next, outcome)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

## <a id="checkpoint-77j-route-transform-input-cancellation-and-current-plane-changes"></a>29. Route transform input cancellation and current plane changes

**What this code does:** Route transform input cancellation and current plane changes.

### 1. `src/editor.rs`

Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .as_ref()
            .filter(|d| {
                d.primitive.is_none()
                    && d.tool.count() == 0
```

Replace it with:

```rust
            .as_ref()
            .filter(|d| {
                d.primitive.is_none()
                    && d.transform.is_none()
                    && d.tool.count() == 0
```

**Locate this block:**

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.primitive.is_some() && d.plane_points.is_none())
        {
            return self.primitive_text(text);
```

Replace it with:

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.transform.is_some() && d.plane_points.is_none())
        {
            return self.transform_text(text);
        }

        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.primitive.is_some() && d.plane_points.is_none())
        {
            return self.primitive_text(text);
```

**Locate this block:**

```rust
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
```

Replace it with:

```rust
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.transform.is_some() && d.plane_points.is_none())
        {
            return self.transform_point(point);
        }

        let draft = self
            .draft
            .as_mut()
```

**Locate this block:**

```rust
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count() == 0 {
            return Some(Ok(Action::CloseTool));
        }
```

Replace it with:

```rust
        }
        if draft.plane_points.is_none() && draft.transform.is_some() {
            let first = text.split_whitespace().next();
            if text.trim().is_empty()
                || first.is_some_and(|word| {
                    word.parse::<f64>().is_ok()
                        || word.contains(',')
                        || word.contains('<')
                        || word.starts_with('@')
                        || word.starts_with('w')
                        || word.starts_with('W')
                        || ["1D", "2D", "3D"]
                            .iter()
                            .any(|mode| word.eq_ignore_ascii_case(mode))
                })
            {
                return Some(Ok(Action::ToolText(text.into())));
            }
            return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count() == 0 {
            return Some(Ok(Action::CloseTool));
        }
```

## <a id="checkpoint-77k-connect-transformation-lifecycle-and-part-move-dispatch"></a>30. Connect transformation lifecycle and part Move dispatch

**What this code does:** Connect transformation lifecycle and part Move dispatch.

### 1. `src/editor.rs`

Register transform lifecycle actions before connecting current input dispatch. Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self
```

Replace it with:

```rust
                        scene.add_box().map(|_| ())
                    })?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some() {
                    let target = self.selection();
                    return self.apply_transform(
                        crate::transform_draft::Kind::Move,
                        &session_rust::Xform::translation(
                            offset[0], offset[1], offset[2],
                        ),
                        &[],
                        target,
                    );
                }
                return self.move_selection(offset);
            }
            Action::ToggleExtra => self
```

**Locate this block:**

```rust
                return self.update_edit_preview();
            }
            Action::CancelTool => {
                let changed = self.edit_preview.take().is_some();
                self.draft = None;
                self.plane_menu = false;
                self.plane_only = false;
```

Replace it with:

```rust
                return self.update_edit_preview();
            }
            Action::CancelTool => {
                let changed = self.edit_preview.take().is_some()
                    | self.transform_preview.take().is_some();
                self.draft = None;
                self.plane_menu = false;
                self.plane_only = false;
```

**Locate this block:**

```rust
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview = if matches!(shape, crate::tool::Shape::Move) {
                    if self.selected_part.is_some() {
                        return Err("Use Select Object before Move");
```

Replace it with:

```rust
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape, crate::tool::Shape::Move)
                    && self.selected_part.is_some()
                {
                    return self.begin_transform(crate::transform_draft::Kind::Move);
                }
                self.transform_preview = None;
                let preview = if matches!(shape, crate::tool::Shape::Move) {
                    if self.selected_part.is_some() {
                        return Err("Use Select Object before Move");
```

**Locate this block:**

```rust
                    if hit.is_some_and(|h| {
                        self.draft.as_ref().is_some_and(|d| {
                            d.primitive.is_none()
                                && d.tool.count() == 0
                                && d.points.len() >= 2
                                && d.points.first().is_some_and(|p| {
```

Replace it with:

```rust
                    if hit.is_some_and(|h| {
                        self.draft.as_ref().is_some_and(|d| {
                            d.primitive.is_none()
                                && d.transform.is_none()
                                && d.tool.count() == 0
                                && d.points.len() >= 2
                                && d.points.first().is_some_and(|p| {
```

**Locate this block:**

```rust
                            "Shape input invalid"
                        })?;
                }
                self.construction_plane = plane;
                crate::drawing_coords::set_plane(plane);
                self.plane_menu = false;
```

Replace it with:

```rust
                            "Shape input invalid"
                        })?;
                }
                if let Some(transform) = self
                    .draft
                    .as_mut()
                    .and_then(|d| d.transform.as_mut())
                {
                    transform.frame = frame;
                }
                self.construction_plane = plane;
                crate::drawing_coords::set_plane(plane);
                self.plane_menu = false;
```

**Locate this block:**

```rust
            self.plane_only = false;
        }
        match action {
            Action::PrimitiveBegin {
                shape,
                option,
```

Replace it with:

```rust
            self.plane_only = false;
        }
        match action {
            Action::BeginTransformation(kind) => return self.begin_transform(kind),
            Action::Transformation { kind, delta } => {
                let ids = self.selected_objects();
                let target = self
                    .selection()
                    .filter(|_| self.selected_part.is_some());
                return self.apply_transform(kind, &delta, &ids, target);
            }
            Action::PrimitiveBegin {
                shape,
                option,
```

**Locate this block:**

```rust
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
            change = Change::Preview;
        }
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
```

Replace it with:

```rust
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change == Change::View {
            change = Change::Preview;
        }
        if !keep_edit
            && self
                .draft
                .as_ref()
                .is_some_and(|d| d.transform.is_some())
        {
            self.transform_preview = None;
            self.draft = None;
            if change == Change::View {
                change = Change::Preview;
            }
        }
        if !keep_edit && self.edit_preview.take().is_some() {
            self.draft = None;
            if change == Change::View {
```

**Locate this block:**

```rust
                | Action::PlaneMenu
                | Action::Plane3
                | Action::ToolSides(_)
                | Action::PrimitiveBegin { .. }
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

Replace it with:

```rust
                | Action::PlaneMenu
                | Action::Plane3
                | Action::ToolSides(_)
                | Action::BeginTransformation(_)
                | Action::Transformation { .. }
                | Action::PrimitiveBegin { .. }
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
```

**Locate this block:**

```rust
    StreamRefresh,
    PrimitiveBegin {
        shape: &'static crate::primitive_shape::Shape,
        option: usize,
```

Replace it with:

```rust
    StreamRefresh,
    BeginTransformation(crate::transform_draft::Kind),
    Transformation {
        kind: crate::transform_draft::Kind,
        delta: session_rust::Xform,
    },
    PrimitiveBegin {
        shape: &'static crate::primitive_shape::Shape,
        option: usize,
```

### 2. `src/selection_tests.rs`

Keep invalid Move and parent-deletion guards when source-part Move becomes available.

**Locate this existing block:**

```rust
        editor
            .apply(Action::Translate([1., 0., 0.]))
            .is_err()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        editor
            .apply(Action::Translate([f64::INFINITY, 0., 0.]))
            .is_err()
```

### 3. `src/edit_preview_tests.rs`

Allow part Move previews and cancellation while retaining locked, cold and stale target guards.

**Locate this existing block:**

```rust
    e.selected_part = Some(crate::pick_sources::Part::Face(0));
    assert!(
        e.apply(Action::BeginTool(Shape::Move))
            .is_err()
    );
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    e.selected_part = Some(crate::pick_sources::Part::Face(0));
    e.apply(Action::BeginTool(Shape::Move))
        .unwrap();
    assert!(e.transform_preview.is_some());
    e.apply(Action::CancelTool).unwrap();
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
```

## <a id="checkpoint-77l-register-copy-scale-rotate-and-their-current-aliases"></a>31. Register Copy Scale Rotate and their current aliases

**What this code does:** Register Copy Scale Rotate and their current aliases.

### 1. `src/transform_commands.rs`

Parse current direct and interactive transform names, options and aliases.

Create the file and type:

```rust
use crate::{editor::Action, transform_draft::Kind};
pub const NAMES: &[&str] = &[
    "Copy", "Scale", "Rotate", "Rotate x", "Rotate y", "Rotate z",
];
pub fn action(name: &str, rest: &[&str]) -> Option<Result<Action, String>> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some((|| match name {
        "Copy" => {
            if rest.is_empty() {
                Ok(Action::BeginTransformation(Kind::Copy))
            } else {
                let offset = crate::offset::parse(&format!("Move {}", rest.join(" ")))
                    .map_err(str::to_owned)?;
                Ok(Action::Transformation {
                    kind: Kind::Copy,
                    delta: session_rust::Xform::translation(
                        offset[0], offset[1], offset[2],
                    ),
                })
            }
        }
        "Scale" => match rest {
            [] => Ok(Action::BeginTransformation(Kind::Scale)),
            [value] => {
                let factor = value
                    .parse::<f64>()
                    .ok()
                    .filter(|v| v.is_finite() && *v > 0.)
                    .ok_or("Scale wants a finite factor above zero")?;
                Ok(Action::ScaleFactor(factor))
            }
            _ => Err("Scale takes one factor or interactive points".into()),
        },
        "Rotate" if rest.is_empty() => Ok(Action::BeginTransformation(Kind::Rotate)),
        _ => {
            let (axis, values) = if let Some(axis) = name.strip_prefix("Rotate ") {
                (axis, rest)
            } else {
                let Some((axis, values)) = rest.split_first() else {
                    return Err("Rotate x 90".into());
                };
                (*axis, values)
            };
            let axis = match axis.to_ascii_lowercase().as_str() {
                "x" => crate::gizmo::Axis::X,
                "y" => crate::gizmo::Axis::Y,
                "z" => crate::gizmo::Axis::Z,
                _ => return Err("Rotate axis must be x y or z".into()),
            };
            let [degrees] = values else {
                return Err("Rotate needs one angle in degrees".into());
            };
            let degrees = degrees
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
                .ok_or("Rotate needs finite degrees")?;
            Ok(Action::RotateWorld { axis, degrees })
        }
    })())
}
pub fn alias(line: &str) -> String {
    let mut words = line.split_whitespace();
    let Some(first) = words.next() else {
        return String::new();
    };
    let name = match first.to_ascii_lowercase().as_str() {
        "m" => "Move",
        "s" => "Scale",
        "rot" => "Rotate",
        _ => return line.trim().to_owned(),
    };
    std::iter::once(name)
        .chain(words)
        .collect::<Vec<_>>()
        .join(" ")
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod transform_draft;

pub mod source_mesh_topology;
pub mod source_part;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod transform_draft;

pub mod source_part;

pub mod source_mesh_topology;
pub mod transform_commands;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/commands.rs`

Register current transforms and preserve longest-name matching and typed aliases.

**Locate this existing block:**

```rust
        )
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action) =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .chain(
            crate::transform_commands::NAMES
                .iter()
                .copied(),
        )
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let alias = crate::transform_commands::alias(line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let alias = crate::transform_commands::alias(line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action) = crate::transform_commands::action(name, &words[count..]) {
        return action.map(Command::Editor);
    }
    if let Some(action) =
```

### 4. `src/editor.rs`

Carry typed scale factors and world-axis rotation angles to the existing Editor.

**Locate this existing block:**

```rust
    BeginTransformation(crate::transform_draft::Kind),
    Transformation {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    BeginTransformation(crate::transform_draft::Kind),
    ScaleFactor(f64),
    RotateWorld {
        axis: crate::gizmo::Axis,
        degrees: f64,
    },
    Transformation {
```

## <a id="checkpoint-77m-apply-direct-scale-and-rotate-about-the-current-selection-center"></a>32. Apply direct Scale and Rotate about the current selection center

**What this code does:** Apply direct Scale and Rotate about the current selection center.

### 1. `src/editor.rs`

Apply direct current Scale and Rotate about the selection gizmo center.

**Locate this existing block:**

```rust
        match action {
            Action::BeginTransformation(kind) => return self.begin_transform(kind),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match action {
            Action::ScaleFactor(factor) => {
                if !factor.is_finite() || factor <= 0. {
                    return Err("Scale wants a finite factor above zero");
                }
                let origin = self.gizmo_origin().unwrap_or([0.; 3]);
                let delta = session_rust::Xform::scale_uniform(
                    &session_rust::Point::new(origin[0], origin[1], origin[2]),
                    factor,
                );
                let ids = self.selected_objects();
                let target = self
                    .selection()
                    .filter(|_| self.selected_part.is_some());
                return self.apply_transform(
                    crate::transform_draft::Kind::Scale,
                    &delta,
                    &ids,
                    target,
                );
            }
            Action::RotateWorld { axis, degrees } => {
                if !degrees.is_finite() {
                    return Err("Rotate needs finite degrees");
                }
                let origin = self.gizmo_origin().unwrap_or([0.; 3]);
                let (o, p) = (
                    session_rust::Xform::translation(origin[0], origin[1], origin[2]),
                    session_rust::Xform::translation(
                        -origin[0], -origin[1], -origin[2],
                    ),
                );
                let turn = match axis {
                    crate::gizmo::Axis::X => {
                        session_rust::Xform::rotation_x(degrees, true)
                    }
                    crate::gizmo::Axis::Y => {
                        session_rust::Xform::rotation_y(degrees, true)
                    }
                    crate::gizmo::Axis::Z => {
                        session_rust::Xform::rotation_z(degrees, true)
                    }
                };
                let delta = &(&o * &turn) * &p;
                let ids = self.selected_objects();
                let target = self
                    .selection()
                    .filter(|_| self.selected_part.is_some());
                return self.apply_transform(
                    crate::transform_draft::Kind::Rotate,
                    &delta,
                    &ids,
                    target,
                );
            }
            Action::BeginTransformation(kind) => return self.begin_transform(kind),
```

## <a id="checkpoint-77n-retain-earlier-selection-checks-with-source-part-support"></a>33. Retain earlier selection checks with source part support

**What this code does:** Retain earlier selection checks with source part support.

### 1. `src/edit_preview_tests.rs`

Copy retained selection/preview fixtures with current source-part Move support.

**Locate this existing block:**

```rust
#[test]
fn preview_rejects_parts_locked_or_cold_sources_and_stale_targets() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene, vec![]).is_err());
    let mut preview = Preview::new(&e.scene, vec![id, id]).unwrap();
    assert_eq!(preview.ids(), &[id]);
    preview
        .set(Xform::translation(1., 0., 0.))
        .unwrap();
    e.scene.translate(id, [2., 0., 0.]).unwrap();
    assert!(
        preview
            .commit(
                &mut e.scene,
                &mut crate::history::History::default()
            )
            .is_err()
    );
    e.apply(Action::SelectAll).unwrap();
    e.selected_part = Some(crate::pick_sources::Part::Face(0));
    e.apply(Action::BeginTool(Shape::Move))
        .unwrap();
    assert!(e.transform_preview.is_some());
    e.apply(Action::CancelTool).unwrap();
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn preview_checks_locked_cold_and_stale_targets_and_routes_parts_to_source_editing() {
    let mut e = editor();
    let id = e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene, vec![]).is_err());
    let mut preview = Preview::new(&e.scene, vec![id, id]).unwrap();
    assert_eq!(preview.ids(), &[id]);
    preview
        .set(Xform::translation(1., 0., 0.))
        .unwrap();
    e.scene.translate(id, [2., 0., 0.]).unwrap();
    assert!(
        preview
            .commit(
                &mut e.scene,
                &mut crate::history::History::default()
            )
            .is_err()
    );
    e.apply(Action::SelectAll).unwrap();
    e.selected_part = Some(crate::pick_sources::Part::Face(0));
    let original = state(&e.scene);
    e.apply(Action::BeginTool(Shape::Move))
        .unwrap();
    assert!(e.transform_preview.is_some());
    e.apply(Action::CancelTool).unwrap();
    assert_eq!(state(&e.scene), original);
    let mut mesh = session_rust::Mesh::create_box(1., 1., 1.);
```

## <a id="checkpoint-77o-exercise-original-copy-transforms-history-and-exact-persistence"></a>34. Exercise copy transforms history and exact persistence

**What this code does:** Exercise original copy transforms history and exact persistence.

### 1. `src/transform_tests.rs`

Copy integrated current copy/transform/original-edit/history/nested SaveOpen fixtures.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Part, Target},
    transform_draft::Kind,
};
use session_rust::Xform;
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, line: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(line) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(line)? else {
            return Err("Editor command".into());
        };
        a
    };
    e.apply(action).map_err(|message| {
        if message == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            message.into()
        }
    })?;
    Ok(())
}
#[test]
fn copy_keeps_parent_name_colours_source_and_selection_with_new_identity_and_one_undo()
{
    let mut e = editor();
    command(&mut e, "Box w0,0,0 1 2 3").unwrap();
    let id = e.scene.objects()[0].id;
    let original = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    e.select_objects([id], false);
    std::rc::Rc::make_mut(&mut e.scene.view)
        .faces
        .insert(id, [201, 12, 33]);
    let parent = e.scene.forest.documents[0]
        .nodes
        .iter()
        .find(|n| n.object == Some(id))
        .unwrap()
        .parent;
    command(&mut e, "Copy 4,0,0").unwrap();
    let copy = e.scene.objects()[1].id;
    assert_ne!(
        e.scene.objects()[0].guid,
        e.scene.objects()[1].guid
    );
    assert!(Rc::ptr_eq(
        &e.scene.objects()[1]
            .cad
            .as_ref()
            .unwrap()
            .source,
        &original
    ));
    assert_eq!(e.selected_objects(), vec![id]);
    assert_eq!(
        e.scene.view.faces.get(&copy),
        Some(&[201, 12, 33])
    );
    let doc = &e.scene.forest.documents[0];
    let a = doc
        .nodes
        .iter()
        .find(|n| n.object == Some(id))
        .unwrap();
    let b = doc
        .nodes
        .iter()
        .find(|n| n.object == Some(copy))
        .unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(b.parent, parent);
    assert_eq!(e.scene.objects()[1].model.m[12], 4.);
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    let mut saved = crate::scene::Scene::empty();
    crate::scene_document::open(&mut saved, &bytes).unwrap();
    assert_eq!(saved.objects().len(), 2);
    assert!(
        saved
            .objects()
            .iter()
            .any(|row| row.model.m[12] == 4.)
    );
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 2);
}
#[test]
fn repeated_copy_targets_each_commit_once_and_enter_or_escape_keeps_originals_selected()
{
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Copy").unwrap();
    command(&mut e, "w0,0,0 w2,0,0 w4,0,0").unwrap();
    assert_eq!(e.scene.lines().len(), 3);
    assert_eq!(
        e.draft.as_ref().unwrap().points,
        vec![[0.; 3]]
    );
    assert_eq!(e.selected_objects(), vec![id]);
    command(&mut e, "").unwrap();
    assert!(e.draft.is_none());
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines().len(), 2);
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines().len(), 1);
    command(&mut e, "Redo").unwrap();
    command(&mut e, "Copy").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.lines().len(), 3);
}
#[test]
fn scaling_modes_reference_lengths_and_pending_preview_keep_original_source_unchanged()
{
    let mut e = editor();
    command(&mut e, "Box w0,0,0 1 2 3").unwrap();
    let id = e.scene.objects()[0].id;
    let source = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    e.select_objects([id], false);
    command(&mut e, "Scale").unwrap();
    command(&mut e, "1D w0,0,0").unwrap();
    assert!(command(&mut e, "2").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "w1,0,0").unwrap();
    e.apply(Action::ToolHover(Some([2., 0., 0.])))
        .unwrap();
    assert_eq!(
        e.scene.objects()[0].model.m,
        Xform::identity().m
    );
    assert_eq!(e.display_scene().objects()[0].model.m[0], 2.);
    command(&mut e, "3").unwrap();
    assert_eq!(e.scene.objects()[0].model.m[0], 3.);
    assert_eq!(e.scene.objects()[0].model.m[5], 1.);
    assert!(Rc::ptr_eq(
        &e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source,
        &source
    ));
    command(&mut e, "Undo").unwrap();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Scale").unwrap();
    command(&mut e, "2D w0,0,0 2").unwrap();
    let model = &e.scene.objects()[0].model;
    assert_eq!(
        [model.m[0], model.m[5], model.m[10]],
        [2., 1., 2.]
    );
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Scale").unwrap();
    command(&mut e, "3D w0,0,0 w1,0,0 w2,0,0").unwrap();
    assert_eq!(
        [
            e.scene.objects()[0].model.m[0],
            e.scene.objects()[0].model.m[5],
            e.scene.objects()[0].model.m[10]
        ],
        [2.; 3]
    );
}
#[test]
fn rotate_uses_gizmo_center_for_direct_axes_and_plane_normal_for_references() {
    let mut e = editor();
    command(&mut e, "Line w1,0,0 w2,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    e.select_objects([id], false);
    command(&mut e, "rot z 90").unwrap();
    let p = e.scene.lines()[0]
        .model
        .transform_point(&source.start());
    assert!((p[0] - 1.5).abs() < 1e-12 && (p[1] + 0.5).abs() < 1e-12);
    assert!(Rc::ptr_eq(
        &e.scene.lines()[0].prepared.source,
        &source
    ));
    command(&mut e, "Undo").unwrap();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Rotate").unwrap();
    command(&mut e, "w0,0,0 w1,0,0 w0,0,1").unwrap();
    let p = e.scene.lines()[0]
        .model
        .transform_point(&source.start());
    assert!(p[0].abs() < 1e-12 && (p[2] - 1.).abs() < 1e-12);
}
#[test]
fn invalid_transform_commands_batches_and_copy_parts_preserve_active_input_and_redo() {
    let mut e = editor();
    command(&mut e, "Curve w0,0,0 w1,1,0 w2,0,0").unwrap();
    let id = e.scene.paths()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Copy 1,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Scale").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    for line in [
        "0",
        "NaN",
        "2 extra",
        "Rotate q 90",
        "Copy NaN,0,0",
    ] {
        assert!(command(&mut e, line).is_err(), "{line}");
        assert_eq!(
            e.draft.as_ref().unwrap().tool.name(),
            "Scale"
        );
        assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
        assert_eq!(e.scene.paths().len(), 1);
    }
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.paths().len(), 2);
    e.apply(Action::Controls(true)).unwrap();
    e.apply(Action::PickedTarget(Some(Target {
        owner: id,
        part: Part::Control(1),
    })))
    .unwrap();
    assert!(command(&mut e, "Copy").is_err());
    assert!(e.draft.is_none());
}
#[test]
fn typed_move_and_scale_edit_a_rational_curve_control_without_changing_its_weight() {
    let mut e = editor();
    command(&mut e, "Nurbs Curve Arc Center w0,0,0 1 90").unwrap();
    let id = e.scene.paths()[0].id;
    let crate::chain::Source::Curve(original, _) =
        e.scene.paths()[0].prepared.source.clone()
    else {
        panic!("Curve")
    };
    e.apply(Action::Controls(true)).unwrap();
    e.apply(Action::PickedTarget(Some(Target {
        owner: id,
        part: Part::Control(1),
    })))
    .unwrap();
    command(&mut e, "m 0.125,0.25,0").unwrap();
    let crate::chain::Source::Curve(edited, _) = &e.scene.paths()[0].prepared.source
    else {
        panic!("Curve")
    };
    assert_eq!(
        edited.get_cv_4d(1).unwrap().3,
        original.get_cv_4d(1).unwrap().3
    );
    assert_eq!(edited.m_nurbsknot, original.m_nurbsknot);
    let a = original.get_cv(1).unwrap();
    let b = edited.get_cv(1).unwrap();
    assert!((b[0] - a[0] - 0.125).abs() < 1e-12 && (b[1] - a[1] - 0.25).abs() < 1e-12);
    command(&mut e, "Undo").unwrap();
    e.apply(Action::PickedTarget(Some(Target {
        owner: id,
        part: Part::Control(1),
    })))
    .unwrap();
    command(&mut e, "Scale").unwrap();
    command(&mut e, "w0,0,0 2").unwrap();
    let crate::chain::Source::Curve(edited, _) = &e.scene.paths()[0].prepared.source
    else {
        panic!("Curve")
    };
    assert_eq!(
        edited.get_cv_4d(1).unwrap().3,
        original.get_cv_4d(1).unwrap().3
    );
    assert!((edited.get_cv(1).unwrap()[0] - 2. * a[0]).abs() < 1e-12);
    command(&mut e, "Undo").unwrap();
    let crate::chain::Source::Curve(restored, _) = &e.scene.paths()[0].prepared.source
    else {
        panic!("Curve")
    };
    assert!(Rc::ptr_eq(restored, &original));
}
#[test]
fn cad_face_edits_rebuild_original_boundary_geometry_and_undo_to_the_shared_source() {
    let mut e = editor();
    command(&mut e, "Box w0,0,0 1 2 3").unwrap();
    let id = e.scene.objects()[0].id;
    let original = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    e.apply(Action::PickedTarget(Some(Target {
        owner: id,
        part: Part::Face(0),
    })))
    .unwrap();
    command(&mut e, "Move 0,0,0.125").unwrap();
    let next = &e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source;
    assert!(!Rc::ptr_eq(next, &original));
    assert_eq!(next.guid(), original.guid());
    assert!(next.is_valid() && next.is_closed(0));
    assert_eq!(
        e.scene.objects()[0].model.m,
        Xform::identity().m
    );
    command(&mut e, "Undo").unwrap();
    assert!(Rc::ptr_eq(
        &e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source,
        &original
    ));
    command(&mut e, "Redo").unwrap();
    assert!(!Rc::ptr_eq(
        &e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source,
        &original
    ));
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    let mut saved = crate::scene::Scene::empty();
    crate::scene_document::open(&mut saved, &bytes).unwrap();
    assert_eq!(
        saved.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto()
    );
}
#[test]
fn cancelling_a_transform_releases_temporary_owners_and_keeps_placement_and_history() {
    let mut e = editor();
    command(&mut e, "Box w0,0,0 1 1 1").unwrap();
    let id = e.scene.objects()[0].id;
    e.select_objects([id], false);
    e.apply(Action::BeginTransformation(Kind::Rotate))
        .unwrap();
    command(&mut e, "w0,0,0 w1,0,0").unwrap();
    e.apply(Action::ToolHover(Some([0., 1., 0.])))
        .unwrap();
    assert_ne!(
        e.display_scene().objects()[0].model.m,
        e.scene.objects()[0].model.m
    );
    command(&mut e, "Escape").unwrap();
    assert!(e.transform_preview.is_none());
    assert_eq!(
        e.scene.objects()[0].model.m,
        Xform::identity().m
    );
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
}
#[test]
fn copy_under_duplicate_named_nested_groups_preserves_local_save_placement_and_other_source_holders()
 {
    use prost::Message;
    let mut original = session_rust::Session::new("placed");
    original.add_group("parts");
    let group = original.add_group("parts");
    original.set_xform(
        &group.borrow().name,
        Xform::translation(10., 0., 0.),
    );
    let node = original.add_line(
        session_rust::Line::new(0., 0., 0., 1., 0., 0.),
        Some(&group),
    );
    let guid = node.borrow().name.clone();
    let source = original.to_proto();
    let bytes = source.encode_to_vec();
    let mut e = editor();
    crate::scene_document::import(
        &mut e.scene,
        &bytes,
        "placed",
        &Xform::translation(100., 0., 0.),
        0.,
    )
    .unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Copy 0,5,0").unwrap();
    assert_eq!(e.scene.lines()[1].model.m[12], 110.);
    assert_eq!(e.scene.lines()[1].model.m[13], 5.);
    let doc = &e.scene.forest.documents[0];
    let a = doc
        .nodes
        .iter()
        .find(|node| node.object == Some(id))
        .unwrap();
    let b = doc
        .nodes
        .iter()
        .find(|node| node.object == Some(e.scene.lines()[1].id))
        .unwrap();
    assert_eq!(a.parent, b.parent);
    assert!(Rc::ptr_eq(
        &e.scene.lines()[0].prepared.source,
        &e.scene.lines()[1].prepared.source
    ));
    assert_eq!(original.to_proto(), source);
    assert!(original.lookup.contains_key(&guid));
    let archive = crate::scene_archive::save(&e.scene).unwrap();
    let mut saved = crate::scene::Scene::empty();
    crate::scene_document::open(&mut saved, &archive).unwrap();
    assert_eq!(saved.lines().len(), 2);
    assert!(
        saved
            .lines()
            .iter()
            .any(|row| row.model.m[12] == 110. && row.model.m[13] == 5.)
    );
}
```

### 2. `src/lib.rs`

Register the source transformation component.

**Locate this existing block:**

```rust
pub mod source_part;

pub mod source_mesh_topology;
pub mod transform_commands;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_part;

pub mod transform_commands;

pub mod source_mesh_topology;
#[cfg(test)]
mod transform_tests;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77zza-describe-curve-gathering-answers"></a>35. Describe curve gathering answers

**What this code does:** Describe ordered curve questions and typed answers.

### 1. `src/gather_input.rs`

Type the shared curve-gathering declarations.

Create the file and type:

```rust
use crate::{drawing_coords as coords, primitive_options as options};
use session_rust::{Geometry, NurbsCurve, Point, Vector, Xform};
fn typed_number(word: &str) -> Option<f64> {
    word.parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}
/// One question a gathering command asks; Curves steps come first.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    Curves {
        prompt: &'static str,
        min: usize,
        max: usize,
    }, // curves clicked in order; reaching max or Enter moves on
    Point(&'static str), // a click, snap or typed x,y,z
    Number {
        prompt: &'static str,
        default: f64,
        low: f64,
        high: f64,
    }, // low < value <= high; Enter takes the default
    Distance(&'static str), // along the recipe's axis: a click, a signed number or a typed vector
}

/// An answer to a Point, Number or Distance step.
#[derive(Clone, Debug)]
pub enum Answer {
    Point(Point),   // a location
    Number(f64),    // a value, or a distance along the axis
    Vector(Vector), // a Distance given as a vector
}

/// What a command gathered, in the world.
#[derive(Clone)]
pub struct Input {
    pub curves: Vec<Vec<Picked>>, // per Curves step, in pick order
    pub answers: Vec<Answer>,     // per other step
    pub choice: &'static str,     // the chosen option, empty when none
    pub normal: Vector,           // the construction plane the camera faces
}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

### 2. `src/lib.rs`

Register these helpers for the later kernel surfacing recipes.

**Locate this existing block:**

```rust
pub mod transform_commands;

pub mod source_mesh_topology;
#[cfg(test)]
mod transform_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod transform_commands;

#[cfg(test)]
mod transform_tests;

pub mod gather_input;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-77zzb-read-curve-gathering-values"></a>36. Read curve gathering values

**What this code does:** Read finite numbers, absolute points and distance vectors through the existing coordinate reader.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

**Locate this existing block:**

```rust
}

/// An answer to a Point, Number or Distance step.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Step {
    /// What the step asks for.
    pub fn prompt(&self) -> &'static str {
        match self {
            Step::Curves { prompt, .. } | Step::Number { prompt, .. } => prompt,
            Step::Point(prompt) | Step::Distance(prompt) => prompt,
        }
    }

    /// True for a step answered by picking curves.
    pub fn curves(&self) -> bool {
        matches!(self, Step::Curves { .. })
    }

    /// A typed word as this step's answer.
    pub fn read(&self, word: &str) -> Result<Answer, String> {
        match self {
            Step::Number {
                prompt,
                low,
                high,
                ..
            } => match typed_number(word) {
                Some(value) if value > *low && value <= *high => {
                    Ok(Answer::Number(value))
                }
                Some(_) => Err(format!(
                    "The {prompt} must be above {low} and at most {high}"
                )),
                None => Err(format!("`{word}` is not a number")),
            },
            Step::Point(_) => match coords::absolute(word) {
                Some([x, y, z]) => Ok(Answer::Point(Point::new(x, y, z))),
                None => Err(format!("`{word}` is not a point x,y,z")),
            },
            Step::Distance(_) => {
                match (typed_number(word), coords::parse_world(word)) {
                    (Some(value), _) => Ok(Answer::Number(value)),
                    (None, Some((coords::Typed::Absolute { x, y, z }, world))) => {
                        let [x, y, z] = coords::offset([x, y, z.unwrap_or(0.0)], world);
                        Ok(Answer::Vector(Vector::new(x, y, z)))
                    }
                    _ => Err(format!(
                        "`{word}` is not a distance or a vector x,y,z"
                    )),
                }
            }
            Step::Curves { .. } => Err("Click the curves".into()),
        }
    }
}

/// An answer to a Point, Number or Distance step.
```

## <a id="checkpoint-77zzc-connect-gathering-recipes"></a>37. Connect gathering recipes

**What this code does:** Connect typed input to a recipe, its questions and kernel builder.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

**Locate this existing block:**

```rust
}

/// A picked curve in world coordinates.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Input {
    /// The Distance answer as a vector along `axis`.
    pub fn along(&self, index: usize, axis: &Vector) -> Option<Vector> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(axis * *value),
            Answer::Vector(vector) => Some(vector.clone()),
            Answer::Point(_) => None,
        }
    }

    /// The Point answer at `index`.
    pub fn point(&self, index: usize) -> Option<Point> {
        match self.answers.get(index)? {
            Answer::Point(p) => Some(p.clone()),
            _ => None,
        }
    }

    /// The Number answer at `index`.
    pub fn number(&self, index: usize) -> Option<f64> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(*value),
            _ => None,
        }
    }
}

/// What a build made and what to say about it.
pub struct Made {
    pub geometries: Vec<Geometry>, // added as one undo step
    pub message: String,           // e.g. `Lofted 3 curves into a NURBS surface`
}

/// A command that gathers curves and answers, then builds.
pub struct Recipe {
    pub name: &'static str,                             // shown name
    pub chips: &'static [(&'static str, &'static str)], // buttons: options, then Finish and Cancel
    pub steps: &'static [Step],                         // what it asks, in order
    pub axis: fn(&Input) -> (Point, Vector), // where a Distance step slides, unit direction
    pub build: fn(&Input) -> Result<Made, String>, // the objects from the answers
}

impl std::fmt::Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Recipe {
    /// The option labels, e.g. Open and Closed.
    pub fn choices(&self) -> impl Iterator<Item = &'static str> {
        options::choices(self.chips)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.chips, words)
    }

    /// The steps that are not curve picks.
    pub fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps
            .iter()
            .filter(|step| !step.curves())
    }
}

/// A picked curve in world coordinates.
```

## <a id="checkpoint-77zzd-parse-gathering-command-words"></a>38. Parse gathering command words

**What this code does:** Read options anywhere in a command and consume other answers in order.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

**Locate this existing block:**

```rust
}

/// A picked curve in world coordinates.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Read typed words: options anywhere, the last one winning, and answers in step order.
pub fn read(recipe: &Recipe, words: &[&str]) -> Result<(usize, Vec<Answer>), String> {
    let others: Vec<&Step> = recipe.others().collect();
    let mut choice = 0;
    let mut answers = Vec::new();
    let mut rest = words;

    while !rest.is_empty() {
        if let Some((index, count)) = recipe.option_in(rest) {
            choice = index;
            rest = &rest[count..];
            continue;
        }

        let Some(step) = others.get(answers.len()) else {
            let options: Vec<_> = recipe.choices().collect();
            let listed = if options.is_empty() {
                String::new()
            } else {
                format!("; its options are {}", options.join(", "))
            };
            return Err(format!(
                "`{}` is not a value of {}{listed}",
                rest[0], recipe.name
            ));
        };
        answers.push(step.read(rest[0])?);
        rest = &rest[1..];
    }

    // typed values take the defaults of the numbers left, e.g. a full turn
    if !answers.is_empty()
        && others[answers.len()..]
            .iter()
            .all(|step| matches!(step, Step::Number { .. }))
    {
        for step in &others[answers.len()..] {
            if let Step::Number { default, .. } = step {
                answers.push(Answer::Number(*default));
            }
        }
    }

    Ok((choice, answers))
}

/// A picked curve in world coordinates.
```

## <a id="checkpoint-77zze-track-ordered-curve-picks"></a>39. Track ordered curve picks

**What this code does:** Track preselected curve rows and skip answers already typed.

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers.

**Locate this existing block:**

```rust
}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Where a gathering command is: rows per Curves step, answers per other step.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Progress {
    pub rows: Vec<Vec<u32>>, // picked rows per Curves step, in pick order
    pub answered: usize,     // other steps answered, in order
    pub step: usize,         // the step waiting; steps.len() when done
}

impl Progress {
    /// Nothing picked; `answered` other steps already have their answers.
    pub fn new(steps: &[Step], answered: usize) -> Self {
        let mut progress = Self {
            rows: vec![
                Vec::new();
                steps
                    .iter()
                    .filter(|step| step.curves())
                    .count()
            ],
            answered,
            step: 0,
        };
        progress.skip(steps);
        progress
    }

    /// Selected rows answer the Curves steps in order, up to each step's max; stop at the first short one.
    pub fn preselect(&mut self, steps: &[Step], rows: &[u32]) {
        let mut rest = rows;

        while let Some(Step::Curves { min, max, .. }) = steps.get(self.step) {
            let taken = rest.len().min(*max);
            self.rows[self.step].extend_from_slice(&rest[..taken]);
            rest = &rest[taken..];

            if taken < *min {
                return;
            }

            self.step += 1;
            self.skip(steps);
        }
    }

    /// Step over other steps whose answers were typed already.
    fn skip(&mut self, steps: &[Step]) {
        while let Some(step) = steps.get(self.step)
            && !step.curves()
            && self.other(steps) < self.answered
        {
            self.step += 1;
        }
    }

    /// The index of the current step among the other steps.
    fn other(&self, steps: &[Step]) -> usize {
        steps[..self.step]
            .iter()
            .filter(|step| !step.curves())
            .count()
    }

    /// Every row picked so far, in step and pick order.
    pub fn picked(&self) -> Vec<u32> {
        self.rows.iter().flatten().copied().collect()
    }
}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}
```

## <a id="checkpoint-77zzf-advance-and-cancel-curve-picks"></a>40. Advance and cancel curve picks

**What this code does:** Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

**Locate this existing block:**

```rust
        self.rows.iter().flatten().copied().collect()
    }
}

/// A picked curve in world coordinates.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.rows.iter().flatten().copied().collect()
    }

    /// Pick `row` for the current Curves step; picking it again takes it back.
    pub fn pick(&mut self, steps: &[Step], row: u32) -> Result<(), String> {
        let Some(Step::Curves { max, .. }) = steps.get(self.step) else {
            return Err("Not picking curves now".into());
        };
        let rows = &mut self.rows[self.step];

        if let Some(at) = rows.iter().position(|r| *r == row) {
            rows.remove(at);
            return Ok(());
        }

        if self.rows.iter().flatten().any(|r| *r == row) {
            return Err("That curve is picked already".into());
        }

        self.rows[self.step].push(row);

        if self.rows[self.step].len() >= *max {
            self.step += 1;
            self.skip(steps);
        }

        Ok(())
    }

    /// Enter: a Curves step with enough picks or a Number step's default moves on.
    pub fn next(
        &mut self,
        steps: &[Step],
        answers: &mut Vec<Answer>,
    ) -> Result<(), String> {
        match steps.get(self.step) {
            Some(Step::Curves { min, .. }) if self.rows[self.step].len() < *min => Err(
                format!("Select at least {}", count(*min, "curve")),
            ),
            Some(Step::Curves { .. }) => {
                self.step += 1;
                self.skip(steps);
                Ok(())
            }
            Some(Step::Number { default, .. }) => {
                self.answer(steps, answers, Answer::Number(*default));
                Ok(())
            }
            Some(step) => Err(format!("Give the {}", step.prompt())),
            None => Ok(()),
        }
    }

    /// Answer the current step.
    pub fn answer(
        &mut self,
        steps: &[Step],
        answers: &mut Vec<Answer>,
        answer: Answer,
    ) {
        answers.truncate(self.answered);
        answers.push(answer);
        self.answered += 1;
        self.step += 1;
        self.skip(steps);
    }

    /// True once every step is answered.
    pub fn done(&self, steps: &[Step]) -> bool {
        self.step >= steps.len()
    }
}

/// A picked curve in world coordinates.
```

## <a id="checkpoint-77zzg-prepare-placed-kernel-curves"></a>41. Prepare placed kernel curves

**What this code does:** Convert lines, polylines and NURBS curves to placed kernel curves.

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers. Supply the original gathering-input validation fixtures separately.

**Locate this existing block:**

```rust
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// A line, polyline or NURBS curve moved into the world by `place`; None for anything else.
pub fn picked(geometry: &Geometry, place: &Xform) -> Option<Picked> {
    let corners = match geometry {
        Geometry::Line(line) => Some(vec![line.start(), line.end()]),
        Geometry::Polyline(polyline) => {
            let mut polyline = (**polyline).clone();
            polyline.remove_consecutive_duplicates(1e-12);
            Some(polyline.get_points())
        }
        Geometry::NurbsCurve(_) => None,
        _ => return None,
    };
    let corners: Option<Vec<Point>> = corners.map(|points| {
        points
            .iter()
            .map(|p| p.transformed(place))
            .collect()
    });
    let mut curve = match (&corners, geometry) {
        (Some(points), _) if points.len() >= 2 => NurbsCurve::create(false, 1, points),
        (None, Geometry::NurbsCurve(curve)) => {
            let mut curve = (**curve).clone();

            // loft and sweep cannot mix periodic and clamped knots
            if curve.is_periodic() {
                curve.clamp_end(2);
            }

            curve.transform(place);
            curve
        }
        _ => return None,
    };

    if !curve.is_valid() || !curve.set_domain(0.0, 1.0) {
        return None;
    }

    Some(Picked { curve, corners })
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

impl Picked {
    /// Points along the curve: its corners, else even samples.
    pub fn outline(&self) -> Vec<Point> {
        match &self.corners {
            Some(corners) => corners.clone(),
            None => self.curve.divide_by_count(SAMPLES, true).0,
        }
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
#[cfg(test)]
mod tests {
    use super::*;

    const LOFT: &[Step] = &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX,
    }];
    const SWEEP: &[Step] = &[
        Step::Curves {
            prompt: "select the rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
    ];
    const REVOLVE: &[Step] = &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ];

    /// Build nothing; tests read the recipe only.
    fn nothing(_: &Input) -> Result<Made, String> {
        Err("no build".into())
    }

    /// No axis.
    fn up(_: &Input) -> (Point, Vector) {
        (
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        )
    }

    static REVOLVING: Recipe = Recipe {
        name: "Revolve",
        chips: &[("Finish", ""), ("Cancel", "Escape")],
        steps: REVOLVE,
        axis: up,
        build: nothing,
    };
    static EXTRUDING: Recipe = Recipe {
        name: "Extrude",
        chips: &[
            ("Cap On", "Cap On"),
            ("Cap Off", "Cap Off"),
            ("Finish", ""),
            ("Cancel", "Escape"),
        ],
        steps: &[
            Step::Curves {
                prompt: "select curves",
                min: 1,
                max: usize::MAX,
            },
            Step::Distance("distance"),
        ],
        axis: up,
        build: nothing,
    };

    /// Clicks keep their order; a second click takes a row back.
    #[test]
    fn a_curves_step_keeps_click_order_and_toggles() {
        let mut progress = Progress::new(LOFT, 0);
        progress.pick(LOFT, 7).unwrap();
        progress.pick(LOFT, 3).unwrap();
        progress.pick(LOFT, 5).unwrap();
        assert_eq!(progress.rows[0], vec![7, 3, 5]);
        progress.pick(LOFT, 3).unwrap();
        assert_eq!(progress.rows[0], vec![7, 5]);
        assert!(!progress.done(LOFT));
    }

    /// Enter with too few curves is refused.
    #[test]
    fn enter_below_min_is_refused() {
        let mut progress = Progress::new(LOFT, 0);
        progress.pick(LOFT, 1).unwrap();
        let mut answers = Vec::new();
        assert_eq!(
            progress.next(LOFT, &mut answers),
            Err("Select at least 2 curves".into())
        );
        progress.pick(LOFT, 2).unwrap();
        progress.next(LOFT, &mut answers).unwrap();
        assert!(progress.done(LOFT));
    }

    /// A rail step of one curve moves on by itself; the rail cannot be a profile too.
    #[test]
    fn a_full_single_curve_step_advances_on_its_own() {
        let mut progress = Progress::new(SWEEP, 0);
        progress.pick(SWEEP, 4).unwrap();
        assert_eq!(progress.step, 1);
        assert!(progress.pick(SWEEP, 4).is_err());
        progress.pick(SWEEP, 9).unwrap();
        assert_eq!(progress.rows, vec![vec![4], vec![9]]);
    }

    /// Selected rows fill the curve steps in order and stop at the first short step.
    #[test]
    fn preselection_answers_curve_steps_in_order_and_stops_at_the_first_open_one() {
        let mut progress = Progress::new(SWEEP, 0);
        progress.preselect(SWEEP, &[3, 1, 2]);
        assert_eq!(progress.rows, vec![vec![3], vec![1, 2]]);
        assert!(progress.done(SWEEP));
        let mut short = Progress::new(LOFT, 0);
        short.preselect(LOFT, &[8]);
        assert_eq!(
            (short.step, short.rows[0].clone()),
            (0, vec![8])
        );
        let mut revolve = Progress::new(REVOLVE, 0);
        revolve.preselect(REVOLVE, &[6]);
        assert_eq!(revolve.step, 1, "axis start waits");
    }

    /// Typed values answer the steps in order; the angle takes its default.
    #[test]
    fn answers_are_appended_in_step_order() {
        let (_, answers) = read(&REVOLVING, &["0,0,0", "0,0,10", "90"]).unwrap();
        assert!(matches!(answers[2], Answer::Number(value) if value == 90.0));
        let (_, answers) = read(&REVOLVING, &["0,0,0", "0,0,10"]).unwrap();
        assert!(matches!(answers[2], Answer::Number(value) if value == 360.0));
        assert!(read(&REVOLVING, &["0,0,0", "0,0,10", "400"]).is_err());
        assert!(read(&REVOLVING, &["0,0,0", "0,0,10", "90", "5"]).is_err());
        let mut progress = Progress::new(REVOLVE, answers.len());
        progress.preselect(REVOLVE, &[2]);
        assert!(progress.done(REVOLVE));
    }

    /// Enter at a Number step keeps the default.
    #[test]
    fn a_number_step_keeps_the_default_on_enter() {
        let mut progress = Progress::new(REVOLVE, 0);
        let mut answers = Vec::new();
        progress.preselect(REVOLVE, &[1]);
        progress.answer(
            REVOLVE,
            &mut answers,
            Answer::Point(Point::new(0., 0., 0.)),
        );
        progress.answer(
            REVOLVE,
            &mut answers,
            Answer::Point(Point::new(0., 0., 1.)),
        );
        let mut again = progress.clone();
        again.next(REVOLVE, &mut answers).unwrap();
        assert!(again.done(REVOLVE));
        assert!(matches!(answers[2], Answer::Number(value) if value == 360.0));
    }

    /// Options take several words, the last one winning; unknown words are refused.
    #[test]
    fn options_are_words_and_the_last_wins() {
        assert_eq!(
            EXTRUDING.option_in(&["cap", "off", "5"]),
            Some((1, 2))
        );
        let (choice, answers) = read(
            &EXTRUDING,
            &["Cap", "Off", "Cap", "On", "10"],
        )
        .unwrap();
        assert_eq!((choice, answers.len()), (0, 1));
        let (_, answers) = read(&EXTRUDING, &["0,0,5"]).unwrap();
        assert!(matches!(&answers[0], Answer::Vector(v) if v[2] == 5.0));
        assert!(read(&EXTRUDING, &["sideways"]).is_err());
    }
}

pub fn no_axis(_input: &Input) -> (Point, Vector) {
    (
        Point::new(0., 0., 0.),
        Vector::new(0., 0., 1.),
    )
}
```

## <a id="checkpoint-77zzh-check-original-loops-and-surface-controls"></a>42. Check loops and surface controls

**What this code does:** Find a loop’s unit normal and scale-aware planarity tolerance.

### 1. `src/surfacing.rs`

Type loop geometry, planarity and finite surface validation.

Create the file and type:

```rust
use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point, Primitives, Vector};
use std::rc::Rc;

pub use crate::gather_input::SAMPLES;

pub use crate::gather_input::{Picked, count, picked};

/// The unit normal of a loop by Newell's method; None when it encloses no area.
pub fn loop_normal(points: &[Point]) -> Option<Vector> {
    let mut sum = [0.0; 3];

    for (index, a) in points.iter().enumerate() {
        let b = &points[(index + 1) % points.len()];
        sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let length = (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt();
    let size = diagonal(points);

    if length <= 1e-12 * size * size {
        return None;
    }

    Some(Vector::new(
        sum[0] / length,
        sum[1] / length,
        sum[2] / length,
    ))
}

/// The diagonal of the points' box, at least 1.
pub fn diagonal(points: &[Point]) -> f64 {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in points {
        for i in 0..3 {
            low[i] = low[i].min(p[i]);
            high[i] = high[i].max(p[i]);
        }
    }

    let size = (0..3)
        .map(|i| (high[i] - low[i]).powi(2))
        .sum::<f64>()
        .sqrt();
    size.max(1.0)
}

/// True when every point lies on the plane through the first with `normal`, within a millionth of the size.
pub fn planar(points: &[Point], normal: &Vector) -> bool {
    let Some(first) = points.first() else {
        return false;
    };
    let tolerance = 1e-6 * diagonal(points);
    points
        .iter()
        .all(|p| (p - first).dot(normal).abs() <= tolerance)
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!(
            "The {name} surface could not be built"
        ));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked
        .iter()
        .map(|picked| picked.curve.clone())
        .collect()
}
```

### 2. `src/lib.rs`

Compile the shared surfacing owner while its recipes are prepared.

**Locate this existing block:**

```rust
mod cloud_shader_contract;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cloud_shader_contract;
mod shader_check;

pub mod surfacing;
```

## <a id="checkpoint-77zzi-align-original-sections-and-loft-through-them"></a>43. Align sections and loft through them

**What this code does:** Align open curve directions and closed curve seams before asking the Session kernel for a loft.

### 1. `src/surfacing.rs`

Add section alignment and lofting before the existing result validator. Supply the five original surfacing validation fixtures.

**Locate this existing block:**

```rust
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!(
            "The {name} surface could not be built"
        ));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked
        .iter()
        .map(|picked| picked.curve.clone())
        .collect()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Sections run the same way with their seams lined up; all open or all closed.
pub fn align_sections(sections: &mut [NurbsCurve]) -> Result<(), String> {
    let closed = sections
        .first()
        .is_some_and(|curve| curve.is_closed());

    if sections
        .iter()
        .any(|curve| curve.is_closed() != closed)
    {
        return Err("Curves must be all open or all closed".into());
    }

    for index in 1..sections.len() {
        let (before, after) = sections.split_at_mut(index);
        let previous = &before[index - 1];
        let section = &mut after[0];

        if closed {
            let normal = loop_normal(&previous.divide_by_count(SAMPLES, false).0);
            let own = loop_normal(&section.divide_by_count(SAMPLES, false).0);

            if let (Some(normal), Some(own)) = (normal, own)
                && normal.dot(&own) < 0.0
            {
                section.reverse();
            }

            let t = section.closest_parameter(&previous.point_at_start());
            section.change_closed_curve_seam(t);
        } else {
            let (start, end) = (
                previous.point_at_start(),
                previous.point_at_end(),
            );
            let (a, b) = (
                section.point_at_start(),
                section.point_at_end(),
            );
            let same = a.distance(&start, None) + b.distance(&end, None);
            let flipped = a.distance(&end, None) + b.distance(&start, None);

            if flipped < same {
                section.reverse();
            }
        }

        section.set_domain(0.0, 1.0);
    }

    Ok(())
}

/// A loft through `sections` in order, cubic or `degree` in v; `closed` repeats the first.
pub fn loft(
    sections: &[NurbsCurve],
    closed: bool,
    degree: usize,
) -> Result<NurbsSurface, String> {
    if sections.len() < 2 {
        return Err("Select at least 2 curves".into());
    }

    if closed && sections.len() < 3 {
        return Err("A closed loft needs at least 3 curves".into());
    }

    let mut sections = sections.to_vec();
    align_sections(&mut sections)?;

    if closed {
        sections.push(sections[0].clone());
    }

    let surface = Primitives::create_loft(&sections, degree);

    if !surface.is_valid() {
        return Err("The curves could not be lofted".into());
    }

    Ok(surface)
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!(
            "The {name} surface could not be built"
        ));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked
        .iter()
        .map(|picked| picked.curve.clone())
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use session_rust::{Line, Polyline, Xform};

    /// A point from coordinates.
    pub fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A curve through control points, domain 0..1.
    pub fn curve(points: &[Point]) -> NurbsCurve {
        let mut curve = NurbsCurve::create(false, (points.len() - 1).min(3), points);
        curve.set_domain(0.0, 1.0);
        curve
    }

    /// A closed square of side `size` at height `z`, degree 1.
    pub fn square(size: f64, z: f64) -> NurbsCurve {
        curve(&[
            p(0.0, 0.0, z),
            p(size, 0.0, z),
            p(size, size, z),
            p(0.0, size, z),
            p(0.0, 0.0, z),
        ])
    }

    /// Lines, polylines and curves land where their placement puts them.
    #[test]
    fn world_curve_applies_the_row_placement() {
        let place = Xform::translation(10.0, 0.0, 5.0);
        let line = Geometry::Line(Rc::new(Line::from_points(
            &p(0., 0., 0.),
            &p(1., 0., 0.),
        )));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![
            p(0., 0., 0.),
            p(0., 0., 0.),
            p(1., 0., 0.),
            p(1., 1., 0.),
        ])));
        let nurbs = Geometry::NurbsCurve(Rc::new(curve(&[
            p(0., 0., 0.),
            p(1., 2., 0.),
            p(2., 0., 0.),
        ])));

        for geometry in [line, polyline, nurbs] {
            let picked = picked(&geometry, &place).unwrap();
            assert!(
                picked
                    .curve
                    .point_at_start()
                    .distance(&p(10., 0., 5.), None)
                    < 1e-12
            );
            assert_eq!(picked.curve.domain(), (0.0, 1.0));
        }

        let Some(Picked {
            corners: Some(corners),
            ..
        }) = picked(
            &Geometry::Polyline(Rc::new(Polyline::new(vec![
                p(0., 0., 0.),
                p(0., 0., 0.),
                p(1., 0., 0.),
            ]))),
            &place,
        )
        else {
            panic!()
        };
        assert_eq!(corners.len(), 2, "duplicates go");
        assert!(
            picked(
                &Geometry::Point(Rc::new(p(0., 0., 0.))),
                &place
            )
            .is_none()
        );
    }

    /// A periodic curve becomes clamped and stays closed.
    #[test]
    fn a_periodic_curve_is_clamped() {
        let ring = NurbsCurve::create(
            true,
            3,
            &[
                p(0., 0., 0.),
                p(10., 0., 0.),
                p(10., 10., 0.),
                p(0., 10., 0.),
            ],
        );
        assert!(ring.is_periodic());
        let picked = picked(
            &Geometry::NurbsCurve(Rc::new(ring)),
            &Xform::identity(),
        )
        .unwrap();
        assert!(!picked.curve.is_periodic());
        assert!(picked.curve.is_closed());
    }

    /// An open section pointing the other way is turned round.
    #[test]
    fn align_sections_reverses_an_opposite_open_section() {
        let mut sections = vec![
            curve(&[p(0., 0., 0.), p(10., 0., 0.)]),
            curve(&[p(10., 5., 0.), p(0., 5., 0.)]),
        ];
        align_sections(&mut sections).unwrap();
        assert!(
            sections[1]
                .point_at_start()
                .distance(&p(0., 5., 0.), None)
                < 1e-9
        );
        let mut mixed = vec![
            square(5.0, 0.0),
            curve(&[p(0., 0., 0.), p(1., 0., 0.)]),
        ];
        assert!(align_sections(&mut mixed).is_err());
    }

    /// Closed sections share their winding and start near each other.
    #[test]
    fn closed_sections_share_winding_and_seam() {
        let mut turned = square(10.0, 5.0);
        turned.reverse();
        let t = turned.closest_parameter(&p(10.0, 10.0, 5.0));
        turned.change_closed_curve_seam(t);
        let mut sections = vec![square(10.0, 0.0), turned];
        align_sections(&mut sections).unwrap();
        let a = loop_normal(&sections[0].divide_by_count(SAMPLES, false).0).unwrap();
        let b = loop_normal(&sections[1].divide_by_count(SAMPLES, false).0).unwrap();
        assert!(a.dot(&b) > 0.99);
        assert!(
            sections[1]
                .point_at_start()
                .distance(&p(0., 0., 5.), None)
                < 1e-6
        );
    }

    /// A square is planar with normal z; a bent loop is not planar.
    #[test]
    fn loop_normal_and_planar_on_a_square_and_a_bent_loop() {
        let square = [
            p(0., 0., 0.),
            p(4., 0., 0.),
            p(4., 3., 0.),
            p(0., 3., 0.),
        ];
        let normal = loop_normal(&square).unwrap();
        assert!((normal[2] - 1.0).abs() < 1e-12);
        assert!(planar(&square, &normal));
        let bent = [
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 10.),
            p(0., 30., 0.),
        ];
        assert!(!planar(&bent, &loop_normal(&bent).unwrap()));
        assert!(loop_normal(&[p(0., 0., 0.), p(1., 0., 0.), p(2., 0., 0.)]).is_none());
    }
}
```

## <a id="checkpoint-77zzj-find-extrusion-axes-and-cap-eligibility"></a>44. Find extrusion axes and cap eligibility

**What this code does:** Reuse the original loop normal and planarity helpers to choose an extrusion axis.

### 1. `src/extrude_geometry.rs`

Type the shared extrusion axis, orientations and cap eligibility.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{Picked, checked, count, loop_normal, planar};
use session_rust::{
    BRep, BRepOrientation, BRepRef, Geometry, NurbsCurve, NurbsSurface, Point,
    Polyline, Primitives, Vector, Xform,
};
use std::rc::Rc;

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;

/// The first curve's start and its plane's normal, facing like the construction plane; else that plane's normal.
fn axis(input: &Input) -> (Point, Vector) {
    let Some(first) = input
        .curves
        .first()
        .and_then(|curves| curves.first())
    else {
        return (
            Point::new(0.0, 0.0, 0.0),
            input.normal.clone(),
        );
    };
    let outline = first.outline();
    let normal = loop_normal(&outline)
        .filter(|normal| planar(&outline, normal))
        .map(|normal| {
            if normal.dot(&input.normal) < 0.0 {
                -normal
            } else {
                normal
            }
        })
        .unwrap_or_else(|| input.normal.clone());
    (first.curve.point_at_start(), normal)
}

/// Why a cap was left off, if it was.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Skip {
    None,
    Open,
    Bent,
    Flat,
}

/// The normal of a closed planar outline, signed toward `offset`; why not when it cannot be capped.
fn cap_normal(
    outline: &[Point],
    closed: bool,
    offset: &Vector,
) -> Result<Vector, Skip> {
    if !closed {
        return Err(Skip::Open);
    }

    let normal = loop_normal(outline).ok_or(Skip::Bent)?;

    if !planar(outline, &normal) {
        return Err(Skip::Bent);
    }

    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 {
        -normal
    } else {
        normal
    })
}
```

### 2. `src/lib.rs`

Compile the original extrusion helpers before their command registration.

**Locate this existing block:**

```rust
mod shader_check;

pub mod surfacing;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shader_check;

pub mod surfacing;

pub mod extrude_geometry;
```

## <a id="checkpoint-77zzk-map-cap-curves-to-surface-parameters"></a>45. Map cap curves to surface parameters

**What this code does:** Build a planar cap patch around the original control points.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}

/// A straight pcurve from (u0, v0) to (u1, v1).
fn uv_line(u0: f64, v0: f64, u1: f64, v1: f64) -> NurbsCurve {
    NurbsCurve::create(
        false,
        1,
        &[
            Point::new(u0, v0, 0.0),
            Point::new(u1, v1, 0.0),
        ],
    )
}

/// A flat patch in the plane through `origin` with outward `normal` that covers `points`.
fn cap_patch(points: &[Point], origin: &Point, normal: &Vector) -> NurbsSurface {
    let mut x = normal.cross(&Vector::new(0.0, 0.0, 1.0));

    if !x.normalize_self() {
        x = Vector::new(1.0, 0.0, 0.0);
    }

    let y = normal.cross(&x);
    let (mut low, mut high) = ([f64::MAX; 2], [f64::MIN; 2]);

    for p in points {
        let d = p - origin;
        let (u, v) = (d.dot(&x), d.dot(&y));
        low = [low[0].min(u), low[1].min(v)];
        high = [high[0].max(u), high[1].max(v)];
    }

    let pad = 0.01 * (high[0] - low[0]).max(high[1] - low[1]);
    let corner = |u: f64, v: f64| origin + &(&x * u + &y * v);
    let (u0, v0, u1, v1) = (
        low[0] - pad,
        low[1] - pad,
        high[0] + pad,
        high[1] + pad,
    );
    let points = [
        corner(u0, v0),
        corner(u0, v1),
        corner(u1, v0),
        corner(u1, v1),
    ];
    NurbsSurface::create(false, false, 1, 1, 2, 2, &points).unwrap_or_default()
}

/// The curve in the patch's parameters: the affine image of its control points.
fn on_patch(curve: &NurbsCurve, patch: &NurbsSurface) -> NurbsCurve {
    let p00 = patch.get_cv(0, 0).unwrap_or_default();
    let eu = &patch.get_cv(1, 0).unwrap_or_default() - &p00;
    let ev = &patch.get_cv(0, 1).unwrap_or_default() - &p00;
    let ((u0, u1), (v0, v1)) = (
        patch.domain(0).unwrap_or((0.0, 1.0)),
        patch.domain(1).unwrap_or((0.0, 1.0)),
    );
    let mut flat = curve.clone();

    for i in 0..curve.cv_count() {
        let (wx, wy, wz, w) = curve
            .get_cv_4d(i)
            .unwrap_or((0.0, 0.0, 0.0, 1.0));
        let d = &Point::new(wx / w, wy / w, wz / w) - &p00;
        let u = u0 + (u1 - u0) * d.dot(&eu) / eu.dot(&eu);
        let v = v0 + (v1 - v0) * d.dot(&ev) / ev.dot(&ev);
        flat.set_cv_4d(i, u * w, v * w, 0.0, w);
    }

    flat
}

/// Twice the signed area a closed pcurve encloses, counter-clockwise positive.
fn winding(curve: &NurbsCurve) -> f64 {
    let points = curve
        .divide_by_count((curve.cv_count() * 4).max(16), true)
        .0;
    points
        .windows(2)
        .map(|pair| pair[0][0] * pair[1][1] - pair[1][0] * pair[0][1])
        .sum()
}

/// A cap face bounded by one closed edge lying on `patch`.
fn cap_face(
    brep: &mut BRep,
    patch: &NurbsSurface,
    curve: &NurbsCurve,
    edge: usize,
) -> usize {
    let surface = brep.add_surface(patch);
    let flat = on_patch(curve, patch);
    let orientation = if winding(&flat) > 0.0 {
        F
    } else {
        R
    };
    let pcurve = brep.add_curve_2d(&flat);
    brep.add_pcurve(edge, surface, pcurve as i32, -1);
    let wire = brep.add_wire(&[BRepRef::new(edge as i32, orientation)]);
    brep.add_face(
        surface as i32,
        &[BRepRef::new(wire as i32, F)],
        0.0,
    )
}
```

## <a id="checkpoint-77zzl-build-closed-curved-extrusion-topology"></a>46. Build closed curved extrusion topology

**What this code does:** Build the side surface from an oriented original curve.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

**Locate this existing block:**

```rust
        0.0,
    )
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        0.0,
    )
}

/// A closed planar curve extruded into a solid: one side face with a seam and two flat caps.
fn capped(
    curve: &NurbsCurve,
    offset: &Vector,
    normal: &Vector,
) -> Result<BRep, String> {
    let mut bottom = curve.clone();

    // the side faces outward when the curve turns counter-clockwise about the offset
    let outline = bottom.divide_by_count(64, false).0;

    if loop_normal(&outline).is_some_and(|own| own.dot(normal) < 0.0) {
        bottom.reverse();
    }

    bottom.set_domain(0.0, 1.0);
    let top = bottom.transformed(&Xform::translation(
        offset[0], offset[1], offset[2],
    ));
    let body = Primitives::create_extrusion(&bottom, offset);

    if !body.is_valid() {
        return Err("The curve could not be extruded".into());
    }

    let mut brep = BRep::new();
    let low = bottom.point_at_start();
    let high = &low + offset;
    let start = brep.add_vertex(&low, 0.0) as i32;
    let end = brep.add_vertex(&high, 0.0) as i32;
    let curve_low = brep.add_curve_3d(&bottom) as i32;
    let edge_low = brep.add_edge(curve_low, start, start);
    let curve_high = brep.add_curve_3d(&top) as i32;
    let edge_high = brep.add_edge(curve_high, end, end);
    let curve_seam = brep.add_curve_3d(&NurbsCurve::create(
        false,
        1,
        &[low.clone(), high],
    )) as i32;
    let edge_seam = brep.add_edge(curve_seam, start, end);
    let surface = brep.add_surface(&body);
    let (u0, u1) = body.domain(0).unwrap_or((0.0, 1.0));
    let (v0, v1) = body.domain(1).unwrap_or((0.0, 1.0));
    let pcurve_low = brep.add_curve_2d(&uv_line(u0, v0, u1, v0));
    brep.add_pcurve(edge_low, surface, pcurve_low as i32, -1);
    let pcurve_high = brep.add_curve_2d(&uv_line(u0, v1, u1, v1));
    brep.add_pcurve(edge_high, surface, pcurve_high as i32, -1);
    let right = brep.add_curve_2d(&uv_line(u1, v0, u1, v1));
    let left = brep.add_curve_2d(&uv_line(u0, v0, u0, v1));
    brep.add_pcurve(edge_seam, surface, right as i32, left as i32);
    let wire = brep.add_wire(&[
        BRepRef::new(edge_low as i32, F),
        BRepRef::new(edge_seam as i32, F),
        BRepRef::new(edge_high as i32, R),
        BRepRef::new(edge_seam as i32, R),
    ]);
    let side = brep.add_face(
        surface as i32,
        &[BRepRef::new(wire as i32, F)],
        0.0,
    );
    let points: Vec<Point> = (0..bottom.cv_count())
        .filter_map(|i| bottom.get_cv(i))
        .collect();
    let lifted: Vec<Point> = points.iter().map(|p| p + offset).collect();
    let floor = cap_face(
        &mut brep,
        &cap_patch(&points, &low, &-normal),
        &bottom,
        edge_low as usize,
    );
    let roof = cap_face(
        &mut brep,
        &cap_patch(&lifted, &(&low + offset), normal),
        &top,
        edge_high as usize,
    );
    let shell = brep.add_shell(&[
        BRepRef::new(side as i32, F),
        BRepRef::new(floor as i32, F),
        BRepRef::new(roof as i32, F),
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}
```

## <a id="checkpoint-77zzm-extrude-original-corners-and-curves"></a>47. Extrude corners and curves

**What this code does:** Dispatch original corner chains and curved sections through the shared extrusion helpers.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

**Locate this existing block:**

```rust
}

/// A straight pcurve from (u0, v0) to (u1, v1).
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// One picked curve extruded by `offset`, capped when asked and possible.
fn extrude(
    picked: &Picked,
    offset: &Vector,
    cap: bool,
) -> Result<(Geometry, Skip), String> {
    let named = |mut brep: BRep| {
        brep.name = "extrusion".into();
        Geometry::BRep(Rc::new(brep))
    };

    match &picked.corners {
        Some(corners) if corners.len() > 2 => {
            let closed = corners.len() >= 4
                && corners[0].distance(&corners[corners.len() - 1], None) <= 1e-12;
            let lift = |p: &Point| p + offset;
            let mut faces: Vec<Polyline> = corners
                .windows(2)
                .map(|pair| {
                    Polyline::new(vec![
                        pair[0].clone(),
                        pair[1].clone(),
                        lift(&pair[1]),
                        lift(&pair[0]),
                        pair[0].clone(),
                    ])
                })
                .collect();
            let skip = match cap_normal(&corners[..corners.len() - 1], closed, offset) {
                Ok(_) if cap => {
                    faces.push(Polyline::new(corners.clone()));
                    faces.push(Polyline::new(
                        corners.iter().map(lift).collect(),
                    ));
                    Skip::None
                }
                Ok(_) => Skip::None,
                Err(skip) => skip,
            };
            let brep = BRep::from_polylines(&faces, &[]);

            if brep.face_count() == 0 {
                return Err("The polyline could not be extruded".into());
            }

            Ok((named(brep), skip))
        }
        _ => {
            let outline = picked.outline();
            let normal = cap_normal(&outline, picked.curve.is_closed(), offset);

            if let (true, Ok(normal)) = (cap && picked.corners.is_none(), &normal) {
                return Ok((
                    named(capped(&picked.curve, offset, normal)?),
                    Skip::None,
                ));
            }

            let surface = Primitives::create_extrusion(&picked.curve, offset);
            let skip = normal.err().unwrap_or(Skip::None);
            Ok((checked(surface, "extrusion")?, skip))
        }
    }
}

/// A straight pcurve from (u0, v0) to (u1, v1).
```

## <a id="checkpoint-77zzn-connect-the-original-extrusion-recipe"></a>48. Connect the extrusion recipe

**What this code does:** Complete the original recipe with curve selection, signed distance and cap choices.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers. Supply the four original extrusion validation fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}
```

Replace it with:

```rust
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}

/// Every picked curve extruded by the answered distance, as one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (_, direction) = axis(input);
    let offset = input
        .along(0, &direction)
        .ok_or("Extrude needs a distance")?;

    if !(offset.magnitude() > 1e-12 && (0..3).all(|i| offset[i].is_finite())) {
        return Err("The extrusion distance must not be zero".into());
    }

    let cap = input.choice != "Cap Off";
    let mut geometries = Vec::new();
    let mut skipped = [0; 3];

    for picked in &input.curves[0] {
        let (geometry, skip) = extrude(picked, &offset, cap)?;

        match skip {
            Skip::Open if cap => skipped[0] += 1,
            Skip::Bent if cap => skipped[1] += 1,
            Skip::Flat if cap => skipped[2] += 1,
            _ => {}
        }

        geometries.push(geometry);
    }

    let solids = geometries
        .iter()
        .filter(|geometry| matches!(geometry, Geometry::BRep(brep) if brep.is_solid()))
        .count();
    let mut message = format!(
        "Extruded {}: {}, {}",
        count(geometries.len(), "curve"),
        count(solids, "solid"),
        count(geometries.len() - solids, "open surface")
    );

    for (number, why) in skipped
        .iter()
        .zip(["open", "not planar", "along its plane"])
    {
        if *number > 0 {
            message.push_str(&format!(
                " · no cap on {} ({why})",
                count(*number, "curve")
            ));
        }
    }

    Ok(Made {
        geometries,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gather_input::Answer;
    use crate::surfacing::picked;
    use crate::surfacing::tests::p;
    use session_rust::Line;

    /// The input of `geometries` extruded by `answer`.
    fn input(geometries: &[Geometry], answer: Answer, choice: &'static str) -> Input {
        Input {
            curves: vec![
                geometries
                    .iter()
                    .map(|geometry| picked(geometry, &Xform::identity()).unwrap())
                    .collect(),
            ],
            answers: vec![answer],
            choice,
            normal: Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// A polyline geometry.
    fn polyline(points: &[Point]) -> Geometry {
        Geometry::Polyline(Rc::new(Polyline::new(points.to_vec())))
    }

    /// The one made geometry.
    fn one(input: &Input) -> Geometry {
        build(input).unwrap().geometries.remove(0)
    }

    /// A closed rectangle becomes a box; open or uncapped it stays a sheet.
    #[test]
    fn polylines_become_boxes_and_sheets() {
        let rectangle = polyline(&[
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 0.),
            p(0., 30., 0.),
            p(0., 0., 0.),
        ]);
        let Geometry::BRep(solid) = one(&input(
            &[rectangle.clone()],
            Answer::Number(25.0),
            "Cap On",
        )) else {
            panic!()
        };
        assert!(solid.is_solid());
        assert_eq!(solid.face_count(), 6);
        assert!((solid.volume().abs() - 40.0 * 30.0 * 25.0).abs() < 1e-6);
        let Geometry::BRep(sheet) = one(&input(
            &[rectangle],
            Answer::Number(25.0),
            "Cap Off",
        )) else {
            panic!()
        };
        assert_eq!(
            (sheet.face_count(), sheet.is_solid()),
            (4, false)
        );
        let open = polyline(&[
            p(0., 0., 0.),
            p(10., 0., 0.),
            p(10., 10., 0.),
            p(0., 10., 0.),
        ]);
        let Geometry::BRep(open) = one(&input(
            &[open],
            Answer::Number(5.0),
            "Cap On",
        )) else {
            panic!()
        };
        assert_eq!(
            (open.face_count(), open.is_solid()),
            (3, false)
        );
        let bent = polyline(&[
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 10.),
            p(0., 30., 0.),
            p(0., 0., 0.),
        ]);
        let made = build(&input(
            &[bent],
            Answer::Number(20.0),
            "Cap On",
        ))
        .unwrap();
        assert!(made.message.contains("not planar"));
        let Geometry::BRep(bent) = &made.geometries[0] else {
            panic!()
        };
        assert_eq!(
            (bent.face_count(), bent.is_solid()),
            (4, false)
        );
    }

    /// A closed circle-like curve becomes a solid cylinder in either winding.
    #[test]
    fn a_closed_curve_becomes_a_solid() {
        let ring = |turn: f64| {
            let points: Vec<Point> = (0..8)
                .map(|i| {
                    let angle = turn * std::f64::consts::TAU * i as f64 / 8.0;
                    p(10.0 * angle.cos(), 10.0 * angle.sin(), 0.0)
                })
                .collect();
            let curve = NurbsCurve::create(true, 3, &points);
            Geometry::NurbsCurve(Rc::new(curve))
        };
        let mut volume = None;

        for turn in [1.0, -1.0] {
            let Geometry::BRep(solid) = one(&input(
                &[ring(turn)],
                Answer::Number(30.0),
                "Cap On",
            )) else {
                panic!()
            };
            assert!(solid.is_solid());
            assert_eq!(solid.face_count(), 3);
            let v = solid.volume();
            assert!(v > 0.0, "{v}");
            let area = picked(&ring(turn), &Xform::identity())
                .unwrap()
                .outline();
            let area = 0.5
                * area
                    .windows(2)
                    .map(|w| w[0][0] * w[1][1] - w[1][0] * w[0][1])
                    .sum::<f64>()
                    .abs();
            assert!(
                (v - area * 30.0).abs() < 0.01 * area * 30.0,
                "{v} {}",
                area * 30.0
            );
            volume = Some(v);
        }

        assert!(volume.is_some());
        let Geometry::NurbsSurface(tube) = one(&input(
            &[ring(1.0)],
            Answer::Number(30.0),
            "Cap Off",
        )) else {
            panic!()
        };
        assert!(tube.is_closed(0));
    }

    /// A line gives a flat strip; a zero distance is refused.
    #[test]
    fn a_line_gives_a_strip() {
        let line = Geometry::Line(Rc::new(Line::from_points(
            &p(0., 0., 0.),
            &p(50., 0., 0.),
        )));
        let Geometry::NurbsSurface(strip) = one(&input(
            &[line.clone()],
            Answer::Vector(Vector::new(0., 0., 10.)),
            "Cap On",
        )) else {
            panic!()
        };
        assert_eq!((strip.degree(0), strip.degree(1)), (1, 1));
        assert!(
            build(&input(
                &[line],
                Answer::Number(0.0),
                "Cap On"
            ))
            .is_err()
        );
    }

    /// The default direction is the plane's normal, facing the construction plane.
    #[test]
    fn the_direction_follows_the_construction_plane() {
        let square = polyline(&[
            p(0., 0., 0.),
            p(0., 10., 0.),
            p(10., 10., 0.),
            p(10., 0., 0.),
            p(0., 0., 0.),
        ]);
        let (_, direction) = axis(&input(
            &[square],
            Answer::Number(1.0),
            "Cap On",
        ));
        assert!((direction[2] - 1.0).abs() < 1e-12);
        let line = Geometry::Line(Rc::new(Line::from_points(
            &p(0., 0., 0.),
            &p(50., 0., 0.),
        )));
        let (_, direction) = axis(&input(
            &[line],
            Answer::Number(1.0),
            "Cap On",
        ));
        assert!((direction[2] - 1.0).abs() < 1e-12);
    }
}
```

**Locate this block:**

```rust
use std::rc::Rc;

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;
```

Replace it with:

```rust
use std::rc::Rc;

pub static RECIPE: Recipe = Recipe {
    name: "Extrude",
    chips: &[
        ("Cap On", "Cap On"),
        ("Cap Off", "Cap Off"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[
        Step::Curves {
            prompt: "select curves to extrude",
            min: 1,
            max: usize::MAX,
        },
        Step::Distance("distance"),
    ],
    axis,
    build,
};

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;
```

## <a id="checkpoint-77zzo-revolve-original-profiles-around-an-axis"></a>49. Revolve profiles around an axis

**What this code does:** Turn original curves into exact kernel surfaces around a chosen axis.

### 1. `src/revolve_geometry.rs`

Type the original axis validation, exact revolution and complete recipe result.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Revolve",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// `profile` turned `degrees` right-handed about the axis from `start` to `end`.
pub fn revolve(
    profile: &NurbsCurve,
    start: &Point,
    end: &Point,
    degrees: f64,
) -> Result<NurbsSurface, String> {
    let axis = end - start;

    if axis.magnitude() <= 1e-12 {
        return Err("The axis start and end must differ".into());
    }

    if !(degrees > 0.0 && degrees <= 360.0) {
        return Err("The angle must be above 0 and at most 360".into());
    }

    Ok(Primitives::create_revolve(
        profile,
        start,
        &axis,
        degrees.to_radians(),
    ))
}

/// One surface per profile, all in one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (Some(start), Some(end), Some(degrees)) = (
        input.point(0),
        input.point(1),
        input.number(2),
    ) else {
        return Err("Nurbs Surface Revolve needs an axis and an angle".into());
    };
    let geometries = input.curves[0]
        .iter()
        .map(|picked| {
            checked(
                revolve(&picked.curve, &start, &end, degrees)?,
                "revolve",
            )
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Made {
        message: format!(
            "Revolved {} by {degrees}°",
            count(geometries.len(), "curve")
        ),
        geometries,
    })
}
```

### 2. `src/lib.rs`

Compile the prepared original revolution recipe.

**Locate this existing block:**

```rust
pub mod surfacing;

pub mod extrude_geometry;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod surfacing;

pub mod extrude_geometry;

pub mod revolve_geometry;
```

### 3. `src/revolve_geometry.rs`

Supply the unchanged original radius, seam and axis validation fixture.

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfacing::tests::{curve, p};

    /// A full turn closes; every sample keeps its radius; a quarter stays open.
    #[test]
    fn a_line_revolves_into_a_cylinder() {
        let line = curve(&[p(20., 0., 0.), p(20., 0., 50.)]);
        let surface = revolve(&line, &p(0., 0., 0.), &p(0., 0., 10.), 360.0).unwrap();
        assert!(surface.is_valid() && surface.is_closed(0));
        let (u, v) = (
            surface.domain(0).unwrap(),
            surface.domain(1).unwrap(),
        );

        for i in 0..=8 {
            for j in 0..=4 {
                let q = surface
                    .point_at(
                        u.0 + (u.1 - u.0) * i as f64 / 8.0,
                        v.0 + (v.1 - v.0) * j as f64 / 4.0,
                    )
                    .unwrap();
                assert!((q[0].hypot(q[1]) - 20.0).abs() < 1e-9);
            }
        }

        let quarter = revolve(&line, &p(0., 0., 0.), &p(0., 0., 10.), 90.0).unwrap();
        assert!(!quarter.is_closed(0));
        assert!(revolve(&line, &p(0., 0., 0.), &p(0., 0., 0.), 90.0).is_err());
        assert!(revolve(&line, &p(0., 0., 0.), &p(0., 0., 1.), 400.0).is_err());
    }
}
```

## Build & Run

![Recorded visual output for Lesson 15.1 (77zzo-revolve-original-profiles-around-an-axis)](../../screenshots/journey/77zzo-revolve-original-profiles-around-an-axis.png)

```sh
npm --prefix ../session_tests run course -- dependencies 15.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 15.1
npm --prefix ../session_tests run course -- save 15.1
```

Continue with [15.2 · Profile collection and one surfacing transaction](78-copy-current-curve-collection-and-surfacing-recipes.md).
