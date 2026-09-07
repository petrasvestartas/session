# Ink visibility from the depth buffer alone

2026-09-07. Design for the session_viewer line, edge, marker, BRep and NURBS display, replacing
the face-identity hidden-line machinery. Decisions taken with the user: drop the Tubes style,
drop view-dependent silhouettes on smooth tessellations, include the kernel change that makes
BRep edge and face tessellations share their boundary points, rewrite the lesson docs in the
same plan.

## 1. Goal

- Every stroke has the same weight everywhere: on its own faces, across a joint where two solids
  touch, at a silhouette, at a crease, at a vertex marker.
- A stroke behind another surface stays hidden. The floor census and the probe matrix that
  certify this today must still pass.
- BRep edges are stable under orbit and independent of face orientation.
- Per ink fragment: a handful of depth reads and a few dozen flops. No face identities, no
  support lists, no per-triangle planes, no compute pass, no second colour attachment.
- One flat ink shader for segments, one for glyphs. No Tubes.

Non-goals: screen-space silhouettes, analytic hidden-line removal, isocurves, line joins other
than the existing round caps (a later polish item).

## 2. Why the current rule fails

- **Half weight at joints.** `ink_visible_mask` tests the fragment pixel against whatever face
  covers it. A neighbouring solid's face is never a supporter, so the shader falls to the
  plane-side test with the axis lying exactly on that plane. Zero margin, f32 noise, pixel by
  pixel. The half of the stroke over the neighbour loses.
- **BRep edges flicker.** `walk_brep_edges` pushes them as free ribbons with no supporters at
  all (`hosts.associate` handles Line, Polyline, NurbsCurve, Point only). Every BRep edge is
  decided by that zero-margin test against its own tessellation. Face orientation is not the
  cause. On curved faces the edge samples come from the 3D curve and the facets are chords, so
  they never coincide.
- **Cost.** Per fragment: both endpoints re-placed, three matrix multiplies, depth and token
  loads at two pixels for up to four samples, a support-list scan, a plane fetch. Per frame: a
  second Rg16Uint attachment, a compute pass rewriting the whole index buffer on every camera
  move. Per scene: per-triangle planes and vertex copies. Measured in
  `_HIDDEN_LINE_VERIFICATION.md`: view_meshes +99% frame time, +175% memory.
- **Seam flicker.** `is_feature_edge` has a view-dependent silhouette term decided from two
  packed normals per segment.

## 3. The rule

The physical pass writes unbiased reverse-Z `Depth32Float` and nothing else. The ink pass reads
it as a **piecewise-planar surface** and asks one question per fragment.

### 3.1 One question per fragment

- The question is the same for every ink lane: **the surface under the fragment, carried to the
  stroke's axis, must not be nearer than the axis.**
- `z = depth(q)` at the fragment pixel `q`, at this fragment's sample index. `z == 0` (cleared)
  is background: visible, no fit.
- `step` is the unit texel step AWAY from the stroke, along the dominant component of the
  screen-space perpendicular, so both texels of the fit lie on the fragment's own surface.
- Fit the plane from `z` and `z_side = depth(q + step)`. Rim fallback: if that texel is cleared,
  or if `depth(q + 2 * step)` does not extend the same slope (`|g_far - g| > |z| * 2^-19 +
  (|g| + |g_far|) * 2^-8 px`, so the pair straddles two surfaces rather than sampling one),
  fit toward the stroke instead (`q - step`), so a face two texels wide, or one whose edge
  runs beside the stroke, still carries its own plane.
- Carry it to the axis. Write the displacement `A - q` as `a * along + b * side` (`along` is the
  stroke's unit screen direction, never parallel to `side`); the axis's own depth slope along
  itself supplies the `along` term, the fitted gradient `g = z_side - z` the `side` term:

```
predicted = z + a * slope_along + b * g
```

- **Visible iff `predicted <= zA + tol`**, where `zA` is the axis depth at `A` and

```
tol = |zA| * 2^-19  +  (|g| + |slope_along|) * 2^-8 px * (1 + lever)
```

  `lever = |b|` is the carry distance in texels. The first term is about 16 float ULPs. The
  second absorbs the rasterizer's subpixel quantisation (vertex positions snap to 1/256 px, so a
  plane's depth is off by slope times that; 2^-8 is exactly that quantisation, with the factor
  four of headroom measured away: at 2^-8 the close-up still counts 242406 non-background pixels
  and the probe matrix still passes 54 cases with its nine distance-1 counts unchanged, while the
  floor census residual falls from 29 samples to 13) over the distance it was extrapolated across.
