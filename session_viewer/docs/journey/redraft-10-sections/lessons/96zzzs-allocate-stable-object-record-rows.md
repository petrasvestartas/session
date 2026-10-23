# 20.5 · Original picks, ambient metadata and cap consumers

**Goal:** Connect both cap crossing passes to current records and retain original owners. · **Checkpoints:** `96zzzs-allocate-stable-object-record-rows` → `96zzzzh-pick-caps-through-shared-records` (16 steps) · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`session_viewer/src/lib.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">96zzzs-allocate-stable-object-record-rows -&gt; 96zzzzh-pick-caps-through-shared-records</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Allocate stable object re…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original picks, ambient met…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Pick caps through shared…</text>
</svg>
</div>

## <a id="checkpoint-96zzzs-allocate-stable-object-record-rows"></a>Step 1 of 16 · Allocate stable object record rows (`96zzzs-allocate-stable-object-record-rows`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/16: 96zzzs-allocate-s…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained rows and reused…</text>
</svg>
</div>

**What this code does:** Prepare bounded row reuse without moving surviving originals.

### 1. `src/instance_gpu.rs`

Retain surviving original IDs, reuse the lowest free row and enforce the shared device bound.

**Locate this existing block:**

```rust
pub(crate) fn source()->String {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/instance_gpu.rs`

Supply original-ID reorder, retired-hole reuse, bounded growth, overflow and empty-release checks.

**Locate this existing block:**

```text
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzs-allocate-stable-object-record-rows-02.rs" skip=248
```

## <a id="checkpoint-96zzzt-retain-live-surface-record-rows"></a>Step 2 of 16 · Retain live surface record rows (`96zzzt-retain-live-surface-record-rows`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/16: 96zzzt-retain-liv…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing draw slots</text>
</svg>
</div>

**What this code does:** Connect stable row reuse to the existing surface record table.

### 1. `src/instance_gpu.rs`

Reserve surviving rows before regrouping instance geometry and reuse retired holes. Point each instance slot at its stable original record row. Write the packed instance record in place, leaving retired holes zero. Use the same retained-row lookup for ordinary and individually blended surfaces.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();
        if own.len()<=limit-count {
            for mesh in own {
                let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;
                let row=u32::try_from(all_records.len()).map_err(|_|"Ordinary record row")?;
                all_slots.push([row,mesh.primitive_base()]);all_records.push(mesh.packed());ordinary.insert(mesh.id,(first,row));
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
```

Replace it with:

```rust
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
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
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

Supply retained original-row checks through removal and insertion, exact mapped records and full release.

**Locate this existing block:**

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzt-retain-live-surface-record-rows-05.rs"
```

## <a id="checkpoint-96zzzu-prepare-shared-record-pick-consumers"></a>Step 3 of 16 · Prepare shared record pick consumers (`96zzzu-prepare-shared-record-pick-consumers`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/16: 96zzzu-prepare-sh…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">integer pick fragment</text>
</svg>
</div>

**What this code does:** Share the storage layout and shader adaptation before connecting first picks.

### 1. `src/object_rows.rs`

Own the record layout and shader adaptation once for every shared consumer.

**Locate this existing block:**

```rust
pub struct Rows {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared object records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]})
}
pub fn source(shader:&str)->String {
 shader.replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;").replace("return object_settings;","return objects[index];")
}
pub struct Rows {
```

### 2. `src/instance_gpu.rs`

Reuse the exact existing storage binding contract. Adapt the same surface shader through the shared owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
 Ok(rows)
}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
```

Replace it with:

```rust
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

## <a id="checkpoint-96zzzv-pick-originals-through-shared-records"></a>Step 4 of 16 · Pick originals through shared records (`96zzzv-pick-originals-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/16: 96zzzv-pick-origi…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing asynchronous rea…</text>
</svg>
</div>

**What this code does:** Connect first surface picks to current shared object records.

### 1. `src/instance_gpu.rs`

Share current-record validation and the existing record/slot draw across color and first picks. Use the requested pipeline with the same object row, slot and geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.ordinary_pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

Replace it with:

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

### 2. `src/pick_pipeline.rs`

Reuse one pick pipeline descriptor for the uniform and shared-record entries. Adapt the existing pick shader through the shared record owner. Choose the slot-aware entry for shared records. Read the same eight-byte object-row and primitive-base slot.

Replace the entire file with:

