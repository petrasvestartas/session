# Tutorial 13 · Command dock, completion, history and native CLI

## Goal

Mount the decoupled `command_dock` widget (`Commands` trait, inline completion suffix, option chips, 200-line history, mobile IME proxy) and ship the native `session-viewer-cli` binary (`exec`, `serve --stdio`, `schema`, `render-stage`).

## Diagram

![Mobile IME `agent.rs` + `command_dock::CommandLine` (`Commands` trait) and native `session-viewer-cli` converging on `CommandHost` (native wgpu readback of stage deck)](../../examples/stage/tutorials-systems/13-commands.png)

Stage deck source: [`13-commands.yaml`](../../examples/stage/tutorials-systems/13-commands.yaml).

## Prerequisites and files

Requires [Tutorial 6](tutorial-six-modeling.md) and [Tutorial 12](tutorial-twelve-visibility.md). Owns `src/command_dock/*`, `app/ui/{command_line*,phone}.rs`, `app/agent.rs`, `command/tests.rs`, 7 session verbs, `src/bin/session_viewer_cli.rs`, and `examples/course/dock.rs`.

Owned files in build order (`17` files):

1. `src/command_dock/mod.rs`
2. `src/command_dock/theme.rs`
3. `src/command_dock/view.rs`
4. `src/app/ui/command_line.rs`
5. `src/app/ui/command_line/tests.rs`
6. `src/app/ui/phone.rs`
7. `src/app/agent.rs`
8. `viewer-core/src/command/tests.rs`
9. `viewer-core/src/command/verbs/undo.rs`
10. `viewer-core/src/command/verbs/redo.rs`
11. `viewer-core/src/command/verbs/escape.rs`
12. `viewer-core/src/command/verbs/close.rs`
13. `viewer-core/src/command/verbs/report.rs`
14. `viewer-core/src/command/verbs/fit.rs`
15. `viewer-core/src/command/verbs/view.rs`
16. `src/bin/session-viewer-cli.rs`
17. `examples/course/dock.rs`

## Build

### Step 1 · `src/command_dock/mod.rs`

Create `src/command_dock/mod.rs` and declare `mod command_dock;` in `src/lib.rs`. `src/command_dock/mod.rs` separates persistent `CommandLine` state (`command` buffer, `200`-entry `VecDeque` history, character-indexed prefix completion, option buttons and focus flags) from the one-frame `Option<String>` returned when `Enter` or an option chip commits, while `Escape` cancels pending input without executing it: Spanning `663` lines, this module stays free of duplicate state by exposing `Commands`, `CommandLine`, `Control`, `record` without introducing UI/GPU coupling cycles. Spanning `663` lines, this module stays free of duplicate state by exposing `Commands`, `CommandLine`, `Control`, `record` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/command_dock/mod.rs"
```

### Step 2 · `src/command_dock/theme.rs`

`src/command_dock/theme.rs` registers `Noto Sans` plus both `Noto Sans Symbols` fallbacks across proportional and monospace families and sets `text_cursor.blink = false` so idle focused frames schedule zero repaints: Spanning `50` lines, this module stays free of duplicate state by exposing `fonts`, `visuals` without introducing UI/GPU coupling cycles. Spanning `50` lines, this module stays free of duplicate state by exposing `fonts`, `visuals` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/command_dock/theme.rs"
```

### Step 3 · `src/command_dock/view.rs`

