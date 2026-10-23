# 5.4 · NURBS source ownership and derived samples

**Goal:** Insert an original NURBS curve through real input and distinguish its actual curve pixels from its indexed control markers. · **Steps:** 5 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl) · [app/walk/points.rs](../../../../src/app/walk/points.rs) · [shaders/glyph.wgsl](../../../../src/shaders/glyph.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/grid.wgsl](../../../../src/shaders/grid.wgsl) (`src/shaders/ribbon.wgsl,src/shaders/glyph.wgsl,src/engine/gpu/segments.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 5.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/curve.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/curve_layout_tests.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">5 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ribbon.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/glyph.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Validate an NURBS curve b…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Screen-Space Ribbon Quad</text>
  <polygon points="606,112 764,56 782,94 624,146" fill="#bfdbfe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="615" y1="129" x2="773" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="615" cy="129" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <circle cx="773" cy="75" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <line x1="694" y1="102" x2="682" y2="72" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="664" y="70" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">NURBS source ownership and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGBA Ink &amp; SDF Edge AA</text>
  <line x1="884" y1="136" x2="1064" y2="136" stroke="#111111" stroke-width="1.5"/>
  <path d="M 896 136 L 928 68 L 1020 68 L 1052 136" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <circle cx="916" cy="54" r="6" fill="#ef4444" stroke="#111111" stroke-width="1.25"/>
  <circle cx="936" cy="54" r="6" fill="#22c55e" stroke="#111111" stroke-width="1.25"/>
  <circle cx="956" cy="54" r="6" fill="#3b82f6" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1020" y="58" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">RGBA + SDF</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Insert an curve and inspe…</text>
</svg>
</div>

## <a id="checkpoint-38-layout"></a>1. Validate an NURBS curve before sampling

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/curve.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original NURBS curve</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite positive weights</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/ribbon.…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">nonempty domain</text>
</svg>
</div>

**What this code does:** Validate public kernel curve layout and domain before evaluating an original source.

### 1. `src/curve.rs`

Guard layout arithmetic and public arrays before original kernel evaluation.

Create the file and type:

```rust
use session_rust::NurbsCurve;

pub fn validate(source: &NurbsCurve) -> Result<(f64, f64), &'static str> {
    if !(2..=3).contains(&source.m_dim)
        || !(2..=10_000).contains(&source.m_cv_count)
        || !(2..=32).contains(&source.m_order)
        || source.m_order > source.m_cv_count
    {
        return Err("Curve dimension, count or order is invalid");
    }
    let size = source.m_dim + usize::from(source.m_is_rat);
    if source.m_cv_stride < size {
        return Err("Curve control stride is too short");
    }
    let end = (source.m_cv_count - 1)
        .checked_mul(source.m_cv_stride)
        .and_then(|offset| offset.checked_add(size))
        .ok_or("Curve control range overflows")?;
    if source.m_cv.len() < end {
        return Err("Curve control values are incomplete");
    }
    for index in 0..source.m_cv_count {
        let start = index * source.m_cv_stride;
        let values = &source.m_cv[start..start + size];
        if values.iter().any(|v| !v.is_finite()) {
            return Err("Curve controls must be finite");
        }
        if source.m_is_rat && values[source.m_dim] <= 0.0 {
            return Err("Curve display requires positive rational weights");
        }
    }
    let count = source.m_cv_count + source.m_order - 2;
    if source.m_nurbsknot.len() != count
        || source
            .m_nurbsknot
            .iter()
            .any(|v| !v.is_finite())
        || source
            .m_nurbsknot
            .windows(2)
            .any(|pair| pair[0] > pair[1])
    {
        return Err("Curve knot vector is incomplete or unordered");
    }
    let domain = source.domain();
    if domain.0 >= domain.1 || !(domain.1 - domain.0).is_finite() {
        return Err("Curve domain must have positive length");
    }
    Ok(domain)
}
```

### 2. `src/lib.rs`

Register source validation and malformed-input checks.

**Locate this existing block:**

```rust
pub mod controls;
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod controls;
pub mod curve;
#[cfg(test)]
mod curve_layout_tests;
pub mod diagnostic;
```

### 3. `src/curve_layout_tests.rs`

Copy valid original domains, unchanged sources and malformed-layout refusal checks.

Create the file and type:

```rust
use crate::curve::validate;
use session_rust::{NurbsCurve, Point};

