# 20.6 · Retained allocation, live lookup and safe reclamation

**Goal:** Release empty source records independently of the current arena extent. · **Steps:** 16 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`src/engine/gpu/arena.rs,src/shaders/pulled_scene.wgsl,src/engine/gpu/indirect.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 20.6 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/slot_space.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/instance_members.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Adapt incremental slots t…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Shared Storage Buffer Arena</text>
  <rect x="596" y="52" width="208" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices</text>
  <rect x="596" y="100" width="208" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Free-List Arena Slots</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Retained allocation, live l…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Pulled Draws &amp; Compaction</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@builtin(vertex_index)</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Compacted Draw Ranges</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Retire empty arena ranges…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzi-adapt-incremental-slots-to-pulled-draws"></a>1. Adapt incremental slots to pulled draws

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retained allocator run</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed members and local…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">correct physical IDs</text>
</svg>
</div>

**What this code does:** Keep physical triangle bases correct when pulled draws begin at local occurrence zero.

### 1. `src/slot_space.rs`

Adapt physical bases to local pulled occurrences and report only changed member positions to the existing allocator. Supply local/global occurrence equivalence, exact physical IDs, changed-member reporting, reorder and retired-hole checks while retaining all five original allocator fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
    }

    /// Instances number their triangles one after another, past the arena's: each slot's base
```

Replace it with:

```rust
        );
    }

    #[test]
    fn local_pulled_slots_keep_physical_ids_and_detect_reordered_members() {
        let mut space = SlotSpace::default();
        let definitions = [
            definition(1, &[7, 8], 30, 2),
            definition(2, &[9], 60, 1),
        ];
        space.reset(&definitions, 100);
        assert_eq!(space.local_slot(1), [7, 100]);
        assert_eq!(space.local_slot(2), [8, 102]);
        assert_eq!(space.local_slot(3), [9, 104]);
        assert_eq!(
            space.local_slot(2)[1] + 1,
            space.slot(2)[1] + 11
        );
        assert!(space.touched(&definitions).is_empty());
        let changed = [
            definition(1, &[8, 7], 30, 2),
            definition(2, &[9], 60, 1),
        ];
        let touched = space.touched(&changed);
        assert_eq!(touched, [(1, 0), (1, 1)]);
        let change = space.update(&changed, &touched, 100);
        assert_eq!(change.written(), Some(3));
        assert_eq!(space.local_slot(1), [8, 100]);
        assert_eq!(space.local_slot(2), [7, 102]);
        assert!(space.touched(&changed).is_empty());
        let shrunk = [
            definition(1, &[8], 30, 2),
            definition(2, &[9], 60, 1),
        ];
        let touched = space.touched(&shrunk);
        assert!(touched.is_empty());
        space.update(&shrunk, &touched, 100);
        assert_eq!(space.texel(2)[3], 0);
        assert_eq!(space.local_slot(3), [9, 104]);
    }
    /// Instances number their triangles one after another, past the arena's: each slot's base
```

**Locate this block:**

```rust
    }
    pub fn texel(&self, slot: u32) -> [u32; 4] {
        if slot == 0 {
            return [self.end() - 1, self.arena, 0, 0];
```

Replace it with:

```rust
    }
    pub fn local_slot(&self, slot: u32) -> Slot {
        let texel = self.texel(slot);
        [texel[0], texel[1]]
    }
    pub fn touched(&self, definitions: &[Definition]) -> Vec<(u32, u32)> {
        let mut out = Vec::new();
        for definition in definitions {
            let Some(run) = self
                .runs
                .iter()
                .find(|run| run.key == Some(definition.key))
            else {
                continue;
            };
            for (position, row) in definition.rows.iter().enumerate() {
                if position < run.cap as usize
                    && self.rows.get(run.start as usize + position) != Some(row)
                {
                    out.push((definition.key, position as u32));
                }
            }
        }
        out
    }
    pub fn texel(&self, slot: u32) -> [u32; 4] {
        if slot == 0 {
            return [self.end() - 1, self.arena, 0, 0];
```

## <a id="checkpoint-96zzzzj-prepare-exact-arena-slot-definitions"></a>2. Prepare exact arena slot definitions

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_members.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Shared arena range plus s…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one definition per source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing slot allocator</text>
</svg>
</div>

**What this code does:** Describe allocator members using exact arena ranges and stable object rows.

### 1. `src/instance_members.rs`

Group stable object rows by exact shared arena ranges; reject standalone buffers and mixed arenas instead of narrowing pointers into IDs.

Create the file and type:

```rust
use crate::gpu_geometry::GpuGeometry;
use std::{collections::BTreeMap, ops::Range, rc::Rc};
pub struct Members {
    pub key: u32,
    pub faces: Range<u32>,
    pub rows: Vec<u32>,
}
impl Members {
    pub fn definition(&self) -> crate::slot_space::Definition<'_> {
        crate::slot_space::Definition {
            key: self.key,
            rows: &self.rows,
            faces: self.faces.clone(),
            pipes: 0..0,
            ribbons: 0..0,
        }
    }
}
pub fn collect<'a>(
    sources: impl IntoIterator<Item = (&'a Rc<GpuGeometry>, Vec<u32>)>,
) -> Option<(Vec<Members>, u32)> {
    let mut arena = None;
    let mut count = 0;
    let mut members = BTreeMap::<u32, Members>::new();
    for (geometry, rows) in sources {
        let current = geometry.arena()?;
        if arena.is_some_and(|first| !Rc::ptr_eq(first, current)) {
            return None;
        }
        arena = Some(current);
        count = u32::try_from(current.borrow().rows.indices.len() / 3).ok()?;
        let first = geometry.index_start;
        let end = first.checked_add(geometry.index_count)?;
        if first % 3 != 0 || geometry.index_count % 3 != 0 {
            return None;
        }
        let member = members
            .entry(first)
            .or_insert_with(|| Members {
                key: first,
                faces: first..end,
                rows: Vec::new(),
            });
        if member.faces.end != end {
            return None;
        }
        member.rows.extend(rows);
    }
    Some((members.into_values().collect(), count))
}
```

### 2. `src/lib.rs`

Declare the preparatory definition adapter.

**Locate this existing block:**

```rust
mod instance_scene_tests;

pub mod instance_gpu;

pub mod cad_shape_key;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod instance_scene_tests;

mod instance_gpu;
pub mod instance_members;

pub mod cad_shape_key;
```

### 3. `src/gpu_geometry.rs`

Expose shared arena identity as a borrowed owner without changing range or allocation ownership.

**Locate this existing block:**

```rust
    }
    pub fn allocated_bytes(&self) -> u64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub(crate) fn arena(&self) -> Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {
        match &self.storage {
            Storage::Shared(arena) => Some(arena),
            Storage::Owned { .. } => None,
        }
    }
    pub fn allocated_bytes(&self) -> u64 {
```

### 4. `src/instance_members.rs`

Supply exact range keys, repeated-definition merging, stable rows, original arena count, standalone/mixed-arena refusal and empty checks.

**Locate this existing block:**

```rust
    Some((members.into_values().collect(), count))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Some((members.into_values().collect(), count))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_ranges_merge_members_and_reject_unrelated_storage() {
        let renderer = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
        let ctx =
            crate::text_gpu_support::GpuCtx::new(&renderer.device, &renderer.queue);
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
        let source = scene.objects()[0].mesh.clone();
        let other = Rc::new((*source).clone());
        let arena = Rc::new(std::cell::RefCell::new(
            crate::mesh_arena::Arena::new(&ctx),
        ));
        let a =
            Rc::new(GpuGeometry::shared(&ctx, source.clone(), arena.clone()).unwrap());
        let b = Rc::new(GpuGeometry::shared(&ctx, other, arena).unwrap());
        let (members, count) = collect([
            (&a, vec![7, 8]),
            (&b, vec![9]),
            (&a, vec![10]),
        ])
        .unwrap();
        assert_eq!(count, 2);
        assert_eq!(members.len(), 2);
        assert_eq!((members[0].key, members[1].key), (0, 3));
        assert_eq!(members[0].rows, [7, 8, 10]);
        assert_eq!(members[1].rows, [9]);
        let definition = members[1].definition();
        assert_eq!(definition.faces, 3..6);
        assert_eq!(definition.rows, [9]);
        let owned = Rc::new(GpuGeometry::upload(
            &ctx.device,
            source.clone(),
        ));
        assert!(collect([(&owned, vec![7])]).is_none());
        let different = Rc::new(std::cell::RefCell::new(
            crate::mesh_arena::Arena::new(&ctx),
        ));
        let different = Rc::new(GpuGeometry::shared(&ctx, source, different).unwrap());
        assert!(collect([(&a, vec![7]), (&different, vec![8])]).is_none());
        let (empty, count) = collect(std::iter::empty()).unwrap();
        assert!(empty.is_empty());
        assert_eq!(count, 0);
    }
}
```

## <a id="checkpoint-96zzzzk-retain-live-allocator-member-runs"></a>3. Retain live allocator member runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/in…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current stable object rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact definitions</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded retained runs</text>
</svg>
</div>

**What this code does:** Update retained source runs from the live stable object rows.

### 1. `src/instance_gpu.rs`

Keep the existing allocator under the surface lane without adding a GPU owner. Begin without retained allocation state. Update retained source runs from exact arena definitions and changed member positions; unsupported or oversized plans retain earlier drawing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
        let mut batches = BTreeMap::new();
```

Replace it with:

```rust
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
        let members = crate::instance_members::collect(
            groups
                .values()
                .map(|(geometry, records)| {
                    (
                        geometry,
                        records
                            .iter()
                            .map(|(id, _)| indices[id])
                            .collect(),
                    )
                })
                .chain(
                    own.iter()
                        .map(|mesh| (&mesh.geometry, vec![indices[&mesh.id]])),
                ),
        );
        if ids.is_empty() {
            self.space = None;
        } else if let Some((members, arena)) = members.filter(|(members, arena)| {
            crate::instance_members::bounded(
                members,
                self.space
                    .as_ref()
                    .and_then(|space| space.triangle_count())
                    .unwrap_or(*arena)
                    .max(*arena),
            )
        }) {
            let definitions = members
                .iter()
                .map(crate::instance_members::Members::definition)
                .collect::<Vec<_>>();
            let space = self
                .space
                .get_or_insert_with(Default::default);
            let touched = space.touched(&definitions);
            space.update(&definitions, &touched, arena);
        } else {
            self.space = None;
        }
        let mut batches = BTreeMap::new();
