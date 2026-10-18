# 14.2.1 · Place original coordinates in the selected plane

**Chapter 14 · Coordinates and geometry tools · Section 14.2 · Coordinates and drawing tools** · [Contents](contents.md)

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

Complete absolute coordinates, offsets and point placement with the original construction-plane rules. Reuse the existing parser and resolver, transform plane axes once, and honor the world prefix without changing the selected plane. The following input bridge connects these prepared helpers.

## Type

Continue from [Resolve original relative and polar coordinates](71ze-resolve-original-relative-and-polar-coordinates.md). [Save or recover your work](recovery.md).

### 1. `src/drawing_coords.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    let text = text.trim();

    match text.strip_prefix(['w', 'W']) {
        Some(rest) => Some((parse_plain(rest)?, true)),
        None => Some((parse_plain(text)?, false)),
    }
}
```

</details>

After that block, type:

```rust
/// A typed absolute point in the world: measured in the fixed construction plane, in world coordinates in View mode or after `w`.
pub fn absolute(text: &str) -> Option<[f64; 3]> {
    let (Typed::Absolute { x, y, z }, world) = parse_world(text)? else {
        return None;
    };
    let uvw = [x, y, z.unwrap_or(0.0)];

    match plane().filter(|_| !world) {
        Some(plane) => {
            let p = plane.world(uvw);
            Some([p[0], p[1], p[2]])
        }
        None => Some(uvw),
    }
}

/// A typed offset in the world: along the fixed construction plane's axes, along the world axes in View mode or after `w`.
pub fn offset(uvw: [f64; 3], world: bool) -> [f64; 3] {
    match plane().filter(|_| !world) {
        Some(plane) => {
            let (o, p) = (plane.origin(), plane.world(uvw));
            [p[0] - o[0], p[1] - o[1], p[2] - o[2]]
        }
        None => uvw,
    }
}