- A texel already nearer than the axis (`z > zA + |zA| * 2^-19`) carries ink only when its
  fitted surface passes THROUGH the axis, `|predicted - zA| <= tol`, so a plane fitted in front
  of the axis that lands behind it cannot uncover a covered stroke.
- Raw compare only when no neighbour is written (both `q + step` and `q - step` cleared):
  visible iff `z <= zA + |zA| * 2^-19`.

Why one question and not a separate centreline and fragment test:

- On its own face, or a neighbouring face that touches the edge, the carry lands ON the axis:
  `predicted == zA` within tolerance. Both faces at a crease contain the edge, so both sides
  match at a convex or concave edge. A coplanar neighbour matches. A rising neighbour face (a
  plate in front of a box seen from above) matches because its plane passes through the edge.
- A nearer occluder carries nearer and hides the fragment even where the occluder recedes past
  the axis depth at this pixel. Comparing `z` against `zA` at the fragment instead lets a wide
  footprint over a grazing occluder uncover its own rim: the occluder's depth crosses the ink's
  depth within the footprint's half width. A beam 4 mm in front of an outline at 20 m is 2e-4
  relative, a hundred times the tolerance: hidden at every pixel of the footprint.
- A farther surface beyond a silhouette, or background, carries farther, so the stroke overhangs
  it at full width.

Discs (vertex markers, free dots) are camera-facing billboards at one constant depth, so the
whole disc stands or falls with its centre:

- `d = q - centre`; fit along BOTH axes, each stepping away from the centre:
  `gx = (depth(q + (sx,0)) - z) * sx`, `gy = (depth(q + (0,sy)) - z) * sy`.
- Carry to the centre: `predicted = z - d.x * gx - d.y * gy`. Visible iff
  `predicted <= depth_centre + tol` with `lever = |d.x| + |d.y|`.
- Both axes carry the same planarity guard as a stroke's fit (`depth(q + 2*(sx,0))` and
  `depth(q + 2*(0,sy))` must extend their slopes). There is no fallback direction, because both
  taps already step away from the centre: if either neighbour is cleared or either pair is not
  planar, the raw compare `z <= depth_centre + |depth_centre| * 2^-19` decides.

This is the polygon-offset look with the slope term replaced by the surface's measured plane,
so nothing is ever pushed by more than the rasteriser's own quantisation error.

### 3.3 Sampling

- MSAA: the shader is invoked per sample (`@builtin(sample_index)`); every read uses that
  sample index at the neighbouring texels, so the 1 px spacing is exact whatever the sample
  pattern. Coverage stays the per-pixel exact box filter. No `sample_mask`.
- The picking pass is single sampled and applies the same rule with sample 0.
- Reads outside the viewport count as cleared.
- Markers and dots: the plane is fitted along both axes away from the centre, from the
  fragment's texel and its x and y neighbours, and carried to the centre; the same question.
- Ortho and perspective are identical: `z/w` is affine in screen space for any plane in both.

### 3.4 Known residuals (measured, not argued away)

- A face under about 2 px wide on screen has no same-face neighbours: its own edge falls to the
  raw compare and may drop out at a grazing angle.
- A sliver face within about 1 degree of edge-on has depth quantised by up to slope/256 px. A
  hidden line exactly under such a sliver can show at isolated pixels. The compute filter that
  removed these is deleted; if the probe in section 7 shows leaks, the face fragment shader
  discards fragments whose `dpdx/dpdy` depth slope exceeds 50 px-eq per px (5 lines), and the
  gap it leaves is measured in the same probe.
- Measured: at 16x the fit distance the hidden-line fixture is 44 x 32 px, and a hidden edge
  within a pen width of its cover's silhouette paints up to 11 magenta pixels at 1400 x 900
  (regular top view; 3 iso, 4 warped top). Distances 1 and 4 are clean.
