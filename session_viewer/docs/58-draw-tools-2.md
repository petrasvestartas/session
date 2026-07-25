# 58 Draw tools II — polyline, rectangle, box, and the ghost preview

> **Big picture.** *Phase 9.* Three more tools, but the real subject is the **ghost preview** — the
> rubber-band that follows the cursor between clicks. Its rule is absolute in every CAD app: the
> preview is *pure viewport* — it never enters the Session, never hashes, never saves, never undoes.
> A tool shows you a possible future; only `finish` makes it real. Get that boundary wrong and 39's
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

Identical mechanism to the gumball's overlay tables (52), one level simpler — no depth-clear pass
needed, ghosts draw in the main cylinder pass, just from their own small buffer:

```rust
    // fields (mirror gb_segments): preview_segments buffer + count, fixed small capacity
    /// Replace the ghost. Called every mouse-move while a tool previews — a few hundred bytes.
    pub fn set_preview(&mut self, segs: &[CylinderSegment]) {
        /* write buffer + count, grow like 38a */ }
    pub fn clear_preview(&mut self) { self.preview_count = 0; }
```

Draw them right after the main cylinder draw, same pipeline, `preview_bind_group` at group 3. Ghost
rows use a dedicated reserved row — export it as `pub const PREVIEW_ROW: u32` (the analog of `GB_ROW`,
52) with identity model and a translucent gray, so a segment's raw world points draw untransformed and
visually unmistakable as "not real yet". Make `CylinderSegment` and `PREVIEW_ROW` `pub` here — the
tools in Step 3/4 build ghost segments and import both.

## Step 2 — tools learn about motion: `src/app/getloop.rs` + `src/state.rs`

```rust
pub trait ActiveCommand {
    // …feed_point / feed_text / options / back…
    /// Cursor moved while this command runs (world point under the cursor, snap already applied
    /// once 59 lands). Default: tools without previews ignore it.
    fn on_move(&mut self, _state: &mut crate::state::State, _p: Point) {}      // ← ADD
}
```

In `state.rs`, the cursor-move handler (after the gumball hover check): if a command is active,
compute the world point exactly like a Get-loop click (pick-or-z=0, 48) and forward it:

```rust
        if self.active.is_some() {
            // the 48 click resolver, factored
            if let Some(p) = self.cursor_world_point() {
                if let Some(mut cmd) = self.active.take() {
                    cmd.on_move(self, p);
                    self.active = Some(cmd);
                }
            }
        }
```

And one routing addition: **Enter on an empty CLI while a command runs** already reaches
`feed_text(state, "")` (48) — polyline uses that as "finish".

## Step 3 — PolylineTool: `src/app/tools/polyline.rs`

Add at the top of the file — the shared ghost helper `ghost_segment` (used here and in `rect.rs`),
plus the `CylinderSegment` import it returns. `PREVIEW_ROW` is the reserved identity-model row Step 1
set aside in `gpu`, so the segment's raw world points draw untransformed:

```rust
use session_rust::{Geometry, Point, Polyline};
use crate::app::getloop::{ActiveCommand, CmdStep, GetState};
use crate::engine::gpu::{CylinderSegment, PREVIEW_ROW};

/// A translucent-gray, screen-constant segment on the preview row — "not real yet".
pub fn ghost_segment(a: &Point, b: &Point) -> CylinderSegment {
    CylinderSegment {
        p0: a.to_f32(),
        radius: 0.0,                 // screen-constant px, like every edge
        p1: b.to_f32(),
        instance_id: PREVIEW_ROW,    // Step 1's reserved identity row
        color: [0.6, 0.6, 0.6, 0.5], // translucent gray
    }
}

pub struct PolylineTool {
    points: Vec<Point>,
}

impl PolylineTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(PolylineTool { points: Vec::new() }),
         GetState::WaitingPoint { prompt: "polyline: pick point (Enter finishes)".into() })
    }
    fn ask(&self) -> CmdStep {
        CmdStep::Prompt(GetState::WaitingPoint {
            prompt: format!("polyline: pick point {} (Enter finishes)", self.points.len() + 1),
        })
    }
    fn ghost(&self, state: &mut crate::state::State, cursor: Option<&Point>) {
        let mut segs = Vec::new();
        let pts: Vec<&Point> = self.points.iter().chain(cursor).collect();
        for w in pts.windows(2) {
            segs.push(ghost_segment(w[0], w[1]));       // gray CylinderSegment on the preview row
        }
        state.gpu.set_preview(&segs);
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
        self.ask()
    }
    fn back(&mut self) -> CmdStep { self.points.pop(); self.ask() }   // un-click the last point
}
```

