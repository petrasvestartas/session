# 01 · Draw one WebGPU frame

**Start:** checkpoint 00. **Finish:** a red/green/blue triangle on a dark canvas. Camera gestures arrive in the next lesson.

## From a canvas to a submitted frame

A **surface** is the connection to the canvas. An **adapter** represents an available GPU implementation. A **device** creates resources and pipelines; its **queue** submits work. A pipeline describes how a particular drawing operation reads data and runs shaders.

```text
canvas → compatible adapter → device + queue
                                ↓
                     shader + render pipeline
                                ↓
surface texture ← render pass ← command encoder
                                ↓
                         queue submission
```

`Tutorial::open` creates these long-lived resources once. `render_frame` obtains the next surface texture, begins a pass that clears it, draws three vertices, submits the encoder and presents the texture. A render pass borrows its encoder while recording commands; Rust prevents using that borrowed encoder incompatibly at the same time.

The asynchronous adapter/device requests use `.await`. They allow initialization to wait for the browser without a blocking loop. Error paths set an explicit status message instead of leaving a silent blank canvas.

## The first shader

`src/shaders/first.wgsl` has a vertex entry point and a fragment entry point. The vertex stage runs three times. `@builtin(vertex_index)` supplies 0, 1 and 2, which index fixed arrays of positions and colors. This first triangle needs no vertex buffer.

`@builtin(position)` is clip-space position, including its fourth component `w`. `@location(0)` carries color from the vertex output to the fragment input. Rasterization interpolates that color across the triangle. The fragment stage returns it to color attachment zero.

Read the complete shader beside that description; the ordered file list supplies these same bytes:

<!-- include-code: 01 session_viewer/src/shaders/first.wgsl -->

The matrix at `@group(0) @binding(0)` is a **uniform**: the same input is available to every vertex. Its Rust bind-group layout, uploaded size and WGSL type must agree. A binding mismatch can compile as Rust and still fail when the browser creates the GPU pipeline.

## Connect Rust and WGSL

Read `device.create_render_pipeline` in the complete Rust file beside the shader. Its `vs_main` and `fs_main` names must match the WGSL functions. Its target format must match the surface. The empty vertex-buffer list is intentional because the shader uses vertex indices.

`Some(value)` supplies an optional descriptor field; `None` means no value. `Default::default()` fills a descriptor with the library's defaults. We spell out fields when they establish this viewer's behavior and use defaults for unchanged options.

## Write the files

Follow [Complete file changes for 01](../lessons/01/index.md): replace the full browser entry and page, and create the shader. Do not append the new entry beneath checkpoint 00's `start`; the file page explicitly says to replace the whole file.

The direct-canvas teaching shell keeps this first frame small. Lesson 12 replaces it with the production winit/State shell; that transition is supplied as complete files and explicit removals.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Expect a colored triangle and a one-object status. Resize the window: the canvas and surface must resize together. The automated `--verify` check captures actual pixels at DPR 1 and 2 and rejects page/GPU errors.

If the background appears without the triangle, compare entry-point names, draw vertex count and the uniform binding against the complete files. If initialization fails, read the page's adapter/device error before changing shaders.

**Before continuing:** identify which function creates the pipeline and which function uses it each frame. Continue to [camera and spaces](02-camera.md).
