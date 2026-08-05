# 38b Reconcile II — diff by guid, touch only what changed

> **Big picture.** *Phase 6 — the file is the source of truth.* 38a made single objects addressable on
> the GPU. Now the payoff: when a file is reloaded, don't rebuild the scene — **diff** the incoming
> `Session` against the loaded one by `guid`, and apply only the difference. Edit one wall in a
> 42,232-object drawing → one object re-flattened, 42,231 skipped. This diff is also the engine that
> 40 (watch) reuses verbatim, and its content-hash is what 39 (save) uses to skip pointless writes.

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
# content_hash; hashes: guid→u64; reconcile(new) → Diff; apply_object; row allocator
src/app/scene.rs
# GPU-typed verbs: add_mesh_data / add_segments / add_glyphs / remove_object / set_object_row
src/engine/gpu/mod.rs
src/state.rs              # reload(url): fetch → diff → apply per bucket → commit
```

## Step 1 — the content fingerprint: `src/app/scene.rs`

Reconcile needs to know an object *changed*, not just that its guid still exists. The kernel has no
content hash — and the obvious `format!("{:?}", geom)` is a **trap**: `Mesh` stores its `vertex`,
`halfedge`, and `face` data in `HashMap`s, whose `Debug` iteration order is randomized *per map
instance*. The freshly-loaded object and the stored one are different map instances, so a `{:?}` hash
would differ even when the geometry is byte-identical — marking every mesh "changed" on every reload.

Use the kernel's `jsondump` instead: it emits **sorted** JSON (deterministic regardless of map order),
and every variant that has it round-trips through it in the minitests. Mind the return types, though —
they are **not** uniform: `Mesh::jsondump` returns a `serde_json::Value` (so we `.to_string()` it),
while `Line`/`Polyline`/`Point` return `Result<String, _>` (so those `.unwrap_or_default()`). `BRep`
has no `jsondump` of its own, so fingerprint it by its tessellation (a `Mesh`, hence `.to_string()`)
plus placement (kernel-gap #6 in `_KERNEL_GAPS.md`: a single uniform `Geometry::jsondump()` /
`content_hash()` returning one type would collapse this whole match to one line):

```rust
use std::hash::{Hash, Hasher};

/// Deterministic content fingerprint — sorted JSON, so a HashMap-backed Mesh hashes the SAME
/// every load.
/// Same geometry → same u64; any field change → a different one. (A production app might hash proto
/// bytes; the diff logic is identical either way.)
fn content_hash(geom: &Geometry) -> u64 {
    let s = match geom {
        Geometry::Mesh(m)     => m.jsondump().to_string(),          // jsondump -> serde_json::Value
        Geometry::Line(l)     => l.jsondump().unwrap_or_default(),  // jsondump -> Result<String,_>
        Geometry::Polyline(p) => p.jsondump().unwrap_or_default(),
        Geometry::Point(p)    => p.jsondump().unwrap_or_default(),
        Geometry::BRep(b)     => format!("{}|{:?}",
            b.mesh().jsondump().to_string(), b.xform.to_cols()),    // mesh() -> Mesh -> Value
        _ => String::new(),
    };
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}
```

`Scene` remembers last load's hashes — add `hashes: HashMap<String, u64>` to the struct, and fill
it in `Scene::new`'s loop, right after the `order.push(guid.clone());` line:

```rust
                hashes.insert(guid.clone(), content_hash(geom));
```

(declare `let mut hashes = HashMap::new();` beside `order`, add `hashes` to the `Self { … }`).
That map is the "current document state" the next load diffs against.

## Step 2 — the diff: `src/app/scene.rs`

One pass over the union of old and new guids sorts every object into exactly one bucket:

```rust
pub struct Diff {
    pub added:   Vec<String>,   // guid in new, not in old
    pub removed: Vec<String>,   // guid in old, not in new
    pub changed: Vec<String>,   // guid in both, hash differs
// unchanged (both, hash equal) is implicit — the whole point: it's skipped
}

