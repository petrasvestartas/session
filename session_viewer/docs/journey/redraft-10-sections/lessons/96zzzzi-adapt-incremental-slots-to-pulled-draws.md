# 20.6 · Retained allocation, live lookup and safe reclamation

**Goal:** Release empty source records independently of the current arena extent. · **Steps:** 16 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Adapt incremental slots t…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Retained allocation, live l…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Procedural matrix.wgsl</text>
  <rect x="914" y="54" width="120" height="84" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="914" y="54" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <rect x="954" y="82" width="40" height="28" fill="#f97316"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="100" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+2.4</text>
  <rect x="994" y="54" width="40" height="28" fill="#93c5fd"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">-0.8</text>
  <rect x="994" y="110" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <line x1="954" y1="54" x2="954" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="994" y1="54" x2="994" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="82" x2="1034" y2="82" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="110" x2="1034" y2="110" stroke="#111111" stroke-width="1"/>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">correct physical IDs</text>
</svg>
</div>

**What this code does:** Keep physical triangle bases correct when pulled draws begin at local occurrence zero.

### 1. `src/slot_space.rs`

Adapt physical bases to local pulled occurrences and report only changed member positions to the existing allocator.

**Locate this existing block:**

```rust
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  pub fn local_slot(&self,slot:u32)->Slot {let texel=self.texel(slot);[texel[0],texel[1]]}
  pub fn touched(&self,definitions:&[Definition])->Vec<(u32,u32)> {
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/slot_space.rs`

Supply local/global occurrence equivalence, exact physical IDs, changed-member reporting, reorder and retired-hole checks while retaining all five original allocator fixtures.

**Locate this existing block:**

```text
    /// Instances number their triangles one after another, past the arena's:
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzi-adapt-incremental-slots-to-pulled-draws-02.rs"
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing slot allocator</text>
</svg>
</div>

**What this code does:** Describe allocator members using exact arena ranges and stable object rows.

### 1. `src/instance_members.rs`

Group stable object rows by exact shared arena ranges; reject standalone buffers and mixed arenas instead of narrowing pointers into IDs.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-01.rs"
```

### 2. `src/lib.rs`

Declare the preparatory definition adapter.

**Locate this existing block:**

```rust
pub mod instance_gpu;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_members;
mod instance_gpu;
```

### 3. `src/gpu_geometry.rs`

Expose shared arena identity as a borrowed owner without changing range or allocation ownership.

**Locate this existing block:**

```rust
    pub fn allocated_bytes(&self)->u64{
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/instance_members.rs`

Supply exact range keys, repeated-definition merging, stable rows, original arena count, standalone/mixed-arena refusal and empty checks.

**Locate this existing block:**

```text
use std::{collections::BTreeMap,ops::Range,rc::Rc};
use crate::gpu_geometry::GpuGeometry;
pub struct Members {pub key:u32,pub faces:Range<u32>,pub rows:Vec<u32>}
impl Members {
 pub fn definition(&self)->crate::slot_space::Definition<'_> {
  crate::slot_space::Definition {key:self.key,rows:&self.rows,faces:self.faces.clone(),pipes:0..0,ribbons:0..0}
 }
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
 let mut arena=None;let mut count=0;let mut members=BTreeMap::<u32,Members>::new();
 for(geometry,rows)in sources {
  let current=geometry.arena()?;if arena.is_some_and(|first|!Rc::ptr_eq(first,current)){return None;}arena=Some(current);
  count=u32::try_from(current.borrow().rows.indices.len()/3).ok()?;
  let first=geometry.index_start;let end=first.checked_add(geometry.index_count)?;
  if first%3!=0||geometry.index_count%3!=0{return None;}
  let member=members.entry(first).or_insert_with(||Members {key:first,faces:first..end,rows:Vec::new()});
  if member.faces.end!=end{return None;}member.rows.extend(rows);
 }
 Some((members.into_values().collect(),count))
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-04.rs" skip=1140
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded retained runs</text>
</svg>
</div>

**What this code does:** Update retained source runs from the live stable object rows.

### 1. `src/instance_gpu.rs`

Keep the existing allocator under the surface lane without adding a GPU owner. Begin without retained allocation state. Update retained source runs from exact arena definitions and changed member positions; unsupported or oversized plans retain earlier drawing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
```

Replace it with:

```rust
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
```

**Locate this block:**

```rust
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

Replace it with:

```rust
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

**Locate this block:**

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

Replace it with:

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

### 2. `src/instance_members.rs`

Keep enough integer headroom for retained runs, growth and free intervals before invoking the existing allocator.

**Locate this existing block:**

```rust
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn bounded(members:&[Members],arena:u32)->bool {
 let ids=members.iter().map(|member|member.rows.len()as u64*(member.faces.len()as u64/3)).sum::<u64>();
 u64::from(arena)*2+ids*2+8192*(members.len()as u64+1)<u64::from(crate::primitive_image::CAP_START)
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/instance_members.rs`

Supply small, empty, reserved-arena and overflowing-instance budget checks without allocating large GPU buffers.

**Locate this existing block:**

