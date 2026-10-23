# 15.4 · Two-direction networks and corner patches

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Checkpoints:** `80-build-a-surface-through-two-directions-of-original-curves` → `80za-read-original-projection-planes` (6 steps) · **Production `session_viewer/src` target:** [app/deform.rs](../../../../src/app/deform.rs) · [state/edit.rs](../../../../src/state/edit.rs) · [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs) · [app/command/tool/surfacing.rs](../../../../src/app/command/tool/surfacing.rs) · [app/walk/mesh.rs](../../../../src/app/walk/mesh.rs) · [app/walk/brep.rs](../../../../src/app/walk/brep.rs) (`session_viewer/src/lib.rs`) · **Goal:** Reuse absolute coordinate parsing for named construction planes and three-point input.

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 15.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/network_geometry.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/patch_geometry.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">80-build-a-surface-through-two-directions-of-original-curves -&gt; 80za-read-original-projection-planes</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Build a surface through o…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Screen-Space Ribbon Quad</text>
  <polygon points="606,112 764,56 782,94 624,146" fill="#bfdbfe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="615" y1="129" x2="773" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="615" cy="129" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <circle cx="773" cy="75" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <line x1="694" y1="102" x2="682" y2="72" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <text x="664" y="70" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Two-direction networks and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGBA Ink &amp; SDF Edge AA</text>
  <line x1="884" y1="136" x2="1064" y2="136" stroke="#111111" stroke-width="1.5"/>
  <path d="M 896 136 L 928 68 L 1020 68 L 1052 136" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <circle cx="916" cy="54" r="6" fill="#ef4444" stroke="#111111" stroke-width="1.25"/>
  <circle cx="936" cy="54" r="6" fill="#22c55e" stroke="#111111" stroke-width="1.25"/>
  <circle cx="956" cy="54" r="6" fill="#3b82f6" stroke="#111111" stroke-width="1.25"/>
  <text x="1020" y="58" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">RGBA + SDF</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Read original projection…</text>
</svg>
</div>

## <a id="checkpoint-80-build-a-surface-through-two-directions-of-original-curves"></a>Step 1 of 6 · Build a surface through original curve networks (`80-build-a-surface-through-two-directions-of-original-curves`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/6: 80-build-a-surfac…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/network_geometry.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original curve crossings</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sampled lofts</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">fitted surface</text>
</svg>
</div>

Connect the complete network helpers and retain the four-boundary Coons path. Register the original recipe and validate the final fitted surface.

> **Check:** *Why keep the four-boundary path?* — The existing kernel edge constructor directly describes that supported case. Larger grids use the Gordon construction just written.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

After that block, type:

```rust
/// The surface through a grid of curves: Coons for four boundaries, else a Gordon surface.
pub fn network(input: &[NurbsCurve]) -> Result<NurbsSurface, String> {
    let two = "Network curves must cross in two directions";

    if input.len() > 32 {
        return Err("A network takes at most 32 curves".into());
    }

    if input.len() < 4 {
        return Err(two.into());
    }

    let mut cross = crossings(input);
    let (mut us, mut vs) = directions(&cross)?;
    let curves = oriented(input, &mut us, &mut vs, &mut cross)?;

    if us.len() == 2 && vs.len() == 2 {
        let edge = Primitives::create_edge(
            &curves[us[0]],
            &curves[vs[1]],
            &curves[us[1]],
            &curves[vs[0]],
        );

        if edge.is_valid() {
            return Ok(edge);
        }
    }

    fitted(sampled(&curves, &us, &vs, &cross))
}

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Network",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[Step::Curves {
        prompt: "select the network curves, both directions",
        min: 2, // fewer than four is refused with the reason
        max: 32,
    }],
    axis: crate::gather_input::no_axis,
    build,
};

/// The surface through the picked network.
fn build(input: &Input) -> Result<Made, String> {
    let network_curves = curves(&input.curves[0]);
    let surface = network(&network_curves)?;
    Ok(Made {
        geometries: vec![checked(surface, "network")?],
        message: format!(
            "Built a NURBS surface through {}",
            count(network_curves.len(), "curve")
        ),
    })
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/network_geometry.rs`

