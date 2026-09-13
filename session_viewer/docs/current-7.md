# 7 · Match the maintained source exactly

[Previous](current-6.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/README-02.svg)

Finish the shared ownership comments, event routing and formatting. No feature is omitted from this final checkpoint.

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

## Check

```bash
cargo xtest -j4 --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Stop the server with **Ctrl+C**.

### Reproduce the screenshots

The screenshots use the small [nested fixture](extensions/nested.pb) and [manifest](extensions/nested.yaml), not private project files. Save both into your workspace:

```bash
cp "$COURSE_REPO/docs/extensions/nested.pb" assets/extension-nested.pb
cp "$COURSE_REPO/docs/extensions/nested.yaml" assets/extension-nested.yaml
```

Open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## What changed

This sequence combines the supported implementations; unsupported geometry edits retain the limits in the feature inventory.

## Try

Use the command walkthrough, tree selection and gumball controls. Every feature is present.

## Questions and answers

**What goes to the GPU?** Modeling rebuilds existing geometry lanes; panels change object flags; controls upload a small preview. The solid gumball owns a fixed mesh, an unlit shader and a bounded antialiasing tile.

**Why clear row selection after rebuilding?** Row numbers are upload addresses, not permanent identities. A rebuild can assign the same number to a different object.

**Where is the exact patch?** [step 1](extensions/integrated-1.patch), [step 2](extensions/integrated-2.patch), [step 3](extensions/integrated-3.patch), [step 4](extensions/integrated-4.patch), [step 5](extensions/integrated-5.patch), [step 6](extensions/integrated-6.patch), [step 7](extensions/integrated-7.patch). The patch and these visible instructions are generated from the same changes.

## Answers and next action

**What is now complete?** All runtime source files, Cargo dependencies and the browser entry point match the maintained viewer. The independent lessons are alternatives; do not apply their patches over this completed sequence.

**What edits are supported?** Create points/lines/polylines; trim or extend lines/NURBS by normalized parameters; explode polylines; drag polyline/NURBS controls. Mesh vertices and surface controls remain read-only. Streamed scenes refuse commits that their rebuild path cannot preserve.

**Where do I continue?** Use the command walkthrough below. It shows complete commands and their expected geometry. Return to any chapter to see its full implementation and answers.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. The command window, nested panel, solid gumball and control editing work together. Use the supplied fixture instructions above to reproduce the screenshots. The next page walks through create, trim, extend, explode and undo.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then follow [Use the command line](command-line-walkthrough.md) to exercise the finished interface.

[Previous](current-6.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)
