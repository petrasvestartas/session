# 18.6 · Matched rendering comparisons from original sources

**Goal:** Prove the original comparison and inspect differences. · **Steps:** 4 · **Source:** [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs) (`src/shaders/hzb.wgsl,src/shaders/ssao.wgsl,src/app/wood_elements.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 18.6 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/comparison.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Cargo.toml</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">4 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/hzb.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ssao.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Declare the rendering com…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. HZB Mip Pyramid &amp; SSAO</text>
  <rect x="606" y="54" width="84" height="84" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="648" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mip 0</text>
  <rect x="702" y="68" width="52" height="52" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="728" y="98" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Mip 1</text>
  <rect x="764" y="82" width="28" height="28" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Matched rendering compariso…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Timber WoodElement &amp; Joints</text>
  <polygon points="888,122 924,62 1054,62 1018,122" fill="#fed7aa" stroke="#111111" stroke-width="2"/>
  <rect x="954" y="62" width="34" height="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="971" y1="46" x2="971" y2="76" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mortise/Tenon + Dir</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove the comparison and…</text>
</svg>
</div>

Compare actual maintained and course frames under declared identical inputs and preserve differences as evidence.

## Ownership and data flow

A comparison names original source bytes, camera, viewport/DPR, samples, settings, shader/source revision and output encoding before accepting images. Native reference capture reads the maintained renderer without mutating its source. A build or buffer-layout test cannot prove pixel parity, and a previously captured frame is historical when runtime dependencies change. Inspect feature-specific witnesses for real holes, borders, isolines, point density and stroke ends; record unresolved differences rather than inventing a substitute shader or relabeling proposals as results.

## The maintained implementation

selftest.rs and selftest/oracle.rs expose maintained native render reference behavior; engine/gpu/render.rs and actual src/shaders define the frame. build.rs and engine/pipelines/mod.rs control generated shader identity and variant composition. This prose batch captured no frames; current cloud/ink/CAD parity acceptance stays pending the integrator’s matched-witness queue.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-94-declare-the-rendering-comparison-contract"></a>1. Declare the rendering comparison contract

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/comparison.rs, src/lib.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">declared camera and pipel…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">actual maintained frame</text>
</svg>
</div>

**What this code does:** Declare the rendering comparison contract.

### 1. `src/comparison.rs`

Own one original comparison specimen and explicit maintained-renderer differences.

Create the file and type:

```rust
use prost::Message;
use session_rust::{BRep, Color, Session};

pub struct Difference {
    pub subject: &'static str,
    pub teaching: &'static str,
    pub maintained: &'static str,
    pub owner: &'static str,
}

pub const DIFFERENCES: &[Difference] = &[
    Difference {
        subject: "camera",
        teaching: "forward depth, scene units",
        maintained: "reversed depth, f64 camera, mm to metres and anchor rebasing",
        owner: "src/camera.rs",
    },
    Difference {
        subject: "surfaces",
        teaching: "per-owner glass drawing, separate normal prepass",
        maintained: "packed arenas, primitive IDs, MRT and adaptive 1x/4x MSAA",
        owner: "src/engine/gpu/render.rs",
    },
    Difference {
        subject: "ink",
        teaching: "finite tile lists and centreline depth predicate",
        maintained: "plane-fit visibility, sag profiles and ribbon convolution",
        owner: "src/shaders/ink_visibility.wgsl",
    },
    Difference {
        subject: "ambient",
        teaching: "current GTAO with an explicit forward-depth adapter",
        maintained: "current reversed-depth inputs and production pass order",
        owner: "src/engine/gpu/ssao.rs",
    },
    Difference {
        subject: "defaults",
        teaching: "white background, opacity 1, no ground grid",
        maintained: "linear 0.9 background, desktop opacity 0.95, grid on",
        owner: "src/engine/gpu/view.rs",
    },
];

pub fn specimen() -> Vec<u8> {
    let mut session = Session::new("viewer comparison");
    for (mut brep, z) in [
        (BRep::create_box(1500., 1500., 100.), -500.),
        (BRep::create_box(350., 350., 700.), -100.),
        (BRep::create_cylinder(120., 450.), 250.),
        (BRep::create_cylinder(650., 120.), 700.),
    ] {
        brep.surfacecolor = Color::new(0.35, 0.65, 0.85, 1.);
        brep.transform(&session_rust::Xform::translation(0., 0., z));
        session.add_brep(brep, None);
    }
    session.to_proto().encode_to_vec()
}

pub fn original_scene(bytes: &[u8]) -> Result<crate::scene::Scene, String> {
    let mut scene = crate::scene::Scene::empty();
    let source =
        crate::document_source::prepare(bytes, &session_rust::Xform::identity(), 0.)?;
    crate::document_geometry::append(
        &mut scene,
        &source,
        &session_rust::Xform::identity(),
        0.,
    )?;
    Ok(scene)
}
```

### 2. `src/lib.rs`

Register the comparison owner.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod opacity_command_tests;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod opacity_command_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod comparison;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

## <a id="checkpoint-94a-link-the-maintained-native-rendering-reference"></a>2. Link the maintained native rendering reference

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Cargo.toml</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">declared camera and pipel…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">actual maintained frame</text>
</svg>
</div>

**What this code does:** Link the maintained native rendering reference.

### 1. `Cargo.toml`

Link the exact maintained viewer from the reconstructed checkpoint.

**Locate this existing block:**

```toml
pollster = "=0.4.0"
log = "=0.4.34"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
pollster = "=0.4.0"
log = "=0.4.34"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }
```

## <a id="checkpoint-94b-read-actual-maintained-frames-without-source-mutation"></a>3. Read actual maintained frames without source mutation

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/reference_example.rs, C…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">declared camera and pipel…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">actual maintained frame</text>
</svg>
</div>

**What this code does:** Read actual maintained frames without source mutation.

### 1. `src/reference_example.rs`

Copy the supplied maintained-frame command line example.

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
struct Logger;
#[cfg(not(target_arch = "wasm32"))]
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, message: &log::Record) {
        eprintln!("[{}] {}", message.level(), message.args());
    }
    fn flush(&self) {}
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = log::set_logger(&Logger);
    log::set_max_level(log::LevelFilter::Info);
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "current.ppm".into());
    let source = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "comparison.pb".into());
    if !std::path::Path::new(&source).exists() {
        std::fs::write(
            &source,
            viewer_journey::comparison::specimen(),
        )
        .unwrap();
    }
    print!(
        "{}",
        viewer_journey::reference_frame::render(
            std::path::Path::new(&source),
            std::path::Path::new(&output),
            [640, 480]
        )
    );
}
#[cfg(target_arch = "wasm32")]
fn main() {}
```

