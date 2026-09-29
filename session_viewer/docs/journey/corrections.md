# Small corrections to a working project

A correction should let you keep learning from the project you have. Save your work first, change only the affected block, then run the focused check. You do not need to retype earlier lessons.

## September 29 · Keep the canvas still when it receives focus

**Affected:** readers who typed lesson 22, 23 or 24 before this correction. The current listings already include the fix, including lesson 25 and later. **Allow about 10 minutes.**

A Chrome check clicked the top orange beam but selected a triangle lower down. The camera maths was correct. Calling `canvas.focus()` scrolled the page by 176 pixels between pointer press and release, moving the canvas underneath the pointer. We need keyboard focus without moving the page.

From `session_viewer`, save your handwritten project:

```sh
npm --prefix ../session_tests run course -- save before-focus-correction
```

In your project's `Cargo.toml`, add `"FocusOptions"` to the existing `web-sys` feature list. Keep all the other features. This enables an additional browser binding; it does not change a dependency version.

In `src/browser.rs`, find the pointer-down branch:

```rust
                event.prevent_default();
                if canvas.focus().is_err() || canvas.set_pointer_capture(id).is_err() {
```

Replace just those two lines with:

```rust
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err() || canvas.set_pointer_capture(id).is_err() {
```

Keep the existing `gesture.cancel()` and closing braces below them. `prevent_default` stops the pointer event's default behaviour; `prevent_scroll` separately controls the focus call. We need both.

Let Trunk rebuild. Use a short browser window so the canvas extends below the window. Click a visible triangle or beam near the top of the drawing. The page should stay still, and the object under the pointer should become selected. Click empty space to clear it. After lesson 25, repeat in Perspective and Orthographic modes.

Compare with the current source for the lesson you have reached, for example:

```sh
npm --prefix ../session_tests run course -- check 24-fit
```

The Chrome regression checks that scroll position stays unchanged, that the clicked orange face turns yellow, and that clearing selection restores the original pixels. These checks run in both projections. Native checks cannot observe the browser moving the page.

[Return to recovery instructions](recovery.md) · [Current verification evidence](release.md)

## Command-line revision

The interactive checkpoints now use the production egui command dock. Lessons 03a–03d introduce it before lesson 04. The later camera, scene, history and renderer concepts continue in the same project.

If you already typed part of the earlier revision, save that working checkpoint before changing it. Keep your geometry and state files. Read the new dock lessons, then use the current lesson’s browser and page listings to connect your existing state to typed commands. The comparison tool reports differences; it never overwrites your handwritten work. You do not need to discard your project and start again.
