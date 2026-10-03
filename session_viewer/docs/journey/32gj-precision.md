# 32gj · Prove restored Save keeps the source doubles

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 33 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Verify the automatic browser Save download retains exact source coordinates after display-only unloading.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Original f64 sentinel → derived f32 display → unload kernel → reload original bytes → Save → exact source doubles.

**Before you finish, explain:** Why is equal drawing insufficient proof that Save preserved source coordinates?

The display arrays are deliberately derived and may round source doubles to floats. precise_bytes adds one non-f32-representable coordinate to the uploaded specimen while retaining the ordinary specimen function. The browser and native checks must read this actual source rather than inventing an expected drawing.

The native acceptance test moves the object, unloads its editable source, completes a captured Save with the original bytes, and compares every original vertex. It also proves the sentinel would change through f32 and Save leaves the previous Move available to Undo and Redo.

Visible Chrome downloads a warm reference, unloads sources and downloads the automatic restored Save. A small test-only protobuf reader follows the documented Session → Objects → Mesh → vertex map fields, orders vertex keys and compares all double values with the actual uploaded file. It rejects a float-rounded substitute and checks both actual downloads, separate placement matrices, delayed download URL cleanup and Close releasing source URLs. This reader is verification tooling, not a second viewer loader.

The inherited automatic-command checks remain active. Broader network, body and multiple-origin failure acceptance follows next.

![Prove original-coordinate Save](../illustrations/journey-32gj.svg)

The final proof placement moves the post clear of the beam. Keeping two front faces in exactly the same plane can produce depth competition; this demonstration separates the solids instead of claiming the later rendering-quality lessons are already implemented.

## Type the change

Continue [Automatically restore sources for Move, Delete and Save](32gif-auto.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gj-precision`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/specimen.rs`

Add a source coordinate that cannot survive an f64-to-f32 round trip; keep the original specimen available.

Find this exact block:

```rust
pub fn bytes() -> Vec<u8> {
    use session_rust::{Color, Mesh, Session, Xform};
    let mut session = Session::new("Three-piece frame");
    for (name, size, centre) in [
        ("Left post", [0.25, 0.35, 1.0], [-0.65, 0.0, 0.5]),
        ("Right post", [0.25, 0.35, 1.0], [0.65, 0.0, 0.5]),
        ("Top beam", [1.8, 0.35, 0.25], [0.0, 0.0, 1.125]),
    ] {
        let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
        mesh.name = name.to_owned();
        mesh.transform(&Xform::translation(centre[0], centre[1], centre[2]));
        mesh.set_objectcolor(Color::new(0.75, 0.35, 0.1, 1.0));
        assert!(session.add_mesh(mesh, None).is_some());
    }
    session.pb_dumps()
}
```

Replace that block with:

```rust
--8<-- "journey/code/32gj-precision-01.rs"
```

### 2. `examples/sample.rs`

The browser’s actual uploaded sample now contains the precision sentinel.

Find this exact block:

```rust
viewer_journey::specimen::bytes()
```

Replace that block with:

```rust
--8<-- "journey/code/32gj-precision-02.rs"
```

### 3. `src/lib.rs`

Register the exact-coordinate restoration acceptance check.

Find this exact block:

```rust
mod reload_complete_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/32gj-precision-03.rs"
```

### 4. `src/reload_precision_tests.rs`

Verify exact source-coordinate restoration with a real precision sentinel and unchanged edit history.

Create the file and type:

```rust
--8<-- "journey/code/32gj-precision-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open sample.pb with Open Replace, Select Next, Move 0.35,0,0.25, View Isometric and Fit. Unload Sources, then type Move 0.25,0,0.15 without Reload Sources. The command restores its source and moves once; Undo restores the placement. Repeat with Delete and Undo, then Unload Sources and Save. Save downloads the editable document without creating an Undo step. The sample now includes an original double that differs from its f32 display value. Inspect the actual Save download with the precision checker. Type Orbit Up and Fit for the final proof view. Finish with Move 0,-0.5,0 and Fit to separate the moved post from the beam; the final proof view avoids coplanar overlapping faces.

**Actual Chrome screenshot.**

Chrome checks actual warm and cold-source Save downloads against the precision specimen, exact double coordinates, placement preservation, delayed download URL cleanup and source URL release on Close.

![Actual browser result: Prove restored Save keeps the source doubles.](../screenshots/journey/32gj-precision-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Replace snapshot geometry with the display floats. Predict why screenshots could still pass, then run the native and downloaded-coordinate checks.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The GPU display already rounds coordinates to f32. Two different f64 values may draw identically. Compare the actual downloaded source coordinates and include a value whose f32 round trip differs.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gj-precision
npm --prefix ../session_tests run course -- save 32gj-precision
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Original source geometry determines editable Save; equal rendered pixels cannot establish coordinate precision.

[Validation status and course release](release.md).
