# 9.2 · Shared CAD boundaries across incident faces

**Goal:** Prove straight and circular shared boundaries. · **Steps:** 3 · **Source:** [app/walk/brep.rs](../../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) (`src/app/walk/brep_shapes.rs,src/app/walk/brep_orient.rs,src/shaders/normals.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 9.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cad_edges.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cad_boundary.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">3 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_shapes.…</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/walk/brep_orient.…</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Connect CAD edge chains</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Trimmed B-Rep (u,v)</text>
  <rect x="620" y="52" width="160" height="88" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="700" cy="96" r="24" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Hole</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Shared CAD boundaries acros…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Crease Split Normals</text>
  <polyline points="894,136 974,90 1054,136" fill="none" stroke="#111111" stroke-width="2.5"/>
  <circle cx="974" cy="90" r="4.5" fill="#111111"/>
  <line x1="974" y1="90" x2="942" y2="52" stroke="#2563eb" stroke-width="2.25" marker-end="url(#arrB)"/>
  <line x1="974" y1="90" x2="1006" y2="52" stroke="#dc2626" stroke-width="2.25" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="924" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">N1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1012" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">N2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove straight and circul…</text>
</svg>
</div>

## <a id="checkpoint-56-chains"></a>1. Connect CAD edge chains

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cad_edges.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original edge use</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face-mesh samples</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/walk/brep_s…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ordered CAD boundary chain</text>
</svg>
</div>

**What this code does:** Reuse constrained-label and isoparametric lookup for each original edge.

### 1. `src/cad_edges.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    None
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    None
}

pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>> {
    let mut out = Vec::with_capacity(b.m_edges.len());

    for (ei, e) in b.m_edges.iter().enumerate() {
        if e.degenerated {
            out.push(None);
            continue;
        }

        let uses = b.edge_faces(ei);
        let mut found: Option<EdgeChain> = None;

        for (k, u) in uses.iter().enumerate() {
            let eu = EdgeUse {
                edge: ei,
                face: u.index as usize,
                orientation: u.orientation,
            };
            let keys = match constrained_chain(&fms[eu.face], ei) {
                Some(keys) => keys,
                None => match iso_chain(b, &fms[eu.face], &eu) {
                    Some(keys) => keys,
                    None => continue,
                },
            };
            let mut other = None;

            for (j, candidate) in uses.iter().enumerate() {
                if j != k && candidate.index as usize != eu.face {
                    other = Some(candidate.index as usize);
                    break;
                }
            }

            found = Some(EdgeChain {
                edge: ei,
                face: eu.face,
                keys,
                other,
            });
            break;
        }

        out.push(found);
    }

    out
}
```

## <a id="checkpoint-56a-shared"></a>2. Mesh incident faces with shared boundary constraints

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cad_boundary.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated BRep</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face meshes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/walk/brep_s…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source-owned edge chains</text>
</svg>
</div>

**What this code does:** Mesh incident faces with shared boundary constraints.

### 1. `src/cad_boundary.rs`

Validate references and retain one source-owned preparation of all incident face meshes.

Create the file and type:

```rust
use session_rust::{BRep, Mesh};
use std::rc::Rc;

