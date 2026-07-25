# 38a Reconcile I — a per-object GPU arena

> **Big picture.** *Phase 6 — the `.pb` file becomes the live source of truth, like a real CAD app.*
> The plan: reload only what changed (38b), save only when something changed (39), watch for external
> edits (40). None of that is possible while the GPU side is all-or-nothing — lesson 30's arena is one
> flat `Vec`, rebuilt wholesale; there is no way to touch *one object's* bytes. So before any diffing,
> this lesson upgrades the arena: every object gets an addressable slice of the GPU buffers that can be
> freed and replaced **without disturbing its neighbours**.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the static flat arena is rebuilt wholesale; the free-list arena maps each guid to a range that can be freed and reused individually" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="18" fill="#888" text-anchor="middle">lesson 30 — static arena</text>
  <rect x="20" y="28" width="260" height="26" fill="none" stroke="#3a3a3a"/>
  <text x="150" y="45" fill="#666" text-anchor="middle">all objects, concatenated once</text>
  <text x="150" y="74" fill="#888" text-anchor="middle" font-size="10">any change → rebuild + re-upload EVERYTHING</text>
  <text x="480" y="18" fill="#888" text-anchor="middle">38a — free-list arena</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2">
    <rect x="360" y="28" width="70" height="26"/><rect x="430" y="28" width="90" height="26"/><rect x="520" y="28" width="60" height="26"/><rect x="580" y="28" width="80" height="26"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle" font-size="10">
    <text x="395" y="45">guid A</text><text x="475" y="45">guid B</text><text x="550" y="45">free</text><text x="620" y="45">guid C</text>
  </g>
  <text x="480" y="74" fill="#6fb3ff" text-anchor="middle" font-size="10">free(B) → hole; next allocate first-fits into it</text>
  <text x="480" y="90" fill="#666" text-anchor="middle" font-size="10">neighbours never re-uploaded</text>
  <text x="340" y="128" fill="#888" text-anchor="middle">slots: HashMap&lt;guid, ArenaSlot{vertex_range, index_range}&gt; — the address book</text>
  <text x="340" y="148" fill="#666" text-anchor="middle">meshes: free-list arena · lines/points: guid→Range + drain-shift (small, dense tables)</text>
</svg>

## Files we touch

```
src/engine/gpu/arena.rs   # NEW — GpuArena: free-list allocator over vbo+vids+ibo, guid → slot
# Gpu owns a GpuArena + guid→segment/glyph range maps; append/remove helpers
src/engine/gpu/mod.rs
```

## Step 1 — the allocator type: `src/engine/gpu/arena.rs` (NEW)

