# 10.3.22 · Build world-text color and ID pipelines

**Chapter 10 · Text and labels · Section 10.3 · Original scene labels** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

The plane shader reads coverage while color and integer targets retain their distinct fragment contracts. Declare the same vertex layout for both passes.

## Type

Continue from [Build the world-plane text quad](62ik-build-the-world-plane-text-quad.md). [Save or recover your work](recovery.md).

### 1. `src/text_plane.rs`

Add this part of build world-text color and id pipelines.

<details>
<summary>Locate the existing block</summary>

```rust
        ]);
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/62il-build-world-text-color-and-id-pipelines-01.rs" skip=20
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 10.3.22
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

World-text pipelines share placement and coverage while keeping color and ID targets distinct. The browser frame connection follows in [62j](62j-frame-pass.md).

### Recorded visual result

**Previous native render — current verification pending.**

The CAD bore and real-font screen specimen remain visible; this preparation adds no interactive scene-text example yet.

![Native renderer output for build world-text color and id pipelines.](../screenshots/journey/62i-coverage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



build world-text color and id pipelines.



Which contracts must the world-text color and ID passes share?

They share vertex placement and coverage rejection. The color pass blends visible coverage; the ID pass writes the exact original owner.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the world-plane color and ID pipelines. Find the shared coverage cutoff and their depth contracts in the already-written shaders.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 10.3.22
npm --prefix ../session_tests run course -- save 10.3.22
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The world-plane pipelines share the already-authored coverage and clipping contracts.

Compile the added build world-text color and id pipelines code for WebAssembly; compare the declared preparation frame.

Preparation: the original browser glyph reference remains unchanged. The completed text lane connects at 62j.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 10.3.22
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