Supply only the unchanged original validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/80-build-a-surface-through-two-directions-of-original-curves-2.rs" skip=11853
```

**Result after `80-build-a-surface-through-two-directions-of-original-curves`:** The complete original network-surface recipe compiles; its editing/input connection follows later.

## <a id="checkpoint-80a-construct-the-patch-through-four-ordered-corners"></a>Step 2 of 6 · Construct the patch through four ordered corners (`80a-construct-the-patch-through-four-ordered-corners`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/6: 80a-construct-the…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/patch_geometry.rs, src/…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">four ordered world corners</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">area check</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">bilinear original surface</text>
</svg>
</div>

Describe four ordered corner inputs with the existing gather recipe. Reject a degenerate area, build the exact bilinear kernel patch, and return the validated source surface as one result.

> **Check:** *What do the corner order and u v directions mean?* — u runs from corner1 to corner2; v runs from corner1 to corner4. The patch reproduces all four corners and their mean at its center.

### 1. `src/patch_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/patch_geometry.rs`

Supply only the unchanged original geometry validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry-fixtures.rs" skip=1504
```

### 3. `src/lib.rs`

Register the current surface component.

After that block, type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-02.rs" skip=27
```

## <a id="checkpoint-80b-guide-corner-questions-without-retaining-unrelated-profiles"></a>Step 3 of 6 · Guide corner questions without retaining unrelated profiles (`80b-guide-corner-questions-without-retaining-unrelated-profiles`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/6: 80b-guide-corner-…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gather_draft.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">accepted corner points an…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary guide</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unfinished recipe</text>
</svg>
</div>

Keep accepted world points and the cursor as a temporary guide. Point-only recipes retain no unrelated selected curve input copies.

> **Check:** *Does hovering corner4 add an original surface?* — No. The guide belongs to the pending owner. Only a checked fourth answer completes a real geometry transaction.

### 1. `src/gather_draft.rs`

Guide current point questions and retain original profile copies only for curve-gathering recipes.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{cplane::CPlane,gather_input::{Answer,Input,Made,Picked,Progress,Recipe,Step},scene::{Scene,ObjectId}};
use session_rust::{Geometry,Point,Vector};
#[derive(Clone)]
pub struct Draft {pub recipe:&'static Recipe,pub choice:usize,pub progress:Progress,pub answers:Vec<Answer>,pub curves:Vec<(ObjectId,Picked)>,pub before:Vec<ObjectId>,pub frame:CPlane,pub typed:String,pub originals:Vec<(ObjectId,Geometry,session_rust::Xform)>}
impl Draft {
    pub fn new(recipe:&'static Recipe,text:&str,scene:&Scene,before:Vec<ObjectId>,frame:CPlane)->Result<Self,String>{
        let words=text.split_whitespace().collect::<Vec<_>>();let(choice,answers)=crate::gather_input::read(recipe,&words)?;for answer in &answers{match answer{Answer::Point(p)=>{crate::tool_coords::checked([p[0],p[1],p[2]]).map_err(str::to_owned)?;},Answer::Vector(v)if(0..3).any(|i|!v[i].is_finite()||v[i].abs()>1e12)=>return Err("Distance vector exceeds the accepted range".into()),_=>{}}}let mut result=Self{recipe,choice,progress:Progress::new(recipe.steps,answers.len()),answers,curves:Vec::new(),before:before.clone(),frame,typed:String::new(),originals:Vec::new()};
        if recipe.steps.iter().any(Step::curves){for id in before{if scene.pick_id(id.value())==Some(id){if let Some(curve)=Self::world(scene,id){result.originals.push((id,Self::source(scene,id).unwrap(),scene.placement(id).unwrap()));result.curves.push((id,curve));}}}let rows=result.curves.iter().map(|(id,_)|id.value()).collect::<Vec<_>>();result.progress.preselect(recipe.steps,&rows);}Ok(result)
    }
    pub fn current(&self)->Option<&'static Step>{self.recipe.steps.get(self.progress.step)}
    pub fn picks(&self)->bool{self.current().is_some_and(Step::curves)}
    pub fn world(scene:&Scene,id:ObjectId)->Option<Picked>{
        let geometry=Self::source(scene,id)?;
        crate::gather_input::picked(&geometry,&scene.placement(id)?)
    }
    pub fn source(scene:&Scene,id:ObjectId)->Option<Geometry>{
        if let Some(row)=scene.lines().iter().find(|row|row.id==id){return Some(Geometry::Line(row.prepared.source.clone()))}
        let row=scene.paths().iter().find(|row|row.id==id)?;Some(match &row.prepared.source{crate::chain::Source::Line(source)=>Geometry::Line(source.clone()),crate::chain::Source::Polyline(source)=>Geometry::Polyline(source.clone()),crate::chain::Source::Curve(source,_)=>Geometry::NurbsCurve(source.clone())})
    }
    pub fn valid(&self,scene:&Scene)->Result<(),String>{
        for(id,source,place)in &self.originals{if !self.progress.picked().contains(&id.value()){continue}let current=Self::source(scene,*id).ok_or("Picked curve was removed")?;let same=match(source,current){(Geometry::Line(a),Geometry::Line(b))=>std::rc::Rc::ptr_eq(a,&b),(Geometry::Polyline(a),Geometry::Polyline(b))=>std::rc::Rc::ptr_eq(a,&b),(Geometry::NurbsCurve(a),Geometry::NurbsCurve(b))=>std::rc::Rc::ptr_eq(a,&b),_=>false};if !same||scene.placement(*id).is_none_or(|p|p.m!=place.m)||scene.pick_id(id.value())!=Some(*id){return Err("Picked curve changed; start again".into())}}Ok(())
    }
    pub fn input(&self)->Result<Input,String>{
        let curves=self.progress.rows.iter().map(|rows|rows.iter().map(|row|self.curves.iter().find(|(id,_)|id.value()==*row).map(|(_,curve)|curve.clone()).ok_or_else(||"Picked source is missing".into())).collect::<Result<Vec<_>,String>>()).collect::<Result<Vec<_>,_>>()?;
        let(x,y)=self.frame.axes();Ok(Input{curves,answers:self.answers[..self.progress.answered.min(self.answers.len())].to_vec(),choice:self.recipe.choices().nth(self.choice).unwrap_or(""),normal:x.cross(&y)})
    }
    pub fn build(&self)->Result<Option<Made>,String>{if self.progress.done(self.recipe.steps){(self.recipe.build)(&self.input()?).map(Some)}else{Ok(None)}}
    pub fn prompt(&self)->String{let step=self.current().map_or("complete",Step::prompt);format!("{} · {step} · Esc cancels",self.recipe.name)}
    pub fn pick(&mut self,scene:&Scene,id:Option<ObjectId>)->Result<Option<Made>,String>{
        let id=id.ok_or("Nothing there · pick a line, polyline or curve")?;if scene.pick_id(id.value())!=Some(id){return Err("Pick a visible unlocked curve".into())}let mut next=self.clone();if !next.curves.iter().any(|(old,_)|*old==id){let curve=Self::world(scene,id).ok_or("Pick a line, polyline or curve")?;next.originals.push((id,Self::source(scene,id).unwrap(),scene.placement(id).unwrap()));next.curves.push((id,curve));}next.progress.pick(next.recipe.steps,id.value())?;let made=next.build()?;*self=next;Ok(made)
    }
    pub fn point(&mut self,point:[f64;3])->Result<Option<Made>,String>{
        let point=crate::tool_coords::checked(point).map_err(str::to_owned)?;let mut next=self.clone();let answer=match next.current(){Some(Step::Point(_))=>Answer::Point(Point::new(point[0],point[1],point[2])),Some(Step::Distance(_))=>{let(base,_)=next.axis()?;Answer::Vector(Vector::new(point[0]-base[0],point[1]-base[1],point[2]-base[2]))},_=>return Err("The current step needs a curve pick or a typed number".into())};next.progress.answer(next.recipe.steps,&mut next.answers,answer);let made=next.build()?;*self=next;Ok(made)
    }
    pub fn text(&mut self,text:&str,fixed:Option<CPlane>)->Result<Option<Made>,String>{
        crate::drawing_coords::set_plane(fixed);let mut next=self.clone();let words=text.split_whitespace().collect::<Vec<_>>();
        if words.is_empty(){next.progress.next(next.recipe.steps,&mut next.answers)?;}else{
            let mut index=0;while index<words.len(){
                let typed=format!("{}{}",next.typed,crate::primitive_options::compact(words[index]));
                if let Some(choice)=next.recipe.choices().position(|choice|crate::primitive_options::compact(choice)==typed){next.choice=choice;next.typed.clear();index+=1;continue}
                if next.recipe.choices().any(|choice|crate::primitive_options::compact(choice).starts_with(&typed)){next.typed=typed;index+=1;continue}
                if let Some((choice,count))=next.recipe.option_in(&words[index..]){next.choice=choice;next.typed.clear();index+=count;continue}
                let step=next.current().ok_or("Extra input after the completed command")?;let answer=step.read(words[index])?;
                match &answer{Answer::Point(point)=>{crate::tool_coords::checked([point[0],point[1],point[2]]).map_err(str::to_owned)?;},Answer::Vector(vector)if(0..3).any(|i|!vector[i].is_finite()||vector[i].abs()>1e12)=>return Err("Distance vector exceeds the accepted range".into()),_=>{}}
                next.typed.clear();next.progress.answer(next.recipe.steps,&mut next.answers,answer);index+=1;
            }
        }let made=next.build()?;*self=next;Ok(made)
    }
    pub fn axis(&self)->Result<(Point,Vector),String>{Ok((self.recipe.axis)(&self.input()?))}
    pub fn on_axis(&self,camera:&crate::camera::Camera,screen:[f32;2])->Option<[f64;3]>{
        let ray=camera.ray(screen)?;let(base,axis)=self.axis().ok()?;let direction=ray.direction;let(a,b)=(direction.dot(&direction),direction.dot(&axis));let denominator=a-b*b;if denominator<=0.01*a{return None}let offset=&ray.origin-&base;let distance=(a*offset.dot(&axis)-b*offset.dot(&direction))/denominator;let p=&base+&(axis*distance);crate::tool_coords::checked([p[0],p[1],p[2]]).ok()
    }
    pub fn outlines(&self,cursor:Option<[f64;3]>)->Vec<Vec<[f64;3]>>{
        if matches!(self.current(),Some(Step::Point(_))){let mut points=self.answers.iter().filter_map(|answer|if let Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect::<Vec<_>>();if let Some(cursor)=cursor{points.push(cursor)}return vec![points]}
        if !matches!(self.current(),Some(Step::Distance(_))){return Vec::new()}let Some(cursor)=cursor else{return Vec::new()};let Ok((base,_))=self.axis()else{return Vec::new()};let offset=Vector::new(cursor[0]-base[0],cursor[1]-base[1],cursor[2]-base[2]);let Ok(input)=self.input()else{return Vec::new()};input.curves.iter().flatten().map(|curve|curve.outline().into_iter().map(|point|{let p=&point+&offset;[p[0],p[1],p[2]]}).collect()).collect()
    }
}
```

