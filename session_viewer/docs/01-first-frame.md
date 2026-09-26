# 01 · First WebGPU frame

To light one pixel, the GPU needs a chain of objects. The page's canvas becomes a surface; the surface picks an adapter, one real GPU; the adapter opens a device, our connection to it. Through the device we make buffers (GPU memory), textures (images to draw into) and pipelines (the recipe for one kind of draw). Each frame we record commands into an encoder, submit them, and the canvas shows the result.

This lesson writes that whole renderer core, the part every later lesson plugs into. It draws one thing so far: a white background, as one triangle that covers the screen. Nothing calls it until lesson 12 opens the window, so you prove it works with tests, and two of them open your real GPU.

![From canvas to screen: create_surface, request_adapter, request_device, then a pipeline and an encoder whose render pass clears and draws three vertices, and finally submit and present.](illustrations/01-01.svg)

We build it in this order: shaders into the crate, opening the GPU, settings, GPU memory, textures, pipelines, the shared shader code, per-frame data, lanes and passes, the object table, the background, one frame, and last the `Gpu` struct that holds it all. The crate compiles only once every file exists, so the checkpoint comes at the end.

## Shaders become part of the crate

A shader is a small program that runs on the GPU, written in WGSL, the WebGPU shading language. Ours live in `src/shaders/`, and the build pastes them into the binary.

### Strip the shaders before they ship

`lessons/01/build.rs` · new file

A build script is a program cargo runs before compiling the crate. Ours copies each `.wgsl` file into cargo's output folder, `OUT_DIR`, with `#include "file"` lines replaced by that file and comments, indentation and blank lines removed. The shaders reach the browser smaller, while the source keeps its comments.

```rust
--8<-- "lessons/01/build.rs"
```

### Load a shader by name

`lessons/01/src/lib.rs` · append at the end of the file

`shader!("background.wgsl")` becomes that file's stripped text, built into the binary. The macro must come before `mod engine;`, because the engine uses it.

```rust
--8<-- "lessons/01/src/lib.rs:shader-macro"
```

### Split the engine in three

`lessons/01/src/engine/mod.rs` · new file

The engine is everything that draws: the GPU code, frame timing, and the pipelines.

```rust
--8<-- "lessons/01/src/engine/mod.rs"
```

## Open the GPU

### List the files of the gpu folder

`lessons/01/src/engine/gpu/mod.rs` · new file

This file starts as a table of contents, one line per file you are about to write. A line tagged `register:<name>` is a registration: later lessons add their own lines to lists like this one, and never edit the lines already there.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:modules"
```

### Say what opening returns

`lessons/01/src/engine/gpu/device.rs` · new file

Opening the GPU gives us six things: the surface, the device, the queue, the canvas configuration, the GPU type and a slot for the first error. The queue is where commands wait for the GPU. The surface is `None` in a test, which draws with no window.

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:setup"
```

### Open it in four calls

`lessons/01/src/engine/gpu/device.rs` · append at the end of the file

The chain from the picture, in order: instance, surface, adapter, then device and queue. The browser answers each request later, so `open` is an `async fn` and waits with `.await`. At the end we configure the canvas: its size, and an sRGB format, so the GPU turns our linear colours into what the screen expects.

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:open"
```

### Choose a GPU, keep the first error

`lessons/01/src/engine/gpu/device.rs` · append at the end of the file

On your machine, `VIEWER_ADAPTER=nvidia` picks a GPU by name. One broken draw often causes ten more errors, and only the first tells you why, so we store the first and ignore the rest. Natively, a GPU error stops the program, so no test can miss one.

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:helpers"
```

### Prove a broken shader is caught

`lessons/01/src/engine/gpu/device.rs` · copy this part, append at the end of the file

This test compiles a shader with a type error on your real GPU and expects the error callback to fire. `#[ignore]` skips it unless you ask for GPU tests.

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:shader-test"
```

### Catch the browser's GPU errors

`lessons/01/src/engine/gpu/device.rs` · append at the end of the file

The browser reports GPU errors and a lost device through callbacks, long after the call that caused them. Both go into the same first-error slot.

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:browser-errors"
```

### Prove the first error survives

`lessons/01/src/engine/gpu/device.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:tests"
```

