# 14.5 · Round solids and exact planar conics

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 14.5** (Redraft `8.5`) · **11 checkpoints (`75-reuse-current-round-solid-builders` → `75zf-build-original-parabola-previews-and-sources`)** · **Study time: 5.5–11 h**

> **What You Will Learn & Build:** Collect a center and two radii with the existing question owner. By the final checkpoint (`75zf-build-original-parabola-previews-and-sources`), you will read the prepared start, apex and end from the checked local frame.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 14.5 Execution Flow: Round solids and exact planar conics</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (11 Checkpoints: 75-reuse-current-round-solid-builders → 75zf-build-original-parabola-previews-and-sources)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">existing questions</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">checked local frame</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">exact kernel solid</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">shared questions</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">and frame</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">original exact</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">curve</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 14.5:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/round_torus.rs  ·  src/lib.rs  ·  src/round_capsule.rs  ·  src/round_quad_sphere.rs  ·  src/primitive_registry.rs  ·  src/round_tests.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">11 Checkpoints: 75-reuse-current-round-solid-builders → 75zf-build-original-parabola-previews-and-sources — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 8 Precision CAD Frames, Solid Primitives, Surfacing & Sub-Object Deformation (Chapters 14–16)</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How double-precision coordinate frames (Plane), parametric solids, Loft/Sweep/Extrude surfaces, and Gumball control edits work.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Orthonormal Frames (Ch 14)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Plane { origin: Vec3d, x, y, z }</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Rejects non-finite / collinear axes</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• Basis change: World &lt;-&gt; Local Plane</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Exact Mirror, Rotate3D, Orient,</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  and Align frame transformations</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="295" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Solids &amp; Surfacing (Ch 15–16)</text>
    <text x="307" y="108" font-size="10.5" fill="#111111">• Parametric Box, Cylinder, Sphere, Cone</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#111111">• Extrude, Revolve, Loft, Sweep, Planar</text>
    <text x="307" y="144" font-size="10.5" fill="#111111">• Curve Split, Trim, Extend, Join, Fillet</text>
    <text x="307" y="162" font-size="10.5" fill="#111111">• Every operation commits an atomic</text>
    <text x="307" y="178" font-size="10.5" fill="#111112">  session_rust kernel transaction</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="592" y="68" width="228" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Rational Control-Point Deform</text>
    <text x="604" y="108" font-size="10.5" fill="#111111">• Pick NURBS control point or vertex</text>
    <text x="604" y="126" font-size="10.5" font-weight="600" fill="#111111">• Preserve homogeneous weight w</text>
    <text x="604" y="142" font-size="10.5" font-weight="600" fill="#111111">  on rational curve control drags!</text>
    <text x="604" y="162" font-size="10.5" fill="#111111">• Rebuild derived GPU tessellation</text>
    <text x="604" y="178" font-size="10.5" fill="#111111">  in-place while keeping Object ID</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Authoritative production source (`session_viewer/src`):** [app/walk/sheet.rs](../../../../src/app/walk/sheet.rs), [app/sheet_query.rs](../../../../src/app/sheet_query.rs), [app/stream.rs](../../../../src/app/stream.rs), [app/cplane.rs](../../../../src/app/cplane.rs), [app/coords.rs](../../../../src/app/coords.rs), [app/command/tool/shape.rs](../../../../src/app/command/tool/shape.rs), [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs), [app/deform.rs](../../../../src/app/deform.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`75-reuse-current-round-solid-builders`](#checkpoint-75-reuse-current-round-solid-builders) | Collect a center and two radii with the existing question owner. | `src/round_torus.rs`, `src/lib.rs` | `existing questions → checked local frame → exact kernel solid` |
| **2** | [`75aa-build-original-capsules-with-shared-dimensions`](#checkpoint-75aa-build-original-capsules-with-shared-dimensions) | Reuse the cylinder’s existing center, radius and signed-height questions. | `src/round_capsule.rs`, `src/lib.rs` | `existing questions → checked local frame → exact kernel solid` |
| **3** | [`75a-retain-mesh-only-round-families`](#checkpoint-75a-retain-mesh-only-round-families) | Ask the kernel for six original quad-sphere patches. | `src/round_quad_sphere.rs`, `src/lib.rs` | `existing questions → checked local frame → exact kernel solid` |
| **4** | [`75b-register-current-round-names-and-choices`](#checkpoint-75b-register-current-round-names-and-choices) | Register current round names and choices. | `src/primitive_registry.rs` | `registered verb and option → shared questions → one original commit` |
| **5** | [`75c-exercise-round-originals-and-one-transaction`](#checkpoint-75c-exercise-round-originals-and-one-transaction) | Exercise round originals and one transaction. | `src/round_tests.rs`, `src/lib.rs` | `current round tool → original geometry → Undo/Redo → exact archive identity` |
| **6** | [`75za-build-original-planar-circles`](#checkpoint-75za-build-original-planar-circles) | Reuse the checked frame and positive-radius helpers from 73h–74. | `src/freeform_circle.rs`, `src/lib.rs` | `shared questions and frame → original exact curve → derived preview` |
| **7** | [`75zb-build-original-planar-ellipses`](#checkpoint-75zb-build-original-planar-ellipses) | Reuse the same checked frame and radius owner. | `src/freeform_ellipse.rs`, `src/lib.rs` | `shared questions and frame → original exact curve → derived preview` |
| **8** | [`75zc-build-exact-rational-arc-spans`](#checkpoint-75zc-build-exact-rational-arc-spans) | Build the original circular arc from rational quadratic spans. | `src/freeform_arc.rs`, `src/lib.rs` | `shared questions and frame → original exact curve → derived preview` |
| **9** | [`75zd-read-original-arcs-from-center-and-angle`](#checkpoint-75zd-read-original-arcs-from-center-and-angle) | Read the existing center-based arc inputs using the checked frame. | `src/freeform_arc.rs` | `shared questions and frame → original exact curve → derived preview` |
| **10** | [`75ze-read-three-point-arcs-and-connect-the-recipe`](#checkpoint-75ze-read-three-point-arcs-and-connect-the-recipe) | Complete the existing arc recipe with its two input modes. | `src/freeform_arc.rs` | `shared questions and frame → original exact curve → derived preview` |
| **11** | [`75zf-build-original-parabola-previews-and-sources`](#checkpoint-75zf-build-original-parabola-previews-and-sources) | Read the prepared start, apex and end from the checked local frame. | `src/freeform_parabola.rs`, `src/lib.rs` | `shared questions and frame → original exact curve → derived preview` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-75-reuse-current-round-solid-builders"></a>

### Step 1 of 11 · Build original tori with checked radii (`75-reuse-current-round-solid-builders`)

- **Goal:** Collect a center and two radii with the existing question owner.
- **Data Flow Trace:** `existing questions → checked local frame → exact kernel solid`
- **Files You Touch:** `src/round_torus.rs` (*Type the whole original solid builder using the existing frame and question helpers.*) · `src/lib.rs` (*Register this shared original solid builder.*)

Collect a center and two radii with the existing question owner. Reject an invalid radius pair before building the torus. Draw its pending rings using the checked frame, then ask the kernel for the chosen original representation.

> **Self-Check Question:** *Why check both torus radii before building?*  
> **Answer:** Reject an invalid ring before publishing any source geometry or changing undo history. The existing question owner can retain the answers for correction.

- **What You Should See:** The original Box, Pyramid and Block With Hole remain visible while the round family connects. The final exercise displays a Cylinder, Cone, Sphere and Torus.
- **Hands-On Lab Experiment:** Trace a valid torus radius pair and an invalid pair. Compare the pending guide with the final original surface.

<details>
<summary>Implementation 75-reuse-current-round-solid-builders: Build original tori with checked radii</summary>

Collect a center and two radii with the existing question owner.

Data flow: existing questions → checked local frame → exact kernel solid.

### 1. `src/round_torus.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_torus.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, positive,
};
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Torus",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, major radius, then minor radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Major radius")),
        2 => Some(Ask::size("Minor radius")),
        _ => None,
    }
}

/// The minor radius is a click's distance from the major circle, below the major radius.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(major) = answers.get(1) {
        part.sizes
            .push(positive(frame.size(major), "The major radius")?);
    }

    if let Some(minor) = answers.get(2) {
        let major = part.sizes[0];
        let minor = match minor {
            Answer::Number(value) => *value,
            Answer::Point(_) => (frame.size(minor) - major).abs(),
        };

        if minor >= major {
            return Err("The minor radius must be below the major radius".into());
        }

        part.sizes.push(positive(minor, "The minor radius")?);
    }

    Ok(part)
}

/// The major circle, then the inner, outer, upper and lower rings.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [major] => vec![frame.ring(major, 0.0)],
        [major, minor] => vec![
            frame.ring(major - minor, 0.0),
            frame.ring(major + minor, 0.0),
            frame.ring(major, minor),
            frame.ring(major, -minor),
        ],
        _ => Vec::new(),
    }
}

/// A kernel torus about the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [major, minor] = part.sizes[..] else {
        return Err("Torus needs two radii".into());
    };
    Ok(shape::solid(
        BRep::create_torus(major, minor),
        option,
        &part.frame.to_xform(),
        "torus",
    ))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75-reuse-current-round-solid-builders-typed-round_torus-fixtures.rs" skip=2228
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_sphere;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75-reuse-current-round-solid-builders-08.rs" skip=23
```

</details>

<a id="checkpoint-75aa-build-original-capsules-with-shared-dimensions"></a>

### Step 2 of 11 · Build original capsules with shared dimensions (`75aa-build-original-capsules-with-shared-dimensions`)

- **Goal:** Reuse the cylinder’s existing center, radius and signed-height questions.
- **Data Flow Trace:** `existing questions → checked local frame → exact kernel solid`
- **Files You Touch:** `src/round_capsule.rs` (*Type the whole original solid builder using the existing frame and question helpers.*) · `src/lib.rs` (*Register this shared original solid builder.*)

Reuse the cylinder’s existing center, radius and signed-height questions. Require a height larger than the diameter, draw the two cap junctions, and build the kernel capsule around its local central axis. Transform that original mesh through the same checked frame.

> **Self-Check Question:** *Why must capsule height exceed its diameter?*  
> **Answer:** The two hemispherical caps need a positive central span. Reuse the existing dimensions but reject overlapping caps before building.

- **What You Should See:** The capsule builder shares one dimension contract across preview and source preparation.
- **Hands-On Lab Experiment:** Trace heights above and equal to twice the radius. Check where the two cap centers lie before transforming the source mesh.

<details>
<summary>Implementation 75aa-build-original-capsules-with-shared-dimensions: Build original capsules with shared dimensions</summary>

Reuse the cylinder’s existing center, radius and signed-height questions.

Data flow: existing questions → checked local frame → exact kernel solid.

### 1. `src/round_capsule.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_capsule.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ MESH_ONLY, Part, Shape};
use crate::round_cylinder as cylinder;
use session_rust::{Geometry, Line, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Capsule",
    options: MESH_ONLY,
    upfront: false,
    ask: cylinder::ask,
    read,
    outline,
    build,
};

