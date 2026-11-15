# Paint shapes and text

Draw a pink triangle and a black `Command:` label with egui over the white GPU frame.

This candidate follows [Open a GPU frame](completion-frame.md). Its partial project passes wasm typechecking and desktop/upright browser checks; complete subject acceptance remains pending.

## Prerequisites and ownership

Keep the preceding device setup, launcher, canvas fixture and clear helper. This subject adds `src/engine/gpu/ui.rs`, the `painting` registration and `examples/course/painting.rs`. The painted command label is text, not an input widget. The later dock subject owns editing and focus.

## Build

Add the production GPU painter and the painting experiment without replacing the preceding files.

From `session_tests`, assemble a fresh comparison project:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint painting ../session_viewer/target/learn-painting
```

### Points, pixels and primitives

The egui context owns layout state and font data. The painter emits shapes in logical points. Dividing the surface's pixel dimensions by the scale factor produces the logical rectangle; passing that same factor into egui keeps rasterization consistent with the display density. Increasing density should sharpen the image without doubling the triangle's apparent size.

```rust
--8<-- "session_viewer/examples/course/painting.rs:course-painting"
```

`Painter::new` installs the repository's Noto Sans subset because this dependency configuration supplies no default fonts. It permits one layout pass per frame, matching the production setting. `draw` receives raw input, supplies the logical screen rectangle and time in seconds, then runs a closure with the root egui UI. This experiment passes empty input and uses the UI's painter; the dock subject can supply events and create widgets through the same interface. The closure returns whether animation requires another frame; a static triangle returns false.

The triangle vertices are offsets from `screen.center()`. `convex_polygon` fills their interior in order. The label uses a baseline alignment near the bottom-left corner, so changing the window height moves it with the bottom edge. Both shapes use the same context and rendering pass.

### Upload and lifetime

Read [the production GPU painter](../../src/engine/gpu/ui.rs). `Ui` owns the egui renderer, clipped primitives, screen descriptor and texture lifetime records. Its constructor accepts only a device and target format; no scene pipeline or object rows are prerequisites.

`prepare` frees textures retired by the previous frame, uploads the current texture deltas, and tessellates shapes into clipped triangles. The returned texture-free list is saved for the next frame. This delay keeps a texture alive while the current frame still references it. Vertex and index buffers are updated before the draw pass uses them; callback command buffers are submitted together with the upload encoder.

`draw` begins a pass with `LoadOp::Load`, preserving the white background already cleared by the frame helper. The renderer applies clipping and draws the tessellated primitives. `StoreOp::Store` retains the composite for presentation. `Drop` releases the renderer's remaining textures when this owner ends. It does not acquire a second surface or device.

The example's clear and draw commands share one encoder. Texture and buffer uploads are submitted before that encoder's command buffer, preserving the queue order required by the painter.

## Run and observe

Serve the assembled project through the shared storage guard and build slot with `trunk serve --port 8780`. Open `http://127.0.0.1:8780/?tutorial=painting`.

Expect a pink triangle centred in the white canvas and a readable black `Command:` label near the bottom-left edge. Resizing must keep the triangle centred and the label attached to the lower edge. Reload with `?tutorial=frame`: the retained first experiment must still show only white. The mathematics subject comes next and uses this same GPU painter.

## Predict and try

Predict how replacing `screen.center()` with the top-left corner changes the triangle. Its negative offsets should move part of it outside the canvas, where clipping removes it. Restore the centre before continuing.

## Question and answer

Why does the egui pass load its attachment instead of clearing it?

It adds shapes over an already prepared frame. Clearing here would erase the background or scene rendered earlier in the same frame.

## Validation

The partial painting project passes its locked wasm Cargo check and the cumulative-source tests. The production painter also passes full viewer native and wasm typechecks. The linked browser check renders the triangle and command label with the expected white background at desktop and upright dimensions. Long-running texture-lifetime and physical-phone acceptance remain separate.

The [current interaction receipt](release/completion-partials-windows-20261115.json) records Chrome 157 and the Intel Gen-12LP adapter. Upright viewport emulation is not a physical-phone test, and these checks do not accept the remaining cumulative course.
