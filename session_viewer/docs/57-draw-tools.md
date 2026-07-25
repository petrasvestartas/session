# 57 Draw tools I — creating geometry

> **Big picture.** *Phase 9.* Everything so far edits what a file provided; now the viewer *creates*.
> Architecturally this is pattern (b), and it's deliberately anticlimactic: a drawing tool is **just
> a multi-step command** (49's `ActiveCommand` — prompts, clicks, Esc) whose final act is an
> `AddGeometry` Command (51 — so creation is undoable for free). No new machinery, no `DrawTool` enum
> (the archive's documented dead-end, same disease as `UndoAction`): each tool is its own struct in
> its own file, and `State.active` — the slot 48 built — *is* the "ToolHost".

<svg viewBox="0 0 680 110" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the line verb starts a tool which is an active command; two fed points construct a kernel Line; finish commits an AddGeometry command making creation undoable" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="8" y="30" width="90" height="30" fill="none" stroke="#6fb3ff"/><text x="53" y="49" fill="#d7dae0" text-anchor="middle">`line` ⏎</text>
  <rect x="130" y="30" width="150" height="30" fill="none" stroke="#6fb3ff"/><text x="205" y="43" fill="#d7dae0" text-anchor="middle">LineTool (ActiveCommand)</text><text x="205" y="55" fill="#666" text-anchor="middle" font-size="9">from → to (Get-loop, 49)</text>
  <rect x="312" y="30" width="140" height="30" fill="none" stroke="#6fb3ff"/><text x="382" y="49" fill="#d7dae0" text-anchor="middle">Line::from_points</text>
  <rect x="484" y="30" width="186" height="30" fill="none" stroke="#6fb3ff"/><text x="577" y="43" fill="#d7dae0" text-anchor="middle">AddGeometry → history</text><text x="577" y="55" fill="#666" text-anchor="middle" font-size="9">= RemoveObjects, mirrored</text>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="98" y1="45" x2="128" y2="45" marker-end="url(#ah57)"/><line x1="280" y1="45" x2="310" y2="45" marker-end="url(#ah57)"/><line x1="452" y1="45" x2="482" y2="45" marker-end="url(#ah57)"/></g>
  <text x="340" y="92" fill="#888" text-anchor="middle">click-click → a Line exists in the Session · Ctrl+Z → it never happened</text>
  <defs><marker id="ah57" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/history/add.rs   # NEW — AddGeometry: RemoveObjects with apply/revert swapped
src/app/tools/mod.rs     # NEW — pub mod point; pub mod line;
src/app/tools/point.rs   # NEW — PointTool: one click
src/app/tools/line.rs    # NEW — LineTool: two clicks
src/app/commands.rs      # `point` / `line` verbs (+ VERBS/ALIASES entries)
src/state.rs             # commit(cmd) — the one-line bridge tools use
```

## Step 1 — AddGeometry is RemoveObjects, mirrored: `src/app/history/add.rs`

Adding and deleting are the same operation with time reversed — 51's `RemoveObjects` already knows
how to take objects out *and* put snapshots back. Wrap it, swap the verbs:

```rust
use super::{remove::RemoveObjects, Command};
use crate::{app::scene::Scene, engine::gpu::Gpu};
use session_rust::Geometry;

/// Insert new objects; undo removes them. Literally RemoveObjects with apply/revert swapped —
/// the duality is the point: one tested body, two directions.
pub struct AddGeometry {
    inner: RemoveObjects,
}

impl AddGeometry {
    pub fn one(geom: Geometry) -> Self { Self { inner: RemoveObjects::of_snapshots(vec![geom]) } }
}

impl Command for AddGeometry {
    // revert-of-remove = insert
    fn apply(&mut self, scene: &mut Scene, gpu: &mut Gpu)  { self.inner.revert(scene, gpu); }
    // apply-of-remove = delete
    fn revert(&mut self, scene: &mut Scene, gpu: &mut Gpu) { self.inner.apply(scene, gpu); }
    fn label(&self) -> String { format!("add {} object(s)", self.inner.len()) }
}
```

(Two tiny additions to `RemoveObjects` in `src/app/history/remove.rs` (51): a
`pub fn of_snapshots(snapshots: Vec<Geometry>) -> Self` constructor beside `of_selection`, and
`pub fn len(&self) -> usize`. Its `revert` — restore +
`assign_row` + `apply_object` — is exactly "insert a new object"; guids mint lazily on first
`geom.guid()` read, so a freshly built object is already identity-stable when the snapshot clones it.)

And the bridge on `State` — add this method to `impl State` in `src/state.rs` (the borrow-safe
destructure from 51's note, factored once):

```rust
    pub fn commit(&mut self, cmd: Box<dyn crate::app::history::Command>) {
        let State { history, scene, gpu, .. } = self;
        history.execute(cmd, scene, gpu);
        scene.rebuild_bvh();   // new/removed objects change the box set (36)
    }
```

## Step 2 — the one-click tool: `src/app/tools/point.rs`

```rust
use session_rust::{Geometry, Point};
use crate::app::getloop::{ActiveCommand, CmdStep, GetState};

pub struct PointTool;

impl PointTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(PointTool), GetState::WaitingPoint { prompt: "point: pick location".into() })
    }
}

