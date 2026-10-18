# 15.3 · Lofts, rail sweeps and network foundations

Build exact surfaces from ordered original sections and rails while preserving their source precision.

## Ownership and data flow

Retain section order, orientation and degree options before lofting. One-rail and two-rail sweeps frame profiles on original rails, align corresponding shapes and blend without mutating picked source owners. Freshness and degeneracy checks precede one generated-source transaction. Network station sampling, direction classification, orientation and Gordon blending prepare the two-direction recipe connected in the next lesson; temporary samples remain derived construction data.

## The maintained implementation

app/command/tool/surfacing.rs performs maintained section alignment and surface recipes; verbs/nurbs_surface_loft.rs, nurbs_surface_sweep1.rs and nurbs_surface_sweep2.rs register actual commands. Original session_rust NurbsCurve/NurbsSurface outputs remain authoritative; no display mesh is promoted to the final editable surface.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/deform.rs](../../../src/app/deform.rs), [state/edit.rs](../../../src/state/edit.rs), [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs), [app/command/tool/surfacing.rs](../../../src/app/command/tool/surfacing.rs), [app/walk/mesh.rs](../../../src/app/walk/mesh.rs), [app/walk/brep.rs](../../../src/app/walk/brep.rs), [build.rs](../../../build.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-79-retain-current-loft-section-order-and-degree-options"></a>

<details>
<summary>Implementation 79-retain-current-loft-section-order-and-degree-options: Retain current loft section order and degree options</summary>

Retain current loft section order and degree options.

Data flow: ordered original sections → Open Closed or Smooth Straight → original loft surface.

### 1. `src/loft_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/79-retain-current-loft-section-order-and-degree-options-typed-loft_geometry.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/loft_geometry.rs`

Supply only the unchanged original geometry validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
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
    (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
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
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/79-retain-current-loft-section-order-and-degree-options-typed-loft_geometry-fixtures.rs" skip=1087
```

</details>

### 3. `src/lib.rs`

Register the current surfacing component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod gather_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79-retain-current-loft-section-order-and-degree-options-02.rs" skip=31
```

### 4. `src/nurbs_loft_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/79-retain-current-loft-section-order-and-degree-options-typed-nurbs_loft_geometry.rs"
```

### 5. `src/lib.rs`

Register the current surfacing component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod loft_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79-retain-current-loft-section-order-and-degree-options-04.rs" skip=24
```

</details>

<a id="checkpoint-79aa-frame-original-profiles-on-their-rail"></a>

<details>
<summary>Implementation 79aa-frame-original-profiles-on-their-rail: Frame original profiles on their rail</summary>

Find each profile’s station from its sampled center and the nearest rail parameter.

Data flow: original rail and profiles → local section frames → kernel sweep surface.

### 1. `src/sweep1_geometry.rs`

Type the whole original sweep helpers using the existing kernel and gather owners.

Create the file and type:

```rust
--8<-- "journey/code/79aa-frame-original-profiles-on-their-rail-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original sweep module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod nurbs_loft_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79aa-frame-original-profiles-on-their-rail-2.rs" skip=30
```

</details>

<a id="checkpoint-79ab-carry-and-blend-original-profiles-along-the-rail"></a>

<details>
<summary>Implementation 79ab-carry-and-blend-original-profiles-along-the-rail: Carry and blend original profiles along the rail</summary>

Sort original profile stations, match their control rows with a straight kernel loft, and include their stations in the sampled rail frames.

Data flow: original rail and profiles → local section frames → kernel sweep surface.

### 1. `src/sweep1_geometry.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    let local = Xform::world_to_frame(
        &frame.origin(),
        &frame.x_axis(),
        &frame.y_axis(),
        &frame.z_axis(),
    );
    (t, local)
}
```

</details>

After that block, type:

```rust
/// The profiles carried along the rail, each kept where it sits; between two they blend.
pub fn sweep1(rail: &NurbsCurve, profiles: &[NurbsCurve]) -> Result<NurbsSurface, String> {
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
        section.transform(&Xform::to_frame(&rail.perpendicular_plane_at(t, false)));
        sections.push(section);
    }

    let surface = Primitives::create_loft(&sections, 3);

    if !surface.is_valid() {
        return Err("The profiles could not be swept along the rail".into());
    }

    Ok(surface)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/79ab-carry-and-blend-original-profiles-along-the-rail-1.rs"
```

</details>

</details>

<a id="checkpoint-79a-carry-original-profiles-along-one-rail"></a>

<details>
<summary>Implementation 79a-carry-original-profiles-along-one-rail: Connect original one-rail sweep inputs</summary>

Complete the existing gather recipe with one rail and one or more profiles.

Data flow: original rail and profiles → local section frames → kernel sweep surface.

### 1. `src/sweep1_geometry.rs`

Extend this original owner with the day’s complete input helpers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    section
```

