# 15.3 · Lofts, rail sweeps and network foundations

**Goal:** Combine the two lofts minus their shared tensor. · **Steps:** 12 · **Source:** [app/deform.rs](../../../src/app/deform.rs) · [state/edit.rs](../../../src/state/edit.rs) · [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs) · [app/command/tool/surfacing.rs](../../../src/app/command/tool/surfacing.rs) · [app/walk/mesh.rs](../../../src/app/walk/mesh.rs) · [app/walk/brep.rs](../../../src/app/walk/brep.rs) (`src/app/transforms.rs,src/app/surfacing.rs,src/app/nurbs_eval.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Section Profiles C_0(u)..C_k(u) + Rail Curve R(v)</text>
  <path d="M 58 142 Q 112 96 168 142" fill="none" stroke="#2563eb" stroke-width="2.5"/>
  <path d="M 88 106 Q 142 60 198 106" fill="none" stroke="#16a34a" stroke-width="2.5"/>
  <path d="M 118 70 Q 172 24 228 70" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <line x1="58" y1="142" x2="118" y2="70" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <line x1="168" y1="142" x2="228" y2="70" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Tensor-Product Loft / Sweep / Coons Patch S(u,v)</text>
  <polygon points="434,142 494,66 664,66 604,142" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="464" y1="104" x2="634" y2="104" stroke="#2563eb" stroke-width="1.5"/>
  <line x1="491" y1="142" x2="551" y2="66" stroke="#16a34a" stroke-width="1.5"/>
  <line x1="548" y1="142" x2="608" y2="66" stroke="#16a34a" stroke-width="1.5"/>
  <circle cx="550" cy="104" r="4.5" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Shaded Freeform Surface + Tessellated Mesh</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,138 888,68 1028,68 978,138" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <line x1="888" y1="68" x2="978" y2="138" stroke="#1e3a8a" stroke-width="1.2"/>
  <line x1="863" y1="103" x2="1003" y2="103" stroke="#1e3a8a" stroke-width="1.2"/>
</svg>
</div>

Build exact surfaces from ordered original sections and rails while preserving their source precision.

## Ownership and data flow

Retain section order, orientation and degree options before lofting. One-rail and two-rail sweeps frame profiles on original rails, align corresponding shapes and blend without mutating picked source owners. Freshness and degeneracy checks precede one generated-source transaction. Network station sampling, direction classification, orientation and Gordon blending prepare the two-direction recipe connected in the next lesson; temporary samples remain derived construction data.

## The maintained implementation

app/command/tool/surfacing.rs performs maintained section alignment and surface recipes; verbs/nurbs_surface_loft.rs, nurbs_surface_sweep1.rs and nurbs_surface_sweep2.rs register actual commands. Original session_rust NurbsCurve/NurbsSurface outputs remain authoritative; no display mesh is promoted to the final editable surface.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/deform.rs](../../../src/app/deform.rs), [state/edit.rs](../../../src/state/edit.rs), [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs), [app/command/tool/surfacing.rs](../../../src/app/command/tool/surfacing.rs), [app/walk/mesh.rs](../../../src/app/walk/mesh.rs), [app/walk/brep.rs](../../../src/app/walk/brep.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-79-retain-current-loft-section-order-and-degree-options"></a>1. Retain current loft section order and degree options

**What this code does:** Retain current loft section order and degree options.

### 1. `src/loft_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners. Supply only the unchanged original geometry validation fixtures.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{self, checked, count, curves};
use session_rust::{Point, Vector};

pub static RECIPE: Recipe = Recipe {
    name: "Loft",
    chips: &[
        ("Open", "Open"),
        ("Closed", "Closed"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX,
    }],
    axis: no_axis,
    build,
};

/// Loft has no distance to slide.
pub fn no_axis(_input: &Input) -> (Point, Vector) {
    (
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
    )
}

/// A cubic loft through the curves in pick order; Closed returns to the first.
fn build(input: &Input) -> Result<Made, String> {
    let sections = curves(&input.curves[0]);
    let surface = surfacing::loft(&sections, input.choice == "Closed", 3)?;
    Ok(Made {
        geometries: vec![checked(surface, "loft")?],
        message: format!(
            "Lofted {} into a NURBS surface",
            count(sections.len(), "curve")
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfacing::Picked;
    use crate::surfacing::tests::{curve, p, square};
    use session_rust::{Geometry, NurbsCurve};

    /// The input of curves picked in this order.
    fn input(sections: Vec<NurbsCurve>, choice: &'static str) -> Input {
        Input {
            curves: vec![
                sections
                    .into_iter()
                    .map(|curve| Picked {
                        curve,
                        corners: None,
                    })
                    .collect(),
            ],
            answers: Vec::new(),
            choice,
            normal: Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// Three arcs give a surface through the middle one.
    #[test]
    fn three_arcs_loft_through_the_middle_section() {
        let arcs = vec![
            curve(&[
                p(0., 0., 0.),
                p(50., 0., 20.),
                p(100., 0., 0.),
            ]),
            curve(&[
                p(0., 50., 10.),
                p(50., 50., 40.),
                p(100., 50., 10.),
            ]),
            curve(&[
                p(0., 100., 0.),
                p(50., 100., 20.),
                p(100., 100., 0.),
            ]),
        ];
        let middle = arcs[1].point_at(0.5);
        let made = build(&input(arcs, "Open")).unwrap();
        let Geometry::NurbsSurface(surface) = &made.geometries[0] else {
            panic!()
        };
        assert!(surface.is_valid());
        assert_eq!(surface.cv_count(1), 3);
        assert!(
            surface
                .closest_point(&middle)
                .distance(&middle, None)
                < 1e-6
        );
    }

    /// Closed returns to the first section and needs three.
    #[test]
    fn closed_loops_back_and_needs_three() {
        let squares = vec![
            square(20.0, 0.0),
            square(20.0, 30.0),
            square(20.0, 15.0),
        ];
        let made = build(&input(squares.clone(), "Closed")).unwrap();
        let Geometry::NurbsSurface(surface) = &made.geometries[0] else {
            panic!()
        };
        assert!(surface.is_closed(1));
        assert!(surface.is_closed(0));
        assert!(build(&input(squares[..2].to_vec(), "Closed")).is_err());
        assert!(build(&input(squares[..1].to_vec(), "Open")).is_err());
    }
}
```

### 2. `src/lib.rs`

Register the current surfacing component.

**Locate this existing block:**

```rust
mod gather_tests;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod gather_tests;

pub mod loft_geometry;
pub mod source_mesh_topology;
```

### 3. `src/nurbs_loft_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{self, checked, count, curves};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Loft",
    chips: &[
        ("Smooth", "Smooth"),
        ("Straight", "Straight"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX,
    }],
    axis: crate::gather_input::no_axis,
    build,
};

/// A loft cubic across the sections, or straight strips between them.
fn build(input: &Input) -> Result<Made, String> {
    let sections = curves(&input.curves[0]);
    let degree = if input.choice == "Straight" {
        1
    } else {
        3
    };
    let surface = surfacing::loft(&sections, false, degree)?;
    Ok(Made {
        geometries: vec![checked(surface, "loft")?],
        message: format!(
            "Lofted {} into a NURBS surface",
            count(sections.len(), "curve")
        ),
    })
}
```

### 4. `src/lib.rs`

Register the current surfacing component.

**Locate this existing block:**

```rust
pub mod loft_geometry;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod loft_geometry;

pub mod nurbs_loft_geometry;
pub mod source_mesh_topology;
```

## <a id="checkpoint-79aa-frame-original-profiles-on-their-rail"></a>2. Frame profiles on their rail

**What this code does:** Find each profile’s station from its sampled center and the nearest rail parameter.

### 1. `src/sweep1_geometry.rs`

Type the whole original sweep helpers using the existing kernel and gather owners.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{SAMPLES, checked, count, curves};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives, Xform};

/// The rail parameter nearest the profile's middle and world to the rail frame there.
fn on_rail(rail: &NurbsCurve, profile: &NurbsCurve) -> (f64, Xform) {
    let mut points = profile.divide_by_count(SAMPLES, true).0;

    // a closed profile's end repeats its start
    if profile.is_closed() {
        points.pop();
    }

    let count = points.len().max(1) as f64;
    let sum = points.iter().fold([0.0; 3], |sum, p| {
        [sum[0] + p[0], sum[1] + p[1], sum[2] + p[2]]
    });
    let middle = Point::new(
        sum[0] / count,
        sum[1] / count,
        sum[2] / count,
    );
    let t = rail.closest_parameter(&middle);
    let frame = rail.perpendicular_plane_at(t, false);
    let local = Xform::world_to_frame(
        &frame.origin(),
        &frame.x_axis(),
        &frame.y_axis(),
        &frame.z_axis(),
    );
    (t, local)
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
fn blend(a: &NurbsCurve, b: &NurbsCurve, s: f64) -> NurbsCurve {
    let mut section = a.clone();

    for i in 0..a.cv_count() {
        let (Some(p), Some(q)) = (a.get_cv_4d(i), b.get_cv_4d(i)) else {
            continue;
        };
        let mix = |x: f64, y: f64| x + (y - x) * s;
        section.set_cv_4d(
            i,
            mix(p.0, q.0),
            mix(p.1, q.1),
            mix(p.2, q.2),
            mix(p.3, q.3),
        );
    }

    section
}
```

### 2. `src/lib.rs`

Compile this shared original sweep module.

**Locate this existing block:**

```rust
pub mod nurbs_loft_geometry;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod nurbs_loft_geometry;

pub mod source_mesh_topology;
pub mod sweep1_geometry;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-79ab-carry-and-blend-original-profiles-along-the-rail"></a>3. Carry and blend profiles along the rail

**What this code does:** Sort original profile stations, match their control rows with a straight kernel loft, and include their stations in the sampled rail frames.

### 1. `src/sweep1_geometry.rs`

Extend this original owner with the day’s complete input helpers.

**Locate this existing block:**

```rust
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The profiles carried along the rail, each kept where it sits; between two they blend.
pub fn sweep1(
    rail: &NurbsCurve,
    profiles: &[NurbsCurve],
) -> Result<NurbsSurface, String> {
    if profiles.is_empty() {
        return Err("Select at least 1 profile".into());
    }

    let mut placed: Vec<(f64, NurbsCurve)> = profiles
        .iter()
        .map(|profile| {
            let (t, local) = on_rail(rail, profile);
            (t, profile.transformed(&local))
        })
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (stations, locals): (Vec<f64>, Vec<NurbsCurve>) = placed.into_iter().unzip();
    let shared = if locals.len() == 1 {
        locals
    } else {
        // a straight loft makes the profiles share knots; its rows are the profiles again
        let bridge = Primitives::create_loft(&locals, 1);
        let rows: Vec<NurbsCurve> = bridge
            .get_nurbsknots(1)
            .iter()
            .take(locals.len())
            .filter_map(|v| bridge.iso_curve(0, *v))
            .collect();

        if rows.len() != locals.len() {
            return Err("The profiles could not be matched to each other".into());
        }

        rows
    };

    // frames at even lengths plus one at every profile, so the surface runs through them
    let count = (rail.span_count() * 2 + 1).clamp(5, 200);
    let mut params = rail.divide_by_count(count + 1, true).1;
    let (t0, t1) = rail.domain();
    let near = 1e-6 * (t1 - t0);
    params.retain(|t| stations.iter().all(|s| (t - s).abs() > near));
    params.extend_from_slice(&stations);
    params.sort_by(f64::total_cmp);
    params.dedup_by(|a, b| (*a - *b).abs() <= near);
    let mut sections = Vec::with_capacity(params.len());

    for t in params {
        let upper = stations.partition_point(|s| *s < t);
        let mut section = match upper {
            0 => shared[0].clone(),
            n if n == stations.len() => shared[n - 1].clone(),
            n => blend(
                &shared[n - 1],
                &shared[n],
                (t - stations[n - 1]) / (stations[n] - stations[n - 1]),
            ),
        };
        section.transform(&Xform::to_frame(
            &rail.perpendicular_plane_at(t, false),
        ));
        sections.push(section);
    }

    let surface = Primitives::create_loft(&sections, 3);

    if !surface.is_valid() {
        return Err("The profiles could not be swept along the rail".into());
    }

    Ok(surface)
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
```

## <a id="checkpoint-79a-carry-original-profiles-along-one-rail"></a>4. Connect one-rail sweep inputs

**What this code does:** Complete the existing gather recipe with one rail and one or more profiles.

### 1. `src/sweep1_geometry.rs`

Extend this original owner with the day’s complete input helpers. Supply only the unchanged original sweep validation fixtures.

**Locate this existing block:**

```rust
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives, Xform};

/// The rail parameter nearest the profile's middle and world to the rail frame there.
fn on_rail(rail: &NurbsCurve, profile: &NurbsCurve) -> (f64, Xform) {
    let mut points = profile.divide_by_count(SAMPLES, true).0;

    // a closed profile's end repeats its start
    if profile.is_closed() {
        points.pop();
    }

    let count = points.len().max(1) as f64;
    let sum = points.iter().fold([0.0; 3], |sum, p| {
        [sum[0] + p[0], sum[1] + p[1], sum[2] + p[2]]
    });
    let middle = Point::new(
        sum[0] / count,
        sum[1] / count,
        sum[2] / count,
    );
    let t = rail.closest_parameter(&middle);
    let frame = rail.perpendicular_plane_at(t, false);
    let local = Xform::world_to_frame(
        &frame.origin(),
        &frame.x_axis(),
        &frame.y_axis(),
        &frame.z_axis(),
    );
    (t, local)
}

/// The profiles carried along the rail, each kept where it sits; between two they blend.
pub fn sweep1(
    rail: &NurbsCurve,
    profiles: &[NurbsCurve],
) -> Result<NurbsSurface, String> {
    if profiles.is_empty() {
        return Err("Select at least 1 profile".into());
    }

    let mut placed: Vec<(f64, NurbsCurve)> = profiles
        .iter()
        .map(|profile| {
            let (t, local) = on_rail(rail, profile);
            (t, profile.transformed(&local))
        })
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (stations, locals): (Vec<f64>, Vec<NurbsCurve>) = placed.into_iter().unzip();
    let shared = if locals.len() == 1 {
        locals
    } else {
        // a straight loft makes the profiles share knots; its rows are the profiles again
        let bridge = Primitives::create_loft(&locals, 1);
        let rows: Vec<NurbsCurve> = bridge
            .get_nurbsknots(1)
            .iter()
            .take(locals.len())
            .filter_map(|v| bridge.iso_curve(0, *v))
            .collect();

        if rows.len() != locals.len() {
            return Err("The profiles could not be matched to each other".into());
        }

        rows
    };

    // frames at even lengths plus one at every profile, so the surface runs through them
    let count = (rail.span_count() * 2 + 1).clamp(5, 200);
    let mut params = rail.divide_by_count(count + 1, true).1;
    let (t0, t1) = rail.domain();
    let near = 1e-6 * (t1 - t0);
    params.retain(|t| stations.iter().all(|s| (t - s).abs() > near));
    params.extend_from_slice(&stations);
    params.sort_by(f64::total_cmp);
    params.dedup_by(|a, b| (*a - *b).abs() <= near);
    let mut sections = Vec::with_capacity(params.len());

    for t in params {
        let upper = stations.partition_point(|s| *s < t);
        let mut section = match upper {
            0 => shared[0].clone(),
            n if n == stations.len() => shared[n - 1].clone(),
            n => blend(
                &shared[n - 1],
                &shared[n],
                (t - stations[n - 1]) / (stations[n] - stations[n - 1]),
            ),
        };
        section.transform(&Xform::to_frame(
            &rail.perpendicular_plane_at(t, false),
        ));
        sections.push(section);
    }

    let surface = Primitives::create_loft(&sections, 3);

    if !surface.is_valid() {
        return Err("The profiles could not be swept along the rail".into());
    }

    Ok(surface)
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
fn blend(a: &NurbsCurve, b: &NurbsCurve, s: f64) -> NurbsCurve {
    let mut section = a.clone();

    for i in 0..a.cv_count() {
        let (Some(p), Some(q)) = (a.get_cv_4d(i), b.get_cv_4d(i)) else {
            continue;
        };
        let mix = |x: f64, y: f64| x + (y - x) * s;
        section.set_cv_4d(
            i,
            mix(p.0, q.0),
            mix(p.1, q.1),
            mix(p.2, q.2),
            mix(p.3, q.3),
        );
    }

    section
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives, Xform};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Sweep1",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select the rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// The rail parameter nearest the profile's middle and world to the rail frame there.
fn on_rail(rail: &NurbsCurve, profile: &NurbsCurve) -> (f64, Xform) {
    let mut points = profile.divide_by_count(SAMPLES, true).0;

    // a closed profile's end repeats its start
    if profile.is_closed() {
        points.pop();
    }

    let count = points.len().max(1) as f64;
    let sum = points.iter().fold([0.0; 3], |sum, p| {
        [sum[0] + p[0], sum[1] + p[1], sum[2] + p[2]]
    });
    let middle = Point::new(
        sum[0] / count,
        sum[1] / count,
        sum[2] / count,
    );
    let t = rail.closest_parameter(&middle);
    let frame = rail.perpendicular_plane_at(t, false);
    let local = Xform::world_to_frame(
        &frame.origin(),
        &frame.x_axis(),
        &frame.y_axis(),
        &frame.z_axis(),
    );
    (t, local)
}

/// The profiles carried along the rail, each kept where it sits; between two they blend.
pub fn sweep1(
    rail: &NurbsCurve,
    profiles: &[NurbsCurve],
) -> Result<NurbsSurface, String> {
    if profiles.is_empty() {
        return Err("Select at least 1 profile".into());
    }

    let mut placed: Vec<(f64, NurbsCurve)> = profiles
        .iter()
        .map(|profile| {
            let (t, local) = on_rail(rail, profile);
            (t, profile.transformed(&local))
        })
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (stations, locals): (Vec<f64>, Vec<NurbsCurve>) = placed.into_iter().unzip();
    let shared = if locals.len() == 1 {
        locals
    } else {
        // a straight loft makes the profiles share knots; its rows are the profiles again
        let bridge = Primitives::create_loft(&locals, 1);
        let rows: Vec<NurbsCurve> = bridge
            .get_nurbsknots(1)
            .iter()
            .take(locals.len())
            .filter_map(|v| bridge.iso_curve(0, *v))
            .collect();

        if rows.len() != locals.len() {
            return Err("The profiles could not be matched to each other".into());
        }

        rows
    };

    // frames at even lengths plus one at every profile, so the surface runs through them
    let count = (rail.span_count() * 2 + 1).clamp(5, 200);
    let mut params = rail.divide_by_count(count + 1, true).1;
    let (t0, t1) = rail.domain();
    let near = 1e-6 * (t1 - t0);
    params.retain(|t| stations.iter().all(|s| (t - s).abs() > near));
    params.extend_from_slice(&stations);
    params.sort_by(f64::total_cmp);
    params.dedup_by(|a, b| (*a - *b).abs() <= near);
    let mut sections = Vec::with_capacity(params.len());

    for t in params {
        let upper = stations.partition_point(|s| *s < t);
        let mut section = match upper {
            0 => shared[0].clone(),
            n if n == stations.len() => shared[n - 1].clone(),
            n => blend(
                &shared[n - 1],
                &shared[n],
                (t - stations[n - 1]) / (stations[n] - stations[n - 1]),
            ),
        };
        section.transform(&Xform::to_frame(
            &rail.perpendicular_plane_at(t, false),
        ));
        sections.push(section);
    }

    let surface = Primitives::create_loft(&sections, 3);

    if !surface.is_valid() {
        return Err("The profiles could not be swept along the rail".into());
    }

    Ok(surface)
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
fn blend(a: &NurbsCurve, b: &NurbsCurve, s: f64) -> NurbsCurve {
    let mut section = a.clone();

    for i in 0..a.cv_count() {
        let (Some(p), Some(q)) = (a.get_cv_4d(i), b.get_cv_4d(i)) else {
            continue;
        };
        let mix = |x: f64, y: f64| x + (y - x) * s;
        section.set_cv_4d(
            i,
            mix(p.0, q.0),
            mix(p.1, q.1),
            mix(p.2, q.2),
            mix(p.3, q.3),
        );
    }

    section
}

/// The sweep of the profiles along the rail.
fn build(input: &Input) -> Result<Made, String> {
    let profiles = curves(&input.curves[1]);
    let surface = sweep1(&input.curves[0][0].curve, &profiles)?;
    Ok(Made {
        geometries: vec![checked(surface, "sweep")?],
        message: format!(
            "Swept {} along the rail",
            count(profiles.len(), "profile")
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfacing::tests::{curve, p};

    /// The middle of a surface's domain.
    fn middle(surface: &NurbsSurface) -> Point {
        let (u, v) = (
            surface.domain(0).unwrap(),
            surface.domain(1).unwrap(),
        );
        surface
            .point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0)
            .unwrap()
    }

    /// A closed polyline square of half size `h` around `(x, 0, 0)` in the plane x = `x`.
    fn square(x: f64, h: f64) -> NurbsCurve {
        let corners = [
            p(x, -h, -h),
            p(x, h, -h),
            p(x, h, h),
            p(x, -h, h),
            p(x, -h, -h),
        ];
        let mut square = NurbsCurve::create(false, 1, &corners);
        square.set_domain(0.0, 1.0);
        square
    }

    /// True when every control point of `profile` lies on `surface`.
    fn runs_through(surface: &NurbsSurface, profile: &NurbsCurve) -> bool {
        (0..profile.cv_count())
            .filter_map(|i| profile.get_cv(i))
            .all(|q| surface.closest_point(&q).distance(&q, None) < 1e-6)
    }

    /// A profile stays where it is drawn and is carried rigidly along the rail.
    #[test]
    fn one_profile_keeps_its_place_on_the_rail() {
        let rail = curve(&[
            p(0., 0., 0.),
            p(50., 50., 0.),
            p(100., 0., 0.),
        ]);
        let profile = square(0.0, 5.0);
        let surface = sweep1(&rail, std::slice::from_ref(&profile)).unwrap();
        assert!(surface.is_closed(0));
        assert!(runs_through(&surface, &profile));
        let off = middle(&surface).distance(&rail.point_at(0.5), None);
        assert!(
            (off - 50f64.sqrt()).abs() < 0.2,
            "middle {off} from the rail"
        );
    }

    /// Profiles sweep in rail order whatever order they were picked, each kept in place.
    #[test]
    fn profiles_blend_between_their_own_places() {
        let rail = curve(&[
            p(0., 0., 0.),
            p(50., 50., 0.),
            p(100., 0., 0.),
        ]);
        let (big, small) = (square(0.0, 5.0), square(100.0, 2.0));
        let one = sweep1(&rail, &[big.clone(), small.clone()]).unwrap();
        let two = sweep1(&rail, &[small.clone(), big.clone()]).unwrap();
        assert!(runs_through(&one, &big) && runs_through(&one, &small));
        assert!(middle(&one).distance(&middle(&two), None) < 1e-9);
        assert!(sweep1(&rail, &[]).is_err());
    }
}
```

## <a id="checkpoint-79b-align-shape-curves-between-two-original-rails"></a>5. Align shape curves between two rails

**What this code does:** Align shape curves between two original rails.

### 1. `src/sweep2_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners. Supply only the unchanged original geometry validation fixtures.

Create the file and type:

```rust
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count, curves};
use session_rust::{NurbsCurve, NurbsSurface, Primitives};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Sweep2",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select the first rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select the second rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select shape curves",
            min: 1,
            max: usize::MAX,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// Rail 2 runs with rail 1 and every shape starts on rail 1.
pub fn sweep2(
    first: &NurbsCurve,
    second: &NurbsCurve,
    shapes: &[NurbsCurve],
) -> Result<NurbsSurface, String> {
    let mut second = second.clone();
    let start = first.point_at_start();

    if second
        .point_at_start()
        .distance(&first.point_at_end(), None)
        < second
            .point_at_start()
            .distance(&start, None)
    {
        second.reverse();
    }

    let shapes: Vec<NurbsCurve> = shapes
        .iter()
        .map(|shape| {
            let mut shape = shape.clone();
            let near = |curve: &NurbsCurve, rail: &NurbsCurve| {
                curve.point_at_start().distance(
                    &rail.point_at(rail.closest_parameter(&curve.point_at_start())),
                    None,
                )
            };

            if near(&shape, &second) < near(&shape, first) {
                shape.reverse();
            }

            shape
        })
        .collect();
    let surface = Primitives::create_sweep2(first, &second, &shapes);

    if !surface.is_valid() {
        return Err("The shapes could not be swept along the rails".into());
    }

    Ok(surface)
}

/// The sweep of the shapes between the two rails.
fn build(input: &Input) -> Result<Made, String> {
    let shapes = curves(&input.curves[2]);
    let surface = sweep2(
        &input.curves[0][0].curve,
        &input.curves[1][0].curve,
        &shapes,
    )?;
    Ok(Made {
        geometries: vec![checked(surface, "sweep")?],
        message: format!(
            "Swept {} along 2 rails",
            count(shapes.len(), "shape")
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfacing::tests::{curve, p};

    /// The shape's ends ride the rails; a reversed rail 2 changes nothing.
    #[test]
    fn shape_ends_lie_on_the_rails() {
        let first = curve(&[p(0., 0., 0.), p(100., 0., 0.)]);
        let second = curve(&[p(0., 50., 0.), p(100., 50., 20.)]);
        let shape = curve(&[
            p(0., 0., 0.),
            p(0., 25., 15.),
            p(0., 50., 0.),
        ]);
        let surface = sweep2(&first, &second, std::slice::from_ref(&shape)).unwrap();

        for t in [0.0, 0.5, 1.0] {
            for rail in [&first, &second] {
                let q = rail.point_at(t);
                assert!(
                    surface.closest_point(&q).distance(&q, None) < 1e-3,
                    "{t}"
                );
            }
        }

        let mut turned = second.clone();
        turned.reverse();
        let again = sweep2(&first, &turned, &[shape]).unwrap();
        let (u, v) = (
            surface.domain(0).unwrap(),
            surface.domain(1).unwrap(),
        );
        let (a, b) = (
            again.domain(0).unwrap(),
            again.domain(1).unwrap(),
        );
        let one = surface
            .point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0)
            .unwrap();
        let two = again
            .point_at((a.0 + a.1) / 2.0, (b.0 + b.1) / 2.0)
            .unwrap();
        assert!(one.distance(&two, None) < 1e-6);
    }
}
```

### 2. `src/lib.rs`

Register the current surfacing component.

**Locate this existing block:**

```rust
pub mod nurbs_loft_geometry;

pub mod source_mesh_topology;
pub mod sweep1_geometry;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod nurbs_loft_geometry;

pub mod sweep1_geometry;

pub mod source_mesh_topology;
pub mod sweep2_geometry;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-79c-register-loft-sweep-names-options-and-aliases"></a>6. Register loft sweep names options and aliases

**What this code does:** Register loft sweep names options and aliases.

### 1. `src/gather_registry.rs`

Register maintained loft/sweep recipes and current aliases through shared gather input.

**Locate this existing block:**

```rust
pub const ENTRIES: &[(&str, &Recipe)] = &[
    ("Extrude", &crate::extrude_geometry::RECIPE),
    (
        "Nurbs Surface Revolve",
        &crate::revolve_geometry::RECIPE,
    ),
];
pub fn command(name: &str, text: &str) -> Option<Action> {
    let (_, recipe) = ENTRIES
        .iter()
        .find(|(label, _)| *label == name)?;
    Some(Action::GatherBegin {
        recipe,
        text: text.into(),
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const ENTRIES: &[(&str, &Recipe)] = &[
    ("Loft", &crate::loft_geometry::RECIPE),
    (
        "Nurbs Surface Loft",
        &crate::nurbs_loft_geometry::RECIPE,
    ),
    (
        "Nurbs Surface Sweep1",
        &crate::sweep1_geometry::RECIPE,
    ),
    (
        "Nurbs Surface Sweep2",
        &crate::sweep2_geometry::RECIPE,
    ),
    ("Extrude", &crate::extrude_geometry::RECIPE),
    (
        "Nurbs Surface Revolve",
        &crate::revolve_geometry::RECIPE,
    ),
];
pub fn command(name: &str, text: &str) -> Option<Action> {
    let (_, recipe) = ENTRIES
        .iter()
        .find(|(label, _)| *label == name)?;
    Some(Action::GatherBegin {
        recipe,
        text: text.into(),
    })
}
pub fn alias(line: &str) -> String {
    let mut words = line.split_whitespace();
    let Some(first) = words.next() else {
        return line.into();
    };
    let name = match first.to_ascii_lowercase().as_str() {
        "nurbssurface_loft" => "Nurbs Surface Loft",
        "nurbssurface_sweep1" => "Nurbs Surface Sweep1",
        "nurbssurface_sweep2" => "Nurbs Surface Sweep2",
        "nurbssurface_revolve" => "Nurbs Surface Revolve",
        _ => return line.into(),
    };
    std::iter::once(name)
        .chain(words)
        .collect::<Vec<_>>()
        .join(" ")
}
```

### 2. `src/commands.rs`

Register maintained loft/sweep recipes and current aliases through shared gather input.

**Locate this existing block:**

```rust
pub fn canonical(line: &str) -> String {
    let line = if let Some(rest) = line
        .trim()
        .strip_prefix("nurbssurface_revolve")
    {
        format!("Nurbs Surface Revolve{rest}")
    } else {
        line.to_owned()
    };
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line = if let Some(rest) = line
        .trim()
        .strip_prefix("nurbssurface_revolve")
    {
        format!("Nurbs Surface Revolve{rest}")
    } else {
        line.to_owned()
    };
    let alias = crate::transform_commands::alias(&line);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn canonical(line: &str) -> String {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
```

## <a id="checkpoint-79d-exercise-original-sections-rail-groups-and-exact-persistence"></a>7. Exercise sections rail groups and exact persistence

**What this code does:** Exercise original sections rail groups and exact persistence.

### 1. `src/loft_tests.rs`

Copy integrated maintained loft/sweep group/order/options/source/archive fixtures.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
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
#[test]
fn original_loft_options_keep_order_placement_sources_one_undo_and_exact_archive() {
    let mut e = editor();
    for line in [
        "Curve w-1,-1,0 w0,-1,1 w1,-1,0",
        "Curve w-1,0,0 w0,0,2 w1,0,0",
        "Curve w-1,1,0 w0,1,1 w1,1,0",
    ] {
        command(&mut e, line).unwrap()
    }
    let ids = e
        .scene
        .paths()
        .iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();
    let originals = e
        .scene
        .paths()
        .iter()
        .map(|r| r.prepared.source.clone())
        .collect::<Vec<_>>();
    for id in &ids {
        e.scene
            .place_following(
                *id,
                session_rust::Xform::translation(10., 0., 0.),
            )
            .unwrap()
    }
    e.select_objects(ids.clone(), false);
    command(&mut e, "Nurbs Surface Loft Straight").unwrap();
    let surface = e.scene.objects()[0]
        .surface
        .as_ref()
        .unwrap()
        .source
        .clone();
    assert_eq!(surface.degree(1), 1);
    let b = e
        .scene
        .selected_bounds(e.scene.objects()[0].id)
        .unwrap();
    assert!((b.min[0] - 9.).abs() < 1e-6 && (b.max[0] - 11.).abs() < 1e-6);
    let _ = surface.guid();
    let proto = surface.to_proto();
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    let mut saved = Scene::empty();
    crate::scene_document::open(&mut saved, &bytes).unwrap();
    assert_eq!(
        saved.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        proto
    );
    assert_eq!(saved.paths().len(), 3);
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    e.select_objects(ids, false);
    command(&mut e, "Nurbs Surface Loft Smooth").unwrap();
    assert!(
        e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
            .degree(1)
            > 1
    );
    for (row, source) in e.scene.paths().iter().zip(originals) {
        assert!(
            matches!((&row.prepared.source,source),(crate::chain::Source::Curve(a,_),crate::chain::Source::Curve(b,_))if Rc::ptr_eq(a,&b))
        )
    }
}
#[test]
fn closed_loft_short_input_refuses_before_adoption_and_retains_redo_and_cancel_selection()
 {
    let mut e = editor();
    command(
        &mut e,
        "Polyline w0,0,0 w1,0,0 w1,1,0 w0,1,0 w0,0,0",
    )
    .unwrap();
    command(
        &mut e,
        "Polyline w0,0,1 w1,0,1 w1,1,1 w0,1,1 w0,0,1",
    )
    .unwrap();
    command(&mut e, "Point w4,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    let ids = e
        .scene
        .paths()
        .iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();
    e.select_objects(ids.clone(), false);
    assert!(command(&mut e, "Loft Closed").is_err());
    assert!(e.draft.is_none());
    assert_eq!(e.selected_objects(), ids);
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    e.select_objects([], false);
    command(&mut e, "Loft").unwrap();
    e.apply(Action::GatherPick(Some(ids[0])))
        .unwrap();
    assert!(command(&mut e, "").is_err());
    assert_eq!(
        e.draft
            .as_ref()
            .unwrap()
            .gather
            .as_ref()
            .unwrap()
            .progress
            .rows[0],
        vec![ids[0].value()]
    );
    command(&mut e, "Escape").unwrap();
    assert!(e.selected_objects().is_empty());
}
#[test]
fn one_rail_pick_advances_automatically_and_cannot_be_reused_as_a_profile() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w3,0,0").unwrap();
    command(
        &mut e,
        "Polyline w0,-0.2,-0.2 w0,0.2,-0.2 w0,0.2,0.2 w0,-0.2,0.2 w0,-0.2,-0.2",
    )
    .unwrap();
    let rail = e.scene.lines()[0].id;
    let profile = e.scene.paths()[0].id;
    command(&mut e, "Nurbs Surface Sweep1").unwrap();
    e.apply(Action::GatherPick(Some(rail)))
        .unwrap();
    assert_eq!(
        e.draft
            .as_ref()
            .unwrap()
            .gather
            .as_ref()
            .unwrap()
            .progress
            .step,
        1
    );
    assert!(
        e.apply(Action::GatherPick(Some(rail)))
            .is_err()
    );
    e.apply(Action::GatherPick(Some(profile)))
        .unwrap();
    command(&mut e, "").unwrap();
    assert!(
        e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
            .is_closed(0)
    );
    assert_eq!(e.scene.paths().len(), 1);
    assert_eq!(e.scene.lines().len(), 1);
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn two_rail_preselection_keeps_group_order_original_shapes_and_aliases() {
    let mut e = editor();
    for line in [
        "Line w0,0,0 w3,0,0",
        "Line w0,1,0 w3,1,0.5",
        "Curve w0,0,0 w0,0.5,0.3 w0,1,0",
    ] {
        command(&mut e, line).unwrap()
    }
    let ids = vec![
        e.scene.lines()[0].id,
        e.scene.lines()[1].id,
        e.scene.paths()[0].id,
    ];
    let source = e.scene.lines()[1].prepared.source.clone();
    let original = source.to_proto();
    e.select_objects(ids, false);
    command(&mut e, "NURBSSURFACE_SWEEP2").unwrap();
    assert!(e.draft.is_none());
    assert_eq!(e.scene.objects().len(), 1);
    assert!(
        e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
            .is_valid()
    );
    assert_eq!(source.to_proto(), original);
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.lines()[1].prepared.source
    ));
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    assert_eq!(
        crate::commands::canonical("nurbssurface_sweep1"),
        "Nurbs Surface Sweep1"
    );
    assert_eq!(
        crate::commands::canonical("nurbssurface_loft Straight"),
        "Nurbs Surface Loft Straight"
    );
    assert_eq!(
        crate::commands::canonical("nurbssurface_revolve"),
        "Nurbs Surface Revolve"
    );
}
```

### 2. `src/lib.rs`

Register the current surfacing component.

**Locate this existing block:**

```rust
pub mod sweep1_geometry;

pub mod source_mesh_topology;
pub mod sweep2_geometry;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod sweep1_geometry;

pub mod sweep2_geometry;

#[cfg(test)]
mod loft_tests;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-79zza-sample-original-network-curve-stations"></a>8. Sample network curve stations

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Drag Left-&gt;Right (Window) vs Right-&gt;Left (Crossing)</text>
  <rect x="54" y="54" width="108" height="88" fill="#dbeafe" fill-opacity="0.35" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">dx &gt; 0: Inside Only</text>
  <rect x="180" y="54" width="108" height="88" fill="#dcfce7" fill-opacity="0.35" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="234" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">dx &lt; 0: Any Overlap</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen AABB Containment vs Segment-Rect Intersection</text>
  <rect x="434" y="56" width="112" height="84" fill="none" stroke="#2563eb" stroke-width="2"/>
  <polygon points="458,122 490,74 522,122" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="574" y="56" width="112" height="84" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <polygon points="638,126 676,68 712,126" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Multi-Object Selection Set Updated</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 876,74 914,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <polygon points="944,136 982,74 1020,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

**What this code does:** Write control bounds, tolerant crossing checks and interpolation helpers.

### 1. `src/network_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
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

    if (0..3)
        .any(|k| one[0][k] > two[1][k] + tolerance || two[0][k] > one[1][k] + tolerance)
    {
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
```

### 2. `src/lib.rs`

Register the original network-surface owner.

**Locate this existing block:**

```rust
mod loft_tests;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod loft_tests;

pub mod network_geometry;
pub mod source_mesh_topology;
```

## <a id="checkpoint-79zzb-classify-two-original-network-directions"></a>9. Classify two network directions

**What this code does:** Find original curve crossings and assign opposite directions through their graph.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    DENSITY.min((MOST - 1) / spans).max(1) * spans + 1
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
            if let Some((s, t)) = crossing(
                &input[i],
                &input[j],
                (&boxes[i], &boxes[j]),
                tolerance,
            ) {
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

    let us: Vec<usize> = (0..count)
        .filter(|&i| side[i] == Some(false))
        .collect();
    let vs: Vec<usize> = (0..count)
        .filter(|&i| side[i] == Some(true))
        .collect();

    if us.len() < 2 || vs.len() < 2 {
        return Err(two.into());
    }

    if us
        .iter()
        .any(|&i| vs.iter().any(|&j| cross[i][j].is_none()))
    {
        return Err(
            "Every curve must cross each curve of the other direction once".into(),
        );
    }

    Ok((us, vs))
}
```

## <a id="checkpoint-79zzc-order-and-orient-original-network-curves"></a>10. Order and orient network curves

**What this code does:** Sort each family along the other and orient its crossing parameters consistently.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    Ok((us, vs))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok((us, vs))
}

fn oriented(
    input: &[NurbsCurve],
    us: &mut [usize],
    vs: &mut [usize],
    cross: &mut Crossings,
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
            let params: Vec<f64> = others
                .iter()
                .map(|&j| cross[i][j].unwrap())
                .collect();

            if params
                .windows(2)
                .all(|pair| pair[0] < pair[1])
            {
                continue;
            }

            if !params
                .windows(2)
                .all(|pair| pair[0] > pair[1])
            {
                return Err(
                    "Every curve must cross each curve of the other direction once"
                        .into(),
                );
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
```

## <a id="checkpoint-79zzd-sample-original-network-lofts-and-tensor"></a>11. Sample network lofts and tensor

**What this code does:** Sample both curve-family lofts and the tensor through their shared crossings.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    Ok(curves)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(curves)
}

struct Samples {
    across_u: Vec<Vec<Point>>,
    across_v: Vec<Vec<Point>>,
    tensor: Vec<Vec<Point>>,
}

fn sampled(
    curves: &[NurbsCurve],
    us: &[usize],
    vs: &[usize],
    cross: &Crossings,
) -> Samples {
    let (rows, columns) = (us.len(), vs.len());
    // u curve i crosses v curve j at s[i][j] on the u curve and r[i][j] on the v curve
    let s: Vec<Vec<f64>> = us
        .iter()
        .map(|&i| {
            vs.iter()
                .map(|&j| cross[i][j].unwrap())
                .collect()
        })
        .collect();
    let r: Vec<Vec<f64>> = vs
        .iter()
        .map(|&j| {
            us.iter()
                .map(|&i| cross[j][i].unwrap())
                .collect()
        })
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
    let xs: Vec<f64> = (0..kx)
        .map(|a| a as f64 / (kx - 1) as f64)
        .collect();
    let ys: Vec<f64> = (0..ky)
        .map(|b| b as f64 / (ky - 1) as f64)
        .collect();
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
            let row: Vec<Point> = lines
                .iter()
                .map(|line| at(line, *y))
                .collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    Samples {
        across_u,
        across_v,
        tensor,
    }
}
```

## <a id="checkpoint-79zze-fit-the-original-gordon-surface"></a>12. Fit the Gordon surface

**What this code does:** Combine the two lofts minus their shared tensor.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

fn fitted(samples: Samples) -> Result<NurbsSurface, String> {
    let Samples {
        across_u,
        across_v,
        tensor,
    } = samples;
    let (kx, ky) = (across_u.len(), across_v.len());
    // fit: rows through the samples, then columns through the rows' control points
    let fitted: Vec<NurbsCurve> = (0..ky)
        .map(|b| {
            let row: Vec<Point> = (0..kx)
                .map(|a| {
                    let (p, q, t) = (
                        &across_u[a][b],
                        &across_v[b][a],
                        &tensor[b][a],
                    );
                    Point::new(
                        p[0] + q[0] - t[0],
                        p[1] + q[1] - t[1],
                        p[2] + q[2] - t[2],
                    )
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
    let mut surface = NurbsSurface::new(
        3,
        false,
        u.order(),
        v.order(),
        width,
        v.cv_count(),
    );

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
```

## Build & Run

![Recorded visual output for Lesson 15.3 (79zze-fit-the-original-gordon-surface)](../../screenshots/journey/79zze-fit-the-original-gordon-surface.png)

```sh
npm --prefix ../session_tests run course -- dependencies 15.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 15.3
npm --prefix ../session_tests run course -- save 15.3
```

Continue with [15.4 · Two-direction networks and corner patches](80-build-a-surface-through-two-directions-of-original-curves.md).
