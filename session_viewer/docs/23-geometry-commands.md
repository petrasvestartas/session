# 23 · The command line: one verb per file

A command is one file in `src/app/command/verbs/` plus one line in the `verbs!` list; nothing else in the viewer names it. This lesson builds that registry, typed coordinates, the drawing draft, the Tool trait for commands that ask for points, and the command dock.

![The command dock: typing `La` completes `Layers` in the field, and the list offers the match first, then every other verb alphabetically.](screenshots/command-completion.png)

## Step 1 · src/app/coords.rs

New file: a coordinate as typed, in its four forms, and the parser that reads one word.

`lessons/23/src/app/coords.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/coords.rs:coords-parse"
```

## Step 2 · src/app/coords.rs

Place a typed coordinate on the construction plane, measured from the previous point when it is relative.

`lessons/23/src/app/coords.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/coords.rs:coords-resolve"
```

## Step 3 · src/app/coords.rs

Tests: the four forms, words that are not coordinates, and a millimetre typed a kilometre out.

`lessons/23/src/app/coords.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/coords.rs:coords-tests"
```

## Step 4 · src/app/command/mod.rs

New file: the Action a parsed line becomes, the Spec each verb fills in, and the Verb trait REGISTRY holds.

`lessons/23/src/app/command/mod.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-action"
```

## Step 5 · src/app/command/mod.rs

Find the verb the first words spell, in any case and with or without spaces.

`lessons/23/src/app/command/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-lookup"
```

## Step 6 · src/app/command/mod.rs

Text for the dock: a line in its shown spelling, whether an option is being typed, the option label and the hint.

`lessons/23/src/app/command/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-text"
```

## Step 7 · src/app/command/mod.rs

Parse a line into an Action, and complete, browse and accept names and options.

`lessons/23/src/app/command/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-parse"
```

## Step 8 · src/app/command/mod.rs

Readers the verbs share: an offset, an axis and a number, a number alone, On or Off.

`lessons/23/src/app/command/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-arguments"
```

## Step 9 · src/app/command/mod.rs

Tests of completion, Title Case names and parsing; several name verbs of later lessons, so the whole module passes from lesson 37 on.

`lessons/23/src/app/command/mod.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/mod.rs:command-tests"
```

## Step 10 · src/app/command/verbs/mod.rs

New file: the verbs folder's shared module for drawing verbs.

`lessons/23/src/app/command/verbs/mod.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/mod.rs:verbs-modules"
```

## Step 11 · src/app/command/verbs/mod.rs

The `verbs!` macro: one name per verb becomes its `pub mod` line and its REGISTRY entry.

`lessons/23/src/app/command/verbs/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/verbs/mod.rs:verbs-macro"
```

## Step 12 · src/app/command/verbs/mod.rs

The list itself: this lesson's seventeen verbs, each on its own tagged line.

`lessons/23/src/app/command/verbs/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/verbs/mod.rs:verbs-list"
```

## Step 13 · src/app/command/verbs/geometry.rs

New file: Draw, the verb that turns points into one geometry, and its shared parser.

`lessons/23/src/app/command/verbs/geometry.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/geometry.rs:draw-verb"
```

## Step 14 · src/app/command/verbs/geometry.rs

The Create and Edit actions, and the helpers that add geometry as one undo step and select it.

`lessons/23/src/app/command/verbs/geometry.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/verbs/geometry.rs:draw-create"
```

## Step 15 · src/app/command/verbs/geometry.rs

Tests: Wedge, a drawing verb that exists only in tests, registers from its own file alone.

`lessons/23/src/app/command/verbs/geometry.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/verbs/geometry.rs:draw-tests"
```

## Step 16 · src/app/modeling.rs

New file: add geometry to the current layer, or a Created document, as one undo step.

`lessons/23/src/app/modeling.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/modeling.rs:modeling-create"
```

## Step 17 · src/app/modeling.rs

Trim or extend the selected line or curve to a part of its length, then close the impl.

`lessons/23/src/app/modeling.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/modeling.rs:modeling-interval"
```

## Step 18 · src/app/modeling.rs

Tests: creation undoes and redoes, several objects are one step, trim and extend keep the pen.

`lessons/23/src/app/modeling.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/modeling.rs:modeling-tests"
```

## Step 19 · src/app/command/verbs/point.rs

The first verb, typed in full: a Draw constant, its parser and its build function.

