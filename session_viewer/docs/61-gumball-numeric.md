# 61 Gumball V — click a handle, type a number

> **Big picture.** *Phase 9.* Dragging is for roughing; CAD work is exact. Rhino's gumball has a
> beloved shortcut: *click* (don't drag) an arrow, type `500`, Enter — an exact 500 mm move. 59's
> deferred-drag threshold left the clean click unclaimed on purpose; this lesson claims it. Small
> feature, three real archive gotchas — all three are the difference between "works in the demo" and
> "works".

<svg viewBox="0 0 680 110" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="press and release under the drag threshold opens a numeric popup at the cursor; enter applies an exact relative transform through the same command path as a drag" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="150" height="30" fill="none" stroke="#6fb3ff"/><text x="85" y="49" fill="#d7dae0" text-anchor="middle">press + release &lt; 4 px</text>
  <rect x="200" y="24" width="180" height="42" fill="none" stroke="#6fb3ff"/>
  <text x="290" y="41" fill="#d7dae0" text-anchor="middle">Move X (mm)</text>
  <rect x="214" y="46" width="152" height="14" fill="none" stroke="#3a3a3a"/><text x="222" y="57" fill="#888" font-size="10">500▏</text>
  <rect x="430" y="30" width="240" height="30" fill="none" stroke="#3a3a3a"/><text x="550" y="49" fill="#888" text-anchor="middle">Enter → exact delta → TransformObjects</text>
  <line x1="160" y1="45" x2="198" y2="45" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah56)"/>
  <line x1="380" y1="45" x2="428" y2="45" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah56)"/>
  <text x="340" y="92" fill="#666" text-anchor="middle">same commit path as a drag — exact moves are undoable for free</text>
  <defs><marker id="ah56" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gumball.rs   # manual_delta(handle, value, origin) → the exact Xform; HandleKind::label()
src/ui/mod.rs           # the popup: one egui window at the cursor, one focused line edit
src/state.rs            # release-under-threshold opens it; Enter applies; THE THREE GOTCHAS
```

## Step 1 — value → transform: `src/engine/gumball.rs`

One function maps (handle, typed value) to a delta; the unit depends on the handle group — that's
also what the popup's title shows. (`manual_delta` names two more kernel types — grow the import to
`use session_rust::{Line, Point, Vector, Xform};`.)

```rust
impl HandleKind {
    /// Popup title + unit: "Move X (mm)", "Rotate Z (deg)", "Scale (factor)".
    pub fn label(self) -> &'static str {
        use HandleKind::*;
        match self {
            TranslateX => "Move X (mm)",   TranslateY => "Move Y (mm)",
            TranslateZ => "Move Z (mm)",
            RotateX    => "Rotate X (deg)", RotateY   => "Rotate Y (deg)",
            RotateZ   => "Rotate Z (deg)",
            ScaleX     => "Scale X (factor)", ScaleY  => "Scale Y (factor)",
            ScaleZ  => "Scale Z (factor)",
            ScaleUniform => "Scale (factor)",
        }
    }
}