impl ActiveCommand for PointTool {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        state.commit(Box::new(crate::app::history::add::AddGeometry::one(Geometry::Point(p))));
        CmdStep::Done("point added".into())
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        // 'x,y,z' typed — same convergence as probe (49)
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        CmdStep::Prompt(GetState::WaitingPoint { prompt: "point: pick location".into() })
    }
    fn back(&mut self) -> CmdStep { CmdStep::Cancel }   // one step — back means out
    fn prompt(&self) -> GetState {                      // 49 made prompt() a REQUIRED trait method
        GetState::WaitingPoint { prompt: "point: pick location".into() }
    }
}
```

That's a complete tool. The Get-loop supplies clicks-or-text, Esc-cancel, and the prompt line; the
Command supplies undo. ~25 lines of actual tool.

## Step 3 — the two-click tool: `src/app/tools/line.rs`

Structurally `ProbeCmd` (49) with a different `Done`:

```rust
use session_rust::{Geometry, Line, Point};
use crate::app::getloop::{ActiveCommand, CmdStep, GetState};

pub struct LineTool {
    from: Option<Point>,
}

impl LineTool {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(LineTool { from: None }),
         GetState::WaitingPoint { prompt: "line: pick FROM point".into() })
    }
    fn ask(&self) -> CmdStep {
        let what = if self.from.is_none() { "line: pick FROM point" }
                   else { "line: pick TO point" };
        CmdStep::Prompt(GetState::WaitingPoint { prompt: what.into() })
    }
}

impl ActiveCommand for LineTool {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.from.take() {
            None => { self.from = Some(p); self.ask() }
            Some(a) => {
                let l = Line::from_points(&a, &p);
                state.commit(Box::new(
                    crate::app::history::add::AddGeometry::one(Geometry::Line(l))));
                CmdStep::Done("line added".into())
            }
        }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        self.ask()
    }
    fn back(&mut self) -> CmdStep { self.from = None; self.ask() }   // forget FROM → first prompt
    fn prompt(&self) -> GetState {                                   // 49 requires it (return GetState, not CmdStep)
        let what = if self.from.is_none() { "line: pick FROM point" } else { "line: pick TO point" };
        GetState::WaitingPoint { prompt: what.into() }
    }
}
```

## Step 4 — register the verbs: `src/app/commands.rs`

Find the verb `match` that `dispatch()` runs (the one 48/49 grew — where `probe` etc. already
resolve) and splice these two arms in beside the others:

```rust
        "point" => { let (cmd, get) = crate::app::tools::point::PointTool::start();
                     Dispatch::Start(cmd, get) }
        "line"  => { let (cmd, get) = crate::app::tools::line::LineTool::start();
                     Dispatch::Start(cmd, get) }
```

plus `VERBS`: add `"point"`, `"line"`; `ALIASES`: `("pt","point")`, `("l","line")`, `("ln","line")`.
Remember 48's empty-click rule — a click on bare grid intersects `z = 0` and still yields a point —
that's what makes these tools usable on an empty scene.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **`line`** ⏎, click the grid twice → a line appears exactly between the clicks (32a spheres on its
  ends, 31 tube for its body — the render pipelines don't know or care that it was drawn, not
  loaded). The log: `line added`.
- **Ctrl+Z** → gone. **Ctrl+Y** → back, same guid. `delete` on it → gone → Ctrl+Z → back. Creation,
  deletion, and transform all interleave in one history because they're all just Commands.
- **`pt`** ⏎, type `100,200,0` ⏎ → a point object lands at exactly those coordinates — typed and
  clicked input converge, as designed in 49.
- **Save** (39) after drawing → reload the downloaded file → your drawn line is in it. It was a
  first-class Session object from birth.
- `back` mid-`line` → re-asks FROM. Esc → `cancelled`, nothing added.

## Recap

```
Ch 56: numeric entry — the gumball is complete.
Ch 57: CREATION = pattern (b), and it's small on purpose: a drawing tool IS an ActiveCommand (49's
       machinery: prompts, click-or-type convergence, back, Esc) whose finish COMMITS an AddGeometry
       (51's machinery: undo). AddGeometry = RemoveObjects mirrored — apply calls revert and vice
       versa; one tested insert/delete body, two directions. No DrawTool enum (same dead-end as
       UndoAction — every new tool would grow a central match); State.active from 48 IS the
       ToolHost. PointTool = 1 fed point; LineTool = from/to with back support. state.commit(cmd)
       bridges tools to history with the borrow-safe destructure. Drawn objects are
       Session-first-class: they save (39), diff (38b), pick (42), and undo like everything else.
```

Edited: `app/history/add.rs` (NEW — `AddGeometry` wrapper; `RemoveObjects` gains
`of_snapshots`/`len`), `app/tools/{mod,point,line}.rs` (NEW), `app/commands.rs` (verbs + aliases),
`state.rs` (`commit`).

## Next

`58-draw-tools-2.md` — tools that need *N* clicks and live feedback: `PolylineTool` (Enter finishes),
`RectangleTool` and `BoxTool` on the z=0 plane — plus the **ghost preview**: the rubber-band line that
follows the cursor before you commit, drawn from a transient row that never touches the Session.