/// A typed coordinate as a point: in the fixed construction plane's coordinates from its origin, else as `resolve` places it on the drawing plane.
pub fn place(
    typed: Typed,
    world: bool,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    let Some(plane) = plane().filter(|_| !world) else {
        // world coordinates; a fixed plane's axes do not apply after `w`
        let (x, y) = match world && plane().is_some() {
            true => CPlane::Xy.axes(),
            false => (x_axis.clone(), y_axis.clone()),
        };
        return resolve(typed, &Point::new(0.0, 0.0, 0.0), &x, &y, previous, along);
    };
    // `base` plus an offset in plane coordinates
    let moved = |base: &Point, uvw: [f64; 3]| {
        let d = offset(uvw, false);
        Point::new(base[0] + d[0], base[1] + d[1], base[2] + d[2])
    };

    match typed {
        Typed::Absolute { x, y, z } => Some(plane.world([x, y, z.unwrap_or(0.0)])),
        Typed::Relative { x, y, z } => Some(moved(previous?, [x, y, z.unwrap_or(0.0)])),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.cloned().unwrap_or_else(|| plane.origin());
            Some(moved(&base, [distance * r.cos(), distance * r.sin(), 0.0]))
        }
        Typed::Distance(_) => resolve(typed, &plane.origin(), x_axis, y_axis, previous, along),
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/72-copy-current-construction-planes-and-coordinate-forms-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/drawing_coords.rs`

Supply only the unchanged original frame and coordinate validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::cplane::CPlane;
use session_rust::{Point, Vector};
use std::cell::Cell;

thread_local! {
    /// The fixed construction plane typed coordinates are measured in; None in View mode, where they are world coordinates.
    static PLANE: Cell<Option<CPlane>> = const { Cell::new(None) };
}

/// Fix the plane typed coordinates are measured in, None for world coordinates.
pub fn set_plane(plane: Option<CPlane>) {
    PLANE.with(|cell| cell.set(plane));
}

/// The fixed construction plane typed coordinates are measured in, if any.
pub fn plane() -> Option<CPlane> {
    PLANE.with(Cell::get)
}

/// A typed coordinate, not yet placed in the world.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Typed {
    Absolute { x: f64, y: f64, z: Option<f64> }, // `1,2,3` or `1,2` on the plane
    Relative { x: f64, y: f64, z: Option<f64> }, // `@1,2` from the previous point
    Polar { distance: f64, degrees: f64 },       // `5<45` in the plane
    Distance(f64),                               // `5` along the current direction
}

/// How the prompt names typed points: `x,y,z`, or plane coordinates while a construction plane is fixed.
pub fn hint() -> &'static str {
    match plane() {
        Some(_) => "plane x,y,z (w for world)",
        None => "x,y,z",
    }
}

/// Parse coordinate text, a leading `w` included; None when it is not a coordinate.
pub fn parse(text: &str) -> Option<Typed> {
    parse_world(text).map(|(typed, _)| typed)
}

/// Parse coordinate text and say whether a leading `w` asks for world coordinates, e.g. `w500,0,0` or `w@0,0,100`.
pub fn parse_world(text: &str) -> Option<(Typed, bool)> {
    let text = text.trim();

    match text.strip_prefix(['w', 'W']) {
        Some(rest) => Some((parse_plain(rest)?, true)),
        None => Some((parse_plain(text)?, false)),
    }
}

/// A typed absolute point in the world: measured in the fixed construction plane, in world coordinates in View mode or after `w`.
pub fn absolute(text: &str) -> Option<[f64; 3]> {
    let (Typed::Absolute { x, y, z }, world) = parse_world(text)? else {
        return None;
    };
    let uvw = [x, y, z.unwrap_or(0.0)];

    match plane().filter(|_| !world) {
        Some(plane) => {
            let p = plane.world(uvw);
            Some([p[0], p[1], p[2]])
        }
        None => Some(uvw),
    }
}

/// A typed offset in the world: along the fixed construction plane's axes, along the world axes in View mode or after `w`.
pub fn offset(uvw: [f64; 3], world: bool) -> [f64; 3] {
    match plane().filter(|_| !world) {
        Some(plane) => {
            let (o, p) = (plane.origin(), plane.world(uvw));
            [p[0] - o[0], p[1] - o[1], p[2] - o[2]]
        }
        None => uvw,
    }
}

/// A typed coordinate as a point: in the fixed construction plane's coordinates from its origin, else as `resolve` places it on the drawing plane.
pub fn place(
    typed: Typed,
    world: bool,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    let Some(plane) = plane().filter(|_| !world) else {
        // world coordinates; a fixed plane's axes do not apply after `w`
        let (x, y) = match world && plane().is_some() {
            true => CPlane::Xy.axes(),
            false => (x_axis.clone(), y_axis.clone()),
        };
        return resolve(typed, &Point::new(0.0, 0.0, 0.0), &x, &y, previous, along);
    };
    // `base` plus an offset in plane coordinates
    let moved = |base: &Point, uvw: [f64; 3]| {
        let d = offset(uvw, false);
        Point::new(base[0] + d[0], base[1] + d[1], base[2] + d[2])
    };

    match typed {
        Typed::Absolute { x, y, z } => Some(plane.world([x, y, z.unwrap_or(0.0)])),
        Typed::Relative { x, y, z } => Some(moved(previous?, [x, y, z.unwrap_or(0.0)])),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.cloned().unwrap_or_else(|| plane.origin());
            Some(moved(&base, [distance * r.cos(), distance * r.sin(), 0.0]))
        }
        Typed::Distance(_) => resolve(typed, &plane.origin(), x_axis, y_axis, previous, along),
    }
}

/// Parse coordinate text without the world prefix.
fn parse_plain(text: &str) -> Option<Typed> {
    let text = text.trim();

    if text.is_empty() {
        return None;
    }

    let (body, relative) = match text.strip_prefix('@') {
        Some(rest) => (rest.trim(), true),
        None => (text, false),
    };

    if let Some((d, a)) = body.split_once('<') {
        let distance = number(d)?;
        let degrees = number(a)?;
        return Some(Typed::Polar { distance, degrees });
    }

    let parts: Vec<&str> = body.split(',').map(str::trim).collect();

    match parts.len() {
        1 if relative => None, // `@5` has no direction
        1 => Some(Typed::Distance(number(parts[0])?)),
        2 | 3 => {
            let x = number(parts[0])?;
            let y = number(parts[1])?;
            let z = match parts.get(2) {
                Some(v) => Some(number(v)?),
                None => None,
            };
            Some(if relative {
                Typed::Relative { x, y, z }
            } else {
                Typed::Absolute { x, y, z }
            })
        }
        _ => None,
    }
}

/// A typed coordinate as a point on the plane.
pub fn resolve(
    typed: Typed,
    origin: &Point,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    // `base` plus u along x and v along y
    let on_plane = |u: f64, v: f64, base: &Point| {
        Point::new(
            base[0] + x_axis[0] * u + y_axis[0] * v,
            base[1] + x_axis[1] * u + y_axis[1] * v,
            base[2] + x_axis[2] * u + y_axis[2] * v,
        )
    };

    match typed {
        Typed::Absolute { x, y, z: Some(z) } => Some(Point::new(x, y, z)),
        Typed::Absolute { x, y, z: None } => Some(on_plane(x, y, origin)),
        Typed::Relative { x, y, z: Some(z) } => {
            let p = previous?;
            Some(Point::new(p[0] + x, p[1] + y, p[2] + z))
        }
        Typed::Relative { x, y, z: None } => Some(on_plane(x, y, previous?)),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.unwrap_or(origin);
            Some(on_plane(distance * r.cos(), distance * r.sin(), base))
        }
        Typed::Distance(d) => {
            let p = previous?;
            let v = along?;
            Some(Point::new(
                p[0] + v[0] * d,
                p[1] + v[1] * d,
                p[2] + v[2] * d,
            ))
        }
    }
}

/// A finite number, or None.
fn number(text: &str) -> Option<f64> {
    let value: f64 = text.trim().parse().ok()?;
    value.is_finite().then_some(value)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/72-copy-current-construction-planes-and-coordinate-forms-2.rs" skip=6855
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.2.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The original plan remains visible while the drawing tools gain current construction planes and typed coordinates. The completed exercise draws original points, lines and polylines.

### Recorded visual result

**Previous native render — current verification pending.**

Original black physical drawing pens and the blue hairline remain visible.

![Native renderer output for place original coordinates in the selected plane.](../screenshots/journey/71aj-exercise-complete-and-ranged-original-drawings.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original frame → parsed coordinates → world drawing point.



Why keep the world prefix local to one input?

It changes how that coordinate is interpreted. It should not change the construction plane used by later drawing inputs.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare plane coordinates with a world-prefixed input and trace a relative offset through the selected axes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.2.1
npm --prefix ../session_tests run course -- save 14.2.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited drawing/sheet contracts remain authored and unrun. The completed exercise adds fixed-plane input, original coordinate diagnostics, invalid drafts, Close, Undo/Redo and Save/Open; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.2.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
