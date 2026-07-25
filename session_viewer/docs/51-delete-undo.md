# 51 Delete + undo/redo — the pattern everything stands on

> **Big picture.** *Phase 8 closes.* This is the most consequential lesson of the phase: the first
> command that **destroys** data, and therefore the machinery that makes destruction safe. The
> pattern — `trait Command { apply / revert }` + two stacks — is how every serious editor does undo,
> and *every* later mutation (gumball drags 54, drawing tools 57, CV edits 73) will be born as one of
> these objects and become undoable for free. The archive got this wrong with an `UndoAction` enum
> and documented the dead-end; we start on the trait.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="executing a command pushes it onto the done stack; undo pops done, reverts, pushes onto undone; redo pops undone, applies, pushes back onto done" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="30" y="26" width="150" height="34" fill="none" stroke="#6fb3ff"/><text x="105" y="47" fill="#d7dae0" text-anchor="middle">execute: apply()</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2"><rect x="260" y="16" width="140" height="54"/></g>
  <text x="330" y="34" fill="#d7dae0" text-anchor="middle">done</text>
  <text x="330" y="50" fill="#666" text-anchor="middle" font-size="10">Delete "box_3"</text>
  <text x="330" y="63" fill="#666" text-anchor="middle" font-size="10">Delete "wall_7"</text>
  <g fill="none" stroke="#3a3a3a"><rect x="480" y="16" width="140" height="54"/></g>
  <text x="550" y="34" fill="#888" text-anchor="middle">undone</text>
  <line x1="180" y1="43" x2="258" y2="43" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah51)"/>
  <path d="M400,32 H478" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah51)"/>
  <text x="439" y="26" fill="#6fb3ff" font-size="10">Ctrl+Z: revert()</text>
  <path d="M478,58 H400" stroke="#888" stroke-width="1.2" marker-end="url(#ah51g)"/>
  <text x="439" y="72" fill="#888" font-size="10">Ctrl+Y: apply()</text>
  <text x="340" y="110" fill="#888" text-anchor="middle">a NEW command clears `undone` — the classic branch-point rule</text>
  <text x="340" y="130" fill="#666" text-anchor="middle">snapshots are ABSOLUTE (the whole Geometry), so revert can't drift</text>
  <defs>
    <marker id="ah51" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker>
    <marker id="ah51g" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#888"/></marker>
  </defs>
</svg>

## Why a trait, not an enum — the archive's documented dead-end

The archive shipped `enum UndoAction { AddGeom(..), DeleteGeom(..), EditSnapshot(..), … }` — and every
new feature meant a new variant *plus* new arms in the central undo/redo matches. Ten features later
that enum was a bottleneck everything had to be threaded through. A `trait Command` inverts it: each
feature ships its own `apply`/`revert` next to the code it belongs to, and `History` never changes
again. That inversion — **open for extension, closed at the core** — is the entire lesson.

## Files we touch

```
src/app/history/mod.rs      # NEW — trait Command + History { done, undone }
src/app/history/remove.rs   # NEW — RemoveObjects: absolute snapshots, apply/revert
src/app/commands.rs         # `delete` verb; `undo`/`redo` verbs
src/state.rs                # State.history; Ctrl+Z / Ctrl+Y / Del key
src/app/scene.rs            # restore_geometry — Geometry → the right Session::add_* call
```

## Step 1 — the pattern: `src/app/history/mod.rs` (NEW)