```

**Locate this block:**

```rust
            &layout,
        );
        Self {
            pipeline,
            ordinary_pipeline,
            layout,
```

Replace it with:

```rust
            &layout,
        );
        Self {
            space: None,
            pipeline,
            ordinary_pipeline,
            layout,
```

**Locate this block:**

```rust
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
```

Replace it with:

```rust
pub struct Lane {
    pub(crate) space: Option<crate::slot_space::SlotSpace>,
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
```

### 2. `src/instance_members.rs`

Keep enough integer headroom for retained runs, growth and free intervals before invoking the existing allocator. Supply small, empty, reserved-arena and overflowing-instance budget checks without allocating large GPU buffers.

**Locate this existing block:**

```rust
}
pub fn collect<'a>(
    sources: impl IntoIterator<Item = (&'a Rc<GpuGeometry>, Vec<u32>)>,
) -> Option<(Vec<Members>, u32)> {
    let mut arena = None;
    let mut count = 0;
    let mut members = BTreeMap::<u32, Members>::new();
    for (geometry, rows) in sources {
        let current = geometry.arena()?;
        if arena.is_some_and(|first| !Rc::ptr_eq(first, current)) {
            return None;
        }
        arena = Some(current);
        count = u32::try_from(current.borrow().rows.indices.len() / 3).ok()?;
        let first = geometry.index_start;
        let end = first.checked_add(geometry.index_count)?;
        if first % 3 != 0 || geometry.index_count % 3 != 0 {
            return None;
        }
        let member = members
            .entry(first)
            .or_insert_with(|| Members {
                key: first,
                faces: first..end,
                rows: Vec::new(),
            });
        if member.faces.end != end {
            return None;
        }
        member.rows.extend(rows);
    }
    Some((members.into_values().collect(), count))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub fn bounded(members: &[Members], arena: u32) -> bool {
    let ids = members
        .iter()
        .map(|member| member.rows.len() as u64 * (member.faces.len() as u64 / 3))
        .sum::<u64>();
    u64::from(arena) * 2 + ids * 2 + 8192 * (members.len() as u64 + 1)
        < u64::from(crate::primitive_image::CAP_START)
}
pub fn collect<'a>(
    sources: impl IntoIterator<Item = (&'a Rc<GpuGeometry>, Vec<u32>)>,
) -> Option<(Vec<Members>, u32)> {
    let mut arena = None;
    let mut count = 0;
    let mut members = BTreeMap::<u32, Members>::new();
    for (geometry, rows) in sources {
        let current = geometry.arena()?;
        if arena.is_some_and(|first| !Rc::ptr_eq(first, current)) {
            return None;
        }
        arena = Some(current);
        count = u32::try_from(current.borrow().rows.indices.len() / 3).ok()?;
        let first = geometry.index_start;
        let end = first.checked_add(geometry.index_count)?;
        if first % 3 != 0 || geometry.index_count % 3 != 0 {
            return None;
        }
        let member = members
            .entry(first)
            .or_insert_with(|| Members {
                key: first,
                faces: first..end,
                rows: Vec::new(),
            });
        if member.faces.end != end {
            return None;
        }
        member.rows.extend(rows);
    }
    Some((members.into_values().collect(), count))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plans_leave_bounded_headroom_before_reserved_caps() {
        let small = [Members {
            key: 0,
            faces: 0..3,
            rows: vec![7, 8],
        }];
        assert!(bounded(&small, 1));
        assert!(bounded(&[], 0));
        assert!(!bounded(
            &[],
            crate::primitive_image::CAP_START
        ));
        let large = [Members {
            key: 0,
            faces: 0..u32::MAX,
            rows: vec![7, 8],
        }];
        assert!(!bounded(&large, 1));
    }
    #[test]
```

### 3. `src/global_outline_tests.rs`

Supply actual retained allocator locations alongside the existing exact GPU comparisons. Verify a surviving original retains its allocator slot through removal and insertion. Verify complete allocator release with the existing empty-scene GPU release checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    scene.clear();
    gpu.set_scene(&scene, None);
    assert!(gpu.instances.ordinary.is_empty());
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
```

Replace it with:

```rust
    scene.clear();
    gpu.set_scene(&scene, None);
    assert!(gpu.instances.ordinary.is_empty());
    assert!(gpu.instances.space.is_none());
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
```

**Locate this block:**

```rust
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    for mesh in &gpu.meshes {
        let row = gpu.instances.ordinary[&mesh.id].1;
        let bytes = crate::growing_buffer_tests::read(
```

Replace it with:

```rust
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    assert_eq!(location(&gpu, a), first_slot);
    for mesh in &gpu.meshes {
        let row = gpu.instances.ordinary[&mesh.id].1;
        let bytes = crate::growing_buffer_tests::read(
```

**Locate this block:**

```rust
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    let fresh = scene
        .insert(
            PreparedMesh::triangle(
```

Replace it with:

```rust
        gpu.instances.ordinary[&copy].1,
        retained[&copy].1
    );
    assert_eq!(location(&gpu, a), first_slot);
    let fresh = scene
        .insert(
            PreparedMesh::triangle(
```

**Locate this block:**

```rust
    pixels(&gpu);
    let retained = gpu.instances.ordinary.clone();
    assert!(scene.remove(b));
    gpu.set_scene(&scene, None);
```

Replace it with:

```rust
    pixels(&gpu);
    let location = |gpu: &super::Renderer, id: crate::scene::ObjectId| {
        let space = gpu.instances.space.as_ref().unwrap();
        let row = gpu.instances.ordinary[&id].1;
        space
            .draws()
            .into_iter()
            .flat_map(|draw| draw.slots)
            .find(|slot| space.local_slot(*slot)[0] == row)
            .unwrap()
    };
    let first_slot = location(&gpu, a);
    let retained = gpu.instances.ordinary.clone();
    assert!(scene.remove(b));
    gpu.set_scene(&scene, None);
```

## <a id="checkpoint-96zzzzl-prepare-one-plan-for-draw-slots-and-lookup"></a>4. Prepare one plan for draw slots and lookup

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/draw_slots.rs, src/slot…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One retained plan</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local draw-slot bytes and…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local draw-slot bytes and…</text>
</svg>
</div>

**What this code does:** Publish one retained plan through the existing draw-slot and integer-texture owners.

### 1. `src/draw_slots.rs`

Publish one retained plan through both owners. Merge slot boundary changes with allocator reports and refresh the lookup header after shrinking; reuse the existing range coalescer.

**Locate this existing block:**

```rust
    }
    pub fn clear(&mut self, ctx: &GpuCtx, free: bool) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn publish(
        &mut self,
        ctx: &GpuCtx,
        space: &crate::slot_space::SlotSpace,
        change: &crate::slot_space::Change,
        table: &mut crate::primitive_slot_table::Table,
    ) -> bool {
        let rows = (1..space.end())
            .map(|slot| space.local_slot(slot))
            .collect::<Vec<_>>();
        assert!(
            change
                .slots
                .iter()
                .all(|range| range.start <= range.end && range.end <= space.end())
        );
        let mut ranges = change.slots.clone();
        ranges.extend(
            changed_runs(&self.previous, &rows)
                .into_iter()
                .map(|run| (run.start + 1) as u32..(run.end + 1) as u32),
        );
        if rows.len() != self.previous.len() {
            ranges.push(0..1);
        }
        let change = crate::slot_space::Change {
            all: change.all,
            slots: crate::slot_space::coalesce(ranges, space.end()),
        };
        if rows.is_empty() && self.instances() > 0 {
            self.clear(ctx, true);
        } else {
            self.set(ctx, &rows);
        }
        table.publish(ctx, space, &change)
    }
    pub fn clear(&mut self, ctx: &GpuCtx, free: bool) {
```

### 2. `src/slot_space.rs`

Expose the range coalescer already written so publication shares its bounds and merging instead of duplicating it.

**Locate this existing block:**

```rust
}
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
    let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub(crate) fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
    let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());
```

### 3. `src/primitive_slot_table_tests.rs`

Supply exact mapped writes and actual lookup shader results through high IDs, hole splitting/reuse, unchanged neighbours, shrinking headers, arena following and release.

**Locate this existing block:**

```rust
    let mut space = SlotSpace::default();
    let rows = [7, 0x80000001];
    let other = [u32::MAX - 1];
    let definitions = [
        Definition {
            key: 1,
            rows: &rows,
            faces: 30..36,
            pipes: 0..0,
            ribbons: 0..0,
        },
        Definition {
            key: 2,
            rows: &other,
            faces: 60..63,
            pipes: 0..0,
            ribbons: 0..0,
        },
    ];
    let change = space.reset(&definitions, 100);
    table.publish(&ctx, &space, &change);
    let source = format!(
        "@group(0) @binding(0) var slot_table: texture_2d<u32>;\n@group(0) @binding(1) var<storage,read_write> output: array<vec2<u32>>;\n{}\n@compute @workgroup_size(1) fn run(@builtin(global_invocation_id) p: vec3<u32>) {{ let ids=array<u32,8>(0u,100u,101u,102u,103u,104u,105u,106u);output[p.x]=placed_triangle(ids[p.x],100u); }}",
        include_str!("primitive_slot_lookup.wgsl")
    );
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual integer slot consumer"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline =
        ctx.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &shader,
                entry_point: Some("run"),
                compilation_options: Default::default(),
                cache: None,
            });
    let output = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[0; 64],
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let mapped = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let resolve = |table: &Table| {
        let group = ctx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&table.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: output.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = ctx
            .device
            .create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(8, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &mapped, 0, 64);
        ctx.queue.submit([encoder.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        mapped
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| {
                tx.send(r).unwrap()
            });
        ctx.device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        rx.recv().unwrap().unwrap();
        let bytes = mapped.slice(..).get_mapped_range();
        let result = bytes
            .chunks_exact(8)
            .map(|v| {
                [
                    u32::from_le_bytes(v[..4].try_into().unwrap()),
                    u32::from_le_bytes(v[4..].try_into().unwrap()),
                ]
            })
            .collect::<Vec<_>>();
        drop(bytes);
        mapped.unmap();
        result
    };
    let none = [u32::MAX; 2];
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [10, 7],
            [11, 7],
            [10, 0x80000001],
            [11, 0x80000001],
            [20, u32::MAX - 1],
            none
        ]
    );
    let mut cross = vec![[0, 100, 0, 0]; 1026];
    cross[0] = [1025, 100, 0, 0];
    cross[1024] = [0x80000001, 100, 30, 2];
    cross[1025] = [u32::MAX - 1, 104, 40, 1];
    table.write(&ctx, &cross);
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [30, 0x80000001],
            [31, 0x80000001],
            none,
            none,
            [40, u32::MAX - 1],
            none
        ]
    );
    table.write(
        &ctx,
        &(0..space.end())
            .map(|i| space.texel(i))
            .collect::<Vec<_>>(),
    );
    let remaining = [Definition {
        key: 2,
        rows: &other,
        faces: 60..63,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&remaining, &[], 100);
    table.publish(&ctx, &space, &change);
    let values = resolve(&table);
    assert_eq!(&values[2..6], &[none; 4]);
    assert_eq!(values[6], [20, u32::MAX - 1]);
    let change = space.reset(&[], 0);
    table.publish(&ctx, &space, &change);
    let values = resolve(&table);
    assert_eq!(values[1], [99, u32::MAX]);
    assert_eq!(values[0], none);
    assert!(values[2..].iter().all(|v| *v == none));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut space = SlotSpace::default();
    let mut slots = crate::draw_slots::Slots::new(&ctx);
    let rows = [7, 0x80000001];
    let other = [u32::MAX - 1];
    let definitions = [
        Definition {
            key: 1,
            rows: &rows,
            faces: 30..36,
            pipes: 0..0,
            ribbons: 0..0,
        },
        Definition {
            key: 2,
            rows: &other,
            faces: 60..63,
            pipes: 0..0,
            ribbons: 0..0,
        },
    ];
    let change = space.reset(&definitions, 100);
    slots.publish(&ctx, &space, &change, &mut table);
    let source = format!(
        "@group(0) @binding(0) var slot_table: texture_2d<u32>;\n@group(0) @binding(1) var<storage,read_write> output: array<vec2<u32>>;\n{}\n@compute @workgroup_size(1) fn run(@builtin(global_invocation_id) p: vec3<u32>) {{ let ids=array<u32,8>(0u,100u,101u,102u,103u,104u,105u,106u);output[p.x]=placed_triangle(ids[p.x],100u); }}",
        include_str!("primitive_slot_lookup.wgsl")
    );
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual integer slot consumer"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline =
        ctx.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &shader,
                entry_point: Some("run"),
                compilation_options: Default::default(),
                cache: None,
            });
    let output = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[0; 64],
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let mapped = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let resolve = |table: &Table| {
        let group = ctx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&table.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: output.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = ctx
            .device
            .create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(8, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &mapped, 0, 64);
        ctx.queue.submit([encoder.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        mapped
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| {
                tx.send(r).unwrap()
            });
        ctx.device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        rx.recv().unwrap().unwrap();
        let bytes = mapped.slice(..).get_mapped_range();
        let result = bytes
            .chunks_exact(8)
            .map(|v| {
                [
                    u32::from_le_bytes(v[..4].try_into().unwrap()),
                    u32::from_le_bytes(v[4..].try_into().unwrap()),
                ]
            })
            .collect::<Vec<_>>();
        drop(bytes);
        mapped.unmap();
        result
    };
    let none = [u32::MAX; 2];
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [10, 7],
            [11, 7],
            [10, 0x80000001],
            [11, 0x80000001],
            [20, u32::MAX - 1],
            none
        ]
    );
    let mut cross = vec![[0, 100, 0, 0]; 1026];
    cross[0] = [1025, 100, 0, 0];
    cross[1024] = [0x80000001, 100, 30, 2];
    cross[1025] = [u32::MAX - 1, 104, 40, 1];
    table.write(&ctx, &cross);
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [30, 0x80000001],
            [31, 0x80000001],
            none,
            none,
            [40, u32::MAX - 1],
            none
        ]
    );
    table.write(
        &ctx,
        &(0..space.end())
            .map(|i| space.texel(i))
            .collect::<Vec<_>>(),
    );
    let remaining = [Definition {
        key: 2,
        rows: &other,
        faces: 60..63,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&remaining, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    let values = resolve(&table);
    assert_eq!(&values[2..6], &[none; 4]);
    assert_eq!(values[6], [20, u32::MAX - 1]);
    let joined = [
        Definition {
            key: 2,
            rows: &other,
            faces: 60..63,
            pipes: 0..0,
            ribbons: 0..0,
        },
        Definition {
            key: 3,
            rows: &[11],
            faces: 90..93,
            pipes: 0..0,
            ribbons: 0..0,
        },
    ];
    let change = space.update(&joined, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [30, 11],
            none,
            none,
            none,
            [20, u32::MAX - 1],
            none
        ]
    );
    let shortened = [Definition {
        key: 3,
        rows: &[11],
        faces: 90..93,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&shortened, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(read(&ctx, &table)[0], [1, 100, 0, 0]);
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [30, 11],
            none,
            none,
            none,
            none,
            none
        ]
    );
    let change = space.reset(&[], 0);
    slots.publish(&ctx, &space, &change, &mut table);
    let values = resolve(&table);
    assert_eq!(values[1], [99, u32::MAX]);
    assert_eq!(values[0], none);
    assert!(values[2..].iter().all(|v| *v == none));
}

#[test]
fn actual_local_slots_and_lookup_share_one_plan_through_changes_holes_growth_and_release()
 {
    use crate::{
        draw_slots::{OWN_ROW, Slots},
        slot_space::{Definition, SlotSpace},
    };
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut slots = Slots::new(&ctx);
    let mut table = Table::new(&ctx);
    let mut space = SlotSpace::default();
    let mut a = [7, 0x80000001];
    let b = [u32::MAX - 1];
    fn definitions<'a>(a: &'a [u32], b: &'a [u32]) -> Vec<Definition<'a>> {
        vec![
            Definition {
                key: 1,
                rows: a,
                faces: 30..36,
                pipes: 0..0,
                ribbons: 0..0,
            },
            Definition {
                key: 2,
                rows: b,
                faces: 60..63,
                pipes: 0..0,
                ribbons: 0..0,
            },
        ]
    }
    let change = space.reset(&definitions(&a, &b), 100);
    assert!(slots.publish(&ctx, &space, &change, &mut table));
    let buffer = slots.buffer.buf.clone();
    let texture = table.texture.clone();
    let wanted = [
        [OWN_ROW, 0],
        [7, 100],
        [0x80000001, 102],
        [u32::MAX - 1, 104],
    ]
    .into_iter()
    .flatten()
    .flat_map(u32::to_ne_bytes)
    .collect::<Vec<_>>();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &buffer, 32),
        wanted
    );
    assert_eq!(
        &read(&ctx, &table)[..4],
        &[
            [3, 100, 0, 0],
            [7, 100, 10, 2],
            [0x80000001, 102, 10, 2],
            [u32::MAX - 1, 104, 20, 1]
        ]
    );
    a[0] = 0x80000007;
    let specs = definitions(&a, &b);
    let touched = space.touched(&specs);
    let change = space.update(&specs, &touched, 100);
    assert!(!slots.publish(&ctx, &space, &change, &mut table));
    assert_eq!(slots.buffer.buf, buffer);
    assert_eq!(table.texture, texture);
    assert_eq!(slots.written_bytes, 8);
    let changed = crate::growing_buffer_tests::read(&ctx, &buffer, 32);
    assert_eq!(&changed[..8], &wanted[..8]);
    assert_eq!(&changed[16..], &wanted[16..]);
    assert_eq!(
        u32::from_ne_bytes(changed[8..12].try_into().unwrap()),
        0x80000007
    );
    let change = space.update(&specs, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(slots.written_bytes, 0);
    let remaining = [Definition {
        key: 2,
        rows: &b,
        faces: 60..63,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&remaining, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    let values = read(&ctx, &table);
    assert_eq!(values[1][3], 0);
    assert_eq!(values[2][3], 0);
    assert_eq!(values[3], [u32::MAX - 1, 104, 20, 1]);
    assert_eq!(slots.instances(), 3);
    let joined = [
        Definition {
            key: 2,
            rows: &b,
            faces: 60..63,
            pipes: 0..0,
            ribbons: 0..0,
        },
        Definition {
            key: 3,
            rows: &[11],
            faces: 90..93,
            pipes: 0..0,
            ribbons: 0..0,
        },
    ];
    let change = space.update(&joined, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(space.local_slot(1), [11, 100]);
    let lookup = read(&ctx, &table);
    assert_eq!(lookup[1], [11, 100, 30, 1]);
    assert_eq!(lookup[2], [0x80000001, 101, 0, 0]);
    assert_eq!(slots.written_bytes, 16);
    assert_eq!(slots.buffer.buf, buffer);
    let wanted = [
        [OWN_ROW, 0],
        [11, 100],
        [0x80000001, 101],
        [u32::MAX - 1, 104],
    ]
    .into_iter()
    .flatten()
    .flat_map(u32::to_ne_bytes)
    .collect::<Vec<_>>();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &buffer, 32),
        wanted
    );
    let shortened = [Definition {
        key: 3,
        rows: &[11],
        faces: 90..93,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&shortened, &[], 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(slots.instances(), 1);
    assert_eq!(slots.written_bytes, 0);
    assert_eq!(slots.buffer.buf, buffer);
    assert_eq!(read(&ctx, &table)[0], [1, 100, 0, 0]);
    let change = space.follow(120);
    assert!(change.all);
    slots.publish(&ctx, &space, &change, &mut table);
    let wanted = (0..space.end())
        .map(|i| space.texel(i))
        .collect::<Vec<_>>();
    assert_eq!(&read(&ctx, &table)[..wanted.len()], wanted);
    let wanted = std::iter::once([OWN_ROW, 0])
        .chain((1..space.end()).map(|i| space.local_slot(i)))
        .flatten()
        .flat_map(u32::to_ne_bytes)
        .collect::<Vec<_>>();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, wanted.len() as u64),
        wanted
    );
    let change = space.reset(&[], 0);
    assert!(slots.publish(&ctx, &space, &change, &mut table));
    assert_eq!(slots.instances(), 0);
    assert_eq!(slots.allocated_bytes(), 8);
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
    let sentinel = slots.buffer.buf.clone();
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(slots.buffer.buf, sentinel);
    assert_eq!(slots.written_bytes, 0);
}
```

## <a id="checkpoint-96zzzzm-prepare-one-owner-for-live-address-updates"></a>5. Prepare one owner for live address updates

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Live mesh slice plus memb…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one address owner</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measured allocation parts</text>
</svg>
</div>

**What this code does:** Prepare synchronized mesh and record updates without changing primitive addresses.

### 1. `src/instance_gpu.rs`

Pass the live meshes to their record owner so the next connection can synchronize addresses before preparing consumers. Keep the same instance membership while accepting the complete mesh slice. Retain the existing ordinary-row capacity fallback.

**Locate this existing block:**

```rust
    }
    pub fn set<'a>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rows: impl IntoIterator<Item = &'a GpuMesh>,
        ordinary_rows: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: BTreeMap<
            usize,
            (
                Rc<GpuGeometry>,
                Vec<(crate::scene::ObjectId, Record)>,
            ),
        > = BTreeMap::new();
        for row in rows {
            let key = Rc::as_ptr(&row.geometry) as usize;
            groups
                .entry(key)
                .or_insert_with(|| (row.geometry.clone(), Vec::new()))
                .1
                .push((row.id, row.packed()));
        }
        let count = groups
            .values()
            .try_fold(0usize, |n, (_, rows)| {
                n.checked_add(rows.len())
            })
            .ok_or("Instance record count")?;
        if count > limit {
            return Err("Shared instance records exceed device limits");
        }
        let own = ordinary_rows.into_iter().collect::<Vec<_>>();
        let own = if own.len() <= limit - count {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        meshes: &mut [GpuMesh],
        instances: &std::collections::HashSet<crate::scene::ObjectId>,
        _geometry_changed: bool,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: BTreeMap<
            usize,
            (
                Rc<GpuGeometry>,
                Vec<(crate::scene::ObjectId, Record)>,
            ),
        > = BTreeMap::new();
        for row in meshes
            .iter()
            .filter(|row| instances.contains(&row.id))
        {
            let key = Rc::as_ptr(&row.geometry) as usize;
            groups
                .entry(key)
                .or_insert_with(|| (row.geometry.clone(), Vec::new()))
                .1
                .push((row.id, row.packed()));
        }
        let count = groups
            .values()
            .try_fold(0usize, |n, (_, rows)| {
                n.checked_add(rows.len())
            })
            .ok_or("Instance record count")?;
        if count > limit {
            return Err("Shared instance records exceed device limits");
        }
        let own = meshes
            .iter()
            .filter(|row| !instances.contains(&row.id))
            .collect::<Vec<_>>();
        let own = if own.len() <= limit - count {
```

### 2. `src/renderer.rs`

Distinguish source membership updates from style changes before preparing projection. Keep style-only updates distinguishable so explicit primitive addresses remain intact.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.instances.set(
            &self.device,
            &self.queue,
            self.meshes
                .iter()
                .filter(|row| self.instance_rows.contains(&row.id)),
            self.meshes
                .iter()
                .filter(|row| !self.instance_rows.contains(&row.id)),
        )?;
```

Replace it with:

```rust
        self.instances.set(
            &self.device,
            &self.queue,
            &mut self.meshes,
            &self.instance_rows,
            false,
        )?;
```

**Locate this block:**

```rust
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
```

Replace it with:

```rust
                &self.queue,
                &mut self.meshes,
                &self.instance_rows,
                true,
            )
            .expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(
```

### 3. `src/instance_gpu.rs`

Expose the existing allocation owners individually for the upcoming sparse allocation check. Derive the total from the same measured allocations instead of repeating the accounting.

**Locate this existing block:**

```rust
    }
    pub fn usage(&self) -> [u64; 5] {
        [
            self.batches.len() as u64,
            self.batches
                .values()
                .map(|b| u64::from(b.count))
                .sum(),
            self.records.allocated_bytes()
                + if self.slots.instances() == 0 {
                    0
                } else {
                    self.slots.allocated_bytes()
                },
            self.writes,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn allocation_parts(&self) -> [u64; 2] {
        [
            self.records.allocated_bytes(),
            if self.slots.instances() == 0 {
                0
            } else {
                self.slots.allocated_bytes()
            },
        ]
    }
    pub fn usage(&self) -> [u64; 5] {
        [
            self.batches.len() as u64,
            self.batches
                .values()
                .map(|b| u64::from(b.count))
                .sum(),
            self.allocation_parts().into_iter().sum(),
            self.writes,
```

### 4. `src/renderer.rs`

Make the two existing GPU allocation parts available through renderer diagnostics.

**Locate this existing block:**

```rust
    }
    pub fn stats(&self) -> [usize; 4] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn instance_allocations(&self) -> [u64; 2] {
        self.instances.allocation_parts()
    }
    pub fn stats(&self) -> [usize; 4] {
```

### 5. `src/browser.rs`

Expose measured allocation parts beside the existing browser GPU diagnostic.

**Locate this existing block:**

```rust
    )?;
    let instances=editor.scene.instances.iter().map(|(id,owner)|serde_json::json!({"id":id.value(),"guid":editor.scene.row_guid(*id),"source_guid":owner.instance.guid(),"definition_guid":owner.definition.guid(),"document_guid":owner.document.guid(),"model":editor.scene.placement(*id).map(|m|m.m),"flags":owner.instance.flags,"colour":editor.scene.face_rgba(*id)})).collect::<Vec<_>>();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    )?;
    canvas.set_attribute(
        "data-instance-allocations",
        &serde_json::to_string(&renderer.instance_allocations()).unwrap(),
    )?;
    let instances=editor.scene.instances.iter().map(|(id,owner)|serde_json::json!({"id":id.value(),"guid":editor.scene.row_guid(*id),"source_guid":owner.instance.guid(),"definition_guid":owner.definition.guid(),"document_guid":owner.document.guid(),"model":editor.scene.placement(*id).map(|m|m.m),"flags":owner.instance.flags,"colour":editor.scene.face_rgba(*id)})).collect::<Vec<_>>();
```

### 6. `src/global_outline_tests.rs`

Supply exact allocation parts alongside the unchanged two-definition GPU pixels, slots, selection and release checks.

**Locate this existing block:**

```rust
    assert_eq!(&gpu.instance_usage()[..3], &[2, 2, 376]);
    assert_eq!(gpu.instances.slots.instances(), 2);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(&gpu.instance_usage()[..3], &[2, 2, 376]);
    assert_eq!(gpu.instance_allocations(), [352, 24]);
    assert_eq!(gpu.instances.slots.instances(), 2);
```

## <a id="checkpoint-96zzzzn-adopt-retained-gpu-slots-and-physical-bases"></a>6. Adopt retained GPU slots and physical bases

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/pr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retained runs</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">synchronized records and…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coherent sparse GPU slots…</text>
</svg>
</div>

**What this code does:** Draw retained sparse slots with synchronized original uniforms and shared records.

### 1. `src/instance_gpu.rs`

Retain the inverse lookup under the same live surface owner. Allocate the lookup only when a complete shared plan is active. Use source versus style updates to preserve explicit primitive addresses. Keep style-only explicit address overrides on the earlier path. Activate inverse lookup only for a complete supported shared table. Retain the allocator change report for the GPU writers. Preserve existing fallback behavior when the complete plan cannot be used. Publish the same retained change that assigned physical intervals. Synchronize final mesh uniforms and shared records before any consumer, using sparse locations or unchanged legacy addresses. Use the existing coherent GPU publication, with full resource release on the legacy or empty path.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.writes += 1;
            self.written_bytes += self.records.written_bytes;
        }
        if all_slots.is_empty() && self.slots.instances() > 0 {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
        }
```

Replace it with:

```rust
            self.writes += 1;
            self.written_bytes += self.records.written_bytes;
        }
        if let (Some(space), Some(change)) = (&self.space, &change) {
            let table = self
                .lookup
                .get_or_insert_with(|| crate::primitive_slot_table::Table::new(&ctx));
            self.slots
                .publish(&ctx, space, change, table);
            let bytes = table.written_bytes.get();
            if bytes > 0 {
                self.writes += 1;
                self.written_bytes += bytes;
            }
        } else {
            self.lookup = None;
            if all_slots.is_empty() && self.slots.instances() > 0 {
                self.slots.clear(&ctx, true);
            } else {
                self.slots.set(&ctx, &all_slots);
            }
        }
```

**Locate this block:**

```rust
            all_records[row as usize] = mesh.packed();
            ordinary.insert(mesh.id, (first, row));
        }
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        self.records
            .set(&ctx, &self.layout, &all_records)?;
```

Replace it with:

```rust
            all_records[row as usize] = mesh.packed();
            ordinary.insert(mesh.id, (first, row));
        }
        if let Some(space) = &self.space {
            let locations = space
                .draws()
                .into_iter()
                .flat_map(|draw| draw.slots)
                .map(|slot| {
                    let [row, base] = space.local_slot(slot);
                    (row, (slot, base))
                })
                .collect::<std::collections::HashMap<_, _>>();
            for (first, row) in ordinary.values_mut() {
                *first = locations[&*row].0;
            }
            for batch in batches.values_mut() {
                batch.first_slot =
                    locations[&all_slots[(batch.first_slot - 1) as usize][0]].0;
            }
        }
        let mut dense = 0u32;
        for mesh in meshes {
            let entry = ordinary.get(&mesh.id);
            let base = if let (Some(space), Some((first, _))) = (&self.space, entry) {
                space.local_slot(*first)[1]
            } else if geometry_changed {
                dense
            } else {
                mesh.primitive_base()
            };
            mesh.set_primitive_base(queue, base)?;
            if let Some((first, row)) = entry {
                all_records[*row as usize] = mesh.packed();
                if self.space.is_none() {
                    all_slots[(*first - 1) as usize] = [*row, base];
                }
            }
            dense = dense
                .checked_add(mesh.geometry.index_count / 3)
                .filter(|n| *n < crate::primitive_image::CAP_START)
                .ok_or("Primitive range overlaps reserved caps")?;
        }
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        self.records
            .set(&ctx, &self.layout, &all_records)?;
```

**Locate this block:**

```rust
                .space
                .get_or_insert_with(Default::default);
            let touched = space.touched(&definitions);
            space.update(&definitions, &touched, arena);
        } else {
            self.space = None;
        }
```

Replace it with:

```rust
                .space
                .get_or_insert_with(Default::default);
            let touched = space.touched(&definitions);
            change = Some(space.update(&definitions, &touched, arena));
        } else {
            self.space = None;
        }
```

**Locate this block:**

```rust
        if ids.is_empty() {
            self.space = None;
        } else if let Some((members, arena)) = members.filter(|(members, arena)| {
            crate::instance_members::bounded(
                members,
                self.space
                    .as_ref()
                    .and_then(|space| space.triangle_count())
                    .unwrap_or(*arena)
                    .max(*arena),
            )
        }) {
            let definitions = members
                .iter()
```

Replace it with:

```rust
        if ids.is_empty() {
            self.space = None;
        } else if let Some((members, arena)) = members.filter(|(members, arena)| {
            complete
                && compatible
                && crate::instance_members::bounded(
                    members,
                    self.space
                        .as_ref()
                        .and_then(|space| space.triangle_count())
                        .unwrap_or(*arena)
                        .max(*arena),
                )
        }) {
            let definitions = members
                .iter()
```

**Locate this block:**

```rust
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
        let members = crate::instance_members::collect(
            groups
                .values()
```

Replace it with:

```rust
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
        let mut change = None;
        let members = crate::instance_members::collect(
            groups
                .values()
```

**Locate this block:**

```rust
            .iter()
            .filter(|row| !instances.contains(&row.id))
            .collect::<Vec<_>>();
        let own = if own.len() <= limit - count {
            own
        } else {
            Vec::new()
```

Replace it with:

```rust
            .iter()
            .filter(|row| !instances.contains(&row.id))
            .collect::<Vec<_>>();
        let complete = own.len() <= limit - count;
        let own = if complete {
            own
        } else {
            Vec::new()
```

**Locate this block:**

```rust
        queue: &wgpu::Queue,
        meshes: &mut [GpuMesh],
        instances: &std::collections::HashSet<crate::scene::ObjectId>,
        _geometry_changed: bool,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
            return Err("Device cannot hold one instance record");
```

Replace it with:

```rust
        queue: &wgpu::Queue,
        meshes: &mut [GpuMesh],
        instances: &std::collections::HashSet<crate::scene::ObjectId>,
        geometry_changed: bool,
    ) -> Result<(), &'static str> {
        let compatible = geometry_changed
            || self.space.as_ref().is_some_and(|space| {
                meshes.iter().all(|mesh| {
                    self.ordinary
                        .get(&mesh.id)
                        .is_some_and(|(slot, row)| {
                            *slot > 0
                                && *slot < space.end()
                                && space.local_slot(*slot)
                                    == [*row, mesh.primitive_base()]
                        })
                })
            });
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
            return Err("Device cannot hold one instance record");
```

**Locate this block:**

```rust
            &layout,
        );
        Self {
            space: None,
            pipeline,
            ordinary_pipeline,
```

Replace it with:

```rust
            &layout,
        );
        Self {
            lookup: None,
            space: None,
            pipeline,
            ordinary_pipeline,
```

**Locate this block:**

```rust
pub struct Lane {
    pub(crate) space: Option<crate::slot_space::SlotSpace>,
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
pub struct Lane {
    pub(crate) lookup: Option<crate::primitive_slot_table::Table>,
    pub(crate) space: Option<crate::slot_space::SlotSpace>,
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
```

### 2. `src/primitive_slot_table.rs`

Measure actual lookup upload bytes, including padded full writes. Begin with zero lookup upload bytes. Count only validated texture writes. Measure each publication independently.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        space: &crate::slot_space::SlotSpace,
        change: &crate::slot_space::Change,
    ) -> bool {
        let previous = self.texture.clone();
```

Replace it with:

```rust
        space: &crate::slot_space::SlotSpace,
        change: &crate::slot_space::Change,
    ) -> bool {
        self.written_bytes.set(0);
        let previous = self.texture.clone();
```

**Locate this block:**

```rust
        {
            return false;
        }
        let mut at = first;
        let mut rest = texels;
        while !rest.is_empty() {
```

Replace it with:

```rust
        {
            return false;
        }
        self.written_bytes
            .set(self.written_bytes.get() + texels.len() as u64 * 16);
        let mut at = first;
        let mut rest = texels;
        while !rest.is_empty() {
```

**Locate this block:**

```rust
    pub fn new(ctx: &GpuCtx) -> Self {
        let texture = texture(ctx, 1, 1);
        let view = texture.create_view(&Default::default());
        Self { texture, view }
    }
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = u32::try_from(texels.len()).expect("Bounded slot table");
```

Replace it with:

```rust
    pub fn new(ctx: &GpuCtx) -> Self {
        let texture = texture(ctx, 1, 1);
        let view = texture.create_view(&Default::default());
        Self {
            written_bytes: Default::default(),
            texture,
            view,
        }
    }
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = u32::try_from(texels.len()).expect("Bounded slot table");
```

**Locate this block:**

```rust
pub struct Table {
    pub(crate) texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}
```

Replace it with:

```rust
pub struct Table {
    pub written_bytes: std::cell::Cell<u64>,
    pub(crate) texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}
```

### 3. `src/instance_gpu.rs`

Include the actual inverse lookup allocation in the same accounting. Count lookup bytes once alongside existing record and slot buffers.

**Locate this existing block:**

```rust
    }
    pub fn allocation_parts(&self) -> [u64; 2] {
        [
            self.records.allocated_bytes(),
            if self.slots.instances() == 0 {
                0
            } else {
                self.slots.allocated_bytes()
            },
        ]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn allocation_parts(&self) -> [u64; 3] {
        [
            self.records.allocated_bytes(),
            if self.slots.instances() == 0 {
                0
            } else {
                self.slots.allocated_bytes()
            },
            self.lookup
                .as_ref()
                .map_or(0, |table| table.allocated_bytes()),
        ]
```

### 4. `src/renderer.rs`

Expose all three measured allocation owners. The lane now synchronizes addresses once before record upload and projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn instance_usage(&self) -> [u64; 5] {
        self.instances.usage()
    }
    pub fn instance_allocations(&self) -> [u64; 2] {
        self.instances.allocation_parts()
```

Replace it with:

```rust
    pub fn instance_usage(&self) -> [u64; 5] {
        self.instances.usage()
    }
    pub fn instance_allocations(&self) -> [u64; 3] {
        self.instances.allocation_parts()
```

**Locate this block:**

```rust
        }
        let mut base = 0u32;
        for mesh in &mut self.meshes {
            mesh.set_primitive_base(&self.queue, base)
                .expect("Bounded projected triangle base");
            base = base
                .checked_add(mesh.geometry.index_count / 3)
                .filter(|n| *n < crate::primitive_image::CAP_START)
                .expect("Bounded projected triangle range");
        }
        let radii = self
            .ambient_rows
            .specs
```

Replace it with:

```rust
        }
        let radii = self
            .ambient_rows
            .specs
```

### 5. `src/global_outline_tests.rs`

Include the actual one-row 1024-texel lookup in exact two-definition allocation accounting. Keep the original projection relocation fixture explicitly based at zero, preserving every gap, byte, shrink and fallback assertion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .unwrap();
    let mut gpu = renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let baseline = pixels(&gpu);
```

Replace it with:

```rust
        .unwrap();
    let mut gpu = renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    gpu.meshes[0]
        .set_primitive_base(&gpu.queue, 0)
        .unwrap();
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    let baseline = pixels(&gpu);
```

**Locate this block:**

```rust
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(&gpu.instance_usage()[..3], &[2, 2, 376]);
    assert_eq!(gpu.instance_allocations(), [352, 24]);
    assert_eq!(gpu.instances.slots.instances(), 2);
    let slots =
        crate::growing_buffer_tests::read(&ctx, &gpu.instances.slots.buffer.buf, 24);
```

Replace it with:

```rust
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(&gpu.instance_usage()[..3], &[2, 2, 16760]);
    assert_eq!(gpu.instance_allocations(), [352, 24, 16384]);
    assert_eq!(gpu.instances.slots.instances(), 2);
    let slots =
        crate::growing_buffer_tests::read(&ctx, &gpu.instances.slots.buffer.buf, 24);
```

### 6. `src/slot_space.rs`

Report newly active retained members even when their row number is unchanged; inverse lookup counts must refresh after hide/show.

**Locate this existing block:**

```rust
                if position < run.cap as usize
                    && self.rows.get(run.start as usize + position) != Some(row)
                {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                if position < run.cap as usize
                    && (position >= run.len as usize
                        || self.rows.get(run.start as usize + position) != Some(row))
                {
```

### 7. `src/pick_gpu_tests.rs`

Retain all original copied/placed/nested owner, pixel, history and release assertions while adding the actual padded 16384-byte inverse lookup to exact allocation totals.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    assert_eq!(colour_pixels(&mut gpu), plain);
    Rc::make_mut(&mut scene.view).hide(&[b], true);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 376]);
    gpu.request_pick([-0.45, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut gpu).unwrap()[0], a.value());
    Rc::make_mut(&mut scene.view).hide(&[b], false);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    assert_eq!(colour_pixels(&mut gpu), plain);
```

Replace it with:

```rust
    assert_eq!(colour_pixels(&mut gpu), plain);
    Rc::make_mut(&mut scene.view).hide(&[b], true);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 16760]);
    gpu.request_pick([-0.45, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut gpu).unwrap()[0], a.value());
    Rc::make_mut(&mut scene.view).hide(&[b], false);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 16760]);
    assert_eq!(colour_pixels(&mut gpu), plain);
```

**Locate this block:**

```rust
    }
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    let uploads = gpu.stats()[0];
    let plain = colour_pixels(&mut gpu);
    let mut blue = [0; 2];
```

Replace it with:

```rust
    }
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 16760]);
    let uploads = gpu.stats()[0];
    let plain = colour_pixels(&mut gpu);
    let mut blue = [0; 2];
```

**Locate this block:**

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    for row in scene.objects() {
        let point = row
            .model
```

Replace it with:

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 16760]);
    for row in scene.objects() {
        let point = row
            .model
```

**Locate this block:**

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(&renderer.instance_usage()[..3], &[1, 2, 376]);
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

Replace it with:

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(
        &renderer.instance_usage()[..3],
        &[1, 2, 16760]
    );
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

### 8. `src/primitive_slot_table_tests.rs`

Supply actual lookup shader readback for inactive and reactivated retained rows, while checking unchanged draw-slot bytes and all earlier reuse, shrink, high-ID and release assertions.

**Locate this existing block:**

```rust
    );
    let mut cross = vec![[0, 100, 0, 0]; 1026];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    let shortened_members = [
        Definition {
            key: 1,
            rows: &rows[..1],
            faces: 30..36,
            pipes: 0..0,
            ribbons: 0..0,
        },
        Definition {
            key: 2,
            rows: &other,
            faces: 60..63,
            pipes: 0..0,
            ribbons: 0..0,
        },
    ];
    let touched = space.touched(&shortened_members);
    let change = space.update(&shortened_members, &touched, 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(&resolve(&table)[4..6], &[none; 2]);
    let touched = space.touched(&definitions);
    assert_eq!(touched, [(1, 1)]);
    let change = space.update(&definitions, &touched, 100);
    slots.publish(&ctx, &space, &change, &mut table);
    assert_eq!(slots.written_bytes, 0);
    assert_eq!(
        resolve(&table),
        vec![
            none,
            [99, u32::MAX],
            [10, 7],
            [11, 7],
            [10, 0x80000001],
            [11, 0x80000001],
            [20, u32::MAX - 1],
            none
        ]
    );
    let mut cross = vec![[0, 100, 0, 0]; 1026];
```

### 9. `src/global_outline_tests.rs`

Retain all original owner, background, selection, overlay, crop, density and reserved-ID guards while testing placed physical intervals after the arena range; compare later placements with their assigned non-overlapping object records.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let y = window.copy[1] + window.cursor[1] - window.draw[1];
    assert_eq!(
        pixels[(y * texture.width() + x) as usize],
        1,
        "Visible intervals must rebuild without a color frame"
    );
```

Replace it with:

```rust
    let y = window.copy[1] + window.cursor[1] - window.draw[1];
    assert_eq!(
        pixels[(y * texture.width() + x) as usize],
        2,
        "Visible survivor uses its current retained run without a color frame"
    );
```

**Locate this block:**

```rust
            width: 320 * density,
            height: 240 * density,
        });
        for (position, owner, primitive) in [([-0.5, 0.], a, 1u32), ([0.5, 0.], b, 2)] {
            gpu.request_pick(position, matrix()).unwrap();
            wait(&gpu);
            assert_eq!(
```

Replace it with:

```rust
            width: 320 * density,
            height: 240 * density,
        });
        for (position, owner, primitive) in [([-0.5, 0.], a, 2u32), ([0.5, 0.], b, 3)] {
            gpu.request_pick(position, matrix()).unwrap();
            wait(&gpu);
            assert_eq!(
```

**Locate this block:**

```rust
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = primitive_pixels(&gpu, &gpu.primitive.texture);
    assert_eq!(placed[120 * 320 + 64], 1);
    assert_eq!(placed[120 * 320 + 256], 2);
    gpu.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
```

Replace it with:

```rust
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = primitive_pixels(&gpu, &gpu.primitive.texture);
    let ids = gpu
        .meshes
        .iter()
        .map(|mesh| mesh.primitive_base() + 1)
        .collect::<Vec<_>>();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert!(
        ids.iter()
            .all(|id| *id > 0 && *id < crate::primitive_image::CAP_START)
    );
    assert_eq!(placed[120 * 320 + 64], ids[0]);
    assert_eq!(placed[120 * 320 + 256], ids[1]);
    gpu.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
```

**Locate this block:**

```rust
        pixels(&gpu);
        let before = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 1);
        assert_eq!(before[120 * 320 + 240], 2);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
```

Replace it with:

```rust
        pixels(&gpu);
        let before = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 2);
        assert_eq!(before[120 * 320 + 240], 3);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
```

**Locate this block:**

```rust
    assert_eq!(gpu.meshes[1].packed(), before);
    Rc::make_mut(&mut scene.view).hide(&[a], true);
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![0]);
    assert_eq!(gpu.meshes[0].id, b);
    assert!(gpu.meshes[0].selected());
    pixels(&gpu);
```

Replace it with:

```rust
    assert_eq!(gpu.meshes[1].packed(), before);
    Rc::make_mut(&mut scene.view).hide(&[a], true);
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![1]);
    assert_eq!(gpu.meshes[0].id, b);
    assert!(gpu.meshes[0].selected());
    pixels(&gpu);
```

**Locate this block:**

```rust
    assert_ne!(selected, baseline);
    scene.translate(b, [0., 0.1, 0.]).unwrap();
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert!(gpu.meshes[1].selected());
    for (position, owner) in [([-0.5, -0.1], a), ([0.5, 0.], b)] {
        gpu.request_pick(position, matrix()).unwrap();
```

Replace it with:

```rust
    assert_ne!(selected, baseline);
    scene.translate(b, [0., 0.1, 0.]).unwrap();
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![1, 2]);
    assert!(gpu.meshes[1].selected());
    for (position, owner) in [([-0.5, -0.1], a), ([0.5, 0.], b)] {
        gpu.request_pick(position, matrix()).unwrap();
```

**Locate this block:**

```rust
    };
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert!(gpu.meshes.iter().all(|row| !row.selected()));
    let baseline = pixels(&gpu);
    gpu.select_objects(&scene, &[b]).unwrap();
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert_eq!(
        gpu.meshes
            .iter()
```

Replace it with:

```rust
    };
    assert_eq!(bases(&gpu), vec![1, 2]);
    assert!(gpu.meshes.iter().all(|row| !row.selected()));
    let baseline = pixels(&gpu);
    gpu.select_objects(&scene, &[b]).unwrap();
    assert_eq!(bases(&gpu), vec![1, 2]);
    assert_eq!(
        gpu.meshes
            .iter()
```

### 10. `src/tile_gpu_tests.rs`

Read the first placed projected polygon after the two original demo arena triangles; retain exact clipping-corner, tile-depth, list bounds, cache and release checks.

**Locate this existing block:**

```rust
        .create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(&buffers[0], 0, &read, 0, 96);
    encoder.copy_buffer_to_buffer(&buffers[1], 0, &read, 96, buffers[1].size());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .create_command_encoder(&Default::default());
    let arena = scene
        .objects()
        .iter()
        .map(|object| (object.mesh.indices().len() / 3) as u64)
        .sum::<u64>();
    encoder.copy_buffer_to_buffer(&buffers[0], arena * 96, &read, 0, 96);
    encoder.copy_buffer_to_buffer(&buffers[1], 0, &read, 96, buffers[1].size());
```

### 11. `src/instance_gpu.rs`

Assign new record rows in original-ID order instead of geometry pointer order; retained rows and the original scene draw order remain unchanged.

**Locate this existing block:**

```rust
        };
        let ids = groups
            .values()
            .flat_map(|(_, rows)| rows.iter().map(|(id, _)| *id))
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        };
        let mut ids = groups
            .values()
            .flat_map(|(_, rows)| rows.iter().map(|(id, _)| *id))
            .chain(own.iter().map(|mesh| mesh.id))
            .collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
        let indices = stable_rows(&self.ordinary, &ids, limit)?;
```

## <a id="checkpoint-96zzzzo-separate-projection-rows-and-arena-offsets"></a>7. Separate projection rows and arena offsets

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/finite_project.wgsl</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Explicit object row and f…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared projection helper</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged projected recor…</text>
</svg>
</div>

**What this code does:** Make source addressing explicit before projection consumes the inverse lookup.

### 1. `src/finite_project.wgsl`

Pass the model row and index offset explicitly instead of closing over one dispatch interval. Read exactly the requested shared record or the unchanged uniform object. Keep arena addressing separate from object-row selection. Let the same projection algorithm accept either local or global arena triangles. Use the same explicit address contract for every triangle corner. Preserve the current local draw behavior while preparing lookup-selected arena inputs.

**Locate this existing block:**

```wgsl
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
    let determinant = dot(edge0, input[0].xyw);

    if (determinant == 0.0) {
        return polygon;
    }

    let plane = (input[0].z * edge0 + input[1].z * cross(input[2].xyw, input[0].xyw)
        + input[2].z * cross(input[0].xyw, input[1].xyw)) / determinant;
    polygon.depth = vec3<f32>(2.0 * plane.x / line.vp_w, -2.0 * plane.y / line.vp_h, plane.z);
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;

        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

    // clip space to screen pixels
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];

        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }

        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }

    return polygon;
}
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
};
fn physical_clip_corner(index:u32,row:u32,first:u32)->vec4<f32>{
 let object=object_at(row);
 let point=packed_position(physical_vertices[physical_indices[index+first]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32, first:u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base,row,first), physical_clip_corner(base+1u,row,first), physical_clip_corner(base+2u,row,first));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
    let determinant = dot(edge0, input[0].xyw);

    if (determinant == 0.0) {
        return polygon;
    }

    let plane = (input[0].z * edge0 + input[1].z * cross(input[2].xyw, input[0].xyw)
        + input[2].z * cross(input[0].xyw, input[1].xyw)) / determinant;
    polygon.depth = vec3<f32>(2.0 * plane.x / line.vp_w, -2.0 * plane.y / line.vp_h, plane.z);
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;

        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

    // clip space to screen pixels
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];

        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }

        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }

    return polygon;
}
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);
    var out: ProjectedTriangle;
