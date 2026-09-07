# Ink Visibility Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the face-identity hidden-line machinery with a depth-buffer-only visibility rule so every stroke has uniform weight across joints, silhouettes and creases, hidden lines stay hidden, and the ink lanes cost a handful of depth reads per fragment.

**Architecture:** The physical pass writes unbiased reverse-Z depth and nothing else. Each ink fragment fits the plane of the surface under it from its own depth texel and the next one away from the stroke, and is visible when that plane passes through the stroke's axis within a few float ULPs, or when nothing nearer than the axis covers it. Everything that carried face identity (tokens, planes, support lists, host association, the compute filter, the occluder rectangle, the Tubes style) is deleted.

**Tech Stack:** Rust 2024, wgpu 29.0.4 (WebGPU in the browser, Vulkan in the native harness), WGSL, naga 29.0.4 for shader validation in tests, Python 3 for the probe scripts.

**Spec:** `docs/superpowers/specs/2026-09-07-ink-visibility-design.md` (sections 3, 4, 5, 7 and phase 1 of section 9).

## Global Constraints

- Every `cargo` command in `session_viewer` defaults to wasm32; native tests are `cargo xtest`, native examples take `--target x86_64-unknown-linux-gnu`. Set `REGEN_PROTO=0` for every cargo command.
- Read `.claude/skills/wgpu/SKILL.md` (restore it from git with `git show HEAD:.claude/skills/wgpu/SKILL.md` if the working copy is deleted) before editing any `.wgsl` or `engine/` file. Never write a wgpu call from memory; grep `~/.cargo/registry/src/*/wgpu-29.0.4/src/`.
- `vec3<T>` has size 12 and alignment 16 in WGSL. Every host-shareable struct change carries computed offsets and a mirror test.
- The frame is reverse-Z `Depth32Float`: nearer is GREATER, cleared is 0.
- At most four parameters per function; grouped inputs become a named struct. No closures unless they are the fastest way. A docstring on every function. Comments say WHY.
- Numbers in comments, commit messages and docs are measured or absent.
- Commit messages start with `viewer:`; never add an AI as author or co-author.
- Working directory for every command: `/home/petras/code/code_cpp/wood_research/session/session_viewer`.
- Scratch directory for renders and probes: `/tmp/claude-1000/-home-petras-code-code-cpp-wood-research-session-session-viewer/ef2bd6d5-a627-451a-8fa4-3a542f12a6f0/scratchpad`, referred to below as `$SCRATCH`.

---

## File map

| file | after phase 1 |
|---|---|
| `src/shaders/ink_visibility.wgsl` | rewritten: `InkAxis`, `ink_depth`, `ink_tolerance`, `ink_step`, `ink_visible`, `ink_disc_visible`, `InkColor` |
| `src/shaders/ribbon.wgsl` | one flat ink shader for pipes and ribbons; `end_depth` flat output; `ink_axis`; no silhouette term |
| `src/shaders/sphere.wgsl`, `glyph.wgsl` | `centre` + `depth` flat outputs, `ink_disc_visible` |
| `src/shaders/triangle.wgsl` | one colour target, no face token |
| `src/shaders/cylinder.wgsl`, `face_filter.wgsl` | deleted |
| `src/engine/gpu/segments.rs` | one shader, two 40 B tables, no supports |
| `src/engine/gpu/glyphs.rs` | 48 B rows, no supports |
| `src/engine/gpu/arena.rs` | no planes, no face ids, no filter |
| `src/engine/gpu/targets.rs` | depth + colour only |
| `src/engine/gpu/frame.rs` | 64 B `LineUniform` |
| `src/engine/gpu/objects.rs` | no occluders |
| `src/engine/gpu/face_filter.rs`, `occlusion_bounds.rs`, `plane_place.rs` | deleted |
| `src/engine/pipelines/layouts.rs` | ink instance group binds two depth views; ink rows group binds one table |
| `src/app/walk/mesh.rs`, `mesh_ink.rs` | no tokens, no supports |
| `src/app/walk/mesh_faces.rs`, `mesh_raw_faces.rs`, `hosts.rs` | deleted |
| `src/app/scene.rs` | no host association, no face bases |
| `src/engine/gpu/view.rs`, `src/app/input.rs` | no `LineStyle`, no `L` |
| `examples/mk_joint_probe.rs`, `docs/_stroke_weight.py`, `docs/_orbit_check.py`, `docs/_probe_matrix.py`, `docs/_ink_suite.sh` | new verification |
| `ARCHITECTURE.md`, `docs/_PERF.md` | updated to the tree as it is |

---

### Task 1: Drop the Tubes style

**Files:**
- Delete: `src/shaders/cylinder.wgsl`
- Modify: `src/engine/gpu/segments.rs`, `src/engine/gpu/view.rs`, `src/engine/gpu/render.rs`, `src/app/input.rs`, `src/selftest/lifecycle.rs`, `src/engine/gpu/instance.rs`, `docs/_hidden_line_matrix.py`

**Interfaces:**
- Produces: `SegmentLane::draw_pipes(&self, pass, b: &Binds) -> u32` and `draw_pipe_ids(&self, pass, b: &Binds) -> u32` with no style parameter. `View` has no `line_style` field and no `toggle_line_style`.

- [ ] **Step 1: Delete the cylinder shader and its lane code**

```bash
git rm src/shaders/cylinder.wgsl
```

In `src/engine/gpu/segments.rs`:

Find:
```rust
use crate::engine::pipelines::{build, ink_module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use super::view::LineStyle;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")), ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Sides of the unit cylinder: six is the fewest that reads as round at pen widths.
const CYL_SIDES: u32 = 6;
```
Replace with:
```rust
use crate::engine::pipelines::{build, ink_module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];
```

Find:
```rust
/// The two shader modules the lane's pipelines are built from.
struct SegShaders {
    cylinder: wgpu::ShaderModule,
    ribbon: wgpu::ShaderModule,
}

/// The pipelines over the two tables. `ribbon` serves both lanes' colour pass: the same
/// blended, depth-read-only quad.
struct SegPipelines {
    cylinder: wgpu::RenderPipeline,
    ribbon: wgpu::RenderPipeline,
    id_cylinder: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the unit cylinder, the shaders, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    supports: GrowBuf,
    template: Template,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the unit cylinder, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let template = Template::new(ctx, "cyl.template", &cyl_v, &cyl_i);
        let shaders = SegShaders {
            cylinder: ink_module(&ctx.device, "cylinder.shader", include_str!("../../shaders/cylinder.wgsl")),
            ribbon: ink_module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        let supports = GrowBuf::new(ctx, "segments.supports", std::mem::size_of::<InkSupport>() as u64, ROWS);
        let pipes = SegTable::new(ctx, l, "pipes", &supports);
        let ribbons = SegTable::new(ctx, l, "ribbons", &supports);
        Self { pipes, ribbons, supports, template, shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }
```
Replace with:
```rust
/// The pipelines over the two tables: the same blended quad for both, and its id twin.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the shader, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    supports: GrowBuf,
    shader: wgpu::ShaderModule,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the shader and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl"));
        let gpu = build_pipelines(ctx, l, &shader, target);

        let supports = GrowBuf::new(ctx, "segments.supports", std::mem::size_of::<InkSupport>() as u64, ROWS);
        let pipes = SegTable::new(ctx, l, "pipes", &supports);
        let ribbons = SegTable::new(ctx, l, "ribbons", &supports);
        Self { pipes, ribbons, supports, shader, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shader, target);
    }
```

Find:
```rust
    /// Mesh/BRep edges draw once as tubes or camera-facing quads against physical depth.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, style: LineStyle) -> u32 {
        match style {
            LineStyle::Tubes => self.draw_tubes(pass, b, &self.gpu.cylinder),
            LineStyle::Flat => self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes),
        }
    }
```
Replace with:
```rust
    /// Mesh/BRep edges: camera-facing quads against physical depth.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes)
    }
```

Find:
```rust
    /// The id pass for the solid lane, in the style the colour pass used.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, style: LineStyle) -> u32 {
        match style {
            LineStyle::Tubes => self.draw_tubes(pass, b, &self.gpu.id_cylinder),
            LineStyle::Flat => self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes),
        }
    }
```
Replace with:
```rust
    /// The id pass for the solid lane: opaque quads.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes)
    }
```

Find and **delete** the whole `draw_tubes` function (from `/// The pipes as instanced cylinders through `pipeline`; 0 draws when empty.` through its closing `}`).

Find:
```rust
/// Every segment pipeline reads physical visibility without writing or biasing that depth.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let template = [template_layout()];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let tube = PipelineDesc::new(&s.cylinder, &groups, &template, TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let dev = &ctx.device;

    SegPipelines {
        cylinder: build(dev, target, &tube.with("cylinder", "fs_main")),
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended)),
        id_cylinder: build(dev, Target::ID, &tube.with("cylinder.id", "fs_id")),
        id_ribbon: build(dev, Target::ID, &quad.with("ribbon.id", "fs_id")),
    }
}
```
Replace with:
```rust
/// Both segment pipelines read physical visibility without writing or biasing that depth.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended)),
        id_ribbon: build(dev, Target::ID, &quad.with("ribbon.id", "fs_id")),
    }
}
```

Find and **delete** the whole `unit_cylinder` function (from `/// Unit-cylinder template along +Z` through its closing `}`).

In the test at the end of `segments.rs`, find:
```rust
    /// cylinder.wgsl and ribbon.wgsl read the same 48 B segment row (ends as scalars).
```
Replace with:
```rust
    /// ribbon.wgsl reads the 48 B segment row (ends as scalars).
```

- [ ] **Step 2: Remove `LineStyle` from the view, the keys and the frame list**

In `src/engine/gpu/view.rs`, find:
```rust
/// How the SOLID lane draws mesh/BRep edges. Both read the same segment table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge, with visibility decided at its underlying axis.
    Tubes,
    /// A camera-facing quad per edge through the flat lane's shader. Cheaper.
    Flat,
}

```
Replace with nothing (delete the block, including the trailing blank line).

Find:
```rust
    /// Solid-lane style; `VIEWER_LINE_STYLE=tubes` picks Tubes at startup. `L`.
    pub line_style: LineStyle,
```
Replace with nothing.

Find:
```rust
        let tubes = knob("VIEWER_LINE_STYLE", "style").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false);

```
Replace with nothing.

Find:
```rust
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
```
Replace with nothing.

Find:
```rust

    /// Flip the solid-lane style.
    pub fn toggle_line_style(&mut self) {
        self.line_style = match self.line_style {
            LineStyle::Tubes => LineStyle::Flat,
            LineStyle::Flat => LineStyle::Tubes,
        };
    }
```
Replace with nothing.

In `src/engine/gpu/view.rs` the module doc line `//! `View` - the runtime knobs a frame reads: what to show, how the solid ink is drawn, the` becomes `//! `View` - the runtime knobs a frame reads: what to show, the`.

In `src/app/input.rs`, find:
```rust
//! L line style, D face lighting, B the back-face flag, H hides the selection and S shows
```
Replace with:
```rust
//! D face lighting, B the back-face flag, H hides the selection and S shows
```
Find:
```rust
            Key::Character("l" | "L") => state.gpu.view.toggle_line_style(),
```
Replace with nothing.

In `src/engine/gpu/render.rs`, find:
```rust
            draws += self.segments.draw_pipes(pass, &b, v.line_style);
```
Replace with:
```rust
            draws += self.segments.draw_pipes(pass, &b);
```
Find:
```rust
                self.segments.draw_pipe_ids(&mut pass, &b, v.line_style);
```
Replace with:
```rust
                self.segments.draw_pipe_ids(&mut pass, &b);
```

In `src/selftest/lifecycle.rs`, find:
```rust
    engine::gpu::{view::LineStyle, FrameInput, Gpu, Pick},
```
Replace with:
```rust
    engine::gpu::{FrameInput, Gpu, Pick},
```
Find:
```rust
/// Switch live pipelines and targets, requiring exact restoration after every round trip.
fn states(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Vec<Frame> {
    let mut frames = Vec::new();
    for msaa in [1, 4] {
        gpu.view.msaa_forced = Some(msaa);
        gpu.resize(800, 600);
        gpu.view.line_style = LineStyle::Flat;
        frames.push(render(gpu, scene, camera));
        gpu.view.toggle_line_style();
        frames.push(render(gpu, scene, camera));
        gpu.view.toggle_line_style();
        same(&format!("MSAA{msaa} FLAT→TUBE→FLAT"), &frames[frames.len() - 2], &render(gpu, scene, camera));
    }
```
Replace with:
```rust
/// Switch live targets, requiring exact restoration after every round trip.
fn states(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> Vec<Frame> {
    let mut frames = Vec::new();
    for msaa in [1, 4] {
        gpu.view.msaa_forced = Some(msaa);
        gpu.resize(800, 600);
        frames.push(render(gpu, scene, camera));
        same(&format!("MSAA{msaa} repeat"), &frames[frames.len() - 1], &render(gpu, scene, camera));
    }
```
Find:
```rust
    println!("lifecycle OK: no-face rendering, runtime style/MSAA toggles, resize, rebuild, release, incremental uploads and picking");
```
Replace with:
```rust
    println!("lifecycle OK: no-face rendering, runtime MSAA toggles, resize, rebuild, release, incremental uploads and picking");
```

In `src/engine/gpu/instance.rs`, find:
```rust
            let source = if source.contains("fn footprint(") || name == "cylinder.wgsl" {
```
Replace with:
```rust
            let source = if source.contains("fn footprint(") {
```

In `docs/_hidden_line_matrix.py`, find:
```python
            for style in ("flat", "tubes"):
```
Replace with:
```python
            for style in ("flat",):
```
Find:
```python
                knobs = dict(settings, VIEWER_W="1800", VIEWER_H="1400", VIEWER_MSAA="4", VIEWER_DISTANCE_SCALE=str(scale), VIEWER_LINE_STYLE=style, VIEWER_IDS=str(output / f"{stem}.ids"))
```
Replace with:
```python
                knobs = dict(settings, VIEWER_W="1800", VIEWER_H="1400", VIEWER_MSAA="4", VIEWER_DISTANCE_SCALE=str(scale), VIEWER_IDS=str(output / f"{stem}.ids"))
```
Also change the docstring `"""Run all 42 cases by default; optional filters support focused diagnosis."""` to `"""Run all 21 cases by default; optional filters support focused diagnosis."""`.

In `examples/bench_frame.rs`, find:
```rust
// Median frame time for a still and a moving camera. BENCH_FRAMES=N frames per leg;
// VIEWER_LINE_STYLE=tubes|flat picks the solid-lane style; VIEWER_W / VIEWER_H size it.
```
Replace with:
```rust
// Median frame time for a still and a moving camera. BENCH_FRAMES=N frames per leg;
// VIEWER_W / VIEWER_H size it.
```

- [ ] **Step 3: Build and test**

Run:
```bash
export REGEN_PROTO=0
cargo xtest 2>&1 | tail -5
cargo check 2>&1 | tail -3
grep -rn 'LineStyle\|line_style\|VIEWER_LINE_STYLE\|cylinder' src examples docs/*.py docs/*.sh | grep -v '^docs/0' || echo "no references left"
```
Expected: `test result: ok. 22 passed` (or 21 if a count changed), `Finished` for the wasm check, and the grep prints `no references left` (references in the lesson docs `docs/0*.md` are handled in phase 4).

- [ ] **Step 4: Render the local scene**

Run:
```bash
export REGEN_PROTO=0
VIEWER_W=1400 VIEWER_H=900 cargo run --release --target x86_64-unknown-linux-gnu --example selftest -- $SCRATCH/t1_local.ppm assets/view_local.yaml 2>&1 | tail -2
```
Expected: `non-background pixels: N` with N between 60000 and 65000 (the gate band).

- [ ] **Step 5: Commit**

```bash
git add -A src examples docs/_hidden_line_matrix.py
git commit -m "viewer: drop the Tubes line style

One flat ink shader draws mesh edges and free linework. The cylinder
template, its shader, LineStyle, the L key and ?style= are gone."
```

---

### Task 2: The depth-surface visibility rule

**Files:**
- Rewrite: `src/shaders/ink_visibility.wgsl`
- Modify: `src/shaders/ribbon.wgsl`, `src/shaders/sphere.wgsl`, `src/shaders/glyph.wgsl`, `src/engine/gpu/instance.rs`

