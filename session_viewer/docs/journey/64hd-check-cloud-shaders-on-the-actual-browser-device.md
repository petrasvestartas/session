# 11.2.13 · Check cloud shaders on the actual browser device

**Chapter 11 · Outlines and point clouds · Section 11.2 · Point clouds and detail levels** · [Contents](contents.md)

**Typing: 14–28 minutes.** [Estimate](typing-load.md).

The WGSL sections are complete. Check them with the actual browser GPU, then inspect compilation info in the developer console. A Rust build alone does not prove that a shader is valid.

## Type

Continue from [Encode original cloud normals and resolve point targets](64hc-encode-original-cloud-normals-and-resolve-point-targets.md). [Save or recover your work](recovery.md).

### 1. `src/shader_check.rs`

Await one real browser GPU compilation report without blocking input.

Create the file and type:

```rust
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-01.rs"
```

### 2. `src/lib.rs`

Share the completed-source GPU check with later shader owners.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod cloud_normals;
```

</details>

After that block, type:

```rust
mod shader_check;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-02.rs"
```

</details>

### 3. `Cargo.toml`

Enable browser developer-console compilation reports in the existing web-sys dependency.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-03.rs"
```

</details>

### 4. `src/renderer.rs`

Check both completed cloud modules on the actual device before their pipeline connection.

<details>
<summary>Locate the existing block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        crate::shader_check::check(&device, include_str!("cloud_points.wgsl").into(), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/cloud_shader_contract.rs`

Supply an independent completed-source shader interface witness.

Copy this supplied file:

```text
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-05.rs"
```

</details>

### 6. `src/lib.rs`

Register the supplied optional shader interface witness.

<details>
<summary>Locate the existing block</summary>

```rust
mod shader_check;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod cloud_shader_contract;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/64hd-check-cloud-shaders-on-the-actual-browser-device-06.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.2.13
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Both completed cloud modules compile on the actual browser GPU; the developer console reports shader errors before pass connection.

### Recorded visual result

**Previous native render — current verification pending.**

The selected CAD bore and black original and silhouette contours remain visible.

![Native renderer output for check cloud shaders on the actual browser device.](../screenshots/journey/63h-outline.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Write one shared shader-check helper. Later finite-ink lessons reuse it; they do not introduce another checker. The optional supplied native fixture provides additional authoring evidence, while the learner runs the browser and reads the actual GPU report.

check cloud shaders on the actual browser device.



Why does the shader check run asynchronously in the browser?

Shader compilation info arrives through the browser GPU task queue. Await it without blocking drawing or input, then inspect its errors and source locations.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the browser and read the Cloud points and Cloud resolve reports. Temporarily change a WGSL return type, inspect the compiler message and location, then restore it and reload. The source cloud and visible mesh remain unchanged while this pass is not connected.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.2.13
npm --prefix ../session_tests run course -- save 11.2.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Compile the cumulative browser app. The completed source is checked on the actual browser device at64hd before pipeline connection.

The inherited silhouette browser contract remains authored and unrun. Cloud acceptance connects at the final endpoint; native software rendering is not Lenovo timing.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 11.2.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