## Settings from the page address

### Collect the display settings

`lessons/01/src/engine/gpu/view.rs` · new file

`View` holds every display switch: grid, lines, points, outlines, pen width and more. Each starts from the page address, so `?msaa=4` or `?nogrid` changes a setting without a rebuild. Later lessons flip them from keys and commands.

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:view"
```

### Count real pixels

`lessons/01/src/engine/gpu/view.rs` · append at the end of the file

A CSS pixel is the unit the page is laid out in; a phone draws each one with 2 or 3 real pixels. We draw at the real size, so an 800 × 600 canvas at ratio 2 is 1600 × 1200 pixels. After a lost GPU the page drops to ratio 1 until reload.

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:pixel-ratio"
```

### Read one setting

`lessons/01/src/engine/gpu/view.rs` · append at the end of the file

A knob is one setting read at start: `?msaa=4` in the browser, the environment variable `VIEWER_MSAA=4` in a native test. One function hides that difference.

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:knobs"
```

## GPU memory

A buffer is a block of GPU memory. We fill it from the CPU; shaders read it.

### Keep device, queue and cache together

`lessons/01/src/engine/gpu/buffers.rs` · new file

Almost every function below needs the device, the queue and the cache of compiled pipelines, which the Pipelines part of this lesson builds. So they travel as one value, `GpuCtx`.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:ctx"
```

### Say what a buffer is for

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

Usage flags tell the GPU what a buffer may do. A row is one fixed-size record in a buffer, such as one vertex or one object. Our growing buffers hold rows for shaders, vertices, or the indices that say which three vertices make each triangle; each can be written and copied.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:usages"
```

### A buffer that grows

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

A GPU buffer cannot change size. So when 100 rows are full and a 101st arrives, we make a buffer for 150 and copy the old rows across, GPU to GPU. Growing by half keeps the number of copies small.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:growbuf"
```

### Write rows in place

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

An edit overwrites rows where they are. `fill` repeats one row over a range, 4 MB per write; `packed` copies only the rows still in use into a new, smaller buffer.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:rows"
```

### Forget, free, and know the limit

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

A reset forgets the rows but keeps the memory for the next scene; a release gives the memory back. Every GPU has a largest buffer it allows, so a buffer never grows past it.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:growbuf-state"
```

### Upload a small mesh once

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

A template is a small mesh, such as the square behind a point marker, uploaded once and drawn many times.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:template"
```

### Uniforms and bind groups

`lessons/01/src/engine/gpu/buffers.rs` · append at the end of the file

A uniform buffer holds one small value that every run of a shader reads, such as the camera matrix. A bind group bundles buffers so a draw can read them; its layout says which slot holds what.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:helpers"
```

### Prove the growth rule

`lessons/01/src/engine/gpu/buffers.rs` · copy this part, append at the end of the file

100 rows grow to 150; a need of 300 gets exactly 300; nothing grows past the limit.

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:tests"
```

## Textures to draw into

A render target is a texture a draw writes into. Besides the colours we keep a depth texture: for every pixel, how near the closest surface is, so a far face never paints over a near one.

### Set the anti-aliasing budget

`lessons/01/src/engine/gpu/targets.rs` · new file

MSAA (multisample anti-aliasing) takes 4 samples per pixel, which smooths jagged edges and costs 4 times the memory. So each kind of GPU gets a pixel budget: 9 million pixels on a discrete GPU, a separate card with its own memory, and 2.5 million on an integrated one, which shares the CPU's.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:budgets"
```

### The frame's textures

`lessons/01/src/engine/gpu/targets.rs` · append at the end of the file

Three textures per frame: depth, colour, and triangle ids. The triangle id texture stores, per pixel, which triangle covers it; later lessons read it to hide lines behind faces. Shaders expect both a 1-sample and a 4-sample version bound, so the unused one is a 1 × 1 placeholder.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:targets"
```

### Choose 1 or 4 samples

`lessons/01/src/engine/gpu/targets.rs` · append at the end of the file

4x only when there are solids (objects with filled faces), the canvas fits the budget, and the screen is not already sharp at ratio 2. `?msaa=` overrides the rule.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:samples"
```

### Open the two passes

`lessons/01/src/engine/gpu/targets.rs` · append at the end of the file

