# 14.6 · Exact parabola recipes and editable freeform controls

**Goal:** Exercise exact curves and rational control edits. · **Steps:** 5 · **Source:** [app/walk/sheet.rs](../../../src/app/walk/sheet.rs) · [app/sheet_query.rs](../../../src/app/sheet_query.rs) · [app/stream.rs](../../../src/app/stream.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/coords.rs](../../../src/app/coords.rs) · [app/command/tool/shape.rs](../../../src/app/command/tool/shape.rs) · [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs) · [app/deform.rs](../../../src/app/deform.rs) (`src/app/cplane.rs,src/app/primitives.rs,src/app/curves.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 14.6 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/freeform_parabola.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/primitive_registry.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/freeform_curve.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">5 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/cplane.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/primitives.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Read parabola inputs in a…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Local C-Plane Triad &amp; OSnap</text>
  <polygon points="602,130 650,74 798,74 750,130" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="686" y1="108" x2="742" y2="108" stroke="#dc2626" stroke-width="2.25" marker-end="url(#arrR)"/>
  <line x1="686" y1="108" x2="658" y2="132" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <line x1="686" y1="108" x2="686" y2="54" stroke="#2563eb" stroke-width="2.25" marker-end="url(#arrB)"/>
  <circle cx="686" cy="108" r="4.5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Exact parabola recipes and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Exact Curves &amp; Primitives</text>
  <circle cx="928" cy="94" r="32" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="982" y="64" width="62" height="60" rx="4" fill="#dcfce7" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">NURBS Arc + B-Rep Box</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Exercise exact curves and…</text>
</svg>
</div>

## <a id="checkpoint-76-copy-exact-planar-curve-definitions"></a>1. Read parabola inputs in a checked frame

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/freeform_parabola.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared questions and frame</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original exact curve</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">derived preview</text>
</svg>
</div>

**What this code does:** Complete the original parabola questions and input reader.

### 1. `src/freeform_parabola.rs`

Extend this original owner with the day’s complete input helpers. Supply only the unchanged original curve validation fixtures.

**Locate this existing block:**

```rust
use std::rc::Rc;

const SAMPLES: usize = 32; // preview segments

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
        return part
            .sizes
            .first()
            .map_or_else(Vec::new, |length| {
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
            let (a, b, c) = (
                (1.0 - t) * (1.0 - t),
                2.0 * t * (1.0 - t),
                t * t,
            );
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

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        return part
            .sizes
            .first()
            .map_or_else(Vec::new, |length| {
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
            let (a, b, c) = (
                (1.0 - t) * (1.0 - t),
                2.0 * t * (1.0 - t),
                t * t,
            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::tests::{XY, build, p, plane};

    /// It passes start, apex half way and end; an apex on the chord is refused.
    #[test]
    fn the_parabola_passes_its_three_points() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(1000.0),
            Answer::Number(400.0),
        ];
        let Geometry::NurbsCurve(curve) = build(&SHAPE, &top, &answers, "").unwrap()
        else {
            panic!()
        };
        let (a, b) = curve.domain();
        assert_eq!(curve.cv_count(), 3);
        assert!(
            curve
                .point_at(a)
                .distance(&p(0.0, 0.0, 0.0), None)
                < 1e-9
        );
        assert!(
            curve
                .point_at(0.5 * (a + b))
                .distance(&p(500.0, 400.0, 0.0), None)
                < 1e-9
        );
        assert!(
            curve
                .point_at(b)
                .distance(&p(1000.0, 0.0, 0.0), None)
                < 1e-9
        );
        let flat = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(5.0, 0.0, 0.0)),
        ];
        assert!(build(&SHAPE, &top, &flat, "").is_err());
    }
}
```

## <a id="checkpoint-76a-expose-exact-curve-choices-in-the-existing-registry"></a>2. Expose exact curve choices in the existing registry

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/primitive_registry.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">registered curve name</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current option</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared answer owner</text>
</svg>
</div>

**What this code does:** Expose exact curve choices in the existing registry.

### 1. `src/primitive_registry.rs`

Register all current exact planar curve names and supported Arc options.

**Locate this existing block:**

```rust
    ),
    ("bo", &crate::primitive_box::SHAPE, 0),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ),
    (
        "Nurbs Curve Circle",
        &crate::freeform_circle::SHAPE,
        0,
    ),
    (
        "Nurbs Curve Ellipse",
        &crate::freeform_ellipse::SHAPE,
        0,
    ),
    (
        "Nurbs Curve Arc",
        &crate::freeform_arc::SHAPE,
        0,
    ),
    (
        "Nurbs Curve Arc Center",
        &crate::freeform_arc::SHAPE,
        0,
    ),
    (
        "Nurbs Curve Arc 3 Points",
        &crate::freeform_arc::SHAPE,
        1,
    ),
    (
        "Nurbs Curve Parabola",
        &crate::freeform_parabola::SHAPE,
        0,
    ),
    ("bo", &crate::primitive_box::SHAPE, 0),
