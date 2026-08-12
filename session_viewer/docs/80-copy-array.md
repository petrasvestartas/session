# 80 Copy, duplicate, array — daily-use editing, nearly free

> **Big picture.** *Phase 14.* Copying is the most-used editing operation in any CAD session — and
> on this architecture it costs almost nothing, because it's three existing rails composed:
> **`duplicate()`** (the kernel's copy primitive — a clone that mints a FRESH guid, on every
> geometry type), **doc-aware insertion** (57's `add_object` rail, wrapped in one `AddGeometry` so
> every batch is undoable for free), and **`apply_world_delta`** (54 — placement lives in
> `session.xforms`, so a copy is *placed*, never re-coordinated). The lesson's real content is the
> identity trap `Rc` handles create and the Alt-drag wrinkle.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="copy duplicates the selection minting fresh guids, inserts the batch into each source doc as one AddGeometry command, then places every copy with apply world delta" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="30" width="130" height="34"/><rect x="170" y="30" width="140" height="34"/>
    <rect x="342" y="30" width="150" height="34"/><rect x="524" y="30" width="148" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="73" y="47">duplicate() per arm</text><text x="73" y="59" fill="#e05555" font-size="9">Rc clone = SAME object!</text>
    <text x="240" y="47">source doc + local xf</text><text x="240" y="59" fill="#666" font-size="9">guid_to_row → doc_of_row</text>
    <text x="417" y="47">ONE AddGeometry (57)</text><text x="417" y="59" fill="#666" font-size="9">insert batch → one undo</text>
    <text x="598" y="47">apply_world_delta</text><text x="598" y="59" fill="#666" font-size="9">54, one call per copy</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3">
    <line x1="138" y1="47" x2="168" y2="47" marker-end="url(#ah80)"/>
    <line x1="310" y1="47" x2="340" y2="47" marker-end="url(#ah80)"/>
    <line x1="492" y1="47" x2="522" y2="47" marker-end="url(#ah80)"/>
  </g>
  <text x="340" y="100" fill="#888" text-anchor="middle">array = the same pipeline in a loop; Alt+gumball = the same pipeline on 54's release</text>
  <defs><marker id="ah80" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
    <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## The identity trap — why `.clone()` cannot copy

`Geometry` variants hold **`Rc<T>`** (51). So `lookup.get(guid).cloned()` clones the *handle*: the
"copy" **is** the original — same allocation, same guid. Insert it and it *overwrites* its source
in `lookup`; the scene loses an object. And you can't fix the guid through the handle either: the
kernel's `refresh_guid` takes `&mut self`, which an `Rc` won't hand out — a naive copy doesn't
merely forget the fix, it can't reach it.

The kernel's honest primitive is **`duplicate()`** — on every geometry type
(`mesh.rs`: `clone_with_new_guid` — a full clone whose guid `OnceLock` is reset, so a fresh uuid
mints lazily on first read). One idiom, per variant:

```rust
Geometry::Mesh(m) => Geometry::Mesh(Rc::new((**m).duplicate()))
```

`(**m)` reaches through `&Rc<Mesh>` to the `Mesh`, `duplicate()` deep-copies it with a new
identity, `Rc::new` wraps the copy as its own handle. (`refresh_guid` still exists — it's the
in-place form `duplicate()` is built from — but `duplicate()` is the copy primitive; you should
never need the low-level one here.)

Two more things a copy must resolve, because neither lives in the object anymore:

- **its doc** — `guid_to_row` + `doc_of_row` (36). Copies go into the **source object's doc**, so
  manifest `place` and the session's `world_xform` compose identically for copy and original.
- **its placement** — `session.xforms` (the Xform refactor). A copy that forgets to carry the
  source's local xform lands at the doc origin, not on its source.

## Files we touch

```
src/app/history/add.rs # AddGeometry::of_snapshots — the plural constructor (57 shipped ::one)
src/app/scene.rs       # clone_selection() → doc-resolved snapshots with fresh guids
src/app/commands.rs    # `copy` (two-point Get-loop) and `array` verbs
src/state.rs           # place_copies helper; Alt held at gumball-press → drag a COPY
```

## Step 0 — the plural constructor: `src/app/history/add.rs`