`GpuArena` is a game-engine allocator (ported from the archive's `arena.rs`): a **bump cursor** for
fast first-fill, a **free list** of reclaimed ranges for reuse after deletes, and a `guid → slot` map
so any object can be found, freed, or overwritten later. It carries our per-vertex instance-id buffer
(`vids`, 30) alongside the vertex buffer, since the two are always parallel.

```rust
//! Free-list allocator over a wgpu vertex buffer (+ its parallel vids buffer + an index buffer).
//! Bump-allocate until full, then first-fit from freed ranges; grow 2x (copy old → new) when neither
//! fits. `slots` maps each guid to its ranges so reconcile (38b) can free/replace one object in
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

## Step 2 — allocate and free: `src/engine/gpu/arena.rs`

Add to `impl GpuArena`:

```rust
    /// Place one object's mesh data; records and returns its slot. `local_idx` are 0-based into
    /// this object's own vertices — rebased onto the arena vertex start here, so the ibo is
    /// arena-global.
    pub fn allocate(&mut self, guid: &str, verts: &[RenderVertex], row: u32, local_idx: &[u32],
                    device: &wgpu::Device, queue: &wgpu::Queue) -> ArenaSlot {
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
    /// them: the freed index range is overwritten with a degenerate triangle so it renders zero
    /// area.
    pub fn free(&mut self, guid: &str, queue: &wgpu::Queue) -> Option<ArenaSlot> {
        let slot = self.slots.remove(guid)?;
        // Repeat a VALID vertex index (this object's own first vertex) — three identical indices
        // → zero area. NOT index_range.start: that's an ibo offset and could point past the vbo's
        // vertex count.
        let dead = vec![slot.vertex_range.start; slot.index_range.len()];
        queue.write_buffer(&self.ibo, slot.index_range.start as u64 * 4,
            bytemuck::cast_slice(&dead));
        self.free_v.push(slot.vertex_range.clone());
        self.free_i.push(slot.index_range.clone());
        Some(slot)
    }
```

> **Free without shifting.** Deleting a mesh doesn't compact the buffer — it drops the ranges onto the
> free list and stamps the *index* range with a degenerate triangle (three identical indices → zero
> area, drawn but invisible). The next `allocate` fits into that hole. This is why the arena never has
> to re-upload untouched neighbours — the property reconcile (38b) needs, and the one a flat `Vec`
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
    // alloc_i is identical over cursor_i / cap_i / free_i / grow_i — copy alloc_v.

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
    // grow_i mirrors grow_v for the single ibo (INDEX|COPY_DST|COPY_SRC).

    /// Indices to draw = the high-water mark. Replaces 30's `arena_index_count` field: the draw now
    /// calls `self.arena.index_count()`. Freed ranges still fall inside this span but were stamped
    /// degenerate in `free`, so drawing them costs nothing.
    pub fn index_count(&self) -> u32 { self.cursor_i }
```

## Step 4 — segments & glyphs: range maps + drain-shift: `src/engine/gpu/mod.rs`

Lines and points have no index buffer, so they skip the free-list machinery — a `guid → Range` into
the CPU-side vec, plus **drain-and-shift** on delete, is simpler and is what the archive uses. Add to
`Gpu`:

```rust
    arena: GpuArena,                  // ← replaces 30's arena_vbo/vids/ibo
    guid_to_seg: std::collections::HashMap<String, std::ops::Range<usize>>,
    guid_to_glyph: std::collections::HashMap<String, std::ops::Range<usize>>,
    segments: Vec<CylinderSegment>,   // keep the CPU mirror (was upload-only) so we can splice it
    glyphs:   Vec<GlyphPoint>,
```

Append records the guid's range and pushes to the tail; delete drains the slice and shifts every later
range down. Both re-upload the (dense) segment buffer once:

```rust
    fn append_segments(&mut self, guid: &str, segs: &[CylinderSegment]) {
        let start = self.segments.len();
        self.segments.extend_from_slice(segs);
        self.guid_to_seg.insert(guid.to_string(), start..self.segments.len());
        self.ensure_seg_capacity();   // grow the wgpu buffer 2x if the Vec outgrew it (see note)
        self.queue.write_buffer(&self.segment_buffer, 0, bytemuck::cast_slice(&self.segments));
        self.segment_count = self.segments.len() as u32;
    }
    fn remove_segments(&mut self, guid: &str) {
        if let Some(r) = self.guid_to_seg.remove(guid) {
            let n = r.len();
            self.segments.drain(r.clone());
            for range in self.guid_to_seg.values_mut() {
                if range.start >= r.end { range.start -= n; range.end -= n; }   // shift the tail
            }
            self.queue.write_buffer(&self.segment_buffer, 0, bytemuck::cast_slice(&self.segments));
            self.segment_count = self.segments.len() as u32;
        }
    }
    // append_glyphs / remove_glyphs are identical over guid_to_glyph / glyphs / glyph_buffer /
    // glyph_count.
```

> **One growth pattern, four buffers.** The arena's `grow_v`/`grow_i` — allocate a 2x buffer,
> `copy_buffer_to_buffer` the live prefix, swap the handle — is the same move the segment, glyph, and
> instance buffers need when an added object overflows them. Factor one
> `fn grow_buffer(old, used, elem, usage) -> Buffer` and call it from `ensure_seg_capacity` /
> `ensure_glyph_capacity` / `grow_instances`.
>
> **But swapping the handle is not enough for the storage buffers.** The arena's `vbo`/`vids`/`ibo`
> are re-bound every frame by `set_vertex_buffer`/`set_index_buffer`, so a grown handle is picked up
> automatically. The segment, glyph, and instance buffers are *storage* buffers wired into a bind
> group **once** in `Gpu::new` via `as_entire_binding()` — after you swap the handle, the bind group
> still points at the dropped buffer, and the frame reads stale (or freed) memory. So `grow_buffer`
> must be followed by **recreating that buffer's bind group** (`segment_bind_group` /
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
  <text x="515" y="152" fill="#e06c6c" text-anchor="middle" font-size="10">as_entire_binding() once in Gpu::new</text>
  <text x="515" y="166" fill="#666" text-anchor="middle" font-size="10">bind group still points at DROPPED buffer</text>
  <text x="515" y="186" fill="#6fb3ff" text-anchor="middle" font-size="10">fix → recreate the bind group after the swap</text>
</svg>

**4b. The growth methods (code for the two notes above).** One prerequisite: hoist the three storage
buffers' **bind-group layouts** onto `Gpu` — `segment_layout`, `glyph_layout`, `instance_layout` are
locals in `new` today; a grown buffer needs its bind group rebuilt, and that needs the layout. Because
segments/glyphs/instances each keep a **CPU mirror** that is re-uploaded in full right after, growth is
just *allocate bigger + rebuild the bind group* — **no** `copy_buffer_to_buffer` (that's only the arena,
which has no full mirror, which is why *its* buffers carry `COPY_SRC` and these don't need it):

```rust
    /// Grow the segment storage buffer if the CPU mirror outgrew it. Called by append_segments
    /// BEFORE its write_buffer, so the freshly-grown buffer is filled from `self.segments` that
    /// same line — no copy needed.
    fn ensure_seg_capacity(&mut self) {
        let need = (self.segments.len() * std::mem::size_of::<CylinderSegment>()) as u64;
        if need <= self.segment_buffer.size() { return; }
        let mut cap = self.segment_buffer.size().max(std::mem::size_of::<CylinderSegment>() as u64);
        while cap < need { cap *= 2; }
        self.segment_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("segment.buffer"), size: cap, mapped_at_creation: false,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        // bound ONCE via as_entire_binding → rebuild the bind group against the new handle
        self.segment_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("segment.bind_group"), layout: &self.segment_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, resource: self.segment_buffer.as_entire_binding() }],
        });
    }
    // ensure_glyph_capacity is identical over glyphs / GlyphPoint / glyph_buffer / glyph_bind_group /
    // glyph_layout. grow_instances the same over instances / Instance / instance_buffer /
    // instance_bind_group / instance_layout (call it from rebuild_instances, 33, before its write).
```

> **Two strategies, one reason.** Meshes get the free-list (their vert+index blocks are big; tail
> re-uploads on every delete are the very cost we're avoiding); segments/glyphs get drain-shift
> (small, contiguous, no index — a tail rewrite is cheap and keeps them dense for the one draw).

## Step 5 — fill through the arena: `src/engine/gpu/mod.rs`

**5a. Give the upload per-object boundaries.** Extend 35's `ArenaUpload` so each object arrives with its
guid and its own slices instead of one concatenated blob:

```rust
pub struct ArenaUpload {
    pub meshes:   Vec<(String, Vec<RenderVertex>, Vec<u32>)>,  // guid, verts, 0-based LOCAL indices
    pub segments: Vec<(String, Vec<CylinderSegment>)>,
    pub glyphs:   Vec<(String, Vec<GlyphPoint>)>,
    // instances / objects_base stay one row per object, in guid order (33)
}
```

`Scene::build` (35) already loops per object — the only change is *where it pushes*: into these
per-object entries instead of three shared vecs.

**5b. Fill per object in `Gpu::new`.** Meshes go through the free-list allocator; segments/glyphs build
their dense vec + range map inline (the `append_*` helpers are `&mut self`, unusable while `Gpu` is
still under construction — they take over for incremental adds in 38b):

```rust
        let mut arena = GpuArena::new(&device, INITIAL_CAP_V, INITIAL_CAP_I);
        for (row, (guid, verts, local_idx)) in upload.meshes.iter().enumerate() {
            arena.allocate(guid, verts, row as u32, local_idx, &device, &queue);
        }

        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut guid_to_seg: std::collections::HashMap<String, std::ops::Range<usize>> =
            std::collections::HashMap::new();
        for (guid, segs) in &upload.segments {
            let start = segments.len();
            segments.extend_from_slice(segs);
            guid_to_seg.insert(guid.clone(), start..segments.len());
        }
        // glyphs / guid_to_glyph: identical over upload.glyphs
```

Then wire all of these into the `Ok(Self { … })` initializer — **and delete** 30's `arena_vbo`,
`arena_vids`, `arena_ibo`, `arena_index_count` (from *both* `struct Gpu` and `Self`), replacing them with
the Step-4 fields `arena`, `guid_to_seg`, `guid_to_glyph`, `segments`, `glyphs` (plus the hoisted
`segment_layout` / `glyph_layout` / `instance_layout` from 4b). Miss one → *"missing field … in
initializer of `Gpu`"* (E0063).

**5c. Point the mesh draw at the arena.** Same *shape* as 30's mesh draw — one `draw_indexed` over the
whole scene — but its three sources move onto the arena (`self.arena.vbo` / `.vids` / `.ibo`) and the
count becomes `self.arena.index_count()` (replacing 30's `arena_vbo`/`arena_vids`/`arena_ibo` and the
`arena_index_count` field). The instance range is unchanged from 30 (per-vertex `vids` selects each
model row, so it stays a single-instance draw). Segment and glyph draws are untouched — same buffers,
same counts.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Pixels identical to 35 — floor model and stress file both draw exactly as before. The new capability
is invisible until something uses it, so prove it with a log: after `Gpu::new` fills the arena,
`log::info!("arena: {} slots, {}/{} verts", …)` — 201 mesh slots for the floor model, high-water mark
equal to the old flat build. 38b is where freeing and replacing start actually happening.

## Recap

```
Ch 37: frustum cull — per-object FLAG_CULLED, one draw preserved.
Ch 38a: PER-OBJECT ARENA. GpuArena (engine/gpu/arena.rs) replaces 30's flat arena: guid →
        ArenaSlot{vertex_range, index_range}; bump-fill, first-fit reuse of freed ranges, grow 2x
        (copy old → new, swap handle). free() stamps the index range with a degenerate triangle
        (repeat the object's OWN first vertex index — an ibo offset would read past the vbo)
        so holes
        draw nothing and neighbours are never re-uploaded. Segments/glyphs: guid→Range + drain-shift
        (small dense tables; archive's split). Gpu::new fills per object instead of wholesale.
        Zero visual change — addressability is the product.
```

Edited: `engine/gpu/arena.rs` (NEW — `GpuArena`: `allocate`/`free`/`alloc_v`/`grow_v`, `guid→ArenaSlot`),
`engine/gpu/mod.rs` (arena + `guid_to_seg`/`guid_to_glyph` + append/remove helpers; per-object fill).

## Next

`38b-reconcile.md` — the arena can now touch one object, so the diff arrives: fingerprint every object
(a deterministic content hash — with a real `HashMap`-ordering trap to dodge), diff the incoming
`Session` against the loaded one by guid, and apply only `added` / `removed` / `changed`. Edit 1 of
491 objects → 1 re-flatten, 490 skips.
