# Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity

> **Gemini Notebook Student Guide** · **Absorbs:** Chapter 19 (*Integration and original ownership*), Chapter 20 (*Shared GPU rendering*), Chapter 21 (*Precise camera and browser coordinates*), Chapter 22 (*Surface lighting, contact and display quality*) · **Lessons:** 21 (`19.1`–`22.6`) · **Checkpoints:** 198 (`96-retain-non-drawing-originals-and-validate-raw-planes` → `96zzzzzzzq-draw-original-cad-wire-curves` + `97-finish-with-the-maintained-browser-runtime`)

## 1. Learning Outcome

Complete the production Session Viewer architecture: resolve hosted `Element` feature trees and family-aware control edits (`Ch 19`); replace per-object vertex buffers with one shared GPU storage arena, 8-byte draw slots, `ObjectRecord` tables (`slot_table.wgsl`), and vertex pulling (`Ch 20`); eliminate jitter at 10^12-unit coordinates via `f64` floating-origin camera anchoring and reversed-Z (`1.0 -> 0.0`) depth (`Ch 21`); and integrate production surface lighting, contact fills, boundary pens, 1x/4x MSAA sample coherence, semantic family visibility ranges, the **Matrix** procedural digit/cell shader (`matrix.wgsl`), coalesced frame scheduling, and live reload camera/grid preservation (`Ch 22` + Reference `97`).

## 2. WebGPU & Architecture Schematic (First-Year Student Guide)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 10 Production Architecture: Shared GPU Storage Arena, f64 Floating Origin, Reversed-Z, 4x MSAA & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How the maintained viewer pulls vertices from one growable GPU arena, eliminates jitter at 10^12 units, and renders procedural Matrices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Shared GPU Arena (Ch 19–20)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• 1 Growable Storage Buffer Arena</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Vertex Pulling (@builtin(vertex_index))</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• 8-byte Draw Slots -&gt; ObjectRecord</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• slot_table.wgsl maps Rg32Uint</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  primitive IDs to Element/Face owners</text>
    <text x="28" y="194" font-size="10" fill="#111111">• Free-list span reuse after GPU lease</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="300" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. f64 Floating Origin &amp; Reversed-Z (Ch 21)</text>
    <text x="312" y="108" font-size="10.5" font-weight="600" fill="#111111">• rel_f32 = (world_f64 - anchor_f64) as f32</text>
    <text x="312" y="125" font-size="10" fill="#111111">  Zero vertex/grid jitter even at 10^12 m!</text>
    <text x="312" y="144" font-size="10.5" font-weight="600" fill="#111111">• Reversed-Z Depth (GreaterEqual):</text>
    <text x="312" y="160" font-size="10" fill="#111111">  Near plane = 1.0, Far plane = 0.0</text>
    <text x="312" y="178" font-size="10.5" fill="#111111">• Held world base grid &amp; preserved</text>
    <text x="312" y="193" font-size="10.5" fill="#111111">  live camera during Session reload</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="597" y="68" width="223" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. 4x MSAA &amp; Matrix Shader (Ch 22)</text>
    <text x="609" y="108" font-size="10.5" fill="#111111">• Coherent 1x/4x MSAA sample targets</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#111111">• matrix.wgsl (16w header + 3w/cell):</text>
    <text x="609" y="142" font-size="10" fill="#111111">  Heatmap + procedural 3-decimal digits</text>
    <text x="609" y="157" font-size="10" fill="#111111">  (0 cell blank; plate-less title; yellow</text>
    <text x="609" y="172" font-size="10" fill="#111111">  selected border; Rg32Uint whole pick)</text>
    <text x="609" y="190" font-size="10" fill="#111111">• Coalesced single-flight frame scheduler</text>
    
</svg>
</div>

