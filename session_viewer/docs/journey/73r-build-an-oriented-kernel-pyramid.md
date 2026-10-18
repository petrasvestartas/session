# 14.3.19 · Build an oriented kernel pyramid

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 28–55 minutes.** [Estimate](typing-load.md).

Read a base diagonal and signed height. Use the shared upright frame for the kernel pyramid and draw the matching temporary edges.

## Type

Continue from [Read box questions and corner clicks](73q-read-box-questions-and-corner-clicks.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_pyramid.rs`

Type the kernel builder and reuse the shared original-coordinate helpers.

Create the file and type:

```rust
--8<-- "journey/code/73r-build-an-oriented-kernel-pyramid-1.rs"
```

### 2. `src/lib.rs`

Register the completed original solid module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_box;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73r-build-an-oriented-kernel-pyramid-2.rs" skip=24
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_pyramid.rs`

Supply the existing accepted-solid validation fixture separately.

<details>
<summary>Locate the existing block</summary>

```text
/// A kernel pyramid on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [edge, height] = part.sizes[..] else {
        return Err("Pyramid needs an edge and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_pyramid(edge, height),
        option,
        &frame.to_xform(),
        "pyramid",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73r-build-an-oriented-kernel-pyramid-3.rs" skip=433
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.19
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

![Native renderer output for build an oriented kernel pyramid.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Build an oriented kernel pyramid → Session kernel geometry → existing preview and placement helpers.



Why make the height positive before building?

The flipped frame carries the downward direction, so the kernel receives a positive size without losing placement.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the pyramid below the Top plane with one along the Front plane. Predict the apex before checking the supplied fixture.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.19
npm --prefix ../session_tests run course -- save 14.3.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This writes original solid builders before the later creation-tool registration. The current browser scene remains unchanged.



[Full validation scope](release.md).

</details>
