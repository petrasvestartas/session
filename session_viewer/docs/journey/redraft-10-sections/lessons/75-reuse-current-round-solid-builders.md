# 14.5 · Round solids and exact planar conics

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Checkpoints:** `75-reuse-current-round-solid-builders` → `75zf-build-original-parabola-previews-and-sources` (11 steps) · **Production source:** [app/walk/sheet.rs](../../../../src/app/walk/sheet.rs) · [app/sheet_query.rs](../../../../src/app/sheet_query.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/cplane.rs](../../../../src/app/cplane.rs) · [app/coords.rs](../../../../src/app/coords.rs) · [app/command/tool/shape.rs](../../../../src/app/command/tool/shape.rs) · [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs) · [app/deform.rs](../../../../src/app/deform.rs)

> **Goal:** Read the prepared start, apex and end from the checked local frame.

## 1. Visual WebGPU Schematic

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
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Rg32Uint ID Map</text>
  <rect x="75" y="56" width="120" height="100" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="115" y1="56" x2="115" y2="156" stroke="#111111" stroke-width="1"/>
  <line x1="155" y1="56" x2="155" y2="156" stroke="#111111" stroke-width="1"/>
  <line x1="75" y1="90" x2="195" y2="90" stroke="#111111" stroke-width="1"/>
  <line x1="75" y1="123" x2="195" y2="123" stroke="#111111" stroke-width="1"/>
  <rect x="115" y="90" width="40" height="33" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <text x="135" y="110" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">7, 3</text>
  <text x="135" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">[owner_id, sub_id]</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. 256-Byte Readback</text>
  <rect x="326" y="76" width="110" height="44" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="436" y="76" width="74" height="44" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="381" y="102" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Crop Row</text>
  <text x="473" y="102" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Pad</text>
  <text x="419" y="146" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">|-- 256 Bytes --|</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">map_async(Read)</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Priority &amp; Ticket</text>
  <rect x="614" y="64" width="180" height="32" rx="5" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="84" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. Point &gt; 2. Wire &gt; 3. Face</text>
  <rect x="614" y="110" width="180" height="42" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="135" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">ticket.rev == view.rev</text>
  <text x="704" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Drop Stale Picks</text>
</svg>
</div>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-75-reuse-current-round-solid-builders"></a>

### Step 1 of 11 · Build original tori with checked radii (`75-reuse-current-round-solid-builders`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/round_torus.rs, src/l…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">checked local frame</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">exact kernel solid</text>
</svg>
</div>

Collect a center and two radii with the existing question owner. Reject an invalid radius pair before building the torus. Draw its pending rings using the checked frame, then ask the kernel for the chosen original representation.

> **Check:** *Why check both torus radii before building?* — Reject an invalid ring before publishing any source geometry or changing undo history. The existing question owner can retain the answers for correction.

### 1. `src/round_torus.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_torus.rs`

Supply only the unchanged original representation and topology fixture.

After that block, append this supplied code:

```text
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus-fixtures.rs" skip=2228
```

### 3. `src/lib.rs`

Register this shared original solid builder.

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-08.rs" skip=23
```

**Result after `75-reuse-current-round-solid-builders`:** The original Box, Pyramid and Block With Hole remain visible while the round family connects. The final exercise displays a Cylinder, Cone, Sphere and Torus.

---

<a id="checkpoint-75aa-build-original-capsules-with-shared-dimensions"></a>

### Step 2 of 11 · Build original capsules with shared dimensions (`75aa-build-original-capsules-with-shared-dimensions`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/round_capsule.rs, src…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">checked local frame</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">exact kernel solid</text>
</svg>
</div>

Reuse the cylinder’s existing center, radius and signed-height questions. Require a height larger than the diameter, draw the two cap junctions, and build the kernel capsule around its local central axis. Transform that original mesh through the same checked frame.

> **Check:** *Why must capsule height exceed its diameter?* — The two hemispherical caps need a positive central span. Reuse the existing dimensions but reject overlapping caps before building.

### 1. `src/round_capsule.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_capsule.rs`

Supply only the unchanged original representation and topology fixture.

After that block, append this supplied code:

```text
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule-fixtures.rs" skip=2191
```

### 3. `src/lib.rs`

Register this shared original solid builder.

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-02.rs" skip=22
```