```rust
pub fn crop_clipping_layout(device:&wgpu::Device)->wgpu::BindGroupLayout{
    let entries=[80u64,448];
    let entries=entries.into_iter().enumerate().map(|(binding,size)|wgpu::BindGroupLayoutEntry{
        binding:binding as u32,visibility:wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(size)},count:None,
    }).collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("pick crop and clipping"),entries:&entries})
}
pub fn crop_clipping_group(device:&wgpu::Device,crop:&wgpu::Buffer,clipping:&wgpu::Buffer)->wgpu::BindGroup{
    device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("pick crop and clipping"),layout:&crop_clipping_layout(device),entries:&[
        wgpu::BindGroupEntry{binding:0,resource:crop.as_entire_binding()},
        wgpu::BindGroupEntry{binding:1,resource:clipping.as_entire_binding()},
    ]})
}

pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout, crop_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    mesh_with(device,view_layout,model_layout,crop_layout,false)
}
pub fn mesh_with(device:&wgpu::Device,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,crop_layout:&wgpu::BindGroupLayout,shared:bool)->wgpu::RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("mesh IDs"),
            bind_group_layouts: &[Some(view_layout), Some(model_layout), Some(&crop_clipping_layout(device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))], immediate_size: 0 });
        let source=concat!(include_str!("triangle.wgsl"), "
", include_str!("pick_crop.wgsl"), "
", include_str!("triangle_pick.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("mesh IDs"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,1).into())});let slots=[crate::draw_slots::layout()];
        // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if shared {"pick_record_vertex"}else{"pick_vertex"}), compilation_options: Default::default(),
                buffers: if shared {&slots}else{&[]}},
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into()),Some(crate::primitive_image::target(true))] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}
```

### 3. `src/pick_gpu.rs`

