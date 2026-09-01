# 83 Draw tools II — polyline, rectangle, box, and the ghost preview

> **Big picture.** *Phase 9.* Three more tools, but the real subject is the **ghost preview** — the
> rubber-band that follows the cursor between clicks. Its rule is absolute in every CAD app: the
> preview is *pure viewport* — it never enters the Session, never hashes, never saves, never undoes.
> A tool shows you a possible future; only `finish` makes it real. Get that boundary wrong and 50's
> save fires mid-draw and half-drawn objects leak into files.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="clicks accumulate committed points while the segment from the last point to the cursor is a transient ghost; enter or the final click turns only the committed points into a session object" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <circle cx="60" cy="80" r="4" fill="#6fb3ff"/><circle cx="170" cy="40" r="4" fill="#6fb3ff"/><circle cx="290" cy="70" r="4" fill="#6fb3ff"/>
  <line x1="60" y1="80" x2="170" y2="40" stroke="#6fb3ff" stroke-width="2"/><line x1="170" y1="40" x2="290" y2="70" stroke="#6fb3ff" stroke-width="2"/>
  <line x1="290" y1="70" x2="400" y2="95" stroke="#888" stroke-width="2" stroke-dasharray="5 4"/>
  <circle cx="400" cy="95" r="3" fill="none" stroke="#888"/><text x="412" y="99" fill="#888">cursor</text>
  <text x="180" y="110" fill="#6fb3ff">clicked points — the tool's state</text>
  <text x="345" y="60" fill="#888">ghost — preview table only</text>
  <g transform="translate(480,20)">
    <text x="0" y="20" fill="#d7dae0">Enter / last click:</text>
    <text x="0" y="40" fill="#666" font-size="10">ghost cleared,</text>
    <text x="0" y="55" fill="#666" font-size="10">Polyline::new(points)</text>
    <text x="0" y="70" fill="#666" font-size="10">→ AddGeometry → history</text>
    <text x="0" y="95" fill="#888" font-size="10">Session never saw the ghost</text>
  </g>
</svg>

## Files we touch

```
# preview table: set_preview(segments) / clear_preview — gumball's pattern
src/engine/gpu/mod.rs
src/app/getloop.rs         # ActiveCommand gains on_move (default no-op)
src/state.rs               # cursor moves feed on_move while a command runs; Enter routes as ""
src/app/tools/polyline.rs  # NEW — N clicks, Enter finishes
src/app/tools/rect.rs      # NEW — RectangleTool + BoxTool on z=0
src/app/commands.rs        # verbs: polyline / rect / box
```

## Step 1 — the preview table: `src/engine/gpu/mod.rs`

Identical mechanism to the gumball's overlay tables (65), one level simpler — no depth-clear pass
needed, ghosts draw in the main cylinder pass, just from their own small buffer. Five new `struct
Gpu` fields, below 65's `gb_*` block:

```rust
    // ghost preview table (71) — replaced whole on every tool mouse-move
    pub preview_buffer: wgpu::Buffer,
    pub preview_bind_group: wgpu::BindGroup,
    pub preview_count: u32,
    pub preview_row: u32,        // reserved identity row, the gumball's gb_row pattern
    preview_scratch: Vec<CylinderSegment>,  // staging for set_preview — reused, never realloc'd
```

In `Gpu::new`, find 65's reserved-row block (`let gb_row = objects_base.len() as u32;` …) → insert
after it:

```rust
        let preview_row = objects_base.len() as u32;
        objects_base.push((Xform::identity(), [1.0, 1.0, 1.0, 1.0], 0));
```

and find 65's `gb_glyph_bind_group` creation → insert after it (same fixed-capacity trick):

```rust
        // module-level const (set_preview enforces it too):
        //   const PREVIEW_MAX_SEGMENTS: usize = 4096;
        let preview_buffer = storage_buffer(&device, "preview.segments",
            &vec![CylinderSegment::zeroed(); PREVIEW_MAX_SEGMENTS]);
        let preview_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("preview.bind_group"),
            layout: &segment_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0,
                resource: preview_buffer.as_entire_binding() }],
        });
```

