# 10.3.24 · Rebuild only invalid world-text cache entries

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Match a label to its cache entry, validate the total texture budget before upload, and replace or append one owned entry.

## Type

Continue from [Own the world-text buffer and pipeline lifetime](62im-own-the-world-text-buffer-and-pipeline-lifetime.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of rebuild only invalid world-text cache entries.

<details>
<summary>Locate the existing block</summary>

```rust
        total
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62in-rebuild-only-invalid-world-text-cache-entries-01.rs" skip=22
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.24
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Only invalid label cache entries require replacement coverage. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for rebuild only invalid world-text cache entries.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



rebuild only invalid world-text cache entries.



When does a cached label need a new GPU texture?

When its font, text or required raster no longer matches the entry. Check the aggregate texture budget before replacing or appending GPU storage.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict cache hits for repeated frames, a font revision, and a changed label. Trace the byte budget check before texture allocation.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.24
npm --prefix ../session_tests run course -- save 10.3.24
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

cached_plane checks font, label and scale witnesses before replacing a retained texture.

Compile the added rebuild only invalid world-text cache entries code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.24
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