### 2. `Cargo.toml`

Expose the maintained reference as a native example.

**Locate this existing block:**

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }

[[example]]
name = "reference"
path = "src/reference_example.rs"
```

### 3. `src/reference_frame.rs`

Supply the optional native comparison wrapper; the browser application does not call it.

Create the file and type:

```rust
use current_viewer::selftest::{SceneFile, render_scene};
use session_rust::Xform;
use std::path::Path;

pub fn render(source: &Path, output: &Path, size: [u32; 2]) -> String {
    render_scene(
        &[SceneFile {
            path: source.to_string_lossy().into_owned(),
            place: Xform::identity(),
            point_px: 0.,
            display_only: false,
        }],
        size[0],
        size[1],
        output.to_str().expect("UTF-8 output path"),
    )
}

pub fn rgb(bytes: &[u8], size: [u32; 2]) -> Result<&[u8], &'static str> {
    let header = format!("P6\n{} {}\n255\n", size[0], size[1]);
    let body = bytes
        .strip_prefix(header.as_bytes())
        .ok_or("Unexpected PPM header")?;
    if body.len() != size[0] as usize * size[1] as usize * 3 {
        return Err("Incomplete frame");
    }
    Ok(body)
}
```

### 4. `src/lib.rs`

Keep the maintained reference native-only.

**Locate this existing block:**

```rust
mod opacity_command_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod comparison;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod opacity_command_tests;

