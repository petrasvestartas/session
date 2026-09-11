# Adding a gumball

## The widget

- Transform gizmo on the selected object: three arrows translate along a world axis, three quarter-arcs rotate about one, three balls scale along one, a centre ball scales uniformly.
- Screen-constant at any zoom, depth and projection: a control that shrinks with distance stops being grabbable exactly when the object needs it.
- First thing here that **changes a document** — `Session::set_xform` inside a transaction. `history.rs` already stores `Op::Xform` absolute before/after, and `set_xform` records it whenever a transaction is open (`session_rust/src/session.rs`), so the viewer's commit is three lines.

## Where every piece goes

| file | what lands there |
|---|---|
| `src/app/gizmo.rs` (new) | `Handle`, handle geometry, screen-space hit test, drag laws. Pure CPU: no wgpu, no kernel, no `State`. |
| `src/engine/gpu/gizmo.rs` (new) | The lane: two `GrowBuf` tables, its one-row instance group, pipelines, `draw`. |
| `src/shaders/gizmo.wgsl` (new) | Vertex/fragment pair on the scene contract. |
| `src/math.rs` | `mat_translation`, `mat_rotation_about`, `mat_scale_about`, an `Aabb` midpoint. |
| `src/camera.rs` | `world_per_px(at, vp_h)`, `ray_at(cursor, viewport, aspect)`. |
| `src/engine/gpu/objects.rs` | `set_translation`, `set_place`, the retained local box per row. |
| `src/engine/gpu/mod.rs` | `pub gizmo: GizmoLane` beside `controls`; `set_place` / `set_translation` beside `set_selected`. |
| `src/engine/gpu/render.rs` | Two draws at the end of `scene_list` (`render.rs`). |
| `src/state.rs` | `gizmo` field beside `controls` (`state.rs`), the named actions, the per-frame rebuild in `render`. |
| `src/app/input.rs` | The gesture: press, drag, hover, release, cancel, Escape, Ctrl+Z. |
| `src/app/scene.rs` | `Scene::place_object` — the `Rc::make_mut` split, one kernel transaction. |
| `src/app/inspection.rs` | A `"gizmo"` key beside `"model"`. |

- Nothing in `pick.rs` changes, nothing in `id_pass`; reasons under hit testing.

## The handle set

```rust
// sketch
pub enum Handle {
 TranslateX, TranslateY, TranslateZ,
 RotateX, RotateY, RotateZ,
 ScaleX, ScaleY, ScaleZ,
 ScaleUniform,
}

impl Handle {
 /// (verb, axis, unit) — title of the numeric entry. Axis is "" for uniform scale.
 pub fn labels(self) -> (&'static str, &'static str, &'static str);
}
```

- Ten handles, no plane quads: a plane translate is two axis drags, and quads would crowd where arcs and arrows already make a pick ambiguous.
- No free-move on the centre ball in v1: one ball cannot be both uniform scale and screen-plane translate. Later a modifier, not an eleventh handle.
- World-axis only: the drag maths stays independent of the object, so a delta is a plain world matrix for one row or many. `gpu.objects.row(row).model` makes an object-aligned mode cheap later.

## Screen-constant size

- ONE length in CSS px, everything else a fraction of it, so retuning is one number:

```rust
// sketch, all CSS px
const ARC_R: f64 = 72.0; // the one length: arc radius and arrow tip
const AXIS_BALL: f64 = ARC_R * 0.5;
const BALL_R: f64 = 5.0;
const SHAFT_HW: f64 = 2.0;
const GRAB: f64 = 8.0;
const ARC_CHORDS: usize = 16;
```

