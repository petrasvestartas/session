# 15.4 · Two-direction networks and corner patches

**Goal:** Reuse absolute coordinate parsing for named construction planes and three-point input. · **Steps:** 6 · **Source:** [app/deform.rs](../../../../src/app/deform.rs) · [state/edit.rs](../../../../src/state/edit.rs) · [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs) · [app/command/tool/surfacing.rs](../../../../src/app/command/tool/surfacing.rs) · [app/walk/mesh.rs](../../../../src/app/walk/mesh.rs) · [app/walk/brep.rs](../../../../src/app/walk/brep.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 15.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/network_geometry.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/patch_geometry.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">6 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Build a surface through c…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Screen-Space Ribbon Quad</text>
  <polygon points="606,112 764,56 782,94 624,146" fill="#bfdbfe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="615" y1="129" x2="773" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="615" cy="129" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <circle cx="773" cy="75" r="4.5" fill="#dc2626" stroke="#111111" stroke-width="1.25"/>
  <line x1="694" y1="102" x2="682" y2="72" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="664" y="70" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Two-direction networks and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. RGBA Ink &amp; SDF Edge AA</text>
  <line x1="884" y1="136" x2="1064" y2="136" stroke="#111111" stroke-width="1.5"/>
  <path d="M 896 136 L 928 68 L 1020 68 L 1052 136" fill="#93c5fd" stroke="#2563eb" stroke-width="2"/>
  <circle cx="916" cy="54" r="6" fill="#ef4444" stroke="#111111" stroke-width="1.25"/>
  <circle cx="936" cy="54" r="6" fill="#22c55e" stroke="#111111" stroke-width="1.25"/>
  <circle cx="956" cy="54" r="6" fill="#3b82f6" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1020" y="58" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">RGBA + SDF</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Read projection planes</text>
</svg>
</div>

## <a id="checkpoint-80-build-a-surface-through-two-directions-of-original-curves"></a>1. Build a surface through curve networks

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/network_geometry.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original curve crossings</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sampled lofts</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fitted surface</text>
</svg>
</div>

**What this code does:** Connect the complete network helpers and retain the four-boundary Coons path.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    for (c, column) in stacks.iter().enumerate() {
        for d in 0..column.cv_count() {
            surface.set_cv(c, d, &column.get_cv(d).unwrap_or_default());
        }
    }

    Ok(surface)
}
```

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

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/network_geometry.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

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

After that block, append this supplied code:

```text
--8<-- "journey/code/80-build-a-surface-through-two-directions-of-original-curves-2.rs" skip=11853
```

## <a id="checkpoint-80a-construct-the-patch-through-four-ordered-corners"></a>2. Construct the patch through four ordered corners

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/patch_geometry.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">four ordered world corners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">area check</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bilinear original surface</text>
</svg>
</div>

**What this code does:** Construct the patch through four ordered corners.

### 1. `src/patch_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/patch_geometry.rs`

Supply only the unchanged original geometry validation fixtures.

**Locate this existing block:**

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

After that block, append this supplied code:

```text
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-typed-patch_geometry-fixtures.rs" skip=1504
```

### 3. `src/lib.rs`

Register the current surface component.

**Locate this existing block:**

```rust

pub mod network_geometry;
```

After that block, type:

```rust
--8<-- "journey/code/80a-construct-the-patch-through-four-ordered-corners-02.rs" skip=27
```

## <a id="checkpoint-80b-guide-corner-questions-without-retaining-unrelated-profiles"></a>3. Guide corner questions without retaining unrelated profiles

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gather_draft.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">accepted corner points an…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary guide</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unfinished recipe</text>
</svg>
</div>

**What this code does:** Guide corner questions without retaining unrelated profiles.

### 1. `src/gather_draft.rs`

Guide current point questions and retain original profile copies only for curve-gathering recipes.

Replace the entire file with:

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

## <a id="checkpoint-80c-register-network-patch-aliases-and-initial-corner-arity"></a>4. Register network patch aliases and initial corner arity

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gather_registry.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current name or alias</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">initial argument validati…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared recipe owner</text>
</svg>
</div>

**What this code does:** Register network patch aliases and initial corner arity.

### 1. `src/gather_registry.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

Replace the entire file with:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-01.rs"
```

### 2. `src/commands.rs`

Register current network/patch names, aliases and initial four-corner argument validation.

**Locate this existing block:**

```rust
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return Ok(Command::Editor(action));}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/80c-register-network-patch-aliases-and-initial-corner-arity-02.rs"
```

## <a id="checkpoint-80d-exercise-networks-corner-retries-and-exact-original-archives"></a>5. Exercise networks corner retries and exact archives

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/network_tests.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original network and corn…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one history transaction</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact Save/Open</text>
</svg>
</div>

**What this code does:** Exercise networks corner retries and exact original archives.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/network_tests.rs`

Copy integrated current network/patch/original-source/history/archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the current surface component.

**Locate this existing block:**

```text

pub mod patch_geometry;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/80d-exercise-networks-corner-retries-and-exact-original-archives-02.rs" skip=25
```

## <a id="checkpoint-80za-read-original-projection-planes"></a>6. Read projection planes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/project_geometry.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Reuse absolute coordinate parsing for named construction planes and three-point input.

### 1. `src/project_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-1.rs"
```

### 2. `src/lib.rs`

Register the original projection helpers.

**Locate this existing block:**

```rust
#[cfg(test)]
mod network_tests;
```

After that block, type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-2.rs" skip=32
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 15.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 15.4
npm --prefix ../session_tests run course -- save 15.4
```

Continue with [16.1 · Projection, extension and stable edit targets](../../consolidation/81-project-originals-through-the-current-plane-algorithms.md).