A render pass is one run of draws into a set of textures. The face pass clears colour and depth and draws the solids. The ink pass draws lines over them; it reads depth but never writes it. We use reverse-Z depth: near is 1 and far is 0, which keeps float depth precise far away.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:passes"
```

### Create a texture and free it on drop

`lessons/01/src/engine/gpu/targets.rs` · append at the end of the file

`Attachment` pairs a texture with its view. A view is how a pass or shader sees a texture. Implementing `Drop` runs code when a value goes out of scope; here it frees the GPU memory at once, instead of whenever the browser gets to it.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:textures"
```

### Prove the budget

`lessons/01/src/engine/gpu/targets.rs` · copy this part, append at the end of the file

A 4K canvas gets 4x on a discrete GPU and 1x on an integrated one.

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:tests"
```

## Pipelines

A render pipeline is the fixed recipe for one kind of draw: which shader, how vertices are read, how colour blends, how depth is tested. Compiling one takes milliseconds, and the viewer needs dozens. So we build them lazily: a pipeline compiles on its first use, and asking twice returns the same one.

### Count what was compiled

`lessons/01/src/engine/pipelines/mod.rs` · new file

Two counters for the startup log: pipelines and shader modules compiled so far.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:head"
```

### Make an object on its first use

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

`Lazy<T>` holds a closure, a function stored as a value, that makes a `T`. The first time you use the value, the closure runs; every clone shares that one result.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:lazy"
```

### Remember everything asked for

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

The cache maps a description to the object made from it. Switching MSAA from 1x to 4x and back asks for the same pipelines again, and nothing compiles twice.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:cache"
```

### Name the ways to use depth and colour

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

A handful of modes cover every draw: opaque faces write depth, lines only test it, the background ignores it; colour is replaced, blended, or kept at the larger value.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:modes"
```

### Describe one pipeline

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

`PipelineDesc` starts from sensible defaults, and each method changes one part: `base.with("grid", "fs_main").depth(DepthMode::ReadOnly)`.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:desc"
```

### Share WGSL between shaders

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

WGSL has no `import`. So every scene shader gets the shared code pasted after its own; the list starts with one file, `scene.wgsl`, and grows by one line per shared snippet.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:prelude"
```

### Cut a buffer into vertices

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

A vertex buffer layout tells the pipeline how many bytes one vertex takes and which bytes feed which shader input. We need two: one object row number per vertex, and one bare position.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:vertex-layouts"
```

### Compile each shader text once

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

A shader module is WGSL compiled for this GPU. The cache keys it by a hash, a short number computed from the text, so two lanes with the same text share one module.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:shader-modules"
```

### Build a pipeline, compile it later

`lessons/01/src/engine/pipelines/mod.rs` · append at the end of the file

`build` only records the description and returns a `Lazy` pipeline; `compile` runs when a pass first sets it.

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:build"
```

### Prove nothing is made early

`lessons/01/src/engine/pipelines/mod.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:tests"
```

### Describe each slot a shader reads

`lessons/01/src/engine/pipelines/layouts.rs` · new file

A bind group layout is the type of a bind group: which binding holds a uniform, a storage buffer (a large buffer a shader reads by index, like the object rows) or a texture, and which shader stages see it. These helpers make one entry each.

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:entries"
```

### One layout per kind of group

`lessons/01/src/engine/pipelines/layouts.rs` · append at the end of the file

A shader reaches each bind group by number. Group 0 is the camera matrix, group 1 the pen and view settings, group 2 the object rows. Lines, markers and point clouds need a few more; they are all here, so every lane shares the same layouts.

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:groups"
```

### Build them all once

`lessons/01/src/engine/pipelines/layouts.rs` · append at the end of the file

