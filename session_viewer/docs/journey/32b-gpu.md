# 32b · Count each shared GPU buffer once

**Typing: 7–14 minutes.** [Estimate](typing-load.md).

The upload counters tell us how much work happened over time. They cannot tell us how much document storage is alive now. Add a live GPU ledger that visits each geometry allocation once and every object-settings buffer separately.

Its four fields are unique geometry owners, their vertex/index buffer bytes, object-settings buffers, and settings-buffer bytes. [Buffer::size](https://docs.rs/wgpu/29.0.4/wgpu/struct.Buffer.html#method.size) reports the buffer size selected at creation. The ledger excludes camera/depth/dock resources, pending staging work, driver overhead and memory pooling; it is document buffer ownership, not a driver-memory measurement.

## Type

Continue from [Count shared CPU displays once](32a-cpu.md). [Save or recover your work](recovery.md).

### 1. `src/gpu_geometry.rs`

Read actual buffer sizes, including initialization alignment.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32b-gpu-01.rs"
```

### 2. `src/gpu_mesh.rs`

Report the independent settings buffer owned by one GPU row.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, selected: bool) -> [usize; 2] {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32b-gpu-02.rs"
```

### 3. `src/memory.rs`

Count shared geometry once while retaining each row’s separate settings cost.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod tests {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32b-gpu-03.rs"
```

### 4. `src/renderer.rs`

Distinguish live document resources from cumulative upload/write counters.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn stats(&self) -> [usize; 4] {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32b-gpu-04.rs"
```

### 5. `src/browser.rs`

Expose live document buffer sizes without adding runtime controls.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32b-gpu-05.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the GPU accounting checks below. Live document buffer bytes must count shared allocations once; cumulative upload counters do not fall when owners drop.

**Verified checkpoint in Chrome.**

![Actual browser result: Count each shared GPU buffer once.](../screenshots/journey/32b-gpu-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

The native GPU proof creates two rows sharing one three-vertex geometry. It expects one seventy-two-byte vertex buffer, one eight-byte index buffer, and two eighty-byte settings buffers. That exercises sharing and alignment with an independent known input.

The browser exposes data-gpu-usage alongside data-cpu-usage. After document close these live document figures must become zero, while cumulative uploads remain a useful record of work already performed.

Live GPU rows → unique geometry owners and independent settings → buffer-size ledger.

![Shared geometry contributes buffer bytes once; every row contributes its own settings buffer.](../illustrations/journey-32b.svg)

Why can index buffer bytes exceed the six index bytes we supplied?

Buffer initialization rounds its allocation to the copy-alignment requirement. Three u16 indices carry six payload bytes but this upload has an eight-byte GPU buffer. Read Buffer::size for allocated buffer size rather than equating it with the source slice length.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

For three u16 indices, calculate six payload bytes and eight allocated upload bytes. Then calculate two object uniforms sharing that geometry. Explain why adding geometry bytes for both rows would be wrong.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32b-gpu
npm --prefix ../session_tests run course -- save 32b-gpu
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production accounting also includes packed arenas, textures, staging and renderer infrastructure. This chapter establishes exact document-buffer sharing before those later resource families are introduced.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32b-gpu
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