## <a id="checkpoint-80c-register-network-patch-aliases-and-initial-corner-arity"></a>Step 4 of 6 · Register network patch aliases and initial corner arity (`80c-register-network-patch-aliases-and-initial-corner-arity`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/6: 80c-register-netw…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gather_registry.rs, src…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current name or alias</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">initial argument validati…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">shared recipe owner</text>
</svg>
</div>

Extend the shared recipe registry and return initial arity errors before replacing the active drawing owner.

> **Check:** *May an initial four-corner command contain only three points?* — The maintained parser accepts no initial corners or exactly four. Reject an initial partial list; interactive answers begin after the empty command.

### 1. `src/gather_registry.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-01.rs"
```

### 2. `src/commands.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-02.rs"
```

## <a id="checkpoint-80d-exercise-networks-corner-retries-and-exact-original-archives"></a>Step 5 of 6 · Exercise networks corner retries and exact original archives (`80d-exercise-networks-corner-retries-and-exact-original-archives`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/6: 80d-exercise-netw…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/network_tests.rs, src/l…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original network and corn…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history transaction</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">exact Save/Open</text>
</svg>
</div>

Copy integrated current network/corner/history fixtures and build the rendered originals through registered commands.

> **Check:** *Can a failed group keep its earlier picks?* — Yes. Failed completion keeps the ordered group. Add its missing crossing curves and retry; no failed result may erase Redo or the originals.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/network_tests.rs`

