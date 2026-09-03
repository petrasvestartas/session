# CAD Sketcher — Implementation Plan

A Rhino-style command-line + click/snap CAD sketcher built on the `session` geometry
kernel. This is the standalone handoff doc — read it fresh in a future session.

---

## STATUS (2026-05-29): single-view direction adopted — two-pane shell below is SUPERSEDED

The "two panes + eframe" shell described later in this doc was **dropped**. Drawing now
happens **directly in the existing single 3D view** (no new windows; switch named views
T/B/L/R as needed). The CAD *core* design below (snap engine, getpoint loop, command
state machine, coordinate parser, CPlane math, snap priority) is still the model and was
implemented — only the shell changed.

What is implemented (Phase 1 + 2), all as plain modules inside `session_viewer/src`
(no separate `session_cad` crate):

- `cad_plane.rs` — construction plane follows the active view (ortho forward axis →
  XY/YZ/XZ, perspective → XY); ray→plane via `intersection::line_plane(.., false)`.
- `coord_parser.rs` — `x,y[,z]` / `@dx,dy[,dz]` / `@dist<ang` / bare distance.
- `tool_state.rs` — `DrawTool {Idle,Point,Line,Polyline}` + `ToolState`.
- `snap.rs` — `SnapKind`/`SnapModes`/`SnapResult` + candidate gather (End/Mid/Vertex/Near)
  + screen-pixel ranking by priority. (Intersection/Knot variants exist; emitted in Ph3.)
- `state_tool.rs` — `impl State`: tool_on_click / tool_on_text / tool_on_move, compute_snap,
  rubber-band preview into the `line` arena (`__draw_preview__`), commit→session+gpu+undo.

Wiring: `state_update.rs` routes viewport clicks to the tool before `process_pick`;
`state_interaction.rs` routes hover + Esc-cancel; `state_cmd.rs` makes `point`/`line`/
`poly` with no args start the interactive tool (args keep the old immediate behavior);
`state_ui.rs` routes typed input to the tool, draws the snap marker, handles Enter=finish.

Done: Ph3 — intersection snap (pairwise segment `line_line`, ✕ marker) + NURBS knot snap
(`get_nurbsknots`/`point_at`, ◇ marker) in `snap.rs`; both enabled in `SnapModes::default_on`.
Done: Ph4 — interactive `curve [deg]` tool (`DrawTool::NurbsCurve{degree}`): click/type control
points, Enter=finish, u=undo, Esc=cancel; commits `NurbsCurve::create(false, deg, pts)` into
`objects.nurbscurves` with `UndoAction::AddNurbsCurve`; `del` + its undo extended to curves.
Also: CLI box keeps focus while a tool is active (seamless click-then-type).
Roadmap: Ph5 NURBS surface creation. Known gaps: curve preview is the control polygon (straight),
not the evaluated smooth curve; gumball *transform* of a standalone NurbsCurve isn't baked yet.

---

## Goal

Draw 2D/3D CAD geometry interactively:
- **Command line** — type a command (`polyline`, `point`, `nurbssurface`, …), then enter
  points by typing coordinates *or* by clicking on screen.
- **Construction plane (CPlane)** — clicks resolve onto the active plane (currently XY).
- **Object snapping (osnaps)** — snap the cursor to meaningful points on existing
  geometry: End, Vertex, Mid, Center, Near, Intersection, Perp, Tangent, and **Knot of a
  NurbsCurve / NurbsSurface**.

The hard part is **not** the renderer — it's the CAD logic (snap engine + command state
machine + CPlane math), which is pure geometry and renderer-agnostic. So the renderer is
chosen for least friction, and the effort goes into the logic.

---

## Crate structure (decided)

Split into two layers — separate the reusable core *now*, keep the prototype shell:

- **`session_cad`** — NEW pure-Rust **library crate**. The reusable, production-grade core:
  cplane, snap engine, getpoint, command state machine, coordinate parser.
  Depends **only on `session_rust`** (the kernel). **No egui, no wgpu.**
- **`session_viewer`** — the prototype **shell** (egui + wgpu panes, rendering, input
  routing). Depends on `session_cad`.

One Cargo workspace, two members for now. `session_cad` can be promoted to its own
repo/submodule later with **zero code change**, because the dependency boundary is clean
from day one (the compiler enforces "no UI in the core").

Rationale:
1. The boundary is nearly free at the start, painful to retrofit once egui/wgpu types leak
   into snap code.
2. The core is the valuable, reusable IP — already renderer-agnostic (pure geometry + a
   `Projector` trait that is just matrix math). A future production UI (Vello, web/WASM, a
   different shell) reuses it unchanged.