**Result after `75aa-build-original-capsules-with-shared-dimensions`:** The capsule builder shares one dimension contract across preview and source preparation.

---

<a id="checkpoint-75a-retain-mesh-only-round-families"></a>

### Step 3 of 11 · Mesh original quad-sphere patches (`75a-retain-mesh-only-round-families`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/round_quad_sphere.rs,…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">checked local frame</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">exact kernel solid</text>
</svg>
</div>

Ask the kernel for six original quad-sphere patches. Sample each patch with the existing quad mesher, then reuse the welding helper from 73h–74 to connect their seams. Transform the closed mesh through the checked frame and retain the mesh-only representation.

> **Check:** *Why weld the six patch meshes?* — Adjacent patches meet along common seams. Reuse the existing weld helper so the result shares vertices and forms one closed mesh.

### 1. `src/round_quad_sphere.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_quad_sphere.rs`

Supply only the unchanged original representation and topology fixture.

After that block, append this supplied code:

```text
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere-fixtures.rs" skip=987
```

### 3. `src/lib.rs`

Register this shared original solid builder.

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-04.rs" skip=24
```

---

<a id="checkpoint-75b-register-current-round-names-and-choices"></a>

### Step 4 of 11 · Register current round names and choices (`75b-register-current-round-names-and-choices`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/primitive_registry.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">shared questions</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">one original commit</text>
</svg>
</div>

Connect each current name to its maintained definition and available representation. The shared input owner already handles defaults, cursor previews, cancellation and atomic invalid input.

> **Check:** *What happens after an invalid torus minor radius?* — The accepted center and major radius remain. Correct the minor radius or cancel; no scene object or history transaction is created by the error.

### 1. `src/primitive_registry.rs`

Expose every current round verb and only its supported representation choices.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75b-register-current-round-names-and-choices-01.rs"
```

---

<a id="checkpoint-75c-exercise-round-originals-and-one-transaction"></a>

### Step 5 of 11 · Exercise round originals and one transaction (`75c-exercise-round-originals-and-one-transaction`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/round_tests.rs, src/l…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Undo/Redo</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">exact archive identity</text>
</svg>
</div>

Check current original source geometry, signed frames, temporary outlines and archive identity through the same Editor routes used by the learner.

> **Check:** *Does Undo remove only the tessellated display?* — No. Completion adds one original object in one transaction. Undo removes that object; Redo restores it without rebuilding the tool answers.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/round_tests.rs`

Copy integrated current round representation, frame, preview, invalid-input, history and archive identity fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the maintained round component.

After that block, append this supplied code:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-02.rs" skip=28
```

**Result after `75c-exercise-round-originals-and-one-transaction`:** An original Cylinder, Cone, Sphere and Torus display on white. Capsule and Quad Sphere create original meshes. Invalid dimensions keep accepted answers and Redo; completion is one transaction.

---

<a id="checkpoint-75za-build-original-planar-circles"></a>

### Step 6 of 11 · Build original planar circles (`75za-build-original-planar-circles`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_circle.rs, s…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Reuse the checked frame and positive-radius helpers from 73h–74. Ask for a center and radius, draw the pending ring, and build an exact kernel circle in the chosen plane. Preserve its editable NURBS source instead of saving the preview polygon.

> **Check:** *Why keep the exact circle after drawing its ring?* — The ring is only a display sample. The original rational curve retains its exact radius and editable source data.

### 1. `src/freeform_circle.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

After that block, type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-2.rs" skip=30
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_circle.rs`

Supply only the unchanged original curve validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/75za-build-original-planar-circles-3.rs" skip=1409
```

**Result after `75za-build-original-planar-circles`:** The exact planar circle source helper is prepared; the current browser drawing is unchanged.

---

<a id="checkpoint-75zb-build-original-planar-ellipses"></a>

### Step 7 of 11 · Build original planar ellipses (`75zb-build-original-planar-ellipses`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_ellipse.rs,…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Reuse the same checked frame and radius owner. Orient the first ellipse axis from a clicked point, measure the second radius across that axis, and transform the exact kernel ellipse into the drawing plane. Numeric answers retain the existing axes.

