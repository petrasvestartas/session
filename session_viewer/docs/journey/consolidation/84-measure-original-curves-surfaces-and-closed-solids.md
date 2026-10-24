# 16.4 · Measurements, temporary marks and retained CAD pieces

**Goal:** Count derived pieces before allocating them. · **Steps:** 37 · **Source:** [app/command/verbs/project_to_plane.rs](../../../src/app/command/verbs/project_to_plane.rs) · [app/command/verbs/extend.rs](../../../src/app/command/verbs/extend.rs) · [app/splitting.rs](../../../src/app/splitting.rs) · [app/command/verbs/measure.rs](../../../src/app/command/verbs/measure.rs)

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

## <a id="checkpoint-84-measure-original-curves-surfaces-and-closed-solids"></a>1. Measure closed-solid volume

Reject open geometry, choose a world point near the mesh and reuse the shared swept-volume helper.

### 1. `src/measure_volume.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
use crate::cad_edges::edge_chains;
use crate::cad_orient::face_signs;
use crate::measure_math::compute_swept;
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Mesh, Point, Xform};
const OPEN: &str = "open";
/// World volume of a closed mesh or solid BRep; Err when open, None when the kind has no volume.
pub fn compute_volume(
    geometry: &Geometry,
    place: &Xform,
) -> Option<Result<f64, &'static str>> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
        Geometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
        Geometry::NurbsSurface(_) => Some(Err(OPEN)),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_volume(mesh, place)),
            ElementGeometry::BRep(brep) => Some(compute_brep_volume(brep, place)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// Volume of a closed mesh, from its drawn triangles.
fn compute_mesh_volume(mesh: &Mesh, place: &Xform) -> Result<f64, &'static str> {
    if !mesh.is_closed() {
        return Err(OPEN);
    }

    let origin = origin_of(mesh, place);
    Ok(compute_swept(mesh, place, &origin).abs() / 6.0)
}

/// Volume of a solid BRep as the viewer meshes it, each face turned outward first.
fn compute_brep_volume(brep: &BRep, place: &Xform) -> Result<f64, &'static str> {
    if !brep.is_solid() {
        return Err(OPEN);
    }

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    let chains = edge_chains(brep, &meshes);
    let signs = face_signs(brep, &meshes, &chains);
    let Some(origin) = meshes
        .iter()
        .find(|mesh| !mesh.vertex.is_empty())
    else {
        return Err(OPEN);
    };
    let origin = origin_of(origin, place);
    let swept: f64 = meshes
        .iter()
        .zip(&signs)
        .map(|(mesh, sign)| sign * compute_swept(mesh, place, &origin))
        .sum();
    Ok(swept.abs() / 6.0)
}

/// A world point on the mesh, so far-away models keep their precision.
fn origin_of(mesh: &Mesh, place: &Xform) -> Point {
    mesh.faces()
        .first()
        .and_then(|face| mesh.vertex_point(*mesh.face[face].first()?))
        .map_or(Point::new(0.0, 0.0, 0.0), |p| {
            p.transformed(place)
        })
}
```

### 2. `src/lib.rs`

Register this original measurement owner.

**Locate this existing block:**

```rust
pub mod measure_area;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_area;

pub mod measure_volume;
pub mod source_mesh_topology;
```

### 3. `src/measure_volume.rs`

Supply only the unchanged original validation fixtures.

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
    use session_rust::{Element, Line, NurbsSurface};
    use std::rc::Rc;

    fn near(value: f64, expected: f64, relative: f64) -> bool {
        (value - expected).abs() <= relative * expected.abs().max(1.0)
    }

    fn measured(
        geometry: Geometry,
        place: &Xform,
    ) -> Option<Result<f64, &'static str>> {
        compute_volume(&geometry, place)
    }

    #[test]
    fn closed_meshes_measure_and_open_ones_are_refused() {
        let identity = Xform::identity();
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let volume = measured(
            Geometry::Mesh(Rc::new(mesh.clone())),
            &identity,
        );
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let far =
            &Xform::translation(1e6, -2e6, 5e5) * &Xform::scale_xyz(2.0, 2.0, 2.0);
        let volume = measured(Geometry::Mesh(Rc::new(mesh)), &far);
        assert!(near(volume.unwrap().unwrap(), 12.8, 1e-9));
        let corners = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
        ];
        let sides = vec![
            vec![0, 3, 2, 1],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
        ];
        let lidless = Mesh::from_vertices_and_faces(corners, sides);
        let volume = measured(Geometry::Mesh(Rc::new(lidless)), &identity);
        assert_eq!(volume, Some(Err(OPEN)));
    }

    #[test]
    fn solid_breps_measure_whatever_their_face_flags() {
        let identity = Xform::identity();
        let brep = BRep::create_box(2.0, 2.0, 0.4);
        let volume = measured(
            Geometry::BRep(Rc::new(brep.clone())),
            &identity,
        );
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let mut flipped = brep.clone();
        let face = &mut flipped.m_shells[0].faces[0];
        face.orientation = session_rust::brep::brep_reverse(face.orientation);
        let volume = measured(Geometry::BRep(Rc::new(flipped)), &identity);
        assert!(near(volume.unwrap().unwrap(), 1.6, 1e-9));
        let mut shell = brep.clone();
        shell.m_solids.clear();
        let volume = measured(Geometry::BRep(Rc::new(shell)), &identity);
        assert_eq!(volume, Some(Err(OPEN)));
        let cylinder = Geometry::BRep(Rc::new(BRep::create_cylinder(150.0, 400.0)));
        let exact = std::f64::consts::PI * 150.0 * 150.0 * 400.0;
        assert!(near(
            measured(cylinder, &identity)
                .unwrap()
                .unwrap(),
            exact,
            5e-3
        ));
        let element = Geometry::Element(Rc::new(Element::from_brep(brep, "beam")));
        assert!(near(
            measured(element, &identity)
                .unwrap()
                .unwrap(),
            1.6,
            1e-9
        ));
        let corners = [
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let surface = NurbsSurface::create(false, false, 1, 1, 2, 2, &corners).unwrap();
        let surface = Geometry::NurbsSurface(Rc::new(surface));
        assert_eq!(measured(surface, &identity), Some(Err(OPEN)));
        let line = Geometry::Line(Rc::new(Line::new(
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        )));
        assert_eq!(measured(line, &identity), None);
    }
}
```

## <a id="checkpoint-84a-validate-an-original-box-and-derive-wire-only-display"></a>2. Validate an box and derive wire-only display

Validate an original box and derive wire-only display.

### 1. `src/scene.rs`

Retain an independent original geometry owner beside the existing mesh display.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            cad: None,
```

Replace it with:

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            original: None,
            cad: None,
```

**Locate this block:**

```rust
pub struct Object {
    pub cad: Option<Rc<crate::cad_display::Prepared>>,
    pub surface: Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
```

Replace it with:

```rust
pub struct Object {
    pub original: Option<session_rust::Geometry>,
    pub cad: Option<Rc<crate::cad_display::Prepared>>,
    pub surface: Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
```

### 2. `src/bounds_original.rs`

Validate original box frames and prepare a wire-only display.

Create the file and type:

```rust
use session_rust::{Mesh, OBB, Point, Xform};
use std::rc::Rc;
pub fn valid(source: &OBB) -> bool {
    let axes = [
        &source.x_axis,
        &source.y_axis,
        &source.z_axis,
    ];
    (0..3).all(|i| {
        source.center[i].is_finite()
            && source.half_size[i].is_finite()
            && source.half_size[i] >= 0.
    }) && axes
        .iter()
        .all(|a| (0..3).all(|i| a[i].is_finite()) && (a.magnitude() - 1.).abs() < 1e-9)
        && axes[0].dot(axes[1]).abs() < 1e-9
        && axes[0].dot(axes[2]).abs() < 1e-9
        && axes[1].dot(axes[2]).abs() < 1e-9
}
pub fn prepared(source: &OBB) -> Result<crate::prepared::PreparedMesh, &'static str> {
    if !valid(source) {
        return Err("Invalid original box frame");
    }
    let points = source.corners().to_vec();
    let mut mesh = Mesh::from_vertices_and_faces(
        points,
        vec![
            vec![0, 3, 2, 1],
            vec![4, 5, 6, 7],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
        ],
    );
    mesh.set_guid(source.guid().to_owned());
    mesh.name = source.name.clone();
    mesh.is_visible = source.is_visible;
    mesh.is_locked = source.is_locked;
    let points = source.corners();
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    let endpoints = edges
        .iter()
        .map(|&(a, b)| [&points[a], &points[b]].map(|p| [p[0], p[1], p[2]]))
        .collect();
    let mut display = crate::mesh::Mesh::new(
        points
            .iter()
            .map(|p| {
                [
                    p[0] as f32,
                    p[1] as f32,
                    p[2] as f32,
                    0.,
                    0.,
                    0.,
                ]
            })
            .collect(),
        Vec::new(),
    )?;
    display.parts = crate::mesh_parts::Parts {
        faces: Vec::new(),
        triangles: Vec::new(),
        edges,
        endpoints,
    };
    Ok(crate::prepared::PreparedMesh {
        geometry: Rc::new(mesh),
        display,
        source: None,
        model: Xform::identity(),
    })
}
```

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust
pub mod measure_volume;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_volume;

pub mod bounds_original;
pub mod source_mesh_topology;
```

## <a id="checkpoint-84b-adopt-original-boxes-and-retain-precise-placed-bounds"></a>3. Adopt boxes and retain precise placed bounds

Adopt original boxes and retain precise placed bounds.

### 1. `src/bounds_original.rs`

Adopt and place original boxes without substituting their display proxy.

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}
pub fn placed(source: &OBB, model: &Xform) -> crate::bounds::Bounds {
    let points = source
        .corners()
        .map(|p| p.transformed(model));
    let p = &points[0];
    let mut result = crate::bounds::Bounds::point([p[0], p[1], p[2]]);
    for p in points {
        result.include([p[0], p[1], p[2]]);
    }
    result
}
pub fn insert(
    scene: &mut crate::scene::Scene,
    source: Rc<OBB>,
) -> Result<crate::scene::ObjectId, &'static str> {
    let id = scene.insert(prepared(&source)?)?;
    scene
        .objects_mut()
        .iter_mut()
        .find(|r| r.id == id)
        .unwrap()
        .original = Some(session_rust::Geometry::OBB(source));
    Ok(id)
}
pub fn box_geometry(bounds: &crate::bounds::Bounds) -> OBB {
    let c = bounds.centre();
    OBB::new(
        Point::new(c[0], c[1], c[2]),
        session_rust::Vector::x_axis(),
        session_rust::Vector::y_axis(),
        session_rust::Vector::z_axis(),
        session_rust::Vector::new(
            (bounds.max[0] - bounds.min[0]) * 0.5,
            (bounds.max[1] - bounds.min[1]) * 0.5,
            (bounds.max[2] - bounds.min[2]) * 0.5,
        ),
    )
}
```

### 2. `src/scene.rs`

Use original box bounds and preserve the original family during source replacement.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if matches!(target.part, Part::CloudPoint(_)) {
            return Err("Cloud point editing connects with source transforms");
        }
```

Replace it with:

```rust
        if matches!(target.part, Part::CloudPoint(_)) {
            return Err("Cloud point editing connects with source transforms");
        }
        if self
            .objects
            .iter()
            .any(|row| row.id == target.owner && row.original.is_some())
        {
            return Err(
                "Part editing for this original family connects in the complete command inventory",
            );
        }
```

**Locate this block:**

```rust
            .ok_or("Source placement is missing")?;
        geometry.set_guid(original.guid());
        match geometry {
            Geometry::Point(source) => {
                let prepared = crate::marker::PreparedPoint::new(source)?;
                let p = model.transform_point(&prepared.source);
```

Replace it with:

```rust
            .ok_or("Source placement is missing")?;
        geometry.set_guid(original.guid());
        match geometry {
            Geometry::OBB(source) => {
                let prepared = crate::bounds_original::prepared(&source)?;
                let bounds = crate::bounds_original::placed(&source, &model);
                if bounds
                    .min
                    .into_iter()
                    .chain(bounds.max)
                    .any(|v| !(v as f32).is_finite())
                {
                    return Err("Edited box exceeds display range");
                }
                let row = self
                    .objects
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or("Expected a box source")?;
                row.mesh = Rc::new(prepared.display);
                row.original = Some(Geometry::OBB(source));
                row.editable = crate::edit_source::EditSource::Loaded {
                    geometry: prepared.geometry,
                    source: None,
                };
            }
            Geometry::Point(source) => {
                let prepared = crate::marker::PreparedPoint::new(source)?;
                let p = model.transform_point(&prepared.source);
```

**Locate this block:**

```rust
            .find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
```

Replace it with:

```rust
            .find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::OBB(source)) = &object.original {
            return Some(crate::bounds_original::placed(
                source,
                &object.model,
            ));
        }
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
```

### 3. `src/source_edit.rs`

Return the original box allocation before considering display geometry.

**Locate this existing block:**

```rust
    if let Some(row) = scene.objects().iter().find(|r| r.id == id) {
        if let Some(cad) = &row.cad {
            return Some(Geometry::BRep(cad.source.clone()));
        }
        if let Some(s) = &row.surface {
            return Some(Geometry::NurbsSurface(s.source.clone()));
        }
        return row
            .geometry()
            .map(|g| Geometry::Mesh(g.clone()));
    }
    if let Some(row) = scene.points().iter().find(|r| r.id == id) {
        return Some(Geometry::Point(row.prepared.source.clone()));
    }
    crate::gather_draft::Draft::source(scene, id)
}
pub fn same(a: &Geometry, b: &Geometry) -> bool {
    match (a, b) {
        (Geometry::Line(a), Geometry::Line(b)) => Rc::ptr_eq(a, b),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if let Some(row) = scene.objects().iter().find(|r| r.id == id) {
        if let Some(original) = &row.original {
            return Some(original.clone());
        }
        if let Some(cad) = &row.cad {
            return Some(Geometry::BRep(cad.source.clone()));
        }
        if let Some(s) = &row.surface {
            return Some(Geometry::NurbsSurface(s.source.clone()));
        }
        return row
            .geometry()
            .map(|g| Geometry::Mesh(g.clone()));
    }
    if let Some(row) = scene.points().iter().find(|r| r.id == id) {
        return Some(Geometry::Point(row.prepared.source.clone()));
    }
    crate::gather_draft::Draft::source(scene, id)
}
pub fn same(a: &Geometry, b: &Geometry) -> bool {
    match (a, b) {
        (Geometry::OBB(a), Geometry::OBB(b)) => Rc::ptr_eq(a, b),
        (Geometry::Line(a), Geometry::Line(b)) => Rc::ptr_eq(a, b),
```

### 4. `src/gpu_geometry.rs`

Skip empty triangle draws before binding a wire-only box index buffer.

**Locate this existing block:**

```rust
    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_vertex_buffer(0, self.vertices.slice(..));
```

### 5. `src/highlight.rs`

Show whole selection on wire-only original edges without creating selected proxy faces.

**Locate this existing block:**

```rust
                    ..r.stroke()
                }),
        )?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    ..r.stroke()
                })
                .chain(
                    scene
                        .objects()
                        .iter()
                        .filter(|r| {
                            ids.contains(&r.id)
                                && scene.visible(r.id)
                                && r.mesh.indices().is_empty()
                        })
                        .flat_map(|r| {
                            r.mesh
                                .parts
                                .endpoints
                                .iter()
                                .map(move |pair| {
                                    let ends = pair.map(|p| {
                                        r.model
                                            .transform_point(&session_rust::Point::new(
                                                p[0], p[1], p[2],
                                            ))
                                            .to_f32()
                                    });
                                    crate::stroke::Stroke {
                                        start: ends[0],
                                        end: ends[1],
                                        colour: yellow,
                                        width: 3.,
                                    }
                                })
                        }),
                ),
        )?;
