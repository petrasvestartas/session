# 63 Reconcile II — diff by guid, touch only what changed

> **Big picture.** *Phase 6 — the file is the source of truth.* 45 made single objects addressable on
> the GPU. Now the payoff: when a file is reloaded, don't rebuild the scene — **diff** the incoming
> `Session` against the loaded one by `guid`, and apply only the difference. Edit one wall in a
> 42,232-object drawing → one object re-flattened, 42,231 skipped. This diff is also the engine that
> 51 (watch) reuses verbatim, and its content-hash is what 50 (save) uses to skip pointless writes.

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
# content_hash; hashes: guid→u64; reconcile(new) → Diff; apply_object; row allocator; commit
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
they are **not** uniform: `Mesh::jsondump` and `Element::jsondump` return a `serde_json::Value` (so we
`.to_string()` it), while the other eight kinds return `Result<String, _>` (so those
`.unwrap_or_default()`). `BRep` has no `jsondump` of its own, so fingerprint it by its tessellation
(a `Mesh`, hence `.to_string()`) — kernel-gap #6 in `_KERNEL_GAPS.md`: a single uniform
`Geometry::jsondump()` / `content_hash()` returning one type would collapse this whole match to one
line.

One more input, and it is not optional: since the Xform refactor **no geometry carries its
placement** — an object's world xform lives in `Session.xforms`. A pure move therefore changes
nothing in the geometry's `jsondump`, so a JSON-only hash would file a moved wall under *unchanged*
and the GPU would keep drawing it in the old place. The fingerprint takes the object's session
world xform as a second input and hashes it in:

```rust
use std::hash::{Hash, Hasher};

/// Deterministic content fingerprint — sorted JSON, so a HashMap-backed Mesh hashes the SAME
/// every load — plus the object's session world xform (no geometry carries a placement; a pure
/// MOVE is invisible to the JSON alone).
/// Same geometry+placement → same u64; any field change → a different one. (A production app
/// might hash proto bytes; the diff logic is identical either way.)
fn content_hash(geom: &Geometry, world: &Xform) -> u64 {
    let s = match geom {
        // jsondump -> serde_json::Value
        Geometry::Mesh(m)          => m.jsondump().to_string(),
        Geometry::Element(e)       => e.jsondump().to_string(),
        // jsondump -> Result<String, _>
        Geometry::Line(l)          => l.jsondump().unwrap_or_default(),
        Geometry::Polyline(p)      => p.jsondump().unwrap_or_default(),
        Geometry::Point(p)         => p.jsondump().unwrap_or_default(),
        Geometry::Plane(p)         => p.jsondump().unwrap_or_default(),
        Geometry::OBB(b)           => b.jsondump().unwrap_or_default(),
        Geometry::PointCloud(pc)   => pc.jsondump().unwrap_or_default(),
        Geometry::NurbsCurve(c)    => c.jsondump().unwrap_or_default(),
        Geometry::NurbsSurface(sf) => sf.jsondump().unwrap_or_default(),
        // no jsondump of its own — fingerprint the tessellation (a Mesh, hence -> Value).
        // NO wildcard arm, same rule as add_file's match: a 12th kernel type must fail to
        // compile until the diff decides how to fingerprint it.
        Geometry::BRep(b)          => b.mesh().jsondump().to_string(),
    };
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    // Hash the xform's 16 f64 BIT PATTERNS — not format!("{:?}"), which would allocate a fresh
    // String per object per diff. (Bits, not values: -0.0 and 0.0 hash differently — the
    // conservative side for a change-detector, which may over-report but never under-report.)
    for c in world.to_cols().into_iter().flat_map(|col| col) {
        c.to_bits().hash(&mut h);
    }
    h.finish()
}
```

`Scene` remembers last load's hashes — add `hashes: HashMap<String, u64>` to the struct (init
`hashes: HashMap::new()` in `Scene::new`'s literal), and fill it in `add_file`'s walk loop, right
after the `self.guid_to_row.insert(guid.clone(), ri);` line (BEFORE `self.order.push(guid);` moves
the guid; `placement` is the closure the walk already has, so the hash sees the same session xform
the row was placed with — the manifest `place` stays out: it is viewer arrangement, not document
content):

