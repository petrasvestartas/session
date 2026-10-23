# 11.1 · Coverage-based screen-space outlines

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 11.1** (Redraft `6.10`) · **10 checkpoints (`63-radius` → `63h-outline`)** · **Study time: 5–10 h**

> **What You Will Learn & Build:** Measure outline width in CSS pixels. By the final checkpoint (`63h-outline`), you will check real contours widths reuse picks and cleanup.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 11.1 Execution Flow: Coverage-based screen-space outlines</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (10 Checkpoints: 63-radius → 63h-outline)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">CSS radius</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">physical density</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">bounded sorted taps</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">shared coverage</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">width</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">actual pixels and</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">alpha</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 11.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/outline_math.rs  ·  src/lib.rs  ·  src/outline.wgsl  ·  src/outline_mask.wgsl  ·  src/outline_gpu.rs  ·  src/gpu_mesh.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">10 Checkpoints: 63-radius → 63h-outline — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 9 CAD B-Rep Pipeline: Topological Loops/Trims, Crease Normals & Analytical Boundary Wires</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How exact Boundary Representations (B-Rep) combine shaded trimmed surface meshes with crisp analytical boundary & isoline curves.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. B-Rep Topological Validation</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Solid -&gt; Faces -&gt; Loops (Outer + Holes)</text>
    <text x="28" y="126" font-size="10.5" fill="#111111">• Trim curves in 2D (u, v) domain</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Validate orientation, loop closure,</text>
    <text x="28" y="160" font-size="10.5" font-weight="600" fill="#111111">  and manifold edge adjacency</text>
    <text x="28" y="180" font-size="10.5" fill="#111111">• Preserve exact NURBS surface source</text>

    <line x1="263" y1="112" x2="295" y2="112" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="263" y1="175" x2="295" y2="175" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="297" y="68" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="86" font-size="11" font-weight="700" fill="#111111">2A. Shaded Trimmed Faces (Surface Lane)</text>
    <text x="309" y="104" font-size="10.5" fill="#111111">• Tessellate within (u,v) trim loops</text>
    <text x="309" y="120" font-size="10.5" fill="#111111">• Smooth normals across G1; split at creases!</text>

    <rect x="297" y="145" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="163" font-size="11" font-weight="700" fill="#111111">2B. Boundary Wires &amp; Isolines (Ribbon Lane)</text>
    <text x="309" y="181" font-size="10.5" fill="#111111">• Extract 3D topological edges + UV isolines</text>
    <text x="309" y="197" font-size="10.5" fill="#111111">• Render via ribbon.wgsl with depth bias</text>

    <line x1="572" y1="138" x2="602" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="604" y="68" width="216" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="712" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Unified B-Rep Sub-Picking</text>
    <text x="616" y="108" font-size="10.5" fill="#111111">• Every triangle &amp; wire carries</text>
    <text x="616" y="124" font-size="10.5" font-weight="600" fill="#111111">  [brep_owner_id, sub_id]</text>
    <text x="616" y="144" font-size="10" fill="#111111">• Click selects whole B-Rep solid</text>
    <text x="616" y="159" font-size="10" fill="#111111">  or drills into Face #i / Edge #j</text>
    <text x="616" y="178" font-size="10.5" fill="#111111">• Outlines &amp; Point Clouds (Sec 6)</text>
    <text x="616" y="193" font-size="10" fill="#111111">  share the same owner ID contract</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **CAD Topology** | `EdgeUse { edge_index, reversed, pcurve_2d }` | Connects two adjacent curved faces along one shared 3D `Edge` while giving each face its own oriented `(u, v)` trim curve. |
| **WGSL (`splat.wgsl`)** | `let dist2 = dot(pixel_offset, pixel_offset); if dist2 > r2 { discard; }` | Turns each point quad into a crisp circular disc around an integer pixel center and discards corner fragments. |
| **WGSL (`splat_resolve`)** | `exp(-edl_sum * 75.0 * strength)` | Computes Eye-Dome Lighting from neighbor depth discontinuities so unlit point clouds show 3D surface relief. |
| **WGSL (`surface_outline`)** | Conservative mask dilation -> single composite | Detects silhouette and selection boundaries in screen space without duplicating 3D wire geometry. |

