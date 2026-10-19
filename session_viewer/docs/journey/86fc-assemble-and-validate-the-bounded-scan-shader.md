# 17.1.15 · Assemble and validate the bounded scan shader

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 3–6 minutes.** [Estimate](typing-load.md).

Reuse the shader source owner introduced at86c.

## Type

Continue from [Finalize bounded tile list offsets and demand](86fb-finalize-bounded-tile-list-offsets-and-demand.md). [Save or recover your work](recovery.md).

### 1. `src/renderer.rs`

Check the complete recipe on the actual browser GPU before the later pass connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-01-source-reuse-check.rs"
```

</details>

### 2. `src/finite_project_source.rs`

Reuse one source expansion owner for the completed recipe.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn scan() -> String { expand(include_str!("finite_scan_triangle_tiles.wgsl")) }

pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-02-source.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/finite_scan_contract.rs`

Supply independent assembled WGSL entry-point witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-03-source.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied shader interface fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod finite_scan_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86fc-assemble-and-validate-the-bounded-scan-shader-04-source.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.15
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The actual browser device compiles the assembled scan shader and reports its compilation info before pass connection.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for assemble and validate the bounded scan shader.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The completed scan recipe already targets the application’s original buffer layout; do not copy a second recipe and replace its geometry adapter. Validate the actual assembled text, then use this same source in the later pipeline. The browser device now compiles the complete shader and reports compilation info in the developer console. The later connection checks execution and pixels.

original buffers → finite projected records → tile references.



What must the complete scan shader validate before dispatch?

Both scan entry points must agree on the shared prefix helper, bindings and workgroup types. Validate the actual expanded module, as with the [count/fill shader](86e-count-and-fill-original-triangle-references.md).

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Read the Finite scan compilation report in the browser developer console. Temporarily introduce and restore a WGSL type error. Then list the storage bindings and predict the ordering with projection/count/scan/fill before the later connected fixture.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.15
npm --prefix ../session_tests run course -- save 17.1.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Retain finite original-triangle projection and tile lists. The tutorial adapter connects identical center-line visibility to color and cropped integer picking; full plane fitting, curved-edge sag and packed production arenas are audited in94–96.

Naga parses and validates the complete assembled source and its named stage entry points. The connected finite tests execute these same shader sources.

The browser contract is authored and requires an actual browser run. Native software-GPU results do not measure the Lenovo.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.1.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