```

## <a id="checkpoint-76b-retain-editable-freeform-controls-in-the-point-collector"></a>3. Create editable freeform curves

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/freeform_curve.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">collected original contro…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">NURBS curve</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history edit</text>
</svg>
</div>

**What this code does:** Write the original degree and closure rules for collected control points.

### 1. `src/freeform_curve.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use session_rust::{NurbsCurve, Point};
/// A curve through control points; ending on the start closes it smoothly.
pub fn curve(points: &[Point]) -> NurbsCurve {
    let count = points.len();
    let closed = count >= 4 && points[0].distance(&points[count - 1], None) <= 1e-12;

    if closed {
        return NurbsCurve::create(
            true,
            (count - 2).min(3),
            &points[..count - 1],
        );
    }

    NurbsCurve::create(false, (count - 1).min(3), points)
}
```

### 2. `src/lib.rs`

Register the original freeform-curve constructor.

**Locate this existing block:**

```rust
pub mod freeform_parabola;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod freeform_parabola;

pub mod freeform_curve;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/freeform_curve.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    NurbsCurve::create(false, (count - 1).min(3), points)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    NurbsCurve::create(false, (count - 1).min(3), points)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A curve that ends on its start closes without a kink.
    #[test]
    fn a_curve_ending_on_its_start_is_closed() {
        let square = [
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(0.0, 10.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ];
        let closed = curve(&square);
        assert!(closed.is_valid() && closed.is_closed());
        assert!(
            closed
                .point_at_start()
                .distance(&closed.point_at_end(), None)
                < 1e-9
        );
        let triangle = curve(&[
            square[0].clone(),
            square[1].clone(),
            square[2].clone(),
            square[0].clone(),
        ]);
        assert!(triangle.is_valid() && triangle.is_closed());
        let open = curve(&square[..4]);
        assert!(open.is_valid() && !open.is_closed());
    }
}
```

### 4. `src/tool.rs`

Build original freeform curves from accepted controls and offer existing Finish/Close input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        session_rust::Polyline::new(points),
                    ))?,
                ))
```

Replace it with:

```rust
                        session_rust::Polyline::new(points),
                    ))?,
                ))
            }
            (Self::Curve, points) if (2..=MAX_POINTS).contains(&points.len()) => {
                if !points.windows(2).any(|p| p[0] != p[1]) {
                    return Err("Curve controls must differ");
                }
                let controls = points
                    .iter()
                    .map(|p| session_rust::Point::new(p[0], p[1], p[2]))
                    .collect::<Vec<_>>();
                let source = crate::freeform_curve::curve(&controls);
                Ok(Built::Geometry(
                    session_rust::Geometry::NurbsCurve(Rc::new(source)),
                ))
```

**Locate this block:**

```rust
                buttons.push((name.into(), format!("Polyline {name}")));
            }
        }
        if matches!(self, Self::Polyline) {
            buttons.insert(0, ("Close".into(), "Close".into()));
        }
        buttons
```

Replace it with:

```rust
                buttons.push((name.into(), format!("Polyline {name}")));
            }
        }
        if matches!(self, Self::Polyline | Self::Curve) {
            buttons.insert(0, ("Close".into(), "Close".into()));
        }
        buttons
```

**Locate this block:**

```rust
        match self {
            Self::Point => 1,
            Self::Line | Self::Move => 2,
            Self::Polyline => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) => 0,
        }
```

Replace it with:

```rust
        match self {
            Self::Point => 1,
            Self::Line | Self::Move => 2,
            Self::Polyline | Self::Curve => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) => 0,
        }
```

**Locate this block:**

```rust
            Self::Line => "Line",
            Self::Move => "Move",
            Self::Polyline => "Polyline",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
```

Replace it with:

```rust
            Self::Line => "Line",
            Self::Move => "Move",
            Self::Polyline => "Polyline",
            Self::Curve => "Curve",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
```

**Locate this block:**

```rust
    Polyline,
    Rectangle,
    Polygon { sides: usize },
    Primitive(&'static crate::primitive_shape::Shape),
```

