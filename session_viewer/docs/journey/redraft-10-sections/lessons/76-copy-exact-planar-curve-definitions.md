# 14.6 · Exact parabola recipes and editable freeform controls

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 14.6** (Redraft `8.6`) · **5 checkpoints (`76-copy-exact-planar-curve-definitions` → `76d-exercise-exact-curves-and-rational-control-edits`)**

> **Learning Goal:** Complete the original parabola questions and input reader.

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
shared questions and frame → original exact curve → derived preview.
collected original controls → degree and closure → NURBS curve → one history edit.
original curve → checked control preview → one transaction → exact restoration.
```

- Complete the original parabola questions and input reader.
- Write the original degree and closure rules for collected control points.
- Exercise exact curves and rational control edits.
- **Why it works (Why retain the apex in the chord frame?):** The original curve can use the same input interpretation in any construction plane. Its display samples remain derived from that source.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Production source (`session_viewer/src`):** [app/walk/sheet.rs](../../../../src/app/walk/sheet.rs), [app/sheet_query.rs](../../../../src/app/sheet_query.rs), [app/stream.rs](../../../../src/app/stream.rs), [app/cplane.rs](../../../../src/app/cplane.rs), [app/coords.rs](../../../../src/app/coords.rs), [app/command/tool/shape.rs](../../../../src/app/command/tool/shape.rs), [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs), [app/deform.rs](../../../../src/app/deform.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-76-copy-exact-planar-curve-definitions"></a>

<details>
<summary>Implementation 76-copy-exact-planar-curve-definitions: Read original parabola inputs in a checked frame</summary>

Complete the original parabola questions and input reader.

Data flow: shared questions and frame → original exact curve → derived preview.

### 1. `src/freeform_parabola.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
const SAMPLES: usize = 32; // preview segments
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/76-copy-exact-planar-curve-definitions-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/freeform_parabola.rs`

Supply only the unchanged original curve validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::primitive_shape::{ Answer, Ask, CURVE, Frame, Part, Shape, positive};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Parabola",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

const SAMPLES: usize = 32; // preview segments

/// Start, end, then the apex.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Start")),
        1 => Some(Ask::size("End or length")),
        2 => Some(Ask::size("Apex or height")),
        _ => None,
    }
}

/// The frame runs from start to end; the apex is where the curve passes half way, off the chord.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(end) = end {
        let [u, v, _] = frame.local(end);
        part.frame = frame.turned(u, v);
    }

    let length = positive(frame.size(end), "The length")?; // a click lands on the plane
    part.sizes.push(length);

    if let Some(apex) = answers.get(2) {
        let [u, v, w] = match apex {
            Answer::Number(height) => [length * 0.5, *height, 0.0], // to the left of start → end
            Answer::Point(p) => part.frame.local(p),
        };

        if v.hypot(w) <= 1e-9 * length {
            return Err("The apex must lie off the line from start to end".into());
        }

        part.sizes.extend([u, w, v]); // the height last: the cursor readout shows it
    }

    Ok(part)
}

/// Start, apex and end in the world.
fn points(part: &Part) -> Option<[Point; 3]> {
    let [length, u, w, v] = part.sizes[..] else {
        return None;
    };
    let frame = &part.frame;
    Some([
        frame.at(0.0, 0.0, 0.0),
        frame.at(u, v, w),
        frame.at(length, 0.0, 0.0),
    ])
}

/// The chord, then the parabola.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let Some([start, apex, end]) = points(part) else {
        return part.sizes.first().map_or_else(Vec::new, |length| {
            vec![vec![
                part.frame.at(0.0, 0.0, 0.0),
                part.frame.at(*length, 0.0, 0.0),
            ]]
        });
    };
    // the Bezier control that makes the curve pass the apex half way
    let control = [0, 1, 2].map(|i| 2.0 * apex[i] - 0.5 * (start[i] + end[i]));
    let curve = (0..=SAMPLES)
        .map(|i| {
            let t = i as f64 / SAMPLES as f64;
            let (a, b, c) = ((1.0 - t) * (1.0 - t), 2.0 * t * (1.0 - t), t * t);
            Point::new(
                a * start[0] + b * control[0] + c * end[0],
                a * start[1] + b * control[1] + c * end[1],
                a * start[2] + b * control[2] + c * end[2],
            )
        })
        .collect();
    vec![curve]
}

/// The kernel parabola through start, apex and end.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [start, apex, end] =
        points(part).ok_or("Nurbs Curve Parabola needs a start, an end and an apex")?;
    let mut curve = Primitives::parabola(&start, &apex, &end);
    curve.name = "parabola".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/76-copy-exact-planar-curve-definitions-2.rs" skip=3319
```

