# 32 · Ambient occlusion

Ambient occlusion darkens creases and contacts by how much nearby geometry hides each point from the sky. Each object row already carries a radius, 5% of its half-diagonal, so a bolt and a building both get a contact shadow that fits.

![Contact shadows at the column bases of the floor model](screenshots/ssao-floor.png)

## Step 1 · registration lines

One line in `PASSES` adds the ambient pass, and two module lines add its file and the pass timer.

`lessons/32/src/engine/gpu/pass.rs` · type the line tagged `register:ambient`

```rust
--8<-- "lessons/32/src/engine/gpu/pass.rs:passes"
```

`lessons/32/src/engine/gpu/mod.rs` · type the lines tagged `register:ssao` and `register:timing`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:modules"
```

Copy the lines tagged `register:gtao` from these files of `lessons/32/`:

- `src/engine/gpu/mod.rs`: the `timer` field of `Gpu` and its `None` in the constructor.
- `src/engine/gpu/present.rs`: resolve the timer before submit and collect it after the readback.
- `src/engine/gpu/render.rs`: a mark between the frame's passes.
- `src/engine/gpu/surface_outline.rs` and `src/engine/gpu/clip.rs`: marks inside the outline and clipping passes.

## Step 2 · src/engine/gpu/timing.rs

New file: a pass timer that stamps the GPU clock at named marks and keeps milliseconds per span.

`lessons/32/src/engine/gpu/timing.rs` · type this, new file

```rust
--8<-- "lessons/32/src/engine/gpu/timing.rs:pass-timer"
```

## Step 3 · src/engine/gpu/render.rs

A second `impl Gpu` block: `mark` stamps the clock only when a benchmark installed a timer.

`lessons/32/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/render.rs:pass-marks"
```

## Step 4 · src/engine/gpu/present.rs

A second `impl Gpu` block: resolve the timestamps before submit and read them back after, natively only.

`lessons/32/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/present.rs:timer"
```

## Step 5 · src/shaders/ambient_geometry.wgsl

New file: the object rows and mesh buffers the AO shader reads, and the exact face normal under a pixel.

`lessons/32/src/shaders/ambient_geometry.wgsl` · type this, new file

```wgsl
--8<-- "lessons/32/src/shaders/ambient_geometry.wgsl:ambient-geometry"
```

## Step 6 · src/shaders/ssao.wgsl

New file: the bindings of every AO pass, the camera uniform, and the full-screen triangle they all draw.

`lessons/32/src/shaders/ssao.wgsl` · type this, new file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-bindings"
```

## Step 7 · src/shaders/ssao.wgsl

Rebuild a pixel's world point and ray, with a virtual floor under every empty pixel, and pack normals into two numbers.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-rays"
```

## Step 8 · src/shaders/ssao.wgsl

Write the first pyramid level, position, packed radius and normal, and read an occluder back from any level.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-prepare"
```

## Step 9 · src/shaders/ssao.wgsl

Floor shadows: how high the solids around each floor point rise above it, in 24 directions.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-ground"
```

## Step 10 · src/shaders/ssao.wgsl

The GTAO horizon search: four slices per pixel, turned into the share of sky that is blocked.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-horizons"
```

## Step 11 · src/shaders/ssao.wgsl

Two blurs that stay on one surface, the second blending in last frame's result where the same surface was seen.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-filter"
```

## Step 12 · src/shaders/ssao.wgsl

Back to full resolution, with its own value for each MSAA sample that sees a different surface.

`lessons/32/src/shaders/ssao.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl:ao-upsample"
```

## Step 13 · src/shaders/ambient_depth.wgsl

New file: build each coarser pyramid level, and mark the tiles that have geometry nearby.

`lessons/32/src/shaders/ambient_depth.wgsl` · type this, new file

```wgsl
--8<-- "lessons/32/src/shaders/ambient_depth.wgsl:ambient-depth"
```

## Step 14 · src/shaders/ambient_composite.wgsl

New file: darken the frame by the AO cache, and correct single MSAA samples inside flagged tiles.

`lessons/32/src/shaders/ambient_composite.wgsl` · type this, new file

```wgsl
--8<-- "lessons/32/src/shaders/ambient_composite.wgsl:ambient-composite"
```

## Step 15 · src/engine/gpu/ssao.rs

New file: the pipelines the effect needs, and two helpers for bind group layout entries.

`lessons/32/src/engine/gpu/ssao.rs` · type this, new file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-layouts"
```

## Step 16 · src/engine/gpu/ssao.rs

Compile every AO pipeline for one colour format and one sample count.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-pipelines"
```

## Step 17 · src/engine/gpu/ssao.rs

Keep the 1x and 4x pipelines: natively compiled on first use, in the browser during idle time.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-cache"
```

## Step 18 · src/engine/gpu/ssao.rs

Small helpers: an owned texture, the pyramid bind group, a colour attachment, and the AO resolution.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-images"
```

## Step 19 · src/engine/gpu/ssao.rs

The images and buffers at one canvas size; `impl Ssao` opens with the floor height under the visible solids.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-struct"
```

## Step 20 · src/engine/gpu/ssao.rs

Inside `impl Ssao`: allocate the pyramid, the working images, the history and the 4x correction buffers.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-new"
```

## Step 21 · src/engine/gpu/ssao.rs

Inside `impl Ssao`: report the memory, and tell when a resize or an MSAA change needs new images.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-bytes"
```

## Step 22 · src/engine/gpu/ssao.rs

Record every AO pass when the camera or scene changed, then blend the cache; the closing brace ends `impl Ssao`.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-draw"
```

## Step 23 · src/engine/gpu/ssao.rs

The screen rectangle of the solids, the 1x and 4x shader source, the pixel rays, and a precise inverse matrix.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-math"
```

## Step 24 · src/engine/gpu/ssao.rs

Tests: contact shading and memory release, no compiling after toggles, and the same image through a drag.

`lessons/32/src/engine/gpu/ssao.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ssao-tests"
```

## Step 25 · src/engine/gpu/ssao.rs

The pass: allocate on switch-on, draw after the faces at the same quality while navigating, drop the images on switch-off.

`lessons/32/src/engine/gpu/ssao.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs:ambient-pass"
```

## Step 26 · src/engine/gpu/ambient_warm.rs

New file: the browser's idle-time compile job, and the two calls `ssao.rs` makes into it.

`lessons/32/src/engine/gpu/ambient_warm.rs` · type this, new file

```rust
--8<-- "lessons/32/src/engine/gpu/ambient_warm.rs:warm-job"
```

## Step 27 · src/engine/gpu/ambient_warm.rs

Request an idle callback that compiles one sample count, and ask again until both are ready.

`lessons/32/src/engine/gpu/ambient_warm.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/32/src/engine/gpu/ambient_warm.rs:warm-idle"
```

Copy `tests/ambient-lighting.cjs` from `lessons/32/`: a browser check of real WebGPU, contact lighting, projection changes and released memory.

Run `cargo check` in `lessons/32/`.

## Check

`cargo check` compiles, and `cargo xtest --lib ssao` passes. Serve the lesson, open a scene and press G: creases and the floor under each solid darken, and the shading stays the same while you orbit. The [ambient occlusion reference](ssao.md) lists its resolution, memory and timings.