/// The exact delta for a typed value — RELATIVE, about the gumball origin (like the drags).
/// Degrees for rotation (users think in degrees); scale floored at 0.01 to match the drag floor.
pub fn manual_delta(handle: HandleKind, value: f64, o: &Point) -> Xform {
    use HandleKind::*;
    let ax = |i: usize| { let mut a = [0.0; 3]; a[i] = 1.0; Vector::new(a[0], a[1], a[2]) };
    match handle {
        TranslateX => Xform::translation(value, 0.0, 0.0),
        TranslateY => Xform::translation(0.0, value, 0.0),
        TranslateZ => Xform::translation(0.0, 0.0, value),
        RotateX | RotateY | RotateZ => {
            let i = matches!(handle, RotateY) as usize + 2 * matches!(handle, RotateZ) as usize;
            let axis_line = Line::from_points(o, &(o.clone() + ax(i)));
            Xform::rotation_around_line(&axis_line, value, true)     // true = the value IS degrees
        }
        ScaleX => Xform::scale_non_uniform(o, value.max(0.01), 1.0, 1.0),
        ScaleY => Xform::scale_non_uniform(o, 1.0, value.max(0.01), 1.0),
        ScaleZ => Xform::scale_non_uniform(o, 1.0, 1.0, value.max(0.01)),
        ScaleUniform => { let f = value.max(0.01); Xform::scale_non_uniform(o, f, f, f) }
    }
}
```

## Step 2 — the popup: `src/ui/mod.rs`

A borderless one-field window pinned at the click position. Add **two** fields to `UiState`
(and initialize both in `UiState::new`/`Default`): `gb_input: Option<(HandleKind, String, (f32, f32))>`
— handle, buffer, screen pos — and `gb_submit: bool` (`false`), the closure→state flag the popup sets.
In `build_ui`:

```rust
    if let Some((handle, buffer, pos)) = &mut ui_state.gb_input {
        let mut submit = false;
        egui::Window::new(handle.label())
            .fixed_pos([pos.0, pos.1]).collapsible(false).resizable(false)
            .show(ctx, |ui| {
                let r = ui.add(egui::TextEdit::singleline(buffer).desired_width(90.0));
                r.request_focus();                                   // typing starts immediately
                if r.lost_focus() &&
                    ui.input(|i| i.key_pressed(egui::Key::Enter)) { submit = true; }
            });
        // State applies after the closure (52's rule)
        if submit { ui_state.gb_submit = true; }
    }
```

## Step 3 — open, apply, and the three gotchas: `src/state.rs`

**Open** — in the mouse-release handler. A subtlety: 59's commit block has already `take()`n
`gb_drag` by this point, so "was this a drag?" must be read *before* it. Insert
`let dragged = self.gb_drag.is_some();` as the first line of the release handler (above 59's
`if let Some(ctx) = self.gb_drag.take()`), then find 59's trailing `self.gb_pressed = None;` →
replace it with:

```rust
        if let Some((handle, _press_at)) = self.gb_pressed.take() {
            // never crossed the 4 px threshold → this was a CLICK on a handle
            if !dragged {
                self.ui.gb_input = Some((handle, String::new(),
                                         (self.cursor.0 as f32, self.cursor.1 as f32)));
            }
        }
```

**Apply** — after `build_ui`, where `&mut self` is free again:

```rust
        if self.ui.gb_submit {
            self.ui.gb_submit = false;
            if let Some((handle, buffer, _)) = self.ui.gb_input.take() {
                // guard: selection may have been cleared while the popup was open
                if let (Ok(v), Some(c)) =
                    (buffer.trim().parse::<f64>(), self.scene.selection_centroid()) {
                    let o = Point::new(c[0] as f64, c[1] as f64, c[2] as f64);
                    let delta = crate::engine::gumball::manual_delta(handle, v, &o);
                    self.apply_transform_command(&delta);            // 59's commit path, factored:
                    // snapshots → apply_world_delta → execute
                }
            }
        }
```

`apply_transform_command` is 59's release-commit path as a named method: snapshot the current
selection's PLACEMENTS, `apply_world_delta` each row, wrap `before`/`after` in `TransformObjects`,
execute. It works for the drag release too, because 59's live path never mutates durable state — a
release-time "before" snapshot equals the press-time one. Add it to `impl State` (imports: the
`XformSnap` 54 already brought in):

```rust
    /// Commit `delta` (WORLD space) to the current selection as one undoable TransformObjects
    /// (59's path: L' = L·W⁻¹·D·W per row — placement-only, kind-agnostic, place-conjugated).
    pub fn apply_transform_command(&mut self, delta: &Xform) {
        let snap = |scene: &Scene, guid: &String| -> Option<XformSnap> {
            let &row = scene.guid_to_row.get(guid)?;
            let d = scene.doc_of_row(row);
            Some(XformSnap {
                row,
                guid: guid.clone(),
                local: scene.docs[d].session.xform(guid),
                placed: scene.placed_frame(row).duplicate(),
            })
        };
        let before: Vec<XformSnap> = self.scene.selected.iter()
            .filter_map(|g| snap(&self.scene, g)).collect();
        for s in &before {
            self.scene.apply_world_delta(s.row, delta);                   // 59's commit primitive
        }
        let after: Vec<XformSnap> = before.iter()
            .filter_map(|s| snap(&self.scene, &s.guid)).collect();
        let cmd = Box::new(TransformObjects { before, after });
        self.history.execute(cmd, &mut self.scene, &mut self.gpu);        // applies `after` — idempotent
        self.scene.rebuild_bvh();                                         // boxes moved (40)
        self.refresh_gumball();                                           // widget follows the move
    }
