# 95 Control-point editing — F10, grab a CV, reshape

> **Big picture.** *Phase 13 — sub-object editing (78–81).* Until now the gumball moves whole
> objects; real modeling moves their **insides** — drag one control point and the surface flows. All
> the parts exist: CV glyphs (43), sub-object picking (56), the drag skeleton (67), Commands (64).
> What's genuinely new is the **update economics**: a shape edit must not re-flatten the object per
> mouse-move — with 48's arena the live path re-tessellates *nothing* and re-uploads *only the
> changed vertex range* (without it, the honest v1 previews the glyphs live and rebuilds once on
> release). Plus two kernel gotchas the archive paid for: `set_cv` silently drops rational weights
> (`set_cv_4d` keeps them), and a mesh edit without `invalidate_triangle_bvh` leaves picking on the
> **old** shape.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="f10 shows control points; dragging one moves the cv in the kernel and re-evaluates only affected samples; release commits one command" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 30,90 C 100,30 190,110 260,50" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <g fill="#d7dae0"><circle cx="30" cy="90" r="4"/><circle cx="100" cy="43" r="4"/><circle cx="190" cy="97" r="4"/><circle cx="260" cy="50" r="4"/></g>
  <circle cx="100" cy="43" r="8" fill="none" stroke="#e0b040" stroke-width="1.6"/>
  <path d="M 100,43 L 128,20" stroke="#e0b040" stroke-width="1.3" marker-end="url(#ah73)"/>
  <path d="M 30,90 C 128,10 190,110 260,50" fill="none" stroke="#888" stroke-width="1.4" stroke-dasharray="5 4"/>
  <text x="145" y="120" fill="#888" text-anchor="middle">drag CV → curve re-evaluates live</text>
  <g transform="translate(360,20)">
    <text x="0" y="14" fill="#d7dae0">live: set_cv_4d → resample → PARTIAL upload</text>
    <text x="0" y="34" fill="#666" font-size="10">write_buffer into 45's arena slot (v1: glyph preview only)</text>
    <text x="0" y="58" fill="#d7dae0">release: one EditShape Command</text>
    <text x="0" y="78" fill="#666" font-size="10">before/after Rc snapshots — undo = whole drag</text>
    <text x="0" y="100" fill="#e05555" font-size="10">traps: set_cv drops weights · stale triangle BVH</text>
  </g>
  <defs><marker id="ah73" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#e0b040"/></marker></defs>
</svg>

## Files we touch

```
src/state.rs         # F10 edit mode; CV picking (56's screen radius over CV glyphs); drag routing
src/app/scene.rs     # move_cv (set_cv_4d!) + live resample + partial upload; cache invalidation
src/engine/gpu/mod.rs # update_vertex_range — write_buffer into ONE arena slot's range
```

## Step 1 — edit mode: `src/state.rs`

