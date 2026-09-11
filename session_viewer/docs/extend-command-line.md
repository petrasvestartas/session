# Adding a command line

## The shape of the finished thing

- An `<input>` and a `<pre>` in `index.html`, a keydown listener in `src/app/input.rs`, two `Msg` variants, a table in a new `src/state/command.rs`, one named action per command on `State`.
- Commands are `pub fn` on `State` beside `hide_selected` (`src/state.rs`) and `escape_selection`; the typed line picks one, never mutates a document.
- Every editing command opens and closes a kernel transaction (`Session::begin`, `session_rust/src/session.rs`). No viewer-side undo stack: two stacks, two answers on undo.
- Rubber band and snap marker are a `SegmentLane`/`GlyphLane` pair in `Gpu`, not a bespoke renderer.

## The field is markup, not Rust-created DOM

```html
<div id="viewer-command">
 <pre id="viewer-command-log" aria-live="polite"></pre>
 <input id="viewer-command-input" autocomplete="off" autocapitalize="off" spellcheck="false">
</div>
```

- Beside `#viewer-status` (`index.html:54`); it and `#viewer-error` are markup Rust only *finds* (`feedback.rs:15`). Nothing under `src/` creates an element; one would put layout in a string literal.
- No `pointer-events:none` (the status line has it): the input must take clicks or the mouse can never focus it.
- `position:fixed; bottom:0`, z-index under `#viewer-docs`'s 10 (`index.html:34`): the folded-corner link stays reachable.
- Overlay the canvas, never shrink it (`100vw`/`100dvh`, `index.html:28`): shrinking fires `State::resize` (`src/state.rs`) per show/hide, re-uploading controls and re-targeting every lane.

## Keystrokes: the browser already arbitrates, so nothing needs a mode flag

```rust
WindowEvent::KeyboardInput { event,.. } => {
 viewer_focused
 && event.state == ElementState::Pressed
 && !event.repeat
 && self.input.key(state, event.logical_key.as_ref)
}
```

- `viewer_focused` (`src/lib.rs:238-249`) is `active_element.id == "canvas"`. With the field focused it is false, `Input::key` never runs, and `f`, `h`, `[`, `]`, `1`-`7` type characters.
- So `Input::key` (`src/app/input.rs`) needs no guard, boolean or mode enum: each is a weaker copy of the browser's decision.

## The listener owns the field; one message carries the line

```rust
#[cfg(target_arch = "wasm32")]
pub struct CommandLine {
 input: web_sys::HtmlInputElement,
 keydown: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::KeyboardEvent)>,
 history: std::rc::Rc<std::cell::RefCell<History>>, // lines, cursor, stashed buffer
}
impl CommandLine {
 pub fn new(proxy: EventLoopProxy<crate::Msg>) -> Result<Self, wasm_bindgen::JsValue>;
}
impl Drop for CommandLine { /* remove_event_listener_with_callback */ }
```

- Shaped on `PointerCancellation` (`src/app/input.rs`): own element and closure, detach in `Drop` (`input.rs`), so JavaScript cannot hold a handle into freed wasm memory.
- Install in `App::resumed` beside the pointer listener (`src/lib.rs`), another `proxy.clone`.
- Two `Msg` variants (`src/lib.rs`), two `user_event` arms:

```rust
Command(String),
CommandCancel,
Msg::Command(line) => state.command(&line),
Msg::CommandCancel => state.command_cancel,
```

- The picture follows by itself: `user_event` ends in `request_if_needed` (`src/lib.rs`), every named action in `State::touch` (`src/state.rs:242`).
- `State` lives inside the event loop (`src/lib.rs`), so a DOM callback's one path to it is the `EventLoopProxy` (`loader::post`).
- Not a `#[wasm_bindgen]` export: `reload_scene` is one because the embedding page calls it (`index.html:76`); the inline script stays lifetime wiring.

## Three web-sys features, or step 4 does not compile

- `Cargo.toml:24-43` has `Document`, `Window`, `Element`, `HtmlCanvasElement`, `EventTarget`, `Event`, and lacks:

```toml
"HtmlInputElement", #.value /.set_value
"KeyboardEvent", #.key /.prevent_default
"HtmlElement", # canvas.focus, handing focus back
```

- web-sys is gated per type: without the gate the type does not exist and the error lands far from the cause.