</details>

Replace it with:

```rust
    }

    section
}

/// The sweep of the profiles along the rail.
fn build(input: &Input) -> Result<Made, String> {
    let profiles = curves(&input.curves[1]);
    let surface = sweep1(&input.curves[0][0].curve, &profiles)?;
    Ok(Made {
        geometries: vec![checked(surface, "sweep")?],
        message: format!("Swept {} along the rail", count(profiles.len(), "profile")),
    })
```

<details>
<summary>Locate this block</summary>

```rust
/// The rail parameter nearest the profile's middle and world to the rail frame there.
fn on_rail(rail: &NurbsCurve, profile: &NurbsCurve) -> (f64, Xform) {
    let mut points = profile.divide_by_count(SAMPLES, true).0;
```

</details>

Replace it with:

```rust
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
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/79a-carry-original-profiles-along-one-rail-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/sweep1_geometry.rs`

Supply only the unchanged original sweep validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{SAMPLES, checked, count, curves};
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
    let middle = Point::new(sum[0] / count, sum[1] / count, sum[2] / count);
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
pub fn sweep1(rail: &NurbsCurve, profiles: &[NurbsCurve]) -> Result<NurbsSurface, String> {
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
        section.transform(&Xform::to_frame(&rail.perpendicular_plane_at(t, false)));
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
        message: format!("Swept {} along the rail", count(profiles.len(), "profile")),
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/79a-carry-original-profiles-along-one-rail-2.rs" skip=4831
```

</details>

</details>

<a id="checkpoint-79b-align-shape-curves-between-two-original-rails"></a>

<details>
<summary>Implementation 79b-align-shape-curves-between-two-original-rails: Align shape curves between two original rails</summary>

Align shape curves between two original rails.

Data flow: first rail and second rail → aligned shape copies → original swept surface.

### 1. `src/sweep2_geometry.rs`

Type the original recipe and whole geometry helpers using the earlier shared owners.

Create the file and type:

```rust
--8<-- "journey/code/79b-align-shape-curves-between-two-original-rails-typed-sweep2_geometry.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/sweep2_geometry.rs`

Supply only the unchanged original geometry validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
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
        < second.point_at_start().distance(&start, None)
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
        message: format!("Swept {} along 2 rails", count(shapes.len(), "shape")),
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/79b-align-shape-curves-between-two-original-rails-typed-sweep2_geometry-fixtures.rs" skip=2342
```

</details>

### 3. `src/lib.rs`

Register the current surfacing component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod sweep1_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79b-align-shape-curves-between-two-original-rails-02.rs" skip=26
```

</details>

<a id="checkpoint-79c-register-loft-sweep-names-options-and-aliases"></a>

<details>
<summary>Implementation 79c-register-loft-sweep-names-options-and-aliases: Register loft sweep names options and aliases</summary>

Register loft sweep names options and aliases.

Data flow: current registered names and aliases → existing gather owner → one surfacing transaction.

### 1. `src/gather_registry.rs`

Register maintained loft/sweep recipes and current aliases through shared gather input.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gather_input::Recipe,editor::Action};
pub const ENTRIES:&[(&str,&Recipe)]=&[("Extrude",&crate::extrude_geometry::RECIPE),("Nurbs Surface Revolve",&crate::revolve_geometry::RECIPE)];
pub fn command(name:&str,text:&str)->Option<Action>{let(_,recipe)=ENTRIES.iter().find(|(label,_)|*label==name)?;Some(Action::GatherBegin{recipe,text:text.into()})}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/79c-register-loft-sweep-names-options-and-aliases-01.rs"
```

### 2. `src/commands.rs`

Register maintained loft/sweep recipes and current aliases through shared gather input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line=if let Some(rest)=line.trim().strip_prefix("nurbssurface_revolve"){format!("Nurbs Surface Revolve{rest}")}else{line.to_owned()};
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
```

</details>

Replace it with:

```rust
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
```

<details>
<summary>Locate this block</summary>

```rust
}

pub fn canonical(line: &str) -> String {
    let line=if let Some(rest)=line.trim().strip_prefix("nurbssurface_revolve"){format!("Nurbs Surface Revolve{rest}")}else{line.to_owned()};
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
```

</details>

Replace it with:

```rust
}

pub fn canonical(line: &str) -> String {
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
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
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).chain(crate::transform_commands::NAMES.iter().copied()).chain(crate::gather_registry::ENTRIES.iter().map(|(name,_)|*name)).collect()
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
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
    })
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::gather_registry::command(name,&words[count..].join(" ")){return Ok(Command::Editor(action));}
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
--8<-- "journey/code/79c-register-loft-sweep-names-options-and-aliases-02.rs"
```

