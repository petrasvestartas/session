# 14.5 · Round solids and exact planar conics

Build checked round kernel solids and preserve circles, ellipses and arcs as original rational curves.

## Ownership and data flow

Round recipes share dimensions and a right-handed placement frame while retaining signed-height semantics and checked radii. Tori, capsules and mesh-only quad-sphere patches keep their original geometry family distinction. Curve recipes then form exact rational circle/ellipse/arc spans; centre-angle and three-point arc inputs validate their frame and degeneracy before a source exists. Preview samples remain derived and cannot replace the rational source. Original ownership/history connects through the same answer machinery as other shapes.

## The maintained implementation

app/command/tool/shape.rs supplies maintained round and conic recipes; verbs/torus.rs, capsule.rs, cylinder.rs and cone.rs register families, with verb-specific exact curve builders for circle/ellipse/arc. session_rust NurbsCurve and BRep/Mesh constructors remain authoritative original outputs. Parabola helpers prepare the final recipe connection next.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/walk/sheet.rs](../../../src/app/walk/sheet.rs), [app/sheet_query.rs](../../../src/app/sheet_query.rs), [app/stream.rs](../../../src/app/stream.rs), [app/cplane.rs](../../../src/app/cplane.rs), [app/coords.rs](../../../src/app/coords.rs), [app/command/tool/shape.rs](../../../src/app/command/tool/shape.rs), [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs), [app/deform.rs](../../../src/app/deform.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-75-reuse-current-round-solid-builders"></a>

<details>
<summary>Implementation 75-reuse-current-round-solid-builders: Build original tori with checked radii</summary>

Collect a center and two radii with the existing question owner.

Data flow: existing questions → checked local frame → exact kernel solid.

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

</details>

<a id="checkpoint-75aa-build-original-capsules-with-shared-dimensions"></a>

<details>
<summary>Implementation 75aa-build-original-capsules-with-shared-dimensions: Build original capsules with shared dimensions</summary>

Reuse the cylinder’s existing center, radius and signed-height questions.

Data flow: existing questions → checked local frame → exact kernel solid.

### 1. `src/round_capsule.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_capsule.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ MESH_ONLY, Part, Shape};
use crate::round_cylinder as cylinder;
use session_rust::{Geometry, Line, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Capsule",
    options: MESH_ONLY,
    upfront: false,
    ask: cylinder::ask,
    read,
    outline,
    build,
};

/// The cylinder's radius and height, taller than the two caps.
fn read(frame: &shape::Frame, answers: &[shape::Answer], option: &str) -> Result<Part, String> {
    let part = cylinder::read(frame, answers, option)?;

    if let [radius, height] = part.sizes[..]
        && height.abs() <= 2.0 * radius
    {
        return Err("The height must exceed the diameter".into());
    }

    Ok(part)
}

/// The base ring, then the rings where the caps begin and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius, height] = part.sizes[..] else {
        return part
            .sizes
            .first()
            .map_or_else(Vec::new, |radius| vec![part.frame.ring(*radius, 0.0)]);
    };
    let (frame, height) = part.frame.upright(height);
    let (low, high) = (radius, height - radius);
    let mut wires = vec![frame.ring(radius, low), frame.ring(radius, high)];

    for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
        wires.push(vec![
            frame.at(0.0, 0.0, 0.0),
            frame.at(u * radius, v * radius, low),
            frame.at(u * radius, v * radius, high),
            frame.at(0.0, 0.0, height),
        ]);
    }

    wires
}

/// The kernel capsule on the axis, its caps ending at the base and the height.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Capsule needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let axis = Line::from_points(
        &Point::new(0.0, 0.0, radius),
        &Point::new(0.0, 0.0, height - radius),
    );
    let mut mesh = Primitives::capsule_mesh(&axis, radius);
    mesh.transform(&frame.to_xform());
    mesh.name = "capsule".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule-fixtures.rs" skip=2191
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_torus;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-02.rs" skip=22
```

</details>

<a id="checkpoint-75a-retain-mesh-only-round-families"></a>

<details>
<summary>Implementation 75a-retain-mesh-only-round-families: Mesh original quad-sphere patches</summary>

Ask the kernel for six original quad-sphere patches.

Data flow: existing questions → checked local frame → exact kernel solid.

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

</details>

<a id="checkpoint-75b-register-current-round-names-and-choices"></a>

<details>
<summary>Implementation 75b-register-current-round-names-and-choices: Register current round names and choices</summary>

Register current round names and choices.

Data flow: registered verb and option → shared questions → one original commit.

### 1. `src/primitive_registry.rs`

Expose every current round verb and only its supported representation choices.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Dodecahedron Brep",&crate::primitive_dodecahedron::SHAPE,0),
    ("Dodecahedron Mesh",&crate::primitive_dodecahedron::SHAPE,1),
    ("bo",&crate::primitive_box::SHAPE,0),
];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75b-register-current-round-names-and-choices-01.rs"
```