## Escape, in the order a user expects it

- The listener branches only on whether the field is empty, so it never asks Rust.
- Not empty: clear the field and send `Msg::CommandCancel`. Empty: send it and `canvas.focus`.
- The next Escape reaches winit and `escape_selection` (`src/app/input.rs`). Chain: text, tool, focus, selection; two presses at most to get out.
- `command_cancel` is idempotent because it has two senders; a double cancel shows only under a fast second press.

## One table, never three

```rust
struct Cmd {
 name: &'static str,
 aliases: &'static [&'static str],
 args: &'static str, // "[sx sy sz]" - shown by `help` and by completion
 run: fn(&mut State, &Args) -> Reply,
}
const CMDS: &[Cmd] = &[ /*... */ ];
fn lookup(verb: &str) -> Option<&'static Cmd>; // pure, natively testable
```

- Dispatcher, `help` and completion all read `CMDS`. The archive's three lists disagree already: `move`, `pt`, `ln`, `crv`, `cylinder`, `polyline`, `delete`, `rm` dispatch without completing; `help` omits `curve` and `move`.
- `lookup` is a free function over a const slice, so it tests in `cargo test` without a window.

## Tokenising

```rust
let line = line.trim;
if line.is_empty { return; }
self.log(&format!("> {line}"));
let mut parts = line.split_whitespace;
let verb = parts.next.unwrap.to_ascii_lowercase;
```

- Whitespace is the only separator; only the verb is lowercased, because guids and document names are case-sensitive everywhere else.
- No quoting or escaping, so a name with a space cannot be typed; state it on `help`. Revisit when a command takes a file path.
- Unknown verb: one line, nothing changes — `unknown: 'foo' (type 'help')`.

## Arguments: "missing" and "wrong" are different answers

```rust
enum ArgError { NotANumber(String), OutOfRange(String) }
fn f64_at(parts: &[&str], i: usize) -> Result<Option<f64>, ArgError>;
fn positive(name: &str, v: f64) -> Result<f64, ArgError>;
```

- `Ok(None)`: absent, default applies. `Err(_)`: not a number, so the command does nothing and says so.
- Cascading defaults stay: one argument to `box` is a cube (`sy`, `sz` default to `sx`); two is an error, since no reading of "two of three extents" is obviously right.
- Validate sign and finiteness in the arm: `Session::add_brep` refuses only zero faces *and* zero vertices (`session_rust/src/session.rs`), so a negative radius sails through.
- Report the kernel's refusals: `add_polyline` `None` below 2 points (`session.rs:1379`), `add_nurbscurve` below 2 CVs, `add_mesh` when empty or faceless.

## The command table

Selection-scoped commands act on `Scene::selected` (`src/app/scene.rs`) and report "nothing selected" rather than guessing.

### Document and history

| Command | Arguments | Kernel call | Records |
|---|---|---|---|
| `undo` | — | `Session::undo` (`session.rs:1622`) | cursor back |
| `redo` | — | `Session::redo` (`session.rs:1629`) | cursor forward |
| `hist` | — | `History::{depth, can_undo, can_redo}` (`history.rs:214-224`) | reads only |
| `save` | — | `file_json_dumps` / `pb_dumps` (`session.rs:581`, `605`) | clears history |

- `save` purges undo by design: both dumps call `history.clear` first and take `&mut self` (`session.rs:582`, `606`), so it cannot run through a shared `&Session` and the log line must say the history is gone.

### Solids

| Command | Arguments | Kernel call |
|---|---|---|
| `box` | `[sx [sy sz]]` | `BRep::create_box` (`session_rust/src/brep.rs`) then `Session::add_brep` (`session.rs:1454`) |
| `cyl` | `[r h]` | `BRep::create_cylinder` (`session_rust/src/brep.rs`) |
| `sphere` | `[r]` | `BRep::create_sphere` (`session_rust/src/brep.rs`) |
| `cone` | `[r h]` | `BRep::create_cone` (`session_rust/src/brep.rs`) |
| `pyramid` | `[base h]` | `BRep::create_pyramid` (`session_rust/src/brep.rs`) |
| `torus` | `[R r]` | `BRep::create_torus` (`session_rust/src/brep.rs`) |