```
+---------------------------------------------------------------------------------------------------+
|      SHARED GPU ARENA, FLOATING-ORIGIN PRECISION & MATRIX SHADER (CHAPTERS 19, 20, 21 & 22)      |
+---------------------------------------------------------------------------------------------------+
  1. Double-Precision Floating Origin (Ch 21):
     World f64 (e.g. 10^12 + 0.125m) --(Subtract f64 Camera Anchor)--> Local f32 (0.125m) -> Zero Jitter!
     Depth Buffer: Reversed-Z [Near = 1.0  ======(GreaterEqual)======>  Far = 0.0]

  2. Bindless-Style Shared GPU Arena & Vertex Pulling (Ch 20):
     +-------------------------+     +------------------------+     +-------------------------------+
     | Draw Slot (8 Bytes)     | --> | ObjectRecord Table     | --> | Shared Vertex/Index Arena     |
     | [record_idx, flags/sub] |     | Transform, Color, Radii|     | Pulled via @builtin(vertex_id)|
     +-------------------------+     +------------------------+     +-------------------------------+
                   |
                   v
     [Primitive ID Texture (Rg32Uint)] + [slot_table.wgsl] ===> Exact Face / Ink / Cap / Matrix Pick!

  3. Matrix Procedural Shader (matrix.wgsl in 22.6) & Coalesced Frame Scheduler:
     [16-word Header + 3 words/cell] ---> Cell Color Ramp + Procedural 3-Decimal SDF/Segment Digits
                                          (Zero cells blank; plate-less world title; yellow select)
     Input Burst (Wheel/Drag) ---> Update State Immediately ---> Coalesce into 1 Single-Flight Frame!
```

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | What It Does in Plain English |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

## 3. Existing Chapters & Lessons Absorbed