/// The cylinder's radius and height, taller than the two caps.
fn read(frame: &shape::Frame, answers: &[shape::Answer], option: &str) -> Result<Part, String> {
    let part = cylinder::read(frame, answers, option)?;

    if let [radius, height] = part.sizes[..]
        && height.abs() <= 2.0 * radius
    {
        return Err("The height must exceed the diameter".into());
    }

    Ok(part)
}

/// The base ring, then the rings where the caps begin and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius, height] = part.sizes[..] else {
        return part
            .sizes
            .first()
            .map_or_else(Vec::new, |radius| vec![part.frame.ring(*radius, 0.0)]);
    };
    let (frame, height) = part.frame.upright(height);
    let (low, high) = (radius, height - radius);
    let mut wires = vec![frame.ring(radius, low), frame.ring(radius, high)];

    for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
        wires.push(vec![
            frame.at(0.0, 0.0, 0.0),
            frame.at(u * radius, v * radius, low),
            frame.at(u * radius, v * radius, high),
            frame.at(0.0, 0.0, height),
        ]);
    }

    wires
}

/// The kernel capsule on the axis, its caps ending at the base and the height.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Capsule needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let axis = Line::from_points(
        &Point::new(0.0, 0.0, radius),
        &Point::new(0.0, 0.0, height - radius),
    );
    let mut mesh = Primitives::capsule_mesh(&axis, radius);
    mesh.transform(&frame.to_xform());
    mesh.name = "capsule".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75aa-build-original-capsules-with-shared-dimensions-typed-round_capsule-fixtures.rs" skip=2191
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_torus;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-02.rs" skip=22
```

</details>

<a id="checkpoint-75a-retain-mesh-only-round-families"></a>

### Step 3 of 11 · Mesh original quad-sphere patches (`75a-retain-mesh-only-round-families`)

- **Goal:** Ask the kernel for six original quad-sphere patches.
- **Data Flow Trace:** `existing questions → checked local frame → exact kernel solid`
- **Files You Touch:** `src/round_quad_sphere.rs` (*Type the whole original solid builder using the existing frame and question helpers.*) · `src/lib.rs` (*Register this shared original solid builder.*)

Ask the kernel for six original quad-sphere patches. Sample each patch with the existing quad mesher, then reuse the welding helper from 73h–74 to connect their seams. Transform the closed mesh through the checked frame and retain the mesh-only representation.

> **Self-Check Question:** *Why weld the six patch meshes?*  
> **Answer:** Adjacent patches meet along common seams. Reuse the existing weld helper so the result shares vertices and forms one closed mesh.

- **Hands-On Lab Experiment:** Count six patches with eight by eight quads each. Trace shared seam vertices through the existing weld helper.

<details>
<summary>Implementation 75a-retain-mesh-only-round-families: Mesh original quad-sphere patches</summary>

Ask the kernel for six original quad-sphere patches.

Data flow: existing questions → checked local frame → exact kernel solid.

### 1. `src/round_quad_sphere.rs`

Type the whole original solid builder using the existing frame and question helpers.

Create the file and type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/round_quad_sphere.rs`

