# 9.1.10 · Follow original isoparametric boundaries

**Chapter 9 · CAD surfaces and topology · Section 9.1 · Surface controls and trimming** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Follow an exact constant-u or constant-v boundary through the existing face mesh. Refuse unsupported pcurves and preserve the closing vertex of an original closed edge.

## Type

Continue from [Read original edge-use samples](55za-read-original-edge-use-samples.md). [Save or recover your work](recovery.md).

### 1. `src/cad_edges.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        }

        if match best {
            Some((bd, _)) => d < bd,
            None => true,
        } {
            best = Some((d, v));
        }
    }

    Some(best?.1)
}
```

</details>

After that block, type:

```rust
pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let e = b.m_edges.get(eu.edge)?;

    if e.degenerated {
        return None;
    }

    let (p0, p1) = pcurve_ends(b, eu)?;

    if !p0.into_iter().chain(p1).all(f64::is_finite) || (p0[0] != p1[0] && p0[1] != p1[1]) {
        return None;
    }

    let fixed = if (p1[0] - p0[0]).abs() <= (p1[1] - p0[1]).abs() {
        0
    } else {
        1
    };
    let (fixed_name, free_name) = if fixed == 0 { ("u", "v") } else { ("v", "u") };
    let vals = sample_values(fm, fixed_name);

    if vals.is_empty() {
        return None;
    }

    let face = b.m_faces.get(eu.face)?;
    let srf = b.m_surfaces.get(face.surface_index as usize)?;
    let wrap = if srf.is_closed(fixed) {
        srf.domain(fixed)
    } else {
        None
    };
    let at = nearest_sample(&vals, p0[fixed], wrap)?;
    let wrapped_start = match wrap {
        Some((start, end)) => p0[fixed] - (end - start) == at,
        None => false,
    };

    if at != p0[fixed] && !wrapped_start {
        return None;
    }

    let mut on_line: Vec<(f64, usize)> = Vec::new();

    for (&key, vd) in fm.vertex.iter() {
        let (Some(&f), Some(&t)) = (vd.attributes.get(fixed_name), vd.attributes.get(free_name))
        else {
            continue;
        };
        let lo = p0[1 - fixed].min(p1[1 - fixed]);
        let hi = p0[1 - fixed].max(p1[1 - fixed]);

        if f == at && t >= lo && t <= hi {
            on_line.push((t, key));
        }
    }

    if on_line.len() < 2 {
        return None;
    }

    on_line.sort_unstable_by(parameter_order);
    let mut keys = Vec::with_capacity(on_line.len());

    for (_, key) in on_line {
        keys.push(key);
    }

    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }

    Some(keys)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/55zb-follow-original-isoparametric-boundaries-1.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original isoparametric boundaries can be followed; BRep display connection remains preparatory.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for follow original isoparametric boundaries.](../screenshots/journey/55za-read-original-edge-use-samples.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [9.1.9 · Read original edge-use samples](55za-read-original-edge-use-samples.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original edge use → face-mesh samples → ordered CAD boundary chain.



Why require the exact parameter sample?

The chain must follow triangles already drawn at that boundary. A nearby independent curve sample would describe a different edge path.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace fixed and free parameters along a cylindrical rim. Identify the periodic start match and closing vertex.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 9.1.10
npm --prefix ../session_tests run course -- save 9.1.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original boundary helpers prepare coherent CAD edge drawing from the existing face mesh.



[Full validation scope](release.md).

</details>
