# 38 Document ↔ scene reconcile — never rebuild the whole scene

Reloading a file today throws everything away: `Scene::build()` (35) re-flattens all 491 objects, and
`Gpu::new` rebuilds every buffer from scratch. Edit one wall in a 42,232-object drawing and the viewer
re-uploads 42,231 unchanged objects to move one. This lesson makes reload **incremental**: diff the
incoming `Session` against the current one by `guid`, and touch only what actually changed — one
object re-flattened, the rest skipped.

That needs something the static arena (30) never had: the ability to free and replace **one object's**
slice of the GPU buffers without disturbing its neighbours. So this lesson is two joined halves — first
a real per-object allocator, then the diff that drives it.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a new Session is diffed against the current one by guid into added, removed, changed and unchanged; only the first three touch the GPU arena via allocate, free and replace" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="12" y="26" width="150" height="30" fill="none" stroke="#6fb3ff"/><text x="87" y="45" fill="#d7dae0" text-anchor="middle">new Session</text>
  <rect x="12" y="120" width="150" height="30" fill="none" stroke="#3a3a3a"/><text x="87" y="139" fill="#888" text-anchor="middle">current hashes{guid→h}</text>
  <text x="200" y="92" fill="#6fb3ff" text-anchor="middle" font-size="13">diff by guid</text>
  <line x1="162" y1="41" x2="250" y2="82" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah38)"/>
  <line x1="162" y1="135" x2="250" y2="98" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah38)"/>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2"><rect x="300" y="20" width="120" height="24"/><rect x="300" y="52" width="120" height="24"/><rect x="300" y="84" width="120" height="24"/></g>
  <rect x="300" y="116" width="120" height="24" fill="none" stroke="#3a3a3a"/>
  <g fill="#d7dae0"><text x="310" y="37">added</text><text x="310" y="69">changed (h≠)</text><text x="310" y="101">removed</text></g>
  <text x="310" y="133" fill="#666">unchanged → skip</text>
  <g stroke="#6fb3ff" stroke-width="1.2">
    <line x1="420" y1="32" x2="500" y2="32" marker-end="url(#ah38)"/>
    <line x1="420" y1="64" x2="500" y2="64" marker-end="url(#ah38)"/>
    <line x1="420" y1="96" x2="500" y2="96" marker-end="url(#ah38)"/>
  </g>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2"><rect x="502" y="20" width="166" height="24"/><rect x="502" y="52" width="166" height="24"/><rect x="502" y="84" width="166" height="24"/></g>
  <g fill="#d7dae0"><text x="512" y="37">arena.allocate(guid)</text><text x="512" y="69">free + allocate (replace)</text><text x="512" y="101">arena.free(guid)</text></g>
  <text x="585" y="133" fill="#666" text-anchor="middle">only these hit the GPU</text>
  <text x="340" y="175" fill="#888" text-anchor="middle">reload edits 1 of N objects → 1 allocate, N−1 skips — not N re-uploads</text>
  <defs><marker id="ah38" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gpu/arena.rs   # NEW — GpuArena: free-list allocator over vbo+vids+ibo, guid → slot
src/engine/gpu/mod.rs     # Gpu owns a GpuArena + guid→segment/glyph ranges; add/remove/replace_object
src/app/scene.rs          # Scene keeps guid → content-hash; reconcile(new) → Diff{added,removed,changed}
src/state.rs              # a reload path that calls reconcile instead of rebuilding from zero
```

## Part A — a per-object allocator: `src/engine/gpu/arena.rs` (NEW)

The static arena (30) is one flat `Vec` uploaded once; there's no notion of "object X lives at
vertices 400..612". `GpuArena` adds exactly that: a **bump cursor** for fast first-fill, a **free list**
of reclaimed ranges for best-fit reuse after deletes, and a `guid → slot` map so any object can be
found, freed, or overwritten later. It's the game-engine allocator the archive ships in `arena.rs`,
trimmed to what reconcile needs — and extended to carry our per-vertex instance-id buffer (`vids`, 30)
alongside the vertex buffer, since the two are always parallel.

```rust
//! Free-list allocator over a wgpu vertex buffer (+ its parallel vids buffer + an index buffer).
//! Bump-allocate until full, then best-fit from freed ranges; grow 2x (copy old → new) when neither
//! fits. `slots` maps each guid to its ranges so reconcile (38) can free/replace one object in place.

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
    free_v: Vec<Range<u32>>,   // reclaimed vertex ranges, best-fit
    free_i: Vec<Range<u32>>,
    pub slots: HashMap<String, ArenaSlot>,
}