Own the shared pick state beside the existing fallback state. Create the shared pick state from the same descriptor and record layout. Retain the shared pick state for the browser lifetime. Keep isolated picker fixtures on the existing uniform path. Pass the existing shared record owner into first picking. Pair the current table with its shared pick state without changing readback ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for lane in ink { lane.view(queue, &request.transform, window.full, request.density); }
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
        queue.submit([crate::pick_encode::draw(device, image, window, &self.pipeline,
            &self.view_group, &self.crop_group, meshes, ink, faces, request.mode,text,clouds,sheets,&crate::text_gpu_support::GpuCtx {device:device.clone(),queue:queue.clone()},tables,clipping,caps,&self.crop,clipping_buffer)]);
        let ready = Arc::new(AtomicU8::new(0)); self.active = Some(Flight { ready: Arc::clone(&ready), ticket: queued.ticket, view: queued.view }); let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image.readback.slice(..).map_async(wgpu::MapMode::Read, move |result| {
```

Replace it with:

```rust
        for lane in ink { lane.view(queue, &request.transform, window.full, request.density); }
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
        queue.submit([crate::pick_encode::draw(device, image, window, &self.pipeline,
            &self.view_group, &self.crop_group, meshes, ink, faces, request.mode,text,clouds,sheets,&crate::text_gpu_support::GpuCtx {device:device.clone(),queue:queue.clone()},tables,clipping,caps,&self.crop,clipping_buffer,records.map(|rows|(rows,&self.record_pipeline)))]);
        let ready = Arc::new(AtomicU8::new(0)); self.active = Some(Flight { ready: Arc::clone(&ready), ticket: queued.ticket, view: queued.view }); let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image.readback.slice(..).map_async(wgpu::MapMode::Read, move |result| {
```

**Locate this block:**

```rust
    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
```

Replace it with:

```rust
    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>,records:Option<&crate::instance_gpu::Lane>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
```

**Locate this block:**

```rust
    pub fn new(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(device, view_layout, model_layout, &crop_layout);
        let buffer = |size| device.create_buffer(&wgpu::BufferDescriptor { label: Some("pick view"), size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let view = buffer(80); let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self { legacy_clipping:None,pipeline, view, view_group, crop, crop_group, image: None, active: None, pending: None,
            alive: Arc::new(AtomicBool::new(true)), gate: Default::default(), submitted: 0, coalesced: 0 }
    }
```

Replace it with:

```rust
    pub fn new(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pipeline = crate::pick_pipeline::mesh(device, view_layout, model_layout, &crop_layout);
        let record_pipeline=crate::pick_pipeline::mesh_with(device,view_layout,&crate::object_rows::layout(device),&crop_layout,true);
        let buffer = |size| device.create_buffer(&wgpu::BufferDescriptor { label: Some("pick view"), size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let view = buffer(80); let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self { legacy_clipping:None,pipeline,record_pipeline, view, view_group, crop, crop_group, image: None, active: None, pending: None,
            alive: Arc::new(AtomicBool::new(true)), gate: Default::default(), submitted: 0, coalesced: 0 }
    }
```

**Locate this block:**

```rust
pub struct Picker {
    legacy_clipping:Option<crate::clipping_gpu::Clipping>,
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

Replace it with:

```rust
pub struct Picker {
    legacy_clipping:Option<crate::clipping_gpu::Clipping>,
    pipeline: wgpu::RenderPipeline,
    record_pipeline:wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
```

### 4. `src/pick_encode.rs`

Accept shared surface records without altering the other pick passes. Draw through current records or the earlier exact uniform fallback.

Replace the entire file with:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group);
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite integer ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}
```

### 5. `src/renderer.rs`

Use the renderer’s existing record owner for original first picks.

**Locate this existing block:**

```rust
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut())); }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
```

## <a id="checkpoint-96zzzw-read-source-faces-through-shared-records"></a>Step 5 of 16 · Read source faces through shared records (`96zzzw-read-source-faces-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/16: 96zzzw-read-sourc…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing face readback</text>
</svg>
</div>

**What this code does:** Connect original source-face picking to the same current object records.

### 1. `src/instance_gpu.rs`

Reuse current-record validation and slot binding for consumers with different geometry layouts. Bind only the shared object record and slot; let each consumer draw its own geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

Replace it with:

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

### 2. `src/face_pick.wgsl`

Carry original ownership into the source-face fragment as an unsigned value. Share one source-face geometry function between ordinary and slot-selected records. Read the original owner selected by the source-face vertex.

Replace the entire file with:

```wgsl
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
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

Replace the entire file with:

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline,record_pipeline:wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let create=|shared| {let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)}; let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let source=if shared {crate::object_rows::source(source)}else{source.into()};let slots=[crate::draw_slots::layout()];
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,4).into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if shared {"face_record_vertex"}else{"face_vertex"}), buffers: if shared {&slots}else{&[]}, compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        pipeline};let pipeline=create(false);let record_pipeline=create(true);Self {pipeline,record_pipeline,rows:Vec::new()}
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)],clipping:&wgpu::Buffer) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup,records:Option<&crate::instance_gpu::Lane>) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            if records.is_some_and(|records|records.bind_record(pass,mesh)){pass.set_pipeline(&self.record_pipeline);}else{pass.set_pipeline(&self.pipeline);mesh.bind_model(pass);}pass.set_bind_group(2,&row.group,&[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
```

### 4. `src/pick_encode.rs`

Share the original object table with both first and source-face pick passes.

**Locate this existing block:**

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
```

## <a id="checkpoint-96zzzx-pack-existing-ambient-radius-in-object-records"></a>Step 6 of 16 · Pack existing ambient radius in object records (`96zzzx-pack-existing-ambient-radius-in-object-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6/16: 96zzzx-pack-exist…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared property vertex…</text>
</svg>
</div>

**What this code does:** Carry the existing source radius in a spare aligned record word.

### 1. `src/gpu_mesh.rs`

Store the existing source radius in the spare aligned word and write only a changed value. Retain derived source metadata when rebuilding the current record. Selection and style updates keep the current radius bits.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

**Locate this block:**

```rust
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
```

Replace it with:

```rust
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
```

### 2. `src/renderer.rs`

Reuse the existing source-bound calculations before refreshing shared records.

**Locate this existing block:**

```rust
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
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
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"),2,0).into())});
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl")),2,0).into())});
```

## <a id="checkpoint-96zzzy-read-ambient-properties-through-shared-records"></a>Step 7 of 16 · Read ambient properties through shared records (`96zzzy-read-ambient-properties-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7/16: 96zzzy-read-ambie…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing ambient recipe</text>
</svg>
</div>

**What this code does:** Connect source normal and radius drawing to the existing object table.

### 1. `src/ambient_source_pipeline.rs`

Own the shared property state beside its existing fallback. Move the fallback radius layout to the shared descriptor owner. Build both property states from one descriptor. Select the current shared record layout or the earlier uniform layout. Use the existing arena geometry binding for pulled properties. Shared properties read radius from their record; fallback properties retain their uniform. Adapt the same property shader through the existing record owner. Share the shader source across both property states. Select the slot-aware property entry for shared records. Keep the indexed vertex attributes alive while creating either property state. Read draw slots for shared properties and retain the earlier indexed vertex format for fallback. Create both states with the same property outputs and depth policy.

Replace the entire file with:

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline,pub radius:wgpu::BindGroupLayout}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let radius=crate::gpu_layout::uniform(device,16);let create=|shared| {
  let view=crate::gpu_layout::uniform(device,80);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(if shared {&geometry}else{&radius})],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw,radius}
 }
}
```

### 2. `src/ambient_source_gpu.rs`

Accept the existing renderer object table. Read current shared properties or the earlier exact uniform fallback.

Replace the entire file with:

```rust
use std::collections::HashMap;
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline,rows:HashMap<u32,(wgpu::Buffer,wgpu::BindGroup,f32)>,pub writes:u64,pub draws:u64}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device),rows:Default::default(),writes:0,draws:0}}
 pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,specs:&[(u32,f32)]){
  let live=specs.iter().map(|(id,_)|*id).collect::<std::collections::HashSet<_>>();self.rows.retain(|id,_|live.contains(id));for &(id,radius)in specs{let row=self.rows.entry(id).or_insert_with(||{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("original ambient contact radius"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let group=crate::gpu_layout::group(device,&self.pipes.radius,&buffer);(buffer,group,f32::NAN)});if row.2!=radius{queue.write_buffer(&row.0,0,&radius.to_ne_bytes());row.2=radius;self.writes+=1;}}if self.rows.is_empty(){self.rows.shrink_to_fit();}
 }
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if let Some((_,group,_))=self.rows.get(&mesh.id.value()){if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.draw);mesh.bind_model(&mut pass);pass.set_bind_group(3,group,&[]);mesh.geometry.draw(&mut pass);}self.draws+=1;}}
 }
 pub fn bytes(&self)->u64{self.rows.values().map(|r|r.0.size()).sum()}
}
```

### 3. `src/ambient_gpu.rs`

Keep the object table available while preparing original source properties. Pass the existing record owner through the unchanged ambient recipe.

Replace the entire file with:

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self,device:&wgpu::Device,queue:&wgpu::Queue){self.state=None;self.cached=None;self.sources.set(device,queue,&[]);self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.sources.bytes()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

### 4. `src/renderer.rs`

Use the same current object records for rendering, picking and ambient properties.

**Locate this existing block:**

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_gpu_tests.rs`

