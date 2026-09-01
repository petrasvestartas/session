# 104 Measure + status bar — the viewer answers questions

> **Big picture.** *Phase 14.* CAD models exist to be interrogated: how long, how far apart, what
> angle, how big. Lesson 71's `probe` was secretly the prototype — a Get-loop conversation ending in
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

Each is 62's template with a different `Done`. Build each as a `ProbeCmd`-shaped struct (62) that
collects its picks in a `Vec<Point>`; the fragments below are only the final `feed_point`, run once
the last point lands. There the collected picks are named `a`, `b` (and `c` for the three-point
verbs), with `v` the angle's vertex — bind them off the vec (e.g. `let (v, a, b) = (pts[0].clone(),
pts[1].clone(), pts[2].clone());`) before the math. The math itself is kernel calls or three-liners:

```rust
    // distance — probe, renamed, unrounded by default:
    let d = a.distance(&b, None);
    CmdStep::Done(format!("distance = {d:.3}"))

    // angle — three points: vertex, then one point per arm; kernel Vector::angle does the rest:
    let u = a - v.clone();                               // Point − Point → Vector
    let w = b - v;
    let deg = u.angle(&w, false);                        // kernel angle — already DEGREES, unsigned
    CmdStep::Done(format!("angle = {deg:.2}°"))

    // radius — three points on the arc; circumradius R = abc / 4A (pure triangle math).
    // A from the CROSS PRODUCT, not Heron: s·(s−a)·(s−b)·(s−c) cancels catastrophically on the
    // needle-thin triangles three clicks along a shallow arc produce; the cross product keeps
    // full precision all the way to collinearity.
    let (la, lb, lc) = (b.distance(&c, None), a.distance(&c, None), a.distance(&b, None));
    let (u, v) = (b - a.clone(), c - a.clone());       // edge vectors from a (Point − Point → Vector)
    let cross2 = u.cross(&v).magnitude();              // = 2A
    // near-collinear REFUSAL, relative to the edges — an absolute epsilon misjudges both a
    // 1e-3 mm sliver and a 1e6 mm site plan
    if cross2 <= 1e-9 * u.magnitude() * v.magnitude() {
        return CmdStep::Done("points are collinear — no circle".into());
    }
    let r = (la * lb * lc) / (2.0 * cross2);           // 4A = 2·cross2
    CmdStep::Done(format!("radius = {r:.3}  (diameter {:.3})", r * 2.0))
```

<svg viewBox="0 0 640 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="left: angle from a vertex v with arms to a and b, theta from u.angle(w); right: circumradius through three points a b c, R = product of side lengths over four times the area" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="18" fill="#888" text-anchor="middle">angle — vertex + two arms</text>
  <line x1="90" y1="150" x2="50" y2="55" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="90" y1="150" x2="255" y2="75" stroke="#6fb3ff" stroke-width="1.5"/>
  <path d="M78.4,122.3 A 30 30 0 0 1 117.3,137.6" fill="none" stroke="#5bbf87" stroke-width="1.3"/>
  <text x="112" y="118" fill="#5bbf87">θ</text>
  <circle cx="90" cy="150" r="3.5" fill="#d7dae0"/>
  <circle cx="50" cy="55" r="3.5" fill="#6fb3ff"/>
  <circle cx="255" cy="75" r="3.5" fill="#6fb3ff"/>
  <text x="76" y="167" fill="#d7dae0">v</text>
  <text x="38" y="50" fill="#d7dae0">a</text>
  <text x="263" y="72" fill="#d7dae0">b</text>
  <text x="150" y="190" fill="#666" text-anchor="middle">u=a−v  w=b−v  →  u.angle(w) = θ (deg)</text>
  <line x1="320" y1="30" x2="320" y2="180" stroke="#3a3a3a"/>
  <text x="490" y="18" fill="#888" text-anchor="middle">circumradius — three points</text>
  <circle cx="490" cy="110" r="70" fill="none" stroke="#555"/>
  <line x1="490" y1="40" x2="429" y2="145" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="429" y1="145" x2="551" y2="145" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="551" y1="145" x2="490" y2="40" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="490" y1="110" x2="490" y2="40" stroke="#5bbf87" stroke-width="1.2" stroke-dasharray="3 3"/>
  <text x="497" y="80" fill="#5bbf87">R</text>
  <circle cx="490" cy="110" r="3" fill="#888"/>
  <circle cx="490" cy="40" r="3.5" fill="#d7dae0"/>
  <circle cx="429" cy="145" r="3.5" fill="#d7dae0"/>
  <circle cx="551" cy="145" r="3.5" fill="#d7dae0"/>
  <text x="490" y="32" fill="#d7dae0" text-anchor="middle">a</text>
  <text x="417" y="150" fill="#d7dae0">b</text>
  <text x="557" y="150" fill="#d7dae0">c</text>
  <text x="490" y="190" fill="#666" text-anchor="middle">R = |ab|·|bc|·|ca| / 4·Area  (Area from the cross product)</text>
</svg>

`what` is instant, not a conversation — it reports the selection via kernel accessors:

```rust
        "what" => {
            let Some(&row) = state.scene.selected.iter().next() else {
                return Dispatch::Instant("nothing selected".into());
            };
            // Selection is row-keyed (58); the guid is order[row]. Resolve the OWNING doc and
            // use .get — lookup indexing PANICS on a stale guid.
            let g = &state.scene.order[row as usize];
            let Some(geo) = state.scene.docs.iter().find_map(|d| d.session.lookup.get(g)) else {
                return Dispatch::Instant("stale guid".into());
            };
            let msg = match geo {
                Geometry::Mesh(m) => format!("Mesh '{}': {} verts, {} faces, area {:.3}",
                    m.name, m.number_of_vertices(), m.number_of_faces(), m.area()),
                Geometry::Line(l) => format!("Line '{}': length {:.3}", l.name, l.length()),
                Geometry::Polyline(p) => format!("Polyline '{}': {} points", p.name,
                    p.get_points().len()),
                Geometry::Point(p) => format!("Point '{}': ({:.3}, {:.3}, {:.3})",
                    p.name, p[0], p[1], p[2]),
                // b.mesh() RE-TESSELLATES per call — tracked in _KERNEL_GAPS.md — fine for a
                // click, not per-frame:
                Geometry::BRep(b) => format!("BRep '{}': {} faces, area {:.3}",
                    b.name, b.m_faces.len(), b.mesh().area()),
                Geometry::NurbsCurve(c) => format!("NurbsCurve '{}': degree {}, {} CVs",
                    c.name, c.degree(), c.cv_count()),
                other => format!("{:?}", std::mem::discriminant(other)),
            };
            Dispatch::Instant(msg)
        }
