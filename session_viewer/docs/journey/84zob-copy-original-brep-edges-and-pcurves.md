# 16.4.35 · Copy original BRep edges and pcurves

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Copy an edge, its endpoint vertices and its surface pcurves into a new BRep. Reuse vertex mappings and preserve seam curves, tolerances and degenerated edges.

## Type

Continue from [Classify originals and isolate one mesh face](84zoa-classify-originals-and-isolate-one-mesh-face.md). [Save or recover your work](recovery.md).

### 1. `src/explode_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        let key = *part.faces().first()?;
        part.set_face_holes(key, rings);
    }

    part.name = mesh.name.clone();
    part.set_objectcolor(mesh.get_objectcolor().clone());
    Some(part)
}
```

</details>

After that block, type:

```rust
/// Copy edge `old` into `part`: its curve, its two vertices once each, and its pcurves on `surface`.
fn copy_edge(
    brep: &BRep,
    part: &mut BRep,
    old: i32,
    surface: i32,
    vertices: &mut HashMap<i32, i32>,
) -> Option<i32> {
    let edge = brep.m_edges.get(usize::try_from(old).ok()?)?;
    let curve = match usize::try_from(edge.curve_3d_index) {
        Ok(curve) if !edge.degenerated => part.add_curve_3d(brep.m_curves_3d.get(curve)?) as i32,
        _ => -1, // a pole or apex has no curve
    };
    let mut ends = [0; 2];

    for (end, old) in ends.iter_mut().zip([edge.start_vertex, edge.end_vertex]) {
        *end = match vertices.get(&old) {
            Some(&vertex) => vertex,
            None => {
                let source = brep.m_vertices.get(usize::try_from(old).ok()?)?;
                let vertex = part.add_vertex(&source.point, source.tolerance) as i32;
                vertices.insert(old, vertex);
                vertex
            }
        };
    }

    let copied = part.add_edge(curve, ends[0], ends[1]);
    part.m_edges[copied].tolerance = edge.tolerance;
    part.m_edges[copied].degenerated = edge.degenerated;

    for pcurve in edge
        .pcurves
        .iter()
        .filter(|pcurve| pcurve.surface_index == surface)
    {
        let first = part.add_curve_2d(
            brep.m_curves_2d
                .get(usize::try_from(pcurve.curve_2d_index).ok()?)?,
        ) as i32;
        // a seam runs twice over the surface, once each way
        let second = match usize::try_from(pcurve.curve_2d_index_2) {
            Ok(seam) => part.add_curve_2d(brep.m_curves_2d.get(seam)?) as i32,
            Err(_) => -1,
        };
        part.add_pcurve(copied, 0, first, second);
    }

    Some(copied as i32)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zob-copy-original-brep-edges-and-pcurves-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.35
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

BRep face extraction can preserve the required original edge and pcurve records.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for copy original brep edges and pcurves.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why keep both pcurves on a seam?

A periodic surface uses the same spatial edge on two parameter boundaries. Both uses must survive the separated face.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a cylindrical seam and a degenerated pole edge through the copied tables.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.35
npm --prefix ../session_tests run course -- save 16.4.35
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
