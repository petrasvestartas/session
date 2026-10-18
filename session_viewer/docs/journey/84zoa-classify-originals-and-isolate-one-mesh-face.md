# 16.4.34 · Classify originals and isolate one mesh face

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Name each original geometry kind once. Build one mesh face with compact vertex keys, preserve its hole rings and retain its display attributes.

## Type

Continue from [Connect window and bounds selection tests](84znf-connect-window-and-bounds-selection-tests.md). [Save or recover your work](recovery.md).

### 1. `src/source_shape.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-1.rs"
```

### 2. `src/lib.rs`

Compile the shared original geometry classifier.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod selection_region;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-2.rs" skip=27
```

### 3. `src/explode_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-3.rs"
```

### 4. `src/lib.rs`

Register this original command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod trim_parts;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/84zoa-classify-originals-and-isolate-one-mesh-face-4.rs" skip=21
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.34
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original-family classification and single mesh-face extraction are prepared.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for classify originals and isolate one mesh face.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why remap vertices for a separated face?

The new mesh owns a compact vertex table. All outer and hole rings must refer to that new table.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a face with a hole and a vertex shared by both rings. Check that the new mesh owns each copied vertex once.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.34
npm --prefix ../session_tests run course -- save 16.4.34
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