Supply an exact shared-versus-uniform ambient comparison while retaining every original cache, source, shading and release assertion.

**Locate this existing block:**

```text
fn actual_ambient_darkens_original_contacts_in_one_frame_caches_and_releases_large_images(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;let matrix=camera.uniform();let r=renderer(&scene);let plain=pixels(&r,&matrix);let uploads=r.stats()[0];r.set_ambient(true);let shaded=pixels(&r,&matrix);assert_eq!(r.ambient_error(),None);let usage=r.ambient_usage();assert!(usage[0]>100_000);assert_eq!(usage[1],1);assert_eq!(usage[3],1);let second=pixels(&r,&matrix);assert_eq!(second,shaded,"one frame completes ambient shading");assert_eq!(&r.ambient_usage()[1..4],&usage[1..4]);assert_eq!(r.stats()[0],uploads);let darker=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]!=[255,255,255]&&a[..3].iter().zip(&b[..3]).any(|(a,b)|a.saturating_sub(*b)>3)).count();assert!(darker>100,"actual ambient shading: {darker} pixels");let shadow=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<235)).count();assert!(shadow>20,"actual virtual-ground contacts: {shadow}");assert_eq!(scene.objects()[0].id,owner);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);r.set_ambient(false);assert_eq!(r.ambient_usage()[0],320);assert_eq!(pixels(&r,&matrix),plain);r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded);assert_eq!(r.ambient_usage()[1],1,"style toggles retain compiled pipelines");assert_eq!(r.stats()[0],uploads);
}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-17.rs"
```

## <a id="checkpoint-96zzzz-share-the-object-record-shader-schema"></a>Step 8 of 16 · Share the object record shader schema (`96zzzz-share-the-object-record-shader-schema`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8/16: 96zzzz-share-the-…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged GPU bytes</text>
</svg>
</div>

**What this code does:** Keep one aligned object record declaration across drawing, caps and projection.

### 1. `src/object_settings.wgsl`

Write the existing aligned record schema once.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-01.rs"
```

### 2. `src/triangle.wgsl`

Use the record declaration supplied by shared shader assembly.

**Locate this existing block:**

```wgsl
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
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

### 3. `src/cap_count.wgsl`

Cap crossings use the identical record declaration.

**Locate this existing block:**

```wgsl
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
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

### 4. `src/finite_project.wgsl`

Projection uses the same byte-compatible model and range fields.

**Locate this existing block:**

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view:View;
```

### 5. `src/object_record.rs`

Expose the shared declaration through the existing record owner.

**Locate this existing block:**

