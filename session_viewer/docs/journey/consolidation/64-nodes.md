# 11.2 · Validated original cloud columns and placed owners

**Goal:** Include clouds in whole selection history and rectangle checks. · **Steps:** 6 · **Source:** [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/cloud.rs](../../../src/engine/gpu/cloud.rs) · [engine/gpu/lod.rs](../../../src/engine/gpu/lod.rs) · [engine/gpu/splat.rs](../../../src/engine/gpu/splat.rs) · [shaders/splat.wgsl](../../../src/shaders/splat.wgsl) · [shaders/splat_resolve.wgsl](../../../src/shaders/splat_resolve.wgsl) · [app/cloud_query.rs](../../../src/app/cloud_query.rs) (`src/engine/gpu/points.rs,src/shaders/point.wgsl,src/shaders/outline.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 11.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cloud.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">6 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/points.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/point.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Check cloud cubes against…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Point Cloud SDF Splats</text>
  <rect x="606" y="52" width="188" height="88" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="646" cy="84" r="14" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="646" cy="84" r="3" fill="#111111"/>
  <circle cx="702" cy="76" r="18" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <circle cx="702" cy="76" r="3" fill="#111111"/>
  <circle cx="752" cy="96" r="11" fill="#fee2e2" stroke="#dc2626" stroke-width="2"/>
  <circle cx="752" cy="96" r="3" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">length(uv) &lt;= radius_px</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Validated original cloud co…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Silhouette &amp; Crease Outlines</text>
  <polygon points="902,130 946,56 1046,56 1002,130" fill="#f8fafc" stroke="#111111" stroke-width="3"/>
  <line x1="946" y1="56" x2="974" y2="130" stroke="#2563eb" stroke-width="2" stroke-dasharray="4 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="146" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">ID Edge + Depth Jump</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Include clouds in whole s…</text>
</svg>
</div>

Retain original point rows and a valid octree while giving each placed cloud its own scene identity.

## Ownership and data flow

Validate coordinate/colour/normal/ID column lengths before allocation. Node point ranges and contained cubes must form one rooted tree with unique child ownership and disjoint source ranges. Prepared source owns original columns, bounds and spacing; independent f64 placement supplies world bounds without rewriting those columns. Scene insertion, current layer, whole selection and history preserve shared source ownership.

## The maintained implementation

app/walk/cloud.rs prepares maintained CloudRows, while engine/gpu/cloud.rs CloudLane owns growing position/colour/normal GPU buffers, Cloud descriptors, LodNode tables and resident Chunk mappings. An object row identifies placement; original point IDs remain a separate source address domain.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/cloud.rs](../../../src/engine/gpu/cloud.rs), [engine/gpu/lod.rs](../../../src/engine/gpu/lod.rs), [engine/gpu/splat.rs](../../../src/engine/gpu/splat.rs), [shaders/splat.wgsl](../../../src/shaders/splat.wgsl), [shaders/splat_resolve.wgsl](../../../src/shaders/splat_resolve.wgsl), [app/cloud_query.rs](../../../src/app/cloud_query.rs).

## <a id="checkpoint-64-nodes"></a>1. Check cloud cubes against their point rows

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original point columns</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">contained node cubes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived descriptors</text>
</svg>
</div>

**What this code does:** Check cloud cubes against their original point rows.

### 1. `src/cloud.rs`

Own original cloud sources and validate each derived node against its source ranges and parent cube.

Create the file and type:

```rust
use session_rust::{PointCloud, Xform};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3],
    pub size: f32,
    pub spacing: f32,
    pub first: u32,
    pub count: u32,
    pub children: [i32; 8],
}
pub struct Cloud {
    pub spacing: f32,
    pub node_first: u32,
    pub node_count: u32,
    pub resident: u32,
}
#[derive(Clone)]
pub struct Prepared {
    pub source: Rc<PointCloud>,
    pub nodes: Rc<Vec<LodNode>>,
    pub bounds: Rc<crate::bounds::Bounds>,
    pub spacing: f32,
}