- `ARC_R = 72` spans 144 px on a 400 CSS px phone canvas, a third of the width: grabbable, object still visible.
- Axis balls at half the arc radius: 36 px from the centre ball and from the arrow tip, over 4× `GRAB`, so collinear handles never conflict. They sit on the **positive** axis with the arrows, so pulling outward grows the object and press distance stays positive — no sign-preserving floor anywhere.
- `BALL_R = 5` beats the 3.5 px control dot (`state.rs:596`): a handle never reads as a control point.
- `GRAB = 8` beats `PICK_RADIUS = 6` (`pick.rs:34`) — grabbed, not aimed at — and `CLICK_SLOP = 4` (`input.rs:17`), so a click-length press stays on its handle.
- 16 chords per quarter arc: sagitta `r(1 − cos(θ/2))`, `θ = π/32`, is 0.087 px at `r = 72`.
- 51 `CylinderSegment` (3 shafts + 48 chords) + 4 `GlyphPoint` = 55 rows, rebuilt every frame; nothing to cache or invalidate.

### World units per pixel

```rust
// sketch, Camera, world units (mm), vp_h in framebuffer px
fn world_per_px(&self, at: [f64; 3], vp_h: f64) -> f64 {
 let d = if self.perspective { depth_along_view(at) } else { self.distance_world };
 2.0 * d * (FOVY_DEG * 0.5).to_radians.tan / vp_h
}
```

- Ortho is depth-free: `view_proj_anchored` half-height is `dist * tan(FOVY/2)` (`camera.rs`).
- Perspective takes the gizmo origin's own depth: the frustum widens with depth, so `distance` resizes the widget as you orbit an off-centre object.
- `vp_h` is `gpu.config.height`, framebuffer px; CSS constants scale by `f64::from(config.width) / logical_size[0]` first, as `upload_controls` does (`state.rs`).
- `FOVY_DEG` lives once (`math.rs`). `target`/`distance` are metres, `origin`/`distance_world` world units (`camera.rs`); mixing them silently disables camera-relative rendering.

## Drawing it

```rust
// sketch
pub struct GizmoLane {
 bars: GrowBuf, // CylinderSegment rows: shafts and arc chords
 dots: GrowBuf, // GlyphPoint rows: the four balls
 bars_group: wgpu::BindGroup, // group 3, l.ink_rows
 dots_group: wgpu::BindGroup, // group 3, same layout
 instance: wgpu::Buffer, // ONE Instance row
 translation: wgpu::Buffer, // ONE anchored translation
 group: wgpu::BindGroup, // group 2, l.instance
 gpu: GizmoPipelines,
}
```

- No new row type: `CylinderSegment` (`segments.rs`, 40 B) for shaft and chord, `GlyphPoint` (`glyphs.rs`, 48 B) for ball, as `upload_controls` fills (`state.rs`). Group 3 reuses `Layouts.ink_rows`, no new layout.
- Pipelines from `scene_module` (`pipelines/mod.rs:191`), **not** `ink_module`: ink fragments are gated on `ink_disc_visible` (`glyph.wgsl`), so a handle behind a face would vanish. Hence not a `GlyphLane`/`SegmentLane` pair like `controls`.
- Two more reasons for a private lane: the ink lanes bind `objects.ink_group` at group 2, not a one-row group; and `msaa_now` (`src/engine/gpu/mod.rs`) flips the canvas to 4× MSAA once `glyphs.sphere_count > 0`, so balls in the shared glyph lane would change every frame's antialiasing.
- `DepthMode::Always` (`pipelines/mod.rs:38`), already used for marker discs, gives always-on-top with no private pass. It also draws over itself: CPU-sort the 55 rows back-to-front before upload, rather than add a depth attachment and a pass.
- Wiring: field beside `controls` (`src/engine/gpu/mod.rs`), into `build`, `retarget`, `reset`, `release`, `allocated_bytes`; two draws after `self.text.draw(pass)` at the end of `scene_list` (`render.rs`), over authored text too; `SHADERS` into `lane_shaders` (`src/engine/gpu/mod.rs`) for the layout mirror test.

### Two shader conventions that will catch you

