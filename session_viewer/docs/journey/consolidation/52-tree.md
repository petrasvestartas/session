# 8.1 · Nested original sources and exact parent edits

**Goal:** Save nested rows and their local placements. · **Steps:** 7 · **Source:** [app/hierarchy.rs](../../../src/app/hierarchy.rs) · [app/layers.rs](../../../src/app/layers.rs) · [app/ui/layers.rs](../../../src/app/ui/layers.rs) · [app/gesture/mod.rs](../../../src/app/gesture/mod.rs) · [state/drag.rs](../../../src/state/drag.rs) · [state/box_select.rs](../../../src/state/box_select.rs) (`src/app/layers.rs,src/command_dock/mod.rs,src/app/gesture/box_select.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 8.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/hierarchy.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">7 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/layers.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/command_dock/mod.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Validate and place a nest…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Scene Tree Hierarchy</text>
  <rect x="650" y="50" width="100" height="28" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="68" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Group (Mp)</text>
  <line x1="680" y1="78" x2="640" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <line x1="720" y1="78" x2="760" y2="108" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="600" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part A</text>
  <rect x="720" y="112" width="80" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="760" y="129" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Part B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Nested original sources and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Layer Colours &amp; Lamps</text>
  <rect x="880" y="52" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="70" r="6" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="63" width="14" height="14" rx="3" fill="#ef4444" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="74" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Layer_01 (Lamp ON)</text>
  <rect x="880" y="98" width="188" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="898" cy="116" r="6" fill="#e5e7eb" stroke="#111111" stroke-width="1.25"/>
  <rect x="912" y="109" width="14" height="14" rx="3" fill="#3b82f6" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="994" y="120" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[Rename Layer_02|]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Save nested rows and thei…</text>
</svg>
</div>

## <a id="checkpoint-52-tree"></a>1. Validate and place a nested source tree

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded source walk</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">parent × local placement</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">world display</text>
</svg>
</div>

**What this code does:** Validate and place a nested source tree.

### 1. `src/hierarchy.rs`

Bound tree walks and compose precise placements along source paths.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::{Xform, proto};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub const MAX_NODES: usize = 4096;
pub const MAX_DEPTH: usize = 64;

pub fn validate(
    root: &proto::TreeNode,
    guids: &HashSet<&str>,
) -> Result<(), &'static str> {
    let mut stack = vec![(root, 0)];
    let mut seen = HashSet::new();
    let mut count = 0;
    while let Some((node, depth)) = stack.pop() {
        count += 1;
        if count > MAX_NODES || depth > MAX_DEPTH {
            return Err("Tree exceeds this checkpoint's node or depth limit");
        }
        if guids.contains(node.name.as_str()) && !seen.insert(node.name.as_str()) {
            return Err("A source object occurs twice in the tree");
        }
        stack.extend(
            node.children
                .iter()
                .rev()
                .map(|child| (child, depth + 1)),
        );
    }
    if seen != *guids {
        return Err("The tree must contain every source mesh once");
    }
    Ok(())
}

pub fn world(
    root: Option<&proto::TreeNode>,
    xforms: &HashMap<String, Xform>,
    guid: &str,
) -> Xform {
    let mut stack = root
        .into_iter()
        .map(|node| (node, Xform::identity()))
        .collect::<Vec<_>>();
    while let Some((node, parent)) = stack.pop() {
        let local = xforms
            .get(&node.name)
            .cloned()
            .unwrap_or_else(Xform::identity);
        let matrix = &parent * &local;
        if node.name == guid {
            return matrix;
        }
        stack.extend(
            node.children
                .iter()
                .rev()
                .map(|child| (child, matrix.clone())),
        );
    }
    xforms
        .get(guid)
        .cloned()
        .unwrap_or_else(Xform::identity)
}
```

### 2. `src/lib.rs`

Register the new source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod number_box_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod number_box_tests;

pub mod hierarchy;
```

## <a id="checkpoint-52a-records"></a>2. Keep lightweight tree records outside kernel ownership

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs, src/scene…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source identity</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">parent and subtree endpoi…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent lightweight o…</text>
</svg>
</div>

**What this code does:** Keep lightweight tree records outside kernel ownership.

### 1. `src/hierarchy.rs`

Store immutable document-scoped node records shared by previews and history.

**Locate this existing block:**

```rust
        .unwrap_or_else(Xform::identity)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .unwrap_or_else(Xform::identity)
}