```text
--8<-- "journey/code/79c-register-loft-sweep-names-options-and-aliases-03.rs"
```

</details>

</details>

<a id="checkpoint-79d-exercise-original-sections-rail-groups-and-exact-persistence"></a>

<details>
<summary>Implementation 79d-exercise-original-sections-rail-groups-and-exact-persistence: Exercise original sections rail groups and exact persistence</summary>

Exercise original sections rail groups and exact persistence.

Data flow: original curves in order → current Loft or Sweep → one history edit → original Save/Open.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/loft_tests.rs`

Copy integrated maintained loft/sweep group/order/options/source/archive fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/79d-exercise-original-sections-rail-groups-and-exact-persistence-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the current surfacing component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod sweep2_geometry;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/79d-exercise-original-sections-rail-groups-and-exact-persistence-02.rs" skip=26
```

</details>

</details>

<a id="checkpoint-79zza-sample-original-network-curve-stations"></a>

<details>
<summary>Implementation 79zza-sample-original-network-curve-stations: Sample original network curve stations</summary>

Write control bounds, tolerant crossing checks and interpolation helpers.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/79zza-sample-original-network-curve-stations-1.rs"
```

### 2. `src/lib.rs`

Register the original network-surface owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod loft_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79zza-sample-original-network-curve-stations-2.rs" skip=29
```

</details>

<a id="checkpoint-79zzb-classify-two-original-network-directions"></a>

<details>
<summary>Implementation 79zzb-classify-two-original-network-directions: Classify two original network directions</summary>

Find original curve crossings and assign opposite directions through their graph.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
/// Samples between crossings: `DENSITY` per span, at most `MOST` in all, crossings included.
fn sample_count(crossings: usize) -> usize {
    let spans = crossings - 1;
    DENSITY.min((MOST - 1) / spans).max(1) * spans + 1
}
```

</details>

After that block, type:

```rust
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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/79zzb-classify-two-original-network-directions-1.rs"
```

</details>

</details>

<a id="checkpoint-79zzc-order-and-orient-original-network-curves"></a>

<details>
<summary>Implementation 79zzc-order-and-orient-original-network-curves: Order and orient original network curves</summary>

Sort each family along the other and orient its crossing parameters consistently.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        .any(|&i| vs.iter().any(|&j| cross[i][j].is_none()))
    {
        return Err("Every curve must cross each curve of the other direction once".into());
    }

    Ok((us, vs))
}
```

</details>

After that block, type:

```rust
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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/79zzc-order-and-orient-original-network-curves-1.rs"
```

</details>

</details>

<a id="checkpoint-79zzd-sample-original-network-lofts-and-tensor"></a>

<details>
<summary>Implementation 79zzd-sample-original-network-lofts-and-tensor: Sample original network lofts and tensor</summary>

Sample both curve-family lofts and the tensor through their shared crossings.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
            curves[i].set_domain(t0, t1);

            for &j in others.iter() {
                cross[i][j] = cross[i][j].map(|t| t0 + t1 - t);
            }
        }
    }

    Ok(curves)
}
```

</details>

After that block, type:

```rust
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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/79zzd-sample-original-network-lofts-and-tensor-1.rs"
```

</details>

</details>

<a id="checkpoint-79zze-fit-the-original-gordon-surface"></a>

<details>
<summary>Implementation 79zze-fit-the-original-gordon-surface: Fit the original Gordon surface</summary>

Combine the two lofts minus their shared tensor.

Data flow: original curve crossings → ordered stations → sampled lofts → fitted surface.

### 1. `src/network_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
            let row: Vec<Point> = lines.iter().map(|line| at(line, *y)).collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    Samples { across_u, across_v, tensor }
}
```

</details>

After that block, type:

```rust
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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/79zze-fit-the-original-gordon-surface-1.rs"
```

</details>

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 79zze-fit-the-original-gordon-surface
npm --prefix ../session_tests run course -- check 79zze-fit-the-original-gordon-surface
npm --prefix ../session_tests run course -- save 79zze-fit-the-original-gordon-surface
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The original loft and sweep commands connected at checkpoint 79d-exercise-original-sections-rail-groups-and-exact-persistence remain available. Exercise ordered sections and one/two rails, then Undo and compare exact saved source data. The final network helpers classify curve directions and fit Gordon controls before the next lesson assembles that full recipe; trace one station through row/column control arrays without treating its helper as a new visible command.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Two-direction networks and corner patches](80-build-a-surface-through-two-directions-of-original-curves.md).
