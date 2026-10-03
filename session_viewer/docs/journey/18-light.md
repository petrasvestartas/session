# 18 · Read the shape through light

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 6–11 minutes.** 9 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Shade the box faces according to their direction, using the same mesh and renderer.

**Follow:** World position → neighbouring surface directions → normal → light alignment → fragment colour.

Shade the grey box according to surface direction. Pass world positions from the vertex shader to the fragment shader.

`dpdx` and `dpdy` estimate horizontal and vertical position changes. Their cross product gives the triangle normal; normalisation makes it unit length. Dot it with the unit world-space light direction to measure alignment.

Use `abs` for this display's two-sided lighting and retain a minimum brightness of 0.3. Multiply RGB before the existing sRGB output conversion.

Only shader logic changes. Mesh still supplies positions and colours, Camera positions the view, and Renderer submits the triangles. Adjoining box faces now have visibly different brightness.

![Two directions along a face give its normal; comparing that normal with the light controls brightness.](../illustrations/journey-18.svg)

## Type the change

Continue from [Bring a solid into the scene](17-solid.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-18-light` (from `session_viewer`).

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

Connect read the shape through light to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

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

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Type `Example Box` and `View Isometric`. Faces pointing in different directions should have different brightness, making the box’s shape easier to read.

**Verified checkpoint in Chrome.**

![Actual browser result: Read the shape through light.](../screenshots/journey/18-light-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Change the light to vec3<f32>(0.0, 0.0, 1.0). Predict the box first: the top should be bright and both vertical sides equally dark. Try it, then restore the original light. Explain why this changes colour without moving a single vertex.

</details>

## Explain the change

Why should orbiting the camera leave the brightness of a particular box face unchanged?

<details>
<summary>Compare your explanation</summary>

Both the normal and the light direction are expressed in world coordinates. Orbit changes where we look from, not the face direction or the light. A different face may become visible, but the same face keeps the same brightness. Mixing a camera-space normal with a world-space light would break that relationship.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 18-light
npm --prefix ../session_tests run course -- save 18-light
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This is flat lighting: each triangle has one surface direction. The final viewer also needs smooth normals, materials, two-sided surface rules and contact shadows. Those features will extend the same distinction between geometry, view and shading.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Compare the box with lesson 17: the same geometry now has three face brightnesses.

[Full validation scope](release.md).

</details>
