# 15.4 · Two-direction networks and corner patches

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 15.4** (Redraft `8.10`) · **6 checkpoints (`80-build-a-surface-through-two-directions-of-original-curves` → `80za-read-original-projection-planes`)** · **Study time: 3–6 h**

> **What You Will Learn & Build:** Connect the complete network helpers and retain the four-boundary Coons path. By the final checkpoint (`80za-read-original-projection-planes`), you will reuse absolute coordinate parsing for named construction planes and three-point input.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 8 Visual Explanation: Orthonormal Plane Frames, Parametric Solids, Loft/Sweep & Rational Control Edits</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Orthonormal Plane (origin, X, Y, Z). Center: Loft/Extrude surfacing across section curves. Right: Rational NURBS control drag.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Orthonormal Plane Frame (f64)</text>
    <circle cx="115" cy="162" r="4" fill="#111111"/>
    <line x1="115" y1="162" x2="115" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="115" y1="162" x2="195" y2="178" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="115" y1="162" x2="175" y2="122" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="115" y="94" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Z = X × Y</text>
    <text x="206" y="182" font-size="9.5" font-weight="700" fill="#111111">X</text>
    <text x="186" y="122" font-size="9.5" font-weight="700" fill="#111111">Y</text>
    <text x="140" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Rejects collinear / non-finite axes</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="287" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="424" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Loft / Sweep / Extrude Surfacing</text>
    <path d="M 320 175 Q 370 120 420 165 L 520 135 Q 470 90 420 135 Z" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <text x="424" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Parametric Solids &amp; Surfacing Recipes</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Rational Control-Point Deform</text>
    <polyline points="615,175 665,110 745,115 790,175" fill="none" stroke="#111111" stroke-width="1" stroke-dasharray="3,3"/>
    <path d="M 615 175 C 655 125, 750 125, 790 175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="665" cy="110" r="4.5" fill="#111111"/>
    <circle cx="745" cy="115" r="4.5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Preserves rational weight w on drag</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 15.4 Pipeline Schematic: Two-direction networks and corner patches</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (6 Checkpoints: 80-build-a-surface-through-two-directions-of-original-curves → 80za-read-original-projection-planes)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original curve</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">crossings</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">ordered stations</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">sampled lofts</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">fitted surface</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">original family</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 15.4:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/network_geometry.rs  ·  src/patch_geometry.rs  ·  src/lib.rs  ·  src/gather_draft.rs  ·  src/gather_registry.rs  ·  src/commands.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">6 Checkpoints: 80-build-a-surface-through-two-directions-of-original-curves → 80za-read-original-projection-planes — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Authoritative production source (`session_viewer/src`):** [app/deform.rs](../../../../src/app/deform.rs), [state/edit.rs](../../../../src/state/edit.rs), [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs), [app/command/tool/surfacing.rs](../../../../src/app/command/tool/surfacing.rs), [app/walk/mesh.rs](../../../../src/app/walk/mesh.rs), [app/walk/brep.rs](../../../../src/app/walk/brep.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`80-build-a-surface-through-two-directions-of-original-curves`](#checkpoint-80-build-a-surface-through-two-directions-of-original-curves) | Connect the complete network helpers and retain the four-boundary Coons path. | `src/network_geometry.rs` | `original curve crossings → ordered stations → sampled lofts → fitted surface` |
