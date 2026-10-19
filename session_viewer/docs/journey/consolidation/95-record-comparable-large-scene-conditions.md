# 18.7 · Measured frame costs and dependency-aware cache reports

Measure a comparable large scene and reuse immutable presentation data only under valid dependency witnesses.

## Ownership and data flow

Record actual scene counts, source identity, camera, viewport/DPR, sample count, adapter/backend and warmup before timing. Report warmed distributions and the slow tail separately from startup and GPU completion. Reject before/after runs with mismatched conditions. Tree/layer/graph caches borrow immutable source structure and invalidate after actual dependency changes, not every camera frame; source ownership must remain unchanged. Private optimization proposals and uncompiled drafts are not completed improvements. Software-GPU measurements describe that environment and cannot become hardware-FPS claims or final responsiveness acceptance.

## The maintained implementation

engine/performance.rs tracks maintained frames, draw counts, interaction windows and adaptive quality tiers; engine/gpu/timing.rs records GPU scopes. app/hierarchy.rs refreshes on row_revision and app/ui/layers.rs consumes current rows; app/inspection.rs reports actual owners/counters. Measure same final runtime inputs after integration before claiming a performance gain; this consolidation runs no benchmark.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs), [build.rs](../../../build.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

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

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 95f-prove-cache-invalidation-and-honest-performance-reports
npm --prefix ../session_tests run course -- check 95f-prove-cache-invalidation-and-honest-performance-reports
npm --prefix ../session_tests run course -- save 95f-prove-cache-invalidation-and-honest-performance-reports
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The following is the expected runnable-checkpoint behavior. This prose batch did not rerun its GPU fixtures; current finite-ink, section and comparison acceptance remains pending after algorithm reconciliation.

The original comparison scene stays visible; cache fixtures prove that camera movement avoids repeated tree reconstruction.

Inspect the supplied cache, graph and measurement fixtures. Follow the linked floor/dome recipe; record browser errors, raw timing data, quality and memory release separately.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.
