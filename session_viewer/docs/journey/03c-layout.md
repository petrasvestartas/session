# 03c · Draw completion and history

**Combined study estimate: 8–12 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 252–504 minutes.** 577 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Lay out the production command dock from its model.

**Follow:** Browser → Panel → CommandLine → layout → existing GPU.

Draw the dock's history, command field and completion list from `CommandLine`. A preloaded history sentence makes the expanded layout visible. Browser keyboard events arrive in the next lesson.

Follow one path first: Enter takes the line from the model and returns it to the caller. Then read completion: `browse` moves through suggestions, and `inline_suffix` selects the suggested ending so the next letter can replace it.

`Commands` is a trait listing the vocabulary operations the dock needs. The application implements it; layout can request suggestions without owning a document or camera. `&mut` lends the existing model to layout for updates. The return value requests canvas focus when editing ends.

![Browser → Panel → CommandLine → layout → existing GPU.](../illustrations/journey-03c.svg)

## Type the change

Continue from [Give the command line its memory](03b-state.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-03c-layout` (from `session_viewer`).

### 1. `src/command_dock/mod.rs`

Keep command text, history and completion state together. The application supplies vocabulary through Commands; this component owns editing and drawing.

<details>
<summary>Locate the existing block</summary>

```rust
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03c-layout-dock-06.rs"
```

### 2. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::command_dock::{CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            model: CommandLine {
                status: "The field now belongs to CommandLine.".into(),
                ..Default::default()
            },
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(
                        ui,
                        &mut self.model.command,
                        placeholder("", &self.model.status, false),
                        0.0,
                        false,
                    );
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter
                .update_texture(&renderer.device, &renderer.queue, *id, delta);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03c-layout-scale-1.rs"
```

### 3. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust
            &renderer.queue,
            &mut encoder,
            &jobs,
            &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03c-layout-scale-2.rs"
```

### 4. `src/panel.rs`

Keep one Panel alive beside the Renderer. Read the layout, input and painting paths separately; they communicate through the stored model and FullOutput.

<details>
<summary>Locate the existing block</summary>

```rust
            ..Default::default()
        });
        self.painter
            .render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03c-layout-scale-3.rs"
```

### 5. `src/browser.rs`

Connect this checkpoint to the existing owners. The event callback retains Panel and Renderer for the lifetime of the page.

<details>
<summary>Locate the existing block</summary>

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    report("The command model owns the text and history.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03c-layout-window-1.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The dock now draws its field, completion and history from the model. Resize the window: the dock should remain attached to the bottom of the drawing.

**Verified checkpoint in Chrome.**

![Actual browser result: Draw completion and history.](../screenshots/journey/03c-layout-browser.png)

[What this screenshot checks](release.md).

<details>
<summary>Optional experiment</summary>

Set command_expanded to false, run, and compare the folded panel. Restore true. The history remains in memory even when its layout is hidden.

</details>

## Explain the change

Who decides which commands exist?

<details>
<summary>Compare your explanation</summary>

The application’s Commands implementation supplies names and completion choices. The dock edits and displays text, then returns a submitted line. It does not create geometry.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 03c-layout
npm --prefix ../session_tests run course -- save 03c-layout
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This is the production command dock and styling. Its vocabulary grows with the course; the scene renderer stays independent of text editing.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Actual Chrome capture of this checkpoint. The result described above distinguishes drawing-only stages from connected input.

[Full validation scope](release.md).

</details>
