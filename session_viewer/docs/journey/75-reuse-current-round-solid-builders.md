# 14.5.1 · Build original tori with checked radii

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Collect a center and two radii with the existing question owner. Reject an invalid radius pair before building the torus. Draw its pending rings using the checked frame, then ask the kernel for the chosen original representation.

## Type

Continue from [Build original cones and spheres](74zb-build-original-cones-and-spheres.md). [Save or recover your work](recovery.md).

### 1. `src/round_torus.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_torus.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, positive,
};
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Torus",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, major radius, then minor radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Major radius")),
        2 => Some(Ask::size("Minor radius")),
        _ => None,
    }
}

/// The minor radius is a click's distance from the major circle, below the major radius.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(major) = answers.get(1) {
        part.sizes
            .push(positive(frame.size(major), "The major radius")?);
    }

    if let Some(minor) = answers.get(2) {
        let major = part.sizes[0];
        let minor = match minor {
            Answer::Number(value) => *value,
            Answer::Point(_) => (frame.size(minor) - major).abs(),
        };

        if minor >= major {
            return Err("The minor radius must be below the major radius".into());
        }

        part.sizes.push(positive(minor, "The minor radius")?);
    }

    Ok(part)
}

/// The major circle, then the inner, outer, upper and lower rings.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [major] => vec![frame.ring(major, 0.0)],
        [major, minor] => vec![
            frame.ring(major - minor, 0.0),
            frame.ring(major + minor, 0.0),
            frame.ring(major, minor),
            frame.ring(major, -minor),
        ],
        _ => Vec::new(),
    }
}

/// A kernel torus about the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [major, minor] = part.sizes[..] else {
        return Err("Torus needs two radii".into());
    };
    Ok(shape::solid(
        BRep::create_torus(major, minor),
        option,
        &part.frame.to_xform(),
        "torus",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus-fixtures.rs" skip=2228
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_sphere;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-08.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The original Box, Pyramid and Block With Hole remain visible while the round family connects. The final exercise displays a Cylinder, Cone, Sphere and Torus.

### Recorded visual result

**Previous native render — current verification pending.**

Original Box, Pyramid and Block With Hole remain visible.

![Native renderer output for build original tori with checked radii.](../screenshots/journey/74za-build-original-cylinders-with-signed-height.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.4.18 · Build original cylinders with signed height](74za-build-original-cylinders-with-signed-height.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



existing questions → checked local frame → exact kernel solid.



Why check both torus radii before building?

Reject an invalid ring before publishing any source geometry or changing undo history. The existing question owner can retain the answers for correction.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a valid torus radius pair and an invalid pair. Compare the pending guide with the final original surface.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.1
npm --prefix ../session_tests run course -- save 14.5.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding solid browser contract remains authored and unrun. The final round contract adds current representations, invalid torus input, correction, cancellation, Undo/Redo and the six round families. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.5.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
