# 18 Index buffer

Every triangle lists its **three** corners in full — a cube's 36 vertices (12 triangles × 3) if flat,
each duplicated 4-5×. An **index buffer** fixes this: store 8 corners **once**, then 36 integers
saying "corner 3, 0, 7…"; GPU fetches and **caches** each vertex once — how every real mesh is drawn,
and the shape `GpuMesh` hands us next.

## Why

A draw call has two modes: `draw(0..n)` walks the vertex buffer straight through, wasteful once
corners are shared; `draw_indexed(0..m)` walks an **index buffer** instead, each index pointing into
the vertex buffer:

```
vertices: [ v0  v1  v2  v3  v4  v5  v6  v7 ]      8 unique corners, stored once
indices:  [ 0 1 2  0 2 3  4 5 6 … ]               36 entries → 12 triangles, just u16 each
draw_indexed(0..36) → GPU reads indices, fetches+caches each vertex, assembles triangles
```

Vertex buffer holds *what* corners are; index buffer holds *how* they connect. Memory drops (264 B vs
864 B here, gap widening with size); post-transform cache runs each vertex shader ~once. Pipeline and
shader don't change — indexing happens *before* the vertex shader.

## Files we touch

```
src/engine/gpu.rs   # cube vertices + index list; an index buffer; draw_indexed in clear()
```

(Pipeline and `triangle.wgsl` untouched — indexing is invisible to the shader.)

## Step 1 — the cube data: `src/engine/gpu.rs`

Replace `TRIANGLES` (6 vertices) with **8 cube corners** + a **36-entry index list**. Cube is 1 m wide
in mm (`H = 500`), sized for the camera's start distance; each corner gets a distinct colour so faces
read as gradients:

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

(Winding doesn't matter yet: `cull_mode` is `None`, every face draws, depth test sorts them. Matters
once back-face culling + normals arrive, lesson 21.)

## Step 2 — an index buffer: `src/engine/gpu.rs`

`Gpu` currently has `vertex_buffer` + `num_vertices`. Swap `num_vertices` for `index_buffer` and
`num_indices`:

```rust
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,   // u16 indices into vertex_buffer
    pub num_indices: u32,
```

In `new()`, build the vertex buffer from `CUBE` as before, and add the index buffer with `INDEX`
usage:

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

In `clear()`, after the vertex buffer, **bind the index buffer** and switch `draw` → `draw_indexed`:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..self.num_indices, 0, 0..1);
```

`set_index_buffer` takes the format (`Uint16` — indices fit in `u16`); `draw_indexed(indices,
base_vertex, instances)` adds `0` base-vertex to every index (packing many meshes — lesson 31), `0..1`
is the single instance.

## Step 4 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A solid colour-cube replaces the two triangles. Orbit it (right-drag), `1`–`7` snap to named views —
now you *see* "Top" vs "Front" on a real solid. Drawn from **8 vertices and 36 indices**, the memory
layout every real mesh uses.

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

`19-link-the-kernel.md` — draw your first **real `Mesh`**: `mesh.gpu_mesh(&device)` returns a cached
`GpuMesh { vbo, ibo, index_count }` via `to_render()`. The hand-rolled cube becomes a kernel mesh —
same vertex/index pair you just learned.
