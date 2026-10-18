# 15.1.49 · Revolve original profiles around an axis

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Describe one shared recipe for profile selection, two axis points and a bounded angle. Reject a zero-length axis before calling the kernel. Convert degrees to radians once, validate every resulting surface, and return all selected profiles in one undo transaction.

## Type

Continue from [Connect the original extrusion recipe](77zzn-connect-the-original-extrusion-recipe.md). [Save or recover your work](recovery.md).

### 1. `src/revolve_geometry.rs`

Type the original axis validation, exact revolution and complete recipe result.

Create the file and type:

```rust
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-1.rs"
```

### 2. `src/lib.rs`

Compile the prepared original revolution recipe.

<details>
<summary>Locate the existing block</summary>

```rust
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

pub mod surfacing;

pub mod extrude_geometry;
```

</details>

After that block, type:

```rust

pub mod revolve_geometry;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/revolve_geometry.rs`

Supply the unchanged original radius, seam and axis validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Revolve",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// `profile` turned `degrees` right-handed about the axis from `start` to `end`.
pub fn revolve(
    profile: &NurbsCurve,
    start: &Point,
    end: &Point,
    degrees: f64,
) -> Result<NurbsSurface, String> {
    let axis = end - start;

    if axis.magnitude() <= 1e-12 {
        return Err("The axis start and end must differ".into());
    }

    if !(degrees > 0.0 && degrees <= 360.0) {
        return Err("The angle must be above 0 and at most 360".into());
    }

    Ok(Primitives::create_revolve(
        profile,
        start,
        &axis,
        degrees.to_radians(),
    ))
}

/// One surface per profile, all in one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (Some(start), Some(end), Some(degrees)) = (input.point(0), input.point(1), input.number(2))
    else {
        return Err("Nurbs Surface Revolve needs an axis and an angle".into());
    };
    let geometries = input.curves[0]
        .iter()
        .map(|picked| checked(revolve(&picked.curve, &start, &end, degrees)?, "revolve"))
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Made {
        message: format!(
            "Revolved {} by {degrees}°",
            count(geometries.len(), "curve")
        ),
        geometries,
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-3.rs" skip=1946
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.49
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for revolve original profiles around an axis.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original profile → axis and angle → kernel surface → one result.



Why revolve the original curve instead of its display polyline?

The kernel retains its exact rational surface and editable source geometry. The display polyline is only a tessellation for drawing.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a 90-degree turn and a complete turn of a straight profile. Compare the surface seam and radius. The following registry checkpoint connects this prepared recipe to the browser command.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.49
npm --prefix ../session_tests run course -- save 15.1.49
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This original recipe remains preparatory until the existing command registry connects it. Earlier browser drawing stays unchanged.



[Full validation scope](release.md).

</details>