pub const QUALITY: (f64, f64) = (5., 0.001);
pub struct Prepared {
    pub source: Rc<BRep>,
    pub faces: Vec<Mesh>,
    pub chains: Vec<Option<crate::cad_edges::EdgeChain>>,
}
impl Prepared {
    pub fn new(source: Rc<BRep>) -> Result<Self, &'static str> {
        if !source.is_valid() {
            return Err("BRep topology contains an invalid reference");
        }
        if source.m_faces.len() > 65_536 || source.m_edges.len() > 262_144 {
            return Err("BRep topology exceeds the display budget");
        }
        for surface in &source.m_surfaces {
            crate::surface::validate(surface)?;
        }
        for curve in source
            .m_curves_3d
            .iter()
            .chain(&source.m_curves_2d)
        {
            crate::curve::validate(curve)?;
        }
        let _ = source.guid();
        let faces = source.face_meshes_q(Some(QUALITY));
        if faces.len() != source.m_faces.len()
            || faces.iter().any(|face| face.face.is_empty())
        {
            return Err("BRep face could not be tessellated");
        }
        let chains = crate::cad_edges::edge_chains(&source, &faces);
        for (index, (edge, chain)) in source
            .m_edges
            .iter()
            .zip(&chains)
            .enumerate()
        {
            if !edge.degenerated
                && !source.edge_faces(index).is_empty()
                && chain.is_none()
            {
                return Err("A CAD edge has no coherent face boundary");
            }
        }
        Ok(Self {
            source,
            faces,
            chains,
        })
    }
    pub fn points(&self, edge: usize) -> Option<Vec<[f64; 3]>> {
        let chain = self.chains.get(edge)?.as_ref()?;
        let face = self.faces.get(chain.face)?;
        chain
            .keys
            .iter()
            .map(|key| face.vertex.get(key).map(|v| [v.x, v.y, v.z]))
            .collect()
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod surface_tests;

pub mod cad_edges;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod surface_tests;

pub mod cad_edges;

pub mod cad_boundary;
```

## <a id="checkpoint-56b-boundaries"></a>3. Prove straight and circular shared boundaries

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cad_boundary_tests.rs,…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared boundary chain</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/walk/brep_s…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">released owner</text>
</svg>
</div>

**What this code does:** Prove straight and circular shared boundaries.

### 1. `src/cad_boundary_tests.rs`

Copy independent incident-face position, source ownership and malformed-input checks.

Create the file and type:

```rust
use crate::cad_boundary::Prepared;
use session_rust::BRep;
use std::rc::Rc;

#[test]
fn shared_box_edges_follow_original_face_samples_and_preserve_brep_source() {
    let source = Rc::new(BRep::create_box(4., 3., 2.));
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    assert!(Rc::ptr_eq(&prepared.source, &source));
    assert_eq!(prepared.faces.len(), 6);
    assert_eq!(prepared.chains.len(), 12);
    for (edge, chain) in prepared.chains.iter().enumerate() {
        let chain = chain.as_ref().unwrap();
        assert_eq!(chain.edge, edge);
        assert!(chain.other.is_some());
        let points = prepared.points(edge).unwrap();
        assert!(points.len() >= 2);
        let other = &prepared.faces[chain.other.unwrap()];
        for p in points {
            assert!(
                other.vertex.values().any(|v| [v.x, v.y, v.z]
                    .iter()
                    .zip(p)
                    .all(|(a, b)| (a - b).abs() < 1e-10)),
                "Shared edge {edge} follows both incident faces"
            );
        }
    }
    assert_eq!(source.to_proto(), before);
    let weak = Rc::downgrade(&source);
    drop(source);
    assert!(weak.upgrade().is_some());
    drop(prepared);
    assert!(weak.upgrade().is_none());
}

#[test]
fn circular_boundary_samples_are_shared_by_cap_and_wall() {
    let source = Rc::new(BRep::create_cylinder(2., 3.));
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    for edge in [0, 1] {
        let chain = prepared.chains[edge].as_ref().unwrap();
        let other = &prepared.faces[chain.other.unwrap()];
        let points = prepared.points(edge).unwrap();
        assert!(points.len() > 16);
        for p in points {
            assert!(
                other.vertex.values().any(|v| [v.x, v.y, v.z]
                    .iter()
                    .zip(p)
                    .all(|(a, b)| (a - b).abs() < 1e-9)),
                "Circle {edge} follows both incident faces"
            );
        }
    }
    assert_eq!(source.to_proto(), before);
}

#[test]
fn invalid_topology_and_control_storage_fail_before_meshing() {
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_faces[0].surface_index = -1;
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_surfaces[0].m_cv.pop();
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_curves_3d[0].m_cv.pop();
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_curves_2d[0].m_nurbsknot.reverse();
    source.m_curves_2d[0].m_nurbsknot[0] = f64::NAN;
    assert!(Prepared::new(Rc::new(source)).is_err());
}

fn adjacent() -> BRep {
    use session_rust::BRep;
    use session_rust::NurbsCurve;
    use session_rust::NurbsSurface;
    use session_rust::Point;
    use session_rust::brep::BRepOrientation;
    use session_rust::brep::BRepRef;

    let mut b = BRep::new();
    let mut surfaces = Vec::new();

    for face in 0..2 {
        let mut points = Vec::new();

        for i in 0..3 {
            for j in 0..2 {
                let z = if i != 1 {
                    0.0
                } else if j == 0 || face == 0 {
                    0.5
                } else {
                    4.0
                };
                points.push(Point::new(
                    i as f64 * 0.5,
                    j as f64
                        * if face == 0 {
                            1.0
                        } else {
                            -1.0
                        },
                    z,
                ));
            }
        }

        let surface = NurbsSurface::create(false, false, 2, 1, 3, 2, &points).unwrap();
        let si = b.add_surface(&surface);
        let corners = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let mut vertices = Vec::new();

        for corner in &corners {
            vertices.push(
                b.add_vertex(
                    &surface
                        .point_at(corner[0], corner[1])
                        .unwrap(),
                    0.0,
                ),
            );
        }

        let mut edges = Vec::new();

        for side in 0..4 {
            let a = corners[side];
            let z = corners[(side + 1) % 4];
            let mut edge = 0;

            if side != 0 || face != 1 {
                let dir = if a[0] != z[0] {
                    0
                } else {
                    1
                };
                let mut curve = surface.iso_curve(dir, a[1 - dir]).unwrap();

                if a[dir] > z[dir] {
                    curve.reverse();
                }

                let ci = b.add_curve_3d(&curve);
                edge = b.add_edge(
                    ci as i32,
                    vertices[side] as i32,
                    vertices[(side + 1) % 4] as i32,
                );
            }

            let pc = NurbsCurve::create(
                false,
                1,
                &[
                    Point::new(a[0], a[1], 0.0),
                    Point::new(z[0], z[1], 0.0),
                ],
            );
            let ci = b.add_curve_2d(&pc);
            b.add_pcurve(edge, si, ci as i32, -1);
            edges.push(BRepRef::new(
                edge as i32,
                BRepOrientation::Forward,
            ));
        }

        let wi = b.add_wire(&edges);
        b.add_face(
            si as i32,
            &[BRepRef::new(
                wi as i32,
                BRepOrientation::Forward,
            )],
            1e-8,
        );
        surfaces.push(surface);
    }

    b
}

#[test]
fn unequal_independent_grids_become_a_coherent_curved_shared_boundary() {
    use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
    let source = Rc::new(adjacent());
    let _ = source.guid();
    let before = source.to_proto();
    let points = |mesh: &session_rust::Mesh| {
        let mut p = mesh
            .vertex
            .values()
            .filter(|v| v.attributes.get("v") == Some(&0.))
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<_>>();
        p.sort_by(|a, b| a[0].total_cmp(&b[0]));
        p.dedup();
        p
    };
    let a = RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[0], 0, 0, 20., 0.005);
    let b = RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[1], 0, 0, 20., 0.005);
    assert_ne!(points(&a).len(), points(&b).len());
    let prepared = Prepared::new(source.clone()).unwrap();
    let a = points(&prepared.faces[0]);
    let b = points(&prepared.faces[1]);
    assert_eq!(a, b);
    assert!(a.len() > 7);
    assert_eq!(source.to_proto(), before);
    let normal = |surface: &session_rust::NurbsSurface| surface.normal_at(0.5, 0.);
    assert_ne!(
        normal(&source.m_surfaces[0]).to_f32(),
        normal(&source.m_surfaces[1]).to_f32()
    );
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod cad_edges;

pub mod cad_boundary;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cad_edges;

pub mod cad_boundary;

#[cfg(test)]
mod cad_boundary_tests;
```

## Build & Run

![Recorded visual output for Lesson 9.2 (56b-boundaries)](../../../screenshots/journey/56b-boundaries.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.2
npm --prefix ../session_tests run course -- save 9.2
```

Continue with [9.3 · Trimmed curved faces with real open holes](../../consolidation/57-parameters.md).