</details>

</details>

<a id="checkpoint-76a-expose-exact-curve-choices-in-the-existing-registry"></a>

<details>
<summary>Implementation 76a-expose-exact-curve-choices-in-the-existing-registry: Expose exact curve choices in the existing registry</summary>

Expose exact curve choices in the existing registry.

Data flow: registered curve name → current option → shared answer owner.

### 1. `src/primitive_registry.rs`

Register all current exact planar curve names and supported Arc options.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Quad Sphere",&crate::round_quad_sphere::SHAPE,0),
    ("Quad Sphere Mesh",&crate::round_quad_sphere::SHAPE,0),
    ("bo",&crate::primitive_box::SHAPE,0),
];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/76a-expose-exact-curve-choices-in-the-existing-registry-01.rs"
```

</details>

<a id="checkpoint-76b-retain-editable-freeform-controls-in-the-point-collector"></a>

<details>
<summary>Implementation 76b-retain-editable-freeform-controls-in-the-point-collector: Create editable freeform curves</summary>

Write the original degree and closure rules for collected control points.

Data flow: collected original controls → degree and closure → NURBS curve → one history edit.

### 1. `src/freeform_curve.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-1.rs"
```

### 2. `src/lib.rs`

Register the original freeform-curve constructor.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod freeform_parabola;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-2.rs" skip=28
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/freeform_curve.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{NurbsCurve,Point};
/// A curve through control points; ending on the start closes it smoothly.
pub fn curve(points: &[Point]) -> NurbsCurve {
    let count = points.len();
    let closed = count >= 4 && points[0].distance(&points[count - 1], None) <= 1e-12;

    if closed {
        return NurbsCurve::create(true, (count - 2).min(3), &points[..count - 1]);
    }

    NurbsCurve::create(false, (count - 1).min(3), points)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-3.rs" skip=445
```

</details>

### 4. `src/tool.rs`

Build original freeform curves from accepted controls and offer existing Finish/Close input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                let points=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                Ok(Built::Polyline(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points)))?))
            }
            (Self::Rectangle|Self::Polygon {..},_)=>self.build_on_plane(points,crate::cplane::CPlane::Xy),
            _ => Err("The tool needs more points"),
        }
```

</details>

Replace it with:

```rust
                let points=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                Ok(Built::Polyline(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points)))?))
            }
            (Self::Curve,points) if (2..=MAX_POINTS).contains(&points.len())=>{
                if !points.windows(2).any(|p|p[0]!=p[1]) {return Err("Curve controls must differ");}
                let controls=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect::<Vec<_>>();
                let source=crate::freeform_curve::curve(&controls);
                Ok(Built::Geometry(session_rust::Geometry::NurbsCurve(Rc::new(source))))
            },
            (Self::Rectangle|Self::Polygon {..},_)=>self.build_on_plane(points,crate::cplane::CPlane::Xy),
            _ => Err("The tool needs more points"),
        }
