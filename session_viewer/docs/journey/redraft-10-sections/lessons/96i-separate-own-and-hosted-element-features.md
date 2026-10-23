# 19.2 · Original Element ancestry and derived feature trees

**Goal:** Share the existing dictionary wrapper with the real widget regression and browser typing proof. · **Steps:** 10 · **Source:** [app/session_io.rs](../../../../src/app/session_io.rs) · [app/command/mod.rs](../../../../src/app/command/mod.rs) · [app/hierarchy.rs](../../../../src/app/hierarchy.rs) · [app/scene.rs](../../../../src/app/scene.rs) · [app/selection.rs](../../../../src/app/selection.rs) · [app/deform.rs](../../../../src/app/deform.rs) · [app/walk/mod.rs](../../../../src/app/walk/mod.rs) (`src/app/inspect.rs,src/app/families.rs,src/command_dock/families.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 19.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/feature_plan.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/hierarchy.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">10 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/inspect.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/families.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Separate own attributes f…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Retained Source Originals</text>
  <rect x="598" y="52" width="204" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Raw Planes &amp; Source IDs</text>
  <rect x="598" y="100" width="204" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Validated Frame Records</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original Element ancestry a…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Family Key Controls</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Family: Curves [ON]</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#fef9c3" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Family: Solids [ON]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove exact commands acro…</text>
</svg>
</div>

## <a id="checkpoint-96i-separate-own-and-hosted-element-features"></a>1. Separate own attributes from hosted features

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/feature_plan.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry.

### 1. `src/feature_plan.rs`

Give this operation its own small owner.

Create the file and type:

```rust
use session_rust::element::ElementFeature;
use session_rust::{Element, Geometry, Session};

#[derive(Debug, PartialEq)]
pub struct Hosted {
    pub label: String,
    pub features: Vec<usize>,
}

pub fn interaction(feature: &ElementFeature) -> bool {
    matches!(
        feature.feature_type.as_str(),
        "contact" | "joint" | "cut" | "drill"
    )
}

pub fn own(element: &Element) -> Vec<usize> {
    element
        .features
        .iter()
        .enumerate()
        .filter_map(|(i, feature)| (!interaction(feature)).then_some(i))
        .collect()
}

pub fn label(feature: &ElementFeature) -> String {
    let mut label = feature.feature_type.clone();
    if !feature.name.is_empty() && feature.name != feature.feature_type {
        label.push(' ');
        label.push_str(&feature.name);
    }
    if feature.face_index >= 0 {
        label.push_str(&format!(" · face {}", feature.face_index));
    }
    label
}

fn source_name<'a>(session: &'a Session, guid: &'a str) -> &'a str {
    match session.lookup.get(guid) {
        Some(Geometry::Element(element)) if !element.name.is_empty() => &element.name,
        _ => guid,
    }
}
```

### 2. `src/lib.rs`

Register the new owner without changing the browser entry point.

**Locate this existing block:**

```rust
mod metadata_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod metadata_tests;

pub mod feature_plan;

#[cfg(test)]
```

## <a id="checkpoint-96j-group-features-by-original-interaction"></a>2. Group pieces by their interaction

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/feature_plan.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Walk graph edges in their original order. Group contact and joint pieces under the full interaction type, using the relation name or source Element name, and retain unmatched features.

### 1. `src/feature_plan.rs`

Claim each feature once in graph order; retain unknown and orphaned interaction features.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub fn hosted(session: &Session, element: &Element) -> Vec<Hosted> {
    let mut edges: Vec<_> = session
        .graph
        .edges
        .get(element.guid())
        .into_iter()
        .flatten()
        .collect();
    edges.sort_by_key(|(_, edge)| edge.index);
    let mut claimed = vec![false; element.features.len()];
    let mut groups = Vec::new();
    for (other, edge) in edges {
        for relation in session
            .interactions
            .get(edge.guid())
            .into_iter()
            .flatten()
        {
            let features: Vec<_> = element
                .features
                .iter()
                .enumerate()
                .filter(|(i, feature)| {
                    !claimed[*i]
                        && interaction(feature)
                        && feature.has_guid()
                        && (feature.guid() == relation.guid()
                            || relation
                                .interaction_type_name()
                                .starts_with("InteractionFeature")
                                && feature.guid().starts_with(other.as_str()))
                })
                .map(|(i, _)| i)
                .collect();
            if features.is_empty() {
                continue;
            }
            for &i in &features {
                claimed[i] = true;
            }
            let name = if relation.name().is_empty() {
                source_name(session, other)
            } else {
                relation.name()
            };
            groups.push(Hosted {
                label: format!("{} {name}", relation.interaction_type_name()),
                features,
            });
        }
    }
    for (i, feature) in element.features.iter().enumerate() {
        if !claimed[i] && interaction(feature) {
            groups.push(Hosted {
                label: label(feature),
                features: vec![i],
            });
        }
    }
    groups
}
```

## <a id="checkpoint-96k-separate-derived-rows-from-editable-elements"></a>3. Keep nested Elements editable and projections unsaved

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs, src/layer…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Mark generated display rows separately from original objects. Nested child Elements remain editable even inside attributes, and saved documents omit derived geometry.

### 1. `src/hierarchy.rs`

Record display provenance explicitly on each tree node. Imported tree nodes initially belong to the source.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                children: Vec::new(),
            };
            nodes.push(Node {
                placement: Default::default(),
```

Replace it with:

```rust
                children: Vec::new(),
            };
            nodes.push(Node {
                derived: false,
                placement: Default::default(),
```

**Locate this block:**

```rust
pub struct Node {
    pub placement: crate::nested_placement::Node,
    pub portable_guid: String,
    pub source: proto::TreeNode,
```

Replace it with:

```rust
pub struct Node {
    pub derived: bool,
    pub placement: crate::nested_placement::Node,
    pub portable_guid: String,
    pub source: proto::TreeNode,
```

### 2. `src/layer_edit.rs`

Keep user-created groups and source objects in the saved tree.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let group_guid = source.guid.clone();
    let mut group = Branch {
        node: Node {
            placement: crate::nested_placement::Node {
```

Replace it with:

```rust
    let group_guid = source.guid.clone();
    let mut group = Branch {
        node: Node {
            derived: false,
            placement: crate::nested_placement::Node {
```

**Locate this block:**

```rust
    };
    let child = Branch {
        node: Node {
            placement: Default::default(),
            portable_guid: source.guid.clone(),
            source,
```

Replace it with:

```rust
    };
    let child = Branch {
        node: Node {
            derived: false,
            placement: Default::default(),
            portable_guid: source.guid.clone(),
            source,
```

**Locate this block:**

```rust
            node: Node {
                placement: Default::default(),
                portable_guid: source.guid.clone(),
                source,
```

Replace it with:

```rust
            node: Node {
                derived: false,
                placement: Default::default(),
                portable_guid: source.guid.clone(),
                source,
```

### 3. `src/hierarchy.rs`

Reset read-only inheritance at each real child Element; retain it for feature pieces. Skip projected subtrees when saving originals.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    placed: &mut HashSet<ObjectId>,
) -> Result<Option<proto::TreeNode>, &'static str> {
    let original = &doc.nodes[index];
    let mut node = original.source.clone();
```

Replace it with:

```rust
    placed: &mut HashSet<ObjectId>,
) -> Result<Option<proto::TreeNode>, &'static str> {
    let original = &doc.nodes[index];
    if original.derived {
        return Ok(None);
    }
    let mut node = original.source.clone();
```

**Locate this block:**

```rust
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.readonly_ids.contains(&id)
    }
```

Replace it with:

```rust
    }

    pub fn classify_elements(&mut self, key: u32, elements: &HashSet<ObjectId>) {
        let owner = self
            .documents
            .iter()
            .position(|doc| doc.key == key)
            .unwrap();
        let mut doc = (*self.documents[owner]).clone();
        let mut inside = vec![false; doc.nodes.len()];
        for i in 0..doc.nodes.len() {
            let parent = doc.nodes[i].parent;
            let element = doc.nodes[i]
                .object
                .is_some_and(|id| elements.contains(&id));
            let inherited = parent.is_some_and(|p| inside[p]);
            let held = parent.is_some_and(|p| doc.nodes[p].readonly);
            if element || inherited {
                doc.nodes[i].readonly = !element
                    && (held
                        || matches!(
                            doc.nodes[i].source.name.as_str(),
                            "attributes" | "interactions"
                        ));
            }
            inside[i] = element || inherited;
        }
        self.replace(owner, doc);
    }

    pub fn derived(&self, id: ObjectId) -> bool {
        self.documents.iter().any(|doc| {
            doc.nodes
                .iter()
                .any(|node| node.object == Some(id) && node.derived)
        })
    }

    pub fn readonly(&self, id: ObjectId) -> bool {
        self.readonly_ids.contains(&id)
    }
```

### 4. `src/scene_archive.rs`

Only original tree objects enter each archive namespace. Do not reclassify derived paths as loose original objects.

**Locate this existing block:**

```rust
                .iter()
                .filter_map(|node| node.object)
                .filter(|id| scene.contains(*id))
                .collect(),
        })
        .collect::<Vec<_>>();
    let all = scene
        .objects()
        .iter()
        .map(|row| row.id)
        .chain(scene.lines().iter().map(|row| row.id))
        .chain(scene.paths().iter().map(|row| row.id))
        .chain(scene.points().iter().map(|row| row.id))
        .chain(scene.clouds().iter().map(|row| row.id))
        .chain(scene.texts().iter().map(|row| row.id))
        .chain(scene.sheets().iter().map(|row| row.id))
        .collect::<HashSet<_>>();
    let assigned = out
        .iter()
        .flat_map(|namespace| namespace.ids.iter().copied())
        .collect::<HashSet<_>>();
    let ids = all
        .difference(&assigned)
        .copied()
        .collect::<HashSet<_>>();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .iter()
                .filter(|node| !node.derived)
                .filter_map(|node| node.object)
                .filter(|id| scene.contains(*id))
                .collect(),
        })
        .collect::<Vec<_>>();
    let all = scene
        .objects()
        .iter()
        .map(|row| row.id)
        .chain(scene.lines().iter().map(|row| row.id))
        .chain(scene.paths().iter().map(|row| row.id))
        .chain(scene.points().iter().map(|row| row.id))
        .chain(scene.clouds().iter().map(|row| row.id))
        .chain(scene.texts().iter().map(|row| row.id))
        .chain(scene.sheets().iter().map(|row| row.id))
        .collect::<HashSet<_>>();
    let assigned = out
        .iter()
        .flat_map(|namespace| namespace.ids.iter().copied())
        .collect::<HashSet<_>>();
    let ids = all
        .difference(&assigned)
        .copied()
        .filter(|id| !scene.forest.derived(*id))
        .collect::<HashSet<_>>();
