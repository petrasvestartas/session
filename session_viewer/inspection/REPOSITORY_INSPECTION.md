# Repository inspection: session_viewer

Phase 1 of the audit (`inspection/audit_agent.txt`). Inspected 2026-11-19 at superproject `session` HEAD `032cd8c0f`. `session_viewer/` is a plain directory of the `session` repo. It is not a submodule. Its only build dependency is the `session_rust` submodule (path dep).

The source tree was not modified. Builds ran under `wood/tools/buildslot.sh` with `-j8`, `timeout` and `systemd-run --user --scope -p MemoryMax=6G`. Every number below was either printed by a command shown here or counted with `git ls-files`, `wc` or `du`.

> **Update to ARCHITECTURE.md / PERFORMANCE_AUDIT.md.** Both say that nothing could be built. That is no longer true. Builds, native tests and the headless frame harness all ran (see [What was run](#what-was-run)). The real blocker was a broken working tree, not storage.

## Layout

The table counts tracked files (`git ls-files`). Lines are text lines. Disk is `du -sh` and includes untracked files.

| Path | Files | Lines | Disk | Contents |
|---|---|---|---|---|
| `src/` | 352 (319 `.rs`, 33 `.wgsl`) | 111,492 | 4.6 M | the crate |
| `src/app/` | 221 | 68,718 | | `command/` (95), top-level (48), `stage/` (26), `walk/` (24), `scene_sync/` (12), `ui/` (10), `gesture/` (5), `inspection/` (1) |
| `src/engine/` | 72 | 26,083 | | `gpu/` (62), `pipelines/` (5), `text/` (1), 4 files |
| `src/shaders/` | 33 `.wgsl` | 5,472 | | expanded and minified by `build.rs` |
| `src/state/`, `src/state.rs` | 14 + 1 | 5,286 + 1,128 | | |
| `src/selftest/`, `src/selftest.rs` | 4 + 1 | 2,062 + 707 | | native headless harness (`profile.rs`, `oracle.rs`, `lifecycle.rs`, `stroke_witness.rs`) |
| `src/command_dock/` | 3 | 774 | | |
| `src/camera.rs`, `text_quality.rs`, `lib.rs` | 3 | 866 / 363 / 33 | | |
| `examples/` | 82 | 5,856 `.rs` + 2,087 `.yaml` | 1.2 M | 23 top-level `.rs` (selftest, `mk_*` fixture makers, `check_*`), `course/` (13), `stage/` (8 decks, 37 yaml), `cable_net.py` |
| `tests/` | 65 | 6,750 (text) | 9.2 M | 44 `.cjs` browser checks, 7 `.py`, `depth/` (8 scripts), `fixtures/` (`timber-floor.pb`, `vault-dome.pb`), `README.md`, `final-review.md` |
| `assets/` | 20 tracked | | 108 M | `text/` (8, fonts), `stage/` (7), `pb/` (1 tracked: `view_mixed_teapot.pb`; plus about 107 MB of untracked `view_local_*.pb`), `view_local.yaml`, `diagnostics.js` (134 lines), `text-quality.html` |
| `docs/` | 26,321 | 1,520,270 in `.md` | 546 M | `journey/` (10,847 files, 203 M), `lessons/` (7,979, 148 M), `screenshots/` (3,709, 160 M), `illustrations/` (1,349), `diagrams/` (1,236 `.d2`), `typing/` (979), `archived/` (156), plus 30 top-level files |
| root | | | | `Cargo.toml` (91), `Cargo.lock` (3,610; 368 packages), `build.rs` (61), `Trunk.toml` (23), `index.html` (282), `mkdocs.yml` (75), `README.md`, `ARCHITECTURE.md` (226), `AGENTS.md` |
| `.cargo/config.toml` | 1 | 17 | | |
| `dist/` (untracked) | | | 159 M | stale Trunk output from 2026-10-07 |
| `target/` (untracked) | 1,908 entries | | **201 G** | see [Storage](#storage) |

## Toolchain & dependency versions

There is no `rust-toolchain*` file in `session_viewer/` or `session/`, so the active toolchain is the rustup default.

| Item | Version | Evidence |
|---|---|---|
| rustc / cargo | 1.97.1 (8bab26f4f 2026-07-14) / 1.97.1 | `rustc --version`, `rustup show active-toolchain` (default, x86_64-unknown-linux-gnu) |
| Installed targets | `wasm32-unknown-unknown`, `x86_64-unknown-linux-gnu` | `rustup target list --installed` |
| Edition | viewer 2024, session_rust 2021 | `Cargo.toml` |
| Compiler cache | sccache (`rustc-wrapper`) | `~/.cargo/config.toml` |
| wgpu / wgpu-core / wgpu-hal / wgpu-types | 29.0.4 | `Cargo.lock` (spec `"29.0"`) |
| naga | 29.0.4 (dev-dep `=29.0.4`, `wgsl-in`) | used by tests to validate WGSL |
| winit | 0.30.13 | |
| egui / egui-wgpu / egui-winit / epaint | 0.34.3 (pinned `=`) | `default-features = false` |
| glyphon / cosmic-text / swash / skrifa | 0.11.0 / 0.18.2 / 0.2.10 / 0.40.0 | `Cargo.toml` pins skrifa to 0.40 so only one font stack ships |
| ttf-parser / png | 0.25.1 / 0.17.16 | |
| wasm-bindgen / wasm-bindgen-futures | 0.2.128 / 0.4.78 | |
| web-sys / js-sys | 0.3.105 | 29 web-sys features listed in `Cargo.toml` |
| prost / bytemuck | 0.14.4 / 1.25.2 | |
| serde / serde_json / serde_yaml_ng / toml | 1.0.229 / 1.0.151 / 0.10.0 / 0.8.23 (optional, `toml-manifests`) | |
| getrandom | 0.2.17 (`js`), and also 0.3.4 and 0.4.3 transitively | |
| pollster | 0.4.0 (native only) | |
| session_rust | 0.1.0, path `../session_rust`, `default-features = false, features = ["viewer-display"]` | |
| Trunk | 0.21.14 (CI pin) | `viewer-pages.yml` |
| binaryen (`wasm-opt`) | version_123 (CI only, sha256-pinned) | `viewer-pages.yml` |
| Playwright (browser checks) | 1.58.2 | `tests/README.md` |

`Cargo.lock` holds 27 crates in more than one version: bitflags, hashbrown (3), getrandom (3), windows-sys (3), objc2*, syn 2/3, thiserror 1/2, toml_edit, rand 0.8/0.10, and others.

**Cargo features.** `tutorial`, `json-sessions` and `toml-manifests` are all off in the page build. The comment in `Cargo.toml` says each costs more than 30 KB gzipped.

**Profiles.**
- `[profile.dev.package."*"] opt-level = 3`
- `[profile.release]`: `strip = true`, `lto = "fat"`, `codegen-units = 1`

The CI env vars `CARGO_PROFILE_RELEASE_LTO=fat` and `CODEGEN_UNITS=1` repeat what `Cargo.toml` already sets. The CI comment "local builds keep the quick release profile" is stale.

### Browser targets

The viewer is **WebGPU only**. There is no WebGL2 fallback.

- `src/engine/gpu/device.rs:31-35`: `BACKENDS = BROWSER_WEBGPU` on wasm32, and `PRIMARY` (Vulkan/Metal/DX12) natively. `src/text_quality.rs:129` is also `BROWSER_WEBGPU`.
- `index.html:276` shows a `#no-webgpu` message when `navigator.gpu` is missing.
- `assets/diagnostics.js` wraps `navigator.gpu.requestAdapter` to report the adapter.
- `glow` 0.17.0 appears in `Cargo.lock` only as a wgpu-hal transitive. No code path selects GL.

## Build & bundle

| File | What it does |
|---|---|
| `.cargo/config.toml` | Sets `[build] target = "wasm32-unknown-unknown"`, so plain `cargo check` checks wasm. Adds wasm rustflag `-C link-arg=--max-memory=4294967296`. Defines the alias `xtest = "test --target x86_64-unknown-linux-gnu"`. Sets env `RUST_TEST_THREADS=4` and `VIEWER_ADAPTER=nvidia` (not forced). |
| `build.rs` | Copies `src/shaders/*.wgsl` into `OUT_DIR/shaders`. It expands `#include "file"` once per shader, strips `//` comments and collapses whitespace. It asserts that there are no `/* */` comments. Note: `cargo:rerun-if-changed=src/shaders` watches directory mtime only, so run `touch src/shaders` after editing an existing `.wgsl` file in place. |
| `Trunk.toml` | `release = true`, `no_sri = true`, `public_url = "./"`. The pre_build hook runs `node ../session_tests/scripts/build-viewer-docs.cjs`. Custom watch set includes `../session_rust/src` and `docs`. Serves on 127.0.0.1:8770 with `Cache-Control: no-store`. |
| `index.html` | `<link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="0">`, so Trunk skips wasm-opt and CI runs `wasm-opt -O3` afterwards. Copies `assets/pb` to `pb/`, `assets/text`, `assets/stage`, `examples/stage` to `decks/`, `text-quality.html`, `diagnostics.js`, and `target/docs/vue` to `docs/`. Has 4 inline `<script>` blocks: the error overlay, the docs-link removal for `?stage`, a `message` listener that calls `window.wasmBindings.reload_scene`, and the WebGPU check. |
| Rust↔JS glue | 10 `#[wasm_bindgen]` sites in `src/`, one of them `inline_js` (`src/app/session_io.rs:335`). |
| `session_rust/build.rs` | Regenerates the committed `session_rust/src/proto/session_proto.rs` from `../session_proto/*.proto` unless `REGEN_PROTO=0`. This is the cause of the current local build failure (see [What was run](#what-was-run)). |

**Bundle size.** This is the stale local `dist/` from 2026-10-07, built without wasm-opt. It is not the deployed artifact.
- `session_viewer-9e76f9d16fd3b6fd_bg.wasm`: 8,887,485 B raw, 3,080,758 B with `gzip -9`
- JS glue: 127,466 B

No fresh `trunk build` was run.

## Tests

| Kind | Count | Where | Runs in CI? |
|---|---|---|---|
| Native Rust unit tests (`#[test]`) | 675 (cargo reported 614 + 0 + 61 ignored) in 149 `#[cfg(test)]` files | `src/**` | Yes, `cargo xtest -- --nocapture` in `viewer-check.yml` |
| … of which `#[ignore]` | 61: 52 "requires a native GPU adapter", 5 benchmarks, 1 Windows WARP/DXC, 1 local scene files, others | | No (ignored; count reported in `$GITHUB_STEP_SUMMARY`) |
| … GPU tests that skip without an adapter | 10 (`vectors.rs` ×3, `selftest/oracle.rs` ×6, `walk/frames/application_tests.rs` ×1) print `no GPU adapter; skipped` and return | | `viewer-check.yml` runs with `--nocapture` and emits a notice + step summary counting `no GPU adapter; skipped` so CPU-only CI runs visibly state GPU tests were skipped |
| Cargo integration tests (`tests/*.rs`) | 0 | | n/a |
| Examples (compile-checked by `cargo check --all-targets` and `cargo test`) | 23 `.rs` + `examples/course/*.rs` | `examples/` | Compile only |
| Playwright browser checks (`.cjs`) | 44 | `tests/*.cjs`; run against `trunk serve` on :8770. See `tests/README.md` | **No** |
| Python image and format checks | 7 | `tests/*.py` (cad-quality, triangle-visibility, stroke-joins, selection-overlap, publication, format, cad-boundary-plot) | **No** |
| Depth / ink gate scripts | 8 | `tests/depth/` (`_gate.sh`, `_ink_suite.sh`, `_*.py`) | **No** |
| WGSL validation | via naga dev-dep inside unit tests | | Yes (part of xtest) |

## Docs

- **`mkdocs.yml`** is navigation data only. Its first line says "No MkDocs runtime is used". The Vue app in `../session_tests` renders the course: Trunk's pre_build hook builds it locally, and `viewer-pages.yml` builds it in CI into `dist/docs`.
- **Course chapters.** These are `docs/NN-*.md`, listed in `mkdocs.yml` from "00 · Empty project to a WASM message" upwards.
- **`docs/journey/`.** 1,445 entries, including 1,642 `.md` and 7,507 `.rs` files. These are the step-by-step reconstruction (`docs/journey.md` is the route).
- **`docs/lessons/`.** 47 lesson directories with 5,501 `.rs` snapshots.
- **`docs/typing/`.** 890 `.rs` and `.sh` typing checkpoints.
- **Reconstruction tooling.**
  - Capture scripts: `docs/capture_journey.cjs`, `capture_current.cjs`, `capture_practice.cjs`, `capture_typing.cjs`, `capture_viewer.cjs`
  - Checks: `check_illustrations.cjs`, `check_svg.py`
  - Build and serve: `build_site.sh`, `serve.sh`, `open-chrome.sh`
  - Data: `chapters.json`, `practice.json`
  - `hooks/` (3 files)
- **Process docs.** `docs/maintaining.md` (maintenance rules) and `AGENTS.md` (points to `../.claude/rules/viewer-docs.md`).
- **Performance notes.** `docs/phone-performance.md`, `docs/performance-patterns.md`, `docs/ssao.md`, `docs/journey/large-scenes.md` (the native reference timing recipe).
- **Generated output.** `docs/` is 546 MB tracked. `target/docs` (generated) is 13 GB.

## CI/CD

These are the workflows in `../.github/workflows/` that touch the viewer:

| Workflow | Trigger | Viewer jobs |
|---|---|---|
| `viewer-check.yml` | push to main on `session_viewer/**`, `session_rust`, `session_rust/**` or the workflow file; also PRs, `workflow_call` and dispatch | One `check` job on ubuntu-latest:<br>1. `cargo check --target wasm32-unknown-unknown --lib`<br>2. `cargo check --target x86_64-unknown-linux-gnu --all-targets`<br>3. `cargo xtest -- --nocapture` + notice and `$GITHUB_STEP_SUMMARY` reporting `no GPU adapter; skipped` and `#[ignore]` counts<br>Uses rust-cache. Only the `session_rust` submodule is checked out. |
| `viewer-pages.yml` | push to main on `session_viewer/**` or `session_tests/**`; completion of "Session mini tests"; dispatch | 1. `check`, which calls `viewer-check.yml`<br>2. `build`, which runs `trunk build --release` (Trunk 0.21.14) and `wasm-opt -O3` (binaryen 123) on each `*_bg.wasm`, copies `index.html` to `404.html` for short URLs, fetches the kernel test data, and builds the `session_tests` docs into `dist/docs`<br>3. `deploy` to GitHub Pages<br>Scene data is not deployed; it is served from R2. |
| `session-minitest.yml` | push to main or develop; PR | Kernel tests. `session_viewer` is listed only as a rust-cache workspace, and `minitest.sh` runs with `--no-viewer`. |
| `session-reviewer.yml` | schedule / dispatch | Prompt text forbids editing `session_viewer`. No viewer job. |

**Latest CI.** `wood_research/bash/ci_status.sh` shows `viewer-check` and `viewer-pages` at `032cd8c0` completed **success**. Run: https://github.com/petrasvestartas/session/actions/runs/44422216841

## Git history

| Metric | Value |
|---|---|
| Commits touching `session_viewer/` | 1,031 of 1,730 in `session` |
| First and last commit | `f1abe8d35` 2026-04-24 "loft" … `032cd8c0f` 2026-11-19 |
| Authors (name strings) | petras 457, petrasvestartas 447, Petras Vestartas 116, pv 11 |
| Commits since 2026-10-20 | 425 |
| Busiest ISO weeks | W37 157, W46 156, W43 131, W36 128 |

The breakdown below covers file touches since 2026-10-20 (`git log --name-only`):

| Area | Touches |
|---|---|
| `docs/journey` | 6,665 |
| `docs/screenshots` | 4,294 |
| `examples` | 495 |
| `src/app` | 326 |
| `docs/archived` | 156 |
| `src/engine` | 104 |
| `docs/lessons` | 66 |
| `assets` | 43 |
| `docs/illustrations` | 42 |
| `src/shaders` | 32 |
| `tests` | 15 |
| `src/state` + `state.rs` | 17 |
| `Cargo.toml` / `Cargo.lock` | 6 / 4 |

Most of the recent activity is course and tutorial material, not engine code. The most common subject prefixes are `session_viewer:` (46), `docs(viewer):` (22), `docs:` (11) and `stage:` / `feat(stage):` / `fix(stage):`.

## What was run

**Machine state.**
- CPU: `scaling_max_freq` = `cpuinfo_max_freq` = 5,200,000 kHz before each timed run, so the frequency cap is not throttled. The governor is `powersave`.
- Cores and memory: 32 cores, 30 GB RAM (about 14 GB available).
- GPU: NVIDIA GeForce RTX 4080 Laptop, driver 580.178.04. wgpu picked `DiscreteGpu, Vulkan`.
- Load average was about 2.7 at the start, with VS Code and rust-analyzer running.

**Real-tree verification (2026-11-23, after M0 + M3).** `session_proto` is at `edc614e1e` and matches the committed `session_rust/src/proto/session_proto.rs` without needing a snapshot workaround. All commands below ran in the real `session_viewer` working tree under `workspace_storage.py run`, `buildslot.sh`, `timeout 10m` and `systemd-run --user --scope -p MemoryMax=6G` (`scaling_max_freq` = `cpuinfo_max_freq` = `5,200,000` kHz):

| # | Command (inside the wrapper) | Wall | Result |
|---|---|---|---|
| 1 | `cargo check -j8 --target wasm32-unknown-unknown --lib` | 4.16 s | **PASS** (0 warnings) |
| 2 | `cargo check -j8 --target x86_64-unknown-linux-gnu --all-targets` | 8.36 s | **PASS** (3 dead-code warnings in wasm-only `command_dock` helpers on native) |
| 3 | `cargo xtest -j8 engine::gpu::vectors -- --nocapture` | 3.96 s | **PASS**: 5 passed, 0 failed (`worst tip error 0.010`, `center line 0.938`) |
| 4 | `cargo xtest -j8 -- --nocapture` (`RUST_TEST_THREADS=4`) | 104.64 s | **PASS**: **614 passed, 0 failed, 61 ignored** |
| 5 | `DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm run build -- --outDir ../session_viewer/target/docs/vue --emptyOutDir` | 41.51 s | **PASS** (all source includes and strict doc links resolved) |
| 6 | `cargo build -j8 --locked --release --target x86_64-unknown-linux-gnu --example selftest` (2026-11-19) | 222.1 s | PASS. 247 crates compiled (fat LTO, cgu=1). |
| 7 | headless `selftest` runs (below, 2026-11-19), via `buildslot --run` | 0.4–6.4 s each | All exit 0 |

Two tests reported "has been running for over 60 seconds":
- `app::scene::sync::commands_tests::edit_cost_does_not_scale_with_the_scene`
- `selftest::oracle::close_floor_edges_are_continuous_at_phone_and_desktop_sizes`

### Measured headless frame numbers

The binary is `target/x86_64-unknown-linux-gnu/release/examples/selftest`, run from the snapshot directory as `selftest <out.ppm> <scene>`. The size is the default 900×700 unless noted, there is one run per row, and all numbers are exactly as printed.

Read these numbers with three limits in mind:
- They are native Vulkan on an RTX 4080, not browser WebGPU.
- The frame wall time includes the offscreen render and readback (`docs/journey/large-scenes.md`: "its CPU timings include readback").
- There was one run per configuration, so variance is unknown.

| Run | Env | Frames: median / min / max (ms) | GPU median per pass (ms) | GPU bytes: buffers / textures / tile tables | Load (decode / walk) |
|---|---|---|---|---|---|
| timber static | `VIEWER_FRAMES=60 VIEWER_GPU_TIMING=1` | 8.4 (119 fps) / 7.8 / 431.3 | ink 7.577, faces 0.049, others ≤ 0.002 | 50,340,888 / 30,279,399 / 24,246,208 | 4.7 MB: 19.3 ms / 389.5 ms |
| timber orbit drag | `+ VIEWER_FRAMES_ORBIT=2 VIEWER_INTERACT=1` | 8.7 / 7.4 / 28.4; drag p50 8.7, p95 13.0 | ink 7.338, tiles 0.424 | same | 17.2 / 388.1 ms |
| timber, opacity 1 | `VIEWER_OPACITY=1 VIEWER_FRAMES=60 VIEWER_GPU_TIMING=1` | 5.5 / 4.0 / 21.6 | ink 4.657, faces 0.083 | | |
| timber, phone size | `VIEWER_W=824 VIEWER_H=1830 VIEWER_OPACITY=1 VIEWER_FRAMES=20 VIEWER_GPU_TIMING=1 VIEWER_PROFILE=1` | 4.3 / 3.7 / 24.3 | ink 2.395, faces 0.089 | 51,373,880 / 72,474,532 / 25,279,200 | 18.6 ms / 1.276 s (includes the profile walk) |
| vault-dome static | `VIEWER_FRAMES=60 VIEWER_GPU_TIMING=1` | 2.0 / 1.9 / 17.3 | ink 1.118, faces 0.042 | 8,736,956 / 30,279,399 / 4,564,928 | 3.7 MB: 20.0 / 15.0 ms |
| vault-dome orbit drag | `+ ORBIT=2 INTERACT=1` | 2.3 / 2.0 / 17.6; drag p50 2.3, p95 2.5 | ink 1.100, tiles 0.137 | same | |
| `assets/view_local.yaml` static (5 files, 104 MB) | `VIEWER_FRAMES=60 VIEWER_GPU_TIMING=1` | **18.5 (54 fps)** / 18.1 / 157.3 | **ink 17.413**, faces 0.051 | 97,494,208 / 50,439,399 / 12,380,448 | treppenhaus 48.6 MB: 318.9 ms / **3.342 s**; querschnitt 32.0 MB: 222.7 / 502.7 ms; lion 19.1 MB: 39.2 / 15.7 ms; bunny 4.7 MB: 38.7 / 32.7 ms |
| `view_local.yaml` orbit drag | `+ ORBIT=2 INTERACT=1` | 3.2 / 2.8 / 61.8; drag p50 3.2, **p95 37.3**; "tier 1 from frame 4, 115 ms after the first slow frame; p50 3.1 ms there" | ink 1.373, points 0.810, tiles 0.035 | same | treppenhaus walk 3.413 s |

**Walk profile** (`VIEWER_PROFILE=1`, timber-floor). The file has 529 objects and 0 instances.

| Kind | Objects | Walk | Triangles | Edges | Face meshing |
|---|---|---|---|---|---|
| `element.brep` | 312 | 398.7 ms | 120,992 | 4,840 | 406.8 ms |
| `element.mesh` | 217 | 7.6 ms | | | |

The slowest objects were `column_0..3`, at 21.9–22.5 ms each.

Every run wrote a PPM. I checked `local.png` by eye: the lion cloud, bunny, sheets, boxes and grid all render.

**What these runs show** (measured, single runs):
- The `ink` pass accounts for nearly all of the GPU frame time in every scene.
- On the 104 MB local scene, a resting full-quality frame costs 17.4 ms of ink. The drag tier brings that down to 1.4 ms.
- Setting opacity to 1 cuts timber ink from 7.6 to 4.7 ms.
- Loading is dominated by the BRep walk, especially face meshing: 3.3 s for `treppenhaus`.

## What could not be done

| Item | Why | How to do it |
|---|---|---|
| Build or test the **real** working tree | `session_proto` is at a commit the superproject does not record (`edc614e1e` is missing locally), and the regenerated `session_rust/src/proto/session_proto.rs` breaks `session_rust`. Restoring it was outside this read-only brief. | The user should run `git -C session_proto fetch && git submodule update session_proto`, then either rebuild (the regeneration then matches HEAD) or restore `session_rust/src/proto/session_proto.rs`. Keep `REGEN_PROTO=0` for viewer work, as `tests/README.md` does. |
| Browser (WebGPU) frame timings, `trunk serve`, Playwright `.cjs`, Python checks | Running a browser or `trunk serve` was excluded from this brief. | `REGEN_PROTO=0 NO_COLOR=true trunk serve`, then `?perf=1` / `?inspect=1`, and `node tests/navigation-pacing.cjs` with Playwright 1.58.2 (see `tests/README.md` and PERFORMANCE_AUDIT §5.1 B/C). |
| Fresh wasm bundle size | `trunk build --release` + `wasm-opt -O3` were not run. | Run the same two steps as `viewer-pages.yml`, then `gzip -9c dist/*_bg.wasm \| wc -c`. |
| The 61 ignored tests (GPU, benchmarks) | Not requested. Some say "run with buildslot --exclusive". | `cargo xtest -j8 -- --ignored` (a long run; give each benchmark its own slot). |
| Variance and repeatability of timings | One run per configuration | Repeat each `selftest` row 5 times and report the median of medians. |
| Whether CI GPU tests actually execute | No CI log was inspected | `gh run view <viewer-check id> --log \| grep "no GPU adapter"` |

### Storage

- Free disk was 75 G at the start and 65 G at the end. My builds added about 10 G to `target/`, mainly the native test and example debug artifacts.
- `target/` (201 G) breaks down as:

  | Directory | Size |
  |---|---|
  | `x86_64-unknown-linux-gnu` | 59 G |
  | `course-build` | 58 G |
  | `wasm32-unknown-unknown` | 22 G |
  | `docs` | 13 G |
  | `course-checks` | 11 G |
  | `maintained-visual-review` | 6 G |
  | `debug` | 5 G |
  | `course-shared` | 4 G |
  | `scratch-cache-archive` | 3 G |
  | `lessons`, `agents` | 2 G each |

  The remaining ~1,890 entries (`typing-*`, `tutorial-sync-*`, probes, …) are each 1 G or less.
- Nothing was deleted. The user decides what to remove; course and tutorial scratch builds make up about 75 G or more.
- The build wrote into `target/`, which can overwrite uplifted example binaries such as `release/examples/selftest`.
- Scratch copies are in the session scratchpad (tmpfs) and can be discarded.