```rust
pub type Record = [u8; BYTES];
```

After that block, type:

```rust
pub fn shader()->&'static str {include_str!("object_settings.wgsl")}
```

### 6. `src/clipping_gpu.rs`

Assemble every clipped raster consumer with the shared record declaration. Keep the shared declaration separate from shader resource bindings.

Replace the entire file with:

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),crate::object_record::shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

### 7. `src/finite_project_source.rs`

Compute projection includes the same declaration before its packed geometry helper.

**Locate this existing block:**

```rust
pub fn source() -> String { format!("{}\n{}",include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source() -> String { format!("{}\n{}\n{}",crate::object_record::shader(),include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/object_record_tests.rs`

Verify that projection uses the same aligned members as both raster states after consolidating the flag vector.

**Locate this existing block:**

```text
(crate::finite_project_source::source(), vec![0, 64, 68, 72, 80, 144, 160])
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-layout-fixture.rs"
```

## <a id="checkpoint-96zzzza-prepare-shared-record-triangle-projection"></a>Step 9 of 16 · Prepare shared record triangle projection (`96zzzza-prepare-shared-record-triangle-projection`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9/16: 96zzzza-prepare-s…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared projection shade…</text>
</svg>
</div>

**What this code does:** Prepare the compute shader to select the same current object row.

### 1. `src/instance_gpu.rs`

Use one packed-record match guard for raster and compute consumers.

**Locate this existing block:**

```rust
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-01.rs"
```

### 2. `src/finite_project.wgsl`

Use the shared object shader adaptation without duplicating the projection algorithm. Read the selected row from the existing unsigned projection interval.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
```

Replace it with:

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
```

**Locate this block:**

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
```

Replace it with:

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
```

### 3. `src/finite_project_source.rs`

Prepare the storage compute source through the existing record owner.

**Locate this existing block:**

```rust
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn records()->String {crate::object_rows::source(&source())}

#[cfg(test)]
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/finite_project_source.rs`

Supply validation of both uniform and storage compute interfaces.

**Locate this existing block:**

```text
        let source = super::source();
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-05.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/finite_project_source.rs`

Both assembled shader sources must retain the existing compute entry.

**Locate this existing block:**

```text
            && entry.stage == naga::ShaderStage::Compute));
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzza-prepare-shared-record-triangle-projection-06.rs" skip=59
```

## <a id="checkpoint-96zzzzb-project-triangles-through-shared-records"></a>Step 10 of 16 · Project triangles through shared records (`96zzzzb-project-triangles-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10/16: 96zzzzb-project-t…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged finite visibili…</text>
</svg>
</div>

**What this code does:** Connect finite triangle projection to the current shared object table.

### 1. `src/tile_pipelines.rs`

Own a shared projection binding state beside the earlier fallback. Keep both compute states with the existing projection owner. Build uniform and record projection states from one binding recipe and algorithm. Remove the replaced single-state constructor. Retain the shared projection state with the existing scans and bins.

Replace the entire file with:

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub record_project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub record_project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let create=|shared| {let project_layout=layout("finite original projection",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,if shared {Type::Storage{read_only:true}}else{Type::Uniform}),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)]);
   let source=if shared {crate::finite_project_source::records()}else{crate::finite_project_source::source()};let project=compute(device,&source,"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);(project_layout,project)};
  let(project_layout,project)=create(false);let(record_project_layout,record_project)=create(true);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,record_project_layout,raster_layout,scan_layout,project,record_project,count,fill,scans}
 }
}
```

### 2. `src/tile_gpu.rs`

Observe source arena offsets and stable row selectors as well as model and buffer identity. Each projection row selects its current resource contract. Preserve isolated original fixtures while connecting the renderer’s record owner. Use current table rows or the exact earlier model buffer. Invalidate moved arena intervals and changed record selectors without reprojecting selection-only changes. Store the current object row in the existing sixteen-byte interval. Bind the selected model resource without a second table. Use the matching uniform or storage layout for the cached projection group. Dispatch the same projection algorithm through the selected resource state.

Replace the entire file with:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32,u32,u32,Option<u32>);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,shared:bool,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){self.set_records(device,meshes,None)}
 pub fn set_records(&mut self,device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let current=|mesh:&GpuMesh|records.and_then(|rows|rows.current_row(mesh)).map(|(_,row)|row);
  let model=|mesh:&GpuMesh|if current(mesh).is_some(){records.unwrap().record_buffer().clone()}else{mesh.model_buffer().clone()};
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,model(row),row.packed()[..64].try_into().unwrap(),row.primitive_base(),row.geometry.index_start,row.geometry.index_count,current(row))}).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),current(row).unwrap_or(0),0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:model(row),count,triangles,shared:current(row).is_some(),group:None});
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
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
  for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,if row.shared {&self.pipes.record_project_layout}else{&self.pipes.project_layout},&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(if row.shared {&self.pipes.record_project}else{&self.pipes.project});pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64,self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

### 3. `src/renderer.rs`

Refresh projection after the object table, so its row selectors and replacement buffers are current. Use the renderer’s current table for finite projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        self.tiles.borrow_mut().set(&self.device,&self.meshes);
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
```