```

## <a id="checkpoint-96l-attach-a-bounded-derived-feature-tree"></a>4. Attach a bounded feature tree to its host

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/layer_edit.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Build read-only attributes, interaction and piece rows under the owning Element. Validate the complete tree before replacing its saved structure.

### 1. `src/layer_edit.rs`

Reuse the existing branch transaction and node/depth limits for feature rows.

**Locate this existing block:**

```rust
    replace(scene, owner, root);
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    replace(scene, owner, root);
    Ok(())
}

pub enum FeatureTree {
    Group(String, Vec<FeatureTree>),
    Piece(String, crate::scene::ObjectId),
}

pub fn attach_features(
    scene: &mut Scene,
    id: crate::scene::ObjectId,
    groups: Vec<FeatureTree>,
) -> Result<(), &'static str> {
    fn build(scene: &Scene, tree: FeatureTree) -> Result<Branch, &'static str> {
        let (label, object, children) = match tree {
            FeatureTree::Group(label, children) => (label, None, children),
            FeatureTree::Piece(label, id) => (label, Some(id), Vec::new()),
        };
        let name = match object {
            Some(id) => scene
                .row_guid(id)
                .ok_or("Feature piece is missing")?
                .to_owned(),
            None => label.clone(),
        };
        let source = session_rust::proto::TreeNode {
            guid: uuid::Uuid::new_v4().to_string(),
            name,
            ..Default::default()
        };
        let node = Node {
            source: source.clone(),
            portable_guid: source.guid,
            object,
            label,
            parent: None,
            depth: 0,
            end: 1,
            readonly: true,
            derived: true,
            placement: Default::default(),
        };
        let children = children
            .into_iter()
            .map(|child| build(scene, child))
            .collect::<Result<_, _>>()?;
        Ok(Branch { node, children })
    }
    let (owner, index) = scene
        .forest
        .documents
        .iter()
        .enumerate()
        .find_map(|(owner, doc)| {
            doc.nodes
                .iter()
                .position(|node| node.object == Some(id))
                .map(|index| (owner, index))
        })
        .ok_or("Element tree is missing")?;
    let doc = &scene.forest.documents[owner];
    let guid = doc.nodes[index].source.guid.clone();
    let mut tree = branch(doc, 0);
    let parent = find(&mut tree, &guid).unwrap();
    for group in groups {
        let group = build(scene, group)?;
        if let Some(existing) = parent.children.iter_mut().find(|child| {
            child.node.object.is_none()
                && child.node.source.name == group.node.source.name
        }) {
            existing.children.extend(group.children);
        } else {
            parent.children.push(group);
        }
    }
    let mut nodes = Vec::new();
    flatten(tree.clone(), None, 0, &mut nodes);
    if nodes.len() > crate::hierarchy::MAX_NODES
        || nodes
            .iter()
            .any(|node| node.depth > crate::hierarchy::MAX_DEPTH)
    {
        return Err("Element features exceed the tree limit");
    }
    replace(scene, owner, tree);
    Ok(())
}
```

## <a id="checkpoint-96m-project-original-feature-outlines"></a>5. Project outlines without changing their coordinates

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/feature_display.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Create black display paths from original feature outlines, apply the host placement once, and keep attributes Off and interactions On by default.

### 1. `src/feature_display.rs`

Give this operation its own small owner.

Create the file and type:

```rust
use crate::{
    layer_edit::FeatureTree,
    scene::{ObjectId, Scene},
};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(
    scene: &mut Scene,
    id: ObjectId,
    session: &Session,
    element: &Element,
) -> Result<(), &'static str> {
    let model = scene
        .placement(id)
        .ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let mut source = outline.clone();
                    source.set_guid(uuid::Uuid::new_v4().to_string());
                    source.name = format!("piece {}", part + 1);
                    source.linecolor = session_rust::Color::black();
                    let piece = scene.insert_path(
                        crate::chain::PreparedChain::polyline(Rc::new(source))?,
                    )?;
                    scene.place_following(piece, model.clone())?;
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(
                        format!("piece {}", part + 1),
                        piece,
                    ));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(
                        crate::feature_plan::label(feature),
                        outlines,
                    ));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() {
            groups.push(FeatureTree::Group("attributes".into(), own));
        }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() {
                hosted.push(FeatureTree::Group(relation.label, children));
            }
        }
        if !hosted.is_empty() {
            groups.push(FeatureTree::Group(
                "interactions".into(),
                hosted,
            ));
        }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