- `GlyphPoint.radius < 0` means **pixels** (`glyph.wgsl:52`): the balls use a negative, CSS-scaled radius.
- `CylinderSegment` has no pixel mode — `half_width_px` returns the global pen for radius ≤ 0 (`ribbon.wgsl`) — so shafts and arcs take a positive world radius, `SHAFT_HW × scale × world_per_px`, recomputed each frame.
- The instance row needs `flags = 0`, `color = [1,1,1,1]`: both shaders compute `g.color * inst.color`, then substitute `SELECT_COLOR` under `FLAG_SELECTED` (`glyph.wgsl`, `ribbon.wgsl`). A borrowed selected row turns the widget yellow.

## Anchoring it

- Shaders place a point as `place(i, p) = instances[i].model * p + translations[i]` (`scene.wgsl:57`), `translations[i] = (world − anchor) as f32`. A kilometre out, absolute f32 loses a millimetre (`objects.rs` test `the_anchor_is_what_keeps_a_small_move`) and drifts again on every re-anchor.
- So the lane owns **one** instance row in `l.instance` (`layouts.rs:46`): identity model, `translation = (origin − gpu.objects.anchor) as f32`, flags 0, white. Handle vertices are **local offsets about the origin**, which the size formula already produces, and moving the widget is one 16-byte write.
- Rewrite the origin after `rebase_anchor` in `State::render` (`state.rs`): a re-anchor changes what the offset means, and the scale is recomputed there too, from the frame's camera.

## Hit testing: pixels for the grab, a ray for the drag

**Join the id pass?** No:

- It is asynchronous: `Picker::request` (`pick.rs`) → `take_pending` next frame (`render.rs`) → `poll` a frame or more later (`state.rs`). A press must know instantly, or the object behind the handle is selected first.
- `State::touch` cancels any pick in flight (`state.rs:242`): a GPU-picking gizmo cancels its own question every frame of its own drag.
- Hover fires on every `CursorMoved` (`input.rs`); a pass, a copy and an async map per pointer event is not a hover budget.
- The id path earns its complexity on data the CPU lacks — half a million sheet segments behind one row, streamed cloud points (`pick.rs:1`). Ten analytic handles are the opposite case.

**So:** CPU, in **screen space** — project with the frame's own `gpu.frame.mvp_f32` and measure pixel distances, the space the widget is defined in.

```rust
// sketch
pub fn hit(&self, cursor: [f64; 2], projected: &Handles, grab_px: f64) -> Option<Handle>;
```

- Priority, first match wins: centre ball → axis balls → shafts → arcs. Shafts all start at the origin, so centre-first keeps the centre ball grabbable; arcs last, largest ambiguous area.
- Arcs test as **projected polylines** over the 16 drawn chords: the angular bound is free, where a plane-radius test picks the three quarters that are not drawn.
- Shafts test as projected 2D segments clamped to drawn length — no ray-parallel branch to get the sign wrong.
- `PickMode` (`pick.rs`) gains no arm and `id_pass` no draw: invisible to picks in all four modes. It is not an object.
- Consequence: no occlusion test, so a handle behind a wall still picks. Right, because it also *draws* over the wall — drawn-always and picked-always must be one rule.

### The ray the drag does need

- A drag needs a world answer — the cursor on the axis line, or crossing the drag plane. One ray per press and per drag frame, never per pointer event.
- No matrix inverse: `Camera::zoom_at` (`camera.rs`) already builds the view-plane frame — `right`, `self.up`, `half_h = distance * tan(FOVY/2)`, `half_w = half_h * aspect`. The ortho branch of `view_proj_anchored` uses the same `h` (`camera.rs`), so one rectangle serves both projections.

```rust
// sketch, Camera, world units
let ndc = [2.0 * cx / vp_w - 1.0, 1.0 - 2.0 * cy / vp_h];
let on_plane = target + right * ndc[0] * half_w + up * ndc[1] * half_h;
// perspective: origin = eye, direction = normalize(on_plane - eye)
// orthographic: origin = on_plane, direction = forward
```

## The drag laws

- **Every frame computes an absolute delta from the press state, never an increment.** A dropped frame changes nothing and no float drift accumulates.

