# 13.1.18 · Assemble an original rigid shape key

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Assemble the existing encoders for every original surface, curve, vertex and topology record. Keep the original frame and physical quantization step beside the words.

## Type

Continue from [Encode original parameters and topology](69zzc-encode-original-parameters-and-topology.md). [Save or recover your work](recovery.md).

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
    pub words: Vec<u64>, // the shape, word by word
    pub frame: Frame,    // where this BRep lies
    pub step: f64,       // the length one point word counts
}

impl ShapeKey {
```

</details>

After that block, type:

```rust
    /// The key of `b`; None when it has no frame.
    pub fn of(b: &BRep) -> Option<Self> {
        let (frame, size) = frame_of(&surface_points(b))?;
        let mut key = Self {
            words: Vec::new(),
            frame,
            step: size * SAME_POINT,
        };
        key.push_counts(b);
        key.push_float(b.width);
        key.push_color(&b.surfacecolor);

        for surface in &b.m_surfaces {
            key.push_words(&[
                surface.m_dim as u64,
                surface.m_is_rat as u64,
                surface.m_order[0] as u64,
                surface.m_order[1] as u64,
                surface.m_cv_count[0] as u64,
                surface.m_cv_count[1] as u64,
            ]);
            key.push_knots(&surface.m_nurbsknot[0]);
            key.push_knots(&surface.m_nurbsknot[1]);

            for i in 0..surface.cv_count(0) {
                for j in 0..surface.cv_count(1) {
                    let p = surface.get_cv(i, j).unwrap_or_default();
                    key.push_point([p[0], p[1], p[2]]);
                    key.push_number(surface.weight(i, j));
                }
            }
        }

        for curve in &b.m_curves_3d {
            key.push_curve_shape(curve);

            for i in 0..curve.cv_count() {
                let p = curve.get_cv(i).unwrap_or_default();
                key.push_point([p[0], p[1], p[2]]);
                key.push_number(curve.weight(i));
            }
        }

        for curve in &b.m_curves_2d {
            key.push_curve_shape(curve);
            key.push_parameters(&curve.m_cv);
        }

        for vertex in &b.m_vertices {
            key.push_point([vertex.point[0], vertex.point[1], vertex.point[2]]);
        }

        key.push_topology(b);
        Some(key)
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69a-describe-a-rigid-frame-and-original-shape-key-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.18
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The complete key can identify candidates. Physical-size and original-source checks must still approve display reuse.

### Recorded visual result

**Previous native render — current verification pending.**

Independent yellow and blue shared-definition placements remain visible beside original CAD borders.

![Native renderer output for assemble an original rigid shape key.](../screenshots/journey/69zza-move-between-original-rigid-frames.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.15 · Move between original rigid frames](69zza-move-between-original-rigid-frames.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original controls → rigid frame → local shape words → candidate rigid copy.



Does a matching normalized key prove a rigid copy?

No. Validate the original source and compare physical size before accepting a recorded display walk.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare moved, rotated, mirrored, differently sized and changed-hole originals without changing their source GUIDs.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.18
npm --prefix ../session_tests run course -- save 13.1.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited instance acceptance remains authored and unrun. The final congruent-CAD exercise inspects one walk/one replay, original identities, Save/Open, Delete/Undo/Redo and Close; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 13.1.18
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