`Layouts` holds them all, made once when the GPU opens; every pipeline borrows from it.

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:layouts"
```

## Code every shader shares

### Declare the three bind groups in WGSL

`lessons/01/src/shaders/scene.wgsl` · new file

The WGSL side of groups 0 to 2, byte for byte like the Rust structs. `place(i, p)` moves point `p` of object `i` into the scene: its rotation and scale, then its position.

```wgsl
--8<-- "lessons/01/src/shaders/scene.wgsl"
```

### Write a colour and a triangle id

`lessons/01/src/shaders/physical.wgsl` · new file

A fragment is one pixel a triangle covers. A face fragment writes two things: its colour to target 0 and its triangle id to target 1.

```wgsl
--8<-- "lessons/01/src/shaders/physical.wgsl"
```

### Turn normals correctly

`lessons/01/src/shaders/normals.wgsl` · new file

A normal is the direction a surface faces. Stretch an object and its normals bend the wrong way if you move them like points, so they get their own rule. Normals also arrive packed in 2 or 4 bytes; these functions unpack them.

```wgsl
--8<-- "lessons/01/src/shaders/normals.wgsl"
```

## What changes every frame

### Hand the renderer its frame

`lessons/01/src/engine/gpu/frame.rs` · new file

The renderer keeps no camera and no clock. Each frame the app hands it the camera matrix, the clear colour and the time. `Binds::set` sets groups 0, 1 and 2 in one call, because every draw needs all three.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:inputs"
```

### Match the WGSL structs byte for byte

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

The shader reads these structs as raw bytes, so field order and padding must match the WGSL exactly. The `const _: () = assert!(...)` blocks check sizes when you compile, so a mismatch fails the build instead of drawing garbage.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:uniforms"
```

### Map the canvas onto a small window

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

Picking, in lesson 12, draws only a small window around the cursor, 19 × 19 pixels for example. This matrix stretches that window to fill the whole target, so the same shaders draw it.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:pick-view"
```

### Create the frame's uniforms

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

One uniform buffer and one bind group per struct above, and a second set for the pick window. They are made once and rewritten every frame.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-uniforms"
```

### Write them every frame

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

`write_buffer` queues the bytes; they land before the frame's commands run. The clipping planes, a block this file adds last, are written only when they changed.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-write"
```

### Prove the window matrix

`lessons/01/src/engine/gpu/frame.rs` · copy this part, append at the end of the file

A 19 × 19 window at (100, 250) maps its corners to ±1.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:tests"
```

### Count the frame's bytes

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

A second `impl` block, this time for a trait we define next, `Lane`. It lets the memory counter see these buffers too.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-lane"
```

### The clipping planes and the field of view

`lessons/01/src/engine/gpu/frame.rs` · append at the end of the file

Clipping, in lesson 18b, cuts the scene with up to 6 planes; every shader binds this block from the start, so none has to change later. The camera sees 60° from top to bottom.

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:clip-uniform"
```

## Lanes and passes

A lane is one kind of drawing, such as the background, the meshes or the lines, with its own buffers and pipelines. A pass is an optional stage of the frame, such as clipping or shading. Both are traits, so the renderer can call every lane and every pass the same way, and a new one is one file plus one line in a list.

### Say what a pick answers

`lessons/01/src/engine/gpu/lane.rs` · new file

A pick asks "what is under the cursor?". The answer can be a whole object, an edge, a face or a control point.

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:pick-mode"
```

### The Lane trait

`lessons/01/src/engine/gpu/lane.rs` · append at the end of the file

A trait is a set of methods a type promises. Every method here has a default body that does nothing, so a lane writes only the hooks it needs.

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:lane-trait"
```

### The lane registry, still empty

`lessons/01/src/engine/gpu/lane.rs` · append at the end of the file

`REGISTRY` lists lanes by the functions that make them. It is empty now; lesson 04b adds the first entry, one line.

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:registry"
```

### Rows for any lane, found by type

`lessons/01/src/engine/gpu/lane.rs` · append at the end of the file

Each registered lane collects its rows in a table of its own type. `LaneRows` keeps one table per type, so the upload needs no field per lane.

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:lane-rows"
```

### Prove one table per type

`lessons/01/src/engine/gpu/lane.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:tests"
```

### What every pass sees

`lessons/01/src/engine/gpu/pass.rs` · new file

Every hook of a pass receives the same three things: the canvas view, the clear colour and the drag tier, which `performance.rs` defines below.

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:frame"
```

### The Pass trait

`lessons/01/src/engine/gpu/pass.rs` · append at the end of the file

Each hook is one moment of the frame: before the faces, inside the face pass, after them, and so on. A pass fills the moments it needs.

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:pass-trait"
```

### The pass list, still empty

`lessons/01/src/engine/gpu/pass.rs` · append at the end of the file

Passes run in the order of `PASSES`. `Nothing` is a pass that does nothing; the next step needs it.

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:passes"
```

