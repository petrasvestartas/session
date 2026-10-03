# 18 · Read the shape through light

**Typing: 6–11 minutes.** [Estimate](typing-load.md).

Shade the grey box according to surface direction. Pass world positions from the vertex shader to the fragment shader.

`dpdx` and `dpdy` estimate horizontal and vertical position changes. Their cross product gives the triangle normal; normalisation makes it unit length. Dot it with the unit world-space light direction to measure alignment.

## Type

Continue from [Bring a solid into the scene](17-solid.md). [Save or recover your work](recovery.md).

### 1. `src/triangle.wgsl`

Pass world positions through the pipeline and turn face direction into brightness.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = transform * vec4<f32>(position, 1.0);
    output.colour = colour;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.colour, 1.0);
}
```

</details>

Replace that block with:

```wgsl
--8<-- "journey/code/18-light-01.wgsl"
```

### 2. `src/browser.rs`

Report that lighting changes brightness without changing geometry.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Kernel geometry becomes an undoable scene object.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/18-light-window-1.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Example Box` and `View Isometric`. Faces pointing in different directions should have different brightness, making the box’s shape easier to read.

**Verified checkpoint in Chrome.**

![Actual browser result: Read the shape through light.](../screenshots/journey/18-light-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Use `abs` for this display's two-sided lighting and retain a minimum brightness of 0.3. Multiply RGB before the existing sRGB output conversion.

Only shader logic changes. Mesh still supplies positions and colours, Camera positions the view, and Renderer submits the triangles. Adjoining box faces now have visibly different brightness.

World position → neighbouring surface directions → normal → light alignment → fragment colour.

![Two directions along a face give its normal; comparing that normal with the light controls brightness.](../illustrations/journey-18.svg)

Why should orbiting the camera leave the brightness of a particular box face unchanged?

Both the normal and the light direction are expressed in world coordinates. Orbit changes where we look from, not the face direction or the light. A different face may become visible, but the same face keeps the same brightness. Mixing a camera-space normal with a world-space light would break that relationship.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the light to vec3<f32>(0.0, 0.0, 1.0). Predict the box first: the top should be bright and both vertical sides equally dark. Try it, then restore the original light. Explain why this changes colour without moving a single vertex.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18-light
npm --prefix ../session_tests run course -- save 18-light
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This is flat lighting: each triangle has one surface direction. The final viewer also needs smooth normals, materials, two-sided surface rules and contact shadows. Those features will extend the same distinction between geometry, view and shading.

Compare the box with lesson 17: the same geometry now has three face brightnesses.

[Full validation scope](release.md).

</details>
