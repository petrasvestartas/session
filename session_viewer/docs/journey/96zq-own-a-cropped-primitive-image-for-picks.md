# 20.2.4 · Own a cropped primitive image for picks

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 3–6 minutes.** [Estimate](typing-load.md).

A pick can happen before a color frame or after its camera became stale.

## Type

Continue from [Fit a neighboring surface along the stroke](96zp-fit-a-neighboring-surface-along-the-stroke.md). [Save or recover your work](recovery.md).

### 1. `src/pick_image.rs`

Own matching bounded primitive and depth targets. Reuse the primitive image owner at the actual cropped dimensions.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::pick_window::Window;

pub(crate) struct Image {
    pub texture: wgpu::Texture,
    pub colour: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub section:wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
}

impl Image {
    pub fn new(device: &wgpu::Device, window: Window) -> Self {
        let size = wgpu::Extent3d { width: window.draw[2], height: window.draw[3], depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor { label: Some("bounded pick image"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg32Uint, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
        let colour = texture.create_view(&Default::default());
        let depth = device.create_texture(&wgpu::TextureDescriptor { label: Some("pick depth"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] }).create_view(&Default::default());
        // GPU image rows need 256-byte copy alignment; the decoder skips that padding and reads only the logical columns.
        let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bounded pick readback"), size: window.bytes(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let section=device.create_texture(&wgpu::TextureDescriptor{label:Some("bounded section depth"),size,mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rg32Float,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,view_formats:&[]}).create_view(&Default::default());
        Self { texture, colour, depth, section,readback, window }
    }

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::pick_window::Window;

pub(crate) struct Image {
    pub texture: wgpu::Texture,
    pub colour: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub primitive:crate::primitive_image::Image,
    pub section:wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
}

impl Image {
    pub fn new(device: &wgpu::Device, window: Window) -> Self {
        let size = wgpu::Extent3d { width: window.draw[2], height: window.draw[3], depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor { label: Some("bounded pick image"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg32Uint, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
        let colour = texture.create_view(&Default::default());
        let depth = device.create_texture(&wgpu::TextureDescriptor { label: Some("pick depth"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] }).create_view(&Default::default());
        // GPU image rows need 256-byte copy alignment; the decoder skips that padding and reads only the logical columns.
        let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bounded pick readback"), size: window.bytes(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let section=device.create_texture(&wgpu::TextureDescriptor{label:Some("bounded section depth"),size,mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rg32Float,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,view_formats:&[]}).create_view(&Default::default());
        let primitive=crate::primitive_image::Image::new(device,[size.width,size.height]);
        Self { texture, colour, depth, primitive, section,readback, window }
    }

}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zq-own-a-cropped-primitive-image-for-picks-1.rs"
```

```text
--8<-- "journey/code/96zq-own-a-cropped-primitive-image-for-picks-2.rs"
```

</details>

### 2. `src/pick_gpu.rs`

Count owner, depth, section and primitive bytes honestly.

<details>
<summary>Locate the existing block</summary>

```rust
            image.readback.size() + u64::from(image.window.draw[2]) * u64::from(image.window.draw[3]) * 20) + 160+u64::from(self.legacy_clipping.is_some())*448]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            image.readback.size() + u64::from(image.window.draw[2]) * u64::from(image.window.draw[3]) * 24) + 160+u64::from(self.legacy_clipping.is_some())*448]
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zq-own-a-cropped-primitive-image-for-picks-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pick_gpu_tests.rs`

Include the actual four primitive bytes per cropped pixel while retaining the same bounded readback allocation assertion.

<details>
<summary>Locate the existing block</summary>

```text
31 * 31 * 20
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zq-own-a-cropped-primitive-image-for-picks-4.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.4
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

![Native renderer output for own a cropped primitive image for picks.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

It needs its own primitive image with the same cropped extent as pick depth, not the previous canvas image. Reuse the existing RG16Uint image owner and allocate it only when the bounded pick target changes. Include its four bytes per pixel in the pick memory report. This preparation does not attach the image yet.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why not reuse the last color image?

Its camera, scene or size may differ from the queued pick. The integer answer must come from the precise request that is currently in flight.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Request a pick before drawing and after resizing. Allocation remains bounded by the crop rather than by the whole canvas.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.4
npm --prefix ../session_tests run course -- save 20.2.4
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
npm --prefix ../session_tests run course -- capture 20.2.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
