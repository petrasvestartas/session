# 10.3.21 · Build the world-plane text quad

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Transform the original text plane into rebased GPU vertices. Keep original label placement and color independent of the generated quad.

## Type

Continue from [Upload and bind one world-text coverage texture](62ij-upload-and-bind-one-world-text-coverage-texture.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of build the world-plane text quad.

<details>
<summary>Locate the existing block</summary>

```rust
        bind,
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62ik-build-the-world-plane-text-quad-01.rs" skip=22
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.21
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The world-text quad derives its vertices from the original label plane. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for build the world-plane text quad.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



build the world-plane text quad.



Which original data does the generated world-text quad retain?

The original label supplies placement, size and color. The quad is a derived render input, not a replacement label or new CAD object.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the four original world corners into the generated six-vertex quad. Identify which values depend on camera origin.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.21
npm --prefix ../session_tests run course -- save 10.3.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

append_quad supplies geometry for the original world-label extent.

Compile the added build the world-plane text quad code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.21
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