### Run every pass in order

`lessons/01/src/engine/gpu/pass.rs` · append at the end of the file

A pass's hook needs the pass and the rest of the `Gpu` at once, both mutable, and Rust forbids borrowing `self` twice. So we take the pass out of its slot, put `Nothing` there, call the hook, and put the pass back.

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:each-pass"
```

## One row per object

Every object gets one row of 96 bytes: its rotation and scale, colour and flags. To select a mesh of a million triangles we rewrite its row, not its triangles.

### The row the shaders read

`lessons/01/src/engine/gpu/instance.rs` · new file

`#[repr(C)]` fixes the field order so the bytes match `Instance` in `scene.wgsl`. Each flag is one bit of `flags`: bit 0 selected, bit 1 hidden, and so on.

```rust
--8<-- "lessons/01/src/engine/gpu/instance.rs:instance"
```

### Prove Rust and WGSL agree

`lessons/01/src/engine/gpu/instance.rs` · copy this part, append at the end of the file

The test reads the field names out of the WGSL text and compares them with the Rust ones.

```rust
--8<-- "lessons/01/src/engine/gpu/instance.rs:tests"
```

### Box a turned object exactly

`lessons/01/src/engine/gpu/hull.rs` · new file

To frame the camera we need each object's world box. Turning the object's own box gives a box that is too big, since its corners swing out. The exact box comes from the corners of the convex hull: the points that stick out farthest in some direction. A dragon of 437,645 points has only 5,583 of them. First, small vector helpers.

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:math"
```

### Read points out of rows

`lessons/01/src/engine/gpu/hull.rs` · append at the end of the file

A vertex row holds a position among other fields. `Points` reads just the positions, without copying the rows.

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:points"
```

### Keep only the extreme points

`lessons/01/src/engine/gpu/hull.rs` · append at the end of the file

Start from the tetrahedron of four far-apart points. Every point outside a face lifts a tent from that face to the point, and points under the tent are dropped. What remains are the hull corners. Past 8,192 corners we give up and keep the plain box.

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:extreme"
```

### Box the kept points

`lessons/01/src/engine/gpu/hull.rs` · append at the end of the file

Place each kept point with the object's matrix and take the lowest and highest value on each axis: that is the world box.

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:boxes"
```

### Prove the box is exact

`lessons/01/src/engine/gpu/hull.rs` · copy this part, append at the end of the file

A beam turned 37° is boxed exactly by its 10 extreme points, where its own box turned is looser.

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:tests"
```

### Collect one file's rows first

`lessons/01/src/engine/gpu/upload.rs` · new file

Loading a file produces rows for many lanes. We collect them all on the CPU and upload each table in one write, instead of thousands of small ones.

```rust
--8<-- "lessons/01/src/engine/gpu/upload.rs:upload"
```

### The row the CPU builds

`lessons/01/src/engine/gpu/objects.rs` · new file

The CPU keeps more than the GPU needs: the exact placement in `f64`, the object's own box and its extreme points.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:rows"
```

### Keep positions exact far from zero

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

The GPU works in `f32`, which keeps about 7 digits. At 1,000,000 mm from the origin a 1 mm step is lost. So the GPU gets each position measured from an anchor, a chosen point near the camera, and the full `f64` position stays on the CPU.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:boxes"
```

### The object table

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

Two GPU buffers, the rows and the positions, plus CPU lists that answer questions without asking the GPU.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table"
```

### Append a file's rows

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

An empty scene binds one placeholder row, because a storage buffer may not be empty. The first upload replaces it; later uploads append.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table-append"
```

### Move the anchor when the camera travels

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

When the camera has moved farther than a quarter of its view distance, we pick a new anchor and rewrite every position, at most once per 200 ms.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:anchor"
```

### Notice when the camera is inside

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

A camera inside a box must see its inner faces, so those rows get `FLAG_INSIDE`. The gumball, the drag handle of lesson 25, draws with one extra row that always stays last.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:inside"
```

### Move one object

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

Moving an object rewrites its row and its position, nothing else.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:placement"
```

