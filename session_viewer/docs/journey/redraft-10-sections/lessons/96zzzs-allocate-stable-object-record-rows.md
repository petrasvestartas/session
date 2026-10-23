# 20.5 · Original picks, ambient metadata and cap consumers

**Goal:** Connect both cap crossing passes to current records and retain original owners. · **Steps:** 16 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`src/engine/gpu/arena.rs,src/shaders/pulled_scene.wgsl,src/engine/gpu/indirect.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 20.5 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/instance_gpu.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/global_outline_tests.…</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/object_rows.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">16 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/arena.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/pulled_scene.…</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Allocate stable object re…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Shared Storage Buffer Arena</text>
  <rect x="596" y="52" width="208" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices</text>
  <rect x="596" y="100" width="208" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Free-List Arena Slots</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original picks, ambient met…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Pulled Draws &amp; Compaction</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@builtin(vertex_index)</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Compacted Draw Ranges</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Pick caps through shared…</text>
</svg>
</div>

## <a id="checkpoint-96zzzs-allocate-stable-object-record-rows"></a>1. Allocate stable object record rows

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original IDs plus previou…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained rows and reused…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained rows and reused…</text>
</svg>
</div>

**What this code does:** Prepare bounded row reuse without moving surviving originals.

### 1. `src/instance_gpu.rs`

Retain surviving original IDs, reuse the lowest free row and enforce the shared device bound. Supply original-ID reorder, retired-hole reuse, bounded growth, overflow and empty-release checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.written_bytes,
        ]
    }
```

Replace it with:

```rust
            self.written_bytes,
        ]
    }
}

#[cfg(test)]
mod stable_row_tests {
    use super::*;
    #[test]
    fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
        let mut scene = crate::scene::Scene::empty();
        let mesh = || {
            crate::prepared::PreparedMesh::triangle(
                [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                [0.1, 0.4, 0.9],
            )
            .unwrap()
        };
        let a = scene.insert(mesh()).unwrap();
        let b = scene.insert(mesh()).unwrap();
        let c = scene.insert(mesh()).unwrap();
        let old =
            std::collections::HashMap::from([(a, (1, 0)), (b, (2, 1)), (c, (3, 2))]);
        let rows = stable_rows(&old, &[c, a, b], 3).unwrap();
        assert_eq!((rows[&a], rows[&b], rows[&c]), (0, 1, 2));
        let d = scene.insert(mesh()).unwrap();
        let rows = stable_rows(&old, &[c, d, a], 3).unwrap();
        assert_eq!((rows[&a], rows[&d], rows[&c]), (0, 1, 2));
        assert!(!rows.contains_key(&b));
        assert!(stable_rows(&old, &[a, b, c, d], 3).is_err());
        assert!(stable_rows(&old, &[c], 2).is_err());
        assert!(stable_rows(&old, &[], 0).unwrap().is_empty());
    }
```

**Locate this block:**

```rust
}
pub(crate) fn source() -> String {
    let source = include_str!("triangle.wgsl")
        .replace(
```

Replace it with:

```rust
}
pub(crate) fn stable_rows(
    previous: &std::collections::HashMap<crate::scene::ObjectId, (u32, u32)>,
    ids: &[crate::scene::ObjectId],
    limit: usize,
) -> Result<std::collections::HashMap<crate::scene::ObjectId, u32>, &'static str> {
    let mut rows = std::collections::HashMap::new();
    let mut used = std::collections::HashSet::new();
    for id in ids {
        if let Some(&(_, row)) = previous.get(id) {
            if row as usize >= limit || !used.insert(row) {
                return Err("Invalid retained object row");
            }
            rows.insert(*id, row);
        }
    }
    let mut next = 0usize;
    for id in ids {
        if rows.contains_key(id) {
            continue;
        }
        while next < limit && used.contains(&(next as u32)) {
            next += 1;
        }
        if next >= limit || u32::try_from(next).is_err() {
            return Err("Object records exceed device limits");
        }
        let row = next as u32;
        rows.insert(*id, row);
        used.insert(row);
    }
    Ok(rows)
}
pub(crate) fn source() -> String {
    let source = include_str!("triangle.wgsl")
        .replace(
```

## <a id="checkpoint-96zzzt-retain-live-surface-record-rows"></a>2. Retain live surface record rows

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/gl…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Live original IDs</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stable record indices</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing draw slots</text>
</svg>
</div>

**What this code does:** Connect stable row reuse to the existing surface record table.

### 1. `src/instance_gpu.rs`

Reserve surviving rows before regrouping instance geometry and reuse retired holes. Point each instance slot at its stable original record row. Write the packed instance record in place, leaving retired holes zero. Use the same retained-row lookup for ordinary and individually blended surfaces.

**Locate this existing block:**

```rust
        }
        let mut batches = BTreeMap::new();
        let mut ordinary = std::collections::HashMap::new();
        let mut all_slots = Vec::new();
        let mut all_records = Vec::with_capacity(count);
        for (source, (geometry, records)) in groups {
            let first_slot = u32::try_from(all_slots.len() + 1)
                .map_err(|_| "Instance slot range")?;
            for (id, record) in &records {
                let row = u32::try_from(all_records.len())
                    .map_err(|_| "Instance record row")?;
                all_slots.push([
                    row,
                    u32::from_ne_bytes(
                        record[crate::object_record::RANGE + 4
                            ..crate::object_record::RANGE + 8]
                            .try_into()
                            .unwrap(),
                    ),
                ]);
                ordinary.insert(
                    *id,
                    (
                        u32::try_from(all_slots.len())
                            .map_err(|_| "Instance slot range")?,
                        row,
                    ),
                );
                all_records.push(*record);
            }
            batches.insert(
                source,
                Batch {
                    geometry,
                    count: records.len() as u32,
                    first_slot,
                },
            );
        }
        let own = ordinary_rows.into_iter().collect::<Vec<_>>();
        if own.len() <= limit - count {
            for mesh in own {
                let first = u32::try_from(all_slots.len() + 1)
                    .map_err(|_| "Ordinary slot range")?;
                let row = u32::try_from(all_records.len())
                    .map_err(|_| "Ordinary record row")?;
                all_slots.push([row, mesh.primitive_base()]);
                all_records.push(mesh.packed());
                ordinary.insert(mesh.id, (first, row));
            }
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let own = ordinary_rows.into_iter().collect::<Vec<_>>();
        let own = if own.len() <= limit - count {
            own
        } else {
            Vec::new()
        };
        let ids = groups
            .values()
            .flat_map(|(_, rows)| rows.iter().map(|(id, _)| *id))
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
        let mut batches = BTreeMap::new();
        let mut ordinary = std::collections::HashMap::new();
        let mut all_slots = Vec::new();
        let mut all_records = vec![
            [0; BYTES];
            indices
                .values()
                .max()
                .map_or(0, |row| *row as usize + 1)
        ];
        for (source, (geometry, records)) in groups {
            let first_slot = u32::try_from(all_slots.len() + 1)
                .map_err(|_| "Instance slot range")?;
            for (id, record) in &records {
                let row = indices[id];
                all_slots.push([
                    row,
                    u32::from_ne_bytes(
                        record[crate::object_record::RANGE + 4
                            ..crate::object_record::RANGE + 8]
                            .try_into()
                            .unwrap(),
                    ),
                ]);
                ordinary.insert(
                    *id,
                    (
                        u32::try_from(all_slots.len())
                            .map_err(|_| "Instance slot range")?,
                        row,
                    ),
                );
                all_records[row as usize] = *record;
            }
            batches.insert(
                source,
                Batch {
                    geometry,
                    count: records.len() as u32,
                    first_slot,
                },
            );
        }
        for mesh in own {
            let first = u32::try_from(all_slots.len() + 1)
                .map_err(|_| "Ordinary slot range")?;
            let row = indices[&mesh.id];
            all_slots.push([row, mesh.primitive_base()]);
            all_records[row as usize] = mesh.packed();
            ordinary.insert(mesh.id, (first, row));
        }
```

### 2. `src/global_outline_tests.rs`

Supply retained original-row checks through removal and insertion, exact mapped records and full release.

**Locate this existing block:**

```rust
    pixels(&gpu);
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pixels(&gpu);
    let retained = gpu.instances.ordinary.clone();
    assert!(scene.remove(b));
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.instances.ordinary[&a].1, retained[&a].1);
    assert_eq!(
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    let fresh = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [0.7, -0.3, -0.4],
                    [0.9, -0.3, -0.4],
                    [0.8, -0.1, -0.4],
                ],
                [0.1, 0.4, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    gpu.set_scene(&scene, None);
    assert_eq!(
        gpu.instances.ordinary[&fresh].1,
        retained[&b].1
    );
    assert_eq!(gpu.instances.ordinary[&a].1, retained[&a].1);
    assert_eq!(
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    for mesh in &gpu.meshes {
        let row = gpu.instances.ordinary[&mesh.id].1;
        let bytes = crate::growing_buffer_tests::read(
            &ctx,
            gpu.instances.record_buffer(),
            (row as u64 + 1) * crate::object_record::BYTES as u64,
        );
        assert_eq!(
            &bytes[row as usize * crate::object_record::BYTES..],
            &mesh.packed()
        );
    }
    pixels(&gpu);
    scene.clear();
```

## <a id="checkpoint-96zzzu-prepare-shared-record-pick-consumers"></a>3. Prepare shared record pick consumers

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/object_rows.rs, src/ins…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Object record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">flat original owner plus…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">integer pick fragment</text>
</svg>
</div>

**What this code does:** Share the storage layout and shader adaptation before connecting first picks.

### 1. `src/object_rows.rs`

Own the record layout and shader adaptation once for every shared consumer.

**Locate this existing block:**

```rust
}
pub struct Rows {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("shared object records"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new(BYTES as u64),
            },
            count: None,
        }],
    })
}
pub fn source(shader: &str) -> String {
    shader
        .replace(
            "var<uniform> object_settings: ObjectSettings;",
            "var<storage, read> objects: array<ObjectSettings>;",
        )
        .replace(
            "return object_settings;",
            "return objects[index];",
        )
}
pub struct Rows {
```

### 2. `src/instance_gpu.rs`

Reuse the exact existing storage binding contract. Adapt the same surface shader through the shared owner.

**Locate this existing block:**

```rust
pub(crate) fn source() -> String {
    let source = include_str!("triangle.wgsl")
        .replace(
            "var<uniform> object_settings: ObjectSettings;",
            "var<storage, read> objects: array<ObjectSettings>;",
        )
        .replace(
            "return object_settings;",
            "return objects[index];",
        );
    crate::clipping_gpu::source(&source, 2, 0)
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        view: &wgpu::BindGroupLayout,
    ) -> Self {
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("shared instance records"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(BYTES as u64),
                    },
                    count: None,
                }],
            });
        let source = source();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn source() -> String {
    crate::clipping_gpu::source(
        &crate::object_rows::source(include_str!("triangle.wgsl")),
        2,
        0,
    )
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        view: &wgpu::BindGroupLayout,
    ) -> Self {
        let layout = crate::object_rows::layout(device);
        let source = source();
```

### 3. `src/triangle_pick.wgsl`

Carry the original owner as a flat unsigned vertex result. Share one pick geometry function between ordinary and slot-selected object records. Use the selected original owner without reading a per-object uniform in the fragment.

Replace the entire file with:

