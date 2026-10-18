# 14.5.9 · Read original arcs from center and angle

**Chapter 14 · Coordinates and geometry tools · Section 14.5 · Tori, circles and conics** · [Contents](contents.md)

**Typing: 13–25 minutes.** [Estimate](typing-load.md).

Read the existing center-based arc inputs using the checked frame. Turn its first axis toward a clicked start, derive the clicked angle or accept typed degrees, and preserve the signed orientation through the original radius and angle rules.

## Type

Continue from [Build exact rational arc spans](75zc-build-exact-rational-arc-spans.md). [Save or recover your work](recovery.md).

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

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.5.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The center-and-angle arc reader compiles before its browser tool connection.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original arcs from center and angle.](../screenshots/journey/75za-build-original-planar-circles.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.5.6 · Build original planar circles](75za-build-original-planar-circles.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared questions and frame → original exact curve → derived preview.



Why turn the local frame toward the picked start?

Angle zero must lie at that original start. The same local arc builder can then place the result through the existing frame transform.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a clicked start and a typed clockwise angle. These input helpers remain preparatory until the recipe is complete.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.5.9
npm --prefix ../session_tests run course -- save 14.5.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original curve helpers are preparatory until the existing freeform command registry connects them. The earlier round-solid drawing stays unchanged.



[Full validation scope](release.md).

</details>