### Hide deleted rows instead of freeing them

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

Freeing a row would shift every row after it. So a deleted object's row is hidden and kept: retired for good, or buried until an undo brings it back with one write.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:bury"
```

### Ask where the rows are

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

The box around every live row, and whether a row already sits where a move would put it, both answered from the CPU lists.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:queries"
```

### Colour and flag one row

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

A layer colour or a flag change is one 96-byte write. A dead row can never be shown or selected again.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:colors"
```

### Forget or free every row

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

Reset and release, as for a single buffer, now over every list of the table.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:forget"
```

### Look up one row

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

A row, its box and the anchor, read from the CPU copies, so nothing waits on the GPU.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:lookups"
```

### Prove moves stay small and exact

`lessons/01/src/engine/gpu/objects.rs` · copy this part, append at the end of the file

A 1 mm step at 1,000,000 mm survives only relative to a near anchor. The ignored test uploads two objects to your GPU and moves one.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:tests"
```

### The table is a lane too

`lessons/01/src/engine/gpu/objects.rs` · append at the end of the file

Implementing `Lane` lets the reset, the release and the byte count reach the table the same way they reach every other lane.

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table-lane"
```

## The first draw: a white background

![Clip space is a square from -1 to +1 with y up; the viewport transform turns it into pixels with y down.](illustrations/clip-space.svg)

A shader has two parts: the vertex shader places each corner, the fragment shader colours each covered pixel. After the vertex shader, the screen is a square from -1 to +1 on both axes, called clip space. One triangle with corners (-1, -1), (3, -1) and (-1, 3) covers that whole square, so one draw of three vertices fills the screen.

### The backdrop lane

`lessons/01/src/engine/gpu/backdrop.rs` · new file

The backdrop owns the background shader and its pipeline. `draw(0..3, 0..1)` draws three vertices with no vertex buffer: the shader makes the corners from their index.

```rust
--8<-- "lessons/01/src/engine/gpu/backdrop.rs:backdrop"
```

### A pipeline that ignores depth

`lessons/01/src/engine/gpu/backdrop.rs` · append at the end of the file

The background is behind everything, so it skips the depth test. On a new sample count the lane rebuilds its pipeline, and the cache returns it if it exists.

```rust
--8<-- "lessons/01/src/engine/gpu/backdrop.rs:background-pipeline"
```

### Three corners and a white fill

`lessons/01/src/shaders/background.wgsl` · new file

Three corners from the vertex index, then white, or light grey while the soft shading of lesson 32 is on.

```wgsl
--8<-- "lessons/01/src/shaders/background.wgsl"
```

## One frame

![One frame: take the canvas texture, make a view, record a render pass in an encoder, submit, present.](illustrations/01-05.svg)

### Time the frames

`lessons/01/src/engine/performance.rs` · new file

Every frame is timed. While you drag, a slow GPU may give up some quality to stay smooth; each step down is a tier. Tier 0 is full quality, tier 2 the lightest.

```rust
--8<-- "lessons/01/src/engine/performance.rs:performance"
```

### Move the tier by the median

`lessons/01/src/engine/performance.rs` · append at the end of the file

One slow frame may be a pause, not a slow GPU. So the tier moves by the median of the last few drag frames, at most 5: slower than 33 ms gives up a tier, faster than 20 ms takes one back.

```rust
--8<-- "lessons/01/src/engine/performance.rs:performance-frame"
```

### Read the clock

`lessons/01/src/engine/performance.rs` · append at the end of the file

The same functions twice: once for the browser, once for native tests. `#[cfg(...)]` keeps the right one.

```rust
--8<-- "lessons/01/src/engine/performance.rs:clock"
```

### Prove the tiers

`lessons/01/src/engine/performance.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/performance.rs:tests"
```

### Mark when the module starts

`lessons/01/src/lib.rs` · add the line tagged `register:frame`

Add one line to `run_web`: a named mark on the browser's performance timeline, so you can see how long startup took.

```rust
--8<-- "lessons/01/src/lib.rs:entry"
```

### Encode one frame

`lessons/01/src/engine/gpu/render.rs` · new file

An `impl Gpu` block may sit in any file of the crate. This one records a frame: every pass prepares, the face pass draws, then each pass adds what comes after the faces.