fn curve(periodic: bool) -> NurbsCurve {
    NurbsCurve::create(
        periodic,
        2,
        &[
            Point::new(-0.6, -0.4, 0.0),
            Point::new(0.0, 0.6, 0.0),
            Point::new(0.6, -0.4, 0.0),
        ],
    )
}

#[test]
fn valid_curve_domains_keep_original_knots_and_controls() {
    for periodic in [false, true] {
        let source = curve(periodic);
        let cvs = source.m_cv.clone();
        let knots = source.m_nurbsknot.clone();
        let domain = validate(&source).unwrap();
        assert!(domain.0 < domain.1);
        assert_eq!(source.m_cv, cvs);
        assert_eq!(source.m_nurbsknot, knots);
    }
    let mut rational = NurbsCurve::new(3, true, 2, 2);
    rational.m_cv = vec![0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 1.0];
    rational.m_nurbsknot = vec![0.0, 1.0];
    assert_eq!(validate(&rational).unwrap(), (0.0, 1.0));
}

#[test]
fn malicious_curve_layouts_return_errors_before_kernel_evaluation() {
    for case in 0..9 {
        let mut source = curve(false);
        match case {
            0 => source.m_cv_count = usize::MAX,
            1 => source.m_cv_stride = usize::MAX,
            2 => source.m_order = usize::MAX,
            3 => source.m_cv.clear(),
            4 => source.m_nurbsknot.clear(),
            5 => source.m_nurbsknot[0] = f64::NAN,
            6 => source.m_cv[0] = f64::INFINITY,
            7 => source.m_nurbsknot.fill(0.0),
            _ => source.m_nurbsknot[0] = 1e6,
        }
        assert!(validate(&source).is_err());
    }
    let mut rational = NurbsCurve::new(3, true, 2, 2);
    rational.m_nurbsknot = vec![0.0, 1.0];
    rational.m_cv = vec![0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0];
    assert!(validate(&rational).is_err());
}
```

## <a id="checkpoint-38a-samples"></a>2. Sample a validated curve without replacing its source

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/curve.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated original curve</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded parameter samples…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/ribbon.…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite double-precision c…</text>
</svg>
</div>

**What this code does:** Build a bounded display approximation from an original curve and retain exact source controls separately.

### 1. `src/curve.rs`

Sample the original evaluator with bounded uniform chords and exact interior knot breaks.

**Locate this existing block:**

```rust
    Ok(domain)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(domain)
}

pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0;
    let mut previous: Option<[f64; 3]> = None;
    let control = |index: usize| {
        let offset = index * source.m_cv_stride;
        let weight = if source.m_is_rat {
            source.m_cv[offset + source.m_dim]
        } else {
            1.0
        };
        std::array::from_fn::<_, 3, _>(|axis| {
            if axis < source.m_dim {
                source.m_cv[offset + axis] / weight
            } else {
                0.0
            }
        })
    };
    for index in 1..source.m_cv_count {
        let a = control(index - 1);
        let b = control(index);
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction
            .iter()
            .map(|v| v * v)
            .sum::<f64>()
            .sqrt();
        if length <= 1e-12 {
            continue;
        }
        if !length.is_finite() {
            return Err("Curve control extent exceeds the display range");
        }
        let unit = direction.map(|v| v / length);
        if let Some(before) = previous {
            let dot = (0..3)
                .map(|axis| before[axis] * unit[axis])
                .sum::<f64>()
                .clamp(-1.0, 1.0);
            turning += dot.acos().to_degrees();
        }
        previous = Some(unit);
    }
    let count = ((turning / 3.0).ceil() as usize).clamp(1, 512);
    let mut parameters: Vec<_> = (0..=count)
        .map(|i| start + (end - start) * i as f64 / count as f64)
        .collect();
    parameters.extend(
        source
            .m_nurbsknot
            .iter()
            .copied()
            .filter(|&t| t > start && t < end),
    );
    parameters.sort_by(f64::total_cmp);
    parameters.dedup();
    let mut points = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let point = source.point_at(parameter);
        let point: [f64; 3] = std::array::from_fn(|axis| point[axis]);
        if point
            .iter()
            .any(|&v| !(v as f32).is_finite())
        {
            return Err("Sampled curve exceeds the display range");
        }
        points.push(point);
    }
    Ok(points)
}
```

### 2. `src/lib.rs`

Register actual sampled geometry checks.

**Locate this existing block:**

```rust
mod curve_layout_tests;
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod curve_layout_tests;
#[cfg(test)]
mod curve_sample_tests;
pub mod diagnostic;
```

### 3. `src/curve_sample_tests.rs`

Copy original preservation, dense measured error, periodic closure and knot-break checks.

Create the file and type:

```rust
use crate::curve::sample;
use session_rust::{NurbsCurve, Point};

#[test]
fn sampling_draws_the_curve_and_keeps_exact_original_controls() {
    let controls = [
        Point::new(-0.6, -0.4, 0.0),
        Point::new(0.0, 0.6, 0.0),
        Point::new(0.6, -0.4, 0.0),
    ];
    let source = NurbsCurve::create(false, 2, &controls);
    let original = source.m_cv.clone();
    let points = sample(&source).unwrap();
    assert!(points.len() > 20 && points.len() <= 513);
    assert_eq!(source.m_cv, original);
    assert_eq!(points.first().unwrap(), &[-0.6, -0.4, 0.0]);
    assert_eq!(points.last().unwrap(), &[0.6, -0.4, 0.0]);
    let apex = points
        .iter()
        .map(|p| p[1])
        .fold(f64::NEG_INFINITY, f64::max);
    assert!((apex - 0.1).abs() < 0.001);
    assert!(apex < controls[1][1] - 0.4);
    let (a, b) = source.domain();
    for i in 0..=1000 {
        let p = source.point_at(a + (b - a) * i as f64 / 1000.0);
        let error = points
            .windows(2)
            .map(|pair| {
                let d = [
                    pair[1][0] - pair[0][0],
                    pair[1][1] - pair[0][1],
                ];
                let length = d[0] * d[0] + d[1] * d[1];
                let t = (((p[0] - pair[0][0]) * d[0] + (p[1] - pair[0][1]) * d[1])
                    / length)
                    .clamp(0.0, 1.0);
                (p[0] - pair[0][0] - d[0] * t).hypot(p[1] - pair[0][1] - d[1] * t)
            })
            .fold(f64::INFINITY, f64::min);
        assert!(
            error < 0.001,
            "Measured chord error: {error}"
        );
    }
}