(All five new fields go into the `Ok(Self { … })` initializer — `preview_count: 0`,
`preview_scratch: Vec::new()`, plus the three locals.) The upload pair mirrors 65's, with one extra
move: `set_preview` **stamps every segment onto the reserved row**, so tools never need to know
which row it is:

```rust
    /// Replace the ghost. Called every mouse-move while a tool previews — a few hundred bytes,
    /// and NO allocation: the scratch Vec's capacity is reused move after move. This matters
    /// beyond the usual GC-fodder argument — wasm linear memory grows but never shrinks, so
    /// every per-mouse-move Vec you allocate raises the process's high-water mark permanently.
    pub fn set_preview(&mut self, segs: &[CylinderSegment]) {
        if segs.len() > PREVIEW_MAX_SEGMENTS {
            log::warn!("set_preview: {} segments, truncated to {PREVIEW_MAX_SEGMENTS}", segs.len());
        }
        let row = self.preview_row;
        self.preview_scratch.clear();
        self.preview_scratch.extend(segs.iter().take(PREVIEW_MAX_SEGMENTS).map(|s| {
            let mut s = *s; s.instance_id = row; s
        }));
        self.queue.write_buffer(&self.preview_buffer, 0,
            bytemuck::cast_slice(&self.preview_scratch));
        self.preview_count = self.preview_scratch.len() as u32;
    }

    pub fn clear_preview(&mut self) { self.preview_count = 0; }
```

Note the `take(PREVIEW_MAX_SEGMENTS)` now *says so* when it fires — a silent truncation would
otherwise show up as a polyline ghost that mysteriously stops growing past 4096 points.

Draw the ghosts right after the main cylinder block in `clear()` — find its closing brace (after
`draws += 1;` of the `if self.segment_count > 0` block) → insert:

```rust
            // ghost preview (71) — same pipeline, its own table at group 3. Fully self-bound:
            // on an EMPTY scene (drawing the very first object) the block above never ran.
            if self.preview_count > 0 {
                pass.set_pipeline(&self.pipelines.cylinder);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.preview_bind_group, &[]);
                pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.preview_count);
                draws += 1;
            }
```

(The reserved identity row means a ghost's raw world points draw untransformed, translucent gray:
visually unmistakable as "not real yet". `CylinderSegment` has been `pub` since 35 — the tools in
Step 3/4 just import it.)

## Step 2 — tools learn about motion: `src/app/getloop.rs` + `src/state.rs`

```rust
pub trait ActiveCommand {
    // …feed_point / feed_text / options / back…
    /// Cursor moved while this command runs (world point under the cursor, snap already applied
    /// once 64 lands). Default: tools without previews ignore it.
    fn on_move(&mut self, _state: &mut crate::state::State, _p: Point) {}      // ← ADD
}
```

