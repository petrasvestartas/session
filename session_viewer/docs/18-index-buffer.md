# 18 Index buffer

Every triangle lists its **three** corners in full — a cube's 36 vertices (12 triangles × 3) if flat,
each duplicated 4-5×. An **index buffer** fixes this: store 8 corners **once**, then 36 integers
saying "corner 3, 0, 7…"; GPU fetches and **caches** each vertex once — how every real mesh is drawn,
and the shape `GpuMesh` hands us next.

<svg viewBox="0 0 680 160" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="without indices a cube stores 36 vertices with each corner duplicated; with an index buffer 8 vertices are stored once and 36 small integers reference them" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="160" y="16" fill="#888" text-anchor="middle">draw(0..36) — vertices only</text>
  <g fill="none" stroke="#e0b040" stroke-width="1.1">
    <rect x="20" y="26" width="26" height="20"/><rect x="48" y="26" width="26" height="20"/><rect x="76" y="26" width="26" height="20"/><rect x="104" y="26" width="26" height="20"/><rect x="132" y="26" width="26" height="20"/><rect x="160" y="26" width="26" height="20"/><rect x="188" y="26" width="26" height="20"/><rect x="216" y="26" width="26" height="20"/><rect x="244" y="26" width="26" height="20"/><rect x="272" y="26" width="26" height="20"/>
  </g>
  <g fill="#e0b040" text-anchor="middle" font-size="9"><text x="33" y="40">v3</text><text x="61" y="40">v0</text><text x="89" y="40">v7</text><text x="117" y="40">v3</text><text x="145" y="40">v7</text><text x="173" y="40">v0</text><text x="201" y="40">v3</text><text x="229" y="40">v1</text><text x="257" y="40">v0</text><text x="285" y="40">…36</text></g>
  <text x="160" y="66" fill="#666" text-anchor="middle" font-size="10">v3 appears 5× — 5 full 24-byte copies, shaded 5×</text>
  <text x="500" y="16" fill="#888" text-anchor="middle">draw_indexed(0..36) — 8 vertices + 36 u16</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2">
    <rect x="380" y="26" width="30" height="20"/><rect x="412" y="26" width="30" height="20"/><rect x="444" y="26" width="30" height="20"/><rect x="476" y="26" width="30" height="20"/><rect x="508" y="26" width="30" height="20"/><rect x="540" y="26" width="30" height="20"/><rect x="572" y="26" width="30" height="20"/><rect x="604" y="26" width="30" height="20"/>
  </g>
  <g fill="#6fb3ff" text-anchor="middle" font-size="9"><text x="395" y="40">v0</text><text x="427" y="40">v1</text><text x="459" y="40">v2</text><text x="491" y="40">v3</text><text x="523" y="40">v4</text><text x="555" y="40">v5</text><text x="587" y="40">v6</text><text x="619" y="40">v7</text></g>
  <text x="500" y="62" fill="#888" text-anchor="middle" font-size="10">indices: 3 0 7 · 3 7 0 · 3 1 0 · … (36 small ints)</text>
  <g stroke="#6fb3ff" stroke-width="0.9" opacity="0.7"><path d="M 430,70 Q 470,88 491,48" fill="none" marker-end="url(#ah18)"/><path d="M 545,70 Q 520,90 495,48" fill="none" marker-end="url(#ah18)"/></g>
  <text x="500" y="86" fill="#666" text-anchor="middle" font-size="10">each index POINTS at a vertex — stored once, cached once</text>
  <text x="340" y="130" fill="#555" text-anchor="middle" font-size="10">cube: 864 B of vertices → 192 B + 72 B of indices; on real meshes the win compounds with the post-transform cache</text>
  <defs><marker id="ah18" markerWidth="7" markerHeight="7" refX="5" refY="2.5" orient="auto"><path d="M0,0 L5,2.5 L0,5 Z" fill="#6fb3ff"/></marker></defs>
</svg>

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
