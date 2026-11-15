# Enter commands

Type and complete commands, choose an option, and expand retained command history using the production command dock.

This is a draft cumulative candidate after [mathematics](completion-mathematics.md). Its new assembly has not yet compiled or passed browser acceptance.

## Prerequisites and ownership

Keep the device, event-loop launcher, canvas fixture, egui GPU painter and mathematics implementation. This subject owns the complete `src/command_dock` module: the persistent model and input behavior in `mod.rs`, panel and field rendering in `view.rs`, and fonts and visual settings in `theme.rs`.

The fixture adds a small vocabulary and connects the dock to egui-winit. Its `Hello`, `Color` and `Clear` commands exercise the dock without scene geometry. The later command subject supplies production operations through the same `Commands` interface. Phone keyboard bridging belongs to the later input and phone-layout subject.

## Build

Add the complete dock modules and its registration; retain the earlier implementations.

From `session_tests`, create a fresh comparison project:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint dock ../session_viewer/target/learn-dock
```

### Persistent model and one-frame output

Read [the command dock](../../src/command_dock/mod.rs). `CommandLine` persists text, focus requests, completion selection, history and the rectangles used for pointer interaction. It is separate from the optional command returned by one draw. Drawing a frame must not execute a command repeatedly; the owner consumes that one-frame result after egui finishes.

`Commands` supplies canonical spelling, option detection, acceptance and completion candidates. The dock therefore owns text interaction while the application owns what a command means. The fixture implements that boundary directly:

```rust
--8<-- "session_viewer/examples/course/dock.rs:course-dock"
```

`take_command` transfers the owned string out of the model using `mem::take`, leaving an empty string in its place. It rejects whitespace-only input. `remember` keeps at most 200 history entries by removing the oldest before appending. `VecDeque` supports those opposite-end operations without shifting every retained line.

### Focus, completion and cancellation

Follow `focus_command` in `mod.rs`. A requested focus transfer is consumed once; the field then owns its ordinary egui focus state. Re-requesting focus every frame would prevent another widget from keeping it. The command-open flag and current response distinguish a live text edit from an inactive field.

Completion retains both a prefix and a highlighted index. Keeping the prefix separate from the displayed inline suffix lets browsing change the proposed completion without treating every proposal as newly typed input. Cursor operations use character indices, not UTF-8 byte offsets. `spelled` counts non-space characters when matching a compact command name to its displayed spelling.

Enter asks the vocabulary whether the line can run. An incomplete command can keep its canonical text and open an option list instead. Option buttons produce the same command output as accepted text. Escape clears the pending line and completion state, releases text focus, and returns focus to the canvas when cancellation is allowed. Another active editor can reserve Escape by passing `escape_allowed = false`.

The fixture forwards each real window event through egui-winit, takes one `RawInput` snapshot for the frame, and returns egui's platform output to the adapter afterwards. Platform output includes cursor and text-input requests; dropping it would make a widget appear drawn while parts of its browser interaction failed. The painter's existing raw-input and UI callback boundary supports this without changing GPU upload or drawing code.

### Panel, history and fonts

Read [the view functions](../../src/command_dock/view.rs). The folded panel has a fixed height; expanded history uses a resizable bottom panel with bounded height. The history area sticks to its newest entry and leaves room for the input row. Keeping these rectangles in logical points makes the same pointer positions work at different display densities.

Read [the theme](../../src/command_dock/theme.rs). The font list starts with Noto Sans and adds the two symbol fallbacks for both proportional and monospace text. The white visual theme explicitly sets selection, cursor and widget colours. Cursor blinking is disabled because the production interface renders on demand; a static focused field does not require an endless animation loop.

The `Control` records emitted during drawing identify the actual widget rectangles. The fixture exposes those records for browser checks, so a test clicks the rendered control rather than guessing a coordinate that only works at one viewport size.

## Run and observe

Serve the assembled project through the workspace storage guard and build slot with `trunk serve --port 8780`. Open `http://127.0.0.1:8780/?tutorial=dock`.

Expect a pink circle, an instruction in the centre, and a command field along the bottom. Type `Hello` and press Enter: the centre should say `Hello!`. Expand history with `+` to see the accepted line. Type `Color` and press Enter to reveal its options, then choose Blue: the circle should turn blue and the accepted colour command should enter history. `Clear` empties that history. Escape must cancel an unfinished entry without executing it.

Reload the page with `?tutorial=mathematics`, `?tutorial=painting` and `?tutorial=frame` to exercise the retained earlier registrations. Browser emulation can check the narrow layout and pointer targets; it does not establish physical-phone virtual-keyboard behavior.

## Predict and try

Type a prefix such as `He`, then browse its completion before accepting it. Predict whether the stored command should remain the prefix or become the full accepted word. After accepting, expand history: it should contain the canonical command once, regardless of how many frames were drawn while choosing it.

## Question and answer

Why does the dock return an optional command instead of executing it inside the text field?

The application owns command effects, and egui can draw the field again without new input. Returning a one-frame value keeps execution separate from rendering and lets the application apply the accepted command exactly once.

## Validation

The 45-file partial project passes its locked wasm Cargo check. Three source-retention tests verify unchanged earlier implementations and the actual three production dock files. The browser check enters Hello, chooses Color/Blue with the actual option control, verifies blue circle pixels and clears history at desktop and upright dimensions. The native candidate suite passes 19 tests with one GPU-dependent test ignored; the separate historical focus regressions retain their original scope.

The [current interaction receipt](release/completion-partials-windows-20261115.json) records Chrome 157 and the Intel Gen-12LP adapter. Upright viewport emulation is not a physical-phone test, and these checks do not accept the remaining cumulative course.