**Authoritative production source (`session_viewer/src`):** [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs), [engine/gpu/cloud.rs](../../../../src/engine/gpu/cloud.rs), [engine/gpu/lod.rs](../../../../src/engine/gpu/lod.rs), [engine/gpu/splat.rs](../../../../src/engine/gpu/splat.rs), [shaders/splat.wgsl](../../../../src/shaders/splat.wgsl), [shaders/splat_resolve.wgsl](../../../../src/shaders/splat_resolve.wgsl), [app/cloud_query.rs](../../../../src/app/cloud_query.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`63-radius`](#checkpoint-63-radius) | Measure outline width in CSS pixels. | `src/outline_math.rs`, `src/lib.rs` | `CSS radius → physical density → bounded sorted taps → shared coverage width` |
| **2** | [`63a-coverage`](#checkpoint-63a-coverage) | Write bounded contour sampling and black-alpha composition. | `src/outline.wgsl`, `src/outline_mask.wgsl` | `original solid transforms → visible R8 coverage → coarse search → black alpha` |
| **3** | [`63aa-pool-and-dilate-a-conservative-outline-mask`](#checkpoint-63aa-pool-and-dilate-a-conservative-outline-mask) | Accelerate bounded contour sampling with conservative coarse coverage. | `src/outline.wgsl` | `solid coverage → block maximum → neighboring-block dilation → skip empty contour work` |
| **4** | [`63b-resources`](#checkpoint-63b-resources) | Own and count the derived outline textures. | `src/outline_gpu.rs`, `src/lib.rs` | `derived textures → owning masks → allocated bytes → explicit release` |
| **5** | [`63c-pipelines`](#checkpoint-63c-pipelines) | Create the mask and fullscreen pipeline layouts. | `src/outline_gpu.rs` | `original vertex layout → depth mask pipeline → fullscreen pipeline → alpha blending` |
| **6** | [`63d-bindings`](#checkpoint-63d-bindings) | Build bounded mask and tap bindings. | `src/outline_gpu.rs` | `texture layouts → bounded parameter buffers → current shader → reusable pipelines` |
| **7** | [`63e-reuse`](#checkpoint-63e-reuse) | Reuse masks while camera selection and density stay unchanged. | `src/outline_gpu.rs` | `viewport and density → prepared masks → radius-keyed taps → invalidated frame key` |
| **8** | [`63f-union`](#checkpoint-63f-union) | Outline the visible union without internal seams. | `src/gpu_mesh.rs`, `src/outline_gpu.rs` | `selected solids → depth-visible union → outside alpha → black load pass` |
| **9** | [`63g-composite`](#checkpoint-63g-composite) | Connect a lazy silhouette owner to the renderer. | `src/renderer.rs` | `whole selection → lazy owner → black silhouette → immediate release` |
| **10** | [`63h-outline`](#checkpoint-63h-outline) | Check real contours widths reuse picks and cleanup. | `src/outline_gpu.rs`, `src/renderer.rs`, `src/outline_math_tests.rs`, `src/lib.rs` | `actual pixels and alpha → original picks and topology → cache reuse → resource cleanup` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-63-radius"></a>

### Step 1 of 10 · Measure outline width in CSS pixels (`63-radius`)

- **Goal:** Measure outline width in CSS pixels.
- **Data Flow Trace:** `CSS radius → physical density → bounded sorted taps → shared coverage width`
- **Files You Touch:** `src/outline_math.rs` (*Convert the current production CSS radius and build bounded nearest-first coverage taps.*) · `src/lib.rs` (*Register the source component.*)

Convert one production CSS radius to physical pixels and sort bounded coverage taps nearest first. Flat and curved solids use the same screen-space rule.

> **Self-Check Question:** *Should a smooth cylinder get a different outline radius from a box?*  
> **Answer:** No. Their visible coverage differs, but both use the same CSS radius and tap weights. Physical resolution changes with density; geometry size does not set screen ink width.

- **What You Should See:** The selected original scene label and blue CAD bore remain visible. Select the CAD whole object after the compositor connects to see its silhouette.
- **Hands-On Lab Experiment:** Compare densities 1, 1.5 and 2. Check the 12-physical-pixel cap, 512-tap bound and decreasing tap weights.

<details>
<summary>Implementation 63-radius: Measure outline width in CSS pixels</summary>

Measure outline width in CSS pixels.

Data flow: CSS radius → physical density → bounded sorted taps → shared coverage width.

### 1. `src/outline_math.rs`

Convert the current production CSS radius and build bounded nearest-first coverage taps.

Create the file and type:

```rust
--8<-- "journey/code/63-radius-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod scene_label_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/63-radius-02.rs" skip=36
```

</details>

<a id="checkpoint-63a-coverage"></a>

### Step 2 of 10 · Convolve solid coverage into external contour ink (`63a-coverage`)

- **Goal:** Write bounded contour sampling and black-alpha composition.
- **Data Flow Trace:** `original solid transforms → visible R8 coverage → coarse search → black alpha`
- **Files You Touch:** `src/outline.wgsl` (*Type the bounded contour predicate and separate composition entry points.*) · `src/outline_mask.wgsl` (*Transform the original solid vertices and write only depth-visible coverage.*)

Read center coverage and conservatively skip pixels whose coarse neighborhood is empty.

Visit a bounded list of preweighted taps, keeping the strongest nearby coverage. Suppress the solid interior with one minus center coverage. The selected and ordinary masks retain separate radius witnesses; take their maximum before black-alpha composition so overlap is not blended twice.

The mask shader writes derived solid coverage. The next lesson supplies conservative pooling and dilation; the existing resource and pipeline lessons then connect this shader to the browser frame.

> **Self-Check Question:** *Why must contour ink exclude the source solid’s covered interior?*  
> **Answer:** The mask describes the solid; the contour describes nearby uncovered pixels. Multiply sampled coverage by one minus center coverage, then compose the contour once.

- **What You Should See:** The contour and composition entries are written, with the original geometry picture unchanged during preparation.
- **Hands-On Lab Experiment:** Trace a covered interior pixel, an uncovered pixel beside an edge, and an empty distant pixel through coverage_at. Predict each output and identify the maximum number of texture reads.

<details>
<summary>Implementation 63a-coverage: Convolve solid coverage into external contour ink</summary>

Write bounded contour sampling and black-alpha composition.

Data flow: original solid transforms → visible R8 coverage → coarse search → black alpha.

### 1. `src/outline.wgsl`

Type the bounded contour predicate and separate composition entry points.

Create the file and type:

```wgsl
--8<-- "journey/code/63a-coverage-typed.wgsl"
```

### 2. `src/outline_mask.wgsl`

Transform the original solid vertices and write only depth-visible coverage.

Create the file and type:

```wgsl
--8<-- "journey/code/63a-coverage-02.rs"
```

</details>

<a id="checkpoint-63aa-pool-and-dilate-a-conservative-outline-mask"></a>

### Step 3 of 10 · Pool and dilate a conservative outline mask (`63aa-pool-and-dilate-a-conservative-outline-mask`)

- **Goal:** Accelerate bounded contour sampling with conservative coarse coverage.
- **Data Flow Trace:** `solid coverage → block maximum → neighboring-block dilation → skip empty contour work`
- **Files You Touch:** `src/outline.wgsl` (*Pool and dilate coverage without losing a possible contour hit.*)

Pool each block with a maximum, then dilate by the neighboring blocks.

The resulting mask conservatively marks where contour taps may encounter solid coverage. Clamp border reads and skip out-of-size pooled coordinates. This changes the amount of sampling work without changing the original CAD surface or the final coverage rule.

> **Self-Check Question:** *Why can the coarse mask use a maximum rather than an average?*  
> **Answer:** A maximum preserves any covered source pixel. An average could erase a small feature and incorrectly skip its outline work.

- **What You Should See:** The completed shader contains contour, composition, pooling and dilation entry points; the following existing resource lessons connect them.
- **Hands-On Lab Experiment:** Predict pooled and dilated values for a block containing one covered pixel, an empty block beside it, and an odd viewport edge. Explain why a mean would violate the early-out contract.

<details>
<summary>Implementation 63aa-pool-and-dilate-a-conservative-outline-mask: Pool and dilate a conservative outline mask</summary>

Accelerate bounded contour sampling with conservative coarse coverage.

Data flow: solid coverage → block maximum → neighboring-block dilation → skip empty contour work.

### 1. `src/outline.wgsl`

Pool and dilate coverage without losing a possible contour hit.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn fs_alpha(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(coverage_at(position.xy), 0.0, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let alpha = textureLoad(mask, vec2<i32>(position.xy), 0).r;

    if (alpha <= 0.0) {
        discard;
    }

    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
```

</details>

After that block, type:

```wgsl
@fragment
fn fs_pool(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let block = i32(radius.z);
    let base = vec2<i32>(position.xy) * block;
    var highest = 0.0;

    for (var y = 0; y < block; y++) {
        for (var x = 0; x < block; x++) {
            let q = base + vec2<i32>(x, y);

            if (any(q >= size)) {
                continue;
            }

            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}

@fragment
fn fs_dilate(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let c = vec2<i32>(position.xy);
    var highest = 0.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let q = clamp(c + vec2<i32>(x, y), vec2<i32>(0), size - 1);
            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/63aa-pool-and-dilate-a-conservative-outline-mask-01.wgsl"
```

</details>

</details>

<a id="checkpoint-63b-resources"></a>

### Step 4 of 10 · Own and count the derived outline textures (`63b-resources`)

- **Goal:** Own and count the derived outline textures.
- **Data Flow Trace:** `derived textures → owning masks → allocated bytes → explicit release`
- **Files You Touch:** `src/outline_gpu.rs` (*Own R8 images, frame keys, pipeline state and the reusable fullscreen preparation pass.*) · `src/lib.rs` (*Register the source component.*)

Keep full-size coverage and alpha alongside two coarse masks. Count actual textures and retained parameter buffers; dropping an image destroys its GPU texture.

> **Self-Check Question:** *Does a zero live draw count mean no GPU allocation remains?*  
> **Answer:** No. Reserved textures and buffers still occupy memory. Report their actual bytes while owned; dropping the complete lazy outline owner releases those allocations.

- **Hands-On Lab Experiment:** Compare full and coarse dimensions. Pooling rounds up at the right and bottom edges, including a viewport smaller than one block.

<details>
<summary>Implementation 63b-resources: Own and count the derived outline textures</summary>

Own and count the derived outline textures.

Data flow: derived textures → owning masks → allocated bytes → explicit release.

### 1. `src/outline_gpu.rs`

Own R8 images, frame keys, pipeline state and the reusable fullscreen preparation pass.

Create the file and type:

```rust
--8<-- "journey/code/63b-resources-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod outline_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/63b-resources-02.rs" skip=23
```

</details>

<a id="checkpoint-63c-pipelines"></a>

### Step 5 of 10 · Create the mask and fullscreen pipeline layouts (`63c-pipelines`)

- **Goal:** Create the mask and fullscreen pipeline layouts.
- **Data Flow Trace:** `original vertex layout → depth mask pipeline → fullscreen pipeline → alpha blending`
- **Files You Touch:** `src/outline_gpu.rs` (*Create compatible mask and fullscreen pipelines without reallocating primary geometry.*)

Bind original mesh positions for the depth-visible mask. Fullscreen stages read R8 textures; only the final black-alpha pass blends with scene colour.

> **Self-Check Question:** *Should the mask pipeline replace the primary colour or integer pipeline?*  
> **Answer:** No. It reuses original vertex buffers and compatible view/object bindings for a derived pass. The primary colour draw and integer ownership remain intact.

- **Hands-On Lab Experiment:** Check the 24-byte vertex stride and forward LessEqual comparison. Compare the R8 preparation target with the scene-colour alpha-blended target.

<details>
<summary>Implementation 63c-pipelines: Create the mask and fullscreen pipeline layouts</summary>

Create the mask and fullscreen pipeline layouts.

Data flow: original vertex layout → depth mask pipeline → fullscreen pipeline → alpha blending.

### 1. `src/outline_gpu.rs`

Create compatible mask and fullscreen pipelines without reallocating primary geometry.

<details>
<summary>Locate the existing block</summary>

```rust
  pass.draw(0..3, 0..1);
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/63c-pipelines-01.rs" skip=27
```

</details>

<a id="checkpoint-63d-bindings"></a>

### Step 6 of 10 · Build bounded mask and tap bindings (`63d-bindings`)

- **Goal:** Build bounded mask and tap bindings.
- **Data Flow Trace:** `texture layouts → bounded parameter buffers → current shader → reusable pipelines`
- **Files You Touch:** `src/outline_gpu.rs` (*Build explicit current shader bindings and fixed parameter capacity.*)

Create explicit texture layouts, a 16-byte radius uniform and one fixed 8208-byte sorted-tap buffer. Keep pipelines reusable for the lazy outline owner.

> **Self-Check Question:** *Why is the sorted-tap buffer larger than the used tap count?*  
> **Answer:** It has fixed capacity for 512 sixteen-byte taps and a sixteen-byte header. The shader reads the live count, while diagnostics report the complete allocation.

- **Hands-On Lab Experiment:** Match each shader group to its layout. The two search bindings share the same selected union mask, so the optional second production channel stays inactive.

<details>
<summary>Implementation 63d-bindings: Build bounded mask and tap bindings</summary>

Build bounded mask and tap bindings.

Data flow: texture layouts → bounded parameter buffers → current shader → reusable pipelines.

### 1. `src/outline_gpu.rs`

Build explicit current shader bindings and fixed parameter capacity.

<details>
<summary>Locate the existing block</summary>

```rust
}
impl Outline {
}
fn raster(
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/63d-bindings-01.rs"
```

</details>

<a id="checkpoint-63e-reuse"></a>

### Step 7 of 10 · Reuse masks while camera selection and density stay unchanged (`63e-reuse`)

- **Goal:** Reuse masks while camera selection and density stay unchanged.
- **Data Flow Trace:** `viewport and density → prepared masks → radius-keyed taps → invalidated frame key`
- **Files You Touch:** `src/outline_gpu.rs` (*Prepare exact-size derived targets, bounded tap bytes and truthful resource counters.*)

Allocate masks only when viewport size or coarse block changes. Upload the tap table only when physical radius changes; keep an explicit revision for scene and selection edits.

> **Self-Check Question:** *Can an unchanged camera alone prove a cached outline is current?*  
> **Answer:** No. Geometry, visibility and whole selection can change without moving the camera. Include their explicit revision, physical size and density in the reuse key.

- **Hands-On Lab Experiment:** Read allocated-byte counters before and after a same-size preparation. Compare a camera change with a whole-selection change and a density change.

<details>
<summary>Implementation 63e-reuse: Reuse masks while camera selection and density stay unchanged</summary>

Reuse masks while camera selection and density stay unchanged.

Data flow: viewport and density → prepared masks → radius-keyed taps → invalidated frame key.

### 1. `src/outline_gpu.rs`

Prepare exact-size derived targets, bounded tap bytes and truthful resource counters.

<details>
<summary>Locate the existing block</summary>

```rust
    self.key = None;
  }
}
fn raster(
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/63e-reuse-01.rs"
```

</details>

<a id="checkpoint-63f-union"></a>

### Step 8 of 10 · Outline the visible union without internal seams (`63f-union`)

- **Goal:** Outline the visible union without internal seams.
- **Data Flow Trace:** `selected solids → depth-visible union → outside alpha → black load pass`
- **Files You Touch:** `src/gpu_mesh.rs` (*Read whole-selection state from the existing uploaded object uniform.*) · `src/outline_gpu.rs` (*Encode visible selected coverage, cached alpha and a black compositor after geometry ink.*)

Draw all whole-selected solids into one depth-visible mask, then pool, dilate and search outside that union. The black pass loads colour after surface and source-edge ink.

> **Self-Check Question:** *Should two touching selected objects have an outline along their shared join?*  
> **Answer:** No. A union silhouette surrounds visible coverage, including real holes. Internal tessellation and shared solid joins remain inside the coverage and contribute no outline alpha.

- **Hands-On Lab Experiment:** Compare touching boxes, a smooth cylinder and a block with a bore. Keep the bore center open and its inner contour black; leave original mesh/BRep topology untouched.

<details>
<summary>Implementation 63f-union: Outline the visible union without internal seams</summary>

Outline the visible union without internal seams.

Data flow: selected solids → depth-visible union → outside alpha → black load pass.

### 1. `src/gpu_mesh.rs`

Read whole-selection state from the existing uploaded object uniform.

<details>
<summary>Locate the existing block</summary>

```rust
    }

    pub fn settings_bytes(&self) -> u64 { self.model.size() }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/63f-union-01.rs"
```

### 2. `src/outline_gpu.rs`

Encode visible selected coverage, cached alpha and a black compositor after geometry ink.

<details>
<summary>Locate the existing block</summary>

```rust
    }
  }
}
fn raster(
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/63f-union-02.rs"
```

</details>

<a id="checkpoint-63g-composite"></a>

### Step 9 of 10 · Connect a lazy silhouette owner to the renderer (`63g-composite`)

- **Goal:** Connect a lazy silhouette owner to the renderer.
- **Data Flow Trace:** `whole selection → lazy owner → black silhouette → immediate release`
- **Files You Touch:** `src/renderer.rs` (*Connect whole-object silhouette invalidation, lazy compositing, current allocation counters and immediate owner release.*)

Create the outline owner only when a whole solid is selected. Invalidate it on scene or selection edits, composite before widgets and text, and drop it immediately on deselection or Close.

> **Self-Check Question:** *Should selecting one source face expand its highlight to the whole solid silhouette?*  
> **Answer:** No. Partial face and edge highlights keep their local footprint. This pass follows whole-object selection only; the earlier locality regression remains unchanged.

- **Hands-On Lab Experiment:** Select a whole CAD object, then deselect it. Compare derived bytes, primary upload counts and stable original IDs; repeat an unchanged frame to inspect the skip counter.

<details>
<summary>Implementation 63g-composite: Connect a lazy silhouette owner to the renderer</summary>

Connect a lazy silhouette owner to the renderer.

Data flow: whole selection → lazy owner → black silhouette → immediate release.

### 1. `src/renderer.rs`

Connect whole-object silhouette invalidation, lazy compositing, current allocation counters and immediate owner release.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
```

</details>

Replace it with:

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
```

</details>

Replace it with:

```rust
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
```

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }
```

</details>

Replace it with:

```rust
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }
```

<details>
<summary>Locate this block</summary>

```rust
        }
        drop(old);
        self.geometry.prune();
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }
```

</details>

Replace it with:

```rust
        }
        drop(old);
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }
```

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
```

</details>

Replace it with:

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
```

<details>
<summary>Locate this block</summary>

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

</details>

Replace it with:

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

<details>
<summary>Locate this block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
```

</details>

Replace it with:

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout)], immediate_size: 0 });
        let mesh_pipeline = |depth| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }, wgpu::VertexBufferLayout {
                    array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.view.faces.get(&row.id) {marker.colour=rgba(*colour);}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.view.faces.get(&row.id) {for segment in &mut segments {segment.stroke.colour=rgba(*colour);}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.view.faces.get(&row.id) {stroke.colour=rgba(*colour);}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        if let Some(request) = self.picker.next(view) { self.picker.submit(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref()); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self, settings: Option<crate::grid::Settings>) -> Result<(), &'static str> {
        if settings == self.grid_settings { return Ok(()); }
        let strokes = settings.map(crate::grid::Settings::strokes).transpose()?.unwrap_or_default();
        self.grid.set(&self.device, strokes)?; self.grid_settings = settings; Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:[[0.;4];4]}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(merged)?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(merged)?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, 0.0, 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.view.faces.get(&id).copied()}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/63g-composite-01.rs"
```

```text
--8<-- "journey/code/63g-composite-02.rs"
```

```text
--8<-- "journey/code/63g-composite-03.rs"
```

```text
--8<-- "journey/code/63g-composite-04.rs"
```

```text
--8<-- "journey/code/63g-composite-05.rs"
```

```text
--8<-- "journey/code/63g-composite-06.rs"
```

```text
--8<-- "journey/code/63g-composite-07.rs"
```

</details>

</details>

<a id="checkpoint-63h-outline"></a>

### Step 10 of 10 · Check real contours widths reuse picks and cleanup (`63h-outline`)

- **Goal:** Check real contours widths reuse picks and cleanup.
- **Data Flow Trace:** `actual pixels and alpha → original picks and topology → cache reuse → resource cleanup`
- **Files You Touch:** `src/outline_gpu.rs` (*Copy the test-only actual-alpha readback accessor.*) · `src/renderer.rs` (*Copy test-only R8 alpha readback with bounded row pitch and wait.*) · `src/outline_math_tests.rs` (*Copy the current production radius, tap capacity and order regressions.*) · `src/lib.rs` (*Register the source component.*) · `src/pick_gpu_tests.rs` (*Copy actual joined-solid, occlusion, curved/flat density, bore-contour, source/pick/reuse/release regressions.*) · `src/browser.rs` (*Expose actual outline allocation, rebuild, skip and compositor draw counters.*)

Copy the actual GPU silhouette regressions and connect browser diagnostics. Verify both original source lanes and two CSS densities before calling silhouette acceptance complete.

> **Self-Check Question:** *Does a native software-GPU check establish smooth Lenovo rotation?*  
> **Answer:** No. It establishes rendered behavior and source/resource invariants. Fresh browser acceptance and same-hardware full-floor and dome timing remain separate required checks.

- **What You Should See:** Type Close, Example CAD, Fit and View Isometric, then Select Next. The selected CAD solid has black outer and bore contours. Whole selection creates derived outline resources; deselection and Close drop them. Repeat an unchanged frame to reuse coverage, and compare curved and flat solids at both CSS densities.
- **Hands-On Lab Experiment:** Type Close, Example CAD, Fit and View Isometric, then Select Next. Inspect the black outer and bore contours. Deselect, select a source face, and confirm its highlight remains local.

<details>
<summary>Implementation 63h-outline: Check real contours widths reuse picks and cleanup</summary>

Check real contours widths reuse picks and cleanup.

Data flow: actual pixels and alpha → original picks and topology → cache reuse → resource cleanup.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/outline_gpu.rs`

Copy the test-only actual-alpha readback accessor.

<details>
<summary>Locate the existing block</summary>

```text
    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/63h-outline-01.rs" skip=95
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/renderer.rs`

Copy test-only R8 alpha readback with bounded row pitch and wait.

<details>
<summary>Locate the existing block</summary>

```text

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/63h-outline-02.rs" skip=94
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/outline_math_tests.rs`

Copy the current production radius, tap capacity and order regressions.

Copy this supplied file:

```text
--8<-- "journey/code/63h-outline-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod outline_gpu;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/63h-outline-04.rs" skip=22
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Copy actual joined-solid, occlusion, curved/flat density, bore-contour, source/pick/reuse/release regressions.

<details>
<summary>Locate the existing block</summary>

```text
    assert!(changed>50);renderer.request_pick([-0.0625,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([0.125,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/63h-outline-05.rs" skip=235
```

</details>

### 6. `src/browser.rs`

Expose actual outline allocation, rebuild, skip and compositor draw counters.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-scene-text-stats",&serde_json::to_string(&renderer.text_stats()).unwrap())?;
    canvas.set_attribute("data-scene-text-cpu",&serde_json::to_string(&crate::memory::texts(editor.presentation_scenes())).unwrap())?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/63h-outline-06.rs"
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 63h-outline
npm --prefix ../session_tests run course -- check 63h-outline
npm --prefix ../session_tests run course -- save 63h-outline
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Type Close, Example CAD, Fit and View Isometric, then Select Next. The selected CAD solid has black outer and bore contours. Whole selection creates derived outline resources; deselection and Close drop them. Repeat an unchanged frame to reuse coverage, and compare curved and flat solids at both CSS densities.

- **Final Lesson Experiment:** Type Close, Example CAD, Fit and View Isometric, then Select Next. Inspect the black outer and bore contours. Deselect, select a source face, and confirm its highlight remains local.

Continue with [Validated original cloud columns and placed owners](../../consolidation/64-nodes.md).