Supply only the unchanged original representation and topology fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape as shape;
use crate::primitive_shape::{ MESH_ONLY, Part, Shape};
use session_rust::{Geometry, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Quad Sphere",
    options: MESH_ONLY,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

const QUADS: usize = 8; // per patch side

/// Six kernel patches meshed and welded into one closed mesh.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Quad Sphere needs a radius".into());
    };
    let patches: Vec<_> = Primitives::quad_sphere(0.0, 0.0, 0.0, radius)
        .iter()
        .map(|patch| Primitives::quad_mesh(patch, QUADS, QUADS))
        .collect();
    let mut mesh = shape::to_mesh(&patches);
    mesh.transform(&part.frame.to_xform());
    mesh.name = "quad sphere".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75a-retain-mesh-only-round-families-typed-round_quad_sphere-fixtures.rs" skip=987
```

</details>

### 3. `src/lib.rs`

Register this shared original solid builder.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod round_capsule;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75a-retain-mesh-only-round-families-04.rs" skip=24
```

</details>

<a id="checkpoint-75b-register-current-round-names-and-choices"></a>

### Step 4 of 11 · Register current round names and choices (`75b-register-current-round-names-and-choices`)

- **Goal:** Register current round names and choices.
- **Data Flow Trace:** `registered verb and option → shared questions → one original commit`
- **Files You Touch:** `src/primitive_registry.rs` (*Expose every current round verb and only its supported representation choices.*)

