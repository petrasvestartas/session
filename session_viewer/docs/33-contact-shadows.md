# 33 · Contact shadows that follow object size
Each object carries its own ambient-occlusion radius, so a small part and a large one both get a soft contact shadow that fits.

## Step 1 · src/engine/gpu/instance.rs
Each object row carries a contact radius where the padding word used to be.

`lessons/33/src/engine/gpu/instance.rs` · edit · type this

Replaces the line `pub _pad0: f32,` in `lessons/32/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/instance.rs:step-1a"
```

Replaces the line `_pad0: 0.0,` in `lessons/32/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/instance.rs:step-1b"
```

Replaces the line `let rust = ["model", "color", "flags", "_pad0", "spacing"…` in `lessons/32/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/instance.rs:step-1c"
```

## Step 2 · src/engine/gpu/objects.rs
The radius is 5% of the object's diagonal, so it survives a move and scales with a resize.

`lessons/33/src/engine/gpu/objects.rs` · edit · type this

Added after the line `r.bounds.transformed(&r.place)` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2a"
```

Replaces the line `_pad0: 0.0,` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2b"
```

Added after the line `instance.model = model;` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2c"
```

Added after the line `#[test]` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2d"
```

Added after the line `assert_eq!(gpu.objects.len(), 2);` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2e"
```

Added after the line `assert_eq!(second.min_point()[1], -1.0, "the neighbour di…` in `lessons/32/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/objects.rs:step-2f"
```

## Step 3 · src/engine/gpu/render.rs
The projected-triangle tiles are handed to the ambient pass.

`lessons/33/src/engine/gpu/render.rs` · edit · type this

Added after the line `&self.targets,` in `lessons/32/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/33/src/engine/gpu/render.rs:step-3"
```

## Step 4 · src/engine/gpu/ssao.rs
Replace the whole file: the ambient pass samples a hemisphere per pixel and blends it with a contact term.

`lessons/33/src/engine/gpu/ssao.rs` · replace the whole file · type this

```rust
--8<-- "lessons/33/src/engine/gpu/ssao.rs:step-4"
```

## Step 5 · src/shaders/scene.wgsl
The shared scene layout names the new radius field.

`lessons/33/src/shaders/scene.wgsl` · edit · type this

Replaces the line `_pad0: f32,` in `lessons/32/src/shaders/scene.wgsl`

```wgsl
--8<-- "lessons/33/src/shaders/scene.wgsl:step-5b"
```

## Step 6 · src/shaders/project_triangles.wgsl
The projection pass writes each triangle's contact radius next to its depth slope.

`lessons/33/src/shaders/project_triangles.wgsl` · edit · type this

Replaces the line `_pad0: f32,` in `lessons/32/src/shaders/project_triangles.wgsl`

```wgsl
--8<-- "lessons/33/src/shaders/project_triangles.wgsl:step-6a"
```

Replaces the line `out.gradient = vec4<f32>(gradient, nearest, 0.0);` in `lessons/32/src/shaders/project_triangles.wgsl`

```wgsl
--8<-- "lessons/33/src/shaders/project_triangles.wgsl:step-6b"
```

## Step 7 · src/shaders/ssao.wgsl
Replace the whole file: the shader reconstructs positions from depth and softens the contact by each object's radius.

`lessons/33/src/shaders/ssao.wgsl` · replace the whole file · type this

```wgsl
--8<-- "lessons/33/src/shaders/ssao.wgsl:step-8"
```

## Check

Run `trunk serve` in `lessons/33/` and open <http://127.0.0.1:8770/>.

Load a scene with several solids of different sizes; each one sits in a soft shadow proportional to itself, and moving one keeps its shadow.

## Next

[34 · Polyline options, ordered input and a remembered view](34-polyline-options.md)