(Esc must also clear the ghost — add `state.gpu.clear_preview()` to 48's cancel arm, once, centrally:
every future previewing tool is then leak-proof.)

## Step 4 — RectangleTool + BoxTool: `src/app/tools/rect.rs`

Both are two-corner tools on the `z = 0` plane (the work plane arrives in 75; until then the ground
is the canvas). Rectangle emits a closed `Polyline`; Box runs the same two corners, then one more
prompt for the height — typed or clicked — and emits a placed `Mesh::create_box`:

```rust
    // rectangle finish (corners a, b — z forced to 0):
    let (x0, x1) = (a[0].min(b[0]), a[0].max(b[0]));
    let (y0, y1) = (a[1].min(b[1]), a[1].max(b[1]));
    let pl = Polyline::new(vec![
        Point::new(x0, y0, 0.0),
        Point::new(x1, y0, 0.0),
        Point::new(x1, y1, 0.0),
        Point::new(x0, y1, 0.0),
        Point::new(x0, y0, 0.0),   // closed — last point repeats the first
    ]);

    // box finish (same corners + height h from prompt 3):
    let mut m = Mesh::create_box(x1 - x0, y1 - y0, h);
    // create_box is centered
    m.xform = Xform::translation((x0 + x1) * 0.5, (y0 + y1) * 0.5, h * 0.5);
    // → AddGeometry::one(Geometry::Mesh(m))
```

Their `on_move` ghosts are the rectangle outline (4 ghost segments from corner a to the cursor) — the
same `ghost_segment` helper. Height prompt: `WaitingPoint` whose `feed_text` parses a single number —
this is where 49's "typed value at a point prompt" grammar quietly pays off again.

Register: `"polyline" | "rect" | "box"` verbs (+ `VERBS`, alias `("poly","polyline")`).

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
- The leak test: mid-polyline with 3 ghost points, hit **Save** (Ctrl+S if wired, or wait for 39's
  debounce) → the saved file contains *no trace* of the ghost. Esc → ghost gone, nothing committed.

## Recap

```
Ch 57: creation — tools are ActiveCommands, finish = AddGeometry.
Ch 58: N-CLICK TOOLS + THE GHOST. Preview = a dedicated small segment buffer (gumball's table
       pattern, drawn in the main cylinder pass, translucent gray on a reserved identity row) —
       PURE VIEWPORT: never in the Session, never hashed/saved/undone; Esc's cancel arm clears it
       centrally. ActiveCommand gains on_move (default no-op); State feeds it the same pick-or-z=0
       point as clicks. PolylineTool: points Vec + rubber tail, Enter (empty feed_text) finishes,
       back un-clicks. Rectangle: two corners → CLOSED Polyline (last = first). Box: two corners +
       height prompt → Mesh::create_box + translation xform (centered → lift by h/2). One Command
       per finished object — N clicks undo as one.
```

Edited: `engine/gpu/mod.rs` (preview table + draw), `app/getloop.rs` (`on_move`), `state.rs` (motion
feed, central ghost clear on cancel), `app/tools/polyline.rs` + `app/tools/rect.rs` (NEW),
`app/commands.rs` (three verbs).

## Next

`59-snapping.md` — drawing becomes *precise*: endpoints, vertices, and grid intersections within a
few pixels of the cursor pull the point to themselves, with a marker glyph showing the live snap.
The Get-loop consults snap before every point it hands out — so every tool built so far becomes
snap-aware without changing a line of tool code.