| Redraft Lesson | Existing Lesson | Title | Checkpoints | Range (`first` → `last`) | Consolidated Lesson Page |
| :--- | :--- | :--- | ---: | :--- | :--- |
| `10.1` | `19.1` | **Complete source metadata and the actual command contract** | 9 | `96-retain-non-drawing-originals-and-validate-raw-planes` → `96h-prove-current-source-grammar-and-explicit-differences` | [96-retain-non-drawing-originals-and-validate-raw-planes.md](../../consolidation/96-retain-non-drawing-originals-and-validate-raw-planes.md) · [Redraft](../lessons/96-retain-non-drawing-originals-and-validate-raw-planes.md) |
| `10.2` | `19.2` | **Original Element ancestry and derived feature trees** | 10 | `96i-separate-own-and-hosted-element-features` → `96r-preserve-exact-command-completions` | [96i-separate-own-and-hosted-element-features.md](../../consolidation/96i-separate-own-and-hosted-element-features.md) · [Redraft](../lessons/96i-separate-own-and-hosted-element-features.md) |
| `10.3` | `19.3` | **Original owners across selection, copying and display** | 8 | `96s-resolve-feature-rows-to-original-element-owners` → `96z-preserve-source-feature-visibility-and-pixel-styles` | [96s-resolve-feature-rows-to-original-element-owners.md](../../consolidation/96s-resolve-feature-rows-to-original-element-owners.md) · [Redraft](../lessons/96s-resolve-feature-rows-to-original-element-owners.md) |
| `10.4` | `19.4` | **Family-aware original controls and source previews** | 7 | `96za-address-original-controls-with-family-keys` → `96zg-rebuild-an-edited-element-without-replacing-its-owner` | [96za-address-original-controls-with-family-keys.md](../../consolidation/96za-address-original-controls-with-family-keys.md) · [Redraft](../lessons/96za-address-original-controls-with-family-keys.md) |
| `10.5` | `20.1` | **Shared surface geometry and finite visibility addresses** | 21 | `96zh-bind-the-shared-geometry-as-storage` → `96zzb-read-section-visibility-from-one-image` | [96zh-bind-the-shared-geometry-as-storage.md](../../consolidation/96zh-bind-the-shared-geometry-as-storage.md) · [Redraft](../lessons/96zh-bind-the-shared-geometry-as-storage.md) |
| `10.6` | `20.2` | **Physical formats, shared section inputs and display defaults** | 4 | `96zzc-share-section-addresses-with-outlines-and-ambient` → `96zzf-start-with-current-browser-display-defaults` | [96zzc-share-section-addresses-with-outlines-and-ambient.md](../../consolidation/96zzc-share-section-addresses-with-outlines-and-ambient.md) · [Redraft](../lessons/96zzc-share-section-addresses-with-outlines-and-ambient.md) |
| `10.7` | `20.3` | **Shared arenas, object records and source preparation** | 14 | `96zzg-grow-buffers-without-losing-rows` → `96zzt-share-the-current-selection-yellow` | [96zzg-grow-buffers-without-losing-rows.md](../../consolidation/96zzg-grow-buffers-without-losing-rows.md) · [Redraft](../lessons/96zzg-grow-buffers-without-losing-rows.md) |
| `10.8` | `20.4` | **Slots, independent instance owners and stable record updates** | 24 | `96zzu-separate-primitive-addresses-from-selection` → `96zzzr-share-transparent-instance-records` | [96zzu-separate-primitive-addresses-from-selection.md](../../consolidation/96zzu-separate-primitive-addresses-from-selection.md) · [Redraft](../lessons/96zzu-separate-primitive-addresses-from-selection.md) |
| `10.9` | `20.5` | **Original picks, ambient metadata and cap consumers** | 16 | `96zzzs-allocate-stable-object-record-rows` → `96zzzzh-pick-caps-through-shared-records` | [96zzzs-allocate-stable-object-record-rows.md](../../consolidation/96zzzs-allocate-stable-object-record-rows.md) · [Redraft](../lessons/96zzzs-allocate-stable-object-record-rows.md) |
| `10.10` | `20.6` | **Retained allocation, live lookup and safe reclamation** | 16 | `96zzzzi-adapt-incremental-slots-to-pulled-draws` → `96zzzzx-retire-empty-arena-ranges-after-tail-trimming` | [96zzzzi-adapt-incremental-slots-to-pulled-draws.md](../../consolidation/96zzzzi-adapt-incremental-slots-to-pulled-draws.md) · [Redraft](../lessons/96zzzzi-adapt-incremental-slots-to-pulled-draws.md) |
| `10.11` | `21.1` | **Units and one precise render origin** | 11 | `96zzzzy-prepare-unit-aware-anchored-camera-space` → `96zzzzzi-share-one-camera-and-measurement-unit` | [96zzzzy-prepare-unit-aware-anchored-camera-space.md](../../consolidation/96zzzzy-prepare-unit-aware-anchored-camera-space.md) · [Redraft](../lessons/96zzzzy-prepare-unit-aware-anchored-camera-space.md) |
| `10.12` | `21.2` | **Precise CPU queries and origin-stamped picks** | 7 | `96zzzzzj-project-cpu-queries-around-the-precise-target` → `96zzzzzp-share-derived-inputs-for-origin-refresh` | [96zzzzzj-project-cpu-queries-around-the-precise-target.md](../../consolidation/96zzzzzj-project-cpu-queries-around-the-precise-target.md) · [Redraft](../lessons/96zzzzzj-project-cpu-queries-around-the-precise-target.md) |
| `10.13` | `21.3` | **Browser ownership of frame matrices and wakeups** | 3 | `96zzzzzq-prepare-one-shared-render-origin` → `96zzzzzs-adopt-the-shared-origin-in-the-browser` | [96zzzzzq-prepare-one-shared-render-origin.md](../../consolidation/96zzzzzq-prepare-one-shared-render-origin.md) · [Redraft](../lessons/96zzzzzq-prepare-one-shared-render-origin.md) |
| `10.14` | `21.4` | **Reversed depth and homogeneous near clipping** | 3 | `96zzzzzt-share-depth-endpoints-and-comparisons` → `96zzzzzz-use-reversed-depth-across-the-frame` | [96zzzzzt-share-depth-endpoints-and-comparisons.md](../../consolidation/96zzzzzt-share-depth-endpoints-and-comparisons.md) · [Redraft](../lessons/96zzzzzt-share-depth-endpoints-and-comparisons.md) |
| `10.15` | `21.5` | **Physical camera, world grid and framed navigation** | 6 | `96zzzzzza-match-physical-camera-defaults-and-limits` → `96zzzzzzf-match-wheel-input-and-framed-projection` | [96zzzzzza-match-physical-camera-defaults-and-limits.md](../../consolidation/96zzzzzza-match-physical-camera-defaults-and-limits.md) · [Redraft](../lessons/96zzzzzza-match-physical-camera-defaults-and-limits.md) |
| `10.16` | `22.1` | **Original surface colours and lighting controls** | 2 | `96zzzzzzg-share-current-surface-lighting-and-camera-uniforms` → `96zzzzzzh-connect-headlight-and-backface-controls` | [96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md](../../consolidation/96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) · [Redraft](../lessons/96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) |
| `10.17` | `22.2` | **Contact fills and original face queries** | 5 | `96zzzzzzi-share-one-frame-across-color-and-ink` → `96zzzzzzm-query-original-faces-under-contact-overlays` | [96zzzzzzi-share-one-frame-across-color-and-ink.md](../../consolidation/96zzzzzzi-share-one-frame-across-color-and-ink.md) · [Redraft](../lessons/96zzzzzzi-share-one-frame-across-color-and-ink.md) |
| `10.18` | `22.3` | **Original boundary pens, joins and vertex markers** | 4 | `96zzzzzzn-respect-original-boundary-pens` → `96zzzzzzq-taper-crowded-original-mesh-vertices` | [96zzzzzzn-respect-original-boundary-pens.md](../../consolidation/96zzzzzzn-respect-original-boundary-pens.md) · [Redraft](../lessons/96zzzzzzn-respect-original-boundary-pens.md) |
| `10.19` | `22.4` | **Original ambient inputs and real pass ordering** | 3 | `96zzzzzzr-share-geometric-ambient-inputs` → `96zzzzzzt-keep-contact-ambient-inputs-on-originals` | [96zzzzzzr-share-geometric-ambient-inputs.md](../../consolidation/96zzzzzzr-share-geometric-ambient-inputs.md) · [Redraft](../lessons/96zzzzzzr-share-geometric-ambient-inputs.md) |
| `10.20` | `22.5` | **Coherent multisampling and frame-resource ownership** | 19 | `96zzzzzzu-keep-depth-and-address-samples-separate` → `96zzzzzzzl-choose-bounded-frame-quality` | [96zzzzzzu-keep-depth-and-address-samples-separate.md](../../consolidation/96zzzzzzu-keep-depth-and-address-samples-separate.md) · [Redraft](../lessons/96zzzzzzu-keep-depth-and-address-samples-separate.md) |
| `10.21` | `22.6` | **Original display controls and remaining integration boundaries** | 5 | `96zzzzzzzm-retain-visible-instance-ranges` → `96zzzzzzzq-draw-original-cad-wire-curves` | [96zzzzzzzm-retain-visible-instance-ranges.md](../../consolidation/96zzzzzzzm-retain-visible-instance-ranges.md) · [Redraft](../lessons/96zzzzzzzm-retain-visible-instance-ranges.md) |
| `10.ref` | `Ref` | **Compare with the maintained browser runtime** | 1 | `97-finish-with-the-maintained-browser-runtime` | [97-finish-with-the-maintained-browser-runtime.md](../../97-finish-with-the-maintained-browser-runtime.md) |