```wgsl
@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32,@location(2) @interpolate(flat) primitive:u32,@location(3) @interpolate(flat) owner:u32}
fn pick_geometry(occurrence:u32,object:ObjectSettings,base:u32)->MeshPickOutput {
 let input=mesh_vertex(occurrence,object.range.x);let placed=geometry(input.position,input.colour,input.normal,object);
 return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,base+occurrence/3u+1u,object.selection.y);
}
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput {let object=object_at(0u);return pick_geometry(occurrence,object,object.range.y);}
@vertex fn pick_record_vertex(@builtin(vertex_index) occurrence:u32,@location(0) row:u32,@location(1) base:u32)->MeshPickOutput {return pick_geometry(occurrence,object_at(row),base);}
struct PickFragment{@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>}
@fragment fn pick_fragment(input:MeshPickOutput)->PickFragment{
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,input.owner,crop.mode==0u),0u),physical_primitive(input.primitive));
}
```

## <a id="checkpoint-96zzzv-pick-originals-through-shared-records"></a>4. Pick originals through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/pi…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current shared record and…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original owner and physic…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing asynchronous rea…</text>
</svg>
</div>

**What this code does:** Connect first surface picks to current shared object records.

### 1. `src/instance_gpu.rs`

Share current-record validation and the existing record/slot draw across color and first picks. Use the requested pipeline with the same object row, slot and geometry.

**Locate this existing block:**

```rust
    ) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(&self.ordinary_pipeline);
        self.records.bind(pass);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> bool {
        self.draw_with(pass, mesh, &self.ordinary_pipeline)
    }
    pub fn draw_with(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        mesh: &GpuMesh,
        pipeline: &wgpu::RenderPipeline,
    ) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(pipeline);
        self.records.bind(pass);
```

### 2. `src/pick_pipeline.rs`

Reuse one pick pipeline descriptor for the uniform and shared-record entries. Adapt the existing pick shader through the shared record owner. Choose the slot-aware entry for shared records. Read the same eight-byte object-row and primitive-base slot.

**Locate this existing block:**

```rust
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("mesh IDs"),
        bind_group_layouts: &[
            Some(view_layout),
            Some(model_layout),
            Some(&crop_clipping_layout(device)),
            Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device)),
        ],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mesh IDs"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(
                concat!(
                    include_str!("triangle.wgsl"),
                    "\n",
                    include_str!("pick_crop.wgsl"),
                    "\n",
                    include_str!("triangle_pick.wgsl")
                ),
                2,
                1,
            )
            .into(),
        ),
    });
    // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("mesh IDs"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("pick_vertex"),
            compilation_options: Default::default(),
            buffers: &[],
        },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> wgpu::RenderPipeline {
    mesh_with(
        device,
        view_layout,
        model_layout,
        crop_layout,
        false,
    )
}
pub fn mesh_with(
    device: &wgpu::Device,
    view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout,
    crop_layout: &wgpu::BindGroupLayout,
    shared: bool,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("mesh IDs"),
        bind_group_layouts: &[
            Some(view_layout),
            Some(model_layout),
            Some(&crop_clipping_layout(device)),
            Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device)),
        ],
        immediate_size: 0,
    });
    let source = concat!(
        include_str!("triangle.wgsl"),
        "
",
        include_str!("pick_crop.wgsl"),
        "
",
        include_str!("triangle_pick.wgsl")
    );
    let source = if shared {
        crate::object_rows::source(source)
    } else {
        source.into()
    };
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mesh IDs"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(&source, 2, 1).into(),
        ),
    });
    let slots = [crate::draw_slots::layout()];
    // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("mesh IDs"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(if shared {
                "pick_record_vertex"
            } else {
                "pick_vertex"
            }),
            compilation_options: Default::default(),
            buffers: if shared {
                &slots
            } else {
                &[]
            },
        },
```

### 3. `src/pick_gpu.rs`

Own the shared pick state beside the existing fallback state. Create the shared pick state from the same descriptor and record layout. Retain the shared pick state for the browser lifetime. Keep isolated picker fixtures on the existing uniform path. Pass the existing shared record owner into first picking. Pair the current table with its shared pick state without changing readback ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            caps,
            &self.crop,
            clipping_buffer,
        )]);
```

Replace it with:

```rust
            caps,
            &self.crop,
            clipping_buffer,
            records.map(|rows| (rows, &self.record_pipeline)),
        )]);
```

**Locate this block:**

```rust
        clipping: &wgpu::BindGroup,
        clipping_buffer: &wgpu::Buffer,
        caps: Option<&mut crate::caps_gpu::Caps>,
    ) {
        assert!(
            self.active.is_none(),
```

Replace it with:

```rust
        clipping: &wgpu::BindGroup,
        clipping_buffer: &wgpu::Buffer,
        caps: Option<&mut crate::caps_gpu::Caps>,
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        assert!(
            self.active.is_none(),
```

**Locate this block:**

```rust
        let buffer = clipping.buffer.clone();
        self.submit_finite(
            device, queue, queued, meshes, ink, faces, text, clouds, sheets, None,
            &group, &buffer, None,
        )
    }
    pub(crate) fn submit_finite(
```

Replace it with:

```rust
        let buffer = clipping.buffer.clone();
        self.submit_finite(
            device, queue, queued, meshes, ink, faces, text, clouds, sheets, None,
            &group, &buffer, None, None,
        )
    }
    pub(crate) fn submit_finite(
```

**Locate this block:**

```rust
        Self {
            legacy_clipping: None,
            pipeline,
            view,
            view_group,
            crop,
```

Replace it with:

```rust
        Self {
            legacy_clipping: None,
            pipeline,
            record_pipeline,
            view,
            view_group,
            crop,
```

**Locate this block:**

```rust
            model_layout,
            &crop_layout,
        );
        let buffer = |size| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("pick view"),
```

Replace it with:

```rust
            model_layout,
            &crop_layout,
        );
        let record_pipeline = crate::pick_pipeline::mesh_with(
            device,
            view_layout,
            &crate::object_rows::layout(device),
            &crop_layout,
            true,
        );
        let buffer = |size| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("pick view"),
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    record_pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

### 4. `src/pick_encode.rs`

Accept shared surface records without altering the other pick passes. Draw through current records or the earlier exact uniform fallback.

**Locate this existing block:**

```rust
    clipping_buffer: &wgpu::Buffer,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    let mesh_crop =
        crate::pick_pipeline::crop_clipping_group(device, crop, clipping_buffer);
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.colour,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, &mesh_crop, &[]);
        for mesh in meshes {
            mesh.draw_pulled(&mut pass);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    clipping_buffer: &wgpu::Buffer,
    records: Option<(
        &crate::instance_gpu::Lane,
        &wgpu::RenderPipeline,
    )>,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    let mesh_crop =
        crate::pick_pipeline::crop_clipping_group(device, crop, clipping_buffer);
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.colour,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, &mesh_crop, &[]);
        for mesh in meshes {
            if !records
                .is_some_and(|(rows, shared)| rows.draw_with(&mut pass, mesh, shared))
            {
                pass.set_pipeline(pipeline);
                mesh.draw_pulled(&mut pass);
            }
        }
```

### 5. `src/renderer.rs`

Use the renderer’s existing record owner for original first picks.

**Locate this existing block:**

```rust
                Some(&mut self.caps.borrow_mut()),
            );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                Some(&mut self.caps.borrow_mut()),
                Some(&self.instances),
            );
```

## <a id="checkpoint-96zzzw-read-source-faces-through-shared-records"></a>5. Read source faces through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/fa…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current object record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source face plus…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing face readback</text>
</svg>
</div>

**What this code does:** Connect original source-face picking to the same current object records.

### 1. `src/instance_gpu.rs`

Reuse current-record validation and slot binding for consumers with different geometry layouts. Bind only the shared object record and slot; let each consumer draw its own geometry.

**Locate this existing block:**

```rust
    ) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(pipeline);
        self.records.bind(pass);
        self.slots.bind(pass, 0, first);
        mesh.geometry.draw_pulled(pass, 1);
        true
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> bool {
        if !self.bind_record(pass, mesh) {
            return false;
        }
        pass.set_pipeline(pipeline);
        mesh.geometry.draw_pulled(pass, 1);
        true
    }
    pub fn bind_record(&self, pass: &mut wgpu::RenderPass<'_>, mesh: &GpuMesh) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        self.records.bind(pass);
        self.slots.bind(pass, 0, first);
        true
```

### 2. `src/face_pick.wgsl`

Carry original ownership into the source-face fragment as an unsigned value. Share one source-face geometry function between ordinary and slot-selected records. Read the original owner selected by the source-face vertex.

**Locate this existing block:**

```wgsl
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object_settings.selection.y, (0x20000000u | input.face) + 1u);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) owner:u32,
}
fn face_geometry(index:u32,object:ObjectSettings)->FaceOutput {
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),vec3<f32>(0.),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
}
@vertex fn face_vertex(@builtin(vertex_index) index:u32)->FaceOutput {return face_geometry(index,object_at(0u));}
@vertex fn face_record_vertex(@builtin(vertex_index) index:u32,@location(0) row:u32)->FaceOutput {return face_geometry(index,object_at(row));}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(input.owner, (0x20000000u | input.face) + 1u);
}
```

### 3. `src/face_pick.rs`

Keep a shared-record source-face state beside the existing fallback. Share the source-face pipeline descriptor across both record resources. Use the shared storage contract or the current uniform contract. Adapt the same source-face shader through the existing record owner. Preserve source-face clipping after shader adaptation. Select the source-face record entry and existing draw-slot layout. Build both source-face states from one descriptor and shader. Accept the existing renderer record owner for original source-face reads. Use current records or the exact uniform fallback, retaining original face metadata and crop bindings.

**Locate this existing block:**

```rust
    pipeline: wgpu::RenderPipeline,
    rows: Vec<Option<Row>>,
}

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 {
        entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: if binding == 3 {
                    wgpu::BufferBindingType::Uniform
                } else {
                    wgpu::BufferBindingType::Storage { read_only: true }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
    }
    entries.push(crate::tile_pipelines::entry(
        4,
        wgpu::ShaderStages::FRAGMENT,
        wgpu::BufferBindingType::Uniform,
    ));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("source face metadata"),
        entries: &entries,
    })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let storage = layout(device);
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("source face IDs"),
            bind_group_layouts: &[
                Some(&view),
                Some(&model),
                Some(&storage),
                Some(&crop),
            ],
            immediate_size: 0,
        });
        let source = concat!(
            include_str!("triangle.wgsl"),
            "\n",
            include_str!("pick_crop.wgsl"),
            "\n",
            include_str!("face_pick.wgsl")
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("source faces"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(source, 2, 4).into(),
            ),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("source face IDs"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("face_vertex"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("face_fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            rows: Vec::new(),
        }
    }

    pub fn set(
        &mut self,
        device: &wgpu::Device,
        meshes: &[GpuMesh],
        bases: &[(crate::scene::ObjectId, u32)],
        clipping: &wgpu::Buffer,
    ) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner);
            let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() {
                self.rows.push(None);
                continue;
            }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry
                .source
                .parts
                .triangles
                .iter()
                .flat_map(|n| n.to_ne_bytes())
                .collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("source triangle faces"),
                contents: &bytes,
                usage: wgpu::BufferUsages::STORAGE,
            });
            let data: Vec<u8> = [base, 0, 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect();
            let settings =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("source face base"),
                    contents: &data,
                    usage: wgpu::BufferUsages::UNIFORM,
                });
            let (vertices, indices) = geometry.buffers();
            let buffers = [
                &vertices, &indices, &faces, &settings, clipping,
            ];
            let entries: Vec<_> = buffers
                .iter()
                .enumerate()
                .map(|(binding, buffer)| wgpu::BindGroupEntry {
                    binding: binding as u32,
                    resource: buffer.as_entire_binding(),
                })
                .collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("source face metadata"),
                layout: &self.pipeline.get_bind_group_layout(2),
                entries: &entries,
            });
            self.rows.push(Some(Row {
                group,
                bytes: faces.size() + settings.size(),
            }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        meshes: &[GpuMesh],
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) {
            if let Some(row) = row {
                mesh.bind_model(pass);
                pass.set_bind_group(2, &row.group, &[]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pipeline: wgpu::RenderPipeline,
    record_pipeline: wgpu::RenderPipeline,
    rows: Vec<Option<Row>>,
}

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 {
        entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: if binding == 3 {
                    wgpu::BufferBindingType::Uniform
                } else {
                    wgpu::BufferBindingType::Storage { read_only: true }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
    }
    entries.push(crate::tile_pipelines::entry(
        4,
        wgpu::ShaderStages::FRAGMENT,
        wgpu::BufferBindingType::Uniform,
    ));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("source face metadata"),
        entries: &entries,
    })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let create = |shared| {
            let storage = layout(device);
            let view = crate::gpu_layout::uniform(device, 80);
            let model = if shared {
                crate::object_rows::layout(device)
            } else {
                crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64)
            };
            let crop = crate::gpu_layout::uniform(device, 80);
            let layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("source face IDs"),
                    bind_group_layouts: &[
                        Some(&view),
                        Some(&model),
                        Some(&storage),
                        Some(&crop),
                    ],
                    immediate_size: 0,
                });
            let source = concat!(
                include_str!("triangle.wgsl"),
                "\n",
                include_str!("pick_crop.wgsl"),
                "\n",
                include_str!("face_pick.wgsl")
            );
            let source = if shared {
                crate::object_rows::source(source)
            } else {
                source.into()
            };
            let slots = [crate::draw_slots::layout()];
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("source faces"),
                source: wgpu::ShaderSource::Wgsl(
                    crate::clipping_gpu::source(&source, 2, 4).into(),
                ),
            });
            let pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("source face IDs"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some(if shared {
                            "face_record_vertex"
                        } else {
                            "face_vertex"
                        }),
                        buffers: if shared {
                            &slots
                        } else {
                            &[]
                        },
                        compilation_options: Default::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("face_fragment"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())],
                    }),
                    primitive: Default::default(),
                    depth_stencil: Some(crate::depth::ink()),
                    multisample: Default::default(),
                    multiview_mask: None,
                    cache: None,
                });
            pipeline
        };
        let pipeline = create(false);
        let record_pipeline = create(true);
        Self {
            pipeline,
            record_pipeline,
            rows: Vec::new(),
        }
    }

    pub fn set(
        &mut self,
        device: &wgpu::Device,
        meshes: &[GpuMesh],
        bases: &[(crate::scene::ObjectId, u32)],
        clipping: &wgpu::Buffer,
    ) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner);
            let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() {
                self.rows.push(None);
                continue;
            }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry
                .source
                .parts
                .triangles
                .iter()
                .flat_map(|n| n.to_ne_bytes())
                .collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("source triangle faces"),
                contents: &bytes,
                usage: wgpu::BufferUsages::STORAGE,
            });
            let data: Vec<u8> = [base, 0, 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect();
            let settings =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("source face base"),
                    contents: &data,
                    usage: wgpu::BufferUsages::UNIFORM,
                });
            let (vertices, indices) = geometry.buffers();
            let buffers = [
                &vertices, &indices, &faces, &settings, clipping,
            ];
            let entries: Vec<_> = buffers
                .iter()
                .enumerate()
                .map(|(binding, buffer)| wgpu::BindGroupEntry {
                    binding: binding as u32,
                    resource: buffer.as_entire_binding(),
                })
                .collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("source face metadata"),
                layout: &self.pipeline.get_bind_group_layout(2),
                entries: &entries,
            });
            self.rows.push(Some(Row {
                group,
                bytes: faces.size() + settings.size(),
            }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        meshes: &[GpuMesh],
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) {
            if let Some(row) = row {
                if records.is_some_and(|records| records.bind_record(pass, mesh)) {
                    pass.set_pipeline(&self.record_pipeline);
                } else {
                    pass.set_pipeline(&self.pipeline);
                    mesh.bind_model(pass);
                }
                pass.set_bind_group(2, &row.group, &[]);
```

### 4. `src/pick_encode.rs`

Share the original object table with both first and source-face pick passes.

**Locate this existing block:**

```rust
        });
        faces.draw(&mut pass, meshes, view_group, crop_group);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        faces.draw(
            &mut pass,
            meshes,
            view_group,
            crop_group,
            records.map(|(rows, _)| rows),
        );
    }
```

## <a id="checkpoint-96zzzx-pack-existing-ambient-radius-in-object-records"></a>6. Pack existing ambient radius in object records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/render…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing placed-source ra…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact float bits in align…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared property vertex…</text>
</svg>
</div>

**What this code does:** Carry the existing source radius in a spare aligned record word.

### 1. `src/gpu_mesh.rs`

Store the existing source radius in the spare aligned word and write only a changed value. Retain derived source metadata when rebuilding the current record. Selection and style updates keep the current radius bits.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.primitive_base,
            self.geometry.index_start,
        );
        let mut writes = [0, 0];
```

Replace it with:

```rust
            self.primitive_base,
            self.geometry.index_start,
        );
        next[RANGE + 8..RANGE + 12]
            .copy_from_slice(&self.previous[RANGE + 8..RANGE + 12]);
        let mut writes = [0, 0];
```

**Locate this block:**

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let next = settings(
            object,
            style,
            self.cap_flags
```

Replace it with:

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let mut next = settings(
            object,
            style,
            self.cap_flags
```

**Locate this block:**

```rust
    }

    pub fn primitive_base(&self) -> u32 {
        self.primitive_base
    }
```

Replace it with:

```rust
    }

    pub fn set_ambient_radius(&mut self, queue: &wgpu::Queue, radius: f32) {
        let bytes = radius.to_ne_bytes();
        if self.previous[RANGE + 8..RANGE + 12] != bytes {
            self.previous[RANGE + 8..RANGE + 12].copy_from_slice(&bytes);
            queue.write_buffer(&self.model, (RANGE + 8) as u64, &bytes);
        }
    }
    pub fn primitive_base(&self) -> u32 {
        self.primitive_base
    }
