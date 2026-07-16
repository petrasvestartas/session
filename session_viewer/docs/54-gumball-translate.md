# 54 Gumball III — drag to translate

> **Big picture.** *Phase 9.* The first real transform. Three ideas carry every drag interaction from
> here to the end of the course: **deferred drag** (a press isn't a drag until the mouse travels — a
> click means something else, 56), **live = matrix-only** (during the drag, only instance matrices
> change; geometry is never re-tessellated — this is why dragging 5,000 objects holds 60 fps), and
> **commit = a Command** (release produces absolute before/after snapshots → undo is exact and free,
> 51). Rotate and scale (55) will be *only* new delta math on this identical skeleton.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="press stores state, movement past a threshold begins the drag, each mouse move applies a matrix-only delta live, release commits a TransformObjects command onto the undo stack" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="30" width="110" height="34"/><rect x="150" y="30" width="130" height="34"/><rect x="312" y="30" width="160" height="34"/><rect x="504" y="30" width="166" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="63" y="47">press handle</text><text x="63" y="59" fill="#666" font-size="9">snapshot models</text>
    <text x="215" y="47">travel ≥ 4 px?</text><text x="215" y="59" fill="#666" font-size="9">else: click (56)</text>
    <text x="392" y="47">drag: delta · original</text><text x="392" y="59" fill="#666" font-size="9">matrix-only, LIVE</text>
    <text x="587" y="47">release: Command</text><text x="587" y="59" fill="#666" font-size="9">before/after → undo stack</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="118" y1="47" x2="148" y2="47" marker-end="url(#ah54)"/><line x1="280" y1="47" x2="310" y2="47" marker-end="url(#ah54)"/><line x1="472" y1="47" x2="502" y2="47" marker-end="url(#ah54)"/></g>
  <text x="340" y="100" fill="#888" text-anchor="middle">the kernel objects are untouched until release — the Session never sees a half-drag</text>
  <defs><marker id="ah54" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gumball.rs        # axis_drag_delta — closest-point-on-axis math (f64)
src/app/history/transform.rs # NEW — TransformObjects: absolute before/after snapshots
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

(Unit vectors on both sides — `ray.dir` is normalized (41), `u` is a bare axis. The `None` arm
matters: dragging the Z arrow while looking straight down Z has no answer; the drag simply holds.)

## Step 2 — the Command: `src/app/history/transform.rs` (NEW)

Same absolute-snapshot pattern as 51's delete — `before` cloned at press, `after` cloned at release.
Apply/revert = swap the whole object back in. One code path for mesh *and* thin geometry, because
`apply_object` (38b) already re-flattens whatever it's given:

```rust
use session_rust::Geometry;
use super::Command;
use crate::{app::scene::Scene, engine::gpu::Gpu};

pub struct TransformObjects {
    pub before: Vec<Geometry>,   // cloned at PRESS
    // cloned at RELEASE (kernel objects already carry the new placement)
    pub after: Vec<Geometry>,
}

impl TransformObjects {
    fn restore(set: &[Geometry], scene: &mut Scene, gpu: &mut Gpu) {
        for geom in set {
            let guid = geom.guid().to_string();
            // overwrite in place — same guid
            scene.session.lookup.insert(guid.clone(), geom.clone());
            scene.hashes.insert(guid.clone(), Scene::hash_of(geom));   // keep 38b/39's gates honest
            let row = scene.guid_to_row[&guid];
            scene.apply_object(gpu, &guid, geom, row);
        }
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

(`hash_of` is 38b's `content_hash` made `pub(crate)` on `Scene`. Overwriting `lookup` directly is
correct for a transform — guid, tree node, and graph edges all survive; only the object body changes.
One subtlety: `execute` (51) would apply `after` a second time on commit — harmless (idempotent
restore), so the commit path *is* `history.execute(cmd, …)`, no special casing.)

## Step 3 — the drag machine: `src/state.rs`

Four small handlers around the fields 53 already added:

```rust
    // fields: gb_pressed: Option<(HandleKind, (f64, f64))>   (53)
    //         gb_drag: Option<DragCtx>,  with
    struct DragCtx {
        handle: HandleKind,
        origin: Point,                       // gumball anchor at press
        axis: Vector,                        // world axis for this handle
        t0: f64,                             // axis param at press
        before: Vec<Geometry>,               // absolute snapshots (guid inside)
        base_models: Vec<(String, u32, Xform)>,  // (guid, row, model-at-press) for the live path
    }
```

**Mouse move** — begin past the threshold, then live-update:

```rust
        if let Some((handle, press_at)) = self.gb_pressed {
            let d2 = (self.cursor.0 - press_at.0).powi(2) + (self.cursor.1 - press_at.1).powi(2);
            // 4 px — the deferred-drag gate
            if self.gb_drag.is_none() && d2 >= 16.0 && self.lmb_down {
                // fills DragCtx: snapshots + t0
                self.begin_drag(handle);
            }
        }
        if let Some(ctx) = &self.gb_drag {
            // 41's screen_to_world_ray, factored
            let ray = self.cursor_ray();
            if let Some(t) = crate::engine::gumball::closest_param_on_axis(
                &ray, &ctx.origin, &ctx.axis) {
                let dt = t - ctx.t0;
                let delta = Xform::translation(ctx.axis[0]*dt, ctx.axis[1]*dt, ctx.axis[2]*dt);
                for (_, row, base) in &ctx.base_models {
                    // matrix-only — see the note
                    self.gpu.set_live_model(*row, &(&delta * base));
                }
                self.refresh_gumball_at(&delta);                          // the widget rides along
            }
        }
