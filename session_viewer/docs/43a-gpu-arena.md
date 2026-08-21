# 43a Reconcile I — a per-object GPU arena

> **Big picture.** *Phase 6 — the `.pb` file becomes the live source of truth, like a real CAD app.*
> The plan: reload only what changed (43b), save only when something changed (44), watch for external
> edits (45). None of that is possible while the GPU side is all-or-nothing — lesson 30's arena is one
> flat `Vec`, rebuilt wholesale; there is no way to touch *one object's* bytes. So before any diffing,
> this lesson upgrades the arena: every object gets an addressable slice of the GPU buffers that can be
> freed and replaced **without disturbing its neighbours**.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the static flat arena is rebuilt wholesale; the free-list arena maps each guid to a range that can be freed and reused individually" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="18" fill="#888" text-anchor="middle">lesson 30 — static arena</text>
  <rect x="20" y="28" width="260" height="26" fill="none" stroke="#3a3a3a"/>
  <text x="150" y="45" fill="#666" text-anchor="middle">all objects, concatenated once</text>
  <text x="150" y="74" fill="#888" text-anchor="middle" font-size="10">any change → rebuild + re-upload EVERYTHING</text>
  <text x="480" y="18" fill="#888" text-anchor="middle">43a — free-list arena</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2">
    <rect x="360" y="28" width="70" height="26"/><rect x="430" y="28" width="90" height="26"/><rect x="520" y="28" width="60" height="26"/><rect x="580" y="28" width="80" height="26"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle" font-size="10">
    <text x="395" y="45">guid A</text><text x="475" y="45">guid B</text><text x="550" y="45">free</text><text x="620" y="45">guid C</text>
  </g>
  <text x="480" y="74" fill="#6fb3ff" text-anchor="middle" font-size="10">free(B) → hole; next allocate first-fits into it</text>
  <text x="480" y="90" fill="#666" text-anchor="middle" font-size="10">neighbours never re-uploaded</text>
  <text x="340" y="128" fill="#888" text-anchor="middle">slots: HashMap&lt;guid, ArenaSlot{vertex_range, index_range}&gt; — the address book</text>
  <text x="340" y="148" fill="#666" text-anchor="middle">meshes: free-list arena · lane tables (pipes/spheres/segments/glyphs): guid→Range + drain-shift</text>
</svg>

## Files we touch

```
src/engine/gpu/arena.rs   # NEW — GpuArena: free-list allocator over vbo+vids+ibo, guid → slot
# Gpu owns a GpuArena + guid→range maps over the four lane mirrors; append/remove helpers;
# set_scene fills per object
src/engine/gpu/mod.rs
src/app/scene.rs          # flatten_mesh (35's push_mesh, split); add_file fills per-object entries
```

## Step 1 — the allocator type: `src/engine/gpu/arena.rs` (NEW)

