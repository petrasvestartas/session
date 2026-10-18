# 14.5.3 · Mesh original quad-sphere patches

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

Ask the kernel for six original quad-sphere patches. Sample each patch with the existing quad mesher, then reuse the welding helper from 73h–74 to connect their seams. Transform the closed mesh through the checked frame and retain the mesh-only representation.

## Type

Continue from [Build original capsules with shared dimensions](75aa-build-original-capsules-with-shared-dimensions.md). [Save or recover your work](recovery.md).

### 1. `src/round_quad_sphere.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_quad_sphere.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ MESH_ONLY, Part, Shape};
use session_rust::{Geometry, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Quad Sphere",
    options: MESH_ONLY,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

const QUADS: usize = 8; // per patch side

/// Six kernel patches meshed and welded into one closed mesh.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Quad Sphere needs a radius".into());
    };
    let patches: Vec<_> = Primitives::quad_sphere(0.0, 0.0, 0.0, radius)
        .iter()
        .map(|patch| Primitives::quad_mesh(patch, QUADS, QUADS))
        .collect();
    let mut mesh = shape::to_mesh(&patches);
    mesh.transform(&part.frame.to_xform());
    mesh.name = "quad sphere".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere-fixtures.rs" skip=987
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_capsule;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-04.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

Original Box, Pyramid and Block With Hole remain visible.

![Native renderer output for mesh original quad-sphere patches.](../screenshots/journey/74za-build-original-cylinders-with-signed-height.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.4.18 · Build original cylinders with signed height](74za-build-original-cylinders-with-signed-height.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



existing questions → checked local frame → exact kernel solid.



Why weld the six patch meshes?

Adjacent patches meet along common seams. Reuse the existing weld helper so the result shares vertices and forms one closed mesh.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Count six patches with eight by eight quads each. Trace shared seam vertices through the existing weld helper.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.3
npm --prefix ../session_tests run course -- save 14.5.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding solid browser contract remains authored and unrun. The final round contract adds current representations, invalid torus input, correction, cancellation, Undo/Redo and the six round families. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.5.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