```text
 #[test]
 fn exact_ranges_merge_members_and_reject_unrelated_storage()
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-05.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply actual retained allocator locations alongside the existing exact GPU comparisons.

**Locate this existing block:**

```text
 let retained=gpu.instances.ordinary.clone();
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-06.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Verify a surviving original retains its allocator slot through removal and insertion.

**Locate this existing block:**

```text
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-07.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Verify complete allocator release with the existing empty-scene GPU release checks.

**Locate this existing block:**

```text
assert!(gpu.instances.ordinary.is_empty());
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-08.rs" skip=43
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local draw-slot bytes and…</text>
</svg>
</div>

**What this code does:** Publish one retained plan through the existing draw-slot and integer-texture owners.

### 1. `src/draw_slots.rs`

Publish one retained plan through both owners. Merge slot boundary changes with allocator reports and refresh the lookup header after shrinking; reuse the existing range coalescer.

**Locate this existing block:**

```rust
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change,table:&mut crate::primitive_slot_table::Table)->bool {
  let rows=(1..space.end()).map(|slot|space.local_slot(slot)).collect::<Vec<_>>();
  assert!(change.slots.iter().all(|range|range.start<=range.end&&range.end<=space.end()));
  let mut ranges=change.slots.clone();ranges.extend(changed_runs(&self.previous,&rows).into_iter().map(|run|(run.start+1)as u32..(run.end+1)as u32));
  if rows.len()!=self.previous.len(){ranges.push(0..1);}
  let change=crate::slot_space::Change {all:change.all,slots:crate::slot_space::coalesce(ranges,space.end())};
  if rows.is_empty()&&self.instances()>0{self.clear(ctx,true);}else{self.set(ctx,&rows);}
  table.publish(ctx,space,&change)
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

### 2. `src/slot_space.rs`

Expose the range coalescer already written so publication shares its bounds and merging instead of duplicating it.

**Locate this existing block:**

```rust
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_slot_table_tests.rs`

Supply exact mapped writes and actual lookup shader results through high IDs, hole splitting/reuse, unchanged neighbours, shrinking headers, arena following and release.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-02.rs"
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measured allocation parts</text>
</svg>
</div>

**What this code does:** Prepare synchronized mesh and record updates without changing primitive addresses.

### 1. `src/instance_gpu.rs`

Pass the live meshes to their record owner so the next connection can synchronize addresses before preparing consumers. Keep the same instance membership while accepting the complete mesh slice. Retain the existing ordinary-row capacity fallback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
```

Replace it with:

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,_geometry_changed:bool)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
```

### 2. `src/renderer.rs`

Distinguish source membership updates from style changes before preparing projection. Keep style-only updates distinguishable so explicit primitive addresses remain intact.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

Replace it with:

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

**Locate this block:**

```rust
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
```

Replace it with:

```rust
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
```

### 3. `src/instance_gpu.rs`

Expose the existing allocation owners individually for the upcoming sparse allocation check. Derive the total from the same measured allocations instead of repeating the accounting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}

#[cfg(test)]
```

Replace it with:

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
```

### 4. `src/renderer.rs`

Make the two existing GPU allocation parts available through renderer diagnostics.

**Locate this existing block:**

```rust
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
```

After that block, type:

```rust
    pub fn instance_allocations(&self)->[u64;2] {self.instances.allocation_parts()}
```

### 5. `src/browser.rs`

Expose measured allocation parts beside the existing browser GPU diagnostic.

**Locate this existing block:**

```rust
    canvas.set_attribute("data-instance-gpu",&serde_json::to_string(&renderer.instance_usage()).unwrap())?;
```

After that block, type:

```rust
    canvas.set_attribute("data-instance-allocations",&serde_json::to_string(&renderer.instance_allocations()).unwrap())?;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Supply exact allocation parts alongside the unchanged two-definition GPU pixels, slots, selection and release checks.

**Locate this existing block:**

