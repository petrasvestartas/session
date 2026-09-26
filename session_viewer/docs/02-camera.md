# 02 · Camera

Every shader multiplies each point by one matrix, group 0, which maps the world onto the screen. You never think in matrices, though: you drag to orbit, drag to pan, and roll the wheel to zoom. So the camera stores three things you can picture, a target point, a distance and an orientation, and rebuilds the matrix from them every frame.

Each gesture changes one of the three. Orbit turns the orientation, pan slides the target, the wheel scales the distance. For example, at distance 3 m with the 60° view from lesson 01, the picture is 2 × 3 × tan 30° = 3.46 m tall at the target.

![Orbit turns the orientation about the target, pan slides the target across the camera's own plane, and the wheel scales the distance; the view-projection is rebuilt from those three every frame.](illustrations/camera-basis.svg)

This lesson writes the whole camera, standard views and zoom-to-cursor included, and a floor grid drawn through it.

## The camera

### Add the module

`lessons/02/src/lib.rs` · append at the end of the file

The camera is its own file; this line brings it into the crate.

```rust
--8<-- "lessons/02/src/lib.rs:camera-mod"
```

### Units and the named views

`lessons/02/src/camera.rs` · new file

The camera works in meters; a scene file may be written in millimetres, so `Unit` converts. The near plane is the closest distance the camera sees, one ten-thousandth of the distance to the target: 0.3 mm at 3 m. `View` names the seven standard views a key will jump to.

```rust
--8<-- "lessons/02/src/camera.rs:units"
```

### Store target, distance and orientation

`lessons/02/src/camera.rs` · append at the end of the file

A quaternion stores a rotation in four numbers. Unlike three angles, it never locks up when you look straight down. The eye position and the up direction are computed from the three stored values, never set by hand.

```rust
--8<-- "lessons/02/src/camera.rs:camera"
```

### Start at the isometric view

`lessons/02/src/camera.rs` · append at the end of the file

Turn 30° about the vertical axis, then tilt 30° down: the familiar three-quarter view.

```rust
--8<-- "lessons/02/src/camera.rs:camera-new"
```

### Orbit, pan and zoom

`lessons/02/src/camera.rs` · append at the end of the file

Orbit turns 0.005 radians per mouse pixel, so a 100-pixel drag turns about 29°. Pan moves the target by 0.15 % of the distance per pixel, so a pixel covers more when you are far away. Zoom multiplies the distance.

```rust
--8<-- "lessons/02/src/camera.rs:navigate"
```

### Shoot a ray through a pixel

`lessons/02/src/camera.rs` · append at the end of the file

A ray is a start point and a direction. The cursor pixel becomes a point on the plane through the target. In perspective, where far things look smaller, every ray starts at the eye; in orthographic, where size does not shrink with distance, all rays are parallel. Zoom-to-cursor needs it now, picking in lesson 12.

```rust
--8<-- "lessons/02/src/camera.rs:ray"
```

### Zoom toward the cursor

`lessons/02/src/camera.rs` · append at the end of the file

Zooming in by 10 % also moves the target 10 % of the way toward the point under the cursor, so that point stays under the cursor. Switching from orthographic to perspective refits the view to what was on screen.

```rust
--8<-- "lessons/02/src/camera.rs:zoom-at"
```

### Build the matrix

`lessons/02/src/camera.rs` · append at the end of the file

Read `projection * view * scale` from right to left: scale scene units to meters, turn the world so the eye sits at the origin looking ahead, then project onto the screen. Near and far are swapped for reverse-Z, and everything is measured from the anchor, as the object rows are.

```rust
--8<-- "lessons/02/src/camera.rs:view-proj"
```

### Jump to a standard view

`lessons/02/src/camera.rs` · append at the end of the file

Front, top, iso and the rest set the orientation directly and switch to orthographic, so a drawing measures the same anywhere on screen.

```rust
--8<-- "lessons/02/src/camera.rs:standard-views"
```

### Fit a box in view

`lessons/02/src/camera.rs` · append at the end of the file

Look at the box centre, then back off until all eight corners are inside the picture, plus 5 %. A box that arrives later only widens the far plane, so the view does not jump.

```rust
--8<-- "lessons/02/src/camera.rs:fit"
```

### Prove poses and depth

`lessons/02/src/camera.rs` · copy this part, append at the end of the file

The near plane cuts just ahead of the eye at any distance, and orthographic depth still tells apart two faces 4 mm apart.

```rust
--8<-- "lessons/02/src/camera.rs:tests"
```

### Turn wheel steps into a distance

`lessons/02/src/camera.rs` · append at the end of the file

Each wheel step multiplies the distance by 0.9: 3 m becomes 2.7 m. A fast wheel sends many steps in one event, so we count at most ten, and the distance never reaches zero.

```rust
--8<-- "lessons/02/src/camera.rs:zoom-distance"
```

### Prove rays and the wheel

`lessons/02/src/camera.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/02/src/camera.rs:wheel-tests"
```

## A floor grid

The grid is 22 floor lines, 1 m apart over 10 m, and three short axes: x red, y green, z blue. Like the background, it needs no vertex buffer.

### Place 50 vertices from their index

`lessons/02/src/shaders/grid.wgsl` · new file

Vertex 0 and 1 are the two ends of the first line, vertex 2 and 3 the next, and so on. From its index alone each vertex works out which line it belongs to and which end it is, then goes through the camera matrix.

```wgsl
--8<-- "lessons/02/src/shaders/grid.wgsl"
```

### Give the backdrop a grid

`lessons/02/src/engine/gpu/backdrop.rs` · add the lines tagged `register:camera`

Eight lines, each tagged: the grid shader in the test list, two fields, making them, storing them, and rebuilding the grid pipeline on a new sample count.

```rust
--8<-- "lessons/02/src/engine/gpu/backdrop.rs:backdrop"
```

### Draw lines that hide behind geometry

`lessons/02/src/engine/gpu/backdrop.rs` · append at the end of the file

The grid tests depth but never writes it, so a solid hides the grid and the grid never hides a solid.

```rust
--8<-- "lessons/02/src/engine/gpu/backdrop.rs:grid"
```

### Draw it after the background

`lessons/02/src/engine/gpu/render.rs` · add the line tagged `register:camera`

The grid draws in the face pass, right after the background, so the depth test can hide it behind solids.

```rust
--8<-- "lessons/02/src/engine/gpu/render.rs:face-passes"
```

### Only while it is switched on

`lessons/02/src/engine/gpu/render.rs` · append at the end of the file

`?nogrid` in the page address turns it off.

```rust
--8<-- "lessons/02/src/engine/gpu/render.rs:grid-list"
```

## Checkpoint

Run `cargo xtest camera`. You should see `10 passed`: poses, near and far planes, rays and the wheel. The whole suite, `cargo xtest`, now shows `36 passed; 0 failed; 2 ignored`. From lesson 12 on, the grid turns under your mouse.

## Recap

The camera stores a target, a distance and an orientation; each gesture changes one, and the matrix is rebuilt from them every frame. A ray through a pixel keeps the point under the cursor fixed while zooming. The grid shows that group 0 now holds a real camera. Next, every object's row gets the edits a click will need.

Next: [03 · Object rows and identity](03-identity.md)