```

<details>
<summary>Locate this block</summary>

```rust
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
```

</details>

Replace it with:

```rust
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline|Self::Curve) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
```

<details>
<summary>Locate this block</summary>

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

</details>

Replace it with:

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]), Polyline(crate::chain::PreparedChain), Geometry(session_rust::Geometry) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Result<Built,&'static str> {self.build(points)}
    fn outline(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Vec<[f64;3]> {points.to_vec()}
    fn sides(&self)->Option<usize> {None}
    fn set_sides(&mut self,_sides:usize)->Result<(),&'static str> {Err("Sides works while drawing a polygon")}
    fn buttons(&self) -> Vec<(String, String)> { vec![("Cancel".into(), "Escape".into())] }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
        let Self::Polygon {sides}=self else {return Err("Sides works while drawing a polygon")};*sides=value;Ok(())
    }
    fn build_on_plane(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Result<Built,&'static str> {
        let (kind,sides)=match self {Self::Rectangle=>("rectangle",4),Self::Polygon {sides}=>("polygon",*sides),_=>return self.build(points)};
        let points=crate::planar::points(kind,plane,sides,points)?;Self::Polyline.build(&points)
    }
    fn outline(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Vec<[f64;3]> {
        match self {Self::Rectangle if points.len()==2=>crate::planar::points("rectangle",plane,4,points).unwrap_or_default(),
            Self::Polygon {sides} if points.len()==2=>crate::planar::points("polygon",plane,*sides,points).unwrap_or_default(),_=>points.to_vec()}
    }
    fn buttons(&self)->Vec<(String,String)> {
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
        let mut buttons=vec![("Cancel".into(),"Escape".into())];
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline|Self::Curve) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
            (Self::Line, [a, b]) => {
                if a.iter().zip(b).all(|(a, b)| (a-b).abs() < 1e-12) { return Err("Line endpoints must differ"); }
                Ok(Built::Line(PreparedLine::new(Rc::new(session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2])))?))
            }
            (Self::Move, [a,b]) => Ok(Built::Move(std::array::from_fn(|i| b[i]-a[i]))),
            (Self::Polyline, points) if (2..=MAX_POINTS).contains(&points.len()) => {
                if !points.windows(2).any(|p| p[0] != p[1]) { return Err("Polyline points must differ"); }
                let points=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                Ok(Built::Polyline(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points)))?))
            }
            (Self::Curve,points) if (2..=MAX_POINTS).contains(&points.len())=>{
                if !points.windows(2).any(|p|p[0]!=p[1]) {return Err("Curve controls must differ");}
                let controls=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect::<Vec<_>>();
                let source=crate::freeform_curve::curve(&controls);
                Ok(Built::Geometry(session_rust::Geometry::NurbsCurve(Rc::new(source))))
            },
            (Self::Rectangle|Self::Polygon {..},_)=>self.build_on_plane(points,crate::cplane::CPlane::Xy),
            _ => Err("The tool needs more points"),
        }
    }
}

pub const MAX_POINTS: usize = 4096;

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
            else if self.tool.count()==0 {format!("{} point {} · Enter finishes · Esc cancels",self.tool.name(),self.points.len()+1)}
            else {format!("{} point {} of {} · Esc cancels", self.tool.name(), self.points.len()+1, self.tool.count())};
        if let Some(sides)=self.tool.sides() {prompt.push_str(&format!(" · Sides {sides}"));}
        if let Some(kind)=self.snapped { prompt.push_str(&format!(" · Snap {kind:?}")); }
        prompt
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> { self.submit_many(&[point]) }
    pub fn submit_many(&mut self, added: &[[f64;3]]) -> Result<Option<Built>, &'static str> {
        let mut points=self.points.clone();
        for &point in added { points.push(crate::tool_coords::checked(point)?); }
        let count=self.tool.count(); let limit=if count==0 {MAX_POINTS} else {count};
        if points.len()>limit {return Err("The tool has enough points");}
        let built=if count>0 && points.len()==count {Some(self.build_world(&points)?)} else {None};
        self.points=points;self.cursor=None;self.snapped=None;Ok(built)
    }
    pub fn finish(&self) -> Result<Built,&'static str> { self.build_world(&self.points) }
    // Finished coordinates already belong to the world; their prefix prevents applying a fixed plane a second time during replay.
    pub fn command(&self, points:&[[f64;3]]) -> String {
        let mut line=self.tool.name().to_owned();
        for p in points {line.push_str(&format!(" w{},{},{}",p[0],p[1],p[2]));}line
    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=&command[self.tool.name().len()..];
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build_on_plane(&world,self.frame)
    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
    pub fn outline(&self)->Vec<[f64;3]> {
        let mut points=self.points.clone();if let Some(cursor)=self.cursor {points.push(cursor);}
        self.tool.outline(&points,self.frame)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
        self.cursor = point.map(crate::tool_coords::checked).transpose()?; Ok(())
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-4.rs"
```

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-5.rs"
```

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-6.rs"
```