3. The snap/command logic is headless-testable (no GPU).
4. Don't over-engineer — only the *core* moves out; the prototype stays in `session_viewer`.

---

## App architecture (decided)

- **One OS window; egui is the shell.** Layout = two panes + command bar:
  - **2D sketch pane** — `egui::Painter`, orthographic top view locked to the CPlane,
    pan/zoom, no orbit.
  - **3D pane** — the existing wgpu scene + GPU picking, embedded via an `egui_wgpu`
    paint callback into the pane's rect; egui Painter draws feedback on top.
  - **Command bar** (`egui::TextEdit`, bottom) + snap-toggle panel (side).
- **One shared `Session` document** feeds both panes. Nothing is "transferred" — anything
  drawn in either pane already exists in both.
- **Draw + snap in BOTH panes.**

```
┌──────────────┬──────────────┐
│  2D SKETCH   │   3D VIEW    │   one window
│  egui Painter│  wgpu+pick   │   one shared Session
│  ortho,pan/z │  orbit       │   draw+snap in EITHER pane
├──────────────┴──────────────┤
│ Command: _polyline   [snaps]│
└─────────────────────────────┘
```

---

## Key idea: one snap engine, two projectors

The snap engine produces **world-space** candidates and ranks them by **pixel distance in
the active pane**. The only per-pane difference is the world→pixel projection:

- 2D pane → `Ortho2D` (pan + zoom affine on the CPlane).
- 3D pane → `Camera3D` (existing view-proj into the pane rect — matrix-only, no wgpu in core).

So `SnapContext` carries a `projector` + cursor-in-pane-pixels + aperture; candidate
generation and ranking are identical for both panes.

---

## `session_cad` modules (pure Rust, view-agnostic)

| File | Responsibility |
|---|---|
| `cplane.rs` | Construction plane (XY now, but plane is an argument → any plane later). 2D-pane click → ortho inverse → world; 3D-pane click → ray → `Intersection::line_plane`; grid snap via `Plane::closest_point`. |
| `snap.rs` | **Key layer.** `trait SnapProvider`; `SnapContext{cursor_px, projector, reference, enabled, aperture_px}`; `SnapCandidate{world, kind, screen_dist_px}`; `SnapKind`; `SnapModes` bitflags; ranking. `trait Projector { world→pane_pixels }` with matrix-based impls. Per-geometry-type providers (point/line/polyline/nurbscurve/nurbssurface/mesh). |
| `getpoint.rs` | The interactive point resolver. Merges typed input + active-pane cursor + snap → one `Point`. Coordinate parser: absolute `x,y[,z]`, relative `@dx,dy`, polar `@dist<ang`, bare distance. Typing overrides mouse; snap overrides the bare plane. |
| `command.rs` | `Command` trait + state machine sequencing prompts (`GetPoint`/`GetNumber`/`GetOption`). Pane-agnostic — the active pane supplies the projector + cursor. Commands: `Point`, `Polyline` first → later `NurbsCurve`, `NurbsSurface`. |
| `parser.rs` | Coordinate/command tokenizer (command name vs coordinate vs option keyword vs bare number vs Enter/Esc). |

---

## The snap engine (the algorithm)

Per frame while a `GetPoint` is active, in whichever pane the pointer is over:

1. Cursor → pick info: 2D pane = inverse ortho; 3D pane = camera unproject ray.
2. Narrow to objects within the **aperture** (~12 px): 3D via the existing BVH, 2D via
   projected bbox.
3. Each candidate object's `SnapProvider` emits candidates for the **enabled** snap modes.
4. Project each candidate's 3D point → pane pixels (via the pane's `Projector`); keep those
   inside the aperture.
5. Rank by **(priority class, then pixel distance)**. Discrete/derived snaps outrank Near.
6. Winner → resolved point + glyph + tooltip. Fallback if nothing hit: Grid snap → bare
   CPlane projection.

### Three snap families + kernel backing (confirmed via session-api MCP)

| Family | Snaps | Kernel calls |
|---|---|---|
| **Discrete** (cheap, cache per object until it moves) | End, Vertex, Mid, Center, **Knot**, Quad | polyline verts / segment mids; curve eval at t0/tmid/t1; **`GeometricKnots::from_nurbs_curve(curve).points()`** and **`GeometricKnots::from_nurbs_surface(surface).points()`** |
| **Continuous** (computed live each frame) | Near, Perp, Tangent | `*.closest_point` / `closest_point_with_param` (returns the param too) on Line/Polyline/NurbsCurve/NurbsSurface/Mesh/Brep |
| **Derived** (pairwise, aperture-limited) | Int | `Intersection::line_line / polyline_polyline / curve_curve / segment_segment` |