```

The reported numbers are the object's own; its world frame is the row's placed frame
(`scene.tables.objects[row].0` via `guid_to_row`), and a good `what` improvement is one more line
reporting placement via `session.world_xform(guid)` — note it walks the chain per call (quadratic
over many objects; bulk queries use `world_xforms()`).

Every point in these conversations comes through `cursor_world_point` — **snapped** (72). Measuring
corner-to-corner is exact to the digit, which is the entire difference between a measure tool and a
guess tool.

## Step 2 — the status bar: `src/ui/mod.rs` + `src/state.rs`

One always-on line above the CLI input, in the panel 53 built. It renders three facts `State`
already knows and just has to hand over each frame. First **add three fields to the `UiState` struct
(61)**:

```rust
    // find `struct UiState { … }` and add:
    pub cursor_world: Option<[f64; 3]>,
    pub snap_label: &'static str,
    pub sel_count: usize,
```

They default fine (`None` / `""` / `0`) if `UiState` derives `Default`; otherwise seed them in its
ctor. Then feed all three each frame — in `state.rs`, before `build_ui`, where every value already exists:

```rust
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
nothing needs snap, resolve the readout against the work plane only (87's one intersection) —
full pick-based resolution stays reserved for when a tool is actually listening. With multiple docs
in the scene, the readout should also name WHICH sheet the cursor is over — each doc has its own
place, and a coordinate without its sheet is ambiguous.

## Step 3 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- `distance`, click two column corners (both snap `[End]`) → matches `what`'s reported dimensions
  exactly. `angle` on a corner → `90.00°`. `radius` on three points of a drawn arc-ish curve → a
  sane radius; three collinear picks → the honest refusal.
- `what` on a beam → name, counts, area — kernel numbers, so cross-checkable against the minitests.
- Sweep the mouse: coordinates track live; hover a corner mid-command → the amber `End` chip
  appears in the bar; selection count updates on every click. Idle GPU cost: zero extra (71 — the
  bar only repaints when egui repaints).

## Recap

```
Ch 86: layers.
Ch 87: INTERROGATION. distance/angle/radius = 62's conversation template + kernel math (distance,
       Vector::angle, cross-product circumradius — NOT naive Heron, which cancels on shallow-arc
       needles — with a RELATIVE near-collinear refusal); every picked point arrives SNAPPED
       (72), which is what makes measurements exact rather than approximate. `what` = instant verb
       over kernel accessors (counts, area, length, degree). Status bar = one always-on panel row of
       three facts State already tracks: cursor world coords (work-plane-resolved when idle — don't
       run picking per move for a readout), active snap chip, selection count.
```

Edited: `app/commands.rs` (four verbs), `ui/mod.rs` (status row), `state.rs` (three UiState feeds).

## Next

`105-dev-toolbox.md` — the developer-experience lesson: a headless selftest, GPU errors that surface
in the viewer's own CLI, a black-screen checklist, and CI.
