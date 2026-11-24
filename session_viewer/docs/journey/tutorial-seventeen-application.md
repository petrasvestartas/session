# Tutorial 17 · Complete application reconstruction and real-scene stage

## Goal

Connect `StageApp` to real 3D kernel geometry rows through `spatial::Scene3` and camera-projected 2D TeX lenses, wire the production `Trunk.toml` and `index.html` entry shell, and verify 100% parity across all `388` production files.

## Diagram

![Production `Trunk.toml` + `index.html` shell, `App -> State -> Mode<State>`, `StageApp + spatial::Scene3` 3D cable-net/arrow bridge, and 388-file parity (native wgpu readback of stage deck)](../../examples/stage/tutorials-systems/17-application.png)

Stage deck source: [`17-application.yaml`](../../examples/stage/tutorials-systems/17-application.yaml).

## Prerequisites and files

Requires [Tutorials 1–16](tutorial-one-startup.md), uniting the 2D mathematics engine from [Tutorial 3.2](tutorial-three-mathematics.md) with the 3D `Scene` and `Camera` from Tutorials 4–16.

Owned files in build order (`8` files):

1. `src/app/stage/geometry.rs`
2. `src/app/stage/net.rs`
3. `src/app/stage/view.rs`
4. `src/app/stage/spatial.rs`
5. `src/app/stage/mode.rs`
6. `src/app/stage/application.rs`
7. `Trunk.toml`
8. `index.html`

## Build

### Step 1 · `src/app/stage/geometry.rs`

`src/app/stage/geometry.rs` evaluates named `3D` points at clock `t`, emits `Spatial` descriptors (`Point`, `Line`, `Polyline`, `12`-edge cuboids, stroke width, `scale.powf(0.3)` pen scaling, dash and reveal), and places `16`-direction collision-scored anchor labels (`+15` hysteresis bonus).

```rust
--8<-- "session_viewer/src/app/stage/geometry.rs"
```

### Step 2 · `src/app/stage/net.rs`

`src/app/stage/net.rs` constructs free/fixed branch-node incidence matrices `(C_N, C_F)`, assembles `D = C^T Q C`, solves `D_N x_N = p - D_F x_F` across `XYZ` for anchored positive-density nets, and exposes `Net::matrix` directly to the slide column.

```rust
--8<-- "session_viewer/src/app/stage/net.rs"
```

### Step 3 · `src/app/stage/view.rs`

`src/app/stage/view.rs` frames the `3D` points orthographically inside the slide's scene half (side-by-side on desktop, top half via `Frame::stacked` on upright screens) so `spatial::lens` projects node labels and `fill`/`fly` animations from live orbited coordinates.

```rust
--8<-- "session_viewer/src/app/stage/view.rs"
```

### Step 4 · `src/app/stage/spatial.rs`

`src/app/stage/spatial.rs` (`Scene3`) sets `state.scene.arrowhead_pens = 24.0` for elongated `0.30`-aspect heads, bakes each `3D` `Point` as an outer `ink` disc plus a `0.58`-scale `bg` paper-centre disc, and maps stage IDs to non-pickable `112 B` GPU rows updated in place.

```rust
--8<-- "session_viewer/src/app/stage/spatial.rs"
```

### Step 5 · `src/app/stage/mode.rs`

`src/app/stage/mode.rs` runs each frame in strict order: tick stage clock -> apply orthographic camera -> derive `2×4` world-to-slide `lens` -> `Presentation::paint` (emitting current `Spatial` list) -> `Scene3::sync` -> return `egui::FullOutput`.

```rust
--8<-- "session_viewer/src/app/stage/mode.rs"
```

### Step 6 · `src/app/stage/application.rs`

`src/app/stage/application.rs` (`StageApp`) routes `?stage=...` index/chapter/picture/font prefetches, keyboard steps/chapters/themes (`T`), scene-half mouse orbit/zoom/double-click reset, column-click advance, and touch gestures across desktop and `Frame::stacked` upright layouts.