```

### 2. `src/renderer.rs`

Reuse the existing source-bound calculations before refreshing shared records.

**Locate this existing block:**

```rust
            .set(&self.device, &self.meshes);
        self.instance_rows = crate::instance_gpu::batch_rows(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .set(&self.device, &self.meshes);
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
```

### 3. `src/ambient_sources.wgsl`

Reuse the surface view and object declarations instead of repeating their storage contract. Carry the exact per-object radius into the property fragment. Share one property vertex function and prepare a slot-selected record entry without changing the earlier normal policy. Use the radius carried by the selected vertex entry.

Replace the entire file with:

```wgsl
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,normal:vec3<f32>,object:ObjectSettings,radius:f32)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z,radius);
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex {return ambient_vertex(position,oct32_decode(normal),object_at(0u),ambient_radius.x);}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.normal,object,bitcast<f32>(object.range.z));}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
}
```

### 4. `src/ambient_source_pipeline.rs`

Assemble property shading with the surface record and geometry helpers already written.

**Locate this existing block:**

```rust
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"), 2, 0)
                    .into(),
            ),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(
                    concat!(
                        include_str!("triangle.wgsl"),
                        "
",
                        include_str!("ambient_sources.wgsl")
                    ),
                    2,
                    0,
                )
                .into(),
            ),
```

## <a id="checkpoint-96zzzy-read-ambient-properties-through-shared-records"></a>7. Read ambient properties through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/ambient_source_pipeline…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current object row and ar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source normals a…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing ambient recipe</text>
</svg>
</div>

**What this code does:** Connect source normal and radius drawing to the existing object table.

### 1. `src/ambient_source_pipeline.rs`

Own the shared property state beside its existing fallback. Move the fallback radius layout to the shared descriptor owner. Build both property states from one descriptor. Select the current shared record layout or the earlier uniform layout. Use the existing arena geometry binding for pulled properties. Shared properties read radius from their record; fallback properties retain their uniform. Adapt the same property shader through the existing record owner. Share the shader source across both property states. Select the slot-aware property entry for shared records. Keep the indexed vertex attributes alive while creating either property state. Read draw slots for shared properties and retain the earlier indexed vertex format for fallback. Create both states with the same property outputs and depth policy.

**Locate this existing block:**

```rust
    pub draw: wgpu::RenderPipeline,
    pub radius: wgpu::BindGroupLayout,
}
impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let clip = crate::clipping_gpu::layout(device);
        let radius = crate::gpu_layout::uniform(device, 16);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ambient original source metadata"),
            bind_group_layouts: &[
                Some(&view),
                Some(&model),
                Some(&clip),
                Some(&radius),
            ],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ambient original normals and radii"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(
                    concat!(
                        include_str!("triangle.wgsl"),
                        "
",
                        include_str!("ambient_sources.wgsl")
                    ),
                    2,
                    0,
                )
                .into(),
            ),
        });
        let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ambient original normals and radii"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 20,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::R32Float,
                        blend: None,
                        write_mask: wgpu::ColorWrites::RED,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rg8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::state_for_ambient()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { draw, radius }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub draw: wgpu::RenderPipeline,
    pub record_draw: wgpu::RenderPipeline,
    pub radius: wgpu::BindGroupLayout,
}
impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let radius = crate::gpu_layout::uniform(device, 16);
        let create = |shared| {
            let view = crate::gpu_layout::uniform(device, 80);
            let model = if shared {
                crate::object_rows::layout(device)
            } else {
                crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64)
            };
            let clip = crate::clipping_gpu::layout(device);
            let geometry = crate::gpu_geometry::GpuGeometry::storage_layout(device);
            let layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("ambient original source metadata"),
                    bind_group_layouts: &[
                        Some(&view),
                        Some(&model),
                        Some(&clip),
                        Some(if shared {
                            &geometry
                        } else {
                            &radius
                        }),
                    ],
                    immediate_size: 0,
                });
            let source = concat!(
                include_str!("triangle.wgsl"),
                "
",
                include_str!("ambient_sources.wgsl")
            );
            let source = if shared {
                crate::object_rows::source(source)
            } else {
                source.into()
            };
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ambient original normals and radii"),
                source: wgpu::ShaderSource::Wgsl(
                    crate::clipping_gpu::source(&source, 2, 0).into(),
                ),
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ambient original normals and radii"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(if shared {
                        "vs_record"
                    } else {
                        "vs_main"
                    }),
                    compilation_options: Default::default(),
                    buffers: &[if shared {
                        crate::draw_slots::layout()
                    } else {
                        wgpu::VertexBufferLayout {
                            array_stride: 20,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &attributes,
                        }
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::R32Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::RED,
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rg8Unorm,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::state_for_ambient()),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            });
            draw
        };
        let draw = create(false);
        let record_draw = create(true);
        Self {
            draw,
            record_draw,
            radius,
        }
    }
