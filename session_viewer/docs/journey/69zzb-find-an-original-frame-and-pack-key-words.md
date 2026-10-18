# 13.1.16 · Find an original frame and pack key words

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Choose well-separated original surface control points for a right-handed frame. Define the key records and encode counts, references, exact colours and rounded weights.

## Type

Continue from [Move between original rigid frames](69zza-move-between-original-rigid-frames.md). [Save or recover your work](recovery.md).

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    [a[0] / length, a[1] / length, a[2] / length]
}
```

</details>

After that block, type:

```rust
/// Every surface control point of a BRep, in storage order.
fn surface_points(b: &BRep) -> Vec<[f64; 3]> {
    let mut points = Vec::new();

    for surface in &b.m_surfaces {
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                let p = surface.get_cv(i, j).unwrap_or_default();
                points.push([p[0], p[1], p[2]]);
            }
        }
    }

    points
}

/// The frame of a BRep and its size: the first control point, the first one clearly away from
/// it for x, the first one clearly off that line for z. None for a flat line or nothing.
fn frame_of(points: &[[f64; 3]]) -> Option<(Frame, f64)> {
    let origin = *points.first()?;
    let size = points
        .iter()
        .map(|p| dot(sub(*p, origin), sub(*p, origin)).sqrt())
        .fold(0.0, f64::max);

    if !(size.is_finite() && size > 0.0) {
        return None;
    }

    let x = points
        .iter()
        .map(|p| sub(*p, origin))
        .find(|d| dot(*d, *d).sqrt() > AXIS_SPAN * size)?;
    let x = unit(x);
    let z = points
        .iter()
        .map(|p| cross(x, sub(*p, origin)))
        .find(|c| dot(*c, *c).sqrt() > AXIS_SPAN * size)?;
    let z = unit(z);
    let y = cross(z, x);
    let frame = Frame {
        origin,
        axes: [x, y, z],
    };

    Some((frame, size))
}

/// What makes two BReps the same shape: their topology, parameters, weights, colours and widths
/// exactly, and their 3D points in their own frames up to `SAME_POINT`.
pub struct ShapeKey {
    pub words: Vec<u64>, // the shape, word by word
    pub frame: Frame,    // where this BRep lies
    pub step: f64,       // the length one point word counts
}

impl ShapeKey {
    fn push_words(&mut self, words: &[u64]) {
        self.words.extend_from_slice(words);
    }

    fn push_float(&mut self, value: f64) {
        self.words.push(value.to_bits());
    }

    /// A weight, rounded to `SAME_NUMBER`.
    fn push_number(&mut self, value: f64) {
        self.words.push((value / SAME_NUMBER).round() as i64 as u64);
    }

    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }

}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69zzb-find-an-original-frame-and-pack-key-words-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.16
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Well-separated controls provide a stable frame and key words. Rigid-copy acceptance is still deferred.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for find an original frame and pack key words.](../screenshots/journey/69zza-move-between-original-rigid-frames.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.15 · Move between original rigid frames](69zza-move-between-original-rigid-frames.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original controls → rigid frame → local shape words → candidate rigid copy.



Why choose axes relative to the original size?

A relative separation threshold avoids unstable axes for both very small and very large originals. A line or empty control set has no usable frame.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace an empty set, a line and three non-collinear controls. Compare exact colour bits with rounded weight words.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.16
npm --prefix ../session_tests run course -- save 13.1.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original frame and key helpers prepare reuse of one display walk for congruent CAD originals.



[Full validation scope](release.md).

</details>
