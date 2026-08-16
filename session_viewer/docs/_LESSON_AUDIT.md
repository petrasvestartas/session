# Lesson audit — 32b → 85 (2026-07-20)

Source-verified audit of the un-implemented draft lessons, triggered after **32a** shipped with real
bugs (WGSL typos that failed to compile, a zero-sized storage-buffer bind that crashed the frame, a
pipeline-layout arg-order mismatch). Every finding below was checked against the real
`session_rust/src` + `session_viewer/src` and adversarially re-verified before it survived. These are
**latent** bugs — they bite the reader the moment they follow the lesson, exactly like 32a's did.

Method: one reviewer per lesson → per-finding adversarial verify → synthesis. 55 lessons; 73 & 77 clean,
74 is a placeholder stub.

## 1. Critical & High bugs (confirmed, code-breaking)

### Critical (won't compile / renders nothing / wrong output)
- **32b** — Step 2 `vs_main` + Step 4b `draw(0..6*count, 0..1)` — point row read via `instance_index` but only 1 instance issued, so all N points collapse onto point 0 — **Fix:** `let p = points[vid / 6u];`, drop the `instance_index` param.
- **33** — Step 2e — `self.instance_buffer` referenced but it's a local in `Gpu::new`, never a struct field (E0609) — **Fix:** add `instance_buffer: wgpu::Buffer` to `struct Gpu` + the `Ok(Self{…})` initializer.
- **34a** — Step 3a/1b — `mod app;` added and `src/app/persistence.rs` created, but no `src/app/mod.rs` (edition 2024, E0583) — **Fix:** create `src/app/mod.rs` with `pub mod persistence;`.
- **34b** — Step 2b — `Geometry` match handles 9 of 11 variants, no wildcard; `NurbsCurve`/`NurbsSurface` uncovered (E0004) — **Fix:** add both to the no-op arm.
- **37** — Step 3 — `out_clipped()` writes `o.clip`; `VsOut`'s builtin field is `pos` in all 3 shaders (naga fail) — **Fix:** `o.pos = vec4<f32>(0.0);`.
- **38b** — Step 1 `content_hash` — `Mesh::jsondump` returns `serde_json::Value`, not `Result`; `.unwrap_or_default()` doesn't compile (propagates to 39) — **Fix:** `m.jsondump().to_string()`; keep Line/Polyline/Point on `.unwrap_or_default()`.
- **45** — Step 4 — click and marquee branches both run every release; a plain click gives a zero-area rect → `2.0/(x1-x0)` NaN frustum, and `select_marquee` clears the click's selection — **Fix:** guard on drag distance (`if drag < ~3px { click } else { marquee }`).
- **63** — Step 2 — `for c in &b.m_curves` — no such field; edge curves are `m_curves_3d` (`m_curves_2d` = trim pcurves) — **Fix:** `for c in &b.m_curves_3d`.
- **85** — Step 3 — new `Pipelines::new(...)` block drops the existing trailing `&glyph_layout` arg (9 params, 8 supplied) — **Fix:** insert `&material_layout` after `&instance_layout`, keep `&glyph_layout` last.