### 2. `src/lib.rs`

Register the new owner without changing the browser entry point.

**Locate this existing block:**

```rust
pub mod feature_plan;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod feature_plan;

pub mod feature_display;

#[cfg(test)]
```

## <a id="checkpoint-96n-connect-imported-interaction-hierarchies"></a>6. Connect imported Elements to their feature hierarchy

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_document.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Import a document atomically, classify its editable child Elements, then attach feature projections from its original graph and retained Element payloads.

### 1. `src/scene_document.rs`

Connect projections only after original tree placement and source retention are ready.

**Locate this existing block:**

```rust
    .header = Some(source.origin);
    let colours = candidate.forest.colours(key);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    .header = Some(source.origin);
    let elements = owners
        .iter()
        .filter_map(|(guid, id, _)| {
            matches!(
                source.document.lookup.get(guid),
                Some(session_rust::Geometry::Element(_))
            )
            .then_some(*id)
        })
        .collect::<std::collections::HashSet<_>>();
    candidate
        .forest
        .classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) =
            source.document.lookup.get(guid)
        {
            crate::feature_display::append(
                &mut candidate,
                *id,
                &source.document,
                element,
            )
            .map_err(str::to_owned)?;
        }
    }
    let colours = candidate.forest.colours(key);
```

## <a id="checkpoint-96o-reuse-projections-for-created-elements"></a>7. Use the same projection path for created Elements

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/element_adopt.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Created Elements and imported Elements now share feature display ownership. Visibility commands stop at independent child Elements and collect each feature piece once.

