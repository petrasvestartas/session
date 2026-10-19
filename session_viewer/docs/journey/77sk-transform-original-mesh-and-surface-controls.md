# 15.1.12 · Transform original mesh and surface controls

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

Clone the original source before changing addressed points.

## Type

Continue from [Preserve rational weights and validate BRep boundaries](77sj-preserve-rational-weights-and-validate-brep-boundaries.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Transform original mesh and surface controls.

<details>
<summary>Locate the existing block</summary>

```rust
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}
```

</details>

After that block, type:

```rust

fn transform_mesh(source: &Rc<session_rust::Mesh>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut mesh = (**source).clone();

    for key in mesh_keys(&mesh, target)? {
        let point = mesh
            .vertex_point(key)
            .ok_or("Missing vertex")?
            .transformed(delta);
        mesh.vertex
            .get_mut(&key)
            .ok_or("Missing vertex")?
            .set_position(point);
    }

    mesh.clear_triangle_bvh(); // stale after moving vertices
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

fn transform_nurbssurface(source: &Rc<session_rust::NurbsSurface>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut surface = (**source).clone();

    for (u, v) in surface_keys(&surface, target)? {
        let p = surface
            .get_cv(u, v)
            .ok_or("Missing surface control")?
            .transformed(delta);

        if !surface.set_cv(u, v, &p) {
            return Err("Cannot set surface control".into());
        }
    }

    surface.m_mesh = None; // drop the cached mesh
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sk-transform-original-mesh-and-surface-controls-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.12
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

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for transform original mesh and surface controls.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Mesh edits update only resolved vertex keys and clear the triangle BVH. Surface edits update original controls and drop the cached mesh. The original source object remains available for rollback and Undo; later display preparation regenerates geometry from the edited source.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Why clear derived acceleration and tessellation caches after source controls move?

Those caches describe the previous source positions and must be regenerated.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Move one face of two adjacent mesh faces. List the shared vertices that change and the untouched vertices that remain identical.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.12
npm --prefix ../session_tests run course -- save 15.1.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
