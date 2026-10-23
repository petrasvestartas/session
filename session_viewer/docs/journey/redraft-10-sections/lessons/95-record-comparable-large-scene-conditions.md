# 18.7 · Measured frame costs and dependency-aware cache reports

**Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** · **Lesson 18.7** (Redraft `9.11`) · **7 checkpoints (`95-record-comparable-large-scene-conditions` → `95f-prove-cache-invalidation-and-honest-performance-reports`)**

> **Learning Goal:** Record comparable large scene conditions.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 9 Compute Tile Visibility, Clipping Half-Spaces, Winding Section Caps & Horizon GTAO (Ch 17–18)</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How @compute shaders bin triangles into screen tiles for hidden-line removal, clip solids with winding caps, and shade crevices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. @compute Tile Visibility (Ch 17)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• projected_triangle.wgsl projects tris</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• atomicAdd tile counts + prefix-sum CSR</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• ink_visibility.wgsl tests center-line</text>
    <text x="28" y="160" font-size="10.5" fill="#1e3a8a">  against covering triangles in tile</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">• Hidden lines dashed/faded cleanly!</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="300" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. Clipping &amp; Winding Caps (Ch 17)</text>
    <text x="312" y="108" font-size="10.5" fill="#7c2d12">• Half-space: dot(pos_world, plane) &lt; 0</text>
    <text x="312" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• Signed front/back crossing count</text>
    <text x="312" y="142" font-size="10" fill="#7c2d12">  at clip plane depth (cap.wgsl):</text>
    <text x="312" y="158" font-size="10" fill="#7c2d12">  Fills closed solid interiors while</text>
    <text x="312" y="173" font-size="10" fill="#7c2d12">  leaving internal bores hollow!</text>
    <text x="312" y="191" font-size="10.5" fill="#7c2d12">• Cap picks resolve to solid owner_id</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="597" y="68" width="223" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Horizon GTAO &amp; X-Ray (Ch 18)</text>
    <text x="609" y="108" font-size="10.5" fill="#064e3b">• ambient_depth.wgsl -&gt; 6-mip pyramid</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#047857">• ssao.wgsl + bilateral blur +</text>
    <text x="609" y="142" font-size="10.5" font-weight="600" fill="#047857">  ambient_composite.wgsl (Arctic)</text>
    <text x="609" y="160" font-size="10" fill="#064e3b">• Glass / X-Ray translucent shells</text>
    <text x="609" y="175" font-size="10" fill="#064e3b">  allow picking interior Wood joints</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.
```

- Record comparable large scene conditions.
- Witness immutable tree inputs across camera frames.
- Prove cache invalidation and honest performance reports.
- **Why it works (Can another GPU or smaller viewport validate the Lenovo fix?):** No. Record the machine, browser, adapter, exact original scene hash, viewport/DPR, saved camera, display settings and measurement method. Keep load time separate from rotation and zoom.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

**Production source (`session_viewer/src`):** [engine/gpu/ssao.rs](../../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../../src/app/command/tool/elements.rs), [build.rs](../../../../build.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-95-record-comparable-large-scene-conditions"></a>

<details>
<summary>Implementation 95-record-comparable-large-scene-conditions: Record comparable large scene conditions</summary>

Record comparable large scene conditions.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/bench_config.rs`

Own explicit conditions instead of unqualified performance claims.

Create the file and type:

```rust
--8<-- "journey/code/95-record-comparable-large-scene-conditions-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod comparison_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/95-record-comparable-large-scene-conditions-02.rs" skip=35
```

</details>

<a id="checkpoint-95a-measure-warmed-frame-times-and-the-slow-tail"></a>

<details>
<summary>Implementation 95a-measure-warmed-frame-times-and-the-slow-tail: Measure warmed frame times and the slow tail</summary>

Measure warmed frame times and the slow tail.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/bench_samples.rs`

Summarize real samples with an explicit warmup and slow-frame tail.

Create the file and type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod bench_config;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-02.rs" skip=23
```

</details>

<a id="checkpoint-95b-reject-incomparable-before-and-after-runs"></a>

<details>
<summary>Implementation 95b-reject-incomparable-before-and-after-runs: Reject incomparable before and after runs</summary>

Reject incomparable before and after runs.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/bench_compare.rs`

Compare matched measurements and retain original-source memory accounting.

Create the file and type:

```rust
--8<-- "journey/code/95b-reject-incomparable-before-and-after-runs-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod bench_samples;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/95b-reject-incomparable-before-and-after-runs-02.rs" skip=24
```

</details>

<a id="checkpoint-95c-witness-immutable-tree-inputs-across-camera-frames"></a>

<details>
<summary>Implementation 95c-witness-immutable-tree-inputs-across-camera-frames: Witness immutable tree inputs across camera frames</summary>

Witness immutable tree inputs across camera frames.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/tree_witness.rs`

Witness shared layer inputs without retaining geometry payloads.

Create the file and type:

```rust
--8<-- "journey/code/95c-witness-immutable-tree-inputs-across-camera-frames-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod bench_compare;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/95c-witness-immutable-tree-inputs-across-camera-frames-02.rs" skip=24
```

