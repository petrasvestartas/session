# 18.7 · Measured frame costs and dependency-aware cache reports

**Goal:** Prove cache invalidation and honest performance reports. · **Steps:** 7 · **Source:** [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs) (`src/shaders/hzb.wgsl,src/shaders/ssao.wgsl,src/app/wood_elements.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 18.7 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/bench_config.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/bench_samples.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">7 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/hzb.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ssao.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Record comparable large s…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. HZB Mip Pyramid &amp; SSAO</text>
  <rect x="606" y="54" width="84" height="84" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="648" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mip 0</text>
  <rect x="702" y="68" width="52" height="52" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="728" y="98" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Mip 1</text>
  <rect x="764" y="82" width="28" height="28" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Measured frame costs and de…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Timber WoodElement &amp; Joints</text>
  <polygon points="888,122 924,62 1054,62 1018,122" fill="#fed7aa" stroke="#111111" stroke-width="2"/>
  <rect x="954" y="62" width="34" height="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="971" y1="46" x2="971" y2="76" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mortise/Tenon + Dir</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove cache invalidation…</text>
</svg>
</div>

Measure a comparable large scene and reuse immutable presentation data only under valid dependency witnesses.

## Ownership and data flow

Record actual scene counts, source identity, camera, viewport/DPR, sample count, adapter/backend and warmup before timing. Report warmed distributions and the slow tail separately from startup and GPU completion. Reject before/after runs with mismatched conditions. Tree/layer/graph caches borrow immutable source structure and invalidate after actual dependency changes, not every camera frame; source ownership must remain unchanged. Private optimization proposals and uncompiled drafts are not completed improvements. Software-GPU measurements describe that environment and cannot become hardware-FPS claims or final responsiveness acceptance.

## The maintained implementation

engine/performance.rs tracks maintained frames, draw counts, interaction windows and adaptive quality tiers; engine/gpu/timing.rs records GPU scopes. app/hierarchy.rs refreshes on row_revision and app/ui/layers.rs consumes current rows; app/inspection.rs reports actual owners/counters. Measure same final runtime inputs after integration before claiming a performance gain; this consolidation runs no benchmark.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-95-record-comparable-large-scene-conditions"></a>1. Record comparable large scene conditions

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bench_config.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Record comparable large scene conditions.

### 1. `src/bench_config.rs`

Own explicit conditions instead of unqualified performance claims.

Create the file and type:

```rust
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conditions {
    pub machine: String,
    pub browser: String,
    pub adapter: String,
    pub scene_sha256: String,
    pub viewport: [u32; 2],
    pub dpr: f64,
    pub camera: String,
    pub settings: String,
    pub measurement: String,
}

impl Conditions {
    pub fn validate(&self) -> Result<(), &'static str> {
        if [
            &self.machine,
            &self.browser,
            &self.adapter,
            &self.camera,
            &self.settings,
            &self.measurement,
        ]
        .iter()
        .any(|v| v.trim().is_empty())
        {
            return Err("Record every measurement condition");
        }
        if self.scene_sha256.len() != 64
            || !self
                .scene_sha256
                .bytes()
                .all(|v| v.is_ascii_hexdigit())
        {
            return Err("Record the original scene SHA-256");
        }
        if self.viewport.contains(&0) || !self.dpr.is_finite() || self.dpr <= 0. {
            return Err("Invalid viewport or pixel density");
        }
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Register the completed performance owner.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod comparison_tests;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod comparison_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_config;
pub mod finite_project_source;
```

## <a id="checkpoint-95a-measure-warmed-frame-times-and-the-slow-tail"></a>2. Measure warmed frame times and the slow tail

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bench_samples.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Measure warmed frame times and the slow tail.

### 1. `src/bench_samples.rs`

Summarize real samples with an explicit warmup and slow-frame tail.

Create the file and type:

```rust
#[derive(Debug, PartialEq, serde::Serialize)]
pub struct Summary {
    pub samples: usize,
    pub median_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
}

pub fn summarize(samples: &[f64], warmup: usize) -> Result<Summary, &'static str> {
    if samples
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.)
    {
        return Err("Frame durations must be positive and finite");
    }
    let measured = samples
        .get(warmup..)
        .filter(|v| !v.is_empty())
        .ok_or("No measured frames after warmup")?;
    let mut sorted = measured.to_vec();
    sorted.sort_by(f64::total_cmp);
    let count = sorted.len();
    let middle = count / 2;
    let median = if count % 2 == 0 {
        sorted[middle - 1] * 0.5 + sorted[middle] * 0.5
    } else {
        sorted[middle]
    };
    let rank = (count * 95).div_ceil(100) - 1;
    Ok(Summary {
        samples: count,
        median_ms: median,
        p95_ms: sorted[rank],
        max_ms: sorted[count - 1],
    })
}
```

### 2. `src/lib.rs`

Register the completed performance owner.

**Locate this existing block:**

```rust
mod comparison_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_config;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod comparison_tests;