**Interfaces:**
- Produces (WGSL, appended to every ink shader by `ink_module`): `struct InkAxis { at: vec2<f32>, depth: f32, along: vec2<f32>, slope: f32 }`, `fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool`, `fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool`, `struct InkColor { @location(0) color: vec4<f32> }`. The bindings stay `@group(2) @binding(2)` (single-sampled depth) and `@binding(3)` (multisampled depth); bindings 4 to 6 of the current layout are simply unused until Task 4 removes them.
- Consumes: `line.vp_w`, `line.vp_h` from `LineUniform`, declared by each lane shader.

- [ ] **Step 1: Write the new visibility shader**

Replace the entire content of `src/shaders/ink_visibility.wgsl` with:

```wgsl
// Shared by the ink lanes. Visibility comes from the physical depth alone, read as a
// piecewise-planar surface: a fragment fits the plane of the surface under it from its own
// texel and the next one AWAY from the stroke, and is on its surface when that plane passes
// through the stroke's axis. Otherwise only something nearer than the axis hides it.
// Reverse-Z: nearer is greater, 0 is cleared. Pixels are framebuffer coordinates, y down.
@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d;

override SCENE_MSAA: bool = false;

// 2^-19: about 16 ULPs of a float, relative to the depth.
const DEPTH_REL_TOL: f32 = 1.9073486e-6;
// The rasterizer snaps vertices to 1/256 px, so a fitted plane's depth is off by its slope
// times that; 2^-6 carries a factor four of headroom.
const SLOPE_PX: f32 = 0.015625;

struct InkColor {
    @location(0) color: vec4<f32>,
};

// The stroke as one fragment sees it: the closest axis point, its depth, the unit screen
// direction of the stroke and the axis's depth change per pixel along it.
struct InkAxis {
    at: vec2<f32>,
    depth: f32,
    along: vec2<f32>,
    slope: f32,
};

// A texel's physical depth at one sample; outside the viewport counts as cleared.
fn ink_depth(pixel: vec2<f32>, sample: u32) -> f32 {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return 0.0;
    }
    let at = vec2<i32>(pixel);
    if (SCENE_MSAA) {
        return textureLoad(scene_depth_msaa, at, i32(sample));
    }
    return textureLoad(scene_depth_single, at, 0);
}

// How far a fitted plane may miss: float precision plus the slope's snapping error over the
// lever arm it was extrapolated across.
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}

// The unit texel step away from the stroke on this fragment's side, along the dominant
// component of the perpendicular, so both texels of the fit lie on the fragment's surface.
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);
    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }
    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}

// A stroke fragment: the surface under it continues through the axis, or nothing nearer
// than the axis covers it.
fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let step = ink_step(pixel, axis);
    let z_out = ink_depth(pixel + step, sample);
    if (z_out != 0.0) {
        // The displacement to the axis as a * along + b * step; the two are never parallel.
        let e = axis.at - pixel;
        let det = axis.along.x * step.y - axis.along.y * step.x;
        let a = (e.x * step.y - e.y * step.x) / det;
        let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
        let g = z_out - z;
        let predicted = z + a * axis.slope + b * g;
        if (abs(predicted - axis.depth) <= ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b))) {
            return true;
        }
    }
    return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
}

// A disc fragment: the plane under it, fitted along both axes away from the centre, passes
// through the centre, or nothing nearer than the centre covers it.
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let d = pixel - centre;
    let sx = select(1.0, -1.0, d.x < 0.0);
    let sy = select(1.0, -1.0, d.y < 0.0);
    let zx = ink_depth(pixel + vec2<f32>(sx, 0.0), sample);
    let zy = ink_depth(pixel + vec2<f32>(0.0, sy), sample);
    if (zx != 0.0 && zy != 0.0) {
        let gx = (zx - z) * sx;
        let gy = (zy - z) * sy;
        let predicted = z - d.x * gx - d.y * gy;
        if (abs(predicted - depth) <= ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
            return true;
        }
    }
    return z <= depth + abs(depth) * DEPTH_REL_TOL;
}
```

- [ ] **Step 2: Point the ribbon shader at the rule**

In `src/shaders/ribbon.wgsl`, find:
```wgsl
    @location(7) @interpolate(flat) inst_id: u32,
    @location(8) @interpolate(flat) segment_index: u32,
};
```
Replace with:
```wgsl
    @location(7) @interpolate(flat) inst_id: u32,
    @location(8) @interpolate(flat) segment_index: u32,
    @location(9) @interpolate(flat) end_depth: vec2<f32>,
};
```

Find:
```wgsl
    dead.solid = 0.0;
    dead.inst_id = 0u;
    return dead;
}
```
Replace with:
```wgsl
    dead.solid = 0.0;
    dead.inst_id = 0u;
    dead.segment_index = 0u;
    dead.end_depth = vec2<f32>(0.0);
    return dead;
}
```

Find:
```wgsl
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    return o;
}
```
Replace with:
```wgsl
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    o.end_depth = vec2<f32>(e0.z / e0.w, e1.z / e1.w);
    return o;
}
```

Find the block from the comment `// Keep capsule coverage and screen endpoints in the original vertex stage. Only physical` through the end of the file (it holds `VisibilityAxis`, `visibility_axis`, `axis_sample`, `footprint`, `fs_main` and `fs_id`). Replace that whole block with:
```wgsl
// The stroke at this fragment: the closest axis point in framebuffer pixels (y down), its
// depth (z/w is affine in screen space), the stroke direction and its depth slope per pixel.
fn ink_axis(in: VsOut) -> InkAxis {
    let ba = in.b - in.a;
    let len2 = max(dot(ba, ba), 1e-6);
    let h = clamp(dot(in.p - in.a, ba) / len2, 0.0, 1.0);
    let at = in.a + ba * h;
    let len = sqrt(len2);
    let along = select(vec2<f32>(1.0, 0.0), vec2<f32>(ba.x, -ba.y) / len, len > 1e-3);
    let slope = (in.end_depth.y - in.end_depth.x) / max(len, 1e-3);
    return InkAxis(vec2<f32>(at.x, line.vp_h - at.y), mix(in.end_depth.x, in.end_depth.y, h), along, slope);
}

@fragment
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, (in.segment_index + 1u) | 0x80000000u);
}
```

Also in `ribbon.wgsl` update the header comment: find `// Flat linework: one camera-facing quad per segment (6 verts pulled by index, no vertex` and the next two lines; replace the three-line header with:
```wgsl
// Flat ink: one camera-facing quad per segment (6 verts pulled by index, no vertex buffer),
// a capsule SDF in the fragment, visibility from the physical depth. Draws the ribbon table
// (free linework) and the pipe table (mesh edges). Group 3 = the segment table.
```

- [ ] **Step 3: Point the marker and dot shaders at the rule**

In `src/shaders/sphere.wgsl`, find:
```wgsl
    @location(3) @interpolate(flat) inst_id: u32,
    @location(4) @interpolate(flat) support: vec2<u32>,
    @location(5) @interpolate(flat) center: vec3<f32>,
};
```
Replace with:
```wgsl
    @location(3) @interpolate(flat) inst_id: u32,
    @location(4) @interpolate(flat) centre: vec2<f32>,
    @location(5) @interpolate(flat) depth: f32,
};
```
Find:
```wgsl
    dead.px = 0.0;
    dead.inst_id = 0u;
    return dead;
}
```
Replace with:
```wgsl
    dead.px = 0.0;
    dead.inst_id = 0u;
    dead.centre = vec2<f32>(0.0);
    dead.depth = 0.0;
    return dead;
}
```
Find:
```wgsl
    o.inst_id = g.instance_id;
    o.support = vec2<u32>(g.support_start, g.support_count);
    o.center = centre;
    return o;
}
```
Replace with:
```wgsl
    o.inst_id = g.instance_id;
    o.centre = vec2<f32>((clip.x / clip.w * 0.5 + 0.5) * line.vp_w, (0.5 - clip.y / clip.w * 0.5) * line.vp_h);
    o.depth = clip.z / clip.w;
    return o;
}
```
Find the block from `fn footprint(in: VsOut) -> InkFootprint {` through the end of the file and replace it with:
```wgsl
@fragment
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let alpha = coverage(in);
    if (alpha <= 0.0 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        discard;
    }
    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_disc_visible(in.pos.xy, in.centre, in.depth, 0u)) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

In `src/shaders/glyph.wgsl`, find:
```wgsl
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat) support: vec2<u32>,
    @location(6) @interpolate(flat) center: vec3<f32>,
};
```
Replace with:
```wgsl
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat) centre: vec2<f32>,
    @location(6) @interpolate(flat) depth: f32,
};
```
Find:
```wgsl
    dead.fade = 0.0;
    dead.inst_id = 0u;
    return dead;
}
```
Replace with:
```wgsl
    dead.fade = 0.0;
    dead.inst_id = 0u;
    dead.centre = vec2<f32>(0.0);
    dead.depth = 0.0;
    return dead;
}
```
Find:
```wgsl
    o.inst_id = g.instance_id;
    o.support = vec2<u32>(g.support_start, g.support_count);
    o.center = world;
    return o;
}
```
Replace with:
```wgsl
    o.inst_id = g.instance_id;
    o.centre = vec2<f32>((clip.x / clip.w * 0.5 + 0.5) * line.vp_w, (0.5 - clip.y / clip.w * 0.5) * line.vp_h);
    o.depth = clip.z / clip.w;
    return o;
}
```
Find the block from `fn footprint(in: VsOut) -> InkFootprint {` through the end of the file and replace it with the same two entry points as in `sphere.wgsl` above (identical text: `fs_main` with `@builtin(sample_index) sample: u32` calling `ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)`, `fs_id` calling it with `0u`).

- [ ] **Step 4: Validate through naga in the mirror test**

In `src/engine/gpu/instance.rs`, find:
```rust
            let source = if source.contains("fn footprint(") {
```
Replace with:
```rust
            let source = if source.contains("-> InkColor") {
```

Run:
```bash
export REGEN_PROTO=0
cargo xtest 2>&1 | tail -5
cargo check 2>&1 | tail -3
```
Expected: all tests pass (naga validates the four ink shaders with the new rule appended) and the wasm check finishes.

- [ ] **Step 5: The gate and the close-up**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
docs/_gate.sh
VIEWER_W=1400 VIEWER_H=900 VIEWER_ZOOM=5 VIEWER_MSAA=4 cargo run --release --target x86_64-unknown-linux-gnu --example selftest -- $SCRATCH/t2_close.ppm assets/pb/view_local_boxes.pb 2>&1 | tail -1
```
Expected: `gate OK (local ink N)`. Open `$SCRATCH/t2_close.ppm` with the Read tool (convert first: `python3 -c "from PIL import Image; Image.open('$SCRATCH/t2_close.ppm').save('$SCRATCH/t2_close.png')"`) and confirm: red mesh edges full width to every corner, no doubling, black vertex markers on top. If the gate prints a magenta count, the rule leaks and the task is not done: inspect the failing plate render before changing any constant.

- [ ] **Step 6: The hidden-line probe subset**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
T=x86_64-unknown-linux-gnu
cargo build -q --release --target $T --example selftest --example mk_hidden_line_probe
B=$CARGO_TARGET_DIR/$T/release/examples
mkdir -p $SCRATCH/probes
$B/mk_hidden_line_probe $SCRATCH/probes/regular.pb
HIDDEN_LINE_PROBE_WARPED=1 $B/mk_hidden_line_probe $SCRATCH/probes/warped.pb
HIDDEN_LINE_PROBE_AUTHORED=1 $B/mk_hidden_line_probe $SCRATCH/probes/authored.pb
for f in regular warped authored; do for cam in "top VIEWER_VIEW=top" "down VIEWER_ORBIT=0,209" "iso VIEWER_NOTHING=1"; do set -- $cam; for msaa in 1 4; do
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_MSAA=$msaa $2 $B/selftest $SCRATCH/probes/${f}_$1_$msaa.ppm $SCRATCH/probes/$f.pb > /dev/null 2>&1
  echo "$f $1 msaa$msaa: $(python3 docs/_count_colors.py $SCRATCH/probes/${f}_$1_$msaa.ppm)"
done; done; done
```
Expected: every line ends in `magenta 0`, and `blue` is above 500 for every case. A nonzero magenta count is a leak: record the case, look at the image, and fix the rule before continuing (the likely causes are the sign of `step` or the y-flip of `at`).

- [ ] **Step 7: Commit**

```bash
git add src/shaders src/engine/gpu/instance.rs
git commit -m "viewer: ink visibility from the depth buffer read as planes

Each ink fragment fits the surface under it from its own depth texel
and the next one away from the stroke; the stroke is on that surface
when the plane passes through its axis within 16 ULPs plus the
rasterizer's snapping error. Per sample under MSAA."
```

---

### Task 3: Producers stop building face identities

**Files:**
- Delete: `src/app/walk/mesh_faces.rs`, `src/app/walk/mesh_raw_faces.rs`, `src/app/walk/hosts.rs`
- Modify: `src/app/walk/mod.rs`, `src/app/walk/mesh.rs`, `src/app/walk/mesh_ink.rs`, `src/app/scene.rs`, `src/app/walk/curves.rs`, `src/app/walk/points.rs`, `src/app/walk/brep.rs`

**Interfaces:**
- Produces: `Row { bounds, spacing, flags, faces, thickness }` (no `host_faces`); `InkCx { row, vpos, slots, lap }` (no `tokens`); `edge_normals(topo: &MeshTopo, ei: usize) -> (Option<[f64; 3]>, Option<[f64; 3]>)`.
- The arena still receives one zero face id per vertex (`ArenaRows.face_ids`) until Task 4 removes the column, so `ArenaLane::append`'s assertion holds.

- [ ] **Step 1: Delete the three files and their module lines**

```bash
git rm src/app/walk/mesh_faces.rs src/app/walk/mesh_raw_faces.rs src/app/walk/hosts.rs
```

In `src/app/walk/mod.rs`, find:
```rust
pub mod mesh;
pub mod mesh_faces;
pub mod mesh_raw_faces;
pub mod hosts;
pub mod mesh_ink;
```
Replace with:
```rust
pub mod mesh;
pub mod mesh_ink;
```
Find:
```rust
    /// The object's thickness in its own units, whatever its orientation (section 6 of
    /// ARCHITECTURE.md): the depth budget the shaders may spend on it.
    pub thickness: f32,
    /// Physical face identities for the file-local authored-line association sweep.
    pub host_faces: Vec<hosts::HostFace>,
}

impl Row {
    /// Linework, points, frames: a box, no spacing, no flags, no faces; as thick as the box.
    pub fn thin(bounds: Aabb) -> Self {
        Self { bounds, spacing: 0.0, flags: 0, faces: false, thickness: bounds.thinnest(), host_faces: Vec::new() }
    }
}
```
Replace with:
```rust
    /// The object's thickness in its own units, whatever its orientation: retained metadata
    /// the instance row carries.
    pub thickness: f32,
}