- `create_cone`/`create_torus`, not `Primitives::*_surface`: the archive's `NurbsSurface` versions cost four special cases (delete, undo, redo, snap). A BRep walks the same tail as every other solid.

### Curves

| Command | Arguments | Kernel call |
|---|---|---|
| `line` | `[x0 y0 z0 x1 y1 z1]`, else interactive | `Line::from_points` (`line.rs:79`) then `Session::add_line` (`session.rs:1359`) |
| `poly` | interactive | `Polyline::new` (`polyline.rs:44`) then `Session::add_polyline` (`session.rs:1379`) |
| `circle` | `[r]` | `Primitives::circle` (`primitives.rs:108`) then `Session::add_nurbscurve` (`session.rs:1422`) |
| `ellipse` | `[a b]` | `Primitives::ellipse` (`primitives.rs:132`) |
| `arc` | interactive, three points | `Primitives::arc` (`primitives.rs:156`) |
| `curve` | `[deg]`, interactive control points | `NurbsCurve::create` (`nurbscurve.rs:259`) |
| `interpcrv` | interactive through-points | `NurbsCurve::create_interpolated` (`nurbscurve.rs:303`) |

### Surfaces and meshes

| Command | Arguments | Kernel call |
|---|---|---|
| `extrude` | `dx dy dz`, selected curve | `Primitives::create_extrusion` (`primitives.rs:822`) then `Session::add_nurbssurface` (`session.rs:1438`) |
| `loft` | `[deg_v]`, selected curves | `Primitives::create_loft` (`primitives.rs:1033`) |
| `revolve` | `[angle]`, selected curve | `Primitives::create_revolve` (`primitives.rs:1271`) |
| `mesh` | `[u v]`, selected surface | `Primitives::quad_mesh` (`primitives.rs:1946`) then `Session::add_mesh` (`session.rs:1411`) |
| `pipe` | `r`, selected line | `Primitives::cylinder_mesh` (`primitives.rs:392`) |

### Edits

| Command | Arguments | Kernel call | Records |
|---|---|---|---|
| `move` | `dx dy dz`, else interactive | `Xform::translation` (`xform.rs:139`) then `Session::set_xform` (`session.rs:346`) | `Op::Xform`, absolute before/after |
| `rotate` | `deg [ax ay az]` | `Xform::rotation` (`xform.rs:203`) | `Op::Xform` |
| `scale` | `factor` | `Xform::scale_uniform` (`xform.rs:913`) | `Op::Xform` |
| `orient` | interactive, plane to plane | `Xform::plane_to_plane` (`xform.rs:721`) | `Op::Xform` |
| `delete` | — | `Session::remove_object` (`session.rs:1575`) | `Op::Remove(Tombstone)` |
| `name` | `<text>` | `Session::replace` (`session.rs:1589`) | `Op::Replace`, absolute snapshots |

- `remove_object` empties the typed vector, `lookup`, the xform table, the tree node with its subtree and the graph node with its edges at once, and records the tombstone undo restores from (`session.rs:1575-1586`, `history.rs:46-58`). The archive removed three tables by hand and left tree and graph nodes dangling.
- `Session::replace` is the only recorded reshape; mutating through `lookup` compiles and is explicitly not recorded (`session.rs:1589-1604`), so undo cannot see it.

### Queries — read-only, no transaction

| Command | Arguments | Kernel call |
|---|---|---|
| `dist` | two picked points or objects | `Closest::{curve_point, polyline_point, mesh_point}` (`closest.rs:25`, `260`, `990`) |
| `xsect` | two selected objects | `intersection::{line_line, line_plane}` (`intersection.rs:94`, `198`) |
| `bbox` | — | `InstanceTable::row_bounds` (`src/engine/gpu/objects.rs`) |

- `bbox` reads the GPU table: `fit_selected_or_all` already reads `row_bounds` (`src/state.rs`), current after every upload.

### Commands that already exist — zero new code

| Command | Calls |
|---|---|
| `fit` | `State::fit_selected_or_all` (`src/state.rs`) |
| `hide` | `State::hide_selected` (`src/state.rs`) |
| `show` | `State::show_all` (`src/state.rs`) |
| `clear` | `State::clear` (`src/state.rs`) plus `history.clear` |
| `xray` | `State::toggle_xray` (`src/state.rs`) |
| `controls` | `State::enable_controls` (`src/state.rs`) |
| `names` | `State::toggle_selected_names` (`src/state.rs`) |
| `cloudsize` | `State::set_cloud_size` (`src/state.rs`) |
| `view` | `Camera::set_view` (`src/camera.rs`) |
| `esc` | `State::escape_selection` (`src/state.rs`) |
| `help` | `CMDS` itself |