Replace it with:

```rust
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply exact storage-versus-uniform projected bytes and color comparisons while retaining the original range, owner and release assertions.

**Locate this existing block:**

```text
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzb-project-triangles-through-shared-records-17.rs"
```

## <a id="checkpoint-96zzzzc-prepare-shared-record-cap-counting"></a>Step 11 of 16 · Prepare shared record cap counting (`96zzzzc-prepare-shared-record-cap-counting`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11/16: 96zzzzc-prepare-s…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
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
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

Replace it with:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
```

### 2. `src/cap_pipeline.rs`

Reuse the existing pipeline descriptor with an explicit vertex-buffer format. Remove the repeated format owner from the shared descriptor. Use the selected format without changing targets, blending or depth policy.

Replace the entire file with:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED}],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
 }
}
```

## <a id="checkpoint-96zzzzd-prepare-cap-plane-and-arena-bindings"></a>Step 12 of 16 · Prepare cap plane and arena bindings (`96zzzzd-prepare-cap-plane-and-arena-bindings`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12/16: 96zzzzd-prepare-c…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable cap geometry gro…</text>
</svg>
</div>

**What this code does:** Prepare explicit plane parameters with current immutable arena bindings.

### 1. `src/cap_geometry.rs`

Keep explicit plane parameters and arena binding replacement in one resource owner.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-01.rs"
```

### 2. `src/lib.rs`

Declare the resource without connecting it to drawing yet.

**Locate this existing block:**

```rust
pub mod cap_pipeline;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cap_geometry;
mod cap_pipeline;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/cap_geometry.rs`

Supply actual mapped plane values, exact binding reuse/replacement through arena growth, bounded refusal and full release.

**Locate this existing block:**

```text
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let group=crate::tile_pipelines::group(device,layout,&[&vertices,parameter,&indices]);self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-03.rs" skip=2026
```

## <a id="checkpoint-96zzzze-count-cap-color-through-shared-records"></a>Step 13 of 16 · Count cap color through shared records (`96zzzze-count-cap-color-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13/16: 96zzzze-count-cap…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged cap color and d…</text>
</svg>
</div>

**What this code does:** Connect cap crossing color to current object records and explicit planes.

### 1. `src/cap_count.wgsl`

Pull the current arena range and keep explicit plane selection independent of the object slot.

**Locate this existing block:**

```wgsl
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(3) @binding(0) var<storage,read> cap_vertices:array<PackedVertex>;
@group(3) @binding(1) var<uniform> cap_plane:vec4<u32>;
@group(3) @binding(2) var<storage,read> cap_indices:array<u32>;
@vertex fn vs_record_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=cap_vertices[cap_indices[occurrence+object.range.x]];return count_corner(packed_position(vertex),cap_plane.x,object);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
```

### 2. `src/cap_pipeline.rs`

Own the shared crossing state with the existing cap color pipeline. Reuse the crossing descriptor, shader, additive blend and depth policy with current records. Retain the shared count state beside the earlier uniform fallback.