```text
assert_eq!(&gpu.instance_usage()[..3],&[2,2,376]);
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-10.rs" skip=50
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coherent sparse GPU slots…</text>
</svg>
</div>

**What this code does:** Draw retained sparse slots with synchronized original uniforms and shared records.

### 1. `src/instance_gpu.rs`

Retain the inverse lookup under the same live surface owner. Allocate the lookup only when a complete shared plan is active. Use source versus style updates to preserve explicit primitive addresses. Keep style-only explicit address overrides on the earlier path. Activate inverse lookup only for a complete supported shared table. Retain the allocator change report for the GPU writers. Preserve existing fallback behavior when the complete plan cannot be used. Publish the same retained change that assigned physical intervals. Synchronize final mesh uniforms and shared records before any consumer, using sparse locations or unchanged legacy addresses. Use the existing coherent GPU publication, with full resource release on the legacy or empty path.

Replace the entire file with:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let complete=own.len()<=limit-count;let own=if complete {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut change=None;let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|complete&&compatible&&crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);change=Some(space.update(&definitions,&touched,arena));
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        if let Some(space)=&self.space {let locations=space.draws().into_iter().flat_map(|draw|draw.slots).map(|slot|{let[row,base]=space.local_slot(slot);(row,(slot,base))}).collect::<std::collections::HashMap<_,_>>();for(first,row)in ordinary.values_mut(){*first=locations[&*row].0;}for batch in batches.values_mut(){batch.first_slot=locations[&all_slots[(batch.first_slot-1)as usize][0]].0;}}
        let mut dense=0u32;for mesh in meshes {let entry=ordinary.get(&mesh.id);let base=if let(Some(space),Some((first,_)))=(&self.space,entry){space.local_slot(*first)[1]}else if geometry_changed {dense}else{mesh.primitive_base()};mesh.set_primitive_base(queue,base)?;if let Some((first,row))=entry {all_records[*row as usize]=mesh.packed();if self.space.is_none(){all_slots[(*first-1)as usize]=[*row,base];}}dense=dense.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).ok_or("Primitive range overlaps reserved caps")?;}
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if let(Some(space),Some(change))=(&self.space,&change){let table=self.lookup.get_or_insert_with(||crate::primitive_slot_table::Table::new(&ctx));self.slots.publish(&ctx,space,change,table);let bytes=table.written_bytes.get();if bytes>0{self.writes+=1;self.written_bytes+=bytes;}}else{self.lookup=None;if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

### 2. `src/primitive_slot_table.rs`

Measure actual lookup upload bytes, including padded full writes. Begin with zero lookup upload bytes. Count only validated texture writes. Measure each publication independently.

Replace the entire file with:

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH:u32=1024;
pub struct Table {pub written_bytes:std::cell::Cell<u64>,pub(crate) texture:wgpu::Texture,pub view:wgpu::TextureView}
fn dimensions(count:u32)->(u32,u32){if count<=1{(1,1)}else{(WIDTH,count.div_ceil(WIDTH))}}
fn texture(ctx:&GpuCtx,width:u32,height:u32)->wgpu::Texture{
 ctx.device.create_texture(&wgpu::TextureDescriptor {label:Some("primitive slot lookup"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba32Uint,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
impl Table {
 pub fn new(ctx:&GpuCtx)->Self{let texture=texture(ctx,1,1);let view=texture.create_view(&Default::default());Self {written_bytes:Default::default(),texture,view}}
 pub fn write(&mut self,ctx:&GpuCtx,texels:&[[u32;4]]){
  let count=u32::try_from(texels.len()).expect("Bounded slot table");let(width,height)=dimensions(count);
  assert!(height<=ctx.device.limits().max_texture_dimension_2d);
  if self.texture.width()!=width||self.texture.height()!=height{self.texture=texture(ctx,width,height);self.view=self.texture.create_view(&Default::default());}
  let mut data=texels.to_vec();data.resize((width*height)as usize,[0;4]);self.write_at(ctx,0,&data);
 }
 pub fn write_at(&self,ctx:&GpuCtx,first:u32,texels:&[[u32;4]])->bool{
  let width=self.texture.width();if u64::from(first)+texels.len()as u64>u64::from(width)*u64::from(self.texture.height()){return false;}
  self.written_bytes.set(self.written_bytes.get()+texels.len()as u64*16);
  let mut at=first;let mut rest=texels;
  while !rest.is_empty(){
   let(x,y)=(at%width,at/width);let(row,tail)=rest.split_at(((width-x)as usize).min(rest.len()));let bytes:Vec<u8>=row.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
   ctx.queue.write_texture(wgpu::TexelCopyTextureInfo {origin:wgpu::Origin3d {x,y,z:0},..self.texture.as_image_copy()},&bytes,wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(row.len()as u32*16),rows_per_image:Some(1)},wgpu::Extent3d {width:row.len()as u32,height:1,depth_or_array_layers:1});
   at+=row.len()as u32;rest=tail;
  }true
 }
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change)->bool {
  self.written_bytes.set(0);let previous=self.texture.clone();let (width,height)=dimensions(space.end());
  if change.all||self.texture.width()!=width||self.texture.height()!=height {
   let texels=(0..space.end()).map(|slot|space.texel(slot)).collect::<Vec<_>>();self.write(ctx,&texels);
  }else {
   for range in &change.slots {
    assert!(range.start<=range.end&&range.end<=space.end());
    let texels=range.clone().map(|slot|space.texel(slot)).collect::<Vec<_>>();assert!(self.write_at(ctx,range.start,&texels));
   }
  }
  self.texture!=previous
 }
 pub fn allocated_bytes(&self)->u64{u64::from(self.texture.width())*u64::from(self.texture.height())*16}
}
```

### 3. `src/instance_gpu.rs`

Include the actual inverse lookup allocation in the same accounting. Count lookup bytes once alongside existing record and slot buffers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}
```

Replace it with:

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;3] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.lookup.as_ref().map_or(0,|table|table.allocated_bytes())]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}
```

### 4. `src/renderer.rs`

Expose all three measured allocation owners. The lane now synchronizes addresses once before record upload and projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;2] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

Replace it with:

```rust
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

**Locate this block:**

```rust
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let mut base=0u32;
        for mesh in &mut self.meshes{
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
```

Replace it with:

```rust
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Include the actual one-row 1024-texel lookup in exact two-definition allocation accounting.

**Locate this existing block:**

```text
assert_eq!(&gpu.instance_usage()[..3],&[2,2,376]);assert_eq!(gpu.instance_allocations(),[352,24]);
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-19.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Keep the original projection relocation fixture explicitly based at zero, preserving every gap, byte, shrink and fallback assertion.

**Locate this existing block:**

```text
let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-20.rs"
```

### 7. `src/slot_space.rs`

Report newly active retained members even when their row number is unchanged; inverse lookup counts must refresh after hide/show.

**Locate this existing block:**

```rust
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&(position>=run.len as usize||self.rows.get(run.start as usize+position)!=Some(row)){out.push((definition.key,position as u32));}}}out
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/pick_gpu_tests.rs`

Retain all original copied/placed/nested owner, pixel, history and release assertions while adding the actual padded 16384-byte inverse lookup to exact allocation totals.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-22.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/primitive_slot_table_tests.rs`

Supply actual lookup shader readback for inactive and reactivated retained rows, while checking unchanged draw-slot bytes and all earlier reuse, shrink, high-ID and release assertions.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-23.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

Retain all original owner, background, selection, overlay, crop, density and reserved-ID guards while testing placed physical intervals after the arena range; compare later placements with their assigned non-overlapping object records.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![0,1]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],1);assert_eq!(before[120*320+240],2);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(placed[120*320+64],1);assert_eq!(placed[120*320+256],2);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,1u32),([0.5,0.],b,2)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],1,"Visible intervals must rebuild without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-24.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/tile_gpu_tests.rs`

Read the first placed projected polygon after the two original demo arena triangles; retain exact clipping-corner, tile-depth, list bounds, cache and release checks.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-25.rs"
```

### 12. `src/instance_gpu.rs`

Assign new record rows in original-ID order instead of geometry pointer order; retained rows and the original scene draw order remain unchanged.

**Locate this existing block:**

```rust
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged projected recor…</text>
</svg>
</div>

**What this code does:** Make source addressing explicit before projection consumes the inverse lookup.

### 1. `src/finite_project.wgsl`

Pass the model row and index offset explicitly instead of closing over one dispatch interval. Read exactly the requested shared record or the unchanged uniform object. Keep arena addressing separate from object-row selection. Let the same projection algorithm accept either local or global arena triangles. Use the same explicit address contract for every triangle corner. Preserve the current local draw behavior while preparing lookup-selected arena inputs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
```

Replace it with:

```wgsl
        return;
    }

    let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
```

**Locate this block:**

```wgsl

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
```

Replace it with:

```wgsl

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base,row,first), physical_clip_corner(base+1u,row,first), physical_clip_corner(base+2u,row,first));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
```

**Locate this block:**

```wgsl
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
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
```

Replace it with:

```wgsl
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared projection</text>
</svg>
</div>

**What this code does:** Resolve physical primitive IDs to their shared source triangle and object row.

### 1. `src/finite_project_source.rs`

Assemble lookup-selected projection from the existing shader and unsigned lookup helper; keep one clipping and projection algorithm.

**Locate this existing block:**

```rust
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn lookup()->String {
 let source=records().replace("let index = local + live_count.y;","let index = local;").replace("let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);","let source=placed_triangle(index+1u,live_count.y);if(source.x==NO_TRIANGLE||source.y==SLOT_OWN_ROW){return;}let polygon=project_physical_triangle(source.x+1u,source.y,0u);");
 format!("@group(2) @binding(5) var slot_table:texture_2d<u32>;\n{}\n{}",include_str!("primitive_slot_lookup.wgsl"),source)
}

#[cfg(test)]
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/finite_project_source.rs`

Supply validation of uniform, shared per-mesh and lookup-selected compute interfaces.

**Locate this existing block:**

```text
[super::source(),super::records()]
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzp-prepare-lookup-selected-triangle-projection-02.rs"
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared projection pipel…</text>
</svg>
</div>

**What this code does:** Add the inverse lookup texture to the existing compute binding recipe.

### 1. `src/tile_pipelines.rs`

Keep the lookup projection layout beside the two existing resource contracts. Own the lookup state with the existing tile pipelines. Extend the existing binding recipe with one unsigned texture input instead of duplicating the compute descriptor. Select the shared projection algorithm assembled with the inverse lookup. Prepare uniform, shared per-mesh and lookup states through one constructor. Retain the new layout under the existing pipeline lifetime. Retain the prepared compute state without changing dispatches yet.

Replace the entire file with:

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub record_project_layout:wgpu::BindGroupLayout,pub lookup_project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub record_project:wgpu::ComputePipeline,pub lookup_project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let create=|shared,lookup| {let mut entries=vec![entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,if shared {Type::Storage{read_only:true}}else{Type::Uniform}),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)];if lookup{entries.push(crate::text_gpu_support::texture_entry(5,Stage::COMPUTE,wgpu::TextureSampleType::Uint,false));}let project_layout=layout("finite original projection",&entries);
   let source=if lookup {crate::finite_project_source::lookup()}else if shared {crate::finite_project_source::records()}else{crate::finite_project_source::source()};let project=compute(device,&source,"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);(project_layout,project)};
  let(project_layout,project)=create(false,false);let(record_project_layout,record_project)=create(true,false);let(lookup_project_layout,lookup_project)=create(true,true);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,record_project_layout,lookup_project_layout,raster_layout,scan_layout,project,record_project,lookup_project,count,fill,scans}
 }
}
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">output-aware immutable bi…</text>
</svg>
</div>

**What this code does:** Keep current shared inputs and replace immutable projection bindings when needed.

### 1. `src/projection_lookup.rs`

Own immutable current arena/record/lookup bindings and one bounded dispatch interval; replace the group when the output buffer changes.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzr-own-current-lookup-projection-bindings-01.rs"
```