- Ten commands before a kernel call is written: one function per action, reachable from a key, a typed line or anything later.

## The coordinate parser

- New `src/app/coords.rs`, `pub mod coords;` in the portable block of `src/app/mod.rs:5-16`; it names no `web_sys`, `wgpu` or `State`, so it tests natively.

```rust
pub enum Coord {
 Absolute([f64; 3]),
 Relative([f64; 3]),
 Polar { distance: f64, degrees: f64 },
 Distance(f64),
}
pub enum ParseError { Empty, BadNumber(String), BadTuple(String) }
pub fn parse(s: &str) -> Result<Coord, ParseError>;
```

- f64, not the archive's f32: every destination is f64 (`PickedPoint.position`, `src/app/scene.rs:108`; `InstanceTable.translation`) and the narrowing happens once, at the anchor.
- `Result`, not `Option`: only the parser can tell "you typed nothing" from "you typed `1,2,`".
- Check order is the grammar: trim, empty is `Empty`; `@` with a `<` is polar `distance<degrees`, else a relative tuple; a comma anywhere is an absolute tuple; a bare number is a distance.
- Consequences for the `help` line: `@5` is an error; `5` is always a distance, never a coordinate; a trailing comma is a parse failure, not an empty third field.
- A two-field tuple takes the third from the active frame's origin plane, not world zero: two numbers are typed on the plane being looked at.

### One basis for all four forms

- All four resolve in the active construction plane's frame. `cplane` sets it; unset it is world XY, so the rules coincide until the user changes it, and the prompt names the active frame.
- The archive uses world axes for `Absolute`/`Relative`, plane axes for `Polar`/`Distance`. In a front view `100,0` lands on world XY and nothing on screen says which basis ran, so the mixed rule is undiscoverable.

## Where a world point under the cursor comes from

- No unprojection exists today and the pick answer carries no position: `Pick { row: u32, sub: u32 }` (`src/engine/gpu/pick.rs:15-18`). A plane hit is new camera math.

```rust
/// A cursor ray in WORLD units, from the anchored view-projection's inverse.
pub fn ray(&self, cursor: (f64, f64), viewport: (f64, f64), aspect: f64, anchor: &Point)
 -> Option<(Point, Vector)>;
```

- In `src/camera.rs` beside `view_proj_anchored`: it inverts a matrix built there, and elsewhere duplicates the reverse-Z and unit conventions.
- Unit trap: `view_proj_anchored` works in metres (`self.unit.to_meters`, `src/camera.rs`), world positions in millimetres. Divide back, or the ray is off by a thousand and the hit lands past the far plane.
- `Xform::inverse` returns `Option<Xform>` (`session_rust/src/xform.rs`), so the signature is `Option` out: a degenerate projection is real right after a resize to zero.
- The hit itself is kernel math: a `Line` from the ray through `intersection::line_plane(&line, plane, false)` (`session_rust/src/intersection.rs`), the plane's projection of the ray origin as the parallel fallback.
- Do not port the archive's rule for *choosing* the plane (`ProjMode` plus a view-matrix column); re-derive from `set_view` (`src/camera.rs`) and `toggle_projection_framed`.

## Snap is CPU work, and picking is not

- The pick path answers *which object*: async, once per click, halo, generation guard (`src/state.rs:498-525`, applied atop `render`). Tools needing identity use it untouched.
- The snap answers *where, precisely*, every mouse-move. The id buffer carries no position, so it cannot answer that in principle, and a readback per mouse-move puts a GPU round trip in the cursor's path.
- Candidates come from `Scene::geometry(row)` (`src/app/scene.rs:525`) per document across `Scene::docs`. Gather once per tool session, only *project* per frame: `point_at` per mouse-move stalls the cursor in a dense scene.
- `session.world_xforms` once per document, never `world_xform` per object (as `Scene::add_file` does): the per-object call rescans the tree and is quadratic.
- `Scene::geometry` is `None` for streamed clouds, sheets and text rows, so they offer no candidates; say so on screen the first time a tool runs there.
- Aperture 8 CSS px, between the numbers around it: click slop 4 px (`src/app/input.rs:16`), snap marker 3.5 px (`src/state.rs:594`, `-3.5 * scale`, negative meaning screen-space). Below ~`3.5 + 4` the user aims at what the marker covers.
- Do not exclude the object being moved: it stays put until commit, and excluding it makes "move this corner onto that corner" impossible.

