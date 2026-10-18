# 16.1.9 · Edit original curve intervals

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Trim or extend supported original curves by a normalized physical-length interval. Reuse the cutter and extension helpers already written.

## Type

Continue from [Dispatch original extension families](81zzg-dispatch-original-extension-families.md). [Save or recover your work](recovery.md).

### 1. `src/interval_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/81a-find-source-cutters-and-current-extension-reach-1.rs"
```

### 2. `src/lib.rs`

Register this original curve-editing owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod extend_reach;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/81a-find-source-cutters-and-current-extension-reach-2.rs" skip=23
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/interval_geometry.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Geometry,Line};
use std::rc::Rc;
pub const MAX_POINTS:usize=4096;
/// A part of the selected curve over 0..1 of its length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    Trim(f64, f64),   // keep this part
    Extend(f64, f64), // grow to this range
}

/// The source trimmed or extended to `a..b` of its length.
pub fn edited(source: &Geometry, interval: Interval) -> Result<Geometry, String> {
    let (a, b, trim) = match interval {
        Interval::Trim(a, b) => (a, b, true),
        Interval::Extend(a, b) => (a, b, false),
    };

    if !a.is_finite() || !b.is_finite() || a >= b || a.abs() > 1e6 || b.abs() > 1e6 {
        return Err("parameters must be finite, increasing, and within ±1e6".into());
    }

    if (trim && (a < 0.0 || b > 1.0)) || (!trim && (a > 0.0 || b < 1.0)) {
        return Err("trim keeps 0 <= a < b <= 1; extend needs a <= 0 and b >= 1".into());
    }

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&line.point_at(a), &line.point_at(b));
            next.name = line.name.clone();
            next.linecolor = line.linecolor.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.arrowhead = line.arrowhead;
            Ok(Geometry::Line(Rc::new(next)))
        }
        Geometry::NurbsCurve(curve) => {
            if curve.m_cv_count > MAX_POINTS {
                return Err(format!("curve edits are limited to {MAX_POINTS} controls"));
            }

            let mut next = (**curve).clone();
            let (lo, hi) = next.domain();
            let a = lo + a * (hi - lo); // 0..1 into the curve domain
            let b = lo + b * (hi - lo);
            let ok = if trim {
                next.trim(a, b)
            } else {
                next.extend(a, b)
            };

            if !ok {
                return Err("kernel refused this curve interval".into());
            }

            Ok(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => Err("trim and extend currently accept lines and NURBS curves".into()),
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/81a-find-source-cutters-and-current-extension-reach-3.rs" skip=2129
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The complete cutter/reach helpers compile before the pending edit uses them.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

The original network and four-corner patch remain visible.

![Native renderer output for edit original curve intervals.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why use physical-length fractions?

The requested interval is measured along the curve rather than through nonuniform parameters.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a trim, an extension and an unsupported family.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.9
npm --prefix ../session_tests run course -- save 16.1.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding network/patch contract remains authored and unrun. The final editing contract adds projected originals, atomic Three Point retry, interval extension and active distance/cancellation. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.1.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
