# 14.4.19 · Build original cones and spheres

**Chapter 14 · Coordinates and geometry tools · Section 14.4 · Polyhedra and round solids** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Reuse the cylinder’s existing center, radius and height questions for a cone. Share the checked frame and preserve the cone tip and base ring. A sphere needs only its center and radius; ask the kernel for the chosen representation and validate the source geometry.

## Type

Continue from [Build original cylinders with signed height](74za-build-original-cylinders-with-signed-height.md). [Save or recover your work](recovery.md).

### 1. `src/round_cone.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/74zb-build-original-cones-and-spheres-typed-round_cone.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_cone.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use crate::round_cylinder as cylinder;
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cone",
    options: BREP_MESH,
    upfront: false,
    ask: cylinder::ask,
    read: cylinder::read,
    outline,
    build,
};

/// The base ring, then four lines to the apex.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let apex = frame.at(0.0, 0.0, height);
            let mut wires = vec![frame.ring(radius, 0.0)];

            for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
                wires.push(vec![frame.at(u * radius, v * radius, 0.0), apex.clone()]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cone on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cone needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_cone(radius, height),
        option,
        &frame.to_xform(),
        "cone",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/74zb-build-original-cones-and-spheres-typed-round_cone-fixtures.rs" skip=1360
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_cylinder;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-04.rs" skip=25
```

### 4. `src/round_sphere.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/74zb-build-original-cones-and-spheres-typed-round_sphere.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/round_sphere.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use session_rust::{BRep, Geometry};

pub static SHAPE: Shape = Shape {
    name: "Sphere",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

/// A kernel sphere at the center, poles along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Sphere needs a radius".into());
    };
    Ok(shape::solid(
        BRep::create_sphere(radius),
        option,
        &part.frame.to_xform(),
        "sphere",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/74zb-build-original-cones-and-spheres-typed-round_sphere-fixtures.rs" skip=698
```

</details>

### 6. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_cone;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-06.rs" skip=21
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.4.19
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original cone and sphere builders compile while the existing solids remain visible.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build original cones and spheres.](../screenshots/journey/74za-build-original-cylinders-with-signed-height.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.4.18 · Build original cylinders with signed height](74za-build-original-cylinders-with-signed-height.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



existing questions → checked local frame → exact kernel solid.



Why reuse the cylinder questions for the cone?

They collect the same center, radius and signed height. Only the exact geometry builder and its pending outline change.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a cone and sphere at radius 0.5. Trace the shared questions and independent original output.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.4.19
npm --prefix ../session_tests run course -- save 14.4.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original builders are preparatory until the existing round command registry connects them. The earlier solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