```rust
// sketch
pub struct Drag {
 handle: Handle,
 start_world: [f64; 3], // translate: point on the axis; uniform scale: the plane point
 start_angle: f64, // rotate
 start_dist: f64, // scale
 plane: [f64; 3], // uniform scale: drag plane normal, frozen at press
 pivot: [f64; 3],
 turns: f64, // rotate: accumulated winding, so a drag can exceed ±180°
}

pub fn begin_drag(handle: Handle, ray: Ray, pivot: [f64; 3]) -> Option<Drag>;
pub fn update_drag(d: &mut Drag, ray: Ray) -> Option<Mat4>; // an absolute world delta
```

- **Translate.** Press: closest point on the axis line to the press ray, `t = ((w·axis) − (d·axis)(d·w)) / (1 − (d·axis)²)`, `w = ray.origin − pivot`. Frame: `mat_translation(cur − start)`.
 - Reject when `|1 − (d·axis)²| < 1e-8`: the ray is parallel, the answer unbounded, and the handle edge-on. That frame does not move.
- **Rotate.** Press: ray against the plane through the pivot normal to the axis, `θ0 = atan2(dp·v, dp·u)` in right-handed `(u, v, axis)`, so the angle grows CCW about `+axis`. Frame: `T(pivot) · R(axis, θ − θ0) · T(−pivot)`.
 - `atan2` returns `(−π, π]`: a naive difference caps a drag at ±180°, so 200° reads as −160°. Add ±2π to `turns` when the step exceeds π — two lines, full turns.
 - Refuse the press if the ray misses the plane (grazing view): a `θ0 = 0` fallback jumps the first frame by the whole measured angle.
- **Uniform scale.** Press: freeze the drag plane as the world axis most aligned with the view direction, so a mid-drag view change cannot jump the factor; `d0 = |p − pivot|`, floored at `1e-4`. Frame: `f = d / d0`, `T(pivot) · S(f) · T(−pivot)`.
 - **No damping exponent**: the archive's 0.25 power meant sixteen times the cursor travel to double the object.
 - Floor the **factor** at 0.01, not a ratio before a power: a zero must never commit a singular or mirroring matrix, and a pre-power floor gives an arbitrary smallest size (`0.01^0.25 ≈ 0.316`).
- **Axis scale.** Press: `t0 = (p − pivot)·axis`. Frame: `f = t / t0` floored at 0.01, `S(1 + ax(f−1), 1 + ay(f−1), 1 + az(f−1))` about the pivot.
 - Past the pivot `t` goes negative and the object collapses rather than mirroring: a mirror is a change nobody asked for by dragging.
- **Pivot** = midpoint of `gpu.objects.row_bounds(row)` (`objects.rs:466`), the point `fit_selected_or_all` already frames with (`state.rs`); `Aabb` has no midpoint helper, so one line in `math.rs`.
- **Only a translate moves the widget**; rotate and scale leave the pivot put, or the handles crawl away from what they turn.
- Orbiting mid-drag is impossible: the drag branch in `CursorMoved` returns before the camera controller sees the move, as `orbiting`/`panning` do (`input.rs`).

## Writing the move: 16 bytes or 112

- The branch is on the **write**, not the geometry type: placement here is a 96 B model row plus a 16 B anchored translation (`layouts.rs:44`).

```rust
// sketch, InstanceTable
pub fn set_translation(&mut self, ctx: &GpuCtx, row: u32, world: [f64; 3]); // 16 B
pub fn set_place(&mut self, ctx: &GpuCtx, row: u32, place: &Mat4); // 112 B
```