```

### 2. `src/ambient_source_gpu.rs`

Accept the existing renderer object table. Read current shared properties or the earlier exact uniform fallback.

**Locate this existing block:**

```rust
        meshes: &[crate::gpu_mesh::GpuMesh],
    ) {
        let attachments = [radius, normals].map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original ambient source properties"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipes.draw);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(2, clip, &[]);
        for mesh in meshes {
            if let Some((_, group, _)) = self.rows.get(&mesh.id.value()) {
                mesh.bind_model(&mut pass);
                pass.set_bind_group(3, group, &[]);
                mesh.geometry.draw(&mut pass);
                self.draws += 1;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        meshes: &[crate::gpu_mesh::GpuMesh],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let attachments = [radius, normals].map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original ambient source properties"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipes.draw);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(2, clip, &[]);
        for mesh in meshes {
            if let Some((_, group, _)) = self.rows.get(&mesh.id.value()) {
                if records.is_some_and(|rows| rows.bind_record(&mut pass, mesh)) {
                    pass.set_pipeline(&self.pipes.record_draw);
                    mesh.geometry.draw_pulled(&mut pass, 1);
                } else {
                    pass.set_pipeline(&self.pipes.draw);
                    mesh.bind_model(&mut pass);
                    pass.set_bind_group(3, group, &[]);
                    mesh.geometry.draw(&mut pass);
                }
                self.draws += 1;
```

### 3. `src/ambient_gpu.rs`

Keep the object table available while preparing original source properties. Pass the existing record owner through the unchanged ambient recipe.

**Locate this existing block:**

```rust
        revision: [u64; 2],
    ) {
        if ground[1] <= 0.0 {
            self.clear(device, queue);
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
            || full.contains(&0)
            || !density.is_finite()
            || density <= 0.
        {
            self.error = Some("Invalid ambient view");
            return;
        }
        self.prepare(device, full, density, depth, sections, clip);
        let state = self.state.as_ref().unwrap();
        let Some(uniform) =
            crate::ambient_camera::uniform(matrix, full, state.images.size, ground)
        else {
            self.error = Some("Invalid ambient camera inverse");
            return;
        };
        if self.cached != Some((uniform, revision)) {
            queue.write_buffer(
                &self.frame,
                0,
                &uniform
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect::<Vec<_>>(),
            );
            self.writes += 1;
            let images = &state.images;
            let groups = &state.groups;
            let source = &state.input.2;
            self.sources.encode(
                encoder,
                &images.source_radius.view,
                &images.source_normals.view,
                depth,
                view,
                clipping,
                meshes,
            );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        revision: [u64; 2],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if ground[1] <= 0.0 {
            self.clear(device, queue);
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
            || full.contains(&0)
            || !density.is_finite()
            || density <= 0.
        {
            self.error = Some("Invalid ambient view");
            return;
        }
        self.prepare(device, full, density, depth, sections, clip);
        let state = self.state.as_ref().unwrap();
        let Some(uniform) =
            crate::ambient_camera::uniform(matrix, full, state.images.size, ground)
        else {
            self.error = Some("Invalid ambient camera inverse");
            return;
        };
        if self.cached != Some((uniform, revision)) {
            queue.write_buffer(
                &self.frame,
                0,
                &uniform
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect::<Vec<_>>(),
            );
            self.writes += 1;
            let images = &state.images;
            let groups = &state.groups;
            let source = &state.input.2;
            self.sources.encode(
                encoder,
                &images.source_radius.view,
                &images.source_normals.view,
                depth,
                view,
                clipping,
                meshes,
                records,
            );
```

### 4. `src/renderer.rs`

Use the same current object records for rendering, picking and ambient properties.

**Locate this existing block:**

```rust
                    [self.pyramid_revision, clip_revision],
                );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    [self.pyramid_revision, clip_revision],
                    Some(&self.instances),
                );
```

### 5. `src/ambient_gpu_tests.rs`

Supply an exact shared-versus-uniform ambient comparison while retaining every original cache, source, shading and release assertion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "style toggles retain compiled pipelines"
    );
    assert_eq!(r.stats()[0], uploads);
}
```

Replace it with:

```rust
        "style toggles retain compiled pipelines"
    );
    assert_eq!(r.stats()[0], uploads);
    r.set_ambient(false);
    r.instances.ordinary.clear();
    r.set_ambient(true);
    assert_eq!(
        pixels(&r, &matrix),
        shaded,
        "Exact uniform and shared ambient properties"
    );
    r.set_scene(&scene, None);
    r.set_ambient(false);
    r.set_ambient(true);
    assert_eq!(pixels(&r, &matrix), shaded);
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}
```

**Locate this block:**

```rust
    let matrix = camera.uniform();
    let r = renderer(&scene);
    let plain = pixels(&r, &matrix);
    let uploads = r.stats()[0];
    r.set_ambient(true);
```

Replace it with:

```rust
    let matrix = camera.uniform();
    let mut r = renderer(&scene);
    let plain = pixels(&r, &matrix);
    let uploads = r.stats()[0];
    r.set_ambient(true);
```

## <a id="checkpoint-96zzzz-share-the-object-record-shader-schema"></a>8. Share the object record shader schema

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/object_settings.wgsl, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One aligned object declar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">raster and compute shader…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged GPU bytes</text>
</svg>
</div>

**What this code does:** Keep one aligned object record declaration across drawing, caps and projection.

### 1. `src/object_settings.wgsl`

Write the existing aligned record schema once.

Create the file and type:

```wgsl
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
```

### 2. `src/triangle.wgsl`

Use the record declaration supplied by shared shader assembly.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

### 3. `src/cap_count.wgsl`

Cap crossings use the identical record declaration.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

### 4. `src/finite_project.wgsl`

Projection uses the same byte-compatible model and range fields.

**Locate this existing block:**

```wgsl
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
@group(0) @binding(0) var<uniform> view:View;
```

### 5. `src/object_record.rs`

Expose the shared declaration through the existing record owner.

**Locate this existing block:**

```rust
pub const BYTES: usize = RANGE + 16;
pub type Record = [u8; BYTES];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const BYTES: usize = RANGE + 16;
pub type Record = [u8; BYTES];
pub fn shader() -> &'static str {
    include_str!("object_settings.wgsl")
}
```

### 6. `src/clipping_gpu.rs`

Assemble every clipped raster consumer with the shared record declaration. Keep the shared declaration separate from shader resource bindings.

**Locate this existing block:**

```rust
    format!(
        "{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        crate::colour::selection_shader(),
        crate::draw_slots::sentinel_shader(),
        concat!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    format!(
        "{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        crate::colour::selection_shader(),
        crate::draw_slots::sentinel_shader(),
        crate::object_record::shader(),
        concat!(
```

### 7. `src/finite_project_source.rs`

Compute projection includes the same declaration before its packed geometry helper.

**Locate this existing block:**

```rust
    format!(
        "{}\n{}",
        include_str!("packed_vertex.wgsl"),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    format!(
        "{}\n{}\n{}",
        crate::object_record::shader(),
        include_str!("packed_vertex.wgsl"),
```

### 8. `src/object_record_tests.rs`

Verify that projection uses the same aligned members as both raster states after consolidating the flag vector.

**Locate this existing block:**

```rust
            crate::finite_project_source::source(),
            vec![0, 64, 68, 72, 80, 144, 160],
        ),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            crate::finite_project_source::source(),
            vec![0, 64, 80, 144, 160],
        ),
```

## <a id="checkpoint-96zzzza-prepare-shared-record-triangle-projection"></a>9. Prepare shared record triangle projection

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/fi…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current packed-record mat…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">object row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared projection shade…</text>
</svg>
</div>

**What this code does:** Prepare the compute shader to select the same current object row.

### 1. `src/instance_gpu.rs`

Use one packed-record match guard for raster and compute consumers.

**Locate this existing block:**

```rust
    }
    pub fn bind_record(&self, pass: &mut wgpu::RenderPass<'_>, mesh: &GpuMesh) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        self.records.bind(pass);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn current_row(&self, mesh: &GpuMesh) -> Option<(u32, u32)> {
        let &(first, row) = self.ordinary.get(&mesh.id)?;
        self.records
            .matches(row, &mesh.packed())
            .then_some((first, row))
    }
    pub fn bind_record(&self, pass: &mut wgpu::RenderPass<'_>, mesh: &GpuMesh) -> bool {
        let Some((first, _)) = self.current_row(mesh) else {
            return false;
        };
        self.records.bind(pass);
```

### 2. `src/finite_project.wgsl`

Use the shared object shader adaptation without duplicating the projection algorithm. Read the selected row from the existing unsigned projection interval.

**Locate this existing block:**

```wgsl
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
```

### 3. `src/finite_project_source.rs`

Prepare the storage compute source through the existing record owner. Supply validation of both uniform and storage compute interfaces. Both assembled shader sources must retain the existing compute entry.

**Locate this existing block:**

```rust
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        let source = super::source();
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();
        assert!(
            module
                .entry_points
                .iter()
                .any(|entry| entry.name == "cs_main"
                    && entry.stage == naga::ShaderStage::Compute)
        );
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn records() -> String {
    crate::object_rows::source(&source())
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        for source in [super::source(), super::records()] {
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
            assert!(
                module
                    .entry_points
                    .iter()
                    .any(|entry| entry.name == "cs_main"
                        && entry.stage == naga::ShaderStage::Compute)
            );
        }
    }
```

## <a id="checkpoint-96zzzzb-project-triangles-through-shared-records"></a>10. Project triangles through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tile_pipelines.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current object row and ar…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing triangle project…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged finite visibili…</text>
</svg>
</div>

**What this code does:** Connect finite triangle projection to the current shared object table.

### 1. `src/tile_pipelines.rs`

Own a shared projection binding state beside the earlier fallback. Keep both compute states with the existing projection owner. Build uniform and record projection states from one binding recipe and algorithm. Remove the replaced single-state constructor. Retain the shared projection state with the existing scans and bins.

**Locate this existing block:**

```rust
    pub project_layout: wgpu::BindGroupLayout,
    pub raster_layout: wgpu::BindGroupLayout,
    pub scan_layout: wgpu::BindGroupLayout,
    pub project: wgpu::ComputePipeline,
    pub count: wgpu::RenderPipeline,
    pub fill: wgpu::RenderPipeline,
    pub scans: [wgpu::ComputePipeline; 3],
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, view: &wgpu::BindGroupLayout) -> Self {
        let layout = |label, entries: &[wgpu::BindGroupLayoutEntry]| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries,
            })
        };
        let line = layout(
            "finite view dimensions",
            &[entry(
                0,
                Stage::VERTEX_FRAGMENT | Stage::COMPUTE,
                Type::Uniform,
            )],
        );
        let project_layout = layout(
            "finite original projection",
            &[
                entry(
                    0,
                    Stage::COMPUTE,
                    Type::Storage { read_only: true },
                ),
                entry(
                    1,
                    Stage::COMPUTE,
                    Type::Storage { read_only: true },
                ),
                entry(2, Stage::COMPUTE, Type::Uniform),
                entry(
                    3,
                    Stage::COMPUTE,
                    Type::Storage { read_only: false },
                ),
                entry(4, Stage::COMPUTE, Type::Uniform),
            ],
        );
        let raster_layout = layout(
            "finite triangle tiles",
            &[
                entry(
                    0,
                    Stage::VERTEX,
                    Type::Storage { read_only: true },
                ),
                entry(
                    1,
                    Stage::FRAGMENT,
                    Type::Storage { read_only: false },
                ),
            ],
        );
        let scan_layout = layout(
            "finite prefix scan",
            &[entry(
                0,
                Stage::COMPUTE,
                Type::Storage { read_only: false },
            )],
        );

        let project = compute(
            device,
            &crate::finite_project_source::source(),
            "cs_main",
            &[
                Some(view),
                Some(&line),
                Some(&project_layout),
            ],
        );
        let scans = ["scan_tiles", "scan_blocks", "finish_offsets"].map(|entry| {
            compute(
                device,
                &crate::finite_project_source::scan(),
                entry,
                &[Some(&line), Some(&scan_layout)],
            )
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("finite bins"),
            source: wgpu::ShaderSource::Wgsl(
                crate::finite_project_source::tiles().into(),
            ),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("finite bins"),
            bind_group_layouts: &[Some(&line), Some(&raster_layout)],
            immediate_size: 0,
        });

        let count = raster(device, &shader, &layout, "fs_count");
        let fill = raster(device, &shader, &layout, "fs_fill");
        Self {
            line,
            project_layout,
            raster_layout,
            scan_layout,
            project,
            count,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub project_layout: wgpu::BindGroupLayout,
    pub record_project_layout: wgpu::BindGroupLayout,
    pub raster_layout: wgpu::BindGroupLayout,
    pub scan_layout: wgpu::BindGroupLayout,
    pub project: wgpu::ComputePipeline,
    pub record_project: wgpu::ComputePipeline,
    pub count: wgpu::RenderPipeline,
    pub fill: wgpu::RenderPipeline,
    pub scans: [wgpu::ComputePipeline; 3],
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, view: &wgpu::BindGroupLayout) -> Self {
        let layout = |label, entries: &[wgpu::BindGroupLayoutEntry]| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries,
            })
        };
        let line = layout(
            "finite view dimensions",
            &[entry(
                0,
                Stage::VERTEX_FRAGMENT | Stage::COMPUTE,
                Type::Uniform,
            )],
        );
        let create = |shared| {
            let project_layout = layout(
                "finite original projection",
                &[
                    entry(
                        0,
                        Stage::COMPUTE,
                        Type::Storage { read_only: true },
                    ),
                    entry(
                        1,
                        Stage::COMPUTE,
                        Type::Storage { read_only: true },
                    ),
                    entry(
                        2,
                        Stage::COMPUTE,
                        if shared {
                            Type::Storage { read_only: true }
                        } else {
                            Type::Uniform
                        },
                    ),
                    entry(
                        3,
                        Stage::COMPUTE,
                        Type::Storage { read_only: false },
                    ),
                    entry(4, Stage::COMPUTE, Type::Uniform),
                ],
            );
            let source = if shared {
                crate::finite_project_source::records()
            } else {
                crate::finite_project_source::source()
            };
            let project = compute(
                device,
                &source,
                "cs_main",
                &[
                    Some(view),
                    Some(&line),
                    Some(&project_layout),
                ],
            );
            (project_layout, project)
        };
        let (project_layout, project) = create(false);
        let (record_project_layout, record_project) = create(true);
        let raster_layout = layout(
            "finite triangle tiles",
            &[
                entry(
                    0,
                    Stage::VERTEX,
                    Type::Storage { read_only: true },
                ),
                entry(
                    1,
                    Stage::FRAGMENT,
                    Type::Storage { read_only: false },
                ),
            ],
        );
        let scan_layout = layout(
            "finite prefix scan",
            &[entry(
                0,
                Stage::COMPUTE,
                Type::Storage { read_only: false },
            )],
        );

        let scans = ["scan_tiles", "scan_blocks", "finish_offsets"].map(|entry| {
            compute(
                device,
                &crate::finite_project_source::scan(),
                entry,
                &[Some(&line), Some(&scan_layout)],
            )
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("finite bins"),
            source: wgpu::ShaderSource::Wgsl(
                crate::finite_project_source::tiles().into(),
            ),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("finite bins"),
            bind_group_layouts: &[Some(&line), Some(&raster_layout)],
            immediate_size: 0,
        });

        let count = raster(device, &shader, &layout, "fs_count");
        let fill = raster(device, &shader, &layout, "fs_fill");
        Self {
            line,
            project_layout,
            record_project_layout,
            raster_layout,
            scan_layout,
            project,
            record_project,
            count,
```

### 2. `src/tile_gpu.rs`

Observe source arena offsets and stable row selectors as well as model and buffer identity. Each projection row selects its current resource contract. Preserve isolated original fixtures while connecting the renderer’s record owner. Use current table rows or the exact earlier model buffer. Invalidate moved arena intervals and changed record selectors without reprojecting selection-only changes. Store the current object row in the existing sixteen-byte interval. Bind the selected model resource without a second table. Use the matching uniform or storage layout for the cached projection group. Dispatch the same projection algorithm through the selected resource state.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                label: Some("finite original projection"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipes.project);
            pass.set_bind_group(0, view, &[]);
```

Replace it with:

```rust
                label: Some("finite original projection"),
                timestamp_writes: None,
            });
            pass.set_pipeline(if row.shared {
                &self.pipes.record_project
            } else {
                &self.pipes.project
            });
            pass.set_bind_group(0, view, &[]);
```

**Locate this block:**

```rust
            let bindings = row.group.get_or_insert_with(|| {
                group(
                    device,
                    &self.pipes.project_layout,
                    &[
                        &row.vertices,
                        &row.indices,
```

Replace it with:

```rust
            let bindings = row.group.get_or_insert_with(|| {
                group(
                    device,
                    if row.shared {
                        &self.pipes.record_project_layout
                    } else {
                        &self.pipes.project_layout
                    },
                    &[
                        &row.vertices,
                        &row.indices,
```

**Locate this block:**

```rust
            self.rows.push(Row {
                vertices: row.geometry.buffers().0,
                indices: row.geometry.buffers().1,
                model: row.model_buffer().clone(),
                count,
                triangles,
                group: None,
            });
        }
```

Replace it with:

```rust
            self.rows.push(Row {
                vertices: row.geometry.buffers().0,
                indices: row.geometry.buffers().1,
                model: model(row),
                count,
                triangles,
                shared: current(row).is_some(),
                group: None,
            });
        }
```

**Locate this block:**

```rust
            .filter(|row| row.geometry.index_count > 0)
        {
            let triangles = row.geometry.index_count / 3;
            let bytes = [triangles, row.primitive_base(), 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect::<Vec<_>>();
            let count = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("finite original triangle interval"),
                contents: &bytes,
```

Replace it with:

```rust
            .filter(|row| row.geometry.index_count > 0)
        {
            let triangles = row.geometry.index_count / 3;
            let bytes = [
                triangles,
                row.primitive_base(),
                current(row).unwrap_or(0),
                0,
            ]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
            let count = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("finite original triangle interval"),
                contents: &bytes,
```

**Locate this block:**

```rust
                (
                    vertices,
                    indices,
                    row.model_buffer().clone(),
                    row.packed()[..64].try_into().unwrap(),
                    row.primitive_base(),
                )
            })
            .collect::<Vec<_>>();
```

Replace it with:

```rust
                (
                    vertices,
                    indices,
                    model(row),
                    row.packed()[..64].try_into().unwrap(),
                    row.primitive_base(),
                    row.geometry.index_start,
                    row.geometry.index_count,
                    current(row),
                )
            })
            .collect::<Vec<_>>();
```

**Locate this block:**

```rust
            self.key = None;
            return;
        }
        let witness = meshes
            .iter()
            .filter(|row| row.geometry.index_count > 0)
```

Replace it with:

```rust
            self.key = None;
            return;
        }
        let current = |mesh: &GpuMesh| {
            records
                .and_then(|rows| rows.current_row(mesh))
                .map(|(_, row)| row)
        };
        let model = |mesh: &GpuMesh| {
            if current(mesh).is_some() {
                records.unwrap().record_buffer().clone()
            } else {
                mesh.model_buffer().clone()
            }
        };
        let witness = meshes
            .iter()
            .filter(|row| row.geometry.index_count > 0)
```

**Locate this block:**

```rust
        }
    }
    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh]) {
        let limit = u64::from(
            device
                .limits()
```

Replace it with:

```rust
        }
    }
    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh]) {
        self.set_records(device, meshes, None)
    }
    pub fn set_records(
        &mut self,
        device: &wgpu::Device,
        meshes: &[GpuMesh],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let limit = u64::from(
            device
                .limits()
```

**Locate this block:**

```rust
    model: wgpu::Buffer,
    count: wgpu::Buffer,
    triangles: u32,
    group: Option<wgpu::BindGroup>,
}
pub struct Tiles {
```

Replace it with:

```rust
    model: wgpu::Buffer,
    count: wgpu::Buffer,
    triangles: u32,
    shared: bool,
    group: Option<wgpu::BindGroup>,
}
pub struct Tiles {
```

**Locate this block:**

```rust
    u32,
);
struct Row {
    vertices: wgpu::Buffer,
```

Replace it with:

```rust
    u32,
    u32,
    u32,
    Option<u32>,
);
struct Row {
    vertices: wgpu::Buffer,
```

### 3. `src/renderer.rs`

Refresh projection after the object table, so its row selectors and replacement buffers are current. Use the renderer’s current table for finite projection.

**Locate this existing block:**

```rust
        }
        self.tiles
            .borrow_mut()
            .set(&self.device, &self.meshes);
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                self.meshes
                    .iter()
                    .filter(|row| self.instance_rows.contains(&row.id)),
                self.meshes
                    .iter()
                    .filter(|row| !self.instance_rows.contains(&row.id)),
            )
            .expect("Validated instance records fit device limits");
        self.sheets
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let radii = self
            .ambient_rows
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for mesh in &mut self.meshes {
            mesh.set_ambient_radius(
                &self.queue,
                radii
                    .get(&mesh.id.value())
                    .copied()
                    .unwrap_or(0.),
            );
        }
        self.instance_rows = crate::instance_gpu::batch_rows(
            self.meshes.iter().map(|row| {
                (
                    row.id,
                    std::rc::Rc::as_ptr(&row.geometry) as usize,
                    scene.instances.contains_key(&row.id),
                )
            }),
            crate::object_rows::capacity(&self.device),
        );
        self.instances
            .set(
                &self.device,
                &self.queue,
                self.meshes
                    .iter()
                    .filter(|row| self.instance_rows.contains(&row.id)),
                self.meshes
                    .iter()
                    .filter(|row| !self.instance_rows.contains(&row.id)),
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        self.sheets
```

### 4. `src/global_outline_tests.rs`

Supply exact storage-versus-uniform projected bytes and color comparisons while retaining the original range, owner and release assertions.

**Locate this existing block:**

```rust
    pixels(&gpu);
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pixels(&gpu);
    let projected = gpu.tiles.borrow().projected.clone();
    let expected =
        crate::growing_buffer_tests::read(&ctx, &projected, projected.size());
    let image = pixels(&gpu);
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    assert_eq!(pixels(&gpu), image);
    let uniform = gpu.tiles.borrow().projected.clone();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &uniform, uniform.size()),
        expected
    );
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    assert_eq!(pixels(&gpu), image);
    let restored = gpu.tiles.borrow().projected.clone();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &restored, restored.size()),
        expected
    );
    scene.clear();