```

## <a id="checkpoint-96zzzzp-prepare-lookup-selected-triangle-projection"></a>8. Prepare lookup-selected triangle projection

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/finite_project_source.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Physical primitive ID</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">arena triangle and object…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared projection</text>
</svg>
</div>

**What this code does:** Resolve physical primitive IDs to their shared source triangle and object row.

### 1. `src/finite_project_source.rs`

Assemble lookup-selected projection from the existing shader and unsigned lookup helper; keep one clipping and projection algorithm. Supply validation of uniform, shared per-mesh and lookup-selected compute interfaces.

**Locate this existing block:**

```rust
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        for source in [super::source(), super::records()] {
            let module = naga::front::wgsl::parse_str(&source)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn lookup() -> String {
    let source=records().replace("let index = local + live_count.y;","let index = local;").replace("let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);","let source=placed_triangle(index+1u,live_count.y);if(source.x==NO_TRIANGLE||source.y==SLOT_OWN_ROW){return;}let polygon=project_physical_triangle(source.x+1u,source.y,0u);");
    format!(
        "@group(2) @binding(5) var slot_table:texture_2d<u32>;\n{}\n{}",
        include_str!("primitive_slot_lookup.wgsl"),
        source
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        for source in [
            super::source(),
            super::records(),
            super::lookup(),
        ] {
            let module = naga::front::wgsl::parse_str(&source)
```

## <a id="checkpoint-96zzzzq-prepare-the-lookup-projection-pipeline"></a>9. Prepare the lookup projection pipeline

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/tile_pipelines.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing compute recipe p…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared projection pipel…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared projection pipel…</text>
</svg>
</div>

**What this code does:** Add the inverse lookup texture to the existing compute binding recipe.

### 1. `src/tile_pipelines.rs`

Keep the lookup projection layout beside the two existing resource contracts. Own the lookup state with the existing tile pipelines. Extend the existing binding recipe with one unsigned texture input instead of duplicating the compute descriptor. Select the shared projection algorithm assembled with the inverse lookup. Prepare uniform, shared per-mesh and lookup states through one constructor. Retain the new layout under the existing pipeline lifetime. Retain the prepared compute state without changing dispatches yet.

**Locate this existing block:**

```rust
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

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub record_project_layout: wgpu::BindGroupLayout,
    pub lookup_project_layout: wgpu::BindGroupLayout,
    pub raster_layout: wgpu::BindGroupLayout,
    pub scan_layout: wgpu::BindGroupLayout,
    pub project: wgpu::ComputePipeline,
    pub record_project: wgpu::ComputePipeline,
    pub lookup_project: wgpu::ComputePipeline,
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
        let create = |shared, lookup| {
            let mut entries = vec![
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
            ];
            if lookup {
                entries.push(crate::text_gpu_support::texture_entry(
                    5,
                    Stage::COMPUTE,
                    wgpu::TextureSampleType::Uint,
                    false,
                ));
            }
            let project_layout = layout("finite original projection", &entries);
            let source = if lookup {
                crate::finite_project_source::lookup()
            } else if shared {
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
        let (project_layout, project) = create(false, false);
        let (record_project_layout, record_project) = create(true, false);
        let (lookup_project_layout, lookup_project) = create(true, true);
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
            lookup_project_layout,
            raster_layout,
            scan_layout,
            project,
            record_project,
            lookup_project,
            count,
```

## <a id="checkpoint-96zzzzr-own-current-lookup-projection-bindings"></a>10. Own current lookup projection bindings

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/projection_lookup.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current shared inputs</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one dispatch interval</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">output-aware immutable bi…</text>
</svg>
</div>

**What this code does:** Keep current shared inputs and replace immutable projection bindings when needed.

### 1. `src/projection_lookup.rs`

Own immutable current arena/record/lookup bindings and one bounded dispatch interval; replace the group when the output buffer changes.

Create the file and type:

```rust
use crate::{gpu_mesh::GpuMesh, instance_gpu::Lane};
use wgpu::util::DeviceExt;
pub struct Bindings {
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    objects: wgpu::Buffer,
    pub(crate) table: wgpu::TextureView,
    pub(crate) parameters: wgpu::Buffer,
    group: Option<(wgpu::Buffer, wgpu::BindGroup)>,
}
impl Bindings {
    pub fn source<'a>(
        meshes: &[GpuMesh],
        records: Option<&'a Lane>,
    ) -> Option<&'a Lane> {
        records.filter(|records| {
            records.lookup.is_some()
                && records.space.is_some()
                && meshes
                    .iter()
                    .any(|mesh| mesh.geometry.index_count > 0)
                && meshes
                    .iter()
                    .all(|mesh| records.current_row(mesh).is_some())
        })
    }
    pub fn new(
        device: &wgpu::Device,
        meshes: &[GpuMesh],
        records: Option<&Lane>,
        count: u32,
    ) -> Option<Self> {
        if count == 0 {
            return None;
        }
        let records = Self::source(meshes, records)?;
        let mesh = meshes
            .iter()
            .find(|mesh| mesh.geometry.index_count > 0)?;
        let (vertices, indices) = mesh.geometry.buffers();
        let table = records.lookup.as_ref()?.view.clone();
        let arena = records.space.as_ref()?.texel(0)[1];
        let bytes = [count, arena, 0, 0]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        let parameters = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("lookup projection interval"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
        });
        Some(Self {
            vertices,
            indices,
            objects: records.record_buffer().clone(),
            table,
            parameters,
            group: None,
        })
    }
    pub fn group(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        projected: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        if let Some((old, group)) = &self.group {
            if old == projected {
                return group.clone();
            }
        }
        let buffers = [
            &self.vertices,
            &self.indices,
            &self.objects,
            projected,
            &self.parameters,
        ];
        let mut entries = buffers
            .into_iter()
            .enumerate()
            .map(|(i, buffer)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect::<Vec<_>>();
        entries.push(wgpu::BindGroupEntry {
            binding: 5,
            resource: wgpu::BindingResource::TextureView(&self.table),
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("current lookup projection"),
            layout,
            entries: &entries,
        });
        self.group = Some((projected.clone(), group.clone()));
        group
    }
    pub fn bytes(&self) -> u64 {
        self.parameters.size()
    }
}
```

### 2. `src/lib.rs`

Declare the binding owner without changing live projection dispatch.

**Locate this existing block:**

```rust
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod inventory_tests;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod ambient_shader_contract;
mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod inventory_tests;
pub mod projection_lookup;
pub mod source_mesh_topology;
```

### 3. `src/global_outline_tests.rs`

Supply actual mapped dispatch words, exact immutable-group reuse and replacement, missing/current-record refusal and empty checks.

**Locate this existing block:**

```rust
    assert!(gpu.instances.space.is_none());
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(gpu.instances.space.is_none());
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
    let mut scene = Scene::empty();
    scene
        .insert(
            PreparedMesh::triangle(
                [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                [0.1, 0.4, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    let mut gpu = renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let count = gpu.meshes[0].primitive_base() + 1;
    assert_eq!(count, 2);
    let mut state = crate::projection_lookup::Bindings::new(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
        count,
    )
    .unwrap();
    assert_eq!(state.bytes(), 16);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &state.parameters, 16),
        [2u32, 1, 0, 0]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>()
    );
    let pipes = crate::tile_pipelines::Pipelines::new(
        &gpu.device,
        &gpu.pipeline.get_bind_group_layout(0),
    );
    let output = |size| {
        gpu.device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("lookup projection output witness"),
                size,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
    };
    let first = output(192);
    let group = state.group(
        &gpu.device,
        &pipes.lookup_project_layout,
        &first,
    );
    assert_eq!(
        state.group(
            &gpu.device,
            &pipes.lookup_project_layout,
            &first
        ),
        group
    );
    let larger = output(384);
    let next = state.group(
        &gpu.device,
        &pipes.lookup_project_layout,
        &larger,
    );
    assert_ne!(next, group);
    assert_eq!(
        state.group(
            &gpu.device,
            &pipes.lookup_project_layout,
            &larger
        ),
        next
    );
    assert!(
        crate::projection_lookup::Bindings::new(&gpu.device, &gpu.meshes, None, count)
            .is_none()
    );
    assert!(
        crate::projection_lookup::Bindings::new(
            &gpu.device,
            &gpu.meshes,
            Some(&gpu.instances),
            0
        )
        .is_none()
    );
    gpu.instances.ordinary.clear();
    assert!(
        crate::projection_lookup::Bindings::new(
            &gpu.device,
            &gpu.meshes,
            Some(&gpu.instances),
            count
        )
        .is_none()
    );
    scene.clear();
    gpu.set_scene(&scene, None);
    assert!(
        crate::projection_lookup::Bindings::new(
            &gpu.device,
            &gpu.meshes,
            Some(&gpu.instances),
            0
        )
        .is_none()
    );
}
```

## <a id="checkpoint-96zzzzs-project-live-triangles-through-the-lookup"></a>11. Project live triangles through the lookup

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/projection_lookup.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Physical IDs</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original arena triangle a…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing tile bins</text>
</svg>
</div>

**What this code does:** Resolve shared source triangles and object rows on the GPU for live projection.

### 1. `src/projection_lookup.rs`

Release cached output references when tile resources are replaced or released.

**Locate this existing block:**

```rust
    }
    pub fn bytes(&self) -> u64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn invalidate(&mut self) {
        self.group = None;
    }
    pub fn bytes(&self) -> u64 {
```

### 2. `src/tile_gpu.rs`

Own the current global lookup binding state under the existing tile lifetime. Begin with the existing per-mesh fallback. Drop global inputs when the physical projection interval exceeds the device budget. Include inverse lookup identity in the existing source/model witness without invalidating projection for selection-only bytes. Allocate one global dispatch interval, or retain the existing guarded per-mesh intervals. Keep one active projection resource path. Release stale immutable output groups during resize and hidden-viewport release. Resolve all live physical IDs through the lookup in one dispatch, preserving the earlier projection algorithm and fallback. Count only the active interval owner; geometry, object and lookup allocations remain counted by their existing owners.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        [
            self.projected.size(),
            self.buffer.size(),
            32 + 16 + 16 * self.rows.len() as u64,
            self.triangles as u64,
```

Replace it with:

```rust
        [
            self.projected.size(),
            self.buffer.size(),
            32 + 16
                + 16 * self.rows.len() as u64
                + self
                    .lookup
                    .as_ref()
                    .map_or(0, |state| state.bytes()),
            self.triangles as u64,
```

**Locate this block:**

```rust
            return;
        }
        encoder.clear_buffer(&self.projected, 0, None);
        for row in &mut self.rows {
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
                        &row.model,
                        &self.projected,
                        &row.count,
                    ],
                )
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("finite original projection"),
                timestamp_writes: None,
            });
            pass.set_pipeline(if row.shared {
                &self.pipes.record_project
            } else {
                &self.pipes.project
            });
            pass.set_bind_group(0, view, &[]);
            pass.set_bind_group(1, &self.line_group, &[]);
            pass.set_bind_group(2, &*bindings, &[]);
            let (x, y) = project_groups(row.triangles);
            pass.dispatch_workgroups(x, y, 1);
        }
        self.projections += 1;
        encoder.clear_buffer(
```

Replace it with:

```rust
            return;
        }
        encoder.clear_buffer(&self.projected, 0, None);
        if let Some(lookup) = &mut self.lookup {
            let bindings = lookup.group(
                device,
                &self.pipes.lookup_project_layout,
                &self.projected,
            );
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("shared lookup triangle projection"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipes.lookup_project);
            pass.set_bind_group(0, view, &[]);
            pass.set_bind_group(1, &self.line_group, &[]);
            pass.set_bind_group(2, &bindings, &[]);
            let (x, y) = project_groups(self.triangles);
            pass.dispatch_workgroups(x, y, 1);
        } else {
            for row in &mut self.rows {
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
                            &row.model,
                            &self.projected,
                            &row.count,
                        ],
                    )
                });
                let mut pass =
                    encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("finite original projection"),
                        timestamp_writes: None,
                    });
                pass.set_pipeline(if row.shared {
                    &self.pipes.record_project
                } else {
                    &self.pipes.project
                });
                pass.set_bind_group(0, view, &[]);
                pass.set_bind_group(1, &self.line_group, &[]);
                pass.set_bind_group(2, &*bindings, &[]);
                let (x, y) = project_groups(row.triangles);
                pass.dispatch_workgroups(x, y, 1);
            }
        }
        self.projections += 1;
        encoder.clear_buffer(
```

**Locate this block:**

```rust
        for row in &mut self.rows {
            row.group = None;
        }
    }
    fn release(&mut self, device: &wgpu::Device) {
        if self.layout.take().is_none()
```

Replace it with:

```rust
        for row in &mut self.rows {
            row.group = None;
        }
        if let Some(lookup) = &mut self.lookup {
            lookup.invalidate();
        }
    }
    fn release(&mut self, device: &wgpu::Device) {
        if self.layout.take().is_none()
```

**Locate this block:**

```rust
                )
            })
            .collect::<Vec<_>>();
        if witness == self.witness {
            return;
        }
        self.rows.clear();
        for row in meshes
            .iter()
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
                usage: wgpu::BufferUsages::UNIFORM,
            });
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
        self.witness = witness;
        self.triangles = count.unwrap();
```

Replace it with:

```rust
                )
            })
            .collect::<Vec<_>>();
        let lookup = crate::projection_lookup::Bindings::source(meshes, records);
        let lookup_view = lookup
            .and_then(|records| records.lookup.as_ref())
            .map(|table| &table.view);
        if witness == self.witness
            && self
                .lookup
                .as_ref()
                .map(|state| &state.table)
                == lookup_view
        {
            return;
        }
        self.rows.clear();
        self.lookup = crate::projection_lookup::Bindings::new(
            device,
            meshes,
            lookup,
            count.unwrap(),
        );
        if self.lookup.is_none() {
            for row in meshes
                .iter()
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
                let count =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("finite original triangle interval"),
                        contents: &bytes,
                        usage: wgpu::BufferUsages::UNIFORM,
                    });
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
        }
        self.witness = witness;
        self.triangles = count.unwrap();
```

**Locate this block:**

```rust
        self.fallback =
            count.is_none_or(|count| u64::from(count) * PROJECTED_BYTES > limit);
        if self.fallback {
            self.rows.clear();
            self.witness.clear();
            self.triangles = 0;
```

Replace it with:

```rust
        self.fallback =
            count.is_none_or(|count| u64::from(count) * PROJECTED_BYTES > limit);
        if self.fallback {
            self.lookup = None;
            self.rows.clear();
            self.witness.clear();
            self.triangles = 0;
```

**Locate this block:**

```rust
        );
        let scan = group(device, &pipes.scan_layout, &[&buffer]);
        Self {
            projected,
            buffer,
            pipes,
```

Replace it with:

```rust
        );
        let scan = group(device, &pipes.scan_layout, &[&buffer]);
        Self {
            lookup: None,
            projected,
            buffer,
            pipes,
```

**Locate this block:**

```rust
pub struct Tiles {
    pub projected: wgpu::Buffer,
    pub buffer: wgpu::Buffer,
    pipes: Pipelines,
```

Replace it with:

```rust
pub struct Tiles {
    pub(crate) lookup: Option<crate::projection_lookup::Bindings>,
    pub projected: wgpu::Buffer,
    pub buffer: wgpu::Buffer,
    pipes: Pipelines,
```

### 3. `src/renderer.rs`

Observe explicit address overrides after style changes while the unchanged witness retains cached projections for ordinary selection.

**Locate this existing block:**

```rust
        )?;
        if !self.meshes.iter().any(GpuMesh::selected) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )?;
        self.tiles.borrow_mut().set_records(
            &self.device,
            &self.meshes,
            Some(&self.instances),
        );
        if !self.meshes.iter().any(GpuMesh::selected) {
```

### 4. `src/global_outline_tests.rs`

Supply exact global/per-mesh projected bytes and pixels across distinct and copied definitions, selection cache reuse, unchanged geometry uploads and full release.

**Locate this existing block:**

```rust
        .is_none()
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .is_none()
    );
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap())
        .unwrap();
    let other = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [0.1, 0.1, -0.3],
                    [0.3, 0.1, -0.3],
                    [0.2, 0.3, -0.3],
                ],
                [0.1, 0.4, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    let copy = scene.copy_row(other, false).unwrap();
    scene.translate(copy, [0.4, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(0.95);
    gpu.set_scene(&scene, None);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let image = pixels(&gpu);
    assert!(gpu.tiles.borrow().lookup.is_some());
    let output = gpu.tiles.borrow().projected.clone();
    let wanted = crate::growing_buffer_tests::read(&ctx, &output, output.size());
    let uploads = gpu.stats()[0];
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    assert!(gpu.tiles.borrow().lookup.is_none());
    assert_eq!(pixels(&gpu), image);
    let output = gpu.tiles.borrow().projected.clone();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &output, output.size()),
        wanted
    );
    gpu.tiles.borrow_mut().set_records(
        &gpu.device,
        &gpu.meshes,
        Some(&gpu.instances),
    );
    assert_eq!(pixels(&gpu), image);
    assert!(gpu.tiles.borrow().lookup.is_some());
    let projections = gpu.tile_usage()[5];
    gpu.select_objects(&scene, &[owner]).unwrap();
    assert_ne!(pixels(&gpu), image);
    assert_eq!(gpu.tile_usage()[5], projections);
    gpu.select_objects(&scene, &[]).unwrap();
    assert_eq!(pixels(&gpu), image);
    assert_eq!(gpu.tile_usage()[5], projections);
    assert_eq!(gpu.stats()[0], uploads);
    scene.clear();
    gpu.set_scene(&scene, None);
    pixels(&gpu);
    assert!(gpu.tiles.borrow().lookup.is_none());
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}
```

## <a id="checkpoint-96zzzzt-plan-reusable-arena-spans"></a>12. Plan reusable arena spans

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/arena_spans.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Free intervals plus tail</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded plan</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">take after both ranges va…</text>
</svg>
</div>

**What this code does:** Prepare bounded hole reuse without moving any surviving geometry.

### 1. `src/arena_spans.rs`

Plan reusable spans before committing either geometry range, and reuse the existing coalescer for retirement.

Create the file and type:

```rust
use std::ops::Range;
pub struct Plan {
    pub range: Range<u32>,
    at: Option<usize>,
}
pub fn plan(free: &[Range<u32>], count: u32, end: u32) -> Option<Plan> {
    if count > 0 {
        if let Some(at) = free
            .iter()
            .position(|span| span.end - span.start >= count)
        {
            let first = free[at].start;
            return Some(Plan {
                range: first..first + count,
                at: Some(at),
            });
        }
    }
    Some(Plan {
        range: end..end.checked_add(count)?,
        at: None,
    })
}
impl Plan {
    pub fn take(self, free: &mut Vec<Range<u32>>) -> Range<u32> {
        if let Some(at) = self.at {
            free[at].start = self.range.end;
            if free[at].is_empty() {
                free.remove(at);
            }
        }
        self.range
    }
}
pub fn retire(free: &mut Vec<Range<u32>>, range: Range<u32>, mut end: u32) -> u32 {
    assert!(range.start <= range.end && range.end <= end);
    free.push(range);
    *free = crate::slot_space::coalesce(std::mem::take(free), end);
    while free
        .last()
        .is_some_and(|span| span.end == end)
    {
        end = free.pop().unwrap().start;
    }
    end
}
```

### 2. `src/lib.rs`

Declare the complete preparatory span helper.

**Locate this existing block:**

```rust
pub mod navigation;

pub mod arena_rows;
#[cfg(test)]
mod arena_rows_tests;
#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod navigation;

mod arena_rows;
#[cfg(test)]
mod arena_rows_tests;
pub mod arena_spans;
#[cfg(target_arch = "wasm32")]
```

### 3. `src/arena_spans.rs`

Supply hole reuse, unchanged preflight state, overflow refusal, zero spans, coalescing and free-tail checks.

**Locate this existing block:**

```rust
    end
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    end
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plans_reuse_holes_merge_free_tails_and_refuse_overflow() {
        let mut free = vec![3..6, 9..12];
        let first = plan(&free, 2, 12).unwrap();
        assert_eq!(first.range, 3..5);
        assert_eq!(free, [3..6, 9..12]);
        assert_eq!(first.take(&mut free), 3..5);
        assert_eq!(free, [5..6, 9..12]);
        assert_eq!(
            plan(&free, 3, 12).unwrap().take(&mut free),
            9..12
        );
        assert_eq!(free, [5..6]);
        assert_eq!(
            plan(&free, 4, 12).unwrap().take(&mut free),
            12..16
        );
        assert_eq!(free, [5..6]);
        let zero = plan(&free, 0, 16).unwrap();
        assert_eq!(zero.take(&mut free), 16..16);
        assert_eq!(free, [5..6]);
        assert!(plan(&[], 3, u32::MAX - 1).is_none());
        assert_eq!(
            plan(&[0..4], 3, u32::MAX - 1).unwrap().range,
            0..3
        );
        let vertex = plan(&free, 1, 16).unwrap();
        assert!(plan(&[], 3, u32::MAX - 1).is_none());
        assert_eq!(free, [5..6]);
        assert_eq!(vertex.take(&mut free), 5..6);
        let mut free = Vec::new();
        assert_eq!(retire(&mut free, 3..6, 9), 9);
        assert_eq!(free, [3..6]);
        assert_eq!(retire(&mut free, 6..9, 9), 3);
        assert!(free.is_empty());
        assert_eq!(retire(&mut free, 0..3, 3), 0);
        assert!(free.is_empty());
    }
}
```

## <a id="checkpoint-96zzzzu-reuse-retired-arena-record-ranges"></a>13. Reuse retired arena record ranges

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/arena_rows.rs, src/aren…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retired source association</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked vertex/index plans</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current packed records</text>
</svg>
</div>

**What this code does:** Connect bounded span plans to CPU arena records while preserving live offsets.

### 1. `src/arena_rows.rs`

Keep free vertex and index spans under the existing arena record owner. Validate both ranges and rebased addresses before taking spans. Retire the old source mapping and trim only a free tail; surviving addresses remain fixed. Reset free-span metadata with the existing logical arena reset.

**Locate this existing block:**

```rust
    sources: HashMap<usize, (Weak<Mesh>, MeshRange)>,
}
impl Rows {
    pub fn append(&mut self, source: &Rc<Mesh>) -> Result<MeshRange, &'static str> {
        let key = Rc::as_ptr(source) as usize;
        if let Some((weak, range)) = self.sources.get(&key) {
            if weak
                .upgrade()
                .is_some_and(|saved| Rc::ptr_eq(&saved, source))
            {
                return Ok(range.clone());
            }
        }
        let start = u32::try_from(self.vertices.len() / 20)
            .map_err(|_| "Arena vertex range")?;
        let count =
            u32::try_from(source.vertices().len()).map_err(|_| "Arena vertex count")?;
        let end = start
            .checked_add(count)
            .ok_or("Arena vertex range")?;
        let first =
            u32::try_from(self.indices.len()).map_err(|_| "Arena index range")?;
        let count =
            u32::try_from(source.indices().len()).map_err(|_| "Arena index count")?;
        let last = first
            .checked_add(count)
            .ok_or("Arena index range")?;
        let range = MeshRange {
            vertices: start..end,
            indices: first..last,
        };
        self.vertices
            .extend(crate::packed_vertex::bytes(source));
        self.indices
            .extend(source.indices().iter().map(|i| i + start));
        self.sources
            .insert(key, (Rc::downgrade(source), range.clone()));
        Ok(range)
    }
    pub fn index_bytes(&self) -> Vec<u8> {
        self.index_bytes_from(0)
    }
    pub fn index_bytes_from(&self, start: usize) -> Vec<u8> {
        self.indices[start..]
            .iter()
            .flat_map(|i| i.to_le_bytes())
            .collect()
    }
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
    pub fn reset(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.sources.clear();
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    sources: HashMap<usize, (Weak<Mesh>, MeshRange)>,
    free_vertices: Vec<Range<u32>>,
    free_indices: Vec<Range<u32>>,
}
impl Rows {
    pub fn append(&mut self, source: &Rc<Mesh>) -> Result<MeshRange, &'static str> {
        let key = Rc::as_ptr(source) as usize;
        if let Some((weak, range)) = self.sources.get(&key) {
            if weak
                .upgrade()
                .is_some_and(|saved| Rc::ptr_eq(&saved, source))
            {
                return Ok(range.clone());
            }
        }
        let vertex_count =
            u32::try_from(source.vertices().len()).map_err(|_| "Arena vertex count")?;
        let index_count =
            u32::try_from(source.indices().len()).map_err(|_| "Arena index count")?;
        let vertex_end = u32::try_from(self.vertices.len() / 20)
            .map_err(|_| "Arena vertex range")?;
        let index_end =
            u32::try_from(self.indices.len()).map_err(|_| "Arena index range")?;
        let vertices =
            crate::arena_spans::plan(&self.free_vertices, vertex_count, vertex_end)
                .ok_or("Arena vertex range")?;
        let indices =
            crate::arena_spans::plan(&self.free_indices, index_count, index_end)
                .ok_or("Arena index range")?;
        let byte_end = usize::try_from(u64::from(vertices.range.end) * 20)
            .map_err(|_| "Arena vertex bytes")?;
        let rebased = source
            .indices()
            .iter()
            .map(|i| {
                i.checked_add(vertices.range.start)
                    .ok_or("Arena index address")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let bytes = crate::packed_vertex::bytes(source);
        let range = MeshRange {
            vertices: vertices.take(&mut self.free_vertices),
            indices: indices.take(&mut self.free_indices),
        };
        self.vertices
            .resize(self.vertices.len().max(byte_end), 0);
        self.indices.resize(
            self.indices
                .len()
                .max(range.indices.end as usize),
            0,
        );
        self.vertices[range.vertices.start as usize * 20..byte_end]
            .copy_from_slice(&bytes);
        self.indices[range.indices.start as usize..range.indices.end as usize]
            .copy_from_slice(&rebased);
        self.sources
            .insert(key, (Rc::downgrade(source), range.clone()));
        Ok(range)
    }
    pub(crate) fn retire(&mut self, key: usize) -> bool {
        let Some((_, range)) = self.sources.remove(&key) else {
            return false;
        };
        let vertices = crate::arena_spans::retire(
            &mut self.free_vertices,
            range.vertices,
            (self.vertices.len() / 20) as u32,
        );
        let indices = crate::arena_spans::retire(
            &mut self.free_indices,
            range.indices,
            self.indices.len() as u32,
        );
        self.vertices
            .truncate(vertices as usize * 20);
        self.indices.truncate(indices as usize);
        true
    }
    pub fn index_bytes(&self) -> Vec<u8> {
        self.index_bytes_from(0)
    }
    pub fn index_bytes_from(&self, start: usize) -> Vec<u8> {
        self.indices[start..]
            .iter()
            .flat_map(|i| i.to_le_bytes())
            .collect()
    }
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
    pub fn reset(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.sources.clear();
        self.free_vertices.clear();
        self.free_indices.clear();
    }
```

### 2. `src/arena_rows_tests.rs`

Supply exact packed bytes, rebased indices, surviving offsets, retained CPU history, safe source restoration and coalesced tail retirement.

**Locate this existing block:**

```rust
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
}

#[test]
fn retired_ranges_reuse_holes_without_pinning_or_moving_surviving_sources() {
    let a = source();
    let b = source();
    let history = Rc::clone(&a);
    let weak = Rc::downgrade(&a);
    let mut rows = crate::arena_rows::Rows::default();
    let first = rows.append(&a).unwrap();
    let survivor = rows.append(&b).unwrap();
    let held = rows.vertices[60..120].to_vec();
    assert!(!rows.retire(usize::MAX));
    assert!(rows.retire(Rc::as_ptr(&a) as usize));
    assert!(weak.upgrade().is_some());
    assert_eq!(rows.source_count(), 1);
    let c = Rc::new(
        (*source())
            .clone()
            .with_alphas(vec![0.5; 3])
            .unwrap(),
    );
    let reused = rows.append(&c).unwrap();
    assert_eq!(reused, first);
    assert_eq!(rows.append(&b).unwrap(), survivor);
    assert_eq!(rows.vertices.len(), 120);
    assert_eq!(
        &rows.vertices[..60],
        crate::packed_vertex::bytes(&c)
    );
    assert_eq!(&rows.vertices[60..120], held);
    assert_eq!(rows.indices, [0, 1, 2, 3, 4, 5]);
    let restored = rows.append(&history).unwrap();
    assert_eq!(restored.vertices, 6..9);
    assert_eq!(restored.indices, 6..9);
    assert_ne!(restored, reused);
    assert_eq!(a.indices(), &[0, 1, 2]);
    assert_eq!(b.indices(), &[0, 1, 2]);
    assert_eq!(c.indices(), &[0, 1, 2]);
    assert!(rows.retire(Rc::as_ptr(&c) as usize));
    assert!(rows.retire(Rc::as_ptr(&a) as usize));
    assert_eq!(rows.vertices.len(), 120);
    assert_eq!(rows.append(&b).unwrap(), survivor);
    assert!(rows.retire(Rc::as_ptr(&b) as usize));
    assert!(rows.vertices.is_empty());
    assert!(rows.indices.is_empty());
    assert_eq!(rows.source_count(), 0);
    assert_eq!(rows.append(&history).unwrap().vertices, 0..3);
}
```

## <a id="checkpoint-96zzzzv-upload-reused-arena-spans-in-place"></a>14. Upload reused arena spans in place

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_gpu_support.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Current arena range</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained/grown buffers</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact upload totals</text>
</svg>
</div>

**What this code does:** Write reused geometry ranges without replacing unchanged GPU buffers.

### 1. `src/text_gpu_support.rs`

Use the same device-limit calculation for append and bounded overwrite operations. Validate alignment, start and complete end before any write, then reuse existing bounded overwrite and growth operations.

**Locate this existing block:**

```rust
    }

    pub fn append_bytes(&mut self, ctx: &GpuCtx, bytes: &[u8]) -> bool {
        assert_eq!(bytes.len() as u64 % self.stride, 0);
        if bytes.is_empty() {
            return false;
        }
        let rows = u32::try_from(bytes.len() as u64 / self.stride)
            .expect("Bounded GPU records");
        let end = self
            .len
            .checked_add(rows)
            .expect("GPU row count");
        let limits = ctx.device.limits();
        let limit = if self
            .usage
            .contains(wgpu::BufferUsages::STORAGE)
        {
            limits.max_buffer_size.min(u64::from(
                limits.max_storage_buffer_binding_size,
            ))
        } else {
            limits.max_buffer_size
        };
        let most = limit / self.stride;
        assert!(u64::from(end) <= most);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    fn max_rows(&self, ctx: &GpuCtx) -> u64 {
        let limits = ctx.device.limits();
        let bytes = if self
            .usage
            .contains(wgpu::BufferUsages::STORAGE)
        {
            limits.max_buffer_size.min(u64::from(
                limits.max_storage_buffer_binding_size,
            ))
        } else {
            limits.max_buffer_size
        };
        bytes / self.stride
    }
    pub fn write_or_append(&mut self, ctx: &GpuCtx, at: u32, bytes: &[u8]) -> bool {
        assert_eq!(bytes.len() as u64 % self.stride, 0);
        assert!(at <= self.len);
        let rows = u32::try_from(bytes.len() as u64 / self.stride)
            .expect("Bounded GPU records");
        let end = at.checked_add(rows).expect("GPU row count");
        assert!(u64::from(end) <= self.max_rows(ctx));
        let inside =
            usize::try_from(u64::from((self.len - at).min(rows)) * self.stride)
                .unwrap();
        self.write_bytes_at(ctx, at, &bytes[..inside]);
        self.append_bytes(ctx, &bytes[inside..])
    }
    pub fn append_bytes(&mut self, ctx: &GpuCtx, bytes: &[u8]) -> bool {
        assert_eq!(bytes.len() as u64 % self.stride, 0);
        if bytes.is_empty() {
            return false;
        }
        let rows = u32::try_from(bytes.len() as u64 / self.stride)
            .expect("Bounded GPU records");
        let end = self
            .len
            .checked_add(rows)
            .expect("GPU row count");
        let most = self.max_rows(ctx);
        assert!(u64::from(end) <= most);
```

### 2. `src/mesh_arena.rs`

Measure real new geometry uploads and written source bytes under the arena owner. Start upload counters with the arena. Write exact reused ranges and count real writes; preserve current groups unless a buffer allocation changes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        ctx: &GpuCtx,
        source: &Rc<Mesh>,
    ) -> Result<MeshRange, &'static str> {
        let vertex_start = self.rows.vertices.len();
        let index_start = self.rows.indices.len();
        let range = self.rows.append(source)?;
        let vertices = self
            .vertices
            .append_bytes(ctx, &self.rows.vertices[vertex_start..]);
        let indices = self
            .indices
            .append_bytes(ctx, &self.rows.index_bytes_from(index_start));
        if vertices || indices {
```

Replace it with:

```rust
        ctx: &GpuCtx,
        source: &Rc<Mesh>,
    ) -> Result<MeshRange, &'static str> {
        let sources = self.rows.source_count();
        let range = self.rows.append(source)?;
        if sources == self.rows.source_count() {
            return Ok(range);
        }
        let bytes = &self.rows.vertices
            [range.vertices.start as usize * 20..range.vertices.end as usize * 20];
        let index_bytes = self.rows.indices
            [range.indices.start as usize..range.indices.end as usize]
            .iter()
            .flat_map(|i| i.to_le_bytes())
            .collect::<Vec<_>>();
        let vertices = self
            .vertices
            .write_or_append(ctx, range.vertices.start, bytes);
        let indices =
            self.indices
                .write_or_append(ctx, range.indices.start, &index_bytes);
        self.uploads += 1;
        self.uploaded_bytes += (bytes.len() + index_bytes.len()) as u64;
        self.vertices
            .truncate((self.rows.vertices.len() / 20) as u32);
        self.indices
            .truncate(self.rows.indices.len() as u32);
        if vertices || indices {
```

**Locate this block:**

```rust
            layout,
            group,
            revision: 0,
        }
    }
    fn bind(
```

Replace it with:

```rust
            layout,
            group,
            revision: 0,
            uploads: 0,
            uploaded_bytes: 0,
        }
    }
    fn bind(
```

**Locate this block:**

```rust
    pub indices: GrowBuf,
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
    pub revision: u64,
```

Replace it with:

```rust
    pub indices: GrowBuf,
    pub uploads: usize,
    pub uploaded_bytes: u64,
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
    pub revision: u64,
```

### 3. `src/mesh_arena_tests.rs`

Supply mapped reused bytes, exact upload totals, retained CPU source, real blue/green/yellow pixels, old/fresh immutable bindings and refused-write preservation.

**Locate this existing block:**

```rust
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
}

#[test]
fn actual_gpu_arena_reuses_ranges_preserves_bindings_and_counts_exact_writes() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut arena = crate::mesh_arena::Arena::new(&ctx);
    let a = mesh(-0.45, [1., 0., 0.]);
    let b = mesh(0.45, [0., 1., 0.]);
    let history = Rc::clone(&a);
    let first = arena.upload(&ctx, &a).unwrap();
    let survivor = arena.upload(&ctx, &b).unwrap();
    let group = arena.group.clone();
    let buffers = [
        arena.vertices.buf.clone(),
        arena.indices.buf.clone(),
    ];
    let revision = arena.revision;
    assert_eq!(
        (arena.uploads, arena.uploaded_bytes),
        (2, 144)
    );
    assert!(arena.rows.retire(Rc::as_ptr(&a) as usize));
    let c = mesh(-0.45, [0., 0., 1.]);
    assert_eq!(arena.upload(&ctx, &c).unwrap(), first);
    assert_eq!(arena.upload(&ctx, &b).unwrap(), survivor);
    assert_eq!(arena.group, group);
    assert_eq!(arena.revision, revision);
    assert_eq!(
        [
            arena.vertices.buf.clone(),
            arena.indices.buf.clone()
        ],
        buffers
    );
    assert_eq!(
        (arena.uploads, arena.uploaded_bytes),
        (3, 216)
    );
    assert_eq!(history.indices(), &[0, 1, 2]);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &arena.vertices.buf, 120),
        [
            crate::packed_vertex::bytes(&c),
            crate::packed_vertex::bytes(&b)
        ]
        .concat()
    );
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &arena.indices.buf, 24),
        [0u32, 1, 2, 3, 4, 5]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>()
    );
    let image = render(&ctx, &arena, &group);
    let left = (32 * 64 + 16) * 4;
    let right = (32 * 64 + 48) * 4;
    let center = (32 * 64 + 32) * 4;
    assert_eq!(&image[left..left + 4], &[0, 0, 255, 255]);
    assert_eq!(&image[right..right + 4], &[0, 255, 0, 255]);
    let d = mesh(0., [1., 1., 0.]);
    arena.upload(&ctx, &d).unwrap();
    assert_ne!(arena.group, group);
    assert!(arena.revision > revision);
    assert_eq!(
        (arena.uploads, arena.uploaded_bytes),
        (4, 288)
    );
    let fresh = render(&ctx, &arena, &arena.group);
    let stale = render(&ctx, &arena, &group);
    assert_eq!(
        &fresh[center..center + 4],
        &[255, 255, 0, 255]
    );
    assert_eq!(&stale[center..center + 4], &[255; 4]);
    assert_eq!(&stale[left..left + 4], &[0, 0, 255, 255]);
    let before = crate::growing_buffer_tests::read(&ctx, &arena.vertices.buf, 180);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| arena
            .vertices
            .write_or_append(&ctx, u32::MAX, &[0; 20])))
        .is_err()
    );
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &arena.vertices.buf, 180),
        before
    );
    assert!(!arena.vertices.write_or_append(&ctx, 9, &[]));
}
```

## <a id="checkpoint-96zzzzw-reclaim-ranges-after-the-last-gpu-lease"></a>15. Reclaim ranges after the last GPU lease

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh_arena.rs, src/gpu_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Last shared GPU owner dro…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retire source/span</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later geometry reuses the…</text>
</svg>
</div>

**What this code does:** Retire GPU ranges independently of precise CPU history ownership.

### 1. `src/mesh_arena.rs`

Use weak range tokens independently of CPU source ownership. Keep weak range leases under the existing arena owner. Begin without live range leases. Acquire one shared range token and retire only expired GPU leases; synchronize logical GPU lengths after free-tail trimming. Reset lease metadata with the existing explicit arena reset.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        Ok(range)
    }
    pub fn reset(&mut self) {
        self.rows.reset();
```