## A command with no arguments starts a tool

```rust
pub enum Tool { Idle, Point, Line, Polyline, Curve { degree: usize }, Move }
pub struct ToolState {
 tool: Tool,
 points: Vec<[f64; 3]>, // committed, world
 cursor: Option<Snap>,
 origins: Vec<(u32, Mat4)>, // Move: row and its placement at start
 dirty: bool,
}
```

- `Idle` doubles as "no tool", so nothing wraps it in an `Option`.
- While a tool is live every typed line goes to the tool, and the prompt says so: `box` during a polyline is a coordinate that fails to parse.
- Multi-point keywords (`c`/`close`, `u`/`undo`) match before the parser, only for many-point tools; otherwise `c` as a control point during `line` reads as "close" and the parser must know the tool.
- Empty Enter finishes an open polyline or curve, Escape cancels; both reach `command_cancel`/`tool_finish` from the listener.
- A bare distance as a tool's *first* input is an error, not a point: the archive falls back to plane origin plus x-axis, so `50` silently places (50, 0, 0).
- `Move` captures each row's placement before it starts, so cancel restores it exactly.

## The preview is its own lane pair

- Two fields on `Gpu` (`src/engine/gpu/mod.rs`), after `controls` and `control_net`:

```rust
pub preview: SegmentLane,
pub preview_marks: GlyphLane,
```

- Five one-line edits beside their `controls` twins: `allocated_bytes` (`src/engine/gpu/mod.rs`), construction, `retarget`, `reset`, `release`. Missing one leaks buffers across a scene change or leaves the lane on the wrong sample count after an MSAA flip.
- Two draw calls in `scene_list` (`src/engine/gpu/render.rs`), beside `control_net.draw_ribbons` and `controls.draw_dots`.
- Zero lines in the id pass (`render.rs:203-295`). `control_net` proves it is free: colour only, in no id pass. The preview is unpickable by construction, not by a filter.
- Do not reuse `controls`/`control_net`: `upload_controls` (`src/state.rs`) resets both unconditionally on every resize and selection change, so a shared rubber band vanishes mid-drag.
- Preview geometry is world-space and every ink lane composes `model[row]` with `anchored_translation[row]`, so it needs one row with identity model and zero translation:

```rust
scene.push_row(usize::MAX, "__preview__", Xform::identity.m, 0);
```

- `usize::MAX` as owner is the idiom for a row with no kernel object (`register_text`, `src/app/scene_text.rs`); `push_row` is `pub(super)`.
- Push it wherever rows are minted from empty — `Scene::new` and the tail of `reset_rows` — not at start-up alone and not in `Scene::rebuild` alone. `reset_rows` clears `order`, `owners` and `guid_to_row` on every rebuild, so a cached row goes stale; and `rebuild` runs on no ordinary load path (a document arrives through `State::append` → `add_file` → `upload_to`), so a row reserved only there does not exist until the first edit. Keep the number on `Scene` and re-read it after every rebuild.
- Colour it distinctly from the control net's `0xffcc8866` (`src/state.rs:606`).

## Reporting back

```rust
/// Append one line to the command scrollback. textContent only.
pub fn log(line: &str);
```

- Beside `status` in `src/app/feedback.rs`. `set_text_content`, never `set_inner_html`: the module doc states it (`feedback.rs`), both writers follow it, and a log echoing typed text is where the rule is load-bearing.
- `status` stays a one-line transient: "Select one object before pressing F10" (`src/state.rs`).
- Cap the scrollback at 200 lines, in one place. The `<pre>`'s text is replaced wholesale per write: 200 lines of ~80 characters is 16 KB rewritten, 20 000 lines 1.6 MB per logging keystroke.
- Three shapes: `> {line}` echoes input before dispatch, `+ box_0 (100x100x100 mm)` reports an add, a leading `? ` marks a parse failure.
- Prompts from a click have no return value, so the tool pushes them into the same capped function.