`src/command_dock/view.rs` builds a `30 pt` folded bottom bar or a resizable `64..0.75 * height` history panel in logical points, drawing an explicit caret on empty focused fields and recording `Control` hit rectangles for deterministic browser automation. Spanning `61` lines, this module stays free of duplicate state by exposing `panel`, `prepare`, `field` without introducing UI/GPU coupling cycles. Spanning `61` lines, this module stays free of duplicate state by exposing `panel`, `prepare`, `field` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/command_dock/view.rs"
```

### Step 4 · `src/app/ui/command_line.rs`

`src/app/ui/command_line.rs` binds `CommandLine` to `REGISTRY` completions, active `Tool` prompts/chips and `Escape` priority, returning keyboard focus to the canvas after command execution. Spanning `140` lines, this module stays free of duplicate state by exposing `remember`, `Hooks` without introducing UI/GPU coupling cycles. Spanning `140` lines, this module stays free of duplicate state by exposing `remember`, `Hooks` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/command_line.rs"
```

### Step 5 · `src/app/ui/command_line/tests.rs`

`src/app/ui/command_line/tests.rs` drives character-by-character `egui` frame sequences verifying preserved spaces (`Orient 3 Points`), `Down`/`Up` option cycling (`Snap Off` -> `Snap End`) and `Enter` commit rules. Spanning `107` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `a_space_the_name_spells_types_on` (+2 more tests). Spanning `107` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `a_space_the_name_spells_types_on` (+2 more tests).

```rust
--8<-- "session_viewer/src/app/ui/command_line/tests.rs"
```

### Step 6 · `src/app/ui/phone.rs`

Routes `CommandAgent` text/keys to the topmost open panel field ahead of the dock, skipping inline completion suffixes and active IME compositions. Spanning `159` lines, this module stays free of duplicate state by exposing `agent`, `follow_field` without introducing UI/GPU coupling cycles. Spanning `159` lines, this module stays free of duplicate state by exposing `agent`, `follow_field` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/ui/phone.rs"
```

### Step 7 · `src/app/agent.rs`

`src/app/agent.rs` (`CommandAgent`) manages the hidden mobile `<input>`, forwarding full-string replacements, tracking IME `compositionstart`/`compositionend` so active compositions are never clobbered, and detaching all DOM closures on `Drop`. Spanning `201` lines, this module stays free of duplicate state by exposing `AgentEvent`, `CommandAgent`, `sync`, `edit` without introducing UI/GPU coupling cycles. Spanning `201` lines, this module stays free of duplicate state by exposing `AgentEvent`, `CommandAgent`, `sync`, `edit` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/agent.rs"
```

### Step 8 · `viewer-core/src/command/tests.rs`

