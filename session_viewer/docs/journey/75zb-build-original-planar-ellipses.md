# 14.5.7 · Build original planar ellipses

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Reuse the same checked frame and radius owner. Orient the first ellipse axis from a clicked point, measure the second radius across that axis, and transform the exact kernel ellipse into the drawing plane. Numeric answers retain the existing axes.

## Type

Continue from [Build original planar circles](75za-build-original-planar-circles.md). [Save or recover your work](recovery.md).

### 1. `src/freeform_ellipse.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_circle;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-2.rs" skip=26
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_ellipse.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, CURVE, Frame, Part, Shape, positive};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Ellipse",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, the first axis end, then the second radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("First axis end or radius")),
        2 => Some(Ask::size("Second radius")),
        _ => None,
    }
}

/// A click turns the first axis toward it; the second radius is a click's distance from that axis.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(first) = answers.get(1) {
        if let Answer::Point(end) = first {
            let [u, v, _] = frame.local(end);
            part.frame = frame.turned(u, v);
        }

        part.sizes
            .push(positive(frame.size(first), "The first radius")?);
    }

    if let Some(second) = answers.get(2) {
        let radius = match second {
            Answer::Number(value) => *value,
            Answer::Point(p) => part.frame.local(p)[1].abs(),
        };
        part.sizes.push(positive(radius, "The second radius")?);
    }

    Ok(part)
}

/// The first axis, then the ellipse.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [first] => vec![vec![frame.at(-first, 0.0, 0.0), frame.at(first, 0.0, 0.0)]],
        [first, second] => vec![frame.oval(first, second, 0.0)],
        _ => Vec::new(),
    }
}

/// The kernel ellipse, its first radius along the first axis.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [first, second] = part.sizes[..] else {
        return Err("Nurbs Curve Ellipse needs two radii".into());
    };
    let mut curve = Primitives::ellipse(0.0, 0.0, 0.0, first, second);
    curve.transform(&part.frame.to_xform());
    curve.name = "ellipse".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75zb-build-original-planar-ellipses-3.rs" skip=2204
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The planar ellipse helper can use separate original radii before command input is connected.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build original planar ellipses.](../screenshots/journey/75za-build-original-planar-circles.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.5.6 · Build original planar circles](75za-build-original-planar-circles.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared questions and frame → original exact curve → derived preview.



Why measure the second clicked radius across the first axis?

Its perpendicular distance sets the second semiaxis. Its distance from the center would incorrectly include movement along the first axis.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a numeric first radius with a clicked first-axis endpoint. Trace the second click in the turned frame.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.7
npm --prefix ../session_tests run course -- save 14.5.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original curve helpers are preparatory until the existing freeform command registry connects them. The earlier round-solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