### 1. `src/element_adopt.rs`

Replace serialized display copies with the shared feature projection and preserve independent child owners.

**Locate this existing block:**

```rust
    let mut ids = Vec::new();
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        let mut attributes = Vec::new();
        let mut interactions = Vec::new();
        if let Some(plane) = item.plane {
            attributes.push(crate::plane_original::insert(
                scene,
                Rc::new(plane),
            )?);
        }
        // Own axes and sections are attributes; contact, joint, cut and drill geometry comes from interactions.
        for feature in &item.source.features {
            let interaction = matches!(
                feature.feature_type.as_str(),
                "contact" | "joint" | "cut" | "drill"
            );
            for outline in &feature.outlines {
                let mut source = outline.clone();
                source.set_guid(uuid::Uuid::new_v4().to_string());
                source.name = format!("{} {}", feature.feature_type, feature.name);
                let piece = scene.insert_path(
                    crate::chain::PreparedChain::polyline(Rc::new(source))?,
                )?;
                if interaction {
                    interactions.push(piece);
                } else {
                    attributes.push(piece);
                }
            }
        }
        if !attributes.is_empty() {
            Rc::make_mut(&mut scene.view).hide(&attributes, true);
            crate::layer_edit::attach_attributes(scene, id, attributes, "attributes")?;
        }
        if !interactions.is_empty() {
            crate::layer_edit::attach_attributes(
                scene,
                id,
                interactions,
                "interactions",
            )?;
        }
    }
    for id in scene.with_descendants(consumed) {
        scene.remove(id);
    }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    scene
        .forest
        .documents
        .iter()
        .flat_map(|doc| {
            doc.nodes
                .iter()
                .enumerate()
                .filter(move |(_, node)| node.source.name == name && node.readonly)
                .flat_map(|(index, node)| {
                    doc.nodes[index + 1..node.end]
                        .iter()
                        .filter_map(|node| node.object)
                })
        })
        .collect()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut ids = Vec::new();
    let session = session_rust::Session::new("Created");
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        if let Some(plane) = item.plane {
            let plane = crate::plane_original::insert(scene, Rc::new(plane))?;
            Rc::make_mut(&mut scene.view).hide(&[plane], true);
            crate::layer_edit::attach_attributes(scene, id, vec![plane], "attributes")?;
        }
        crate::feature_display::append(scene, id, &session, &item.source)?;
    }
    for id in scene.with_descendants(consumed) {
        scene.remove(id);
    }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    let mut ids = Vec::new();
    for doc in &scene.forest.documents {
        for (index, group) in doc.nodes.iter().enumerate() {
            if group.source.name != name || !group.readonly {
                continue;
            }
            let mut child = index + 1;
            while child < group.end {
                let node = &doc.nodes[child];
                if node.object.is_some() && !node.readonly {
                    child = node.end;
                    continue;
                }
                if let Some(id) = node.object.filter(|id| scene.contains(*id)) {
                    ids.push(id);
                }
                child += 1;
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
```