`lessons/23/src/app/command/verbs/point.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/point.rs"
```

## Step 20 · src/app/command/verbs/line.rs

Line: two points, refused when they are the same point; `segment` is shared with Arrow.

`lessons/23/src/app/command/verbs/line.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/line.rs"
```

## Step 21 · src/app/command/verbs/arrow.rs

Arrow: the Line segment with a head at its end.

`lessons/23/src/app/command/verbs/arrow.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/arrow.rs"
```

## Step 22 · src/app/command/verbs/polyline.rs

Polyline: two points or more, with Rectangle and Polygon constructions offered as buttons.

`lessons/23/src/app/command/verbs/polyline.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/polyline.rs"
```

## Step 23 · src/app/command/verbs/curve.rs

Curve: a NURBS curve on the control points, closed smoothly when it ends on its start.

`lessons/23/src/app/command/verbs/curve.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/curve.rs"
```

## Step 24 · src/app/command/verbs/close.rs

Close: join the polyline or curve being drawn back to its first point.

`lessons/23/src/app/command/verbs/close.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/close.rs"
```

## Step 25 · src/app/command/verbs/undo.rs

The first verb without points: a Spec, a parser and a unit struct that implements Action.

`lessons/23/src/app/command/verbs/undo.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/verbs/undo.rs"
```

## Step 26 · src/app/command/verbs/redo.rs

Redo: the same shape as Undo.

`lessons/23/src/app/command/verbs/redo.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/redo.rs"
```

## Step 27 · src/app/command/verbs/delete.rs

Delete: needs a selection, so a bare Delete first asks for objects.

`lessons/23/src/app/command/verbs/delete.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/delete.rs"
```

## Step 28 · src/app/command/verbs/hide.rs

Hide: hide the selection.

`lessons/23/src/app/command/verbs/hide.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/hide.rs"
```

## Step 29 · src/app/command/verbs/show.rs

Show: show everything hidden.

`lessons/23/src/app/command/verbs/show.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/show.rs"
```

## Step 30 · src/app/command/verbs/fit.rs

Fit: frame the selection, or the whole scene.

`lessons/23/src/app/command/verbs/fit.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/fit.rs"
```

## Step 31 · src/app/command/verbs/escape.rs

Escape: cancel the running command, else clear the selection.

`lessons/23/src/app/command/verbs/escape.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/escape.rs"
```

## Step 32 · src/app/command/verbs/explode.rs

Explode: a polyline into lines, a BRep or mesh into faces, a cloud into points, as one undo step.

`lessons/23/src/app/command/verbs/explode.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/explode.rs"
```

## Step 33 · src/app/session_io.rs

New file: write the scene as a `.session` file, read one back, and the browser download and file picker.

`lessons/23/src/app/session_io.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/session_io.rs"
```

## Step 34 · src/app/command/verbs/save.rs

Save: download the scene, once every released document is back.

`lessons/23/src/app/command/verbs/save.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/save.rs"
```

## Step 35 · src/app/command/verbs/open.rs

Open: ask the browser for a `.session` file.

`lessons/23/src/app/command/verbs/open.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/open.rs"
```

## Step 36 · src/state/hydrate.rs

Another `impl State` block: save once the released documents are back.

`lessons/23/src/state/hydrate.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/state/hydrate.rs:save-when-back"
```

## Step 37 · src/app/loader.rs

Hand an opened scene to the viewer as a message, like a loaded one.

`lessons/23/src/app/loader.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/loader.rs:install-saved"
```

## Step 38 · src/lib.rs

Replace the scene with the opened one and frame it.

`lessons/23/src/lib.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/lib.rs:open-saved"
```

## Step 39 · src/app/command/verbs/clipping_plane.rs

Clipping Plane: the command for the planes of [lesson 18b](18b-clipping.md), in the Title Case every name uses.

`lessons/23/src/app/command/verbs/clipping_plane.rs` · copy the file

```rust
--8<-- "lessons/23/src/app/command/verbs/clipping_plane.rs"
```

## Step 40 · src/state/clipping.rs

Another `impl State` block: size, pick, create, switch, flip and fill clipping planes for the verb.

`lessons/23/src/state/clipping.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/state/clipping.rs:clipping-verbs"
```

## Step 41 · src/app/command/tool.rs

New file: the tool module's imports; later lessons add their tool families here.