</details>

<a id="checkpoint-95d-reuse-layer-rows-while-rotating-and-zooming"></a>

<details>
<summary>Implementation 95d-reuse-layer-rows-while-rotating-and-zooming: Reuse layer rows while rotating and zooming</summary>

Reuse layer rows while rotating and zooming.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/tree_panel.rs`

Cache unchanged scene/selection/epoch inputs before rebuilding layer state.

<details>
<summary>Locate the existing block</summary>

```rust

#[derive(Default)]
pub struct TreePanel {pub graph_open:bool,pub graph:Vec<(String,String,String,String)>,pub edit:Option<(String,crate::layer_edit::Edit)>,pub rename:Option<(String,String)>,pub rename_error:String,rename_focus:bool,pub confirm:Option<(String,usize)>,pub property:Option<(String,crate::layer_properties::Property)>,pub menu_rects:Vec<egui::Rect>,states:std::collections::HashMap<String,RowState>,pub open:bool,pub rows:Vec<Row>,pub selected:Vec<ObjectId>,expanded:HashSet<String>,pub rect:Option<egui::Rect>,pub action:Option<(Vec<ObjectId>,bool)>,page:usize}
impl TreePanel {
    pub fn refresh(&mut self,scene:&crate::scene::Scene,selected:Vec<ObjectId>) {
        self.graph=crate::graph_edit::rows(scene);
        for doc in &scene.forest.documents{for(index,node)in doc.nodes.iter().enumerate(){if scene.groups.contains(&(doc.key,node.source.guid.clone()))&&doc.nodes[index+1..node.end].iter().filter_map(|n|n.object).any(|id|selected.contains(&id)){let mut parent=Some(index);while let Some(i)=parent{self.expanded.insert(format!("{}/{}",doc.key,doc.nodes[i].source.guid));parent=doc.nodes[i].parent;}}}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95d-reuse-layer-rows-while-rotating-and-zooming-01.rs"
```

### 2. `src/browser.rs`

Use the cache from the actual teaching frame loop.

<details>
<summary>Locate the existing block</summary>

```rust
        panel.number(number);
        panel.lasso=editor.lasso.as_ref().map_or_else(Vec::new,|l|l.points.iter().map(|p|[(p.0/l.density) as f32,(p.1/l.density) as f32]).collect());
        panel.tree.refresh(&editor.scene,editor.selected_objects());
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95d-reuse-layer-rows-while-rotating-and-zooming-02.rs"
```

</details>

<a id="checkpoint-95e-borrow-graph-edges-and-index-their-labels"></a>

<details>
<summary>Implementation 95e-borrow-graph-edges-and-index-their-labels: Borrow graph edges and index their labels</summary>

Borrow graph edges and index their labels.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

### 1. `src/graph_edit.rs`

Avoid full graph protobuf conversion and repeated node scans for display rows.

<details>
<summary>Locate the existing block</summary>

```rust

pub fn rows(scene:&Scene)->Vec<(String,String,String,String)>{
    scene.forest.documents.iter().flat_map(|doc|{let graph=scene.graphs.get(&doc.key).map(|g|g.to_proto()).or_else(||doc.header.as_ref().and_then(|h|h.header.graph.clone()));let name=|guid:&str|doc.nodes.iter().find(|n|n.source.name==guid||n.object.is_some_and(|id|scene.row_guid(id)==Some(guid))).map_or_else(||guid.to_owned(),|n|n.label.clone());graph.into_iter().flat_map(|g|g.edges.into_iter().map(|edge|(doc.label.clone(),name(&edge.v0),name(&edge.v1),edge.attribute))).collect::<Vec<_>>()}).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95e-borrow-graph-edges-and-index-their-labels-01.rs"
```

</details>

<a id="checkpoint-95f-prove-cache-invalidation-and-honest-performance-reports"></a>

<details>
<summary>Implementation 95f-prove-cache-invalidation-and-honest-performance-reports: Prove cache invalidation and honest performance reports</summary>

Prove cache invalidation and honest performance reports.

Data flow: record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/bench_tests.rs`

Copy supplied invalid-data, comparability, original-source, cache and history fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/95f-prove-cache-invalidation-and-honest-performance-reports-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied performance correctness fixtures.

<details>
<summary>Locate the existing block</summary>

```text

pub mod tree_witness;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/95f-prove-cache-invalidation-and-honest-performance-reports-02.rs" skip=23
```

</details>

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 95f-prove-cache-invalidation-and-honest-performance-reports
npm --prefix ../session_tests run course -- check 95f-prove-cache-invalidation-and-honest-performance-reports
npm --prefix ../session_tests run course -- save 95f-prove-cache-invalidation-and-honest-performance-reports
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The original comparison scene stays visible; cache fixtures prove that camera movement avoids repeated tree reconstruction.

- **Try this:** Inspect the supplied cache, graph and measurement fixtures. Follow the linked floor/dome recipe; record browser errors, raw timing data, quality and memory release separately.