## How an editing command reaches the document history

```rust
fn edit<R>(&mut self, label: &str, f: impl FnOnce(&mut Session) -> R) -> Result<R, EditRefused>;
```

- In order: resolve the document, take a mutable session, `begin(label)`, run the closure, `commit`, rebuild or write the GPU row, reselect, `touch`.
- Label the transaction with the typed line: the label lives on the `Transaction` (`session_rust/src/history.rs`), so a history listing reads back as a command log for free.
- `Session::begin` commits any open transaction first (`history.rs:227-231`), so an early return cannot leave one open.

### Taking the session mutably

- The document holds `Rc<Session>` (`src/app/scene.rs`) and every mutator takes `&mut self`: the split is `Rc::make_mut`.
- Why: one manifest listing a file twice hands both documents the same `Rc`, so without the split, moving one placement moves the other and the live source's cached copy (`an_edit_must_split_a_session_two_placements_share`; `FileDoc` comment).
- The deep copy costs only when the `Rc` is shared; a loader-made document is uniquely owned.
- Refuse an edit on a live document rather than split it: `LiveSource` keeps its own `Rc<Session>` per URL (`src/app/live.rs`), so the count is at least two and the poller would overwrite the edit on its next tick. Say so on the log line.

### Which rows can be edited at all

```rust
/// The owning document index, or None when the row has no kernel object.
pub fn editable_doc(&self, row: u32) -> Option<usize>;
```

- Beside `hidden_rows` in `src/app/scene.rs`. `None` for streamed clouds, sheets and text rows, as `Scene::geometry` already is: `add_streamed_cloud` and `add_sheet` push a `FileDoc` with an empty `Session::new(&name)` shell, `display_only: true` (, `430-436`).
- Refuse a whole-scene rebuild while `scene.streamed` or `scene.sheets` is non-empty — "Streamed clouds and sheets cannot come back (no kernel object)" (, warnings at) — or an edit elsewhere silently discards a multi-million-point resident prefix.
- A sheet is one row for tens of thousands of segments (`push_row` once), so deleting the selection deletes the whole sheet; the entity is metadata (`SheetBatch.resolved`), not editable geometry.

### Keeping the selection across a rebuild

- `reset_rows` sets `selected = None` and clears `guid_to_row` (`src/app/scene.rs`), so a rebuild destroys every row number.
- Capture `Scene::identity_of(row)` first, re-resolve after through a new `Scene::row_of(&identity)` over the private `guid_to_row`.
- The hide set is the model: it survives because it stores `(document index, guid)`, and `add_file` re-reads it into `FLAG_HIDDEN` as rows return.
- Anything in `SelectionMode` but `Object` is dropped: an edge index into re-walked geometry is not the same edge, and a control-point id into a replaced object is not the same point.

## Move is the one edit that skips the rebuild

- No per-row placement writer exists: `append` alone writes `translation` and `rows[i].model`, `rebuild` rewrites the whole table, `set_flag` touches flags only (`src/engine/gpu/objects.rs`). Add one beside `set_flag`:

```rust
/// Rewrite one row's placement. `translate_only` writes the 16 B translation row alone.
pub fn set_place(&mut self, ctx: &GpuCtx, row: u32, place: &Mat4, translate_only: bool);
```

- A pure translate updates `translation[row]` in f64, re-anchors that one value against `last_origin`, writes one `[f32; 4]`: **16 bytes**. Rotate or scale also rewrites `rows[row].model` with columns 12/13/14 zeroed and writes one `Instance`, **96 bytes** by the const assert at `src/engine/gpu/instance.rs` — **112 bytes**, seven times a translate.
- Write the increment into the f64 base, never the f32 the shader reads: `rebuild` recomputes every row from `translation`, so a delta on the returned f32 is erased by the next re-anchor (`anchored` doc comment, `objects.rs`).
- Both writes must refresh `world_bounds[row]` and its bounded-row entry. Stale values break `fit_selected_or_all` (`src/state.rs`), `update_inside` and the selection box, and none of those failures points back at the move.
- Write the kernel side in the same transaction: `Session::set_xform(guid, xf)` (`session_rust/src/session.rs:346`), recorded as `Op::Xform` with absolute before and after (`history.rs:107-113`). GPU only and the next rebuild snaps it back; kernel only and nothing moves until something else rebuilds.
- Compose the GPU matrix as the walk does — the document's `place` times the session's *world* xform for that guid (the private free function `placement`, `src/app/scene.rs`, fed the map `session.world_xforms` returns) — because `set_xform` sets the **local** transform relative to the tree parent (`session.rs:345`). Expose it once as `Scene::placement_of(row)`.
- So `move` needs no rebuild: rows keep identity, the selection survives, pick tables stay valid. The cheapest edit, and the right one to build first on a large document.