`lessons/23/src/app/command/tool.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/command/tool.rs:tool-modules"
```

## Step 42 · src/app/command/tool.rs

Next and the Tool trait: every hook a command that asks for points may answer.

`lessons/23/src/app/command/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/tool.rs:tool-trait"
```

## Step 43 · src/app/command/tool.rs

Strokes, squares and a label a tool draws over the scene, and two small helpers.

`lessons/23/src/app/command/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/command/tool.rs:tool-overlay"
```

## Step 44 · src/state/drawing.rs

New file: the Draft, the command being drawn, and `drafting`.

`lessons/23/src/state/drawing.rs` · type this, new file

```rust
--8<-- "lessons/23/src/state/drawing.rs:draft"
```

## Step 45 · src/state/drawing.rs

A line typed while drawing: a new drawing verb, a tool's word, an option, Sides N, Enter or coordinates.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-command"
```

## Step 46 · src/state/drawing.rs

Ask for points, close a shape, add typed coordinates, and finish by running the line as if typed.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-points"
```

## Step 47 · src/state/drawing.rs

The draft as JSON for tests, and the prompt the dock shows while drawing.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-prompt"
```

## Step 48 · src/state/drawing.rs

The cursor snaps within 12 pixels or lands on the plane; a click places the point.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-cursor"
```

## Step 49 · src/state/drawing.rs

The rubber band on screen and the buttons under the field, then close the impl.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-overlay"
```

## Step 50 · src/state/drawing.rs

A rectangle from two corners and a polygon from centre and corner, on the plane's axes.

`lessons/23/src/state/drawing.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-construction"
```

## Step 51 · src/state/drawing.rs

Tests: rectangle and polygon follow the construction plane.

`lessons/23/src/state/drawing.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/state/drawing.rs:drawing-tests"
```

## Step 52 · src/state/tool.rs

New file: start a tool on the selection or without one, and call it with the draft taken out.

`lessons/23/src/state/tool.rs` · type this, new file

```rust
--8<-- "lessons/23/src/state/tool.rs:tool-start"
```

## Step 53 · src/state/tool.rs

Pass clicks, picks, hovers and drags to the running tool, and ask for objects when nothing is selected.

`lessons/23/src/state/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/tool.rs:tool-events"
```

## Step 54 · src/state/tool.rs

Typed words go to the tool, then its answer decides: ask again, repeat, finish or refuse.

`lessons/23/src/state/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/tool.rs:tool-command"
```

## Step 55 · src/state/tool.rs

The tool's prompt and rubber band, its preview on the GPU only, and cancelling it.

`lessons/23/src/state/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/tool.rs:tool-show"
```

## Step 56 · src/state/tool.rs

A second `impl State` block with the hooks Esc, Enter and the pick call.

`lessons/23/src/state/tool.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/tool.rs:tool-hooks"
```

## Step 57 · src/state/edit.rs

Another `impl State` block: run one line, with the draft first and objects asked for when needed.

`lessons/23/src/state/edit.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/state/edit.rs:run-command"
```

## Step 58 · src/app/ui/command_line.rs

New file: what the dock remembers between frames, and its history of 200 lines.

`lessons/23/src/app/ui/command_line.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:command-line-state"
```

## Step 59 · src/app/ui/command_line.rs

The dock's Panel hooks: focus on a press, keys while open, and its state for tests.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:command-line-panel"
```

## Step 60 · src/app/ui/command_line.rs

Open `draw`: the folded row or resizable dock, the history, and the drawing buttons.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:dock-frame"
```

## Step 61 · src/app/ui/command_line.rs

The field's row: focus, the mouse wheel and arrow keys that browse, Enter and Tab.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:dock-input"
```

## Step 62 · src/app/ui/command_line.rs

Option buttons before the field, the field itself, and completion as the person types.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:dock-field"
```

## Step 63 · src/app/ui/command_line.rs

The completion list floating above the field, following the wheel and the arrow keys.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:dock-completions"
```

## Step 64 · src/app/ui/command_line.rs

