# 77 Gumball III — drag to translate

> **Big picture.** *Phase 9.* The first real transform. Three ideas carry every drag interaction from
> here to the end of the course: **deferred drag** (a press isn't a drag until the mouse travels — a
> click means something else, 69), **live = matrix-only** (during the drag, only instance matrices
> change; geometry is never re-tessellated — this is why dragging 5,000 objects holds 60 fps), and
> **commit = a Command** (release records before/after placements → undo is exact and free, 56).
> And since the Xform refactor the commit is *also* matrix-only: placement lives in
> `session.xforms`, so a move never touches coordinates, for ANY geometry kind. Rotate and scale
> (68) will be *only* new delta math on this identical skeleton.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="press stores state, movement past a threshold begins the drag, each mouse move applies a matrix-only delta live, release commits a TransformObjects command onto the undo stack" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="30" width="110" height="34"/><rect x="150" y="30" width="130" height="34"/><rect x="312" y="30" width="160" height="34"/><rect x="504" y="30" width="166" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="63" y="47">press handle</text><text x="63" y="59" fill="#666" font-size="9">snapshot placements</text>
    <text x="215" y="47">travel ≥ 4 px?</text><text x="215" y="59" fill="#666" font-size="9">else: click (69)</text>
    <text x="392" y="47">drag: delta · original</text><text x="392" y="59" fill="#666" font-size="9">matrix-only, LIVE</text>
    <text x="587" y="47">release: Command</text><text x="587" y="59" fill="#666" font-size="9">before/after xforms → undo</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="118" y1="47" x2="148" y2="47" marker-end="url(#ah54)"/><line x1="280" y1="47" x2="310" y2="47" marker-end="url(#ah54)"/><line x1="472" y1="47" x2="502" y2="47" marker-end="url(#ah54)"/></g>
  <text x="340" y="100" fill="#888" text-anchor="middle">the kernel objects are untouched forever — only session.xforms and the row's placed frame move</text>
  <defs><marker id="ah54" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## The one formula this lesson adds

The drag produces a **world-space** delta `D`. The object's full world placement is the row's
placed frame `W = place × ancestors × local` (52's `placed_frame` — the exact matrix `add_file`
stored). The Session stores only the LOCAL xform `L`, so committing `D` means solving
`W' = D · W` for the new local:

```
L' = L · W⁻¹ · D · W
```

— correct under any manifest `place` and any ancestor group transforms, because both live inside
`W`. (For an ungrouped sheet object `W = place · L` and the formula collapses to
`place⁻¹ · D · place · L` — the conjugation that makes a world translation land right on a placed
sheet, and a world ROTATION rotate about the right point. Skip it and every sheet but the first
transforms around the wrong origin.)

## Files we touch

```
src/engine/gumball.rs        # axis_drag_delta — closest-point-on-axis math (f64)
src/app/scene.rs             # apply_world_delta — THE commit primitive (60/61/85 reuse it)
src/app/history/transform.rs # NEW — TransformObjects: before/after placement snapshots
src/engine/gpu/mod.rs        # set_live_model / set_live_models — the live path (batched)
src/state.rs                 # the drag state machine: press → threshold → live → commit
```

## Step 1 — the axis delta: `src/engine/gumball.rs`

Dragging `TranslateX` means: wherever the mouse ray points, the motion is locked to the X axis. The
math is closest-approach between two lines — the pick ray and the axis line — solved for the axis
parameter:

```rust
/// Parameter t along the axis (a, u) closest to `ray`. The drag delta is
/// t_now − t_press — motion LOCKED to the axis regardless of where the ray actually points.
pub fn closest_param_on_axis(ray: &crate::engine::pick::Ray, a: &Point, u: &Vector) -> Option<f64> {
    let w = ray.origin.clone() - a.clone();                    // Point − Point → Vector
    let b = ray.dir.dot(u);                                    // cos between ray and axis
    let denom = 1.0 - b * b;
    if denom < 1e-9 { return None; }                           // axis ∥ view ray — no stable answer
    // sign matters: swap these and drags INVERT
    Some((w.dot(u) - b * w.dot(&ray.dir)) / denom)
}
```

