# 13.1.17 · Encode original parameters and topology

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Encode knots, local points, pcurve values and curve descriptors. Record the ordered edge, wire, face, shell and solid references.

## Type

Continue from [Find an original frame and pack key words](69zzb-find-an-original-frame-and-pack-key-words.md). [Save or recover your work](recovery.md).

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
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
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69zzc-encode-original-parameters-and-topology-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.17
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Ordered geometry and topology can contribute to a normalized key without changing the original source.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for encode original parameters and topology.](../screenshots/journey/69zza-move-between-original-rigid-frames.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.15 · Move between original rigid frames](69zza-move-between-original-rigid-frames.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original controls → rigid frame → local shape words → candidate rigid copy.



Why keep topology in a geometric key?

Identical control coordinates can describe different holes or connectivity. Their ordered references distinguish those originals.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a changed hole, reversed edge reference and knot span through the words.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.17
npm --prefix ../session_tests run course -- save 13.1.17
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original frame and key helpers prepare reuse of one display walk for congruent CAD originals.



[Full validation scope](release.md).

</details>
