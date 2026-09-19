# 1 · Refresh diagnostics and resource checks

[Previous](extend-integrated-tutorial.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-2.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/16-01.svg)

Bring the frozen runtime helpers to their maintained state before adding optional features. Existing rendering remains usable.

### `src/engine/gpu/device.rs`

**TYPE THIS**

**CURRENT**

```rust
/// Remember failure without unwinding through a browser callback.
#[cfg(target_arch = "wasm32")]
fn remember_failure(failure: &std::sync::Mutex<Option<String>>, message: String) {
    if let Ok(mut state) = failure.lock() {
        *state = Some(message);
    }
```

**REPLACE WITH**

```rust
/// Remember failure without unwinding through a browser callback.
#[cfg(any(target_arch = "wasm32", test))]
fn remember_failure(failure: &std::sync::Mutex<Option<String>>, message: String) {
    if let Ok(mut state) = failure.lock() {
        state.get_or_insert(message);
    }
```

**TYPE THIS**

**CURRENT**

```rust
    );
}
```

**ADD BELOW**

```rust

#[cfg(test)]
#[test]
fn first_gpu_error_survives_follow_on_submission_errors() {
    let failure = std::sync::Mutex::new(None);
    remember_failure(&failure, "texture allocation failed".into());
    remember_failure(&failure, "invalid command buffer".into());
    assert_eq!(
        failure.lock().unwrap().as_deref(),
        Some("texture allocation failed")
    );
}
```

### `src/engine/gpu/surface_outline.rs`

**TYPE THIS**

**CURRENT**

```rust
                        };
                        gpu.view.show_mesh_edges = false;
                        let silhouette = gpu.render_offscreen(&input);
                        gpu.view.show_mesh_edges = true;
                        let edged = gpu.render_offscreen(&input);
                        let mut black = 0;

                        for (plain, inked) in silhouette.chunks_exact(4).zip(edged.chunks_exact(4))
                        {
                            if plain[..3].iter().all(|channel| *channel < 8) {
                                black += 1;
                                assert!(
```

**REPLACE WITH**

```rust
                        };
                        gpu.view.show_mesh_edges = true;
                        gpu.segments.set_selected(0, false);
                        let silhouette = gpu.render_offscreen(&input);
                        gpu.segments.set_selected(0, true);
                        let edged = gpu.render_offscreen(&input);
                        let mut coverage = 0_u32;

                        for (plain, inked) in silhouette.chunks_exact(4).zip(edged.chunks_exact(4))
                        {
                            let lo = *plain[..3].iter().min().unwrap();
                            let hi = *plain[..3].iter().max().unwrap();

                            if hi - lo <= 2 {
                                coverage += u32::from(255 - hi);
                            }

                            if hi < 8 {
                                assert!(
```

**TYPE THIS**

**CURRENT**

```rust
                        assert!(
                            black > 500,
                            "the perspective silhouette must remain visible"
                        );
```

**REPLACE WITH**

```rust
                        assert!(
                            coverage > gpu.config.height * 255 / 2,
                            "visible silhouette includes fractional coverage: {coverage}"
                        );
```

**TYPE THIS**

**CURRENT**

```rust
                .count();
            assert!(
                plain_black > 0 && plain_black < black,
                "ordinary and selected outlines both stay visible"
            );
```

**REPLACE WITH**

```rust
                .count();
            assert_eq!(
                plain_black, black,
                "ordinary and selected outlines have the same width"
            );
```

### `src/engine/gpu/upload.rs`

**TYPE THIS**

**CURRENT**

```rust
pub fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
```

**REPLACE WITH**

```rust
pub fn drop_rows<T>(v: &mut Vec<T>) {
    *v = Vec::new();
}
```

### Check step 1

**Verified:** the complete step compiles for WebAssembly.

```bash
cargo check -j4 --lib
```


## Answers and next action

**What changed?** Resource counters and failure handling now match the maintained runtime. Formatting changes do not add rendering work.

**Who owns memory?** `Scene` owns source documents. `Gpu` owns buffers and textures. A geometry rebuild may reuse capacity; scene replacement releases scene-sized storage. The inspection counters exclude browser overhead, undo history and renderer-private allocations.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. You see the empty grid and can orbit, pan and zoom. This checkpoint adds no editing button.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then open [Create, trim, extend and explode](current-2.md) and apply its blocks in order.

[Previous](extend-integrated-tutorial.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-2.md)

## Expected viewer result

With an empty scene, the viewer shows the grid and world axes. Orbit, pan and zoom should work. Runtime diagnostics add no visible editing control. Captured in the maintained viewer with an empty manifest and no object selected.

[![Full viewer result for current 1](screenshots/current-empty.png)](screenshots/current-empty.png)