impl GpuArena {
    pub fn new(device: &wgpu::Device, cap_v: u32, cap_i: u32) -> Self {
        let vbo = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.vbo"),
            size: cap_v as u64 * std::mem::size_of::<RenderVertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        let vids = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.vids"),
            size: cap_v as u64 * 4, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        let ibo = device.create_buffer(&wgpu::BufferDescriptor { label: Some("arena.ibo"),
            size: cap_i as u64 * 4, usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false });
        Self { vbo, vids, ibo, cap_v, cap_i, cursor_v: 0, cursor_i: 0,
                free_v: Vec::new(), free_i: Vec::new(), slots: HashMap::new() }
    }

    /// Place one object's mesh data; records and returns its slot. `local_idx` are 0-based into this
    /// object's own vertices — rebased onto the arena vertex start here, so the ibo is arena-global.
    pub fn allocate(&mut self, guid: &str, verts: &[RenderVertex], row: u32, local_idx: &[u32],
                    device: &wgpu::Device, queue: &wgpu::Queue) -> ArenaSlot {
        let v = self.alloc_v(verts.len() as u32, device, queue);
        let i = self.alloc_i(local_idx.len() as u32, device, queue);
        queue.write_buffer(&self.vbo, v.start as u64 * std::mem::size_of::<RenderVertex>() as u64, bytemuck::cast_slice(verts));
        let vids: Vec<u32> = vec![row; verts.len()];                      // every vertex tags its instance row (30)
        queue.write_buffer(&self.vids, v.start as u64 * 4, bytemuck::cast_slice(&vids));
        let global: Vec<u32> = local_idx.iter().map(|ix| ix + v.start).collect();   // 0-based → arena-global
        queue.write_buffer(&self.ibo, i.start as u64 * 4, bytemuck::cast_slice(&global));
        let slot = ArenaSlot { vertex_range: v, index_range: i };
        self.slots.insert(guid.to_string(), slot.clone());
        slot
    }

    /// Reclaim an object's ranges for reuse. The buffers keep the stale bytes, but nothing draws them:
    /// the freed index range is overwritten with a degenerate triangle so it renders zero area.
    pub fn free(&mut self, guid: &str, queue: &wgpu::Queue) -> Option<ArenaSlot> {
        let slot = self.slots.remove(guid)?;
        // Repeat a VALID vertex index (this object's own first vertex) — three identical indices → zero
        // area. NOT index_range.start: that's an ibo offset and could point past the vbo's vertex count.
        let dead = vec![slot.vertex_range.start; slot.index_range.len()];
        queue.write_buffer(&self.ibo, slot.index_range.start as u64 * 4, bytemuck::cast_slice(&dead));
        self.free_v.push(slot.vertex_range.clone());
        self.free_i.push(slot.index_range.clone());
        Some(slot)
    }

