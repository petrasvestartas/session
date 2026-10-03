# 03a · Draw our command line

**Combined study estimate: 4–6 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 81–162 minutes.** 196 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Draw the production command panel and its Noto text over the triangle, using the same GPU.

**Follow:** Panel layout → shapes and font atlas → egui triangles → second GPU pass → one presented image.

Paint the command dock over the triangle using the same GPU. This step draws the field and history toggle; keyboard input is connected in 03d.

`theme.rs` supplies the production fonts and colours; `view.rs` supplies panel and field layout. `Panel` owns egui's context and GPU painter. Its context remembers interface state, and its font atlas stores letter shapes in a texture.

Each draw lays out widgets, uploads changed font pixels, tessellates shapes into triangles, then paints them. Use **Load** in the interface pass to preserve the scene underneath. Present only after both passes.

`String` owns the field text. `&mut self` permits updates; `|ui| ...` borrows it for layout. `Arc` shares embedded font bytes, and `zip` pairs names with their bytes. The dependency command supplies the binary fonts. One layout pass prevents later input events from being replayed.

![One GPU receives the scene pass followed by the egui interface pass.](../illustrations/journey-03a.svg)

## Type the change

Continue from [Give the GPU three corners](03-triangle.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-03a-panel` (from `session_viewer`).

### 1. `Cargo.toml`

Add egui for layout and egui-wgpu for painting. Both use the same wgpu 29.0.4 device as our scene; no second GPU connection is created. After this edit, run `npm --prefix ../session_tests run course -- dependencies 03a-panel` from `session_viewer`. It supplies the locked libraries and three binary fonts, without writing Rust code.

<details>
<summary>Locate the existing block</summary>

```toml
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

Replace that block with:

```toml
--8<-- "journey/code/03a-panel-panel-01.toml"
```

### 2. `src/command_dock/theme.rs`

Type the viewer’s actual font families and white theme. The main font draws letters; the two fallback fonts supply symbols. A static byte slice borrows bytes embedded in the program for its whole lifetime.

Create the file and type:

```rust
--8<-- "journey/code/03a-panel-panel-06.rs"
```

### 3. `src/command_dock/view.rs`

Type the production panel and field layout. Sizes are egui points. panel chooses folded or expanded dimensions; prepare sets text and the top rule; field draws the editable area. Its future input state is passed in, not stored in the style functions.

Create the file and type:

```rust
--8<-- "journey/code/03a-panel-panel-07.rs"
```

### 4. `src/command_dock/mod.rs`

Group the appearance code under one owner. pub(crate) lets the rest of this project use these modules without making them a public library API.

Create the file and type:

```rust
--8<-- "journey/code/03a-panel-panel-05.rs"
```

### 5. `src/panel.rs`

Keep the context and painter alive. Each draw lays out widgets, uploads changed font pixels, tessellates shapes, then paints with Load. forget_lifetime lets egui receive the render pass; finish the pass before finishing its encoder. The temporary pass still ends at the semicolon; forget_lifetime does not keep it alive forever.

Create the file and type:

```rust
--8<-- "journey/code/03a-panel-scale-1.rs"
```

### 6. `src/lib.rs`

Register the interface modules only for browser builds. Native scene checks continue to use the existing Renderer.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    });
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03a-panel-fullscreen-1.rs"
```

### 7. `src/browser.rs`

Create the interface once beside the scene renderer. Pass that same mutable Panel to present, then paint the interface after the scene and before presenting the frame.

<details>
<summary>Locate the existing block</summary>

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    present(&surface, &renderer)?;
    report("Three corners became a triangle.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03a-panel-window-1.rs"
```

### 8. `src/browser.rs`

Create the interface once beside the scene renderer. Pass that same mutable Panel to present, then paint the interface after the scene and before presenting the frame.

<details>
<summary>Locate the existing block</summary>

```rust
        ..Default::default()
    });
    renderer.draw(&view);
    frame.present();
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03a-panel-window-2.rs"
```

## Run and look

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 03a-panel
```

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The command dock appears over the triangle, using the supplied Noto fonts. Text input is not connected yet. Check that the drawing still fills the window.

**Verified checkpoint in Chrome.**

![Actual browser result: Draw our command line.](../screenshots/journey/03a-panel-browser.png)

[What this screenshot checks](release.md).

<details>
<summary>Optional experiment</summary>

Change only the interface pass from Load to Clear with a white colour. Predict what will happen to the triangle, then try it. Restore Load afterward. Which painter still drew correctly, and whose work did the later pass erase?

</details>

## Explain the change

Why does the interface pass load the existing image instead of clearing it?

<details>
<summary>Compare your explanation</summary>

The scene pass has already drawn the triangle. Load keeps those pixels. The interface pass draws its own triangles over them; another Clear would erase the scene.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 03a-panel
npm --prefix ../session_tests run course -- save 03a-panel
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This is the actual production panel and field styling, introduced before command behaviour. The next command-line parts connect input, submission, completion and history. We will keep these theme and layout functions as the application grows.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Actual Chrome capture of this typed checkpoint: the triangle and real command-panel styling share one canvas. This image proves the drawing step, not keyboard handling.

[Full validation scope](release.md).

</details>