```

## <a id="checkpoint-84c-save-and-reopen-the-original-box-frame"></a>4. Save and reopen the box frame

Save and reopen the original box frame.

### 1. `src/geometry_snapshot.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
    {
        if let Some(cad) = row.cad.as_ref() {
            push!(breps, row.id, cad.source.to_proto());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    {
        if let Some(session_rust::Geometry::OBB(source)) = &row.original {
            push!(bboxes, row.id, source.to_proto());
        } else if let Some(cad) = row.cad.as_ref() {
            push!(breps, row.id, cad.source.to_proto());
```

### 2. `src/document_source.rs`

Retain validated original OBB records through mixed document persistence.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.planes.is_empty()
        || !objects.bboxes.is_empty()
        || !objects.elements.is_empty()
```

Replace it with:

```rust
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.planes.is_empty()
        || !objects.elements.is_empty()
```

**Locate this block:**

```rust
    }
    identity!(meshes);
    identity!(points);
    identity!(lines);
```

Replace it with:

```rust
    }
    identity!(bboxes);
    identity!(meshes);
    identity!(points);
    identity!(lines);
```

### 3. `src/document_geometry.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
    }
    for source in &loaded.document.objects.points {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    for source in &loaded.document.objects.bboxes {
        add!(
            source,
            crate::bounds_original::insert(scene, source.clone())
        );
    }
    for source in &loaded.document.objects.points {
```

### 4. `src/archive_identity.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
    }
    remap!(meshes);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    remap!(bboxes);
    remap!(meshes);
```

### 5. `src/source_validate.rs`

Retain validated original OBB records through mixed document persistence.

**Locate this existing block:**

```rust
    if let Some(objects) = &source.objects {
        let count = objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.points {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if let Some(objects) = &source.objects {
        let count = objects.bboxes.len()
            + objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.bboxes {
            if !crate::bounds_original::valid(
                &session_rust::OBB::from_proto(item.clone())
                    .map_err(|e| e.to_string())?,
            ) {
                return Err("Invalid original box frame".into());
            }
        }
        for item in &objects.points {
```

## <a id="checkpoint-84d-own-temporary-measurement-labels-without-scene-identities"></a>5. Own temporary measurement labels without scene identities

### 1. `src/measure_mark.rs`

Own temporary points, unit suffixes and collision-free labels.

Create the file and type:

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
pub struct Mark {
    pub points: Vec<[f64; 3]>,
    pub label: String,
}
impl Mark {
    pub fn labels(&self, existing: &[TextLabel]) -> Vec<TextLabel> {
        if self.points.is_empty() {
            return Vec::new();
        }
        let mut used = existing
            .iter()
            .map(|l| l.id)
            .collect::<std::collections::HashSet<_>>();
        let mut next = u32::MAX;
        let mut label =
            |text: String, font_size: f32, placement: TextPlacement, color: [u8; 4]| {
                while used.contains(&next) {
                    next = next
                        .checked_sub(1)
                        .expect("Bounded labels leave free identities")
                }
                let id = next;
                used.insert(id);
                TextLabel {
                    id,
                    object: None,
                    text,
                    font_size,
                    line_height: font_size * 1.25,
                    color,
                    placement,
                    clip: None,
                }
            };
        let c = std::array::from_fn(|i| {
            self.points.iter().map(|p| p[i]).sum::<f64>() / self.points.len() as f64
        });
        let mut out = vec![label(
            self.label.clone(),
            14.,
            TextPlacement::Nameplate {
                world: c,
                padding: [6., 4.],
                rounded: true,
            },
            [255; 4],
        )];
        if self.points.len() > 1 {
            for &world in &self.points {
                out.push(label(
                    "■".into(),
                    8.,
                    TextPlacement::Anchor {
                        world,
                        offset: [-4., -5.],
                    },
                    [20, 20, 20, 255],
                ));
            }
        }
        out
    }
}
```

### 2. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust
pub mod bounds_original;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod bounds_original;

pub mod measure_mark;
pub mod source_mesh_topology;
```

## <a id="checkpoint-84e-report-distance-and-world-bounding-lengths"></a>6. Report distance and world bounding lengths

### 1. `src/measure_mark.rs`

Resolve world selected bounds and distance without changing source geometry.

**Locate this existing block:**

```rust
        out
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        out
    }
}
pub fn selected_bounds(
    scene: &Scene,
    ids: &[ObjectId],
) -> Result<crate::bounds::Bounds, String> {
    let mut out: Option<crate::bounds::Bounds> = None;
    for id in ids {
        let Some(bounds) = scene.selected_bounds(*id) else {
            continue;
        };
        if let Some(out) = &mut out {
            out.include(bounds.min);
            out.include(bounds.max);
        } else {
            out = Some(bounds);
        }
    }
    out.ok_or_else(|| "Select an object with a bounding box".into())
}
pub fn distance(a: [f64; 3], b: [f64; 3], unit: Unit) -> (Mark, String) {
    let (a, b) = (
        Point::new(a[0], a[1], a[2]),
        Point::new(b[0], b[1], b[2]),
    );
    let delta = &b - &a;
    let text = crate::measure_math::to_text;
    let value = format!(
        "{} {}",
        text(delta.magnitude()),
        unit.suffix(1)
    );
    let message = format!(
        "Distance {value} · dx {} · dy {} · dz {}",
        text(delta[0]),
        text(delta[1]),
        text(delta[2])
    );
    (
        Mark {
            points: vec![[a[0], a[1], a[2]], [b[0], b[1], b[2]]],
            label: value,
        },
        message,
    )
}
#[derive(Clone, Copy)]
pub enum Kind {
    Length,
    Area,
    Volume,
    BoundingLengths,
}
```

## <a id="checkpoint-84f-collect-only-supported-finite-original-measurements"></a>7. Collect only supported finite measurements

Collect only supported finite original measurements.

### 1. `src/measure_mark.rs`

Collect original length, area and volume while keeping refusals explicit.

**Locate this existing block:**

```rust
    BoundingLengths,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    BoundingLengths,
}
pub struct Total {
    total: f64,
    count: usize,
    skipped: usize,
    open: Vec<String>,
}
impl Total {
    pub fn collect(
        scene: &Scene,
        ids: &[ObjectId],
        face: Option<(ObjectId, usize)>,
        kind: Kind,
    ) -> Result<Self, String> {
        let mut result = Self {
            total: 0.,
            count: 0,
            skipped: 0,
            open: Vec::new(),
        };
        for id in ids {
            let Some(source) = crate::source_edit::original(scene, *id) else {
                if scene
                    .objects()
                    .iter()
                    .any(|r| r.id == *id && r.geometry().is_none())
                {
                    return Err("Reload editable originals before measuring".into());
                }
                result.skipped += 1;
                continue;
            };
            let model = scene
                .placement(*id)
                .ok_or("Selected placement is missing")?;
            let value = match kind {
                Kind::Length => crate::measure_length::compute_length(&source, &model),
                Kind::Area => crate::measure_area::compute_area(
                    &source,
                    &model,
                    face.filter(|(owner, _)| owner == id)
                        .map(|(_, face)| face),
                ),
                Kind::Volume => {
                    match crate::measure_volume::compute_volume(&source, &model) {
                        Some(Ok(value)) => Some(value),
                        Some(Err(_)) => {
                            result.open.push(source.name().to_owned());
                            continue;
                        }
                        None => None,
                    }
                }
                Kind::BoundingLengths => unreachable!(),
            };
            if let Some(value) = value {
                if !value.is_finite() {
                    return Err("Measurement exceeds numerical range".into());
                }
                result.total += value;
                result.count += 1;
            } else {
                result.skipped += 1;
            }
        }
        Ok(result)
    }
}
```

## <a id="checkpoint-84g-format-totals-refusals-and-three-axis-box-extents"></a>8. Format totals refusals and three-axis box extents

### 1. `src/measure_mark.rs`

Report current formatted totals and world bounding lengths with bounded errors.

**Locate this existing block:**

```rust
        Ok(result)
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Ok(result)
    }
    fn open_names(&self) -> String {
        match self.open.len() {
            0..=3 => self.open.join(", "),
            count => format!(
                "{} and {} more",
                self.open[..3].join(", "),
                count - 3
            ),
        }
    }
    pub fn answer(
        &self,
        kind: Kind,
        unit: Unit,
        centre: [f64; 3],
    ) -> Result<(Mark, String), String> {
        let text = crate::measure_math::to_text;
        let (name, power, one) = match kind {
            Kind::Length => ("Length", 1, "curve"),
            Kind::Area => ("Area", 2, "object"),
            Kind::Volume => ("Volume", 3, "object"),
            Kind::BoundingLengths => unreachable!(),
        };
        if self.count == 0 {
            return Err(if self.open.is_empty() {
                format!("{name}: no supported original is selected")
            } else {
                format!(
                    "Volume needs closed objects; open or inconsistently wound: {}",
                    self.open_names()
                )
            });
        }
        let value = format!("{} {}", text(self.total), unit.suffix(power));
        let refused = if self.open.is_empty() {
            String::new()
        } else {
            format!(
                " · open, not measured: {}",
                self.open_names()
            )
        };
        let message = format!(
            "{name} {value} · {}{refused}{}",
            crate::measure_math::plural(self.count, one),
            crate::measure_math::skipped_text(self.skipped)
        );
        Ok((
            Mark {
                points: vec![centre],
                label: value,
            },
            message,
        ))
    }
}
pub fn selection(
    scene: &Scene,
    ids: &[ObjectId],
    face: Option<(ObjectId, usize)>,
    kind: Kind,
    unit: Unit,
) -> Result<(Mark, String), String> {
    for id in ids {
        if scene
            .objects()
            .iter()
            .any(|r| r.id == *id && r.geometry().is_none())
        {
            return Err("Reload editable originals before measuring".into());
        }
    }
    let bounds = selected_bounds(scene, ids)?;
    let text = crate::measure_math::to_text;
    if matches!(kind, Kind::BoundingLengths) {
        let d = std::array::from_fn::<_, 3, _>(|i| bounds.max[i] - bounds.min[i]);
        let value = format!(
            "X {} · Y {} · Z {} {}",
            text(d[0]),
            text(d[1]),
            text(d[2]),
            unit.suffix(1)
        );
        return Ok((
            Mark {
                points: vec![bounds.centre()],
                label: value.clone(),
            },
            format!("Length BoundingBox · {value}"),
        ));
    }
    Total::collect(scene, ids, face, kind)?.answer(kind, unit, bounds.centre())
}
```

## <a id="checkpoint-84h-keep-measurements-outside-geometry-history"></a>9. Keep measurements outside geometry history

### 1. `src/editor.rs`

Own temporary marks and connect selected-original numerical results and box creation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .map_err(|message| self.primitive_failure(message))?;
        self.tool_status = next.prompt();
        self.split = Some(next);
        Ok(Change::View)
    }
```

Replace it with:

```rust
            .map_err(|message| self.primitive_failure(message))?;
        self.tool_status = next.prompt();
        self.split = Some(next);
        Ok(Change::View)
    }
    fn measured_ids(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() {
            self.selected
                .into_iter()
                .filter(|id| self.scene.contains(*id))
                .collect()
        } else {
            self.selected_objects()
        }
    }
    fn measure_selection(
        &mut self,
        kind: crate::measure_mark::Kind,
    ) -> Result<Change, &'static str> {
        let face = match (self.selected, self.selected_part) {
            (Some(id), Some(crate::pick_sources::Part::Face(face))) => Some((id, face)),
            _ => None,
        };
        let (mark, message) = crate::measure_mark::selection(
            &self.scene,
            &self.measured_ids(),
            face,
            kind,
            self.unit,
        )
        .map_err(|message| self.primitive_failure(message))?;
        self.mark = Some(mark);
        self.tool_status = message;
        Ok(Change::View)
    }
    fn distance_mark(&mut self, points: [[f64; 3]; 2]) -> Result<Change, &'static str> {
        let (mark, message) =
            crate::measure_mark::distance(points[0], points[1], self.unit);
        self.mark = Some(mark);
        self.tool_status = message;
        self.draft = None;
        Ok(Change::View)
    }
    fn bounding_box(&mut self) -> Result<Change, &'static str> {
        let ids = self.measured_ids();
        for id in &ids {
            if self
                .scene
                .objects()
                .iter()
                .any(|r| r.id == *id && r.geometry().is_none())
            {
                return Err("Reload original sources before BoundingBox");
            }
        }
        let bounds = crate::measure_mark::selected_bounds(&self.scene, &ids)
            .map_err(|message| self.primitive_failure(message))?;
        let mut original = crate::bounds_original::box_geometry(&bounds);
        original.name = "bounding box".into();
        let mut id = None;
        self.history
            .try_edit(&mut self.scene, |scene| {
                id = Some(crate::bounds_original::insert(
                    scene,
                    std::rc::Rc::new(original),
                )?);
                Ok(())
            })?;
        self.select_objects(id, false);
        self.tool_status = "Created a world axis aligned bounding box".into();
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            split: None,
            canvas_drag: None,
            gizmo_edit: None,
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            mark: None,
            unit: crate::measure_mark::Unit::Millimeters,
            split: None,
            canvas_drag: None,
            gizmo_edit: None,
```

**Locate this block:**

```rust
pub struct Editor {
    pub split: Option<crate::split_draft::Draft>,
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
```

Replace it with:

```rust
pub struct Editor {
    pub mark: Option<crate::measure_mark::Mark>,
    pub unit: crate::measure_mark::Unit,
    pub split: Option<crate::split_draft::Draft>,
    pub canvas_drag: Option<crate::canvas_drag::Drag>,
    pub gizmo_edit: Option<crate::gizmo_edit::Edit>,
```

## <a id="checkpoint-84i-accept-two-distance-points-through-the-drawing-tool"></a>10. Accept two distance points through the drawing tool

### 1. `src/tool.rs`

Add a two-point measurement result to the existing drawing tool.

**Locate this existing block:**

```rust
pub enum Built {
    Point(PreparedPoint),
    Line(PreparedLine),
    Move([f64; 3]),
    Polyline(crate::chain::PreparedChain),
    Geometry(session_rust::Geometry),
}

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(
        &self,
        points: &[[f64; 3]],
        _plane: crate::cplane::CPlane,
    ) -> Result<Built, &'static str> {
        self.build(points)
    }
    fn outline(
        &self,
        points: &[[f64; 3]],
        _plane: crate::cplane::CPlane,
    ) -> Vec<[f64; 3]> {
        points.to_vec()
    }
    fn sides(&self) -> Option<usize> {
        None
    }
    fn set_sides(&mut self, _sides: usize) -> Result<(), &'static str> {
        Err("Sides works while drawing a polygon")
    }
    fn buttons(&self) -> Vec<(String, String)> {
        vec![("Cancel".into(), "Escape".into())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Project,
    Extend,
    Gather(&'static crate::gather_input::Recipe),
    Point,
    Line,
    Move,
    Polyline,
    Curve,
    Rectangle,
    Polygon { sides: usize },
    Primitive(&'static crate::primitive_shape::Shape),
    Transform(crate::transform_draft::Kind),
}

impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Project => "Project To Plane",
            Self::Extend => "Extend",
            Self::Gather(recipe) => recipe.name,
            Self::Point => "Point",
            Self::Line => "Line",
            Self::Move => "Move",
            Self::Polyline => "Polyline",
            Self::Curve => "Curve",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
            Self::Transform(kind) => kind.name(),
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Project | Self::Extend => 0,
            Self::Gather(_) => 0,
            Self::Point => 1,
            Self::Line | Self::Move => 2,
            Self::Polyline | Self::Curve => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) | Self::Transform(_) => 0,
        }
    }
    fn sides(&self) -> Option<usize> {
        if let Self::Polygon { sides } = self {
            Some(*sides)
        } else {
            None
        }
    }
    fn set_sides(&mut self, value: usize) -> Result<(), &'static str> {
        if !(3..MAX_POINTS).contains(&value) {
            return Err("Polygon sides must be between 3 and 4095");
        }
        let Self::Polygon { sides } = self else {
            return Err("Sides works while drawing a polygon");
        };
        *sides = value;
        Ok(())
    }
    fn build_on_plane(
        &self,
        points: &[[f64; 3]],
        plane: crate::cplane::CPlane,
    ) -> Result<Built, &'static str> {
        let (kind, sides) = match self {
            Self::Rectangle => ("rectangle", 4),
            Self::Polygon { sides } => ("polygon", *sides),
            _ => return self.build(points),
        };
        let points = crate::planar::points(kind, plane, sides, points)?;
        Self::Polyline.build(&points)
    }
    fn outline(
        &self,
        points: &[[f64; 3]],
        plane: crate::cplane::CPlane,
    ) -> Vec<[f64; 3]> {
        match self {
            Self::Rectangle if points.len() == 2 => {
                crate::planar::points("rectangle", plane, 4, points).unwrap_or_default()
            }
            Self::Polygon { sides } if points.len() == 2 => {
                crate::planar::points("polygon", plane, *sides, points)
                    .unwrap_or_default()
            }
            _ => points.to_vec(),
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if matches!(self, Self::Project) {
            return vec![("Cancel".into(), "Escape".into())];
        }
        if matches!(self, Self::Extend) {
            return vec![
                ("Distance".into(), "Distance".into()),
                ("Next / Done".into(), "".into()),
                ("Cancel".into(), "Escape".into()),
            ];
        }
        if let Self::Gather(recipe) = self {
            return recipe
                .chips
                .iter()
                .map(|(label, line)| ((*label).into(), (*line).into()))
                .collect();
        }
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
                .iter()
                .map(|(label, line)| ((*label).into(), (*line).into()))
                .collect();
        }
        let mut buttons = vec![("Cancel".into(), "Escape".into())];
        if matches!(
            self,
            Self::Polyline | Self::Rectangle | Self::Polygon { .. }
        ) {
            for name in ["Points", "Rectangle", "Polygon"] {
                buttons.push((name.into(), format!("Polyline {name}")));
            }
        }
        if matches!(self, Self::Polyline | Self::Curve) {
            buttons.insert(0, ("Close".into(), "Close".into()));
        }
        buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub enum Built {
    Measured([[f64; 3]; 2]),
    Point(PreparedPoint),
    Line(PreparedLine),
    Move([f64; 3]),
    Polyline(crate::chain::PreparedChain),
    Geometry(session_rust::Geometry),
}

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(
        &self,
        points: &[[f64; 3]],
        _plane: crate::cplane::CPlane,
    ) -> Result<Built, &'static str> {
        self.build(points)
    }
    fn outline(
        &self,
        points: &[[f64; 3]],
        _plane: crate::cplane::CPlane,
    ) -> Vec<[f64; 3]> {
        points.to_vec()
    }
    fn sides(&self) -> Option<usize> {
        None
    }
    fn set_sides(&mut self, _sides: usize) -> Result<(), &'static str> {
        Err("Sides works while drawing a polygon")
    }
    fn buttons(&self) -> Vec<(String, String)> {
        vec![("Cancel".into(), "Escape".into())]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Measure,
    Project,
    Extend,
    Gather(&'static crate::gather_input::Recipe),
    Point,
    Line,
    Move,
    Polyline,
    Curve,
    Rectangle,
    Polygon { sides: usize },
    Primitive(&'static crate::primitive_shape::Shape),
    Transform(crate::transform_draft::Kind),
}

impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Measure => "Measure Distance",
            Self::Project => "Project To Plane",
            Self::Extend => "Extend",
            Self::Gather(recipe) => recipe.name,
            Self::Point => "Point",
            Self::Line => "Line",
            Self::Move => "Move",
            Self::Polyline => "Polyline",
            Self::Curve => "Curve",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
            Self::Transform(kind) => kind.name(),
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Project | Self::Extend => 0,
            Self::Gather(_) => 0,
            Self::Point => 1,
            Self::Line | Self::Move | Self::Measure => 2,
            Self::Polyline | Self::Curve => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) | Self::Transform(_) => 0,
        }
    }
    fn sides(&self) -> Option<usize> {
        if let Self::Polygon { sides } = self {
            Some(*sides)
        } else {
            None
        }
    }
    fn set_sides(&mut self, value: usize) -> Result<(), &'static str> {
        if !(3..MAX_POINTS).contains(&value) {
            return Err("Polygon sides must be between 3 and 4095");
        }
        let Self::Polygon { sides } = self else {
            return Err("Sides works while drawing a polygon");
        };
        *sides = value;
        Ok(())
    }
    fn build_on_plane(
        &self,
        points: &[[f64; 3]],
        plane: crate::cplane::CPlane,
    ) -> Result<Built, &'static str> {
        let (kind, sides) = match self {
            Self::Rectangle => ("rectangle", 4),
            Self::Polygon { sides } => ("polygon", *sides),
            _ => return self.build(points),
        };
        let points = crate::planar::points(kind, plane, sides, points)?;
        Self::Polyline.build(&points)
    }
    fn outline(
        &self,
        points: &[[f64; 3]],
        plane: crate::cplane::CPlane,
    ) -> Vec<[f64; 3]> {
        match self {
            Self::Rectangle if points.len() == 2 => {
                crate::planar::points("rectangle", plane, 4, points).unwrap_or_default()
            }
            Self::Polygon { sides } if points.len() == 2 => {
                crate::planar::points("polygon", plane, *sides, points)
                    .unwrap_or_default()
            }
            _ => points.to_vec(),
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if matches!(self, Self::Project) {
            return vec![("Cancel".into(), "Escape".into())];
        }
        if matches!(self, Self::Extend) {
            return vec![
                ("Distance".into(), "Distance".into()),
                ("Next / Done".into(), "".into()),
                ("Cancel".into(), "Escape".into()),
            ];
        }
        if let Self::Gather(recipe) = self {
            return recipe
                .chips
                .iter()
                .map(|(label, line)| ((*label).into(), (*line).into()))
                .collect();
        }
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
                .iter()
                .map(|(label, line)| ((*label).into(), (*line).into()))
                .collect();
        }
        let mut buttons = vec![("Cancel".into(), "Escape".into())];
        if matches!(
            self,
            Self::Polyline | Self::Rectangle | Self::Polygon { .. }
        ) {
            for name in ["Points", "Rectangle", "Polygon"] {
                buttons.push((name.into(), format!("Polyline {name}")));
            }
        }
        if matches!(self, Self::Polyline | Self::Curve) {
            buttons.insert(0, ("Close".into(), "Close".into()));
        }
        buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Measure, [a, b]) => Ok(Built::Measured([*a, *b])),
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(
```

### 2. `src/editor.rs`

Complete accepted distance points without opening scene history and expose measurement actions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.split = None;
        }
        match action {
            Action::Split => return self.split_command(),
```

Replace it with:

```rust
            self.split = None;
        }
        match action {
            Action::Measure(kind) => return self.measure_selection(kind),
            Action::MeasureDistance(text) => return self.begin_measure_distance(&text),
            Action::BoundingBox => return self.bounding_box(),
            Action::Split => return self.split_command(),
```

**Locate this block:**

```rust
        self.draft = None;
        Ok(Change::View)
    }
    fn bounding_box(&mut self) -> Result<Change, &'static str> {
        let ids = self.measured_ids();
        for id in &ids {
```

Replace it with:

```rust
        self.draft = None;
        Ok(Change::View)
    }
    fn begin_measure_distance(&mut self, text: &str) -> Result<Change, &'static str> {
        crate::drawing_coords::set_plane(self.construction_plane);
        let frame = self.construction_plane.unwrap_or_else(|| {
            crate::cplane::CPlane::view(
                &self
                    .camera
                    .orientation
                    .rotate_vector(session_rust::Vector::y_axis()),
            )
        });
        let points = if text.trim().is_empty() {
            Vec::new()
        } else {
            crate::drawing_input::points(
                text,
                self.construction_plane,
                frame,
                &[],
                None,
            )?
        };
        if points.len() > 2 {
            return Err("Measure Distance takes two points");
        };
        self.apply(Action::BeginTool(
            crate::tool::Shape::Measure,
        ))?;
        self.snapping.objects = true;
        if !points.is_empty() {
            let built = self
                .draft
                .as_mut()
                .unwrap()
                .submit_many(&points)?;
            if let Some(crate::tool::Built::Measured(points)) = built {
                return self.distance_mark(points);
            }
        }
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn bounding_box(&mut self) -> Result<Change, &'static str> {
        let ids = self.measured_ids();
        for id in &ids {
```

**Locate this block:**

```rust
                crate::tool::Built::Geometry(geometry) => {
                    crate::primitive_geometry::insert(scene, geometry).map(|_| ())
                }
                crate::tool::Built::Move(_) => {
                    unreachable!("Move uses its placement preview")
                }
```

Replace it with:

```rust
                crate::tool::Built::Geometry(geometry) => {
                    crate::primitive_geometry::insert(scene, geometry).map(|_| ())
                }
                crate::tool::Built::Measured(_) => {
                    unreachable!("Measurement changes no original geometry")
                }
                crate::tool::Built::Move(_) => {
                    unreachable!("Move uses its placement preview")
                }
```

**Locate this block:**

```rust
        &mut self,
        built: crate::tool::Built,
    ) -> Result<Change, &'static str> {
        let name = self
            .draft
            .as_ref()
```

Replace it with:

```rust
        &mut self,
        built: crate::tool::Built,
    ) -> Result<Change, &'static str> {
        if let crate::tool::Built::Measured(points) = built {
            return self.distance_mark(points);
        }
        let name = self
            .draft
            .as_ref()
```

**Locate this block:**

```rust
    StreamRefresh,
    Split,
    SplitPick(Option<ObjectId>),
    CancelSplit,
```

Replace it with:

```rust
    StreamRefresh,
    Measure(crate::measure_mark::Kind),
    MeasureDistance(String),
    BoundingBox,
    Split,
    SplitPick(Option<ObjectId>),
    CancelSplit,
```

## <a id="checkpoint-84j-register-measurements-and-end-temporary-marks-predictably"></a>11. Register measurements and end temporary marks predictably

### 1. `src/editor.rs`

Connect transient mark cleanup and completed-mark Escape.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
```

Replace it with:

```rust
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if !matches!(
            &action,
            Action::Zoom(_)
                | Action::ZoomAt { .. }
                | Action::PanScreen { .. }
                | Action::Pan(_, _)
                | Action::Orbit(_, _)
                | Action::Fit
                | Action::FitSelected
                | Action::Projection(_)
                | Action::ResetView
                | Action::Isometric
                | Action::Background
                | Action::NormalView(_)
                | Action::Grid(_)
        ) {
            self.mark = None;
        }
        match action {
```

**Locate this block:**

```rust
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.split.is_some() {
            return match text.trim() {
                "" => Some(Ok(Action::Split)),
```

Replace it with:

```rust
    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.mark.is_some() && text.trim().eq_ignore_ascii_case("Escape") {
            return Some(Ok(Action::CancelTool));
        }
        if self.split.is_some() {
            return match text.trim() {
                "" => Some(Ok(Action::Split)),
```

### 2. `src/commands.rs`

Register original BoundingBox and current numerical measurement commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Split" {
```

Replace it with:

```rust
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Measure Distance" {
        if words.len() > count + 2 {
            return Err("Measure Distance takes two points".into());
        }
        return Ok(Command::Editor(Action::MeasureDistance(
            words[count..].join(" "),
        )));
    }
    if name == "Split" {
```

**Locate this block:**

```rust
    }),
    ("Curve", || {
        Action::BeginTool(crate::tool::Shape::Curve)
    }),