Replace it with:

```rust
        }
        Ok(range)
    }
    pub fn acquire(
        &mut self,
        ctx: &GpuCtx,
        source: &Rc<Mesh>,
    ) -> Result<Rc<MeshRange>, &'static str> {
        self.prune();
        let key = Rc::as_ptr(source) as usize;
        if let Some(range) = self.leases.get(&key).and_then(Weak::upgrade) {
            return Ok(range);
        }
        let range = Rc::new(self.upload(ctx, source)?);
        self.leases
            .insert(key, Rc::downgrade(&range));
        Ok(range)
    }
    pub fn prune(&mut self) {
        let retired = self
            .leases
            .iter()
            .filter(|(_, range)| range.strong_count() == 0)
            .map(|(&key, _)| key)
            .collect::<Vec<_>>();
        for key in retired {
            self.leases.remove(&key);
            self.rows.retire(key);
        }
        self.vertices
            .truncate((self.rows.vertices.len() / 20) as u32);
        self.indices
            .truncate(self.rows.indices.len() as u32);
    }
    pub fn reset(&mut self) {
        self.leases.clear();
        self.rows.reset();
```

**Locate this block:**

```rust
        let layout = crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group = Self::bind(ctx, &layout, &vertices, &indices);
        Self {
            rows: Rows::default(),
            vertices,
            indices,
```