pub mod comparison;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(not(target_arch = "wasm32"))]
pub mod reference_frame;
pub mod source_mesh_topology;
```

## <a id="checkpoint-94c-prove-the-original-comparison-and-inspect-differences"></a>4. Prove the comparison and inspect differences

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/comparison_tests.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">declared camera and pipel…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">actual maintained frame</text>
</svg>
</div>

**What this code does:** Prove the original comparison and inspect differences.

### 1. `src/comparison_tests.rs`

Copy supplied actual source and maintained-frame fixtures.

Create the file and type:

```rust
use crate::comparison;
use prost::Message;

#[test]
fn comparison_loads_the_same_four_original_solids() {
    let bytes = comparison::specimen();
    let source = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    assert_eq!(
        source.objects.as_ref().unwrap().breps.len(),
        4
    );
    let scene = comparison::original_scene(&bytes).unwrap();
    assert_eq!(scene.objects().len(), 4);
    assert!(
        scene
            .objects()
            .iter()
            .all(|row| row.cad.is_some())
    );
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    camera.fit(&scene.bounds().unwrap());
    assert!(camera.radius > 500.);
    assert!(
        camera
            .view_projection()
            .m
            .iter()
            .all(|v| v.is_finite())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn maintained_renderer_draws_the_original_curved_specimen() {
    let directory = std::env::temp_dir().join(format!(
        "viewer-course-reference-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).unwrap();
    let source = directory.join("source.pb");
    let image = directory.join("current.ppm");
    let original = comparison::specimen();
    std::fs::write(&source, &original).unwrap();
    let report = crate::reference_frame::render(&source, &image, [640, 480]);
    assert!(report.contains("640x480"));
    let pixels = std::fs::read(&image).unwrap();
    let rgb = crate::reference_frame::rgb(&pixels, [640, 480]).unwrap();
    let blue = rgb
        .chunks_exact(3)
        .filter(|p| p[2] > p[0].saturating_add(20) && p[2] > p[1].saturating_add(10))
        .count();
    assert!(
        blue > 4000,
        "maintained original blue faces: {blue}"
    );
    assert_eq!(std::fs::read(&source).unwrap(), original);
    assert!(
        crate::reference_frame::rgb(&pixels[..pixels.len() - 1], [640, 480]).is_err()
    );
    std::fs::remove_dir_all(directory).unwrap();
}
```

### 2. `src/lib.rs`

Register supplied comparison fixtures.

**Locate this existing block:**

```rust
pub mod comparison;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(not(target_arch = "wasm32"))]
pub mod reference_frame;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod comparison;

#[cfg(not(target_arch = "wasm32"))]
pub mod reference_frame;

#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod comparison_tests;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

### 3. `index.html`

Package the actual original comparison and later metadata protobuf fixtures for browser acceptance.

**Locate this existing block:**

```html
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
  <style>
```

Edit the existing block to match this code; keep unchanged lines:

```html
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
  <link data-trunk rel="copy-dir" href="assets/pb" data-target-path="assets/pb">
  <style>
```

## Build & Run

![Recorded visual output for Lesson 18.6 (94c-prove-the-original-comparison-and-inspect-differences)](../../screenshots/journey/94c-prove-the-original-comparison-and-inspect-differences.png)

```sh
npm --prefix ../session_tests run course -- dependencies 18.6
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.6
npm --prefix ../session_tests run course -- save 18.6
```

Continue with [18.7 · Measured frame costs and dependency-aware cache reports](95-record-comparable-large-scene-conditions.md).