</details>

### 5. `src/commands.rs`

Route current Curve invocation and direct controls through the existing world-point collector.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
        let shape=match name {"Point"=>crate::tool::Shape::Point,"Line"=>crate::tool::Shape::Line,"Polyline Rectangle"=>crate::tool::Shape::Rectangle,"Polyline Polygon"=>crate::tool::Shape::Polygon {sides:6},_=>crate::tool::Shape::Polyline};
        return Ok(Command::Editor(Action::Draw {shape,text:words[count..].join(" ")}));
    }
    if name == "Move" && words.len()>count {
```

</details>

Replace it with:

```rust
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
        let shape=match name {"Point"=>crate::tool::Shape::Point,"Curve"=>crate::tool::Shape::Curve,"Line"=>crate::tool::Shape::Line,"Polyline Rectangle"=>crate::tool::Shape::Rectangle,"Polyline Polygon"=>crate::tool::Shape::Polygon {sides:6},_=>crate::tool::Shape::Polyline};
        return Ok(Command::Editor(Action::Draw {shape,text:words[count..].join(" ")}));
    }
    if name == "Move" && words.len()>count {
```

<details>
<summary>Locate this block</summary>

```rust
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Rectangle",||Action::BeginTool(crate::tool::Shape::Rectangle)),
```

</details>

Replace it with:

```rust
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Curve",||Action::BeginTool(crate::tool::Shape::Curve)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Rectangle",||Action::BeginTool(crate::tool::Shape::Rectangle)),
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { TextDemo, Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Snap", || Action::Snap(crate::snap::Setting::Objects(None))),
    ("Snap On", || Action::Snap(crate::snap::Setting::Objects(Some(true)))),
    ("Snap Off", || Action::Snap(crate::snap::Setting::Objects(Some(false)))),
    ("Grid Snap", || Action::Snap(crate::snap::Setting::Grid(None))),
    ("Grid Snap On", || Action::Snap(crate::snap::Setting::Grid(Some(true)))),
    ("Grid Snap Off", || Action::Snap(crate::snap::Setting::Grid(Some(false)))),
    ("Snap End", || Action::Snap(crate::snap::Setting::Mode(crate::snap::END))),
    ("Snap Near", || Action::Snap(crate::snap::Setting::Mode(crate::snap::NEAR))),
    ("Snap Mid", || Action::Snap(crate::snap::Setting::Mode(crate::snap::MID))),
    ("Snap Center", || Action::Snap(crate::snap::Setting::Mode(crate::snap::CENTER))),
    ("Snap Perp", || Action::Snap(crate::snap::Setting::Mode(crate::snap::PERP))),
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Curve",||Action::BeginTool(crate::tool::Shape::Curve)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Rectangle",||Action::BeginTool(crate::tool::Shape::Rectangle)),
    ("Polyline Polygon",||Action::BeginTool(crate::tool::Shape::Polygon {sides:6})),
    ("CPlane",||Action::PlaneMenu),
    ("Hide",||Action::Hide),
    ("Show",||Action::Show),
    ("Example Box", || Action::AddBox),
    ("Example Cloud LOD",||Action::AddCloudLod),
    ("Example Cloud",||Action::AddCloud),
    ("Example CAD",||Action::AddCad),
    ("Example Instances",||Action::AddInstances),
    ("Example Shape Reuse",||Action::AddShapeReuse),
    ("Example Nested Instances",||Action::AddNestedInstances),
    ("Example Sheet",||Action::AddSheet),
    ("Example Scene Text",||Action::AddText(crate::text::TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})),
    ("Example Billboard",||Action::AddText(crate::text::TextPlacement::WorldBillboard {world:[-1.5,0.,1.],world_height:0.22})),
    ("Example Anchor",||Action::AddText(crate::text::TextPlacement::Anchor {world:[0.,0.,1.],offset:[8.,-12.]})),
    ("Example Nameplate",||Action::AddText(crate::text::TextPlacement::Nameplate {world:[0.,0.,1.],padding:[17.,4.],rounded:true})),
    ("Example Surface",||Action::AddSurface),
    ("Example Nested",||Action::AddNested),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
    ("Example Curve", || Action::AddCurve),
    ("Example Object Color", || Action::AddColours(crate::colour::Mode::Object)),
    ("Example Point Colors", || Action::AddColours(crate::colour::Mode::Points)),
    ("Example Face Colors", || Action::AddColours(crate::colour::Mode::Faces)),
    ("View Normals On", || Action::NormalView(true)),
    ("View Normals Off", || Action::NormalView(false)),
    ("View Grid On", || Action::Grid(Some(crate::grid::Settings::default()))),
    ("View Grid Off", || Action::Grid(None)),
    ("Example Sharp", || Action::AddNormals(crate::normal_example::Mode::Sharp)),
    ("Example Smooth", || Action::AddNormals(crate::normal_example::Mode::Smooth)),
    ("Example Winding", || Action::AddNormals(crate::normal_example::Mode::Winding)),
    ("Controls", || Action::Controls(true)),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
    ("Example Polyline", || Action::AddPolyline),
    ("Example Arrow", || Action::AddArrow),
    ("Example Triangle", || Action::ToggleExtra),
    ("Select Next", || Action::SelectNext),
    ("Select All", || Action::SelectAll),
    ("Select None", || Action::Picked(None)),
    ("Select Object", || Action::PickMode(crate::pick_sources::Mode::Object)),
    ("Select Face", || Action::PickMode(crate::pick_sources::Mode::Face)),
    ("Select Edge", || Action::PickMode(crate::pick_sources::Mode::Edge)),
    ("Select Control", || Action::PickMode(crate::pick_sources::Mode::Control)),
    ("Delete", || Action::Delete),
    ("Undo", || Action::Undo),
    ("Redo", || Action::Redo),
    ("Background", || Action::Background),
    ("Zoom In", || Action::Zoom(2.0)),
    ("Zoom Out", || Action::Zoom(0.5)),
    ("Pan Left", || Action::Pan(-0.25, 0.0)),
    ("Pan Right", || Action::Pan(0.25, 0.0)),
    ("Orbit Right", || Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)),
    ("Orbit Up", || Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)),
    ("View Isometric", || Action::Isometric),
    ("Fit", || Action::Fit),
    ("Fit Selected", || Action::FitSelected),
    ("View Reset", || Action::ResetView),
    ("View Perspective", || Action::Projection(crate::camera::Projection::Perspective)),
    ("View Orthographic", || Action::Projection(crate::camera::Projection::Orthographic)),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
];