CPlane click resolution uses `Intersection::line_plane(ray, cplane)`. Plane projection /
grid snap uses `Plane::closest_point`.

> `GeometricKnots` is purpose-built for the knot-snap requirement — it returns the knot
> points of a curve or surface directly.

### Snap priority (highest → lowest, Rhino-like)
Int / End / Knot / Mid / Center / Quad / Perp / Tangent / Vertex → then **Near** (the
sliding catch-all) → Grid → bare CPlane.

---

## `session_viewer` responsibilities (prototype shell)

- egui shell: two panes, command bar (`egui::TextEdit`), snap-toggle panel.
- Embed the wgpu scene in the 3D pane via `egui_wgpu::CallbackTrait` (the scene becomes a
  render callback into a sub-rect instead of owning the full surface).
- Implement the core `Projector` trait: `Ortho2D` from pan/zoom; `Camera3D` from
  `camera.rs` view-proj + viewport rect.
- Input routing: the hovered pane sets the active `Projector` for GetPoint/snap; GPU
  picking runs only when the 3D pane is hovered and egui isn't capturing the pointer
  (`ctx.wants_pointer_input()`); selection highlight is synced across both panes (same
  `Session`).
- Rendering:
  - **2D pane** (egui Painter): committed geometry projected via `Ortho2D` (points/
    polylines direct; meshes as edges/tris; nurbs tessellated to polylines) + transient
    feedback.
  - **3D pane** (wgpu via callback): committed geometry through existing line/point/mesh
    pipelines; remains GPU-pickable. egui Painter draws feedback over the pane rect.
  - **Both panes:** snap glyphs (□ End, △ Mid, ○ Center, ✕ Int, ◇ Knot, ⊥ Perp),
    rubber-band preview, coordinate readout.

### Existing pieces to reuse
`camera.rs` (unproject), `state_picking.rs` + BVH (aperture narrowing), `cad/grid.rs`
(CPlane/grid), `commands.rs` (extend), and the gumball click-to-type popup (proves
screen-space text + input already exist).

---

## Build order (incremental)

1. **egui shell + two panes + shared render.** Adopt `eframe` (or manual `egui-winit` +
   `egui-wgpu`); move the current full-window wgpu viewer into the 3D pane callback; 2D
   pane renders the `Session` with Painter (ortho pan/zoom); add the bottom command bar.
   *(Biggest structural step — the wgpu scene goes from owning the surface to rendering
   into a sub-rect.)*
2. **CPlane point input in both panes.** 2D click → ortho inverse → world; 3D click → ray
   → `line_plane(XY)`; type `x,y[,z]` in the command bar; draw a marker in both panes.
3. **GetPoint + command state machine.** Ship `Point` & `Polyline` with rubber-band in the
   active pane, commit to the shared `Session` (appears in both panes), `Close`/`Undo`/
   Esc/Enter.
4. **Discrete snaps.** End / Vertex / Mid / Center / **Knot** via the shared engine +
   per-pane projector + egui glyphs + snap-toggle UI. Works in both panes.
5. **Continuous + derived snaps.** Near (closest_point along ray / 2D), Int (pairwise),
   Perp/Tangent vs the reference point. 3D-aware in the 3D pane.
6. **NurbsCurve / NurbsSurface creation.** Control points via repeated GetPoint →
   `NurbsCurve::create` / surface from a control grid. Knot snapping then "just works" on
   the results.
7. **Polish.** Relative/polar input (`@`), ortho/distance constraints, grid snap, command
   history, command-name autocomplete, persistent snap settings, cross-pane selection
   highlight.

---

## To confirm before step 1

- **eframe vs manual egui-wgpu** for embedding the wgpu scene in a pane. Recommendation:
  **eframe** (egui's app framework owns winit + wgpu and hands you the device/queue; render
  the 3D scene in an `egui_wgpu::CallbackTrait`) for the least boilerplate. The alternative
  (manual `egui-winit` + `egui-wgpu` on the current custom winit loop) keeps the existing
  setup but needs more wiring (surface config, resize, event routing).

---

## Notes / non-goals (for now)

- **Direct draw + snapping only** — no geometric constraint solver (coincident, parallel,
  dimensions). If a parametric sketcher is wanted later, a `planegcs` Rust/WASM port is the
  path, added as a separate concern.
- **Geometry rendering** uses the existing wgpu pipelines (3D) and egui Painter (2D). Lyon
  (tessellation through the wgpu pipeline) or Vello (GPU vector engine) are optional future
  upgrades for crisper 2D — they don't touch the CAD core.
- The core stays **100% UI-free** so it remains reusable: it returns world-space points and
  `SnapCandidate`s; the viewer decides how to draw glyphs and geometry.