Replace it with:

```rust
    Polyline,
    Curve,
    Rectangle,
    Polygon { sides: usize },
    Primitive(&'static crate::primitive_shape::Shape),
```

### 5. `src/commands.rs`

Route current Curve invocation and direct controls through the existing world-point collector.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    {
        let shape = match name {
            "Point" => crate::tool::Shape::Point,
            "Line" => crate::tool::Shape::Line,
```

Replace it with:

```rust
    {
        let shape = match name {
            "Point" => crate::tool::Shape::Point,
            "Curve" => crate::tool::Shape::Curve,
            "Line" => crate::tool::Shape::Line,
```

**Locate this block:**

```rust
        name,
        "Point"
            | "Line"
            | "Polyline"
            | "Polyline Points"
            | "Polyline Rectangle"
```

Replace it with:

```rust
        name,
        "Point"
            | "Line"
            | "Curve"
            | "Polyline"
            | "Polyline Points"
            | "Polyline Rectangle"
```

**Locate this block:**

```rust
    }),
    ("Polyline", || {
        Action::BeginTool(crate::tool::Shape::Polyline)
    }),
```

Replace it with:

```rust
    }),
    ("Curve", || {
        Action::BeginTool(crate::tool::Shape::Curve)
    }),
    ("Polyline", || {
        Action::BeginTool(crate::tool::Shape::Polyline)
    }),