Connect each current name to its maintained definition and available representation. The shared input owner already handles defaults, cursor previews, cancellation and atomic invalid input.

> **Self-Check Question:** *What happens after an invalid torus minor radius?*  
> **Answer:** The accepted center and major radius remain. Correct the minor radius or cancel; no scene object or history transaction is created by the error.

- **Hands-On Lab Experiment:** Type Torus, w0,0,0, 2, 3, then0.5. Compare Cylinder Mesh with default Cylinder. Try Capsule Brep and Quad Sphere Brep and verify both fail before replacing another active tool.

<details>
<summary>Implementation 75b-register-current-round-names-and-choices: Register current round names and choices</summary>

Register current round names and choices.

Data flow: registered verb and option → shared questions → one original commit.

### 1. `src/primitive_registry.rs`

Expose every current round verb and only its supported representation choices.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Dodecahedron Brep",&crate::primitive_dodecahedron::SHAPE,0),
    ("Dodecahedron Mesh",&crate::primitive_dodecahedron::SHAPE,1),
    ("bo",&crate::primitive_box::SHAPE,0),
];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75b-register-current-round-names-and-choices-01.rs"
```

</details>

<a id="checkpoint-75c-exercise-round-originals-and-one-transaction"></a>

### Step 5 of 11 · Exercise round originals and one transaction (`75c-exercise-round-originals-and-one-transaction`)

- **Goal:** Exercise round originals and one transaction.
- **Data Flow Trace:** `current round tool → original geometry → Undo/Redo → exact archive identity`
- **Files You Touch:** `src/round_tests.rs` (*Copy integrated current round representation, frame, preview, invalid-input, history and archive identity fixtures.*) · `src/lib.rs` (*Register the maintained round component.*)

Check current original source geometry, signed frames, temporary outlines and archive identity through the same Editor routes used by the learner.

> **Self-Check Question:** *Does Undo remove only the tessellated display?*  
> **Answer:** No. Completion adds one original object in one transaction. Undo removes that object; Redo restores it without rebuilding the tool answers.

- **What You Should See:** An original Cylinder, Cone, Sphere and Torus display on white. Capsule and Quad Sphere create original meshes. Invalid dimensions keep accepted answers and Redo; completion is one transaction.
- **Hands-On Lab Experiment:** Type Close, CPlane XY, Cylinder w-2,0,0 0.45 1, Cone w-0.7,0,0 0.45 1, Sphere w0.7,0,0.5 0.5, Torus w2,0,0.5 0.5 0.15, View Isometric and Fit. Then create Capsule and Quad Sphere, inspect their Mesh output, Undo/Redo, Save and reopen.

<details>
<summary>Implementation 75c-exercise-round-originals-and-one-transaction: Exercise round originals and one transaction</summary>

Exercise round originals and one transaction.

Data flow: current round tool → original geometry → Undo/Redo → exact archive identity.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/round_tests.rs`