Replace it with:

```rust
        let layout = crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group = Self::bind(ctx, &layout, &vertices, &indices);
        Self {
            leases: HashMap::new(),
            rows: Rows::default(),
            vertices,
            indices,
```

**Locate this block:**

```rust
};
use std::rc::Rc;

pub struct Arena {
    pub rows: Rows,
    pub vertices: GrowBuf,
    pub indices: GrowBuf,
```

Replace it with:

```rust
};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

pub struct Arena {
    leases: HashMap<usize, Weak<MeshRange>>,
    pub rows: Rows,
    pub vertices: GrowBuf,
    pub indices: GrowBuf,
```

### 2. `src/gpu_geometry.rs`

Keep the range reserved for every shared GPU geometry owner, including direct construction outside the cache. Acquire a range lease before publishing its immutable draw offsets. Hold the range token for the geometry lifetime. Read the same arena buffers/groups through leased storage without changing standalone preview ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                indices,
                ..
            } => vertices.size() + indices.size(),
            Storage::Shared(_) => 0,
        }
```

Replace it with:

```rust
                indices,
                ..
            } => vertices.size() + indices.size(),
            Storage::Shared { .. } => 0,
        }
```

**Locate this block:**

```rust
    fn group(&self) -> wgpu::BindGroup {
        match &self.storage {
            Storage::Owned { group, .. } => group.clone(),
            Storage::Shared(arena) => arena.borrow().group.clone(),
        }
    }
    pub(crate) fn arena(&self) -> Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {
        match &self.storage {
            Storage::Shared(arena) => Some(arena),
            Storage::Owned { .. } => None,
        }
    }