impl Scene {
    /// Diff `new_session` against what's loaded; returns which objects actually moved.
    /// Does NOT touch the GPU — the caller applies the diff (Step 3), then swaps in the new
    /// session + hashes.
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

`is_renderable` is the 5-variant test `Scene::new` (35) already uses to build `order` — factor it
into a free function both call, so the diff and the loader agree on what counts as an object. In
`scene.rs`, next to the converters:

```rust
fn is_renderable(g: &Geometry) -> bool {
    matches!(g, Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_) |
                Geometry::Polyline(_) | Geometry::Point(_))
}
```

and in `Scene::new`, the `if matches!(geom, …)` gate becomes `if is_renderable(geom)`.

## Step 3 — apply the diff: `Gpu` verbs + `Scene` dispatch + `State` orchestration

The 35 litmus test still holds: `engine/` names no `Geometry`. So `Gpu` exposes only **GPU-typed**
verbs, and the `Geometry` match — converting an object to those GPU types via 35's `push_mesh` /
`line_to_segment` / … — stays in the app layer.

**3a. Gpu verbs (GPU types only), in `src/engine/gpu/mod.rs`:**

```rust
    /// A mesh object's already-flattened data → arena + edge/naked tables. Scene did the
    /// conversion. Gpu owns its `device`/`queue`, so the verbs take neither — Scene needn't
    /// thread them through.
    pub fn add_mesh_data(&mut self, guid: &str, verts: &[RenderVertex], idx: &[u32],
                         edges: &[CylinderSegment], naked: &[GlyphPoint], row: u32) {
        // disjoint field borrows
        self.arena.allocate(guid, verts, row, idx, &self.device, &self.queue);
        if !edges.is_empty() { self.append_segments(guid, edges); }
        if !naked.is_empty() { self.append_glyphs(guid, naked); }
    }
    pub fn add_segments(&mut self, guid: &str, s: &[CylinderSegment]) {
        self.append_segments(guid, s);
    }
    pub fn add_glyphs(&mut self, guid: &str, g: &[GlyphPoint]) {
        self.append_glyphs(guid, g);
    }

    /// Free an object's GPU data. Leaves its instance row alone (Scene owns rows) — hide it
    /// separately.
    pub fn remove_object(&mut self, guid: &str) {
        self.arena.free(guid, &self.queue);
        self.remove_segments(guid);
        self.remove_glyphs(guid);
    }
    pub fn hide_row(&mut self, row: u32) {
        self.write_row(row, |i| i.flags |= Instance::FLAG_HIDDEN);
    }

    /// (Re)point a row's instance + objects_base (33's rebase source). `row == len` extends both,
    /// growing the instance buffer 2x when it overflows — the same amortized growth as the arena.
    pub fn set_object_row(&mut self, row: u32, model: Xform, color: [f32;4], flags: u32) {
        if row as usize == self.objects_base.len() {
            self.objects_base.push((model.duplicate(), color, flags));
            self.instances.push(Instance { model: model.to_f32(), color, flags, _pad: [0;3] });
            if (self.instances.len() as u64) * SZ as u64 > self.instance_buffer.size() {
                self.grow_instances();
            }
        } else {
            self.objects_base[row as usize] = (model.duplicate(), color, flags);
            self.instances[row as usize] =
                Instance { model: model.to_f32(), color, flags, _pad: [0;3] };
        }
        self.write_row(row, |_| {});   // upload just this row (or the whole buffer after a grow)
    }
```

`SZ` is a new module-level const — add near the top of `gpu/mod.rs`, next to the other consts:

```rust
/// One instance row's byte size — write_row / grow_instances offsets.
const SZ: usize = std::mem::size_of::<Instance>();
```

(a `usize`, so cast it — `SZ as u64` — before comparing against `instance_buffer.size()`, which
is a `u64`.) The two helpers `set_object_row` leans on:

```rust
    /// Mutate one instance row and upload just it (SZ bytes at row*SZ).
    fn write_row(&mut self, row: u32, f: impl FnOnce(&mut Instance)) {
        f(&mut self.instances[row as usize]);
        self.queue.write_buffer(&self.instance_buffer, row as u64 * SZ as u64,
            bytemuck::bytes_of(&self.instances[row as usize]));
    }

    /// Instance buffer overflowed: re-alloc 2x, rebuild the (bound-once) bind group, re-upload all
    /// rows. Same shape as 38a's ensure_seg_capacity; needs `instance_layout` hoisted onto Gpu.
    fn grow_instances(&mut self) {
        let need = (self.instances.len() * SZ) as u64;
        let mut cap = self.instance_buffer.size().max(SZ as u64);
        while cap < need { cap *= 2; }
        self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance.buffer"), size: cap, mapped_at_creation: false,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        self.instance_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instance.bind_group"), layout: &self.instance_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, resource: self.instance_buffer.as_entire_binding() }],
        });
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }
```

**3b. Scene owns the `Geometry` match + rows, in `src/app/scene.rs`** — `apply_object` is 35's
per-variant `build` logic for a *single* object; rows come from a small allocator that reuses freed
rows:

```rust
    /// Flatten one object into the GPU at `row`, converting via 35's helpers (which live here
    /// in app).
    pub fn apply_object(&self, gpu: &mut Gpu, guid: &str, geom: &Geometry, row: u32) {
        // idempotent: clears any prior data for this guid before (re)adding
        gpu.remove_object(guid);
        // instance color stays a WHITE TINT (34h) — the real colors ride the rows flatten_mesh
        // and the adapters bake; placement = the object's own xform, exactly as in build().
        let model = match geom {
            Geometry::Mesh(m) => {
                let (v,i,e,n) = flatten_mesh(m, row);
                gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
                m.xform.duplicate()
            }
            Geometry::BRep(b) => {
                let mut bm = b.mesh();
                bm.set_objectcolor(b.surfacecolor.clone());
                let (v,i,e,n)=flatten_mesh(&bm,row);
                gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
                b.xform.duplicate()
            }
            Geometry::Line(l) => {
                gpu.add_segments(guid, &[line_to_segment(l,row)]);
                l.xform.duplicate()
            }
            Geometry::Polyline(p) => {
                gpu.add_segments(guid, &polyline_to_segments(p,row));
                p.xform.duplicate()
            }
            Geometry::Point(p) => {
                gpu.add_glyphs(guid, &[point_to_glyph(p,row)]);
                p.xform.duplicate()
            }
            _ => return,
        };
        gpu.set_object_row(row, model, [1.0; 4], 0);
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

(`flatten_mesh` landed in 38a Step 5a — the per-object split of 35's `push_mesh`.
`free_rows: Vec<u32>` and `next_row: u32` are new `Scene` fields — add both to `struct Scene` and
to `Scene::new`'s `Self { … }`: `free_rows: Vec::new(), next_row: order.len() as u32`.)

**3c. State orchestrates the reload, in `src/state.rs`:**

```rust
    pub async fn reload(&mut self, url: &str) -> anyhow::Result<()> {
        let bytes = crate::app::persistence::fetch_bytes(url).await.unwrap_or_default();
        let new = crate::app::persistence::session_from_bytes(url, &bytes);
        let diff = self.scene.reconcile(&new);
        // guid_to_row is the pub view of "how many objects are loaded" (order is Scene-private)
        let unchanged = self.scene.guid_to_row.len() - diff.changed.len() - diff.removed.len();
        log::info!("reload: {} added, {} changed, {} removed, {} unchanged",
            diff.added.len(), diff.changed.len(), diff.removed.len(), unchanged);

        for g in &diff.removed {
            let row = self.scene.guid_to_row[g];
            self.gpu.remove_object(g);
            self.gpu.hide_row(row);
            // guid_to_row.remove + free_rows.push
            self.scene.free_row(g);
        }
        // same row, re-flattened in place
        for g in &diff.changed {
            let row = self.scene.guid_to_row[g];
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        for g in &diff.added {                                       // fresh/recycled row
            let row = self.scene.assign_row(g);
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], row);
        }
        // swap session; rebuild order/hashes/bvh — but KEEP guid_to_row (below)
        self.scene.commit(new);
        Ok(())
    }