pub const HOSTS: &[(&str, Host)] = &[
    ("Layers On",Host::LayersOn),("Layers Off",Host::LayersOff),("Layers All",Host::LayersAll),
    ("Help", Host::Help),
    ("Open", Host::Open),
    ("Open Replace", Host::Replace),
    ("Cancel Open", Host::CancelOpen),
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text",Host::TextDemo),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names().into_iter().filter_map(|name| {
        let compact = name.split_whitespace().collect::<String>().to_ascii_lowercase();
        let mut rest = compact.as_str();
        for (index, word) in words.iter().enumerate() {
            let word = word.to_ascii_lowercase();
            rest = rest.strip_prefix(&word)?;
            if rest.is_empty() { return Some((name, index + 1, compact.len())); }
        }
        None
    }).max_by_key(|(_, _, letters)| *letters).map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
    })
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
        let shape=match name {"Point"=>crate::tool::Shape::Point,"Curve"=>crate::tool::Shape::Curve,"Line"=>crate::tool::Shape::Line,"Polyline Rectangle"=>crate::tool::Shape::Rectangle,"Polyline Polygon"=>crate::tool::Shape::Polygon {sides:6},_=>crate::tool::Shape::Polyline};
        return Ok(Command::Editor(Action::Draw {shape,text:words[count..].join(" ")}));
    }
    if name == "Move" && words.len()>count {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
            .map(|offset| Command::Editor(Action::Translate(offset))).map_err(str::to_owned);
    }
    if name == "Grid Snap" && words.len() > count {
        if words.len() != count+1 { return Err("Grid Snap (On Off) or a spacing".into()); }
        let step = words[count].parse::<f64>().map_err(|_| "Grid Snap (On Off) or a spacing")?;
        if !step.is_finite() || step<=0. { return Err("The grid spacing must be above zero · Example: Grid Snap 50".into()); }
        return Ok(Command::Editor(Action::Snap(crate::snap::Setting::Spacing(step))));
    }
    if words.len() != count { return Err(format!("{name} takes no arguments")); }
    if let Some((_, make)) = ACTIONS.iter().find(|(key, _)| *key == name) { return Ok(Command::Editor(make())); }
    let (_, host) = HOSTS.iter().find(|(key, _)| *key == name).ok_or("Command registry is incomplete")?;
    Ok(Command::Host(*host))
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-7.rs"
```

```text
--8<-- "journey/code/76b-retain-editable-freeform-controls-in-the-point-collector-8.rs"
```

</details>

</details>

<a id="checkpoint-76c-inspect-original-curve-data-through-the-browser-contract"></a>

<details>
<summary>Implementation 76c-inspect-original-curve-data-through-the-browser-contract: Inspect original curve data through the browser contract</summary>

Inspect original curve data through the browser contract.

Data flow: original curve owner → explicit diagnostic facts → browser assertions.

### 1. `src/browser.rs`

Expose original curve identity, weighted controls and knots for current browser acceptance.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    let originals=serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/76c-inspect-original-curve-data-through-the-browser-contract-01.rs"
```