</details>

<a id="checkpoint-75c-exercise-round-originals-and-one-transaction"></a>

<details>
<summary>Implementation 75c-exercise-round-originals-and-one-transaction: Exercise round originals and one transaction</summary>

Exercise round originals and one transaction.

Data flow: current round tool → original geometry → Undo/Redo → exact archive identity.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/round_tests.rs`

Copy integrated current round representation, frame, preview, invalid-input, history and archive identity fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the maintained round component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod round_quad_sphere;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-02.rs" skip=28
```

</details>

</details>

<a id="checkpoint-75za-build-original-planar-circles"></a>

<details>
<summary>Implementation 75za-build-original-planar-circles: Build original planar circles</summary>

Reuse the checked frame and positive-radius helpers from 73h–74.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_circle.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod round_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-2.rs" skip=30
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_circle.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, CURVE, Frame, Part, Shape, positive};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Circle",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, then radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Radius")),
        _ => None,
    }
}

/// The radius in the plane.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes.push(positive(frame.size(radius), "The radius")?);
    }

    Ok(part)
}

/// The circle.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    part.sizes
        .iter()
        .map(|radius| part.frame.ring(*radius, 0.0))
        .collect()
}

/// The kernel circle, placed in the plane.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Nurbs Curve Circle needs a radius".into());
    };
    let mut curve = Primitives::circle(0.0, 0.0, 0.0, radius);
    curve.transform(&part.frame.to_xform());
    curve.name = "circle".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75za-build-original-planar-circles-3.rs" skip=1409
```

</details>

</details>

<a id="checkpoint-75zb-build-original-planar-ellipses"></a>

<details>
<summary>Implementation 75zb-build-original-planar-ellipses: Build original planar ellipses</summary>

Reuse the same checked frame and radius owner.

Data flow: shared questions and frame → original exact curve → derived preview.

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

</details>

<a id="checkpoint-75zc-build-exact-rational-arc-spans"></a>

<details>
<summary>Implementation 75zc-build-exact-rational-arc-spans: Build exact rational arc spans</summary>

Build the original circular arc from rational quadratic spans.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_ellipse;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-2.rs" skip=27
```

</details>

<a id="checkpoint-75zd-read-original-arcs-from-center-and-angle"></a>

<details>
<summary>Implementation 75zd-read-original-arcs-from-center-and-angle: Read original arcs from center and angle</summary>