```

Replace it with:

```rust
    fn group(&self) -> wgpu::BindGroup {
        match &self.storage {
            Storage::Owned { group, .. } => group.clone(),
            Storage::Shared { arena, .. } => arena.borrow().group.clone(),
        }
    }
    pub(crate) fn arena(&self) -> Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {
        match &self.storage {
            Storage::Shared { arena, .. } => Some(arena),
            Storage::Owned { .. } => None,
        }
    }
```

**Locate this block:**

```rust
                indices,
                ..
            } => (vertices.clone(), indices.clone()),
            Storage::Shared(arena) => {
                let arena = arena.borrow();
                (
                    arena.vertices.buf.clone(),
```

Replace it with:

```rust
                indices,
                ..
            } => (vertices.clone(), indices.clone()),
            Storage::Shared { arena, .. } => {
                let arena = arena.borrow();
                (
                    arena.vertices.buf.clone(),
```

**Locate this block:**

```rust
        source: Rc<Mesh>,
        arena: Rc<RefCell<crate::mesh_arena::Arena>>,
    ) -> Result<Self, &'static str> {
        let range = arena.borrow_mut().upload(ctx, &source)?;
        Ok(Self {
            source,
            index_count: range.indices.len() as u32,
            index_start: range.indices.start,
            storage: Storage::Shared(arena),
        })
    }
    pub(crate) fn buffers(&self) -> (wgpu::Buffer, wgpu::Buffer) {
```

Replace it with:

```rust
        source: Rc<Mesh>,
        arena: Rc<RefCell<crate::mesh_arena::Arena>>,
    ) -> Result<Self, &'static str> {
        let range = arena.borrow_mut().acquire(ctx, &source)?;
        Ok(Self {
            source,
            index_count: range.indices.len() as u32,
            index_start: range.indices.start,
            storage: Storage::Shared {
                arena,
                _lease: range,
            },
        })
    }
    pub(crate) fn buffers(&self) -> (wgpu::Buffer, wgpu::Buffer) {
```

**Locate this block:**

```rust
    },
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
```

Replace it with:

```rust
    },
    Shared {
        arena: Rc<RefCell<crate::mesh_arena::Arena>>,
        _lease: Rc<crate::arena_rows::MeshRange>,
    },
}