## Testing, and a real limit

- `coords::parse`, `lookup` and the argument helpers are pure and test natively, with in-file `#[cfg(test)] mod tests` as `src/app/scene.rs` and `src/engine/gpu/objects.rs` do.
- `State::command` cannot: `State::new` takes an `Arc<Window>` and is built only in the wasm loader (`src/app/loader.rs`), while the native harness builds `Scene`, `Gpu`, `Camera` directly (`src/selftest/lifecycle.rs`).
- Publish `command_log`, `tool` and `history_depth` (`session_rust/src/history.rs:222`) in the `?inspect=1` snapshot (`src/app/inspection.rs:33-70`): `tests/interaction.cjs` and `tests/streamed-controls.cjs` drive the viewer through it.
- `docs/locator.py` refuses to run when a taught file matches no zone, and `src/app/coords.rs` matches none (`docs/locator.py:43-84`). Add the path to `ZONES` first.

## The order that compiles

1. **The parser alone.** `src/app/coords.rs`, `pub mod coords;`. Tests pass on both targets; nothing references it.
2. **Dispatch over actions that exist.** `mod command;` (`src/state.rs:18-20`), a `shell` field initialised in `State::new`, arms calling `fit_selected_or_all`, `hide_selected`, `show_all`, `clear`, `toggle_xray`, `enable_controls`, `escape_selection`, `set_cloud_size`; results to `self.status`.
3. **The log sink and the markup.** `feedback::log`, the three elements, `State::command` writing there.
4. **The field is live.** Three web-sys features, `CommandLine`, `Msg::Command`/`CommandCancel`, installed in `App::resumed`. Ten commands end to end; `f` in the field no longer fits the view, free from `src/lib.rs`.
5. **Edit targets and selection survival, no edits.** `Scene::row_of`, `Scene::editable_doc`, `reselect(identity)`.
6. **The first kernel edit: add and delete.** The `edit` wrapper, `Rc::make_mut`, `begin`/`commit`, `Scene::rebuild`, `camera.grow_extent(&gpu.bounds)`, reselect, `touch`; rebuild refused while `streamed` or `sheets` is non-empty.
7. **Undo and redo.** `Session::undo`/`redo` plus the same rebuild-and-reselect tail, `history.depth` on the log line.
8. **Move, skipping the rebuild.** `InstanceTable::set_place`, `Scene::placement_of`, `State::move_selected`.
9. **The preview lane pair, drawing nothing.** Two `Gpu` fields, five twin edits, two draw calls, zero id-pass lines, the reserved row in `Scene::rebuild`.
10. **The cursor ray and the snap.** `Camera::ray`, the hit through `intersection::line_plane`, candidates cached per tool session and projected per frame.
11. **The tool.** `ToolState`, `tool_text`, `tool_click`, `tool_cancel`; `line`, `poly`, interactive `move`.
12. **Observability.** The three inspection fields and `tests/command.cjs`, shaped on `tests/interaction.cjs`.

## What we take from the old viewer and what we do not

