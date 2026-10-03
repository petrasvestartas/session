# 31c · Update only changed object settings

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 20–39 minutes.** 41 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Retain GPU rows by object and geometry identity, then write only changed uniform ranges.

**Follow:** Scene row → retained GPU row → matrix or selection byte difference → queued range write → next draw.

The cache removes repeated vertex/index uploads, but every scene synchronization still allocates object settings. Retain each GPU row when its ObjectId and CPU display owner both match. Keep scene order when collecting the new row list; swap_remove only changes the temporary list of old rows.

![A retained row writes only its changed matrix or selection range before the next draw.](../illustrations/journey-31c.svg)

GpuMesh now keeps its uniform buffer and a copy of the last eighty encoded bytes. Compare the sixty-four matrix bytes and sixteen selection bytes separately. A selection change writes sixteen bytes; Move writes sixty-four. Unchanged values queue no writes. Comparing the encoded float bytes also avoids writing a double change that produces the same GPU value.

The buffer adds COPY_DST because updates use the queue. [wgpu’s write_buffer contract](https://docs.rs/wgpu/29.0.4/wgpu/struct.Queue.html#method.write_buffer) copies our bytes into staging immediately and executes their transfer before the commands in the next submission. Our normal draw submits after synchronization; no extra submission is required here. Offsets zero and sixty-four, and lengths sixty-four and sixteen, satisfy the [WebGPU four-byte write alignment](https://www.w3.org/TR/webgpu/#dom-gpuqueue-writebuffer).

The renderer takes its old rows temporarily, finds matching identities, updates their settings, and puts them in scene order. Rows left in the old list are dropped before the weak cache is pruned. The hidden counters now record actual range writes and bytes in addition to allocations.

## Type the change

Continue from [Reuse uploads while their geometry is alive](31b-cache.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-31c-incremental` (from `session_viewer`).

### 1. `src/gpu_mesh.rs`

Retain row identity, its writable settings buffer and the bytes last sent.

Find this exact block:

```rust
    model_group: wgpu::BindGroup,
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-01.rs"
```

### 2. `src/gpu_mesh.rs`

Allow queued updates to the settings buffer.

Find this exact block:

```rust
            usage: wgpu::BufferUsages::UNIFORM,
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-02.rs"
```

### 3. `src/gpu_mesh.rs`

Keep settings ownership beside the bind group.

Find this exact block:

```rust
        Self { geometry, model_group }
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-03.rs"
```

### 4. `src/gpu_mesh.rs`

Queue only changed matrix or selection ranges and count their actual calls and bytes.

Find this exact block:

```rust
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-04.rs"
```

### 5. `src/renderer.rs`

Accumulate settings write evidence.

Find this exact block:

```rust
    settings_allocations: usize,
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-05.rs"
```

### 6. `src/renderer.rs`

Initialize the cumulative write counters.

Find this exact block:

```rust
            geometry: Default::default(), settings_allocations: 0, uniform, view_group, depth };
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-06.rs"
```

### 7. `src/renderer.rs`

Retain matching GPU rows, preserve scene draw order and drop unused rows before cache pruning.

Find this exact block:

```rust
        let mut meshes = Vec::with_capacity(scene.objects().len());
        for object in scene.objects() {
            let geometry = self.geometry.get(&self.device, &object.mesh);
            meshes.push(GpuMesh::with_geometry(&self.device, &layout, object,
                selected == Some(object.id), geometry));
            self.settings_allocations += 1;
        }
        self.meshes = meshes;
        self.geometry.prune();
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-07.rs"
```

### 8. `src/renderer.rs`

Report actual update calls and bytes rather than reserved zeros.

Find this exact block:

```rust
        [self.geometry.uploads, self.settings_allocations, 0, 0]
```

Replace that block with:

```rust
--8<-- "journey/code/31c-incremental-08.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Move a selected object, then Undo. Run the GPU counter checks: these changes update only the matrix range, without reuploading vertex or index buffers.

**Verified checkpoint in Chrome.**

![Actual browser result: Update only changed object settings.](../screenshots/journey/31c-incremental-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Select the same row again without changing placement, or synchronize the same scene directly in the native check. Predict zero additional writes. Explain why a deleted row may need a fresh upload when Undo restores it: the cache deliberately has only weak ownership.

</details>

## Explain the change

Why must row reuse compare both ObjectId and the geometry owner?

<details>
<summary>Compare your explanation</summary>

ObjectId identifies the document row, but a geometry edit may replace its display allocation. Reuse settings and geometry only when both identities match. Otherwise request the correct upload and construct a new GPU row. Matching rows update only changed matrix or selection bytes.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 31c-incremental
npm --prefix ../session_tests run course -- save 31c-incremental
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Incremental ownership is a prerequisite for production arenas, resource accounting and large-scene responsiveness. Their complete packing and performance acceptance remain later lessons.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native GPU and Chrome checks verify no allocation during selection or Move, no writes for unchanged settings, exact selection and Move/Undo pixel round trips, and release/reupload after deleting and undoing a row. Deleting clears selection, and document Undo restores the row without restoring that selection. The checks explicitly reselect the row before comparing its original gold pixels. These counters describe the calls made by this renderer; they do not claim driver memory size or frame-time performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 31c-incremental
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