impl Row {
    /// Linework, points, frames: a box, no spacing, no flags, no faces; as thick as the box.
    pub fn thin(bounds: Aabb) -> Self {
        Self { bounds, spacing: 0.0, flags: 0, faces: false, thickness: bounds.thinnest() }
    }
}
```

- [ ] **Step 2: The mesh walk without tokens**

In `src/app/walk/mesh.rs`, replace the whole `walk_mesh` function (from `/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no.` through its closing `}`) with:
```rust
/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, mc: &MeshCx) -> Row {
    let (cx, o) = (mc.cx, mc.opts);
    let base = cx.vert_base + arena.verts.len() as u32;
    let mut lap = Lap::start("walk_mesh");
    let rm = m.to_render();
    lap.mark("to_render");

    let print = is_print_fill(m);
    let decorated = rm.indices.len() / 3 <= MESH_RAW_MIN && !print;
    let keys = if decorated { m.vertices() } else { Vec::new() };
    let slots = SlotMap::new(&keys);
    let mut vpos64 = Vec::with_capacity(keys.len());
    let mut vpos = Vec::with_capacity(keys.len());
    for &key in &keys {
        let point = &m.vertex[&key];
        vpos64.push([point.x, point.y, point.z]);
        vpos.push([point.x as f32, point.y as f32, point.z as f32]);
    }
    let topo = if decorated { Some(mesh_topology(m, &keys, &vpos64, &slots)) } else { None };
    let mut bounds = Aabb::empty();
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices {
        bounds.grow(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }
    arena.face_ids.resize(arena.face_ids.len() + rm.vertices.len(), 0);
    let idx = index_run(arena, m, o.sheet_lanes && print);
    idx.reserve(rm.indices.len());
    for &i in &rm.indices {
        idx.push(base + i);
    }
    lap.mark("vert+idx push");
    let mut flags = if o.sheet_lanes && print { Instance::FLAG_PRINT } else { 0 };
    if o.smooth && !knobs::seams() {
        flags |= Instance::FLAG_SMOOTH;
    }
    let thickness = mesh_thickness(&positions(&rm.vertices), &rm.indices);
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags, faces: true, thickness };

    if !decorated || knobs::no_edges() {
        return row;
    }

    let topo = topo.expect("decorated mesh has topology");
    lap.mark("topology");
    let mut icx = InkCx { row: cx.row, vpos: &vpos, slots: &slots, lap: &mut lap };
    edges_and_dots(ink, m, &topo, &mut icx);

    // An open mesh is not a solid: the facing cull would strip interior surface seen through
    // the hole, so the shaders skip it like FLAG_INSIDE.
    let open = o.allow_open && !topo.closed;
    Row { flags: if open { row.flags | Instance::FLAG_OPEN } else { row.flags }, ..row }
}
```

- [ ] **Step 3: Mesh ink without supports**

Replace the whole content of `src/app/walk/mesh_ink.rs` with:
```rust
//! The ink a mesh wears: one pipe per visible edge, one marker per vertex - the SOLID lane.
//! Reads the fused topology and the positions by slot; writes `SegRows.pipes` and
//! `GlyphRows.spheres`, nothing else.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use super::encode::{encode_width, oct16, pack_facing, BLACK, FACING_UNKNOWN};
use super::mesh::{Lap, COPLANAR_DOT, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};

/// The two ink lanes a mesh reaches: pipes for its edges, spheres for its vertices.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot,
/// the key -> slot map and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width: one entry broadcasts to every edge, an absent one is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
}

/// Width 0 = hidden: a triangulated fill asks for no wireframe.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal of the face in slot `side` of an edge's pair; None past a border.
fn normal_of(topo: &MeshTopo, faces: [u32; 2], side: usize) -> Option<[f64; 3]> {
    if faces[side] == u32::MAX { return None; }
    topo.normals[faces[side] as usize]
}

/// The two normals the facing test compares for edge `ei`. When the pair's winding disagrees
/// - both faces walk the edge the same way - the second normal points into the solid, so it is
/// negated here: the test wants two outward normals, and the traversal direction is the only
/// local evidence of which of the two is the wrong way round.
fn edge_normals(topo: &MeshTopo, ei: usize) -> (Option<[f64; 3]>, Option<[f64; 3]>) {
    let f = topo.edge_faces[ei];
    let n0 = normal_of(topo, f, 0);
    let n1 = normal_of(topo, f, 1);
    if topo.opposed[ei] {
        return (n0, n1);
    }
    (n0, n1.map(|n| [-n[0], -n[1], -n[2]]))
}

/// Append edge `ei`'s faces to `fkeys`, deduped.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX {
            continue;
        }
        let fk = f as usize;
        if !fkeys.contains(&fk) {
            fkeys.push(fk);
        }
    }
}

/// Word `k` of a marker's facing triple, by `pack_facing`'s rules.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;
            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// The pipe loop: one segment per visible, non-coplanar edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx) {
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN;
    ink.seg.pipes.reserve(topo.edges.len());
    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let (na, nb) = edge_normals(topo, i);
        let facing = pack_facing(na.as_ref(), nb.as_ref());
        if hidden(w, i) {
            continue;
        }
        // Interior tessellation: a diagonal across a flat region shares two coplanar faces.
        if let (Some(n0), Some(n1)) = (na, nb) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue;
            }
        }
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: if black_wire { BLACK } else { *col },
            facing,
            support_start: 0,
            support_count: 0,
        });
    }
}

/// Per vertex: the widest visible incident edge (its width and index), and the incident
/// edge list as CSR (`vstart`, `vinc`). Hidden edges still count for adjacency.
struct Incidence {
    best: Vec<(f64, usize)>,
    vstart: Vec<u32>,
    vinc: Vec<u32>,
}

/// Build the incidence tables over the topology.
fn incidence(m: &Mesh, topo: &MeshTopo, cx: &InkCx) -> Incidence {
    let w = m.widths();
    let nv = cx.vpos.len();
    let mut best = vec![(f64::NEG_INFINITY, 0usize); nv];
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        if hidden(w, i) {
            continue;
        }
        let wi = width_at(w, i);
        for vk in [*a, *b] {
            let e = &mut best[cx.slots.slot(vk)];
            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    let mut vstart = vec![0u32; nv + 1];
    for (a, b, _) in topo.edges.iter() {
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }
    for i in 0..nv {
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * topo.edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    Incidence { best, vstart, vinc }
}

/// The marker loop: one glyph per vertex with a visible edge, carrying up to six incident
/// face normals (widest edge's pair first) so the disc hugs every face at a corner.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx, inc: &Incidence) {
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    ink.glyph.spheres.reserve(nv);
    for (i, &(vw, ei)) in inc.best.iter().enumerate().take(nv) {
        if vw == f64::NEG_INFINITY {
            continue;
        }
        fkeys.clear();
        push_faces(&topo.edge_faces, ei, &mut fkeys);
        for &j in &inc.vinc[inc.vstart[i] as usize..inc.vstart[i + 1] as usize] {
            push_faces(&topo.edge_faces, j as usize, &mut fkeys);
        }
        codes.clear();
        for fk in &fkeys {
            if let Some(n) = topo.normals[*fk] && let Some(code) = oct16(&n) && !codes.contains(&code) { codes.push(code); }
        }
        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
            instance_id: cx.row,
            // A truncated normal list cannot prove every incident face points away.
            facing: if codes.len() > 6 { FACING_UNKNOWN } else { facing_word(&codes, 0) },
            facing_ext: if codes.len() > 6 { [FACING_UNKNOWN; 2] } else { [facing_word(&codes, 1), facing_word(&codes, 2)] },
            support_start: 0,
            support_count: 0,
            _pad: [0; 2],
        });
    }
}

/// Pipes, then markers unless VIEWER_NO_DOTS.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let inc = incidence(m, topo, cx);
    cx.lap.mark("incidence");
    push_pipes(ink, m, topo, cx);
    cx.lap.mark("pipe loop");
    if knobs::no_dots() {
        return;
    }
    push_markers(ink, m, topo, cx, &inc);
    cx.lap.mark("markers");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::mesh::{MeshCx, MeshOpts, walk_mesh};
    use crate::app::walk::WalkCx;
    use crate::engine::gpu::arena::ArenaRows;

    /// A box wears twelve pipes and eight markers, every pipe with two known face normals.
    #[test]
    fn box_ink_rows() {
        let mesh = Mesh::create_box(10.0, 20.0, 30.0);
        let mut arena = ArenaRows::default();
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        let mut ink = Ink { seg: &mut segments, glyph: &mut glyphs };
        let cx = WalkCx { vert_base: 50, face_base: 100, cloud_px: 0.0, row: 7 };
        walk_mesh(&mut arena, &mut ink, &mesh, &MeshCx { cx: &cx, opts: &MeshOpts::OBJECT });
        assert_eq!(segments.pipes.len(), 12);
        assert_eq!(glyphs.spheres.len(), 8);
        for segment in &segments.pipes {
            assert_ne!(segment.facing, FACING_UNKNOWN);
            assert_eq!(segment.instance_id, 7);
        }
        assert_eq!(arena.face_ids.len(), arena.verts.len());
    }
}
```

- [ ] **Step 4: The scene without host association**

In `src/app/scene.rs`, find:
```rust
use crate::app::walk::mesh::Lap;
use crate::app::walk::mesh_ink::Ink;
use crate::app::walk::hosts::{Association, Hosts};
use crate::app::walk::{is_drawable, walk_geometry, Walk, WalkCx};
```
Replace with:
```rust
use crate::app::walk::mesh::Lap;
use crate::app::walk::{is_drawable, walk_geometry, Walk, WalkCx};
```
Find:
```rust
        let mut hosts = Hosts::default();
        let mut pending = Vec::new();
        for guid in session.order() {
```
Replace with:
```rust
        for guid in session.order() {
```
Find:
```rust
            let ribbon_start = self.tables.seg.ribbons.len();
            let dot_start = self.tables.glyph.dots.len();
            let cx = WalkCx { vert_base: self.bases.vert, face_base: self.bases.face, cloud_px: point_px, row };
```
Replace with:
```rust
            let ribbon_start = self.tables.seg.ribbons.len();
            let cx = WalkCx { vert_base: self.bases.vert, face_base: self.bases.face, cloud_px: point_px, row };
```
Find:
```rust
            o.thickness = r.thickness;
            hosts.extend(r.host_faces, &object_place);
            let ribbon_end = self.tables.seg.ribbons.len();
            let dot_end = self.tables.glyph.dots.len();
            if ribbon_start != ribbon_end { self.ribbon_ranges[row as usize] = Some(self.bases.ribbon + ribbon_start as u32..self.bases.ribbon + ribbon_end as u32); }
            if ribbon_start != ribbon_end || dot_start != dot_end { pending.push((guid, object_place, ribbon_start..ribbon_end, dot_start..dot_end)); }
        }
        for (guid, object_place, ribbons, dots) in pending {
            if let Some(geometry) = session.lookup.get(&guid) {
                hosts.associate(&mut Ink { seg: &mut self.tables.seg, glyph: &mut self.tables.glyph }, geometry, &Association { place: &object_place, ribbons, dots });
            }
        }
        lap.mark("objects");
```
Replace with:
```rust
            o.thickness = r.thickness;
            let ribbon_end = self.tables.seg.ribbons.len();
            if ribbon_start != ribbon_end { self.ribbon_ranges[row as usize] = Some(self.bases.ribbon + ribbon_start as u32..self.bases.ribbon + ribbon_end as u32); }
        }
        lap.mark("objects");
```

In `src/app/walk/curves.rs`, find the module doc lines:
```rust
//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one segment per span,
//! `FACING_UNKNOWN` because free linework has no topological facing cull. Scene assembly
//! associates exactly coplanar spans with their supporting mesh faces after the walk.
```
Replace with:
```rust
//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one segment per span,
//! `FACING_UNKNOWN` because free linework has no topological facing cull.
```
Find:
```rust
/// Keep the walk and supporting-face association on identical original f64 curve samples.
pub(super) fn sample_nurbscurve(c: &NurbsCurve) -> Vec<[f64; 3]> {
```
Replace with:
```rust
/// The curve's f64 samples, one chord per `CHORD_DEGREES` of turning.
pub(super) fn sample_nurbscurve(c: &NurbsCurve) -> Vec<[f64; 3]> {
```

In `src/app/walk/points.rs`, find:
```rust
//! A free point into the FLAT glyph lane: one SDF dot with no topology-facing cull.
//! Scene assembly associates coincident dots with their actual supporting mesh faces.
```
Replace with:
```rust
//! A free point into the FLAT glyph lane: one SDF dot with no topology-facing cull.
```

- [ ] **Step 5: Build, test, gate**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
cargo xtest 2>&1 | tail -5
cargo check 2>&1 | tail -3
docs/_gate.sh
grep -rn 'hosts\|HostFace\|mesh_faces\|FaceSupport\|tokens' src | grep -v 'face_ids' || echo "no references left"
```
Expected: tests pass (`box_ink_rows` among them; the deleted tests for supports, hosts and tokens are gone), the wasm check finishes, `gate OK`, `no references left`.

- [ ] **Step 6: Commit**

```bash
git add -A src
git commit -m "viewer: producers stop building face identities

No face tokens, no support lists, no host association: the depth
rule needs none of them. Mesh ink reads face normals straight from
the fused topology again."
```

---

### Task 4: Delete the face-identity engine

**Files:**
- Delete: `src/engine/gpu/face_filter.rs`, `src/engine/gpu/occlusion_bounds.rs`, `src/engine/gpu/plane_place.rs`, `src/shaders/face_filter.wgsl`
- Modify: `src/engine/gpu/arena.rs`, `src/engine/gpu/targets.rs`, `src/engine/pipelines/layouts.rs`, `src/engine/pipelines/mod.rs`, `src/engine/gpu/objects.rs`, `src/engine/gpu/frame.rs`, `src/engine/gpu/present.rs`, `src/engine/gpu/render.rs`, `src/engine/gpu/mod.rs`, `src/engine/gpu/upload.rs`, `src/engine/gpu/instance.rs`, `src/shaders/triangle.wgsl`, every `src/shaders/*.wgsl` declaring `struct LineUniform`, `src/app/scene.rs`, `src/app/walk/mod.rs`, `src/app/walk/mesh.rs`, `src/app/walk/mesh_ink.rs`

**Interfaces:**
- Produces: `ArenaRows { verts, vids, idx, idx_print, idx_text }`; `Targets { depth, msaa, depth_single, depth_msaa, samples }`; `InkScene<'a> { targets: &'a Targets }`; `LineUniform` 64 B with fields `thickness, proj_y, ortho_h, vp_h, vp_w, eye, anchor, feather, lit, backface, _pad: [f32; 2]`; `WalkCx { vert_base, cloud_px, row }`; `FrameCx { view, anchor, size }`.
- Group 2 of the ink layout binds instances (0), translations (1), single depth (2), multisampled depth (3).

- [ ] **Step 1: Delete the files**

```bash
git rm src/engine/gpu/face_filter.rs src/engine/gpu/occlusion_bounds.rs src/engine/gpu/plane_place.rs src/shaders/face_filter.wgsl
```

- [ ] **Step 2: The arena**

Replace the whole content of `src/engine/gpu/arena.rs` with:
```rust
//! The mesh lane: one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it - solid faces, sheet fills (depth write off, document order) and
//! lettering (last of all). `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, instance_id_layout, module, vertex_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use session_rust::RenderVertex;
use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"))];

/// One upload's mesh rows: vertices, their object rows, and the three index runs.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

impl ArenaRows {
    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
    }
}

/// The four pipelines over the arena: solid faces (opaque: the shader writes alpha 1), sheet
/// runs (blended, depth read-only), and their id-pass twins.
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
    sheet: wgpu::RenderPipeline,
    id_faces: wgpu::RenderPipeline,
    id_sheet: wgpu::RenderPipeline,
}

/// The arena on the GPU: five `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
    shader: wgpu::ShaderModule,
    pipes: ArenaPipelines,
}

impl ArenaLane {
    /// Five one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = module(&ctx.device, "triangle.shader", include_str!("../../shaders/triangle.wgsl"));
        let pipes = build_pipelines(ctx, l, &shader, target);