    fn alloc_v(&mut self, n: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Range<u32> {
        if let Some(k) = self.free_v.iter().position(|r| r.len() as u32 >= n) {   // first free range that fits
            let r = self.free_v.remove(k);
            if (r.len() as u32) > n { self.free_v.push((r.start + n)..r.end); }
            return r.start..(r.start + n);
        }
        if self.cursor_v + n > self.cap_v { self.grow_v(self.cursor_v + n, device, queue); }
        let s = self.cursor_v; self.cursor_v += n; s..(s + n)                     // bump
    }
    // alloc_i is identical over cursor_i / cap_i / free_i / grow_i — omitted for length; copy alloc_v.

    fn grow_v(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut cap = self.cap_v.max(1) * 2;
        while cap < needed { cap *= 2; }
        let grow = |old: &wgpu::Buffer, elem: u64, used: u32, usage, label| -> wgpu::Buffer {
            let nb = device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: cap as u64 * elem, usage, mapped_at_creation: false });
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(old, 0, &nb, 0, used as u64 * elem);
            queue.submit([enc.finish()]);
            nb
        };
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        self.vbo  = grow(&self.vbo,  std::mem::size_of::<RenderVertex>() as u64, self.cursor_v, vu, "arena.vbo");
        self.vids = grow(&self.vids, 4, self.cursor_v, vu, "arena.vids");
        self.cap_v = cap;   // the draw sets its vertex buffers from self.arena.vbo/.vids each frame
    }                       // (they're not in a bind group), so it picks up the grown buffer automatically
    // grow_i mirrors grow_v for the single ibo (INDEX|COPY_DST|COPY_SRC).
}
```

> **Free without shifting.** Deleting a mesh doesn't compact the buffer — it drops the ranges onto the
> free list and stamps the *index* range with a degenerate triangle (three identical indices → zero
> area, drawn but invisible). Next `allocate` best-fits into that hole. This is why the arena never has
> to re-upload untouched neighbours — the exact property reconcile needs, and the one the flat `Vec`
> couldn't give.

## Part B — segments & glyphs: range maps + drain-shift: `src/engine/gpu/mod.rs`

Lines and points have no index buffer, so they don't need the free-list machinery — a `guid → Range`
into the CPU-side `segments`/`glyphs` vec, plus **drain-and-shift** on delete, is simpler and is what
the archive uses (`edit.rs`). Add to `Gpu`:

```rust
    arena: GpuArena,                                                   // ← replaces 30's arena_vbo/vids/ibo
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
    // append_glyphs / remove_glyphs are identical over guid_to_glyph / glyphs / glyph_buffer / glyph_count.
```

> **One growth pattern, four buffers.** The arena's `grow_v`/`grow_i` (Part A) — allocate a 2x buffer,
> `copy_buffer_to_buffer` the live prefix, swap the handle — is the *same* move the segment, glyph, and
> instance buffers need when an `added` object overflows them. Rather than four near-identical `grow_*`
> methods, factor one `fn grow_buffer(old, used, elem, usage) -> Buffer` and call it from
> `ensure_seg_capacity` / `ensure_glyph_capacity` / `grow_instances`. Each just re-uploads (or the
> caller does) after the swap. The draw reads every buffer from its `Gpu` field each frame, so a grown
> handle is picked up automatically — no bind group to rebuild for vertex/index/instance buffers.

> **Two strategies, one reason.** Meshes get the free-list (their vert+index blocks are big and
> re-uploading the tail on every delete would be the very cost we're avoiding); segments/glyphs get
> drain-shift (small, contiguous, no index — a tail rewrite is cheap and keeps them dense for the one
> `draw_indexed`). Both share `Gpu`'s existing empty-buffer guards.

`Gpu::new` (from 35) now builds the `GpuArena` and fills it by calling `allocate`/segment-append per
object instead of concatenating flat `Vec`s — but the object walk is unchanged; only the *sink* moved
from three flat buffers to the arena + range maps.

## Part C — the content fingerprint: `src/app/scene.rs`

Reconcile needs to know an object *changed*, not just that its guid still exists. The kernel has no
content hash — and the obvious `format!("{:?}", geom)` is a **trap**: `Mesh` stores its `vertex`,
`halfedge`, and `face` data in `HashMap`s, whose `Debug` iteration order is randomized *per map
instance*. The freshly-loaded object and the stored one are different map instances, so a `{:?}` hash
would differ even when the geometry is byte-identical — marking every mesh "changed" on every reload.

Use the kernel's `jsondump` instead: it emits **sorted** JSON (deterministic regardless of map order),
and every variant that has it round-trips through it in the minitests. `BRep` has no `jsondump`, so
fingerprint it by its tessellation plus placement:

```rust
use std::hash::{Hash, Hasher};