## 4. Rendering, Geometry, Interaction, and Runtime Features Covered

### Rendering & Shaders
- Shared GPU storage arena (`engine/gpu/arena.rs`): packed position/normal/color storage buffers, vertex pulling (`scene.wgsl`, `triangle.wgsl`), 8-byte draw slots, `ObjectRecord` tables, and `Rg32Uint` primitive-to-slot lookup (`shaders/slot_table.wgsl`)
- Reversed-Z (`CompareFunction::GreaterEqual`, clear `0.0`, near `1.0`) floating-point depth buffer and homogeneous near-plane ink clipping
- Production surface lighting (`headlight` & `backface` controls), printed fills (read-only depth when a mesh has a single zero line width), contact polygon fills (`96zzzzzzj`) with joined outline walk, boundary pens/joins, and tapered mesh vertex dots (`shaders/sphere.wgsl`)
- Coherent 1x/4x MSAA multisampling across color, depth, primitive IDs, section crossings, clouds, outlines, and ambient occlusion (`96zzzzzzu`..`96zzzzzzzl`)
- Production **Matrix** renderer (`shaders/matrix.wgsl` in `22.6`): 16-word header + 3 words/cell storage packing, value color interpolation, procedural 3-decimal digit rendering (omitting digits on zero cells), whole-object `Rg32Uint` picking, yellow selected borders, and independent plate-less world title

### Geometry & Data Structures
- Own vs hosted `Element` feature trees (`96i`), family-aware control keys (`96za`), and free-list arena span reclamation after GPU lease completion (`96zzzzw`)
- Double-precision (`f64`) world coordinates & camera target anchored to a throttled local render origin (`gpu_pos_f32 = (world_pos_f64 - anchor_f64) as f32`), preserving sub-millimeter precision even at `10^12` units from world zero

### User Interaction & Selection
- Origin-stamped asynchronous GPU picks (automatically retrying with the latest cursor if a floating-origin rebase occurred while in flight)
- Semantic family visibility controls (`show_points`, `show_lines`, `show_mesh_edges`, `markers`, `Attributes`, `Interactions`) filtering contiguous CPU instance ranges (`src/ink_ranges.rs`) without copying GPU buffers or shifting pick IDs
- Reload camera & held base grid guarantee: live Session reload retains the latest camera pose (including user motion during loading), and the world base grid updates only on an explicit plane-changing construction command