        Self {
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, VERTS),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES),
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, INDICES),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, INDICES),
            shader,
            pipes,
        }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows. The sheet runs index the SAME vertex table.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }

    /// The solid faces, one indexed draw: the physical depth every ink fragment reads.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.faces, &self.faces)
    }

    /// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions
    /// composite in document order. 3D geometry in front still occludes them.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.print)
    }

    /// Lettering, last of everything: a page paints its text on top of hatching and linework.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.text)
    }

    /// The id pass for the faces and the sheet fills, each fragment its object row.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_faces, &self.faces) + self.draw_run(pass, b, &self.pipes.id_sheet, &self.print)
    }

    /// The id pass for the lettering, after the ink as in the colour pass.
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_sheet, &self.text)
    }

    /// One index run through `pipeline`; 0 draws when it is empty.
    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline, run: &GrowBuf) -> u32 {
        if run.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Hand every buffer back: five one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    /// Vertices on the GPU.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Indices in the SOLID faces run - the MSAA policy reads it; sheet fills are not solid.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}

/// The four arena pipelines for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> ArenaPipelines {
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);
    let dev = &ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &base.with("triangle", "fs_main")),
        sheet: build(dev, target, &base.with("triangle.sheet", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnly)),
        id_faces: build(dev, Target::ID, &base.with("triangle.id", "fs_id")),
        id_sheet: build(dev, Target::ID, &base.with("triangle.sheet.id", "fs_id").depth(DepthMode::ReadOnlyEqual)),
    }
}
```

- [ ] **Step 3: The face shader with one colour target**

In `src/shaders/triangle.wgsl`, find:
```wgsl
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32,
    @location(4) face_id: u32,
}
```
Replace with:
```wgsl
    @location(2) color: vec3<f32>,
    @location(3) inst_id: u32,
}
```
Find:
```wgsl
    @location(4) @interpolate(flat) inst_id: u32,
    @location(5) @interpolate(flat, first) face_id: u32,
}
```
Replace with:
```wgsl
    @location(4) @interpolate(flat) inst_id: u32,
}
```
Find:
```wgsl
    dead.inst_id = 0u;
    dead.face_id = 0u;
    return dead;
}
```
Replace with:
```wgsl
    dead.inst_id = 0u;
    return dead;
}
```
Find:
```wgsl
    o.inst_id = in.inst_id;
    o.face_id = in.face_id;
    return o;
}
```
Replace with:
```wgsl
    o.inst_id = in.inst_id;
    return o;
}
```
Find the block from `struct FaceOut {` through the end of the file and replace it with:
```wgsl
@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    return shade(in, front);
}
```
Also delete the now-duplicated earlier `fs_main` (the one directly above `struct FaceOut` in the original file) so exactly one `fs_main` remains.

- [ ] **Step 4: Targets, layouts, pipelines**

In `src/engine/gpu/targets.rs`, find:
```rust
//! `Targets` - physical depth, face identity and colour attachments at the scene's sample
//! count. The face pass establishes occlusion; the ink pass samples it without modifying it.
```
Replace with:
```rust
//! `Targets` - the physical depth and colour attachments at the scene's sample count. The
//! face pass establishes occlusion; the ink pass samples the depth without modifying it.
```
Find:
```rust
/// The attachments of the frame's render pass and the sample count they were made at.
/// `msaa` exists only at 4x.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub faces: wgpu::TextureView,
    pub depth_single: wgpu::TextureView,
    pub depth_msaa: wgpu::TextureView,
    pub faces_single: wgpu::TextureView,
    pub faces_msaa: wgpu::TextureView,
    pub samples: u32,
}

impl Targets {
    /// Frame attachments and opposite-sample-count placeholder bindings.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(ctx, "depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples, usage });
        let msaa = if samples > 1 {
            Some(texture_view(ctx, "msaa_color", &TextureSpec { size, format, samples, usage }))
        } else {
            None
        };

        let faces = texture_view(ctx, "physical.faces", &TextureSpec { size, format: wgpu::TextureFormat::Rg16Uint, samples, usage });
        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = texture_view(ctx, "unused.depth", &TextureSpec { size: (1, 1), format: wgpu::TextureFormat::Depth32Float, samples: other_samples, usage });
        let empty_faces = texture_view(ctx, "unused.faces", &TextureSpec { size: (1, 1), format: wgpu::TextureFormat::Rg16Uint, samples: other_samples, usage });
        let (depth_single, depth_msaa, faces_single, faces_msaa) = if samples == 1 {
            (depth.clone(), empty_depth, faces.clone(), empty_faces)
        } else {
            (empty_depth, depth.clone(), empty_faces, faces.clone())
        };
        Self { depth, msaa, faces, depth_single, depth_msaa, faces_single, faces_msaa, samples }
    }
```
Replace with:
```rust
/// The attachments of the frame's render pass and the sample count they were made at.
/// `msaa` exists only at 4x. The ink layout binds a single-sampled AND a multisampled depth
/// view, so the one not in use is a 1x1 placeholder.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub depth_single: wgpu::TextureView,
    pub depth_msaa: wgpu::TextureView,
    pub samples: u32,
}

impl Targets {
    /// Frame attachments and the opposite-sample-count placeholder binding.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(ctx, "depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples, usage });
        let msaa = if samples > 1 {
            Some(texture_view(ctx, "msaa_color", &TextureSpec { size, format, samples, usage }))
        } else {
            None
        };

        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = texture_view(ctx, "unused.depth", &TextureSpec { size: (1, 1), format: wgpu::TextureFormat::Depth32Float, samples: other_samples, usage });
        let (depth_single, depth_msaa) = if samples == 1 { (depth.clone(), empty_depth) } else { (empty_depth, depth.clone()) };
        Self { depth, msaa, depth_single, depth_msaa, samples }
    }
```
Find:
```rust
    /// Clear physical depth to reverse-Z far and record the nearest face identity.
    /// Multisampled colour resolves only after the following ink pass.
    pub fn begin_faces<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, clear: wgpu::Color) -> wgpu::RenderPass<'a> {
        let target = self.msaa.as_ref().unwrap_or(view);
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            }), Some(wgpu::RenderPassColorAttachment {
                view: &self.faces,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })],
```
Replace with:
```rust
    /// Clear physical depth to reverse-Z far and write the faces. Multisampled colour
    /// resolves only after the following ink pass.
    pub fn begin_faces<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, clear: wgpu::Color) -> wgpu::RenderPass<'a> {
        let target = self.msaa.as_ref().unwrap_or(view);
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            })],
```

In `src/engine/pipelines/layouts.rs`, find:
```rust
/// Physical scene depth or face tokens sampled only after the face pass finishes.
fn scene_texture(binding: u32, multisampled: bool, depth: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: if depth { wgpu::TextureSampleType::Depth } else { wgpu::TextureSampleType::Uint },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

/// Ink keeps instance rows and adds the immutable physical scene attachments.
fn ink_instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.instance.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            buffer_entry(1, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            scene_texture(2, false, true), scene_texture(3, true, true),
            scene_texture(4, false, false), scene_texture(5, true, false),
            buffer_entry(6, wgpu::ShaderStages::FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
        ],
    })
}
```
Replace with:
```rust
/// The physical depth, sampled by the ink after the face pass finishes.
fn scene_depth(binding: u32, multisampled: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Depth,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

/// Ink keeps the instance rows and adds the immutable physical depth, single and multisampled.
fn ink_instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.instance.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            buffer_entry(1, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            scene_depth(2, false),
            scene_depth(3, true),
        ],
    })
}
```

In `src/engine/pipelines/mod.rs`, find:
```rust
const FACE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 4,
    format: wgpu::VertexFormat::Uint32,
}];

```
Replace with nothing. Find:
```rust
/// One exact supporting-face token per vertex at `@location(4)`.
pub fn face_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout { array_stride: 4, step_mode: wgpu::VertexStepMode::Vertex, attributes: &FACE_ID_ATTRIBS }
}

```
Replace with nothing. Find:
```rust
    pub depth: DepthMode,
    pub face_target: Option<bool>,
    pub scene_samples: Option<u32>,
}
```
Replace with:
```rust
    pub depth: DepthMode,
    pub scene_samples: Option<u32>,
}
```
Find:
```rust
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque, face_target: None, scene_samples: None }
```
Replace with:
```rust
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque, scene_samples: None }
```
Find:
```rust
    /// Add the physical face identity attachment; only face-writing fragments modify it.
    pub fn face_target(mut self, write: bool) -> Self {
        self.face_target = Some(write);
        self
    }

```
Replace with nothing. Find:
```rust
    let mut targets = vec![Some(wgpu::ColorTargetState { format: target.format, blend, write_mask })];
    if let Some(write) = desc.face_target {
        targets.push(Some(wgpu::ColorTargetState { format: wgpu::TextureFormat::Rg16Uint, blend: None, write_mask: if write { wgpu::ColorWrites::ALL } else { wgpu::ColorWrites::empty() } }));
    }
    let mut constants = Vec::new();
    if let Some(samples) = desc.scene_samples {
        constants.push(("SCENE_MSAA", f64::from(samples > 1)));

    }
```
Replace with:
```rust
    let targets = [Some(wgpu::ColorTargetState { format: target.format, blend, write_mask })];
    let mut constants = Vec::new();
    if let Some(samples) = desc.scene_samples {
        constants.push(("SCENE_MSAA", f64::from(samples > 1)));
    }
```

- [ ] **Step 5: Objects, frame, present, render, upload, Gpu**

In `src/engine/gpu/objects.rs`, find:
```rust
use crate::engine::pipelines::Layouts;
use crate::math::{mat_scale, mat_to_f32, Aabb, Mat4};
use session_rust::Point;
use std::collections::BTreeMap;
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
```
Replace with:
```rust
use crate::engine::pipelines::Layouts;
use crate::math::{mat_scale, mat_to_f32, Aabb, Mat4};
use session_rust::Point;
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
```
Find:
```rust
    bounded: Vec<BoundedRow>,
    local_bounds: Vec<Aabb>,
    occluders: BTreeMap<u32, bool>,
    last_origin: Option<Point>,
```
Replace with:
```rust
    bounded: Vec<BoundedRow>,
    last_origin: Option<Point>,
```
Find:
```rust
/// Immutable physical scene inputs bound beside each ink lane's instance columns.
pub struct InkScene<'a> {
    pub targets: &'a Targets,
    pub planes: &'a wgpu::Buffer,
}

/// Group 2 for ink: instance columns plus the previous pass's immutable attachments.
fn ink_instance_group(ctx: &GpuCtx, l: &Layouts, buffers: [&wgpu::Buffer; 2], scene: &InkScene) -> wgpu::BindGroup {
    let targets = scene.targets;
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ink.instances.bind_group"),
        layout: &l.ink_instance,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffers[0].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: buffers[1].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&targets.depth_single) },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&targets.depth_msaa) },
            wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&targets.faces_single) },
            wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&targets.faces_msaa) },
            wgpu::BindGroupEntry { binding: 6, resource: scene.planes.as_entire_binding() },
        ],
    })
}

impl InstanceTable {
    /// Current anchored translations, including any replacement after growth/reanchor.
    pub(super) fn translation_buffer(&self) -> &wgpu::Buffer { &self.translations.buf }

    /// Reanchoring is the only translation rewrite outside append/reset, which separately
    /// invalidate the arena filter. Keep f64 bits: distinct anchors may round to equal f32s.
    pub(super) fn translation_origin_bits(&self) -> Option<[u64; 3]> {
        self.last_origin.as_ref().map(|p| [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()])
    }

    /// One placeholder row in both tables, so the first frame binds real buffers.
```
Replace with:
```rust
/// The immutable physical depth bound beside each ink lane's instance columns.
pub struct InkScene<'a> {
    pub targets: &'a Targets,
}

/// Group 2 for ink: the instance columns plus the face pass's depth, both sample counts.
fn ink_instance_group(ctx: &GpuCtx, l: &Layouts, buffers: [&wgpu::Buffer; 2], scene: &InkScene) -> wgpu::BindGroup {
    let targets = scene.targets;
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ink.instances.bind_group"),
        layout: &l.ink_instance,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffers[0].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: buffers[1].as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&targets.depth_single) },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&targets.depth_msaa) },
        ],
    })
}