Copy integrated current round representation, frame, preview, invalid-input, history and archive identity fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the maintained round component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod round_quad_sphere;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75c-exercise-round-originals-and-one-transaction-02.rs" skip=28
```

</details>

</details>

<a id="checkpoint-75za-build-original-planar-circles"></a>

### Step 6 of 11 · Build original planar circles (`75za-build-original-planar-circles`)

- **Goal:** Reuse the checked frame and positive-radius helpers from 73h–74.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_circle.rs` (*Type the whole original curve helpers using the existing frame and geometry owners.*) · `src/lib.rs` (*Compile this shared original curve module.*)

Reuse the checked frame and positive-radius helpers from 73h–74. Ask for a center and radius, draw the pending ring, and build an exact kernel circle in the chosen plane. Preserve its editable NURBS source instead of saving the preview polygon.

> **Self-Check Question:** *Why keep the exact circle after drawing its ring?*  
> **Answer:** The ring is only a display sample. The original rational curve retains its exact radius and editable source data.

- **What You Should See:** The exact planar circle source helper is prepared; the current browser drawing is unchanged.
- **Hands-On Lab Experiment:** Trace one radius through the pending ring, kernel circle and frame transform.

<details>
<summary>Implementation 75za-build-original-planar-circles: Build original planar circles</summary>

Reuse the checked frame and positive-radius helpers from 73h–74.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_circle.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod round_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75za-build-original-planar-circles-2.rs" skip=30
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_circle.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, CURVE, Frame, Part, Shape, positive};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Circle",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, then radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Radius")),
        _ => None,
    }
}

/// The radius in the plane.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes.push(positive(frame.size(radius), "The radius")?);
    }

    Ok(part)
}

/// The circle.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    part.sizes
        .iter()
        .map(|radius| part.frame.ring(*radius, 0.0))
        .collect()
}

/// The kernel circle, placed in the plane.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Nurbs Curve Circle needs a radius".into());
    };
    let mut curve = Primitives::circle(0.0, 0.0, 0.0, radius);
    curve.transform(&part.frame.to_xform());
    curve.name = "circle".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75za-build-original-planar-circles-3.rs" skip=1409
```

</details>

</details>

<a id="checkpoint-75zb-build-original-planar-ellipses"></a>

### Step 7 of 11 · Build original planar ellipses (`75zb-build-original-planar-ellipses`)

- **Goal:** Reuse the same checked frame and radius owner.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_ellipse.rs` (*Type the whole original curve helpers using the existing frame and geometry owners.*) · `src/lib.rs` (*Compile this shared original curve module.*)

Reuse the same checked frame and radius owner. Orient the first ellipse axis from a clicked point, measure the second radius across that axis, and transform the exact kernel ellipse into the drawing plane. Numeric answers retain the existing axes.