pub mod bench_config;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_samples;
pub mod finite_project_source;
```

## <a id="checkpoint-95b-reject-incomparable-before-and-after-runs"></a>3. Reject incomparable before and after runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bench_compare.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Reject incomparable before and after runs.

### 1. `src/bench_compare.rs`

Compare matched measurements and retain original-source memory accounting.

Create the file and type:

```rust
use crate::{
    bench_config::Conditions,
    bench_samples::{Summary, summarize},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Run {
    pub conditions: Conditions,
    pub revision: String,
    pub warmup: usize,
    pub frame_ms: Vec<f64>,
    pub gpu_buffers: u64,
    pub gpu_textures: u64,
    pub original_bytes: u64,
}

pub struct Comparison {
    pub before: Summary,
    pub after: Summary,
    pub median_change_ms: f64,
    pub p95_change_ms: f64,
}

pub fn compare(before: &Run, after: &Run) -> Result<Comparison, &'static str> {
    before.conditions.validate()?;
    after.conditions.validate()?;
    if before.conditions != after.conditions {
        return Err("Runs have different hardware, scene or rendering conditions");
    }
    if before.revision.trim().is_empty() || after.revision.trim().is_empty() {
        return Err("Record both renderer revisions");
    }
    if before.original_bytes != after.original_bytes {
        return Err(
            "Original source memory changed; inspect retention before comparing",
        );
    }
    let a = summarize(&before.frame_ms, before.warmup)?;
    let b = summarize(&after.frame_ms, after.warmup)?;
    Ok(Comparison {
        median_change_ms: b.median_ms - a.median_ms,
        p95_change_ms: b.p95_ms - a.p95_ms,
        before: a,
        after: b,
    })
}
```

### 2. `src/lib.rs`

Register the completed performance owner.

**Locate this existing block:**

```rust
pub mod bench_config;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_samples;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod bench_config;

