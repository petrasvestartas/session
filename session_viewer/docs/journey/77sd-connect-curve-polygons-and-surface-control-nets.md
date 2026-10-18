# 15.1.5 · Connect curve polygons and surface control nets

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Reuse push for finite positions and original addresses.

## Type

Continue from [Collect finite original controls and mesh vertices](77sc-collect-finite-original-controls-and-mesh-vertices.md). [Save or recover your work](recovery.md).

### 1. `src/source_controls.rs`

Connect curve polygons and surface control nets.

<details>
<summary>Locate the existing block</summary>

```rust
            return None;
        }

        let index = self.points.len();
        self.points.push(Control { id, position });
        Some(index)
    }

    /// Every mesh vertex.
    fn mesh(&mut self, mesh: &session_rust::Mesh) {
        for key in mesh.vertices() {
            if let Some(vertex) = mesh.vertex.get(&key) {
                self.push(
                    ControlId::Vertex(key),
                    &Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }
}
```

</details>

After that block, type:

```rust

impl Controls {
    /// A curve's control polygon.
    fn curve(&mut self, curve: &NurbsCurve, index: usize) {
        let mut previous = None;

        for point in 0..curve.cv_count() {
            let current = match curve.get_cv(point) {
                Some(position) => self.push(
                    ControlId::Curve {
                        curve: index,
                        point,
                    },
                    &position,
                ),
                None => None,
            };

            if let (Some(start), Some(end)) = (previous, current) {
                self.links.push([start, end]);
            }

            previous = current;
        }
    }

    /// A surface's control net.
    fn surface(&mut self, surface: &NurbsSurface, index: usize) {
        let [width, height] = surface.m_cv_count;
        let mut previous = vec![None; height];

        for u in 0..width {
            let mut last = None;

            for (v, above) in previous.iter_mut().enumerate() {
                let current = match surface.get_cv(u, v) {
                    Some(position) => self.push(
                        ControlId::Surface {
                            surface: index,
                            u,
                            v,
                        },
                        &position,
                    ),
                    None => None,
                };

                if let (Some(start), Some(end)) = (*above, current) {
                    self.links.push([start, end]);
                }

                if let (Some(start), Some(end)) = (last, current) {
                    self.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sd-connect-curve-polygons-and-surface-control-nets-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.5
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

![Native renderer output for connect curve polygons and surface control nets.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A curve links successive accepted controls. A surface keeps one previous index per v column plus the previous point in the current u row, producing both grid directions with bounded temporary storage. Missing controls clear the corresponding continuation; links use contiguous display indices while edits retain source u/v addresses.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Should a control net connect across a missing or invalid control?

No. Clearing the previous index at that point breaks the link rather than bridging a gap.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Draw the links of a2×3 control grid. Remove its middle control and predict which horizontal and vertical links disappear.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.5
npm --prefix ../session_tests run course -- save 15.1.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