- Pure translate: `translation[row]` in f64 and one 16 B `translations.write_at` — the body of `rebuild`'s loop for one row (`objects.rs`).
- Rotate or scale: the 96 B row with a zeroed translation column as `append` does, **plus** the 16 B translation = 112 B, because the pivot is the box centre, so every rotation moves the translation too.
- Both add to the **f64 base**, never the f32 the GPU holds — `objects.rs` test `an_edit_written_past_the_base_does_not_survive_a_rebase`.
- Both refresh `world_bounds[row]` (`objects.rs:105`) and the matching `BoundedRow.lo/hi` (`objects.rs:64`, walked by `update_inside`); nothing else recomputes them, so skipping it leaves `F` framing the old place and the inside test stale.
- That needs the row's **local** box, dropped after upload (`scene.rs:219`), so `InstanceTable` retains a `Vec<Aabb>` of local boxes — **the one new piece of retained state the feature forces**.
- Both bump `geometry_revision`, cache key of the finite-visibility tiles (`render.rs`) and the outline masks (`MaskKey.geometry`, `render.rs`): **every drag frame re-renders the masks and re-bins the tiles.** Measure that before deciding whether a live drag drops `view.show_outlines`.: **every drag frame re-renders the masks and re-bins the tiles.** Measure that before deciding whether a live drag drops `view.show_outlines`.
- `Gpu::set_place` / `set_translation` forward beside `set_selected` / `set_hidden` (`src/engine/gpu/mod.rs`), each calling `splat.invalidate`: splat records fold `mvp × model` per cloud.
- Set `state.interacting = true` for the drag, as orbit and pan do (`input.rs:96`).

## The commit: one transaction in the kernel

```rust
// sketch, Scene
pub fn place_object(&mut self, row: u32, world_place: &Mat4) -> bool;
```

- Resolve the row to `(document index, guid)` with `Scene::identity_of` (`scene.rs`).
- Invert the walk — forward it is `object_place = doc.place · world_xform(guid)` (`placement`, `scene.rs`, used at), `world_xform = ancestors · local` (`session.rs:379`):

```text
local_new = (ancestors)⁻¹ · doc.place⁻¹ · new_object_place
```

- An identity manifest `place` and no ancestor transform collapse that to `local_new = D · local_old` for world delta `D`. Both forms belong in the code: the collapse runs, the general form survives a manifest that places a file.
- **`Rc::make_mut(&mut doc.session)` first.** `FileDoc.session` is shared — a manifest listing one file twice hands both documents the same `Rc` (`scene.rs`), `live.rs` keeps its own clone — so without the split, moving one placement moves them all. Test: `an_edit_must_split_a_session_two_placements_share` (`scene.rs`).
- Then three lines:

```rust
// sketch
session.begin("move");
session.set_xform(&guid, local_new);
session.commit;
```

- `set_xform` records `Op::Xform` absolute on both sides **by itself** when a transaction is open (`session.rs:346`) — and records nothing when none is.
- **Do not call `Scene::rebuild`** (`scene.rs`): a full re-walk whose own comment says streamed clouds and sheets cannot come back. The instance row holds the answer, and `placement` reproduces it on any later rebuild.
- Call `update_label` (`src/state/text.rs`) after the commit, or the nameplate stays at the old position — it is placed from `row_bounds(row)`.

## Undo and redo

- `State::undo` / `redo` call `Session::undo` / `redo` (`session.rs:1622`), which take the buffer out and put it back, so no borrow problem reaches the viewer. `History` caps at `CAPACITY = 64` and clears redo on commit.
- **No viewer-side undo stack**: a second source of truth for one edit. `XformOp` is absolute on both sides, so a re-baked delta cannot double a move.
- `Session::undo` answers only `bool`, so after undo or redo call `session.world_xforms` once for that document (one downward pass, `session.rs:398`) and re-place its rows through `placement` and `Gpu::set_place` — hundreds of rows, not millions. Then re-park the origin and call `update_label`.
- Bind in `Input::key` (`input.rs`); `Input` already tracks `ctrl` (`input.rs`), and modifiers arrive through `ModifiersChanged`, which the Ctrl-click edge pick already depends on.

## Numeric entry