### Browser Runtime & Production Authority (`session_viewer/src`)
- Coalesced single-flight animation frame scheduling (bursts of input events update camera/state immediately and share one pending GPU presentation after queue completion)
- Detachment of the maintained `session_viewer` crate from normal learner library dependencies (used only in dev/reference comparison `97-finish-with-the-maintained-browser-runtime`)
- Production authority: `src/engine/gpu/{arena,objects,render,pass,targets,frame,view}.rs`, `src/engine/pipelines/mod.rs`, `src/shaders/{scene,triangle,slot_table,matrix,ribbon,vector,ink_visibility,sphere,grid}.wgsl`

## 5. Practical Exercises & Expected Visible Results

| Checkpoint(s) | Practical Student Exercise | Expected Visible Result |
| :--- | :--- | :--- |
| `96zg / 96zzzzx` | Load `interaction-96p.pb`, select a hosted `Element` feature vs its parent owner, deform a family-keyed control point, and delete/re-add meshes to observe arena span reclamation. | Feature rows resolve to their original `Element` owner; control edits rebuild in place; deleted arena spans are reused after the last GPU lease completes. |
| `96zzzzzi / 96zzzzzz / 96zzzzzzf` | Translate a scene and construction grid to `10^12` world units plus a fractional offset, orbit closely, and return to the origin. | The fractional grid lines and geometry render with zero vertex jitter (`0` pixel difference between near, `10^12`, and return frames), and reversed-Z prevents far-plane z-fighting. |
| `96zzzzzzzl / 96zzzzzzzq / 97` | Render the original `Matrix` (`[-2, 1.2, 5; 0, 1.005, 5]`) at 1x and 4x MSAA, click to select the Matrix, toggle `Hide Lines` / `Show Lines`, and orbit while a live Session reload is in flight. | Non-zero Matrix cells display procedural 3-decimal digits while the `0` cell has no digit and the title has no background plate; selection turns Matrix borders yellow; reload keeps your live camera pose and held base grid. |

## 6. Verified Work vs. Remaining Implementation or Validation

### Verified in Repository Evidence
- [x] All 21 consolidated lessons (`19.1`–`19.4`, `20.1`–`20.6`, `21.1`–`21.5`, `22.1`–`22.6`) and all 198 checkpoints (197 coding checkpoints + reference comparison `97-finish-with-the-maintained-browser-runtime`) are integrated and pass `checkpoints.test.ts`, strict Vite docs build, and all 26 `check-consolidated-course.cjs` browser route/layout inspections (`current-state.json` lines 17–21).
- [x] `slot_table.wgsl` matches production 100% after comment/whitespace normalization.
- [x] Matrix shader (`matrix.wgsl` in `22.6`): all 5 new native Matrix tests + 2 normal-matrix tests + 3 compact-Matrix tests pass; all 4 unselected/selected 1x/4x native frames individually inspected (`current-state.json` lines 41, 52).
- [x] Production Mesh parity: 8/8 full-frame production/course pairs across DPR 1/2, samples 1/4, and selection match with zero changed RGBA pixels (`current-state.json` line 61).
- [x] Printed fills (3/3 tests), bounded contacts (all perimeter pixels & picks across projections/angles/mirrored placements/densities), and terminal fixture (51.7s) pass (`current-state.json` lines 57, 68, 71).
- [x] Reload camera & held base grid: 4/4 course native regressions and 3/3 maintained CPU/GPU grid checks at `10^12` + fractional origin pass (`current-state.json` lines 58, 163).
- [x] Scheduler & dependency detachment: 3/3 CPU frame-coalescing scheduler tests pass; normal native dependency tree excludes `session_viewer` (`current-state.json` lines 49–50).
- [x] Full native suite: 1,055 PASS / 4 FAIL in complete run, and all 4 subsequently PASS in focused run `native-owner-sheet-focused.json` (`current-state.json` lines 76, 86).

### Remaining Implementation or Validation
- [ ] Clean final full-suite rerun of all 1,059 native tests from viewer root cwd after source freeze (`current-state.json` lines 86, 97).
- [ ] Browser acceptance of `course-feature-pick.pb`, 4 route/live reload-grid browser combinations, 5 performance measurement categories, and the final `96zzzzzzzq` / `97` release gates (`current-state.json` lines 98–100).
