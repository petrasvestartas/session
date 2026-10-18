# 10.3.25 · Prepare and draw the retained world-text planes

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 26–51 minutes.** [Estimate](typing-load.md).

Prune expired labels, skip clipped/empty planes, reuse valid rasters, build the current quads and issue draws from retained cache indices.

## Type

Continue from [Rebuild only invalid world-text cache entries](62in-rebuild-only-invalid-world-text-cache-entries.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of prepare and draw the retained world-text planes.

<details>
<summary>Locate the existing block</summary>

```rust
        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62io-prepare-and-draw-the-retained-world-text-planes-01.rs" skip=77
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.25
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Prepared draw indices reference the surviving retained text planes. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for prepare and draw the retained world-text planes.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



prepare and draw the retained world-text planes.



How do current draw indices refer to retained cache entries?

Preparation builds indices into the retained cache after pruning expired entries. Draw only those accepted indices for the current frame.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Follow one visible run from cache lookup to draw index. Explain why cache indices cannot be treated as original object IDs.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.25
npm --prefix ../session_tests run course -- save 10.3.25
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Planes::prepare constructs draw indices into retained coverage entries.

Compile the added prepare and draw the retained world-text planes code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.25
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
