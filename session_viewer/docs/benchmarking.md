# How to benchmark

Every performance claim in the viewer comes from one of two harnesses:

1. **Native GPU pass timing (`examples/selftest` / `tests/native-benchmark.py`)** — deterministic Vulkan/Metal/DX12 timestamp queries per render pass, including the `ink.*` sub-draws.
2. **Browser frame pacing & soak (`tests/orbit-benchmark.cjs` / `tests/soak.cjs`)** — real-clock browser `requestAnimationFrame` cadence, CPU split, `onSubmittedWorkDone` completion latency, redraw causes, and long-animation-frame stalls.

## 1. Machine guards before any run

Check that the CPU frequency governor is not thermally or power-capped before recording numbers:

```sh
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq \
    /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq
```

Both numbers must match (`tests/native-benchmark.py` aborts automatically if `scaling_max_freq < cpuinfo_max_freq`). On shared workstations, serialize builds and GPU runs through `wood/tools/buildslot.sh` with `-j8` and `MemoryMax=6G`.

## 2. Reproducible 5-run native GPU baseline

Build the release `selftest` example once, then run `tests/native-benchmark.py`:

```sh
cd session/session_viewer
REGEN_PROTO=0 cargo build -j8 --locked --release \
  --target x86_64-unknown-linux-gnu --example selftest

python3 tests/native-benchmark.py --runs 5 --frames 60 \
  --json-out target/bench/m1-baseline.json
```

The script runs 5 repetitions × 60 frames at `900×700` across `tests/fixtures/timber-floor.pb` and `assets/view_local.yaml` (at rest and under `VIEWER_FRAMES_ORBIT=2 VIEWER_INTERACT=1`) and prints the **median of medians**.

### Direct `selftest` environment knobs

```sh
VIEWER_W=900 VIEWER_H=700 \
VIEWER_FRAMES=60 VIEWER_GPU_TIMING=1 \
VIEWER_FRAMES_ORBIT=2 VIEWER_INTERACT=1 \
  target/x86_64-unknown-linux-gnu/release/examples/selftest \
  /tmp/out.ppm tests/fixtures/timber-floor.pb
```

| Variable | Meaning |
|---|---|
| `VIEWER_W`, `VIEWER_H` | Offscreen framebuffer dimensions in pixels (default `800×600`). |
| `VIEWER_FRAMES` | Number of timed frames to render before writing the final PPM. |
| `VIEWER_GPU_TIMING=1` | Enables `PassTimer` hardware timestamp queries per pass and splits the ink pass into `ink.strokes`, `ink.outlines`, `ink.lanes`, `ink.markers`, `ink.text`, plus `ink_total` (normal rendering keeps a single `begin_ink` pass). |
| `VIEWER_FRAMES_ORBIT=<px>` | Orbits horizontally by `<px>` pointer pixels before each timed frame. |
| `VIEWER_INTERACT=1` | Feeds frame times through the drag quality tiers (`0..=2`) and prints overall `drag: p50 ..., p95 ...`. |
| `VIEWER_TIER=0\|1\|2` | Pins a specific drag quality tier across all timed frames. |
| `VIEWER_OPACITY`, `VIEWER_MSAA`, `VIEWER_LOD` | Isolates translucent ink (`0.95` vs `1`), MSAA (`1` vs `4`), or point-cloud octree LOD (`px`). |
| `VIEWER_PROFILE=1` | Prints per-geometry walk and tessellation breakdown during scene load. |

> **Reading native wall times vs GPU spans:** `selftest` renders offscreen and performs a blocking `render_offscreen` texture readback each frame so pixel tests can inspect the output. Always treat the `gpu ms, median per pass:` hardware timestamps as the primary GPU metric and `Wall p50` as secondary.

### Recorded Milestone M1 → M2 GPU progression (NVIDIA RTX 4080 Laptop, Vulkan, 900×700, 5.20 GHz)

5 runs × 60 frames, median of medians (`ms`). Milestone M1 established the unoptimized baseline; Milestone M2 optimized the ink tile-list walk, `endpoint_clearance` load complexity, pipeline prewarming, and state binding:

| Scenario | Stage | Wall p50 | Drag p95 | `tiles` | `points` | `faces` | `ink.strokes` | `ink.outlines` | `ink.lanes` | `ink.markers` | `ink.text` | `ink_total` |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `timber-floor.pb`, rest | M1 | 8.40 | — | 0.002 | 0.002 | 0.049 | 7.339 | 0.003 | 0.003 | 0.024 | 0.008 | **7.377** |
| `timber-floor.pb`, rest | **M2** | **3.40** | — | 0.002 | 0.002 | 0.049 | **2.456** | 0.003 | 0.003 | 0.024 | 0.008 | **2.493** |
| `timber-floor.pb`, orbit drag | M1 | 8.60 | 10.00 | 0.422 | 0.002 | 0.050 | 7.134 | 0.003 | 0.003 | 0.023 | 0.008 | **7.171** |
| `timber-floor.pb`, orbit drag | **M2** | **3.90** | **4.10** | 0.424 | 0.002 | 0.051 | **2.519** | 0.003 | 0.003 | 0.023 | 0.008 | **2.556** |
| `assets/view_local.yaml`, rest | M1 | 18.50 | — | 0.003 | 0.002 | 0.050 | 17.297 | 0.003 | 0.003 | 0.085 | 0.009 | **17.398** |
| `assets/view_local.yaml`, rest | **M2** | **8.00** | — | 0.003 | 0.002 | 0.050 | **6.957** | 0.003 | 0.003 | 0.085 | 0.008 | **7.056** |
| `assets/view_local.yaml`, orbit drag | M1 | 24.00 | 29.20 | 0.329 | 0.810 | 0.055 | 21.531 | 0.003 | 0.003 | 0.099 | 0.009 | **21.647** |
| `assets/view_local.yaml`, orbit drag | **M2** | **9.70** | **10.50** | 0.295 | 0.810 | 0.055 | **7.487** | 0.003 | 0.003 | 0.096 | 0.008 | **7.598** |

## 3. Browser instrumentation and query parameters

Serve the production bundle with `REGEN_PROTO=0 NO_COLOR=true trunk serve` (`http://127.0.0.1:8770/`).

| Mechanism | Cost | What it exposes |
|---|---|---|
| `window.viewerFrames` & `canvas[data-viewer-frames]` | Always on, O(1) | Monotonic count of presented frames, updated without JSON serialization. |
| `?inspect=perf` | O(1) JSON (~300 B) | Writes `{frames, draw_calls, interacting, navigation_tier, rough, cpu_ms, frame_stats, redraw_causes}` to `canvas[data-viewer-inspection]` without walking scene documents. |
| `?inspect=1` | Full snapshot | Adds full scene, selection, camera, text, and `source_memory` payload accounting for functional tests and `tests/soak.cjs`. |
| `?perf=1` | Continuous redraw | Forces continuous redraws ("render as fast as possible"), updates the top-left HUD (`f<n> gap <ms> (p95 <ms>) gpu <p95> ui <ms> enc <ms> wasm <MiB>`), and logs a 1 Hz console summary. |
| `window.viewerDiagnostics.read().longFrames` | Passive observer | Last 64 `PerformanceObserver` `'long-animation-frame'` (or `'longtask'`) stalls with `startMs`, `durationMs`, and `blockingMs`. |

### Metrics inside `frame_stats`, `cpu_ms`, and `redraw_causes`

- **`cpu_ms`** (`CpuSplit`): wall-clock milliseconds of the latest redraw split into `setup_ms` (visibility/canvas checks), `ui_ms` (egui panel layout, tessellation, and upload), `render_ms` (`State::render` total), and `encode_ms` (`Gpu::encode_frame`).
- **`frame_stats`** (`FrameStats`): `[p50, p95, p99]` in milliseconds over a rolling 600-frame ring for `gap_ms` (frame-to-frame interval), `encode_ms`, `ui_ms`, and `completion_ms` (lock-free `queue.on_submitted_work_done` latency measured in `FramePacing`).
- **`redraw_causes`** (`RedrawCauses`): cumulative counters for `hover`, `egui`, `input`, `refine`, `pick`, `perf`, `rebase`, `async_pass`, `rough_settle`, and `scene`.

## 4. Scripted browser benchmarks

Requires Playwright 1.58.2 and a Chrome instance with hardware WebGPU:

```sh
# 5 s bare hover, 5 s right-drag orbit, and 5 s wheel at desktop (1200x800) and phone (412x915 DPR 2.625):
NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/orbit-benchmark.cjs

# Sustained orbit memory & latency soak (default SOAK_SECONDS=600, SOAK_INTERVAL_MS=10000):
SOAK_SECONDS=600 NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/soak.cjs
```

Use `VIEWER_BENCH_SCENE=view_local` and `VIEWER_BENCH_QUERY="opacity=1&msaa=1"` to compare scenes and view settings.