```rust
            self.hashes.insert(guid.clone(), content_hash(geom, &placement(&guid)));
```

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
    /// session + hashes. (The demo manifest loads ONE file, so "what's loaded" IS the document;
    /// 45's watcher generalizes the diff per doc.)
    pub fn reconcile(&self, new: &Session) -> Diff {
        let world = new.world_xforms();
        let ident = Xform::identity();
        let (mut added, mut changed) = (Vec::new(), Vec::new());
        for (guid, geom) in &new.lookup {
            if !is_renderable(geom) { continue; }
            let h_new = content_hash(geom, world.get(guid).unwrap_or(&ident));
            match self.hashes.get(guid) {
                None => added.push(guid.clone()),                     // new object
                Some(&h) if h != h_new => changed.push(guid.clone()), // edited (or moved)
                Some(_) => {}                                         // unchanged → skip
            }
        }
        let removed = self.order.iter()
            .filter(|g| !new.lookup.contains_key(*g))
            .cloned().collect();
        Diff { added, removed, changed }
    }
}
```

`is_renderable` mirrors `add_file`'s walk (35): EVERY kernel type renders now, so the only
object that never gets a row is an `Element` whose `geometry()` is `ElementGeometry::None` —
`add_file` `continue`s it. The diff must agree, or an empty element would sit in `added` forever
and be pointlessly re-applied on every reload. In `scene.rs`, next to the converters:

```rust
/// Does this object get a row + GPU data? Since 35 everything does — EXCEPT an empty Element.
fn is_renderable(g: &Geometry) -> bool {
    match g {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}
```

## Step 3 — apply the diff: `Gpu` verbs + `Scene` dispatch + `State` orchestration

The 35 litmus test still holds: `engine/` names no `Geometry`. So `Gpu` exposes only **GPU-typed**
verbs, and the `Geometry` match — converting an object to those GPU types via 45's `flatten_mesh` /
35's `line_to_segment` / … — stays in the app layer.

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
            // extent/spacing: 0.0 = "unknown" (the ink lanes fall back gracefully); set_scene
            // measures them from the object bounds, and a size-aware refine could too.
            self.instances.push(Instance { model: model.to_f32(), color, flags,
                extent: 0.0, spacing: 0.0, _pad: 0 });
            if (self.instances.len() as u64) * SZ as u64 > self.instance_buffer.size() {
                self.grow_instances();
            }
        } else {
            self.objects_base[row as usize] = (model.duplicate(), color, flags);
            // Preserve the row's extent/spacing — set_scene measured them from the object's
            // bounds; overwriting with 0.0 would silently switch off the ink-lift clamp.
            let old = self.instances[row as usize];
            self.instances[row as usize] = Instance { model: model.to_f32(), color, flags,
                extent: old.extent, spacing: old.spacing, _pad: 0 };
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
    /// rows. Same shape as 45's ensure_seg_capacity; needs `instance_layout` hoisted onto Gpu.
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
per-variant `add_file` walk for a *single* object; rows come from a small allocator that reuses
freed rows. No geometry carries a placement, so the row's full world frame (`placed` — manifest
place × session world xform, exactly what `add_file` stores and 40's `placed_frame` reads) comes
in as a parameter:

```rust
    /// Flatten one object into the GPU at `row`, converting via 35's helpers (which live here
    /// in app). `placed` is the row's full world frame — the caller composes it (Step 3c).
    pub fn apply_object(&self, gpu: &mut Gpu, guid: &str, geom: &Geometry, placed: Xform,
                        row: u32) {
        // idempotent: clears any prior data for this guid before (re)adding
        gpu.remove_object(guid);
        // instance color stays a WHITE TINT (34h) — the real colors ride the rows flatten_mesh
        // and the converters bake. Same arms as add_file, same NO-wildcard rule.
        match geom {
            Geometry::Mesh(m) => {
                let (v,i,e,n) = flatten_mesh(m, row);
                gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
            }
            Geometry::BRep(b) => {
                let mut bm = b.mesh();
                bm.set_objectcolor(b.surfacecolor.clone());
                let (v,i,e,n) = flatten_mesh(&bm,row);
                gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
            }
            Geometry::NurbsSurface(s) => {
                let mut sm = s.mesh();
                if let Some(c) = s.facecolors.first() { sm.set_objectcolor(c.clone()); }
                let (v,i,e,n) = flatten_mesh(&sm,row);
                gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
            }
            Geometry::Element(el) => match el.geometry() {
                ElementGeometry::Mesh(m) => {
                    let (v,i,e,n) = flatten_mesh(m, row);
                    gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
                }
                ElementGeometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let (v,i,e,n) = flatten_mesh(&bm,row);
                    gpu.add_mesh_data(guid,&v,&i,&e,&n,row);
                }
                ElementGeometry::None => return,   // no GPU presence, no row write
            },
            Geometry::Line(l)        => gpu.add_segments(guid, &[line_to_segment(l, row)]),
            Geometry::Polyline(p)    => gpu.add_segments(guid, &polyline_to_segments(p, row)),
            Geometry::NurbsCurve(c)  => gpu.add_segments(guid, &nurbscurve_to_segments(c, row)),
            Geometry::Plane(p)       => gpu.add_segments(guid, &plane_to_segments(p, row)),
            Geometry::OBB(b)         => gpu.add_segments(guid, &obb_to_segments(b, row)),
            Geometry::Point(p)       => gpu.add_glyphs(guid, &[point_to_glyph(p, row)]),
            Geometry::PointCloud(pc) => gpu.add_glyphs(guid, &pointcloud_to_glyphs(pc, row)),
        }
        let flags = if self.hidden.contains(guid) { Instance::FLAG_HIDDEN } else { 0 };
        gpu.set_object_row(row, placed, [1.0; 4], flags);
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

(`flatten_mesh` landed in 48 Step 5a — the per-object split of 35's `push_mesh`.
`free_rows: Vec<u32>` and `next_row: u32` are new `Scene` fields — add both to `struct Scene`,
init `free_rows: Vec::new(), next_row: 0` in `Scene::new`'s literal — the scene starts EMPTY
since 35 — and keep `next_row` in step with the walk: one line at the bottom of `add_file`,
`self.next_row = self.tables.objects.len() as u32;`.)

**3c. State orchestrates the reload, in `src/state.rs`:**

```rust
    pub async fn reload(&mut self, url: &str) -> anyhow::Result<()> {
        // Guard like 45's poll: a failed/EMPTY fetch must leave the scene as-is. Parsing empty
        // bytes yields an empty Session, and the diff would bucket every loaded object as
        // `removed` — a network hiccup wiping the whole scene off the GPU.
        let bytes = match crate::app::persistence::fetch_bytes(url).await {
            Ok(b) if !b.is_empty() => b,
            _ => {
                log::warn!("reload: fetch failed or empty — keeping the current scene");
                return Ok(());
            }
        };
        // 35's sliced parse — session_from_bytes is GONE, the chunked version replaced it
        let new = crate::app::persistence::session_from_bytes_chunked(url, &bytes).await;
        let diff = self.scene.reconcile(&new);
        // guid_to_row is the pub view of "how many objects are loaded" (order is Scene-private).
        // Exact for the one-doc demo scene (live rows = unchanged + changed + removed); spans ALL
        // docs otherwise — 45's per-doc watcher needs the doc's own count. saturating_sub: a
        // missing row entry must never underflow a LOG line into a panic.
        let unchanged = self.scene.guid_to_row.len()
            .saturating_sub(diff.changed.len() + diff.removed.len());
        log::info!("reload: {} added, {} changed, {} removed, {} unchanged",
            diff.added.len(), diff.changed.len(), diff.removed.len(), unchanged);

        for g in &diff.removed {
            let row = self.scene.guid_to_row[g];
            self.gpu.remove_object(g);
            self.gpu.hide_row(row);
            // guid_to_row.remove + free_rows.push
            self.scene.free_row(g);
        }
        // The row's full world frame, composed exactly as add_file composes it. The reloaded
        // doc keeps its manifest place (the demo scene holds ONE doc; 45 generalizes per doc).
        let world = new.world_xforms();
        let place = self.scene.docs.first()
            .map(|d| d.place.duplicate()).unwrap_or_else(session_rust::Xform::identity);
        let placed = |g: &String| &place * &world.get(g).cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        // same row, re-flattened in place
        for g in &diff.changed {
            let row = self.scene.guid_to_row[g];
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], placed(g), row);
        }
        for g in &diff.added {                                       // fresh/recycled row
            let row = self.scene.assign_row(g);
            self.scene.apply_object(&mut self.gpu, g, &new.lookup[g], placed(g), row);
        }
        // swap session; rebuild order/hashes — but KEEP guid_to_row (below).
        // 52 EXTENDS commit with the touched-rows box re-walk + BVH rebuild.
        self.scene.commit(new, &diff);
        Ok(())
    }
```

> **`commit` must not renumber rows.** It rebuilds `order` + `hashes` for the new document, but
> leaves `guid_to_row`/`free_rows`/`next_row` alone — those rows already point at the GPU data
> this reload just wrote, and 35's "row == order index" only ever held on the *first* load.
> (When the BVH lands, [66](66-scene-bvh.md) extends this exact function with the touched-rows
> box re-walk and the `rebuild_bvh()` call — its step quotes the insertion point.)

```rust
    /// Swap in the reloaded document: rebuild order + hashes for `new`, but KEEP
    /// guid_to_row / free_rows / next_row — those rows already point at the GPU data reload
    /// wrote. (52 extends this with the touched-rows box re-walk + rebuild_bvh.)
    pub fn commit(&mut self, new: Session, diff: &Diff) {
        let world = new.world_xforms();
        let ident = Xform::identity();
        // order + hashes: rebuilt exactly as add_file (35) fills them — kernel-canonical order()
        self.order.clear();
        self.hashes.clear();
        for guid in new.order() {
            let Some(geom) = new.lookup.get(&guid) else { continue };
            if !is_renderable(geom) { continue; }
            self.hashes.insert(guid.clone(),
                content_hash(geom, world.get(&guid).unwrap_or(&ident)));
            self.order.push(guid);
        }
        // (52's hook lands here: touched-rows box re-walk + rebuild_bvh.)
        if let Some(d) = self.docs.first_mut() { d.session = new; }  // the reloaded doc is current
    }

    /// Release a removed object's row for reuse (called in reload's `removed` loop).
    pub fn free_row(&mut self, guid: &str) {
        if let Some(row) = self.guid_to_row.remove(guid) { self.free_rows.push(row); }
    }
```

> **Transform-only edits still re-flatten today.** Moving an object changes its world xform, which the
> fingerprint hashes in, so it lands in `changed` and gets a full remove-then-add — correct, if wasteful
> for a mesh that only slid sideways. The refinement: fingerprint geometry and transform *separately*,
> and route a transform-only delta straight to `set_object_row` (no arena touch). Noted, not required
> for correctness.

## Streamed clouds sit out the diff

`diff_sessions` walks kernel `Session`s by guid — and a streamed cloud (42) has neither:
no `Session`, no guid in any `lookup`, no bytes to hash. `Scene::rebuild` already
preserves its `CloudSlot`s and GPU rows untouched, and reconcile inherits that behaviour
for free as long as the diff only ever iterates `docs`. Do not "fix" that by inventing
hashes for slots: a re-streamed file replaces its cloud wholesale through the 42 path,
not through reconcile.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
cargo test -p session_viewer reconcile --target x86_64-unknown-linux-gnu   # override the wasm pin (52)
```

The headline is the diff log. Load `floor_model.pb`, then reload a copy with **one** wall moved:

```
reload: 0 added, 1 changed, 0 removed, 490 unchanged
```

One object re-flattened, 490 skipped. A `#[cfg(test)]` proves the buckets: `reconcile` against a
session with one object added, one removed, one field-edited → `added/removed/changed` are exactly
those three guids. Visually, the moved wall jumps and nothing else flickers — the untouched arena
ranges were never re-uploaded.

A second test pins the part that actually breaks in the field: after a reconcile, `order` and rows
have diverged, so a broad-phase query must return the row the GPU *drew* the object on — not the
stale order index. Add it inside 40's `#[cfg(test)] mod tests` in `scene.rs`:

```rust
    /// Pick-after-reconcile: move an object (same guid, new xform), commit, and the BVH must
    /// answer with ITS row at the NEW position — and nothing at the old one.
    #[test]
    fn pick_after_reconcile() {
        let mut scene = Scene::new();
        scene.add_file("a".into(), demo_session(), Xform::identity());

        let mut s2 = demo_session();
        let g = s2.lookup.keys().next().cloned().unwrap();
        s2.set_xform(&g, Xform::translation(50_000.0, 0.0, 0.0));   // a pure MOVE
        let diff = scene.reconcile(&s2);
        assert_eq!(diff.changed, vec![g.clone()], "a moved object is changed, not added/removed");
        scene.commit(s2, &diff);

        let row = scene.guid_to_row[&g];   // same row — commit must not renumber
        // AABB::new is CENTER + HALF-EXTENTS (52): the new spot catches it, the old one is empty.
        let new_spot = OBB::from_aabb(AABB::new(50_000.0, 0.0, 0.0, 500.0, 500.0, 500.0));
        assert_eq!(scene.objects_in(&new_spot), vec![row]);
        let old_spot = OBB::from_aabb(AABB::new(0.0, 0.0, 0.0, 500.0, 500.0, 500.0));
        assert!(!scene.objects_in(&old_spot).contains(&row));
    }
```

(The demo session has three lines; the moved one leaves the old box, its two unmoved sisters stay
— which is why the second assert is `!contains`, not `is_empty`. `demo_session` is 40's helper;
`Session::set_xform` is the same entry point 44's in-viewer moves save through.)

## Recap

```
Ch 45: per-object arena — free/replace one object's GPU bytes without touching neighbours.
Ch 46: THE DIFF. content_hash = hash of the kernel's SORTED jsondump + the object's session
        world xform (16 f64 bit patterns — no {:?} String alloc) — a raw {:?} would hash the
        Mesh's HashMap fields in random order and mark
        every object changed every reload (the trap), and a JSON-only hash would file a pure MOVE
        under unchanged (placement lives in Session.xforms, not the geometry).
        Scene keeps guid→hash. reconcile(new) buckets the union of guids: added / removed /
        changed(hash≠) / unchanged(SKIPPED — the whole point). Apply: Gpu = GPU-typed verbs only
        (add_mesh_data/add_segments/add_glyphs/remove_object/set_object_row — 35's litmus holds);
        Scene owns the Geometry match (apply_object — all 11 kinds, like add_file) + row recycling
        (assign_row/free_rows); State.reload GUARDS the fetch (empty bytes → keep the scene, or a
        hiccup diffs as "everything removed"), runs remove → changed-in-place → added, then
        commit(new, &diff)
        (which must NOT renumber guid_to_row — those rows point at live GPU data; order and rows
        diverge, so rebuild_bvh/objects_in map through guid_to_row — and which re-walks world
        boxes ONLY for the diff-touched rows, or it hands back the cost the diff saved). Edit 1
        of N → 1 re-flatten, N−1 skips.
```

Edited: `app/scene.rs` (`content_hash`, `hashes`, `Diff`, `reconcile`, `is_renderable`,
`apply_object`, `assign_row`/`free_row`, `commit(new, &diff)` — boxes re-walked for touched rows
only; `rebuild_bvh`/`objects_in` row-mapping; `pick_after_reconcile` test),
`engine/gpu/mod.rs` (the five GPU-typed verbs, `write_row`, `grow_instances`), `state.rs`
(`reload` — diff-driven, fetch-guarded, not rebuild-from-zero).

## Next

`64-save.md` — the reverse trip. An in-viewer edit marks the object dirty; a debounce coalesces the
burst; the content hash from this lesson decides whether anything *truly* changed; only then does
`pb_dumps` produce bytes for a browser download. Three gates before one write.