```

> **`commit` must not renumber rows.** It rebuilds `order`, `hashes`, and the BVH for the new document,
> but leaves `guid_to_row`/`free_rows`/`next_row` alone — those rows already point at the GPU data this
> reload just wrote, and 35's "row == order index" only ever held on the *first* load. Every consumer
> keys off `guid_to_row`, never `order`'s index, so the two are free to diverge.

```rust
    /// Swap in the reloaded document: rebuild order + hashes + the pick BVH for `new`, but KEEP
    /// guid_to_row / free_rows / next_row — those rows already point at the GPU data reload wrote.
    pub fn commit(&mut self, new: Session) {
        // order + hashes: built exactly as Scene::new (35) does, over new's renderable objects
        self.order  = new.lookup.iter().filter(|(_, g)| is_renderable(g))
                                       .map(|(guid, _)| guid.clone()).collect();
        self.hashes = new.lookup.iter().filter(|(_, g)| is_renderable(g))
                                       .map(|(guid, g)| (guid.clone(), content_hash(g))).collect();
        // rebuild 36's world-AABB BVH + extents cache over the new document
        let (bvh, world_boxes) = Self::build_bvh(&new, &self.order);
        self.bvh = bvh;
        self.world_boxes = world_boxes;
        self.session = new;       // the reloaded doc is now current
    }

    /// Release a removed object's row for reuse (called in reload's `removed` loop).
    pub fn free_row(&mut self, guid: &str) {
        if let Some(row) = self.guid_to_row.remove(guid) { self.free_rows.push(row); }
    }
