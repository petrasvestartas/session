# 14.4.1 · Add the remaining regular kernel solids

**Chapter 14 · Coordinates and geometry tools · Section 14.4 · Polyhedra and round solids** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Add icosahedron and dodecahedron builders using the same radius reader, unit-face cache and placement helper. Only the kernel constructors and solid names change.

## Type

Continue from [Build regular kernel polyhedra](73t-build-regular-kernel-polyhedra.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_icosahedron.rs`

Type the regular-solid wrapper using the existing common helpers.

Create the file and type:

```rust
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-1.rs"
```

### 2. `src/lib.rs`

Register this completed kernel solid.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_octahedron;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-2.rs" skip=31
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_icosahedron.rs`

Supply the original kernel-solid validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "icosahedron")
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-3.rs" skip=190
```

</details>

### 4. `src/primitive_dodecahedron.rs`

Type the regular-solid wrapper using the existing common helpers.

Create the file and type:

```rust
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-4.rs"
```

### 5. `src/lib.rs`

Register this completed kernel solid.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_icosahedron;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-5.rs" skip=32
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/primitive_dodecahedron.rs`

Supply the original kernel-solid validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "dodecahedron")
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/74-copy-current-solid-geometry-and-question-definitions-6.rs" skip=191
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.4.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The original rectangle and polygon remain visible while current solid tools connect. The completed exercise creates a Box, Pyramid and Block With Hole.

### Recorded visual result

**Previous native render — current verification pending.**

A closed original rectangle and an oriented five-sided original polygon remain visible.

![Native renderer output for add the remaining regular kernel solids.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



kernel regular-solid constructor → shared radius normalization → existing frame and solid conversion.



Which parts of the regular-solid pipeline change for a different polyhedron?

The kernel constructor, name and cache identity change. Radius questions, normalization, placement and BRep conversion stay shared.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the accepted icosahedron and dodecahedron face counts. Trace their shared radius normalization without adding a second algorithm.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.4.1
npm --prefix ../session_tests run course -- save 14.4.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The original solid family is now written through shared Session kernel helpers. Later lessons register these builders with browser creation tools.

Supplied fixtures check kernel topology, radius and construction-plane placement.

The inherited planar contract remains authored and unrun. The final solid contract adds typed/clicked dimensions, choices/defaults, atomic invalid input, original creation, Undo/Redo, temporary previews and current quick-click drift; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.4.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