fn checked_node(
    source: &PointCloud,
    p: &session_rust::proto::PointCloud,
    i: usize,
) -> Result<LodNode, &'static str> {
    let n = p.lod_size.len();
    let first = p.lod_first[i];
    let count = p.lod_count[i];
    let size = p.lod_size[i];
    let spacing = p.lod_spacing[i];
    if first < 0
        || count < 0
        || first as usize + count as usize > source.len()
        || !size.is_finite()
        || size <= 0.
        || !(size as f32).is_finite()
        || !spacing.is_finite()
        || spacing <= 0.
        || !(spacing as f32).is_finite()
        || p.lod_level[i] < 0
    {
        return Err("Invalid cloud octree range or size");
    }
    let corner = &p.lod_min[i * 3..i * 3 + 3];
    let tolerance = size * 1e-9 + 1e-10;
    for point in source.coords()
        [first as usize * 3..(first as usize + count as usize) * 3]
        .chunks_exact(3)
    {
        if (0..3).any(|axis| {
            point[axis] < corner[axis] - tolerance
                || point[axis] > corner[axis] + size + tolerance
        }) {
            return Err("Cloud octree cube omits its source points");
        }
    }
    let mut children = [-1; 8];
    children.copy_from_slice(&p.lod_children[i * 8..i * 8 + 8]);
    for child in children.iter().copied().filter(|c| *c >= 0) {
        let child = child as usize;
        if child >= n || child == 0 || p.lod_level[child] != p.lod_level[i] + 1 {
            return Err("Invalid cloud octree child");
        }
        let child_size = p.lod_size[child];
        if !child_size.is_finite()
            || child_size <= 0.
            || child_size > size + tolerance
            || (0..3).any(|axis| {
                p.lod_min[child * 3 + axis] < corner[axis] - tolerance
                    || p.lod_min[child * 3 + axis] + child_size
                        > corner[axis] + size + tolerance
            })
        {
            return Err("Cloud octree child lies outside its parent");
        }
    }
    if children.iter().any(|c| *c < -1) {
        return Err("Invalid cloud octree child sentinel");
    }
    let center = std::array::from_fn(|axis| p.lod_min[i * 3 + axis] + size * 0.5);
    if center
        .iter()
        .any(|v| !v.is_finite() || !(*v as f32).is_finite())
    {
        return Err("Invalid cloud octree center");
    }
    Ok(LodNode {
        center: center.map(|v| v as f32),
        size: size as f32,
        spacing: spacing as f32,
        first: first as u32,
        count: count as u32,
        children,
    })
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod outline_math_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod outline_math_tests;

pub mod cloud;
```

## <a id="checkpoint-64a-tree"></a>2. Validate one rooted tree and disjoint ranges

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">column lengths</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">rooted levels</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete disjoint ranges</text>
</svg>
</div>

**What this code does:** Validate one rooted tree and disjoint original ranges.

### 1. `src/cloud.rs`

Validate complete octree columns, rooted ownership and source-row coverage before traversal.

**Locate this existing block:**

```rust
}

fn checked_node(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn validate_nodes(source: &PointCloud) -> Result<Vec<LodNode>, &'static str> {
    let p = source.to_proto();
    let n = p.lod_size.len();
    if p.lod_min.len() != n * 3
        || p.lod_spacing.len() != n
        || p.lod_level.len() != n
        || p.lod_first.len() != n
        || p.lod_count.len() != n
        || p.lod_children.len() != n * 8
    {
        return Err("Invalid cloud octree columns");
    }
    let mut nodes = Vec::with_capacity(n);
    let mut owners = vec![0u8; n];
    let mut ranges = Vec::new();
    for i in 0..n {
        let node = checked_node(source, &p, i)?;
        if node.count > 0 {
            ranges.push((
                node.first as usize,
                (node.first + node.count) as usize,
            ));
        }
        for child in node
            .children
            .iter()
            .copied()
            .filter(|c| *c >= 0)
        {
            let child = child as usize;
            owners[child] = owners[child]
                .checked_add(1)
                .ok_or("Cloud octree child repeated")?;
            if owners[child] != 1 {
                return Err("Cloud octree child repeated");
            }
        }
        nodes.push(node);
    }
    if n > 0 && (p.lod_level[0] != 0 || owners[1..].iter().any(|v| *v != 1)) {
        return Err("Cloud octree is not one rooted tree");
    }
    ranges.sort_unstable();
    let mut end = 0;
    for (first, to) in ranges {
        if first != end {
            return Err("Cloud octree point ranges overlap or omit source rows");
        }
        end = to;
    }
    if n > 0 && end != source.len() {
        return Err("Cloud octree point ranges omit source rows");
    }
    Ok(nodes)
}

fn checked_node(
```

## <a id="checkpoint-64b-source"></a>3. Prepare bounds and spacing without replacing source words

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked original columns</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared prepared owner</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">resident descriptor</text>
</svg>
</div>

**What this code does:** Prepare bounds and spacing without replacing source words.

### 1. `src/cloud.rs`

Keep original source columns and measured bounds separate from derived node descriptors.

**Locate this existing block:**

```rust
}