```

## <a id="checkpoint-96zzzzc-prepare-shared-record-cap-counting"></a>11. Prepare shared record cap counting

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_count.wgsl, src/cap…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Selected object and plane</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing crossing function</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged cap color and o…</text>
</svg>
</div>

**What this code does:** Separate cap crossing geometry from the object resource and vertex format.

### 1. `src/cap_count.wgsl`

Use the existing object shader adapter for cap crossing entries. Share the closed-solid crossing algorithm across object resource paths. Keep vs_count on its current object and plane path. Keep vs_pick_count on its current object and plane path.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane);output.pos=cap_crop.transform*output.pos;return output;}
```

Replace it with:

```wgsl
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane,object_at(0u));output.pos=cap_crop.transform*output.pos;return output;}
```

**Locate this block:**

```wgsl
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
```

Replace it with:

```wgsl
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32,object:ObjectSettings)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
```

**Locate this block:**

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

Replace it with:

```wgsl
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

### 2. `src/cap_pipeline.rs`

Reuse the existing pipeline descriptor with an explicit vertex-buffer format. Remove the repeated format owner from the shared descriptor. Use the selected format without changing targets, blending or depth policy.

**Locate this existing block:**

```rust
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("original closed-solid sections"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("original closed-solid sections"),
        bind_group_layouts: &groups
            .iter()
            .map(|g| Some(*g))
            .collect::<Vec<_>>(),
        immediate_size: 0,
    });
    let attributes = wgpu::vertex_attr_array![0=>Float32x3];
    let buffers = if count {
        vec![wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }]
    } else {
        Vec::new()
    };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("original closed-solid sections"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(vertex),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> wgpu::RenderPipeline {
    let attributes = wgpu::vertex_attr_array![0=>Float32x3];
    let buffers = if count {
        vec![wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }]
    } else {
        Vec::new()
    };
    pipeline_buffers(
        device, source, groups, vertex, fragment, targets, &buffers, count,
    )
}
pub(crate) fn pipeline_buffers(
    device: &wgpu::Device,
    source: String,
    groups: &[&wgpu::BindGroupLayout],
    vertex: &str,
    fragment: &str,
    targets: &[wgpu::ColorTargetState],
    buffers: &[wgpu::VertexBufferLayout<'_>],
    count: bool,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("original closed-solid sections"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("original closed-solid sections"),
        bind_group_layouts: &groups
            .iter()
            .map(|g| Some(*g))
            .collect::<Vec<_>>(),
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("original closed-solid sections"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(vertex),
            compilation_options: Default::default(),
            buffers,
        },
```

## <a id="checkpoint-96zzzzd-prepare-cap-plane-and-arena-bindings"></a>12. Prepare cap plane and arena bindings

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_geometry.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Explicit plane plus curre…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable cap geometry gro…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable cap geometry gro…</text>
</svg>
</div>

**What this code does:** Prepare explicit plane parameters with current immutable arena bindings.

### 1. `src/cap_geometry.rs`

Keep explicit plane parameters and arena binding replacement in one resource owner.

Create the file and type:

```rust
use std::{collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {
    planes: Vec<wgpu::Buffer>,
    groups: HashMap<(usize, u32), (wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup)>,
}
pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    use wgpu::{BufferBindingType as B, ShaderStages as S};
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("cap plane and arena"),
        entries: &[
            crate::tile_pipelines::entry(0, S::VERTEX, B::Storage { read_only: true }),
            crate::tile_pipelines::entry(1, S::VERTEX, B::Uniform),
            crate::tile_pipelines::entry(2, S::VERTEX, B::Storage { read_only: true }),
        ],
    })
}
impl Geometry {
    pub fn set_planes(
        &mut self,
        device: &wgpu::Device,
        count: u32,
    ) -> Result<(), &'static str> {
        if count as usize > crate::clipping_uniform::MAX_PLANES {
            return Err("Too many cap planes");
        }
        if self.planes.len() == count as usize {
            return Ok(());
        }
        self.clear();
        self.planes = (0..count)
            .map(|plane| {
                let bytes = [plane, 0, 0, 0]
                    .into_iter()
                    .flat_map(u32::to_ne_bytes)
                    .collect::<Vec<_>>();
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("cap plane selector"),
                    contents: &bytes,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
                })
            })
            .collect();
        Ok(())
    }
    pub fn group(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        geometry: &Rc<crate::gpu_geometry::GpuGeometry>,
        plane: u32,
    ) -> Result<wgpu::BindGroup, &'static str> {
        let parameter = self
            .planes
            .get(plane as usize)
            .ok_or("Missing cap plane")?;
        let key = (Rc::as_ptr(geometry) as usize, plane);
        let (vertices, indices) = geometry.buffers();
        if let Some((old_vertices, old_indices, group)) = self.groups.get(&key) {
            if *old_vertices == vertices && *old_indices == indices {
                return Ok(group.clone());
            }
        }
        let group = crate::tile_pipelines::group(
            device,
            layout,
            &[&vertices, parameter, &indices],
        );
        self.groups
            .insert(key, (vertices, indices, group.clone()));
        Ok(group)
    }
    pub fn invalidate(&mut self) {
        self.groups.clear();
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub fn bytes(&self) -> u64 {
        self.planes
            .iter()
            .map(wgpu::Buffer::size)
            .sum()
    }
}
```

### 2. `src/lib.rs`

Declare the resource without connecting it to drawing yet.

**Locate this existing block:**

```rust
pub mod cap_image;

