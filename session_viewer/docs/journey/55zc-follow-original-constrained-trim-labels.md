# 9.1.11 · Follow original constrained trim labels

**Chapter 9 · CAD surfaces and topology · Section 9.1 · Surface controls and trimming** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Read ordered edge-use and interval labels from mesh vertices. Keep unique sample keys, reject gaps and insert finite interior trim samples in total parameter order.

## Type

Continue from [Follow original isoparametric boundaries](55zb-follow-original-isoparametric-boundaries.md). [Save or recover your work](recovery.md).

### 1. `src/cad_edges.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }

    Some(keys)
}

pub struct EdgeChain {
    pub edge: usize,
    pub face: usize,
    pub keys: Vec<usize>,
    pub other: Option<usize>,
}
```

</details>

After that block, type:

```rust
fn constrained_chain(fm: &Mesh, edge: usize) -> Option<Vec<usize>> {
    let prefix = format!("brep_edge/{edge}/");
    let mut uses =
        std::collections::BTreeMap::<usize, std::collections::BTreeMap<usize, usize>>::new();

    for (&key, vertex) in &fm.vertex {
        for name in vertex.attributes.keys() {
            let Some(suffix) = name.strip_prefix(&prefix) else {
                continue;
            };
            let Some((use_id, sample)) = suffix.split_once('/') else {
                continue;
            };
            let (Ok(use_id), Ok(sample)) = (use_id.parse::<usize>(), sample.parse::<usize>())
            else {
                continue;
            };
            let value = uses.entry(use_id).or_default().entry(sample).or_insert(key);
            *value = (*value).min(key);
        }
    }

    for (&use_id, samples) in &uses {
        if samples.len() < 2 {
            continue;
        }

        let mut keys = Vec::with_capacity(samples.len());

        for (expected, (&sample, &key)) in samples.iter().enumerate() {
            if expected != sample {
                return None;
            }

            keys.push(key);
        }

        let interval_prefix = format!("brep_edge_interval/{edge}/{use_id}/");
        let mut ordered = Vec::with_capacity(keys.len());

        for (sample, key) in keys.into_iter().enumerate() {
            ordered.push((sample as f64, key));
        }

        for (&key, vertex) in &fm.vertex {
            for name in vertex.attributes.keys() {
                if let Some(sample) = name
                    .strip_prefix(&interval_prefix)
                    .and_then(parse_sample_index)
                    && let Some(&t) = vertex.attributes.get(name)
                    && sample + 1 < samples.len()
                    && t.is_finite()
                    && t > 0.0
                    && t < 1.0
                {
                    ordered.push((sample as f64 + t, key));
                }
            }
        }

        ordered.sort_unstable_by(total_parameter_order);
        ordered.dedup_by(same_parameter);
        let mut keys = Vec::with_capacity(ordered.len());

        for (_, key) in ordered {
            keys.push(key);
        }

        return Some(keys);
    }

    None
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/55zc-follow-original-constrained-trim-labels-1.rs"
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

Constrained trim labels can resolve their original boundaries before coherent BRep drawing is connected.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for follow original constrained trim labels.](../screenshots/journey/55za-read-original-edge-use-samples.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [9.1.9 · Read original edge-use samples](55za-read-original-edge-use-samples.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original edge use → face-mesh samples → ordered CAD boundary chain.



Why reject a missing integer sample?

The labels describe one continuous ordered boundary. A gap would join vertices across an unrepresented segment.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace duplicate vertex candidates, a missing sample and an interior interval point through the original lookup.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 9.1.11
npm --prefix ../session_tests run course -- save 9.1.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original boundary helpers prepare coherent CAD edge drawing from the existing face mesh.



[Full validation scope](release.md).

</details>