fn validate_nodes(source: &PointCloud) -> Result<Vec<LodNode>, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Prepared {
    pub fn new(source: Rc<PointCloud>) -> Result<Self, &'static str> {
        let n = source.len();
        if n == 0
            || n > u32::MAX as usize
            || source.coords().len() != n * 3
            || source
                .coords()
                .iter()
                .any(|v| !v.is_finite() || !(*v as f32).is_finite())
        {
            return Err("Invalid cloud coordinates");
        }
        if !source.colors().is_empty()
            && (source.colors().len() != n * 4
                || source
                    .colors()
                    .iter()
                    .any(|c| !(0..=255).contains(c)))
        {
            return Err("Invalid cloud colors");
        }
        if !source.normals().is_empty()
            && (source.normals().len() != n * 3
                || source
                    .normals()
                    .iter()
                    .any(|v| !v.is_finite()))
        {
            return Err("Invalid cloud normals");
        }
        if !source.point_size.is_finite()
            || source.point_size <= 0.
            || !(source.point_size as f32).is_finite()
        {
            return Err("Invalid cloud point size");
        }
        if !source.point_ids().is_empty()
            && (source.point_ids().len() != n
                || source
                    .point_ids()
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    != n)
        {
            return Err("Invalid original cloud IDs");
        }
        let nodes = validate_nodes(&source)?;
        let mut bounds = crate::bounds::Bounds {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        };
        for p in source.coords().chunks_exact(3) {
            bounds.include([p[0], p[1], p[2]]);
        }
        let mut extents =
            std::array::from_fn::<_, 3, _>(|i| bounds.max[i] - bounds.min[i]);
        extents.sort_by(|a, b| b.total_cmp(a));
        let area = extents[0] * extents[1];
        let spacing = if n > 1 && area > 0. && area.is_finite() {
            (area / n as f64).sqrt() as f32
        } else {
            20.
        };
        let _ = source.guid();
        Ok(Self {
            source,
            nodes: Rc::new(nodes),
            bounds: Rc::new(bounds),
            spacing,
        })
    }
    pub fn descriptor(&self) -> Cloud {
        Cloud {
            spacing: self.spacing,
            node_first: 0,
            node_count: self.nodes.len() as u32,
            resident: self.source.len() as u32,
        }
    }
}

fn validate_nodes(source: &PointCloud) -> Result<Vec<LodNode>, &'static str> {
```

## <a id="checkpoint-64c-placements"></a>4. Place the cloud through an independent f64 instance model

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared original cloud</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed corners</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">world bounds</text>
</svg>
</div>

**What this code does:** Place the cloud through an independent f64 instance model.

### 1. `src/cloud.rs`

Preserve original cloud ownership and derive world bounds from a separate f64 model.

**Locate this existing block:**

```rust
}

fn checked_node(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub prepared: Prepared,
    pub model: Xform,
}
impl Row {
    pub fn bounds(&self) -> crate::bounds::Bounds {
        let b = &self.prepared.bounds;
        let corners = (0..8).map(|i| {
            self.model
                .transform_point(&session_rust::Point::new(
                    if i & 1 == 0 {
                        b.min[0]
                    } else {
                        b.max[0]
                    },
                    if i & 2 == 0 {
                        b.min[1]
                    } else {
                        b.max[1]
                    },
                    if i & 4 == 0 {
                        b.min[2]
                    } else {
                        b.max[2]
                    },
                ))
        });
        let mut bounds = crate::bounds::Bounds {
            min: [f64::INFINITY; 3],
            max: [f64::NEG_INFINITY; 3],
        };
        for p in corners {
            bounds.include([p[0], p[1], p[2]]);
        }
        bounds
    }
}

