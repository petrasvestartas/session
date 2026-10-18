# 16.4.1 · Measure original closed-solid volume

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 24–47 minutes.** [Estimate](typing-load.md).

Reject open geometry, choose a world point near the mesh and reuse the shared swept-volume helper. Orient solid BRep face meshes before adding their signed volumes.

## Type

Continue from [Integrate original surface area](83ze-integrate-original-surface-area.md). [Save or recover your work](recovery.md).

### 1. `src/measure_volume.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-1.rs"
```

### 2. `src/lib.rs`

Register this original measurement owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod measure_area;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-2.rs" skip=23
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/measure_volume.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::measure_math::compute_swept;
use crate::cad_edges::edge_chains;
use crate::cad_orient::face_signs;
use session_rust::{BRep,Geometry,Mesh,Point,Xform};
use session_rust::element::ElementGeometry;
const OPEN:&str="open";
/// World volume of a closed mesh or solid BRep; Err when open, None when the kind has no volume.
pub fn compute_volume(geometry: &Geometry, place: &Xform) -> Option<Result<f64, &'static str>> {
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
    let Some(origin) = meshes.iter().find(|mesh| !mesh.vertex.is_empty()) else {
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
        .map_or(Point::new(0.0, 0.0, 0.0), |p| p.transformed(place))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/84-measure-original-curves-surfaces-and-closed-solids-dedup-volume-3.rs" skip=2289
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The original split face and its cutter remain visible while measurement connects.

### Recorded visual result

**Previous native render — current verification pending.**

The original trimmed face and retained cutter remain visible.

![Native renderer output for measure original closed-solid volume.](../screenshots/journey/83za-visit-original-mesh-triangles-and-format-measures.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.3.2 · Visit original mesh triangles and format measures](83za-visit-original-mesh-triangles-and-format-measures.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original geometry → placed source → measure or refusal.



Why choose an origin on the mesh?

Subtracting a nearby origin keeps large world translations out of the triple products and preserves volume precision.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the far-translated box and reversed-face fixtures. Compare an open mesh refusal with a geometry kind that has no volume.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.1
npm --prefix ../session_tests run course -- save 16.4.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding face-split contract remains authored and unrun. The final measurement contract adds actual point input, numeric values, transient annotations, Escape and one reversible original bounding box. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.4.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
