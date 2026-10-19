# 14.4.18 · Build original cylinders with signed height

**Chapter 14 · Coordinates and geometry tools · Section 14.4 · Polyhedra and round solids** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Build cylinders with positive or negative height using the solid frame and radius checks already taught.

## Type

Continue from [Exercise current solid input and original persistence](74p-exercise-current-solid-input-and-original-persistence.md). [Save or recover your work](recovery.md).

### 1. `src/round_cylinder.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/74za-build-original-cylinders-with-signed-height-typed-round_cylinder.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_cylinder.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cylinder",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Base center, radius, height: the questions of a cylinder or cone.
pub fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Base center")),
        1 => Some(Ask::size("Radius")),
        2 => Some(Ask::height("Height")),
        _ => None,
    }
}

/// Radius in the plane and signed height.
pub fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes.push(positive(frame.size(radius), "The radius")?);
    }

    if let Some(height) = answers.get(2) {
        part.sizes
            .push(nonzero(frame.height(height), "The height")?);
    }

    Ok(part)
}

/// The base ring, then the top ring and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let mut wires = vec![frame.ring(radius, 0.0), frame.ring(radius, height)];

            for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
                wires.push(vec![
                    frame.at(u * radius, v * radius, 0.0),
                    frame.at(u * radius, v * radius, height),
                ]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cylinder on the base, upside down for a negative height.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cylinder needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let brep = BRep::create_cylinder(radius, height);
    Ok(shape::solid(brep, option, &frame.to_xform(), "cylinder"))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/74za-build-original-cylinders-with-signed-height-typed-round_cylinder-fixtures.rs" skip=2194
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod primitive_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-02.rs" skip=34
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.4.18
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The cylinder builder retains signed height; its browser command connection follows later.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build original cylinders with signed height.](../screenshots/journey/74za-build-original-cylinders-with-signed-height.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Reuse the checked solid frame and radius helpers already taught in 73h–74. Collect a center, radius and signed height, draw the original pending rings and side guides, then build the chosen kernel representation. Keep a negative height in the frame orientation instead of inventing a second shape.

existing questions → checked local frame → exact kernel solid.



Does a negative height require a second shape definition?

No. Orient the existing checked frame along the signed height while keeping the original base location.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace heights 2 and -2 through one cylinder definition. Keep its base location unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.4.18
npm --prefix ../session_tests run course -- save 14.4.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original builders are preparatory until the existing round command registry connects them. The earlier solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
