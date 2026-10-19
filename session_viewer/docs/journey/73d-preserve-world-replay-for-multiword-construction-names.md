# 14.3.5 · Preserve world replay for multiword construction names

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 4–7 minutes.** [Estimate](typing-load.md).

Strip the complete tool name before replay and use the active frame for construction and temporary boundaries.

## Type

Continue from [Build original rectangle and polygon tool results](73c-build-original-rectangle-and-polygon-tool-results.md). [Save or recover your work](recovery.md).

### 1. `src/tool.rs`

Replay original world points independently of multiword names and derive temporary construction boundaries.

<details>
<summary>Locate the existing block</summary>

```rust
    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=command.split_once(' ').map_or("",|(_,text)|text);
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build(&world)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/73d-preserve-world-replay-for-multiword-construction-names-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.5
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

One original horizontal line, a two-segment polyline and its point marker remain visible.

![Native renderer output for preserve world replay for multiword construction names.](../screenshots/journey/72s-exercise-current-drawing-tools-and-original-persistence.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.2.20 · Exercise current drawing tools and original persistence](72s-exercise-current-drawing-tools-and-original-persistence.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



multiword name → explicit world points → active frame construction.

![Preserve world replay for multiword construction names.](../illustrations/73d-preserve-world-replay-for-multiword-construction-names.svg)

Can splitting at the first space recover a Rectangle command?

No. Polyline Rectangle is a multiword name. Remove its full name before parsing the world-prefixed input points.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Finish a tilted Rectangle and verify its first original coordinate is still the accepted world point.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.5
npm --prefix ../session_tests run course -- save 14.3.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited drawing contract remains authored and unrun. The final planar contract adds rectangle and polygon options, Sides, invalid input, Undo/Redo and original coordinates; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.3.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
