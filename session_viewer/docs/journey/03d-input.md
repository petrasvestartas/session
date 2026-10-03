# 03d · Type into the real command dock

**Typing: 68–136 minutes.** [Estimate](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

Translate browser keys into egui events so the real dock accepts `Help` and Enter. A printable key first requests text focus, then supplies its text. Clicking the drawing releases focus. This preserves the first letter and creates no keyboard feature shortcuts.

Follow `KeyboardEvent → egui Text → CommandLine → layout`. Enter returns a submitted line; the vocabulary handles it and appends its answer. A consumed event belongs to the dock, so navigation must ignore it.

## Type

Continue from [Draw completion and history](03c-layout.md). [Save or recover your work](recovery.md).

### 1. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust
            top: f32::INFINITY,
            context,
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
                ..Default::default()
            },
            painter: egui_wgpu::Renderer::new(&renderer.device, format, Default::default()),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03d-input-scale-1.rs"
```

### 2. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust

    pub fn update(
        &mut self,
        _event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, JsValue> {
        let rect = canvas.get_bounding_client_rect();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03d-input-scale-2.rs"
```

### 3. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust
            focused: true,
            ..Default::default()
        };
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / size.x;
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point =
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03d-input-scale-3.rs"
```

### 4. `src/browser.rs`

Connect this checkpoint to the existing owners. The event callback retains Panel and Renderer for the lifetime of the page.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03d-input-window-1.rs"
```

### 5. `src/browser.rs`

Connect this checkpoint to the existing owners. The event callback retains Panel and Renderer for the lifetime of the page.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    report("The real dock lays out history and the command field.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03d-input-window-2.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Type `Help` and press Enter. The field clears and one reply appears in history. Click the canvas and type again: the first character must reach the command field.

**Verified checkpoint in Chrome.**

![Actual browser result: Type into the real command dock.](../screenshots/journey/03d-input-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

The second layout receives no event: it draws the cleared field and new answer. `FullOutput.append` retains earlier texture uploads with the latest shapes.

`Result<Option<String>, JsValue>` means an error, no submission, or one submitted line. `dyn_ref` borrows a matching event type. The `move` callback keeps its owners alive; canvas `tabindex` enables browser focus.

Browser → Panel → CommandLine → layout → existing GPU.

![Browser → Panel → CommandLine → layout → existing GPU.](../illustrations/journey-03d.svg)

Why do we lay out again without replaying the event?

Submitting a line changes the field and history after the first layout. An empty-input update draws that new state while processing the key exactly once.

Study estimate, including typing and experiments: 5–8 hours.

</details>

<details>
<summary>Optional experiment</summary>

Type an unknown word and press Enter. The dock should explain that it is unknown while leaving the triangle unchanged. Type Help again. Explain why input and document changes are separate.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 03d-input
npm --prefix ../session_tests run course -- save 03d-input
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This is the production command dock and styling. Its vocabulary grows with the course; the scene renderer stays independent of text editing.

Actual Chrome capture of this checkpoint. The result described above distinguishes drawing-only stages from connected input.

[Full validation scope](release.md).

</details>
