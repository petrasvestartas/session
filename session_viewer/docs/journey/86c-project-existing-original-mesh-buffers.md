# 17.1.7 · Assemble and validate the shared finite projection shader

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

WGSL has no source-file import in this pipeline.

## Type

Continue from [Write bounded projected triangle records on the GPU](86bb-write-bounded-projected-triangle-records-on-the-gpu.md). [Save or recover your work](recovery.md).

### 1. `src/finite_project_source.rs`

Assemble one shared shader interface and validate its compute entry point.

Create the file and type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-source-browser-info-reuse-check.rs"
```

### 2. `src/lib.rs`

Expose the existing-source assembly boundary for the later pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod tile_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-register.rs" skip=19
```

### 3. `src/renderer.rs`

Compile the completed recipe on the actual browser device and report asynchronous compilation info.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86c-project-existing-original-mesh-buffers-04-source-reuse-check.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The actual browser GPU compiles the assembled projection source and reports its shader errors; the compute pipeline is connected in the following lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for assemble and validate the shared finite projection shader.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Expand the explicit shared-source marker before passing shader text to wgpu. Reuse the projected-triangle declaration and coverage helpers already supplied at86a instead of maintaining another copy. The new compute functions read the original mesh layout from the outset; no second geometry adapter replaces them.

Parse and validate the assembled source using the Naga version already reexported by wgpu. This is a shader interface check; the later real pipeline and pixel tests remain necessary.

The learner checks this source on the actual browser GPU. Reuse the shader-check helper already written at64hd during renderer initialization. Open the developer console for the Finite projection report; no native test command is required. This validates the completed shader before the later pipeline connection.

original buffers → finite projected records → tile references.



Which source must be validated when a shader contains a shared-source marker?

Validate the assembled WGSL passed to wgpu, after expanding the marker. Validating the unexpanded file would not test the actual shader interface.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the browser and read the Finite projection compilation report. Temporarily change a WGSL return type so it disagrees with its return value: inspect the real GPU compiler message and location, then restore the type and reload. Finally compare the shared record layout with the existing Rust buffer.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.7
npm --prefix ../session_tests run course -- save 17.1.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Retain finite original-triangle projection and tile lists. The tutorial adapter connects identical center-line visibility to color and cropped integer picking; full plane fitting, curved-edge sag and packed production arenas are audited in94–96.

A Naga fixture parses and validates the actual assembled source and requires its compute entry point. Existing tile/pixel fixtures later exercise this same source function.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
