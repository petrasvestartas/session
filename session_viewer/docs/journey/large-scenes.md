# Measure rotation and zoom on the full scene

Use the complete floor and the dome from `wood/examples/templates_vault_4_dome.cpp` (40 meridians, 20 hoops). Keep each original protobuf file unchanged and record its SHA-256. Separate scene generation, download, decode, display walking, GPU upload and first presentation from interactive frame times.

On the Lenovo, measure Chrome and Firefox separately. Record the viewer revision, URL, browser version, adapter from `Report`, power mode, viewport, DPR, saved camera and display settings. Keep other applications and the browser tab state consistent. Use the same scene and settings for both revisions.

1. Load the complete scene and wait for loading to finish.
2. Repeat a fixed orbit path, then repeat a fixed wheel-zoom path. Record them as separate runs.
3. Discard the declared warmup. Save raw frame durations and report median, P95 and maximum for at least three repetitions.
4. Repeat with the same AO, opacity, outline, MSAA, point and feature settings. If changing quality, label a separate experiment.
5. Record buffer/texture/source memory after loading, after interaction and after `Close`. Driver memory and process memory are different measurements.
6. Check original IDs, picking, black boundaries, contacts, Undo/Redo and the restored resting image. A faster run with missing geometry fails correctness.

The supplied `bench_*` files validate records and reject unmatched conditions. Their synthetic fixtures are examples, not measurements of your computer. The tree cache proves avoided CPU work; only comparable runs establish its effect on frame time.

## Use the maintained native reference

After subject 94, the `reference` example calls the actual maintained renderer. Run it from the reconstructed lesson crate, substituting an absolute scene path:

```bash
VIEWER_NO_GRID=1 VIEWER_OPACITY=1 VIEWER_LIT=1 VIEWER_MSAA=1 \
VIEWER_FRAMES=60 VIEWER_FRAMES_ORBIT=2 VIEWER_INTERACT=1 \
REGEN_PROTO=0 cargo run --release --locked --target x86_64-unknown-linux-gnu -j4 --example reference -- /tmp/floor.ppm /absolute/path/floor_model.pb
```

The example above names the Linux native host explicitly because this repository defaults to WebAssembly. On another system, use the host triple printed by `rustc -vV`. Repeat for the dome. `VIEWER_GPU_TIMING=1` requests available pass timestamps. `VIEWER_AO=1` and `VIEWER_OUTLINES=1` enable those passes; preserve the settings across comparisons. `VIEWER_ZOOM` changes the initial zoom and does **not** simulate a timed wheel gesture. The native harness renders offscreen and reads pixels back, so its CPU timings include readback. Keep them separate from browser presentation timings. Lavapipe results establish software-renderer behavior only.

## Locate the cost before changing code

| Symptom | Inspect in the maintained viewer |
| --- | --- |
| Long load or first image | `app/walk/brep_shapes.rs`, `app/walk/brep.rs`, `app/loader.rs` |
| CPU cost while orbiting | `engine/performance.rs`, `engine/gpu/pacing.rs`, `state/panel.rs`, `app/ui/mod.rs` |
| Repeated source or vertex uploads | `engine/gpu/arena.rs`, `engine/gpu/instance.rs`, `engine/gpu/mod.rs` |
| Distant-coordinate jitter or wrong depth | `camera.rs`, `engine/gpu/mod.rs` (`rebase_anchor`), `shaders/ink_visibility.wgsl` |
| Dense ink or AO cost | `engine/gpu/triangle_tiles.rs`, `engine/gpu/ssao.rs`, `engine/gpu/timing.rs` |
| Memory after close | `engine/gpu/mod.rs` release/reset paths and retained source owners |

Confirm each path against the current source inventory in subject 96. When reporting a change to an AI assistant, provide the measured trigger, exact original scene, comparable runs, relevant owner and correctness constraints; see [AI pointers](command-line.md#point-an-ai-assistant-at-the-exact-lesson).

[Actual maintained native floor and dome checks](current-native-scenes.md) record the current original files, adapter, camera, settings and limitations.