```rust
--8<-- "lessons/01/src/engine/gpu/render.rs:encode"
```

### Open the face pass and draw

`lessons/01/src/engine/gpu/render.rs` · append at the end of the file

The first pass to open clears the screen and draws the backdrop. `face_list` draws nothing yet; lesson 04a adds the meshes.

```rust
--8<-- "lessons/01/src/engine/gpu/render.rs:face-passes"
```

### Write the uniforms first

`lessons/01/src/engine/gpu/present.rs` · new file

Before a frame is recorded, the frame's uniforms and every pass's own get this frame's values, and the inside flags follow the eye.

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:frame-uniforms"
```

### Show a frame on the canvas

`lessons/01/src/engine/gpu/present.rs` · append at the end of the file

The canvas lends one texture per frame. We record the frame into it, `submit` hands the commands to the GPU, and `present` gives the texture back to be shown.

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:present"
```

### Draw a frame into memory

`lessons/01/src/engine/gpu/present.rs` · append at the end of the file

Tests have no canvas. So they draw into a texture, copy it into a buffer the CPU can read, and wait for the copy. Each copied row is padded to a multiple of 256 bytes, as wgpu requires.

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:offscreen"
```

## The Gpu that holds it all

### Bring in the parts

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

`pub use` re-exports a name, so other code writes `gpu::Upload` instead of `gpu::upload::Upload`.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:uses"
```

### The Gpu struct

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

One field per part you wrote: the device, the frame's uniforms and textures, the settings, and one field per lane.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-struct"
```

### One list of the lanes

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

Every loop over the lanes walks this one list. `lane_list!(shared, self)` becomes `[&self.frame as &dyn Lane, &self.objects as &dyn Lane, ...]`, so a new lane is one more line here.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:lane-list"
```

### Count the GPU's bytes

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

Buffers are counted by each lane; frame textures from the canvas size: a 1600 × 1200 canvas at 1x holds 8 bytes per pixel, 15 MB.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-bytes"
```

### Open with a window or without

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

With a window the canvas shows the frames; without one, a test draws into a texture of the size it asks for.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-new"
```

### Build every part, empty

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

Open the device, then make each part in the order it needs the others. Every lane starts empty and at 1 sample.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-build"
```

### Hand a scene to every lane

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

The object table and every registered lane append their part of the upload, the scene box grows, and the sample count is chosen again.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:set-scene"
```

### Rebuild when the sample count changes

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

A pipeline is fixed to one sample count, so switching 1x to 4x means new textures and new pipelines. The cache makes the switch back free.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:retarget"
```

### Decide 1x or 4x for this scene

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

The budget rule from `targets.rs`, fed this GPU, this canvas and whether any solids exist. `solid` stays false until lesson 04a adds the meshes.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:msaa"
```

### Follow the canvas size

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

A new canvas size reconfigures the surface and remakes the frame textures at that size.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:resize"
```

### Forget every row

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

Loading a new scene resets every lane and pass; the buffers stay for the rows about to arrive.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:reset"
```

### Free every buffer

`lessons/01/src/engine/gpu/mod.rs` · append at the end of the file

Closing a scene for good gives the memory back as well.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:release"
```

### List the shaders for the tests

`lessons/01/src/engine/gpu/mod.rs` · copy this part, append at the end of the file

The shader tests walk this list, so each lane lesson adds its shaders here with one line.

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:lane-shaders"
```

## Checkpoint

Run `cargo check`. It compiles, with warnings about code nothing calls yet.

Run `cargo xtest`. You should see `26 passed; 0 failed; 2 ignored`.

Run `cargo xtest -- --ignored --test-threads=1`. The two ignored tests open your real GPU with no window: one feeds it a broken shader and expects the error, the other uploads two objects and moves one. You should see `2 passed`. The browser shows nothing new yet: lesson 12 opens the window that asks this renderer for frames.

## Recap

You can now say what each GPU object is for: surface, adapter, device and queue to talk to the GPU; buffers and textures to hold data; pipelines, built lazily, to draw. Objects live as 96-byte rows measured from an anchor, and lanes and passes are lists that later lessons extend by one line each. Next, the camera gives group 0 its matrix.

Next: [02 · Camera](02-camera.md)
