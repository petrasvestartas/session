# 82 Measure + status bar — the viewer answers questions

> **Big picture.** *Phase 14.* CAD models exist to be interrogated: how long, how far apart, what
> angle, how big. Lesson 49's `probe` was secretly the prototype — a Get-loop conversation ending in
> a printed number. This lesson makes the family real (`distance`, `angle`, `radius`, `what`) and
> adds the **status bar**: the always-on line telling you where the cursor is, what's snapped, and
> what's selected. All kernel math, all existing machinery.

## Files we touch

```
src/app/commands.rs   # distance / angle / radius / what — four ProbeCmd-shaped conversations
src/ui/mod.rs         # the status line, rendered above the CLI in the same bottom panel
src/state.rs          # feeds cursor world coords + snap kind + selection count into UiState
```

## Step 1 — the measure verbs: `src/app/commands.rs`

Each is 49's template with a different `Done`. The math is kernel calls or three-liners:

```rust
    // distance — probe, renamed, unrounded by default:
    let d = a.distance(&b, None);
    CmdStep::Done(format!("distance = {d:.3}"))

    // angle — three points: vertex, then one point per arm; kernel Vector::angle does the rest:
    let u = a - v.clone();                               // Point − Point → Vector
    let w = b - v;
    let deg = u.angle(&w, false).to_degrees();           // kernel angle, unsigned
    CmdStep::Done(format!("angle = {deg:.2}°"))

    // radius — three points on the arc; circumradius R = abc / 4A (pure triangle math):
    let (la, lb, lc) = (b.distance(&c, None), a.distance(&c, None), a.distance(&b, None));
    let s = (la + lb + lc) * 0.5;
    let area2 = s * (s - la) * (s - lb) * (s - lc);      // Heron; ≤ 0 → collinear
    if area2 <= 0.0 {
        return CmdStep::Done("points are collinear — no circle".into());
    }
    let r = (la * lb * lc) / (4.0 * area2.sqrt());
    CmdStep::Done(format!("radius = {r:.3}  (diameter {:.3})", r * 2.0))
```

`what` is instant, not a conversation — it reports the selection via kernel accessors:

```rust
        "what" => {
            let Some(g) = state.scene.selected.iter().next() else {
                return Dispatch::Instant("nothing selected".into());
            };
            let msg = match &state.scene.session.lookup[g] {
                Geometry::Mesh(m) => format!("Mesh '{}': {} verts, {} faces, area {:.3}",
                    m.name, m.number_of_vertices(), m.number_of_faces(), m.area()),
                Geometry::Line(l) => format!("Line '{}': length {:.3}", l.name, l.length()),
                Geometry::Polyline(p) => format!("Polyline '{}': {} points", p.name,
                    p.get_points().len()),
                Geometry::Point(p) => format!("Point '{}': ({:.3}, {:.3}, {:.3})",
                    p.name, p[0], p[1], p[2]),
                Geometry::BRep(b) => format!("BRep '{}': {} faces, area {:.3}",
                    b.name, b.m_faces.len(), b.mesh().area()),
                Geometry::NurbsCurve(c) => format!("NurbsCurve '{}': degree {}, {} CVs",
                    c.name, c.degree(), c.cv_count()),
                other => format!("{:?}", std::mem::discriminant(other)),
            };
            Dispatch::Instant(msg)
        }
```

Every point in these conversations comes through `cursor_world_point` — **snapped** (59). Measuring
corner-to-corner is exact to the digit, which is the entire difference between a measure tool and a
guess tool.

## Step 2 — the status bar: `src/ui/mod.rs` + `src/state.rs`

One always-on line above the CLI input, in the panel 48 built. It renders three facts `State`
already knows and just has to hand over each frame:

```rust
    // UiState gains: cursor_world: Option<[f64; 3]>, snap_label: &'static str, sel_count: usize
    // state.rs, before build_ui — all three values exist already:
    self.ui.cursor_world = self.last_cursor_world;              // cursor_world_point's last result
    self.ui.snap_label = self.snap_marker.map(|(_, k)| k.label()).unwrap_or("");
    self.ui.sel_count = self.scene.selected.len();
```

```rust
    // inside the bottom panel, above the input row:
    ui.horizontal(|ui| {
        if let Some(c) = ui_state.cursor_world {
            ui.monospace(format!("x {:>10.2}  y {:>10.2}  z {:>10.2}", c[0], c[1], c[2]));
        }
        if !ui_state.snap_label.is_empty() {
            ui.colored_label(egui::Color32::from_rgb(224, 176, 64), ui_state.snap_label);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(format!("{} selected", ui_state.sel_count));
        });
    });
```

One caveat worth its comment: computing `cursor_world_point` on *every* mouse move purely for the
readout would run picking per move even when idle. Cheap rule: when no command is active and
nothing needs snap, resolve the readout against the work plane only (75's one intersection) —
full pick-based resolution stays reserved for when a tool is actually listening.

## Step 3 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- `distance`, click two column corners (both snap `[End]`) → matches `what`'s reported dimensions
  exactly. `angle` on a corner → `90.00°`. `radius` on three points of a drawn arc-ish curve → a
  sane radius; three collinear picks → the honest refusal.
- `what` on a beam → name, counts, area — kernel numbers, so cross-checkable against the minitests.
- Sweep the mouse: coordinates track live; hover a corner mid-command → the amber `End` chip
  appears in the bar; selection count updates on every click. Idle GPU cost: zero extra (66 — the
  bar only repaints when egui repaints).

## Recap

```
Ch 81: layers.
Ch 82: INTERROGATION. distance/angle/radius = 49's conversation template + kernel math (distance,
       Vector::angle, Heron circumradius with a collinear guard); every picked point arrives SNAPPED
       (59), which is what makes measurements exact rather than approximate. `what` = instant verb
       over kernel accessors (counts, area, length, degree). Status bar = one always-on panel row of
       three facts State already tracks: cursor world coords (work-plane-resolved when idle — don't
       run picking per move for a readout), active snap chip, selection count.
```

Edited: `app/commands.rs` (four verbs), `ui/mod.rs` (status row), `state.rs` (three UiState feeds).

## Next

`83-dev-toolbox.md` — the developer-experience lesson: a headless selftest, GPU errors that surface
in the viewer's own CLI, a black-screen checklist, and CI.