impl InstanceTable {
    /// One placeholder row in both tables, so the first frame binds real buffers.
```
Find:
```rust
            bounded: Vec::new(),
            local_bounds: Vec::new(),
            occluders: BTreeMap::new(),
            last_origin: None,
```
Replace with:
```rust
            bounded: Vec::new(),
            last_origin: None,
```
Find:
```rust
    /// Refresh sampled attachment and instance bindings after upload, resize, or release.
```
Replace with:
```rust
    /// Refresh the depth and instance bindings after upload, resize, or release.
```
Find:
```rust
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.local_bounds.reserve(up.rows.len());
        for (i, r) in up.rows.iter().enumerate() {
            self.local_bounds.push(r.bounds);
            let world = r.bounds.placed(&r.place);
```
Replace with:
```rust
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        for (i, r) in up.rows.iter().enumerate() {
            let world = r.bounds.placed(&r.place);
```
Find and **delete** the two whole functions `append_occlusion` (from `/// Only real physical face owners and resident cloud chunks enlarge the lookup region.` through its closing `}`) and `occluder_rect` (from `/// A conservative union of physical raster footprints.` through its closing `}`), including the blank line between them.
Find:
```rust
        self.bounded.clear();
        self.local_bounds.clear();
        self.occluders.clear();
        self.buffer.reset();
```
Replace with:
```rust
        self.bounded.clear();
        self.buffer.reset();
```
Find:
```rust
        self.bounded.shrink_to_fit();
        self.local_bounds.shrink_to_fit();
        self.rows.push(Instance::placeholder());
```
Replace with:
```rust
        self.bounded.shrink_to_fit();
        self.rows.push(Instance::placeholder());
```

In `src/engine/gpu/frame.rs`, find:
```rust
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32),
    pub occluder_rect: [f32; 4],
}
```
Replace with:
```rust
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32),
}
```
Find:
```rust
/// The line/pen block (group 1), 80 B; the physical screen rectangle starts at byte 48.
/// `eye` and `anchor` are in the anchored frame the instance rows use.
```
Replace with:
```rust
/// The line/pen block (group 1), 64 B. `eye` and `anchor` are in the anchored frame the
/// instance rows use. Offsets: thickness 0, proj_y 4, ortho_h 8, vp_h 12, vp_w 16, eye 20,
/// anchor 32 (vec3 aligned to 16), feather 44, lit 48, backface 52, pad 56.
```
Find:
```rust
    pub feather: f32, // antialiasing ramp of the ink lanes, px
    pub occluder_rect: [f32; 4], // conservative physical pixel bounds: left, top, right, bottom
    pub lit: f32,       // 1 = light the mesh faces, 0 = flat colour
    pub backface: f32,  // 1 = paint back faces red, 0 = their own colour
    pub _pad: [f32; 2],
}

const _: () = {
    assert!(std::mem::size_of::<LineUniform>() == 80);
    assert!(std::mem::offset_of!(LineUniform, occluder_rect) == 48);
    assert!(std::mem::offset_of!(LineUniform, lit) == 64);
    assert!(std::mem::offset_of!(LineUniform, backface) == 68);
};
```
Replace with:
```rust
    pub feather: f32, // antialiasing ramp of the ink lanes, px
    pub lit: f32,       // 1 = light the mesh faces, 0 = flat colour
    pub backface: f32,  // 1 = paint back faces red, 0 = their own colour
    pub _pad: [f32; 2],
}

const _: () = {
    assert!(std::mem::size_of::<LineUniform>() == 64);
    assert!(std::mem::offset_of!(LineUniform, lit) == 48);
    assert!(std::mem::offset_of!(LineUniform, backface) == 52);
};
```
Find and delete the two lines `occluder_rect: [1.0, 1.0, -1.0, -1.0],` (in `new`) and `occluder_rect: cx.occluder_rect,` (in `write`). Find:
```rust
    /// The exact camera/line buffers shared with the angular compute filter.
    pub(super) fn face_filter_uniforms(&self) -> (&wgpu::Buffer, &wgpu::Buffer) {
        (&self.mvp_buffer, &self.line_buffer)
    }

```
Replace with nothing.

In every shader that declares `struct LineUniform` (`grep -l 'struct LineUniform' src/shaders/*.wgsl` lists `glyph.wgsl`, `grid.wgsl`, `ribbon.wgsl`, `sphere.wgsl`, `triangle.wgsl`), find:
```wgsl
    feather: f32,
    occluder_rect: vec4<f32>,
    lit: f32,
```
Replace with:
```wgsl
    feather: f32,
    lit: f32,
```

In `src/engine/gpu/present.rs`, find:
```rust
        let size = (self.config.width, self.config.height);
        let occluder_rect = self.objects.occluder_rect(&input.view_proj.to_f32(), size, self.view.cloud_size);
        let cx = FrameCx { view: &self.view, anchor: self.objects.anchor_f32(), size, occluder_rect };
```
Replace with:
```rust
        let size = (self.config.width, self.config.height);
        let cx = FrameCx { view: &self.view, anchor: self.objects.anchor_f32(), size };
```

In `src/engine/gpu/render.rs`, find:
```rust
//! The frame list: physical surfaces first, then ink against their immutable depth and exact
//! face identities. The optional picking pass follows the same visibility rule and toggles.
```
Replace with:
```rust
//! The frame list: physical surfaces first, then ink against their immutable depth. The
//! optional picking pass follows the same visibility rule and toggles.
```
Find:
```rust
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> (u32, u32) {
        self.arena.prepare_faces(&self.ctx, encoder, &self.frame, &self.objects);
        self.point_pass(encoder);
```
Replace with:
```rust
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> (u32, u32) {
        self.point_pass(encoder);
```
Find:
```rust
    /// Physical faces, backdrop and cloud resolve establish immutable occlusion before ink.
```
Replace with:
```rust
    /// Backdrop, physical faces and the cloud resolve write the depth every ink fragment reads.
```

In `src/engine/gpu/mod.rs`, find:
```rust
pub mod device;
mod face_filter;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
mod occlusion_bounds;
pub mod pick;
mod plane_place;
pub mod present;
```
Replace with:
```rust
pub mod device;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
pub mod pick;
pub mod present;
```
Replace every `&InkScene { targets: &self.targets, planes: self.arena.face_plane_buffer() }` with `&InkScene { targets: &self.targets }` (three places: `set_scene`, `retarget`, `release`), and in `build` replace `&InkScene { targets: &targets, planes: arena.face_plane_buffer() }` with `&InkScene { targets: &targets }`. Find:
```rust
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.objects.append_occlusion(up);
        self.arena.append(&self.ctx, &self.layouts, &up.arena);
```
Replace with:
```rust
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
```
Find `self.arena.release(&self.ctx, &self.layouts);` and replace with `self.arena.release(&self.ctx);`. Find:
```rust
    out.extend_from_slice(arena::SHADERS);
    out.extend_from_slice(face_filter::SHADERS);
    out.extend_from_slice(segments::SHADERS);
```
Replace with:
```rust
    out.extend_from_slice(arena::SHADERS);
    out.extend_from_slice(segments::SHADERS);
```

In `src/engine/gpu/upload.rs`, find:
```rust
impl Upload {
    /// Bake fixed face-plane rotation/scale once while these fresh delta rows still have placements.
    pub fn place_face_planes(&mut self, object_base: u32) {
        for plane in &mut self.arena.face_planes {
            let local = plane.instance_id.checked_sub(object_base).expect("face belongs to an earlier upload");
            let object = self.obj.rows.get(local as usize).expect("face instance absent from upload");
            super::plane_place::bake(plane, &object.place);
        }
    }

    /// Forget the uploaded rows and hand their allocations back: the GPU is their only holder now.
```
Replace with:
```rust
impl Upload {
    /// Forget the uploaded rows and hand their allocations back: the GPU is their only holder now.
```
Delete the whole `#[cfg(test)] mod tests { ... }` block at the end of `upload.rs` (its only test placed face planes).

- [ ] **Step 6: The walk context and the scene bases**

In `src/app/walk/mod.rs`, find:
```rust
pub struct WalkCx {
    pub vert_base: u32,
    pub face_base: u32,
    pub cloud_px: f32,
    pub row: u32,
}
```
Replace with:
```rust
pub struct WalkCx {
    pub vert_base: u32,
    pub cloud_px: f32,
    pub row: u32,
}
```
In `src/app/walk/mesh.rs`, find `    arena.face_ids.resize(arena.face_ids.len() + rm.vertices.len(), 0);` and delete that line. In `src/app/walk/mesh_ink.rs` (the test), find `let cx = WalkCx { vert_base: 50, face_base: 100, cloud_px: 0.0, row: 7 };` and replace with `let cx = WalkCx { vert_base: 50, cloud_px: 0.0, row: 7 };`, and delete the line `assert_eq!(arena.face_ids.len(), arena.verts.len());`.

In `src/app/scene.rs`, find:
```rust
struct Bases {
    vert: u32,
    face: u32,
    ribbon: u32,
    obj: u32,
}
```
Replace with:
```rust
struct Bases {
    vert: u32,
    ribbon: u32,
    obj: u32,
}
```
Find:
```rust
        self.tables.place_face_planes(self.bases.obj);
        gpu.set_scene(&self.tables);
        self.bases.vert += self.tables.arena.verts.len() as u32;
        self.bases.face += self.tables.arena.face_planes.len() as u32;
```
Replace with:
```rust
        gpu.set_scene(&self.tables);
        self.bases.vert += self.tables.arena.verts.len() as u32;
```
Find `let cx = WalkCx { vert_base: self.bases.vert, face_base: self.bases.face, cloud_px: point_px, row };` and replace with `let cx = WalkCx { vert_base: self.bases.vert, cloud_px: point_px, row };`.

- [ ] **Step 7: The mirror tests**

In `src/engine/gpu/instance.rs`, replace the whole `shader_validation_and_layouts` test function with:
```rust
    /// Validate the actual shader modules and every storage member offset, not merely field
    /// names: a valid Rust size alone does not prove WGSL's array stride.
    #[test]
    fn shader_validation_and_layouts() {
        use crate::engine::gpu::segments::CylinderSegment;
        use crate::engine::gpu::glyphs::GlyphPoint;
        use std::mem::{offset_of, size_of};
        for (name, source) in lane_shaders() {
            let source = if source.contains("-> InkColor") {
                format!("{source}\n{}", include_str!("../../shaders/ink_visibility.wgsl"))
            } else { source.to_string() };
            let module = naga::front::wgsl::parse_str(&source).unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::default())
                .validate(&module).unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            for (_, ty) in module.types.iter() {
                let Some(structure) = ty.name.as_deref() else { continue };
                let (offsets, size) = match structure {
                    "CylinderSegment" => (vec![0, 4, 8, offset_of!(CylinderSegment, radius), 16, 20, 24,
                        offset_of!(CylinderSegment, instance_id), offset_of!(CylinderSegment, color), offset_of!(CylinderSegment, facing),
                        offset_of!(CylinderSegment, support_start), offset_of!(CylinderSegment, support_count)], size_of::<CylinderSegment>()),
                    "GlyphPoint" => (vec![offset_of!(GlyphPoint, center), offset_of!(GlyphPoint, radius), offset_of!(GlyphPoint, color),
                        offset_of!(GlyphPoint, instance_id), offset_of!(GlyphPoint, facing), offset_of!(GlyphPoint, facing_ext),
                        offset_of!(GlyphPoint, support_start), offset_of!(GlyphPoint, support_count), offset_of!(GlyphPoint, _pad)], size_of::<GlyphPoint>()),
                    "LineUniform" => (vec![0, 4, 8, 12, 16, 20, 24, 28, 32, 44, offset_of!(LineUniform, lit), offset_of!(LineUniform, backface)], size_of::<LineUniform>()),
                    _ => continue,
                };
                let naga::TypeInner::Struct { members, span } = &ty.inner else { panic!("{name}: {structure} is not a struct") };
                assert_eq!(*span as usize, size, "{name}: {structure} stride");
                assert_eq!(members.len(), offsets.len(), "{name}: {structure} members");
                for (member, expected) in members.iter().zip(offsets) {
                    assert_eq!(member.offset as usize, expected, "{name}: {structure}.{:?}", member.name);
                }
            }
        }
    }
```
And in `line_uniform_mirror`, find:
```rust
        let rust = ["thickness", "proj_y", "ortho_h", "vp_h", "vp_w", "eye_x", "eye_y", "eye_z", "anchor", "feather", "occluder_rect", "lit", "backface"];
```
Replace with:
```rust
        let rust = ["thickness", "proj_y", "ortho_h", "vp_h", "vp_w", "eye_x", "eye_y", "eye_z", "anchor", "feather", "lit", "backface"];
```
and `assert_eq!(std::mem::size_of::<LineUniform>(), 80);` becomes `assert_eq!(std::mem::size_of::<LineUniform>(), 64);`.

- [ ] **Step 8: Build, test, gate, probes**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
cargo xtest 2>&1 | tail -5
cargo check 2>&1 | tail -3
cargo clippy --release --all-targets --target x86_64-unknown-linux-gnu 2>&1 | grep -c '^warning\|^error' || true
docs/_gate.sh
grep -rn 'face_ids\|face_planes\|FacePlane\|occluder\|face_filter\|plane_place\|faces_single\|face_target' src examples | grep -v '^examples/check_determinism' || echo "no references left"
```
Expected: tests pass, wasm check finishes, clippy count is 0, `gate OK`, `no references left`. `examples/check_determinism.rs` still names `arena.face_ids`; Task 5 fixes it. Repeat Task 2 step 6 (the probe subset); every case must still print `magenta 0`.

- [ ] **Step 9: Commit**

```bash
git add -A src
git commit -m "viewer: delete the face-identity engine

No face token attachment, no per-vertex face ids, no plane table, no
angular compute filter, no occluder rectangle. The face pass writes
depth and colour; the ink layout binds the depth twice, one view per
sample count. LineUniform is 64 B again."
```

---

### Task 5: Rows shrink back to 40 B and 48 B

**Files:**
- Modify: `src/engine/gpu/segments.rs`, `src/engine/gpu/glyphs.rs`, `src/engine/pipelines/layouts.rs`, `src/shaders/ribbon.wgsl`, `src/shaders/sphere.wgsl`, `src/shaders/glyph.wgsl`, `src/engine/gpu/instance.rs`, `src/app/walk/mesh_ink.rs`, `src/app/walk/curves.rs`, `src/app/walk/frames.rs`, `src/app/walk/points.rs`, `examples/check_determinism.rs`

**Interfaces:**
- Produces: `CylinderSegment { p0: [f32; 3], radius: f32, p1: [f32; 3], instance_id: u32, color: u32, facing: u32 }` (40 B); `GlyphPoint { center: [f32; 3], radius: f32, color: [f32; 4], instance_id: u32, facing: u32, facing_ext: [u32; 2] }` (48 B); `SegRows { pipes, ribbons }`; `GlyphRows { spheres, dots }`; group 3 of the ink layout binds one storage buffer at binding 0.

- [ ] **Step 1: The segment lane**

Replace the whole content of `src/engine/gpu/segments.rs` with:
```rust
//! The segment lane: every straight piece of ink. Two tables of the same 40 B row - pipes
//! (mesh/BRep edges, the SOLID lane, culled by facing) and ribbons (line/polyline/curve, the
//! FLAT lane, always drawn) - through one blended camera-facing quad. `SegRows` is one upload.

use crate::engine::pipelines::{build, ink_module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Vertices per ribbon: two triangles pulled by vertex index, no vertex buffer.
const RIBBON_VERTS: u32 = 6;

/// One segment row, 40 B, the layout ribbon.wgsl declares. The ends are flat f32s: a `vec3`
/// would pad the row to 48 B. Offsets: p0 0, radius 12, p1 16, instance_id 28, color 32,
/// facing 36.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    pub p0: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub p1: [f32; 3],
    pub instance_id: u32,
    /// RGBA8, low byte red.
    pub color: u32,
    /// Two oct16 adjacent face normals; `FACING_UNKNOWN` = no adjacency, always drawn.
    pub facing: u32,
}

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>,
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.ribbons);
    }
}

/// One segment table on the GPU with the group 3 that binds it.
struct SegTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl SegTable {
    /// A one-row table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<CylinderSegment>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Rebind after the backing buffer changed.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf]);
    }
}

/// The pipelines over the two tables: the same blended quad for both, and its id twin.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the shader, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    shader: wgpu::ShaderModule,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the shader and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl"));
        let gpu = build_pipelines(ctx, l, &shader, target);
        let pipes = SegTable::new(ctx, l, "pipes");
        let ribbons = SegTable::new(ctx, l, "ribbons");
        Self { pipes, ribbons, shader, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        if self.pipes.buf.append(ctx, &up.pipes) {
            self.pipes.rebind(ctx, l);
        }
        if self.ribbons.buf.append(ctx, &up.ribbons) {
            self.ribbons.rebind(ctx, l);
        }
    }

    /// Mesh/BRep edges: camera-facing quads against physical depth.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes)
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }

    /// The id pass for the solid lane: opaque quads.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes)
    }

    /// The id pass for the flat lane: opaque quads.
    pub fn draw_ribbon_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.ribbons)
    }

    /// One table as ribbons through `pipeline`; 0 draws when empty.
    fn draw_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline, table: &SegTable) -> u32 {
        if table.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &table.group, &[]);
        pass.draw(0..RIBBON_VERTS * table.buf.len(), 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.pipes.buf.reset();
        self.ribbons.buf.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.pipes.buf.release(ctx);
        self.ribbons.buf.release(ctx);
        self.pipes.rebind(ctx, l);
        self.ribbons.rebind(ctx, l);
    }

    /// Solid-lane rows on the GPU - the MSAA policy reads it.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.buf.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.buf.len()
    }
}

/// Both segment pipelines read physical visibility without writing or biasing that depth.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended)),
        id_ribbon: build(dev, Target::ID, &quad.with("ribbon.id", "fs_id")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// ribbon.wgsl reads the 40 B segment row (ends as scalars).
    #[test]
    fn cylinder_segment_mirror() {
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), 36);
    }
}
```

- [ ] **Step 2: The glyph lane**

In `src/engine/gpu/glyphs.rs`, find:
```rust
use crate::engine::pipelines::{build, ink_module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::segments::InkSupport;
use super::upload::drop_rows;
```
Replace with:
```rust
use crate::engine::pipelines::{build, ink_module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
```
Find:
```rust
/// One marker or dot row, 64 B, the layout sphere.wgsl and glyph.wgsl declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    /// Up to SIX incident face normals as oct16 pairs, widest edge's two first;
    /// `FACING_UNKNOWN` = no adjacency / no more.
    pub facing: u32,
    pub facing_ext: [u32; 2],
    pub support_start: u32,
    pub support_count: u32,
    pub _pad: [u32; 2],
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 64);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>,
    pub dots: Vec<GlyphPoint>,
    pub supports: Vec<InkSupport>,
}

impl GlyphRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
        drop_rows(&mut self.supports);
    }
}
```
Replace with:
```rust
/// One marker or dot row, 48 B, the layout sphere.wgsl and glyph.wgsl declare. Offsets:
/// center 0 (vec3, 16-aligned), radius 12, color 16, instance_id 32, facing 36, facing_ext 40.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    /// Up to SIX incident face normals as oct16 pairs, widest edge's two first;
    /// `FACING_UNKNOWN` = no adjacency / no more.
    pub facing: u32,
    pub facing_ext: [u32; 2],
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>,
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
    }
}
```
Find:
```rust
impl GlyphTable {
    /// A one-row table sharing the lane's exact support identities.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str, supports: &GrowBuf) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf, &supports.buf]);
        Self { label, buf, group }
    }

    /// Rebind both tables after either backing buffer changes.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, supports: &GrowBuf) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf, &supports.buf]);
    }

}
```
Replace with:
```rust
impl GlyphTable {
    /// A one-row table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Rebind after the backing buffer changed.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf]);
    }
}
```
Find:
```rust
pub struct GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    supports: GrowBuf,
    template: Template,
```
Replace with:
```rust
pub struct GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    template: Template,
```
Find:
```rust
        let gpu = build_pipelines(ctx, l, &shaders, target);

        let supports = GrowBuf::new(ctx, "glyphs.supports", std::mem::size_of::<InkSupport>() as u64, ROWS);
        let spheres = GlyphTable::new(ctx, l, "spheres", &supports);
        let dots = GlyphTable::new(ctx, l, "dots", &supports);
        Self { spheres, dots, supports, template, shaders, gpu }
    }
```
Replace with:
```rust
        let gpu = build_pipelines(ctx, l, &shaders, target);
        let spheres = GlyphTable::new(ctx, l, "spheres");
        let dots = GlyphTable::new(ctx, l, "dots");
        Self { spheres, dots, template, shaders, gpu }
    }
```
Find:
```rust
    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        let base = self.supports.len();
        let supports_grew = self.supports.append(ctx, &up.supports);
        let spheres = rebase_supports(&up.spheres, base);
        let dots = rebase_supports(&up.dots, base);
        let spheres_grew = self.spheres.buf.append(ctx, &spheres);
        let dots_grew = self.dots.buf.append(ctx, &dots);
        if supports_grew || spheres_grew {
            self.spheres.rebind(ctx, l, &self.supports);
        }
        if supports_grew || dots_grew {
            self.dots.rebind(ctx, l, &self.supports);
        }
    }
```
Replace with:
```rust
    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        if self.spheres.buf.append(ctx, &up.spheres) {
            self.spheres.rebind(ctx, l);
        }
        if self.dots.buf.append(ctx, &up.dots) {
            self.dots.rebind(ctx, l);
        }
    }
```
Find:
```rust
    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.spheres.buf.reset();
        self.dots.buf.reset();
        self.supports.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.buf.release(ctx);
        self.dots.buf.release(ctx);
        self.supports.release(ctx);
        self.spheres.rebind(ctx, l, &self.supports);
        self.dots.rebind(ctx, l, &self.supports);
    }