Run a chosen completion or the line, Escape, the fold button, the snap bar, and close `draw`.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:dock-run"
```

## Step 65 · src/app/ui/command_line.rs

Put the caret at the end, count the characters a name spells, the grey placeholder, and select a range.

`lessons/23/src/app/ui/command_line.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:command-line-helpers"
```

## Step 66 · src/app/ui/command_line.rs

Tests: arrow keys cycle options, a space inside a name types on, browsing starts from the typed option.

`lessons/23/src/app/ui/command_line.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/command_line.rs:command-line-tests"
```

## Step 67 · src/app/ui/mod.rs

Another `impl Ui` block: run a typed line and remember it with its answer.

`lessons/23/src/app/ui/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/mod.rs:run-line"
```

## Step 68 · src/app/feedback.rs

Open or close the dock, and raise the phone keyboard.

`lessons/23/src/app/feedback.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/feedback.rs:command-line"
```

## Step 69 · src/app/agent.rs

New file: the hidden page input a phone keyboard types into, and its events.

`lessons/23/src/app/agent.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/agent.rs:agent-events"
```

## Step 70 · src/app/agent.rs

Listen to its typing, keys and composition, and send each as a message.

`lessons/23/src/app/agent.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/agent.rs:agent-listeners"
```

## Step 71 · src/app/agent.rs

Remove the listeners when the agent goes away.

`lessons/23/src/app/agent.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/agent.rs:agent-drop"
```

## Step 72 · src/app/agent.rs

Copy text into the input, raise or lower the keyboard, and ask whether a word is being composed.

`lessons/23/src/app/agent.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/agent.rs:agent-calls"
```

## Step 73 · src/app/ui/phone.rs

New file: which open field the phone keyboard types into.

`lessons/23/src/app/ui/phone.rs` · type this, new file

```rust
--8<-- "lessons/23/src/app/ui/phone.rs:phone-field"
```

## Step 74 · src/app/ui/phone.rs

Feed the hidden input's typing into that field, or the command line.

`lessons/23/src/app/ui/phone.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/phone.rs:phone-agent"
```

## Step 75 · src/app/ui/phone.rs

Type the hidden input's text into a field.

`lessons/23/src/app/ui/phone.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/phone.rs:phone-type-into"
```

## Step 76 · src/app/ui/phone.rs

Keep the hidden input on the field that has focus, then close the impl.

`lessons/23/src/app/ui/phone.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/app/ui/phone.rs:phone-follow"
```

## Step 77 · src/lib.rs

Another `impl App` block: listen to the hidden input and turn its keys into key presses.

`lessons/23/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23/src/lib.rs:phone-keys"
```

## Step 78 · registration lines

Copy the lines tagged with a lesson 23 tag from these files of `lessons/23/`:

- `src/app/mod.rs`: the `agent`, `command`, `coords`, `modeling` and `session_io` modules.
- `src/state.rs`: the `drawing` and `tool` modules, and the draft dropped when the scene is cleared (`register:commands`).
- `src/state/features.rs`: the `draft` field (`register:drawing`).
- `src/app/ui/mod.rs`: `command_line` in the `panels!` list, the `phone` module and fields, and the `run_line` and drawing-overlay calls.
- `src/app/keys.rs`: `:` opens the command line.
- `src/lib.rs`: the `Agent` and `SavedScene` messages, the `agent` field, `listen_agent`, and the two message arms.
- `src/app/input.rs`, `src/state/drag.rs`: a draft takes hovers and clicks before the scene does.
- `src/app/feedback.rs`, `src/app/ui/number_box.rs`, `src/state/number_box.rs`, `src/state/hydrate.rs`, `src/app/inspection.rs`: the status line, history, phone keyboard, Save resume and the `drawing` snapshot.

## Step 79 · tests

Copy these tests from `lessons/23/`; they are checked, not explained:

- `src/app/scene_sync.rs`: the `commands_tests` module at the end of the file.
- `src/app/layers.rs`: the `commands_tests` module at the end of the file.
- `src/engine/text.rs`: the `commands_tests` module, which checks the bundled fonts draw every command name.
- `src/app/clipping.rs`: the `editing_tests_23` module, a clipping plane undone and saved.
- `tests/command-workspace.cjs` and `tests/drawing-large-scene.cjs`: browser checks of the dock and of drawing in a large scene.

Run `cargo check` in `lessons/23/`.

## Check

`cargo check` compiles; `cargo xtest --lib coords` runs the coordinate tests. In `trunk serve`, press `:` and type `lin`: the dock completes `Line`; Enter, two clicks, and a line is drawn and selected. `Polyline Rectangle` draws from two corners, Undo removes it, and `poly line` or `POLYLINE` run the same verb.