## <a id="checkpoint-96p-build-an-imported-interaction-specimen"></a>8. Build an interaction specimen

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/interaction_example.rs,…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Build a small original document with a host, an editable nested child, and a connector. Its contact, unnamed joint and orphan feature make the hierarchy inspectable without a large floor model.

### 1. `src/interaction_example.rs`

Author the original geometry, graph and feature associations used by the exercise.

Create the file and type:

```rust
use prost::Message;
use session_rust::element::ElementFeature;
use session_rust::{Element, Mesh, Point, Polyline, Session, TreeNode, Xform};
use session_rust::{Interaction, InteractionUnknown};

pub fn session() -> Session {
    let line = Polyline::new(vec![
        Point::new(0., 0., 0.),
        Point::new(100., 0., 0.),
    ]);
    let mut session = Session::new("interaction exercise");
    let mut host = Element::new("host");
    host.set_geometry(Mesh::create_box(100., 100., 100.));
    host.add_feature(ElementFeature::new(
        "axis",
        -1,
        vec![line.clone()],
        "axis",
    ));
    let contact = InteractionUnknown::new(
        "InteractionContactFace",
        &[0, 255, 7],
        "seam",
    );
    let mut touch = ElementFeature::new(
        "contact",
        2,
        vec![line.clone(), line.clone()],
        "side_side",
    );
    touch.set_guid(contact.guid().to_owned());
    host.add_feature(touch);
    let mut source = Element::new("connector");
    source.set_geometry(Mesh::create_box(20., 20., 20.));
    let mut drill = ElementFeature::new("drill", -1, vec![line.clone()], "d10");
    drill.set_guid(format!("{}-drill-0", source.guid()));
    host.add_feature(drill);
    host.add_feature(ElementFeature::new(
        "cut",
        5,
        vec![line.clone()],
        "unclaimed",
    ));
    let host = session.add_element(host, None);
    session.add(&TreeNode::new("attributes"), Some(&host));
    let attributes = host
        .borrow()
        .children()
        .last()
        .unwrap()
        .clone();
    let mut child = Element::new("child");
    child.set_geometry(Mesh::create_box(15., 15., 15.));
    child.add_feature(ElementFeature::new(
        "axis",
        -1,
        vec![line],
        "axis",
    ));
    let child = session.add_element(child, Some(&attributes));
    let source = session.add_element(source, None);
    let host = host.borrow().name.clone();
    let child = child.borrow().name.clone();
    let source = source.borrow().name.clone();
    session.xforms.insert(
        host.clone(),
        Xform::translation(10., 20., 30.),
    );
    session
        .xforms
        .insert(child, Xform::translation(2., 3., 4.));
    session.xforms.insert(
        source.clone(),
        Xform::translation(130., 20., 30.),
    );
    session
        .graph
        .add_edge(&source, &host, "connection");
    let edge = session.graph.edges[&source][&host]
        .guid()
        .to_owned();
    session.interactions.insert(
        edge,
        vec![
            Box::new(contact),
            Box::new(InteractionUnknown::new(
                "InteractionFeatureSolid",
                &[8, 9],
                "",
            )),
        ],
    );
    session
}

pub fn bytes() -> Vec<u8> {
    session().to_proto().encode_to_vec()
}
```

