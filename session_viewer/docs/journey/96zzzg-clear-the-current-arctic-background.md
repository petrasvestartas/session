# 20.8.1 · Clear the current Arctic background

**Chapter 20 · Shared GPU rendering · Section 20.8 · Background and shared object records** · [Contents](contents.md)

**Typing: 1–2 minutes.** [Estimate](typing-load.md).

Change the existing render-pass clear to linear grey 0.94 when Arctic is enabled. Keep the ordinary background when it is disabled. The sRGB target converts that linear value when storing pixels; do not put its already encoded byte value into the clear colour.

## Type

Continue from [Project triangles at their physical addresses](96zzzf-project-triangles-at-their-physical-addresses.md). [Save or recover your work](recovery.md).

### 1. `src/renderer.rs`

Use the existing colour clear; no extra fullscreen pipeline is needed.

<details>
<summary>Locate the existing block</summary>

```rust
        let [r, g, b] = background.rgb();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/ambient_gpu_tests.rs`

Keep the native UNORM contact-shadow assertion below the linear grey background, so the background itself cannot satisfy it.

<details>
<summary>Locate the existing block</summary>

```text
a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<250)
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/ambient_gpu_tests.rs`

Measure surface darkening only on geometry pixels; the changed background is not ambient shading.

<details>
<summary>Locate the existing block</summary>

```text
filter(|(a,b)|a[..3].iter().zip(&b[..3]).any
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzg-clear-the-current-arctic-background-03.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.8.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Arctic uses the current slightly grey background; ordinary display returns to its previous background.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for clear the current arctic background.](../screenshots/journey/96zzzg-clear-the-current-arctic-background.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



Arctic setting → existing colour attachment clear → sRGB target → stored background pixel.



Why does a clear colour of 0.94 not store byte 240 in an sRGB target?

Clear values are linear. The sRGB attachment encodes them before storage, giving about byte 248. Encoding twice would give the wrong shade.

Study estimate, including typing and experiments: 1–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Toggle Arctic at both display densities. Check a background pixel, then confirm that object identities, placements and geometry uploads remain unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.8.1
npm --prefix ../session_tests run course -- save 20.8.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Current src/shaders/background.wgsl selects linear 0.94 when ambient lighting is enabled. Reuse the learner render-pass clear for the same background colour.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.8.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