/// Deterministic content fingerprint — sorted JSON, so a HashMap-backed Mesh hashes the SAME every load.
/// Same geometry → same u64; any field change → a different one. (A production app might hash proto
/// bytes; the diff logic is identical either way.)
fn content_hash(geom: &Geometry) -> u64 {
    let s = match geom {
        Geometry::Mesh(m)     => m.jsondump().unwrap_or_default(),
        Geometry::Line(l)     => l.jsondump().unwrap_or_default(),
        Geometry::Polyline(p) => p.jsondump().unwrap_or_default(),
        Geometry::Point(p)    => p.jsondump().unwrap_or_default(),
        Geometry::BRep(b)     => format!("{}|{:?}", b.mesh().jsondump().unwrap_or_default(), b.xform.to_cols()),
        _ => String::new(),
    };
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}
```

`Scene` remembers last load's hashes (add `hashes: HashMap<String, u64>` to the struct, fill it in
`Scene::new` alongside `order`). That map is the "current document state" the next load diffs against.

## Part D — the diff: `src/app/scene.rs`

One pass over the union of old and new guids sorts every object into exactly one bucket:

```rust
pub struct Diff {
    pub added:   Vec<String>,   // guid in new, not in old
    pub removed: Vec<String>,   // guid in old, not in new
    pub changed: Vec<String>,   // guid in both, hash differs
}                               // unchanged (both, hash equal) is implicit — the whole point: it's skipped

impl Scene {
    /// Diff `new_session` against what's loaded; returns which objects actually moved. Does NOT touch
    /// the GPU — the caller applies the diff (Part E), then swaps in the new session + hashes.
    pub fn reconcile(&self, new: &Session) -> Diff {
        let (mut added, mut changed) = (Vec::new(), Vec::new());
        for (guid, geom) in &new.lookup {
            if !is_renderable(geom) { continue; }
            match self.hashes.get(guid) {
                None => added.push(guid.clone()),                              // new object
                Some(&h) if h != content_hash(geom) => changed.push(guid.clone()),  // edited
                Some(_) => {}                                                  // unchanged → skip
            }
        }
        let removed = self.order.iter()
            .filter(|g| !new.lookup.contains_key(*g))
            .cloned().collect();
        Diff { added, removed, changed }
    }
}
```

(`is_renderable` is the same 5-variant test `Scene::new` (35) already uses to build `order` — factor it
into a small free function both call, so the diff and the loader agree on what counts as an object.)

## Part E — apply the diff: `src/engine/gpu/mod.rs` + `src/app/scene.rs` + `src/state.rs`

The 35 litmus test still holds: `engine/` names no `Geometry`. So `Gpu` exposes only **GPU-typed**
verbs, and the `Geometry` match — which converts an object to those GPU types via 35's `push_mesh` /
`line_to_segment` / … (already in `scene.rs`) — stays in the app layer.

**Gpu verbs (GPU types only):**

```rust
    /// A mesh object's already-flattened data → arena + edge/naked tables. Scene did the conversion.
    /// Gpu owns its `device`/`queue`, so the verbs take neither — Scene needn't thread them through.
    pub fn add_mesh_data(&mut self, guid: &str, verts: &[RenderVertex], idx: &[u32],
                         edges: &[CylinderSegment], naked: &[GlyphPoint], row: u32) {
        self.arena.allocate(guid, verts, row, idx, &self.device, &self.queue);   // disjoint field borrows
        if !edges.is_empty() { self.append_segments(guid, edges); }
        if !naked.is_empty() { self.append_glyphs(guid, naked); }
    }
    pub fn add_segments(&mut self, guid: &str, s: &[CylinderSegment]) { self.append_segments(guid, s); }
    pub fn add_glyphs(&mut self, guid: &str, g: &[GlyphPoint])        { self.append_glyphs(guid, g); }

    /// Free an object's GPU data. Leaves its instance row alone (Scene owns rows) — hide it separately.
    pub fn remove_object(&mut self, guid: &str) {
        self.arena.free(guid, &self.queue);
        self.remove_segments(guid);
        self.remove_glyphs(guid);
    }
    pub fn hide_row(&mut self, row: u32) { self.write_row(row, |i| i.flags |= Instance::FLAG_HIDDEN); }

    /// (Re)point a row's instance + objects_base (33's rebase source). `row == len` extends both,
    /// growing the instance buffer 2x when it overflows — the same amortized growth as the arena.
    pub fn set_object_row(&mut self, row: u32, model: Xform, color: [f32;4], flags: u32) {
        if row as usize == self.objects_base.len() {
            self.objects_base.push((model.duplicate(), color, flags));
            self.instances.push(Instance { model: model.to_f32(), color, flags, _pad: [0;3] });
            if (self.instances.len() as u64) * SZ > self.instance_buffer.size() { self.grow_instances(); }
        } else {
            self.objects_base[row as usize] = (model.duplicate(), color, flags);
            self.instances[row as usize]    = Instance { model: model.to_f32(), color, flags, _pad: [0;3] };
        }
        self.write_row(row, |_| {});   // upload just this row (or the whole buffer after a grow)
    }