- Measured: the floor census surfaces 13 of 344 840 covered samples over its 21 cases, all of
  them at 16x: `iso_16` 3, `side_16` 2, `top_16` 8; every scale 1 and scale 4 case is zero, so
  `_ink_suite.sh` runs the census with `--require-zero-scales 1,4`. Two causes, both below what
  a depth rule can decide. At 16x the model spans about 40 px, so the cover's raster misses the
  pixel centre and the depth under the fragment is the surface BEHIND the cover, farther than
  the axis and therefore ink by any rule (4 samples; `iso_16` at 893,694 reads 0.00913877
  against an axis at 0.00913945, with the cover one texel away at 0.00916315). Where the cover
  is rasterised, its fitted plane passes through the axis to within the fit's own tolerance, so
  it is indistinguishable from an outline drawn on the covering face (9 samples, at 0.23 to 0.88
  of `tol`).

- Headroom: `SLOPE_PX = 2^-8` equals the 1/256 px vertex snapping of every GPU measured here.
  Vulkan guarantees only 4 subpixel bits; a device that snaps to 1/16 px needs a term 16 times
  larger, which would reopen the far-distance leaks above. The constant is one line in
  `ink_visibility.wgsl`; nothing else depends on it.

## 4. Data model

| row | bytes | fields |
|---|---|---|
| `CylinderSegment` | 40 | p0, radius, p1, instance_id, color, facing |
| `GlyphPoint` | 48 | center, radius, color, instance_id, facing, facing_ext |
| `LineUniform` | 64 | thickness, proj_y, ortho_h, vp_h, vp_w, eye, anchor, feather, lit, backface, pad |
| `Instance` | 96 | unchanged |

Deleted: `InkSupport`, `FacePlane`, per-vertex face ids, the Rg16Uint face attachment and its
placeholder views, `FaceFilter` and its params, `occluder_rect`, `LineStyle`, the cylinder
template. `facing` stays: it drives the vertex-stage cull (both faces away) and the crease
test on smooth tessellations.

Files deleted: `shaders/cylinder.wgsl`, `shaders/face_filter.wgsl`, `gpu/face_filter.rs`,
`gpu/occlusion_bounds.rs`, `gpu/plane_place.rs`, `walk/mesh_faces.rs`, `walk/mesh_raw_faces.rs`,
`walk/hosts.rs`. `shaders/ink_visibility.wgsl` is rewritten to the rule above.

Files simplified: `arena.rs` (one colour target, no planes, no filter), `targets.rs`,
`frame.rs`, `objects.rs` (no occluders), `segments.rs` (one shader, no supports),
`glyphs.rs`, `view.rs` and `input.rs` (no `L`, no `?style=`), `mesh.rs`, `mesh_ink.rs`,
`scene.rs` and `upload.rs` (no hosts, no face bases), `render.rs` (no prepare pass),
`layouts.rs` (ink groups bind depth only), `curves.rs`, `points.rs`, `frames.rs`.

## 5. Producers

- **Mesh edges** (`mesh_ink.rs`): unchanged topology, `facing` from the two adjacent normals
  with the `opposed` repair. Coplanar interior diagonals skipped as today.
- **Smooth tessellations** (`FLAG_SMOOTH`, BRep and NurbsSurface fills): border and crease
  edges only. `is_feature_edge` loses its silhouette term.
- **Markers**: unchanged facing triple, no supports.
- **Lines, polylines, NURBS curves, points**: rows only. No host association.
- **BRep edges**: phase 1 keeps `sample_nurbscurve` off the 3D curves. Phase 3 replaces it with
  the kernel's shared edge polygons (section 6), pushed as solid-lane pipes whose `facing` is
  the two adjacent facets' normals per segment, so BRep edges get the same cull as mesh edges.
- **Raw meshes** above `MESH_RAW_MIN`: faces only, as today, with no planes at all.

## 6. Kernel: one discretisation per BRep edge

The viewer can only be exact on BReps if the edge polyline and every adjacent face boundary
are the same points. Open CASCADE does this with `Poly_PolygonOnTriangulation`; this kernel
does it partially (`edge_bnd` in `face_meshes_q` covers grid-to-planar-CDT seams only).

Change, C++ ground truth first, then Rust and Python with identical names, tests and line
counts under the minitest rules:

- `BRep::edge_polygons_q(quality) -> Vec<Vec<Point>>`: per edge index, the 3D curve sampled
  once by the angle and chord of `quality`, endpoints exactly the edge's vertices, empty for a
  degenerated edge. `edge_polygons()` is the kernel default.
- `face_meshes_q` builds every face's UV loops from those parameters mapped through the face's
  pcurve (reversed for a Reversed use), and takes the 3D position of each boundary vertex from
  the edge polygon, never from the surface, so all faces share bit-identical boundary points.
  Interior refinement keeps the existing angle/chord behaviour (`mesh_render`).