- Under `CLICK_SLOP` a press on a handle is a **click** asking for a typed value, past it a **drag** — the same constant already decides click-versus-drag for selection (`input.rs:16`).
- Title from `Handle::labels` — "Move X (mm)", "Rotate Z (deg)", "Scale (factor)" — anchored at the press point.
- No egui: a DOM element beside `#viewer-status` (`index.html:54`), driven by an accessor shaped like `feedback::status` (`src/app/feedback.rs`) behind `#[cfg(target_arch = "wasm32")]`, applied by `State::gizmo_value(f64)`.
- `manual_delta(handle, value, pivot)` is relative and undamped, with the same 0.01 floor on any scale factor. A parse failure reports through `status` rather than cancelling silently.
- The canvas carries `tabindex="0"` (`index.html:52`): return focus on apply and on cancel, or the next keystroke goes nowhere.

## Multi-selection

- `Scene.selected` is `Option<u32>` (`scene.rs`), changed only in `State::select` (`state.rs`), so there is no centroid question: the pivot is one row's box midpoint, parked or cleared in `select`, `clear` and `escape_selection`.
- Multi-selection first needs `Scene.selected` to become a set, changing `set_selected`, the `selection_revision` outline-mask key and `fit_selected_or_all`; writing it now would be inventing.
- Keep the boundary so it stays cheap later: `begin_drag` / `update_drag` take a pivot and return a world matrix, so N rows is a loop. Keep press-time placements in a map keyed by row.

## A gumball on a streamed cloud or a sheet

- `add_streamed_cloud` (`scene.rs`) and `add_sheet` (`scene.rs`) push a `FileDoc` whose session is an empty `Rc::new(Session::new(&name))`, `display_only: true`: an object row, no guid, nothing to `set_xform`, nothing to save.
- Do not refuse the move — the row is first-class everywhere else. It writes the instance row, opens **no transaction**, and says so in the status line: a **view placement**.
- The cheapest object here to move: half a million sheet segments behind one row move for one 16-byte write, because each is placed through `place(i, p)`. Later slices arrive into the same row, so a move survives streaming.
- Two obligations: `splat.invalidate` on every write to a cloud row; and `gpu.bounds` / `scene.tables.bounds` are unioned once per slice (`scene.rs:352`) and never recomputed, so `fit_all` frames the old place — recompute after a commit, or say so in the code.

## Input routing and cancellation

- Everything reaches `State` as a **named action**, as `request_selection` / `escape_selection` / `enable_controls` do (`input.rs`): `gizmo_press`, `gizmo_hover`, `gizmo_drag`, `gizmo_release`, `gizmo_cancel`, `gizmo_value`, `toggle_gizmo`, `undo`, `redo`. No widget reaches into `Scene`.
- `gizmo_press(x, y) -> bool` goes in the **Pressed** arm of `Input::left` (`input.rs`), before `left_down` is armed, returning early when it grabs, so a press on a handle never becomes an object pick.
- **Left-drag is unclaimed**: RMB orbits, MMB pans (`input.rs:88`), and `CursorMoved` forwards to the camera only while `orbiting || panning`. In it (`input.rs:115`): `gizmo_drag` when a handle is engaged, `gizmo_hover` otherwise, both before the camera branch.
- `Input::cancel` (`input.rs`) already runs on focus loss and `pointercancel` (`Msg::CancelPointer`, `lib.rs`); `gizmo_cancel` hangs there, so a release the viewer never sees cannot leave the widget tracking the cursor.
- Escape (`input.rs:56`) takes a live drag **first** and restores the press-time placement, then falls through to `escape_selection`; otherwise Escape during a drag both commits the move and drops the selection.
- Hover sets `dirty` only when the hovered handle **changes** — `needs_frame` and `dirty` are split for this (`state.rs`) — and must not call `State::touch`, which cancels the pick in flight (`state.rs:242`).
- Touch: one finger orbits, two pan and pinch (`src/app/touch.rs`). No free gesture is left for a handle drag, so touch gets tap-to-select plus the numeric entry.

## The order that compiles

Each step builds on its own.