### 2. `src/lib.rs`

Expose the specimen to native examples and tests.

**Locate this existing block:**

```rust
pub mod feature_display;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod feature_display;

pub mod interaction_example;

#[cfg(test)]
```

### 3. `examples/sample.rs`

Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.

**Locate this existing block:**

```rust
        "sample.pb",
        viewer_journey::specimen::precise_bytes(),
    )
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "sample.pb",
        viewer_journey::interaction_example::bytes(),
    )
```

## <a id="checkpoint-96q-prove-imported-feature-ownership-and-roundtrips"></a>9. Prove imported feature ownership and round trips

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/interaction_bridge_test…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-preserving archive</text>
</svg>
</div>

**What this code does:** Verify that feature projections follow their owner once, preserve original payloads, survive Delete/Undo, and rebuild after Save/Open without serializing display copies. Reject invalid outlines without changing the current document.

### 1. `src/interaction_bridge_tests.rs`

Run supplied regression fixtures against original source payloads and the connected import path.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::{ObjectId, Scene},
};
use prost::Message;
use session_rust::{Element, Geometry, Xform, proto};
use std::rc::Rc;

fn owner(scene: &Scene, name: &str) -> ObjectId {
    scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == name)
        .unwrap()
        .id
}
fn original(scene: &Scene, id: ObjectId) -> Rc<Element> {
    match scene
        .objects()
        .iter()
        .find(|row| row.id == id)
        .unwrap()
        .original
        .as_ref()
        .unwrap()
    {
        Geometry::Element(source) => source.clone(),
        _ => panic!("Element expected"),
    }
}
fn same_element(mut a: proto::Element, mut b: proto::Element) {
    assert_eq!(
        proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(b.geometry_data.as_slice()).unwrap()
    );
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(a, b);
}
fn imported() -> Scene {
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &crate::interaction_example::bytes(),
        "fixture",
        &Xform::translation(1_000_000_000.125, 0., 0.),
        0.,
    )
    .unwrap();
    scene
}