```
Replace with:
```rust
    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.spheres.buf.reset();
        self.dots.buf.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.buf.release(ctx);
        self.dots.buf.release(ctx);
        self.spheres.rebind(ctx, l);
        self.dots.rebind(ctx, l);
    }
```
Find and **delete** the whole `rebase_supports` function (from `/// Rebase upload-local support ranges while preserving the caller's append-only rows.` through its closing `}`). Find:
```rust
    /// sphere.wgsl and glyph.wgsl read the same 64 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext", "support_start", "support_count", "_pad"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 64);
        assert_eq!(std::mem::offset_of!(GlyphPoint, support_start), 48);
        assert_eq!(std::mem::offset_of!(GlyphPoint, support_count), 52);
        assert_eq!(std::mem::offset_of!(GlyphPoint, _pad), 56);
    }
```
Replace with:
```rust
    /// sphere.wgsl and glyph.wgsl read the same 48 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
        assert_eq!(std::mem::offset_of!(GlyphPoint, facing_ext), 40);
    }
```

- [ ] **Step 3: The layout, the shaders and the mirror test**

In `src/engine/pipelines/layouts.rs`, find:
```rust
/// Exact support identities are a separate table shared by both representations of a lane.
fn ink_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.rows.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            buffer_entry(1, wgpu::ShaderStages::FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
        ],
    })
}
```
Replace with:
```rust
/// The ink lanes' row table, read by the vertex stage (and the fragment stage of the id pass).
fn ink_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.rows.layout"),
        entries: &[buffer_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true })],
    })
}
```

In `src/shaders/ribbon.wgsl`, find:
```wgsl
    color: u32,
    facing: u32,
    support_start: u32,
    support_count: u32,
}
```
Replace with:
```wgsl
    color: u32,
    facing: u32,
}
```
In `src/shaders/sphere.wgsl` and `src/shaders/glyph.wgsl`, find:
```wgsl
    facing: u32,
    facing_ext: vec2<u32>,
    support_start: u32,
    support_count: u32,
    _pad: vec2<u32>,
};
```
Replace with:
```wgsl
    facing: u32,
    facing_ext: vec2<u32>,
};
```

In `src/engine/gpu/instance.rs` (`shader_validation_and_layouts`), find:
```rust
                    "CylinderSegment" => (vec![0, 4, 8, offset_of!(CylinderSegment, radius), 16, 20, 24,
                        offset_of!(CylinderSegment, instance_id), offset_of!(CylinderSegment, color), offset_of!(CylinderSegment, facing),
                        offset_of!(CylinderSegment, support_start), offset_of!(CylinderSegment, support_count)], size_of::<CylinderSegment>()),
                    "GlyphPoint" => (vec![offset_of!(GlyphPoint, center), offset_of!(GlyphPoint, radius), offset_of!(GlyphPoint, color),
                        offset_of!(GlyphPoint, instance_id), offset_of!(GlyphPoint, facing), offset_of!(GlyphPoint, facing_ext),
                        offset_of!(GlyphPoint, support_start), offset_of!(GlyphPoint, support_count), offset_of!(GlyphPoint, _pad)], size_of::<GlyphPoint>()),
```
Replace with:
```rust
                    "CylinderSegment" => (vec![0, 4, 8, offset_of!(CylinderSegment, radius), 16, 20, 24,
                        offset_of!(CylinderSegment, instance_id), offset_of!(CylinderSegment, color), offset_of!(CylinderSegment, facing)], size_of::<CylinderSegment>()),
                    "GlyphPoint" => (vec![offset_of!(GlyphPoint, center), offset_of!(GlyphPoint, radius), offset_of!(GlyphPoint, color),
                        offset_of!(GlyphPoint, instance_id), offset_of!(GlyphPoint, facing), offset_of!(GlyphPoint, facing_ext)], size_of::<GlyphPoint>()),
```

- [ ] **Step 4: The producers**

In `src/app/walk/mesh_ink.rs` remove the two lines `support_start: 0,` and `support_count: 0,` from the `CylinderSegment { .. }` literal in `push_pipes`, and the three lines `support_start: 0,`, `support_count: 0,`, `_pad: [0; 2],` from the `GlyphPoint { .. }` literal in `push_markers`.

In `src/app/walk/curves.rs`, find:
```rust
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN, support_start: 0, support_count: 0 });
```
Replace with:
```rust
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
```
Find:
```rust
    seg.ribbons.push(CylinderSegment { p0, radius: encode_width(l.width), p1, instance_id: row, color: pack_rgba(l.linecolor.to_f32()), facing: FACING_UNKNOWN, support_start: 0, support_count: 0 });
```
Replace with:
```rust
    seg.ribbons.push(CylinderSegment { p0, radius: encode_width(l.width), p1, instance_id: row, color: pack_rgba(l.linecolor.to_f32()), facing: FACING_UNKNOWN });
```
In `src/app/walk/frames.rs`, find:
```rust
        seg.ribbons.push(CylinderSegment { p0: pts[i], radius: pen.radius, p1: pts[j], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN, support_start: 0, support_count: 0 });
```
Replace with:
```rust
        seg.ribbons.push(CylinderSegment { p0: pts[i], radius: pen.radius, p1: pts[j], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
```
In `src/app/walk/points.rs`, find:
```rust
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
        support_start: 0,
        support_count: 0,
        _pad: [0; 2],
    });
```
Replace with:
```rust
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
    });
```

In `src/selftest/lifecycle.rs`, find:
```rust
//! Exercise visibility attachments, support-buffer rebasing and picking on one live device.
```
Replace with:
```rust
//! Exercise the depth attachment bindings, row appends and picking on one live device.
```

In `examples/check_determinism.rs`, find:
```rust
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
        same!(arena.face_ids); same!(arena.face_planes); same!(seg.supports); same!(glyph.supports);
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
```
Replace with:
```rust
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
```

- [ ] **Step 5: Build, test, every native example**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
cargo xtest 2>&1 | tail -5
cargo check 2>&1 | tail -3
cargo clippy --release --all-targets --target x86_64-unknown-linux-gnu 2>&1 | grep -c '^warning\|^error' || true
docs/_gate.sh
T=x86_64-unknown-linux-gnu
cargo build -q --release --target $T --examples
B=$CARGO_TARGET_DIR/$T/release/examples
$B/check_hidden_line_lifecycle $SCRATCH/lifecycle $SCRATCH/probes/regular.pb $SCRATCH/probes/warped.pb $SCRATCH/probes/authored.pb 2>&1 | tail -3
$B/check_determinism $SCRATCH/probes/regular.pb $SCRATCH/probes/warped.pb assets/pb/view_local_boxes.pb 2>&1 | tail -3
grep -rn 'support_start\|support_count\|InkSupport\|supports' src examples | grep -v '^docs' || echo "no references left"
```
Expected: tests pass, wasm check finishes, clippy count 0, `gate OK`, `lifecycle OK: ...`, determinism reports no failures, `no references left`.

- [ ] **Step 6: Commit**

```bash
git add -A src examples/check_determinism.rs
git commit -m "viewer: segment rows back to 40 B, glyph rows to 48 B