| Part | Verdict | Why |
|---|---|---|
| `coord_parser.rs`, 44 lines | **Ports**, f32 → f64 | Pure; destinations are f64, narrowing once at the anchor. |
| The parse/resolve split | **Ports** | `parse` decides the form, the caller the basis; testable without a GPU. |
| `ray_to_plane` | **Ports** | Kernel math; plane projection as the parallel fallback. |
| Multi-point keywords before the parser | **Ports** | Keeps a control point named `c` from reading as "close". |
| History cursor with a stashed live buffer | **Ports** | What users expect from an up-arrow. |
| Cancel being idempotent | **Ports** | Two senders reach it here too. |
| Snap gathered once, projected per frame | **Ports** | No `point_at` per mouse-move; the moved object stays a candidate. |
| `execute_command`'s dispatch shape | **Adapts** | Split, verb, match, unknown fallback kept; arms call named actions. |
| The command list | **Adapts** | Three disagreeing lists become one `CMDS`. |
| `ToolState` / `DrawTool` | **Adapts** | One field on `State`; the input layer never reaches a document. |
| The preview | **Adapts** | A reserved row in its own lane pair, not magic guids in the arena. |
| The snap engine | **Adapts** | Source is `Scene::geometry(row)`; non-resident rows give nothing. |
| Construction-plane *selection* | **Adapts** | Math ports; the plane comes from `set_view`/`toggle_projection_framed`. |
| Return-a-string-and-log-it | **Adapts** | Same contract, sink is `feedback::log`, `set_text_content` only. |
| `undo_state.rs` + `state_undo.rs`, 303 lines | **Replaced** | The kernel keeps transactions, tombstones, absolute pairs, a 64-deep cursor. |
| `p(parts, i, default)` | **Replaced** | Cannot fail: `box abc` builds a cube, `sphere -5` a negative radius. |
| `set_guid(name)` on cone, torus, curve | **Replaced** | Non-opaque guids; two `cone_3` collide. `add_*` mints its own. |
| `replace_or_push_nurbs` / `tree.add` bypass | **Replaced** | One bypass spawns four special cases. |
| `del`'s three-table removal | **Replaced** | Leaves tree and graph nodes dangling. |
| `clear` as written | **Replaced** | Keeps the undo stack, so the next undo replays dead guids. |
| `commit_object_transform`'s five branches | **Replaced** | One home for the pose; its baked cases need a geometry snapshot to undo. |
| The whole egui mechanism | **Replaced** | Its borrow dance solves a problem a DOM field does not have. |
| Focus retention and the Enter backstop | **Replaced** | Both exist because egui shares the viewport's keyboard queue. |
| Escape falling to `event_loop.exit` | **Replaced** | Quitting destroys the WebGPU device; Escape already clears the selection. |
| CPU raycasting for identity | **Replaced** | The id pass is better and built; only snap stays on the CPU. |
| `cmd_counter`, one counter per prefix | **Replaced** | `box_0, sphere_1, line_2` reads as a per-type index and is not one. |
| Viewer colours in kernel geometry | **Replaced** | Display concern: `ObjectRow::new` starts white, the walk tints. |
| `tree_ui.rs`, gumball, edit points | **Replaced** | Each is its own ownership question. |
| `fit` unioning `session.cached_boxes` | **Replaced** | `row_bounds` is current after every upload. |

## What to check on screen

- **Step 2.** Nothing visible; `cargo check` passes, `cargo test` runs the lookup.
- **Step 3.** The log box sits above the status line, wraps at phone width, does not eat canvas clicks.
- **Step 4.** In the field `f`, `h`, `1`-`7` type characters; on the canvas they move the camera. `fit` re-frames, up-arrow recalls the previous line, Escape clears the box, then returns focus, then the selection.
- **Step 5.** Nothing visible; `hide` then `show` returns the selection to the same object.
- **Step 6.** `box 100` puts a cube at the origin, log `+ box_0`; `delete` removes it; `box abc` changes nothing and says so; in a streamed scene it is refused with a reason and the cloud survives.
- **Step 7.** `box 100`, `delete`, `undo` brings it back selected, `redo` removes it. Ten rounds without drift; `hist` returns to zero.
- **Step 8.** `move 0 0 500` moves immediately, no reload, still selected; `F` frames it (world bounds); `undo` restores exactly; instant on a large document, unlike step 6.
- **Step 9.** Nothing visible, frame time unchanged, `?inspect=1` shows two lanes at zero rows.
- **Step 10.** The marker follows the cursor onto endpoints and midpoints within ~8 px, never a streamed cloud or sheet, and the log said why the first time.
- **Step 11.** `line`, click, `@100,0`: the second point lands 100 mm along the plane's x-axis and commits. Escape mid-tool leaves nothing; clicking the rubber band picks what is behind it; during `move` the original stays snappable until the second click.
- **Step 12.** `tests/command.cjs` types a line, reads `command_log`, `tool`, `history_depth` from `?inspect=1`, passes headless.