pub struct Node {
    pub source: proto::TreeNode,
    pub object: Option<ObjectId>,
    pub label: String,
    pub parent: Option<usize>,
    pub depth: usize,
    pub end: usize,
    pub readonly: bool,
}
pub struct Document {
    pub key: u32,
    pub label: String,
    pub nodes: Vec<Node>,
}
#[derive(Clone, Default)]
pub struct Forest {
    pub documents: Vec<Rc<Document>>,
}

impl Forest {}
```

### 2. `src/scene.rs`

Allow stable row identities to participate in deduplication sets.

**Locate this existing block:**

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);
```

## <a id="checkpoint-52b-index"></a>3. Index descendants and read-only attribute groups

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document-local source nod…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">attribute ownership</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unique descendant slice</text>
</svg>
</div>

**What this code does:** Index descendants and read-only attribute groups.

### 1. `src/hierarchy.rs`

Create stable document-scoped nodes and deduplicate parent-owned descendants.

**Locate this existing block:**

```rust
}

impl Forest {}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Forest {
    pub fn add(
        &mut self,
        key: u32,
        label: String,
        root: Option<proto::TreeNode>,
        objects: &[(String, ObjectId, String)],
    ) {
        let fallback = proto::TreeNode {
            name: label.clone(),
            children: objects
                .iter()
                .map(|(guid, _, _)| proto::TreeNode {
                    name: guid.clone(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };
        let root = root.unwrap_or(fallback);
        let lookup = objects
            .iter()
            .map(|(guid, id, name)| (guid.as_str(), (*id, name.as_str())))
            .collect::<HashMap<_, _>>();
        let mut nodes: Vec<Node> = Vec::new();
        let mut stack = vec![(&root, None::<usize>, 0, false, false, false)];
        while let Some((node, parent, depth, inside, readonly, exit)) = stack.pop() {
            if exit {
                nodes[parent.unwrap()].end = nodes.len();
                continue;
            }
            let object = lookup.get(node.name.as_str()).copied();
            let readonly = readonly
                || inside
                    && matches!(
                        node.name.as_str(),
                        "attributes" | "interactions"
                    );
            let index = nodes.len();
            let source = proto::TreeNode {
                guid: if node.guid.is_empty() {
                    uuid::Uuid::new_v4().to_string()
                } else {
                    node.guid.clone()
                },
                name: node.name.clone(),
                parent_guid: node.parent_guid.clone(),
                color: node.color.clone(),
                children: Vec::new(),
            };
            nodes.push(Node {
                source,
                object: object.map(|v| v.0),
                label: object.map_or_else(|| node.name.clone(), |v| v.1.to_owned()),
                parent,
                depth,
                end: index + 1,
                readonly,
            });
            stack.push((
                node,
                Some(index),
                depth,
                inside,
                readonly,
                true,
            ));
            stack.extend(node.children.iter().rev().map(|child| {
                (
                    child,
                    Some(index),
                    depth + 1,
                    inside || object.is_some(),
                    readonly,
                    false,
                )
            }));
        }
        self.documents.push(Rc::new(Document {
            key,
            label,
            nodes,
        }));
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.documents
            .iter()
            .flat_map(|doc| &doc.nodes)
            .any(|n| n.object == Some(id) && n.readonly)
    }
    pub fn descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
        let mut out = ids.to_vec();
        for doc in &self.documents {
            for (index, node) in doc.nodes.iter().enumerate() {
                if node
                    .object
                    .is_some_and(|id| ids.contains(&id))
                {
                    out.extend(
                        doc.nodes[index + 1..node.end]
                            .iter()
                            .filter_map(|n| n.object),
                    );
                }
            }
        }
        out.sort_unstable_by_key(|id| id.value());
        out.dedup();
        out
    }
}
```

## <a id="checkpoint-52c-load"></a>4. Import nested source identities and precise placements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/document.rs, src/scene.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated protobuf</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fresh viewer IDs</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document-scoped nested re…</text>
</svg>
</div>

**What this code does:** Import nested source identities and precise placements.

### 1. `src/document.rs`