pub mod cap_pipeline;

pub mod cap_pick_pipeline;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cap_image;

pub mod cap_geometry;
mod cap_pipeline;

pub mod cap_pick_pipeline;
```

### 3. `src/cap_geometry.rs`

Supply actual mapped plane values, exact binding reuse/replacement through arena growth, bounded refusal and full release.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn planes_keep_current_arena_bindings_and_release() {
        let renderer = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
        let ctx =
            crate::text_gpu_support::GpuCtx::new(&renderer.device, &renderer.queue);
        let layout = layout(&ctx.device);
        let mut groups = Geometry::default();
        groups.set_planes(&ctx.device, 2).unwrap();
        assert_eq!(groups.bytes(), 32);
        for (plane, buffer) in groups.planes.iter().enumerate() {
            assert_eq!(
                crate::growing_buffer_tests::read(&ctx, buffer, 16),
                [plane as u32, 0, 0, 0]
                    .into_iter()
                    .flat_map(u32::to_ne_bytes)
                    .collect::<Vec<_>>()
            );
        }
        let mut scene = crate::scene::Scene::empty();
        scene
            .insert(
                crate::prepared::PreparedMesh::triangle(
                    [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                    [0.1, 0.4, 0.9],
                )
                .unwrap(),
            )
            .unwrap();
        let arena = Rc::new(std::cell::RefCell::new(
            crate::mesh_arena::Arena::new(&ctx),
        ));
        let geometry = Rc::new(
            crate::gpu_geometry::GpuGeometry::shared(
                &ctx,
                scene.objects()[0].mesh.clone(),
                arena.clone(),
            )
            .unwrap(),
        );
        let old = groups
            .group(&ctx.device, &layout, &geometry, 1)
            .unwrap();
        assert_eq!(
            groups
                .group(&ctx.device, &layout, &geometry, 1)
                .unwrap(),
            old
        );
        assert!(
            groups
                .group(&ctx.device, &layout, &geometry, 2)
                .is_err()
        );
        scene
            .insert(
                crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false))
                    .unwrap(),
            )
            .unwrap();
        arena
            .borrow_mut()
            .upload(&ctx, &scene.objects()[1].mesh)
            .unwrap();
        let fresh = groups
            .group(&ctx.device, &layout, &geometry, 1)
            .unwrap();
        assert_ne!(fresh, old);
        assert_eq!(
            groups
                .group(&ctx.device, &layout, &geometry, 1)
                .unwrap(),
            fresh
        );
        assert!(
            groups
                .set_planes(
                    &ctx.device,
                    crate::clipping_uniform::MAX_PLANES as u32 + 1
                )
                .is_err()
        );
        assert_eq!(groups.bytes(), 32);
        groups.invalidate();
        assert!(groups.groups.is_empty());
        groups.clear();
        assert_eq!(groups.bytes(), 0);
        assert_eq!(groups.planes.capacity(), 0);
        assert_eq!(groups.groups.capacity(), 0);
    }
}
```

## <a id="checkpoint-96zzzze-count-cap-color-through-shared-records"></a>13. Count cap color through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_count.wgsl, src/cap…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current object row plus e…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original crossing count</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged cap color and d…</text>
</svg>
</div>

**What this code does:** Connect cap crossing color to current object records and explicit planes.

### 1. `src/cap_count.wgsl`

Pull the current arena range and keep explicit plane selection independent of the object slot.

**Locate this existing block:**

```wgsl
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
@group(3) @binding(0) var<storage,read> cap_vertices:array<PackedVertex>;
@group(3) @binding(1) var<uniform> cap_plane:vec4<u32>;
@group(3) @binding(2) var<storage,read> cap_indices:array<u32>;
@vertex fn vs_record_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=cap_vertices[cap_indices[occurrence+object.range.x]];return count_corner(packed_position(vertex),cap_plane.x,object);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

### 2. `src/cap_pipeline.rs`

Own the shared crossing state with the existing cap color pipeline. Reuse the crossing descriptor, shader, additive blend and depth policy with current records. Retain the shared count state beside the earlier uniform fallback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
        Self {
            count,
            color,
```

Replace it with:

```rust
        );
        Self {
            count,
            record_count,
            geometry,
            color,
```

**Locate this block:**

```rust
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let count = pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_count.wgsl"), 2, 0),
            &[&view, &model, &clip],
            "vs_count",
            "fs_count",
            &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::R16Float,
                blend: Some(wgpu::BlendState {
                    color: add,
                    alpha: add,
                }),
                write_mask: wgpu::ColorWrites::RED,
            }],
            true,
        );
        let color = pipeline(
```

Replace it with:

```rust
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let target = wgpu::ColorTargetState {
            format: wgpu::TextureFormat::R16Float,
            blend: Some(wgpu::BlendState {
                color: add,
                alpha: add,
            }),
            write_mask: wgpu::ColorWrites::RED,
        };
        let count = pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_count.wgsl"), 2, 0),
            &[&view, &model, &clip],
            "vs_count",
            "fs_count",
            &[target.clone()],
            true,
        );
        let geometry = crate::cap_geometry::layout(device);
        let source = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let record_count = pipeline_buffers(
            device,
            crate::clipping_gpu::source(&source, 2, 0),
            &[
                &view,
                &crate::object_rows::layout(device),
                &clip,
                &geometry,
            ],
            "vs_record_count",
            "fs_count",
            &[target],
            &[crate::draw_slots::layout()],
            true,
        );
        let color = pipeline(
```

**Locate this block:**

```rust
    pub count: wgpu::RenderPipeline,
    pub color: wgpu::RenderPipeline,
    pub counts: wgpu::BindGroupLayout,
    pub frame: wgpu::BindGroupLayout,
```

Replace it with:

```rust
    pub count: wgpu::RenderPipeline,
    pub record_count: wgpu::RenderPipeline,
    pub geometry: wgpu::BindGroupLayout,
    pub color: wgpu::RenderPipeline,
    pub counts: wgpu::BindGroupLayout,
    pub frame: wgpu::BindGroupLayout,
```

### 3. `src/caps_gpu.rs`

Let the existing crossing target own its plane parameters and cached geometry groups. Drop stale geometry references when scene ownership changes. Keep the new resource under the existing count-image release lifetime. Prepare bounded plane values before encoding crossings. Create current immutable arena bindings on the existing device. Use the renderer’s current record owner for cap color crossings. Refresh the crossing target’s cached geometry groups as needed. Reuse groups for current records and retain exact target-plane filtering. Draw one object slot with an explicit plane, or use the unchanged uniform crossing path. Account plane parameters once and release them with the crossing image.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                + self
                    .counts
                    .as_ref()
                    .map_or(0, |c| c.image.bytes),
            self.rows.len() as u64,
```

Replace it with:

```rust
                + self
                    .counts
                    .as_ref()
                    .map_or(0, |c| c.image.bytes + c.geometry.bytes()),
            self.rows.len() as u64,
```

**Locate this block:**

```rust
                pass.set_pipeline(&pipes.count);
                pass.set_bind_group(0, view, &[]);
                pass.set_bind_group(2, clipping, &[]);
                for mesh in meshes.iter().filter(|m| {
                    m.closed()
                        && self
                            .targets
                            .get(&m.id.value())
                            .is_some_and(|mask| mask & (1 << plane) != 0)
                }) {
                    mesh.bind_model(&mut pass);
                    mesh.geometry
                        .draw_range(&mut pass, plane..plane + 1);
                    self.draws += 1;
                }
            }