```

## <a id="checkpoint-76c-inspect-original-curve-data-through-the-browser-contract"></a>4. Inspect curve data through the browser contract

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original curve owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit diagnostic facts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">browser assertions</text>
</svg>
</div>

**What this code does:** Inspect original curve data through the browser contract.

### 1. `src/browser.rs`

Expose original curve identity, weighted controls and knots for current browser acceptance.

**Locate this existing block:**

```rust
    let originals = serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let originals = serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute(
```

## <a id="checkpoint-76d-exercise-exact-curves-and-rational-control-edits"></a>5. Exercise exact curves and rational control edits

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/freeform_tests.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original curve</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one transaction</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/cplane.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact restoration</text>
</svg>
</div>

**What this code does:** Exercise exact curves and rational control edits.

### 1. `src/freeform_tests.rs`

Copy integrated exact-original, closure/world-replay, failed-input, rational-edit and archive fixtures.

Create the file and type:

```rust
use crate::{
    chain::Source,
    editor::{Action, Editor},
};
use session_rust::Xform;
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, line: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(line) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(line)? else {
            return Err("Editor command".into());
        };
        a
    };
    e.apply(action).map_err(|message| {
        if message == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            message.into()
        }
    })?;
    Ok(())
}
fn curve(e: &Editor, index: usize) -> Rc<session_rust::NurbsCurve> {
    let Source::Curve(source, _) = &e.scene.paths()[index].prepared.source else {
        panic!("Original curve")
    };
    Rc::clone(source)
}
#[test]
fn current_curve_families_preserve_original_controls_knots_weights_and_archive_identity()
 {
    let mut e = editor();
    for line in [
        "Nurbs Curve Circle w0,0,0 1",
        "Nurbs Curve Ellipse w3,0,0 2 1",
        "Nurbs Curve Arc Center w6,0,0 1 270",
        "Nurbs Curve Arc 3 Points w8,0,0 w10,0,0 w9,1,0",
        "Nurbs Curve Parabola w12,0,0 2 1",
        "Curve w15,0,0 w16,1,0 w17,0,0",
    ] {
        let count = e.scene.paths().len();
        command(&mut e, line).unwrap();
        assert!(e.draft.is_none());
        command(&mut e, "Undo").unwrap();
        assert_eq!(e.scene.paths().len(), count);
        command(&mut e, "Redo").unwrap();
        assert_eq!(e.scene.paths().len(), count + 1);
    }
    let originals = (0..6)
        .map(|i| curve(&e, i))
        .collect::<Vec<_>>();
    let archive = crate::scene_archive::save(&e.scene).unwrap();
    e.apply(Action::Close).unwrap();
    crate::scene_document::open(&mut e.scene, &archive).unwrap();
    for source in originals {
        let saved = e
            .scene
            .paths()
            .iter()
            .find_map(|row| match &row.prepared.source {
                Source::Curve(c, _) if c.guid() == source.guid() => Some(c),
                _ => None,
            })
            .unwrap();
        assert_eq!(saved.m_cv, source.m_cv);
        assert_eq!(saved.m_nurbsknot, source.m_nurbsknot);
        assert_eq!(saved.m_is_rat, source.m_is_rat);
    }
}
#[test]
fn closed_curve_replays_world_points_once_and_keeps_an_original_smooth_curve() {
    let mut e = editor();
    e.apply(Action::ConstructionPlane(Some(
        crate::cplane::CPlane::Xz,
    )))
    .unwrap();
    command(&mut e, "Curve").unwrap();
    command(&mut e, "0,0,0 2,0,0 2,2,0 0,2,0").unwrap();
    assert!(e.scene.paths().is_empty());
    command(&mut e, "Close").unwrap();
    let source = curve(&e, 0);
    assert!(source.is_closed());
    for i in 0..source.cv_count() {
        assert!(source.get_cv(i).unwrap()[1].abs() < 1e-12);
    }
    assert!(
        source
            .point_at_start()
            .distance(&source.point_at_end(), None)
            < 1e-9
    );
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.paths().is_empty());
    command(&mut e, "Redo").unwrap();
    assert!(Rc::ptr_eq(&curve(&e, 0), &source));
}
#[test]
fn failed_exact_curve_input_keeps_answers_previews_and_redo_without_scene_mutation() {
    let mut e = editor();
    command(&mut e, "Curve w0,0,0 w1,1,0 w2,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Nurbs Curve Arc Center").unwrap();
    command(&mut e, "w0,0,0 1").unwrap();
    assert!(command(&mut e, "360").is_err());
    assert_eq!(
        e.draft
            .as_ref()
            .unwrap()
            .primitive
            .as_ref()
            .unwrap()
            .answers
            .len(),
        2
    );
    assert_eq!(e.draft.as_ref().unwrap().outlines().len(), 1);
    assert!(e.scene.paths().is_empty());
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.paths().len(), 1);
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Nurbs Curve Parabola w0,0,0 2 0").is_err());
    assert_eq!(
        e.draft.as_ref().unwrap().tool.name(),
        "Curve"
    );
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
}
#[test]
fn editing_a_created_rational_arc_preserves_weights_knots_other_controls_and_undo() {
    let mut e = editor();
    command(&mut e, "Nurbs Curve Arc Center w0,0,0 1 90").unwrap();
    let source = curve(&e, 0);
    assert!(source.m_is_rat);
    e.apply(Action::Controls(true)).unwrap();
    let id = e.scene.paths()[0].id;
    let target = crate::pick_sources::Target {
        owner: id,
        part: crate::pick_sources::Part::Control(1),
    };
    let mut preview = crate::part_preview::Preview::new(&e.scene, target).unwrap();
    preview
        .set(Xform::translation(0.125, 0.25, 0.))
        .unwrap();
    let Source::Curve(edited, _) = &preview.scene.paths()[0].prepared.source else {
        panic!("Original curve")
    };
    assert_eq!(edited.m_nurbsknot, source.m_nurbsknot);
    for i in 0..source.cv_count() {
        assert_eq!(
            edited.get_cv_4d(i).unwrap().3,
            source.get_cv_4d(i).unwrap().3
        );
        let expected = if i == 1 {
            &source.get_cv(i).unwrap() + &session_rust::Vector::new(0.125, 0.25, 0.)
        } else {
            source.get_cv(i).unwrap()
        };
        assert!(
            edited
                .get_cv(i)
                .unwrap()
                .distance(&expected, None)
                < 1e-12
        );
    }
    assert!(Rc::ptr_eq(&curve(&e, 0), &source));
    let mut history = crate::history::History::default();
    assert!(
        preview
            .commit(&mut e.scene, &mut history)
            .unwrap()
    );
    assert!(history.undo(&mut e.scene));
    assert!(Rc::ptr_eq(&curve(&e, 0), &source));
    assert!(history.redo(&mut e.scene));
    assert!(!Rc::ptr_eq(&curve(&e, 0), &source));
}
```

### 2. `src/lib.rs`

Register the current curve component.

**Locate this existing block:**

```rust
pub mod freeform_curve;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod freeform_curve;

#[cfg(test)]
mod freeform_tests;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 14.6 (76d-exercise-exact-curves-and-rational-control-edits)](../../screenshots/journey/76d-exercise-exact-curves-and-rational-control-edits.png)

```sh
npm --prefix ../session_tests run course -- dependencies 14.6
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 14.6
npm --prefix ../session_tests run course -- save 14.6
```

Continue with [15.1 · Original transforms, part edits and solid recipes](77-copy-current-transformation-and-source-edit-algorithms.md).