#[test]
fn hosted_features_keep_full_type_source_name_edge_order_and_orphans() {
    let session = crate::interaction_example::session();
    let host = session
        .objects
        .elements
        .iter()
        .find(|e| e.name == "host")
        .unwrap();
    assert_eq!(crate::feature_plan::own(host), [0]);
    let before = host.to_proto();
    let groups = crate::feature_plan::hosted(&session, host);
    same_element(host.to_proto(), before);
    assert_eq!(
        groups
            .iter()
            .map(|g| (g.label.as_str(), g.features.clone()))
            .collect::<Vec<_>>(),
        [
            ("InteractionContactFace seam", vec![1]),
            ("InteractionFeatureSolid connector", vec![2]),
            ("cut unclaimed · face 5", vec![3]),
        ]
    );
    assert_eq!(
        crate::feature_plan::label(&host.features[1]),
        "contact side_side · face 2"
    );
}

#[test]
fn imported_children_stay_editable_and_features_follow_the_host_once() {
    let mut e = Editor::default();
    e.scene = imported();
    let host = owner(&e.scene, "host");
    let child = owner(&e.scene, "child");
    assert!(!e.scene.forest.readonly(child));
    assert_eq!(e.scene.pick_id(child.value()), Some(child));
    assert_eq!(e.scene.paths().len(), 6);
    let attributes = crate::element_adopt::attribute_ids(&e.scene);
    let interactions = crate::element_adopt::interaction_ids(&e.scene);
    assert_eq!(attributes.len(), 2);
    assert_eq!(interactions.len(), 4);
    assert!(!attributes.contains(&child));
    assert!(
        attributes
            .iter()
            .all(|id| !e.scene.visible(*id))
    );
    assert!(
        interactions
            .iter()
            .all(|id| e.scene.visible(*id))
    );
    assert!(
        e.scene.paths().iter().all(
            |row| e.scene.forest.readonly(row.id) && e.scene.forest.derived(row.id)
        )
    );
    let labels = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .map(|row| row.label)
        .collect::<Vec<_>>();
    assert!(labels.contains(&"InteractionContactFace seam".into()));
    assert!(labels.contains(&"InteractionFeatureSolid connector".into()));
    assert_eq!(
        e.scene.placement(host).unwrap().m[12],
        1_000_000_010.125
    );
    assert_eq!(
        e.scene.placement(child).unwrap().m[12],
        1_000_000_012.125
    );
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    e.apply(Action::ElementInteractions(Some(false)))
        .unwrap();
    assert!(
        attributes
            .iter()
            .all(|id| e.scene.visible(*id))
    );
    assert!(
        interactions
            .iter()
            .all(|id| !e.scene.visible(*id))
    );
    assert!(e.scene.visible(child));
    let ids = e.scene.with_descendants(&[host]);
    let before = ids
        .iter()
        .map(|id| e.scene.placement(*id).unwrap())
        .collect::<Vec<_>>();
    let source = original(&e.scene, host);
    let original_bytes = source.to_proto();
    e.select_objects([host, child], false);
    e.apply(Action::Translate([3., 4., 5.]))
        .unwrap();
    for (id, old) in ids.iter().zip(before) {
        let now = e.scene.placement(*id).unwrap();
        assert_eq!(now.m[12], old.m[12] + 3.);
        assert_eq!(now.m[13], old.m[13] + 4.);
        assert_eq!(now.m[14], old.m[14] + 5.);
    }
    same_element(source.to_proto(), original_bytes);
    e.apply(Action::Undo).unwrap();
    assert_eq!(
        e.scene.placement(child).unwrap().m[12],
        1_000_000_012.125
    );
    e.select_objects([host], false);
    e.apply(Action::Delete).unwrap();
    assert!(ids.iter().all(|id| !e.scene.contains(*id)));
    e.apply(Action::Undo).unwrap();
    assert!(ids.iter().all(|id| e.scene.contains(*id)));
    assert!(Rc::ptr_eq(
        &source,
        &original(&e.scene, host)
    ));
}