```

(An exact typed `500` on a PLACED sheet moves 500 mm in world space, not in the sheet's local
frame — the conjugation inside `apply_world_delta` is what makes the number mean what the user
typed, on every document.)

Then shrink 59's release arm onto it — find the whole `if let Some(ctx) = self.gb_drag.take() { … }`
block (59's bake + Command code) → replace with:

```rust
        if let Some(ctx) = self.gb_drag.take() {
            self.apply_transform_command(&ctx.last_delta);
        }
```

Both the drag release and this Enter path now run one commit path — which is why an exact typed move
is undoable for the same reason a drag is.

**The three gotchas** (each one a real archive bug):

1. **The lmb gate.** Track `lmb_down` from the *raw* winit `MouseInput` **before** egui routing —
   once the popup exists, egui consumes the release, and without the raw flag a stale press could
   start a "no-button drag" the next time the cursor crosses the gumball. 59's threshold check
   already requires `self.lmb_down`; this is why it must be set upstream of `consumed`.
2. **Fast taps.** A quick press-release can deliver the release *before* the deferred pick processes
   the press. Handle it where the state is, not where the event is: opening the popup keys off
   `gb_pressed`-and-no-drag at release time (the code above), never off event ordering.
3. **The Escape guard.** Esc already means "cancel command" (53) — and in some setups "quit". While
   `gb_input.is_some()`, Esc must close *the popup only*:

```rust
        Key::Named(NamedKey::Escape) if self.ui.gb_input.is_some() => { self.ui.gb_input = None; }
        // …the 48 Esc arm stays BELOW this one — match order is the guard
```

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Click (don't drag) the X arrow → `Move X (mm)` popup at the cursor, already focused. Type `500` ⏎ —
  the selection jumps exactly 500 mm; `probe` two matching corners to confirm. **Ctrl+Z** undoes it —
  same Command as a drag.
- Click the blue arc → `Rotate Z (deg)`, type `45` ⏎ → exact 45°. Click the white sphere → `2` ⏎ →
  doubled about the centroid. `0` on a scale handle → clamps to 0.01, no vanishing.
- Click a handle and *drag* → still a drag, no popup (the threshold arbitrates). Esc with the popup
  open → popup closes, command state untouched. Type `abc` ⏎ → nothing happens, no panic.

## Recap

```
Ch 60: rotate + scale — the drag family complete.
Ch 61: NUMERIC ENTRY. Release under 59's 4 px threshold = CLICK → egui popup at the cursor titled by
       HandleKind::label() ("Move X (mm)" / "Rotate Z (deg)" / "Scale (factor)"). manual_delta maps
       the typed value to a RELATIVE delta about the centroid (degrees→radians via the kernel's PI;
       scale floored 0.01 like drags) and commits through 59's exact path — undoable for free. The
       three archive gotchas: lmb tracked from RAW MouseInput before egui consumes releases (no
       phantom drags); popup-open decided from state at release (fast taps beat event order); Esc
       guard arm ABOVE 53's cancel arm (popup closes, nothing else reacts). The gumball is complete.
```

Edited: `engine/gumball.rs` (`HandleKind::label`, `manual_delta`), `ui/mod.rs` (popup + `gb_input`/
`gb_submit`), `state.rs` (open on clean release, apply after closure, the three guards).

## Next

`62-draw-tools.md` — creating geometry at last: the `Tool` trait (ARCHITECTURE's pattern (b)),
`PointTool` and `LineTool` driven by the Get-loop's prompts, finishing as `AddGeometry` Commands —
click-click, a line exists in the Session, and Ctrl+Z removes it.