Replace the entire file with:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub record_count:wgpu::RenderPipeline,pub geometry:wgpu::BindGroupLayout,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[target.clone()],true);
  let geometry=crate::cap_geometry::layout(device);let source=crate::object_rows::source(include_str!("cap_count.wgsl"));let record_count=pipeline_buffers(device,crate::clipping_gpu::source(&source,2,0),&[&view,&crate::object_rows::layout(device),&clip,&geometry],"vs_record_count","fs_count",&[target],&[crate::draw_slots::layout()],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,record_count,geometry,color,counts,frame}
 }
}
```

### 3. `src/caps_gpu.rs`

Let the existing crossing target own its plane parameters and cached geometry groups. Drop stale geometry references when scene ownership changes. Keep the new resource under the existing count-image release lifetime. Prepare bounded plane values before encoding crossings. Create current immutable arena bindings on the existing device. Use the renderer’s current record owner for cap color crossings. Refresh the crossing target’s cached geometry groups as needed. Reuse groups for current records and retain exact target-plane filtering. Draw one object slot with an explicit plane, or use the unchanged uniform crossing path. Account plane parameters once and release them with the crossing image.

Replace the entire file with:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

### 4. `src/renderer.rs`

Pass the existing device to cap count resource preparation. Use the same current object records for cap color, ordinary surfaces and property consumers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0});
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

Replace it with:

```rust
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/cap_gpu_tests.rs`

Include the one actual sixteen-byte plane parameter in exact cap ownership accounting.

**Locate this existing block:**

```text
assert_eq!(r.cap_usage()[0],16+320*240*2);
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-17.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/cap_gpu_tests.rs`

Supply an exact shared/uniform cap comparison and restoration while retaining selected pixels, original closedness cache and full release.

**Locate this existing block:**

```text
scene.clear();r.set_scene(&scene,None);pixels(&r);assert_eq!(r.cap_usage()[0],16);
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-18.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/renderer.rs`

Supply test-only access to the uniform fallback without exposing renderer internals in the browser application.

**Locate this existing block:**

```text
    pub fn set_scene(
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzze-count-cap-color-through-shared-records-test-access.rs"
```

## <a id="checkpoint-96zzzzf-prepare-shared-record-cap-picks"></a>Step 14 of 16 · Prepare shared record cap picks (`96zzzzf-prepare-shared-record-cap-picks`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14/16: 96zzzzf-prepare-s…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged crossing fragme…</text>
</svg>
</div>

**What this code does:** Prepare both cap pick crossing states without changing original ownership.

### 1. `src/cap_count.wgsl`

Retain the original atomic, crop and frame bindings while adding pulled geometry to the shared pick vertex entry.

**Locate this existing block:**

```wgsl
fn wanted(pixel: u32, mode: u32) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(3) @binding(3) var<storage,read> pick_cap_vertices:array<PackedVertex>;
@group(3) @binding(4) var<uniform> pick_cap_plane:vec4<u32>;
@group(3) @binding(5) var<storage,read> pick_cap_indices:array<u32>;
@vertex fn vs_record_pick_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=pick_cap_vertices[pick_cap_indices[occurrence+object.range.x]];var output=count_corner(packed_position(vertex),pick_cap_plane.x,object);output.pos=cap_crop.transform*output.pos;return output;}
fn wanted(pixel: u32, mode: u32) -> bool {
```

### 2. `src/cap_pick_pipeline.rs`

Own both shared crossing passes beside the original fallback. Extend the pick group without replacing its atomic crossing, crop or frame resources. Reuse the original empty color target for both shared passes. Reuse both fragment algorithms, the crop transform and crossing descriptor with the current object record. Keep the preparatory shared states under the original pick pipeline owner.

Replace the entire file with:

```rust
pub struct Pipelines{pub record_count:wgpu::RenderPipeline,pub record_nearest:wgpu::RenderPipeline,pub record_geometry:wgpu::BindGroupLayout,pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let record_geometry=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("shared cap pick crossings"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform),crate::tile_pipelines::entry(3,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(4,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(5,S::VERTEX,B::Storage{read_only:true})]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target.clone()],true);
  let shared=crate::object_rows::source(include_str!("cap_count.wgsl"));let source=crate::clipping_gpu::source(&shared,2,0);let layouts=[&view,&crate::object_rows::layout(device),&clip,&record_geometry];let record_count=crate::cap_pipeline::pipeline_buffers(device,source.clone(),&layouts,"vs_record_pick_count","fs_pick_count",&[target.clone()],&[crate::draw_slots::layout()],true);let record_nearest=crate::cap_pipeline::pipeline_buffers(device,source,&layouts,"vs_record_pick_count","fs_pick_owner",&[target],&[crate::draw_slots::layout()],true);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
  Self{record_count,record_nearest,record_geometry,count,nearest,ids,records,owners,frame}
 }
}
```

**Supplied data:**

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/current_inventory.json`