| **2** | [`80a-construct-the-patch-through-four-ordered-corners`](#checkpoint-80a-construct-the-patch-through-four-ordered-corners) | Construct the patch through four ordered corners. | `src/patch_geometry.rs`, `src/lib.rs` | `four ordered world corners → area check → bilinear original surface` |
| **3** | [`80b-guide-corner-questions-without-retaining-unrelated-profiles`](#checkpoint-80b-guide-corner-questions-without-retaining-unrelated-profiles) | Guide corner questions without retaining unrelated profiles. | `src/gather_draft.rs` | `accepted corner points and cursor → temporary guide → unfinished recipe` |
| **4** | [`80c-register-network-patch-aliases-and-initial-corner-arity`](#checkpoint-80c-register-network-patch-aliases-and-initial-corner-arity) | Register network patch aliases and initial corner arity. | `src/gather_registry.rs`, `src/commands.rs` | `current name or alias → initial argument validation → shared recipe owner` |
| **5** | [`80d-exercise-networks-corner-retries-and-exact-original-archives`](#checkpoint-80d-exercise-networks-corner-retries-and-exact-original-archives) | Exercise networks corner retries and exact original archives. | `src/network_tests.rs`, `src/lib.rs` | `original network and corners → checked surfaces → one history transaction → exact Save/Open` |
| **6** | [`80za-read-original-projection-planes`](#checkpoint-80za-read-original-projection-planes) | Reuse absolute coordinate parsing for named construction planes and three-point input. | `src/project_geometry.rs`, `src/lib.rs` | `original family → pure calculation → result or refusal` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-80-build-a-surface-through-two-directions-of-original-curves"></a>

### Step 1 of 6 · Build a surface through original curve networks (`80-build-a-surface-through-two-directions-of-original-curves`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/6 Visual Trace (80-build-a-surface-through-two-directions-of-original-curves): Build a surface through original curve networks</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/network_geometry.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original curve crossings</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">ordered stations</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">sampled lofts</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">fitted surface</text>
    
</svg>
</div>

- **Goal:** Connect the complete network helpers and retain the four-boundary Coons path.
- **Data Flow Trace:** `original curve crossings → ordered stations → sampled lofts → fitted surface`
- **Files You Touch:** `src/network_geometry.rs` (*Extend this original owner with today’s complete helper.*)

Connect the complete network helpers and retain the four-boundary Coons path. Register the original recipe and validate the final fitted surface.

> **Self-Check Question:** *Why keep the four-boundary path?*  
> **Answer:** The existing kernel edge constructor directly describes that supported case. Larger grids use the Gordon construction just written.

- **What You Should See:** The complete original network-surface recipe compiles; its editing/input connection follows later.
- **Hands-On Lab Experiment:** Compare four boundaries, shuffled rulings and refused crossing graphs.

<details>
<summary>Implementation 80-build-a-surface-through-two-directions-of-original-curves: Build a surface through original curve networks</summary>

Connect the complete network helpers and retain the four-boundary Coons path.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
    for (c, column) in stacks.iter().enumerate() {
        for d in 0..column.cv_count() {
            surface.set_cv(c, d, &column.get_cv(d).unwrap_or_default());
        }
    }

    Ok(surface)
}
```

</details>

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/80-build-a-surface-through-two-directions-of-original-curves-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/network_geometry.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count, curves, diagonal};
use session_rust::nurbsknot::{CurveInterpStyle, CurveNurbsKnotStyle};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

const DENSITY: usize = 6; // samples per span between crossings
const MOST: usize = 64; // samples per direction at most

/// Where two curves cross: the parameter on each, when they come within `tolerance`.
fn crossing(
    a: &NurbsCurve,
    b: &NurbsCurve,
    boxes: (&[[f64; 3]; 2], &[[f64; 3]; 2]),
    tolerance: f64,
) -> Option<(f64, f64)> {
    let (one, two) = boxes;

    if (0..3).any(|k| one[0][k] > two[1][k] + tolerance || two[0][k] > one[1][k] + tolerance) {
        return None;
    }

    let (s, t) = a.closest_parameters_curve(b);
    (a.point_at(s).distance(&b.point_at(t), None) <= tolerance).then_some((s, t))
}

/// The low and high corners of a curve's control points.
fn bounds(curve: &NurbsCurve) -> [[f64; 3]; 2] {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in (0..curve.cv_count()).filter_map(|i| curve.get_cv(i)) {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }

    [low, high]
}

/// The curve through `points` at even parameters.
fn interpolated(points: &[Point]) -> NurbsCurve {
    NurbsCurve::create_interpolated(
        points,
        CurveNurbsKnotStyle::Uniform,
        CurveInterpStyle::Rhino,
    )
}

/// `curve` at `x` of its domain.
fn at(curve: &NurbsCurve, x: f64) -> Point {
    let (t0, t1) = curve.domain();
    curve.point_at(t0 + x * (t1 - t0))
}

/// The piecewise-linear map from even stations 0..1 to `params`, at `x`.
fn station(params: &[f64], x: f64) -> f64 {
    let last = params.len() - 1;
    let at = (x * last as f64).clamp(0.0, last as f64);
    let low = (at.floor() as usize).min(last - 1);
    let s = at - low as f64;
    params[low] + (params[low + 1] - params[low]) * s
}

/// Samples between crossings: `DENSITY` per span, at most `MOST` in all, crossings included.
fn sample_count(crossings: usize) -> usize {
    let spans = crossings - 1;
    DENSITY.min((MOST - 1) / spans).max(1) * spans + 1
}

type Crossings = Vec<Vec<Option<f64>>>;

fn crossings(input: &[NurbsCurve]) -> Crossings {
    let points: Vec<Point> = input
        .iter()
        .flat_map(|curve| (0..curve.cv_count()).filter_map(|i| curve.get_cv(i)))
        .collect();
    let tolerance = 1e-3 * diagonal(&points);
    let boxes: Vec<[[f64; 3]; 2]> = input.iter().map(bounds).collect();
    let count = input.len();
    let mut cross = vec![vec![None; count]; count];

    for i in 0..count {
        for j in i + 1..count {
            if let Some((s, t)) = crossing(&input[i], &input[j], (&boxes[i], &boxes[j]), tolerance)
            {
                cross[i][j] = Some(s);
                cross[j][i] = Some(t);
            }
        }
    }

    cross
}

fn directions(cross: &Crossings) -> Result<(Vec<usize>, Vec<usize>), String> {
    let two = "Network curves must cross in two directions";
    let count = cross.len();
    // two directions: crossing curves take opposite sides, the first picked is u
    let mut side: Vec<Option<bool>> = vec![None; count];
    side[0] = Some(false);
    let mut queue = vec![0];

    while let Some(i) = queue.pop() {
        for j in 0..count {
            if cross[i][j].is_none() {
                continue;
            }

            match side[j] {
                None => {
                    side[j] = side[i].map(|s| !s);
                    queue.push(j);
                }
                Some(s) if Some(s) == side[i] => {
                    return Err("Two curves of the same direction cross".into());
                }
                _ => {}
            }
        }
    }

    if side.iter().any(Option::is_none) {
        return Err(two.into());
    }

    let us: Vec<usize> = (0..count).filter(|&i| side[i] == Some(false)).collect();
    let vs: Vec<usize> = (0..count).filter(|&i| side[i] == Some(true)).collect();

    if us.len() < 2 || vs.len() < 2 {
        return Err(two.into());
    }

    if us
        .iter()
        .any(|&i| vs.iter().any(|&j| cross[i][j].is_none()))
    {
        return Err("Every curve must cross each curve of the other direction once".into());
    }

    Ok((us, vs))
}

fn oriented(
    input: &[NurbsCurve], us: &mut [usize], vs: &mut [usize], cross: &mut Crossings,
) -> Result<Vec<NurbsCurve>, String> {
    // v curves in order along the first u curve, u curves along the first v curve
    let first = us[0];
    let along = |row: usize, a: usize, b: usize| {
        cross[row][a]
            .unwrap()
            .total_cmp(&cross[row][b].unwrap())
            .then(a.cmp(&b)) // ties keep index order, as a stable sort would
    };
    vs.sort_unstable_by(|&a, &b| along(first, a, b));
    let lead = vs[0];
    us.sort_unstable_by(|&a, &b| along(lead, a, b));
    let mut curves: Vec<NurbsCurve> = input.to_vec();

    // each curve runs so its crossings increase
    for (own, others) in [(&*us, &*vs), (&*vs, &*us)] {
        for &i in own.iter() {
            let params: Vec<f64> = others.iter().map(|&j| cross[i][j].unwrap()).collect();

            if params.windows(2).all(|pair| pair[0] < pair[1]) {
                continue;
            }

            if !params.windows(2).all(|pair| pair[0] > pair[1]) {
                return Err("Every curve must cross each curve of the other direction once".into());
            }

            let (t0, t1) = curves[i].domain();
            curves[i].reverse();
            curves[i].set_domain(t0, t1);

            for &j in others.iter() {
                cross[i][j] = cross[i][j].map(|t| t0 + t1 - t);
            }
        }
    }

    Ok(curves)
}

struct Samples {
    across_u: Vec<Vec<Point>>,
    across_v: Vec<Vec<Point>>,
    tensor: Vec<Vec<Point>>,
}

fn sampled(
    curves: &[NurbsCurve], us: &[usize], vs: &[usize], cross: &Crossings,
) -> Samples {
    let (rows, columns) = (us.len(), vs.len());
    // u curve i crosses v curve j at s[i][j] on the u curve and r[i][j] on the v curve
    let s: Vec<Vec<f64>> = us
        .iter()
        .map(|&i| vs.iter().map(|&j| cross[i][j].unwrap()).collect())
        .collect();
    let r: Vec<Vec<f64>> = vs
        .iter()
        .map(|&j| us.iter().map(|&i| cross[j][i].unwrap()).collect())
        .collect();
    let crossings: Vec<Vec<Point>> = (0..rows)
        .map(|i| {
            (0..columns)
                .map(|j| {
                    let a = curves[us[i]].point_at(s[i][j]);
                    let b = curves[vs[j]].point_at(r[j][i]);
                    Point::new(
                        (a[0] + b[0]) / 2.0,
                        (a[1] + b[1]) / 2.0,
                        (a[2] + b[2]) / 2.0,
                    )
                })
                .collect()
        })
        .collect();
    let (kx, ky) = (sample_count(columns), sample_count(rows));
    let xs: Vec<f64> = (0..kx).map(|a| a as f64 / (kx - 1) as f64).collect();
    let ys: Vec<f64> = (0..ky).map(|b| b as f64 / (ky - 1) as f64).collect();
    // lofts across each family and the tensor through the crossings, sampled
    let across_u: Vec<Vec<Point>> = xs
        .iter()
        .map(|x| {
            let column: Vec<Point> = (0..rows)
                .map(|i| curves[us[i]].point_at(station(&s[i], *x)))
                .collect();
            let curve = interpolated(&column);
            ys.iter().map(|y| at(&curve, *y)).collect()
        })
        .collect();
    let across_v: Vec<Vec<Point>> = ys
        .iter()
        .map(|y| {
            let row: Vec<Point> = (0..columns)
                .map(|j| curves[vs[j]].point_at(station(&r[j], *y)))
                .collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    let lines: Vec<NurbsCurve> = (0..columns)
        .map(|j| {
            interpolated(
                &(0..rows)
                    .map(|i| crossings[i][j].clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let tensor: Vec<Vec<Point>> = ys
        .iter()
        .map(|y| {
            let row: Vec<Point> = lines.iter().map(|line| at(line, *y)).collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    Samples { across_u, across_v, tensor }
}

fn fitted(samples: Samples) -> Result<NurbsSurface, String> {
    let Samples { across_u, across_v, tensor } = samples;
    let (kx, ky) = (across_u.len(), across_v.len());
    // fit: rows through the samples, then columns through the rows' control points
    let fitted: Vec<NurbsCurve> = (0..ky)
        .map(|b| {
            let row: Vec<Point> = (0..kx)
                .map(|a| {
                    let (p, q, t) = (&across_u[a][b], &across_v[b][a], &tensor[b][a]);
                    Point::new(p[0] + q[0] - t[0], p[1] + q[1] - t[1], p[2] + q[2] - t[2])
                })
                .collect();
            interpolated(&row)
        })
        .collect();
    let width = fitted[0].cv_count();
    let stacks: Vec<NurbsCurve> = (0..width)
        .map(|c| {
            interpolated(
                &fitted
                    .iter()
                    .filter_map(|row| row.get_cv(c))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let (Some(u), Some(v)) = (fitted.first(), stacks.first()) else {
        return Err("The network surface could not be fitted".into());
    };
    let mut surface = NurbsSurface::new(3, false, u.order(), v.order(), width, v.cv_count());

    for i in 0..surface.nurbsknot_count(0) {
        surface.set_nurbsknot(0, i, u.nurbsknot(i).unwrap_or_default());
    }

    for i in 0..surface.nurbsknot_count(1) {
        surface.set_nurbsknot(1, i, v.nurbsknot(i).unwrap_or_default());
    }

    for (c, column) in stacks.iter().enumerate() {
        for d in 0..column.cv_count() {
            surface.set_cv(c, d, &column.get_cv(d).unwrap_or_default());
        }
    }

    Ok(surface)
}

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

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/80-build-a-surface-through-two-directions-of-original-curves-2.rs" skip=11853
```

</details>

</details>

<a id="checkpoint-80a-construct-the-patch-through-four-ordered-corners"></a>

### Step 2 of 6 · Construct the patch through four ordered corners (`80a-construct-the-patch-through-four-ordered-corners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/6 Visual Trace (80a-construct-the-patch-through-four-ordered-corners): Construct the patch through four ordered corners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/patch_geometry.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">four ordered world corners</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">area check</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">bilinear original surface</text>
    
</svg>
</div>

- **Goal:** Construct the patch through four ordered corners.
- **Data Flow Trace:** `four ordered world corners → area check → bilinear original surface`
- **Files You Touch:** `src/patch_geometry.rs` (*Type the original recipe and whole geometry helpers using the earlier shared owners.*) · `src/lib.rs` (*Register the current surface component.*)

Describe four ordered corner inputs with the existing gather recipe. Reject a degenerate area, build the exact bilinear kernel patch, and return the validated source surface as one result.

> **Self-Check Question:** *What do the corner order and u v directions mean?*  
> **Answer:** u runs from corner1 to corner2; v runs from corner1 to corner4. The patch reproduces all four corners and their mean at its center.

- **Hands-On Lab Experiment:** Inspect the nonplanar corner fixture and degenerate collinear rejection. Compare source controls with the independently derived display mesh.

<details>
<summary>Implementation 80a-construct-the-patch-through-four-ordered-corners: Construct the patch through four ordered corners</summary>

Construct the patch through four ordered corners.

Data flow: four ordered world corners → area check → bilinear original surface.

### 1. `src/patch_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/patch_geometry.rs`

Supply only the unchanged original geometry validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::checked;
use session_rust::{NurbsSurface, Point};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface 4 Points",
    chips: &[("Cancel", "Escape")],
    steps: &[
        Step::Point("corner 1 of 4"),
        Step::Point("corner 2 of 4"),
        Step::Point("corner 3 of 4"),
        Step::Point("corner 4 of 4"),
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// The bilinear patch a, b, c, d: u runs a to b, v runs a to d.
pub fn patch(corners: &[Point; 4]) -> Result<NurbsSurface, String> {
    let [a, b, c, d] = corners;
    let area = (c - a).cross(&(d - b)).magnitude();
    let size = [b, c, d]
        .iter()
        .map(|p| p.distance(a, None))
        .fold(1.0, f64::max);

    if area <= 1e-12 * size * size {
        return Err("The 4 corners enclose no area".into());
    }

    let points = [a.clone(), d.clone(), b.clone(), c.clone()];
    NurbsSurface::create(false, false, 1, 1, 2, 2, &points)
}

/// The patch through the four answered corners.
fn build(input: &Input) -> Result<Made, String> {
    let corners: Vec<Point> = (0..4).filter_map(|index| input.point(index)).collect();
    let corners: [Point; 4] = corners
        .try_into()
        .map_err(|_| "Nurbs Surface 4 Points takes 4 corner points".to_string())?;
    Ok(Made {
        geometries: vec![checked(patch(&corners)?, "surface")?],
        message: "Created a NURBS surface from 4 corners".into(),
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry-fixtures.rs" skip=1504
```

</details>

### 3. `src/lib.rs`

Register the current surface component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod network_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-02.rs" skip=27
```

</details>

<a id="checkpoint-80b-guide-corner-questions-without-retaining-unrelated-profiles"></a>

### Step 3 of 6 · Guide corner questions without retaining unrelated profiles (`80b-guide-corner-questions-without-retaining-unrelated-profiles`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/6 Visual Trace (80b-guide-corner-questions-without-retaining-unrelated-profiles): Guide corner questions without retaining unrelated profiles</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/gather_draft.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">accepted corner points and cursor</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">temporary guide</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">unfinished recipe</text>
    
</svg>
</div>

- **Goal:** Guide corner questions without retaining unrelated profiles.
- **Data Flow Trace:** `accepted corner points and cursor → temporary guide → unfinished recipe`
- **Files You Touch:** `src/gather_draft.rs` (*Guide current point questions and retain original profile copies only for curve-gathering recipes.*)

Keep accepted world points and the cursor as a temporary guide. Point-only recipes retain no unrelated selected curve input copies.

> **Self-Check Question:** *Does hovering corner4 add an original surface?*  
> **Answer:** No. The guide belongs to the pending owner. Only a checked fourth answer completes a real geometry transaction.

- **Hands-On Lab Experiment:** Begin four-corner input under CPlane XZ, hover the fourth point, then cancel. Inspect unchanged scene rows and Redo.

<details>
<summary>Implementation 80b-guide-corner-questions-without-retaining-unrelated-profiles: Guide corner questions without retaining unrelated profiles</summary>

Guide corner questions without retaining unrelated profiles.

Data flow: accepted corner points and cursor → temporary guide → unfinished recipe.

### 1. `src/gather_draft.rs`

Guide current point questions and retain original profile copies only for curve-gathering recipes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{cplane::CPlane,gather_input::{Answer,Input,Made,Picked,Progress,Recipe,Step},scene::{Scene,ObjectId}};
use session_rust::{Geometry,Point,Vector};
#[derive(Clone)]
pub struct Draft {pub recipe:&'static Recipe,pub choice:usize,pub progress:Progress,pub answers:Vec<Answer>,pub curves:Vec<(ObjectId,Picked)>,pub before:Vec<ObjectId>,pub frame:CPlane,pub typed:String,pub originals:Vec<(ObjectId,Geometry,session_rust::Xform)>}
impl Draft {
    pub fn new(recipe:&'static Recipe,text:&str,scene:&Scene,before:Vec<ObjectId>,frame:CPlane)->Result<Self,String>{
        let words=text.split_whitespace().collect::<Vec<_>>();let(choice,answers)=crate::gather_input::read(recipe,&words)?;for answer in &answers{match answer{Answer::Point(p)=>{crate::tool_coords::checked([p[0],p[1],p[2]]).map_err(str::to_owned)?;},Answer::Vector(v)if(0..3).any(|i|!v[i].is_finite()||v[i].abs()>1e12)=>return Err("Distance vector exceeds the accepted range".into()),_=>{}}}let mut result=Self{recipe,choice,progress:Progress::new(recipe.steps,answers.len()),answers,curves:Vec::new(),before:before.clone(),frame,typed:String::new(),originals:Vec::new()};
        for id in before{if scene.pick_id(id.value())==Some(id){if let Some(curve)=Self::world(scene,id){result.originals.push((id,Self::source(scene,id).unwrap(),scene.placement(id).unwrap()));result.curves.push((id,curve));}}}let rows=result.curves.iter().map(|(id,_)|id.value()).collect::<Vec<_>>();result.progress.preselect(recipe.steps,&rows);Ok(result)
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
        if !matches!(self.current(),Some(Step::Distance(_))){return Vec::new()}let Some(cursor)=cursor else{return Vec::new()};let Ok((base,_))=self.axis()else{return Vec::new()};let offset=Vector::new(cursor[0]-base[0],cursor[1]-base[1],cursor[2]-base[2]);let Ok(input)=self.input()else{return Vec::new()};input.curves.iter().flatten().map(|curve|curve.outline().into_iter().map(|point|{let p=&point+&offset;[p[0],p[1],p[2]]}).collect()).collect()
    }
}
```

</details>

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/80b-guide-corner-questions-without-retaining-unrelated-profiles-01.rs"
```

```text
--8<-- "journey/code/80b-guide-corner-questions-without-retaining-unrelated-profiles-02.rs"
```

</details>

</details>

<a id="checkpoint-80c-register-network-patch-aliases-and-initial-corner-arity"></a>

### Step 4 of 6 · Register network patch aliases and initial corner arity (`80c-register-network-patch-aliases-and-initial-corner-arity`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/6 Visual Trace (80c-register-network-patch-aliases-and-initial-corner-arity): Register network patch aliases and initial corner arity</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/gather_registry.rs, src/commands.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">current name or alias</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">initial argument validation</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">shared recipe owner</text>
    
</svg>
</div>

- **Goal:** Register network patch aliases and initial corner arity.
- **Data Flow Trace:** `current name or alias → initial argument validation → shared recipe owner`
- **Files You Touch:** `src/gather_registry.rs` (*Register current network/patch names, aliases and initial four-corner argument validation.*) · `src/commands.rs` (*Register current network/patch names, aliases and initial four-corner argument validation.*)

Extend the shared recipe registry and return initial arity errors before replacing the active drawing owner.

> **Self-Check Question:** *May an initial four-corner command contain only three points?*  
> **Answer:** The maintained parser accepts no initial corners or exactly four. Reject an initial partial list; interactive answers begin after the empty command.

- **Hands-On Lab Experiment:** Try Nurbs Surface 4 Points with three initial corners while Curve has one accepted point. Compare current network aliases and interactive corner submission.

<details>
<summary>Implementation 80c-register-network-patch-aliases-and-initial-corner-arity: Register network patch aliases and initial corner arity</summary>

Register network patch aliases and initial corner arity.

Data flow: current name or alias → initial argument validation → shared recipe owner.

### 1. `src/gather_registry.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gather_input::Recipe,editor::Action};
pub const ENTRIES:&[(&str,&Recipe)]=&[("Loft",&crate::loft_geometry::RECIPE),("Nurbs Surface Loft",&crate::nurbs_loft_geometry::RECIPE),("Nurbs Surface Sweep1",&crate::sweep1_geometry::RECIPE),("Nurbs Surface Sweep2",&crate::sweep2_geometry::RECIPE),("Extrude",&crate::extrude_geometry::RECIPE),("Nurbs Surface Revolve",&crate::revolve_geometry::RECIPE)];
pub fn command(name:&str,text:&str)->Option<Action>{let(_,recipe)=ENTRIES.iter().find(|(label,_)|*label==name)?;Some(Action::GatherBegin{recipe,text:text.into()})}
pub fn alias(line:&str)->String{let mut words=line.split_whitespace();let Some(first)=words.next()else{return line.into()};let name=match first.to_ascii_lowercase().as_str(){"nurbssurface_loft"=>"Nurbs Surface Loft","nurbssurface_sweep1"=>"Nurbs Surface Sweep1","nurbssurface_sweep2"=>"Nurbs Surface Sweep2","nurbssurface_revolve"=>"Nurbs Surface Revolve",_=>return line.into()};std::iter::once(name).chain(words).collect::<Vec<_>>().join(" ")}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-01.rs"
```

### 2. `src/commands.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

<details>
<summary>Locate the existing block</summary>

```rust
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return Ok(Command::Editor(action));}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-02.rs"
```

</details>

<a id="checkpoint-80d-exercise-networks-corner-retries-and-exact-original-archives"></a>

### Step 5 of 6 · Exercise networks corner retries and exact original archives (`80d-exercise-networks-corner-retries-and-exact-original-archives`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/6 Visual Trace (80d-exercise-networks-corner-retries-and-exact-original-archives): Exercise networks corner retries and exact original archives</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/network_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original network and</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">corners</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked surfaces</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">one history transaction</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">exact Save/Open</text>
    
</svg>
</div>

- **Goal:** Exercise networks corner retries and exact original archives.
- **Data Flow Trace:** `original network and corners → checked surfaces → one history transaction → exact Save/Open`
- **Files You Touch:** `src/network_tests.rs` (*Copy integrated current network/patch/original-source/history/archive fixtures.*) · `src/lib.rs` (*Register the current surface component.*)

Copy integrated current network/corner/history fixtures and build the rendered originals through registered commands.

> **Self-Check Question:** *Can a failed group keep its earlier picks?*  
> **Answer:** Yes. Failed completion keeps the ordered group. Add its missing crossing curves and retry; no failed result may erase Redo or the originals.

- **What You Should See:** An original surface passes through six shuffled network rulings beside a four-corner patch. Current group/orientation/area checks, pending guides, original source placement, one transaction and exact archives remain checked.
- **Hands-On Lab Experiment:** Type Close and six lines: w0,1,0 to w2,1,1; w2,0,0 to w2,2,2; w2,0,0 to w0,0,0; w1,0,0 to w1,2,1; w0,2,0 to w2,2,2; w0,0,0 to w0,2,0. Use Line before each pair. Type Select All, Nurbs Surface Network, Nurbs Surface 4 Points w3,0,0 w4,0,0 w4,1,0.3 w3,1,0, View Isometric, Select None and Fit. Try a broken group, a degenerate fourth corner, Escape, Undo/Redo and Save/Open.

<details>
<summary>Implementation 80d-exercise-networks-corner-retries-and-exact-original-archives: Exercise networks corner retries and exact original archives</summary>

Exercise networks corner retries and exact original archives.

Data flow: original network and corners → checked surfaces → one history transaction → exact Save/Open.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/network_tests.rs`

Copy integrated current network/patch/original-source/history/archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the current surface component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod patch_geometry;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-02.rs" skip=25
```

</details>

</details>

<a id="checkpoint-80za-read-original-projection-planes"></a>

### Step 6 of 6 · Read original projection planes (`80za-read-original-projection-planes`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/6 Visual Trace (80za-read-original-projection-planes): Read original projection planes</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/project_geometry.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original family</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pure calculation</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">result or refusal</text>
    
</svg>
</div>

- **Goal:** Reuse absolute coordinate parsing for named construction planes and three-point input.
- **Data Flow Trace:** `original family → pure calculation → result or refusal`
- **Files You Touch:** `src/project_geometry.rs` (*Type the whole original helpers introduced today.*) · `src/lib.rs` (*Register the original projection helpers.*)

Reuse absolute coordinate parsing for named construction planes and three-point input. Reject unknown names, wrong point counts and collinear frame points.

> **Self-Check Question:** *Why validate the three points before projecting?*  
> **Answer:** They must define a stable plane. Collinear or repeated points cannot supply its normal.

- **What You Should See:** Projection-plane parsing is prepared while the existing command behavior remains available.
- **Hands-On Lab Experiment:** Compare XY, the current construction plane and three picked points. Trace the wrong-arity and collinearity refusals.

<details>
<summary>Implementation 80za-read-original-projection-planes: Read original projection planes</summary>

Reuse absolute coordinate parsing for named construction planes and three-point input.

Data flow: original family → pure calculation → result or refusal.

### 1. `src/project_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-1.rs"
```

### 2. `src/lib.rs`

Register the original projection helpers.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod network_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-2.rs" skip=32
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 80za-read-original-projection-planes
npm --prefix ../session_tests run course -- check 80za-read-original-projection-planes
npm --prefix ../session_tests run course -- save 80za-read-original-projection-planes
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Projection-plane parsing is prepared while the existing command behavior remains available.

- **Final Lesson Experiment:** Compare XY, the current construction plane and three picked points. Trace the wrong-arity and collinearity refusals.