`F10` toggles `edit_mode: bool`. In edit mode, the *selected* objects' CVs render emphasized (their
glyphs brighten — a flag on the glyph color at build time), and **clicks test CVs before objects**:
project each CV of each selected object (43's glyph positions), nearest within 8 px wins — exactly
56's vertex resolution, aimed at CVs:

```rust
    // press, edit mode, after gumball check: nearest CV of a selected curve/surface within R_PX →
    self.cv_drag = Some(CvDrag { guid, ij: (i, j), start_world: p, before: snapshot_of(guid) });
```

The drag itself reuses the gumball's *free* translation math: the CV follows the cursor's point on
the **camera-facing plane through the CV** (68's ray∩plane with n = view forward) — screen-natural
motion, no axis lock. (Axis-locked CV drags: select the CV, then use the gumball — later polish.)

## Step 2 — the kernel edit, weights intact: `src/app/scene.rs`

Two contracts meet in this one method, so read them first. **Frames**: the drag hands us a WORLD
point (the cursor's spot on the camera plane), but CVs are stored in the object's **local** frame —
geometry carries no placement, the row does (52) — so the point goes through
`placed_frame(row).inverse()` before it touches a CV, or CVs teleport by the manifest translation
the moment you edit anything on a placed sheet. **Ownership**: `Geometry` variants hold `Rc<T>`
shared between `lookup` and the `objects.*` collections — one allocation, two handles — and
mutation is copy-on-write: `Rc::make_mut` on one handle splits off a private copy while the *other*
handle keeps the old allocation. The kernel's contract (`session.rs`) is **lookup wins**: mutate
through `lookup.get_mut`, and `objects_synced()` re-shares the split at save time. Mutate the
collection entry instead and the edit is invisible to every lookup reader — and *discarded* on
save.

```rust
    /// Move one CV of a curve or surface. `new_world` is the cursor's WORLD point — it converts
    /// through the row's inverse placed frame, because CVs are LOCAL (52). 4-D: read the weight,
    /// write it BACK — set_cv (3-D) would silently reset a rational weight and visibly dent
    /// circles/spheres (the archive's bug). COW: mutate through lookup — lookup wins.
    pub fn move_cv(&mut self, guid: &str, ij: (usize, usize), new_world: &Point) {
        let Some(&row) = self.guid_to_row.get(guid) else { return };
        let Some(inv) = self.placed_frame(row).inverse() else { return };
        let p = inv.transform_point(new_world);                    // world → the object's frame
        let d = self.doc_of_row(row);
        match self.docs[d].session.lookup.get_mut(guid) {
            Some(Geometry::NurbsSurface(rc)) => {
                if let Some((_, _, _, w)) = rc.get_cv_4d(ij.0, ij.1) {
                    // homogeneous!
                    std::rc::Rc::make_mut(rc).set_cv_4d(ij.0, ij.1, p[0] * w, p[1] * w, p[2] * w, w);
                }
            }
            Some(Geometry::NurbsCurve(rc)) => {
                if let Some((_, _, _, w)) = rc.get_cv_4d(ij.0) {
                    std::rc::Rc::make_mut(rc).set_cv_4d(ij.0, p[0] * w, p[1] * w, p[2] * w, w);
                }
            }
            _ => {}
        }
    }
```

(Note the multiply: 4-D CVs store `(x·w, y·w, z·w, w)` — write the *homogeneous* coordinates, or a
weighted CV teleports. This is the whole reason the `_4d` API exists. And note the reads go through
the `Rc` directly — deref coercion; only the *write* pays `make_mut`, so an already-unique handle
mutates in place with zero copies.)

## Step 3 — live update, partial upload: `src/app/scene.rs` + `engine/gpu/mod.rs`

Per mouse-move: `move_cv`, then **resample into the existing layout** — same sample counts as the
cached tessellation/polyline, so vertex *count* is unchanged and the arena slot still fits — and
upload just that range.

**This write is 45's payoff, and it does not exist without the arena.** The per-object address comes
from the arena's guid→slot map (`arena.slots[guid].vertex_range` into `arena.vbo`); before 45 the
scene is one monolithic buffer with no per-object ranges, so there is nothing surgical to write.
The honest v1 without it: during the drag, preview only the **CV glyphs** (the dragged handle and
its polygon — a tiny write, and what your eye actually tracks); the body updates on **release** via
a tables rebuild + `gpu.set_scene(&scene.tables)` — correct, ~100 ms, once per drag, and the drag
itself stays at full frame rate. Do NOT call `set_scene` per mouse-move — a full-scene re-upload
per frame is exactly the economics this lesson exists to avoid. With 45 in place, the live path is:

```rust
    // gpu — the surgical write 45's slot map makes possible:
    pub fn update_vertex_range(&mut self, guid: &str, verts: &[RenderVertex]) {
        if let Some(slot) = self.arena.slots.get(guid) {
            // NOT a debug_assert: in a release build a mismatched write would bleed past the slot
            // and corrupt the NEXT object's vertices. Refuse loud; the caller falls back to the
            // v1 path (tables rebuild + set_scene) for this object.
            if verts.len() != slot.vertex_range.len() {
                log::warn!("update_vertex_range({guid}): {} verts, slot holds {} — skipping",
                           verts.len(), slot.vertex_range.len());
                return;
            }
            self.queue.write_buffer(&self.arena.vbo,
                slot.vertex_range.start as u64 * std::mem::size_of::<RenderVertex>() as u64,
                bytemuck::cast_slice(verts));
        }
    }
```

Curves are even lighter — `update_object_segments` rewrites their slice of the segment table (45's
range map) in place. Either way: **no allocation, no re-flatten, no full-buffer upload** — the perf
HUD's upload counter shows a few kilobytes per frame, and dragging a CV on the stress scene doesn't
move the frame time.

Two refinements to know about, neither needed for v1. **The live resample re-evaluates the whole
object** at the cached sample counts, but a NURBS CV only influences `degree + 1` knot spans —
span-local re-evaluation (find the affected parameter interval from the CV's index, resample just
those spans into the same vertex layout) shrinks the live CPU work from O(samples) to
O(spans × degree²). Worth it on dense surfaces; on curves you won't measure the difference.
**Fixed counts cut both ways**: keeping the cached sample count is what makes the slot write
legal, but sculpt a tight curl into a sparsely-sampled region and the live preview under-samples
it — faceting that the release rebuild's fresh tessellation (44's quality knobs) resolves. If the
preview faceting bothers you, resample *denser* during the drag and take the release path's rebuild
when the count changes (the guard above refuses the write and falls back).

On **release**: one `EditShape` Command — the 54 pattern, with 64's snapshot insight doing the heavy
lifting: `before`/`after` are cloned `Geometry` handles (`lookup.get(guid).cloned()` — an `Rc`
clone IS an absolute snapshot, because every later edit COWs a fresh allocation; restore = write
the handle back with `lookup.insert`, and `objects_synced()` heals the split at save). Then the
real bookkeeping: `tess_cache.remove(guid)` (44's one sanctioned invalidation) + the rebuild —
tables + `set_scene` in v1, one arena-slot rewrite with 45 — so normals and edge tubes true up at
the final shape; `hashes` refresh (46), and the row's world box + BVH refit (40 — the shape
changed, so its box did). And the **second trap**: if the edit target is a `Mesh` (dragging mesh
vertices works the same way), `invalidate_triangle_bvh()` before anything picks again, or 47 casts
rays against the shape *before* the drag. The kernel invalidates on its own mutators, but a direct
vertex write bypasses them.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **F10**, select 43's curve → CVs brighten. Drag one — the curve flows live under the cursor, edge
  tubes and glyph riding along; release → done; **Ctrl+Z** → the whole drag reverts as one action.
- The **weight test**: make a rational curve (a circle via `create_fitted` or a weighted fixture),
  drag a CV *near but not on* it — the circle deforms smoothly. Swap `set_cv_4d` for `set_cv` and
  repeat: the shape dents wrong as the weight resets. Swap back. That's the trap, felt.
- The **placed-sheet test**: drag a CV on any doc *after the first* (a manifest-placed sheet) — the
  CV stays pinned under the cursor. Skip the `placed_frame(row).inverse()` conversion in `move_cv`
  and repeat: the CV leaps by the manifest translation on the first move. That's the world-vs-local
  contract, felt.
- The **stale-BVH test**: drag a surface CV far, release, immediately click the *new* bulge —
  it picks. Comment out the cache/BVH invalidation on release and repeat: the click misses (it's
  casting at the old shape). Restore.
- Perf HUD during a drag: with 45, uploads = the one vertex range, frame time flat — compare
  against a deliberate full `set_scene` per move to feel why the slot write is the lesson. On v1,
  the drag stays smooth (glyphs only) and release costs one ~100 ms rebuild — visible, honest, and
  the exact hitch 45's arena deletes.

## Recap

```
Ch 77: Phase 12 closed.
Ch 78: CV EDITING. F10 mode: CVs pick first (56's radius over 43's glyphs); drag = camera-plane
       follow. Kernel write: resolve doc via guid_to_row/doc_of_row, mutate through
       lookup.get_mut + Rc::make_mut — COW, and LOOKUP WINS (objects_synced re-shares the split at
       save; a collection-side edit is invisible and discarded). Frames: the cursor point is WORLD,
       CVs are LOCAL → placed_frame(row).inverse() first, or CVs jump on placed sheets.
       get_cv_4d → set_cv_4d writing HOMOGENEOUS (x·w, y·w, z·w, w) — set_cv resets rational
       weights and dents circles (archive bug #1). LIVE: resample into the SAME layout →
       update_vertex_range = one write_buffer into 45's arena slot / update_object_segments for
       curves — REQUIRES 45's guid→slot map; v1 without it previews glyphs live and commits via
       tables rebuild + set_scene (~100 ms, once per release — never per move). RELEASE: EditShape
       Command (before/after = cloned Rc handles — stable snapshots by COW), tess_cache.remove +
       ONE rebuild, hash + box/BVH refresh, and invalidate_triangle_bvh for direct mesh-vertex
       writes — or picking targets the pre-drag shape (archive bug #2). Undo restores the whole drag.
```

Edited: `state.rs` (F10, CV pick, drag route), `app/scene.rs` (`move_cv` — doc resolve, COW via
lookup, world→local; live resample; release bookkeeping, `EditShape`), `engine/gpu/mod.rs`
(`update_vertex_range`, 45-gated).

## Next

`96-edit-points.md` — CVs are *off* the curve (they pull from a distance); **edit points** are *on*
it. Dragging a point the curve actually passes through — solved with the kernel's Greville abscissae
and a small linear refit, weights preserved.