#[test]
fn repeated_archives_keep_original_features_and_do_not_accumulate_display_paths() {
    let session = crate::interaction_example::session();
    let message = session.to_proto();
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &message.encode_to_vec(),
        "original",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    for _ in 0..3 {
        let bytes = crate::scene_archive::save(&scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        let saved = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        let objects = saved.objects.as_ref().unwrap();
        assert_eq!(objects.elements.len(), 3);
        assert!(objects.polylines.is_empty());
        assert_eq!(saved.interactions, message.interactions);
        for element in &objects.elements {
            let original = message
                .objects
                .as_ref()
                .unwrap()
                .elements
                .iter()
                .find(|e| e.guid == element.guid)
                .unwrap();
            same_element(element.clone(), original.clone());
        }
        crate::scene_document::open(&mut scene, &bytes).unwrap();
        assert_eq!(scene.paths().len(), 6);
        assert_eq!(
            crate::element_adopt::attribute_ids(&scene).len(),
            2
        );
        assert_eq!(
            crate::element_adopt::interaction_ids(&scene).len(),
            4
        );
        assert!(!scene.forest.readonly(owner(&scene, "child")));
    }
}

#[test]
fn malformed_feature_projection_does_not_replace_the_open_scene() {
    let mut message = crate::interaction_example::session().to_proto();
    message
        .objects
        .as_mut()
        .unwrap()
        .elements
        .iter_mut()
        .find(|e| e.name == "host")
        .unwrap()
        .features[1]
        .outlines[0]
        .coords[0] = f64::NAN;
    let mut scene = imported();
    let before = scene.all_ids();
    let paths = scene.paths().len();
    assert!(
        crate::scene_document::import(
            &mut scene,
            &message.encode_to_vec(),
            "invalid",
            &Xform::identity(),
            0.
        )
        .is_err()
    );
    assert_eq!(scene.all_ids(), before);
    assert_eq!(scene.paths().len(), paths);
}
```

### 2. `src/lib.rs`

Include the ownership and archive proofs in the native suite.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod interaction_bridge_tests;

#[cfg(test)]
mod ambient_shader_contract;
```

## <a id="checkpoint-96r-preserve-exact-command-completions"></a>10. Prove exact commands across the expanded vocabulary

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 10</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/command_vocabulary.rs,…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">typed characters</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact match first</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/inspect.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">egui submission</text>
</svg>
</div>

**What this code does:** Share the existing dictionary wrapper with the real widget regression and browser typing proof.

### 1. `src/command_vocabulary.rs`

Move the already-typed private Commands wrapper from panel.rs into this shared module; retain its existing methods. Import the existing dock trait and expose the shared wrapper and its input list to the panel.

**Locate this existing block:**

```rust
    choices
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    choices
}

use crate::command_dock;

pub(crate) struct Commands(pub(crate) Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}
```

### 2. `src/panel.rs`

Use the exact-first vocabulary in the actual browser panel.

**Locate this existing block:**

```rust
use crate::renderer::Renderer;

struct Commands(Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub struct Panel {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::renderer::Renderer;

use crate::command_vocabulary::Commands;

pub struct Panel {
```

### 3. `src/command_vocabulary.rs`

Feed character-by-character input and Enter through the actual egui command field.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod completion_tests {
    use super::*;
    use crate::command_dock::{CommandLine, theme};

    #[test]
    fn typed_exact_commands_survive_inline_completion_and_enter() {
        for line in [
            "View Isometric",
            "View Isometric Back",
            "Line",
            "Polyline",
            "Select Object",
        ] {
            let context = egui::Context::default();
            context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
            context.set_fonts(theme::fonts([
                include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
                include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
                include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
            ]));
            let mut model = CommandLine {
                focus_command: true,
                command_open: true,
                ..Default::default()
            };
            let commands = Commands(crate::commands::names());
            let frame = |model: &mut CommandLine, events| {
                let mut submitted = None;
                let _ = context.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(900., 760.),
                        )),
                        focused: true,
                        events,
                        ..Default::default()
                    },
                    |root| {
                        command_dock::draw(
                            root,
                            model,
                            &mut None,
                            &mut submitted,
                            &commands,
                            true,
                        );
                    },
                );
                submitted
            };
            frame(&mut model, vec![]);
            for character in line.chars() {
                assert!(
                    frame(
                        &mut model,
                        vec![egui::Event::Text(character.to_string())]
                    )
                    .is_none()
                );
            }
            assert_eq!(model.command, line);
            assert!(!model.inline_suffix);
            let submitted = frame(
                &mut model,
                vec![egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Default::default(),
                }],
            );
            assert_eq!(submitted.as_deref(), Some(line));
            assert!(model.command.is_empty());
        }
    }
}
```

## Build & Run

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 19.2
npm --prefix ../session_tests run course -- save 19.2
```

Continue with [19.3 · Original owners across selection, copying and display](../../consolidation/96s-resolve-feature-rows-to-original-element-owners.md).