#[test]
fn straight_and_periodic_curves_keep_required_endpoints_and_knots() {
    let source = NurbsCurve::create(
        false,
        2,
        &[
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.5, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ],
    );
    assert_eq!(
        sample(&source).unwrap(),
        vec![[0.0; 3], [1.0, 0.0, 0.0]]
    );
    let points = [
        Point::new(-0.6, -0.4, 0.0),
        Point::new(0.0, 0.6, 0.0),
        Point::new(0.6, -0.4, 0.0),
    ];
    let periodic = NurbsCurve::create(true, 2, &points);
    let sampled = sample(&periodic).unwrap();
    assert!(sampled.len() <= 10_513);
    let first = sampled.first().unwrap();
    let last = sampled.last().unwrap();
    assert!((0..3).all(|axis| (first[axis] - last[axis]).abs() < 1e-10));
    let corners = NurbsCurve::create(false, 1, &points);
    let sampled = sample(&corners).unwrap();
    assert!(
        sampled
            .iter()
            .any(|p| (p[0] - points[1][0]).abs() < 1e-12
                && (p[1] - points[1][1]).abs() < 1e-12)
    );
    let mut invalid = corners.clone();
    invalid.m_cv_stride = usize::MAX;
    assert!(sample(&invalid).is_err());
}
```

## <a id="checkpoint-38b-source"></a>3. Retain curves beside their display samples

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/chain.rs, src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original NURBS Rc</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">path row and checked ID</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/ribbon.…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed display chords and…</text>
</svg>
</div>

**What this code does:** Retain original NURBS ownership, cached double samples and placed curve bounds in the existing path document.

### 1. `src/chain.rs`

Retain the original curve separately from shared approximation samples. Keep original curve identity under path ownership. Count curve source owners independently of sampled data. Use cached original evaluator positions when placing display chords. Keep stable original control indices and a conservative positive-weight hull. Validate and cache the original curve before accepting connected display preparation. Use the original curve arrowhead policy.

**Locate this existing block:**

```rust
    Polyline(Rc<Polyline>),
}

#[derive(Clone)]
pub struct PreparedChain {
    pub source: Source,
    pub points: Rc<Vec<[f32; 3]>>,
    pub colour: [f32; 4],
    pub width: f32,
    pub heads: u32,
}

impl Source {
    pub fn guid(&self) -> &str {
        match self {
            Self::Line(line) => line.guid(),
            Self::Polyline(line) => line.guid(),
        }
    }

    pub fn owner(&self) -> (u8, usize) {
        match self {
            Self::Line(line) => (0, Rc::as_ptr(line) as usize),
            Self::Polyline(line) => (1, Rc::as_ptr(line) as usize),
        }
    }

    pub fn coordinates(&self) -> Vec<[f64; 3]> {
        match self {
            Self::Line(line) => [line.start(), line.end()]
                .map(|p| [p[0], p[1], p[2]])
                .to_vec(),
            Self::Polyline(line) => line
                .coords
                .chunks_exact(3)
                .map(|p| [p[0], p[1], p[2]])
                .collect(),
        }
    }
}

impl PreparedChain {
    pub fn line(source: Rc<Line>) -> Result<Self, &'static str> {
        let display = PreparedLine::new(Rc::clone(&source))?.display;
        Self::new(
            Source::Line(source),
            display.colour,
            display.width,
        )
    }

    pub fn polyline(source: Rc<Polyline>) -> Result<Self, &'static str> {
        if source.coords.len() % 3 != 0 || !(6..=30_000).contains(&source.coords.len())
        {
            return Err("Polyline needs 2–10,000 complete points");
        }
        let _ = source.guid();
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.linecolor = source.linecolor.clone();
        line.width = source.width;
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(
            Source::Polyline(source),
            display.colour,
            display.width,
        )
    }

    fn new(source: Source, colour: [f32; 4], width: f32) -> Result<Self, &'static str> {
        let heads = match &source {
            Source::Line(line) => line.arrowhead,
            Source::Polyline(line) => line.arrowhead,
        };
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Polyline(Rc<Polyline>),
    Curve(
        Rc<session_rust::NurbsCurve>,
        Rc<Vec<[f64; 3]>>,
    ),
}

#[derive(Clone)]
pub struct PreparedChain {
    pub source: Source,
    pub points: Rc<Vec<[f32; 3]>>,
    pub colour: [f32; 4],
    pub width: f32,
    pub heads: u32,
}

impl Source {
    pub fn guid(&self) -> &str {
        match self {
            Self::Line(line) => line.guid(),
            Self::Polyline(line) => line.guid(),
            Self::Curve(curve, _) => curve.guid(),
        }
    }

