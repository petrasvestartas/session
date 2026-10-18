# 22.5.1 · Keep depth and address samples separate

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Create matching depth and integer-address images for one or four samples. Resolve only the colour image. Keep the current browser renderer at one sample until every consuming pass supports the same count.

## Type

Continue from [Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). [Save or recover your work](recovery.md).

### 1. `src/frame_targets.rs`

Own matching depth and primitive samples; keep the optional colour image as the only resolve source.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-01.rs"
```

### 2. `src/primitive_image.rs`

Keep request-owned queries single-sample. Four-sample integer images are sampled directly and never copied or resolved.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) const CAP_START:u32=0xfffffff0;
pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
pub(crate) struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView}
impl Image{
    pub fn new(device:&wgpu::Device,size:[u32;2])->Self{
        let texture=device.create_texture(&wgpu::TextureDescriptor{
            label:Some("physical triangle addresses"),
            size:wgpu::Extent3d{width:size[0].max(1),height:size[1].max(1),depth_or_array_layers:1},
            mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:FORMAT,
            usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_SRC,
            view_formats:&[],
        });
        let view=texture.create_view(&Default::default());Self{texture,view}
    }
}
pub(crate) fn target(write:bool)->wgpu::ColorTargetState{
    wgpu::ColorTargetState{format:FORMAT,blend:None,write_mask:if write{wgpu::ColorWrites::ALL}else{wgpu::ColorWrites::empty()}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-02.rs"
```

### 3. `src/lib.rs`

Register the attachment owner. The main renderer remains single-sample.

<details>
<summary>Locate the existing block</summary>

```rust
mod primitive_image;
```

</details>

After that block, type:

```rust
mod frame_targets;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/frame_target_tests.rs`

Read every actual depth/ID sample, resolve only colour and retain readable old bound views across replacement.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/lib.rs`

Register the supplied real-GPU sample fixture.

<details>
<summary>Locate the existing block</summary>

```text
mod frame_targets;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-05.rs" skip=18
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The browser retains its existing single-sample picture. The real GPU resource fixture verifies independent depth and integer samples at one and four samples, colour-only resolve, and readable old bound views after replacing the target owner.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual original single-sample contact panel and separated unequal instances. Original face picks, decoded source values, ambient shading and Move/Undo restoration pass at both densities and projections. A separate actual GPU fixture validates one- and four-sample depth/IDs and colour-only resolve; the browser renderer still uses one sample. Full viewer multisampling, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

![Actual original single-sample contact panel and separated unequal instances. Original face picks, decoded source values, ambient shading and Move/Undo restoration pass at both densities and projections. A separate actual GPU fixture validates one- and four-sample depth/IDs and colour-only resolve; the browser renderer still uses one sample. Full viewer multisampling, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

One multisampled pixel can contain different surfaces. Averaging physical primitive addresses would manufacture an address that never existed. Original-owner queries use their separate ID image. Keep each depth and integer address available by sample index; only colour becomes a single displayed pixel.

The supplied GPU fixture writes distinct depth and both integer halves for every actual sample. It reads them with a compute pass, verifies the resolved colour separately, then replaces the owner and checks that existing bound views still read the original values. It never assumes fixed sample positions.

The browser image intentionally retains the existing single-sample scene. Defining these resources does not enable four-sample rendering. The next checkpoint connects their owner to the renderer; coherent shaders, quality selection and full feature validation still follow.

One sample count → matching depth and primitive images → direct sample reads → colour-only resolve.



Why must a multisampled physical primitive address remain per sample?

Different samples can belong to different physical primitives. Averaging their integer values would invent an address. Read the matching sample’s depth and primitive address directly; original-owner queries retain their separate ID image.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare every actual sample’s depth and two integer halves, then read the resolved colour. Confirm that the1x integer image allows copy readback and the four-sample image is read through a shader instead.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.1
npm --prefix ../session_tests run course -- save 22.5.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

One target owner creates matching depth, primitive-address and optional colour images at one or four samples. Integer IDs and depth remain per sample; only colour resolves. Single-sample query images retain COPY_SRC, while four-sample IDs use direct shader reads. All 991 native tests, WebAssembly/optimized builds, the frame check, declared Chrome and stored evidence pass. The supplied actual GPU fixture verifies every distinct sample depth and both integer halves, resolved colour and readable old bound views across target replacement. Six actual canonical Native/browser pictures were individually inspected. Existing contact shading, original face picks, full decoded sources, Move/Undo camera/pixels and geometry-upload invariants pass in Chrome at both densities and actual projections. The browser renderer still uses one sample; these resource tests do not establish complete four-sample rendering or a frame-rate gain. Full viewer multisampling, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

The supplied actual GPU fixture reads every sample and retains old bound-view leases across owner replacement. Existing Native checks remain intact. This checkpoint defines resources; it does not claim browser multisample rendering or a frame-rate gain.

Actual original single-sample contact panel and separated unequal instances. Original face picks, decoded source values, ambient shading and Move/Undo restoration pass at both densities and projections. A separate actual GPU fixture validates one- and four-sample depth/IDs and colour-only resolve; the browser renderer still uses one sample. Full viewer multisampling, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