(Unit vectors on both sides — `ray.dir` is normalized (54), `u` is a bare axis. The `None` arm
matters: dragging the Z arrow while looking straight down Z has no answer; the drag simply holds.
`gumball.rs` needs `Vector` here — grow 66's `use session_rust::Point;` to `{Point, Vector}`.)

> **Why press-referenced drags never accumulate error.** Every live frame computes its delta against
> the PRESS snapshot — `t0` here, `s.placed` in the move handler — never against the previous frame.
> Mouse jitter, f64 round-off, dropped frames: none of them compound, because nothing reads the
> result of the previous application. The release commits exactly one delta. (This is the property
> 68's rotate/scale inherit for free by stashing `a0`/`d0` the same way.)

## Step 2 — the commit primitive: `src/app/scene.rs`

One method, `pub` — 68's rotate/scale, 69's typed entry, and 92's copy-array all call it. No
per-kind match, no re-tessellation, no `Rc::make_mut`: a transform is a matrix write, twice (the
Session's local xform, and the row's placed frame — the tables are the truth `set_scene` re-derives
from, so a placement that only reached the GPU would be erased by the next streamed-in file):

```rust
    /// Apply a WORLD-space delta to one row's object. L' = L · W⁻¹ · D · W (see the lesson's
    /// formula) — placement-only, kind-agnostic, correct under manifest placement and groups.
    /// Also moves the row's cached world box, so the BVH refit stays cheap.
    pub fn apply_world_delta(&mut self, row: u32, delta: &Xform) {
        let guid = self.order[row as usize].clone();
        let w = self.tables.objects[row as usize].0.duplicate();
        let Some(winv) = w.inverse() else { return };
        let d = self.doc_of_row(row);
        let session = &mut self.docs[d].session;
        let local = session.xform(&guid);
        session.set_xform(&guid, &(&(&local * &winv) * delta) * &w);
        self.tables.objects[row as usize].0 = delta * &w;

        // move the cached box: transform its 8 corners, take min/max (52's extents cache)
        let (lo, hi) = self.world_boxes[row as usize];
        let (mut nlo, mut nhi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for cx in [lo[0], hi[0]] { for cy in [lo[1], hi[1]] { for cz in [lo[2], hi[2]] {
            let p = delta.transform_point(&Point::new(cx, cy, cz));
            for k in 0..3 { nlo[k] = nlo[k].min(p[k]); nhi[k] = nhi[k].max(p[k]); }
        } } }
        self.world_boxes[row as usize] = (nlo, nhi);
    }
```

> **Mid-drag queries: refit the TREE, don't rebuild it.** Moving `world_boxes[row]` keeps
> the extents cache honest, but 54's tree still holds the OLD node boxes — and its lazy
> `bvh_dirty` rebuild costs ~250 ms, a guaranteed hitch if hover/snap queries mid-drag.
> The O(depth × dragged) answer, through the same `pub` nodes 54's shapecast walks: union
> the dragged rows' old+new boxes into one box, then recurse — a node whose box misses the
> union is untouched (prune); a touched node recomputes its `aabb` from its children on
> the way back up. Drags move few rows; the walk touches their root paths and nothing
> else. Quality degrades after a LONG drag (boxes only ever grow) — which is exactly why
> the commit line above still rebuilds from truth.

> **The corner-refit inflates under rotation.** Transforming an AABB's 8 corners by a *rotation* and
> re-boxing gives the box of the rotated **box**, not of the rotated geometry — a 45° spin of a flat
> panel grows its bounds by ~√2, and repeated rotates inflate monotonically (culling and picking get
> looser, never wrong). Translation — this lesson — is exact. 68's rotate accepts the conservatism;
> a `rebuild_bvh` (52) refits from truth, and if the drift ever shows, refit from the row's LOCAL
> box through the new placed frame instead of corner-transforming the old world box.

## Step 3 — the Command: `src/app/history/transform.rs` (NEW)

Snapshots are `(row, guid, local xform, placed frame)` — a few hundred bytes per object instead of
cloned geometry, and exact by definition: a transform IS its placement. Apply/revert = write both
matrices back:

```rust
use session_rust::Xform;
use super::Command;
use crate::{app::scene::Scene, engine::gpu::Gpu};

pub struct XformSnap {
    pub row: u32,
    pub guid: String,
    pub local: Xform,    // the Session-local xform
    pub placed: Xform,   // the row's placed frame (tables.objects[row].0)
}

pub struct TransformObjects {
    pub before: Vec<XformSnap>,   // taken at PRESS
    pub after: Vec<XformSnap>,    // taken at RELEASE (post-commit)
}

impl TransformObjects {
    fn restore(set: &[XformSnap], scene: &mut Scene, gpu: &mut Gpu) {
        for s in set {
            let d = scene.doc_of_row(s.row);
            scene.docs[d].session.set_xform(&s.guid, s.local.duplicate());
            scene.tables.objects[s.row as usize].0 = s.placed.duplicate();
            gpu.set_live_model(s.row, &s.placed);
        }
        scene.rebuild_bvh();   // boxes moved (54) — full rebuild ON COMMIT only; see the
                               // refit note below for what happens DURING the drag
    }
}

impl Command for TransformObjects {
    fn apply(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        Self::restore(&self.after, scene, gpu);
    }
    fn revert(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        Self::restore(&self.before, scene, gpu);
    }
    fn label(&self) -> String { format!("move {} object(s)", self.before.len()) }
}
```

(Restore is idempotent, so the commit path *is* `history.execute(cmd, …)` — its first `apply`
writes the matrices that are already there, no special casing. Note what this Command does NOT
carry: geometry. After the Xform refactor a moved object's bytes never change; anything that
fingerprints geometry — 49's reconcile, 50's save-if-changed — must fingerprint
`session.xform(guid)` alongside it, which their rewrites do.)

## Step 4 — the drag machine: `src/state.rs`

Four small handlers around the fields 58 already added. First the context struct — add it at file
scope in `state.rs` (below the `use` lines), plus the field: `gb_drag: Option<DragCtx>` on
`struct State`, initialized `gb_drag: None` in `State::new` (like 66's fields). New imports for
this lesson: extend `state.rs`'s `session_rust` use to cover `{Point, Vector, Xform}`, add
`use crate::app::history::transform::{TransformObjects, XformSnap};` — and `app/history/mod.rs`
(64) gains `pub mod transform;` beside `pub mod remove;`:

One field this lesson *reuses*: `lmb_down: bool` — 52 added it for the marquee. What matters here
is WHERE it's set: the **raw** winit button state, *before* egui routing (60's snippet sits above
the `let resp = state.shell.state.on_window_event(…)` line), because egui may consume the release
(69's popup) — and a stale press would start a "no-button drag" the next time the cursor crosses
the gumball.

One scratch field this lesson adds: `gb_edits: Vec<(u32, Xform)>` on `struct State` (init
`Vec::new()`) — the per-frame drag stages (row, delta·placed) pairs into it and hands it to the
batched upload below; holding it across frames means no per-move allocation.

```rust
struct DragCtx {
    handle: HandleKind,
    origin: Point,                       // gumball anchor at press
    axis: Vector,                        // world axis for this handle
    t0: f64,                             // axis param at press
    before: Vec<XformSnap>,              // placement snapshots (row/guid/local/placed)
    last_delta: Xform,                   // begin_drag seeds identity; each move overwrites; release commits
}
```

**Two small helpers first** — both in `impl State`. `cursor_ray` is 54's unproject factored into a
method (the drag needs a ray on every mouse move); `begin_drag` fills the `DragCtx`:

```rust
    /// 54's screen_to_world_ray with its three locals, factored (55's click site builds the same).
    fn cursor_ray(&self) -> Option<crate::engine::pick::Ray> {
        let vp = self.camera.view_proj(self.aspect());
        let origin = self.camera.origin();
        let viewport = (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64);
        crate::engine::pick::screen_to_world_ray(&vp, &origin, self.cursor, viewport)
    }

    /// Press crossed the threshold: snapshot the selection's placements + stash the axis frame.
    fn begin_drag(&mut self, handle: HandleKind) {
        let Some(o) = self.scene.selection_centroid() else { return };
        let origin = Point::new(o[0], o[1], o[2]);      // 65's centroid is f64 — no casts
        use HandleKind::*;
        let axis = match handle {
            TranslateX | RotateX | ScaleX => Vector::x_axis(),
            TranslateY | RotateY | ScaleY => Vector::y_axis(),
            _                             => Vector::z_axis(),   // Z handles + ScaleUniform (68)
        };
        let Some(ray) = self.cursor_ray() else { return };
        let t0 = crate::engine::gumball::closest_param_on_axis(&ray, &origin, &axis)
            .unwrap_or(0.0);
        let mut before = Vec::new();
        for &row in &self.scene.selected {              // 58's selection is row-keyed
            let guid = self.scene.order[row as usize].clone();
            let d = self.scene.doc_of_row(row);
            before.push(XformSnap {
                row,
                guid: guid.clone(),
                local: self.scene.docs[d].session.xform(&guid),
                placed: self.scene.placed_frame(row).duplicate(),
            });
        }
        self.gb_drag = Some(DragCtx { handle, origin, axis, t0, before,
                                      last_delta: Xform::identity() });
    }
```

**Mouse move** — begin past the threshold, then live-update (in the cursor-move handler, after 66's
hover block):

```rust
        if let Some((handle, press_at)) = self.gb_pressed {
            let d2 = (self.cursor.0 - press_at.0).powi(2) + (self.cursor.1 - press_at.1).powi(2);
            // 4 px — the deferred-drag gate
            if self.gb_drag.is_none() && d2 >= 16.0 && self.lmb_down {
                // fills DragCtx: snapshots + t0
                self.begin_drag(handle);
            }
        }
        let mut live_delta = None;
        if let Some(ctx) = &self.gb_drag {
            if let Some(t) = self.cursor_ray().and_then(|ray|
                crate::engine::gumball::closest_param_on_axis(&ray, &ctx.origin, &ctx.axis)) {
                let dt = t - ctx.t0;
                // epsilon: sub-micron motion is not a drag — suppress the upload churn AND the
                // no-op Command it would otherwise commit on release
                if dt.abs() > 1e-6 {
                    let delta = Xform::translation(ctx.axis[0]*dt, ctx.axis[1]*dt, ctx.axis[2]*dt);
                    // matrix-only — and BATCHED: stage all rows, one write_buffer per frame
                    // (per-row uploads were thousands of tiny GPU commands per mouse move)
                    self.gb_edits.clear();
                    self.gb_edits.extend(ctx.before.iter()
                        .map(|s| (s.row, &delta * &s.placed)));
                    self.gpu.set_live_models(&self.gb_edits);
                    live_delta = Some(delta);
                }
            }
        }
        // stash the final delta on the ctx so release can commit it (can't touch
        // ctx above — it's borrowed immutably to read origin/axis/before), and
        // move the widget along with the drag:
        if let Some(delta) = live_delta {
            if let Some(ctx) = self.gb_drag.as_mut() { ctx.last_delta = delta.duplicate(); }
            self.refresh_gumball_at(&delta);                          // the widget rides along
        }
```

**Release** — in `on_left_release`, directly ABOVE 66's `gb_pressed.take()` gate (which clears
`gb_pressed` and returns, so this block does neither): write the placements, commit the Command
(or don't — a drag that never beat the
epsilon left `last_delta` at identity, and a no-op must not land on the undo stack):

```rust
        if let Some(ctx) = self.gb_drag.take() {
            let delta = ctx.last_delta.duplicate();   // stashed by the last mouse move
            let ident = Xform::identity();
            let no_op = (0..16).all(|i| (delta.m[i] - ident.m[i]).abs() < 1e-12);
            if no_op {
                // nothing to commit — but snap the live rows back anyway: 68's rotate/scale
                // arms have no epsilon gate, so sub-epsilon moves CAN have poked them
                for s in &ctx.before { self.gpu.set_live_model(s.row, &s.placed); }
            } else {
                for s in &ctx.before {
                    // L' = L · W⁻¹ · D · W — Session xform + tables row + cached box, in one call
                    self.scene.apply_world_delta(s.row, &delta);
                }
                let after = ctx.before.iter().map(|s| {
                    let d = self.scene.doc_of_row(s.row);
                    XformSnap {
                        row: s.row,
                        guid: s.guid.clone(),
                        local: self.scene.docs[d].session.xform(&s.guid),
                        placed: self.scene.placed_frame(s.row).duplicate(),
                    }
                }).collect();
                let cmd = Box::new(TransformObjects { before: ctx.before, after });
                // applies `after` — idempotent — and TransformObjects::restore already runs
                // rebuild_bvh (54), so NO explicit rebuild here: that was a second full rebuild
                self.history.execute(cmd, &mut self.scene, &mut self.gpu);
            }
        }
        // no trailing `gb_pressed = None` here — 66's gate just below takes it and returns
```

**Esc** — cancel mid-drag. In the key handler, insert this arm **above** 61's Escape arm (match
order is the guard, same trick 61 will use):

```rust
        Key::Named(NamedKey::Escape) if self.gb_drag.is_some() => {
            if let Some(ctx) = self.gb_drag.take() {
                for s in &ctx.before {
                    self.gpu.set_live_model(s.row, &s.placed);   // snap back — nothing ever mutated
                }
            }
            self.gb_pressed = None;
            self.refresh_gumball();
        }
```

The live-path helpers. In `engine/gpu/mod.rs`, `impl Gpu` (`Instance` and its buffer are private to
the engine, so the writes live here). Two entry points: `set_live_model` mirrors
`rebuild_instances`' rebase for ONE row (the Esc snap-back and undo's per-row restore); the
per-frame drag path is **batched** — `set_live_models` stages every dragged row, then issues ONE
`write_buffer` over the dirty span. Per-row uploads during a drag were thousands of 96-byte GPU
commands per mouse-move frame; the stress gate below fails on that shape. Note both write ONLY
`.model` — never rebuild the `Instance` literal (color/flags/extent/spacing must survive):