```

**Release** — bake into the kernel, commit the Command:

```rust
        if let Some(ctx) = self.gb_drag.take() {
            let delta = /* final delta from the last move, stashed on ctx */;
            for (guid, _, _) in &ctx.base_models {
                if let Some(geom) = self.scene.session.lookup.get_mut(guid) {
                    // compose xform / bake coords
                    apply_delta(geom, &delta);
                }
            }
            let after = ctx.before.iter()
                .map(|g| self.scene.session.lookup[g.guid()].clone()).collect();
            let cmd = Box::new(TransformObjects { before: ctx.before, after });
            // applies `after` — idempotent
            self.history.execute(cmd, &mut self.scene, &mut self.gpu);
            self.scene.rebuild_bvh();                                     // boxes moved (36)
        }
        self.gb_pressed = None;
```

Two helpers to pin down:

```rust
// engine/gpu/mod.rs — the LIVE path: instance model only. Writes objects_base so 33's per-frame
// rebase carries the delta; does NOT touch the arena, hashes, or the Session.
pub fn set_live_model(&mut self, row: u32, model: &Xform) {
    self.objects_base[row as usize].0 = model.duplicate();
    self.write_row(row, |_| {});      // rebased + re-uploaded by the normal frame path
}

// app/scene.rs — bake a delta into a kernel object at COMMIT time.
// Mesh/BRep: placement lives in xform → compose. Thin: coords are world → bake via transform().
fn apply_delta(geom: &mut Geometry, delta: &Xform) {
    match geom {
        Geometry::Mesh(m) => m.xform = delta * &m.xform,
        Geometry::BRep(b) => b.xform = delta * &b.xform,
        Geometry::Line(l)     => { l.xform = delta.duplicate(); l.transform(); }
        Geometry::Polyline(p) => { p.xform = delta.duplicate(); p.transform(); }
        Geometry::Point(p)    => { p.xform = delta.duplicate(); p.transform(); }
        _ => {}
    }
}
```

> **Why the Session stays clean mid-drag.** The live path writes only `objects_base` + instance rows —
> if the drag is Esc-cancelled, restoring the stashed `base_models` erases all evidence. The kernel
> objects mutate exactly once, at release, immediately wrapped in a Command. There is never a moment
> where a half-drag could be saved (39) or diffed (38b).

## Step 4 — verify (including the stress gate)

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Drag the X arrow → the selection slides **only along X**, wherever the mouse wanders. The motion is
  smooth at any object count — watch the perf HUD: no upload spikes, because nothing re-tessellates.
- Release, **Ctrl+Z** → objects return *exactly* (compare a coordinate via `probe`). **Ctrl+Y** →
  re-applied. Esc mid-drag → snaps back to the press position, no Command recorded.
- A polyline in the selection moves live with the meshes (segments read the instance model), and
  after release it stays put — the commit baked its coordinates (`apply_delta`) and re-flattened.
- **STRESS GATE** — marquee a few thousand entities of the PDF drawing, drag: live motion holds the
  frame rate (matrix-only, the whole point), release commits in one beat, Ctrl+Z restores all of it.

## Recap

```
Ch 53: constant size + hit-test — the widget is grabbable.
Ch 54: TRANSLATE. Deferred drag: press → 4px travel + lmb gate → begin (a clean click stays free for
       56's numeric entry). Delta = closest_param_on_axis(ray, origin, axis) minus its press value —
       two-line closest approach, None when axis ∥ view. LIVE = set_live_model: objects_base + row
       only (33's rebase carries it; arena/Session/hashes untouched — Esc leaves no trace). COMMIT =
       apply_delta into the kernel (Mesh/BRep compose xform; thin geometry bakes via transform() and
       re-flattens) + TransformObjects{before, after} absolute snapshots through history.execute —
       idempotent apply, exact undo. BVH rebuilt (boxes moved). Stress gate: thousands drag at full
       fps because the drag is matrices, never geometry.
```

Edited: `engine/gumball.rs` (`closest_param_on_axis`), `app/history/transform.rs` (NEW —
`TransformObjects`), `engine/gpu/mod.rs` (`set_live_model`), `app/scene.rs` (`apply_delta`,
`hash_of`), `state.rs` (drag machine: press/threshold/live/commit, Esc cancel).

## Next

`55-gumball-rotate-scale.md` — the same skeleton, two new deltas: arcs → ray-plane intersection +
`atan2` angle about the axis; spheres → distance ratios with the archive's two damping fixes (the
sign-preserving clamp and the fourth-root response) that keep scaling controllable.