57 gave `AddGeometry::one`. Copy/array commit a *batch*, so add the matching plural — find the
`impl AddGeometry` block (57) and add this beside `one` (`RemovedObj` is 51's snapshot shape —
row, doc, `Rc` handle, local xform — exactly what a copy is):

```rust
    /// Insert MANY new objects as ONE undoable Command — the same RemoveObjects duality as
    /// `one`, plural. Each snapshot inserts through 57's add_object path into ITS OWN doc.
    pub fn of_snapshots(snaps: Vec<RemovedObj>) -> Self {
        Self { inner: RemoveObjects::of_snapshots(snaps) }
    }
```

## Step 1 — duplicate with fresh identities: `src/app/scene.rs`

```rust
    /// Doc-resolved duplicates of the selection, in 51's snapshot shape — ready for ONE
    /// AddGeometry. duplicate() is the kernel's copy primitive: a deep clone that MINTS a
    /// fresh guid (a bare Rc clone would BE the original — the identity trap). `local`
    /// carries the SOURCE's session xform: placement is not in the object anymore, so a
    /// copy that omits it lands at the doc origin. `row` is assigned by the insert.
    pub fn clone_selection(&self) -> Vec<RemovedObj> {
        let mut out = Vec::new();
        for g in &self.selected {
            let Some(&row) = self.guid_to_row.get(g) else { continue };
            let doc = self.doc_of_row(row);
            let session = &self.docs[doc].session;
            let Some(geom) = session.lookup.get(g) else { continue };
            let copy = match geom {
                Geometry::Mesh(m) => Geometry::Mesh(Rc::new((**m).duplicate())),
                Geometry::BRep(b) => Geometry::BRep(Rc::new((**b).duplicate())),
                Geometry::Line(l) => Geometry::Line(Rc::new((**l).duplicate())),
                Geometry::Polyline(p) => Geometry::Polyline(Rc::new((**p).duplicate())),
                Geometry::Point(p) => Geometry::Point(Rc::new((**p).duplicate())),
                Geometry::NurbsCurve(c) => Geometry::NurbsCurve(Rc::new((**c).duplicate())),
                Geometry::NurbsSurface(s) => Geometry::NurbsSurface(Rc::new((**s).duplicate())),
                _ => continue,
            };
            out.push(RemovedObj { row: 0, doc, geom: copy, local: session.xform(g) });
        }
        out
    }
```

(Imports: `use std::rc::Rc;` and `use crate::app::history::remove::RemovedObj;` join scene.rs's
top-of-file `use` lines. Want a `_copy` name suffix in the tree? Bind the duplicate mutable in an
arm and `push_str` before wrapping — cosmetic, skipped here to keep the arms honest-length.)

And the placement half, on `impl State` (`src/state.rs`) — copies exist *after* the insert, so
their rows resolve through `guid_to_row`, and each gets 54's commit primitive plus the live GPU
poke:

```rust
    /// Post-insert placement for a batch of copies: 54's apply_world_delta per new row
    /// (Session xform + tables + cached box), then the live instance poke so the GPU row
    /// follows. Rows resolve through guid_to_row — the insert assigned them.
    pub fn place_copies(&mut self, guids: &[String], delta: &Xform) {
        for g in guids {
            let Some(&row) = self.scene.guid_to_row.get(g) else { continue };
            self.scene.apply_world_delta(row, delta);
            let m = self.scene.placed_frame(row).duplicate();
            self.gpu.set_live_model(row, &m);
        }
        self.scene.rebuild_bvh();
    }
```

Because the copy carries its source's local xform and sits in its source's doc,
`apply_world_delta`'s `L' = L · W⁻¹ · D · W` conjugation composes with the *same* `W` as the
original — a copy on a placed sheet lands exactly where the drag said, not offset by the manifest.

## Step 2 — the `copy` command: `src/app/commands.rs`

`CopyCmd` is 57's `LineTool` shape exactly — a struct with `from: Option<Point>`, the first click
stashes it, the second runs the batch: duplicate → ONE `AddGeometry` (the insert routes each
snapshot through 57's `add_object` into its own doc and `set_xform`s the carried local — the copy
materializes exactly ON its source) → `place_copies` shifts them. Add both commands below
`dispatch` in `commands.rs` — imports it needs: `use session_rust::Xform;`,
`use crate::app::history::add::AddGeometry;`:

```rust
pub struct CopyCmd {
    from: Option<Point>,
}

