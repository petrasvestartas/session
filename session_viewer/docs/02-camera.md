# 02 · Camera

The orbit camera turns mouse pixels and wheel steps into a view-projection matrix, and the backdrop gains a floor grid drawn through it. The camera is complete here, standard views and the remembered pose included.

![Orbit turns the orientation about the target, pan slides the target across the camera's own plane, and the wheel scales the distance; the view-projection is rebuilt from those three every frame.](illustrations/camera-basis.svg)

## Step 1 · src/lib.rs

Add the camera module.

`lessons/02/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/lib.rs:camera-mod"
```

## Step 2 · src/camera.rs

Scene units, the near plane as a fraction of the distance, and the seven standard views.

`lessons/02/src/camera.rs` · type this, new file

```rust
--8<-- "lessons/02/src/camera.rs:units"
```

## Step 3 · src/camera.rs

The orbit camera, and the pose a view remembers: where the camera looks from and at.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:camera"
```

## Step 4 · src/camera.rs

Read the current pose, and make a camera at the isometric view.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:camera-new"
```

## Step 5 · src/camera.rs

Orbit, pan and zoom by mouse pixels and wheel steps.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:navigate"
```

## Step 6 · src/camera.rs

The world ray under a cursor pixel, for perspective and orthographic views.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:ray"
```

## Step 7 · src/camera.rs

Zoom toward the cursor, and switch projection while keeping what was on screen in view.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:zoom-at"
```

## Step 8 · src/camera.rs

The view-projection matrix, measured from the anchor, with near and far swapped for reverse-Z depth.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:view-proj"
```

## Step 9 · src/camera.rs

Turn to a standard view in orthographic projection, or reset the camera.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:standard-views"
```

## Step 10 · src/camera.rs

Fit a box in view, widen the far plane for boxes that arrive later, and recompute the eye.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:fit"
```

## Step 11 · src/camera.rs

The pose and depth tests.

`lessons/02/src/camera.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:tests"
```

## Step 12 · src/camera.rs

The distance after some wheel steps: 0.9 per step, at most ten per event.

`lessons/02/src/camera.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:zoom-distance"
```

## Step 13 · src/camera.rs

The ray and wheel tests.

`lessons/02/src/camera.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:wheel-tests"
```

## Step 14 · src/shaders/grid.wgsl

The floor grid and the three axes, each endpoint placed from its vertex index alone.

`lessons/02/src/shaders/grid.wgsl` · type this, new file

```wgsl
--8<-- "lessons/02/src/shaders/grid.wgsl"
```

## Step 15 · src/engine/gpu/backdrop.rs

Give the backdrop its grid shader and pipeline beside the background ones.

`lessons/02/src/engine/gpu/backdrop.rs` · add the lines tagged `register:camera`

```rust
--8<-- "lessons/02/src/engine/gpu/backdrop.rs:backdrop"
```

## Step 16 · src/engine/gpu/backdrop.rs

Draw the grid lines, through a pipeline that hides the lines behind geometry.

`lessons/02/src/engine/gpu/backdrop.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/engine/gpu/backdrop.rs:grid"
```

## Step 17 · src/engine/gpu/render.rs

The backdrop draws the grid right after the background.

`lessons/02/src/engine/gpu/render.rs` · add the line tagged `register:camera`

```rust
--8<-- "lessons/02/src/engine/gpu/render.rs:face-passes"
```

## Step 18 · src/engine/gpu/render.rs

Draw the grid only while it is switched on.

`lessons/02/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/02/src/engine/gpu/render.rs:grid-list"
```

Run `cargo check` in `lessons/02/`.

## Check

`cargo xtest camera` runs the ten camera tests: poses, the near and far planes, rays and the wheel. The screen shows nothing new yet; from lesson 12 the grid turns with the mouse.

## Next

[03 · Object rows and identity](03-identity.md)
