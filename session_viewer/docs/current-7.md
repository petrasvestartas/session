# 7 · Finish the shared editing wiring

[Previous](current-6.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-8.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/README-02.svg)

Finish the shared ownership comments, event routing and formatting. The next checkpoint adds the docked workspace, touch editing and portable session files.

### `src/app/edit.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub fn delete_row(&mut self, row: u32) -> bool {
```

**ADD BELOW**

```rust
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return false;
        }
```

**TYPE THIS**

**CURRENT**

```rust
        let session = Rc::make_mut(&mut file.session);
        if back {
            session.undo()
        } else {
            session.redo()
        }
    }
```

**REPLACE WITH**

```rust
        let session = Rc::make_mut(&mut file.session);
        if back { session.undo() } else { session.redo() }
    }
```

**TYPE THIS**

**CURRENT**

```rust

    /// A geometry whose control points the kernel cannot set is refused, not silently ignored:
    /// a drag that appears to do nothing is a bug report waiting to happen.
    #[test]
    fn a_kind_with_no_control_points_is_refused() {
        let mut scene = one_point_twice();
        assert!(!scene.set_control_point(0, 0, &Point::new(1.0, 1.0, 1.0)));
    }

    /// A streamed source is a shell with no kernel object behind it: editing it would write
    /// into an empty session and silently lose the edit, so it is refused.
    #[test]
    fn a_display_only_document_refuses_the_edit() {
        let mut scene = one_point_twice();
        scene.docs[0].display_only = true;
        assert!(
            scene
                .transform_row(0, &Xform::translation(1.0, 0.0, 0.0), "move")
                .is_none()
        );
    }
    #[test]
```

**REPLACE WITH**

```rust

    #[test]
```

**TYPE THIS**

**CURRENT**

```rust
    }

}
```

**REPLACE WITH**

```rust
    }

    /// A geometry whose control points the kernel cannot set is refused, not silently ignored:
    /// a drag that appears to do nothing is a bug report waiting to happen.
    #[test]
    fn a_kind_with_no_control_points_is_refused() {
        let mut scene = one_point_twice();
        assert!(!scene.set_control_point(0, 0, &Point::new(1.0, 1.0, 1.0)));
    }

    /// A streamed source is a shell with no kernel object behind it: editing it would write
    /// into an empty session and silently lose the edit, so it is refused.
    #[test]
    fn a_display_only_document_refuses_the_edit() {
        let mut scene = one_point_twice();
        scene.docs[0].display_only = true;
        assert!(
            scene
                .transform_row(0, &Xform::translation(1.0, 0.0, 0.0), "move")
                .is_none()
        );
    }
}
```

### `src/app/inspection.rs`

**TYPE THIS**

**CURRENT**

```rust
        "draw_calls": state.gpu.performance.draws,
```

**ADD BELOW**

```rust
        "widget": state.gpu.widget.placement,
        "widget_highlight": state.gpu.widget.active,
        "widget_bytes": state.gpu.widget.allocated_bytes(),
```

**TYPE THIS**

**CURRENT**

```rust
        "gpu_texture_estimate_bytes": textures,
```

**ADD BELOW**

```rust
        "egui_private_gpu_capacity": "renderer buffers and font atlas are managed by egui; excluded from totals",
```

### `src/app/mod.rs`

**TYPE THIS**

**CURRENT**

```rust
pub mod touch;
#[cfg(target_arch = "wasm32")]
pub mod ui;
pub mod validate;
```

**REPLACE WITH**

```rust
pub mod touch;
pub mod validate;
```

**TYPE THIS**

**CURRENT**

```rust
pub mod inspection;
```

**ADD BELOW**

```rust

#[cfg(target_arch = "wasm32")]
pub mod ui;
```

### `src/app/scene.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub place: Xform,
    /// Shared with whoever decoded it (the live source keeps its current set), and shared
    /// again between placements: a manifest listing one file twice hands both documents the
    /// same `Rc`. Nothing mutates a session today. Anything that starts to must call
    /// `Rc::make_mut` FIRST, or one placement's edit moves every other placement of that file
    /// and the live source's cached copy with them.
    pub session: Rc<Session>,
```

**REPLACE WITH**

```rust
    pub place: Xform,
    /// Shared placements detach through Rc::make_mut before editing source geometry.
    pub session: Rc<Session>,
```

### `src/state/edit.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub fn undo(&mut self) {
```

**ADD BELOW**

```rust
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Undo requires a scene without streamed sources");
            return;
        }
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn redo(&mut self) {
```

**ADD BELOW**

```rust
        if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
            self.status("Redo requires a scene without streamed sources");
            return;
        }
```

**TYPE THIS**

**CURRENT**

```rust

    /// The gumball, rendered. A headless device draws the same frame twice - once without the
    /// widget and once with it - and the pixels that changed are the widget.
    ///
    /// Differencing rather than looking for colours on a fixed background is what makes this
    /// independent of the backdrop, the grid and the lighting. It is the check the browser
    /// could not give: on the machine this was written on, the page renders black through a
    /// software path.
    #[cfg(not(target_arch = "wasm32"))]
```

**REPLACE WITH**

```rust

    /// GPU pixels contain all three axes and the widget owns no scene rows.
    #[cfg(not(target_arch = "wasm32"))]
```

**TYPE THIS**

**CURRENT**

```rust
    /// for the same CSS pixel, so one CSS pixel is twice as much world - and the arm that is
    /// 72 CSS pixels long stays 72 CSS pixels long.
    #[test]
```

**REPLACE WITH**

```rust
    /// for the same CSS pixel, so one CSS pixel is twice as much world - and the arm that is
    /// 96 CSS pixels long stays 96 CSS pixels long.
    #[test]
```

### Check step 7

**Verified:** the complete step compiles for WebAssembly.

```bash
cargo check -j4 --lib
```


## Answers and next action

**What is complete here?** The original floating-window viewer and its shared editing wiring. The next checkpoint extends source editing and replaces the floating windows with docked panels.

**Why keep this checkpoint?** It is the exact starting state for the next set of edits; do not mix independent extension patches into this sequence.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. The original egui windows, nested panel, solid gumball and polyline/NURBS control editing work together. Continue to checkpoint 8 for the docked workspace and additional source edits.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then open [Dock the workspace, edit source geometry and save](current-8.md) and apply its blocks in order.

[Previous](current-6.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-8.md)

## Expected viewer result

Checkpoint 7 completes the original floating-window interface. The next chapter adds the docked workspace, source subobject edits, touch gumball and Save/Open. This is a maintained-viewer reference; its bottom command dock and toolbar are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 7](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