Capture nested tree metadata and replace flat-only validation with the bounded source walk.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .as_ref()
        .and_then(|tree| tree.root.as_ref())
    {
        let names: HashSet<_> = root
            .children
            .iter()
            .map(|node| node.name.as_str())
            .collect();
        if names != guids
            || root.children.len() != guids.len()
            || root
                .children
                .iter()
                .any(|node| !node.children.is_empty())
        {
            return Err("This checkpoint needs one flat tree row per mesh");
        }
    }
```

Replace it with:

```rust
        .as_ref()
        .and_then(|tree| tree.root.as_ref())
    {
        crate::hierarchy::validate(root, &guids)?;
    }
```

**Locate this block:**

```rust
        }
        Ok(meshes)
    })?;
    Ok(Loaded { document, meshes })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
```

Replace it with:

```rust
        }
        Ok(meshes)
    })?;
    Ok(Loaded {
        document,
        meshes,
        tree,
    })
}

fn decode_raw(bytes: &[u8]) -> Result<proto::Session, &'static str> {
```

**Locate this block:**

```rust
        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model = document
                .xforms
                .get(mesh.guid())
                .cloned()
                .unwrap_or_else(session_rust::Xform::identity);
            prepared.source = Some(Source {
                document: Rc::clone(&document),
                guid: mesh.guid().to_owned(),
```

Replace it with:

```rust
        let mut meshes = Vec::new();
        for mesh in document.objects.meshes.iter() {
            let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
            prepared.model =
                crate::hierarchy::world(tree.as_ref(), &document.xforms, mesh.guid());
            prepared.source = Some(Source {
                document: Rc::clone(&document),
                guid: mesh.guid().to_owned(),
```

**Locate this block:**

```rust
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message)
            .map(Rc::new)
```

Replace it with:

```rust
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let tree = message
        .tree
        .as_ref()
        .and_then(|tree| tree.root.clone());
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message)
            .map(Rc::new)
```

**Locate this block:**

```rust
    pub meshes: Vec<PreparedMesh>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
```

Replace it with:

```rust
    pub meshes: Vec<PreparedMesh>,
    pub tree: Option<proto::TreeNode>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
```

### 2. `src/scene.rs`

Bind imported tree nodes to fresh row IDs and release their records when the document closes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        for prepared in loaded.meshes {
            self.insert(prepared)?;
        }
        Ok(())
```

Replace it with:

```rust
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        let mut objects = Vec::new();
        let key = self.next_id;
        for prepared in loaded.meshes {
            let guid = prepared.geometry.guid().to_owned();
            let name = prepared.geometry.name.clone();
            let id = self.insert(prepared)?;
            objects.push((guid, id, name));
        }
        self.forest.add(
            key,
            loaded.document.name.clone(),
            loaded.tree,
            &objects,
        );
        Ok(())
```

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.forest = Default::default();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
```

**Locate this block:**

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
```

Replace it with:

```rust
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            forest: Default::default(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
```

**Locate this block:**

```rust
pub struct Scene {
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
```

Replace it with:

```rust
pub struct Scene {
    pub forest: crate::hierarchy::Forest,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
```

## <a id="checkpoint-52d-preview"></a>5. Carry parent-owned descendants in one preview

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/edit_prev…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">eligible selected roots</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fixed baseline world delta</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared-source preview</text>
</svg>
</div>

**What this code does:** Carry parent-owned descendants in one preview.

### 1. `src/scene.rs`

Protect independent attribute edits while exposing the checked descendant path for parent-owned transformations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .map(|r| r.id),
            )
            .collect();
        ids.sort_unstable_by_key(|id| id.value());
```

Replace it with:

```rust
                    .map(|r| r.id),
            )
            .collect();
        ids.retain(|id| !self.forest.readonly(*id));
        ids.sort_unstable_by_key(|id| id.value());
```

**Locate this block:**

```rust
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
```

Replace it with:

```rust
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_mesh(id, model)
    }

    fn place_mesh(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
```

**Locate this block:**

```rust
            })
    }

    pub fn place_owner(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id == id) {
            self.place(id, model)
        } else if self.lines.iter().any(|r| r.id == id) {
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
```

Replace it with:

```rust
            })
    }

    pub fn with_descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
        self.forest
            .descendants(ids)
            .into_iter()
            .filter(|id| self.contains(*id))
            .collect()
    }

    pub fn place_owner(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        self.place_following(id, model)
    }

    pub(crate) fn place_following(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id == id) {
            self.place_mesh(id, model)
        } else if self.lines.iter().any(|r| r.id == id) {
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
```

**Locate this block:**

```rust
            )
            .find(|id| id.value() == value)
    }

    pub fn demo() -> Self {
```

Replace it with:

```rust
            )
            .find(|id| id.value() == value && !self.forest.readonly(*id))
    }

    pub fn demo() -> Self {
```

### 2. `src/edit_preview.rs`

Capture selected roots separately from their unique following descendants.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        history.try_edit(scene, |scene| {
            for &id in &self.ids {
                scene.place_owner(
                    id,
```

Replace it with:

```rust
        }
        history.try_edit(scene, |scene| {
            for &id in &self.ids {
                scene.place_following(
                    id,
```

**Locate this block:**

```rust
        history: &mut History,
    ) -> Result<bool, &'static str> {
        for &id in &self.ids {
            if scene.pick_id(id.value()) != Some(id)
                || scene.placement(id).map(|m| m.m)
                    != self.base.placement(id).map(|m| m.m)
            {
```

Replace it with:

```rust
        history: &mut History,
    ) -> Result<bool, &'static str> {
        for &id in &self.ids {
            if self.roots.contains(&id) && scene.pick_id(id.value()) != Some(id)
                || scene.placement(id).map(|m| m.m)
                    != self.base.placement(id).map(|m| m.m)
            {
```

**Locate this block:**

```rust
                    .base
                    .placement(id)
                    .ok_or("Preview owner missing")?;
            scene.place_owner(id, model)?;
        }
        self.scene = scene;
        self.transform = transform;
```

Replace it with:

```rust
                    .base
                    .placement(id)
                    .ok_or("Preview owner missing")?;
            scene.place_following(id, model)?;
        }
        self.scene = scene;
        self.transform = transform;
```

**Locate this block:**

```rust
                return Err("Reload editable sources before Move");
            }
        }
        Ok(Self {
            base: scene.clone(),
            scene: scene.clone(),
            ids,
            transform: Xform::identity(),
        })
    }
```

Replace it with:

```rust
                return Err("Reload editable sources before Move");
            }
        }
        let roots = ids;
        let ids = scene.with_descendants(&roots);
        Ok(Self {
            base: scene.clone(),
            scene: scene.clone(),
            ids,
            roots,
            transform: Xform::identity(),
        })
    }
```

**Locate this block:**

```rust
    ids: Vec<ObjectId>,
    transform: Xform,
}
```

Replace it with:

```rust
    ids: Vec<ObjectId>,
    roots: Vec<ObjectId>,
    transform: Xform,
}
```

## <a id="checkpoint-52e-edits"></a>6. Commit parent moves and deletions as one transaction

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated parent action</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history transaction</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact shared-source undo</text>
</svg>
</div>

**What this code does:** Commit parent moves and deletions as one transaction.

### 1. `src/editor.rs`

Route group selection and parent edits through protected source ownership and one history boundary.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.draft = None;
        }
        match action {
            Action::GizmoPress {
```

Replace it with:

```rust
            self.draft = None;
        }
        match action {
            Action::TreePick { ids, additive } => {
                let eligible = ids
                    .into_iter()
                    .filter(|id| self.scene.pick_id(id.value()) == Some(*id))
                    .collect::<Vec<_>>();
                if eligible.is_empty() {
                    return Err("This tree row has no visible editable objects");
                }
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
            Action::GizmoPress {
```

**Locate this block:**

```rust
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
```

Replace it with:

```rust
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
```

**Locate this block:**

```rust
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    scene.translate(id, offset)?;
                }
                Ok(())
            })?;
        Ok(Change::Scene)
    }