pub mod bench_samples;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_compare;
pub mod finite_project_source;
```

## <a id="checkpoint-95c-witness-immutable-tree-inputs-across-camera-frames"></a>4. Witness immutable tree inputs across camera frames

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tree_witness.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Witness immutable tree inputs across camera frames.

### 1. `src/tree_witness.rs`

Witness shared layer inputs without retaining geometry payloads.

Create the file and type:

```rust
use crate::{
    hierarchy::Document,
    scene::{ObjectId, Scene},
    view_flags::Flags,
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct Witness {
    documents: Vec<Rc<Document>>,
    groups: Rc<HashSet<(u32, String)>>,
    graphs: Rc<HashMap<u32, Rc<session_rust::Graph>>>,
    view: Rc<Flags>,
    selection: Vec<ObjectId>,
    epoch: u64,
}

impl Witness {
    pub fn new(scene: &Scene, selection: &[ObjectId], epoch: u64) -> Self {
        Self {
            documents: scene.forest.documents.clone(),
            groups: scene.groups.clone(),
            graphs: scene.graphs.clone(),
            view: scene.view.clone(),
            selection: selection.to_vec(),
            epoch,
        }
    }
    pub fn matches(&self, scene: &Scene, selection: &[ObjectId], epoch: u64) -> bool {
        self.epoch == epoch
            && self.selection == selection
            && Rc::ptr_eq(&self.groups, &scene.groups)
            && Rc::ptr_eq(&self.graphs, &scene.graphs)
            && Rc::ptr_eq(&self.view, &scene.view)
            && self.documents.len() == scene.forest.documents.len()
            && self
                .documents
                .iter()
                .zip(&scene.forest.documents)
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
```

### 2. `src/lib.rs`

Register the completed performance owner.

**Locate this existing block:**

```rust
pub mod bench_samples;

#[cfg(test)]
mod ambient_shader_contract;
pub mod bench_compare;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod bench_samples;

pub mod bench_compare;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
pub mod tree_witness;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-95d-reuse-layer-rows-while-rotating-and-zooming"></a>5. Reuse layer rows while rotating and zooming

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tree_panel.rs, src/brow…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Reuse layer rows while rotating and zooming.

### 1. `src/tree_panel.rs`

Cache unchanged scene/selection/epoch inputs before rebuilding layer state.

**Locate this existing block:**

```rust
pub struct TreePanel {
    pub graph_open: bool,
    pub graph: Vec<(String, String, String, String)>,
    pub edit: Option<(String, crate::layer_edit::Edit)>,
    pub rename: Option<(String, String)>,
    pub rename_error: String,
    rename_focus: bool,
    pub confirm: Option<(String, usize)>,
    pub property: Option<(String, crate::layer_properties::Property)>,
    pub menu_rects: Vec<egui::Rect>,
    states: std::collections::HashMap<String, RowState>,
    pub open: bool,
    pub rows: Vec<Row>,
    pub selected: Vec<ObjectId>,
    expanded: HashSet<String>,
    pub rect: Option<egui::Rect>,
    pub action: Option<(Vec<ObjectId>, bool)>,
    page: usize,
}
impl TreePanel {
    pub fn refresh(&mut self, scene: &crate::scene::Scene, selected: Vec<ObjectId>) {
        self.graph = crate::graph_edit::rows(scene);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct TreePanel {
    witness: Option<crate::tree_witness::Witness>,
    pub refreshes: u64,
    pub graph_open: bool,
    pub graph: Vec<(String, String, String, String)>,
    pub edit: Option<(String, crate::layer_edit::Edit)>,
    pub rename: Option<(String, String)>,
    pub rename_error: String,
    rename_focus: bool,
    pub confirm: Option<(String, usize)>,
    pub property: Option<(String, crate::layer_properties::Property)>,
    pub menu_rects: Vec<egui::Rect>,
    states: std::collections::HashMap<String, RowState>,
    pub open: bool,
    pub rows: Vec<Row>,
    pub selected: Vec<ObjectId>,
    expanded: HashSet<String>,
    pub rect: Option<egui::Rect>,
    pub action: Option<(Vec<ObjectId>, bool)>,
    page: usize,
}
impl TreePanel {
    pub fn refresh_cached(
        &mut self,
        scene: &crate::scene::Scene,
        selected: Vec<ObjectId>,
        epoch: u64,
    ) {
        if self
            .witness
            .as_ref()
            .is_some_and(|w| w.matches(scene, &selected, epoch))
        {
            return;
        }
        self.refresh(scene, selected.clone());
        self.witness = Some(crate::tree_witness::Witness::new(
            scene, &selected, epoch,
        ));
        self.refreshes += 1;
    }

    pub fn refresh(&mut self, scene: &crate::scene::Scene, selected: Vec<ObjectId>) {
        self.witness = None;
        self.graph = crate::graph_edit::rows(scene);
```

### 2. `src/browser.rs`

Use the cache from the actual teaching frame loop.

**Locate this existing block:**

```rust
                });
            panel
                .tree
                .refresh(&editor.scene, editor.selected_objects());
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                });
            panel.tree.refresh_cached(
                &editor.scene,
                editor.selected_objects(),
                editor.interaction_epoch,
            );
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
```

## <a id="checkpoint-95e-borrow-graph-edges-and-index-their-labels"></a>6. Borrow graph edges and index their labels

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/graph_edit.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Borrow graph edges and index their labels.

### 1. `src/graph_edit.rs`

Avoid full graph protobuf conversion and repeated node scans for display rows.

**Locate this existing block:**

```rust
pub fn rows(scene: &Scene) -> Vec<(String, String, String, String)> {
    scene
        .forest
        .documents
        .iter()
        .flat_map(|doc| {
            let graph = scene
                .graphs
                .get(&doc.key)
                .map(|g| g.to_proto())
                .or_else(|| {
                    doc.header
                        .as_ref()
                        .and_then(|h| h.header.graph.clone())
                });
            let name = |guid: &str| {
                doc.nodes
                    .iter()
                    .find(|n| {
                        n.source.name == guid
                            || n.object
                                .is_some_and(|id| scene.row_guid(id) == Some(guid))
                    })
                    .map_or_else(|| guid.to_owned(), |n| n.label.clone())
            };
            graph
                .into_iter()
                .flat_map(|g| {
                    g.edges.into_iter().map(|edge| {
                        (
                            doc.label.clone(),
                            name(&edge.v0),
                            name(&edge.v1),
                            edge.attribute,
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn rows(scene: &Scene) -> Vec<(String, String, String, String)> {
    let mut rows = Vec::new();
    for doc in &scene.forest.documents {
        let mut labels = std::collections::HashMap::new();
        for node in &doc.nodes {
            labels
                .entry(node.source.name.as_str())
                .or_insert(node.label.as_str());
            if let Some(guid) = node.object.and_then(|id| scene.row_guid(id)) {
                labels
                    .entry(guid)
                    .or_insert(node.label.as_str());
            }
        }
        let mut push = |from: &str, to: &str, attribute: &str| {
            rows.push((
                doc.label.clone(),
                labels
                    .get(from)
                    .copied()
                    .unwrap_or(from)
                    .to_owned(),
                labels
                    .get(to)
                    .copied()
                    .unwrap_or(to)
                    .to_owned(),
                attribute.to_owned(),
            ))
        };
        if let Some(graph) = scene.graphs.get(&doc.key) {
            for (u, neighbors) in &graph.edges {
                for (v, edge) in neighbors {
                    if u <= v {
                        push(&edge.v0, &edge.v1, &edge.attribute);
                    }
                }
            }
        } else if let Some(graph) = doc
            .header
            .as_ref()
            .and_then(|h| h.header.graph.as_ref())
        {
            for edge in &graph.edges {
                push(&edge.v0, &edge.v1, &edge.attribute);
            }
        }
    }
    rows
}
```

## <a id="checkpoint-95f-prove-cache-invalidation-and-honest-performance-reports"></a>7. Prove cache invalidation and honest performance reports

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bench_tests.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">record comparable hardwar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reuse unchanged scene work</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure rotation and zoom</text>
</svg>
</div>

**What this code does:** Prove cache invalidation and honest performance reports.

### 1. `src/bench_tests.rs`

Copy supplied invalid-data, comparability, original-source, cache and history fixtures.

Create the file and type:

```rust
use crate::{
    bench_compare::{Run, compare},
    bench_config::Conditions,
    bench_samples::summarize,
};
fn run() -> Run {
    Run {
        conditions: Conditions {
            machine: "Lenovo fixture".into(),
            browser: "Chrome fixture".into(),
            adapter: "recorded adapter".into(),
            scene_sha256: "a".repeat(64),
            viewport: [1280, 720],
            dpr: 1.,
            camera: "saved pose and orbit path".into(),
            settings: "same opacity, AO, outlines and MSAA".into(),
            measurement: "presented browser frames".into(),
        },
        revision: "before".into(),
        warmup: 2,
        frame_ms: vec![500., 200., 16., 17., 18., 20.],
        gpu_buffers: 100,
        gpu_textures: 200,
        original_bytes: 300,
    }
}
#[test]
fn warmup_and_slow_tail_are_reported_separately() {
    let values: Vec<f64> = std::iter::once(5000.)
        .chain((1..=100).map(|v| v as f64))
        .collect();
    let s = summarize(&values, 1).unwrap();
    assert_eq!(s.samples, 100);
    assert_eq!(s.median_ms, 50.5);
    assert_eq!(s.p95_ms, 95.);
    assert_eq!(s.max_ms, 100.);
}
#[test]
fn missing_or_invalid_timings_are_never_performance_evidence() {
    for (values, warmup) in [
        (vec![], 0),
        (vec![16.], 1),
        (vec![f64::NAN], 0),
        (vec![0.], 0),
        (vec![f64::INFINITY], 0),
    ] {
        assert!(summarize(&values, warmup).is_err());
    }
}
#[test]
fn comparisons_reject_changed_scene_hardware_and_quality() {
    let a = run();
    let mut b = run();
    b.revision = "after".into();
    b.frame_ms = vec![400., 100., 10., 11., 12., 13.];
    let result = compare(&a, &b).unwrap();
    assert_eq!(result.median_change_ms, -6.);
    assert_eq!(result.p95_change_ms, -7.);
    b.conditions.adapter = "different GPU".into();
    assert!(compare(&a, &b).is_err());
    b.conditions = a.conditions.clone();
    b.conditions.scene_sha256 = "b".repeat(64);
    assert!(compare(&a, &b).is_err());
    b.conditions = a.conditions.clone();
    b.conditions.settings = "AO disabled".into();
    assert!(compare(&a, &b).is_err());
    b.conditions = a.conditions.clone();
    b.original_bytes = 0;
    assert!(compare(&a, &b).is_err());
}
#[test]
fn records_round_trip_conditions_without_invented_measurements() {
    let a = run();
    let bytes = serde_json::to_vec(&a).unwrap();
    let b: Run = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(b.conditions, a.conditions);
    assert_eq!(b.frame_ms, a.frame_ms);
    assert!(compare(&a, &b).is_ok());
}
#[test]
fn tree_cache_reuses_camera_frames_but_refreshes_edits_and_history() {
    use crate::editor::{Action, Editor};
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    let id = e.scene.objects()[0].id;
    let original = crate::source_edit::original(&e.scene, id).unwrap();
    let mut panel = crate::tree_panel::TreePanel::default();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert_eq!(panel.refreshes, 1);
    let before = panel
        .rows
        .iter()
        .map(|r| r.label.clone())
        .collect::<Vec<_>>();
    for _ in 0..20 {
        e.apply(Action::Orbit(0.01, 0.)).unwrap();
        panel.refresh_cached(
            &e.scene,
            e.selected_objects(),
            e.interaction_epoch,
        );
    }
    assert_eq!(panel.refreshes, 1);
    assert!(crate::source_edit::same(
        &original,
        &crate::source_edit::original(&e.scene, id).unwrap()
    ));
    e.apply(Action::SelectAll).unwrap();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert_eq!(panel.refreshes, 2);
    e.apply(Action::Hide).unwrap();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert_eq!(panel.refreshes, 3);
    e.apply(Action::Undo).unwrap();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert_eq!(panel.refreshes, 4);
    assert!(panel.rows.is_empty());
    e.apply(Action::Redo).unwrap();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert_eq!(panel.refreshes, 5);
    assert_eq!(
        panel
            .rows
            .iter()
            .map(|r| r.label.clone())
            .collect::<Vec<_>>(),
        before
    );
    e.apply(Action::Close).unwrap();
    panel.refresh_cached(
        &e.scene,
        e.selected_objects(),
        e.interaction_epoch,
    );
    assert!(panel.rows.is_empty());
    assert!(panel.graph.is_empty());
}
#[test]
fn graph_rows_keep_one_original_edge_and_document_labels() {
    use crate::editor::{Action, Editor};
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::AddBox).unwrap();
    e.apply(Action::SelectAll).unwrap();
    e.apply(Action::AddEdge).unwrap();
    let before = e
        .scene
        .graphs
        .iter()
        .map(|(key, graph)| (*key, graph.to_proto()))
        .collect::<Vec<_>>();
    let rows = crate::graph_edit::rows(&e.scene);
    assert_eq!(rows.len(), 1);
    assert!(!rows[0].0.is_empty());
    assert!(!rows[0].1.is_empty());
    assert!(!rows[0].2.is_empty());
    assert_eq!(rows[0].3, "");
    assert_eq!(
        e.scene
            .graphs
            .iter()
            .map(|(key, graph)| (*key, graph.to_proto()))
            .collect::<Vec<_>>(),
        before
    );
    e.apply(Action::Undo).unwrap();
    assert!(crate::graph_edit::rows(&e.scene).is_empty());
    e.apply(Action::Redo).unwrap();
    assert_eq!(crate::graph_edit::rows(&e.scene), rows);
}
```

### 2. `src/lib.rs`

Register supplied performance correctness fixtures.

**Locate this existing block:**

```rust
pub mod bench_compare;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
pub mod tree_witness;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod bench_compare;

pub mod tree_witness;

#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod bench_tests;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 18.7 (95f-prove-cache-invalidation-and-honest-performance-reports)](../../screenshots/journey/95f-prove-cache-invalidation-and-honest-performance-reports.png)

```sh
npm --prefix ../session_tests run course -- dependencies 18.7
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.7
npm --prefix ../session_tests run course -- save 18.7
```

Continue with [19.1 · Complete source metadata and the actual command contract](96-retain-non-drawing-originals-and-validate-raw-planes.md).