1. **`src/math.rs`** — `mat_translation`, `mat_rotation_about(pivot, axis, angle)`, `mat_scale_about(pivot, factors)`, f64 column-major beside `mat_mul` (`math.rs`), plus the `Aabb` midpoint, with unit tests.
2. **`src/camera.rs`** — `world_per_px` and `ray_at`, from the `zoom_at` view-plane frame (`camera.rs`).
3. **`src/engine/gpu/objects.rs`** — retain the local `Aabb` per row; `set_translation` and `set_place`, refreshing `world_bounds[row]` and the matching `BoundedRow`, bumping `geometry_revision`. Test: "a moved row reports the moved world box".
4. **`src/engine/gpu/mod.rs`** — `Gpu::set_place` / `set_translation` beside `set_selected` (`src/engine/gpu/mod.rs`), each calling `splat.invalidate`.
5. **`src/shaders/gizmo.wgsl` + `src/engine/gpu/gizmo.rs`** — the lane, empty: two `GrowBuf` tables on `l.ink_rows`, one-row group 2 on `l.instance`, `new` / `retarget` / `reset` / `release` / `allocated_bytes` / `set_origin` / `upload` / `draw`; pipelines from `scene_module` at `DepthMode::Always`; `SHADERS` into `lane_shaders`.
6. **`src/engine/gpu/mod.rs` + `render.rs`** — the field into `build`, `retarget`, `reset`, `release`, `allocated_bytes`, and two draws at the end of `scene_list`. Tables empty, picture unchanged: the wiring lands before anything about it can be wrong.
7. **`src/app/gizmo.rs`** — the widget, pure CPU: `Handle` and `labels`, geometry builders returning local-offset rows, the hit test, `begin_drag` / `update_drag`, `manual_delta`. Unit tests for hit-test priority and every drag law. `docs/locator.py` refuses to run when a taught file matches no zone, and this path matches none (`docs/locator.py:43-76`): add it to the `scene` zone's matchers first.
8. **`src/state.rs`** — the `gizmo` field (`state.rs:49`); `upload_gizmo` from `render` after `rebase_anchor` (`state.rs`); origin set or cleared in `select`, `clear`, `escape_selection`. **First visibly working point**, no interaction yet.
9. **`src/state.rs` + `src/app/input.rs`** — the named actions and the gesture. Objects move; nothing is recorded yet.
10. **`src/app/scene.rs` + `src/state.rs`** — `Scene::place_object`: the `Rc::make_mut` split, the inverse composition, `begin` → `set_xform` → `commit`, then `update_label`. Streamed and sheet rows take the view-placement branch.
11. **`src/state.rs` + `src/app/input.rs`** — `undo` / `redo` through the kernel, re-placing the document's rows from `world_xforms`, re-parking the origin.
12. **The numeric entry** — the DOM element, `State::gizmo_value`, the status line on a parse failure.
13. **The test surfaces** — `inspection::publish` gains `"gizmo"` (origin, hovered and engaged handle), so a browser check can assert a drag; a headless `src/selftest` case renders a frame with the widget up, and `selftest/lifecycle.rs` must survive the new retained local-box vector.

## What we take from the old viewer and what we do not

**Ports as it stands**

- `HandleKind` and its `labels`, with this viewer's world unit for "mm".
- The absolute-delta discipline — a rule, not code.
- The two-mode gesture, on `CLICK_SLOP` (`input.rs`): the archive's `GUMBALL_DRAG_THRESHOLD_SQ = 16.0` is the same 4 px, and a second constant must not appear.
- `project_ray_on_axis` and the ray-plane intersection, for the **drag** only.
- Freezing the uniform-scale drag plane at press.
- The reason for the scale floor: no singular or mirroring matrix from a zero.

**Adapts**

