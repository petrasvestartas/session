# 16.3 · Joined-face topology and exact surface measures

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 16.3** (Redraft `8.13`) · **4 checkpoints (`83-split-original-faces-without-separating-the-solid` → `83ze-integrate-original-surface-area`)**

> **Learning Goal:** Divide a selected original surface face while preserving joined topology and one-step history.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 8 Precision CAD Frames, Solid Primitives, Surfacing & Sub-Object Deformation (Chapters 14–16)</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How double-precision coordinate frames (Plane), parametric solids, Loft/Sweep/Extrude surfaces, and Gumball control edits work.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. Orthonormal Frames (Ch 14)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Plane { origin: Vec3d, x, y, z }</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Rejects non-finite / collinear axes</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• Basis change: World &lt;-&gt; Local Plane</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• Exact Mirror, Rotate3D, Orient,</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  and Align frame transformations</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="295" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. Solids &amp; Surfacing (Ch 15–16)</text>
    <text x="307" y="108" font-size="10.5" fill="#7c2d12">• Parametric Box, Cylinder, Sphere, Cone</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• Extrude, Revolve, Loft, Sweep, Planar</text>
    <text x="307" y="144" font-size="10.5" fill="#7c2d12">• Curve Split, Trim, Extend, Join, Fillet</text>
    <text x="307" y="162" font-size="10.5" fill="#7c2d12">• Every operation commits an atomic</text>
    <text x="307" y="178" font-size="10.5" fill="#7c2d12">  session_rust kernel transaction</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="592" y="68" width="228" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Rational Control-Point Deform</text>
    <text x="604" y="108" font-size="10.5" fill="#064e3b">• Pick NURBS control point or vertex</text>
    <text x="604" y="126" font-size="10.5" font-weight="600" fill="#047857">• Preserve homogeneous weight w</text>
    <text x="604" y="142" font-size="10.5" font-weight="600" fill="#047857">  on rational curve control drags!</text>
    <text x="604" y="162" font-size="10.5" fill="#064e3b">• Rebuild derived GPU tessellation</text>
    <text x="604" y="178" font-size="10.5" fill="#064e3b">  in-place while keeping Object ID</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
original selected face → on-surface original cutters → joined trimmed regions → one transaction.
original triangles → world transform → area or signed swept volume.
original geometry → placed source → measure or refusal.
```

- Divide a selected original surface face while preserving joined topology and one-step history.
- Reuse the triangle visitor for face area, mesh area and signed swept volume.
- Reuse mesh area for meshes, meshed BReps and Elements.
- **Why it works (Does splitting a face create separate mesh objects?):** No. A NURBS surface becomes one trimmed BRep. Splitting one face of a solid keeps that solid joined, replaces that face with regions and preserves its other faces. A multi-face BRep requires an explicit face selection.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Production source (`session_viewer/src`):** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs), [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs), [app/splitting.rs](../../../../src/app/splitting.rs), [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs), [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs), [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs), [app/walk/frames.rs](../../../../src/app/walk/frames.rs), [build.rs](../../../../build.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-83-split-original-faces-without-separating-the-solid"></a>

<details>
<summary>Implementation 83-split-original-faces-without-separating-the-solid: Split original faces without separating the solid</summary>

Divide a selected original surface face while preserving joined topology and one-step history.

Data flow: original selected face → on-surface original cutters → joined trimmed regions → one transaction.

### 1. `src/split_draft.rs`

Allow original surfaces and validated BRep faces through the shared pending split owner.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn new(scene: &Scene, id: ObjectId, face: Option<usize>) -> Result<Self, String> {
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        if !crate::split_geometry::is_cutter(&target.source) {
            return Err("Select a curve; surface-face splitting connects next".into());
        }
        let face = crate::split_geometry::face_index(&target.source, face)?;
        Ok(Self {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/split_face_tests.rs`

Copy integrated maintained joined-face, original placement, topology, area, archive and history fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/lib.rs`

Register the copied original-face split fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)]
mod split_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-03.rs" skip=30
```

</details>

</details>

<a id="checkpoint-83za-visit-original-mesh-triangles-and-format-measures"></a>

<details>
<summary>Implementation 83za-visit-original-mesh-triangles-and-format-measures: Visit original mesh triangles and format measures</summary>

Write one visitor for the original mesh triangles in world coordinates.

Data flow: original triangles → world transform → area or signed swept volume.

### 1. `src/measure_math.rs`

Type the whole original mesh measurement helpers using the shared visitor.