```rust
--8<-- "session_viewer/src/app/stage/application.rs"
```

### Step 7 · `Trunk.toml`

`Trunk.toml` configures release WASM bundling (`public_url = "./"`, `no_sri = true`), runs `build-viewer-docs.cjs` in `pre_build` so `dist/docs/` ships beside the viewer, and serves locally on `127.0.0.1:8770` with `Cache-Control: no-store`. Trap: `no_sri = true` is required because CI rewrites `*_bg.wasm` in place with `wasm-opt -O3` after Trunk emits `dist/index.html`.

```toml
--8<-- "session_viewer/Trunk.toml"
```

### Step 8 · `index.html`

`index.html` prefetches Cloudflare R2 scene manifests or stage fonts in parallel with `.wasm` download, mounts `#canvas`, `#command-agent` (16 px mobile IME), and `#viewer-docs`, and bridges `postMessage` (`session-viewer:reload-scene` and `session-viewer:op`). Trap: keep `touch-action: none` on `canvas` and `font-size: 16px` on `#command-agent` so iOS Safari neither steals pinch-zoom nor auto-zooms the viewport on command focus.

```html
--8<-- "session_viewer/index.html"
```

## Integration

`StageApp` implements `Mode<State>` (`src/app/stage/mode.rs`): on every frame it advances `stage.tick(now)`, updates `state.camera` for the left-hand 3D split (`Scene3::camera`), projects 3D points through `Scene3::lens` so 2D haloed TeX labels track 3D nodes without distortion, and syncs kernel `Point`/`Line`/`Polyline` rows via `Scene3::sync`.

## Run and observe

Run `npm --prefix ../session_tests run course -- active-audit`, `npm --prefix ../session_tests run course -- teaching`, and `DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm --prefix ../session_tests run build -- --outDir ../session_viewer/target/docs/vue --emptyOutDir`: `active-audit` reports `passed: true` (`0` missing, `0` extra, `0` changed), `teaching` reports `388/388` explained with `0` errors, and the strict Vue documentation bundle builds cleanly.

## Predict and try

Predict why `spatial::Scene3` renders stage node/vector labels (`P_0`, `\mathbf{F}_1`) through `Scene3::lens` on the 2D `egui` slide painter instead of placing 3D textured quads in world space: orbiting or zooming a 3D quad foreshortens or blurs mathematical subscripts, whereas projecting the 3D anchor to 2D slide coordinates keeps every TeX formula upright, crisp, and halo-outlined at `24 px` across any camera orbit.

## Question and answer

**Why must `StageApp::frame` (`src/app/stage/mode.rs`) compute `Scene3::camera` and `Scene3::lens` *before* `context.run_ui` paints the 2D slide, and run `Scene3::sync` immediately *after*?**

**How to work it out:** Follow one step transition (`ArrowRight` or `solve: true`) where a cable-net node animates to a new 3D equilibrium coordinate while carrying an attached TeX label.

**The answer:** `stage.tick(now)` updates animated 3D node positions first; updating `state.camera` and capturing `Scene3::lens` *before* `paint()` ensures 2D TeX labels project through the exact current frame's camera matrix (zero 1-frame label lag), and running `Scene3::sync` before `State::render` encodes GPU passes uploads the matching 3D cable/arrow row positions for the very same WebGPU submission.

## Validation

Verified via `cargo check --target wasm32-unknown-unknown -j8` (`0` warnings), `cargo xtest -j8` (`626` passed: `484` `viewer-core` + `142` `session_viewer`, `0` failed), `pytest ../session_mcp/tests/test_mcp_parity.py` (`3/3` passed), `course -- active-audit` (`passed: true`), `course -- teaching` (`388/388`, `0` errors), and `DOCS_STRICT_LINKS=1` Vite build (`0` errors). Hardware browser WebGPU capture and physical mobile device checks remain pending an environment with a non-black browser WebGPU adapter.