### 2. `src/lib.rs`

Declare the binding owner without changing live projection dispatch.

**Locate this existing block:**

```rust
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod projection_lookup;
mod finite_project_source;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply actual mapped dispatch words, exact immutable-group reuse and replacement, missing/current-record refusal and empty checks.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzr-own-current-lookup-projection-bindings-03.rs" skip=34426
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing tile bins</text>
</svg>
</div>

**What this code does:** Resolve shared source triangles and object rows on the GPU for live projection.

### 1. `src/projection_lookup.rs`

Release cached output references when tile resources are replaced or released.

**Locate this existing block:**

```rust
 pub fn bytes(&self)->u64 {self.parameters.size()}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn invalidate(&mut self){self.group=None;}
 pub fn bytes(&self)->u64 {self.parameters.size()}
```

### 2. `src/tile_gpu.rs`

Own the current global lookup binding state under the existing tile lifetime. Begin with the existing per-mesh fallback. Drop global inputs when the physical projection interval exceeds the device budget. Include inverse lookup identity in the existing source/model witness without invalidating projection for selection-only bytes. Allocate one global dispatch interval, or retain the existing guarded per-mesh intervals. Keep one active projection resource path. Release stale immutable output groups during resize and hidden-viewport release. Resolve all live physical IDs through the lookup in one dispatch, preserving the earlier projection algorithm and fallback. Count only the active interval owner; geometry, object and lookup allocations remain counted by their existing owners.

Replace the entire file with:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32,u32,u32,Option<u32>);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,shared:bool,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub(crate) lookup:Option<crate::projection_lookup::Bindings>,pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{lookup:None,projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){self.set_records(device,meshes,None)}
 pub fn set_records(&mut self,device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.lookup=None;self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let current=|mesh:&GpuMesh|records.and_then(|rows|rows.current_row(mesh)).map(|(_,row)|row);
  let model=|mesh:&GpuMesh|if current(mesh).is_some(){records.unwrap().record_buffer().clone()}else{mesh.model_buffer().clone()};
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,model(row),row.packed()[..64].try_into().unwrap(),row.primitive_base(),row.geometry.index_start,row.geometry.index_count,current(row))}).collect::<Vec<_>>();
  let lookup=crate::projection_lookup::Bindings::source(meshes,records);let lookup_view=lookup.and_then(|records|records.lookup.as_ref()).map(|table|&table.view);
  if witness==self.witness&&self.lookup.as_ref().map(|state|&state.table)==lookup_view{return}
  self.rows.clear();self.lookup=crate::projection_lookup::Bindings::new(device,meshes,lookup,count.unwrap());
  if self.lookup.is_none(){for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),current(row).unwrap_or(0),0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:model(row),count,triangles,shared:current(row).is_some(),group:None});
  }
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}if let Some(lookup)=&mut self.lookup{lookup.invalidate();}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2]){
  if self.triangles==0||size.contains(&0){self.release(device);return}
  let layout=TileLayout::new((size[0],size[1]));let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let report=self.report.poll().filter(|_|self.layout==Some(layout));let capacity=self.buffer.size()/4;
  let ceiling=layout.max_pool_words().min(limit.saturating_sub(layout.header_records()*16)/4);
  let pool=next_pool_words(self.pool,layout.initial_pool_words(self.triangles),capacity,report,ceiling);
  if pool<layout.initial_pool_words(self.triangles)||layout.header_records()*16>limit{self.fallback=true;self.release(device);return}
  let mut moved=false;
  if let Some(records)=projected_records(u64::from(self.triangles),self.projected.size()/PROJECTED_BYTES,limit/PROJECTED_BYTES){self.projected=buffer(device,"finite projected triangles",records*PROJECTED_BYTES);moved=true;}
  if self.layout!=Some(layout)||pool>self.pool{self.buffer=buffer(device,"finite triangle tile lists",layout.buffer_bytes(pool));self.pool=pool;moved=true;}
  if self.layout!=Some(layout){self.target=Some(device.create_texture(&wgpu::TextureDescriptor{label:Some("finite tile raster"),size:wgpu::Extent3d{width:layout.dimensions()[0],height:layout.dimensions()[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::R8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]}).create_view(&Default::default()));}
  if self.viewport!=size{let bytes=[0.,0.,0.,size[1]as f32,size[0]as f32,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.line,0,&bytes);self.viewport=size;self.key=None;}
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,view:&wgpu::BindGroup,matrix:[f32;16]){
  let Some(layout)=self.layout else{return};if self.key==Some(matrix){if self.reported!=Some(self.bins)&&self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}return}
  encoder.clear_buffer(&self.projected,0,None);
  if let Some(lookup)=&mut self.lookup {let bindings=lookup.group(device,&self.pipes.lookup_project_layout,&self.projected);let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("shared lookup triangle projection"),timestamp_writes:None});pass.set_pipeline(&self.pipes.lookup_project);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&bindings,&[]);let(x,y)=project_groups(self.triangles);pass.dispatch_workgroups(x,y,1);}else{for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,if row.shared {&self.pipes.record_project_layout}else{&self.pipes.project_layout},&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(if row.shared {&self.pipes.record_project}else{&self.pipes.project});pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64+self.lookup.as_ref().map_or(0,|state|state.bytes()),self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

### 3. `src/renderer.rs`

Observe explicit address overrides after style changes while the unchanged witness retains cached projections for ordinary selection.

**Locate this existing block:**

```rust
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
```

After that block, type:

```rust
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply exact global/per-mesh projected bytes and pixels across distinct and copied definitions, selection cache reuse, unchanged geometry uploads and full release.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-12.rs" skip=36256
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">take after both ranges va…</text>
</svg>
</div>