Create the file and type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original measurement module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod split_face_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-2.rs" skip=35
```

</details>

<a id="checkpoint-83zb-compute-original-mesh-measures-and-curve-length"></a>

<details>
<summary>Implementation 83zb-compute-original-mesh-measures-and-curve-length: Compute original mesh measures and curve length</summary>

Reuse the triangle visitor for face area, mesh area and signed swept volume.

Data flow: original geometry → placed source → measure or refusal.

### 1. `src/measure_math.rs`

Reuse the original triangle visitor for mesh area and signed swept volume.

<details>
<summary>Locate the existing block</summary>

```rust
    let corners = mesh.face.get(&face).map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}
```

</details>

After that block, type:

```rust
/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/measure_math.rs`

Supply only the unchanged original mesh measurement fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Mesh,Point,Vector,Xform};
/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub fn for_each_triangle(
    mesh: &Mesh,
    face: usize,
    place: &Xform,
    visit: impl FnMut([Point; 3]),
) {
    let at = |key: usize| mesh.vertex_point(key).map(|p| p.transformed(place));
    let triangle = |[a, b, c]: [usize; 3]| Some([at(a)?, at(b)?, at(c)?]);

    if let Some(cached) = mesh
        .triangulation
        .get(&face)
        .filter(|tris| !tris.is_empty())
    {
        cached
            .iter()
            .filter_map(|&keys| triangle(keys))
            .for_each(visit);
        return;
    }

    let corners = mesh.face.get(&face).map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}

/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}

/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(" · {} skipped", plural(count, "other object")),
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-2.rs" skip=3574
```

</details>

### 3. `src/measure_length.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-3.rs"
```

### 4. `src/lib.rs`

Register this original measurement owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod measure_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-4.rs" skip=23
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/measure_length.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Geometry,Xform};
/// World length of a line, polyline or NURBS curve placed by `place`; None for other kinds.
pub fn compute_length(geometry: &Geometry, place: &Xform) -> Option<f64> {
    match geometry {
        Geometry::Line(line) => Some(line.transformed(place).length()),
        Geometry::Polyline(polyline) => Some(polyline.transformed(place).length()),
        Geometry::NurbsCurve(curve) => Some(curve.transformed(place).length(None)),
        _ => None,
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-5.rs" skip=493
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/cad_orient.rs`

Supply only original orientation validation fixtures against the already taught shared owner.

<details>
<summary>Locate the existing block</summary>

```text
use crate::cad_edges::EdgeChain;
use session_rust::{BRep, Mesh};

fn walks(fm: &Mesh, s: usize, n: usize) -> Option<bool> {
    let fwd = occupied_halfedge(fm, s, n);
    let back = occupied_halfedge(fm, n, s);

    match (fwd, back) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

fn occupied_halfedge(fm: &Mesh, from: usize, to: usize) -> bool {
    let Some(neighbours) = fm.halfedge.get(&from) else {
        return false;
    };
    matches!(neighbours.get(&to), Some(Some(_)))
}

fn at(fm: &Mesh, k: usize) -> [f64; 3] {
    let v = &fm.vertex[&k];
    [v.x, v.y, v.z]
}

fn neighbour_along(fm: &Mesh, s: usize, dir: [f64; 3]) -> Option<usize> {
    let p = at(fm, s);
    let mut best: Option<(f64, usize)> = None;

    for &w in fm.halfedge.get(&s)?.keys() {
        let q = at(fm, w);
        let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();

        if l == 0.0 {
            continue;
        }

        let c = (d[0] * dir[0] + d[1] * dir[1] + d[2] * dir[2]) / l;

        if match best {
            Some((bc, bw)) => c > bc || (c == bc && w < bw),
            None => true,
        } {
            best = Some((c, w));
        }
    }

    Some(best?.1)
}

fn vertex_at(fm: &Mesh, p: [f64; 3]) -> Option<usize> {
    let mut best: Option<(f64, usize)> = None;

    for (&k, v) in fm.vertex.iter() {
        let d = (v.x - p[0]).powi(2) + (v.y - p[1]).powi(2) + (v.z - p[2]).powi(2);

        if match best {
            Some((bd, bk)) => d < bd || (d == bd && k < bk),
            None => true,
        } {
            best = Some((d, k));
        }
    }

    Some(best?.1)
}

fn opposed(fms: &[Mesh], c: &EdgeChain) -> Option<bool> {
    let other = c.other?;
    let (fa, fb) = (&fms[c.face], &fms[other]);
    let (s, n) = (c.keys[0], c.keys[1]);
    let away_a = walks(fa, s, n)?;
    let (ps, pn) = (at(fa, s), at(fa, n));
    let dir = [pn[0] - ps[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?;
    let nb = neighbour_along(fb, sb, dir)?;
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable();
    let mut v = 0.0;

    for k in keys {
        let verts = &fm.face[&k];

        if verts.len() < 3 {
            continue;
        }

        let (a, b, c) = (at(fm, verts[0]), at(fm, verts[1]), at(fm, verts[2]));
        v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }

    v
}

pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

    if !b.is_solid() {
        return vec![1.0; nf];
    }

    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf];

    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }

    let mut sign = vec![0.0f64; nf];

    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }

        sign[start] = 1.0;
        let mut group = vec![start];
        let mut head = 0;

        while head < group.len() {
            let f = group[head];
            head += 1;

            for &(g, opp) in &adjacent[f] {
                if sign[g] == 0.0 {
                    sign[g] = if opp { sign[f] } else { -sign[f] };
                    group.push(g);
                }
            }
        }

        let mut volume = 0.0;

        for &face in &group {
            volume += sign[face] * six_volume(&fms[face]);
        }

        if volume < 0.0 {
            for &f in &group {
                sign[f] = -sign[f];
            }
        }
    }

    sign
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-dedup-6.rs" skip=3938
```

