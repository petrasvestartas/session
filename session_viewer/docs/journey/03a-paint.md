# 03a · Paint command text over the scene

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–49 minutes.** 44 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Draw a Command label over the triangle using the same GPU.

**Follow:** RawInput → egui shapes and font atlas → GPU upload → Load pass → present.

Draw the Command label after the triangle and before presenting the image. `RawInput` gives egui the available canvas size. `run_ui` calls our layout closure and returns shapes plus changed font-atlas pixels.

Upload those pixels, turn the shapes into triangle jobs and update their GPU buffers. The second render pass uses `Load`, so it keeps the scene pixels already drawn. The panel uses a white frame and the production margins.

`forget_lifetime` removes the pass’s compile-time link to the encoder. The pass still must end before we finish the encoder. Submit its buffers, then release textures egui no longer needs.

![The current command drawing step.](../illustrations/journey-03a-paint.svg)

## Type the change

Continue from [Prepare the command fonts and painter](03a-fonts.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-03a-paint` (from `session_viewer`).

### 1. `src/panel.rs`

Lay out the label, upload its font atlas and draw over the scene.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { context, painter }
    }

}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03a-paint-01.rs"
```

### 2. `src/browser.rs`

Draw the label after the scene and before presentation.

<details>
<summary>Locate the existing block</summary>

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let _panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
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
--8<-- "journey/code/03a-paint-02.rs"
```

### 3. `src/browser.rs`

Draw the label after the scene and before presentation.

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
--8<-- "journey/code/03a-paint-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Command: appears at the bottom of the full-window canvas. The triangle remains unchanged above it. Keyboard input is not connected yet.

**Verified checkpoint in Chrome.**

![Actual browser result: Paint command text over the scene.](../screenshots/journey/03a-paint-browser.png)

[What this screenshot checks](release.md).

<details>
<summary>Optional experiment</summary>

Change only Load to a white Clear, rebuild and see which earlier drawing disappears. Restore Load.

</details>

## Explain the change

Why does the interface pass use Load?

<details>
<summary>Compare your explanation</summary>

The triangle is already in the target image. Load keeps those pixels while the interface draws over them; another Clear would erase the scene.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 03a-paint
npm --prefix ../session_tests run course -- save 03a-paint
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This introduces the interface upload and second GPU pass that the full command dock keeps.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome verifies visible Noto label ink, white panel pixels and exact preservation of the scene above the dock. Keyboard input is not connected.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 03a-paint
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