`GpuArena` is a game-engine allocator (ported from the archive's `arena.rs`): a **bump cursor** for
fast first-fill, a **free list** of reclaimed ranges for reuse after deletes, and a `guid → slot` map
so any object can be found, freed, or overwritten later. It carries our per-vertex instance-id buffer
(`vids`, 30) alongside the vertex buffer, since the two are always parallel.

Declare the new module in `gpu/mod.rs` — add at the top, next to the existing `use` lines:

```rust
mod arena;
use arena::GpuArena;
```

```rust
//! Free-list allocator over a wgpu vertex buffer (+ its parallel vids buffer + an index buffer).
//! Bump-allocate until full, then first-fit from freed ranges; grow 2x (copy old → new) when neither
//! fits. `slots` maps each guid to its ranges so reconcile (43b) can free/replace one object in
//! place.

use std::collections::HashMap;
use std::ops::Range;
use session_rust::RenderVertex;

#[derive(Clone)]
pub struct ArenaSlot {
    pub vertex_range: Range<u32>,   // into vbo AND vids (parallel)
    pub index_range: Range<u32>,    // into ibo
}

pub struct GpuArena {
    pub vbo: wgpu::Buffer,
    pub vids: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    cap_v: u32,           // capacities, in elements
    cap_i: u32,
    cursor_v: u32,        // bump pointers (high-water mark)
    cursor_i: u32,
    free_v: Vec<Range<u32>>,   // reclaimed vertex ranges
    free_i: Vec<Range<u32>>,
    pub slots: HashMap<String, ArenaSlot>,
}

impl GpuArena {
    pub fn new(device: &wgpu::Device, cap_v: u32, cap_i: u32) -> Self {
        let vbo = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.vbo"),
            size: cap_v as u64 * std::mem::size_of::<RenderVertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST |
                wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        let vids = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.vids"),
            size: cap_v as u64 * 4,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST |
                wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        let ibo = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.ibo"),
            size: cap_i as u64 * 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST |
                wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        Self { vbo, vids, ibo, cap_v, cap_i, cursor_v: 0, cursor_i: 0,
                free_v: Vec::new(), free_i: Vec::new(), slots: HashMap::new() }
    }
}
```

(`vids` stores the same row number once per vertex — 4 B/vertex of pure duplication, ≈4 MB on a
1M-vertex scene. A per-instance indirection (indirect draws, 81) could carry it once per object;
per-vertex is what keeps the whole arena one flat draw call, so it stays.)

## Step 2 — allocate and free: `src/engine/gpu/arena.rs`

Add to `impl GpuArena`:

```rust
    /// Place one object's mesh data; records and returns its slot. `local_idx` are 0-based into
    /// this object's own vertices — rebased onto the arena vertex start here, so the ibo is
    /// arena-global. Re-allocating an EXISTING guid frees its old ranges first — the slot map
    /// holds one entry per guid, so without the self-free the stale ranges would be neither
    /// drawn nor reusable (a silent leak on every replace).
    pub fn allocate(&mut self, guid: &str, verts: &[RenderVertex], row: u32, local_idx: &[u32],
                    device: &wgpu::Device, queue: &wgpu::Queue) -> ArenaSlot {
        if self.slots.contains_key(guid) { let _ = self.free(guid, queue); }
        let v = self.alloc_v(verts.len() as u32, device, queue);
        let i = self.alloc_i(local_idx.len() as u32, device, queue);
        queue.write_buffer(&self.vbo,
            v.start as u64 * std::mem::size_of::<RenderVertex>() as u64,
            bytemuck::cast_slice(verts));
        // every vertex tags its instance row (30)
        let vids: Vec<u32> = vec![row; verts.len()];
        queue.write_buffer(&self.vids, v.start as u64 * 4, bytemuck::cast_slice(&vids));
        // 0-based → arena-global
        let global: Vec<u32> = local_idx.iter().map(|ix| ix + v.start).collect();
        queue.write_buffer(&self.ibo, i.start as u64 * 4, bytemuck::cast_slice(&global));
        let slot = ArenaSlot { vertex_range: v, index_range: i };
        self.slots.insert(guid.to_string(), slot.clone());
        slot
    }

    /// Reclaim an object's ranges for reuse. The buffers keep the stale bytes, but nothing draws
    /// them: the freed index range is overwritten with degenerate triangles so it renders zero
    /// area.
    pub fn free(&mut self, guid: &str, queue: &wgpu::Queue) -> Option<ArenaSlot> {
        let slot = self.slots.remove(guid)?;
        // Repeat a VALID vertex index (this object's own first vertex) — three identical indices
        // → zero area. NOT index_range.start: that's an ibo offset and could point past the vbo's
        // vertex count. A small FIXED staging array, chunked — not a `vec![…]` sized to the
        // object, which would re-allocate per free on the biggest meshes.
        let mut staging = [0u32; 256];
        staging.fill(slot.vertex_range.start);
        let mut offset = slot.index_range.start as u64 * 4;
        let mut left = slot.index_range.len();
        while left > 0 {
            let n = left.min(staging.len());
            queue.write_buffer(&self.ibo, offset, bytemuck::cast_slice(&staging[..n]));
            offset += n as u64 * 4;
            left -= n;
        }
        Self::push_free(&mut self.free_v, slot.vertex_range.clone());
        Self::push_free(&mut self.free_i, slot.index_range.clone());
        Some(slot)
    }

    /// Push a reclaimed range, merging adjacent entries. Without coalescing the free list
    /// fragments into object-sized slivers: a freed 100+100 never fits a 150, the bump cursor
    /// keeps growing, and the arena balloons on edit-heavy sessions.
    fn push_free(list: &mut Vec<Range<u32>>, r: Range<u32>) {
        list.push(r);
        list.sort_by_key(|x| x.start);
        let mut merged: Vec<Range<u32>> = Vec::with_capacity(list.len());
        for x in list.drain(..) {
            if let Some(last) = merged.last_mut() {
                if last.end == x.start { last.end = x.end; continue; }
            }
            merged.push(x);
        }
        *list = merged;
    }
```

(First-fit in `alloc_v`/`alloc_i` below walks the list unordered — order doesn't matter to it, so
keeping `free_v`/`free_i` sorted for the merge costs nothing extra.)

> **Free without shifting.** Deleting a mesh doesn't compact the buffer — it drops the ranges onto the
> free list and stamps the *index* range with a degenerate triangle (three identical indices → zero
> area, drawn but invisible). The next `allocate` fits into that hole. This is why the arena never has
> to re-upload untouched neighbours — the property reconcile (43b) needs, and the one a flat `Vec`
> can't give.

## Step 3 — the allocation strategy + growth: `src/engine/gpu/arena.rs`

```rust
    fn alloc_v(&mut self, n: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Range<u32> {
        // first free range that fits
        if let Some(k) = self.free_v.iter().position(|r| r.len() as u32 >= n) {
            let r = self.free_v.remove(k);
            if (r.len() as u32) > n { self.free_v.push((r.start + n)..r.end); }
            return r.start..(r.start + n);
        }
        if self.cursor_v + n > self.cap_v { self.grow_v(self.cursor_v + n, device, queue); }
        let s = self.cursor_v; self.cursor_v += n; s..(s + n)                     // bump
    }

    // The index-side twin — same strategy over the i-side fields:
    fn alloc_i(&mut self, n: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Range<u32> {
        if let Some(k) = self.free_i.iter().position(|r| r.len() as u32 >= n) {
            let r = self.free_i.remove(k);
            if (r.len() as u32) > n { self.free_i.push((r.start + n)..r.end); }
            return r.start..(r.start + n);
        }
        if self.cursor_i + n > self.cap_i { self.grow_i(self.cursor_i + n, device, queue); }
        let s = self.cursor_i; self.cursor_i += n; s..(s + n)
    }

    fn grow_v(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut cap = self.cap_v.max(1) * 2;
        while cap < needed { cap *= 2; }
        let grow = |old: &wgpu::Buffer, elem: u64, used: u32, usage, label| -> wgpu::Buffer {
            let nb = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label), size: cap as u64 * elem, usage,
                mapped_at_creation: false });
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(old, 0, &nb, 0, used as u64 * elem);
            queue.submit([enc.finish()]);
            nb
        };
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST |
            wgpu::BufferUsages::COPY_SRC;
        self.vbo  = grow(&self.vbo,  std::mem::size_of::<RenderVertex>() as u64,
            self.cursor_v, vu, "arena.vbo");
        self.vids = grow(&self.vids, 4, self.cursor_v, vu, "arena.vids");
        // the draw sets its vertex buffers from self.arena.vbo/.vids each frame
        // (they're not in a bind group), so it picks up the grown buffer automatically
        self.cap_v = cap;
    }

    fn grow_i(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut cap = self.cap_i.max(1) * 2;
        while cap < needed { cap *= 2; }
        let nb = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("arena.ibo"), size: cap as u64 * 4,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST |
                wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        let mut enc = device.create_command_encoder(&Default::default());
        enc.copy_buffer_to_buffer(&self.ibo, 0, &nb, 0, self.cursor_i as u64 * 4);
        queue.submit([enc.finish()]);
        self.ibo = nb;
        self.cap_i = cap;
    }

    /// Indices to draw = the high-water mark. Replaces 30's `arena_index_count` field: the draw now
    /// calls `self.arena.index_count()`. Freed ranges still fall inside this span but were stamped
    /// degenerate in `free`, so drawing them costs nothing.
    pub fn index_count(&self) -> u32 { self.cursor_i }
```

## Step 4 — the lane tables: range maps + drain-shift: `src/engine/gpu/mod.rs`

The linework lanes have no index buffer, so they skip the free-list machinery — a `guid → Range`
into a CPU-side mirror per lane, plus **drain-and-shift** on delete, is simpler and is what the
archive uses. There are FOUR lanes since 35 (SOLID `pipes`/`spheres` from mesh edges/vertices,
FLAT `segments`/`glyphs`), and each pair still shares ONE storage buffer, spliced SOLID-first
exactly as 35's `set_scene` wrote it. Add to `Gpu` — and delete the four `arena_*` fields the
first line replaces (5b rewires the initializer):

```rust
    arena: GpuArena,                  // ← replaces 30's arena_vbo/vids/ibo/arena_index_count
    guid_to_pipe:   std::collections::HashMap<String, std::ops::Range<usize>>,
    guid_to_sphere: std::collections::HashMap<String, std::ops::Range<usize>>,
    guid_to_seg:    std::collections::HashMap<String, std::ops::Range<usize>>,
    guid_to_glyph:  std::collections::HashMap<String, std::ops::Range<usize>>,
    pipes:    Vec<CylinderSegment>,   // CPU mirrors (35 only borrowed the Scene's vecs) — the
    spheres:  Vec<GlyphPoint>,        // drain-shift state each remove edits, then re-splices
    segments: Vec<CylinderSegment>,
    glyphs:   Vec<GlyphPoint>,
```

Append records the guid's range and pushes to the tail; delete drains the slice and shifts every
later range down. The mechanics are lane-generic, so they live in two free helpers — put them next
to `zeroed_buffer` at the bottom of the file:

```rust
/// Append `rows` under `guid` at the tail of a lane's CPU mirror; record its range.
fn lane_append<T: Copy>(vec: &mut Vec<T>,
    map: &mut std::collections::HashMap<String, std::ops::Range<usize>>,
    guid: &str, rows: &[T]) {
    let start = vec.len();
    vec.extend_from_slice(rows);
    map.insert(guid.to_string(), start..vec.len());
}

/// Drain `guid`'s slice out of a lane and shift every later range down. True if it was there.
fn lane_remove<T>(vec: &mut Vec<T>,
    map: &mut std::collections::HashMap<String, std::ops::Range<usize>>,
    guid: &str) -> bool {
    let Some(r) = map.remove(guid) else { return false };
    let n = r.len();
    vec.drain(r.clone());
    for range in map.values_mut() {
        if range.start >= r.end { range.start -= n; range.end -= n; }   // shift the tail
    }
    true
}
```

and the guid-facing entry points on `impl Gpu` — `solid` picks the lane, and every mutation ends in
one splice-upload of the (dense) buffer (`upload_segments`/`upload_glyphs` are 4b):

```rust
    /// One object's linework in/out by guid. 43b's reconcile calls these; set_scene bulk-fills
    /// the mirrors instead (one upload for the whole scene, not one per object).
    pub fn append_segments(&mut self, guid: &str, segs: &[CylinderSegment], solid: bool) {
        if solid { lane_append(&mut self.pipes, &mut self.guid_to_pipe, guid, segs); }
        else { lane_append(&mut self.segments, &mut self.guid_to_seg, guid, segs); }
        self.upload_segments();
    }
    pub fn remove_segments(&mut self, guid: &str) {
        let solid = lane_remove(&mut self.pipes, &mut self.guid_to_pipe, guid);
        let flat = lane_remove(&mut self.segments, &mut self.guid_to_seg, guid);
        if solid || flat { self.upload_segments(); }
    }

    pub fn append_glyphs(&mut self, guid: &str, gs: &[GlyphPoint], solid: bool) {
        if solid { lane_append(&mut self.spheres, &mut self.guid_to_sphere, guid, gs); }
        else { lane_append(&mut self.glyphs, &mut self.guid_to_glyph, guid, gs); }
        self.upload_glyphs();
    }
    pub fn remove_glyphs(&mut self, guid: &str) {
        let solid = lane_remove(&mut self.spheres, &mut self.guid_to_sphere, guid);
        let flat = lane_remove(&mut self.glyphs, &mut self.guid_to_glyph, guid);
        if solid || flat { self.upload_glyphs(); }
    }
```

> **One growth pattern, three buffers.** The arena's `grow_v`/`grow_i` — allocate a 2x buffer,
> refill, swap the handle — is the same move the segment, glyph, and instance buffers need when an
> added object overflows them. For the two lane buffers it lives inside 4b's
> `upload_segments`/`upload_glyphs`; the instance buffer gets its turn in 43b (`grow_instances`).
>
> **But swapping the handle is not enough for the storage buffers.** The arena's `vbo`/`vids`/`ibo`
> are re-bound every frame by `set_vertex_buffer`/`set_index_buffer`, so a grown handle is picked up
> automatically. The segment, glyph, and instance buffers are *storage* buffers wired into a bind
> group **once per creation** via `as_entire_binding()` — after you swap the handle, the bind group
> still points at the dropped buffer, and the frame reads stale (or freed) memory. So every buffer
> swap must be followed by **recreating that buffer's bind group** (`segment_bind_group` /
> `glyph_bind_group` / `instance_bind_group`) against the new handle before the next draw.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="arena vertex/index buffers are re-bound every frame so a grown handle is auto-picked-up; segment glyph and instance storage buffers are bound once in a bind group and go stale unless the bind group is recreated after the swap" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="340" y="18" fill="#888" text-anchor="middle">after grow_buffer swaps the handle…</text>
  <text x="165" y="42" fill="#5bbf87" text-anchor="middle">set every frame → auto</text>
  <g fill="none" stroke="#5bbf87" stroke-width="1.1">
    <rect x="40" y="54" width="250" height="24"/><rect x="40" y="82" width="250" height="24"/><rect x="40" y="110" width="250" height="24"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle" font-size="10">
    <text x="165" y="70">arena.vbo</text><text x="165" y="98">arena.vids</text><text x="165" y="126">arena.ibo</text>
  </g>
  <text x="165" y="152" fill="#5bbf87" text-anchor="middle" font-size="10">set_vertex_buffer / set_index_buffer</text>
  <text x="165" y="166" fill="#666" text-anchor="middle" font-size="10">re-issued each draw → grown handle picked up</text>
  <text x="515" y="42" fill="#e06c6c" text-anchor="middle">bound once (bind group) → STALE</text>
  <g fill="none" stroke="#e06c6c" stroke-width="1.1">
    <rect x="390" y="54" width="250" height="24"/><rect x="390" y="82" width="250" height="24"/><rect x="390" y="110" width="250" height="24"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle" font-size="10">
    <text x="515" y="70">segment_buffer</text><text x="515" y="98">glyph_buffer</text><text x="515" y="126">instance_buffer</text>
  </g>
  <text x="515" y="152" fill="#e06c6c" text-anchor="middle" font-size="10">as_entire_binding() once at buffer creation</text>
  <text x="515" y="166" fill="#666" text-anchor="middle" font-size="10">bind group still points at DROPPED buffer</text>
  <text x="515" y="186" fill="#6fb3ff" text-anchor="middle" font-size="10">fix → recreate the bind group after the swap</text>