impl GpuGeometry {
```

### 3. `src/geometry_cache.rs`

Capture real arena upload counters before acquisition, including hole reuse. Count actual new geometry uploads and written bytes once instead of inferring them from arena growth. Retire expired leases after scene replacement and retain the existing empty-owner release.

**Locate this existing block:**

```rust
            .clone();
        let (sources, bytes) = {
            let arena = arena.borrow();
            (
                arena.rows.source_count(),
                arena.rows.vertices.len() + arena.rows.indices.len() * 4,
            )
        };
        let geometry = Rc::new(
            GpuGeometry::shared(ctx, Rc::clone(source), arena.clone())
                .expect("Prepared shared mesh ranges"),
        );
        let arena = arena.borrow();
        self.uploads += usize::from(arena.rows.source_count() != sources);
        self.uploaded_bytes +=
            (arena.rows.vertices.len() + arena.rows.indices.len() * 4 - bytes) as u64;
        self.entries
            .insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries
            .retain(|_, geometry| geometry.strong_count() > 0);
        if self.entries.is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .clone();
        let (uploads, bytes) = {
            let arena = arena.borrow();
            (arena.uploads, arena.uploaded_bytes)
        };
        let geometry = Rc::new(
            GpuGeometry::shared(ctx, Rc::clone(source), arena.clone())
                .expect("Prepared shared mesh ranges"),
        );
        let arena = arena.borrow();
        self.uploads += arena.uploads - uploads;
        self.uploaded_bytes += arena.uploaded_bytes - bytes;
        self.entries
            .insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries
            .retain(|_, geometry| geometry.strong_count() > 0);
        if let Some(arena) = &self.arena {
            arena.borrow_mut().prune();
        }
        if self.entries.is_empty() {
```

### 4. `src/mesh_arena_tests.rs`

Supply direct shared-construction aliases, last-owner retirement despite retained CPU history, exact reused offsets/bytes, safe restoration and logical empty release.

**Locate this existing block:**

```rust
    assert!(!arena.vertices.write_or_append(&ctx, 9, &[]));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(!arena.vertices.write_or_append(&ctx, 9, &[]));
}

#[test]
fn actual_geometry_leases_reclaim_only_after_the_last_gpu_owner() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let arena = Rc::new(std::cell::RefCell::new(
        crate::mesh_arena::Arena::new(&ctx),
    ));
    let a = mesh(-0.45, [1., 0., 0.]);
    let b = mesh(0.45, [0., 1., 0.]);
    let history = Rc::clone(&a);
    let first = Rc::new(
        crate::gpu_geometry::GpuGeometry::shared(&ctx, a.clone(), arena.clone())
            .unwrap(),
    );
    let alias = Rc::new(
        crate::gpu_geometry::GpuGeometry::shared(&ctx, a.clone(), arena.clone())
            .unwrap(),
    );
    let survivor = Rc::new(
        crate::gpu_geometry::GpuGeometry::shared(&ctx, b.clone(), arena.clone())
            .unwrap(),
    );
    assert_eq!(arena.borrow().uploads, 2);
    let held = survivor.index_start;
    drop(first);
    arena.borrow_mut().prune();
    assert_eq!(arena.borrow().rows.source_count(), 2);
    drop(alias);
    arena.borrow_mut().prune();
    assert_eq!(arena.borrow().rows.source_count(), 1);
    assert!(Rc::strong_count(&history) > 0);
    let c = mesh(-0.45, [0., 0., 1.]);
    let next = Rc::new(
        crate::gpu_geometry::GpuGeometry::shared(&ctx, c, arena.clone()).unwrap(),
    );
    assert_eq!(next.index_start, 0);
    assert_eq!(survivor.index_start, held);
    assert_eq!(
        (
            arena.borrow().uploads,
            arena.borrow().uploaded_bytes
        ),
        (3, 216)
    );
    let restored = Rc::new(
        crate::gpu_geometry::GpuGeometry::shared(&ctx, history, arena.clone()).unwrap(),
    );
    assert_eq!(restored.index_start, 6);
    assert_eq!(survivor.index_start, 3);
    assert_eq!(
        (
            arena.borrow().uploads,
            arena.borrow().uploaded_bytes
        ),
        (4, 288)
    );
    drop((survivor, next, restored));
    arena.borrow_mut().prune();
    assert_eq!(arena.borrow().rows.source_count(), 0);
    assert!(arena.borrow().vertices.is_empty());
    assert!(arena.borrow().indices.is_empty());
}
```

### 5. `src/global_outline_tests.rs`

Supply actual scene deletion/addition/Undo with retained precise CPU history, reused GPU ranges, stable survivor pixels/owners, exact uploads, original identity and empty release.

**Locate this existing block:**

```rust
    assert!(gpu.tiles.borrow().lookup.is_none());
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(gpu.tiles.borrow().lookup.is_none());
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners()
{
    let left = [
        [-0.8, -0.4, -0.4],
        [-0.4, -0.4, -0.4],
        [-0.6, 0.4, -0.4],
    ];
    let right = [
        [0.4, -0.4, -0.4],
        [0.8, -0.4, -0.4],
        [0.6, 0.4, -0.4],
    ];
    let mut scene = Scene::empty();
    let a = scene
        .insert(PreparedMesh::triangle(left, [0.9, 0.1, 0.1]).unwrap())
        .unwrap();
    let b = scene
        .insert(PreparedMesh::triangle(right, [0.1, 0.8, 0.2]).unwrap())
        .unwrap();
    let source = Rc::clone(&scene.objects()[0].mesh);
    let precise = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut history = crate::history::History::default();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let first = pixels(&gpu);
    let expired = Rc::downgrade(&gpu.meshes[0].geometry);
    let survivor = gpu.meshes[1].geometry.index_start;
    assert_eq!(
        (
            gpu.geometry.uploads,
            gpu.geometry.uploaded_bytes
        ),
        (2, 144)
    );
    history.edit(&mut scene, |scene| {
        assert!(scene.remove(a));
    });
    gpu.set_scene(&scene, None);
    assert!(expired.upgrade().is_none());
    assert!(
        history
            .scenes()
            .any(|saved| saved.contains(a))
    );
    assert!(Rc::strong_count(&source) > 0);
    let mut c = None;
    history.edit(&mut scene, |scene| {
        c = Some(
            scene
                .insert(PreparedMesh::triangle(left, [0.1, 0.3, 0.9]).unwrap())
                .unwrap(),
        );
    });
    let c = c.unwrap();
    gpu.set_scene(&scene, None);
    let reused = gpu
        .meshes
        .iter()
        .find(|mesh| mesh.id == c)
        .unwrap();
    assert_eq!(reused.geometry.index_start, 0);
    assert_eq!(
        gpu.meshes
            .iter()
            .find(|mesh| mesh.id == b)
            .unwrap()
            .geometry
            .index_start,
        survivor
    );
    assert_eq!(
        (
            gpu.geometry.uploads,
            gpu.geometry.uploaded_bytes
        ),
        (3, 216)
    );
    let next = pixels(&gpu);
    let l = (120 * 320 + 64) * 4;
    let r = (120 * 320 + 256) * 4;
    assert!(next[l + 2] > next[l].saturating_add(50));
    assert_eq!(&next[r..r + 4], &first[r..r + 4]);
    for (point, id) in [([-0.6, 0.], c), ([0.6, 0.], b)] {
        gpu.request_pick(point, matrix()).unwrap();
        wait(&gpu);
        assert_eq!(
            gpu.take_pick(matrix())
                .unwrap()
                .unwrap()
                .unwrap()[0],
            id.value()
        );
    }
    assert!(history.undo(&mut scene));
    gpu.set_scene(&scene, None);
    assert!(history.undo(&mut scene));
    gpu.set_scene(&scene, None);
    assert!(Rc::ptr_eq(
        &source,
        &scene
            .objects()
            .iter()
            .find(|row| row.id == a)
            .unwrap()
            .mesh
    ));
    assert_eq!(
        scene
            .objects()
            .iter()
            .find(|row| row.id == a)
            .unwrap()
            .geometry()
            .unwrap()
            .to_proto(),
        precise
    );
    assert_eq!(pixels(&gpu), first);
    assert_eq!(
        (
            gpu.geometry.uploads,
            gpu.geometry.uploaded_bytes
        ),
        (4, 288)
    );
    scene.clear();
    gpu.set_scene(&scene, None);
    pixels(&gpu);
    assert_eq!(gpu.geometry.entries(), 0);
    assert_eq!(gpu.geometry.allocated_bytes(), 0);
    assert_eq!(gpu.usage(), [0; 4]);
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
    assert!(history.scenes().count() > 0);
}
```

## <a id="checkpoint-96zzzzx-retire-empty-arena-ranges-after-tail-trimming"></a>16. Retire empty arena ranges after tail trimming

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/arena_spans.rs, src/are…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retire occupied tail</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">release empty source asso…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">keep the new end</text>
</svg>
</div>

**What this code does:** Release empty source records independently of the current arena extent.

### 1. `src/arena_spans.rs`

Empty ranges reserve no bytes; a previous free-tail trim cannot invalidate their release.

**Locate this existing block:**

```rust
pub fn retire(free: &mut Vec<Range<u32>>, range: Range<u32>, mut end: u32) -> u32 {
    assert!(range.start <= range.end && range.end <= end);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn retire(free: &mut Vec<Range<u32>>, range: Range<u32>, mut end: u32) -> u32 {
    if range.start == range.end {
        return end;
    }
    assert!(range.start <= range.end && range.end <= end);
```

### 2. `src/arena_rows_tests.rs`

Supply both source retirement orders after tail trimming, exact empty endpoints, retained originals, subsequent zero-based reuse and unchanged rejection of reversed/out-of-bounds nonempty ranges.

**Locate this existing block:**

```rust
    assert_eq!(rows.append(&history).unwrap().vertices, 0..3);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(rows.append(&history).unwrap().vertices, 0..3);
}

#[test]
fn empty_sources_release_after_earlier_tail_retirement() {
    for empty_first in [false, true] {
        let occupied = source();
        let empty = Rc::new(crate::mesh::Mesh::new(vec![], vec![]).unwrap());
        let mut rows = crate::arena_rows::Rows::default();
        let full = rows.append(&occupied).unwrap();
        let zero = rows.append(&empty).unwrap();
        assert_eq!(zero.vertices, 3..3);
        assert_eq!(zero.indices, 3..3);
        let keys = [
            Rc::as_ptr(&occupied) as usize,
            Rc::as_ptr(&empty) as usize,
        ];
        let order = if empty_first {
            [1, 0]
        } else {
            [0, 1]
        };
        for i in order {
            assert!(rows.retire(keys[i]));
        }
        assert!(rows.vertices.is_empty());
        assert!(rows.indices.is_empty());
        assert_eq!(rows.source_count(), 0);
        assert_eq!(rows.append(&occupied).unwrap(), full);
        assert_eq!(occupied.indices(), &[0, 1, 2]);
        assert!(empty.vertices().is_empty());
    }
    let mut free = vec![];
    assert_eq!(
        crate::arena_spans::retire(&mut free, 36..36, 0),
        0
    );
    assert!(free.is_empty());
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::arena_spans::retire(&mut free, 2..1, 0)
        }))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::arena_spans::retire(&mut free, 1..2, 0)
        }))
        .is_err()
    );
    assert!(free.is_empty());
}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.6
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.6
npm --prefix ../session_tests run course -- save 20.6
```

Continue with [21.1 · Units and one precise render origin](../../consolidation/96zzzzy-prepare-unit-aware-anchored-camera-space.md).