impl CopyCmd {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(CopyCmd { from: None }),
         GetState::WaitingPoint { prompt: "copy: pick FROM point".into() })
    }
    fn ask(&self) -> GetState {
        let what = if self.from.is_none() { "copy: pick FROM point" }
                   else { "copy: pick TO point" };
        GetState::WaitingPoint { prompt: what.into() }
    }
}

impl ActiveCommand for CopyCmd {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.from.take() {
            None => { self.from = Some(p); CmdStep::Prompt(self.ask()) }
            Some(from) => {
                let to = p;
                let delta = Xform::translation(to[0] - from[0], to[1] - from[1], to[2] - from[2]);
                let snaps = state.scene.clone_selection();
                let guids: Vec<String> = snaps.iter()
                    .map(|s| s.geom.guid().to_string()).collect();
                let n = snaps.len();
                state.commit(Box::new(AddGeometry::of_snapshots(snaps)));   // ONE undo step
                state.place_copies(&guids, &delta);                         // 54, per copy
                CmdStep::Done(format!("copied {n} object(s)"))
            }
        }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        CmdStep::Prompt(self.ask())
    }
    fn back(&mut self) -> CmdStep { self.from = None; CmdStep::Prompt(self.ask()) }
    fn prompt(&self) -> GetState { self.ask() }
}
```

(Snap (59) applies to both picks automatically, so copy-from-corner-to-corner is *exact*.)

`array` is the same in a loop — `array 5` parses the count at dispatch, then the same two points;
each round of duplicates remembers its own step delta, and the whole thing is still **one**
Command:

```rust
pub struct ArrayCmd {
    count: usize,
    from: Option<Point>,
}

impl ArrayCmd {
    pub fn start(count: usize) -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(ArrayCmd { count, from: None }),
         GetState::WaitingPoint { prompt: "array: pick FROM point".into() })
    }
    fn ask(&self) -> GetState {
        let what = if self.from.is_none() { "array: pick FROM point" }
                   else { "array: pick TO point (one step)" };
        GetState::WaitingPoint { prompt: what.into() }
    }
}

impl ActiveCommand for ArrayCmd {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.from.take() {
            None => { self.from = Some(p); CmdStep::Prompt(self.ask()) }
            Some(from) => {
                let to = p;
                let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
                let mut all = Vec::new();
                let mut placements = Vec::new();          // (guids, delta) per round
                // step the from→to delta k = 1..=count — ONE Command for all copies
                for k in 1..=self.count {
                    let dk = Xform::translation(dx * k as f64, dy * k as f64, dz * k as f64);
                    let batch = state.scene.clone_selection();
                    placements.push((batch.iter()
                        .map(|s| s.geom.guid().to_string()).collect::<Vec<_>>(), dk));
                    all.extend(batch);
                }
                let n = all.len();
                state.commit(Box::new(AddGeometry::of_snapshots(all)));
                for (guids, dk) in &placements {
                    state.place_copies(guids, dk);
                }
                CmdStep::Done(format!("arrayed {n} object(s)"))
            }
        }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        CmdStep::Prompt(self.ask())
    }
    fn back(&mut self) -> CmdStep { self.from = None; CmdStep::Prompt(self.ask()) }
    fn prompt(&self) -> GetState { self.ask() }
}
```

Register both in `dispatch`'s match (+ `VERBS`: `"copy"`, `"array"`; alias `("co","copy")`):

```rust
        "copy" => { let (cmd, get) = CopyCmd::start(); Dispatch::Start(cmd, get) }
        "array" => match parts.next().and_then(|t| t.parse::<usize>().ok()) {
            Some(count) if count > 0 => { let (cmd, get) = ArrayCmd::start(count);
                                          Dispatch::Start(cmd, get) }
            _ => Dispatch::Instant("array <count>  (then two points)".into()),
        }