Copy the dated current runtime-input inventory after kernel/wood changes. This supplied data refresh preserves the historical inventory and every viewer source hash; it does not replace implementation work.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzf-prepare-shared-record-cap-picks-current-inputs.json"
```

## <a id="checkpoint-96zzzzg-own-shared-record-cap-pick-resources"></a>Step 15 of 16 · Own shared record cap pick resources (`96zzzzg-own-shared-record-cap-pick-resources`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15/16: 96zzzzg-own-share…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing pick lifetime</text>
</svg>
</div>

**What this code does:** Reuse explicit plane and arena bindings under the existing pick lifetime.

### 1. `src/cap_geometry.rs`

Reuse the same plane and arena resource owner for groups with retained pick bindings. Append geometry bindings after existing pick resources; color keeps the original three entries.

Replace the entire file with:

```rust
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {self.group_with(device,layout,geometry,plane,0,&[]) }
 pub fn group_with(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32,first:u32,extra:&[wgpu::BindGroupEntry])->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let buffers=[&vertices,parameter,&indices];let mut entries=extra.to_vec();entries.extend(buffers.into_iter().enumerate().map(|(i,buffer)|wgpu::BindGroupEntry{binding:first+i as u32,resource:buffer.as_entire_binding()}));let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap plane and current arena"),layout,entries:&entries});self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn planes_keep_current_arena_bindings_and_release() {
  let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&renderer.device,&renderer.queue);let layout=layout(&ctx.device);let mut groups=Geometry::default();groups.set_planes(&ctx.device,2).unwrap();assert_eq!(groups.bytes(),32);
  for(plane,buffer)in groups.planes.iter().enumerate(){assert_eq!(crate::growing_buffer_tests::read(&ctx,buffer,16),[plane as u32,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());}
  let mut scene=crate::scene::Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let arena=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let geometry=Rc::new(crate::gpu_geometry::GpuGeometry::shared(&ctx,scene.objects()[0].mesh.clone(),arena.clone()).unwrap());let old=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),old);assert!(groups.group(&ctx.device,&layout,&geometry,2).is_err());
  scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();arena.borrow_mut().upload(&ctx,&scene.objects()[1].mesh).unwrap();let fresh=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_ne!(fresh,old);assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),fresh);
  assert!(groups.set_planes(&ctx.device,crate::clipping_uniform::MAX_PLANES as u32+1).is_err());assert_eq!(groups.bytes(),32);groups.invalidate();assert!(groups.groups.is_empty());groups.clear();assert_eq!(groups.bytes(),0);assert_eq!(groups.planes.capacity(),0);assert_eq!(groups.groups.capacity(),0);
 }
}
```

### 2. `src/cap_pick_gpu.rs`

Keep cap-pick plane parameters and geometry groups under the existing pick lifetime. Initialize the resource owner without allocating a new arena. Invalidate immutable pick groups when crossing buffers or crop bindings can change, then prepare bounded plane values. Count plane parameters once; arena and borrowed crop allocations retain their original owners.

Replace the entire file with:

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&self,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

**Supplied data:**

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/current_inventory.json`

Copy the supplied checkpoint inventory after regeneration of Session protocol documentation comments. This records current inputs without changing the strict inventory check or historical evidence.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzg-own-shared-record-cap-pick-resources-current-inputs.json"
```

## <a id="checkpoint-96zzzzh-pick-caps-through-shared-records"></a>Step 16 of 16 · Pick caps through shared records (`96zzzzh-pick-caps-through-shared-records`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16/16: 96zzzzh-pick-caps…</text>
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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original integer cap owner</text>
</svg>
</div>

**What this code does:** Connect both cap crossing passes to current records and retain original owners.

### 1. `src/cap_pick_gpu.rs`

Prepare immutable current arena groups before encoding either crossing pass. Use the existing renderer record owner without introducing another table. Prepare exact target-plane groups once and reuse them across count and nearest-owner passes. Count and resolve owners from current records, retaining the original uniform path for changed packed records.

Replace the entire file with:

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  let extra=[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.crop.as_ref().unwrap().as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}];let rows=(0..planes).map(|plane|meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|self.geometry.group_with(device,&self.pipes.record_geometry,&mesh.geometry,plane,3,&extra).expect("Prepared cap pick plane"));(mesh,group)}).collect::<Vec<_>>()).collect::<Vec<_>>();
  for(pipeline,shared)in [(&self.pipes.count,&self.pipes.record_count),(&self.pipes.nearest,&self.pipes.record_nearest)]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for(plane,rows)in rows.iter().enumerate(){for(mesh,group)in rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(shared);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(pipeline);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane as u32..plane as u32+1);}}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

### 2. `src/caps_gpu.rs`

Thread the same optional object owner through cap picking. Keep preparation and encoding under the existing bounded cap pick owner.

Replace the entire file with:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

### 3. `src/pick_encode.rs`

Share object records with first picks, source faces and cap picks in the existing submission.

**Locate this existing block:**

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/cap_pick_tests.rs`

Supply full-u32 shared/fallback owner comparison and restoration before a color frame, retaining all original geometry assertions.

**Locate this existing block:**

```text
assert!(r.cap_usage()[0]>24);pixels(&r);
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-08.rs"
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