```rust
    /// The LIVE path, single row (Esc snap-back, undo restore). Rebases against the current
    /// anchor exactly like rebuild_instances, uploads just that row. Does NOT touch the arena,
    /// the Session, or the Scene tables — 67's release (or Esc) is what makes it durable (or
    /// erases it).
    pub fn set_live_model(&mut self, row: u32, model: &Xform) {
        self.set_live_models(&[(row, model.duplicate())]);
    }

    /// The LIVE path, batched: rebase + stage every dragged row, then ONE write_buffer over the
    /// [min, max] row span (rows are scattered, but the span is one upload — one GPU command per
    /// frame, not one per row per move). The origin is read ONCE, not cloned per row.
    pub fn set_live_models(&mut self, edits: &[(u32, Xform)]) {
        if edits.is_empty() { return; }
        let origin = self.last_origin.clone().unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
        let (mut lo, mut hi) = (u32::MAX, 0u32);
        for (row, model) in edits {
            let i = *row as usize;
            self.objects_base[i].0 = model.duplicate();
            let mut m = model.to_f32();
            m[12] = (model.m[12] - origin[0]) as f32;
            m[13] = (model.m[13] - origin[1]) as f32;
            m[14] = (model.m[14] - origin[2]) as f32;
            self.instances[i].model = m;    // ONLY the model — color/flags/extent/spacing stay
            lo = lo.min(*row); hi = hi.max(*row);
        }
        let sz = std::mem::size_of::<Instance>() as u64;
        self.queue.write_buffer(&self.instance_buffer, lo as u64 * sz,
            bytemuck::cast_slice(&self.instances[lo as usize..=hi as usize]));
    }
```