Read the existing center-based arc inputs using the checked frame.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::primitive_shape::{ Answer, Ask, Frame, Part, RING, Shape, positive};
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zd-read-original-arcs-from-center-and-angle-1.rs" skip=147
```

</details>

<a id="checkpoint-75ze-read-three-point-arcs-and-connect-the-recipe"></a>

<details>
<summary>Implementation 75ze-read-three-point-arcs-and-connect-the-recipe: Read three-point arcs and connect the recipe</summary>

Complete the existing arc recipe with its two input modes.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/freeform_arc.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, Frame, Part, RING, Shape, positive};
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Arc",
    options: &[
        ("Center", "Center"),
        ("3 Points", "3Points"),
        ("Cancel", "Escape"),
    ],
    upfront: true,
    ask,
    read,
    outline,
    build,
};

/// Center, start, end; or start, end, a point on the arc.
fn ask(answers: &[Answer], _part: &Part, option: &str) -> Option<Ask> {
    match (option, answers.len()) {
        ("Center", 0) => Some(Ask::point("Center")),
        ("Center", 1) => Some(Ask::size("Start or radius")),
        ("Center", 2) => Some(Ask::size("End or angle in degrees")),
        (_, 0) => Some(Ask::point("Start")),
        (_, 1) => Some(Ask::size("End or chord length")),
        (_, 2) => Some(Ask::size("Point on the arc or bulge")),
        _ => None,
    }
}

/// The circle's frame, x toward the start, and the radius and angle in degrees counterclockwise.
fn read(frame: &Frame, answers: &[Answer], option: &str) -> Result<Part, String> {
    match option {
        "Center" => read_center(frame, answers),
        _ => read_points(frame, answers),
    }
}

/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
    Ok(part)
}

/// The circle through start, a point on it and end, running from start past that point.
fn read_points(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };
    let start = frame.origin.clone();
    let end = match end {
        Answer::Number(length) => frame.at(*length, 0.0, 0.0),
        Answer::Point(end) => end.clone(),
    };
    let chord = positive(start.distance(&end, None), "The chord")?;
    let [u, v, _] = frame.local(&end);
    part.frame = frame.turned(u, v);
    part.sizes.push(chord);
    let Some(middle) = answers.get(2) else {
        return Ok(part);
    };
    let middle = match middle {
        Answer::Number(bulge) => part.frame.at(chord * 0.5, *bulge, 0.0), // to the left of start → end
        Answer::Point(p) => p.clone(),
    };
    let (ab, ac) = (&end - &start, &middle - &start);
    let normal = ac.cross(&ab); // start, middle, end run counterclockwise about it

    if normal.magnitude() <= 1e-9 * ab.magnitude() * ac.magnitude() {
        return Err("The three points are in a line".into());
    }

    let offset = &(&normal.cross(&ab) * ac.dot(&ac)) + &(&ac.cross(&normal) * ab.dot(&ab));
    let center = &start - (&offset * (0.5 / normal.dot(&normal)));
    let radius = center.distance(&start, None);
    let x = (&start - &center).normalized();
    let z = normal.normalized();
    let circle = Frame {
        origin: center,
        y: z.cross(&x),
        x,
        z,
    };
    let [u, v, _] = circle.local(&end);
    let degrees = v.atan2(u).to_degrees();
    part.frame = circle;
    part.sizes = vec![
        radius,
        if degrees <= 0.0 {
            degrees + 360.0
        } else {
            degrees
        },
    ];
    Ok(part)
}

/// The radius or chord as a line, then the arc.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [length] => vec![vec![frame.at(0.0, 0.0, 0.0), frame.at(length, 0.0, 0.0)]],
        [radius, degrees] => {
            let count = ((RING as f64 * degrees / 360.0).ceil() as usize).max(2);
            let arc = (0..=count)
                .map(|i| {
                    let angle = (degrees * i as f64 / count as f64).to_radians();
                    frame.at(radius * angle.cos(), radius * angle.sin(), 0.0)
                })
                .collect();
            vec![arc]
        }
        _ => Vec::new(),
    }
}

/// The exact arc, placed in its circle's frame.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, degrees] = part.sizes[..] else {
        return Err("Nurbs Curve Arc needs a radius and an angle".into());
    };
    let mut curve = arc(radius, degrees);
    curve.transform(&part.frame.to_xform());
    curve.name = "arc".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}

/// A circular arc about z from angle 0 to `degrees`, one rational quadratic span per 90° or less.
pub fn arc(radius: f64, degrees: f64) -> NurbsCurve {
    let spans = ((degrees / 90.0 - 1e-9).ceil() as usize).max(1);
    let step = degrees.to_radians() / spans as f64;
    let weight = (step * 0.5).cos(); // of each span's middle control
    let mut curve = NurbsCurve::new(3, true, 3, 2 * spans + 1);

    for i in 0..2 * spans + 2 {
        curve.set_nurbsknot(i, (i / 2) as f64);
    }

    for i in 0..=2 * spans {
        let angle = step * 0.5 * i as f64;
        let w = if i % 2 == 1 { weight } else { 1.0 };
        let distance = radius / w; // the middle control lies outside the circle
        curve.set_cv_4d(
            i,
            distance * angle.cos() * w,
            distance * angle.sin() * w,
            0.0,
            w,
        );
    }

    curve
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-2.rs" skip=6255
```

</details>

</details>

<a id="checkpoint-75zf-build-original-parabola-previews-and-sources"></a>

<details>
<summary>Implementation 75zf-build-original-parabola-previews-and-sources: Build original parabola previews and sources</summary>

Read the prepared start, apex and end from the checked local frame.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_parabola.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_arc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-2.rs" skip=23
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 75zf-build-original-parabola-previews-and-sources
npm --prefix ../session_tests run course -- check 75zf-build-original-parabola-previews-and-sources
npm --prefix ../session_tests run course -- save 75zf-build-original-parabola-previews-and-sources
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The round tools connected at checkpoint 75c-exercise-round-originals-and-one-transaction remain available: build a round source, Undo/Redo and reopen its archive to compare identity. Circle/ellipse/arc and parabola helpers added afterward prepare exact rational sources and derived preview parameters; their complete command recipe connects in the next lesson. Evaluate a parabola midpoint against its apex without replacing the source with preview samples.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Exact parabola recipes and editable freeform controls](76-copy-exact-planar-curve-definitions.md).