```

## Step 3 — Alt+gumball-drag = drag a copy: `src/state.rs`

One branch on 54's **release**, not its press: the drag runs exactly as 54 built it (live,
matrix-only, on the originals — cheapest possible preview), and Alt changes only what *commits*:

> **New `State` field.** This branch reads `self.alt_down`, which no earlier lesson added — give `State`
> an `alt_down: bool` (init `false`) and set it in the existing `WindowEvent::ModifiersChanged` arm:
> `self.alt_down = mods.state().alt_key();`. (`state.rs` also imports
> `crate::app::history::add::AddGeometry` for the commit below.)

```rust
        // 54's release handler, first lines:
        if let Some(ctx) = self.gb_drag.take() {
            if self.alt_down {
                // originals go BACK to where they were (their kernel objects were never touched —
                // the live path only moved instance rows); the COPIES take the delta.
                for s in &ctx.before {
                    self.gpu.set_live_model(s.row, &s.placed);    // snap originals home
                }                                                 // (rows: guid_to_row, at press)
                let delta = ctx.last_delta.duplicate();
                let snaps = self.scene.clone_selection();
                let guids: Vec<String> = snaps.iter()
                    .map(|s| s.geom.guid().to_string()).collect();
                self.commit(Box::new(AddGeometry::of_snapshots(snaps)));
                self.place_copies(&guids, &delta);
                self.gb_pressed = None;
                return;
            }
            // …the normal commit continues unchanged (54's apply_world_delta + TransformObjects)…
        }
```

The subtlety that makes this clean: 54's live drag **never mutates the kernel objects** — so
"restore the originals" is just re-uploading their stashed placed frames (`ctx.before` carries
them; the rows in it came from `guid_to_row` when `begin_drag` snapshotted), and the copies are
built from pristine originals. That mid-drag discipline was designed in 54 for Esc-cancel;
Alt-copy is its second customer.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://127.0.0.1:8770
```

- Select a beam, `copy`, click its end, click the neighbouring column's corner (watch the `[End]`
  snaps) → an exact copy lands, log `copied 1 object(s)`. **Ctrl+Z** removes *only the copy*.
  Click both — **different guids**, different tree rows.
- Copy something on a PLACED sheet (any doc after the first) → the copy lands where you clicked,
  not offset by the manifest translation — same doc, same `W`, same conjugation.
- **Alt+drag** the X arrow → the original stays put, release → a copy at the drop point, one undo
  step. Without Alt → 54's normal move, unchanged.
- `array 5 …` → five more beams at even spacing, ONE undo step for all five.
- The trap test: change one `clone_selection` arm to `Geometry::Mesh(Rc::clone(m))` and `copy`
  a mesh once — the "copy" has the SAME guid, overwrites its original in `lookup`, and the scene
  loses an object. Restore the line. That's the bug `duplicate()` exists to prevent.

## Recap

```
Ch 79: files in and out.
Ch 80: DUPLICATION = three rails composed: duplicate() + AddGeometry + apply_world_delta. The trap:
       Geometry holds Rc handles — .clone() clones the HANDLE, so the "copy" IS the original (same
       guid, overwrites it in lookup), and refresh_guid is unreachable through &Rc anyway;
       duplicate() (every type — clone_with_new_guid, a reset OnceLock) is the kernel's copy
       primitive. clone_selection resolves each source's DOC (guid_to_row + doc_of_row) and carries
       its LOCAL XFORM (placement left the geometry) → 51-shaped snapshots → ONE
       AddGeometry::of_snapshots (inserts via 57's add_object into the SOURCE doc; undo removes the
       batch) → place_copies: apply_world_delta per new row (rows exist post-insert, via
       guid_to_row) + the live poke. array = the loop form, still one Command. Alt+gumball = 54's
       drag untouched until RELEASE: originals snap home (live path never touched kernel objects —
       54's Esc discipline, second customer), copies take the final delta.
```

Edited: `app/history/add.rs` (`AddGeometry::of_snapshots`), `app/scene.rs` (`clone_selection`),
`app/commands.rs` (`copy`, `array`), `state.rs` (`place_copies`, Alt branch on 54's release).

## Next

`81-layers.md` — organization users expect: layers — which the PDF importer already built as tree
groups; the lesson makes them addressable by NAME, across every loaded sheet at once.