    pub fn owner(&self) -> (u8, usize) {
        match self {
            Self::Line(line) => (0, Rc::as_ptr(line) as usize),
            Self::Polyline(line) => (1, Rc::as_ptr(line) as usize),
            Self::Curve(curve, _) => (2, Rc::as_ptr(curve) as usize),
        }
    }

    pub fn controls(&self) -> Vec<crate::controls::Control> {
        match self {
            Self::Curve(curve, _) => (0..curve.m_cv_count)
                .map(|index| {
                    crate::controls::Control::new(Rc::clone(curve), index)
                        .expect("Validated original curve control")
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn bounds_coordinates(&self) -> Vec<[f64; 3]> {
        if matches!(self, Self::Curve(..)) {
            self.controls()
                .iter()
                .map(|c| c.position().unwrap())
                .collect()
        } else {
            self.coordinates()
        }
    }

    pub fn coordinates(&self) -> Vec<[f64; 3]> {
        match self {
            Self::Line(line) => [line.start(), line.end()]
                .map(|p| [p[0], p[1], p[2]])
                .to_vec(),
            Self::Polyline(line) => line
                .coords
                .chunks_exact(3)
                .map(|p| [p[0], p[1], p[2]])
                .collect(),
            Self::Curve(_, samples) => samples.as_ref().clone(),
        }
    }
}

impl PreparedChain {
    pub fn curve(source: Rc<session_rust::NurbsCurve>) -> Result<Self, &'static str> {
        let samples = Rc::new(crate::curve::sample(&source)?);
        let _ = source.guid();
        for index in 0..source.m_cv_count {
            crate::controls::Control::new(Rc::clone(&source), index)?.marker()?;
        }
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.width = source.width;
        line.linecolor = source
            .linecolors
            .first()
            .cloned()
            .unwrap_or_else(|| session_rust::Color::new(0.0, 0.0, 0.0, 1.0));
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(
            Source::Curve(source, samples),
            display.colour,
            display.width,
        )
    }

    pub fn line(source: Rc<Line>) -> Result<Self, &'static str> {
        let display = PreparedLine::new(Rc::clone(&source))?.display;
        Self::new(
            Source::Line(source),
            display.colour,
            display.width,
        )
    }

    pub fn polyline(source: Rc<Polyline>) -> Result<Self, &'static str> {
        if source.coords.len() % 3 != 0 || !(6..=30_000).contains(&source.coords.len())
        {
            return Err("Polyline needs 2–10,000 complete points");
        }
        let _ = source.guid();
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.linecolor = source.linecolor.clone();
        line.width = source.width;
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(
            Source::Polyline(source),
            display.colour,
            display.width,
        )
    }

    fn new(source: Source, colour: [f32; 4], width: f32) -> Result<Self, &'static str> {
        let heads = match &source {
            Source::Line(line) => line.arrowhead,
            Source::Polyline(line) => line.arrowhead,
            Source::Curve(curve, _) => curve.arrowhead,
        };
```

### 2. `src/scene.rs`

Place conservative curve bounds from original double controls. Include the complete placed control hull in document Fit bounds.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
        }
        for row in &self.paths {
            for point in row.points() {
                let point = [point[0], point[1], point[2]];
```

Replace it with:

```rust
            }
        }
        for row in &self.paths {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
```

**Locate this block:**

```rust
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared
            .source
```

Replace it with:

```rust
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared
            .source
            .bounds_coordinates()
            .into_iter()
            .map(|p| {
                self.model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
            })
            .collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared
            .source
```

### 3. `src/memory.rs`

Count sampled doubles separately from source owners and packed GPU data.

**Locate this existing block:**

```rust
}

pub fn paths<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn curves<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 4] {
    let mut sources = HashSet::new();
    let mut samples = HashSet::new();
    let mut usage = [0; 4];
    for scene in scenes {
        for row in scene.paths() {
            if let crate::chain::Source::Curve(source, points) = &row.prepared.source {
                usage[0] += 1;
                sources.insert(Rc::as_ptr(source));
                if samples.insert(Rc::as_ptr(points)) {
                    usage[3] += points.capacity() * 24;
                }
            }
        }
    }
    usage[1] = sources.len();
    usage[2] = samples.len();
    usage
}

pub fn paths<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
```

### 4. `src/lib.rs`

Register retained source and placed bounds checks.

**Locate this existing block:**

```rust
mod curve_sample_tests;
pub mod diagnostic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod curve_sample_tests;
#[cfg(test)]
mod curve_source_tests;
pub mod diagnostic;
```

### 5. `src/curve_source_tests.rs`

Copy retained source identity, distinct IDs, precise placement, conservative bounds and release checks.

Create the file and type:

```rust
use crate::{
    chain::{PreparedChain, Source},
    memory,
    scene::Scene,
};
use session_rust::{NurbsCurve, Point, Xform};
use std::rc::Rc;

#[test]
fn curve_rows_share_original_sources_and_samples_with_distinct_ids() {
    let curve = NurbsCurve::create(
        false,
        2,
        &[
            Point::new(-0.6, -0.4, 0.0),
            Point::new(0.123456789012345, 0.6, 0.0),
            Point::new(0.6, -0.4, 0.0),
        ],
    );
    let source = Rc::new(curve);
    let weak = Rc::downgrade(&source);
    let guid = source.guid().to_owned();
    let prepared = PreparedChain::curve(source).unwrap();
    let Source::Curve(_, samples) = &prepared.source else {
        panic!("Original curve source")
    };
    let sample_weak = Rc::downgrade(samples);
    let mut scene = Scene::demo();
    scene.clear();
    let first = scene.insert_path(prepared.clone()).unwrap();
    let second = scene.insert_path(prepared).unwrap();
    assert_ne!(first, second);
    assert_eq!(scene.paths()[0].guid, guid);
    assert_ne!(scene.paths()[1].guid, guid);
    assert_eq!(&memory::curves([&scene])[..3], &[2, 1, 1]);
    let controls = scene.paths()[0].prepared.source.controls();
    assert_eq!(controls[1].index, 1);
    assert_eq!(
        controls[1].position().unwrap()[0],
        0.123456789012345
    );
    scene
        .place_path(first, Xform::translation(2.0, 3.0, 4.0))
        .unwrap();
    let bounds = scene.bounds().unwrap();
    assert!(bounds.max[1] >= 3.6);
    assert!(bounds.max[2] >= 4.0);
    assert!(scene.paths()[0].segments().len() > 20);
    assert_eq!(controls[1].source.m_cv[3], 0.123456789012345);
    drop(controls);
    scene.clear();
    assert!(weak.upgrade().is_none());
    assert!(sample_weak.upgrade().is_none());
    assert_eq!(memory::curves([&scene]), [0; 4]);
}
```

## <a id="checkpoint-38c-controls"></a>4. Draw controls with their curve parent and index

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/renderer.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained curve path row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">parent/index marker recor…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/ribbon.…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Close release</text>
</svg>
</div>

**What this code does:** Draw enabled curve controls through the real marker lane while retaining original parent IDs and control indices.

### 1. `src/scene.rs`

Retain explicit control display state on each document path row. Keep original curve controls disabled until requested. Reject invalid placed controls before accepting a curve row transform. Carry document parent and original control index beside each placed marker. Resolve placed original controls with stable parent IDs and control indices.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Path not found")?;
        for p in row.prepared.source.coordinates() {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
```

Replace it with:

```rust
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Path not found")?;
        for p in row
            .prepared
            .source
            .coordinates()
            .into_iter()
            .chain(row.prepared.source.bounds_coordinates())
        {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
```

**Locate this block:**

```rust
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.paths.push(PathObject {
            id,
            guid,
            prepared,
```

Replace it with:

```rust
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.paths.push(PathObject {
            controls_visible: false,
            id,
            guid,
            prepared,
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }
```

Replace it with:

```rust
        Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths {
            row.controls_visible = visible
                && matches!(
                    row.prepared.source,
                    crate::chain::Source::Curve(..)
                );
        }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self
            .paths
            .iter()
            .filter(|row| row.controls_visible)
        {
            for control in row.prepared.source.controls() {
                let p = control
                    .position()
                    .expect("Validated original control");
                let p = row
                    .model
                    .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker {
                    center: p.to_f32(),
                    ..control
                        .marker()
                        .expect("Prepared control marker")
                };
                markers.push(ControlMarker {
                    parent: row.id,
                    control,
                    display,
                });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] {
        &self.paths
    }
```

**Locate this block:**

```rust
    }
}

#[derive(Clone)]
pub struct PointObject {
    pub id: ObjectId,
```

Replace it with:

```rust
    }
}

pub struct ControlMarker {
    pub parent: ObjectId,
    pub control: crate::controls::Control,
    pub display: crate::marker::Marker,
}

#[derive(Clone)]
pub struct PointObject {
    pub id: ObjectId,
```

**Locate this block:**

```rust
pub struct PathObject {
    pub id: ObjectId,
    pub guid: String,
    pub prepared: crate::chain::PreparedChain,
```

Replace it with:

```rust
pub struct PathObject {
    pub controls_visible: bool,
    pub id: ObjectId,
    pub guid: String,
    pub prepared: crate::chain::PreparedChain,
```

### 2. `src/renderer.rs`

Send only placed display bytes to the marker lane, retaining source ownership in the document.

**Locate this existing block:**

```rust
                    .iter()
                    .map(crate::scene::PointObject::marker),
            )
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    .iter()
                    .map(crate::scene::PointObject::marker)
                    .chain(
                        scene
                            .control_markers()
                            .into_iter()
                            .map(|c| c.display),
                    ),
            )
```

### 3. `src/browser.rs`

Inspect sampled curve resources and actual enabled control count separately.

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-point-cpu",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-curve-cpu",
        &serde_json::to_string(&crate::memory::curves(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-control-count",
        &editor
            .scene
            .control_markers()
            .len()
            .to_string(),
    )?;
    canvas.set_attribute(
        "data-point-cpu",
```

### 4. `src/lib.rs`

Register document parent and placement checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod control_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod control_parent_tests;
#[cfg(test)]
mod control_tests;
```

### 5. `src/control_parent_tests.rs`

Copy shared curve identity, distinct parent IDs, original indices, correct placement and release checks.

Create the file and type:

```rust
use crate::{chain::PreparedChain, scene::Scene};
use session_rust::{NurbsCurve, Point, Xform};
use std::rc::Rc;

#[test]
fn placed_controls_share_the_original_curve_but_keep_distinct_document_parents() {
    let source = Rc::new(NurbsCurve::create(
        false,
        2,
        &[
            Point::new(-0.6, -0.4, 0.0),
            Point::new(0.0, 0.6, 0.0),
            Point::new(0.6, -0.4, 0.0),
        ],
    ));
    let weak = Rc::downgrade(&source);
    let prepared = PreparedChain::curve(source).unwrap();
    let mut scene = Scene::demo();
    scene.clear();
    let first = scene.insert_path(prepared.clone()).unwrap();
    let second = scene.insert_path(prepared).unwrap();
    assert!(scene.control_markers().is_empty());
    scene
        .place_path(second, Xform::translation(2.0, 3.0, 4.0))
        .unwrap();
    scene.show_controls(true);
    let markers = scene.control_markers();
    assert_eq!(markers.len(), 6);
    assert_eq!(markers[0].parent, first);
    assert_eq!(markers[3].parent, second);
    assert_eq!(markers[1].control.index, 1);
    assert_eq!(markers[4].control.index, 1);
    assert!(Rc::ptr_eq(
        &markers[1].control.source,
        &markers[4].control.source
    ));
    assert_eq!(markers[1].display.center, [0.0, 0.6, 0.0]);
    assert_eq!(markers[4].display.center, [2.0, 3.6, 4.0]);
    drop(markers);
    scene.show_controls(false);
    assert!(scene.control_markers().is_empty());
    scene.clear();
    assert!(weak.upgrade().is_none());
}
```

## <a id="checkpoint-38d-input"></a>5. Insert an curve and inspect its controls

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/browser.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Example Curve input</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Controls On/Off</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/ribbon.…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">history and Close</text>
</svg>
</div>

**What this code does:** Insert an original NURBS curve through real input and distinguish its actual curve pixels from its indexed control markers.

### 1. `src/editor.rs`

Expose original curve insertion and explicit control display actions. Insert an original curve transaction and keep control visibility out of geometry history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
```

Replace it with:

```rust
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_path(prepared).map(|_| ())
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(
                    false,
                    2,
                    &[
                        session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.6, 0.0),
                        session_rust::Point::new(0.6, -0.4, 0.0),
                    ],
                );
                curve.width = 5.0;
                curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared =
                    crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history
```

**Locate this block:**

```rust
    AddPoint,
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
```

Replace it with:

```rust
    AddPoint,
    AddCurve,
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
```

### 2. `src/browser.rs`

Expose curve and control inputs in dock completion and Help. Dispatch accepted curve and explicit control input through the editor.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example polyline" => Action::AddPolyline,
```

Replace it with:

```rust
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example point" => Action::AddPoint,
                        "example curve" => Action::AddCurve,
                        "controls on" => Action::Controls(true),
                        "controls off" => Action::Controls(false),
                        "example polyline" => Action::AddPolyline,
```

**Locate this block:**

```rust
            "Example Point",
            "Example Polyline",
            "Example Arrow",
            "Example Triangle",
```

Replace it with:

```rust
            "Example Point",
            "Example Curve",
            "Controls On",
            "Controls Off",
            "Example Polyline",
            "Example Arrow",
            "Example Triangle",
```

### 3. `src/lib.rs`

Register actual curve insertion history and ownership checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod curve_layout_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod curve_input_tests;
#[cfg(test)]
mod curve_layout_tests;
```

### 4. `src/curve_input_tests.rs`

Copy original curve identity, control display history independence and complete release checks.

Create the file and type:

```rust
use crate::{
    chain::Source,
    editor::{Action, Editor},
    memory,
};

#[test]
fn curve_command_keeps_original_identity_and_control_visibility_does_not_consume_undo()
{
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddCurve).unwrap();
    let id = editor.scene.paths()[0].id;
    let Source::Curve(source, _) = &editor.scene.paths()[0].prepared.source else {
        panic!("Original curve")
    };
    let weak = std::rc::Rc::downgrade(source);
    assert_eq!(source.m_order, 3);
    assert_eq!(source.width, 5.0);
    assert!(editor.scene.control_markers().is_empty());
    editor.apply(Action::Controls(true)).unwrap();
    assert_eq!(editor.scene.control_markers().len(), 3);
    editor
        .apply(Action::Controls(false))
        .unwrap();
    editor.apply(Action::Controls(true)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.paths().is_empty());
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.paths()[0].id, id);
    assert_eq!(editor.scene.control_markers().len(), 3);
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(memory::curves(editor.scenes()), [0; 4]);
}
```

## Build & Run

![Recorded visual output for Lesson 5.4 (38d-input)](../../../screenshots/journey/38d-input-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.4
npm --prefix ../session_tests run course -- save 5.4
```

Continue with [5.5 · One visibility policy and shared surface depth](../../consolidation/39-visible.md).