```

Replace it with:

```rust
        if offset.iter().all(|&n| n == 0.) {
            return Ok(Change::View);
        }
        let mut preview = crate::edit_preview::Preview::new(&self.scene, ids)?;
        preview.set(session_rust::Xform::translation(
            offset[0], offset[1], offset[2],
        ))?;
        preview.commit(&mut self.scene, &mut self.history)?;
        Ok(Change::Scene)
    }
```

**Locate this block:**

```rust
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
```

Replace it with:

```rust
        {
            return Err("Reload editable sources before Delete");
        }
        let ids = self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {
            for id in ids {
                scene.remove(id);
            }
        });
        if self.edit_preview.take().is_some() {
            self.draft = None;
```

**Locate this block:**

```rust
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
```

Replace it with:

```rust
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if self
            .scene
            .objects()
```

**Locate this block:**

```rust
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
```

Replace it with:

```rust
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let ids = self.scene.with_descendants(&[id]);
        let delta = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        self.history
            .try_edit(&mut self.scene, |scene| {
                for id in ids {
                    let matrix = &delta
                        * &scene
                            .placement(id)
                            .ok_or("Object not found")?;
                    scene.place_following(id, matrix)?;
                }
                Ok(())
            })?;
        if self.edit_preview.take().is_some() {
            self.draft = None;
```

**Locate this block:**

```rust
pub enum Action {
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
```

Replace it with:

```rust
pub enum Action {
    TreePick {
        ids: Vec<ObjectId>,
        additive: bool,
    },
    GizmoPress {
        pointer: i32,
        screen: [f32; 2],
```

## <a id="checkpoint-52f-save"></a>7. Save nested rows and their local placements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs, src/docum…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current saved sources</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">inverse parent × world pl…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/layers.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">portable local records</text>
</svg>
</div>

**What this code does:** Save nested rows and their local placements.

### 1. `src/hierarchy.rs`

Rebuild nested portable nodes and recover local placements from the edited world scene.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn snapshot(
        &self,
        scene: &Scene,
        xforms: &mut Vec<proto::XformEntry>,
    ) -> Result<(Vec<proto::TreeNode>, HashSet<ObjectId>), &'static str> {
        let mut roots = Vec::new();
        let mut placed = HashSet::new();
        for doc in &self.documents {
            if let Some(root) = save_node(
                doc,
                0,
                scene,
                &Xform::identity(),
                "",
                xforms,
                &mut placed,
            )? {
                roots.push(root);
            }
        }
        Ok((roots, placed))
    }
}

fn save_node(
    doc: &Document,
    index: usize,
    scene: &Scene,
    parent: &Xform,
    parent_guid: &str,
    xforms: &mut Vec<proto::XformEntry>,
    placed: &mut HashSet<ObjectId>,
) -> Result<Option<proto::TreeNode>, &'static str> {
    let original = &doc.nodes[index];
    let mut node = original.source.clone();
    node.guid = uuid::Uuid::new_v4().to_string();
    node.parent_guid = parent_guid.to_owned();
    let mut matrix = parent.clone();
    if let Some(id) = original.object {
        let Some(row) = scene.objects().iter().find(|r| r.id == id) else {
            return Ok(None);
        };
        node.name = row.guid.clone();
        matrix = row.model.clone();
        let local = &parent
            .inverse()
            .ok_or("Parent placement cannot be inverted for Save")?
            * &matrix;
        if local.m != Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: row.guid.clone(),
                xform: Some(local.to_proto()),
            });
        }
        placed.insert(id);
    }
    for child in index + 1..original.end {
        if doc.nodes[child].parent == Some(index) {
            if let Some(child) = save_node(
                doc, child, scene, &matrix, &node.guid, xforms, placed,
            )? {
                node.children.push(child);
            }
        }
    }
    Ok(Some(node))
}
```

### 2. `src/document.rs`

Preserve nested imported trees while keeping generated objects beside them in the saved document.

**Locate this existing block:**

```rust
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
    let mut xforms = Vec::new();
    for object in scene.objects() {
        let geometry = object
            .geometry()
            .ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        children.push(proto::TreeNode {
            name: object.guid.clone(),
            ..Default::default()
        });
        if object.model.m != session_rust::Xform::identity().m {
            xforms.push(proto::XformEntry {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut objects = proto::Objects::default();
    let mut xforms = Vec::new();
    let (mut children, placed) = scene.forest.snapshot(scene, &mut xforms)?;
    for object in scene.objects() {
        let geometry = object
            .geometry()
            .ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        if !placed.contains(&object.id) {
            children.push(proto::TreeNode {
                name: object.guid.clone(),
                ..Default::default()
            });
        }
        if !placed.contains(&object.id)
            && object.model.m != session_rust::Xform::identity().m
        {
            xforms.push(proto::XformEntry {
```

## Build & Run

![Recorded visual output for Lesson 8.1 (52f-save)](../../screenshots/journey/52f-save.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 8.1
npm --prefix ../session_tests run course -- save 8.1
```

Continue with [8.2 · Tree presentation with explicit input ownership](52g-rows.md).