> **Self-Check Question:** *Why measure the second clicked radius across the first axis?*  
> **Answer:** Its perpendicular distance sets the second semiaxis. Its distance from the center would incorrectly include movement along the first axis.

- **What You Should See:** The planar ellipse helper can use separate original radii before command input is connected.
- **Hands-On Lab Experiment:** Compare a numeric first radius with a clicked first-axis endpoint. Trace the second click in the turned frame.

<details>
<summary>Implementation 75zb-build-original-planar-ellipses: Build original planar ellipses</summary>

Reuse the same checked frame and radius owner.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_ellipse.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_circle;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zb-build-original-planar-ellipses-2.rs" skip=26
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_ellipse.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, CURVE, Frame, Part, Shape, positive};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Ellipse",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Center, the first axis end, then the second radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("First axis end or radius")),
        2 => Some(Ask::size("Second radius")),
        _ => None,
    }
}

/// A click turns the first axis toward it; the second radius is a click's distance from that axis.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(first) = answers.get(1) {
        if let Answer::Point(end) = first {
            let [u, v, _] = frame.local(end);
            part.frame = frame.turned(u, v);
        }

        part.sizes
            .push(positive(frame.size(first), "The first radius")?);
    }

    if let Some(second) = answers.get(2) {
        let radius = match second {
            Answer::Number(value) => *value,
            Answer::Point(p) => part.frame.local(p)[1].abs(),
        };
        part.sizes.push(positive(radius, "The second radius")?);
    }

    Ok(part)
}

/// The first axis, then the ellipse.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [first] => vec![vec![frame.at(-first, 0.0, 0.0), frame.at(first, 0.0, 0.0)]],
        [first, second] => vec![frame.oval(first, second, 0.0)],
        _ => Vec::new(),
    }
}

/// The kernel ellipse, its first radius along the first axis.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [first, second] = part.sizes[..] else {
        return Err("Nurbs Curve Ellipse needs two radii".into());
    };
    let mut curve = Primitives::ellipse(0.0, 0.0, 0.0, first, second);
    curve.transform(&part.frame.to_xform());
    curve.name = "ellipse".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75zb-build-original-planar-ellipses-3.rs" skip=2204
```

</details>

</details>

<a id="checkpoint-75zc-build-exact-rational-arc-spans"></a>

### Step 8 of 11 · Build exact rational arc spans (`75zc-build-exact-rational-arc-spans`)

- **Goal:** Build the original circular arc from rational quadratic spans.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_arc.rs` (*Type the whole original curve helpers using the existing frame and geometry owners.*) · `src/lib.rs` (*Compile this shared original curve module.*)

Build the original circular arc from rational quadratic spans. Set repeated knots and alternating endpoint/middle weights, then reuse the checked frame to place the original curve. Keep the sampled pending arc separate from the exact source.

> **Self-Check Question:** *Why does the middle control use a cosine weight?*  
> **Answer:** Its weighted position makes the quadratic span follow a circular arc. Moving only an unweighted control point would produce a different curve.

- **What You Should See:** Exact rational arc spans are prepared for the original curve recipe.
- **Hands-On Lab Experiment:** Trace the knots and middle weight of a valid 90-degree arc. The input recipe remains preparatory.

<details>
<summary>Implementation 75zc-build-exact-rational-arc-spans: Build exact rational arc spans</summary>

Build the original circular arc from rational quadratic spans.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_ellipse;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zc-build-exact-rational-arc-spans-2.rs" skip=27
```

</details>

<a id="checkpoint-75zd-read-original-arcs-from-center-and-angle"></a>

### Step 9 of 11 · Read original arcs from center and angle (`75zd-read-original-arcs-from-center-and-angle`)

- **Goal:** Read the existing center-based arc inputs using the checked frame.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_arc.rs` (*Extend this original owner with the day’s complete input helpers.*)

Read the existing center-based arc inputs using the checked frame. Turn its first axis toward a clicked start, derive the clicked angle or accept typed degrees, and preserve the signed orientation through the original radius and angle rules.

> **Self-Check Question:** *Why turn the local frame toward the picked start?*  
> **Answer:** Angle zero must lie at that original start. The same local arc builder can then place the result through the existing frame transform.

- **What You Should See:** The center-and-angle arc reader compiles before its browser tool connection.
- **Hands-On Lab Experiment:** Trace a clicked start and a typed clockwise angle. These input helpers remain preparatory until the recipe is complete.

