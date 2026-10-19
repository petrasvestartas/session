# 14.3.20 · Build a block with a kernel hole

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Reuse the existing box questions and dimension reader. Add a bounded hole radius and place the Session kernel block in the same frame.

## Type

Continue from [Build an oriented kernel pyramid](73r-build-an-oriented-kernel-pyramid.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_block_with_hole.rs`

Type the kernel builder and reuse the shared original-coordinate helpers.

Create the file and type:

```rust
--8<-- "journey/code/73s-build-a-block-with-a-kernel-hole-1.rs"
```

### 2. `src/lib.rs`

Register the completed original solid module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_pyramid;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73s-build-a-block-with-a-kernel-hole-2.rs" skip=28
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_block_with_hole.rs`

Supply the existing accepted-solid validation fixture separately.

<details>
<summary>Locate the existing block</summary>

```text
/// A kernel block centered on the axis, standing on the plane, the hole along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [length, width, height, hole] = part.sizes[..] else {
        return Err("Block With Hole needs its sides and a hole radius".into());
    };
    let place = &part.frame.to_xform() * &Xform::translation(0.0, 0.0, height * 0.5);
    let brep = BRep::create_block_with_hole(length, width, height.abs(), hole);
    Ok(shape::solid(brep, option, &place, "block with hole"))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73s-build-a-block-with-a-kernel-hole-3.rs" skip=537
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.20
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

![Native renderer output for build a block with a kernel hole.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Build a block with a kernel hole → Session kernel geometry → existing preview and placement helpers.



Why reuse the box reader for the outer dimensions?

The block and box accept the same centered dimensions. Only the additional hole question and kernel builder differ.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a radius that fits with one exceeding half the smaller base dimension. Explain the refusal before the kernel call.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.20
npm --prefix ../session_tests run course -- save 14.3.20
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This writes original solid builders before the later creation-tool registration. The current browser scene remains unchanged.



[Full validation scope](release.md).

</details>
