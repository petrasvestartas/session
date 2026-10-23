# Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity

**Absorbs:** Ch 19 (*Integration and original ownership*), Ch 20 (*Shared GPU rendering*), Ch 21 (*Precise camera and browser coordinates*), Ch 22 (*Surface lighting, contact and display quality*) · **Lessons:** 21 · **Checkpoints:** 198 (`96-retain-non-drawing-originals-and-validate-raw-planes` → `96zzzzzzzq-draw-original-cad-wire-curves`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Glyph Atlas (R8)</text>
  <rect x="52" y="60" width="166" height="96" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="64" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="80" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">A</text>
  <rect x="104" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="120" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">B</text>
  <rect x="144" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="160" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">7</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">1px Padded Shelves</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Plane vs Billboard</text>
  <polygon points="324,136 364,74 416,74 376,136" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="370" y="110" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3D</text>
  <rect x="436" y="78" width="68" height="54" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="470" y="109" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2D UI</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">text_plane vs text</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. matrix.wgsl</text>
  <rect x="644" y="64" width="120" height="90" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <line x1="684" y1="64" x2="684" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="724" y1="64" x2="724" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="644" y1="94" x2="764" y2="94" stroke="#111111" stroke-width="1"/>
  <line x1="644" y1="124" x2="764" y2="124" stroke="#111111" stroke-width="1"/>
  <rect x="644" y="64" width="40" height="30" fill="url(#hatch)"/><text x="664" y="83" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">1.0</text>
  <rect x="684" y="94" width="40" height="30" fill="url(#hatch)"/><text x="704" y="113" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">2.4</text>
  <rect x="724" y="124" width="40" height="30" fill="url(#hatch)"/><text x="744" y="143" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">1.0</text>
  <text x="704" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">0-Cells Blank</text>
</svg>
</div>

> **Learning Outcome:** Complete the production Session Viewer architecture: resolve hosted `Element` feature trees and family-aware control edits (`Ch 19`); replace per-object vertex buffers with one shared GPU storage arena, 8-byte draw slots, `ObjectRecord` tables (`slot_table.wgsl`), and vertex pulling (`Ch 20`); eliminate jitter at 10^12-unit coordinates via `f64` floating-origin camera anchoring and reversed-Z (`1.0 -> 0.0`) depth (`Ch 21`); and integrate production surface lighting, contact fills, boundary pens, 1x/4x MSAA sample coherence, semantic family visibility ranges, the **Matrix** procedural digit/cell shader (`matrix.wgsl`), coalesced frame scheduling, and live reload camera/grid preservation (`Ch 22` + Reference `97`).

## 1. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

## 2. Lessons in This Section

| Redraft | Existing | Lesson Title | Checkpoints | Range | Open Lesson |
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

## 3. Core Features & Exercises

| Checkpoint | Exercise | Expected Result |
| :--- | :--- | :--- |
| `96zg / 96zzzzx` | Load `interaction-96p.pb`, select a hosted `Element` feature vs its parent owner, deform a family-keyed control point, and delete/re-add meshes to observe arena span reclamation. | Feature rows resolve to their original `Element` owner; control edits rebuild in place; deleted arena spans are reused after the last GPU lease completes. |
| `96zzzzzi / 96zzzzzz / 96zzzzzzf` | Translate a scene and construction grid to `10^12` world units plus a fractional offset, orbit closely, and return to the origin. | The fractional grid lines and geometry render with zero vertex jitter (`0` pixel difference between near, `10^12`, and return frames), and reversed-Z prevents far-plane z-fighting. |
| `96zzzzzzzl / 96zzzzzzzq / 97` | Render the original `Matrix` (`[-2, 1.2, 5; 0, 1.005, 5]`) at 1x and 4x MSAA, click to select the Matrix, toggle `Hide Lines` / `Show Lines`, and orbit while a live Session reload is in flight. | Non-zero Matrix cells display procedural 3-decimal digits while the `0` cell has no digit and the title has no background plate; selection turns Matrix borders yellow; reload keeps your live camera pose and held base grid. |