And in `impl State` (`state.rs`) — the widget riding along mid-drag: move its row model to the
delta-transformed press origin and rebuild the LOCAL geometry (`refresh_gumball` itself would read
the *unmoved* selection boxes):

```rust
    /// Rebuild the gumball at the drag's live position (row model = delta · press origin).
    fn refresh_gumball_at(&mut self, delta: &Xform) {
        let Some(ctx) = &self.gb_drag else { return };
        let o = delta.transform_point(&ctx.origin);
        self.gpu.place_gumball(&o);                       // 57: the model carries the position
        let s = self.gumball_scale([o[0], o[1], o[2]]);
        self.gb_scale = s;
        let g = crate::engine::gumball::build([0.0, 0.0, 0.0], s,
                                              self.gpu.gb_row, self.gb_hovered);
        self.gpu.upload_gumball(&g);
        self.gb = Some(g);
    }
```

> **Why the Session stays clean mid-drag.** The live path writes only `objects_base` + the dragged
> instance rows (staged, one upload per frame) — Esc restores the stashed placed frames and erases
> all evidence. The durable state moves
> exactly once, at release, through `apply_world_delta` (Session xform + Scene tables + cached box)
> wrapped in a Command. And because the TABLES carry the committed placement, a manifest file
> streaming in mid-session (`Msg::File` → `set_scene`) reproduces it — only an *uncommitted* live
> drag would visually snap back to its press position while the drag continues, which is the
> honest, recoverable behavior.

