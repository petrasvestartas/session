# 15.1.46 · Build closed curved extrusion topology

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 30–59 minutes.** [Estimate](typing-load.md).

Build the side surface from an oriented original curve. Share the lower, upper and seam edges, attach their surface-parameter curves, and join the side and both caps into one solid. Preserve the original curve precision throughout.

## Type

Continue from [Map cap curves to surface parameters](77zzk-map-cap-curves-to-surface-parameters.md). [Save or recover your work](recovery.md).

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    brep.add_pcurve(edge, surface, pcurve as i32, -1);
    let wire = brep.add_wire(&[BRepRef::new(edge as i32, orientation)]);
    brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0)
}
```

</details>

After that block, type:

```rust
/// A closed planar curve extruded into a solid: one side face with a seam and two flat caps.
fn capped(curve: &NurbsCurve, offset: &Vector, normal: &Vector) -> Result<BRep, String> {
    let mut bottom = curve.clone();

    // the side faces outward when the curve turns counter-clockwise about the offset
    let outline = bottom.divide_by_count(64, false).0;

    if loop_normal(&outline).is_some_and(|own| own.dot(normal) < 0.0) {
        bottom.reverse();
    }

    bottom.set_domain(0.0, 1.0);
    let top = bottom.transformed(&Xform::translation(offset[0], offset[1], offset[2]));
    let body = Primitives::create_extrusion(&bottom, offset);

    if !body.is_valid() {
        return Err("The curve could not be extruded".into());
    }

    let mut brep = BRep::new();
    let low = bottom.point_at_start();
    let high = &low + offset;
    let start = brep.add_vertex(&low, 0.0) as i32;
    let end = brep.add_vertex(&high, 0.0) as i32;
    let curve_low = brep.add_curve_3d(&bottom) as i32;
    let edge_low = brep.add_edge(curve_low, start, start);
    let curve_high = brep.add_curve_3d(&top) as i32;
    let edge_high = brep.add_edge(curve_high, end, end);
    let curve_seam = brep.add_curve_3d(&NurbsCurve::create(false, 1, &[low.clone(), high])) as i32;
    let edge_seam = brep.add_edge(curve_seam, start, end);
    let surface = brep.add_surface(&body);
    let (u0, u1) = body.domain(0).unwrap_or((0.0, 1.0));
    let (v0, v1) = body.domain(1).unwrap_or((0.0, 1.0));
    let pcurve_low = brep.add_curve_2d(&uv_line(u0, v0, u1, v0));
    brep.add_pcurve(edge_low, surface, pcurve_low as i32, -1);
    let pcurve_high = brep.add_curve_2d(&uv_line(u0, v1, u1, v1));
    brep.add_pcurve(edge_high, surface, pcurve_high as i32, -1);
    let right = brep.add_curve_2d(&uv_line(u1, v0, u1, v1));
    let left = brep.add_curve_2d(&uv_line(u0, v0, u0, v1));
    brep.add_pcurve(edge_seam, surface, right as i32, left as i32);
    let wire = brep.add_wire(&[
        BRepRef::new(edge_low as i32, F),
        BRepRef::new(edge_seam as i32, F),
        BRepRef::new(edge_high as i32, R),
        BRepRef::new(edge_seam as i32, R),
    ]);
    let side = brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0);
    let points: Vec<Point> = (0..bottom.cv_count())
        .filter_map(|i| bottom.get_cv(i))
        .collect();
    let lifted: Vec<Point> = points.iter().map(|p| p + offset).collect();
    let floor = cap_face(
        &mut brep,
        &cap_patch(&points, &low, &-normal),
        &bottom,
        edge_low as usize,
    );
    let roof = cap_face(
        &mut brep,
        &cap_patch(&lifted, &(&low + offset), normal),
        &top,
        edge_high as usize,
    );
    let shell = brep.add_shell(&[
        BRepRef::new(side as i32, F),
        BRepRef::new(floor as i32, F),
        BRepRef::new(roof as i32, F),
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzl-build-closed-curved-extrusion-topology-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.46
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

![Native renderer output for build closed curved extrusion topology.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curves → offset and cap rules → shared BRep topology.



Why does the seam edge need two parameter curves?

The periodic side meets the same three-dimensional seam at both ends of its surface domain. Each side of that seam needs its own parameter location.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the seam through the side wire twice, once forward and once reversed. These helpers remain preparatory until the recipe connects.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.46
npm --prefix ../session_tests run course -- save 15.1.46
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original extrusion helpers are preparatory. The existing command registry connects the completed recipe in 78; earlier helper checkpoints keep the browser drawing unchanged.



[Full validation scope](release.md).

</details>