```

Replace it with:

```rust
                pass.set_pipeline(&pipes.count);
                pass.set_bind_group(0, view, &[]);
                pass.set_bind_group(2, clipping, &[]);
                for (mesh, group) in &rows {
                    if let Some(group) = group.as_ref().filter(|_| {
                        records
                            .is_some_and(|records| records.bind_record(&mut pass, mesh))
                    }) {
                        pass.set_pipeline(&pipes.record_count);
                        pass.set_bind_group(3, group, &[]);
                        pass.draw(0..mesh.geometry.index_count, 0..1);
                    } else {
                        pass.set_pipeline(&pipes.count);
                        mesh.bind_model(&mut pass);
                        mesh.geometry
                            .draw_range(&mut pass, plane..plane + 1);
                    }
                    self.draws += 1;
                }
            }
```

**Locate this block:**

```rust
        clipping: &wgpu::BindGroup,
        meshes: &[GpuMesh],
        planes: u32,
    ) {
        let (Some(pipes), Some(counts)) = (&self.pipes, &self.counts) else {
            return;
        };
        for plane in 0..planes {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("original closed-solid crossings"),
```

Replace it with:

```rust
        clipping: &wgpu::BindGroup,
        meshes: &[GpuMesh],
        planes: u32,
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let (Some(pipes), Some(counts)) = (&self.pipes, &mut self.counts) else {
            return;
        };
        for plane in 0..planes {
            let rows = meshes
                .iter()
                .filter(|m| {
                    m.closed()
                        && self
                            .targets
                            .get(&m.id.value())
                            .is_some_and(|mask| mask & (1 << plane) != 0)
                })
                .map(|mesh| {
                    let group = records
                        .and_then(|records| records.current_row(mesh))
                        .map(|_| {
                            counts
                                .geometry
                                .group(
                                    device,
                                    &pipes.geometry,
                                    &mesh.geometry,
                                    plane,
                                )
                                .expect("Prepared cap plane")
                        });
                    (mesh, group)
                })
                .collect::<Vec<_>>();
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("original closed-solid crossings"),
```

**Locate this block:**

```rust
    }
    pub fn draw(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        color: &wgpu::TextureView,
        primitive: &wgpu::TextureView,
```

Replace it with:

```rust
    }
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        color: &wgpu::TextureView,
        primitive: &wgpu::TextureView,
```

**Locate this block:**

```rust
                    resource: wgpu::BindingResource::TextureView(&image.view),
                }],
            });
            self.counts = Some(Counts { image, group });
        }
        let bytes = [size[0] as f32, size[1] as f32, 0., 0.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
```

Replace it with:

```rust
                    resource: wgpu::BindingResource::TextureView(&image.view),
                }],
            });
            self.counts = Some(Counts {
                image,
                group,
                geometry: Default::default(),
            });
        }
        self.counts
            .as_mut()
            .unwrap()
            .geometry
            .set_planes(device, self.planes)
            .expect("Validated clipping plane count");
        let bytes = [size[0] as f32, size[1] as f32, 0., 0.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
```

**Locate this block:**

```rust
        meshes: &mut [GpuMesh],
        planes: &[crate::clipping_rows::Row],
    ) {
        self.planes = planes.len() as u32;
        self.rows.clear();
        self.targets.clear();
```

Replace it with:

```rust
        meshes: &mut [GpuMesh],
        planes: &[crate::clipping_rows::Row],
    ) {
        if let Some(counts) = self.counts.as_mut() {
            counts.geometry.invalidate();
        }
        self.planes = planes.len() as u32;
        self.rows.clear();
        self.targets.clear();
```

**Locate this block:**

```rust
    group: wgpu::BindGroup,
}
pub struct Caps {
    pub enabled: bool,
```

Replace it with:

```rust
    group: wgpu::BindGroup,
    geometry: crate::cap_geometry::Geometry,
}
pub struct Caps {
    pub enabled: bool,
```

### 4. `src/renderer.rs`

Pass the existing device to cap count resource preparation. Use the same current object records for cap color, ordinary surfaces and property consumers.

**Locate this existing block:**

```rust
        self.caps.borrow_mut().draw(
            &mut encoder,
            view,
            &self.primitive.view,
            &self.depth,
            &self.view_group,
            &clip_group,
            &self.meshes,
            if self.clipping.borrow().enabled {
                self.clipping.borrow().rows.len() as u32
            } else {
                0
            },
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.caps.borrow_mut().draw(
            &self.device,
            &mut encoder,
            view,
            &self.primitive.view,
            &self.depth,
            &self.view_group,
            &clip_group,
            &self.meshes,
            if self.clipping.borrow().enabled {
                self.clipping.borrow().rows.len() as u32
            } else {
                0
            },
            Some(&self.instances),
        );
```

### 5. `src/cap_gpu_tests.rs`

Include the one actual sixteen-byte plane parameter in exact cap ownership accounting. Supply an exact shared/uniform cap comparison and restoration while retaining selected pixels, original closedness cache and full release.

**Locate this existing block:**

```rust
    assert_eq!(r.cap_usage()[2], 1);
    assert_eq!(r.cap_usage()[0], 16 + 320 * 240 * 2);
    let uploads = r.stats()[0];
    r.set_clipping(true, false);
    let hatch = pixels(&r);
    assert_ne!(hatch, solid);
    assert!(
        hatch
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 400,
        "hatch has actual black ink"
    );
    r.set_clipping(true, true);
    r.select_objects(&scene, &[owner]).unwrap();
    let selected = pixels(&r);
    let center = at(&selected, 208, 96);
    assert!(
        center[0] > 240 && center[1] > 240 && center[2] < 20,
        "selected original section: {center:?}"
    );
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        r.cap_usage()[2],
        1,
        "selection retains original closedness cache"
    );
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(r.cap_usage()[2], 1);
    assert_eq!(r.cap_usage()[0], 16 + 16 + 320 * 240 * 2);
    let uploads = r.stats()[0];
    r.set_clipping(true, false);
    let hatch = pixels(&r);
    assert_ne!(hatch, solid);
    assert!(
        hatch
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 400,
        "hatch has actual black ink"
    );
    r.set_clipping(true, true);
    r.select_objects(&scene, &[owner]).unwrap();
    let selected = pixels(&r);
    let center = at(&selected, 208, 96);
    assert!(
        center[0] > 240 && center[1] > 240 && center[2] < 20,
        "selected original section: {center:?}"
    );
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        r.cap_usage()[2],
        1,
        "selection retains original closedness cache"
    );
    let stored = selected.clone();
    r.use_uniform_surface_records();
    assert_eq!(
        pixels(&r),
        stored,
        "Exact uniform and shared cap crossing pixels"
    );
    r.set_scene(&scene, None);
    r.select_objects(&scene, &[owner]).unwrap();
    assert_eq!(pixels(&r), stored);
    assert_eq!(r.stats()[0], uploads);
    scene.clear();
```

### 6. `src/renderer.rs`

Supply test-only access to the uniform fallback without exposing renderer internals in the browser application.

**Locate this existing block:**

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {
        self.instances.ordinary.clear();
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
```

## <a id="checkpoint-96zzzzf-prepare-shared-record-cap-picks"></a>14. Prepare shared record cap picks

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_count.wgsl, src/cap…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing pick resources p…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared vertex entry</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged crossing fragme…</text>
</svg>
</div>

**What this code does:** Prepare both cap pick crossing states without changing original ownership.

### 1. `src/cap_count.wgsl`

Retain the original atomic, crop and frame bindings while adding pulled geometry to the shared pick vertex entry.

**Locate this existing block:**

```wgsl
struct Crop { transform: mat4x4<f32>, mode: u32, _a: u32, _b: u32, _c: u32 }
fn wanted(pixel: u32, mode: u32) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Crop { transform: mat4x4<f32>, mode: u32, _a: u32, _b: u32, _c: u32 }
@group(3) @binding(3) var<storage,read> pick_cap_vertices:array<PackedVertex>;
@group(3) @binding(4) var<uniform> pick_cap_plane:vec4<u32>;
@group(3) @binding(5) var<storage,read> pick_cap_indices:array<u32>;
@vertex fn vs_record_pick_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=pick_cap_vertices[pick_cap_indices[occurrence+object.range.x]];var output=count_corner(packed_position(vertex),pick_cap_plane.x,object);output.pos=cap_crop.transform*output.pos;return output;}
fn wanted(pixel: u32, mode: u32) -> bool {
```

### 2. `src/cap_pick_pipeline.rs`

Own both shared crossing passes beside the original fallback. Extend the pick group without replacing its atomic crossing, crop or frame resources. Reuse the original empty color target for both shared passes. Reuse both fragment algorithms, the crop transform and crossing descriptor with the current object record. Keep the preparatory shared states under the original pick pipeline owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            false,
        );
        Self {
            count,
```

Replace it with:

```rust
            false,
        );
        Self {
            record_count,
            record_nearest,
            record_geometry,
            count,
```

**Locate this block:**

```rust
            &[&view, &model, &clip, &records],
            "vs_pick_count",
            "fs_pick_owner",
            &[target],
            true,
        );
        let ids = crate::cap_pipeline::pipeline(
```

Replace it with:

```rust
            &[&view, &model, &clip, &records],
            "vs_pick_count",
            "fs_pick_owner",
            &[target.clone()],
            true,
        );
        let shared = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let source = crate::clipping_gpu::source(&shared, 2, 0);
        let layouts = [
            &view,
            &crate::object_rows::layout(device),
            &clip,
            &record_geometry,
        ];
        let record_count = crate::cap_pipeline::pipeline_buffers(
            device,
            source.clone(),
            &layouts,
            "vs_record_pick_count",
            "fs_pick_count",
            &[target.clone()],
            &[crate::draw_slots::layout()],
            true,
        );
        let record_nearest = crate::cap_pipeline::pipeline_buffers(
            device,
            source,
            &layouts,
            "vs_record_pick_count",
            "fs_pick_owner",
            &[target],
            &[crate::draw_slots::layout()],
            true,
        );
        let ids = crate::cap_pipeline::pipeline(
```

**Locate this block:**

```rust
                    crate::tile_pipelines::entry(2, S::FRAGMENT, B::Uniform),
                ],
            });
        let owners =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("cap crossings and original owner table"),
```

Replace it with:

```rust
                    crate::tile_pipelines::entry(2, S::FRAGMENT, B::Uniform),
                ],
            });
        let record_geometry =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("shared cap pick crossings"),
                entries: &[
                    crate::tile_pipelines::entry(
                        0,
                        S::FRAGMENT,
                        B::Storage { read_only: false },
                    ),
                    crate::tile_pipelines::entry(1, S::VERTEX, B::Uniform),
                    crate::tile_pipelines::entry(2, S::FRAGMENT, B::Uniform),
                    crate::tile_pipelines::entry(
                        3,
                        S::VERTEX,
                        B::Storage { read_only: true },
                    ),
                    crate::tile_pipelines::entry(4, S::VERTEX, B::Uniform),
                    crate::tile_pipelines::entry(
                        5,
                        S::VERTEX,
                        B::Storage { read_only: true },
                    ),
                ],
            });
        let owners =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("cap crossings and original owner table"),