**What this code does:** Prepare bounded hole reuse without moving any surviving geometry.

### 1. `src/arena_spans.rs`

Plan reusable spans before committing either geometry range, and reuse the existing coalescer for retirement.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-01.rs"
```

### 2. `src/lib.rs`

Declare the complete preparatory span helper.

**Locate this existing block:**

```rust
pub mod arena_rows;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod arena_spans;
mod arena_rows;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/arena_spans.rs`

Supply hole reuse, unchanged preflight state, overflow refusal, zero spans, coalescing and free-tail checks.

**Locate this existing block:**

```text
use std::ops::Range;
pub struct Plan {pub range:Range<u32>,at:Option<usize>}
pub fn plan(free:&[Range<u32>],count:u32,end:u32)->Option<Plan> {
 if count>0 {if let Some(at)=free.iter().position(|span|span.end-span.start>=count){let first=free[at].start;return Some(Plan {range:first..first+count,at:Some(at)});}}
 Some(Plan {range:end..end.checked_add(count)?,at:None})
}
impl Plan {
 pub fn take(self,free:&mut Vec<Range<u32>>)->Range<u32> {
  if let Some(at)=self.at {free[at].start=self.range.end;if free[at].is_empty(){free.remove(at);}}
  self.range
 }
}
pub fn retire(free:&mut Vec<Range<u32>>,range:Range<u32>,mut end:u32)->u32 {
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
 *free=crate::slot_space::coalesce(std::mem::take(free),end);
 while free.last().is_some_and(|span|span.end==end){end=free.pop().unwrap().start;}end
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-03.rs" skip=854
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current packed records</text>
</svg>
</div>

**What this code does:** Connect bounded span plans to CPU arena records while preserving live offsets.

### 1. `src/arena_rows.rs`

Keep free vertex and index spans under the existing arena record owner. Validate both ranges and rebased addresses before taking spans. Retire the old source mapping and trim only a free tail; surviving addresses remain fixed. Reset free-span metadata with the existing logical arena reset.

Replace the entire file with:

```rust
use crate::mesh::Mesh;
use std::{collections::HashMap,ops::Range,rc::{Rc,Weak}};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MeshRange {pub vertices:Range<u32>,pub indices:Range<u32>}
#[derive(Default)]
pub struct Rows {
    pub vertices:Vec<u8>,pub indices:Vec<u32>,
    sources:HashMap<usize,(Weak<Mesh>,MeshRange)>,
    free_vertices:Vec<Range<u32>>,free_indices:Vec<Range<u32>>,
}
impl Rows {
    pub fn append(&mut self,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let key=Rc::as_ptr(source) as usize;
        if let Some((weak,range))=self.sources.get(&key) {
            if weak.upgrade().is_some_and(|saved|Rc::ptr_eq(&saved,source)){return Ok(range.clone());}
        }
        let vertex_count=u32::try_from(source.vertices().len()).map_err(|_|"Arena vertex count")?;
        let index_count=u32::try_from(source.indices().len()).map_err(|_|"Arena index count")?;
        let vertex_end=u32::try_from(self.vertices.len()/20).map_err(|_|"Arena vertex range")?;
        let index_end=u32::try_from(self.indices.len()).map_err(|_|"Arena index range")?;
        let vertices=crate::arena_spans::plan(&self.free_vertices,vertex_count,vertex_end).ok_or("Arena vertex range")?;
        let indices=crate::arena_spans::plan(&self.free_indices,index_count,index_end).ok_or("Arena index range")?;
        let byte_end=usize::try_from(u64::from(vertices.range.end)*20).map_err(|_|"Arena vertex bytes")?;
        let rebased=source.indices().iter().map(|i|i.checked_add(vertices.range.start).ok_or("Arena index address")).collect::<Result<Vec<_>,_>>()?;
        let bytes=crate::packed_vertex::bytes(source);
        let range=MeshRange {vertices:vertices.take(&mut self.free_vertices),indices:indices.take(&mut self.free_indices)};
        self.vertices.resize(self.vertices.len().max(byte_end),0);self.indices.resize(self.indices.len().max(range.indices.end as usize),0);
        self.vertices[range.vertices.start as usize*20..byte_end].copy_from_slice(&bytes);
        self.indices[range.indices.start as usize..range.indices.end as usize].copy_from_slice(&rebased);
        self.sources.insert(key,(Rc::downgrade(source),range.clone()));Ok(range)
    }
    pub(crate) fn retire(&mut self,key:usize)->bool {
        let Some((_,range))=self.sources.remove(&key)else{return false};
        let vertices=crate::arena_spans::retire(&mut self.free_vertices,range.vertices,(self.vertices.len()/20)as u32);
        let indices=crate::arena_spans::retire(&mut self.free_indices,range.indices,self.indices.len()as u32);
        self.vertices.truncate(vertices as usize*20);self.indices.truncate(indices as usize);true
    }
    pub fn index_bytes(&self)->Vec<u8>{self.index_bytes_from(0)}
    pub fn index_bytes_from(&self,start:usize)->Vec<u8>{self.indices[start..].iter().flat_map(|i|i.to_le_bytes()).collect()}
    pub fn source_count(&self)->usize{self.sources.len()}
    pub fn reset(&mut self){self.vertices.clear();self.indices.clear();self.sources.clear();self.free_vertices.clear();self.free_indices.clear();}
    pub fn release(&mut self){*self=Self::default();}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply exact packed bytes, rebased indices, surviving offsets, retained CPU history, safe source restoration and coalesced tail retirement.

**Locate this existing block:**

```text
use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzu-reuse-retired-arena-record-ranges-04.rs" skip=1687
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">exact upload totals</text>
</svg>
</div>

**What this code does:** Write reused geometry ranges without replacing unchanged GPU buffers.

### 1. `src/text_gpu_support.rs`

Use the same device-limit calculation for append and bounded overwrite operations. Validate alignment, start and complete end before any write, then reuse existing bounded overwrite and growth operations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }),stride,len:0,label,usage}
    }

    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let limits=ctx.device.limits();
        let limit=if self.usage.contains(wgpu::BufferUsages::STORAGE){
            limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))
        }else{limits.max_buffer_size};
        let most=limit/self.stride;assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

