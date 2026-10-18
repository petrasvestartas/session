# 20.2.5 · Share crop and clipping bindings for pulled picks

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

Color vertex pulling uses group three for shared geometry.

## Type

Continue from [Own a cropped primitive image for picks](96zq-own-a-cropped-primitive-image-for-picks.md). [Save or recover your work](recovery.md).

### 1. `src/pick_pipeline.rs`

Prepare an explicit shared layout and bind the existing two uniform buffers.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn crop_clipping_layout(device:&wgpu::Device)->wgpu::BindGroupLayout{
    let entries=[80u64,448];
    let entries=entries.into_iter().enumerate().map(|(binding,size)|wgpu::BindGroupLayoutEntry{
        binding:binding as u32,visibility:wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(size)},count:None,
    }).collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("pick crop and clipping"),entries:&entries})
}
pub fn crop_clipping_group(device:&wgpu::Device,crop:&wgpu::Buffer,clipping:&wgpu::Buffer)->wgpu::BindGroup{
    device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("pick crop and clipping"),layout:&crop_clipping_layout(device),entries:&[
        wgpu::BindGroupEntry{binding:0,resource:crop.as_entire_binding()},
        wgpu::BindGroupEntry{binding:1,resource:clipping.as_entire_binding()},
    ]})
}

pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zr-share-crop-and-clipping-bindings-for-pulled-picks-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for share crop and clipping bindings for pulled picks.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The current mesh pick pipeline already uses that group for clipping. Prepare a two-binding uniform group for crop and clipping together, leaving group three available for the exact same geometry buffers. Keep the original single crop group for source-face, ink, text and other pick pipelines. Both groups refer to existing uniform buffers; no second crop or clipping upload is needed. This preparatory boundary does not switch the mesh pipeline yet.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why keep the original crop group too?

Other pick pipelines have their own explicit layouts. Changing their group implicitly would break otherwise independent face, ink and text picking.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Rebuild and check unchanged integer owners while the combined layout is prepared.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.5
npm --prefix ../session_tests run course -- save 20.2.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Produce bounded primitive addresses for first integer picks through the same shared geometry reader. Section primitive sentinels, active plane fitting, current packed arenas and MSAA remain subsequent bridges.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