Verifies sorted unique `REGISTRY` completions, readable labels, and identical `Action` parsing across aliases and canonical spellings. Spanning `566` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `every_verb_is_offered_in_alphabetical_order` (+21 more tests). Spanning `566` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `every_verb_is_offered_in_alphabetical_order` (+21 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/tests.rs"
```

### Step 9 · `viewer-core/src/command/verbs/undo.rs`

`undo.rs` and `redo.rs` step `Scene` history and run `after_history` (reporting empty stacks); `delete.rs` marks `requires_selection` and calls `delete_selection` without touching GPU buffers directly. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/undo.rs"
```

### Step 10 · `viewer-core/src/command/verbs/redo.rs`

`undo.rs` and `redo.rs` step `Scene` history and run `after_history` (reporting empty stacks); `delete.rs` marks `requires_selection` and calls `delete_selection` without touching GPU buffers directly. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `24` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/redo.rs"
```

### Step 11 · `viewer-core/src/command/verbs/escape.rs`

`hide.rs`, `show.rs` and `fit.rs` invoke `hide_selected`, `show_all` and `fit_selected_or_all`; `escape.rs` (`Esc`) preserves an active draft long enough to call `cancel_drawing` before falling back to `escape_selection`. Spanning `33` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `33` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/escape.rs"
```

### Step 12 · `viewer-core/src/command/verbs/close.rs`

Preserves the active `Draft` during dispatch and appends the start point via `close_drawing` to finish open polylines and curves. Spanning `31` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `31` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/close.rs"
```

### Step 13 · `viewer-core/src/command/verbs/report.rs`

`layers.rs` toggles/expands the hierarchy panel (`All`/`Off`); `report.rs` triggers JSON diagnostics export through the `feedback` boundary. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `23` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/report.rs"
```

### Step 14 · `viewer-core/src/command/verbs/fit.rs`

`hide.rs`, `show.rs` and `fit.rs` invoke `hide_selected`, `show_all` and `fit_selected_or_all`; `escape.rs` (`Esc`) preserves an active draft long enough to call `cancel_drawing` before falling back to `escape_selection`. Spanning `27` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `27` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/fit.rs"
```

### Step 15 · `viewer-core/src/command/verbs/view.rs`

Applies named orientations, perspective/orthographic switches, `Xray` and positive point/plane sizes without cancelling active drafts or splits. Spanning `209` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `209` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/view.rs"
```

### Step 16 · `src/bin/session-viewer-cli.rs`

`src/bin/session_viewer_cli.rs` builds the native `session-viewer-cli` binary supporting `exec`, persistent `serve --stdio` JSONL dispatch, headless PPM `render_frame` capture and Markdown schema generation. Spanning `366` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `366` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/bin/session-viewer-cli.rs"
```

### Step 17 · `examples/course/dock.rs`

`examples/course/dock.rs` runs the Tutorial 13 command-dock checkpoint (`?tutorial=dock`), exercising `command_dock::CommandLine` tab completion, inline suffixes, option chips, and 200-entry history over a standalone `Commands` vocabulary. Trap: gate single-key viewport shortcuts whenever the dock `TextEdit` holds keyboard focus so typing `f` or `h` enters characters instead of firing hotkeys.

```rust
--8<-- "session_viewer/examples/course/dock.rs"
```

## Integration

Cargo automatically discovers `src/bin/session-viewer-cli.rs` as the `session-viewer-cli` binary target without needing a `[[bin]]` override in `Cargo.toml` (so earlier checkpoints without `src/bin/` share the exact same `Cargo.toml`). Both the `egui` dock in `app/ui/command_line.rs` and `--cmd <line>` in `session-viewer-cli exec` call `viewer_core::command::parse` and `Action::run(&mut dyn CommandHost)`.

## Run and observe

Run `./target/x86_64-unknown-linux-gnu/debug/session-viewer-cli exec tests/fixtures/timber-floor.pb --cmd "Select All" --cmd "View Isometric" --render target/cli-floor.ppm --width 640 --height 480`: the CLI loads all `529` timber elements headlessly, switches to isometric projection, and writes a `640x480` WebGPU frame without opening a window.

## Predict and try

Predict what happens on a mobile browser when a user taps the `egui` command dock without `agent.rs` and `ui/phone.rs`: because a `<canvas>` is not an HTML text field, iOS/Android never raise the soft keyboard! Focusing the hidden offscreen DOM `<input>` in `agent.rs` raises the native mobile IME and forwards composition/backspace events into `CommandLine`.

## Question and answer

**Why does `src/command_dock/` define its own `pub trait Commands` instead of calling `viewer_core::command::completions` directly?**

**How to work it out:** Inspect `examples/course/dock.rs` (`?tutorial=dock`), which compiles at checkpoint 4 before `viewer-core`'s 86-verb `command` module and `Scene` exist.

**The answer:** Inverting the dependency through the 5-method `Commands` trait (`canonical`, `choosing_option`, `accept`, `completions`, `browse`) keeps `command_dock` 100% free of `crate::` and `viewer_core` imports, so the exact production `egui` widget runs unchanged against a 4-verb tutorial table in `dock.rs` and against the 86-verb `REGISTRY` in `app/ui/command_line.rs`.

## Validation

Verified via `cargo xtest -p viewer-core command::tests` (`15/15` alphabetical, title-case, alias, and option tests), `cargo xtest -p session_viewer app::ui::command_line::tests` (`3/3` `egui` keystroke tests), and `session-viewer-cli exec` / `schema --markdown`.