</details>

</details>

<a id="checkpoint-83ze-integrate-original-surface-area"></a>

<details>
<summary>Implementation 83ze-integrate-original-surface-area: Integrate original surface area</summary>

Reuse mesh area for meshes, meshed BReps and Elements.

Data flow: original geometry → placed source → measure or refusal.

### 1. `src/measure_area.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/83ze-integrate-original-surface-area-1.rs"
```

### 2. `src/lib.rs`

Register surface area after curve length; reuse the earlier orientation owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod measure_length;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83ze-integrate-original-surface-area-dedup-registration.rs" skip=25
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/measure_area.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::measure_math::compute_mesh_area;
use session_rust::{BRep,Geometry,NurbsSurface,Xform};
use session_rust::element::ElementGeometry;
/// World area of a surface kind placed by `place`, all faces or only `face`; None for other kinds.
pub fn compute_area(geometry: &Geometry, place: &Xform, face: Option<usize>) -> Option<f64> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
        Geometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
        Geometry::NurbsSurface(surface) => Some(compute_surface_area(&surface.transformed(place))),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
            ElementGeometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// World area of a BRep's faces as the viewer meshes them; planar faces are exact.
fn compute_brep_area(brep: &BRep, place: &Xform, face: Option<usize>) -> f64 {
    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    meshes
        .iter()
        .enumerate()
        .filter(|(index, _)| face.is_none_or(|face| face == *index))
        .map(|(_, mesh)| compute_mesh_area(mesh, place, None))
        .sum()
}

const GAUSS: [(f64, f64); 8] = [
    (-0.9602898564975363, 0.1012285362903763),
    (-0.7966664774136267, 0.2223810344533745),
    (-0.5255324099163290, 0.3137066458778873),
    (-0.1834346424956498, 0.3626837833783620),
    (0.1834346424956498, 0.3626837833783620),
    (0.5255324099163290, 0.3137066458778873),
    (0.7966664774136267, 0.2223810344533745),
    (0.9602898564975363, 0.1012285362903763),
]; // 8-point Gauss-Legendre nodes and weights on [-1, 1]

/// Area of a NURBS surface: |Su x Sv| integrated per knot span with 8 x 8 Gauss points.
fn compute_surface_area(surface: &NurbsSurface) -> f64 {
    let us = surface.get_span_vector(0);
    let vs = surface.get_span_vector(1);
    let mut total = 0.0;

    for u in us.windows(2) {
        let (uh, um) = ((u[1] - u[0]) * 0.5, (u[1] + u[0]) * 0.5); // half width, middle

        for v in vs.windows(2) {
            let (vh, vm) = ((v[1] - v[0]) * 0.5, (v[1] + v[0]) * 0.5);

            for (su, wu) in GAUSS {
                for (sv, wv) in GAUSS {
                    let derivatives = surface.evaluate(um + uh * su, vm + vh * sv, 1); // [S, Sv, Su]

                    if let [_, dv, du] = derivatives.as_slice() {
                        total += wu * wv * uh * vh * du.cross(dv).magnitude();
                    }
                }
            }
        }
    }

    total
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83ze-integrate-original-surface-area-3.rs" skip=2714
```

</details>

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 83ze-integrate-original-surface-area
npm --prefix ../session_tests run course -- check 83ze-integrate-original-surface-area
npm --prefix ../session_tests run course -- save 83ze-integrate-original-surface-area
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

Surface-area integration can measure original surface data before the measurement command connection.

- **Try this:** Compare uniform scaling with translation. Check the planar patch and exact cylinder surface against their known areas.

Continue with [Measurements, temporary marks and retained CAD pieces](84-measure-original-curves-surfaces-and-closed-solids.md).