Replace it with:

```rust
        }),stride,len:0,label,usage}
    }

    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let most=self.max_rows(ctx);assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

### 2. `src/mesh_arena.rs`

Measure real new geometry uploads and written source bytes under the arena owner. Start upload counters with the arena. Write exact reused ranges and count real writes; preserve current groups unless a buffer allocation changes.

Replace the entire file with:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/mesh_arena_tests.rs`

Supply mapped reused bytes, exact upload totals, retained CPU source, real blue/green/yellow pixels, old/fresh immutable bindings and refused-write preservation.

**Locate this existing block:**

```text
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-06.rs" skip=5276
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later geometry reuses the…</text>
</svg>
</div>

**What this code does:** Retire GPU ranges independently of precise CPU history ownership.

### 1. `src/mesh_arena.rs`

Use weak range tokens independently of CPU source ownership. Keep weak range leases under the existing arena owner. Begin without live range leases. Acquire one shared range token and retire only expired GPU leases; synchronize logical GPU lengths after free-tail trimming. Reset lease metadata with the existing explicit arena reset.

Replace the entire file with:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::{collections::HashMap,rc::{Rc,Weak}};

pub struct Arena {
    leases:HashMap<usize,Weak<MeshRange>>,
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {leases:HashMap::new(),rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn acquire(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<Rc<MeshRange>,&'static str>{
        self.prune();let key=Rc::as_ptr(source)as usize;
        if let Some(range)=self.leases.get(&key).and_then(Weak::upgrade){return Ok(range);}
        let range=Rc::new(self.upload(ctx,source)?);self.leases.insert(key,Rc::downgrade(&range));Ok(range)
    }
    pub fn prune(&mut self){
        let retired=self.leases.iter().filter(|(_,range)|range.strong_count()==0).map(|(&key,_)|key).collect::<Vec<_>>();
        for key in retired {self.leases.remove(&key);self.rows.retire(key);}
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
    }
    pub fn reset(&mut self){self.leases.clear();self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

### 2. `src/gpu_geometry.rs`

Keep the range reserved for every shared GPU geometry owner, including direct construction outside the cache. Acquire a range lease before publishing its immutable draw offsets. Hold the range token for the geometry lifetime. Read the same arena buffers/groups through leased storage without changing standalone preview ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().upload(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared(arena)})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared(arena)=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared(arena)=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared(_)=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

Replace it with:

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

**Locate this block:**

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
```

Replace it with:

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
}

