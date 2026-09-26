# 01 · First WebGPU frame

One triangle on a dark canvas.

![What one WebGPU frame needs: nine objects made once in create(), seven steps repeated in every render(). The pipeline and bind group made on the left are what the pass on the right uses.](illustrations/01-objects.svg)

Left column: made once. Right column: done every frame.

This lesson is the **Shell** and **Shaders** part of the map.

![The whole viewer as one map: documents come in along the top row, a frame is drawn along the bottom one.](illustrations/map.svg)

## Step 1 · `src/lib.rs`, part 1: the struct

Replace the whole file: one struct owns every GPU object, and its four public methods are what the page calls.

`lessons/01/src/lib.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/01/src/lib.rs:step-1"
```

## Step 2 · part 2: instance, surface, adapter, device

Append: create the GPU connection in four steps, instance → surface → adapter → device and queue.

![The four objects of step 2 and what each one is for. Grey boxes are made in open(), used once and dropped: the Instance is the WebGPU API itself, the Adapter is one physical GPU. Pink boxes are stored in Tutorial and used by every later step: the Surface hands out one texture per frame, the Device makes every GPU object, the Queue is where finished commands are submitted. The dashed notes say which later step uses each one.](illustrations/01-gpu-chain.svg)

`lessons/01/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/lib.rs:step-2"
```

## Step 3 · part 3: surface configuration and the camera uniform

Append: describe the canvas texture, and give the shader one 4×4 matrix through a uniform buffer.

`lessons/01/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/lib.rs:step-3"
```

## Step 4 · part 4: shader module and pipeline

Append: load the shader and build the pipeline, the fixed recipe for drawing.

`lessons/01/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/lib.rs:step-4"
```

## Step 5 · part 5: one frame

Append: draw one frame — resize if needed, clear the canvas, draw three vertices, present.

`lessons/01/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/lib.rs:step-5"
```

## Step 6 · `src/shaders/first.wgsl`

New file: the vertex shader places three corners, the fragment shader colors the pixels between them.

`lessons/01/src/shaders/first.wgsl` · type this, new file

```wgsl
--8<-- "lessons/01/src/shaders/first.wgsl"
```

Run `cargo check` in `lessons/01/`.

Left: clip space, as the shader sees it. Right: the canvas in pixels.
![Clip space is a square from -1 to +1 with y up; the viewport transform turns it into pixels with y down.](illustrations/clip-space.svg)

## Step 7 · `index.html`

Replace the whole file: the page gets a canvas and calls `create` once, then `render` on every resize and pointer event.

`lessons/01/index.html` · edit · copy the file

Replaces the lines from `<title>Session checkpoint 00</title>` to `<output id="status">Loading WASM</output>` in `lessons/00/index.html`

```html
--8<-- "lessons/01/index.html:step-7"
```

## Check

Run `trunk serve` in `lessons/01/` and open <http://127.0.0.1:8770/>.

Expected: a red, green and blue triangle and the status **Checkpoint 01 · 1 objects · W×H**; it stretches with the window.

![Checkpoint 01: the first triangle, colors interpolated from the three vertices.](screenshots/01.png)

If it fails:

- Dark canvas, no triangle: one of the three names in step 6 does not match the Rust, or `draw(0..3, 0..1)` was mistyped.
- Status shows an error instead of the size: read it. It is the adapter or device request failing, and the browser has no WebGPU.
- Canvas stays empty and the console shows *tutorial WebGPU error*: a validation error; the message names the descriptor field.

## What changed

```text
lessons/01/src/
├── shaders/
│   └── first.wgsl  +
└── lib.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/01/`.

## Next

[02 · Camera](02-camera.md)

## Expected viewer result

Checkpoint 01: the first triangle, colors interpolated from the three vertices.

[![Full viewer result for 01 first frame](screenshots/01.png)](screenshots/01.png)