The support tables and their bind group binding are gone; group 3 of
an ink lane is its row table alone."
```

---

### Task 6: Smooth tessellations ink border and crease edges only

**Files:**
- Modify: `src/shaders/ribbon.wgsl`, `src/app/walk/brep.rs`
- Create: `examples/mk_brep_probe.rs`, `docs/_orbit_check.py`

**Interfaces:**
- Produces: `mk_brep_probe <out.pb>` writes a grey plate with a BRep cylinder and a BRep cone standing on it; `BREP_PROBE_FLIPPED=1` reverses the orientation of the first two face uses of the cylinder before writing. `docs/_orbit_check.py <selftest> <scene.pb> <out_dir>` renders 36 orbits and reports the black-pixel series.

- [ ] **Step 1: Drop the silhouette term**

In `src/shaders/ribbon.wgsl`, find:
```wgsl
// On a tessellated surface, ink an edge only where the surface ends, where it genuinely creases,
// or where the two faces straddle the eye direction and the edge IS the silhouette. Everything
// between is a seam that would draw the sampling grid instead of the shape. `pack_facing` gives
// a one-faced border edge the same code twice, which is how a border is told from a seam.
fn is_feature_edge(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if ((facing & 0xffffu) == (facing >> 16u)) {
        return true;
    }
    if (dot(n0, n1) < CREASE_COS) {
        return true;
    }
    return (dot(n0, to_eye) > 0.0) != (dot(n1, to_eye) > 0.0);
}
```
Replace with:
```wgsl
// On a tessellated surface, ink an edge only where the surface ends or genuinely creases.
// Everything between is a seam that would draw the sampling grid instead of the shape, and a
// silhouette decided per segment from two packed normals flips as the camera turns, so it is
// not drawn either. `pack_facing` gives a one-faced border edge the same code twice, which is
// how a border is told from a seam.
fn is_feature_edge(facing: u32, n0: vec3<f32>, n1: vec3<f32>) -> bool {
    if ((facing & 0xffffu) == (facing >> 16u)) {
        return true;
    }
    return dot(n0, n1) < CREASE_COS;
}
```
Find:
```wgsl
        if ((inst.flags & FLAG_SMOOTH) != 0u && !is_feature_edge(seg.facing, n0, n1, to_eye)) {
```
Replace with:
```wgsl
        if ((inst.flags & FLAG_SMOOTH) != 0u && !is_feature_edge(seg.facing, n0, n1)) {
```

In `src/app/walk/brep.rs`, find:
```rust
    // The tessellation is a FILL. Its triangle edges are an artifact of meshing, and the ink
    // pass decides visibility from adjacent triangle normals - so near-coplanar triangles
    // flip in and out as the camera turns, which is the flicker. Width 0 hides them
    // (mesh_ink::hidden); the solid's real edges come off the BRep below.
```
Replace with:
```rust
    // The tessellation is a FILL. Its triangle edges are an artifact of meshing: width 0
    // hides them (mesh_ink::hidden); the solid's real edges come off the BRep below.
```

- [ ] **Step 2: The BRep probe**

Create `examples/mk_brep_probe.rs`:
```rust
// BRep probe: a grey plate with a BRep cylinder and a BRep cone standing on it, edges black.
// BREP_PROBE_FLIPPED=1 reverses the orientation of the cylinder's first two face uses, so a
// render pair proves the ink does not depend on face orientation.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_brep_probe -- <out.pb>
use session_rust::brep::{brep_reverse, BRep};
use session_rust::{Color, Mesh, Session, Xform};

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/brep_probe.pb".into());
    let mut plate = Mesh::create_box(1600.0, 1000.0, 40.0);
    plate.transform(&Xform::translation(0.0, 0.0, -20.0));
    plate.set_objectcolor(Color::grey());
    let count = plate.edges_with_colors().len();
    plate.set_linecolors(vec![Color::black(); count], vec![0.0; count]);

    let mut cylinder = BRep::create_cylinder(150.0, 400.0);
    cylinder.transform(&Xform::translation(-350.0, 0.0, 0.0));
    if std::env::var("BREP_PROBE_FLIPPED").is_ok() {
        for face in cylinder.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }
    }
    let mut cone = BRep::create_cone(150.0, 400.0);
    cone.transform(&Xform::translation(350.0, 0.0, 0.0));

    let mut s = Session::new("brep_probe");
    s.add_mesh(plate, None);
    s.add_brep(cylinder, None);
    s.add_brep(cone, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```
If `session_rust::brep::brep_reverse` is not exported at the crate root path used above, check `../session_rust/src/lib.rs` for the module's visibility and adjust the `use` line to the path it exports (the function is `pub fn brep_reverse(o: BRepOrientation) -> BRepOrientation` in `session_rust/src/brep.rs`).

- [ ] **Step 3: The orbit check**

Create `docs/_orbit_check.py`:
```python
#!/usr/bin/env python3
"""_orbit_check.py <selftest> <scene.pb> <out_dir>  ->  black-pixel series over 36 orbits

Renders the scene at 36 yaw steps of 10 degrees (35 orbit units at 0.005 rad each), counts
near-black pixels (every channel under 60) per frame, and fails when a frame differs from
either neighbour by more than 5% of the series mean: ink that flips with the view."""
import os, pathlib, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

STEP = 35
FRAMES = 36


def black_pixels(path):
    w, h, px = read_ppm(path)
    return sum(1 for k in range(0, w * h * 3, 3) if px[k] < 60 and px[k + 1] < 60 and px[k + 2] < 60)


def main():
    binary, scene, out = sys.argv[1], sys.argv[2], pathlib.Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    base = {k: v for k, v in os.environ.items() if not k.startswith("VIEWER_")}
    series = []
    for i in range(FRAMES):
        ppm = out / f"orbit_{i:02d}.ppm"
        env = dict(base, VIEWER_W="1000", VIEWER_H="700", VIEWER_NO_GRID="1", VIEWER_ORBIT=f"{i * STEP},0")
        subprocess.run([binary, str(ppm), scene], env=env, check=True, capture_output=True)
        series.append(black_pixels(ppm))
    mean = sum(series) / len(series)
    worst = 0.0
    for i, value in enumerate(series):
        for j in (i - 1, (i + 1) % FRAMES):
            worst = max(worst, abs(value - series[j]) / mean)
    print("black pixels per orbit:", series)
    print(f"mean {mean:.0f}, worst neighbour change {100 * worst:.1f}%")
    if worst > 0.05:
        print("FAIL: ink flips with the view")
        sys.exit(1)
    print("orbit OK")


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: Run it, and the orientation pair**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
T=x86_64-unknown-linux-gnu
cargo xtest 2>&1 | tail -3
cargo build -q --release --target $T --example selftest --example mk_brep_probe
B=$CARGO_TARGET_DIR/$T/release/examples
$B/mk_brep_probe $SCRATCH/brep_ok.pb
BREP_PROBE_FLIPPED=1 $B/mk_brep_probe $SCRATCH/brep_flipped.pb
python3 docs/_orbit_check.py $B/selftest $SCRATCH/brep_ok.pb $SCRATCH/orbit_ok
python3 docs/_orbit_check.py $B/selftest $SCRATCH/brep_flipped.pb $SCRATCH/orbit_flipped
python3 - <<'PY'
import sys; sys.path.insert(0, 'docs')
from _count_colors import read_ppm
import os
root = os.environ['SCRATCH']
bad = 0
for i in range(36):
    a = read_ppm(f"{root}/orbit_ok/orbit_{i:02d}.ppm")[2]
    b = read_ppm(f"{root}/orbit_flipped/orbit_{i:02d}.ppm")[2]
    diff = sum(1 for k in range(0, len(a), 3)
               if (a[k] < 60 and a[k+1] < 60 and a[k+2] < 60) != (b[k] < 60 and b[k+1] < 60 and b[k+2] < 60))
    bad += diff
print("black pixels differing between ok and flipped over 36 orbits:", bad)
sys.exit(1 if bad else 0)
PY
```
Expected: both runs print `orbit OK`, and the pair comparison prints `... 0`. Look at `$SCRATCH/orbit_ok/orbit_00.ppm` (convert to PNG as in Task 2): the cylinder shows its two cap circles and no side seams; the cone its base circle. If seams appear, `FLAG_SMOOTH` is not reaching the BRep row; if a cap circle is missing, the crease threshold is being applied to a border (`pack_facing` duplicates a lone face's code, check `facing & 0xffff == facing >> 16`).

- [ ] **Step 5: Commit**

```bash
git add src/shaders/ribbon.wgsl src/app/walk/brep.rs examples/mk_brep_probe.rs docs/_orbit_check.py
git commit -m "viewer: smooth tessellations ink borders and creases only

The per-segment silhouette test flipped seams in and out as the camera
turned. A BRep probe and a 36-orbit check pin the stable result and
prove the ink ignores face orientation."
```

---

### Task 7: The existing verification suite, as one script

**Files:**
- Create: `docs/_probe_matrix.py`, `docs/_ink_suite.sh`

**Interfaces:**
- Produces: `docs/_probe_matrix.py <selftest> <mk_hidden_line_probe> <out_dir>` renders the 108-case matrix and exits nonzero on any magenta pixel; `docs/_ink_suite.sh` runs every check of spec section 7 that exists at this point and prints one `PASS`/`FAIL` line per check. Task 8 appends its checks to the suite.

- [ ] **Step 1: The probe matrix script**

Create `docs/_probe_matrix.py`:
```python
#!/usr/bin/env python3
"""_probe_matrix.py <selftest> <mk_hidden_line_probe> <out_dir>  ->  108 hidden-line renders

Three fixtures (regular, warped, authored) x three cameras (top orthographic, down, iso) x
three distances (1, 4, 16) x MSAA (1, 4). Every render must have zero magenta pixels (hidden
ink showing) and at least 500 blue pixels (visible ink retained)."""
import json, os, pathlib, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

CAMERAS = [("top", {"VIEWER_VIEW": "top"}), ("down", {"VIEWER_ORBIT": "0,209"}), ("iso", {})]


def counts(path):
    w, h, px = read_ppm(path)
    blue = magenta = 0
    for k in range(0, w * h * 3, 3):
        r, g, b = px[k], px[k + 1], px[k + 2]
        if g <= 60 and b >= 195:
            if r <= 60:
                blue += 1
            elif r >= 195:
                magenta += 1
    return blue, magenta


def main():
    selftest, maker, out = sys.argv[1], sys.argv[2], pathlib.Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    base = {k: v for k, v in os.environ.items() if not k.startswith(("VIEWER_", "HIDDEN_LINE_"))}
    fixtures = {"regular": {}, "warped": {"HIDDEN_LINE_PROBE_WARPED": "1"}, "authored": {"HIDDEN_LINE_PROBE_AUTHORED": "1"}}
    for name, flags in fixtures.items():
        subprocess.run([maker, str(out / f"{name}.pb")], env=dict(base, **flags), check=True, capture_output=True)
    results, failures = [], 0
    for fixture in fixtures:
        for camera, settings in CAMERAS:
            for distance in (1, 4, 16):
                for msaa in (1, 4):
                    stem = f"{fixture}_{camera}_{distance}_{msaa}"
                    ppm = out / f"{stem}.ppm"
                    env = dict(base, VIEWER_W="1400", VIEWER_H="900", VIEWER_NO_GRID="1", VIEWER_DISTANCE_SCALE=str(distance), VIEWER_MSAA=str(msaa), **settings)
                    run = subprocess.run([selftest, str(ppm), str(out / f"{fixture}.pb")], env=env, capture_output=True, text=True)
                    (out / f"{stem}.log").write_text(run.stdout + run.stderr)
                    run.check_returncode()
                    blue, magenta = counts(ppm)
                    ok = magenta == 0 and blue >= 500
                    failures += not ok
                    results.append(dict(case=stem, blue=blue, magenta=magenta, ok=ok))
                    print(f"{'ok  ' if ok else 'FAIL'} {stem}: blue {blue} magenta {magenta}")
    (out / "results.json").write_text(json.dumps(results, indent=2))
    print(f"{len(results)} cases, {failures} failures")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: The suite**

Create `docs/_ink_suite.sh`:
```bash
#!/usr/bin/env bash
# Every check the ink rule must pass, in one run. Prints PASS/FAIL per check and exits nonzero
# on the first failure. The floor census needs the floor model, fetched from the bucket into
# $SCRATCH/pb when absent (INK_SUITE_NO_FETCH=1 skips that check instead).
#
#   docs/_ink_suite.sh
set -uo pipefail
cd "$(dirname "$0")/.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}" REGEN_PROTO=0
T=x86_64-unknown-linux-gnu
B=$CARGO_TARGET_DIR/$T/release/examples
OUT="${INK_SUITE_OUT:-${SCRATCH:-/tmp}/ink_suite}"
mkdir -p "$OUT"
DATA=https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev

check() {
    local name=$1; shift
    if "$@" > "$OUT/$name.log" 2>&1; then echo "PASS $name"; else echo "FAIL $name (see $OUT/$name.log)"; exit 1; fi
}

check build cargo build -q --release --target $T --examples
check xtest cargo xtest -q
check wasm cargo check -q --target wasm32-unknown-unknown
check clippy cargo clippy -q --release --all-targets --target $T -- -D warnings
check gate docs/_gate.sh
check probe_matrix python3 docs/_probe_matrix.py "$B/selftest" "$B/mk_hidden_line_probe" "$OUT/matrix"
check lifecycle "$B/check_hidden_line_lifecycle" "$OUT/lifecycle" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" "$OUT/matrix/authored.pb"
check determinism "$B/check_determinism" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" assets/pb/view_local_boxes.pb
check closeup env VIEWER_W=1400 VIEWER_H=900 VIEWER_ZOOM=5 VIEWER_MSAA=4 "$B/selftest" "$OUT/close.ppm" assets/pb/view_local_boxes.pb
"$B/mk_brep_probe" "$OUT/brep_ok.pb" > /dev/null
BREP_PROBE_FLIPPED=1 "$B/mk_brep_probe" "$OUT/brep_flipped.pb" > /dev/null
check orbit_ok python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_ok.pb" "$OUT/orbit_ok"
check orbit_flipped python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_flipped.pb" "$OUT/orbit_flipped"

FLOOR="${SCRATCH:-/tmp}/pb/view_mixed_floor_model.pb"
if [ ! -f "$FLOOR" ] && [ -z "${INK_SUITE_NO_FETCH:-}" ]; then
    mkdir -p "$(dirname "$FLOOR")" && curl -sS -o "$FLOOR" "$DATA/pb/view_mixed_floor_model.pb"
fi
if [ -f "$FLOOR" ]; then
    check floor_census python3 docs/_hidden_line_matrix.py "$B/selftest" "$B/census_plates" "$FLOOR" "$OUT/floor" --require-zero
else
    echo "SKIP floor_census (no floor model)"
fi
echo "ink suite OK"
```
Make it executable: `chmod +x docs/_ink_suite.sh docs/_probe_matrix.py docs/_orbit_check.py`.

- [ ] **Step 3: Run the suite**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
docs/_ink_suite.sh
```
Expected: `PASS` for every check and `ink suite OK`. Total runtime is dominated by the 108-render matrix and the floor census (about 20 minutes on the iGPU). A `FAIL` on `floor_census` names the camera and scale in `$OUT/floor/matrix.json`; render that camera alone with `VIEWER_IDS` and `CENSUS_RECOLOR` (both documented at the top of `examples/census_plates.rs`) before touching any constant.

Also confirm the close-up by eye: convert `$OUT/close.ppm` to PNG and Read it. Red edges reach every corner at full width; markers sit on top.

- [ ] **Step 4: Commit**

```bash
git add docs/_probe_matrix.py docs/_ink_suite.sh
git commit -m "viewer: the ink suite runs every hidden-line and stability check"
```

---

### Task 8: The joint probe, stroke weight and the sliver

**Files:**
- Create: `examples/mk_joint_probe.rs`, `docs/_stroke_weight.py`
- Modify: `docs/_ink_suite.sh`

**Interfaces:**
- Produces: `mk_joint_probe <out.pb>` writes one scene: a box standing on a plate (its four bottom edges red, its other edges blue), two boxes sharing a face (the first box's four shared-face edges red, its other edges blue, the second box black), a box whose top 3 mm sits inside a beam (the box's four top edges magenta), and a beam 4 mm above a plate outline drawn as a magenta polyline. `docs/_stroke_weight.py <ppm>` prints the red and blue stroke weights and exits nonzero when a red edge is under 90% of the blue median, a cross-section is under 80% of its edge's median, or any magenta pixel exists.

- [ ] **Step 1: The probe**

Create `examples/mk_joint_probe.rs`:
```rust
// Joint probe: the strokes where solids touch must weigh the same as free edges, and ink
// inside another solid must stay hidden.
//   1. A box standing on a plate: its four bottom edges lie on the plate's top (red).
//   2. Two boxes sharing a face: the first box's four shared-face edges lie on the second
//      box's face (red); the second box's edges are black.
//   3. A box whose top 3 mm is inside a beam: its four top edges are 3 mm behind the beam's
//      bottom face (magenta = must never show).
//   4. A beam 4 mm above a plate outline polyline (magenta = must never show from above).
// Every other edge of the red boxes is blue, the reference weight.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_joint_probe -- <out.pb>
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};

/// A grey slab with its zero-width edges, so it adds no ink of its own.
fn slab(size: [f64; 3], at: [f64; 3]) -> Mesh {
    let mut m = Mesh::create_box(size[0], size[1], size[2]);
    m.transform(&Xform::translation(at[0], at[1], at[2]));
    m.set_objectcolor(Color::grey());
    let n = m.edges_with_colors().len();
    m.set_linecolors(vec![Color::black(); n], vec![0.0; n]);
    m
}

/// A 400 mm box centred at `at` whose edges satisfying `special` (both ends) take `color`,
/// the rest blue; every edge at the default pen.
fn marked_box(at: [f64; 3], special: fn(&Point) -> bool, color: Color) -> Mesh {
    let mut m = Mesh::create_box(400.0, 400.0, 400.0);
    m.transform(&Xform::translation(at[0], at[1], at[2]));
    m.set_objectcolor(Color::grey());
    let edges = m.edges_with_colors();
    let mut colors = Vec::with_capacity(edges.len());
    for (a, b, _) in &edges {
        let (pa, pb) = (m.vertex_point(*a).unwrap(), m.vertex_point(*b).unwrap());
        colors.push(if special(&pa) && special(&pb) { color.clone() } else { Color::blue() });
    }
    m.set_linecolors(colors, vec![-1.0; edges.len()]);
    m
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/joint_probe.pb".into());
    let mut s = Session::new("joint_probe");

    // 1. box on a plate: plate top at z = 0, box bottom edges at z = 0.
    s.add_mesh(slab([1200.0, 1200.0, 40.0], [0.0, 0.0, -20.0]), None);
    s.add_mesh(marked_box([0.0, 0.0, 200.0], |p| p.z.abs() < 1e-9, Color::red()), None);

    // 2. two boxes sharing the face x = 2200.
    s.add_mesh(marked_box([2000.0, 0.0, 200.0], |p| (p.x - 2200.0).abs() < 1e-9, Color::red()), None);
    s.add_mesh(slab([400.0, 400.0, 400.0], [2400.0, 0.0, 200.0]), None);

    // 3. box top at z = 400 inside a beam whose bottom is at z = 397.
    s.add_mesh(marked_box([4000.0, 0.0, 200.0], |p| (p.z - 400.0).abs() < 1e-9, Color::magenta()), None);
    s.add_mesh(slab([1200.0, 200.0, 200.0], [4000.0, 0.0, 497.0]), None);

    // 4. a plate whose top outline is 4 mm under a beam.
    s.add_mesh(slab([1200.0, 300.0, 40.0], [6000.0, 0.0, -20.0]), None);
    let mut outline = Polyline::new(vec![
        Point::new(5420.0, -130.0, 0.0), Point::new(6580.0, -130.0, 0.0), Point::new(6580.0, 130.0, 0.0),
        Point::new(5420.0, 130.0, 0.0), Point::new(5420.0, -130.0, 0.0),
    ]);
    outline.linecolor = Color::magenta();
    s.add_polyline(outline, None);
    s.add_mesh(slab([1400.0, 200.0, 200.0], [6000.0, 0.0, 104.0]), None);

    s.pb_dump(&out);
    println!("wrote {out}");
}
```

- [ ] **Step 2: The weight script**

Create `docs/_stroke_weight.py`:
```python
#!/usr/bin/env python3
"""_stroke_weight.py <ppm>  ->  stroke weights of the red (joint) and blue (free) edges

A stroke is a 4-connected component of saturated red or blue pixels. Its weight is its pixel
count over its bounding-box diagonal (a straight edge's projected length). Fails when a red
component weighs under 90% of the blue median, when any interior cross-section of a component
is under 80% of that component's median, or when a magenta pixel exists."""
import os, sys
from collections import deque
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm


def classify(r, g, b):
    if r >= 180 and g <= 90 and b <= 90:
        return "red"
    if b >= 180 and r <= 90 and g <= 90:
        return "blue"
    if r >= 180 and g <= 90 and b >= 180:
        return "magenta"
    return None


def components(w, h, cells, kind):
    seen, out = set(), []
    for start in [p for p, k in cells.items() if k == kind]:
        if start in seen:
            continue
        queue, comp = deque([start]), []
        seen.add(start)
        while queue:
            x, y = queue.popleft()
            comp.append((x, y))
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if (nx, ny) in cells and cells[(nx, ny)] == kind and (nx, ny) not in seen:
                    seen.add((nx, ny))
                    queue.append((nx, ny))
        if len(comp) >= 12:
            out.append(comp)
    return out


def measure(comp):
    xs, ys = [p[0] for p in comp], [p[1] for p in comp]
    dx, dy = max(xs) - min(xs) + 1, max(ys) - min(ys) + 1
    weight = len(comp) / (dx * dx + dy * dy) ** 0.5
    axis = 0 if dx >= dy else 1
    sections = {}
    for p in comp:
        sections[p[axis]] = sections.get(p[axis], 0) + 1
    keys = sorted(sections)[2:-2]
    if not keys:
        return weight, 1.0
    counts = sorted(sections[k] for k in keys)
    median = counts[len(counts) // 2]
    return weight, min(counts) / median


def main():
    w, h, px = read_ppm(sys.argv[1])
    cells = {}
    for y in range(h):
        for x in range(w):
            k = 3 * (y * w + x)
            kind = classify(px[k], px[k + 1], px[k + 2])
            if kind:
                cells[(x, y)] = kind
    magenta = sum(1 for k in cells.values() if k == "magenta")
    red = [measure(c) for c in components(w, h, cells, "red")]
    blue = [measure(c) for c in components(w, h, cells, "blue")]
    if not red or not blue:
        print(f"FAIL: red {len(red)} blue {len(blue)} components")
        sys.exit(1)
    blue_median = sorted(v[0] for v in blue)[len(blue) // 2]
    worst_red = min(v[0] for v in red) / blue_median
    worst_section = min(v[1] for v in red + blue)
    print(f"blue: {len(blue)} edges, median weight {blue_median:.2f} px/px")
    print(f"red: {len(red)} edges, weights {[round(v[0], 2) for v in red]}, worst {100 * worst_red:.0f}% of blue")
    print(f"worst cross-section {100 * worst_section:.0f}% of its edge's median; magenta {magenta}")
    ok = worst_red >= 0.9 and worst_section >= 0.8 and magenta == 0
    print("weight OK" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
```

- [ ] **Step 3: Render and measure**

Run:
```bash
export REGEN_PROTO=0 CARGO_TARGET_DIR=$PWD/target
T=x86_64-unknown-linux-gnu
cargo build -q --release --target $T --example selftest --example mk_joint_probe
B=$CARGO_TARGET_DIR/$T/release/examples
$B/mk_joint_probe $SCRATCH/joint.pb
for cam in "iso VIEWER_NOTHING=1" "down VIEWER_ORBIT=0,209" "front VIEWER_ORBIT=0,60" "side VIEWER_ORBIT=300,120" "top VIEWER_VIEW=top" "tilt VIEWER_ORBIT=0,5 VIEWER_VIEW=top"; do
  set -- $cam; name=$1; shift
  for d in 1 4 16; do
    env VIEWER_W=1800 VIEWER_H=1400 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_DISTANCE_SCALE=$d "$@" $B/selftest $SCRATCH/joint_${name}_$d.ppm $SCRATCH/joint.pb > /dev/null 2>&1
    echo "== $name x$d"; python3 docs/_stroke_weight.py $SCRATCH/joint_${name}_$d.ppm | tail -3
  done
done
```
Expected: every case ends in `weight OK`. At distance 16 the boxes are a few pixels across and the component filter may leave too few edges; if a case prints `FAIL: red 0 blue 0 components`, that case is reported as not measurable, not as a failure, and the plan's acceptance is the 1x and 4x cases. Convert `joint_iso_1.ppm` to PNG and Read it: the red bottom edges of the box on the plate and the red edges on the shared face must look exactly as heavy as the blue ones; nothing magenta anywhere.

If a red edge is light, the likely cause is the fit reading a texel on the wrong face; print the per-fragment decision by temporarily returning a colour per branch from `ink_visible` (green for a plane match, yellow for the raw compare) in a scratch copy of the shader, never in the committed one.

- [ ] **Step 4: Add the check to the suite**

In `docs/_ink_suite.sh`, find:
```bash
check orbit_flipped python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_flipped.pb" "$OUT/orbit_flipped"
```
Add below it:
```bash
"$B/mk_joint_probe" "$OUT/joint.pb" > /dev/null
for cam in "iso VIEWER_NOTHING=1" "down VIEWER_ORBIT=0,209" "front VIEWER_ORBIT=0,60" "side VIEWER_ORBIT=300,120" "top VIEWER_VIEW=top" "tilt VIEWER_ORBIT=0,5 VIEWER_VIEW=top"; do
    set -- $cam; name=$1; shift
    for d in 1 4; do
        env VIEWER_W=1800 VIEWER_H=1400 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_DISTANCE_SCALE=$d "$@" "$B/selftest" "$OUT/joint_${name}_$d.ppm" "$OUT/joint.pb" > /dev/null 2>&1
        check "joint_${name}_$d" python3 docs/_stroke_weight.py "$OUT/joint_${name}_$d.ppm"
    done
done
```
Run `docs/_ink_suite.sh` once more; expected `ink suite OK`.

- [ ] **Step 5: Commit**

```bash
git add examples/mk_joint_probe.rs docs/_stroke_weight.py docs/_ink_suite.sh
git commit -m "viewer: the joint probe measures stroke weight where solids touch

Red edges lying on a neighbour's face must weigh what free blue edges
weigh; magenta ink 3 and 4 mm inside a beam must never show."
```

---

### Task 9: Measure, and put the map right

**Files:**
- Modify: `src/engine/gpu/device.rs`, `docs/_PERF.md`, `ARCHITECTURE.md`

**Interfaces:**
- Produces: `VIEWER_ADAPTER=<substring>` (native only) picks the adapter whose name contains the substring, case-insensitive.

- [ ] **Step 1: An adapter knob for the benchmark**

In `src/engine/gpu/device.rs`, find:
```rust
    // LowPower = the GPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames cannot be shared to the compositor and the canvas stays black.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await?;
```
Replace with:
```rust
    // LowPower = the GPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames cannot be shared to the compositor and the canvas stays black.
    let adapter = match named_adapter(&instance, backends).await {
        Some(named) => named,
        None => instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await?,
    };
```
Find:
```rust
/// A failed GPU command must never be mistaken for a valid render.
fn report_gpu_error(e: wgpu::Error) {
```
Add above it:
```rust
/// `VIEWER_ADAPTER=<substring>` names a native adapter for a benchmark (a hybrid laptop has
/// two); unset, or no match, falls through to the compositor's GPU. Never on wasm.
async fn named_adapter(instance: &wgpu::Instance, backends: wgpu::Backends) -> Option<wgpu::Adapter> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (instance, backends);
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let want = std::env::var("VIEWER_ADAPTER").ok()?.to_lowercase();
        instance.enumerate_adapters(backends).await.into_iter().find(|a| a.get_info().name.to_lowercase().contains(&want))
    }
}
```
(`enumerate_adapters` returns a future at wgpu 29: `wgpu-29.0.4/src/api/instance.rs:145`.)

Run `cargo xtest 2>&1 | tail -3` and `cargo check 2>&1 | tail -2`; both must succeed.

- [ ] **Step 2: Fetch the bench scenes and build both trees**

```bash
export REGEN_PROTO=0
DATA=https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev
mkdir -p $SCRATCH/bench/scenes $SCRATCH/bench/pb
for s in view_mixed view_meshes view_lines; do
  curl -sS -o $SCRATCH/bench/scenes/$s.yaml $DATA/scenes/$s.yaml
  for f in $(grep -o 'pb/[A-Za-z0-9_.-]*\.pb' $SCRATCH/bench/scenes/$s.yaml | sort -u); do
    [ -f $SCRATCH/bench/$f ] || curl -sS -o $SCRATCH/bench/$f $DATA/$f
  done
done
BEFORE=$(git log --format=%H --grep='drop the Tubes line style' -1)~1
git worktree add $SCRATCH/before $BEFORE
(cd $SCRATCH/before && CARGO_TARGET_DIR=$SCRATCH/before/target cargo build -q --release --target x86_64-unknown-linux-gnu --example bench_frame)
CARGO_TARGET_DIR=$PWD/target cargo build -q --release --target x86_64-unknown-linux-gnu --example bench_frame
```

- [ ] **Step 3: Measure**

```bash
T=x86_64-unknown-linux-gnu
for adapter in intel nvidia; do
  for s in view_mixed view_meshes view_lines; do
    echo "== before $adapter $s"; VIEWER_ADAPTER=$adapter VIEWER_W=1400 VIEWER_H=900 BENCH_FRAMES=60 $SCRATCH/before/target/$T/release/examples/bench_frame $SCRATCH/bench/scenes/$s.yaml 2>/dev/null
    echo "== after  $adapter $s"; VIEWER_ADAPTER=$adapter VIEWER_W=1400 VIEWER_H=900 BENCH_FRAMES=60 target/$T/release/examples/bench_frame $SCRATCH/bench/scenes/$s.yaml 2>/dev/null
  done
done
git worktree remove --force $SCRATCH/before
```
The "before" tree is the face-identity design; the "after" tree is this plan. Nothing but an idle desktop may run during the measurement. Record every printed still/moving median.

Also measure the browser heap once for `view_meshes` and `view_mixed` (`trunk serve`, open `http://localhost:8770/?scene=view_meshes&perf=1`, wait for every file, read the `heap` figure from the perf line) if a browser is available; otherwise write "not measured" in the ledger, never a carried-over number.

- [ ] **Step 4: The performance ledger**

Replace the whole content of `docs/_PERF.md` with the measured numbers in this shape (fill every cell from step 3; delete a row you could not measure rather than carrying one over):
```markdown
# Performance ledger

Measured with `examples/bench_frame.rs` (median frame, 60 frames per leg, 1400 x 900, native
Vulkan through the same tree the page runs; `VIEWER_ADAPTER` picks the GPU). "before" = the
face-identity design (commit <BEFORE sha, 8 chars>), "after" = the depth-buffer rule
(commit <HEAD sha, 8 chars>). Scenes from the R2 bucket. Date: 2026-MM-DD.

| scene | GPU | before still | before moving | after still | after moving |
|---|---|---|---|---|---|
| view_mixed | Intel RPL-S | | | | |
| view_mixed | RTX 4080 | | | | |
| view_meshes | Intel RPL-S | | | | |
| view_meshes | RTX 4080 | | | | |
| view_lines | Intel RPL-S | | | | |
| view_lines | RTX 4080 | | | | |

Browser heap (Chrome, `?perf=1`, after every file arrived): view_meshes <N> MB, view_mixed <N> MB,
or "not measured".

Ink fragment cost is now at most five depth texture reads per sample and no storage reads.
The face pass writes one colour target; there is no compute pass and no face-token attachment
(19 MiB at 1400 x 900 with 4x MSAA in the previous design).

Rules for this file: every number is measured on the day it is written, with the command that
produced it; a number that was not re-measured after a change is deleted, not carried over.
```
If "after" is slower than "before" on any scene by more than 20%, stop and report the numbers instead of editing the ledger; the spec's budget is the pre-hidden-line ledger and a regression there is a finding for the user, not something to tune away silently.

- [ ] **Step 5: ARCHITECTURE.md**

Apply these edits to `ARCHITECTURE.md`.

Find:
```
| files | `mesh.rs` `mesh_ink.rs` `mesh_topology.rs` `brep.rs` `curves.rs` `points.rs` `frames.rs` `cloud.rs` `bounds.rs` `encode.rs` | `arena.rs` `segments.rs` `glyphs.rs` `cloud.rs`+`splat.rs`+`lod.rs` `backdrop.rs` |
```
Keep it (it is already the post-plan list).

Find:
```
| ink showing through a face, or cut by one | physical face identity and supporting faces (section 6), `ink_visibility.wgsl`, `hosts.rs`, `mesh_faces.rs` |
```
Replace with:
```
| ink showing through a face, or cut by one | the depth-surface rule (section 6), `ink_visibility.wgsl` |
```

Find:
```
    walk/           producers, one file per geometry type (mod.rs dispatches on Geometry)
```
Keep. Find:
```
      arena.rs        mesh faces, sheet fills, lettering (one vertex table, three index runs)
      segments.rs     pipes (solid lane) + ribbons (flat lane) over the 48 B CylinderSegment
      glyphs.rs       spheres (solid lane) + dots (flat lane) over the 64 B GlyphPoint
```
Replace with:
```
      arena.rs        mesh faces, sheet fills, lettering (one vertex table, three index runs)
      segments.rs     pipes (solid lane) + ribbons (flat lane) over the 40 B CylinderSegment
      glyphs.rs       spheres (solid lane) + dots (flat lane) over the 48 B GlyphPoint
```
Find:
```
  shaders/          one .wgsl per lane draw: triangle, cylinder, ribbon, sphere, glyph, grid, background, splat, splat_resolve
```
Replace with:
```
  shaders/          one .wgsl per lane draw: triangle, ribbon, sphere, glyph, grid, background, splat, splat_resolve; ink_visibility is appended to every ink shader
```

Replace the whole of section 3 (from `## 3. Frame order` up to but not including `## 4. Picking`) with:
```
## 3. Frame order

`render.rs::encode_frame` writes physical depth before drawing ink:

1. Physical pass: background, grid, mesh faces, cloud resolve. Meshes write unbiased
   reverse-Z depth and their colour; nothing else.
2. Ink pass: sheet fills, mesh edges (`E`), lines (`W`), vertex markers (`E`, markers),
   lettering, point dots (`Q`). The physical depth is bound read-only and sampled by every
   ink fragment; ink never writes depth, so ink cannot occlude ink. Markers follow strokes
   so their complete footprints stay on top.

The point lane draws before these passes into its own 1x depth + colour (`Splat::prelude`),
skipped while the camera, knobs and tables are unchanged. Its resolve writes physical depth.
Multisampled colour resolves after the ink pass.

MSAA is 4x only when SOLID geometry (faces, pipes, spheres) is on the GPU and the canvas is at
most 4.2 Mpx; `?msaa=` forces. Ribbons, dots and markers antialias themselves with an exact
box filter (strokes) or a feather of `?aa=` px (dots). Under MSAA an ink fragment shader runs
per sample (`sample_index`), so its visibility is decided per sample.
```

Replace the whole of section 6 (from `## 6. Physical occlusion and full-width ink` up to but not including `## 7. Point clouds`) with:
```
## 6. Ink visibility from the depth buffer

- Faces and ink receive no world-distance push, lift or rasterizer bias, and there is no
  face identity anywhere. The only occlusion input is the physical depth attachment.
- `ink_visibility.wgsl` reads that depth as a piecewise-planar surface. A stroke fragment
  reads its own texel and the next texel AWAY from the stroke's axis (along the dominant
  component of the screen perpendicular), fits the plane through them using the axis's own
  depth slope along the stroke, and is visible when that plane passes through the axis within
  16 float ULPs plus the slope times 1/64 px (the rasterizer snaps vertices to 1/256 px).
  Otherwise it is visible only when its texel is not nearer than the axis. A marker or dot
  fits the plane along both axes away from its centre and asks the same question at the
  centre.
- Consequences: a stroke on its own face matches exactly; a stroke on a touching neighbour's
  face matches exactly, so joints weigh what free edges weigh; at a concave joint the rising
  neighbour matches because its plane contains the edge; at a silhouette the overhanging half
  lands on the background or a farther surface and stays; a line 4 mm behind a face at 20 m
  is a hundred times outside the tolerance and is hidden.
- Residuals, measured by `docs/_ink_suite.sh`: a face under about 2 px wide has no same-face
  neighbour, so its own edge falls to the raw compare and can drop out when grazing; a sliver
  within a degree of edge-on carries depth quantised by slope/256 px.
- Smooth tessellations (`FLAG_SMOOTH`: BRep and NURBS fills) ink border and crease edges
  only; there is no view-dependent silhouette term, so nothing flips as the camera turns. The
  vertex-stage facing cull (both adjacent faces away) and `FLAG_INSIDE`/`FLAG_OPEN` are as
  before.
- `CylinderSegment` is 40 B; `GlyphPoint` is 48 B; `LineUniform` is 64 B; `Instance` 96 B.
  Layout tests validate the WGSL member offsets and strides through Naga.
- Coincident ink resolves by draw order. A GPU validation error aborts the render; the
  ignored native test `invalid_gpu_shader_is_fatal` exercises that callback deliberately.
```

In section 9's table, find:
```
| view.rs | `VIEWER_LINE_STYLE=tubes` | `?style=tubes` | solid-lane style at start (`L` flips) |
```
Replace with nothing. Find:
```
Keys: `1`-`7` named views, `Space` projection, `C` reset, `F` fit, `Q` `W` `E` lanes, `L` style,
`[` `]` point size, `Esc` deselect. Mouse: right orbit, middle pan, wheel zoom, left pick.
```
Replace with:
```
Keys: `1`-`7` named views, `Space` projection, `C` reset, `F` fit, `Q` `W` `E` lanes, `D` lighting,
`B` back faces, `H` hide selection, `S` show all, `[` `]` point size, `Esc` deselect. Mouse: right
orbit, middle pan, wheel zoom, left pick.
```
Add a row to the table in section 9, after the `VIEWER_MSAA` row:
```
| device.rs | `VIEWER_ADAPTER` | - | pick the native adapter whose name contains this (benchmarks) |
```

In section 11, find:
```
- `cargo xtest`: the mirror tests and the stream parser tests.
```
Replace with:
```
- `cargo xtest`: the mirror tests and the stream parser tests. `docs/_ink_suite.sh`: every
  hidden-line, stroke-weight and orbit-stability check, one PASS/FAIL line each.
```

- [ ] **Step 6: Verify the map matches the tree, then commit**

Run:
```bash
for f in $(grep -o '`[a-z_]*\.rs`' ARCHITECTURE.md | tr -d '`' | sort -u); do find src -name "$f" | grep -q . || echo "ARCHITECTURE names a missing file: $f"; done
grep -n 'face token\|face identity\|hosts.rs\|Tubes\|tube' ARCHITECTURE.md | grep -v 'no face identity\|there is no face identity' || echo "no stale references"
cargo xtest 2>&1 | tail -3
```
Expected: no missing files, `no stale references`, tests pass.

```bash
git add src/engine/gpu/device.rs docs/_PERF.md ARCHITECTURE.md
git commit -m "viewer: measure the depth rule and put the map right

VIEWER_ADAPTER picks the benchmark GPU. The ledger records before and
after on both adapters; ARCHITECTURE sections 3 and 6 describe the
depth-surface rule, the 40 B and 48 B rows, and the keys as they are."
```

Then push and watch every workflow the push triggers to completion (`gh run list --limit 5`, `gh run watch <id>`): `viewer-check`, `viewer-pages` and `Session mini tests` must all be green before the phase is reported done.

---

## Self-review against the spec

- Section 3 (the rule): Task 2 implements 3.1 and 3.2 as one fragment-local fit (own texel plus the texel away from the axis), which is the same plane test with the same tolerance; 3.3 per-sample shading, picking at sample 0, out-of-viewport as cleared, discs along both axes, ortho and perspective alike: Task 2. 3.4 residuals: measured by the sliver case of Task 8 and the orbit check of Task 6.
- Section 4 (data model): rows and `LineUniform` in Tasks 4 and 5; every deleted file appears in Tasks 1, 3 and 4; `facing` retained.
- Section 5 (producers): mesh edges and markers in Task 3; smooth tessellations in Task 6; lines, curves, points without hosts in Task 3; BRep edges keep `sample_nurbscurve` (phase 3 changes that); raw meshes with no planes in Task 4.
- Section 7 (verification): existing checks in Task 7; joint probe, stroke weight, orbit invariance and sliver in Tasks 6 and 8; performance in Task 9. The floor census script and `census_plates` are unchanged apart from the style loop.
- Section 8 (docs): out of scope here except `ARCHITECTURE.md` and `_PERF.md`, which the tree-as-it-is rule requires now; lesson 05 is phase 4.
- Names used across tasks: `ink_visible`, `ink_disc_visible`, `InkAxis`, `InkColor` (Task 2, used by nothing in Rust); `InkScene { targets }` (Task 4, used by `mod.rs`); `draw_pipes(pass, b)` (Task 1, used by `render.rs`); `WalkCx { vert_base, cloud_px, row }` (Task 4, used by `scene.rs` and the `mesh_ink` test); `Row` without `host_faces` (Task 3); `SegRows { pipes, ribbons }` and `GlyphRows { spheres, dots }` (Task 5, used by producers and `check_determinism`).