fn checked_node(
```

## <a id="checkpoint-64d-scene"></a>5. Insert a checked cloud on the current layer

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current layer</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked placement</text>
</svg>
</div>

**What this code does:** Insert a checked cloud on the current layer.

### 1. `src/scene.rs`

Insert validated cloud rows, route current-layer ownership and release them with the scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.texts = Vec::new();
```

Replace it with:

```rust
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.clouds = Vec::new();
        self.texts = Vec::new();
```

**Locate this block:**

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn texts(&self) -> &[crate::scene_label::Row] {
        &self.texts
    }
```

Replace it with:

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn clouds(&self) -> &[crate::cloud::Row] {
        &self.clouds
    }
    pub fn insert_cloud(
        &mut self,
        prepared: crate::cloud::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid().to_string();
        while self.clouds.iter().any(|r| r.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let name = prepared.source.name.clone();
        let visible = prepared.source.is_visible;
        let locked = prepared.source.is_locked;
        let row = crate::cloud::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id, visible, locked);
        self.clouds.push(row);
        crate::layer_edit::attach_generated(self, id, guid, name)?;
        Ok(id)
    }
    pub fn place_cloud(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid cloud placement");
        }
        let row = self
            .clouds
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Cloud object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        if candidate
            .bounds()
            .min
            .into_iter()
            .chain(candidate.bounds().max)
            .any(|v| !(v as f32).is_finite())
        {
            return Err("Cloud placement exceeds display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn texts(&self) -> &[crate::scene_label::Row] {
        &self.texts
    }
```

**Locate this block:**

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
```

Replace it with:

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
```

**Locate this block:**

```rust
    pub forest: crate::hierarchy::Forest,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
```

Replace it with:

```rust
    pub forest: crate::hierarchy::Forest,
    clouds: Vec<crate::cloud::Row>,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
```

## <a id="checkpoint-64e-identity"></a>6. Include clouds in whole selection history and rectangle checks

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/hierarchy…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/poin…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared source restoration</text>
</svg>
</div>

**What this code does:** Include clouds in whole selection history and rectangle checks.

### 1. `src/scene.rs`

Connect cloud owners to existing placement, bounds, copy, next selection and deletion paths.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .chain(self.texts.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

Replace it with:

```rust
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .chain(self.texts.iter().map(|r| r.id))
            .chain(self.clouds.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

**Locate this block:**

```rust
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
```

Replace it with:

```rust
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        self.clouds.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
```

**Locate this block:**

```rust
                    .map(|row| row.prepared.source.guid())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
```

Replace it with:

```rust
                    .map(|row| row.prepared.source.guid())
            })
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
```

**Locate this block:**

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
```

Replace it with:

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.clouds.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
```

**Locate this block:**

```rust
        if !self.visible(id) {
            return None;
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
```

Replace it with:

```rust
        if !self.visible(id) {
            return None;
        }
        if let Some(row) = self.clouds.iter().find(|row| row.id == id) {
            return Some(row.bounds());
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
```

**Locate this block:**

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        if let Some(row) = self.texts.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_text(id, model);
```

Replace it with:

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        if let Some(row) = self.clouds.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_cloud(id, model);
        }
        if let Some(row) = self.texts.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_text(id, model);
```

**Locate this block:**

```rust
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
```

Replace it with:

```rust
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else if self.clouds.iter().any(|r| r.id == id) {
            self.place_cloud(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
```

**Locate this block:**

```rust
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
```

Replace it with:

```rust
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.clouds
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
```

**Locate this block:**

```rust
        }
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
```

Replace it with:

```rust
        }
        for row in self
            .clouds
            .iter()
            .filter(|r| self.visible(r.id))
        {
            let next = row.bounds();
            if let Some(bounds) = &mut bounds {
                bounds.include(next.min);
                bounds.include(next.max);
            } else {
                bounds = Some(next);
            }
        }
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
```

### 2. `src/hierarchy.rs`

Include original cloud rows in the existing hierarchy/rectangle routes and reject incomplete mixed saves.

**Locate this existing block:**

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        )
        .chain(
            scene
                .clouds()
                .iter()
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

### 3. `src/rect_select.rs`

Include original cloud rows in the existing hierarchy/rectangle routes and reject incomplete mixed saves.

**Locate this existing block:**

```rust
    }
    found.sort_unstable_by_key(|id| id.value());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| eligible(row.id))
    {
        let mut points = row
            .prepared
            .source
            .coords()
            .chunks_exact(3)
            .map(|p| {
                screen(
                    matrix,
                    world(&row.model, [p[0], p[1], p[2]]),
                    size,
                )
            });
        let hit = if rectangle.crossing {
            points.any(|p| p.is_some_and(|p| rectangle.contains(p)))
        } else {
            points.all(|p| p.is_some_and(|p| rectangle.contains(p)))
        };
        if hit {
            found.push(row.id);
        }
    }
    found.sort_unstable_by_key(|id| id.value());
```

### 4. `src/document.rs`

Include original cloud rows in the existing hierarchy/rectangle routes and reject incomplete mixed saves.

**Locate this existing block:**

```rust
    }
    if !scene.texts().is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    if !scene.clouds().is_empty() {
        return Err("This checkpoint cannot save cloud objects yet");
    }
    if !scene.texts().is_empty() {
```

### 5. `src/editor.rs`

Make whole Select All include eligible original cloud owners.

**Locate this existing block:**

```rust
                    .chain(self.scene.texts().iter().map(|r| r.id))
                    .collect();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    .chain(self.scene.texts().iter().map(|r| r.id))
                    .chain(self.scene.clouds().iter().map(|r| r.id))
                    .collect();
```

## Build & Run

![Recorded visual output for Lesson 11.2 (64e-identity)](../../screenshots/journey/64e-identity.png)

```sh
npm --prefix ../session_tests run course -- dependencies 11.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 11.2
npm --prefix ../session_tests run course -- save 11.2
```

Continue with [11.3 · View-dependent LOD and cached point splats](64f-projection.md).