<details>
<summary>Implementation 75zd-read-original-arcs-from-center-and-angle: Read original arcs from center and angle</summary>

Read the existing center-based arc inputs using the checked frame.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::primitive_shape::{ Answer, Ask, Frame, Part, RING, Shape, positive};
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zd-read-original-arcs-from-center-and-angle-1.rs" skip=147
```

</details>

<a id="checkpoint-75ze-read-three-point-arcs-and-connect-the-recipe"></a>

### Step 10 of 11 · Read three-point arcs and connect the recipe (`75ze-read-three-point-arcs-and-connect-the-recipe`)

- **Goal:** Complete the existing arc recipe with its two input modes.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_arc.rs` (*Extend this original owner with the day’s complete input helpers.*)

Complete the existing arc recipe with its two input modes. Derive the original circle through the start, an intermediate point and the end, reject a degenerate chord or bulge, and reuse the rational span builder. Keep all input handling under the shared question owner.

> **Self-Check Question:** *Why use the intermediate point when choosing the arc?*  
> **Answer:** Two endpoints alone leave multiple circular arcs. The intermediate point determines which circle and which route through it the original curve must follow.

- **What You Should See:** Three-point input completes the arc source recipe; the browser command route follows separately.
- **Hands-On Lab Experiment:** Compare Center and 3 Points. Trace a degenerate chord before any original geometry is published.

<details>
<summary>Implementation 75ze-read-three-point-arcs-and-connect-the-recipe: Read three-point arcs and connect the recipe</summary>

Complete the existing arc recipe with its two input modes.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_arc.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/freeform_arc.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, Frame, Part, RING, Shape, positive};
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Arc",
    options: &[
        ("Center", "Center"),
        ("3 Points", "3Points"),
        ("Cancel", "Escape"),
    ],
    upfront: true,
    ask,
    read,
    outline,
    build,
};

/// Center, start, end; or start, end, a point on the arc.
fn ask(answers: &[Answer], _part: &Part, option: &str) -> Option<Ask> {
    match (option, answers.len()) {
        ("Center", 0) => Some(Ask::point("Center")),
        ("Center", 1) => Some(Ask::size("Start or radius")),
        ("Center", 2) => Some(Ask::size("End or angle in degrees")),
        (_, 0) => Some(Ask::point("Start")),
        (_, 1) => Some(Ask::size("End or chord length")),
        (_, 2) => Some(Ask::size("Point on the arc or bulge")),
        _ => None,
    }
}

/// The circle's frame, x toward the start, and the radius and angle in degrees counterclockwise.
fn read(frame: &Frame, answers: &[Answer], option: &str) -> Result<Part, String> {
    match option {
        "Center" => read_center(frame, answers),
        _ => read_points(frame, answers),
    }
}

/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
    Ok(part)
}

/// The circle through start, a point on it and end, running from start past that point.
fn read_points(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };
    let start = frame.origin.clone();
    let end = match end {
        Answer::Number(length) => frame.at(*length, 0.0, 0.0),
        Answer::Point(end) => end.clone(),
    };
    let chord = positive(start.distance(&end, None), "The chord")?;
    let [u, v, _] = frame.local(&end);
    part.frame = frame.turned(u, v);
    part.sizes.push(chord);
    let Some(middle) = answers.get(2) else {
        return Ok(part);
    };
    let middle = match middle {
        Answer::Number(bulge) => part.frame.at(chord * 0.5, *bulge, 0.0), // to the left of start → end
        Answer::Point(p) => p.clone(),
    };
    let (ab, ac) = (&end - &start, &middle - &start);
    let normal = ac.cross(&ab); // start, middle, end run counterclockwise about it

    if normal.magnitude() <= 1e-9 * ab.magnitude() * ac.magnitude() {
        return Err("The three points are in a line".into());
    }

    let offset = &(&normal.cross(&ab) * ac.dot(&ac)) + &(&ac.cross(&normal) * ab.dot(&ab));
    let center = &start - (&offset * (0.5 / normal.dot(&normal)));
    let radius = center.distance(&start, None);
    let x = (&start - &center).normalized();
    let z = normal.normalized();
    let circle = Frame {
        origin: center,
        y: z.cross(&x),
        x,
        z,
    };
    let [u, v, _] = circle.local(&end);
    let degrees = v.atan2(u).to_degrees();
    part.frame = circle;
    part.sizes = vec![
        radius,
        if degrees <= 0.0 {
            degrees + 360.0
        } else {
            degrees
        },
    ];
    Ok(part)
}

