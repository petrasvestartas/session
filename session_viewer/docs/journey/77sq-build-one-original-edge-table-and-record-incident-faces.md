# 15.1.18 · Build one original edge table and record incident faces

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Each low-vertex slot owns a linked list of edge indices.

## Type

Continue from [Compute polygon normals from original boundaries](77sp-compute-polygon-normals-from-original-boundaries.md). [Save or recover your work](recovery.md).

### 1. `src/source_mesh_topology.rs`

Build one original edge table and record incident faces.

<details>
<summary>Locate the existing block</summary>

```rust
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}
```

</details>

After that block, type:

```rust

struct EdgeTable {
    edges: Vec<(usize, usize, u32)>,
    edge_faces: Vec<[u32; 2]>,
    dir0: Vec<u8>,
    opposed: Vec<bool>,
    head: Vec<u32>,
    next: Vec<u32>,
}

impl EdgeTable {
    fn new(slots: usize) -> Self {
        Self { edges: Vec::new(), edge_faces: Vec::new(), dir0: Vec::new(),
            opposed: Vec::new(), head: vec![u32::MAX; slots], next: Vec::new() }
    }

    fn add(&mut self, low_slot: usize, lo: usize, hi: usize, dir: u8, face: usize, cols: &[session_rust::Color]) {
        let ls = low_slot;
        let mut ei = self.head[ls];

        // look for the edge among those at `lo`
        while ei != u32::MAX && self.edges[ei as usize].1 != hi {
            ei = self.next[ei as usize];
        }

        // new edge
        if ei == u32::MAX {
            ei = self.edges.len() as u32;
            let pen = match cols.get(self.edges.len()) {
                Some(color) => pack_rgba(color.to_f32()),
                None => BLACK,
            };
            self.edges.push((lo, hi, pen));
            self.edge_faces.push([u32::MAX; 2]);
            self.dir0.push(dir);
            self.opposed.push(true);
            self.next.push(self.head[ls]);
            self.head[ls] = ei;
        }

        // record this face on the edge, first free slot
        let ef = &mut self.edge_faces[ei as usize];

        if ef[0] == u32::MAX {
            ef[0] = face as u32;
            self.dir0[ei as usize] = dir;
        } else if ef[1] == u32::MAX && ef[0] != face as u32 {
            ef[1] = face as u32;
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sq-build-one-original-edge-table-and-record-incident-faces-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.18
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

![Native renderer output for build one original edge table and record incident faces.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Insert a new unordered edge once, using its first-appearance colour index. Record at most two distinct incident face slots and whether their traversal directions oppose. The sentinel u32::MAX marks an absent list link or incident face. This preserves original edge ordering for display and picking.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



How do adjacent faces reuse the same original edge?

Search the linked edge list at the lower vertex slot for the same upper key.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Walk two adjacent faces across the same edge in opposite directions. Predict one stored edge, two incident faces and its opposed flag.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.18
npm --prefix ../session_tests run course -- save 15.1.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