In `state.rs`, the point resolver becomes a named method — 61's click reroute computed
"pick-or-z=0" inline; motion needs the same answer every mouse-move, so factor it now. Add to
`impl State` (67's `cursor_ray` sits right above it):

```rust
    /// THE point resolver: the world point under the cursor — scene hit if any, else ray ∩ z=0
    /// (46 Step 3's formula). 61's clicks and this lesson's on_move both call it; 64 adds snap,
    /// 80 swaps z=0 for the work plane — one function, every tool inherits.
    fn cursor_world_point(&mut self) -> Option<Point> {
        let ray = self.cursor_ray()?;
        // 57's pick-site tolerance trio, verbatim
        let unit    = self.camera.unit.to_meters();
        let proj_y  = 1.0 / (30.0_f64).to_radians().tan() * unit;
        let ortho_h = if self.camera.perspective { 0.0 }
                      else { 2.0 * self.camera.distance * (30.0_f64).to_radians().tan() * unit };
        let vp_h    = self.gpu.config.height as f64;
        let tol = self.camera.world_per_pixel(self.camera.distance, proj_y, ortho_h, vp_h) * 8.0;
        if let Some(hit) = self.scene.pick_ray(&ray, tol) {
            return Some(hit.point);
        }
        // empty space: intersect the ground plane z = 0
        if ray.dir[2].abs() < 1e-12 { return None; }
        let t = -ray.origin[2] / ray.dir[2];
        if t < 0.0 { return None; }
        Some(Point::new(ray.origin[0] + t * ray.dir[0], ray.origin[1] + t * ray.dir[1], 0.0))
    }
```

(61's `WaitingPoint` click branch now reads `if let Some(p) = self.cursor_world_point()` and feeds
`p` — replace its inline `pick_ray` + z=0 code with the call, one behavior, one place.) Then the
cursor-move handler (after the gumball hover check) forwards motion to the active command:

```rust
        if self.active.is_some() {
            // the 53 click resolver, factored
            if let Some(p) = self.cursor_world_point() {
                if let Some(mut cmd) = self.active.take() {
                    cmd.on_move(self, p);
                    self.active = Some(cmd);
                }
            }
        }
```

And one routing addition: **Enter on an empty CLI while a command runs** already reaches
`feed_text(state, "")` (61) — polyline uses that as "finish".

## Step 3 — PolylineTool: `src/app/tools/polyline.rs`

Add at the top of the file — the shared ghost helper `ghost_segment` (used here and in `rect.rs`),
plus the `CylinderSegment` import it returns. The row id is a placeholder: `set_preview` (Step 1)
stamps every uploaded segment onto the reserved identity row, so tools never know or care which row
that is:

```rust
use session_rust::{Geometry, Point, Polyline};
use crate::app::getloop::{ActiveCommand, CmdStep, GetState};
use crate::engine::gpu::CylinderSegment;

/// A translucent-gray, screen-constant segment — "not real yet".
pub fn ghost_segment(a: &Point, b: &Point) -> CylinderSegment {
    CylinderSegment {
        p0: a.to_f32(),
        radius: 0.0,                 // screen-constant px, like every edge
        p1: b.to_f32(),
        instance_id: 0,              // stamped to the preview row by set_preview (Step 1)
        color: [0.6, 0.6, 0.6, 0.5], // translucent gray
    }
}

pub struct PolylineTool {
    points: Vec<Point>,
    scratch: Vec<CylinderSegment>,   // ghost staging — reused every mouse-move, never realloc'd
}

impl PolylineTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(PolylineTool { points: Vec::new(), scratch: Vec::new() }),
         GetState::WaitingPoint { prompt: "polyline: pick point (Enter finishes)".into() })
    }
    fn ask(&self) -> CmdStep {
        CmdStep::Prompt(GetState::WaitingPoint {
            prompt: format!("polyline: pick point {} (Enter finishes)", self.points.len() + 1),
        })
    }
    fn ghost(&mut self, state: &mut crate::state::State, cursor: Option<&Point>) {
        self.scratch.clear();                                // capacity survives — no per-move alloc
        for w in self.points.windows(2) {
            self.scratch.push(ghost_segment(&w[0], &w[1]));  // gray CylinderSegment on the preview row
        }
        if let (Some(last), Some(c)) = (self.points.last(), cursor) {
            self.scratch.push(ghost_segment(last, c));       // the rubber tail
        }
        state.gpu.set_preview(&self.scratch);
    }
}

impl ActiveCommand for PolylineTool {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        self.points.push(p);
        self.ghost(state, None);
        self.ask()
    }
    fn on_move(&mut self, state: &mut crate::state::State, p: Point) {
        // committed points + rubber tail
        if !self.points.is_empty() { self.ghost(state, Some(&p)); }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        if s.trim().is_empty() {                                      // Enter = finish
            state.gpu.clear_preview();
            if self.points.len() < 2 { return CmdStep::Cancel; }
            let pl = Polyline::new(std::mem::take(&mut self.points));
            state.commit(Box::new(
                crate::app::history::add::AddGeometry::one(Geometry::Polyline(pl))));
            return CmdStep::Done("polyline added".into());
        }
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        state.ui.log = format!("polyline: expected 'x,y,z' numbers, got '{s}'");   // reject loudly
        self.ask()
    }
    fn back(&mut self) -> CmdStep { self.points.pop(); self.ask() }   // un-click the last point
    fn prompt(&self) -> GetState {                                    // 54 requires it
        GetState::WaitingPoint {
            prompt: format!("polyline: pick point {} (Enter finishes)", self.points.len() + 1),
        }
    }
}
```

(Esc must also clear the ghost — add `state.gpu.clear_preview()` to 61's cancel arm, once, centrally:
every future previewing tool is then leak-proof. Same on the path that *replaces* a running command:
typing `rect` mid-`polyline` installs a fresh ActiveCommand over the old one (61's dispatch `Start`),
and without a `clear_preview()` there the abandoned ghost sticks on screen forever.)

## Step 4 — RectangleTool + BoxTool: `src/app/tools/rect.rs`

Both are two-corner tools on the `z = 0` plane (the work plane arrives in 80; until then the ground
is the canvas). Rectangle emits a closed `Polyline`; Box runs the same two corners, then one more
prompt for the height — typed or clicked — and emits a placed `Mesh::create_box`. One file holds
both, sharing a corner-outline helper; add `pub mod rect;` beside `pub mod polyline;` in
`app/tools/mod.rs` (and `pub mod polyline;` itself, if Step 3 didn't already):

```rust
use session_rust::{Geometry, Mesh, Point, Polyline, Xform};
use crate::app::getloop::{ActiveCommand, CmdStep, GetState};
use crate::engine::gpu::CylinderSegment;
use super::polyline::ghost_segment;

/// The 4-segment rectangle outline between two corners, z = 0 — the shared ghost AND the shape.
fn rect_corners(a: &Point, b: &Point) -> [Point; 4] {
    let (x0, x1) = (a[0].min(b[0]), a[0].max(b[0]));
    let (y0, y1) = (a[1].min(b[1]), a[1].max(b[1]));
    [Point::new(x0, y0, 0.0), Point::new(x1, y0, 0.0),
     Point::new(x1, y1, 0.0), Point::new(x0, y1, 0.0)]
}

fn rect_ghost(a: &Point, b: &Point) -> [CylinderSegment; 4] {   // stack array — zero alloc per move
    let c = rect_corners(a, b);
    [0, 1, 2, 3].map(|i| ghost_segment(&c[i], &c[(i + 1) % 4]))
}

pub struct RectangleTool {
    from: Option<Point>,
}

impl RectangleTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(RectangleTool { from: None }),
         GetState::WaitingPoint { prompt: "rect: pick FIRST corner".into() })
    }
}

impl ActiveCommand for RectangleTool {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.from.take() {
            None => { self.from = Some(p); CmdStep::Prompt(self.prompt()) }
            Some(a) => {
                state.gpu.clear_preview();
                let c = rect_corners(&a, &p);
                let pl = Polyline::new(vec![
                    c[0].clone(), c[1].clone(), c[2].clone(), c[3].clone(),
                    c[0].clone(),                       // closed — last point repeats the first
                ]);
                state.commit(Box::new(
                    crate::app::history::add::AddGeometry::one(Geometry::Polyline(pl))));
                CmdStep::Done("rectangle added".into())
            }
        }
    }
    fn on_move(&mut self, state: &mut crate::state::State, p: Point) {
        if let Some(a) = &self.from { let g = rect_ghost(a, &p); state.gpu.set_preview(&g); }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        state.ui.log = format!("rect: expected 'x,y,z' numbers, got '{s}'");   // reject loudly
        CmdStep::Prompt(self.prompt())
    }
    fn back(&mut self) -> CmdStep { self.from = None; CmdStep::Prompt(self.prompt()) }
    fn prompt(&self) -> GetState {
        let what = if self.from.is_none() { "rect: pick FIRST corner" }
                   else { "rect: pick OPPOSITE corner" };
        GetState::WaitingPoint { prompt: what.into() }
    }
}

pub struct BoxTool {
    corners: Vec<Point>,     // 0, 1, then the height prompt
}

impl BoxTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(BoxTool { corners: Vec::new() }),
         GetState::WaitingPoint { prompt: "box: pick FIRST corner".into() })
    }
    fn finish(&mut self, state: &mut crate::state::State, h: f64) -> CmdStep {
        state.gpu.clear_preview();
        if h.abs() < 1e-9 { return CmdStep::Cancel; }
        let (a, b) = (&self.corners[0], &self.corners[1]);
        let (x0, x1) = (a[0].min(b[0]), a[0].max(b[0]));
        let (y0, y1) = (a[1].min(b[1]), a[1].max(b[1]));
        let mut m = Mesh::create_box(x1 - x0, y1 - y0, h);
        // create_box is centered — lift by h/2 so it STANDS on the ground (33 established this)
        m.xform = Xform::translation((x0 + x1) * 0.5, (y0 + y1) * 0.5, h * 0.5);
        state.commit(Box::new(
            crate::app::history::add::AddGeometry::one(Geometry::Mesh(m))));
        CmdStep::Done("box added".into())
    }
}

impl ActiveCommand for BoxTool {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.corners.len() {
            0 | 1 => { self.corners.push(p); CmdStep::Prompt(self.prompt()) }
            // third "point" = a clicked height: its distance above the ground
            _ => { let h = p[2].abs(); self.finish(state, h) }
        }
    }
    fn on_move(&mut self, state: &mut crate::state::State, p: Point) {
        if self.corners.len() == 1 {
            let g = rect_ghost(&self.corners[0], &p);
            state.gpu.set_preview(&g);
        }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        if self.corners.len() == 2 {
            // 62's grammar: a typed value at a point prompt — here, the height
            if let Ok(h) = s.trim().parse::<f64>() { return self.finish(state, h); }
            state.ui.log = format!("box: expected a height number, got '{s}'");   // reject loudly
            return CmdStep::Prompt(self.prompt());
        }
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        state.ui.log = format!("box: expected 'x,y,z' numbers, got '{s}'");       // reject loudly
        CmdStep::Prompt(self.prompt())
    }
    fn back(&mut self) -> CmdStep { self.corners.pop(); CmdStep::Prompt(self.prompt()) }
    fn prompt(&self) -> GetState {
        let what = match self.corners.len() {
            0 => "box: pick FIRST corner",
            1 => "box: pick OPPOSITE corner",
            _ => "box: height (type a number, or click a point at that height)",
        };
        GetState::WaitingPoint { prompt: what.into() }
    }
}
```

Register the verbs — in `dispatch()`'s match (`app/commands.rs`), beside 70's `"point"`/`"line"`
arms:

```rust
        "polyline" => { let (cmd, get) = crate::app::tools::polyline::PolylineTool::start();
                        Dispatch::Start(cmd, get) }
        "rect"     => { let (cmd, get) = crate::app::tools::rect::RectangleTool::start();
                        Dispatch::Start(cmd, get) }
        "box"      => { let (cmd, get) = crate::app::tools::rect::BoxTool::start();
                        Dispatch::Start(cmd, get) }
```

plus `VERBS`: add `"polyline"`, `"rect"`, `"box"`; `ALIASES`: `("poly","polyline")`.

## Step 4b — `NurbsCurveTool`: the geometry block's parked tool

Lesson 52 shipped curves with no way to draw one — the tool trait, the command bus and
undo didn't exist yet. They do now, and the tool is Step 3's polyline with a different
finish — clicks are **control points** (the curve smooths them,
it doesn't pass through), Enter builds a degree-3 curve:

Copy `polyline.rs` to `curve.rs` (add `pub mod curve;` to `app/tools/mod.rs`), rename
`PolylineTool` → `NurbsCurveTool`, reword the prompts (`"curve: pick control point (Enter
finishes)"`), and make exactly two body changes. The Enter branch of `feed_text` — find
`if self.points.len() < 2 { return CmdStep::Cancel; }` and the two `Polyline` lines under it →
replace with:

```rust
            if self.points.len() < 4 { return CmdStep::Cancel; }      // degree 3 needs ≥ 4 CVs
            // open, cubic, from control points
            let nc = NurbsCurve::create(false, 3, &self.points);
            state.commit(Box::new(
                crate::app::history::add::AddGeometry::one(Geometry::NurbsCurve(nc))));
            return CmdStep::Done("curve added".into());
```

(and swap the file's `Polyline` import for `NurbsCurve`). Then the ghost — the preview shows the
real smoothed curve, not the control polygon (cheap: a handful of CVs, ~64 samples). Replace the
copied `ghost` helper's body with:

```rust
    fn ghost(&mut self, state: &mut crate::state::State, cursor: Option<&Point>) {
        let mut cvs: Vec<Point> = self.points.iter().cloned().collect();   // handful of CVs — OK
        if let Some(c) = cursor { cvs.push(c.clone()); }
        if cvs.len() < 2 { state.gpu.clear_preview(); return; }
        let deg = 3.min(cvs.len() - 1);                    // a cubic once there are enough CVs
        let tmp = NurbsCurve::create(false, deg, &cvs);    // tiny kernel object, built per move
        let (t0, t1) = tmp.domain();
        self.scratch.clear();                              // 71's reused staging — no per-move alloc
        let mut prev: Option<Point> = None;
        for i in 0..=64 {
            let p = tmp.point_at(t0 + (t1 - t0) * i as f64 / 64.0);
            if let Some(q) = &prev { self.scratch.push(super::polyline::ghost_segment(q, &p)); }
            prev = Some(p);
        }
        state.gpu.set_preview(&self.scratch);
    }
```

(The per-move `NurbsCurve::create` is deliberate: the CV count is single digits, so the temp curve
is microseconds — the *allocations* were the real churn, and those now ride the copied file's
`scratch` Vec Step 3 already reuses. If a profiler ever disagrees, cache `tmp` keyed on `(points.len(), cursor)`
— but measure first.)

Since curves are `Geometry` variants now, no bespoke command is needed at all — 70's `AddGeometry`
takes them directly, and 64's `restore_geometry` already grew the arm (`session` there is
`self.docs[d].session` — multi-doc since 35, never a singular `self.session`):

```rust
    // already in 64's restore_geometry — the kernel's add_nurbscurve handles
    // objects/lookup/graph/tree:
    Geometry::NurbsCurve(c) => { session.add_nurbscurve((**c).clone(), None); }

    // the tool's finish (Step 3 above) is therefore just:
    state.commit(Box::new(AddGeometry::one(Geometry::NurbsCurve(nc))));
```

(`Session::remove_object` — which `AddGeometry`'s undo runs through — already retains the nurbs
collections since the gap-#4 fix, so the round trip is complete with zero curve-specific plumbing.)

Register `"curve"` (+ aliases `"crv"`, `"nurbscurve"`) in the verb tables.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **`polyline`**, click 4 points → the gray ghost trails the cursor from the last click; **Enter** →
  the real polyline appears (colored, picks, saves), ghost gone. **Ctrl+Z** → the whole polyline
  vanishes as *one* action — one Command, N clicks.
- `back` mid-polyline removes the last clicked point *and* the ghost follows suit.
- **`rect`**, two clicks → closed rectangle on the ground. **`box`**, two clicks + type `400` ⏎ → a
  box of exactly that height standing on the ground plane (not buried half-under it — the
  `h * 0.5` lift; `create_box` builds centered on the origin, lesson 33 established that).
- The leak test: mid-polyline with 3 ghost points, hit **Save** (Ctrl+S if wired, or wait for 50's
  debounce) → the saved file contains *no trace* of the ghost. Esc → ghost gone, nothing committed.

## Recap

```
Ch 70: creation — tools are ActiveCommands, finish = AddGeometry.
Ch 71: N-CLICK TOOLS + THE GHOST. Preview = a dedicated small segment buffer (gumball's table
       pattern, drawn in the main cylinder pass, translucent gray on a reserved identity row) —
       PURE VIEWPORT: never in the Session, never hashed/saved/undone; Esc's cancel arm clears it
       centrally. ActiveCommand gains on_move (default no-op); State feeds it the same pick-or-z=0
       point as clicks. PolylineTool: points Vec + rubber tail, Enter (empty feed_text) finishes,
       back un-clicks; ghost builds go through a REUSED scratch Vec (per-move Vec allocs raise
       wasm's never-shrinking memory high-water mark). Bad typed input is rejected with a reason
       in the log, never eaten. Rectangle: two corners → CLOSED Polyline (last = first). Box: two
       corners + height prompt → Mesh::create_box + translation xform (centered → lift by h/2).
       One Command per finished object — N clicks undo as one.
```

Edited: `engine/gpu/mod.rs` (preview table + draw), `app/getloop.rs` (`on_move`), `state.rs` (motion
feed, central ghost clear on cancel), `app/tools/polyline.rs` + `app/tools/rect.rs` (NEW),
`app/commands.rs` (three verbs).

## Next

`84-snapping.md` — drawing becomes *precise*: endpoints, vertices, and grid intersections within a
few pixels of the cursor pull the point to themselves, with a marker glyph showing the live snap.
The Get-loop consults snap before every point it hands out — so every tool built so far becomes
snap-aware without changing a line of tool code.