```rust
//! Undo = objects, not variants. Every mutation implements Command; History stores boxes and
//! never learns what they do. (The archive's UndoAction enum is the documented anti-pattern
//! this replaces.)

use crate::{app::scene::Scene, engine::gpu::Gpu};

pub trait Command {
    fn apply(&mut self, scene: &mut Scene, gpu: &mut Gpu);
    fn revert(&mut self, scene: &mut Scene, gpu: &mut Gpu);
    fn label(&self) -> String;                     // for the log: "undo: delete 2 object(s)"
}

pub struct History {
    done: Vec<Box<dyn Command>>,
    undone: Vec<Box<dyn Command>>,
}

impl History {
    pub fn new() -> Self { Self { done: Vec::new(), undone: Vec::new() } }

    /// Run a fresh command. A new action invalidates the redo branch — the classic rule.
    pub fn execute(&mut self, mut cmd: Box<dyn Command>, scene: &mut Scene, gpu: &mut Gpu) {
        cmd.apply(scene, gpu);
        self.done.push(cmd);
        self.undone.clear();
    }
    pub fn undo(&mut self, scene: &mut Scene, gpu: &mut Gpu) -> Option<String> {
        let mut cmd = self.done.pop()?;
        cmd.revert(scene, gpu);
        let label = cmd.label();
        self.undone.push(cmd);
        Some(label)
    }
    pub fn redo(&mut self, scene: &mut Scene, gpu: &mut Gpu) -> Option<String> {
        let mut cmd = self.undone.pop()?;
        cmd.apply(scene, gpu);
        let label = cmd.label();
        self.done.push(cmd);
        Some(label)
    }
}
```

Add `pub history: History` to `State`, initialize it in `State::new` (`history: History::new(),` alongside the other field inits), and add the module decl `pub mod history;` in `app/mod.rs`.

## Step 2 — the first Command: `src/app/history/remove.rs` (NEW)

`RemoveObjects` holds **absolute snapshots** — the full cloned `Geometry`, not "what changed".
Absolute snapshots can't drift: revert doesn't *compute* the old state, it *is* the old state. (The
guid lives inside the cloned object, so a restored object keeps its identity — selection, tree rows,
and later references all survive the round trip.)

```rust
use session_rust::Geometry;
use super::Command;
use crate::{app::scene::Scene, engine::gpu::Gpu};

pub struct RemoveObjects {
    snapshots: Vec<Geometry>,     // absolute state of every deleted object
}

impl RemoveObjects {
    /// Snapshot NOW, at construction — before anything mutates.
    pub fn of_selection(scene: &Scene) -> Self {
        let snapshots = scene.selected.iter()
            .filter_map(|g| scene.session.lookup.get(g).cloned())
            .collect();
        Self { snapshots }
    }
}

impl Command for RemoveObjects {
    fn apply(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        for geom in &self.snapshots {
            let guid = geom.guid().to_string();
            // kernel: lookup + collections + tree + graph
            scene.session.remove_object(&guid);
            if let Some(&row) = scene.guid_to_row.get(&guid) {
                gpu.remove_object(&guid);                  // 38a: arena free + segment/glyph drain
                gpu.hide_row(row);
                scene.free_row(&guid);                     // row back on the free list
            }
            scene.selected.remove(&guid);
            scene.hashes.remove(&guid);
        }
    }
    fn revert(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        for geom in &self.snapshots {
            let guid = geom.guid().to_string();
            scene.restore_geometry(geom.clone());          // Session::add_* by variant (Step 3)
            // recycled or fresh — guid_to_row rebinds
            let row = scene.assign_row(&guid);
            // 38b: re-flatten into arena + instance row
            scene.apply_object(gpu, &guid, geom, row);
        }
    }
    fn label(&self) -> String { format!("delete {} object(s)", self.snapshots.len()) }
}
```

## Step 3 — restoring a snapshot: `src/app/scene.rs`

The kernel removes generically (`Session::remove_object(guid)`) but adds per type — a small dispatch
closes the gap (kernel-gap #8 in `_KERNEL_GAPS.md`: a kernel `add_geometry(Geometry)` would delete
this function):

```rust
    /// Re-insert a snapshot into the Session (lookup + collections + tree).
    /// Guid is inside the object.
    pub fn restore_geometry(&mut self, geom: Geometry) {
        match geom {
            Geometry::Mesh(m)     => { self.session.add_mesh(m, None); }
            Geometry::BRep(b)     => { self.session.add_brep(b, None); }
            Geometry::Line(l)     => { self.session.add_line(l, None); }
            Geometry::Polyline(p) => { self.session.add_polyline(p, None); }
            Geometry::Point(p)    => { self.session.add_point(p, None); }
            _ => {}
        }
    }
```