- The one-length factoring (`SCREEN_PX / ARC_RADIUS`); `Camera::unit` and `distance_world` replace the archive's two hard-coded `VIEWER_TO_MM = 1000.0`.
- The screen-constant formula, keeping the depth term and the widget's own depth in perspective. Viewport height is the surface height (`state.rs:96`) — no panels here, so no visible-rect to chase.
- The drag laws, with three corrections: rotation unwraps instead of capping at ±180°, the damping exponents go, the scale floor moves onto the factor.
- The numeric popup — title from `labels`, at the press point, Enter applies, Escape cancels — as a DOM element, reporting a parse failure instead of cancelling silently.
- The purity of `gumball.rs`: `src/app/gizmo.rs` keeps the shape, unit-testable without a GPU.
- Anchoring: the archive's raw-millimetre identity instance becomes one **anchored** row — the original looks right near the origin and disintegrates far from it.

**Replaced**

- `undo_state.rs` and `state_undo.rs` in full — by `session_rust/src/history.rs`.
- `snapshots_after` and the dual-absolute snapshots — `XformOp` and `ReplaceOp` are absolute on both sides.
- `drag_geom_snapshots` and `drag_nurbs_snapshots` — the first serves only a commit that bakes coordinates; the second was never written to.
- The baking commit branch and its remove-then-re-add of the GPU representation — a transform is a placement and stays in `Session::xforms`.
- `commit_object_transform`'s five-way branch by geometry type — by one branch on the write.
- `GumballState` owning four wgpu buffers and their bind groups — by one lane in `Gpu` and one field on `State`.
- The dedicated gumball pass and its depth clear — by `DepthMode::Always` plus the CPU sort.
- `ray_vs_arrow`, `ray_vs_arc`, `ray_vs_sphere_hit` — by the screen-space test, which drops the arrow test's ray-parallel sign error and the arc test that picked a full circle.
- A single `best_dist` across four groups in three incompatible units — by one space: pixels.
- The multi-selection centroid and its ~120 lines of per-type representative points — by the selected row's box midpoint.
- Escape exiting the application, and no mid-drag cancel — by Escape cancelling the drag first and by `Input::cancel`.
- The duplicated `mat4_mul`, the dead `pipelines.gumball`, the dead `gumball_dragged` flag.

**Not taken, and why that is not a gap**

- Arrowhead cones: no cone lane here, and a thickened shaft end reads as an arrow using `CylinderSegment` alone.
- `transform_locked`: no such set exists, and inventing one before a row needs locking is speculative.
- Edit-mode control dragging (~2000 archive lines): out of scope for v1, and the hook exists — `SelectionMode::Controls { parent, selected }` (`src/app/selection.rs`), committing through `Op::Replace`.
- Snapping: the archive wired `snap.rs` into its draw tools only, never the gumball.
- Object-aligned and CPlane-aligned modes, and a relocate gesture: the archive is world-axis-only too.
- A `PickMode::Gizmo` arm: `PickMode` stays at four variants.

## What to check on screen

- **Step 8** — select an object: the widget sits at its box centre. Orbit, zoom to a metre and to a kilometre, Space to flip projection, pan far enough to re-anchor: same pixel size, no jitter. A streamed cloud and a sheet get the same widget at the same size.
- **Step 9** — drag all ten handles: hover colour changes only when the hovered handle changes, a translate carries the widget along, rotate and scale leave the pivot put, a rotate past half a turn keeps turning.
- **Step 9** — press a handle and release without moving: no object selected behind it; press empty space and selection works as before.
- **Step 9** — mid-drag, tab away or let the browser steal the pointer, and press Escape: the drag ends, the object returns to where it started, the selection survives.
- **Step 9** — drag a selected sheet: every segment moves together and the frame time does not change.
- **Step 10** — move, reload from the same source: the move is in the document. Move a file the manifest places twice: the other placement does not move. Move a streamed cloud: the status line names it a view placement.
- **Step 11** — move, undo, redo five times: the object lands in exactly the same two positions each time, and the widget follows.
- **Step 12** — type a number into a handle: it moves by exactly that. Type `abc`: the status line says so, nothing moves. Type `0` into a scale: no collapse.
- **Throughout** — with `?inspect=1`, `data-viewer-inspection` carries `gizmo` and the `model` of the moved row, and `pick_busy` is false while the cursor moves without a drag.
