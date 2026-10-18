# 17.2.6 · Pack the clipping uniform with explicit byte offsets

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 14–27 minutes.** [Estimate](typing-load.md).

The GPU reads a byte layout, not a Rust object.

## Type

Continue from [Gather six visible original world planes](87a-gather-six-visible-original-world-planes.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_uniform.rs`

Write the fixed arrays, scalar fields and explicit padding for the shared GPU layout. Describe only the current camera inputs this adapter borrows.

Create the file and type:

```rust
use session_rust::Xform;
use crate::clipping_geometry::{ClipPlane,dot};
pub const MAX_PLANES:usize=6;
const HATCH_CSS_PX:f64=8.;
#[repr(C)]
#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct ClipUniform {
    // offset size
    pub planes: [[f32; 4]; MAX_PLANES], //   0   96  kept side relative to the anchor
    pub screen: [[f32; 4]; MAX_PLANES], //  96   96  the same over canvas clip space
    pub hatch: [[f32; 4]; MAX_PLANES],  // 192   96  hatch coordinate numerator over (x, y, 1)
    pub hatch_w: [[f32; 4]; MAX_PLANES], // 288   96  its denominator
    pub sides: [[f32; 4]; 2],           // 384   32  side of each plane the eye is on
    pub count: u32,                     // 416    4  planes in use
    pub samples: u32,                   // 420    4  scene samples per pixel
    pub fill: u32,                      // 424    4  0 hatch, 1 solid light grey
    pub spacing: f32,                   // 428    4  hatch spacing, px
    pub width: f32,                     // 432    4  hatch line width, px
    pub outline: f32,                   // 436    4  cut boundary width, px
    pub pad: [f32; 2],                  // 440    8  -> 448
}
pub struct ClipView<'a>{pub view_proj:&'a Xform,pub anchor:[f64;3],pub height:u32,pub pixel_scale:f64,pub samples:u32,pub forward_depth:bool}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/87b-copy-the-maintained-clipping-uniform-layout-01.rs"
```

```text
--8<-- "journey/code/87b-copy-the-maintained-clipping-uniform-layout-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod clipping_rows;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/87b-copy-the-maintained-clipping-uniform-layout-03.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The browser still shows the preceding finite-ink drawing. The clipping uniform layout now compiles; its plane coefficients and visible cutting behavior connect in the following lessons.

### Recorded visual result

**Previous native render — current verification pending.**

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for pack the clipping uniform with explicit byte offsets.](../screenshots/journey/87a-gather-six-visible-original-world-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.2.5 · Gather six visible original world planes](87a-gather-six-visible-original-world-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Group six vec4 plane coefficients, screen coefficients and homogeneous hatch rows in fixed arrays. Keep eye-side values in two vec4 rows. Follow with eight scalar words so the complete uniform occupies 448 bytes. Use repr(C) and explicit padding; the field offsets beside the code let you compare Rust storage with the WGSL declaration written later.

ClipView borrows the current camera matrix and provides the anchor, framebuffer height, CSS density and sample count. It does not own or replace the original clipping planes. The next lessons fill this layout in f64 and convert only the final GPU coefficients to f32.

original source → checked derived clipping → current view.



Why do six eye-side values occupy eight scalar slots?

Two vec4 rows occupy 32 bytes. Keeping that aligned representation gives the same offsets in Rust and WGSL; the last two scalar slots remain unused.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate the offset of count from the preceding arrays, then calculate the total size through the padding. Compare those offsets with the later WGSL ClipUniform.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.6
npm --prefix ../session_tests run course -- save 17.2.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Compile the cumulative browser app. This is uniform preparation. Later projection fixtures check the values and the connected clipping browser exercise checks the actual GPU passes.

Preparation; the clipping pass is connected later.

[Full validation scope](release.md).

</details>