(Deleted objects lose their tree *position* — they re-enter at the root. Remembering the parent node
is a straightforward extension of the snapshot once the tree UI exists, 70.)

## Step 4 — verbs + keys: `src/app/commands.rs` + `src/state.rs`

Three new arms (and add `"delete"`, `"undo"`, `"redo"` to `VERBS`; alias `("rm","delete")`,
`("del","delete")`):

```rust
        "delete" => {
            if state.scene.selected.is_empty() {
                return Dispatch::Instant("nothing selected".into());
            }
            let cmd = Box::new(
                crate::app::history::remove::RemoveObjects::of_selection(&state.scene));
            let label = cmd.label();
            state.history.execute(cmd, &mut state.scene, &mut state.gpu);
            Dispatch::Instant(label)
        }
        "undo" => Dispatch::Instant(state.history.undo(&mut state.scene, &mut state.gpu)
                       .map(|l| format!("undo: {l}")).unwrap_or("nothing to undo".into())),
        "redo" => Dispatch::Instant(state.history.redo(&mut state.scene, &mut state.gpu)
                       .map(|l| format!("redo: {l}")).unwrap_or("nothing to redo".into())),
```

Keyboard shortcuts are just typists (the commands-only philosophy made literal):

```rust
        Key::Named(NamedKey::Delete) => self.run_command("delete"),
        Key::Character("z") if ctrl  => self.run_command("undo"),
        Key::Character("y") if ctrl  => self.run_command("redo"),
```

> **Borrow note.** `dispatch` takes `&mut State` while calling `state.history.execute(cmd, &mut
> state.scene, &mut state.gpu)` — three disjoint fields of the same struct, which Rust allows only
> when accessed *as fields*, not through `&mut State` methods. If the compiler objects in your
> arrangement, make `History::execute` a free call taking the three fields, or destructure:
> `let State { history, scene, gpu, .. } = state;`. The lesson code uses direct field access for
> exactly this reason.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select two objects → **Del** → gone; log `delete 2 object(s)`; HUD object count drops by 2.
- **Ctrl+Z** → both return, same colors, same placement, and clicking one shows the **same guid** as
  before — identity survived the round trip (the snapshot carries it). **Ctrl+Y** → gone again.
- Delete A, delete B, Ctrl+Z ×2 → both back in reverse order; delete C now → **redo is dead** (the
  branch-point rule — `undone` cleared).
- The `#[cfg(test)]`: build a Scene, snapshot-delete an object, assert it's gone from
  `session.lookup`, revert, assert `lookup[guid]`'s jsondump equals the original — *byte-identical*
  restoration, the absolute-snapshot guarantee.

## Recap

```
Ch 50: history/autocomplete — CLI ergonomics.
Ch 51: UNDO. trait Command { apply / revert / label } + History { done, undone } — the archive's
       UndoAction enum is the documented dead-end (every feature = new variant + new central match);
       the trait inverts it so History never changes again. execute → done.push + undone.clear
       (a new action kills the redo branch). RemoveObjects = ABSOLUTE Geometry snapshots taken
       at construction; apply = Session::remove_object + 38a arena free + row free; revert =
       restore_geometry (per-variant add_*) + assign_row + apply_object — byte-identical restore,
       guid preserved (it lives in the clone). delete/undo/redo verbs; Del / Ctrl+Z / Ctrl+Y just
       type them. Phase 8 complete: every future mutation is born a Command and undoable for free.
```

Edited: `app/history/mod.rs` (NEW — trait + stacks), `app/history/remove.rs` (NEW — `RemoveObjects`),
`app/scene.rs` (`restore_geometry`), `app/commands.rs` (delete/undo/redo), `state.rs` (`history`,
Del/Ctrl+Z/Ctrl+Y).

## Next

`52-gumball-geometry.md` — Phase 9: transform & draw. The 3-axis gizmo appears at the selection
centroid — axis cylinders, cone tips, rotate arcs, scale boxes, all built from kernel meshes into
ordinary instance rows with one stable id per handle, drawn last so it floats over the scene.
