# 14.3.21 · Build regular kernel polyhedra

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Connect tetrahedron and octahedron constructors to the shared radius questions, unit-face cache and placed builder. Reuse the same foundation for the remaining regular solids.

## Type

Continue from [Build a block with a kernel hole](73s-build-a-block-with-a-kernel-hole.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_tetrahedron.rs`

Type the kernel builder and reuse the shared original-coordinate helpers.

Create the file and type:

```rust
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-1.rs"
```

### 2. `src/lib.rs`

Register the completed original solid module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_block_with_hole;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-2.rs" skip=36
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_tetrahedron.rs`

Supply the existing accepted-solid validation fixture separately.

<details>
<summary>Locate the existing block</summary>

```text
/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "tetrahedron")
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-3.rs" skip=190
```

</details>

### 4. `src/primitive_octahedron.rs`

Type the kernel builder and reuse the shared original-coordinate helpers.

Create the file and type:

```rust
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-4.rs"
```

### 5. `src/lib.rs`

Register the completed original solid module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_tetrahedron;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-5.rs" skip=32
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/primitive_octahedron.rs`

Supply the existing accepted-solid validation fixture separately.

<details>
<summary>Locate the existing block</summary>

```text
/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "octahedron")
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73t-build-regular-kernel-polyhedra-6.rs" skip=189
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.21
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

![Native renderer output for build regular kernel polyhedra.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Build regular kernel polyhedra → Session kernel geometry → existing preview and placement helpers.



Why share the radius reader and polyhedron builder?

Their question and placement rules match; only the kernel constructor and cached face outlines differ.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare tetrahedron and octahedron corner counts. Explain why both use the same radius and frame rules.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.21
npm --prefix ../session_tests run course -- save 14.3.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This writes original solid builders before the later creation-tool registration. The current browser scene remains unchanged.



[Full validation scope](release.md).

</details>