- Boundary vertices carry the edge index and parameter as vertex attributes, so a consumer can
  walk a face mesh's border by BRep edge.
- Tests, one per method, identical across languages: `edge_polygons` count, endpoints and
  degenerate handling; `face_meshes` boundary points equal the edge polygon points exactly;
  `mesh()` of box, cylinder, cone, torus and block-with-hole is closed (`is_closed`) after the
  1e-6 weld.

Existing `brep_mesh*` tests keep passing with re-measured expectations where the tessellation
changes.

## 7. Verification

Existing, all must pass unchanged in meaning:

- `cargo xtest`, `cargo check` on wasm32 and native, clippy `-D warnings` on all targets.
- `docs/_gate.sh`: gate OK.
- `mk_hidden_line_probe` matrix (`docs/_probe_matrix.py`): 54 renders (regular, warped,
  authored; top, down, iso; 1, 4, 16 distance; MSAA 1 and 4), zero failures. Zero magenta at
  distances 1 and 4; at 16 the fixture is 44 x 32 px and up to 11 magenta pixels are accepted
  per the residual in 3.4. Blue strokes retained: the distance-1 counts hold.
- `docs/_hidden_line_matrix.py` on the floor model: 21 cases (one style now), run with
  `--require-zero-scales 1,4`: zero covered ink pixels at distance scales 1 and 4, with the
  16x residual measured and recorded in 3.4 rather than required to be zero.
- `check_hidden_line_lifecycle`, `check_determinism`.
- Close-up `view_local_boxes` at `VIEWER_ZOOM=5`: red edges full width to every corner.

New probes, added as examples and scripts:

- `mk_joint_probe`: a box standing on a plate, two boxes side by side sharing a face, a box
  inset 3 mm into a beam, a BRep cylinder on a plate, one BRep with two faces reversed. Rendered
  from six cameras at three distances.
- `docs/_stroke_weight.py`: sums ink across perpendicular cross-sections of a named edge from
  the id buffer and reports the ratio of a joint edge's weight to a free edge's weight. Accept
  when every joint edge is within 10% of the free edge and no cross-section is under 80%.
- Orbit invariance: 36 orbits of the BRep probe; the edge pixel count changes smoothly, no
  frame differs from its neighbours by more than 5%, and flipping face orientation changes no
  edge pixel.
- Sliver leak: the beam over the plate seen straight down and at 0.5 degrees off; magenta
  count reported, zero required.

Performance, measured with `bench_frame` on the Intel iGPU and the RTX 4080 here and written
into `docs/_PERF.md` with the command:

- view_mixed, view_meshes, view_lines still and moving. Budget: no worse than the pre-hidden-line
  ledger (view_mixed 10.9 / 22.9 ms on the iGPU). Memory: the Rg16Uint attachment, planes,
  supports and face ids are gone, so view_meshes returns to its base payload.

## 8. Docs

- `docs/05-ink.md` rewritten to teach the depth-buffer rule; Find / Replace pairs only, every
  block claimed by an op, `_replay_check.py --audit` zero orphans and `--stale` clean against
  the final tree. Lessons 4, 6 and 10 amended where they mention supports, tokens, Tubes or
  `L`.
- `ARCHITECTURE.md` sections 3 and 6 rewritten; the row table in section 6 updated.
- `_HIDDEN_LINE_TASK.md` and `_HIDDEN_LINE_VERIFICATION.md` move to `docs_archive/` with a
  one-line pointer to this spec. `_PERF.md` re-measured.
- The lesson for the kernel change lives with the kernel (session_cpp/py/rust), not here.

## 9. Phases

1. **The rule.** Shaders, lanes, producers, deletions, existing probes green, new probes and
   perf recorded. Ships on its own.
2. **Kernel edge polygons.** Section 6 in three languages, minitest green, submodule pointers
   bumped, CI green.
3. **BRep and NURBS in the viewer.** Edge polygons as pipes with facing, border-and-crease on
   smooth fills, orientation probe green.
4. **Docs.** Section 8.

Risks: phase 2 is the largest and touches the tessellator in three languages; phases 1 and 4
do not wait for it, phase 3 does. The sliver residual in 3.4 is the one place the rule is weaker than exact
identity; it is measured before being accepted.