```

> **Transform-only edits still re-flatten today.** Moving an object changes its `jsondump`, so it lands
> in `changed` and gets a full remove-then-add — correct, if wasteful for a mesh that only slid
> sideways. The refinement: fingerprint geometry and transform *separately*, and route a transform-only
> delta straight to `set_object_row` (no arena touch). Noted, not required for correctness.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
cargo test -p session_viewer reconcile
```

The headline is the diff log. Load `floor_model.pb`, then reload a copy with **one** wall moved:

```
reload: 0 added, 1 changed, 0 removed, 490 unchanged
```

One object re-flattened, 490 skipped. A `#[cfg(test)]` proves the buckets: `reconcile` against a
session with one object added, one removed, one field-edited → `added/removed/changed` are exactly
those three guids. Visually, the moved wall jumps and nothing else flickers — the untouched arena
ranges were never re-uploaded.

## Recap

```
Ch 38a: per-object arena — free/replace one object's GPU bytes without touching neighbours.
Ch 38b: THE DIFF. content_hash = hash of the kernel's SORTED jsondump — a raw {:?} would hash the
        Mesh's HashMap fields in random order and mark every object changed every reload (the trap).
        Scene keeps guid→hash. reconcile(new) buckets the union of guids: added / removed /
        changed(hash≠) / unchanged(SKIPPED — the whole point). Apply: Gpu = GPU-typed verbs only
        (add_mesh_data/add_segments/add_glyphs/remove_object/set_object_row — 35's litmus holds);
        Scene owns the Geometry match (apply_object) + row recycling (assign_row/free_rows);
        State.reload runs remove → changed-in-place → added, then commit (which must NOT renumber
        guid_to_row — those rows point at live GPU data). Edit 1 of N → 1 re-flatten, N−1 skips.
```

Edited: `app/scene.rs` (`content_hash`, `hashes`, `Diff`, `reconcile`, `apply_object`,
`assign_row`/`free_row`, `commit`), `engine/gpu/mod.rs` (the five GPU-typed verbs, `write_row`,
`grow_instances`), `state.rs` (`reload` — diff-driven, not rebuild-from-zero).

## Next

`39-save.md` — the reverse trip. An in-viewer edit marks the object dirty; a debounce coalesces the
burst; the content hash from this lesson decides whether anything *truly* changed; only then does
`pb_dumps` produce bytes for a browser download. Three gates before one write.