</svg>

**4b. The splice-uploads (code for the notes above).** One prerequisite is already paid: 35 hoisted
the bind-group layouts onto `Gpu` (`segment_layout` / `glyph_layout` / `instance_layout` are fields
since its step 1c) — a grown buffer needs its bind group rebuilt, and that needs the layout. Because
the lanes keep **CPU mirrors** that are re-written in full right after, growth is
just *allocate bigger + rebuild the bind group* — **no** `copy_buffer_to_buffer` (that's only the arena,
which has no full mirror, which is why *its* buffers carry `COPY_SRC` and these don't need it):

```rust
    /// Splice-upload BOTH segment lanes: SOLID pipes first, FLAT segments after — the same order
    /// 35's set_scene wrote, so `pipe_count` keeps meaning "the solid prefix". Grows the storage
    /// buffer 2x + rebuilds its bind group when the mirrors outgrow it. The buffer never shrinks;
    /// rows past segment_count are stale bytes no draw range touches.
    fn upload_segments(&mut self) {
        self.pipe_count = self.pipes.len() as u32;
        self.segment_count = (self.pipes.len() + self.segments.len()) as u32;
        let elem = std::mem::size_of::<CylinderSegment>() as u64;
        let need = (self.segment_count as u64).max(1) * elem;
        if need > self.segment_buffer.size() {
            let mut cap = self.segment_buffer.size().max(elem);
            while cap < need { cap *= 2; }
            self.segment_buffer = zeroed_buffer(&self.device, "segments.buffer", cap,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            // bound ONCE via as_entire_binding → rebuild the bind group against the new handle
            self.segment_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("segments.bind_group"), layout: &self.segment_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0, resource: self.segment_buffer.as_entire_binding() }],
            });
        }
        self.queue.write_buffer(&self.segment_buffer, 0, bytemuck::cast_slice(&self.pipes));
        self.queue.write_buffer(&self.segment_buffer, self.pipes.len() as u64 * elem,
            bytemuck::cast_slice(&self.segments));
    }
    /// The glyph twin — same shape over the sphere/glyph fields (`sphere_count` = solid prefix).
    fn upload_glyphs(&mut self) {
        self.sphere_count = self.spheres.len() as u32;
        self.glyph_count = (self.spheres.len() + self.glyphs.len()) as u32;
        let elem = std::mem::size_of::<GlyphPoint>() as u64;
        let need = (self.glyph_count as u64).max(1) * elem;
        if need > self.glyph_buffer.size() {
            let mut cap = self.glyph_buffer.size().max(elem);
            while cap < need { cap *= 2; }
            self.glyph_buffer = zeroed_buffer(&self.device, "glyphs.buffer", cap,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.glyph_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("glyphs.bind_group"), layout: &self.glyph_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0, resource: self.glyph_buffer.as_entire_binding() }],
            });
        }
        self.queue.write_buffer(&self.glyph_buffer, 0, bytemuck::cast_slice(&self.spheres));
        self.queue.write_buffer(&self.glyph_buffer, self.spheres.len() as u64 * elem,
            bytemuck::cast_slice(&self.glyphs));
    }
    // The instance buffer needs the same treatment the moment rows can grow — that arrives with
    // 43b's set_object_row, which defines grow_instances on this exact pattern (instances /
    // Instance / instance_buffer / instance_bind_group / instance_layout).
```

> **Two strategies, one reason.** Meshes get the free-list (their vert+index blocks are big; tail
> re-uploads on every delete are the very cost we're avoiding); the lane tables get drain-shift
> (small, contiguous, no index — a tail rewrite is cheap and keeps each lane dense for its draw).

## Step 5 — fill through the arena: `src/engine/gpu/mod.rs` + `src/app/scene.rs`

**5a. Give the upload per-object boundaries.** Reshape 35's `ArenaUpload` so each object arrives
with its guid and its own slices instead of one concatenated blob (`objects`, `min`, `max` stay;
the four lane vecs keep their SOLID/FLAT split, per guid now; `verts`/`vids`/`idx` collapse into
`meshes` — `vids` disappears entirely: the arena writes them from the row number):

```rust
pub struct ArenaUpload {
    // guid, instance row, verts, 0-based LOCAL indices. The row rides along because only
    // MESH-bearing objects appear here — enumerate() would miscount in a mixed scene.
    pub meshes:   Vec<(String, u32, Vec<RenderVertex>, Vec<u32>)>,
    pub pipes:    Vec<(String, Vec<CylinderSegment>)>, // SOLID lane: mesh/BRep edges
    pub spheres:  Vec<(String, Vec<GlyphPoint>)>,      // SOLID lane: mesh/BRep vertices
    pub segments: Vec<(String, Vec<CylinderSegment>)>, // FLAT lane: line/polyline/curve/plane/box
    pub glyphs:   Vec<(String, Vec<GlyphPoint>)>,      // FLAT lane: points + clouds
    pub objects:  Vec<(Xform, [f32; 4], u32)>,   // true model, tint, flags (35) — one row per object
    pub min: [f32; 3],
    pub max: [f32; 3],
}
```

(`ArenaUpload::new()` follows: delete the `verts`/`vids`/`idx` lines, add `meshes: Vec::new(),`.)

`Scene::add_file` (35) changes *where it pushes*. First split 35's `push_mesh` into a function that
**returns** one object's tables (same body, four anchored edits, different sink — 43b's
`apply_object` reuses it), in `scene.rs`:

```rust
/// 35's `push_mesh`, split to RETURN one object's tables instead of pushing into shared vecs.
/// Indices are 0-based LOCAL — the arena rebases them onto its vertex start (Step 2).
fn flatten_mesh(m: &Mesh, ri: u32)
    -> (Vec<RenderVertex>, Vec<u32>, Vec<CylinderSegment>, Vec<GlyphPoint>) {
    let mut verts = Vec::new();
    let mut idx = Vec::new();
    let mut segments = Vec::new();
    let mut glyphs = Vec::new();
    // ── 35's push_mesh BODY goes here, copied in whole from `let rm = m.to_render();` down to
    // its final closing brace (hidden-width gate, width broadcast, vertex-sphere widths and
    // all) — with exactly these FOUR anchored edits:
    //
    //   top of body:        let base = verts.len() as u32;      — DELETE (verts starts empty here)
    //   the vertex loop:        vids.push(ri);                  — DELETE (the arena writes vids)
    //   the index loop:         idx.push(base + i);  →  idx.push(i);   — 0-based LOCAL
    //   the fill early-out:  if m.widths().len() == 1 && m.widths()[0] == 0.0 { return }
    //                        → { return (verts, idx, segments, glyphs) }   — push_mesh returned
    //                          (); we return the four locals, and a bare `return` won't compile
    (verts, idx, segments, glyphs)
}
```

then delete `push_mesh` and repoint the arms of `add_file`'s match. Every SOLID arm — `Mesh`,
`BRep`, `NurbsSurface`, and `Element`'s Mesh/BRep cases — swaps its `push_mesh(…)` call for the
per-object pushes; the Mesh arm shown, the others identical over `&bm` / `&sm`:

```rust
                Geometry::Mesh(m) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    let (v, i, e, d) = flatten_mesh(m, ri);
                    t.meshes.push((guid.clone(), ri, v, i));
                    if !e.is_empty() { t.pipes.push((guid.clone(), e)); }
                    if !d.is_empty() { t.spheres.push((guid.clone(), d)); }
                }
```

Every FLAT arm wraps its converter result in the guid entry — each `t.segments.extend(…)` /
`t.glyphs.extend(…)` becomes a `push` of the pair, and the single-row converters get a `vec![…]`:

```rust
                Geometry::Line(l) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.segments.push((guid.clone(), vec![line_to_segment(l, ri)]));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((placed, [1.0; 4], flags));
                    t.segments.push((guid.clone(), polyline_to_segments(pl, ri)));
                }
```

`NurbsCurve`/`Plane`/`OBB` follow the Polyline shape (`nurbscurve_to_segments` /
`plane_to_segments` / `obb_to_segments`); `Point` and `PointCloud` are the glyph twins
(`vec![point_to_glyph(p, ri)]` / `pointcloud_to_glyphs(pc, ri)`). A multi-row object — a polyline,
a cloud, a plane's 4-edge square, an OBB's 12 edges — is exactly why the entry holds a `Vec`: the
guid owns its whole span, however many rows that is.

Two `add_file` passes still walk the new rows and must nest one level deeper. The per-file bases
at the top now count ENTRIES, not rows — same lines, one rename: `let vert0 =
self.tables.verts.len();` becomes `let mesh0 = self.tables.meshes.len();`. The bounds pass:

```rust
        for (_, row, verts, _) in t.meshes.iter().skip(mesh0) {
            if let Some((xf, _, _)) = t.objects.get(*row as usize) {
                for v in verts { grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position)); }
            }
        }
        for (_, segs) in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)) {
            for s in segs {
                if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize) {
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
                }
            }
        }
        for (_, gs) in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)) {
            for g in gs {
                if let Some((xf, _, _)) = t.objects.get(g.instance_id as usize) {
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, g.center));
                }
            }
        }
```

and the 34f planar-width pass at the bottom keeps its two-lane body inside the same nesting:

```rust
            for (_, segs) in t.pipes.iter_mut().skip(pipe0).chain(t.segments.iter_mut().skip(seg0)) {
                for s in segs {
                    s.radius = if s.radius < 0.0 { -s.radius * 0.5 } else { 0.5 }
                }
            }
```

(40's per-row box append and `rebuild_bvh` read `t.objects` and the geometry, not the lane vecs —
they don't change.)

**5b. Fill per object in `set_scene`.** Meshes go through the free-list allocator; the lanes
bulk-fill their mirrors + range maps with `lane_append` and splice-upload ONCE per buffer (calling
`append_*` per object instead would re-upload the whole buffer N times). Two starting capacities,
next to the other consts at the top of `gpu/mod.rs`:

```rust
/// Arena starting capacities (elements). Undersized is fine — grow_v/grow_i double as needed.
const INITIAL_CAP_V: u32 = 1 << 16;
const INITIAL_CAP_I: u32 = 1 << 17;
```

In `Gpu::new`, the empty scene's three arena placeholders — the `let arena_vbo = zeroed_buffer(…)`
/ `arena_vids` / `arena_ibo` lines — become one real (empty) arena:

```rust
        let arena = GpuArena::new(&device, INITIAL_CAP_V, INITIAL_CAP_I);
```

Then in `set_scene`, where 35's flat `create_buffer_init` arena uploads stood (the whole
`if up.verts.is_empty() { … } else { … }` block under `// Mesh arena`), fill per object:

```rust
        // Mesh arena — reset, then place every object individually so each guid gets a slot.
        // A full refill per set_scene keeps 35's one-upload-path shape; 43b's reconcile is what
        // starts freeing/replacing single slots instead of calling set_scene at all.
        self.arena = GpuArena::new(&self.device, INITIAL_CAP_V, INITIAL_CAP_I);
        for (guid, row, verts, local_idx) in &up.meshes {
            self.arena.allocate(guid, verts, *row, local_idx, &self.device, &self.queue);
        }
```

and replace the two lane-splice blocks below it (from `// The two lane tables` down to the glyph
bind-group's closing `});`) with mirror fills + the 4b uploads — the splice, the counts, the
growth, and the bind-group rebuild all live in `upload_*` now:

```rust
        // The two lane tables: fill the CPU mirrors + range maps, splice-upload once per buffer.
        self.pipes.clear(); self.guid_to_pipe.clear();
        self.segments.clear(); self.guid_to_seg.clear();
        for (guid, segs) in &up.pipes { lane_append(&mut self.pipes, &mut self.guid_to_pipe, guid, segs); }
        for (guid, segs) in &up.segments { lane_append(&mut self.segments, &mut self.guid_to_seg, guid, segs); }
        self.upload_segments();

        self.spheres.clear(); self.guid_to_sphere.clear();
        self.glyphs.clear(); self.guid_to_glyph.clear();
        for (guid, gs) in &up.spheres { lane_append(&mut self.spheres, &mut self.guid_to_sphere, guid, gs); }
        for (guid, gs) in &up.glyphs { lane_append(&mut self.glyphs, &mut self.guid_to_glyph, guid, gs); }
        self.upload_glyphs();
```

The reshaped upload ripples through the rest — four mechanical fixes:

- **`msaa_for`**: its `solid` line reads the dead `up.verts` — make it
  `let solid = !up.meshes.is_empty() || !up.pipes.is_empty() || !up.spheres.is_empty();`.
- **`set_scene`'s first line** `use wgpu::util::DeviceExt;`: delete — nothing in it calls
  `create_buffer_init` anymore. (And amend the ZERO-COPY line of its doc comment: the lanes now
  stage through `Gpu`'s mirrors — one CPU copy is the price of addressability.)
- **Wire `struct Gpu` + the `Ok(Self { … })` initializer** — `arena_vbo`, `arena_vids`,
  `arena_ibo`, `arena_index_count` leave BOTH (also drop `let arena_index_count = 0u32;` from
  `new`'s placeholder block); in come `arena,` the four maps
  (`guid_to_pipe: Default::default(),` …) and the four empty mirrors (`pipes: Vec::new(),` …).
  Miss one → *"missing field … in initializer of `Gpu`"* (E0063).
- **Scene bounds are untouched**: `up.min`/`up.max` still arrive from `add_file`'s (rewritten)
  bounds pass — `set_scene`'s `scene_min`/`scene_max` lines stay as they are.

**5c. Point the mesh draw at the arena.** In `clear()`, find the Arena draw block:

```rust
            pass.set_vertex_buffer(0, self.arena_vbo.slice(..)); // slot 0 - vertices
            pass.set_vertex_buffer(1, self.arena_vids.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, one call
```

and replace it with the arena-owned sources:

```rust
            pass.set_vertex_buffer(0, self.arena.vbo.slice(..));  // slot 0 - vertices
            pass.set_vertex_buffer(1, self.arena.vids.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.arena.ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.arena.index_count(), 0, 0..1); // whole scene, one call
```

The instance range is unchanged from 30 (per-vertex `vids` selects each model row, so it stays a
single-instance draw). One new guard: 35's empty-scene placeholder (`arena_index_count = 3` over a
zeroed ibo) is gone — an empty arena reports 0 —
so wrap the whole triangle block (from `pass.set_pipeline(&self.pipelines.triangle);` through its
`draws += 1;`) in `if self.arena.index_count() > 0 { … }` — a pure-linework file would otherwise
earn the zero-count Dawn warning 34b fought. Segment and glyph draws are untouched — same
buffers, same counts.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Pixels identical to 35 — floor model and stress file both draw exactly as before. The new capability
is invisible until something uses it, so prove it with a log — after the arena fill loop in
`set_scene` (give `GpuArena` a tiny getter or make `cursor_v` `pub` for this):

```rust
        log::info!("arena: {} slots, {} verts high-water",
            self.arena.slots.len(), self.arena.cursor_v);
```

201 mesh slots for the floor model, high-water mark equal to the old flat build. 43b is where
freeing and replacing start actually happening.

## Recap

```
Ch 41: frustum cull — per-object FLAG_CULLED, one draw preserved.
Ch 43a: PER-OBJECT ARENA. GpuArena (engine/gpu/arena.rs) replaces 30's flat arena: guid →
        ArenaSlot{vertex_range, index_range}; bump-fill, first-fit reuse of freed ranges (coalesced
        on free, so the list doesn't fragment into slivers), grow 2x
        (copy old → new, swap handle). allocate self-frees an existing guid first — a replace
        can't leak the old ranges. free() stamps the index range with degenerate triangles
        (repeat the object's OWN first vertex index — an ibo offset would read past the vbo;
        chunked through a small fixed staging array, not a per-object Vec)
        so holes
        draw nothing and neighbours are never re-uploaded. Lane tables (pipes/spheres/segments/
        glyphs): guid→Range + drain-shift over CPU mirrors, splice-uploaded SOLID-first as in 35
        (small dense tables; archive's split). set_scene fills per object instead of wholesale.
        Zero visual change — addressability is the product.
```

Edited: `engine/gpu/arena.rs` (NEW — `GpuArena`: `allocate`/`free`/`alloc_v`/`grow_v`, `guid→ArenaSlot`),
`engine/gpu/mod.rs` (arena + four lane mirrors/range maps + `append_*`/`remove_*` + `upload_*`
splices; `set_scene` fills per object), `app/scene.rs` (`flatten_mesh` replaces `push_mesh`;
per-object entries in `add_file`).

## Next

`43b-reconcile.md` — the arena can now touch one object, so the diff arrives: fingerprint every object
(a deterministic content hash — with a real `HashMap`-ordering trap to dodge), diff the incoming
`Session` against the loaded one by guid, and apply only `added` / `removed` / `changed`. Edit 1 of
491 objects → 1 re-flatten, 490 skips.