## Step 5 — verify (including the stress gate)

```bash
cd session_viewer && trunk serve   # http://127.0.0.1:8770
```

- Drag the X arrow → the selection slides **only along X**, wherever the mouse wanders. The motion is
  smooth at any object count — watch the perf HUD: no upload spikes, because nothing re-tessellates.
- Release, **Ctrl+Z** → objects return *exactly* (compare via `probe` — placements, not coordinates,
  moved). **Ctrl+Y** → re-applied. Esc mid-drag → snaps back to the press position, no Command
  recorded.
- Drag something on a PLACED sheet (any doc after the first): it moves under the cursor, not offset
  by the manifest translation — that's the `W⁻¹ · D · W` conjugation earning its keep.
- **STRESS GATE** — marquee a few thousand entities of a PDF sheet, drag: live motion holds the
  frame rate (matrix-only, the whole point), release commits in one beat (matrix writes, no
  re-tessellation, no re-upload of the arena), Ctrl+Z restores all of it.

## Recap

```
Ch 66: constant size + hit-test — the widget is grabbable.
Ch 67: TRANSLATE. Deferred drag: press → 4px travel + lmb gate (lmb_down is 60's RAW flag, set
       above egui routing) → begin (a clean click stays free for
       69's numeric entry). Delta = closest_param_on_axis(ray, origin, axis) minus its press value —
       two-line closest approach, None when axis ∥ view; an epsilon (|dt| > 1e-6) suppresses
       micro-motion, and a release with last_delta == identity commits NOTHING (no no-op Commands).
       Everything is press-referenced (t0, s.placed), so error never accumulates across frames.
       LIVE = set_live_models: rows staged in gb_edits, ONE write_buffer over the dirty span per
       frame (arena/Session/tables untouched — Esc leaves no trace; the single-row set_live_model
       remains for Esc/undo). COMMIT =
       Scene::apply_world_delta per row: L' = L·W⁻¹·D·W into session.set_xform + the row's placed
       frame in tables (survives set_scene) + the cached box — kind-agnostic, placement-only,
       nothing bakes. (Corner-refit inflates under ROTATION — 60 accepts conservative boxes;
       translate is exact.) TransformObjects snapshots (row, guid, local, placed) pairs — a
       transform IS
       its placement — through history.execute: idempotent apply, exact undo, and restore() already
       runs the BVH rebuild (no second explicit one). Stress
       gate: thousands drag at full fps because a drag is matrices, never geometry — and one
       upload per frame, never one per row per move.
```

Edited: `engine/gumball.rs` (`closest_param_on_axis`), `app/scene.rs` (`apply_world_delta`),
`app/history/transform.rs` (NEW — `XformSnap`, `TransformObjects`), `engine/gpu/mod.rs`
(`set_live_model` + the batched `set_live_models`), `state.rs` (drag machine: press/threshold/
live/commit, Esc cancel, `gb_edits` scratch).

## Next

`78-gumball-rotate-scale.md` — the same skeleton, two new deltas: arcs → ray-plane intersection +
`atan2` angle about the axis; spheres → distance ratios with the archive's two damping fixes (the
sign-preserving clamp and the fourth-root response) that keep scaling controllable. Both commit
through the same `apply_world_delta` — one formula, three transforms.