```

Replace it with:

```rust
    }),
    ("BoundingBox", || Action::BoundingBox),
    ("Length BoundingBox", || {
        Action::Measure(crate::measure_mark::Kind::BoundingLengths)
    }),
    ("Length", || {
        Action::Measure(crate::measure_mark::Kind::Length)
    }),
    ("Area", || {
        Action::Measure(crate::measure_mark::Kind::Area)
    }),
    ("Volume", || {
        Action::Measure(crate::measure_mark::Kind::Volume)
    }),
    ("Measure Distance", || {
        Action::MeasureDistance(String::new())
    }),
    ("Curve", || {
        Action::BeginTool(crate::tool::Shape::Curve)
    }),
```

## <a id="checkpoint-84k-draw-cached-unpickable-measurement-strokes"></a>12. Draw cached unpickable measurement strokes

### 1. `src/stroke_gpu.rs`

Add a colour-only overlay lane with explicit depth behaviour.

**Locate this existing block:**

```rust
    }

    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn overlay(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let mut depth = crate::depth::ink();
        depth.depth_compare = Some(wgpu::CompareFunction::Always);
        Self::layout_depth(
            device,
            format,
            crate::stroke::SHADER,
            44,
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
            depth,
        )
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 2. `src/measure_gpu.rs`

Cache unchanged marks and account for their actual stroke allocation.

Create the file and type:

```rust
use crate::{measure_mark::Mark, stroke_gpu::Lane};
pub struct Measurement {
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
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        self.strokes
            .view(queue, transform, size, density)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass)
    }
    pub fn bytes(&self) -> u64 {
        self.strokes.usage()[1]
    }
}
```

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust
pub mod measure_mark;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_mark;

pub mod measure_gpu;
pub mod source_mesh_topology;
```

## <a id="checkpoint-84l-merge-measurement-chips-with-cached-scene-text"></a>13. Merge measurement chips with cached scene text

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

### 1. `src/renderer.rs`

Draw cached measurement strokes and merge temporary chips with the existing text owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
```

Replace it with:

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("measurement overlay"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.measurement.draw(&mut pass);
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
        self.measurement.view(
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
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(merged)?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
```

Replace it with:

```rust
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
```

**Locate this block:**

```rust
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(merged)?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
```

Replace it with:

```rust
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(self.merge_measurement(merged))?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn set_preview(
        &mut self,
        draft: Option<&crate::tool::Draft>,
```

Replace it with:

```rust
        Ok(())
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
```

**Locate this block:**

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets = crate::sheet_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            sheets,
            instances,
            instance_rows: Default::default(),
```

Replace it with:

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets = crate::sheet_gpu::Lane::new(&device, format);
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let mut renderer = Self {
            measurement,
            sheets,
            instances,
            instance_rows: Default::default(),
```

**Locate this block:**

```rust
pub struct Renderer {
    sheets: crate::sheet_gpu::Lane,
    clouds: std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view: std::cell::Cell<crate::cloud_gpu::View>,
```

Replace it with:

```rust
pub struct Renderer {
    measurement: crate::measure_gpu::Measurement,
    sheets: crate::sheet_gpu::Lane,
    clouds: std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view: std::cell::Cell<crate::cloud_gpu::View>,
```

## <a id="checkpoint-84m-exercise-original-boxes-and-temporary-distance-annotations"></a>14. Exercise boxes and temporary distance annotations

Exercise original boxes and temporary distance annotations.

### 1. `src/browser.rs`

Route current measurement input, physical Escape and truthful annotation diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &originals.to_string(),
    )?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
```

Replace it with:

```rust
        &originals.to_string(),
    )?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute(
        "data-measurement",
        &editor.mark.as_ref().map_or_else(
            || "null".into(),
            |m| serde_json::json!({"points":m.points,"label":m.label}).to_string(),
        ),
    )?;
    canvas.set_attribute(
        "data-measurement-gpu-bytes",
        &renderer.measurement_bytes().to_string(),
    )?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
```

**Locate this block:**

```rust
                    Err(error) => panel.result(&error),
                }
            }
            renderer.refresh_text_bounds(&mut editor.scene);
            text_fonts.need(
                editor
```

Replace it with:

```rust
                    Err(error) => panel.result(&error),
                }
            }
            if let Err(error) = renderer.set_measurement(editor.mark.as_ref()) {
                panel.result(error);
            }
            renderer.refresh_text_bounds(&mut editor.scene);
            text_fonts.need(
                editor
```

**Locate this block:**

```rust
                );
                let tool_action = matches!(
                    &action,
                    Action::Split
                        | Action::SplitPick(_)
                        | Action::CancelSplit
                        | Action::BeginTool(_)
```

Replace it with:

```rust
                );
                let tool_action = matches!(
                    &action,
                    Action::Measure(_)
                        | Action::MeasureDistance(_)
                        | Action::BoundingBox
                        | Action::Split
                        | Action::SplitPick(_)
                        | Action::CancelSplit
                        | Action::BeginTool(_)
```

**Locate this block:**

```rust
                            .map_err(str::to_owned)
                    })
                    .unwrap_or_else(|| crate::commands::parse(&line));
                match command {
                    Err(error) => {
                        panel.result(&error);
```

Replace it with:

```rust
                            .map_err(str::to_owned)
                    })
                    .unwrap_or_else(|| crate::commands::parse(&line));
                editor.mark = None;
                match command {
                    Err(error) => {
                        panel.result(&error);
```

**Locate this block:**

```rust
            };
            let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some())
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .is_some_and(|key| {
```

Replace it with:

```rust
            };
            let line = if (editor.mark.is_some()
                || editor.split.is_some()
                || editor.draft.is_some()
                || editor.gizmo_edit.is_some())
                && event
                    .dyn_ref::<web_sys::KeyboardEvent>()
                    .is_some_and(|key| {
```

### 2. `src/measure_tests.rs`

Copy original box, exact archive, numerical, selected-face, history, distance and label-collision fixtures.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    measure_mark::{Kind, Unit},
    scene::Scene,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, text: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(text) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(text)? else {
            return Err("Editor action".into());
        };
        a
    };
    e.apply(action).map_err(|m| {
        if m == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            m.into()
        }
    })?;
    Ok(())
}
#[test]
fn bounding_box_is_an_original_wire_box_with_world_lengths_exact_archive_and_one_undo()
{
    let mut e = editor();
    command(
        &mut e,
        "Line w1000000000.125,23,31 w1000000006.125,29,31",
    )
    .unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Select All").unwrap();
    command(&mut e, "Bounding Box").unwrap();
    let row = &e.scene.objects()[0];
    let target = row.id;
    let Some(Geometry::OBB(source)) = &row.original else {
        panic!("Original box")
    };
    assert_eq!(source.center[0], 1000000003.125);
    assert_eq!(
        [
            source.half_size[0],
            source.half_size[1],
            source.half_size[2]
        ],
        [3., 3., 0.]
    );
    assert_eq!(row.model.m, Xform::identity().m);
    assert!(row.mesh.indices().is_empty());
    assert_eq!(row.mesh.parts.edges.len(), 12);
    let proto = source.to_proto();
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().label,
        "X 6 · Y 6 · Z 0 mm"
    );
    assert_eq!(e.selected_objects(), [target]);
    assert!(Rc::ptr_eq(
        &old,
        &e.scene.lines()[0].prepared.source
    ));
    let mut restored = Scene::empty();
    crate::scene_document::open(
        &mut restored,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    let Some(Geometry::OBB(source)) = &restored.objects()[0].original else {
        panic!("Reopened original box")
    };
    assert_eq!(source.to_proto(), proto);
    assert!(
        restored.objects()[0]
            .mesh
            .indices()
            .is_empty()
    );
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    assert!(e.mark.is_none());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn selection_measurements_use_placed_originals_and_preserve_scene_history_and_sources()
{
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w3,4,0").unwrap();
    let line = e.scene.lines()[0].id;
    e.scene
        .place_owner(line, Xform::scale_xyz(2., 3., 1.))
        .unwrap();
    let old = e.scene.lines()[0].prepared.source.clone();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([line], false);
    command(&mut e, "Length").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "13.416 mm");
    assert!(Rc::ptr_eq(
        &old,
        &e.scene.lines()[0].prepared.source
    ));
    command(&mut e, "Length BoundingBox").unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().label,
        "X 6 · Y 12 · Z 0 mm"
    );
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    assert!(e.mark.is_none());
}
#[test]
fn area_volume_face_units_and_open_refusal_follow_original_geometry() {
    let mut e = editor();
    command(&mut e, "Box w0,0,0 2 2 0.4").unwrap();
    let target = e.scene.objects()[0].id;
    e.select_objects([target], false);
    command(&mut e, "Area").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "11.2 mm²");
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 mm³");
    e.unit = Unit::Meters;
    command(&mut e, "Volume").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "1.6 m³");
    let source = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    e.apply(Action::PickedTarget(Some(
        crate::pick_sources::Target {
            owner: target,
            part: crate::pick_sources::Part::Face(0),
        },
    )))
    .unwrap();
    command(&mut e, "Area").unwrap();
    let area = crate::measure_area::compute_area(
        &Geometry::BRep(source),
        &Xform::identity(),
        Some(0),
    )
    .unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().label,
        format!("{} m²", crate::measure_math::to_text(area))
    );
    let surface = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(3., 0., 0.),
            Point::new(4., 0., 0.),
            Point::new(4., 1., 0.),
            Point::new(3., 1., 0.),
        ])
        .unwrap(),
    );
    let id = e.scene.insert_surface(surface).unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Volume").is_err());
    assert!(e.mark.is_none());
    assert!(
        e.scene
            .objects()
            .iter()
            .find(|r| r.id == id)
            .unwrap()
            .surface
            .is_some()
    );
}
#[test]
fn distance_accepts_world_relative_and_plane_inputs_without_creating_originals() {
    let mut e = editor();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Measure Distance w0,0,0 w3,4,0").unwrap();
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.tool_status.contains("dx 3 · dy 4 · dz 0"));
    assert!(e.scene.lines().is_empty());
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Snap Off").unwrap();
    command(&mut e, "Measure Distance").unwrap();
    assert!(e.snapping.objects);
    command(&mut e, "0,0,0").unwrap();
    command(&mut e, "@3,4,0").unwrap();
    assert_eq!(
        e.mark.as_ref().unwrap().points,
        vec![[0., 0., 0.], [3., 0., 4.]]
    );
    assert_eq!(e.mark.as_ref().unwrap().label, "5 mm");
    assert!(e.draft.is_none());
    e.apply(Action::Zoom(1.1)).unwrap();
    assert!(e.mark.is_some());
    command(&mut e, "Escape").unwrap();
    assert!(e.mark.is_none());
    assert!(e.scene.lines().is_empty());
}
#[test]
fn invalid_measurement_arguments_keep_pending_answers_and_redo_and_no_source_proxy_is_measured()
 {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    command(&mut e, "Point w1,1,1").unwrap();
    command(&mut e, "Undo").unwrap();
    e.select_objects([], false);
    assert!(command(&mut e, "BoundingBox").is_err());
    assert!(command(&mut e, "Length").is_err());
    assert!(crate::commands::parse("Length BoundingBox extra").is_err());
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Measure Distance bad").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    let id = crate::bounds_original::insert(
        &mut e.scene,
        Rc::new(crate::bounds_original::box_geometry(
            &crate::bounds::Bounds {
                min: [0.; 3],
                max: [1.; 3],
            },
        )),
    )
    .unwrap();
    e.select_objects([id], false);
    assert!(command(&mut e, "Area").is_err());
    assert!(command(&mut e, "Volume").is_err());
    assert!(command(&mut e, "Length").is_err());
}
#[test]
fn temporary_annotation_ids_do_not_collide_with_scene_labels_or_gain_pick_owners() {
    let (mark, _) =
        crate::measure_mark::distance([0.; 3], [3., 4., 0.], Unit::Millimeters);
    let mut existing = mark.labels(&[]);
    existing[0].id = u32::MAX;
    let labels = mark.labels(&existing);
    assert_eq!(labels.len(), 3);
    assert!(labels.iter().all(|l| l.object.is_none()));
    let ids = labels
        .iter()
        .map(|l| l.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(ids.len(), 3);
    assert!(
        labels
            .iter()
            .all(|l| existing.iter().all(|old| old.id != l.id))
    );
    assert_eq!(labels[0].text, "5 mm");
    assert_eq!(labels[1].text, "■");
}
```

### 3. `src/lib.rs`

Register the measurement component.

**Locate this existing block:**

```rust
pub mod measure_gpu;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_gpu;

#[cfg(test)]
mod measure_tests;
pub mod source_mesh_topology;
```

## <a id="checkpoint-84zza-describe-removable-cad-parts"></a>15. Describe removable CAD parts

Represent curve intervals, BRep face groups and mesh sides without replacing the original CAD geometry.

### 1. `src/trim_parts.rs`

Type the original CAD part representation and seam-aware curve helpers.

Create the file and type:

```rust
use crate::source_cut::{Cutter, planar, samples};
use session_rust::{
    BRep, BRepRef, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, Plane, Point,
    Polyline, Vector, intersection, split,
};
use std::collections::HashMap;
use std::rc::Rc;

pub const TOLERANCE: f64 = 1e-6; // local units, as Split

/// What a cut leaves of one object: the parts a click can remove.
pub enum Parts {
    Curve {
        curve: NurbsCurve, // the object as a curve
        cuts: Vec<f64>,    // sorted cut parameters inside its domain
    },
    Faces {
        brep: BRep,               // the object with every cut face split
        groups: Vec<Vec<usize>>,  // faces of each part
        meshes: Vec<Mesh>,        // one mesh per face, for clicks
        section: Vec<Vec<Point>>, // the cut edges
    },
    Mesh {
        sides: Vec<Mesh>,         // above and below the plane
        section: Vec<[Point; 2]>, // the cut segments
    },
}

/// One cutter and, for a straight line, the plane it stands for off the object.
pub struct Blade {
    pub cutter: Cutter,       // in the target's frame
    pub fence: Option<Plane>, // a line's vertical plane, in the target's frame
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 {
        d0
    } else {
        cuts[index - 1]
    };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near)
        .then(|| trimmed(a, d1))
        .flatten();
    let head = (b > d0 + near)
        .then(|| trimmed(d0, b))
        .flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => {
            NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
                .into_iter()
                .next()
        }
        (tail, head) => tail.or(head),
    }
}
```

### 2. `src/lib.rs`

Register the shared part owner.

**Locate this existing block:**

```rust
mod measure_tests;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod measure_tests;

