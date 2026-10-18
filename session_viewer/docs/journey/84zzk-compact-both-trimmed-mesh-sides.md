# 16.4.25 · Compact both trimmed mesh sides

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Compact each clipped mesh side to only its used vertices, rewrite its faces and preserve source name and color. Return both sides and their cut segments.

## Type

Continue from [Split mesh faces at a plane](84zzj-split-mesh-faces-at-a-plane.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

<details>
<summary>Locate the existing block</summary>

```rust
    Err("Mesh table reconstruction is not connected yet".into())
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzk-compact-both-trimmed-mesh-sides-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.25
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

![Native renderer output for compact both trimmed mesh sides.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why build a separate vertex map for each side?

Each output mesh owns only the vertices its own faces reference.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a cut vertex used by both halves and an original vertex used by just one.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.25
npm --prefix ../session_tests run course -- save 16.4.25
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