```

(`write_row(row, f)` mutates `instances[row]` and `queue.write_buffer`s that one row — the small helper
33's per-frame path and 37's cull already imply. `grow_instances` mirrors the arena's 2x copy for the
instance buffer, using `self.device`/`self.queue`. `SZ = size_of::<Instance>()`.)

**Scene owns the `Geometry` match + rows** — `apply_object` is 35's per-variant `build` logic for a
*single* object, calling the Gpu verbs; rows come from a small allocator that reuses removed rows:

```rust
    /// Flatten one object into the GPU at `row`, converting via 35's helpers (which live here in app).
    pub fn apply_object(&self, gpu: &mut Gpu, guid: &str, geom: &Geometry, row: u32) {
        gpu.remove_object(guid);   // idempotent: clears any prior data for this guid before (re)adding
        let (model, color) = match geom {
            Geometry::Mesh(m) => { let (v,i,e,n) = flatten_mesh(m, row); gpu.add_mesh_data(guid,&v,&i,&e,&n,row); (m.xform.duplicate(), m.objectcolor().to_f32()) }
            Geometry::BRep(b) => { let bm=b.mesh(); let (v,i,e,n)=flatten_mesh(&bm,row); gpu.add_mesh_data(guid,&v,&i,&e,&n,row); (bm.xform.duplicate(), b.surfacecolor.to_f32()) }
            Geometry::Line(l)     => { gpu.add_segments(guid, &[line_to_segment(l,row)]);      (Xform::identity(), l.linecolor.to_f32()) }
            Geometry::Polyline(p) => { gpu.add_segments(guid, &polyline_to_segments(p,row));   (Xform::identity(), p.linecolor.to_f32()) }
            Geometry::Point(p)    => { gpu.add_glyphs(guid, &[point_to_glyph(p,row)]);          (Xform::identity(), p.pointcolor.to_f32()) }
            _ => return,
        };
        gpu.set_object_row(row, model, color, 0);
    }

    /// Row for `guid`: its existing one, else a recycled free row, else a fresh one at the end.
    pub fn assign_row(&mut self, guid: &str) -> u32 {
        if let Some(&r) = self.guid_to_row.get(guid) { return r; }
        let r = self.free_rows.pop().unwrap_or(self.next_row);
        if r == self.next_row { self.next_row += 1; }
        self.guid_to_row.insert(guid.to_string(), r);
        r
    }
