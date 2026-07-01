# 18 Index buffer

The two triangles have carried us through the whole camera — but they hide a waste. Every triangle
listed its **three** corners in full, so the six-vertex buffer repeated positions a real shape would
share. A cube makes it obvious: 8 corners, but 12 triangles × 3 = **36 vertices** if listed flat —
each corner duplicated four or five times. An **index buffer** fixes this: store the 8 unique corners
**once**, then a list of 36 small integers that say "use corner 3, then 0, then 7…". The GPU reads
the index list, looks each vertex up, and even **caches** the transformed result so a shared corner
is processed once, not five times. This is exactly how every real mesh is drawn — and exactly the
shape the kernel's `GpuMesh` hands us next lesson.

## Why

A draw call has two modes. `draw(0..n)` walks the vertex buffer straight through — fine for a
throwaway triangle, wasteful for anything with shared corners. `draw_indexed(0..m)` walks an
**index buffer** instead, and each index points into the vertex buffer:

```
vertices: [ v0  v1  v2  v3  v4  v5  v6  v7 ]      8 unique corners, stored once
indices:  [ 0 1 2  0 2 3  4 5 6 … ]               36 entries → 12 triangles, just u16 each
draw_indexed(0..36) → GPU reads indices, fetches+caches each vertex, assembles triangles
```

The vertex buffer holds *what* the corners are; the index buffer holds *how* they connect. Memory
drops (8 × 24 B + 36 × 2 B = 264 B vs 36 × 24 B = 864 B here, and the gap widens with size), and the
post-transform vertex cache means each corner's vertex shader runs ~once. The pipeline and shader
don't change at all — indexing happens *before* the vertex shader, so the same `Vertex::layout()`
just works.

## Files we touch

```
src/engine/gpu.rs   # cube vertices + index list; an index buffer; draw_indexed in clear()
```

(The pipeline and `triangle.wgsl` are untouched — indexing is invisible to the shader.)

## Step 1 — the cube data: `src/engine/gpu.rs`

Replace the `TRIANGLES` const (the 6 triangle vertices) with **8 cube corners** plus a **36-entry
index list**. The cube is 1 m wide expressed in mm (`H = 500`), so it sits nicely at the camera's
start distance; each corner gets a distinct colour so the faces read as gradients:

```rust
        const H: f32 = 500.0;                       // half-size, mm → a 1 m cube (lesson 16 units)
        const CUBE: &[Vertex] = &[
            Vertex { position: [-H, -H, -H], color: [0.0, 0.0, 0.0] },   // 0
            Vertex { position: [ H, -H, -H], color: [1.0, 0.0, 0.0] },   // 1
            Vertex { position: [ H,  H, -H], color: [1.0, 1.0, 0.0] },   // 2
            Vertex { position: [-H,  H, -H], color: [0.0, 1.0, 0.0] },   // 3
            Vertex { position: [-H, -H,  H], color: [0.0, 0.0, 1.0] },   // 4
            Vertex { position: [ H, -H,  H], color: [1.0, 0.0, 1.0] },   // 5
            Vertex { position: [ H,  H,  H], color: [1.0, 1.0, 1.0] },   // 6
            Vertex { position: [-H,  H,  H], color: [0.0, 1.0, 1.0] },   // 7
        ];

        // 12 triangles (2 per face), each face's 4 corners split into two triangles.
        const CUBE_IDX: &[u16] = &[
            0, 1, 2,  0, 2, 3,    // −Z  bottom
            4, 5, 6,  4, 6, 7,    // +Z  top
            0, 1, 5,  0, 5, 4,    // −Y  front
            2, 3, 7,  2, 7, 6,    // +Y  back
            1, 2, 6,  1, 6, 5,    // +X  right
            0, 3, 7,  0, 7, 4,    // −X  left
        ];
```

(Winding doesn't matter yet — the pipeline's `cull_mode` is `None`, so every face draws and the depth
test sorts them. Consistent outward winding starts to matter when we add back-face culling and
normals, lesson 21.)

## Step 2 — an index buffer: `src/engine/gpu.rs`

The `Gpu` struct currently has `vertex_buffer` + `num_vertices`. Swap `num_vertices` for an
`index_buffer` and `num_indices`:

```rust
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,   // u16 indices into vertex_buffer
    pub num_indices: u32,
```

In `new()`, build the vertex buffer from `CUBE` (as before, just the new data) and add the index
buffer with `INDEX` usage:

```rust
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cube.vbo"),
            contents: bytemuck::cast_slice(CUBE),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cube.ibo"),
            contents: bytemuck::cast_slice(CUBE_IDX),   // &[u16] → &[u8]
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = CUBE_IDX.len() as u32;        // 36
```

Return `index_buffer` and `num_indices` in the struct instead of `num_vertices`.

## Step 3 — draw it indexed: `src/engine/gpu.rs`

In `clear()`, after binding the vertex buffer, **bind the index buffer** and switch `draw` →
`draw_indexed`:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..self.num_indices, 0, 0..1);
```

`set_index_buffer` takes the format (`Uint16` — our indices fit in `u16`). `draw_indexed(indices,
base_vertex, instances)`: the `0` base-vertex is added to every index (handy when packing many meshes
into one buffer — lesson 31); `0..1` is the single instance, same as before.

## Step 4 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A solid colour-cube replaces the two triangles. Orbit it (right-drag) — the quaternion camera sails
around it, the depth test keeps near faces in front, and `1`–`7` snap to the named views (now you can
*see* "Top" vs "Front" on a real solid). It's drawn from **8 vertices and 36 indices**, not 36
vertices — the memory layout every real mesh uses.

## Recap

```
Ch 17: the camera was finished — but geometry was still 6 flat triangle vertices.
Ch 18: an index buffer separates "what the corners are" (8 vertices) from "how they connect"
       (36 indices). draw_indexed walks the index list; the shader/pipeline are unchanged.
       This is the vertex-buffer + index-buffer pair the kernel's GpuMesh gives us next.
```

Edited: `engine/gpu.rs` (cube `CUBE`/`CUBE_IDX` replace `TRIANGLES`; `index_buffer`/`num_indices`
replace `num_vertices`; `draw_indexed` in `clear()`).

## Next

`19-link-the-kernel.md` — add `session_rust` as a dependency and draw your first **real `Mesh`** with
one call: `mesh.gpu_mesh(&device)` returns a cached `GpuMesh { vbo, ibo, index_count }` built once via
`to_render()`. The hand-rolled cube becomes a kernel mesh, and the vertex/index pair you just learned
is exactly what it hands back.