</details>

<a id="checkpoint-76d-exercise-exact-curves-and-rational-control-edits"></a>

<details>
<summary>Implementation 76d-exercise-exact-curves-and-rational-control-edits: Exercise exact curves and rational control edits</summary>

Exercise exact curves and rational control edits.

Data flow: original curve → checked control preview → one transaction → exact restoration.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/freeform_tests.rs`

Copy integrated exact-original, closure/world-replay, failed-input, rational-edit and archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/76d-exercise-exact-curves-and-rational-control-edits-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the current curve component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod freeform_curve;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/76d-exercise-exact-curves-and-rational-control-edits-02.rs" skip=25
```

</details>

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 76d-exercise-exact-curves-and-rational-control-edits
npm --prefix ../session_tests run course -- check 76d-exercise-exact-curves-and-rational-control-edits
npm --prefix ../session_tests run course -- save 76d-exercise-exact-curves-and-rational-control-edits
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

Five original NURBS curve families render on white. Curve controls remain editable; rational edits preserve weights and knots. Failed input keeps accepted answers and original source identity survives archive reopening.

- **Try this:** Type Close, CPlane XY, Nurbs Curve Circle w-2,0,0 0.5, Nurbs Curve Ellipse w-0.7,0,0 0.55 0.3, Nurbs Curve Arc Center w0.7,0,0 0.5 270, Nurbs Curve Parabola w1.5,0,0 1 0.5, Curve w-0.5,1,0 w0,1.6,0 w0.5,1,0, View Reset and Fit. Select a curve, use Controls or F10, select a control and move its handle; Undo restores the original control. Save and reopen to retain original weights and knots.