impl GpuGeometry {
```

### 3. `src/geometry_cache.rs`

Capture real arena upload counters before acquisition, including hole reuse. Count actual new geometry uploads and written bytes once instead of inferring them from arena growth. Retire expired leases after scene replacement and retain the existing empty-owner release.

Replace the entire file with:

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (uploads,bytes)={let arena=arena.borrow();(arena.uploads,arena.uploaded_bytes)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=arena.uploads-uploads;
        self.uploaded_bytes+=arena.uploaded_bytes-bytes;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if let Some(arena)=&self.arena {arena.borrow_mut().prune();}
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/mesh_arena_tests.rs`

Supply direct shared-construction aliases, last-owner retirement despite retained CPU history, exact reused offsets/bytes, safe restoration and logical empty release.

**Locate this existing block:**

```text
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}

#[test]
fn actual_gpu_arena_reuses_ranges_preserves_bindings_and_counts_exact_writes(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let history=Rc::clone(&a);let first=arena.upload(&ctx,&a).unwrap();let survivor=arena.upload(&ctx,&b).unwrap();let group=arena.group.clone();let buffers=[arena.vertices.buf.clone(),arena.indices.buf.clone()];let revision=arena.revision;assert_eq!((arena.uploads,arena.uploaded_bytes),(2,144));
 assert!(arena.rows.retire(Rc::as_ptr(&a)as usize));let c=mesh(-0.45,[0.,0.,1.]);assert_eq!(arena.upload(&ctx,&c).unwrap(),first);assert_eq!(arena.upload(&ctx,&b).unwrap(),survivor);assert_eq!(arena.group,group);assert_eq!(arena.revision,revision);assert_eq!([arena.vertices.buf.clone(),arena.indices.buf.clone()],buffers);assert_eq!((arena.uploads,arena.uploaded_bytes),(3,216));assert_eq!(history.indices(),&[0,1,2]);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&c),crate::packed_vertex::bytes(&b)].concat());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());let image=render(&ctx,&arena,&group);let left=(32*64+16)*4;let right=(32*64+48)*4;let center=(32*64+32)*4;assert_eq!(&image[left..left+4],&[0,0,255,255]);assert_eq!(&image[right..right+4],&[0,255,0,255]);
 let d=mesh(0.,[1.,1.,0.]);arena.upload(&ctx,&d).unwrap();assert_ne!(arena.group,group);assert!(arena.revision>revision);assert_eq!((arena.uploads,arena.uploaded_bytes),(4,288));let fresh=render(&ctx,&arena,&arena.group);let stale=render(&ctx,&arena,&group);assert_eq!(&fresh[center..center+4],&[255,255,0,255]);assert_eq!(&stale[center..center+4],&[255;4]);assert_eq!(&stale[left..left+4],&[0,0,255,255]);
 let before=crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||arena.vertices.write_or_append(&ctx,u32::MAX,&[0;20]))).is_err());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180),before);assert!(!arena.vertices.write_or_append(&ctx,9,&[]));
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-13.rs" skip=7560
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Supply actual scene deletion/addition/Undo with retained precise CPU history, reused GPU ranges, stable survivor pixels/owners, exact uploads, original identity and empty release.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-14.rs" skip=37938
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">keep the new end</text>
</svg>
</div>

**What this code does:** Release empty source records independently of the current arena extent.

### 1. `src/arena_spans.rs`

Empty ranges reserve no bytes; a previous free-tail trim cannot invalidate their release.

**Locate this existing block:**

```rust
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 if range.start==range.end{return end;}
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply both source retirement orders after tail trimming, exact empty endpoints, retained originals, subsequent zero-based reuse and unchanged rejection of reversed/out-of-bounds nonempty ranges.

**Locate this existing block:**

```text
use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}

#[test]
fn retired_ranges_reuse_holes_without_pinning_or_moving_surviving_sources() {
 let a=source();let b=source();let history=Rc::clone(&a);let weak=Rc::downgrade(&a);let mut rows=crate::arena_rows::Rows::default();let first=rows.append(&a).unwrap();let survivor=rows.append(&b).unwrap();let held=rows.vertices[60..120].to_vec();
 assert!(!rows.retire(usize::MAX));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert!(weak.upgrade().is_some());assert_eq!(rows.source_count(),1);
 let c=Rc::new((*source()).clone().with_alphas(vec![0.5;3]).unwrap());let reused=rows.append(&c).unwrap();assert_eq!(reused,first);assert_eq!(rows.append(&b).unwrap(),survivor);assert_eq!(rows.vertices.len(),120);assert_eq!(&rows.vertices[..60],crate::packed_vertex::bytes(&c));assert_eq!(&rows.vertices[60..120],held);assert_eq!(rows.indices,[0,1,2,3,4,5]);
 let restored=rows.append(&history).unwrap();assert_eq!(restored.vertices,6..9);assert_eq!(restored.indices,6..9);assert_ne!(restored,reused);assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);assert_eq!(c.indices(),&[0,1,2]);
 assert!(rows.retire(Rc::as_ptr(&c)as usize));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert_eq!(rows.vertices.len(),120);assert_eq!(rows.append(&b).unwrap(),survivor);assert!(rows.retire(Rc::as_ptr(&b)as usize));assert!(rows.vertices.is_empty());assert!(rows.indices.is_empty());assert_eq!(rows.source_count(),0);assert_eq!(rows.append(&history).unwrap().vertices,0..3);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzx-retire-empty-arena-ranges-after-tail-trimming-02.rs" skip=3150
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