Copy integrated current network/patch/original-source/history/archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the current surface component.

After that block, append this supplied code:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-02.rs" skip=25
```

**Result after `80d-exercise-networks-corner-retries-and-exact-original-archives`:** An original surface passes through six shuffled network rulings beside a four-corner patch. Current group/orientation/area checks, pending guides, original source placement, one transaction and exact archives remain checked.

## <a id="checkpoint-80za-read-original-projection-planes"></a>Step 6 of 6 · Read original projection planes (`80za-read-original-projection-planes`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6/6: 80za-read-origina…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/project_geometry.rs, sr…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">result or refusal</text>
</svg>
</div>

Reuse absolute coordinate parsing for named construction planes and three-point input. Reject unknown names, wrong point counts and collinear frame points.

> **Check:** *Why validate the three points before projecting?* — They must define a stable plane. Collinear or repeated points cannot supply its normal.

### 1. `src/project_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-1.rs"
```

### 2. `src/lib.rs`

Register the original projection helpers.

After that block, type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-2.rs" skip=32
```

**Result after `80za-read-original-projection-planes`:** Projection-plane parsing is prepared while the existing command behavior remains available.

## Build, Verify & Visual Result

```sh
npm --prefix ../session_tests run course -- dependencies 15.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 15.4
npm --prefix ../session_tests run course -- save 15.4
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [16.1 · Projection, extension and stable edit targets](../../consolidation/81-project-originals-through-the-current-plane-algorithms.md).