/// The radius or chord as a line, then the arc.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [length] => vec![vec![frame.at(0.0, 0.0, 0.0), frame.at(length, 0.0, 0.0)]],
        [radius, degrees] => {
            let count = ((RING as f64 * degrees / 360.0).ceil() as usize).max(2);
            let arc = (0..=count)
                .map(|i| {
                    let angle = (degrees * i as f64 / count as f64).to_radians();
                    frame.at(radius * angle.cos(), radius * angle.sin(), 0.0)
                })
                .collect();
            vec![arc]
        }
        _ => Vec::new(),
    }
}

/// The exact arc, placed in its circle's frame.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, degrees] = part.sizes[..] else {
        return Err("Nurbs Curve Arc needs a radius and an angle".into());
    };
    let mut curve = arc(radius, degrees);
    curve.transform(&part.frame.to_xform());
    curve.name = "arc".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}

/// A circular arc about z from angle 0 to `degrees`, one rational quadratic span per 90° or less.
pub fn arc(radius: f64, degrees: f64) -> NurbsCurve {
    let spans = ((degrees / 90.0 - 1e-9).ceil() as usize).max(1);
    let step = degrees.to_radians() / spans as f64;
    let weight = (step * 0.5).cos(); // of each span's middle control
    let mut curve = NurbsCurve::new(3, true, 3, 2 * spans + 1);

    for i in 0..2 * spans + 2 {
        curve.set_nurbsknot(i, (i / 2) as f64);
    }

    for i in 0..=2 * spans {
        let angle = step * 0.5 * i as f64;
        let w = if i % 2 == 1 { weight } else { 1.0 };
        let distance = radius / w; // the middle control lies outside the circle
        curve.set_cv_4d(
            i,
            distance * angle.cos() * w,
            distance * angle.sin() * w,
            0.0,
            w,
        );
    }

    curve
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/75ze-read-three-point-arcs-and-connect-the-recipe-2.rs" skip=6255
```

</details>

</details>

<a id="checkpoint-75zf-build-original-parabola-previews-and-sources"></a>

### Step 11 of 11 · Build original parabola previews and sources (`75zf-build-original-parabola-previews-and-sources`)

- **Goal:** Read the prepared start, apex and end from the checked local frame.
- **Data Flow Trace:** `shared questions and frame → original exact curve → derived preview`
- **Files You Touch:** `src/freeform_parabola.rs` (*Type the whole original curve helpers using the existing frame and geometry owners.*) · `src/lib.rs` (*Compile this shared original curve module.*)

Read the prepared start, apex and end from the checked local frame. Derive the quadratic control that passes through the apex halfway, sample it for the pending guide, and build the exact kernel parabola from the three original points.

> **Self-Check Question:** *Why is the apex different from the middle Bezier control?*  
> **Answer:** The apex is a point on the curve at halfway. The control must be derived from that constraint rather than placed directly at the apex.

- **What You Should See:** Parabola preview and original-source helpers share the prepared parameters.
- **Hands-On Lab Experiment:** Evaluate the preview at halfway and compare it with the apex. The question recipe connects in the next checkpoint.

<details>
<summary>Implementation 75zf-build-original-parabola-previews-and-sources: Build original parabola previews and sources</summary>

Read the prepared start, apex and end from the checked local frame.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_parabola.rs`

Type the whole original curve helpers using the existing frame and geometry owners.

Create the file and type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original curve module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_arc;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/75zf-build-original-parabola-previews-and-sources-2.rs" skip=23
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 75zf-build-original-parabola-previews-and-sources
npm --prefix ../session_tests run course -- check 75zf-build-original-parabola-previews-and-sources
npm --prefix ../session_tests run course -- save 75zf-build-original-parabola-previews-and-sources
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Parabola preview and original-source helpers share the prepared parameters.

- **Final Lesson Experiment:** Evaluate the preview at halfway and compare it with the apex. The question recipe connects in the next checkpoint.

Continue with [Exact parabola recipes and editable freeform controls](../../consolidation/76-copy-exact-planar-curve-definitions.md).
