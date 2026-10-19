# 16.4.31 · Collect original boundary strokes

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Build surface grids and closed mesh outlines, then combine BRep vertices, edges and curved faces. Keep each original boundary as an ordered stroke.

## Type

Continue from [Sample original CAD families for selection](84znc-sample-original-cad-families-for-selection.md). [Save or recover your work](recovery.md).

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

<details>
<summary>Locate the existing block</summary>

```rust
        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
}
```

</details>

After that block, type:

```rust
/// The surface's grid lines both ways, `steps` steps each.
fn surface_strokes(surface: &NurbsSurface, steps: usize) -> Vec<Vec<[f64; 3]>> {
    let Some(((u0, u1), (v0, v1))) = surface.domain(0).zip(surface.domain(1)) else {
        return Vec::new();
    };
    let at = |i: usize, j: usize| {
        let u = u0 + (u1 - u0) * i as f64 / steps as f64;
        let v = v0 + (v1 - v0) * j as f64 / steps as f64;
        surface.point_at(u, v).map(|p| xyz(&p))
    };
    let rows = (0..=steps).map(|i| (0..=steps).filter_map(|j| at(i, j)).collect());
    let columns = (0..=steps).map(|j| (0..=steps).filter_map(|i| at(i, j)).collect());
    rows.chain(columns).collect()
}

/// Every face of a mesh as its closed outline.
fn mesh_strokes(mesh: &Mesh) -> Vec<Vec<[f64; 3]>> {
    mesh.face
        .values()
        .map(|face| {
            let mut loop_: Vec<[f64; 3]> = face
                .iter()
                .filter_map(|key| mesh.vertex.get(key).map(|v| [v.x, v.y, v.z]))
                .collect();
            loop_.extend(loop_.first().copied());
            loop_
        })
        .collect()
}

/// Vertices, edges and the grid lines of curved faces.
fn brep_strokes(brep: &BRep) -> Vec<Vec<[f64; 3]>> {
    let vertices = brep
        .m_vertices
        .iter()
        .map(|vertex| vec![xyz(&vertex.point)]);
    let edges = brep
        .m_curves_3d
        .iter()
        .map(|curve| curve_samples(curve, EDGE_STEPS).collect());
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84znd-collect-original-boundary-strokes-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.31
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

![Native renderer output for collect original boundary strokes.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD samples → screen region → window or crossing result.



Why close a mesh face outline explicitly?

The last vertex and first vertex form an edge that crossing selection must also test.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the last edge of a triangular mesh face and a curved BRep face through the stroke collector.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.31
npm --prefix ../session_tests run course -- save 16.4.31
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