pub mod source_mesh_topology;
pub mod trim_parts;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-84zzb-read-trim-part-previews"></a>16. Read trim part previews

Expose part counts, sampled curve outlines, section edges and meshes through one Parts owner.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Parts {
    /// How many parts there are.
    pub fn count(&self) -> usize {
        match self {
            Parts::Curve { curve, cuts } if curve.is_closed() => cuts.len(),
            Parts::Curve { cuts, .. } => cuts.len() + 1,
            Parts::Faces { groups, .. } => groups.len(),
            Parts::Mesh { sides, .. } => sides.len(),
        }
    }

    /// Part `index` of a curve as points, for drawing and clicking.
    pub fn outline(&self, index: usize) -> Vec<Point> {
        let Parts::Curve { curve, cuts } = self else {
            return Vec::new();
        };
        let (a, b) = interval(curve, cuts, index);
        piece(curve, a, b).map_or_else(Vec::new, |part| samples(&part, 48))
    }

    /// The points where a curve was cut.
    pub fn cut_points(&self) -> Vec<Point> {
        match self {
            Parts::Curve { curve, cuts } => cuts
                .iter()
                .map(|t| curve.point_at(*t))
                .collect(),
            _ => Vec::new(),
        }
    }

    /// The cut edges as polylines.
    pub fn section(&self) -> Vec<Vec<Point>> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { section, .. } => section.clone(),
            Parts::Mesh { section, .. } => section
                .iter()
                .map(|pair| pair.to_vec())
                .collect(),
        }
    }

    /// The meshes a click can hit, with the part each belongs to.
    pub fn hit_meshes(&self) -> Vec<(usize, &Mesh)> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces {
                groups, meshes, ..
            } => groups
                .iter()
                .enumerate()
                .flat_map(|(part, faces)| {
                    faces
                        .iter()
                        .filter_map(move |f| Some((part, meshes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
```

## <a id="checkpoint-84zzc-collect-curve-crossings"></a>17. Collect curve crossings

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

Collect kernel curve intersections, filter open endpoints and merge a closed seam.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 {
        d0
    } else {
        cuts[index - 1]
    };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near)
        .then(|| trimmed(a, d1))
        .flatten();
    let head = (b > d0 + near)
        .then(|| trimmed(d0, b))
        .flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => {
            NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
                .into_iter()
                .next()
        }
        (tail, head) => tail.or(head),
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The cut parameters of a curve: crossings with the curves, passes through the planes.
fn curve_cuts(curve: &NurbsCurve, blades: &[Blade]) -> Result<Vec<f64>, String> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let closed = curve.is_closed();
    let curves: Vec<NurbsCurve> = blades
        .iter()
        .filter_map(|blade| match &blade.cutter {
            Cutter::Curve(curve) => Some(curve.clone()),
            Cutter::Plane(_) => None,
        })
        .collect();
    let mut cuts = Vec::new();

    // every piece end is a crossing
    if !curves.is_empty() {
        let pieces = split::split_curve_by_curves(curve, &curves, TOLERANCE)?;

        if pieces.len() > 1 {
            for piece in &pieces {
                for end in [piece.point_at_start(), piece.point_at_end()] {
                    cuts.push(intersection::curve_closest_point(curve, &end, d0, d1).0);
                }
            }
        }
    }

    for blade in blades {
        if let Cutter::Plane(plane) = &blade.cutter {
            cuts.extend(intersection::curve_plane(
                curve,
                plane,
                Some(TOLERANCE),
            ));
        }
    }

    // a closed curve's seam is one point; an open curve's ends cut nothing
    let mut cuts: Vec<f64> = cuts
        .into_iter()
        .map(|t| {
            if closed && t >= d1 - near {
                d0
            } else {
                t
            }
        })
        .filter(|t| closed || (*t > d0 + near && *t < d1 - near))
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() <= near.max(1e-12));
    Ok(cuts)
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 {
        d0
    } else {
        cuts[index - 1]
    };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near)
        .then(|| trimmed(a, d1))
        .flatten();
    let head = (b > d0 + near)
        .then(|| trimmed(d0, b))
        .flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => {
            NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
                .into_iter()
                .next()
        }
        (tail, head) => tail.or(head),
    }
}

/// New indices for the used entries, -1 for the rest.
fn remap(used: &[bool]) -> Vec<i32> {
    let mut next = 0;
    used.iter()
        .map(|&kept| {
            if !kept {
                return -1;
            }

            next += 1;
            next - 1
        })
        .collect()
}

/// The entries of `list` marked in `used`.
fn kept_of<T: Clone>(list: &[T], used: &[bool]) -> Vec<T> {
    list.iter()
        .zip(used)
        .filter(|(_, kept)| **kept)
        .map(|(item, _)| item.clone())
        .collect()
}
```

## <a id="checkpoint-84zzd-preserve-trimmed-source-types"></a>18. Preserve trimmed source types

Rebuild kept curves in their original source family with their presentation.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// New indices for the used entries, -1 for the rest.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The run `a..b` of `curve` as the source's own type, with its name and pen.
fn rebuilt(source: &Geometry, curve: &NurbsCurve, a: f64, b: f64) -> Option<Geometry> {
    let part = piece(curve, a, b)?;

    match source {
        Geometry::Line(line) => {
            let mut next =
                Line::from_points(&part.point_at_start(), &part.point_at_end());
            next.name = line.name.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.linecolor = line.linecolor.clone();
            Some(Geometry::Line(Rc::new(next)))
        }
        Geometry::Polyline(polyline) => {
            let mut next = Polyline::new(samples(&part, 1));
            next.name = polyline.name.clone();
            next.width = polyline.width;
            next.dash = polyline.dash.clone();
            next.linecolor = polyline.linecolor.clone();
            Some(Geometry::Polyline(Rc::new(next)))
        }
        Geometry::NurbsCurve(original) => {
            let mut next = part;
            next.refresh_guid();
            next.name = original.name.clone();
            next.width = original.width;

            // per-control colours only fit the control count they were made for
            if next.pointcolors.len() != next.m_cv_count {
                next.pointcolors.clear();
            }

            if next.linecolors.len() + 1 != next.m_cv_count {
                next.linecolors.clear();
            }

            Some(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => None,
    }
}

/// A surface as a one-face BRep, cut by nothing yet.
fn face_of(surface: &NurbsSurface) -> Result<BRep, String> {
    let corner = surface
        .get_cv(0, 0)
        .ok_or("Surface has no control points")?;
    let size = planar_size(surface);
    // a cutter far off the surface cuts nothing, so the kernel only wraps it
    let far = &corner + &Vector::new(size * 10.0, size * 10.0, size * 10.0);
    let away = NurbsCurve::create(
        false,
        1,
        &[
            far.clone(),
            &far + &Vector::new(size, 0.0, 0.0),
        ],
    );
    let mut brep = split::split_surface_by_curves(surface, &[away], TOLERANCE)?;
    brep.name = surface.name.clone();
    Ok(brep)
}

/// The largest control point coordinate of a surface, at least 1.
fn planar_size(surface: &NurbsSurface) -> f64 {
    let mut size: f64 = 1.0;

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                size = size
                    .max(p[0].abs())
                    .max(p[1].abs())
                    .max(p[2].abs());
            }
        }
    }

    size
}

/// New indices for the used entries, -1 for the rest.
```

## <a id="checkpoint-84zze-group-faces-by-cutting-planes"></a>19. Group faces by cutting planes

Extend a plane-plane intersection past surface control points.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// New indices for the used entries, -1 for the rest.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
fn sides(
    brep: &BRep,
    meshes: &[Mesh],
    planes: &[Plane],
    regions: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut groups: Vec<(Vec<bool>, Option<usize>, Vec<usize>)> = Vec::new();

    for face in 0..brep.face_count() {
        let center = meshes.get(face).map_or_else(
            || brep.point_at(face, 0.5, 0.5),
            |mesh| mesh.centroid(),
        );
        let key: Vec<bool> = planes
            .iter()
            .map(|plane| (&center - &plane.origin()).dot(&plane.z_axis()) >= 0.0)
            .collect();
        let region = regions
            .iter()
            .any(|faces| faces.contains(&face))
            .then_some(face);

        match groups
            .iter_mut()
            .find(|(signs, own, _)| *signs == key && *own == region && region.is_none())
        {
            Some((_, _, faces)) => faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups
        .into_iter()
        .map(|(_, _, faces)| faces)
        .collect()
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
fn across(line: &Line, surface: &NurbsSurface) -> NurbsCurve {
    let start = line.start();
    let mut direction = line.to_vector();
    direction.normalize_self();
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                let t = (&p - &start).dot(&direction);
                low = low.min(t);
                high = high.max(t);
            }
        }
    }

    let margin = (high - low).max(1.0) * 0.1;
    NurbsCurve::create(
        false,
        1,
        &[
            &start + &(&direction * (low - margin)),
            &start + &(&direction * (high + margin)),
        ],
    )
}

/// New indices for the used entries, -1 for the rest.
```

## <a id="checkpoint-84zzf-cut-planar-brep-faces"></a>20. Cut planar BRep faces

Find crossed planar faces, split them through the kernel and collect section edges.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Split every face `plane` passes through; returns the new edges on the plane.
fn cut_by_plane(brep: &mut BRep, plane: &Plane) -> Result<Vec<Vec<Point>>, String> {
    let normal = plane.z_axis();
    let origin = plane.origin();
    let side = |p: &Point| (p - &origin).dot(&normal);
    let size = brep
        .m_vertices
        .iter()
        .map(|v| {
            v.point[0]
                .abs()
                .max(v.point[1].abs())
                .max(v.point[2].abs())
        })
        .fold(1.0, f64::max);
    let near = TOLERANCE * size;
    let mut crossed = false;

    for face in 0..brep.face_count() {
        let surface = &brep.m_surfaces[brep.m_faces[face].surface_index as usize];
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

        for i in 0..surface.m_cv_count[0] {
            for j in 0..surface.m_cv_count[1] {
                let d = surface
                    .get_cv(i, j)
                    .map_or(0.0, |p| side(&p));
                low = low.min(d);
                high = high.max(d);
            }
        }

        // all on one side: nothing to cut
        if low > -near || high < near {
            continue;
        }

        // curved sections are left out: the kernel's surface-plane tracer costs 15 KB of wasm
        let own = planar(surface).ok_or(
            "A plane cuts flat faces only; cut a curved face with a curve that lies on it",
        )?;
        let Some(line) = intersection::plane_plane(&own, plane) else {
            continue;
        };
        let before = brep.face_count();
        let next = split::split_brep_face_by_curves(
            brep,
            face,
            &[across(&line, surface)],
            TOLERANCE,
        )?;
        crossed |= next.face_count() > before;
        *brep = next;
    }

    if !crossed {
        return Err("not crossed".into());
    }

    // edges lying in the plane are the cut
    let mut section = Vec::new();

    for edge in &brep.m_edges {
        let Some(curve) = brep
            .m_curves_3d
            .get(edge.curve_3d_index.max(0) as usize)
            .filter(|_| edge.curve_3d_index >= 0)
        else {
            continue;
        };
        let points = samples(curve, 8);

        if points
            .iter()
            .all(|p| side(p).abs() <= near * 10.0)
        {
            section.push(points);
        }
    }

    Ok(section)
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
```

## <a id="checkpoint-84zzg-assemble-removable-face-groups"></a>21. Assemble removable face groups

Combine curve regions, fallback fence planes and crossed plane sections into one Parts value.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Split the faces of `brep` by the blades: curves on its faces make regions, planes make sides.
fn faces(brep: &BRep, blades: &[Blade]) -> Result<Parts, String> {
    let mut brep = brep.clone();
    let mut regions: Vec<Vec<usize>> = Vec::new(); // faces a curve split, one region each
    let mut planes: Vec<Plane> = Vec::new();
    let mut section: Vec<Vec<Point>> = Vec::new();
    let mut refused = None;

    for blade in blades {
        let curve = match &blade.cutter {
            Cutter::Plane(plane) => {
                planes.push(plane.clone());
                continue;
            }
            Cutter::Curve(curve) => curve,
        };
        let mut hit = false;

        for face in 0..brep.face_count() {
            let before = brep.face_count();

            match split::split_brep_face_by_curves(
                &brep,
                face,
                std::slice::from_ref(curve),
                TOLERANCE,
            ) {
                Ok(next) if next.face_count() > before => {
                    regions.push(
                        [face]
                            .into_iter()
                            .chain(before..next.face_count())
                            .collect(),
                    );
                    brep = next;
                    hit = true;
                }
                Ok(_) => {}
                Err(error) => refused = Some(error),
            }
        }

        match (&blade.fence, hit) {
            (_, true) => section.push(samples(curve, 48)),
            (Some(fence), false) => planes.push(fence.clone()),
            (None, false) => {}
        }
    }

    // a plane that misses this object is some other target's
    let mut crossing = Vec::new();

    for plane in planes {
        match cut_by_plane(&mut brep, &plane) {
            Ok(split) => {
                section.extend(split);
                crossing.push(plane);
            }
            Err(error) if error == "not crossed" => {}
            Err(error) => return Err(error),
        }
    }

    let planes = crossing;

    if regions.is_empty() && section.is_empty() {
        return Err(refused.unwrap_or_else(|| {
            "Curves cut a BRep only where they lie on its faces; use a line, a Plane or a planar surface to cut through it".into()
        }));
    }

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    let groups = if planes.is_empty() {
        // each region of a split face is a part; unsplit faces are no part
        regions
            .into_iter()
            .flatten()
            .map(|face| vec![face])
            .collect()
    } else {
        sides(&brep, &meshes, &planes, &regions)
    };

    if groups.len() < 2 {
        return Err("not crossed".into());
    }

    Ok(Parts::Faces {
        brep,
        groups,
        meshes,
        section,
    })
}

/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
```

## <a id="checkpoint-84zzh-mark-retained-brep-topology"></a>22. Mark retained BRep topology

Walk retained faces through wires and edges to mark their surfaces, vertices and curve tables.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// The entries of `list` marked in `used`.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// `brep` without `removed` faces; every table is compacted and a solid left open is dropped.
pub fn without_faces(brep: &BRep, removed: &[usize]) -> Result<BRep, String> {
    if removed.is_empty() {
        return Ok(brep.clone());
    }

    let keep_face: Vec<bool> = (0..brep.m_faces.len())
        .map(|face| !removed.contains(&face))
        .collect();

    if !keep_face.contains(&true) {
        return Err("The last part stays; use Delete".into());
    }

    let mut keep_wire = vec![false; brep.m_wires.len()];
    let mut keep_surface = vec![false; brep.m_surfaces.len()];

    for (face, kept) in brep.m_faces.iter().zip(&keep_face) {
        if !kept {
            continue;
        }

        keep_surface[face.surface_index as usize] = true;

        for wire in &face.wires {
            keep_wire[wire.index as usize] = true;
        }
    }

    let mut keep_edge = vec![false; brep.m_edges.len()];

    for (wire, kept) in brep.m_wires.iter().zip(&keep_wire) {
        if !kept {
            continue;
        }

        for edge in &wire.edges {
            keep_edge[edge.index as usize] = true;
        }
    }

    let mut keep_vertex = vec![false; brep.m_vertices.len()];
    let mut keep_curve = vec![false; brep.m_curves_3d.len()];
    let mut keep_pcurve = vec![false; brep.m_curves_2d.len()];

    for (edge, kept) in brep.m_edges.iter().zip(&keep_edge) {
        if !kept {
            continue;
        }

        for vertex in [edge.start_vertex, edge.end_vertex] {
            if vertex >= 0 {
                keep_vertex[vertex as usize] = true;
            }
        }

        if edge.curve_3d_index >= 0 {
            keep_curve[edge.curve_3d_index as usize] = true;
        }

        for pcurve in edge
            .pcurves
            .iter()
            .filter(|p| keep_surface[p.surface_index as usize])
        {
            keep_pcurve[pcurve.curve_2d_index as usize] = true;

            if pcurve.curve_2d_index_2 >= 0 {
                keep_pcurve[pcurve.curve_2d_index_2 as usize] = true;
            }
        }
    }

    Err("BRep table reconstruction is not connected yet".into())
}

/// The entries of `list` marked in `used`.
```

## <a id="checkpoint-84zzi-rebuild-retained-brep-tables"></a>23. Rebuild retained BRep tables

Compact marked BRep tables and rewrite every reference.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
    }

    Err("BRep table reconstruction is not connected yet".into())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    let [
        faces,
        wires,
        edges,
        vertices,
        curves,
        pcurves,
        surfaces,
    ] = [
        &keep_face,
        &keep_wire,
        &keep_edge,
        &keep_vertex,
        &keep_curve,
        &keep_pcurve,
        &keep_surface,
    ]
    .map(|used| remap(used));
    let at = |map: &[i32], index: i32| {
        if index < 0 {
            index
        } else {
            map[index as usize]
        }
    };
    let mut out = brep.clone();
    out.m_surfaces = kept_of(&brep.m_surfaces, &keep_surface);
    out.m_curves_3d = kept_of(&brep.m_curves_3d, &keep_curve);
    out.m_curves_2d = kept_of(&brep.m_curves_2d, &keep_pcurve);
    out.m_vertices = kept_of(&brep.m_vertices, &keep_vertex);
    out.m_edges = kept_of(&brep.m_edges, &keep_edge);

    for edge in &mut out.m_edges {
        edge.curve_3d_index = at(&curves, edge.curve_3d_index);
        edge.start_vertex = at(&vertices, edge.start_vertex);
        edge.end_vertex = at(&vertices, edge.end_vertex);
        edge.pcurves
            .retain(|p| keep_surface[p.surface_index as usize]);

        for pcurve in &mut edge.pcurves {
            pcurve.surface_index = at(&surfaces, pcurve.surface_index);
            pcurve.curve_2d_index = at(&pcurves, pcurve.curve_2d_index);
            pcurve.curve_2d_index_2 = at(&pcurves, pcurve.curve_2d_index_2);
        }
    }

    let refs = |list: &[BRepRef], map: &[i32]| -> Vec<BRepRef> {
        list.iter()
            .filter(|r| map[r.index as usize] >= 0)
            .map(|r| BRepRef::new(map[r.index as usize], r.orientation))
            .collect()
    };
    out.m_wires = kept_of(&brep.m_wires, &keep_wire);

    for wire in &mut out.m_wires {
        wire.edges = refs(&wire.edges, &edges);
    }

    out.m_faces = kept_of(&brep.m_faces, &keep_face);

    for face in &mut out.m_faces {
        face.surface_index = at(&surfaces, face.surface_index);
        face.wires = refs(&face.wires, &wires);
    }

    let mut shells = Vec::new(); // new index of each shell, -1 when it lost every face
    out.m_shells.clear();

    for shell in &brep.m_shells {
        let mut shell = shell.clone();
        shell.faces = refs(&shell.faces, &faces);
        shells.push(if shell.faces.is_empty() {
            -1
        } else {
            out.m_shells.len() as i32
        });

        if !shell.faces.is_empty() {
            out.m_shells.push(shell);
        }
    }

    out.m_solids.clear();

    for solid in &brep.m_solids {
        let mut solid = solid.clone();
        solid.shells = refs(&solid.shells, &shells);

        // a solid is only a solid while its shells are closed
        if !solid.shells.is_empty()
            && solid
                .shells
                .iter()
                .all(|r| out.is_closed(r.index as usize))
        {
            out.m_solids.push(solid);
        }
    }

    if !out.is_valid() {
        return Err("Removing these faces leaves an invalid BRep".into());
    }

    Ok(out)
}
```

## <a id="checkpoint-84zzj-split-mesh-faces-at-a-plane"></a>24. Split mesh faces at a plane

Classify mesh vertices, reuse one cut vertex per original edge and build both clipped face rings.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
        .map(|(item, _)| item.clone())
        .collect()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .map(|(item, _)| item.clone())
        .collect()
}

/// A mesh cut by `plane` into its two sides, sharing the cut vertices; the cut as segments.
pub fn mesh_sides(
    mesh: &Mesh,
    plane: &Plane,
) -> Result<(Vec<Mesh>, Vec<[Point; 2]>), String> {
    if !mesh.get_face_holes().is_empty() {
        return Err("Trim cannot cut a mesh with holed faces".into());
    }

    let (mut points, faces) = mesh.to_vertices_and_faces();
    let normal = plane.z_axis();
    let origin = plane.origin();
    let size = points
        .iter()
        .map(|p| p[0].abs().max(p[1].abs()).max(p[2].abs()))
        .fold(1.0, f64::max);
    let near = 1e-9 * size;
    let side: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = (p - &origin).dot(&normal);
            if d.abs() <= near {
                0.0
            } else {
                d
            }
        })
        .collect();

    if side.iter().all(|d| *d >= 0.0) || side.iter().all(|d| *d <= 0.0) {
        return Err("not crossed".into());
    }

    let mut crossings: HashMap<(usize, usize), usize> = HashMap::new(); // edge to its cut vertex
    let mut halves: [Vec<Vec<usize>>; 2] = [Vec::new(), Vec::new()];
    let mut section = Vec::new();

    for face in &faces {
        if face.iter().all(|v| side[*v] >= 0.0) {
            halves[0].push(face.clone());
            continue;
        }

        if face.iter().all(|v| side[*v] <= 0.0) {
            halves[1].push(face.clone());
            continue;
        }

        let mut above = Vec::new();
        let mut below = Vec::new();
        let mut on = Vec::new(); // this face's points on the plane

        for (i, &a) in face.iter().enumerate() {
            let b = face[(i + 1) % face.len()];

            if side[a] >= 0.0 {
                above.push(a);
            }

            if side[a] <= 0.0 {
                below.push(a);
            }

            if side[a] == 0.0 {
                on.push(a);
            }

            if side[a] * side[b] < 0.0 {
                let key = (a.min(b), a.max(b));
                let at = *crossings.entry(key).or_insert_with(|| {
                    let t = side[a] / (side[a] - side[b]);
                    let (p, q) = (&points[a], &points[b]);
                    points.push(Point::new(
                        p[0] + t * (q[0] - p[0]),
                        p[1] + t * (q[1] - p[1]),
                        p[2] + t * (q[2] - p[2]),
                    ));
                    points.len() - 1
                });
                above.push(at);
                below.push(at);
                on.push(at);
            }
        }

        if let [a, b] = on.as_slice() {
            section.push([points[*a].clone(), points[*b].clone()]);
        }

        for (half, ring) in [(0, above), (1, below)] {
            if ring.len() >= 3 {
                halves[half].push(ring);
            }
        }
    }

    Err("Mesh table reconstruction is not connected yet".into())
}
```

## <a id="checkpoint-84zzk-compact-both-trimmed-mesh-sides"></a>25. Compact both trimmed mesh sides

Compact each clipped mesh side to only its used vertices, rewrite its faces and preserve source name and color.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
    }

    Err("Mesh table reconstruction is not connected yet".into())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    let sides = halves
        .into_iter()
        .map(|faces| {
            // only the vertices this side uses
            let mut index = vec![usize::MAX; points.len()];
            let mut used = Vec::new();

            for face in &faces {
                for &v in face {
                    if index[v] == usize::MAX {
                        index[v] = used.len();
                        used.push(points[v].clone());
                    }
                }
            }

            let faces = faces
                .iter()
                .map(|face| face.iter().map(|v| index[*v]).collect())
                .collect();
            let mut side = Mesh::from_vertices_and_faces(used, faces);
            side.name = mesh.name.clone();
            side.set_objectcolor(mesh.get_objectcolor().clone());
            side
        })
        .collect();
    Ok((sides, section))
}
```

## <a id="checkpoint-84zzl-rebuild-kept-cad-parts"></a>26. Rebuild kept CAD parts

Merge adjacent kept curve intervals, remove BRep face groups and return the retained mesh half.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

**Locate this existing block:**

```rust
}

/// The run `a..b` of `curve` as the source's own type, with its name and pen.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// What is left of `source` once `removed` parts are gone: one geometry per unbroken run.
pub fn kept(
    source: &Geometry,
    parts: &Parts,
    removed: &[usize],
) -> Result<Vec<Geometry>, String> {
    let count = parts.count();

    if removed.len() >= count {
        return Err("The last part stays; use Delete".into());
    }

    match parts {
        Parts::Curve { curve, cuts } => {
            let closed = curve.is_closed();
            // start after a removed part, so a run never breaks at a closed curve's seam
            let first = if closed {
                (0..count)
                    .find(|i| removed.contains(i))
                    .map_or(0, |i| (i + 1) % count)
            } else {
                0
            };
            let mut runs: Vec<(f64, f64)> = Vec::new();
            let mut open = false; // the last run can still grow

            for step in 0..count {
                let index = (first + step) % count;

                if removed.contains(&index) {
                    open = false;
                    continue;
                }

                let (a, b) = interval(curve, cuts, index);

                match runs.last_mut() {
                    Some(run) if open => run.1 = b,
                    _ => runs.push((a, b)),
                }

                open = true;
            }

            runs.iter()
                .map(|(a, b)| {
                    rebuilt(source, curve, *a, *b)
                        .ok_or_else(|| "Cannot cut this curve".to_string())
                })
                .collect()
        }
        Parts::Faces { brep, groups, .. } => {
            let faces: Vec<usize> = removed
                .iter()
                .flat_map(|part| groups[*part].iter().copied())
                .collect();
            Ok(vec![Geometry::BRep(Rc::new(without_faces(
                brep, &faces,
            )?))])
        }
        Parts::Mesh { sides, .. } => {
            let side = (0..sides.len())
                .find(|i| !removed.contains(i))
                .ok_or("The last part stays")?;
            Ok(vec![Geometry::Mesh(Rc::new(
                sides[side].clone(),
            ))])
        }
    }
}

/// The run `a..b` of `curve` as the source's own type, with its name and pen.
```

## <a id="checkpoint-84zzm-connect-original-trim-recipes"></a>27. Connect trim recipes

Route original curves, surfaces, BReps and meshes through the shared part builders.

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner. Supply the original trim validation fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .collect();
    Ok((sides, section))
}
```

Replace it with:

```rust
        .collect();
    Ok((sides, section))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A curve cutter.
    fn blade(a: [f64; 3], b: [f64; 3]) -> Blade {
        let curve = NurbsCurve::create(
            false,
            1,
            &[
                Point::new(a[0], a[1], a[2]),
                Point::new(b[0], b[1], b[2]),
            ],
        );
        Blade {
            cutter: Cutter::Curve(curve),
            fence: None,
        }
    }

    /// A plane cutter.
    fn plane(origin: [f64; 3], normal: [f64; 3]) -> Blade {
        let plane = Plane::from_point_normal(
            Point::new(origin[0], origin[1], origin[2]),
            Vector::new(normal[0], normal[1], normal[2]),
            None,
        );
        Blade {
            cutter: Cutter::Plane(plane),
            fence: None,
        }
    }

    /// A line from its ends.
    fn line(a: [f64; 3], b: [f64; 3]) -> Geometry {
        Geometry::Line(Rc::new(Line::from_points(
            &Point::new(a[0], a[1], a[2]),
            &Point::new(b[0], b[1], b[2]),
        )))
    }

    /// The ends of a line geometry.
    fn ends(geometry: &Geometry) -> [[f64; 3]; 2] {
        let Geometry::Line(line) = geometry else {
            panic!("not a line")
        };
        let (a, b) = (line.start(), line.end());
        [[a[0], a[1], a[2]], [b[0], b[1], b[2]]]
    }

    /// Lines keep the part they were not clicked on; three parts keep the two outer ones.
    #[test]
    fn a_line_keeps_what_is_not_removed() {
        let mut source = Line::from_points(
            &Point::new(0., 0., 0.),
            &Point::new(100., 0., 0.),
        );
        source.width = 3.0;
        source.name = "rail".into();
        let source = Geometry::Line(Rc::new(source));
        let parts = cut(
            &source,
            &[blade([50., -5., 0.], [50., 5., 0.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let kept_left = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(
            ends(&kept_left[0]),
            [[0., 0., 0.], [50., 0., 0.]]
        );
        let Geometry::Line(styled) = &kept_left[0] else {
            panic!()
        };
        assert_eq!(
            (styled.width, styled.name.as_str()),
            (3.0, "rail")
        );
        let parts = cut(
            &source,
            &[
                blade([30., -5., 0.], [30., 5., 0.]),
                blade([70., -5., 0.], [70., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let outer = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(outer.len(), 2);
        assert!((ends(&outer[0])[1][0] - 30.0).abs() < 1e-9);
        assert!((ends(&outer[1])[0][0] - 70.0).abs() < 1e-9);
        let right = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(
            right.len(),
            1,
            "the two kept parts stay one line"
        );
        assert!(
            (ends(&right[0])[0][0] - 30.0).abs() < 1e-9
                && (ends(&right[0])[1][0] - 100.0).abs() < 1e-9
        );
        assert!(kept(&source, &parts, &[0, 1, 2]).is_err());
        assert!(
            cut(
                &source,
                &[blade([50., 5., 0.], [50., 15., 0.])]
            )
            .is_err()
        );
    }

    /// A polyline keeps its corner; a NURBS curve ends at the cut; a closed square makes two parts.
    #[test]
    fn polylines_curves_and_closed_curves() {
        let corner = Polyline::new(vec![
            Point::new(0., 60., 0.),
            Point::new(40., 60., 0.),
            Point::new(40., 100., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(corner));
        let parts = cut(
            &source,
            &[blade([20., 50., 0.], [20., 70., 0.])],
        )
        .unwrap();
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::Polyline(rest) = &rest[0] else {
            panic!()
        };
        let points = rest.get_points();
        assert_eq!(points.len(), 3);
        assert!(points[0].distance(&Point::new(20., 60., 0.), None) < 1e-9);
        assert!(points[1].distance(&Point::new(40., 60., 0.), None) < 1e-9);
        let curve = NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0., -60., 0.),
                Point::new(50., 0., 0.),
                Point::new(100., -60., 0.),
            ],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let parts = cut(
            &source,
            &[blade([50., -80., 0.], [50., 10., 0.])],
        )
        .unwrap();
        let left = kept(&source, &parts, &[1]).unwrap();
        let Geometry::NurbsCurve(left) = &left[0] else {
            panic!()
        };
        assert!((left.point_at_end()[0] - 50.0).abs() < 1e-6);
        let square = Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(10., 0., 0.),
            Point::new(10., 10., 0.),
            Point::new(0., 10., 0.),
            Point::new(0., 0., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(square));
        let parts = cut(
            &source,
            &[blade([5., -5., 0.], [5., 15., 0.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let half = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(half.len(), 1);
    }

    /// A plane cuts a vertical line; mixed cutters cut at both.
    #[test]
    fn planes_and_curves_cut_together() {
        let source = line([0., 0., -10.], [0., 0., 10.]);
        let parts = cut(&source, &[plane([0., 0., 0.], [0., 0., 1.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let across = line([0., 0., 0.], [100., 0., 0.]);
        let parts = cut(
            &across,
            &[
                plane([20., 0., 0.], [1., 0., 0.]),
                blade([60., -5., 0.], [60., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let points = parts.cut_points();
        assert!(
            (points[0][0] - 20.0).abs() < 1e-6 && (points[1][0] - 60.0).abs() < 1e-6
        );
    }

    /// A box cut by a plane: removing the top leaves an open, valid five-face BRep.
    #[test]
    fn a_box_cut_by_a_plane_loses_its_top() {
        let source = BRep::create_box(10., 10., 10.);
        let low = source
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let parts = cut(
            &Geometry::BRep(Rc::new(source)),
            &[plane([0., 0., low + 2.], [0., 0., 1.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let Parts::Faces {
            brep,
            groups,
            section,
            ..
        } = &parts
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 10);
        assert_eq!(section.len(), 4, "four cut edges");
        let upper = groups
            .iter()
            .position(|faces| {
                faces
                    .iter()
                    .all(|f| brep.point_at(*f, 0.5, 0.5)[2] > low + 2.0 - 1e-9)
            })
            .unwrap();
        let rest = kept(
            &Geometry::BRep(Rc::new(BRep::new())),
            &parts,
            &[upper],
        )
        .unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        assert!(!rest.is_solid());

        for edge in 0..rest.m_edges.len() {
            assert!(
                !rest.edge_faces(edge).is_empty(),
                "edge {edge} has a face"
            );
        }

        let used: std::collections::HashSet<i32> = rest
            .m_edges
            .iter()
            .flat_map(|e| [e.start_vertex, e.end_vertex])
            .collect();
        assert_eq!(
            used.len(),
            rest.m_vertices.len(),
            "no unused vertex"
        );
        assert!(
            cut(
                &Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.))),
                &[plane([0., 0., 100.], [0., 0., 1.])]
            )
            .is_err()
        );
        assert_eq!(
            without_faces(rest, &[])
                .unwrap()
                .face_count(),
            5
        );
    }

    /// A plane through a curved face is refused rather than cut half way.
    #[test]
    fn a_plane_refuses_curved_faces() {
        let source = Geometry::BRep(Rc::new(BRep::create_cylinder(5., 10.)));
        let Geometry::BRep(cylinder) = &source else {
            panic!()
        };
        let low = cylinder
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let answer = cut(
            &source,
            &[plane([0., 0., low + 5.], [0., 0., 1.])],
        );
        assert!(answer.is_err_and(|error| error.contains("flat faces")));
    }

    /// A line off the box cuts it as a vertical fence; a line on the top face splits that face only.
    #[test]
    fn lines_fence_or_split_a_box() {
        let source = Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.)));
        let Geometry::BRep(boxed) = &source else {
            panic!()
        };
        let low = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let high = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::NEG_INFINITY, f64::max);
        let x = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[0])
            .fold(f64::INFINITY, f64::min)
            + 3.0;
        let fence = crate::source_cut::fence_plane(
            &Line::from_points(
                &Point::new(x, -50., low - 5.),
                &Point::new(x, 50., low - 5.),
            ),
            &Vector::new(0., 0., 1.),
        )
        .unwrap();
        let mut off = blade([x, -50., low - 5.], [x, 50., low - 5.]);
        off.fence = Some(fence);
        let parts = cut(&source, &[off]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        let on = blade([x, -50., high], [x, 50., high]);
        let parts = cut(&source, &[on]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[1]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 6);
        assert!(rest.is_valid());
    }

    /// A planar surface split by a line keeps the chosen region as a BRep.
    #[test]
    fn a_surface_becomes_the_kept_region() {
        let mut surface = NurbsSurface::new(3, false, 2, 2, 2, 2);
        surface.set_cv(0, 0, &Point::new(0., 0., 0.));
        surface.set_cv(1, 0, &Point::new(10., 0., 0.));
        surface.set_cv(0, 1, &Point::new(0., 10., 0.));
        surface.set_cv(1, 1, &Point::new(10., 10., 0.));
        let source = Geometry::NurbsSurface(Rc::new(surface));
        let parts = cut(
            &source,
            &[blade([4., -5., 0.], [4., 15., 0.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 1);
    }

    /// A mesh box cut at z = 2: the areas add up, both sides are open and share the cut.
    #[test]
    fn a_mesh_splits_into_two_open_sides() {
        let mut source = Mesh::create_box(10., 10., 10.);
        source.name = "block".into();
        let area = source.area();
        let (points, _) = source.to_vertices_and_faces();
        let low = points
            .iter()
            .map(|p| p[2])
            .fold(f64::INFINITY, f64::min);
        let cutter = Plane::from_point_normal(
            Point::new(0., 0., low + 2.),
            Vector::new(0., 0., 1.),
            None,
        );
        let (sides, section) = mesh_sides(&source, &cutter).unwrap();
        assert!((sides[0].area() + sides[1].area() - area).abs() < 1e-9);
        assert!(!sides[0].is_closed() && !sides[1].is_closed());
        assert_eq!(sides[1].number_of_vertices(), 8);
        assert_eq!(sides[0].name, "block");
        assert!(!section.is_empty());
        let away = Plane::from_point_normal(
            Point::new(0., 0., 100.),
            Vector::new(0., 0., 1.),
            None,
        );
        assert!(mesh_sides(&source, &away).is_err());
    }
}
```

**Locate this block:**

```rust
}

/// The cut parameters of a curve: crossings with the curves, passes through the planes.
fn curve_cuts(curve: &NurbsCurve, blades: &[Blade]) -> Result<Vec<f64>, String> {
    let (d0, d1) = curve.domain();
```

Replace it with:

```rust
}

/// Cut `target` (in its own frame) by `blades`; an error when nothing is crossed.
pub fn cut(target: &Geometry, blades: &[Blade]) -> Result<Parts, String> {
    match target {
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => {
            let curve = crate::source_cut::as_curve(target)
                .ok_or("This curve has no points")?;
            let cuts = curve_cuts(&curve, blades)?;
            let parts = Parts::Curve { curve, cuts };

            if parts.count() < 2 {
                return Err("not crossed".into());
            }

            Ok(parts)
        }
        Geometry::BRep(brep) => faces(brep, blades),
        Geometry::NurbsSurface(surface) => {
            let brep = face_of(surface)?;
            faces(&brep, blades)
        }
        Geometry::Mesh(mesh) => {
            let mut planes = blades
                .iter()
                .filter_map(|blade| match &blade.cutter {
                    Cutter::Plane(plane) => Some(plane),
                    Cutter::Curve(_) => blade.fence.as_ref(),
                })
                .peekable();

            if planes.peek().is_none() {
                return Err(
                    "A mesh is cut by a plane: a Plane, a planar surface or a straight line".into(),
                );
            }

            // the first plane that passes through it
            let mut answer = Err("not crossed".to_string());

            for plane in planes {
                answer = mesh_sides(mesh, plane);

                if answer.as_ref().is_ok()
                    || answer
                        .as_ref()
                        .is_err_and(|error| error != "not crossed")
                {
                    break;
                }
            }

            let (sides, section) = answer?;
            Ok(Parts::Mesh { sides, section })
        }
        _ => {
            Err("Trim cuts lines, polylines, curves, surfaces, BReps and meshes".into())
        }
    }
}

/// The cut parameters of a curve: crossings with the curves, passes through the planes.
fn curve_cuts(curve: &NurbsCurve, blades: &[Blade]) -> Result<Vec<f64>, String> {
    let (d0, d1) = curve.domain();
```

## <a id="checkpoint-84zna-index-selection-loops-by-pixel-row"></a>28. Index selection loops by pixel row

Build a row index for the even-odd point test.

### 1. `src/selection_region.rs`

Type the shared selection region and row index.

Create the file and type:

```rust
use crate::source_cut::planar;
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Mesh, NurbsCurve, NurbsSurface, Point};
const CURVE_STEPS: usize = 64; // steps along a free curve
pub(crate) const EDGE_STEPS: usize = 16; // steps along a BRep edge
const FACE_STEPS: usize = 8; // steps across a curved BRep face, each way
const SURFACE_STEPS: usize = 16; // steps across a surface, each way
const CLOUD_SAMPLES: usize = 65_536; // most points tested per cloud

/// A closed loop indexed by pixel row: where each row's centre line crosses it, for the even-odd test.
pub(crate) struct Region {
    top: f64,               // the first row's top edge, device pixels
    rows: Vec<Vec<f64>>,    // crossings per row, ascending
    box_: Option<[f64; 4]>, // left, top, right, bottom when the loop is a rectangle
}

/// What a region takes: what lies wholly inside it, or whatever it touches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Reach {
    Window,   // every sample inside
    Crossing, // a sample inside, or a stroke across the rectangle
}

impl Region {
    /// Index the loop through `points`; the last point joins the first.
    pub(crate) fn new(points: &[(f64, f64)]) -> Self {
        let top = points
            .iter()
            .map(|p| p.1)
            .fold(f64::INFINITY, f64::min)
            .floor();
        let bottom = points
            .iter()
            .map(|p| p.1)
            .fold(f64::NEG_INFINITY, f64::max)
            .ceil();
        let mut rows = vec![Vec::new(); (bottom - top).max(0.0) as usize + 1];

        for (index, a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];
            let (low, high) = (a.1.min(b.1), a.1.max(b.1));
            let mut row = (low - top - 0.5).ceil().max(0.0) as usize;

            // rows whose centre lies in [low, high); a flat edge crosses none
            while row < rows.len() && top + row as f64 + 0.5 < high {
                let y = top + row as f64 + 0.5;
                rows[row].push(a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1));
                row += 1;
            }
        }

        for row in &mut rows {
            row.sort_by(f64::total_cmp);
        }

        Self {
            top,
            rows,
            box_: None,
        }
    }

    /// The rectangle with corners `a` and `b`, device pixels.
    pub(crate) fn rectangle(a: (f64, f64), b: (f64, f64)) -> Self {
        let (left, right) = (a.0.min(b.0), a.0.max(b.0));
        let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
        Self {
            box_: Some([left, top, right, bottom]),
            ..Self::new(&[
                (left, top),
                (right, top),
                (right, bottom),
                (left, bottom),
            ])
        }
    }
}
```

### 2. `src/lib.rs`

Register the selection owner.

**Locate this existing block:**

```rust
mod cloud_shader_contract;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cloud_shader_contract;
mod shader_check;

pub mod selection_region;
```

## <a id="checkpoint-84znb-test-window-membership-and-segment-crossings"></a>29. Test window membership and segment crossings

Complete Region with point membership and segment clipping against the rectangle.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    /// True when the segment from `a` to `b` meets the rectangle; false for a loop that is no rectangle.
    fn crosses(&self, a: (f64, f64), b: (f64, f64)) -> bool {
        let Some([left, top, right, bottom]) = self.box_ else {
            return false;
        };
        // clip the segment's parameter range to each side in turn (Liang-Barsky)
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let (mut t0, mut t1) = (0.0_f64, 1.0_f64);

        for (p, q) in [
            (-dx, a.0 - left),
            (dx, right - a.0),
            (-dy, a.1 - top),
            (dy, bottom - a.1),
        ] {
            if p == 0.0 {
                if q < 0.0 {
                    return false;
                }
                continue;
            }

            let t = q / p;

            if p < 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }

        t0 <= t1
    }

    /// True when `at` is inside by the even-odd rule, judged on its row's centre line.
    pub(crate) fn contains(&self, at: (f64, f64)) -> bool {
        let row = (at.1 - self.top).floor();

        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
}
```

## <a id="checkpoint-84znc-sample-original-cad-families-for-selection"></a>30. Sample CAD families for selection

Reuse original curve corners, curved domains, mesh vertices and BRep references as bounded selection samples.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
fn every(
    samples: impl Iterator<Item = [f64; 3]>,
    inside: &dyn Fn([f64; 3]) -> bool,
) -> Option<bool> {
    let mut any = false;

    for sample in samples {
        if !inside(sample) {
            return Some(false);
        }

        any = true;
    }

    any.then_some(true)
}

/// A point as three numbers.
fn xyz(point: &Point) -> [f64; 3] {
    [point[0], point[1], point[2]]
}

/// A straight curve's corners, else `steps` even steps over its domain.
pub(crate) fn curve_samples(
    curve: &NurbsCurve,
    steps: usize,
) -> impl Iterator<Item = [f64; 3]> + '_ {
    let straight = curve.degree() == 1;
    let (t0, t1) = curve.domain();
    let corners = (0..if straight {
        curve.cv_count()
    } else {
        0
    })
        .filter_map(|i| curve.get_cv(i));
    let even = (0..if straight {
        0
    } else {
        steps + 1
    })
        .map(move |i| curve.point_at(t0 + (t1 - t0) * i as f64 / steps as f64));
    corners.chain(even).map(|p| xyz(&p))
}

/// A grid of `steps` by `steps` even steps over the surface's domain.
fn surface_samples(
    surface: &NurbsSurface,
    steps: usize,
) -> impl Iterator<Item = [f64; 3]> + '_ {
    let domains = surface.domain(0).zip(surface.domain(1));
    let ((u0, u1), (v0, v1)) = domains.unwrap_or_default();
    let side = if domains.is_some() {
        steps + 1
    } else {
        0
    };
    (0..side * side)
        .filter_map(move |k| {
            let u = u0 + (u1 - u0) * (k / side) as f64 / steps as f64;
            let v = v0 + (v1 - v0) * (k % side) as f64 / steps as f64;
            surface.point_at(u, v)
        })
        .map(|p| xyz(&p))
}

/// Every vertex of a mesh.
fn mesh_samples(mesh: &Mesh) -> impl Iterator<Item = [f64; 3]> + '_ {
    mesh.vertex.values().map(|v| [v.x, v.y, v.z])
}

/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep
        .m_vertices
        .iter()
        .map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
```

## <a id="checkpoint-84znd-collect-original-boundary-strokes"></a>31. Collect boundary strokes

Build surface grids and closed mesh outlines, then combine BRep vertices, edges and curved faces.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The surface's grid lines both ways, `steps` steps each.
fn surface_strokes(surface: &NurbsSurface, steps: usize) -> Vec<Vec<[f64; 3]>> {
    let Some(((u0, u1), (v0, v1))) = surface.domain(0).zip(surface.domain(1)) else {
        return Vec::new();
    };
    let at = |i: usize, j: usize| {
        let u = u0 + (u1 - u0) * i as f64 / steps as f64;
        let v = v0 + (v1 - v0) * j as f64 / steps as f64;
        surface.point_at(u, v).map(|p| xyz(&p))
    };
    let rows = (0..=steps).map(|i| {
        (0..=steps)
            .filter_map(|j| at(i, j))
            .collect()
    });
    let columns = (0..=steps).map(|j| {
        (0..=steps)
            .filter_map(|i| at(i, j))
            .collect()
    });
    rows.chain(columns).collect()
}

/// Every face of a mesh as its closed outline.
fn mesh_strokes(mesh: &Mesh) -> Vec<Vec<[f64; 3]>> {
    mesh.face
        .values()
        .map(|face| {
            let mut loop_: Vec<[f64; 3]> = face
                .iter()
                .filter_map(|key| mesh.vertex.get(key).map(|v| [v.x, v.y, v.z]))
                .collect();
            loop_.extend(loop_.first().copied());
            loop_
        })
        .collect()
}

/// Vertices, edges and the grid lines of curved faces.
fn brep_strokes(brep: &BRep) -> Vec<Vec<[f64; 3]>> {
    let vertices = brep
        .m_vertices
        .iter()
        .map(|vertex| vec![xyz(&vertex.point)]);
    let edges = brep
        .m_curves_3d
        .iter()
        .map(|curve| curve_samples(curve, EDGE_STEPS).collect());
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
```

## <a id="checkpoint-84zne-test-crossing-selection-on-original-strokes"></a>32. Test crossing selection on strokes

Dispatch original geometry families to their shared stroke builders.

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

**Locate this existing block:**

```rust
}

/// The surface's grid lines both ways, `steps` steps each.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
    project: &dyn Fn([f64; 3]) -> Option<(f64, f64)>,
    region: &Region,
) -> Option<bool> {
    let strokes = strokes(geometry)?;
    let mut any = false;

    for stroke in &strokes {
        let points: Vec<Option<(f64, f64)>> =
            stroke.iter().map(|p| project(*p)).collect();

        if points
            .iter()
            .flatten()
            .any(|at| region.contains(*at))
        {
            return Some(true);
        }

        for pair in points.windows(2) {
            if let [Some(a), Some(b)] = pair
                && region.crosses(*a, *b)
            {
                return Some(true);
            }
        }

        any |= !points.is_empty();
    }

    any.then_some(false)
}

/// The geometry as strokes in its own frame, each a run of points joined in order; None for a plane or a frame.
fn strokes(geometry: &Geometry) -> Option<Vec<Vec<[f64; 3]>>> {
    let strokes = match geometry {
        Geometry::Point(point) => vec![vec![xyz(point)]],
        Geometry::Line(line) => vec![vec![xyz(&line.start()), xyz(&line.end())]],
        Geometry::Polyline(polyline) => vec![
            polyline
                .coords
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect(),
        ],
        Geometry::NurbsCurve(curve) => {
            vec![curve_samples(curve, CURVE_STEPS).collect()]
        }
        Geometry::NurbsSurface(surface) => surface_strokes(surface, SURFACE_STEPS),
        Geometry::Mesh(mesh) => mesh_strokes(mesh),
        Geometry::BRep(brep) => brep_strokes(brep),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => mesh_strokes(mesh),
            ElementGeometry::BRep(brep) => brep_strokes(brep),
            ElementGeometry::None => return None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3)
                .div_ceil(CLOUD_SAMPLES)
                .max(1);
            coords
                .chunks_exact(3)
                .step_by(step)
                .map(|c| vec![[c[0], c[1], c[2]]])
                .collect()
        }
        Geometry::Plane(_) | Geometry::OBB(_) => return None,
    };

    Some(strokes)
}

/// The surface's grid lines both ways, `steps` steps each.
```

## <a id="checkpoint-84znf-connect-window-and-bounds-selection-tests"></a>33. Connect window and bounds selection tests

Add projection, conservative screen bounds and complete-enclosure dispatch.

### 1. `src/selection_region.rs`

Extend the existing original selection owner. Supply the original region-selection validation fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
```

Replace it with:

```rust
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Line;

    /// The rectangle 10..50 by 10..50, the plan seen from above one pixel per unit.
    fn rectangle() -> Region {
        Region::rectangle((50.0, 50.0), (10.0, 10.0))
    }

    fn plan(p: [f64; 3]) -> Option<(f64, f64)> {
        Some((p[0], p[1]))
    }

    /// Either corner first gives one rectangle; segments meet it inside, across or not at all.
    #[test]
    fn a_rectangle_holds_and_crosses() {
        let region = rectangle();
        assert!(region.contains((30.0, 30.0)) && !region.contains((60.0, 30.0)));
        assert!(
            region.crosses((0.0, 30.0), (60.0, 30.0)),
            "straight through"
        );
        assert!(
            region.crosses((20.0, 20.0), (25.0, 25.0)),
            "inside"
        );
        assert!(
            !region.crosses((0.0, 0.0), (0.0, 60.0)),
            "beside"
        );
        assert!(
            !region.crosses((0.0, 70.0), (70.0, 55.0)),
            "below"
        );
        assert!(
            !Region::new(&[(0.0, 0.0), (9.0, 0.0), (9.0, 9.0)])
                .crosses((0.0, 5.0), (9.0, 5.0)),
            "a lasso has no rectangle"
        );
    }

    /// A line half inside: a crossing takes it, a window does not; a line through with both ends out is a crossing too.
    #[test]
    fn a_window_takes_what_is_inside_a_crossing_what_it_touches() {
        let region = rectangle();
        let window = |geometry: &Geometry| {
            enclosed(geometry, &|p| {
                plan(p).is_some_and(|at| region.contains(at))
            })
        };
        let crossing = |geometry: &Geometry| touched(geometry, &plan, &region);

        let half = Geometry::Line(Line::new(30.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!(
            (window(&half), crossing(&half)),
            (Some(false), Some(true))
        );

        let through = Geometry::Line(Line::new(0.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!(
            (window(&through), crossing(&through)),
            (Some(false), Some(true))
        );

        let inside = Geometry::Line(Line::new(20.0, 20.0, 0.0, 40.0, 40.0, 0.0).into());
        assert_eq!(
            (window(&inside), crossing(&inside)),
            (Some(true), Some(true))
        );

        let away = Geometry::Line(Line::new(60.0, 60.0, 0.0, 90.0, 90.0, 0.0).into());
        assert_eq!(
            (window(&away), crossing(&away)),
            (Some(false), Some(false))
        );

        // a box bigger than the rectangle: its edges cross it
        let around = Geometry::Mesh(Mesh::create_box(100.0, 100.0, 1.0).into());
        let shifted = |p: [f64; 3]| plan([p[0] + 30.0, p[1] + 30.0, p[2]]);
        assert_eq!(
            touched(&around, &shifted, &region),
            Some(false),
            "only the outline; the rectangle sits inside the faces"
        );
        let crossing_box = |p: [f64; 3]| plan([p[0] + 80.0, p[1] + 30.0, p[2]]);
        assert_eq!(
            touched(&around, &crossing_box, &region),
            Some(true)
        );
    }
}
```

**Locate this block:**

```rust
    vertices.chain(edges).chain(faces).collect()
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
fn every(
    samples: impl Iterator<Item = [f64; 3]>,
```

Replace it with:

```rust
    vertices.chain(edges).chain(faces).collect()
}

/// Whether every sample of `geometry`, in its own frame, passes `inside`; None when only its box can tell.
pub(crate) fn enclosed(
    geometry: &Geometry,
    inside: &dyn Fn([f64; 3]) -> bool,
) -> Option<bool> {
    match geometry {
        Geometry::Point(point) => every(std::iter::once(xyz(point)), inside),
        Geometry::Line(line) => every(
            [xyz(&line.start()), xyz(&line.end())].into_iter(),
            inside,
        ),
        Geometry::Polyline(polyline) => every(
            polyline
                .coords
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]]),
            inside,
        ),
        Geometry::NurbsCurve(curve) => every(curve_samples(curve, CURVE_STEPS), inside),
        Geometry::NurbsSurface(surface) => every(
            surface_samples(surface, SURFACE_STEPS),
            inside,
        ),
        Geometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
        Geometry::BRep(brep) => every(brep_samples(brep), inside),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
            ElementGeometry::BRep(brep) => every(brep_samples(brep), inside),
            ElementGeometry::None => None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3)
                .div_ceil(CLOUD_SAMPLES)
                .max(1);
            every(
                coords
                    .chunks_exact(3)
                    .step_by(step)
                    .map(|c| [c[0], c[1], c[2]]),
                inside,
            )
        }
        Geometry::Plane(_) | Geometry::OBB(_) => None,
    }
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
fn every(
    samples: impl Iterator<Item = [f64; 3]>,
```

**Locate this block:**

```rust
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

Replace it with:

```rust
}

/// Device pixels of `p` under the clip matrix `m`; None behind the eye.
fn screen(m: &[f64; 16], p: [f64; 3], size: (f64, f64)) -> Option<(f64, f64)> {
    let clip = |r: usize| m[r] * p[0] + m[r + 4] * p[1] + m[r + 8] * p[2] + m[r + 12];
    let w = clip(3);
    (w > 0.0).then(|| {
        (
            (clip(0) / w * 0.5 + 0.5) * size.0,
            (0.5 - clip(1) / w * 0.5) * size.1,
        )
    })
}

/// Whether the screen box around projected corners overlaps the region's rectangle.
fn box_touches(corners: &[Option<(f64, f64)>], region: &Region) -> bool {
    let (Some([left, top, right, bottom]), Some(first)) =
        (region.box_, corners.iter().flatten().next())
    else {
        return false;
    };
    let mut lo = *first;
    let mut hi = *first;

    for at in corners.iter().flatten() {
        lo = (lo.0.min(at.0), lo.1.min(at.1));
        hi = (hi.0.max(at.0), hi.1.max(at.1));
    }

    lo.0 <= right && hi.0 >= left && lo.1 <= bottom && hi.1 >= top
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

## <a id="checkpoint-84zoa-classify-originals-and-isolate-one-mesh-face"></a>34. Classify originals and isolate one mesh face

Name each original geometry kind once.

### 1. `src/source_shape.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use session_rust::Geometry;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Box,
    BRep,
    Element,
    Line,
    Mesh,
    Curve,
    Surface,
    Plane,
    Point,
    Cloud,
    Polyline,
}

impl Shape {
    /// The type of one geometry.
    pub fn of(geometry: &Geometry) -> Self {
        match geometry {
            Geometry::OBB(_) => Shape::Box,
            Geometry::BRep(_) => Shape::BRep,
            Geometry::Element(_) => Shape::Element,
            Geometry::Line(_) => Shape::Line,
            Geometry::Mesh(_) => Shape::Mesh,
            Geometry::NurbsCurve(_) => Shape::Curve,
            Geometry::NurbsSurface(_) => Shape::Surface,
            Geometry::Plane(_) => Shape::Plane,
            Geometry::Point(_) => Shape::Point,
            Geometry::PointCloud(_) => Shape::Cloud,
            Geometry::Polyline(_) => Shape::Polyline,
        }
    }

    /// The name shown for an unnamed object.
    pub fn label(self) -> &'static str {
        match self {
            Shape::Box => "Box",
            Shape::BRep => "BRep",
            Shape::Element => "Element",
            Shape::Line => "Line",
            Shape::Mesh => "Mesh",
            Shape::Curve => "NURBS curve",
            Shape::Surface => "NURBS surface",
            Shape::Plane => "Plane",
            Shape::Point => "Point",
            Shape::Cloud => "Point cloud",
            Shape::Polyline => "Polyline",
        }
    }
}
```

### 2. `src/lib.rs`

Compile the shared original geometry classifier.

**Locate this existing block:**

```rust
mod shader_check;

pub mod selection_region;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shader_check;

pub mod selection_region;

pub mod source_shape;
```

### 3. `src/explode_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use crate::interval_geometry::MAX_POINTS;
use session_rust::{BRep, BRepRef, Geometry, Line, Mesh};
use std::{collections::HashMap, rc::Rc};
/// One face of a mesh as a mesh of its own, holes included.
fn face_mesh(mesh: &Mesh, face: usize) -> Option<Mesh> {
    let outer = mesh.face_vertices(face)?;
    let holes = mesh.face_holes.get(&face);
    let mut index = HashMap::new(); // old vertex key to new
    let mut points = Vec::new();

    for &key in outer
        .iter()
        .chain(holes.into_iter().flatten().flatten())
    {
        if !index.contains_key(&key) {
            index.insert(key, points.len());
            points.push(mesh.vertex_point(key)?);
        }
    }

    let ring = outer.iter().map(|key| index[key]).collect();
    let mut part = Mesh::from_vertices_and_faces(points, vec![ring]);

    if let Some(holes) = holes {
        let rings = holes
            .iter()
            .map(|hole| hole.iter().map(|key| index[key]).collect())
            .collect();
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}
```

### 4. `src/lib.rs`

Register this original command owner.

**Locate this existing block:**

```rust
mod measure_tests;

pub mod source_mesh_topology;
pub mod trim_parts;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod measure_tests;

pub mod trim_parts;

pub mod explode_geometry;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-84zob-copy-original-brep-edges-and-pcurves"></a>35. Copy BRep edges and pcurves

Copy an edge, its endpoint vertices and its surface pcurves into a new BRep.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    Some(part)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep
        .m_edges
        .get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => {
            part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32
        }
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends
        .iter_mut()
        .zip([edge.start_vertex, edge.end_vertex])
    {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep
                    .m_vertices
                    .get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}
```

## <a id="checkpoint-84zoc-build-one-original-brep-face"></a>36. Build one BRep face

Reuse the copied-edge helper to rebuild one face surface, ordered wire references and one oriented shell.

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(
        brep.m_surfaces
            .get(usize::try_from(surface).ok()?)?,
    );
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep
            .m_wires
            .get(usize::try_from(wire.index).ok()?)?
            .edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(
                        brep,
                        &mut part,
                        used.index,
                        surface,
                        &mut vertices,
                    )?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(
            part.add_wire(&refs) as i32,
            wire.orientation,
        ));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
```

## <a id="checkpoint-84zod-explode-supported-original-families"></a>37. Explode supported families

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
use std::{collections::HashMap, rc::Rc};
/// One face of a mesh as a mesh of its own, holes included.
fn face_mesh(mesh: &Mesh, face: usize) -> Option<Mesh> {
    let outer = mesh.face_vertices(face)?;
    let holes = mesh.face_holes.get(&face);
    let mut index = HashMap::new(); // old vertex key to new
    let mut points = Vec::new();

    for &key in outer
        .iter()
        .chain(holes.into_iter().flatten().flatten())
    {
        if !index.contains_key(&key) {
            index.insert(key, points.len());
            points.push(mesh.vertex_point(key)?);
        }
    }

    let ring = outer.iter().map(|key| index[key]).collect();
    let mut part = Mesh::from_vertices_and_faces(points, vec![ring]);

    if let Some(holes) = holes {
        let rings = holes
            .iter()
            .map(|hole| hole.iter().map(|key| index[key]).collect())
            .collect();
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(
        brep.m_surfaces
            .get(usize::try_from(surface).ok()?)?,
    );
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep
            .m_wires
            .get(usize::try_from(wire.index).ok()?)?
            .edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(
                        brep,
                        &mut part,
                        used.index,
                        surface,
                        &mut vertices,
                    )?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(
            part.add_wire(&refs) as i32,
            wire.orientation,
        ));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep
        .m_edges
        .get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => {
            part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32
        }
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends
        .iter_mut()
        .zip([edge.start_vertex, edge.end_vertex])
    {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep
                    .m_vertices
                    .get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{collections::HashMap, rc::Rc};
pub fn pieces(geometry: &Geometry) -> Result<Vec<Geometry>, String> {
    let count = match geometry {
        Geometry::Polyline(line) => line.point_count().saturating_sub(1),
        Geometry::BRep(brep) => brep.face_count(),
        Geometry::Mesh(mesh) => mesh.number_of_faces(),
        Geometry::PointCloud(cloud) => cloud.point_count(),
        _ => 0,
    };

    // counted before anything is made
    if count > MAX_POINTS {
        return Err(format!(
            "Explode makes at most {MAX_POINTS} objects at once"
        ));
    }

    let parts: Vec<Geometry> = match geometry {
        Geometry::Polyline(line) => line
            .get_points()
            .windows(2)
            .filter(|pair| pair[0].distance(&pair[1], None) > 1e-12)
            .map(|pair| {
                let mut part = Line::from_points(&pair[0], &pair[1]);
                part.name = line.name.clone();
                part.width = line.width;
                part.dash = line.dash.clone();
                part.linecolor = line.linecolor.clone();
                Geometry::Line(Rc::new(part))
            })
            .collect(),
        Geometry::BRep(brep) if brep.face_count() > 1 => (0..brep.face_count())
            .map(|face| face_brep(brep, face).map(|part| Geometry::BRep(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This BRep's tables do not resolve")?,
        Geometry::Mesh(mesh) if mesh.number_of_faces() > 1 => mesh
            .faces()
            .into_iter()
            .map(|face| face_mesh(mesh, face).map(|part| Geometry::Mesh(Rc::new(part))))
            .collect::<Option<_>>()
            .ok_or("This mesh's faces do not resolve")?,
        Geometry::PointCloud(cloud) => (0..cloud.point_count())
            .map(|index| {
                let mut point = cloud.get_point(index);
                point.name = cloud.name.clone();
                point.width = cloud.point_size;

                if index < cloud.color_count() {
                    point.pointcolor = cloud.get_color(index);
                }

                Geometry::Point(Rc::new(point))
            })
            .collect(),
        _ => Vec::new(),
    };

    if parts.len() < 2 {
        return Err(format!(
            "{} cannot be exploded",
            crate::source_shape::Shape::of(geometry).label()
        ));
    }

    Ok(parts)
}

/// One face of a mesh as a mesh of its own, holes included.
fn face_mesh(mesh: &Mesh, face: usize) -> Option<Mesh> {
    let outer = mesh.face_vertices(face)?;
    let holes = mesh.face_holes.get(&face);
    let mut index = HashMap::new(); // old vertex key to new
    let mut points = Vec::new();

    for &key in outer
        .iter()
        .chain(holes.into_iter().flatten().flatten())
    {
        if !index.contains_key(&key) {
            index.insert(key, points.len());
            points.push(mesh.vertex_point(key)?);
        }
    }

    let ring = outer.iter().map(|key| index[key]).collect();
    let mut part = Mesh::from_vertices_and_faces(points, vec![ring]);

    if let Some(holes) = holes {
        let rings = holes
            .iter()
            .map(|hole| hole.iter().map(|key| index[key]).collect())
            .collect();
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}

/// One face of a BRep as a BRep of its own: its surface, wires, edges, vertices and pcurves on that surface.
fn face_brep(brep: &BRep, index: usize) -> Option<BRep> {
    let face = brep.m_faces.get(index)?;
    let surface = face.surface_index;
    let mut part = BRep::new();
    part.name = brep.name.clone();
    part.width = brep.width;
    part.surfacecolor = brep.surfacecolor.clone();
    part.add_surface(
        brep.m_surfaces
            .get(usize::try_from(surface).ok()?)?,
    );
    let mut vertices: HashMap<i32, i32> = HashMap::new(); // old index to new
    let mut edges: HashMap<i32, i32> = HashMap::new();
    let mut wires = Vec::with_capacity(face.wires.len());

    for wire in &face.wires {
        let uses = &brep
            .m_wires
            .get(usize::try_from(wire.index).ok()?)?
            .edges;
        let mut refs = Vec::with_capacity(uses.len());

        for used in uses {
            let edge = match edges.get(&used.index) {
                Some(&edge) => edge,
                None => {
                    let edge = copy_edge(
                        brep,
                        &mut part,
                        used.index,
                        surface,
                        &mut vertices,
                    )?;
                    edges.insert(used.index, edge);
                    edge
                }
            };
            refs.push(BRepRef::new(edge, used.orientation));
        }

        wires.push(BRepRef::new(
            part.add_wire(&refs) as i32,
            wire.orientation,
        ));
    }

    let made = part.add_face(0, &wires, face.tolerance);
    part.m_faces[made].facecolor = face.facecolor.clone();
    part.add_shell(&[BRepRef::new(0, brep.face_orientation(index))]);
    Some(part)
}

/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep
        .m_edges
        .get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => {
            part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32
        }
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends
        .iter_mut()
        .zip([edge.start_vertex, edge.end_vertex])
    {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep
                    .m_vertices
                    .get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Color, Point, PointCloud, Vector};
    #[test]
    fn a_brep_becomes_its_faces() {
        let area = |brep: &BRep| {
            brep.face_meshes_q(None)
                .iter()
                .map(|mesh| mesh.area())
                .sum::<f64>()
        };

        for brep in [
            BRep::create_box(2.0, 3.0, 4.0),
            BRep::create_cylinder(1.0, 2.0),
            BRep::create_sphere(1.0),
        ] {
            let parts: Vec<BRep> = (0..brep.face_count())
                .map(|face| face_brep(&brep, face).unwrap())
                .collect();
            let mut total = 0.0;

            for part in &parts {
                assert_eq!(part.face_count(), 1);
                assert!(part.is_valid());
                assert!(!part.face_meshes_q(None).is_empty());
                total += area(part);
            }

            // a smaller object tessellates a little finer
            assert!(
                (total - area(&brep)).abs() < 1e-2 * area(&brep),
                "{total} {}",
                area(&brep)
            );
        }

        let sphere = Geometry::BRep(Rc::new(BRep::create_sphere(1.0)));
        assert!(
            pieces(&sphere).is_err(),
            "one face is nothing to explode"
        );
        assert_eq!(
            pieces(&Geometry::BRep(Rc::new(BRep::create_box(
                2.0, 3.0, 4.0
            ))))
            .unwrap()
            .len(),
            6
        );
    }

    /// Clouds give coloured points, meshes their faces; a line is one piece and refused.
    #[test]
    fn clouds_and_meshes_explode_and_lines_do_not() {
        let points: Vec<Point> = (0..25)
            .map(|i| Point::new(i as f64, 0.0, 0.0))
            .collect();
        let colors = vec![Color::red(); 25];
        let mut cloud = PointCloud::new(
            points,
            vec![Vector::new(0.0, 0.0, 1.0); 25],
            colors,
        );
        cloud.name = "scan".into();
        let parts = pieces(&Geometry::PointCloud(Rc::new(cloud))).unwrap();
        assert_eq!(parts.len(), 25);
        let Geometry::Point(point) = &parts[3] else {
            panic!()
        };
        assert_eq!(
            (
                point.name.as_str(),
                point[0],
                point.pointcolor.r
            ),
            ("scan", 3.0, 1.0)
        );
        assert_eq!(
            pieces(&Geometry::Mesh(Rc::new(Mesh::create_box(
                1.0, 1.0, 1.0
            ))))
            .unwrap()
            .len(),
            6
        );
        let line = Geometry::Line(Rc::new(Line::new(
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        )));
        assert_eq!(
            pieces(&line).err().as_deref(),
            Some("Line cannot be exploded")
        );
    }
}
```

## Build & Run

![Recorded visual output for Lesson 16.4 (84zod-explode-supported-original-families)](../../screenshots/journey/84zod-explode-supported-original-families.png)

```sh
npm --prefix ../session_tests run course -- dependencies 16.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 16.4
npm --prefix ../session_tests run course -- save 16.4
```

Continue with [16.5 · Text, Plane, Trim and integrated editing commands](85-copy-current-original-command-recipes.md).