> **Check:** *Why measure the second clicked radius across the first axis?* — Its perpendicular distance sets the second semiaxis. Its distance from the center would incorrectly include movement along the first axis.

### 1. `src/freeform_ellipse.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

After that block, type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-2.rs" skip=26
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_ellipse.rs`

Supply only the unchanged original curve validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/75zb-build-original-planar-ellipses-3.rs" skip=2204
```

**Result after `75zb-build-original-planar-ellipses`:** The planar ellipse helper can use separate original radii before command input is connected.

---

<a id="checkpoint-75zc-build-exact-rational-arc-spans"></a>

### Step 8 of 11 · Build exact rational arc spans (`75zc-build-exact-rational-arc-spans`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_arc.rs, src/…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Build the original circular arc from rational quadratic spans. Set repeated knots and alternating endpoint/middle weights, then reuse the checked frame to place the original curve. Keep the sampled pending arc separate from the exact source.

> **Check:** *Why does the middle control use a cosine weight?* — Its weighted position makes the quadratic span follow a circular arc. Moving only an unweighted control point would produce a different curve.

### 1. `src/freeform_arc.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

After that block, type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-2.rs" skip=27
```

**Result after `75zc-build-exact-rational-arc-spans`:** Exact rational arc spans are prepared for the original curve recipe.

---

<a id="checkpoint-75zd-read-original-arcs-from-center-and-angle"></a>

### Step 9 of 11 · Read original arcs from center and angle (`75zd-read-original-arcs-from-center-and-angle`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_arc.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Read the existing center-based arc inputs using the checked frame. Turn its first axis toward a clicked start, derive the clicked angle or accept typed degrees, and preserve the signed orientation through the original radius and angle rules.

> **Check:** *Why turn the local frame toward the picked start?* — Angle zero must lie at that original start. The same local arc builder can then place the result through the existing frame transform.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

After that block, type:

```rust
--8<-- "journey/code/75zd-read-original-arcs-from-center-and-angle-1.rs" skip=147
```

**Result after `75zd-read-original-arcs-from-center-and-angle`:** The center-and-angle arc reader compiles before its browser tool connection.

---

<a id="checkpoint-75ze-read-three-point-arcs-and-connect-the-recipe"></a>

### Step 10 of 11 · Read three-point arcs and connect the recipe (`75ze-read-three-point-arcs-and-connect-the-recipe`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_arc.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Complete the existing arc recipe with its two input modes. Derive the original circle through the start, an intermediate point and the end, reject a degenerate chord or bulge, and reuse the rational span builder. Keep all input handling under the shared question owner.

> **Check:** *Why use the intermediate point when choosing the arc?* — Two endpoints alone leave multiple circular arcs. The intermediate point determines which circle and which route through it the original curve must follow.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-1.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/freeform_arc.rs`

Supply only the unchanged original curve validation fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-2.rs" skip=6255
```

**Result after `75ze-read-three-point-arcs-and-connect-the-recipe`:** Three-point input completes the arc source recipe; the browser command route follows separately.

---

<a id="checkpoint-75zf-build-original-parabola-previews-and-sources"></a>

### Step 11 of 11 · Build original parabola previews and sources (`75zf-build-original-parabola-previews-and-sources`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/freeform_parabola.rs,…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original exact curve</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived preview</text>
</svg>
</div>

Read the prepared start, apex and end from the checked local frame. Derive the quadratic control that passes through the apex halfway, sample it for the pending guide, and build the exact kernel parabola from the three original points.

> **Check:** *Why is the apex different from the middle Bezier control?* — The apex is a point on the curve at halfway. The control must be derived from that constraint rather than placed directly at the apex.

### 1. `src/freeform_parabola.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

After that block, type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-2.rs" skip=23
```

**Result after `75zf-build-original-parabola-previews-and-sources`:** Parabola preview and original-source helpers share the prepared parameters.

---

## 3. Build, Verify & Save

```sh
npm --prefix ../session_tests run course -- dependencies 14.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 14.5
npm --prefix ../session_tests run course -- save 14.5
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [14.6 · Exact parabola recipes and editable freeform controls](../../consolidation/76-copy-exact-planar-curve-definitions.md).
