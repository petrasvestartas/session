# 005 · Resize and device pixels

A canvas has two sizes. Its CSS size is what the page lays out, in CSS pixels. Its drawing size is the texture the GPU paints, in device pixels; on a HiDPI screen that is two or three device pixels per CSS pixel. When the window changes, both change, and every texture that follows the canvas must be remade. This lesson follows the size, and decides how many samples each pixel gets.

## Follow the canvas

`lessons/005/src/lib.rs` · type this, the lines tagged `register:resize` in `adopt` and `redraw`, then append

`desired_canvas_size` reads the canvas's CSS size and multiplies by the device pixel ratio. `fit_canvas` applies it once, when the state arrives. `resize_held` applies it before a frame; a resize that came too soon is held, and the frame waits for the next redraw. `page_hidden` skips frames for a tab you cannot see.

```rust
--8<-- "lessons/005/src/lib.rs:005-adopt"
```

```rust
--8<-- "lessons/005/src/lib.rs:005-redraw"
```

```rust
--8<-- "lessons/005/src/lib.rs:005-resize"
```

## The state resizes

`lessons/005/src/state.rs` · type this, the line tagged `register:resize` in `render`, then append

A window drag resizes on every frame; `resize` remakes the textures at most every 100 ms and says no in between. `logical_size` is the CSS size; `follow_logical_size` notices when it changed without the device size changing, as when a phone's toolbar slides away.

```rust
--8<-- "lessons/005/src/state.rs:005-hook"
```

```rust
--8<-- "lessons/005/src/state.rs:005-resize"
```

## Samples per pixel

`lessons/005/src/engine/gpu/targets.rs` · type this, the lines tagged `register:msaa`, then append

MSAA, multisample anti-aliasing, draws 4 samples per pixel and averages them, so an edge that crosses a pixel colours it in proportion. It costs 4 times the texture memory, so `samples_for` allows it only with solid geometry, within a pixel budget that depends on the GPU type, and below device scale 2, where the pixels are small enough already. `?msaa=4` forces it, `?msaa=1` forbids it. The tagged lines make the 4x colour texture and a second depth texture, and a 1x1 placeholder for the sample count not in use, so the shaders can bind both.

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-fields"
```

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-textures"
```

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-init"
```

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-destroy"
```

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-target"
```

```rust
--8<-- "lessons/005/src/engine/gpu/targets.rs:005-msaa"
```

`lessons/005/src/engine/gpu/device.rs` · type this, the lines tagged `register:msaa`

The device type, discrete or integrated, comes out of `open` with the rest.

```rust
--8<-- "lessons/005/src/engine/gpu/device.rs:005-field"
```

```rust
--8<-- "lessons/005/src/engine/gpu/device.rs:005-setup"
```

## The view

`lessons/005/src/engine/gpu/view.rs` · type this, append

`View` is the set of display settings; today one, the forced sample count, and every later lesson adds its own line. `device_pixel_ratio` is the browser's ratio, capped by `?dpr=`. `reduce` is a switch a later lesson throws after a lost GPU device: from then on the page draws at scale 1 without MSAA until reload.

```rust
--8<-- "lessons/005/src/engine/gpu/view.rs:005-view"
```

## The GPU resizes

`lessons/005/src/engine/gpu/mod.rs` · type this, the lines tagged `register:view` and `register:msaa`, then append

`resize` reconfigures the surface and calls `retarget`, which remakes the textures when the size or the sample count changed. `samples_wanted` asks `samples_for` with what the scene holds; the tagged lines are how the mesh, line and marker lessons report solid geometry.

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-use"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-field"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-type"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-setup"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-init"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-init-type"
```

```rust
--8<-- "lessons/005/src/engine/gpu/mod.rs:005-resize"
```

## Checkpoint

Run `trunk serve` and drag the window edge: the grey canvas follows it without a blur. Add `?dpr=1` to the URL on a HiDPI screen: the canvas is drawn at one device pixel per CSS pixel. Add `?msaa=4`: the console still says `viewer init OK`, and the 4x textures exist; you will see their effect on the first edge you draw.

## Recap

The canvas has a CSS size and a device size; `desired_canvas_size` joins them through the device pixel ratio. A resize remakes the textures, at most every 100 ms. `samples_for` decides 1 or 4 samples per pixel from the geometry, the GPU type and the pixel count. Next: the pipeline cache, the object every draw is made from.

Next: [04a · Meshes on the GPU](04a-meshes.md)