```

(`flatten_mesh` is 35's `push_mesh` split to *return* `(verts, idx, edges, naked)` instead of pushing
into shared buffers — the same body, different sink. `free_rows: Vec<u32>` and `next_row: u32` are new
`Scene` fields; `Scene::new` seeds `next_row = order.len()`.)

**State orchestrates the reload:**

```rust
    pub async fn reload(&mut self, url: &str) -> anyhow::Result<()> {
        let bytes = crate::app::persistence::fetch_bytes(url).await.unwrap_or_default();
        let new = crate::app::persistence::session_from_bytes(url, &bytes);
        let diff = self.scene.reconcile(&new);
        let unchanged = self.scene.order.len() - diff.changed.len() - diff.removed.len();
        log::info!("reload: {} added, {} changed, {} removed, {} unchanged",
            diff.added.len(), diff.changed.len(), diff.removed.len(), unchanged);

        for g in &diff.removed {
            let row = self.scene.guid_to_row[g];
            self.gpu.remove_object(g);
            self.gpu.hide_row(row);
            self.scene.free_row(g);                                   // guid_to_row.remove + free_rows.push
        }
        for g in &diff.changed {                                     // same row, re-flattened in place
            let row = self.scene.guid_to_row[g];
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        for g in &diff.added {                                       // fresh/recycled row
            let row = self.scene.assign_row(g);
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        self.scene.commit(new);   // swap session; rebuild order/hashes/bvh — but KEEP guid_to_row (below)
        Ok(())
    }
```

> **`commit` must not renumber rows.** It rebuilds `order`, `hashes`, and the BVH for the new document,
> but leaves `guid_to_row`/`free_rows`/`next_row` alone — those rows already point at the GPU data this
> reload just wrote, and 35's "row == order index" only ever held on the *first* load. After a reload,
> rows are whatever `assign_row` handed out; every consumer keys off `guid_to_row`, never `order`'s
> index, so the two are free to diverge. (`order`/the BVH map object_id → guid, which stays correct.)

> **Transform-only edits still re-flatten today.** Moving an object changes its `jsondump`, so it lands
> in `changed` and gets a full remove-then-add — correct, if wasteful for a mesh that only slid sideways.
> The refinement (a later lesson): fingerprint geometry and transform *separately*, and route a
> transform-only delta straight to `set_object_row` (33's `objects_base` rewrite) with no arena touch.
> The correctness path here — remove, re-add — is always right; that's just an optimization on top.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
cargo test -p session_viewer reconcile
```

The headline test is the diff log. Load `floor_model.pb`, then reload a copy with **one** wall moved:

```
reload: 0 added, 1 changed, 0 removed, 490 unchanged
```

One object re-flattened, 490 skipped — not 491 re-uploaded. A `#[cfg(test)]` proves the buckets:
build a `Scene`, `reconcile` against a session with one object added, one removed, one field-edited, and
assert `added/removed/changed` are exactly those three guids and everything else is unchanged. Visually,
the moved wall jumps to its new place and nothing else flickers — the untouched arena ranges were never
re-uploaded.

## Recap

```
Ch 37: frustum cull — per-object FLAG_CULLED, one draw preserved.
Ch 38: INCREMENTAL RELOAD. Two halves. (A) GpuArena replaces 30's flat arena: a free-list allocator
       over vbo+vids+ibo with guid→ArenaSlot{vertex_range,index_range} — bump-fill, best-fit reuse after
       frees, grow 2x; free() drops ranges on the free list and stamps a degenerate triangle so nothing
       draws the hole, never shifting neighbours. Segments/glyphs use the simpler guid→Range + drain-
       shift (archive's split: free-list for meshes, drain-shift for the small contiguous line/point
       tables). (B) content_hash(geom) = hash of its deterministic SORTED jsondump — a raw `{:?}` would
       hash the Mesh's HashMap fields in random order and read every object as changed. Scene keeps
       guid→hash. reconcile(new) diffs the union of guids → added / removed / changed(hash≠) /
       unchanged(skipped). State.reload applies remove/replace/add per bucket (Gpu = GPU-typed verbs, the
       Geometry match stays in Scene per 35's litmus) and commits the new session, KEEPING guid_to_row.
       Result: edit 1 of N objects → 1 re-flatten, N−1 skips, not N re-uploads. Transform-only edits still
       land in `changed` today (jsondump differs) — routing them to 33's objects_base rewrite is a noted
       optimization, not required for correctness.
```

Edited: `engine/gpu/arena.rs` (NEW — `GpuArena` free-list: `allocate`/`free`/`grow`, `guid→ArenaSlot`),
`engine/gpu/mod.rs` (`Gpu` owns the arena + `guid_to_seg`/`guid_to_glyph` + append/drain-shift +
`add_mesh_data`/`add_segments`/`add_glyphs`/`remove_object`/`hide_row`/`set_object_row`, all GPU-typed),
`app/scene.rs` (`content_hash`, `hashes`, `Diff`, `reconcile`, `apply_object`, `assign_row`/`free_row`,
`commit`), `state.rs` (`reload` — diff-driven, not rebuild-from-zero).

## Next

`39-save.md` — the reverse trip. An in-viewer edit marks the object dirty; a debounce (~1 s) coalesces a
burst of edits, recomputes the content hash, and — only if it actually differs — `pb_dumps` the session
to a `Blob` download (or a File System Access write). New objects get a fresh `guid`. The dirty-flag and
content-hash plumbing this lesson built is exactly what tells save what *not* to write.