### High
- **38a** — Step 4 — "grown handle picked up automatically" is false for segment/glyph/instance **storage** buffers (bound once via `as_entire_binding`); a grown handle leaves bind groups pointing at the dropped buffer — **Fix:** recreate each bind group after swap. (+ `arena_index_count` removed with no accessor — add `index_count()`.)
- **42** — Step 2 — `ray.origin + ray.dir * 1.0e7` moves out of `&Ray`; `Point`/`Vector` aren't `Copy` (E0507) — **Fix:** `&ray.origin + &ray.dir * 1.0e7`.
- **45** — Step 2 tint — `o.color.a` / `mix(...vec4...)` in **triangle.wgsl** where `VsOut.color` is `vec3` — **Fix:** vec3 form, no alpha, mesh shader only.
- **45** — Step 2 tint — `inst.flags` in cylinder/sphere, which have no `inst` local (they read `instances[seg.instance_id]` / `[g.instance_id]`) — **Fix:** bind `let inst = instances[…];` first, per shader.
- **47** — Step 4 — `self.camera.set_ortho(…)` doesn't exist (only `toggle_projection`; field is `perspective`, the inverse) — **Fix:** add `set_ortho(on){ self.perspective = !on; }`, seed `ui.ortho = !camera.perspective`.
- **48** — Step 4 — `set_prompt` called 5× but never defined — **Fix:** show the body (writes both `self.get` and `self.ui.prompt`).
- **49** — Step 2 — `self.refresh_prompt()` undefined and unimplementable from the trait (verb/`what` live in `ProbeCmd`'s private `ask()`) — **Fix:** add `fn prompt(&self)->GetState` to the trait, then define `refresh_prompt`.
- **50** — Step 2 — new State field `history` collides with lesson 61's `pub history: History` on the same struct (E0124) — **Fix:** rename to `cli_history` (+ push/call sites).
- **56** — Step 2 — popup writes/reads `gb_submit` but only `gb_input` is added to `UiState` (E0609) — **Fix:** add `gb_submit: bool`. (Med: Step 3 `selection_centroid().unwrap()` panics if selection cleared — guard.)
- **58** — Step 3 — `ghost_segment(...)` used in polyline.rs + Step 4, defined/imported nowhere (E0425); return `CylinderSegment` not imported — **Fix:** define + import the helper.
- **59** — Step 1 — new `snap.rs` uses `Scene` and `Xform`, neither imported (E0412) — **Fix:** `use session_rust::{Geometry, Point, Xform};` + `use crate::app::scene::Scene;`.
- **61** — Step 2 — feeds `m.xform.duplicate()` (cached tess mesh's identity xform) as the instance model instead of `ns.xform`; a moved/rotated surface snaps back to origin on rebuild — **Fix:** `ns.xform.duplicate()`.
- **64** — Step 3 `trimmed_linework` — trim loops are parametric UV curves; sampling them "like brep_linework" draws boundary tubes flat at world origin — **Fix:** lift each `(u,v)` via `ts.surface().point_at(u,v)` first.
- **65** — Step 1 — `GroundUniform` puts `_pad: vec3<f32>` after a scalar → WGSL 112 B vs 96 B Rust mirror → min-binding-size panic — **Fix:** three scalar pads (96 B both sides).
- **67** — AO target `R16Float` but the pass writes `vec2(ao, encode(bent))` and 68 samples `.gba` — one channel can't hold AO + a direction — **Fix:** `RGBA16Float` (or oct-encoded `RG16Float`); state the channel budget.
- **68** — Step 1 — `decode_bent(ao_tex_sample.gba)` on `R16Float` returns constant `(0,0,1)`; directional GI flattens — **Fix:** reconcile AO format with 67; define `decode_bent`.
- **68** — Steps 1-3 — `base_ambient` / `key_light_term` / albedo don't exist (scene already lit in triangle.wgsl); undefined identifiers — **Fix:** add a G-buffer, or restate effects on `scene_rgb`/`ao`.
- **69** — Step 3 — composite ramp uses `inside`, `WIDTH`, samples `outline_dist`, none declared — **Fix:** derive `inside` from the mask, add `const WIDTH`, declare the `outline_dist` binding + entry.
- **70** — Step 3 — `intent.toggled_is_empty` (no such field) after `for g in intent.toggled` already moved the Vec (E0609 + E0382) — **Fix:** `let any_toggle = !intent.toggled.is_empty();` before the loop.
- **71** — Step 1 `reveal_in_tree` — walks `.parent` (private, E0616) and `.borrow()` on a `Weak` (needs `.upgrade()`, E0599) — **Fix:** use `node.borrow().ancestors()` / `.parent()`.
- **72** — Step 3 — `text.wgsl` shown "complete" but references undeclared `TextVertex`, `VsOut`, `mvp`, `line`, `atlas`, `samp`; uses the Rust CPU struct as the WGSL vertex-input type — **Fix:** prepend the uniforms/texture/sampler bindings + WGSL `TextVertex`/`VsOut`.
- **78** — Step 4 — `pl.flip()`; kernel `Plane` has no `flip` — **Fix:** `*pl = Plane::from_point_normal(pl.origin(), -pl.z_axis());`. Step 2 — discard loop reads `sec` uniform never declared — **Fix:** add `SectionUniform` + binding.
- **79** — Step 2 — `let done = /* clone the callback handle */;` syntax error; callback is `impl Fn` (not `Clone`) captured into `onchange` + `spawn_local` — **Fix:** `let done = Rc::new(done);` then clone inside.
- **80** — Step 3 — `AddGeometry::of_snapshots(...)` doesn't exist (57 gave `AddGeometry::one`; `of_snapshots` is on `RemoveObjects`) — **Fix:** add an `AddGeometry::of_snapshots` wrapper.
- **81** — Step 2 — `let hide = /* first token == "off" */;` syntax error; `Some("off")|Some("on")` discards the discriminating token — **Fix:** `Some(dir @ ("off"|"on"))` then `let hide = dir == "off";`.
- **82** — Step 1 — `u.angle(&w, false)` already returns **degrees**; `.to_degrees()` double-converts (right angle prints ~5156.62°) — **Fix:** drop `.to_degrees()`.
- **83** — Step 2 — `device.on_uncaptured_error(Box::new(...))`; wgpu 29 (and the viewer's gpu.rs) require `Arc::new` — **Fix:** `std::sync::Arc::new(...)`. (Med: selftest.rs `let ray = /* */;` + undefined `reconcile_one_changed`.)
- **84** — Step 2 — a fresh `[profile.release]` table pasted alongside the existing one (`strip = true`) → duplicate-key TOML parse error; `cargo build --release` fails — **Fix:** merge the three keys into the existing table, don't add a new header.
- **84** — Step 1 — calls `push_status(...)`, but lesson 88 named that static-queue channel `push_gpu_error`; `push_status` is defined nowhere (E0425) — **Fix:** call `push_gpu_error` (or introduce/rename in 83).

## 2. Recurring patterns (fix as batches)

- **Symbol used as if it exists (undeclared field/method/const/struct/binding)** — the single most common failure; batch a "grep every identifier against source" pass.
  - WGSL: 41 (`o.clip`), 45 (`inst.flags`), 69 (`inside`/`WIDTH`/`outline_dist`), 72 (`TextVertex`/`mvp`/`line`/`atlas`/`samp`), 78 (`sec`).
  - Rust: 33, 47, 48, 49, 56, 58, 63, 70, 71, 78, 80, 84.
- **`let x = /* … */;` comment-placeholder shipped as code** (comment ≠ expression → syntax error, and always hides the hard part): 79, 81, 83.
- **Struct field added but not initialized in the ctor / `State::new`:** 33, 39, 51, 53, 56, 59, 66, 75.
- **Struct padding / layout mismatch (std140/std430, vec3 16-B) Rust↔WGSL:** 65, 67/68, 78.
  - *Resolved (2026-07-21):* the "park foreign data in `LineUniform`" smell in **32b** (cloud size), **72**
    (`vp_w`/`vp_h` + a `_pad: vec3` size bug), and **78** (offered folding section planes in) — each now
    carries its own dedicated uniform (`CloudUniform` / `TextUniform` / `SectionUniform`).
- **Empty / zero-sized storage-buffer bindings:** 32b, 34b, 35, 58 — always route through `storage_buffer()`, never `create_buffer_init`.
- **Wrong shader filenames in prose** (`mesh.wgsl`/`point.wgsl` don't exist; real = `triangle.wgsl`, points = `sphere.wgsl`): 33, 37, 45, 78.
- **Non-exhaustive `Geometry` walk (11 variants):** 34b, 80.
- **`ri` / instance-row vs lookup-index vs cache confusion:** 34b, 35, 36→37, 61, 62.
- **Cross-lesson locals reused with no anchor** (`vp`/`origin`/`viewport`/`proj_y`/`ortho_h`/`vp_h`/`ray`/`tol`): 41, 42, 44, 45, 53, 59.
- **Load-bearing helper described only in prose, never coded:** 38a, 43, 52, 53, 54, 55, 60, 70, 81, 83.
- **Frame counter assumed to advance but never incremented (`self.frame` stuck at 0):** 39, 40, 66.
- **Duplicate manifest/config table:** 84 (`[profile.release]`).
- **Runtime hazards (unwrap / div-by-zero):** 45 (NaN), 56 (unwrap on empty selection).

## 3. Followability blockers (per lesson)

32b · pipeline-copy renames (fn/label/`include_str!` path) unstated; Step 4a must use `storage_buffer()`.
33 · unused `Color` import; `point.wgsl` missing from "unchanged shaders" list.
34a · console byte count `3070848` wrong (real `3026442`); deprecated setters warn.
34b · arena placeholder is a degenerate triangle (count 3, not 0); double empty-guard vs `storage_buffer` reads contradictory.
35 · Step 4 heading lists `src/lib.rs` but no lib.rs edit; two `pub`-field edits prose-only.
36 · `demo_session()` undefined yet Run says `cargo test bvh`; `AABB::new` is center+half-size; row==order-index unstated.
37 · `order_index` undefined; `let inst` anchor only matches triangle.wgsl; wrong shader names.
38a · `ensure_seg_capacity`/`alloc_i`/`grow_i` + Step-5 arena wiring elided; best-fit vs first-fit contradiction.
38b · `commit()` body never shown (must repopulate `hashes`); "uniform jsondump" false; `SZ` needs `as u64`.
39 · Ctrl+S wiring has no code/anchor; new State-field inits not shown.
40 · `poll_watch` async holds `&mut self` across `.await` but `render()` sync → watcher never runs; queue never coded.
41 · `self.aspect()` undeclared; click-handler anchor + new `self.cursor` field unstated.
42 · `Ray` not imported; 46's `#[derive(Copy)] Ray` invalid; `vp`/`origin`/`viewport` not in scope.
43 · `face_containing` prose-only; `SubHit{guid,kind,key}` prose vs 2-field struct; `Geometry` import unshown.
44 · `proj_y`/`ortho_h`/`vp_h` at pick site undeclared; contradictory borrow note; PointCloud silently unpickable.
45 · `Frustum::cropped` vs actual `Camera::marquee_frustum`; locals unestablished; wrong shader names.
46 · guard key spelling (`&guid` vs `guid` vs `h.guid()`) must match each prior loop.
47 · grid/edges draw-gate wiring prose-only; `Shell::new` accessors unshown.
48 · `cli_panel`→`build_ui` + `pending_command` prose-only/contradictory; `ui.log` naming.
49 · `Point::distance` arity contradiction; Step 2 must replace 53's branch (no anchor); `CmdOption` import missing.
50 · `cli_panel` gains 3 params but `UiState` fields + call site unshown; dispatch alias-removal prose-only.
51 · `history` not initialized in `State::new`; `execute()` note drops the `cmd` arg.
52 · entire GPU side elided (`gb_*` fields, buffers/bind groups, `upload_gumball`/`clear_gumball`, draw body, `GB_ROW`, `gb`).
53 · `ray_segment_distance`/`ray_point_distance` never coded; `ray`/`tol` undeclared; `gb_pressed`/`gb_hovered` + `build()` sig change unshown.
54 · release reads a "final delta on ctx" `DragCtx` never declares; `refresh_gumball_at` undefined; `begin_drag` body unshown.
55 · `now`/`a_dir`/`b_dir` underived; uniform-scale distance sourceless; Step 3 all placeholder + `DragCtx` `a0`/`d0` unshown.
56 · `apply_transform_command` name not established in 54/55.
57 · `RemoveObjects::of_snapshots`/`len` bodies + file unshown; `commit()` location unanchored; verb-match splice unstated.
58 · `cursor_world_point()` no body; `rect.rs` only two finish fragments; verb registration prose-only.
59 · `query_box_around()` no code; `snap_enabled`/`snap_marker` not in struct/ctor; `vp`/`origin` underived.
60 · `as_nurbscurve` undefined; sample cache unwired; `curve.rs` finish fragment; `curve_color` undefined; "~64 segments" arithmetic wrong.
61 · `surface_mesh(&mut self)` vs immutable surface-loop borrow (E0502) hidden by elision; `Option<&Mesh>` unwrap elided.
62 · widened tuple-return contract shown only as a comment; `push_mesh_faces_only` unshown; call-sites not updated.
66 · treadmill `request_redraw()` is the FIRST line of `render()`, not the last; `mark_dirty` vs `poke`; counters init prose-only.
70 · `flatten()` body unshown; drain insertion unanchored; `TreeIntent` struct never given as code.
71 · scroll target under two names (`ui.tree_scroll_to` vs `out.scroll_to`) with no bridge; caller edits unshown.
72 · `create_font_atlas`/`label_verts` bodies elided; 4-corners vs 6-verts ambiguity; group-2 filler + full layout array unstated.
74 · Step 1 has no anchor (placeholder lesson).
75 · `work_plane` ctor init unshown; `on_cplane_changed` prose-only yet called first; "zero tool edits" contradicts uv-conversion prose.
76 · `FLAG_CULLED` credited to 37 (really 36); lever-3 pseudocode not marked non-code.
78 · `SectionDrag`/`SectionBy3Points` undefined; VsOut `world_pos` plumbing prose-only; group slot ambiguous; `section_buffer`/bind-group never created; wrong file/shader names.
79 · STEP arm is a dangling `else if` (no anchor); web-sys features + `JsCast` import unstated.
80 · `CopyCmd` insertion + `from`/`to` bindings unshown; `Xform` import unstated; `n` reused.
81 · `active_layer_node()`/`layer_members()` never coded; `Session::find_group` analog not cited; `layer active` silently creates layer named "active".
82 · `a`/`b`/`c`/`v` picked-point vars declared nowhere; 3 new `UiState` fields only in a comment.
83 · `push_gpu_error` + static queue + drain prose-only; `build_cylinder_pipeline` placeholder args.
84 · features-block anchor missing ("add to web-sys features"); `index.html` `"0"`→`"z"` not an explicit find/replace; `## Next` says "nothing queued" yet `90-textures.md` exists.

## 4. Top illustration opportunities (inline SVG, match existing compact style)

1. **32b** — `vertex_index` decode strip: `point = vid/6`, `corner = vid%6` → `points[point]`. Encodes the fix for the critical collapse bug.
2. **45** — click-vs-marquee state machine (`|release−press| < 3px` → CLICK vs MARQUEE) — exactly the branch the code omits.
3. **65** — `GroundUniform` byte-map showing the vec3 alignment hole (84..96) inflating to 112 B vs the 96 B Rust mirror.
4. **61/62** — "SHAPE vs PLACEMENT": cached mesh (local verts, identity xform) vs `ns.xform` (the instance model the gumball composes into).
5. **67** — AO target channel byte-map: one `R16Float` channel vs `RGBA16Float` for AO + encoded bent normal (the `.gba` contradiction with 68).
6. **64** — before/after: trim loops flat in the UV unit square vs lifted onto the surface via `point_at(u,v)`.
7. **72** — `TextVertex` 44-B byte-map with each field's `@location(N)` + format (where Rust desyncs from WGSL vertex input).
8. **33** — transform-chain strip `clip = projection·view(−origin)·…·model·vertex` with both `−origin` boxes highlighted.
9. **34b** — lookup file-order → compacted `objects_base` rows with skipped variants X'd (why `instance_id` indexes the compacted row, not the map slot).
10. **38a** — two-column "set every frame" (arena vbo/vids/ibo → auto, green) vs "bound once in a bind group" (segment/glyph/instance → STALE unless recreated, red).
11. **84** — one-gulp vs streamed fetch: two-lane timeline (`array_buffer()` dead block + sudden 100% vs the chunk loop yielding frames + `5%…10%…`).

*Runners-up: 82 angle/circumradius geometry; 78 `d_rel = d + n·origin` rebase; 47 collect-in-`UiState`-then-apply borrow boundary.*

## Notes
- **73** and **77** were clean (no confirmed bugs, no followability blockers). **74** is a placeholder stub.
- Verifier softened four items to non-critical impact — **45** (NaN, not a panic), **35** (full sentence still compiles), **58** (empty-buffer crash only if a reader hand-rolls `create_buffer_init` past the guard), **62** (recoverable from lesson 68).
