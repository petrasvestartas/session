# 73 Control-point editing — F10, grab a CV, reshape

> **Big picture.** *Phase 13 — sub-object editing (73–76).* Until now the gumball moves whole
> objects; real modeling moves their **insides** — drag one control point and the surface flows. All
> the parts exist: CV glyphs (60), sub-object picking (43), the drag skeleton (54), Commands (51).
> What's genuinely new is the **update economics**: a shape edit must not re-flatten the object per
> mouse-move — the live path re-tessellates *nothing* and re-uploads *only the changed vertex range*.
> Plus two kernel gotchas the archive paid for: `set_cv` silently drops rational weights
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
    <text x="0" y="34" fill="#666" font-size="10">write_buffer(one vertex range), no re-flatten</text>
    <text x="0" y="58" fill="#d7dae0">release: one EditShape Command</text>
    <text x="0" y="78" fill="#666" font-size="10">before/after snapshots — undo = whole drag</text>
    <text x="0" y="100" fill="#e05555" font-size="10">traps: set_cv drops weights · stale triangle BVH</text>
  </g>
  <defs><marker id="ah73" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#e0b040"/></marker></defs>
</svg>

## Files we touch

```
src/state.rs         # F10 edit mode; CV picking (43's screen radius over CV glyphs); drag routing
src/app/scene.rs     # move_cv (set_cv_4d!) + live resample + partial upload; cache invalidation
src/engine/gpu/mod.rs # update_vertex_range — write_buffer into ONE arena slot's range
```

## Step 1 — edit mode: `src/state.rs`

`F10` toggles `edit_mode: bool`. In edit mode, the *selected* objects' CVs render emphasized (their
glyphs brighten — a flag on the glyph color at build time), and **clicks test CVs before objects**:
project each CV of each selected object (60's glyph positions), nearest within 8 px wins — exactly
43's vertex resolution, aimed at CVs:

```rust
    // press, edit mode, after gumball check: nearest CV of a selected curve/surface within R_PX →
    self.cv_drag = Some(CvDrag { guid, ij: (i, j), start_world: p, before: snapshot_of(guid) });
```

The drag itself reuses the gumball's *free* translation math: the CV follows the cursor's point on
the **camera-facing plane through the CV** (55's ray∩plane with n = view forward) — screen-natural
motion, no axis lock. (Axis-locked CV drags: select the CV, then use the gumball — later polish.)

## Step 2 — the kernel edit, weights intact: `src/app/scene.rs`

```rust
    /// Move one CV of a curve or surface. 4-D: read the weight, write it BACK — set_cv (3-D) would
    /// silently reset a rational weight and visibly dent circles/spheres (the archive's bug).
    pub fn move_cv(&mut self, guid: &str, ij: (usize, usize), new_p: &Point) {
        if let Some(ns) = self.session.objects.nurbssurfaces.iter_mut().find(|s| s.guid() == guid) {
            if let Some((_, _, _, w)) = ns.get_cv_4d(ij.0, ij.1) {
                // homogeneous!
                ns.set_cv_4d(ij.0, ij.1, new_p[0] * w, new_p[1] * w, new_p[2] * w, w);
            }
        } else if let Some(nc) = self.session.objects.nurbscurves.iter_mut()
            .find(|c| c.guid() == guid) {
            if let Some((_, _, _, w)) = nc.get_cv_4d(ij.0) {
                nc.set_cv_4d(ij.0, new_p[0] * w, new_p[1] * w, new_p[2] * w, w);
            }
        }
    }
```

(Note the multiply: 4-D CVs store `(x·w, y·w, z·w, w)` — write the *homogeneous* coordinates, or a
weighted CV teleports. This is the whole reason the `_4d` API exists.)

## Step 3 — live update, partial upload: `src/app/scene.rs` + `engine/gpu/mod.rs`

Per mouse-move: `move_cv`, then **resample into the existing layout** — same sample counts as the
cached tessellation/polyline, so vertex *count* is unchanged and the arena slot still fits — and
upload just that range:

```rust
    // gpu — the surgical write 38a's slot map makes possible:
    pub fn update_vertex_range(&mut self, guid: &str, verts: &[RenderVertex]) {
        if let Some(slot) = self.arena.slots.get(guid) {
            debug_assert_eq!(verts.len() as u32, slot.vertex_range.len() as u32);
            self.queue.write_buffer(&self.arena.vbo,
                slot.vertex_range.start as u64 * std::mem::size_of::<RenderVertex>() as u64,
                bytemuck::cast_slice(verts));
        }
    }
```

Curves are even lighter — `update_object_segments` rewrites their slice of the segment table (38a's
range map) in place. Either way: **no allocation, no re-flatten, no full-buffer upload** — the perf
HUD's upload counter shows a few kilobytes per frame, and dragging a CV on the stress scene doesn't
move the frame time.

On **release**: one `EditShape` Command (the 54 pattern — absolute before/after clones of the
curve/surface), then the real bookkeeping: `tess_cache.remove(guid)` + `apply_object` (one clean
re-flatten at final shape — normals and edge tubes true up), `hashes` refresh, BVH refit — and the
**second trap**: if the edit target is a `Mesh` (dragging mesh vertices works the same way),
`invalidate_triangle_bvh()` before anything picks again, or 42 casts rays against the shape *before*
the drag. The kernel invalidates on its own mutators, but a direct vertex write bypasses them.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **F10**, select 60's curve → CVs brighten. Drag one — the curve flows live under the cursor, edge
  tubes and glyph riding along; release → done; **Ctrl+Z** → the whole drag reverts as one action.
- The **weight test**: make a rational curve (a circle via `create_fitted` or a weighted fixture),
  drag a CV *near but not on* it — the circle deforms smoothly. Swap `set_cv_4d` for `set_cv` and
  repeat: the shape dents wrong as the weight resets. Swap back. That's the trap, felt.
- The **stale-BVH test**: drag a surface CV far, release, immediately click the *new* bulge —
  it picks. Comment out the cache/BVH invalidation on release and repeat: the click misses (it's
  casting at the old shape). Restore.
- Perf HUD during a drag: uploads = the one vertex range, frame time flat — compare against a
  deliberate `apply_object` per move to feel why partial upload is the lesson.

## Recap

```
Ch 72: Phase 12 closed.
Ch 73: CV EDITING. F10 mode: CVs pick first (43's radius over 60's glyphs); drag = camera-plane
       follow. Kernel: get_cv_4d → set_cv_4d writing HOMOGENEOUS (x·w, y·w, z·w, w) — set_cv resets
       rational weights and dents circles (archive bug #1). LIVE: resample into the SAME layout →
       update_vertex_range = one write_buffer into the arena slot (38a's map pays again) /
       update_object_segments for curves — no re-flatten per move. RELEASE: EditShape Command
       (before/after clones), tess_cache.remove + ONE re-flatten, hash + BVH refresh, and
       invalidate_triangle_bvh for direct mesh-vertex writes — or picking targets the pre-drag shape
       (archive bug #2). Undo restores the whole drag.
```

Edited: `state.rs` (F10, CV pick, drag route), `app/scene.rs` (`move_cv`, live resample, release
bookkeeping, `EditShape`), `engine/gpu/mod.rs` (`update_vertex_range`).

## Next

`74-edit-points.md` — CVs are *off* the curve (they pull from a distance); **edit points** are *on*
it. Dragging a point the curve actually passes through — solved with the kernel's Greville abscissae
and a small linear refit, weights preserved.