```

**Locate this block:**

```rust
pub struct Pipelines {
    pub count: wgpu::RenderPipeline,
    pub nearest: wgpu::RenderPipeline,
    pub ids: wgpu::RenderPipeline,
```

Replace it with:

```rust
pub struct Pipelines {
    pub record_count: wgpu::RenderPipeline,
    pub record_nearest: wgpu::RenderPipeline,
    pub record_geometry: wgpu::BindGroupLayout,
    pub count: wgpu::RenderPipeline,
    pub nearest: wgpu::RenderPipeline,
    pub ids: wgpu::RenderPipeline,
```

### 3. `src/current_inventory.json`

Copy the dated current runtime-input inventory after kernel/wood changes. This supplied data refresh preserves the historical inventory and every viewer source hash; it does not replace implementation work.

**Locate this existing block:**

```json
  ]
}
```

Edit the existing block to match this code; keep unchanged lines:

```json
  ]
}
```

## <a id="checkpoint-96zzzzg-own-shared-record-cap-pick-resources"></a>15. Own shared record cap pick resources

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_geometry.rs, src/ca…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Pick preparation</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded plane values and…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing pick lifetime</text>
</svg>
</div>

**What this code does:** Reuse explicit plane and arena bindings under the existing pick lifetime.

### 1. `src/cap_geometry.rs`

Reuse the same plane and arena resource owner for groups with retained pick bindings. Append geometry bindings after existing pick resources; color keeps the original three entries.

**Locate this existing block:**

```rust
    ) -> Result<wgpu::BindGroup, &'static str> {
        let parameter = self
            .planes
            .get(plane as usize)
            .ok_or("Missing cap plane")?;
        let key = (Rc::as_ptr(geometry) as usize, plane);
        let (vertices, indices) = geometry.buffers();
        if let Some((old_vertices, old_indices, group)) = self.groups.get(&key) {
            if *old_vertices == vertices && *old_indices == indices {
                return Ok(group.clone());
            }
        }
        let group = crate::tile_pipelines::group(
            device,
            layout,
            &[&vertices, parameter, &indices],
        );
        self.groups
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<wgpu::BindGroup, &'static str> {
        self.group_with(device, layout, geometry, plane, 0, &[])
    }
    pub fn group_with(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        geometry: &Rc<crate::gpu_geometry::GpuGeometry>,
        plane: u32,
        first: u32,
        extra: &[wgpu::BindGroupEntry],
    ) -> Result<wgpu::BindGroup, &'static str> {
        let parameter = self
            .planes
            .get(plane as usize)
            .ok_or("Missing cap plane")?;
        let key = (Rc::as_ptr(geometry) as usize, plane);
        let (vertices, indices) = geometry.buffers();
        if let Some((old_vertices, old_indices, group)) = self.groups.get(&key) {
            if *old_vertices == vertices && *old_indices == indices {
                return Ok(group.clone());
            }
        }
        let buffers = [&vertices, parameter, &indices];
        let mut entries = extra.to_vec();
        entries.extend(
            buffers
                .into_iter()
                .enumerate()
                .map(|(i, buffer)| wgpu::BindGroupEntry {
                    binding: first + i as u32,
                    resource: buffer.as_entire_binding(),
                }),
        );
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cap plane and current arena"),
            layout,
            entries: &entries,
        });
        self.groups
```

### 2. `src/cap_pick_gpu.rs`

Keep cap-pick plane parameters and geometry groups under the existing pick lifetime. Initialize the resource owner without allocating a new arena. Invalidate immutable pick groups when crossing buffers or crop bindings can change, then prepare bounded plane values. Count plane parameters once; arena and borrowed crop allocations retain their original owners.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let _ = window;
    }
    pub fn bytes(&self) -> u64 {
        self.records.size()
            + self.owners.size()
```

Replace it with:

```rust
        let _ = window;
    }
    pub fn bytes(&self) -> u64 {
        self.geometry.bytes()
            + self.records.size()
            + self.owners.size()
```

**Locate this block:**

```rust
        planes: u32,
        mode: crate::pick_sources::Mode,
    ) -> Result<(), &'static str> {
        let size = [window.draw[2], window.draw[3]];
        let bytes = u64::from(size[0])
            .checked_mul(u64::from(size[1]))
```

Replace it with:

```rust
        planes: u32,
        mode: crate::pick_sources::Mode,
    ) -> Result<(), &'static str> {
        self.geometry.invalidate();
        self.geometry.set_planes(device, planes)?;
        self.crop = Some(crop.clone());
        let size = [window.draw[2], window.draw[3]];
        let bytes = u64::from(size[0])
            .checked_mul(u64::from(size[1]))
```

**Locate this block:**

```rust
        );
        let frame_group = crate::gpu_layout::group(device, &pipes.frame, &frame);
        Self {
            pipes,
            records,
            owners,
```

Replace it with:

```rust
        );
        let frame_group = crate::gpu_layout::group(device, &pipes.frame, &frame);
        Self {
            geometry: Default::default(),
            crop: None,
            pipes,
            records,
            owners,
```

**Locate this block:**

```rust
pub struct Pick {
    pipes: crate::cap_pick_pipeline::Pipelines,
    records: wgpu::Buffer,
    owners: wgpu::Buffer,
```

Replace it with:

```rust
pub struct Pick {
    geometry: crate::cap_geometry::Geometry,
    crop: Option<wgpu::Buffer>,
    pipes: crate::cap_pick_pipeline::Pipelines,
    records: wgpu::Buffer,
    owners: wgpu::Buffer,
```

### 3. `src/current_inventory.json`

Copy the supplied checkpoint inventory after regeneration of Session protocol documentation comments. This records current inputs without changing the strict inventory check or historical evidence.

**Locate this existing block:**

```json
  ]
}
```

Edit the existing block to match this code; keep unchanged lines:

```json
  ]
}
```

## <a id="checkpoint-96zzzzh-pick-caps-through-shared-records"></a>16. Pick caps through shared records

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 16</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_pick_gpu.rs, src/ca…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current record and explic…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">atomic crossings</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original integer cap owner</text>
</svg>
</div>

**What this code does:** Connect both cap crossing passes to current records and retain original owners.

### 1. `src/cap_pick_gpu.rs`

Prepare immutable current arena groups before encoding either crossing pass. Use the existing renderer record owner without introducing another table. Prepare exact target-plane groups once and reuse them across count and nearest-owner passes. Count and resolve owners from current records, retaining the original uniform path for changed packed records.

**Locate this existing block:**

```rust
    pub fn encode(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        image: &crate::pick_image::Image,
        window: Window,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
        meshes: &[crate::gpu_mesh::GpuMesh],
        targets: &std::collections::HashMap<u32, u32>,
        planes: u32,
    ) {
        let Some(target) = &self.target else { return };
        encoder.clear_buffer(&self.records, 0, None);
        for pipeline in [&self.pipes.count, &self.pipes.nearest] {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("original section count and nearest owner"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, view, &[]);
            pass.set_bind_group(2, clipping, &[]);
            pass.set_bind_group(3, self.record_group.as_ref().unwrap(), &[]);
            for plane in 0..planes {
                for mesh in meshes.iter().filter(|m| {
                    m.closed()
                        && targets
                            .get(&m.id.value())
                            .is_some_and(|mask| mask & (1 << plane) != 0)
                }) {
                    mesh.bind_model(&mut pass);
                    mesh.geometry
                        .draw_range(&mut pass, plane..plane + 1);
                }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn encode(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image: &crate::pick_image::Image,
        window: Window,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
        meshes: &[crate::gpu_mesh::GpuMesh],
        targets: &std::collections::HashMap<u32, u32>,
        planes: u32,
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let Some(target) = &self.target else { return };
        encoder.clear_buffer(&self.records, 0, None);
        let extra = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: self.records.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: self
                    .crop
                    .as_ref()
                    .unwrap()
                    .as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: self.frame.as_entire_binding(),
            },
        ];
        let rows = (0..planes)
            .map(|plane| {
                meshes
                    .iter()
                    .filter(|m| {
                        m.closed()
                            && targets
                                .get(&m.id.value())
                                .is_some_and(|mask| mask & (1 << plane) != 0)
                    })
                    .map(|mesh| {
                        let group = records
                            .and_then(|records| records.current_row(mesh))
                            .map(|_| {
                                self.geometry
                                    .group_with(
                                        device,
                                        &self.pipes.record_geometry,
                                        &mesh.geometry,
                                        plane,
                                        3,
                                        &extra,
                                    )
                                    .expect("Prepared cap pick plane")
                            });
                        (mesh, group)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for (pipeline, shared) in [
            (&self.pipes.count, &self.pipes.record_count),
            (
                &self.pipes.nearest,
                &self.pipes.record_nearest,
            ),
        ] {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("original section count and nearest owner"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, view, &[]);
            pass.set_bind_group(2, clipping, &[]);
            pass.set_bind_group(3, self.record_group.as_ref().unwrap(), &[]);
            for (plane, rows) in rows.iter().enumerate() {
                for (mesh, group) in rows {
                    if let Some(group) = group.as_ref().filter(|_| {
                        records
                            .is_some_and(|records| records.bind_record(&mut pass, mesh))
                    }) {
                        pass.set_pipeline(shared);
                        pass.set_bind_group(3, group, &[]);
                        pass.draw(0..mesh.geometry.index_count, 0..1);
                    } else {
                        pass.set_pipeline(pipeline);
                        pass.set_bind_group(
                            3,
                            self.record_group.as_ref().unwrap(),
                            &[],
                        );
                        mesh.bind_model(&mut pass);
                        mesh.geometry
                            .draw_range(&mut pass, plane as u32..plane as u32 + 1);
                    }
                }
```

### 2. `src/caps_gpu.rs`

Thread the same optional object owner through cap picking. Keep preparation and encoding under the existing bounded cap pick owner.

**Locate this existing block:**

```rust
        mode: crate::pick_sources::Mode,
    ) {
        if !self.enabled || self.planes == 0 || self.rows.is_empty() {
            self.pick = None;
            self.pick_error = None;
            return;
        }
        let pick = self
            .pick
            .get_or_insert_with(|| crate::cap_pick_gpu::Pick::new(device));
        if let Err(error) = pick.prepare(
            device,
            queue,
            window,
            crop,
            &self.rows,
            self.planes,
            mode,
        ) {
            self.pick_error = Some(error);
            self.pick = None;
            return;
        }
        self.pick_error = None;
        pick.encode(
            encoder,
            image,
            window,
            view,
            clipping,
            meshes,
            &self.targets,
            self.planes,
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        mode: crate::pick_sources::Mode,
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if !self.enabled || self.planes == 0 || self.rows.is_empty() {
            self.pick = None;
            self.pick_error = None;
            return;
        }
        let pick = self
            .pick
            .get_or_insert_with(|| crate::cap_pick_gpu::Pick::new(device));
        if let Err(error) = pick.prepare(
            device,
            queue,
            window,
            crop,
            &self.rows,
            self.planes,
            mode,
        ) {
            self.pick_error = Some(error);
            self.pick = None;
            return;
        }
        self.pick_error = None;
        pick.encode(
            device,
            encoder,
            image,
            window,
            view,
            clipping,
            meshes,
            &self.targets,
            self.planes,
            records,
        );
```

### 3. `src/pick_encode.rs`

Share object records with first picks, source faces and cap picks in the existing submission.

**Locate this existing block:**

```rust
            mode,
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            mode,
            records.map(|(rows, _)| rows),
        );
```

### 4. `src/cap_pick_tests.rs`

Supply full-u32 shared/fallback owner comparison and restoration before a color frame, retaining all original geometry assertions.

**Locate this existing block:**

```rust
    assert!(r.cap_usage()[0] > 24);
    pixels(&r);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(r.cap_usage()[0] > 24);
    r.use_uniform_surface_records();
    assert_eq!(
        picked(&mut r, [0.3, 0.2]),
        Some([owner.value(), 0]),
        "Uniform fallback retains the full original owner"
    );
    r.set_scene(&scene, None);
    assert_eq!(
        picked(&mut r, [0.3, 0.2]),
        Some([owner.value(), 0]),
        "Shared cap owner restores before a color frame"
    );
    pixels(&r);
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.5
npm --prefix ../session_tests run course -- save 20.5
```

Continue with [20.6 · Retained allocation, live lookup and safe reclamation](../../consolidation/96zzzzi-adapt-incremental-slots-to-pulled-draws.md).
