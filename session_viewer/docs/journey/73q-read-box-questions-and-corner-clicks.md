# 14.3.18 · Read box questions and corner clicks

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Connect the box builder to its question sequence. A corner click supplies two base dimensions, while typed dimensions ask for width separately.

## Type

Continue from [Build a placed kernel box](73p-build-a-placed-kernel-box.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_box.rs`

Insert the shape descriptor and question reader before the helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Mesh, Point, Xform};
use std::rc::Rc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73q-read-box-questions-and-corner-clicks-1.rs" skip=211
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/primitive_box.rs`

Supply the original corner, signed-height, orientation and invalid-size fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    Ok(shape::solid(
        BRep::create_box(length, width, height.abs()),
        option,
        &place,
        "box",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73q-read-box-questions-and-corner-clicks-2.rs" skip=133
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.18
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

![Native renderer output for read box questions and corner clicks.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Read box questions and corner clicks → Session kernel geometry → existing preview and placement helpers.



Why does one corner click supply two dimensions?

Its local u and v coordinates give half the centered base lengths; absolute values choose positive dimensions.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare Center, a corner click and Height with Center, Length, Width and Height. Check their accepted solids have the same corners.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.18
npm --prefix ../session_tests run course -- save 14.3.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This writes original solid builders before the later creation-tool registration. The current browser scene remains unchanged.



[Full validation scope](release.md).

</details>
