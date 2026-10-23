# 20.4 · Slots, independent instance owners and stable record updates

**Goal:** Draw transparent placements from the same shared records in scene order. · **Steps:** 24 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`src/engine/gpu/arena.rs,src/shaders/pulled_scene.wgsl,src/engine/gpu/indirect.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 20.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/primitive_image.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/gpu_mesh.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/triangle.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">24 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/arena.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/pulled_scene.…</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Separate primitive addres…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Shared Storage Buffer Arena</text>
  <rect x="596" y="52" width="208" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices</text>
  <rect x="596" y="100" width="208" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Free-List Arena Slots</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Slots, independent instance…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Pulled Draws &amp; Compaction</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@builtin(vertex_index)</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Compacted Draw Ranges</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Share transparent instanc…</text>
</svg>
</div>

## <a id="checkpoint-96zzu-separate-primitive-addresses-from-selection"></a>1. Separate primitive addresses from selection

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/primitive_image.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">range record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ordinary shader or instan…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">physical primitive image</text>
</svg>
</div>

**What this code does:** Keep full primitive addresses in the range record while selection remains an independent flag.

### 1. `src/primitive_image.rs`

Name the beginning of the existing reserved cap interval.

**Locate this existing block:**

```rust
pub(crate) const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rg16Uint;
pub(crate) struct Image {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) const CAP_START: u32 = 0xfffffff0;
pub(crate) const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rg16Uint;
pub(crate) struct Image {
```

### 2. `src/gpu_mesh.rs`

Keep selection independent of the address. Use the already aligned range record for the full primitive base. Validate the entire emitted interval and upload only its address word.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        queue: &wgpu::Queue,
        base: u32,
    ) -> Result<(), &'static str> {
        if base > 0x7fff_ffff {
            return Err("Primitive base exceeds its packed range");
        }
        if self.primitive_base != base {
            let word = (base << 1) | u32::from(self.selected());
            self.previous[64..68].copy_from_slice(&word.to_ne_bytes());
            queue.write_buffer(&self.model, 64, &self.previous[64..68]);
            self.primitive_base = base;
```

Replace it with:

```rust
        queue: &wgpu::Queue,
        base: u32,
    ) -> Result<(), &'static str> {
        if base
            .checked_add(self.geometry.index_count / 3)
            .is_none_or(|end| end >= crate::primitive_image::CAP_START)
        {
            return Err("Primitive range overlaps reserved caps");
        }
        if self.primitive_base != base {
            self.previous[RANGE + 4..RANGE + 8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(
                &self.model,
                (RANGE + 4) as u64,
                &self.previous[RANGE + 4..RANGE + 8],
            );
            self.primitive_base = base;
```

**Locate this block:**

```rust
        }
    }
    bytes[RANGE..RANGE + 4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}
```

Replace it with:

```rust
        }
    }
    bytes[RANGE..RANGE + 4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE + 4..RANGE + 8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}
```

**Locate this block:**

```rust
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(
        &((primitive_base << 1) | u32::from(style.selected)).to_ne_bytes(),
    );
    let id = if style.pickable {
        object.id.value()
    } else {
```

Replace it with:

```rust
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = if style.pickable {
        object.id.value()
    } else {
```

### 3. `src/triangle.wgsl`

Read the same range word in color and first picking.

**Locate this existing block:**

```wgsl
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=object.range.y+occurrence/3u+1u;
    return output;
```

### 4. `src/triangle_pick.wgsl`

Read the same range word in color and first picking.

**Locate this existing block:**

```wgsl
    let input=mesh_vertex(occurrence,object_settings.range.x);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let input=mesh_vertex(occurrence,object_settings.range.x);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,object_settings.range.y+occurrence/3u+1u);
}
```

### 5. `src/instance_gpu.rs`

Keep the instance shader adaptation attached to the new ordinary expression. Build instance slots from the same full address.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .map(|(row, record)| {
                        [
                            row as u32,
                            u32::from_ne_bytes(record[64..68].try_into().unwrap()) >> 1,
                        ]
```

Replace it with:

```rust
                    .map(|(row, record)| {
                        [
                            row as u32,
                            u32::from_ne_bytes(
                                record[crate::object_record::RANGE + 4
                                    ..crate::object_record::RANGE + 8]
                                    .try_into()
                                    .unwrap(),
                            ),
                        ]
```

**Locate this block:**

```rust
        .replace(
            "(object.selection.x>>1u)+occurrence/3u+1u",
            "primitive_base+occurrence/3u+1u",
        );
    crate::clipping_gpu::source(&source, 2, 0)
```

Replace it with:

```rust
        .replace(
            "object.range.y+occurrence/3u+1u",
            "primitive_base+occurrence/3u+1u",
        );
    crate::clipping_gpu::source(&source, 2, 0)
```

### 6. `src/renderer.rs`

Bound cumulative visible ranges below reserved cap markers.

**Locate this existing block:**

```rust
                .checked_add(mesh.geometry.index_count / 3)
                .filter(|n| *n <= 0x7fff_ffff)
                .expect("Bounded projected triangle range");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .checked_add(mesh.geometry.index_count / 3)
                .filter(|n| *n < crate::primitive_image::CAP_START)
                .expect("Bounded projected triangle range");
```

### 7. `src/global_outline_tests.rs`

Read the independent base in the existing ownership fixture. Reject reserved addresses while keeping the original unchanged-state assertion. Supply full high-bit GPU readback, selection independence, cap collision and overflow assertions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    );
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Replace it with:

```rust
    );
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks() {
    assert!(include_str!("clip.wgsl").contains(&format!(
        "const CAP_PRIMITIVE: u32 = 0x{:08x}u;",
        crate::primitive_image::CAP_START
    )));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.4, -0.4, -0.4],
                    [0.4, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let mut gpu = renderer(&scene);
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let uploads = gpu.stats()[0];
    for base in [
        0x8000_0000,
        crate::primitive_image::CAP_START - 2,
    ] {
        gpu.meshes[0]
            .set_primitive_base(&gpu.queue, base)
            .unwrap();
        pixels(&gpu);
        let plain = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(plain[120 * 320 + 160], base + 1);
        gpu.select_objects(&scene, &[owner]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            plain
        );
        assert_eq!(
            u32::from_ne_bytes(
                gpu.meshes[0].packed()[64..68]
                    .try_into()
                    .unwrap()
            ),
            1
        );
        gpu.select_objects(&scene, &[]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            plain
        );
        let before = gpu.meshes[0].packed();
        for invalid in [
            crate::primitive_image::CAP_START - 1,
            crate::primitive_image::CAP_START,
            u32::MAX,
        ] {
            assert!(
                gpu.meshes[0]
                    .set_primitive_base(&gpu.queue, invalid)
                    .is_err()
            );
            assert_eq!(gpu.meshes[0].packed(), before);
        }
    }
    assert_eq!(gpu.stats()[0], uploads);
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
    let before = gpu.meshes[1].packed();
    assert!(
        gpu.meshes[1]
            .set_primitive_base(&gpu.queue, 0x8000_0000)
            .is_err()
    );
    assert_eq!(gpu.meshes[1].packed(), before);
```

Replace it with:

```rust
    let before = gpu.meshes[1].packed();
    assert!(
        gpu.meshes[1]
            .set_primitive_base(&gpu.queue, crate::primitive_image::CAP_START)
            .is_err()
    );
    assert_eq!(gpu.meshes[1].packed(), before);
```

**Locate this block:**

```rust
            .map(|row| {
                u32::from_ne_bytes(row.packed()[64..68].try_into().unwrap()) >> 1
            })
            .collect::<Vec<_>>()
    };
```

Replace it with:

```rust
            .map(|row| {
                u32::from_ne_bytes(
                    row.packed()[crate::object_record::RANGE + 4
                        ..crate::object_record::RANGE + 8]
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
```

## <a id="checkpoint-96zzv-bind-the-own-row-slot-for-ordinary-surfaces"></a>2. Bind the own-row slot for ordinary surfaces

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/draw_slots.rs, src/clip…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">eight-byte draw slot</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">own uniform or instance r…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared surface vertex</text>
</svg>
</div>

**What this code does:** Use one integer vertex input for ordinary surfaces and placed-instance surfaces.

### 1. `src/draw_slots.rs`

Generate the shader sentinel from the existing integer owner.

**Locate this existing block:**

```rust
pub const OWN_ROW: u32 = u32::MAX;
pub type Slot = [u32; 2];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const OWN_ROW: u32 = u32::MAX;
pub fn sentinel_shader() -> String {
    format!("const OWN_ROW:u32={OWN_ROW}u;")
}
pub type Slot = [u32; 2];
```

### 2. `src/clipping_gpu.rs`

Include the same own-row sentinel in shared shader assembly.

**Locate this existing block:**

```rust
    format!(
        "{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        crate::colour::selection_shader(),
        concat!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    format!(
        "{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        crate::colour::selection_shader(),
        crate::draw_slots::sentinel_shader(),
        concat!(
```

### 3. `src/triangle.wgsl`

Read the shared eight-byte vertex slot in ordinary and instance pipelines. Choose the own-record base for ordinary surfaces and the slot base for instances. Emit the selected independent primitive base.

**Locate this existing block:**

```wgsl
@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=object.range.y+occurrence/3u+1u;
    return output;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @location(0) row:u32,@location(1) slot_base:u32) -> VertexOutput {
    let object=object_at(row);
    let primitive_base=select(slot_base,object.range.y,row==OWN_ROW);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=primitive_base+occurrence/3u+1u;
    return output;
```

### 4. `src/instance_gpu.rs`

Retain only record-owner adaptation; the vertex entry and slots are now shared.

**Locate this existing block:**

```rust
            "return objects[index];",
        )
        .replace(
            "@builtin(instance_index) instance: u32",
            "@location(0) row:u32,@location(1) primitive_base:u32",
        )
        .replace(
            "let object=object_at(instance);",
            "let object=object_at(row);",
        )
        .replace(
            "object.range.y+occurrence/3u+1u",
            "primitive_base+occurrence/3u+1u",
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            "return objects[index];",
        );
```

### 5. `src/renderer.rs`

Own one permanent default slot with the renderer pipeline resources. Declare the same integer vertex layout for ordinary surface pipelines. Create the eight-byte own-row sentinel once. Carry the existing slot owner into the renderer. Bind the own-row input before ordinary surface draws. Restore the own-row input after any instance batch changes it.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
```

Replace it with:

```rust
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass, 0, 0);
            self.highlight.mesh(&mut pass);
```

**Locate this block:**

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
```

Replace it with:

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass, 0, 0);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
```

**Locate this block:**

```rust
            measurement,
            sheets,
            instances,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
```

Replace it with:

```rust
            measurement,
            sheets,
            instances,
            own_slot,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
```

**Locate this block:**

```rust
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

Replace it with:

```rust
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let own_slot = crate::draw_slots::Slots::new(
            &crate::text_gpu_support::GpuCtx::new(&device, &queue),
        );
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

**Locate this block:**

```rust
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
```

Replace it with:

```rust
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
```

**Locate this block:**

```rust
    instances: crate::instance_gpu::Lane,
    instance_rows: std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
    instances: crate::instance_gpu::Lane,
    own_slot: crate::draw_slots::Slots,
    instance_rows: std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

### 6. `src/global_outline_tests.rs`

Supply actual ordinary/instance pixels, placed-face highlights, primitive preservation, original sources and permanent sentinel checks.

**Locate this existing block:**

```rust
        original
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        original
    );
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release()
 {
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &crate::instance_specimen::display_bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let bytes = [crate::draw_slots::OWN_ROW, 0]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect::<Vec<_>>();
    assert_eq!(gpu.own_slot.allocated_bytes(), 8);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.own_slot.buffer.buf, 8),
        bytes
    );
    let originals = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let owner = scene.objects()[0].id;
    for opacity in [1., 0.5] {
        gpu.set_opacity(opacity);
        let plain = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        assert_ne!(pixels(&gpu), plain);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_highlight(&scene, None).unwrap();
        assert_eq!(pixels(&gpu), plain);
    }
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|o| o.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        originals
    );
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert_eq!(gpu.usage(), [0; 4]);
    assert_eq!(gpu.own_slot.allocated_bytes(), 8);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.own_slot.buffer.buf, 8),
        bytes
    );
}
```

## <a id="checkpoint-96zzw-reload-scenes-through-the-existing-publication-owner"></a>3. Reload scenes through the existing publication owner

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_route.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reload_scene</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated candidate</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Live adoption</text>
</svg>
</div>

**What this code does:** Expose scene replacement through the existing reader, including empty-manifest document release.

### 1. `src/publication_route.rs`

Share named-scene resolution between startup and explicit reload. Reuse the existing data-base rules. Use the same path rule at startup without a second algorithm.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            notify,
        });
    }
    let name = named?;
    let mut name = if name.contains('.') {
        name.to_owned()
    } else {
        format!("{name}.yaml")
    };
    if !name.contains('/') {
        name = format!("scenes/{name}");
    }
    Some(Route {
        manifest: join(&base, &name),
        base,
        poll_ms: None,
        notify: None,
    })
}
```

Replace it with:

```rust
            notify,
        });
    }
    Some(self::named(named?, data))
}
```

**Locate this block:**

```rust
            }
        }
        _ => DATA_BASE.into(),
    };
    let named = scene
        .filter(|value| safe_scene(value))
        .or_else(|| path_scene(path));
```

Replace it with:

```rust
            }
        }
        _ => DATA_BASE.into(),
    }
}
pub fn named(path: &str, data: Option<&str>) -> Route {
    let base = data_base(data);
    let mut path = if path.contains('.') {
        path.to_owned()
    } else {
        format!("{path}.yaml")
    };
    if !path.contains('/') {
        path = format!("scenes/{path}");
    }
    Route {
        manifest: join(&base, &path),
        base,
        poll_ms: None,
        notify: None,
    }
}

pub fn select(
    scene: Option<&str>,
    path: &str,
    data: Option<&str>,
    live: Option<&str>,
    poll: Option<&str>,
    notify: Option<&str>,
) -> Option<Route> {
    let base = data_base(data);
    let named = scene
        .filter(|value| safe_scene(value))
        .or_else(|| path_scene(path));
```

**Locate this block:**

```rust
}

pub fn select(
    scene: Option<&str>,
    path: &str,
    data: Option<&str>,
    live: Option<&str>,
    poll: Option<&str>,
    notify: Option<&str>,
) -> Option<Route> {
    let base = match data {
        Some("off" | "") => String::new(),
        Some(value) if value.starts_with("https://") || local(value) => {
            if value.ends_with('/') {
```

Replace it with:

```rust
}

fn data_base(data: Option<&str>) -> String {
    match data {
        Some("off" | "") => String::new(),
        Some(value) if value.starts_with("https://") || local(value) => {
            if value.ends_with('/') {
```

### 2. `src/browser_publication.rs`

Keep one stable publication slot for browser callbacks. Let callback clones observe the current owner through the same slot. Retain a stable control slot even when startup has no scene route. Reuse one publication state constructor for startup and explicit replacement. Keep route-less reload calls as no-ops, matching the current entry point. Keep the existing callback identity when a scene is replaced. Read the latest state through the stable weak control slot. Cancel the old reader and move its replacement into the same slot.

**Locate this existing block:**

```rust
}
pub struct Publication(Option<Rc<RefCell<State>>>);
#[derive(Clone)]
pub struct Control(Option<Weak<RefCell<State>>>);
impl Publication {
    pub fn new(
        document: crate::drawing_document::Shared,
        fault: crate::gpu_fault::Fault,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        let q = url.search_params();
        let scene = q.get("scene");
        let data = q.get("data");
        let live = q.get("live");
        let poll = q.get("poll");
        let notify = q.get("notify");
        let Some(route) = crate::publication_route::select(
            scene.as_deref(),
            &window.location().pathname()?,
            data.as_deref(),
            live.as_deref(),
            poll.as_deref(),
            notify.as_deref(),
        ) else {
            return Ok(Self(None));
        };
        let index = Index::new(route.base.clone());
        Ok(Self(Some(Rc::new(RefCell::new(State {
            active: true,
            inflight: false,
            again: false,
            pending_sheets: Vec::new(),
            sheets: Vec::new(),
            pending_streams: Vec::new(),
            streams: Vec::new(),
            cancelled: Rc::new(Cell::new(false)),
            gate: Default::default(),
            abort: None,
            route,
            index: Some(index),
            document,
            fault,
            delivery: None,
            last_error: None,
            timer: None,
            tick: None,
            notify: None,
            message: None,
        })))))
    }
    pub fn control(&self) -> Control {
        Control(self.0.as_ref().map(Rc::downgrade))
    }
}
impl Drop for Publication {
    fn drop(&mut self) {
        self.control().cancel();
    }
}
impl Control {
    fn state(&self) -> Option<Rc<RefCell<State>>> {
        self.0.as_ref()?.upgrade()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
type Slot = RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);
#[derive(Clone)]
pub struct Control(Weak<Slot>);
impl Publication {
    pub fn new(
        document: crate::drawing_document::Shared,
        fault: crate::gpu_fault::Fault,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        let q = url.search_params();
        let scene = q.get("scene");
        let data = q.get("data");
        let live = q.get("live");
        let poll = q.get("poll");
        let notify = q.get("notify");
        let Some(route) = crate::publication_route::select(
            scene.as_deref(),
            &window.location().pathname()?,
            data.as_deref(),
            live.as_deref(),
            poll.as_deref(),
            notify.as_deref(),
        ) else {
            return Ok(Self(Rc::new(RefCell::new(None))));
        };
        Ok(Self::from_route(document, fault, route))
    }
    pub fn at(
        document: crate::drawing_document::Shared,
        fault: crate::gpu_fault::Fault,
        path: &str,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(
            document,
            fault,
            crate::publication_route::named(
                path,
                url.search_params().get("data").as_deref(),
            ),
        ))
    }
    fn from_route(
        document: crate::drawing_document::Shared,
        fault: crate::gpu_fault::Fault,
        route: Route,
    ) -> Self {
        let index = Index::new(route.base.clone());
        Self(Rc::new(RefCell::new(Some(Rc::new(
            RefCell::new(State {
                active: true,
                inflight: false,
                again: false,
                pending_sheets: Vec::new(),
                sheets: Vec::new(),
                pending_streams: Vec::new(),
                streams: Vec::new(),
                cancelled: Rc::new(Cell::new(false)),
                gate: Default::default(),
                abort: None,
                route,
                index: Some(index),
                document,
                fault,
                delivery: None,
                last_error: None,
                timer: None,
                tick: None,
                notify: None,
                message: None,
            }),
        )))))
    }
    pub fn replace(&mut self, next: Self) {
        self.control().cancel();
        *self.0.borrow_mut() = next.0.borrow_mut().take();
    }
    pub fn has_route(&self) -> bool {
        self.0.borrow().is_some()
    }
    pub fn control(&self) -> Control {
        Control(Rc::downgrade(&self.0))
    }
}
impl Drop for Publication {
    fn drop(&mut self) {
        self.control().cancel();
    }
}
impl Control {
    fn state(&self) -> Option<Rc<RefCell<State>>> {
        self.0.upgrade()?.borrow().clone()
    }
```

### 3. `src/browser_runtime.rs`

Export the current replacement entry point and start reading after the runtime borrow ends.

**Locate this existing block:**

```rust
    ACTIVE.with(|slot| slot.borrow().is_some())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ACTIVE.with(|slot| slot.borrow().is_some())
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn reload_scene(url: Option<String>) -> Result<(), wasm_bindgen::JsValue> {
    let control = ACTIVE.with(|slot| {
        let mut active = slot.borrow_mut();
        let runtime = active
            .as_mut()
            .ok_or("Viewer runtime is not running")?;
        let publication = match url {
            Some(path) => crate::browser_publication::Publication::at(
                runtime.document.clone(),
                runtime.fault.clone(),
                &path,
            )?,
            None => crate::browser_publication::Publication::new(
                runtime.document.clone(),
                runtime.fault.clone(),
            )?,
        };
        if !publication.has_route() {
            return Ok(None);
        }
        runtime.request.borrow_mut().cancel();
        runtime.reload.borrow_mut().cancel();
        runtime._publication.replace(publication);
        let control = runtime._publication.control();
        Ok::<_, wasm_bindgen::JsValue>(Some(control))
    })?;
    if let Some(control) = control {
        control.begin();
    }
    Ok(())
}
```

### 4. `src/reload_complete_tests.rs`

Supply actual history, original-owner and manifest-cache release assertions.

**Locate this existing block:**

```rust
    assert!(e.scene.objects()[0].geometry().is_none());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(e.scene.objects()[0].geometry().is_none());
}

#[test]
fn live_empty_manifest_releases_originals_history_and_cached_source_owners() {
    let mut editor = crate::editor::Editor::default();
    editor
        .apply(crate::editor::Action::Replace(
            crate::specimen::bytes(),
        ))
        .unwrap();
    let geometry =
        std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let origin = std::rc::Rc::downgrade(editor.scene.objects()[0].origin().unwrap());
    editor
        .apply(crate::editor::Action::SelectNext)
        .unwrap();
    editor
        .apply(crate::editor::Action::Translate([
            0.1, 0., 0.,
        ]))
        .unwrap();
    assert!(editor.scenes().count() > 1);
    let mut index = crate::publication_index::Index::new("files/".into());
    index
        .manifest(
            "scene.yaml",
            None,
            b"items: [{file: original.pb}]",
        )
        .unwrap();
    index
        .source(
            "files/original.pb",
            None,
            &crate::specimen::bytes(),
        )
        .unwrap();
    let cached = index
        .replacement(&crate::scene::Scene::empty())
        .unwrap()
        .unwrap();
    let cached_geometry =
        std::rc::Rc::downgrade(cached.objects()[0].geometry().unwrap());
    drop(cached);
    assert!(cached_geometry.upgrade().is_some());
    index
        .manifest(
            "scene.yaml",
            None,
            b"name: empty\nitems: []\n",
        )
        .unwrap();
    let empty = index
        .replacement(&editor.scene)
        .unwrap()
        .unwrap();
    assert!(!index.cached("files/original.pb"));
    assert!(cached_geometry.upgrade().is_none());
    editor
        .apply(crate::editor::Action::Live(Box::new(empty)))
        .unwrap();
    assert!(editor.scene.objects().is_empty());
    assert_eq!(editor.scenes().count(), 1);
    editor
        .apply(crate::editor::Action::Undo)
        .unwrap();
    editor
        .apply(crate::editor::Action::Redo)
        .unwrap();
    assert!(editor.scene.objects().is_empty());
    assert!(geometry.upgrade().is_none());
    assert!(origin.upgrade().is_none());
}
```

### 5. `src/publication_route.rs`

Supply current absolute, nested and bare named-scene routing checks.

**Locate this existing block:**

```rust
        assert_eq!(route.base, "https://example.test/");
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(route.base, "https://example.test/");
    }
}

#[cfg(test)]
#[test]
fn explicit_named_reload_uses_current_base_and_path_rules() {
    for (path, manifest) in [
        ("floor", "scenes/floor.yaml"),
        ("fixtures/empty.yaml", "fixtures/empty.yaml"),
        (
            "https://example.com/empty.yaml",
            "https://example.com/empty.yaml",
        ),
    ] {
        let route = named(path, Some("off"));
        assert_eq!(route.manifest, manifest);
        assert_eq!(route.base, "");
        assert_eq!(route.poll_ms, None);
        assert_eq!(route.notify, None);
    }
    assert_eq!(
        named("floor", Some("http://127.0.0.1:8780/data")).manifest,
        "http://127.0.0.1:8780/data/scenes/floor.yaml"
    );
}
```

## <a id="checkpoint-96zzx-update-only-changed-slot-runs"></a>4. Update only changed slot runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_gpu_support.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">queue.write_buffer</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged draw slots</text>
</svg>
</div>

**What this code does:** Patch changed instance slots without rewriting their unchanged neighbours.

### 1. `src/text_gpu_support.rs`

Write aligned existing rows without changing capacity or the active count.

**Locate this existing block:**

```rust
    }

    pub fn append<const N: usize>(&mut self, ctx: &GpuCtx, vertices: &[[f32; N]]) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn write_bytes_at(&self, ctx: &GpuCtx, at: u32, bytes: &[u8]) {
        assert_eq!(bytes.len() as u64 % self.stride, 0);
        let rows = bytes.len() as u64 / self.stride;
        assert!(u64::from(at) + rows <= u64::from(self.len));
        if !bytes.is_empty() {
            ctx.queue
                .write_buffer(&self.buf, u64::from(at) * self.stride, bytes);
        }
    }

    pub fn append<const N: usize>(&mut self, ctx: &GpuCtx, vertices: &[[f32; N]]) {
```

### 2. `src/draw_slots.rs`

Report the bytes actually uploaded by this slot update. Initialize upload accounting. Patch changed contiguous runs; append new rows and retain capacity on shrink.

**Locate this existing block:**

```rust
    previous: Vec<Slot>,
}
impl Slots {
    pub fn new(ctx: &GpuCtx) -> Self {
        let mut slots = Self {
            buffer: GrowBuf::new(
                ctx,
                "instance draw slots",
                8,
                wgpu::BufferUsages::VERTEX,
            ),
            previous: Vec::new(),
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) -> bool {
        if self.buffer.len() > 0 && self.previous == rows {
            return false;
        }
        let bytes: Vec<u8> = std::iter::once([OWN_ROW, 0])
            .chain(rows.iter().copied())
            .flatten()
            .flat_map(u32::to_ne_bytes)
            .collect();
        self.buffer.reset();
        self.buffer.append_bytes(ctx, &bytes);
        self.previous.clear();
        self.previous.extend_from_slice(rows);
        true
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    previous: Vec<Slot>,
    pub written_bytes: u64,
}
impl Slots {
    pub fn new(ctx: &GpuCtx) -> Self {
        let mut slots = Self {
            buffer: GrowBuf::new(
                ctx,
                "instance draw slots",
                8,
                wgpu::BufferUsages::VERTEX,
            ),
            previous: Vec::new(),
            written_bytes: 0,
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) -> bool {
        self.written_bytes = 0;
        let bytes = |rows: &[Slot]| {
            rows.iter()
                .flatten()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>()
        };
        if self.buffer.len() == 0 {
            self.buffer
                .append_bytes(ctx, &bytes(&[[OWN_ROW, 0]]));
            self.written_bytes = 8;
        }
        let inside = self.previous.len().min(rows.len());
        let mut at = 0;
        while at < inside {
            if self.previous[at] == rows[at] {
                at += 1;
                continue;
            }
            let first = at;
            while at < inside && self.previous[at] != rows[at] {
                at += 1;
            }
            self.buffer.write_bytes_at(
                ctx,
                (first + 1) as u32,
                &bytes(&rows[first..at]),
            );
            self.written_bytes += (at - first) as u64 * 8;
        }
        if rows.len() > self.previous.len() {
            let tail = bytes(&rows[inside..]);
            self.buffer.append_bytes(ctx, &tail);
            self.written_bytes += tail.len() as u64;
        } else if rows.len() < self.previous.len() {
            self.buffer.reset();
            self.buffer
                .append_bytes(ctx, &bytes(&[[OWN_ROW, 0]]));
            self.buffer.append_bytes(ctx, &bytes(rows));
            self.written_bytes = (rows.len() as u64 + 1) * 8;
        }
        self.previous.clear();
        self.previous.extend_from_slice(rows);
        self.written_bytes > 0
    }
```

### 3. `src/instance_gpu.rs`

Count partial slot uploads rather than every slot in the batch.

**Locate this existing block:**

```rust
                        self.writes += 1;
                        self.written_bytes += (slots.len() as u64 + 1) * 8;
                    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        self.writes += 1;
                        self.written_bytes += batch.slots.written_bytes;
                    }
```

### 4. `src/draw_slots_tests.rs`

Supply real GPU readback checks for partial writes, unchanged rows, growth, shrink and bounds.

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
fn actual_partial_slot_writes_preserve_neighbours_growth_and_sentinel() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let mut slots = crate::draw_slots::Slots::new(&ctx);
    let mut rows = vec![
        [7, 16_777_217],
        [9, u32::MAX - 1],
        [11, 33],
        [13, 44],
    ];
    slots.set(&ctx, &rows);
    let capacity = slots.allocated_bytes();
    let buffer = slots.buffer.buf.clone();
    let check = |slots: &crate::draw_slots::Slots, rows: &[[u32; 2]]| {
        let wanted = std::iter::once([u32::MAX, 0])
            .chain(rows.iter().copied())
            .flatten()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        assert_eq!(
            crate::growing_buffer_tests::read(
                &ctx,
                &slots.buffer.buf,
                wanted.len() as u64
            ),
            wanted
        );
    };
    rows[1] = [23, 0x80000000];
    assert!(slots.set(&ctx, &rows));
    assert_eq!(slots.written_bytes, 8);
    assert_eq!(slots.allocated_bytes(), capacity);
    assert_eq!(slots.buffer.buf, buffer);
    check(&slots, &rows);
    assert!(!slots.set(&ctx, &rows));
    assert_eq!(slots.written_bytes, 0);
    check(&slots, &rows);
    rows[0] = [31, 51];
    rows[2] = [37, 61];
    rows[3] = [41, 71];
    slots.set(&ctx, &rows);
    assert_eq!(slots.written_bytes, 24);
    check(&slots, &rows);
    rows.extend([[43, 81], [47, 91], [53, 101]]);
    slots.set(&ctx, &rows);
    assert_eq!(slots.written_bytes, 24);
    check(&slots, &rows);
    rows.truncate(2);
    let capacity = slots.allocated_bytes();
    slots.set(&ctx, &rows);
    assert_eq!(slots.written_bytes, 24);
    assert_eq!(slots.allocated_bytes(), capacity);
    check(&slots, &rows);
    slots.clear(&ctx, true);
    assert_eq!(slots.written_bytes, 8);
    assert_eq!(slots.allocated_bytes(), 8);
    check(&slots, &[]);
    let before = crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 8);
    let invalid = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        slots.buffer.write_bytes_at(&ctx, 1, &[0; 8])
    }));
    assert!(invalid.is_err());
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 8),
        before
    );
}
```

## <a id="checkpoint-96zzy-share-one-slot-buffer-across-surface-draws"></a>5. Share one slot buffer across surface draws

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one slot owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sentinel or batch range</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared surface vertex ent…</text>
</svg>
</div>

**What this code does:** Bind ordinary surfaces and every instance batch from one slot buffer.

### 1. `src/instance_gpu.rs`

Store a range start instead of another slot owner. Own the single surface slot buffer in the lane. Initialize the single sentinel owner. Use the current device queue for the initial sentinel. Keep the concatenated slot order identical to the ordered draw batches. Remove the unused group owner. Gather each batch into the same slot table. Use one queue context after gathering the batches. Remember the first shared slot; row zero remains reserved. Retain the batch’s range alongside its current records. Remove per-batch slot allocation and duplicate sentinels. Upload the shared rows once and release scene capacity when no batches remain. Bind each batch’s shared slot range. Let ordinary draws read the same sentinel owner. Count the shared scene allocation once; the empty sentinel remains a static pipeline resource.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .sum(),
            self.batches
                .values()
                .map(|b| b.records.size() + b.slots.allocated_bytes())
                .sum(),
            self.writes,
```

Replace it with:

```rust
                .sum(),
            self.batches
                .values()
                .map(|b| b.records.size())
                .sum::<u64>()
                + if self.batches.is_empty() {
                    0
                } else {
                    self.slots.allocated_bytes()
                },
            self.writes,
```

**Locate this block:**

```rust
                    let group = crate::gpu_layout::group(device, &self.layout, &buffer);
                    self.writes += 1;
                    self.written_bytes += bytes.len() as u64;
                    {
                        let mut slot_buffer = crate::draw_slots::Slots::new(&ctx);
                        slot_buffer.set(&ctx, &slots);
                        self.writes += 2;
                        self.written_bytes += 8 + (slots.len() as u64 + 1) * 8;
                        Batch {
                            geometry: geometry.clone(),
                            records: buffer,
                            group,
                            bytes,
                            count: records.len() as u32,
                            slots: slot_buffer,
                        }
                    }
                };
                batches.insert(key, batch);
            }
        }
        self.batches = batches;
        Ok(())
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            batch.slots.bind(pass, 0, 1);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
```

Replace it with:

```rust
                    let group = crate::gpu_layout::group(device, &self.layout, &buffer);
                    self.writes += 1;
                    self.written_bytes += bytes.len() as u64;
                    Batch {
                        geometry: geometry.clone(),
                        records: buffer,
                        group,
                        bytes,
                        count: records.len() as u32,
                        first_slot,
                    }
                };
                batches.insert(key, batch);
            }
        }
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        if batches.is_empty() && !old.is_empty() {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
        }
        if self.slots.written_bytes > 0 {
            self.writes += 1;
            self.written_bytes += self.slots.written_bytes;
        }
        self.batches = batches;
        Ok(())
    }
    pub fn bind_own(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.slots.bind(pass, 0, 0);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            self.slots.bind(pass, 0, batch.first_slot);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
```

**Locate this block:**

```rust
                    Rc::ptr_eq(&b.geometry, &geometry)
                        && b.records.size() >= bytes.len() as u64
                }) {
                    if batch.slots.set(&ctx, &slots) {
                        self.writes += 1;
                        self.written_bytes += batch.slots.written_bytes;
                    }
                    if batch.bytes != bytes {
                        queue.write_buffer(&batch.records, 0, &bytes);
                        self.writes += 1;
```

Replace it with:

```rust
                    Rc::ptr_eq(&b.geometry, &geometry)
                        && b.records.size() >= bytes.len() as u64
                }) {
                    batch.first_slot = first_slot;
                    if batch.bytes != bytes {
                        queue.write_buffer(&batch.records, 0, &bytes);
                        self.writes += 1;
```

**Locate this block:**

```rust
                        ]
                    })
                    .collect();
                let key = (source, part);
                let bytes = records
                    .iter()
```

Replace it with:

```rust
                        ]
                    })
                    .collect();
                let first_slot = u32::try_from(all_slots.len() + 1)
                    .map_err(|_| "Instance slot range")?;
                all_slots.extend_from_slice(&slots);
                let key = (source, part);
                let bytes = records
                    .iter()
```

**Locate this block:**

```rust
        }
        let mut old = std::mem::take(&mut self.batches);
        let mut batches = BTreeMap::new();
        for (source, (geometry, records)) in groups {
            for (part, records) in records.chunks(limit).enumerate() {
                let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
                let slots: Vec<crate::draw_slots::Slot> = records
                    .iter()
                    .enumerate()
```

Replace it with:

```rust
        }
        let mut old = std::mem::take(&mut self.batches);
        let mut batches = BTreeMap::new();
        let mut all_slots = Vec::new();
        for (source, (geometry, records)) in groups {
            for (part, records) in records.chunks(limit).enumerate() {
                let slots: Vec<crate::draw_slots::Slot> = records
                    .iter()
                    .enumerate()
```

**Locate this block:**

```rust
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: HashMap<usize, (Rc<GpuGeometry>, Vec<Record>)> = HashMap::new();
        for row in rows {
            let key = Rc::as_ptr(&row.geometry) as usize;
            groups
```

Replace it with:

```rust
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: BTreeMap<usize, (Rc<GpuGeometry>, Vec<Record>)> =
            BTreeMap::new();
        for row in rows {
            let key = Rc::as_ptr(&row.geometry) as usize;
            groups
```

**Locate this block:**

```rust
            pipeline,
            layout,
            batches: BTreeMap::new(),
            writes: 0,
            written_bytes: 0,
        }
    }
    pub fn set<'a>(
```

Replace it with:

```rust
            pipeline,
            layout,
            batches: BTreeMap::new(),
            slots: crate::draw_slots::Slots::new(
                &crate::text_gpu_support::GpuCtx::new(device, queue),
            ),
            writes: 1,
            written_bytes: 8,
        }
    }
    pub fn set<'a>(
```

**Locate this block:**

```rust
impl Lane {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        view: &wgpu::BindGroupLayout,
    ) -> Self {
```

Replace it with:

```rust
impl Lane {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        view: &wgpu::BindGroupLayout,
    ) -> Self {
```

**Locate this block:**

```rust
    group: wgpu::BindGroup,
    bytes: Vec<u8>,
    count: u32,
    slots: crate::draw_slots::Slots,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<(usize, usize), Batch>,
    pub writes: u64,
    pub written_bytes: u64,
}
```

Replace it with:

```rust
    group: wgpu::BindGroup,
    bytes: Vec<u8>,
    count: u32,
    first_slot: u32,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<(usize, usize), Batch>,
    pub(crate) slots: crate::draw_slots::Slots,
    pub writes: u64,
    pub written_bytes: u64,
}
```

**Locate this block:**

```rust
use crate::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use wgpu::util::DeviceExt;

struct Batch {
```

Replace it with:

```rust
use crate::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh};
use std::{collections::BTreeMap, rc::Rc};
use wgpu::util::DeviceExt;

struct Batch {
```

### 2. `src/renderer.rs`

Remove the duplicate ordinary slot owner. Pass the current queue to the lane. Keep only the lane’s sentinel. Construct one surface slot owner. Rebind the shared sentinel before ordinary surfaces or temporary highlights.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass, 0, 0);
            self.highlight.mesh(&mut pass);
```

Replace it with:

```rust
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
```

**Locate this block:**

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.own_slot.bind(&mut pass, 0, 0);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
```

Replace it with:

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
```

**Locate this block:**

```rust
            measurement,
            sheets,
            instances,
            own_slot,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
```

Replace it with:

```rust
            measurement,
            sheets,
            instances,
            instance_rows: Default::default(),
            clouds: Default::default(),
            cloud_view: Default::default(),
```

**Locate this block:**

```rust
            &device,
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let own_slot = crate::draw_slots::Slots::new(
            &crate::text_gpu_support::GpuCtx::new(&device, &queue),
        );
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

Replace it with:

```rust
            &device,
            &view_layout,
        ));
        let instances =
            crate::instance_gpu::Lane::new(&device, &queue, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

**Locate this block:**

```rust
    instances: crate::instance_gpu::Lane,
    own_slot: crate::draw_slots::Slots,
    instance_rows: std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
    instances: crate::instance_gpu::Lane,
    instance_rows: std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
```

### 3. `src/global_outline_tests.rs`

Retain the mapped sentinel and released-scene checks against the shared owner. Supply actual two-definition GPU drawing, mapped range, selection and release checks.

**Locate this existing block:**

```rust
        .collect::<Vec<_>>();
    assert_eq!(gpu.own_slot.allocated_bytes(), 8);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.own_slot.buffer.buf, 8),
        bytes
    );
    let originals = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let owner = scene.objects()[0].id;
    for opacity in [1., 0.5] {
        gpu.set_opacity(opacity);
        let plain = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        assert_ne!(pixels(&gpu), plain);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_highlight(&scene, None).unwrap();
        assert_eq!(pixels(&gpu), plain);
    }
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|o| o.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        originals
    );
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert_eq!(gpu.usage(), [0; 4]);
    assert_eq!(gpu.own_slot.allocated_bytes(), 8);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.own_slot.buffer.buf, 8),
        bytes
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .collect::<Vec<_>>();
    assert!(gpu.instances.slots.allocated_bytes() >= 8);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.instances.slots.buffer.buf, 8),
        bytes
    );
    let originals = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let owner = scene.objects()[0].id;
    for opacity in [1., 0.5] {
        gpu.set_opacity(opacity);
        let plain = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        assert_ne!(pixels(&gpu), plain);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_highlight(&scene, None).unwrap();
        assert_eq!(pixels(&gpu), plain);
    }
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|o| o.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        originals
    );
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert_eq!(gpu.usage(), [0; 4]);
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &gpu.instances.slots.buffer.buf, 8),
        bytes
    );
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing() {
    use prost::Message;
    let mut source = crate::instance_specimen::message();
    let mut second = source.definitions.as_ref().unwrap().meshes[0].clone();
    second.guid = "other-definition".into();
    source
        .definitions
        .as_mut()
        .unwrap()
        .meshes
        .push(second);
    source.objects.as_mut().unwrap().instances[1].definition_guid =
        "other-definition".into();
    source.objects.as_mut().unwrap().instances[0].xform = None;
    source.objects.as_mut().unwrap().instances[1].flags = 0;
    source.xforms = vec![
        session_rust::proto::XformEntry {
            guid: "first".into(),
            xform: Some(session_rust::Xform::translation(-0.6, 0., 0.).to_proto()),
        },
        session_rust::proto::XformEntry {
            guid: "second".into(),
            xform: Some(session_rust::Xform::translation(0.6, 0., 0.).to_proto()),
        },
    ];
    let mut scene = crate::scene::Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &source.encode_to_vec(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut gpu = crate::cap_gpu_tests::renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(&gpu.instance_usage()[..3], &[2, 2, 376]);
    assert_eq!(gpu.instances.slots.instances(), 2);
    let slots =
        crate::growing_buffer_tests::read(&ctx, &gpu.instances.slots.buffer.buf, 24);
    assert_eq!(
        &slots[..8],
        &[u32::MAX, 0]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        u32::from_ne_bytes(slots[8..12].try_into().unwrap()),
        0
    );
    assert_eq!(
        u32::from_ne_bytes(slots[16..20].try_into().unwrap()),
        0
    );
    let baseline = crate::cap_gpu_tests::pixels(&gpu);
    let uploads = gpu.stats()[0];
    let owner = scene.objects()[0].id;
    gpu.select_objects(&scene, &[owner]).unwrap();
    assert_ne!(crate::cap_gpu_tests::pixels(&gpu), baseline);
    gpu.select_objects(&scene, &[]).unwrap();
    assert_eq!(crate::cap_gpu_tests::pixels(&gpu), baseline);
    assert_eq!(gpu.stats()[0], uploads);
    gpu.set_opacity(0.5);
    crate::cap_gpu_tests::pixels(&gpu);
    gpu.set_opacity(1.);
    assert_eq!(crate::cap_gpu_tests::pixels(&gpu), baseline);
    gpu.set_scene(&crate::scene::Scene::empty(), None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}
```

## <a id="checkpoint-96zzz-prepare-the-integer-primitive-slot-texture"></a>6. Prepare the integer primitive-slot texture

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/primitive_slot_table.rs…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">slot index</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">texture row and column</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">four exact integer words</text>
</svg>
</div>

**What this code does:** Store slot lookup records in an integer texture with bounded partial uploads.

### 1. `src/primitive_slot_table.rs`

Prepare one RGBA32Uint lookup owner, full replacement and bounded row writes.

Create the file and type:

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH: u32 = 1024;
pub struct Table {
    pub(crate) texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}
fn texture(ctx: &GpuCtx, width: u32, height: u32) -> wgpu::Texture {
    ctx.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("primitive slot lookup"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
}
impl Table {
    pub fn new(ctx: &GpuCtx) -> Self {
        let texture = texture(ctx, 1, 1);
        let view = texture.create_view(&Default::default());
        Self { texture, view }
    }
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = u32::try_from(texels.len()).expect("Bounded slot table");
        let (width, height) = if count <= 1 {
            (1, 1)
        } else {
            (WIDTH, count.div_ceil(WIDTH))
        };
        assert!(height <= ctx.device.limits().max_texture_dimension_2d);
        if self.texture.width() != width || self.texture.height() != height {
            self.texture = texture(ctx, width, height);
            self.view = self.texture.create_view(&Default::default());
        }
        let mut data = texels.to_vec();
        data.resize((width * height) as usize, [0; 4]);
        self.write_at(ctx, 0, &data);
    }
    pub fn write_at(&self, ctx: &GpuCtx, first: u32, texels: &[[u32; 4]]) -> bool {
        let width = self.texture.width();
        if u64::from(first) + texels.len() as u64
            > u64::from(width) * u64::from(self.texture.height())
        {
            return false;
        }
        let mut at = first;
        let mut rest = texels;
        while !rest.is_empty() {
            let (x, y) = (at % width, at / width);
            let (row, tail) = rest.split_at(((width - x) as usize).min(rest.len()));
            let bytes: Vec<u8> = row
                .iter()
                .flatten()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            ctx.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    origin: wgpu::Origin3d { x, y, z: 0 },
                    ..self.texture.as_image_copy()
                },
                &bytes,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row.len() as u32 * 16),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: row.len() as u32,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
            at += row.len() as u32;
            rest = tail;
        }
        true
    }
    pub fn allocated_bytes(&self) -> u64 {
        u64::from(self.texture.width()) * u64::from(self.texture.height()) * 16
    }
}
```

### 2. `src/lib.rs`

Register the lookup resource without changing the existing draw path.

**Locate this existing block:**

```rust
mod draw_slots_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod draw_slots_tests;
mod primitive_slot_table;

#[cfg(test)]
```

### 3. `src/primitive_slot_table_tests.rs`

Supply actual integer texture readback, row-crossing, bounds and release checks.

Create the file and type:

```rust
use crate::{primitive_slot_table::Table, text_gpu_support::GpuCtx};
fn read(ctx: &GpuCtx, table: &Table) -> Vec<[u32; 4]> {
    let size = table.texture.size();
    let pitch = (size.width * 16 + 255) / 256 * 256;
    let bytes = u64::from(pitch) * u64::from(size.height);
    let output = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("slot texture readback"),
            size: bytes,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = ctx
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        table.texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &output,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(size.height),
            },
        },
        size,
    );
    ctx.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    output
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap()
        });
    ctx.device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let mapped = output.slice(..).get_mapped_range();
    let result = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| {
            row[..size.width as usize * 16]
                .chunks_exact(16)
                .map(|texel| {
                    std::array::from_fn(|i| {
                        u32::from_le_bytes(texel[i * 4..i * 4 + 4].try_into().unwrap())
                    })
                })
        })
        .collect();
    drop(mapped);
    output.unmap();
    result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut table = Table::new(&ctx);
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
    let mut wanted = (0..1050)
        .map(|i| [i, 0x80000000 + i, 16_777_217 + i, 3])
        .collect::<Vec<_>>();
    table.write(&ctx, &wanted);
    assert_eq!(
        [table.texture.width(), table.texture.height()],
        [1024, 2]
    );
    assert_eq!(
        table.texture.format(),
        wgpu::TextureFormat::Rgba32Uint
    );
    let texture = table.texture.clone();
    wanted.resize(2048, [0; 4]);
    assert_eq!(read(&ctx, &table), wanted);
    let changed = [
        [7, u32::MAX - 1, 31, 9],
        [11, 0x80000000, 37, 12],
        [13, 16_777_217, 41, 15],
    ];
    assert!(table.write_at(&ctx, 1023, &changed));
    wanted[1023..1026].copy_from_slice(&changed);
    assert_eq!(table.texture, texture);
    assert_eq!(read(&ctx, &table), wanted);
    assert!(!table.write_at(&ctx, u32::MAX, &changed));
    assert!(!table.write_at(&ctx, 2047, &changed));
    assert!(table.write_at(&ctx, 2048, &[]));
    assert_eq!(read(&ctx, &table), wanted);
    table.write(&ctx, &[]);
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
    assert!(!table.write_at(&ctx, 0, &changed));
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}
```

### 4. `src/lib.rs`

Register the optional GPU validation fixture.

**Locate this existing block:**

```rust
mod primitive_slot_table;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod primitive_slot_table;
#[cfg(test)]
mod primitive_slot_table_tests;

#[cfg(test)]
```

## <a id="checkpoint-96zzza-describe-slot-and-triangle-runs"></a>7. Describe slot and triangle runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs, src/lib.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">definition</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reserved slots and IDs</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">used members</text>
</svg>
</div>

**What this code does:** Represent reserved slot capacity separately from occupied members and triangle IDs.

### 1. `src/slot_space.rs`

Describe slot and ID capacity, definitions and change reports.

Create the file and type:

```rust
use crate::draw_slots::{OWN_ROW, Slot};
use std::{collections::HashMap, ops::Range};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Draw {
    pub faces: Range<u32>,
    pub pipes: Range<u32>,
    pub ribbons: Range<u32>,
    pub slots: Range<u32>,
}
fn headroom(arena: u32) -> u32 {
    arena / 64 + 1024
}
const SPARE_IDS: u32 = 4096;
fn grown_cap(len: u32, triangles: u32) -> u32 {
    len + len / 4 + (SPARE_IDS / triangles.max(1)).min(4)
}
pub struct Definition<'a> {
    pub key: u32,
    pub rows: &'a [u32],
    pub faces: Range<u32>,
    pub pipes: Range<u32>,
    pub ribbons: Range<u32>,
}
#[derive(Clone, Debug, PartialEq)]
struct Run {
    key: Option<u32>,
    start: u32,
    cap: u32,
    len: u32,
    ids: u32,
    span: u32,
    first: u32,
    triangles: u32,
    draw: Draw,
}
impl Run {
    fn free(start: u32, cap: u32, ids: u32, span: u32) -> Self {
        Self {
            key: None,
            start,
            cap,
            len: 0,
            ids,
            span,
            first: 0,
            triangles: 0,
            draw: Draw::default(),
        }
    }
    fn freed(&self) -> Self {
        Self::free(self.start, self.cap, self.ids, self.span)
    }
}
#[derive(Debug, Default, PartialEq)]
pub struct Change {
    pub all: bool,
    pub slots: Vec<Range<u32>>,
}
impl Change {
    fn all() -> Self {
        Self {
            all: true,
            slots: Vec::new(),
        }
    }
    #[cfg(test)]
    fn written(&self) -> Option<u32> {
        (!self.all).then(|| {
            self.slots
                .iter()
                .map(|range| range.len() as u32)
                .sum()
        })
    }
}
#[derive(Default)]
pub struct SlotSpace {
    rows: Vec<u32>,
    runs: Vec<Run>,
    base: u32,
    arena: u32,
    pub written: u32,
}
```

### 2. `src/lib.rs`

Register the preparatory allocator types.

**Locate this existing block:**

```rust
mod primitive_slot_table_tests;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod primitive_slot_table_tests;
mod slot_space;

#[cfg(test)]
```

## <a id="checkpoint-96zzzb-read-slot-runs-and-lookup-texels"></a>8. Read slot runs and lookup texels

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">CPU run</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">vertex slot and fragment…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">vertex slot and fragment…</text>
</svg>
</div>

**What this code does:** Derive GPU slot values and lookup texels from the same CPU run.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

**Locate this existing block:**

```rust
    pub written: u32,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub written: u32,
}

impl SlotSpace {
    pub fn end(&self) -> u32 {
        self.runs
            .last()
            .map_or(1, |run| run.start + run.cap)
    }
    pub fn triangle_count(&self) -> Option<u32> {
        let last = self.runs.last()?;
        let any = self
            .runs
            .iter()
            .any(|run| run.key.is_some() && run.len > 0);
        any.then(|| self.base + last.ids + last.span)
    }
    pub fn draws(&self) -> Vec<Draw> {
        let mut held: Vec<&Run> = Vec::new();
        for run in self
            .runs
            .iter()
            .filter(|run| run.key.is_some() && run.len > 0)
        {
            held.insert(
                held.partition_point(|other| other.key < run.key),
                run,
            );
        }
        held.iter()
            .map(|run| Draw {
                slots: run.start..run.start + run.len,
                ..run.draw.clone()
            })
            .collect()
    }
    fn run_at(&self, slot: u32) -> usize {
        self.runs
            .partition_point(|run| run.start + run.cap <= slot)
    }
    pub fn slot(&self, slot: u32) -> Slot {
        let run = &self.runs[self.run_at(slot)];
        let start = self.base + run.ids + (slot - run.start) * run.triangles;
        [
            self.rows[slot as usize],
            start.wrapping_sub(run.first),
        ]
    }
    pub fn texel(&self, slot: u32) -> [u32; 4] {
        if slot == 0 {
            return [self.end() - 1, self.arena, 0, 0];
        }
        let run = &self.runs[self.run_at(slot)];
        let j = slot - run.start;
        let start = self.base + run.ids + j * run.triangles;
        let count = if run.key.is_some() && j < run.len {
            run.triangles
        } else {
            0
        };
        [
            self.rows[slot as usize],
            start,
            run.first,
            count,
        ]
    }
}
fn shared_draw(definition: &Definition) -> Draw {
    Draw {
        faces: definition.faces.clone(),
        pipes: definition.pipes.clone(),
        ribbons: definition.ribbons.clone(),
        slots: 0..0,
    }
}
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
    let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        sorted.insert(
            sorted.partition_point(|r| r.start <= range.start),
            range,
        );
    }
    let mut out: Vec<Range<u32>> = Vec::with_capacity(sorted.len());
    for range in sorted {
        let range = range.start.min(end)..range.end.min(end);
        if range.is_empty() {
            continue;
        }
        match out.last_mut() {
            Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
            _ => out.push(range),
        }
    }
    out
}
```

## <a id="checkpoint-96zzzc-reuse-and-merge-free-slot-ranges"></a>9. Reuse and merge free slot ranges

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">free range</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fitting allocation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained remainder</text>
</svg>
</div>

**What this code does:** Allocate fitting holes, retain unused room and merge released neighbours.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

**Locate this existing block:**

```rust
    out
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    out
}

impl SlotSpace {
    fn hold(&mut self, at: usize, definition: &Definition) -> Range<u32> {
        let run = &mut self.runs[at];
        let len = definition.rows.len();
        run.key = Some(definition.key);
        run.len = len as u32;
        run.first = definition.faces.start / 3;
        run.triangles = definition.faces.len() as u32 / 3;
        run.draw = shared_draw(definition);
        let (start, end) = (
            run.start as usize,
            (run.start + run.cap) as usize,
        );
        if self.rows.len() < end {
            self.rows.resize(end, 0);
        }
        self.rows[start..start + len].copy_from_slice(definition.rows);
        start as u32..end as u32
    }
    fn allocate(&mut self, cap: u32, triangles: u32) -> usize {
        let span = cap * triangles;
        let fits = |run: &Run| run.key.is_none() && run.cap >= cap && run.span >= span;
        let Some(at) = self.runs.iter().position(fits) else {
            let (start, ids) = self.runs.last().map_or((1, 0), |run| {
                (run.start + run.cap, run.ids + run.span)
            });
            self.runs
                .push(Run::free(start, cap, ids, span));
            return self.runs.len() - 1;
        };
        let hole = self.runs[at].clone();
        if hole.cap > cap {
            self.runs[at] = Run::free(hole.start, cap, hole.ids, span);
            let rest = Run::free(
                hole.start + cap,
                hole.cap - cap,
                hole.ids + span,
                hole.span - span,
            );
            self.runs.insert(at + 1, rest);
        }
        at
    }
    fn merge_free(&mut self) {
        let mut merged: Vec<Run> = Vec::with_capacity(self.runs.len());
        for run in self.runs.drain(..) {
            match merged.last_mut() {
                Some(last) if last.key.is_none() && run.key.is_none() => {
                    last.cap += run.cap;
                    last.span += run.span;
                }
                _ => merged.push(run),
            }
        }
        if merged
            .last()
            .is_some_and(|run| run.key.is_none())
        {
            merged.pop();
        }
        self.runs = merged;
        self.rows.truncate(self.end() as usize);
    }
}
```

## <a id="checkpoint-96zzzd-follow-arena-growth-with-slot-headroom"></a>10. Follow arena growth with slot headroom

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">arena count</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reserved headroom</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">header update or renumber</text>
</svg>
</div>

**What this code does:** Preserve instance IDs while arena growth remains inside reserved headroom.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types.

**Locate this existing block:**

```rust
        self.rows.truncate(self.end() as usize);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.rows.truncate(self.end() as usize);
    }
}

impl SlotSpace {
    fn place_all(&mut self, definitions: &[Definition], base: u32) -> Change {
        self.runs.clear();
        self.rows.clear();
        self.rows.push(OWN_ROW);
        self.base = base;
        for definition in definitions {
            let len = definition.rows.len() as u32;
            if len > 0 {
                let at = self.allocate(len, definition.faces.len() as u32 / 3);
                self.hold(at, definition);
            }
        }
        Change::all()
    }
    pub fn reset(&mut self, definitions: &[Definition], arena: u32) -> Change {
        self.arena = arena;
        self.place_all(definitions, arena)
    }
    pub fn follow(&mut self, arena: u32) -> Change {
        if arena == self.arena {
            return Change::default();
        }
        self.arena = arena;
        if self.runs.is_empty() {
            self.base = arena;
            return Change::default();
        }
        if arena <= self.base && self.base - arena <= 2 * headroom(arena) {
            return Change {
                all: false,
                slots: vec![0..1],
            };
        }
        let held: Vec<(u32, Vec<u32>, Draw)> = self
            .runs
            .iter()
            .filter_map(|run| {
                let rows = self.rows
                    [run.start as usize..(run.start + run.len) as usize]
                    .to_vec();
                Some((run.key?, rows, run.draw.clone()))
            })
            .collect();
        let definitions: Vec<Definition> = held
            .iter()
            .map(|(key, rows, draw)| Definition {
                key: *key,
                rows,
                faces: draw.faces.clone(),
                pipes: draw.pipes.clone(),
                ribbons: draw.ribbons.clone(),
            })
            .collect();
        self.place_all(&definitions, arena + headroom(arena))
    }
    fn retire(
        &mut self,
        definitions: &[Definition],
        live: &HashMap<u32, usize>,
    ) -> Vec<Range<u32>> {
        let mut dirty = vec![0..1];
        for run in &mut self.runs {
            let Some(key) = run.key else {
                continue;
            };
            let same = live.get(&key).is_some_and(|&i| {
                let faces = &definitions[i].faces;
                faces.start / 3 == run.first && faces.len() as u32 / 3 == run.triangles
            });
            if !same {
                dirty.push(run.start..run.start + run.cap);
                *run = run.freed();
            }
        }
        dirty
    }
}
```

## <a id="checkpoint-96zzze-report-only-changed-instance-slots"></a>11. Report only changed instance slots

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/slot_space.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">touched members</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reused runs</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coalesced dirty ranges</text>
</svg>
</div>

**What this code does:** Update touched members and growing definitions while reporting bounded dirty ranges.

### 1. `src/slot_space.rs`

Build this day’s whole allocator methods on the preceding types. Supply the five original allocator fixtures, retaining their allocation and changed-range assertions.

**Locate this existing block:**

```rust
        dirty
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        dirty
    }
}

impl SlotSpace {
    pub fn update(
        &mut self,
        definitions: &[Definition],
        touched: &[(u32, u32)],
        arena: u32,
    ) -> Change {
        if self.follow(arena).all {
            return self.place_all(definitions, self.base);
        }
        let mut members: Vec<u32> = Vec::new();
        let live: HashMap<u32, usize> = (0..definitions.len())
            .map(|i| (definitions[i].key, i))
            .collect();
        let mut dirty = self.retire(definitions, &live);
        let held: HashMap<u32, usize> = self
            .runs
            .iter()
            .filter_map(|run| Some((run.key?, run.start as usize)))
            .collect();
        for &(key, position) in touched {
            let (Some(&start), Some(&i)) = (held.get(&key), live.get(&key)) else {
                continue;
            };
            let run = &self.runs[self.run_at(start as u32)];
            let rows = definitions[i].rows;
            if position < run.cap && (position as usize) < rows.len() {
                let slot = run.start + position;
                self.rows[slot as usize] = rows[position as usize];
                members.push(slot);
            }
        }
        for definition in definitions {
            let len = definition.rows.len() as u32;
            let grow = held
                .get(&definition.key)
                .map(|&start| self.run_at(start as u32));
            if let Some(at) = grow {
                let run = &mut self.runs[at];
                if len <= run.cap {
                    if len < run.len {
                        dirty.push(run.start + len..run.start + run.len);
                    }
                    run.len = len;
                    run.draw = shared_draw(definition);
                    continue;
                }
                dirty.push(run.start..run.start + run.cap);
                *run = run.freed();
            }
            if len == 0 {
                continue;
            }
            let triangles = definition.faces.len() as u32 / 3;
            let cap = if grow.is_some() {
                grown_cap(len, triangles)
            } else {
                len
            };
            let at = self.allocate(cap, triangles);
            let run = self.hold(at, definition);
            dirty.push(run);
        }
        self.merge_free();
        let (mut free, mut used, mut free_ids, mut used_ids) = (0, 0, 0u64, 0u64);
        for run in &self.runs {
            let ids = u64::from(run.len) * u64::from(run.triangles);
            free += run.cap - run.len;
            used += run.len;
            free_ids += u64::from(run.span) - ids;
            used_ids += ids;
        }
        let spare_ids = u64::from(SPARE_IDS) * self.runs.len() as u64;
        if free > used + 256 || free_ids > used_ids / 2 + spare_ids {
            return self.place_all(definitions, self.base);
        }
        members.sort_unstable();
        members.dedup();
        dirty.extend(
            members
                .chunk_by(|a, b| a + 1 == *b)
                .map(|run| run[0]..run[0] + run.len() as u32),
        );
        Change {
            all: false,
            slots: coalesce(dirty, self.end()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A definition of `triangles` triangles from face index `faces` with instance `rows`.
    fn definition(
        key: u32,
        rows: &[u32],
        faces: u32,
        triangles: u32,
    ) -> Definition<'_> {
        Definition {
            key,
            rows,
            faces: faces..faces + 3 * triangles,
            pipes: 0..0,
            ribbons: 0..0,
        }
    }

    /// Every used slot names its member and ids no other slot takes, past the arena's; the table
    /// is sorted by first id, as the shaders' binary search needs.
    fn check(space: &SlotSpace, members: &[(u32, Vec<u32>, u32)], arena: u32) {
        let mut ids = Vec::new();
        let texels: Vec<[u32; 4]> = (0..space.end())
            .map(|slot| space.texel(slot))
            .collect();
        assert_eq!(texels[0], [space.end() - 1, arena, 0, 0]);
        assert!(
            texels[1..]
                .windows(2)
                .all(|pair| pair[0][1] <= pair[1][1]),
            "sorted"
        );
        let draws = space.draws();

        for (key, rows, triangles) in members {
            let run = space
                .runs
                .iter()
                .find(|run| run.key == Some(*key))
                .expect("a run");
            let draw = draws
                .iter()
                .find(|d| d.slots.start == run.start)
                .expect("a draw");
            assert_eq!(draw.slots.len(), rows.len());

            for (position, row) in rows.iter().enumerate() {
                let slot = run.start + position as u32;
                let [texel_row, first, _, count] = texels[slot as usize];
                assert_eq!(
                    (texel_row, count),
                    (*row, *triangles),
                    "slot {slot}"
                );
                assert_eq!(space.slot(slot)[0], *row);
                assert_eq!(
                    space.slot(slot)[1].wrapping_add(run.first),
                    first
                );
                ids.push(first..first + count);
            }
        }

        ids.sort_by_key(|range| range.start);
        assert!(
            ids.windows(2)
                .all(|pair| pair[0].end <= pair[1].start),
            "ids overlap"
        );
        assert!(
            ids.first()
                .is_none_or(|range| range.start >= arena)
        );
        let count = space.triangle_count().unwrap_or(arena);
        assert!(
            ids.last()
                .is_none_or(|range| range.end <= count)
        );
    }

    /// Instances number their triangles one after another, past the arena's: each slot's base
    /// turns a definition triangle into its own id, and the table names the slot back.
    #[test]
    fn instance_ids_follow_the_arena() {
        let mut space = SlotSpace::default();
        let definitions = [
            definition(1, &[7, 8], 30, 2),
            definition(2, &[9], 60, 1),
        ];
        assert!(space.reset(&definitions, 100).all);
        assert_eq!(space.triangle_count(), Some(105));
        let texels: Vec<[u32; 4]> = (0..4)
            .map(|slot| space.texel(slot))
            .collect();
        let want = [
            [3, 100, 0, 0],
            [7, 100, 10, 2],
            [8, 102, 10, 2],
            [9, 104, 20, 1],
        ];
        assert_eq!(texels, want);
        // triangle 11 of the definition, drawn by the second slot, is id 103 zero-based
        assert_eq!(space.slot(2), [8, 92]);
        assert_eq!(space.slot(2)[1] + 11, 103);
        assert_eq!(space.slot(3)[1] + 20, 104);

        assert!(space.reset(&[], 100).all);
        assert_eq!(space.triangle_count(), None);
        assert_eq!(space.texel(0), [0, 100, 0, 0]);
    }

    /// Definitions of `members` (key, rows, triangles), the faces of key k at 30 * k.
    fn definitions(members: &[(u32, Vec<u32>, u32)]) -> Vec<Definition<'_>> {
        members
            .iter()
            .map(|(key, rows, triangles)| definition(*key, rows, 30 * *key, *triangles))
            .collect()
    }

    /// Members join and leave as the scene does, by swap removal: each edit writes its own slot
    /// and the head, a full run moves now and then, and the slots always match the members.
    #[test]
    fn edits_write_only_their_own_slots() {
        let mut members: Vec<(u32, Vec<u32>, u32)> = vec![
            (1, (0..40).collect(), 2),
            (2, (100..110).collect(), 5),
            (3, vec![200], 1),
        ];
        let mut space = SlotSpace::default();
        space.reset(&definitions(&members), 1000);
        check(&space, &members, 1000);
        let mut seed = 5u32;
        let mut next = |n: u32| {
            seed = seed
                .wrapping_mul(1_664_525)
                .wrapping_add(1_013_904_223);
            (seed >> 8) % n
        };
        let mut row = 1000;
        let mut moves = 0;

        for step in 0..600 {
            let batch = next(3) as usize;
            let (key, rows, _) = &mut members[batch];
            let mut touched = Vec::new();

            // join, or leave by swap removal
            if next(2) == 0 || rows.len() < 2 {
                touched.push((*key, rows.len() as u32));
                rows.push(row);
                row += 1;
            } else {
                let at = next(rows.len() as u32) as usize;
                rows.swap_remove(at);

                if at < rows.len() {
                    touched.push((*key, at as u32));
                }
            }

            let change = space.update(&definitions(&members), &touched, 1000);
            check(&space, &members, 1000);

            match change.written() {
                // the head, the member's slot and the slot a leaver vacated
                Some(written) if written > 3 => moves += 1,
                Some(_) => {}
                None => panic!("step {step} numbered everything again"),
            }
        }

        // a full run moves to room a quarter bigger: rarely
        assert!(moves < 60, "{moves} moves");
    }

    /// A heavy definition joined one instance at a time never reserves much more than its
    /// instances use: every reserved id costs the tile tables 96 bytes.
    #[test]
    fn heavy_definitions_reserve_few_ids() {
        let triangles = 870_000;
        let mut rows: Vec<u32> = vec![0];
        let mut space = SlotSpace::default();
        space.reset(&[definition(1, &rows, 0, triangles)], 1000);

        for row in 1..40 {
            let touched = [(1, rows.len() as u32)];
            rows.push(row);
            space.update(
                &[definition(1, &rows, 0, triangles)],
                &touched,
                1000,
            );
            check(&space, &[(1, rows.clone(), triangles)], 1000);
            let used = rows.len() as u64 * u64::from(triangles);
            let reserved = u64::from(space.triangle_count().unwrap() - space.base);
            assert!(
                reserved <= used * 3 / 2 + u64::from(SPARE_IDS),
                "{row}: {reserved} ids for {used}"
            );
        }
    }

    /// The last instance of a definition frees its room for another; a new definition takes it.
    #[test]
    fn freed_room_is_taken_again() {
        let mut space = SlotSpace::default();
        let (a, b): (Vec<u32>, Vec<u32>) = ((0..8).collect(), (10..14).collect());
        space.reset(
            &[
                definition(1, &a, 0, 2),
                definition(2, &b, 30, 2),
            ],
            50,
        );
        let change = space.update(&[definition(2, &b, 30, 2)], &[], 50);
        assert_eq!(
            change.written(),
            Some(9),
            "the head and the freed run"
        );
        let c: Vec<u32> = (20..26).collect();
        let change = space.update(
            &[
                definition(2, &b, 30, 2),
                definition(3, &c, 60, 1),
            ],
            &[],
            50,
        );
        assert_eq!(
            change.written(),
            Some(1 + 6),
            "the head and the room it took"
        );
        let run = space
            .runs
            .iter()
            .find(|run| run.key == Some(3))
            .unwrap();
        assert_eq!(run.start, 1, "the freed room at the front");
        check(
            &space,
            &[(2, b.clone(), 2), (3, c.clone(), 1)],
            50,
        );
    }

    /// The arena grows: within the headroom only the head changes, past it everything is
    /// numbered again with room to spare.
    #[test]
    fn the_arena_grows_into_the_headroom() {
        let mut space = SlotSpace::default();
        let rows: Vec<u32> = (0..5).collect();
        space.reset(&[definition(1, &rows, 0, 3)], 100);
        let change = space.follow(200);
        assert!(change.all, "past the ids: numbered again");
        check(&space, &[(1, rows.clone(), 3)], 200);
        assert_eq!(space.base, 200 + headroom(200));

        for arena in [210, 500, 900] {
            let change = space.follow(arena);
            assert_eq!(
                change.slots,
                vec![0..1],
                "{arena}: the head alone"
            );
            check(&space, &[(1, rows.clone(), 3)], arena);
        }

        assert!(space.follow(10_000).all);
        assert!(space.follow(10_000) == Change::default());
    }
}
```

## <a id="checkpoint-96zzzf-project-triangles-at-their-physical-addresses"></a>12. Project triangles at their physical addresses

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/tile_g…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">object primitive base</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">projected record interval</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite tile references</text>
</svg>
</div>

**What this code does:** Keep finite triangle records at the same addresses written into the primitive image.

### 1. `src/gpu_mesh.rs`

Expose the existing address owner rather than deriving another base.

**Locate this existing block:**

```rust
    }

    pub fn set_primitive_base(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn primitive_base(&self) -> u32 {
        self.primitive_base
    }
    pub fn set_primitive_base(
```

### 2. `src/tile_gpu.rs`

Invalidate projection when physical addresses change. Budget the largest physical interval, including gaps. Retain the base in the same geometry/model witness. Remove the second address accumulator. Write the object’s physical start into its projection metadata. Retain one address owner. Zero inactive records before projecting the active intervals.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            return;
        }
        for row in &mut self.rows {
```

Replace it with:

```rust
            }
            return;
        }
        encoder.clear_buffer(&self.projected, 0, None);
        for row in &mut self.rows {
```

**Locate this block:**

```rust
                triangles,
                group: None,
            });
            offset += triangles;
        }
        self.witness = witness;
        self.triangles = count.unwrap();
```

Replace it with:

```rust
                triangles,
                group: None,
            });
        }
        self.witness = witness;
        self.triangles = count.unwrap();
```

**Locate this block:**

```rust
            return;
        }
        self.rows.clear();
        let mut offset = 0;
        for row in meshes
            .iter()
            .filter(|row| row.geometry.index_count > 0)
        {
            let triangles = row.geometry.index_count / 3;
            let bytes = [triangles, offset, 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect::<Vec<_>>();
```

Replace it with:

```rust
            return;
        }
        self.rows.clear();
        for row in meshes
            .iter()
            .filter(|row| row.geometry.index_count > 0)
        {
            let triangles = row.geometry.index_count / 3;
            let bytes = [triangles, row.primitive_base(), 0, 0]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect::<Vec<_>>();
```

**Locate this block:**

```rust
                    indices,
                    row.model_buffer().clone(),
                    row.packed()[..64].try_into().unwrap(),
                )
            })
            .collect::<Vec<_>>();
```

Replace it with:

```rust
                    indices,
                    row.model_buffer().clone(),
                    row.packed()[..64].try_into().unwrap(),
                    row.primitive_base(),
                )
            })
            .collect::<Vec<_>>();
```

**Locate this block:**

```rust
                .max_storage_buffer_binding_size,
        )
        .min(device.limits().max_buffer_size);
        let count = meshes.iter().try_fold(0u32, |count, row| {
            count.checked_add(row.geometry.index_count / 3)
        });
        self.fallback =
            count.is_none_or(|count| u64::from(count) * PROJECTED_BYTES > limit);
        if self.fallback {
```

Replace it with:

```rust
                .max_storage_buffer_binding_size,
        )
        .min(device.limits().max_buffer_size);
        let count = meshes
            .iter()
            .filter(|row| row.geometry.index_count > 0)
            .try_fold(0u32, |end, row| {
                row.primitive_base()
                    .checked_add(row.geometry.index_count / 3)
                    .map(|next| end.max(next))
            });
        self.fallback =
            count.is_none_or(|count| u64::from(count) * PROJECTED_BYTES > limit);
        if self.fallback {
```

**Locate this block:**

```rust
    [u8; 64],
);
struct Row {
    vertices: wgpu::Buffer,
```

Replace it with:

```rust
    [u8; 64],
    u32,
);
struct Row {
    vertices: wgpu::Buffer,
```

### 3. `src/global_outline_tests.rs`

Supply actual mapped gap, relocated record, cache, pixel restoration and device-budget checks.

**Locate this existing block:**

```rust
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback() {
    let mut scene = Scene::empty();
    scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.4, -0.4, 0.],
                    [0.4, -0.4, 0.],
                    [0., 0.4, 0.],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let mut gpu = renderer(&scene);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let baseline = pixels(&gpu);
    let triangles = gpu.meshes[0].geometry.index_count / 3;
    assert_eq!(gpu.meshes.len(), 1);
    let initial = crate::growing_buffer_tests::read(
        &ctx,
        &gpu.tile_buffers()[0],
        u64::from(triangles) * 96,
    );
    let uploads = gpu.stats()[0];
    gpu.meshes[0]
        .set_primitive_base(&gpu.queue, 7)
        .unwrap();
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(gpu.tile_usage()[3], u64::from(triangles + 7));
    let projected = gpu.tile_buffers()[0].clone();
    let data = crate::growing_buffer_tests::read(
        &ctx,
        &projected,
        u64::from(triangles + 7) * 96,
    );
    assert!(data[..7 * 96].iter().all(|v| *v == 0));
    assert_eq!(&data[7 * 96..], initial);
    let projections = gpu.tile_usage()[5];
    pixels(&gpu);
    assert_eq!(gpu.tile_usage()[5], projections);
    gpu.meshes[0]
        .set_primitive_base(&gpu.queue, 3)
        .unwrap();
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(gpu.tile_usage()[3], u64::from(triangles + 3));
    assert_eq!(gpu.tile_buffers()[0], projected);
    let data = crate::growing_buffer_tests::read(
        &ctx,
        &gpu.tile_buffers()[0],
        projected.size(),
    );
    assert!(data[..3 * 96].iter().all(|v| *v == 0));
    assert_eq!(
        &data[3 * 96..(triangles as usize + 3) * 96],
        initial
    );
    assert!(
        data[(triangles as usize + 3) * 96..]
            .iter()
            .all(|v| *v == 0)
    );
    gpu.meshes[0]
        .set_primitive_base(&gpu.queue, 0)
        .unwrap();
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    assert_eq!(pixels(&gpu), baseline);
    assert_eq!(gpu.tile_usage()[3], u64::from(triangles));
    let current = gpu.tile_buffers()[0].clone();
    assert_eq!(current.size(), initial.len() as u64);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &current, current.size()),
        initial
    );
    assert_eq!(gpu.stats()[0], uploads);
    gpu.meshes[0]
        .set_primitive_base(&gpu.queue, 0x80000000)
        .unwrap();
    gpu.tiles
        .borrow_mut()
        .set(&gpu.device, &gpu.meshes);
    pixels(&gpu);
    assert!(gpu.tiles.borrow().fallback);
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}
```

## <a id="checkpoint-96zzzg-clear-the-current-arctic-background"></a>13. Clear the current Arctic background

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/ambien…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Arctic setting</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sRGB target</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stored background pixel</text>
</svg>
</div>

**What this code does:** Use the current Arctic background without another draw or geometry upload.

### 1. `src/renderer.rs`

Use the existing colour clear; no extra fullscreen pipeline is needed.

**Locate this existing block:**

```rust
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
        let [r, g, b] = if self.ambient_enabled.get() {
            [0.94; 3]
        } else {
            background.rgb()
        };
        let mut encoder = self
```

### 2. `src/ambient_gpu_tests.rs`

Keep the native UNORM contact-shadow assertion below the linear grey background, so the background itself cannot satisfy it. Measure surface darkening only on geometry pixels; the changed background is not ambient shading.

**Locate this existing block:**

```rust
        .filter(|(a, b)| {
            a[..3]
                .iter()
                .zip(&b[..3])
                .any(|(a, b)| a.saturating_sub(*b) > 3)
        })
        .count();
    assert!(
        darker > 100,
        "actual ambient shading: {darker} pixels"
    );
    let shadow = plain
        .chunks_exact(4)
        .zip(shaded.chunks_exact(4))
        .filter(|(a, b)| a[..3] == [255, 255, 255] && b[..3].iter().any(|v| *v < 250))
        .count();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .filter(|(a, b)| {
            a[..3] != [255, 255, 255]
                && a[..3]
                    .iter()
                    .zip(&b[..3])
                    .any(|(a, b)| a.saturating_sub(*b) > 3)
        })
        .count();
    assert!(
        darker > 100,
        "actual ambient shading: {darker} pixels"
    );
    let shadow = plain
        .chunks_exact(4)
        .zip(shaded.chunks_exact(4))
        .filter(|(a, b)| a[..3] == [255, 255, 255] && b[..3].iter().any(|v| *v < 235))
        .count();
```

## <a id="checkpoint-96zzzh-share-changed-record-runs"></a>14. Share changed record runs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/draw_slots.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">previous records</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed contiguous runs</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing bounded buffer w…</text>
</svg>
</div>

**What this code does:** Reuse the existing contiguous-change scan for slot and object records.

### 1. `src/draw_slots.rs`

Extract the existing comparison, retaining its order. Use one changed-run owner; keep slot zero outside the scan.

**Locate this existing block:**

```rust
}
pub struct Slots {
    pub(crate) buffer: GrowBuf,
    previous: Vec<Slot>,
    pub written_bytes: u64,
}
impl Slots {
    pub fn new(ctx: &GpuCtx) -> Self {
        let mut slots = Self {
            buffer: GrowBuf::new(
                ctx,
                "instance draw slots",
                8,
                wgpu::BufferUsages::VERTEX,
            ),
            previous: Vec::new(),
            written_bytes: 0,
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) -> bool {
        self.written_bytes = 0;
        let bytes = |rows: &[Slot]| {
            rows.iter()
                .flatten()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>()
        };
        if self.buffer.len() == 0 {
            self.buffer
                .append_bytes(ctx, &bytes(&[[OWN_ROW, 0]]));
            self.written_bytes = 8;
        }
        let inside = self.previous.len().min(rows.len());
        let mut at = 0;
        while at < inside {
            if self.previous[at] == rows[at] {
                at += 1;
                continue;
            }
            let first = at;
            while at < inside && self.previous[at] != rows[at] {
                at += 1;
            }
            self.buffer.write_bytes_at(
                ctx,
                (first + 1) as u32,
                &bytes(&rows[first..at]),
            );
            self.written_bytes += (at - first) as u64 * 8;
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub(crate) fn changed_runs<T: PartialEq>(
    previous: &[T],
    next: &[T],
) -> Vec<std::ops::Range<usize>> {
    let end = previous.len().min(next.len());
    let mut at = 0;
    let mut runs = Vec::new();
    while at < end {
        if previous[at] == next[at] {
            at += 1;
            continue;
        }
        let first = at;
        while at < end && previous[at] != next[at] {
            at += 1;
        }
        runs.push(first..at);
    }
    runs
}
pub struct Slots {
    pub(crate) buffer: GrowBuf,
    previous: Vec<Slot>,
    pub written_bytes: u64,
}
impl Slots {
    pub fn new(ctx: &GpuCtx) -> Self {
        let mut slots = Self {
            buffer: GrowBuf::new(
                ctx,
                "instance draw slots",
                8,
                wgpu::BufferUsages::VERTEX,
            ),
            previous: Vec::new(),
            written_bytes: 0,
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) -> bool {
        self.written_bytes = 0;
        let bytes = |rows: &[Slot]| {
            rows.iter()
                .flatten()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>()
        };
        if self.buffer.len() == 0 {
            self.buffer
                .append_bytes(ctx, &bytes(&[[OWN_ROW, 0]]));
            self.written_bytes = 8;
        }
        let inside = self.previous.len().min(rows.len());
        for run in changed_runs(&self.previous, rows) {
            self.buffer.write_bytes_at(
                ctx,
                (run.start + 1) as u32,
                &bytes(&rows[run.clone()]),
            );
            self.written_bytes += run.len() as u64 * 8;
        }
```

## <a id="checkpoint-96zzzi-prepare-one-shared-object-record-table"></a>15. Prepare one shared object record table

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/object_rows.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">packed object records</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">growing storage buffer</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current immutable binding</text>
</svg>
</div>

**What this code does:** Prepare one growable storage table without duplicating slot upload logic.

### 1. `src/object_rows.rs`

Reuse the aligned record, growing buffer and shared changed-run helper.

Create the file and type:

```rust
use crate::{
    object_record::{BYTES, Record},
    text_gpu_support::{GpuCtx, GrowBuf},
};
pub struct Rows {
    pub(crate) buffer: GrowBuf,
    group: wgpu::BindGroup,
    previous: Vec<Record>,
    pub written_bytes: u64,
}
impl Rows {
    pub fn new(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "shared object records",
            BYTES as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let group = crate::gpu_layout::group(&ctx.device, layout, &buffer.buf);
        Self {
            buffer,
            group,
            previous: Vec::new(),
            written_bytes: 0,
        }
    }
    pub fn set(
        &mut self,
        ctx: &GpuCtx,
        layout: &wgpu::BindGroupLayout,
        rows: &[Record],
    ) -> Result<bool, &'static str> {
        let limits = ctx.device.limits();
        let limit = limits.max_buffer_size.min(u64::from(
            limits.max_storage_buffer_binding_size,
        ));
        if rows.len() as u64 > limit / BYTES as u64
            || u32::try_from(rows.len()).is_err()
        {
            return Err("Object records exceed device limits");
        }
        self.written_bytes = 0;
        if rows.is_empty() {
            if !self.previous.is_empty() {
                self.buffer.release(ctx);
                self.group =
                    crate::gpu_layout::group(&ctx.device, layout, &self.buffer.buf);
                self.previous = Vec::new();
            }
            return Ok(false);
        }
        let bytes = |rows: &[Record]| {
            rows.iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
        };
        let mut grew = false;
        if rows.len() < self.previous.len() {
            self.buffer.reset();
            let data = bytes(rows);
            grew = self.buffer.append_bytes(ctx, &data);
            self.written_bytes = data.len() as u64;
        } else {
            for run in crate::draw_slots::changed_runs(&self.previous, rows) {
                let data = bytes(&rows[run.clone()]);
                self.buffer
                    .write_bytes_at(ctx, run.start as u32, &data);
                self.written_bytes += data.len() as u64;
            }
            let tail = bytes(&rows[self.previous.len()..]);
            if !tail.is_empty() {
                grew = self.buffer.append_bytes(ctx, &tail);
                self.written_bytes += tail.len() as u64;
            }
        }
        if grew {
            self.group =
                crate::gpu_layout::group(&ctx.device, layout, &self.buffer.buf);
        }
        self.previous.clear();
        self.previous.extend_from_slice(rows);
        Ok(self.written_bytes > 0)
    }
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.group, &[]);
    }
    pub fn len(&self) -> u32 {
        self.previous.len() as u32
    }
    pub fn allocated_bytes(&self) -> u64 {
        if self.previous.is_empty() {
            0
        } else {
            self.buffer.buf.size()
        }
    }
}
```

### 2. `src/lib.rs`

Expose the preparatory owner.

**Locate this existing block:**

```rust
mod draw_slots_tests;
mod primitive_slot_table;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod draw_slots_tests;
mod object_rows;
mod primitive_slot_table;
```

### 3. `src/object_rows.rs`

Supply mapped GPU checks for unchanged neighbors, high-bit fields, growth, binding replacement and release.

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
    fn shared_object_rows_keep_neighbours_growth_and_release() {
        let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
        let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
        let layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(
                                crate::object_record::BYTES as u64,
                            ),
                        },
                        count: None,
                    }],
                });
        let mut table = Rows::new(&ctx, &layout);
        let mut rows = vec![[7; BYTES], [9; BYTES], [11; BYTES]];
        let check = |table: &Rows, rows: &[Record]| {
            let bytes = rows
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            assert_eq!(
                crate::growing_buffer_tests::read(
                    &ctx,
                    &table.buffer.buf,
                    bytes.len() as u64
                ),
                bytes
            );
        };
        table.set(&ctx, &layout, &rows).unwrap();
        check(&table, &rows);
        let buffer = table.buffer.buf.clone();
        let group = table.group.clone();
        rows[1][68..72].copy_from_slice(&0x80000001u32.to_ne_bytes());
        table.set(&ctx, &layout, &rows).unwrap();
        assert_eq!(table.written_bytes, BYTES as u64);
        assert_eq!(table.buffer.buf, buffer);
        assert_eq!(table.group, group);
        check(&table, &rows);
        assert!(!table.set(&ctx, &layout, &rows).unwrap());
        assert_eq!(table.written_bytes, 0);
        rows.extend([
            [13; BYTES],
            [17; BYTES],
            [19; BYTES],
            [23; BYTES],
        ]);
        table.set(&ctx, &layout, &rows).unwrap();
        assert_ne!(table.buffer.buf, buffer);
        assert_ne!(table.group, group);
        check(&table, &rows);
        let capacity = table.allocated_bytes();
        rows.truncate(2);
        table.set(&ctx, &layout, &rows).unwrap();
        assert_eq!(table.allocated_bytes(), capacity);
        check(&table, &rows);
        table.set(&ctx, &layout, &[]).unwrap();
        assert_eq!(table.len(), 0);
        assert_eq!(table.allocated_bytes(), 0);
        assert_eq!(table.previous.capacity(), 0);
        assert_eq!(table.buffer.buf.size(), BYTES as u64);
        table.set(&ctx, &layout, &rows).unwrap();
        check(&table, &rows);
    }
}
```

## <a id="checkpoint-96zzzj-share-instance-object-records-across-definitions"></a>16. Share instance object records across definitions

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/gl…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original placements</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">global slot row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged pulled surface…</text>
</svg>
</div>

**What this code does:** Draw distinct definitions from one shared placement-record buffer.

### 1. `src/instance_gpu.rs`

Remove batch-specific buffer creation. A batch retains shared geometry and its draw interval only. The lane owns one object-record table. Bind the existing instance layout to the shared table. Number records across definitions before publishing any GPU change. Reuse changed-record writes, current bindings and empty release. Every definition reads the same current object table. Count the shared allocation once. One shared table removes the old per-definition chunk key. Retain each original geometry definition once. Expose the current shared buffer for the authoring GPU readback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .values()
                .map(|b| u64::from(b.count))
                .sum(),
            self.batches
                .values()
                .map(|b| b.records.size())
                .sum::<u64>()
                + if self.batches.is_empty() {
```

Replace it with:

```rust
                .values()
                .map(|b| u64::from(b.count))
                .sum(),
            self.records.allocated_bytes()
                + if self.batches.is_empty() {
```

**Locate this block:**

```rust
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            self.slots.bind(pass, 0, batch.first_slot);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
    pub fn usage(&self) -> [u64; 5] {
        [
            self.batches.len() as u64,
```

Replace it with:

```rust
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {
            self.slots.bind(pass, 0, batch.first_slot);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
    pub(crate) fn record_buffer(&self) -> &wgpu::Buffer {
        &self.records.buffer.buf
    }
    pub fn usage(&self) -> [u64; 5] {
        [
            self.batches.len() as u64,
```

**Locate this block:**

```rust
                .1
                .push(row.packed());
        }
        let mut old = std::mem::take(&mut self.batches);
        let mut batches = BTreeMap::new();
        let mut all_slots = Vec::new();
        for (source, (geometry, records)) in groups {
            for (part, records) in records.chunks(limit).enumerate() {
                let slots: Vec<crate::draw_slots::Slot> = records
                    .iter()
                    .enumerate()
                    .map(|(row, record)| {
                        [
                            row as u32,
                            u32::from_ne_bytes(
                                record[crate::object_record::RANGE + 4
                                    ..crate::object_record::RANGE + 8]
                                    .try_into()
                                    .unwrap(),
                            ),
                        ]
                    })
                    .collect();
                let first_slot = u32::try_from(all_slots.len() + 1)
                    .map_err(|_| "Instance slot range")?;
                all_slots.extend_from_slice(&slots);
                let key = (source, part);
                let bytes = records
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>();
                let batch = if let Some(mut batch) = old.remove(&key).filter(|b| {
                    Rc::ptr_eq(&b.geometry, &geometry)
                        && b.records.size() >= bytes.len() as u64
                }) {
                    batch.first_slot = first_slot;
                    if batch.bytes != bytes {
                        queue.write_buffer(&batch.records, 0, &bytes);
                        self.writes += 1;
                        self.written_bytes += bytes.len() as u64;
                        batch.bytes = bytes;
                    }
                    batch.count = records.len() as u32;
                    batch
                } else {
                    let buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("independent source placements"),
                            contents: &bytes,
                            usage: wgpu::BufferUsages::STORAGE
                                | wgpu::BufferUsages::COPY_DST,
                        });
                    let group = crate::gpu_layout::group(device, &self.layout, &buffer);
                    self.writes += 1;
                    self.written_bytes += bytes.len() as u64;
                    Batch {
                        geometry: geometry.clone(),
                        records: buffer,
                        group,
                        bytes,
                        count: records.len() as u32,
                        first_slot,
                    }
                };
                batches.insert(key, batch);
            }
        }
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        if batches.is_empty() && !old.is_empty() {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
```

Replace it with:

```rust
                .1
                .push(row.packed());
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
        let mut batches = BTreeMap::new();
        let mut all_slots = Vec::new();
        let mut all_records = Vec::with_capacity(count);
        for (source, (geometry, records)) in groups {
            let first_slot = u32::try_from(all_slots.len() + 1)
                .map_err(|_| "Instance slot range")?;
            for record in &records {
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
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        self.records
            .set(&ctx, &self.layout, &all_records)?;
        if self.records.written_bytes > 0 {
            self.writes += 1;
            self.written_bytes += self.records.written_bytes;
        }
        if batches.is_empty() && !self.batches.is_empty() {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
```

**Locate this block:**

```rust
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            layout,
            batches: BTreeMap::new(),
            slots: crate::draw_slots::Slots::new(
                &crate::text_gpu_support::GpuCtx::new(device, queue),
            ),
```

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        let records = crate::object_rows::Rows::new(
            &crate::text_gpu_support::GpuCtx::new(device, queue),
            &layout,
        );
        Self {
            pipeline,
            layout,
            batches: BTreeMap::new(),
            records,
            slots: crate::draw_slots::Slots::new(
                &crate::text_gpu_support::GpuCtx::new(device, queue),
            ),
```

**Locate this block:**

```rust
use std::{collections::BTreeMap, rc::Rc};
use wgpu::util::DeviceExt;

struct Batch {
    geometry: Rc<GpuGeometry>,
    records: wgpu::Buffer,
    group: wgpu::BindGroup,
    bytes: Vec<u8>,
    count: u32,
    first_slot: u32,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<(usize, usize), Batch>,
    pub(crate) slots: crate::draw_slots::Slots,
    pub writes: u64,
    pub written_bytes: u64,
```

Replace it with:

```rust
use std::{collections::BTreeMap, rc::Rc};

struct Batch {
    geometry: Rc<GpuGeometry>,
    count: u32,
    first_slot: u32,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<usize, Batch>,
    records: crate::object_rows::Rows,
    pub(crate) slots: crate::draw_slots::Slots,
    pub writes: u64,
    pub written_bytes: u64,
```

### 2. `src/global_outline_tests.rs`

The second definition now addresses global row one, while the ordinary sentinel stays unchanged. Supply exact mapped shared records and actual original-colored pixels in both placed halves; retain selection, opacity restoration and release assertions.

**Locate this existing block:**

```rust
        u32::from_ne_bytes(slots[16..20].try_into().unwrap()),
        0
    );
    let baseline = crate::cap_gpu_tests::pixels(&gpu);
    let uploads = gpu.stats()[0];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        u32::from_ne_bytes(slots[16..20].try_into().unwrap()),
        1
    );
    let records = crate::growing_buffer_tests::read(
        &ctx,
        gpu.instances.record_buffer(),
        2 * crate::object_record::BYTES as u64,
    );
    let mut actual = records
        .chunks_exact(crate::object_record::BYTES)
        .map(|r| r.to_vec())
        .collect::<Vec<_>>();
    actual.sort_unstable();
    let mut expected = gpu
        .meshes
        .iter()
        .map(|m| m.packed().to_vec())
        .collect::<Vec<_>>();
    expected.sort_unstable();
    assert_eq!(
        actual, expected,
        "global records retain both original placements, IDs and ranges"
    );
    let baseline = crate::cap_gpu_tests::pixels(&gpu);
    for half in 0..2 {
        let blue = baseline
            .chunks_exact(4)
            .enumerate()
            .filter(|(i, p)| {
                (*i % 320) / 160 == half
                    && p[2] > p[0].saturating_add(50)
                    && p[2] > p[1].saturating_add(30)
            })
            .count();
        assert!(
            blue > 100,
            "placed original blue triangles in half {half}: {blue}"
        );
    }
    let uploads = gpu.stats()[0];
```

## <a id="checkpoint-96zzzk-reuse-checked-curve-control-coordinates"></a>17. Reuse checked curve control coordinates

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/controls.rs, src/curve.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original curve and index</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared checked control co…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sampling or marker display</text>
</svg>
</div>

**What this code does:** Use one checked rational-control decoder for sampling and control markers.

### 1. `src/controls.rs`

Move the checked method already written into this module position; the next edit changes its signature. The body is reused, not a supplied new algorithm. Give the moved body a borrowed curve and index; remove the former self binding. Keep the source owner and existing public coordinate API.

**Locate this existing block:**

```rust
use std::rc::Rc;

#[derive(Clone)]
pub struct Control {
    pub source: Rc<NurbsCurve>,
    pub index: usize,
}

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index };
        control.position()?;
        Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim)
            || !(1..=10_000).contains(&c.m_cv_count)
            || self.index >= c.m_cv_count
        {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size {
            return Err("Control stride is too short");
        }
        let start = self
            .index
            .checked_mul(c.m_cv_stride)
            .ok_or("Control offset overflows")?;
        let end = start
            .checked_add(size)
            .ok_or("Control range overflows")?;
        let values = c
            .m_cv
            .get(start..end)
            .ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat {
            values[c.m_dim]
        } else {
            1.0
        };
        if !weight.is_finite() || weight == 0.0 {
            return Err("Control weight must be finite and nonzero");
        }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim {
            point[axis] = values[axis] / weight;
        }
        if point.iter().any(|v| !v.is_finite()) {
            return Err("Control position must be finite");
        }
        Ok(point)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;

pub(crate) fn position(c: &NurbsCurve, index: usize) -> Result<[f64; 3], &'static str> {
    if !(2..=3).contains(&c.m_dim)
        || !(1..=10_000).contains(&c.m_cv_count)
        || index >= c.m_cv_count
    {
        return Err("Control dimension, count or index is invalid");
    }
    let size = c.m_dim + usize::from(c.m_is_rat);
    if c.m_cv_stride < size {
        return Err("Control stride is too short");
    }
    let start = index
        .checked_mul(c.m_cv_stride)
        .ok_or("Control offset overflows")?;
    let end = start
        .checked_add(size)
        .ok_or("Control range overflows")?;
    let values = c
        .m_cv
        .get(start..end)
        .ok_or("Control values are incomplete")?;
    let weight = if c.m_is_rat {
        values[c.m_dim]
    } else {
        1.0
    };
    if !weight.is_finite() || weight == 0.0 {
        return Err("Control weight must be finite and nonzero");
    }
    let mut point = [0.0; 3];
    for axis in 0..c.m_dim {
        point[axis] = values[axis] / weight;
    }
    if point.iter().any(|v| !v.is_finite()) {
        return Err("Control position must be finite");
    }
    Ok(point)
}

#[derive(Clone)]
pub struct Control {
    pub source: Rc<NurbsCurve>,
    pub index: usize,
}

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index };
        control.position()?;
        Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        position(&self.source, self.index)
    }
```

### 2. `src/curve.rs`

Remove the second rational-coordinate decoder. Reuse checked coordinates without an Rc clone, retaining the sampler’s existing extent error.

**Locate this existing block:**

```rust
    let mut previous: Option<[f64; 3]> = None;
    let control = |index: usize| {
        let offset = index * source.m_cv_stride;
        let weight = if source.m_is_rat {
            source.m_cv[offset + source.m_dim]
        } else {
            1.0
        };
        std::array::from_fn::<_, 3, _>(|axis| {
            if axis < source.m_dim {
                source.m_cv[offset + axis] / weight
            } else {
                0.0
            }
        })
    };
    for index in 1..source.m_cv_count {
        let a = control(index - 1);
        let b = control(index);
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut previous: Option<[f64; 3]> = None;
    for index in 1..source.m_cv_count {
        let a = crate::controls::position(source, index - 1)
            .map_err(|_| "Curve control extent exceeds the display range")?;
        let b = crate::controls::position(source, index)
            .map_err(|_| "Curve control extent exceeds the display range")?;
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
```

## <a id="checkpoint-96zzzl-publish-allocator-texels-through-the-integer-table"></a>18. Publish allocator texels through the integer table

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/primitive_slot_table.rs…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">SlotSpace change</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">full or bounded texture w…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current texture view</text>
</svg>
</div>

**What this code does:** Connect allocator changes to the existing exact integer texture writer.

### 1. `src/primitive_slot_table.rs`

Share the existing texture-shape rule instead of repeating it. Keep full writes and allocator publication on the same dimensions. Publish existing dirty ranges and report whether immutable consumer bindings must be rebuilt.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        true
    }
```

Replace it with:

```rust
        }
        true
    }
    pub fn publish(
        &mut self,
        ctx: &GpuCtx,
        space: &crate::slot_space::SlotSpace,
        change: &crate::slot_space::Change,
    ) -> bool {
        let previous = self.texture.clone();
        let (width, height) = dimensions(space.end());
        if change.all
            || self.texture.width() != width
            || self.texture.height() != height
        {
            let texels = (0..space.end())
                .map(|slot| space.texel(slot))
                .collect::<Vec<_>>();
            self.write(ctx, &texels);
        } else {
            for range in &change.slots {
                assert!(range.start <= range.end && range.end <= space.end());
                let texels = range
                    .clone()
                    .map(|slot| space.texel(slot))
                    .collect::<Vec<_>>();
                assert!(self.write_at(ctx, range.start, &texels));
            }
        }
        self.texture != previous
    }
```

**Locate this block:**

```rust
    }
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = u32::try_from(texels.len()).expect("Bounded slot table");
        let (width, height) = if count <= 1 {
            (1, 1)
        } else {
            (WIDTH, count.div_ceil(WIDTH))
        };
        assert!(height <= ctx.device.limits().max_texture_dimension_2d);
        if self.texture.width() != width || self.texture.height() != height {
            self.texture = texture(ctx, width, height);
```

Replace it with:

```rust
    }
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = u32::try_from(texels.len()).expect("Bounded slot table");
        let (width, height) = dimensions(count);
        assert!(height <= ctx.device.limits().max_texture_dimension_2d);
        if self.texture.width() != width || self.texture.height() != height {
            self.texture = texture(ctx, width, height);
```

**Locate this block:**

```rust
}
fn texture(ctx: &GpuCtx, width: u32, height: u32) -> wgpu::Texture {
    ctx.device
        .create_texture(&wgpu::TextureDescriptor {
```

Replace it with:

```rust
}
fn dimensions(count: u32) -> (u32, u32) {
    if count <= 1 {
        (1, 1)
    } else {
        (WIDTH, count.div_ceil(WIDTH))
    }
}
fn texture(ctx: &GpuCtx, width: u32, height: u32) -> wgpu::Texture {
    ctx.device
        .create_texture(&wgpu::TextureDescriptor {
```

### 2. `src/primitive_slot_table_tests.rs`

Supply actual mapped allocator words, high-bit rows, retired slots, unchanged neighbors and zero release.

**Locate this existing block:**

```rust
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
    use crate::slot_space::{Definition, SlotSpace};
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
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
    assert!(table.publish(&ctx, &space, &change));
    let texture = table.texture.clone();
    let bytes = read(&ctx, &table);
    assert_eq!(
        &bytes[..4],
        &[
            [3, 100, 0, 0],
            [7, 100, 10, 2],
            [0x80000001, 102, 10, 2],
            [u32::MAX - 1, 104, 20, 1]
        ]
    );
    a[0] = 0x80000007;
    let change = space.update(&definitions(&a, &b), &[(1, 0)], 100);
    assert!(!change.all);
    assert!(!table.publish(&ctx, &space, &change));
    assert_eq!(table.texture, texture);
    let next = read(&ctx, &table);
    assert_eq!(next[1], [0x80000007, 100, 10, 2]);
    assert_eq!(&next[2..], &bytes[2..]);
    let remaining = [Definition {
        key: 2,
        rows: &b,
        faces: 60..63,
        pipes: 0..0,
        ribbons: 0..0,
    }];
    let change = space.update(&remaining, &[], 100);
    table.publish(&ctx, &space, &change);
    let next = read(&ctx, &table);
    assert_eq!(next[1][3], 0);
    assert_eq!(next[2][3], 0);
    assert_eq!(next[3], [u32::MAX - 1, 104, 20, 1]);
    let change = space.reset(&[], 0);
    assert!(table.publish(&ctx, &space, &change));
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}
```

## <a id="checkpoint-96zzzm-resolve-physical-triangles-with-integer-slots"></a>19. Resolve physical triangles with integer slots

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/primitive_slot_lookup.w…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One-based physical ID</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unsigned table search</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">definition triangle and o…</text>
</svg>
</div>

**What this code does:** Resolve a physical triangle ID back to its definition triangle and object row.

### 1. `src/primitive_slot_lookup.wgsl`

Resolve one-based physical IDs with unsigned integer texels and reject unused ranges.

Create the file and type:

```wgsl
const SLOT_OWN_ROW: u32 = 0xffffffffu;
const NO_TRIANGLE: u32 = 0xffffffffu;
fn slot_texel(i: u32) -> vec4<u32> {
 return textureLoad(slot_table, vec2<u32>(i & 1023u, i >> 10u), 0);
}
fn placed_triangle(id: u32, arena: u32) -> vec2<u32> {
 let head = slot_texel(0u);
 if (head.x == 0u || id <= head.y) {
  let own = id != 0u && id <= select(arena, head.y, head.x != 0u);
  return vec2<u32>(select(NO_TRIANGLE, id - 1u, own), SLOT_OWN_ROW);
 }
 let index = id - 1u;
 var lo = 1u;
 var hi = head.x;
 while (lo < hi) {
  let mid = (lo + hi + 1u) / 2u;
  if (slot_texel(mid).y <= index) { lo = mid; }
  else { hi = mid - 1u; }
 }
 let slot = slot_texel(lo);
 if (index - slot.y >= slot.w) { return vec2<u32>(NO_TRIANGLE, SLOT_OWN_ROW); }
 return vec2<u32>(slot.z + index - slot.y, slot.x);
}
```

### 2. `src/primitive_slot_table_tests.rs`

Supply an actual GPU dispatch through the taught shader; check original IDs, high-bit rows, retired holes and an empty table.

**Locate this existing block:**

```rust
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(table.allocated_bytes(), 16);
    assert_eq!(read(&ctx, &table), vec![[0; 4]]);
}

#[test]
fn actual_slot_shader_resolves_arena_instances_holes_high_rows_and_empty_table() {
    use crate::slot_space::{Definition, SlotSpace};
    use wgpu::util::DeviceExt;
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut table = Table::new(&ctx);
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

## <a id="checkpoint-96zzzn-batch-shared-geometry-with-original-ownership"></a>20. Batch shared geometry with ownership

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/pick_g…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Visible original rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">object-record batch</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original IDs</text>
</svg>
</div>

**What this code does:** Reuse instance drawing for copied originals sharing GPU geometry.

### 1. `src/renderer.rs`

Batch reused GPU geometry without turning copied originals into kernel InstanceRef owners.

**Locate this existing block:**

```rust
            .set(&self.device, &self.meshes);
        self.instance_rows = scene.instances.keys().copied().collect();
        self.instances
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .set(&self.device, &self.meshes);
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
```

### 2. `src/pick_gpu_tests.rs`

Supply actual copied-original pixels, high-bit original picks, selection restoration, hide/reopen and CPU/GPU release.

**Locate this existing block:**

```rust
        original
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        original
    );
}

#[test]
fn actual_copied_originals_share_instance_records_keep_original_picks_and_return_to_ordinary_drawing()
 {
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.18, -0.2, 0.4],
                    [0.18, -0.2, 0.4],
                    [0., 0.2, 0.4],
                ],
                [0.1, 0.4, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    scene.translate(a, [-0.45, 0., 0.]).unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(b, [0.9, 0., 0.]).unwrap();
    assert!(scene.instances.is_empty());
    assert!(Rc::ptr_eq(
        &scene.objects()[0].mesh,
        &scene.objects()[1].mesh
    ));
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let weak = Rc::downgrade(&scene.objects()[0].mesh);
    {
        let classify = |limit, declared| {
            crate::instance_gpu::batch_rows(
                scene.objects().iter().map(|row| {
                    (
                        row.id,
                        Rc::as_ptr(&row.mesh) as usize,
                        declared == Some(row.id),
                    )
                }),
                limit,
            )
        };
        assert!(classify(1, None).is_empty());
        assert_eq!(classify(2, None).len(), 2);
        assert_eq!(
            classify(1, Some(b)),
            [b].into_iter().collect()
        );
    }
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    let uploads = gpu.stats()[0];
    let plain = colour_pixels(&mut gpu);
    let mut blue = [0; 2];
    for line in plain.chunks_exact(320 * 4) {
        for (x, p) in line.chunks_exact(4).enumerate() {
            if p[2] > p[0].saturating_add(20) && p[2] > p[1].saturating_add(10) {
                blue[x / 160] += 1;
            }
        }
    }
    assert!(blue.into_iter().all(|n| n > 100));
    for (id, x) in [(a, -0.45), (b, 0.45)] {
        gpu.request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut gpu).unwrap()[0], id.value());
    }
    gpu.select_objects(&scene, &[b]).unwrap();
    let selected = colour_pixels(&mut gpu);
    assert!(
        selected
            .chunks_exact(4)
            .any(|p| p[0] > 140 && p[1] > 90 && p[2] < 40)
    );
    gpu.select_objects(&scene, &[]).unwrap();
    assert_eq!(colour_pixels(&mut gpu), plain);
    Rc::make_mut(&mut scene.view).hide(&[b], true);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    gpu.request_pick([-0.45, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut gpu).unwrap()[0], a.value());
    Rc::make_mut(&mut scene.view).hide(&[b], false);
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    assert_eq!(colour_pixels(&mut gpu), plain);
    assert_eq!(gpu.stats()[0], uploads);
    assert!(scene.instances.is_empty());
    for row in scene.objects() {
        assert_eq!(row.geometry().unwrap().to_proto(), original);
    }
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    assert!(weak.upgrade().is_none());
}
```

### 3. `src/instance_gpu.rs`

Share the existing record capacity and retain ordinary drawing when optional copied-original batches exceed it. Keep optional batching and the record writer on one device-limit calculation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        queue: &wgpu::Queue,
        rows: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = (u64::from(
            device
                .limits()
                .max_storage_buffer_binding_size,
        )
        .min(device.limits().max_buffer_size)
            / BYTES as u64) as usize;
        if limit == 0 {
```

Replace it with:

```rust
        queue: &wgpu::Queue,
        rows: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
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
pub(crate) fn batch_rows(
    rows: impl IntoIterator<Item = (crate::scene::ObjectId, usize, bool)>,
    limit: usize,
) -> std::collections::HashSet<crate::scene::ObjectId> {
    let rows = rows.into_iter().collect::<Vec<_>>();
    let mut uses = std::collections::HashMap::new();
    for &(_, key, _) in &rows {
        *uses.entry(key).or_insert(0usize) += 1;
    }
    let candidates = rows
        .iter()
        .filter(|(_, key, declared)| *declared || uses[key] > 1)
        .map(|(id, _, _)| *id)
        .collect::<std::collections::HashSet<_>>();
    if candidates.len() <= limit {
        candidates
    } else {
        rows.into_iter()
            .filter(|(_, _, declared)| *declared)
            .map(|(id, _, _)| id)
            .collect()
    }
}
pub(crate) fn source() -> String {
    let source = include_str!("triangle.wgsl")
        .replace(
```

### 4. `src/object_rows.rs`

Own the shared record-capacity calculation beside the record table. Use the same limit for optional batching, instance writes and object-record writes.

**Locate this existing block:**

```rust
};
pub struct Rows {
    pub(crate) buffer: GrowBuf,
    group: wgpu::BindGroup,
    previous: Vec<Record>,
    pub written_bytes: u64,
}
impl Rows {
    pub fn new(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "shared object records",
            BYTES as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let group = crate::gpu_layout::group(&ctx.device, layout, &buffer.buf);
        Self {
            buffer,
            group,
            previous: Vec::new(),
            written_bytes: 0,
        }
    }
    pub fn set(
        &mut self,
        ctx: &GpuCtx,
        layout: &wgpu::BindGroupLayout,
        rows: &[Record],
    ) -> Result<bool, &'static str> {
        let limits = ctx.device.limits();
        let limit = limits.max_buffer_size.min(u64::from(
            limits.max_storage_buffer_binding_size,
        ));
        if rows.len() as u64 > limit / BYTES as u64
            || u32::try_from(rows.len()).is_err()
        {
            return Err("Object records exceed device limits");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
};
pub(crate) fn capacity(device: &wgpu::Device) -> usize {
    (u64::from(
        device
            .limits()
            .max_storage_buffer_binding_size,
    )
    .min(device.limits().max_buffer_size)
        / BYTES as u64) as usize
}
pub struct Rows {
    pub(crate) buffer: GrowBuf,
    group: wgpu::BindGroup,
    previous: Vec<Record>,
    pub written_bytes: u64,
}
impl Rows {
    pub fn new(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "shared object records",
            BYTES as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let group = crate::gpu_layout::group(&ctx.device, layout, &buffer.buf);
        Self {
            buffer,
            group,
            previous: Vec::new(),
            written_bytes: 0,
        }
    }
    pub fn set(
        &mut self,
        ctx: &GpuCtx,
        layout: &wgpu::BindGroupLayout,
        rows: &[Record],
    ) -> Result<bool, &'static str> {
        if rows.len() > capacity(&ctx.device) || u32::try_from(rows.len()).is_err() {
            return Err("Object records exceed device limits");
```

## <a id="checkpoint-96zzzo-shrink-record-ranges-without-rewriting-live-prefixes"></a>21. Shrink record ranges without rewriting live prefixes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_gpu_support.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Shorter live range</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed live runs only</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current draw count</text>
</svg>
</div>

**What this code does:** Reuse one logical buffer truncate operation for object records and draw slots.

### 1. `src/text_gpu_support.rs`

Shorten a logical record range without replacing its allocation or rewriting the live prefix.

**Locate this existing block:**

```rust
    }

    pub fn write_bytes_at(&self, ctx: &GpuCtx, at: u32, bytes: &[u8]) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn truncate(&mut self, len: u32) {
        assert!(len <= self.len);
        self.len = len;
    }

    pub fn write_bytes_at(&self, ctx: &GpuCtx, at: u32, bytes: &[u8]) {
```

### 2. `src/object_rows.rs`

Reuse the existing changed-run scan on shrinking tables; write only changed live records.

**Locate this existing block:**

```rust
        if rows.len() < self.previous.len() {
            self.buffer.reset();
            let data = bytes(rows);
            grew = self.buffer.append_bytes(ctx, &data);
            self.written_bytes = data.len() as u64;
        } else {
            for run in crate::draw_slots::changed_runs(&self.previous, rows) {
                let data = bytes(&rows[run.clone()]);
                self.buffer
                    .write_bytes_at(ctx, run.start as u32, &data);
                self.written_bytes += data.len() as u64;
            }
            let tail = bytes(&rows[self.previous.len()..]);
            if !tail.is_empty() {
                grew = self.buffer.append_bytes(ctx, &tail);
                self.written_bytes += tail.len() as u64;
            }
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if rows.len() < self.previous.len() {
            self.buffer.truncate(rows.len() as u32);
        }
        for run in crate::draw_slots::changed_runs(&self.previous, rows) {
            let data = bytes(&rows[run.clone()]);
            self.buffer
                .write_bytes_at(ctx, run.start as u32, &data);
            self.written_bytes += data.len() as u64;
        }
        let tail = bytes(&rows[self.previous.len().min(rows.len())..]);
        if !tail.is_empty() {
            grew = self.buffer.append_bytes(ctx, &tail);
            self.written_bytes += tail.len() as u64;
        }
```

### 3. `src/draw_slots.rs`

Keep slot zero and unchanged instance slots while shortening their logical range.

**Locate this existing block:**

```rust
        } else if rows.len() < self.previous.len() {
            self.buffer.reset();
            self.buffer
                .append_bytes(ctx, &bytes(&[[OWN_ROW, 0]]));
            self.buffer.append_bytes(ctx, &bytes(rows));
            self.written_bytes = (rows.len() as u64 + 1) * 8;
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        } else if rows.len() < self.previous.len() {
            self.buffer
                .truncate(u32::try_from(rows.len() + 1).expect("Bounded draw slots"));
        }
```

### 4. `src/object_rows.rs`

Supply mapped live prefixes, exact partial-write counts and current immutable bindings while shrinking.

**Locate this existing block:**

```rust
        rows.truncate(2);
        table.set(&ctx, &layout, &rows).unwrap();
        assert_eq!(table.allocated_bytes(), capacity);
        check(&table, &rows);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        rows.truncate(2);
        let held = table.buffer.buf.clone();
        let binding = table.group.clone();
        assert!(!table.set(&ctx, &layout, &rows).unwrap());
        assert_eq!(table.written_bytes, 0);
        assert_eq!(table.buffer.len(), 2);
        assert_eq!(table.buffer.buf, held);
        assert_eq!(table.group, binding);
        assert_eq!(table.allocated_bytes(), capacity);
        check(&table, &rows);
        rows[0][68..72].copy_from_slice(&0x80000003u32.to_ne_bytes());
        rows.truncate(1);
        table.set(&ctx, &layout, &rows).unwrap();
        assert_eq!(table.written_bytes, BYTES as u64);
        assert_eq!(table.buffer.len(), 1);
        check(&table, &rows);
```

### 5. `src/draw_slots_tests.rs`

Supply exact retained slot bytes and one changed high-bit slot through a second shrink.

**Locate this existing block:**

```rust
    let capacity = slots.allocated_bytes();
    slots.set(&ctx, &rows);
    assert_eq!(slots.written_bytes, 24);
    assert_eq!(slots.allocated_bytes(), capacity);
    check(&slots, &rows);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let capacity = slots.allocated_bytes();
    let held = slots.buffer.buf.clone();
    assert!(!slots.set(&ctx, &rows));
    assert_eq!(slots.written_bytes, 0);
    assert_eq!(slots.instances(), 2);
    assert_eq!(slots.buffer.buf, held);
    assert_eq!(slots.allocated_bytes(), capacity);
    check(&slots, &rows);
    rows[0] = [0x80000003, 0x80000007];
    rows.truncate(1);
    assert!(slots.set(&ctx, &rows));
    assert_eq!(slots.written_bytes, 8);
    assert_eq!(slots.instances(), 1);
    check(&slots, &rows);
```

## <a id="checkpoint-96zzzp-share-opaque-ordinary-surface-records"></a>22. Share opaque ordinary surface records

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Ordinary visible mesh</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing storage shader</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original pixels and primi…</text>
</svg>
</div>

**What this code does:** Draw ordinary opaque surfaces from the same object-record table as instances.

### 1. `src/instance_gpu.rs`

Keep each ordinary original’s draw slot alongside the shared record table. Start with no ordinary record leases. Receive ordinary visible meshes without changing the existing instance grouping. Append ordinary records when the table has room; keep their legacy path when it does not. Release all scene slots after both ordinary and instance leases end. Adopt current record leases together with the instance batches. Draw ordinary opaque meshes in their existing scene order through the same storage-record shader. Account for every live slot allocation once, including ordinary record consumers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .map(|b| u64::from(b.count))
                .sum(),
            self.records.allocated_bytes()
                + if self.batches.is_empty() {
                    0
```

Replace it with:

```rust
                .map(|b| u64::from(b.count))
                .sum(),
            self.records.allocated_bytes()
                + if self.slots.instances() == 0 {
                    0
```

**Locate this block:**

```rust
            self.written_bytes += self.slots.written_bytes;
        }
        self.batches = batches;
        Ok(())
    }
    pub fn bind_own(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.slots.bind(pass, 0, 0);
    }
```

Replace it with:

```rust
            self.written_bytes += self.slots.written_bytes;
        }
        self.batches = batches;
        self.ordinary = ordinary;
        Ok(())
    }
    pub fn draw_ordinary(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        mesh: &GpuMesh,
    ) -> bool {
        let Some(&(first, row)) = self.ordinary.get(&mesh.id) else {
            return false;
        };
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        self.slots.bind(pass, 0, first);
        mesh.geometry.draw_pulled(pass, 1);
        true
    }
    pub fn bind_own(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.slots.bind(pass, 0, 0);
    }
```

**Locate this block:**

```rust
            self.writes += 1;
            self.written_bytes += self.records.written_bytes;
        }
        if batches.is_empty() && !self.batches.is_empty() {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
```

Replace it with:

```rust
            self.writes += 1;
            self.written_bytes += self.records.written_bytes;
        }
        if all_slots.is_empty() && self.slots.instances() > 0 {
            self.slots.clear(&ctx, true);
        } else {
            self.slots.set(&ctx, &all_slots);
```

**Locate this block:**

```rust
                },
            );
        }
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        self.records
            .set(&ctx, &self.layout, &all_records)?;
```

Replace it with:

```rust
                },
            );
        }
        let own = ordinary.into_iter().collect::<Vec<_>>();
        let mut ordinary = std::collections::HashMap::new();
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
        let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
        self.records
            .set(&ctx, &self.layout, &all_records)?;
```

**Locate this block:**

```rust
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rows: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
```

Replace it with:

```rust
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rows: impl IntoIterator<Item = &'a GpuMesh>,
        ordinary: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
```

**Locate this block:**

```rust
            layout,
            batches: BTreeMap::new(),
            records,
            slots: crate::draw_slots::Slots::new(
                &crate::text_gpu_support::GpuCtx::new(device, queue),
            ),
```

Replace it with:

```rust
            layout,
            batches: BTreeMap::new(),
            records,
            ordinary: Default::default(),
            slots: crate::draw_slots::Slots::new(
                &crate::text_gpu_support::GpuCtx::new(device, queue),
            ),
```

**Locate this block:**

```rust
    records: crate::object_rows::Rows,
    pub(crate) slots: crate::draw_slots::Slots,
    pub writes: u64,
    pub written_bytes: u64,
```

Replace it with:

```rust
    records: crate::object_rows::Rows,
    pub(crate) ordinary: std::collections::HashMap<crate::scene::ObjectId, (u32, u32)>,
    pub(crate) slots: crate::draw_slots::Slots,
    pub writes: u64,
    pub written_bytes: u64,
```

### 2. `src/renderer.rs`

Refresh both ordinary and instance records after scene or selection changes. Use storage records for opaque ordinary surfaces; retain the earlier transparent and capacity-fallback paths.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                mesh.draw_pulled(&mut pass);
            }
```

Replace it with:

```rust
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                if !(self.opacity.get() >= 1.
                    && self.instances.draw_ordinary(&mut pass, mesh))
                {
                    pass.set_pipeline(&self.pipeline);
                    self.instances.bind_own(&mut pass);
                    mesh.draw_pulled(&mut pass);
                }
            }
```

**Locate this block:**

```rust
            self.meshes
                .iter()
                .filter(|row| self.instance_rows.contains(&row.id)),
        )?;
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
```

Replace it with:

```rust
            self.meshes
                .iter()
                .filter(|row| self.instance_rows.contains(&row.id)),
            self.meshes
                .iter()
                .filter(|row| !self.instance_rows.contains(&row.id)),
        )?;
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
```

**Locate this block:**

```rust
                    .filter(|row| self.instance_rows.contains(&row.id)),
            )
            .expect("Validated instance records fit device limits");
        self.sheets
```

Replace it with:

```rust
                    .filter(|row| self.instance_rows.contains(&row.id)),
                self.meshes
                    .iter()
                    .filter(|row| !self.instance_rows.contains(&row.id)),
            )
            .expect("Validated instance records fit device limits");
        self.sheets
```

### 3. `src/pick_gpu_tests.rs`

The surviving ordinary original now retains its shared record allocation; its original pick and full final release remain checked.

**Locate this existing block:**

```rust
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    gpu.request_pick([-0.45, 0.], identity())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    gpu.set_scene(&scene, None);
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 376]);
    gpu.request_pick([-0.45, 0.], identity())
```

### 4. `src/global_outline_tests.rs`

Supply mapped ordinary records and exact storage-versus-legacy pixels and primitive addresses through growth, selection, opacity restoration and release.

**Locate this existing block:**

```rust
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(&gpu.tile_usage()[..5], &[96, 16, 48, 0, 0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release()
 {
    let mut scene = Scene::empty();
    let a = scene
        .insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap())
        .unwrap();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    assert_eq!(gpu.instances.ordinary.len(), 1);
    assert_eq!(
        crate::growing_buffer_tests::read(
            &ctx,
            gpu.instances.record_buffer(),
            crate::object_record::BYTES as u64
        ),
        gpu.meshes[0].packed()
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.instances.ordinary.len(), 1);
    let b = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.2, -0.4],
                    [0.2, -0.2, -0.4],
                    [0., 0.2, -0.4],
                ],
                [0.1, 0.4, 0.9],
            )
            .unwrap(),
        )
        .unwrap();
    scene.translate(b, [0.6, 0., 0.]).unwrap();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.instances.ordinary.len(), 2);
    let wanted = gpu
        .meshes
        .iter()
        .flat_map(|m| m.packed())
        .collect::<Vec<_>>();
    assert_eq!(
        crate::growing_buffer_tests::read(
            &ctx,
            gpu.instances.record_buffer(),
            wanted.len() as u64
        ),
        wanted
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
    gpu.select_objects(&scene, &[a]).unwrap();
    assert_ne!(pixels(&gpu), stored);
    gpu.select_objects(&scene, &[]).unwrap();
    assert_eq!(pixels(&gpu), stored);
    gpu.set_opacity(0.5);
    pixels(&gpu);
    gpu.set_opacity(1.);
    assert_eq!(pixels(&gpu), stored);
    scene.clear();
    gpu.set_scene(&scene, None);
    assert!(gpu.instances.ordinary.is_empty());
    assert_eq!(&gpu.instance_usage()[..3], &[0, 0, 0]);
    assert_eq!(gpu.instances.slots.allocated_bytes(), 8);
}
```

### 5. `src/object_rows.rs`

Reuse the record table’s existing CPU snapshot; use the current ordinary uniform path after a mesh range or record changes.

**Locate this existing block:**

```rust
    }
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub(crate) fn matches(&self, row: u32, record: &Record) -> bool {
        self.previous.get(row as usize) == Some(record)
    }
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
```

## <a id="checkpoint-96zzzq-blend-ordinary-surface-records"></a>23. Blend ordinary surface records

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/re…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Shared ordinary record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">alpha-blended color plus…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">earlier scene order</text>
</svg>
</div>

**What this code does:** Use the shared ordinary record table at the current translucent default.

### 1. `src/instance_gpu.rs`

Keep a blended ordinary pipeline beside the existing instance pipeline. Share the full pipeline descriptor instead of repeating it for blending. Use the same ordinary alpha-blend policy already written in the uniform renderer. Create both states from one shader, layout and descriptor. Own the ordinary blend state with the existing shared record consumers. Blend ordinary records while keeping their exact primitive IDs, draw count and scene order.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
```

Replace it with:

```rust
        if !self.records.matches(row, &mesh.packed()) {
            return false;
        }
        pass.set_pipeline(&self.ordinary_pipeline);
        self.records.bind(pass);
```

**Locate this block:**

```rust
                ],
                immediate_size: 0,
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shared original definition drawing"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[
                    Some(format.into()),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let records = crate::object_rows::Rows::new(
            &crate::text_gpu_support::GpuCtx::new(device, queue),
            &layout,
        );
        Self {
            pipeline,
            layout,
            batches: BTreeMap::new(),
            records,
```

Replace it with:

```rust
                ],
                immediate_size: 0,
            });
        let create = |blend| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("shared original definition drawing"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(true)),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::surface()),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = create(None);
        let ordinary_pipeline = create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records = crate::object_rows::Rows::new(
            &crate::text_gpu_support::GpuCtx::new(device, queue),
            &layout,
        );
        Self {
            pipeline,
            ordinary_pipeline,
            layout,
            batches: BTreeMap::new(),
            records,
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<usize, Batch>,
    records: crate::object_rows::Rows,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<usize, Batch>,
    records: crate::object_rows::Rows,
```

### 2. `src/renderer.rs`

Ordinary storage records now work at the current translucent default as well as Opacity 1.

**Locate this existing block:**

```rust
            }) {
                if !(self.opacity.get() >= 1.
                    && self.instances.draw_ordinary(&mut pass, mesh))
                {
                    pass.set_pipeline(&self.pipeline);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            }) {
                if !self.instances.draw_ordinary(&mut pass, mesh) {
                    pass.set_pipeline(&self.pipeline);
```

### 3. `src/global_outline_tests.rs`

Supply exact storage-versus-uniform RGBA and primitive images at x-ray, half opacity, the current default and full opacity.

**Locate this existing block:**

```rust
    assert_eq!(pixels(&gpu), stored);
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(pixels(&gpu), stored);
    for opacity in [0., 0.5, 0.95, 1.] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact ordinary blend at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
    }
    scene.clear();
```

## <a id="checkpoint-96zzzr-share-transparent-instance-records"></a>24. Share transparent instance records

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 24 of 24</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/instance_gpu.rs, src/gl…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original object ID</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing shared record an…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/engine/gpu/aren…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one blended draw in scene…</text>
</svg>
</div>

**What this code does:** Draw transparent placements from the same shared records in scene order.

### 1. `src/instance_gpu.rs`

Retain each original owner beside its packed instance record. Keep source ownership while preparing shared geometry batches. Build one lookup for individual draws alongside the existing batches. Read the original identity without encoding it as a floating-point value. Index the existing instance slot and record for a single blended draw. Append optional ordinary rows to the same lookup. Distinguish the ordinary input iterator from the individual-draw lookup.

**Locate this existing block:**

```rust
        rows: impl IntoIterator<Item = &'a GpuMesh>,
        ordinary: impl IntoIterator<Item = &'a GpuMesh>,
    ) -> Result<(), &'static str> {
        let limit = crate::object_rows::capacity(device);
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: BTreeMap<usize, (Rc<GpuGeometry>, Vec<Record>)> =
            BTreeMap::new();
        for row in rows {
            let key = Rc::as_ptr(&row.geometry) as usize;
            groups
                .entry(key)
                .or_insert_with(|| (row.geometry.clone(), Vec::new()))
                .1
                .push(row.packed());
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
        let mut batches = BTreeMap::new();
        let mut all_slots = Vec::new();
        let mut all_records = Vec::with_capacity(count);
        for (source, (geometry, records)) in groups {
            let first_slot = u32::try_from(all_slots.len() + 1)
                .map_err(|_| "Instance slot range")?;
            for record in &records {
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
        let own = ordinary.into_iter().collect::<Vec<_>>();
        let mut ordinary = std::collections::HashMap::new();
        if own.len() <= limit - count {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

### 2. `src/global_outline_tests.rs`

Supply exact transparent-copy color and primitive comparisons, independent selection, unchanged originals, uploads and full release.

**Locate this existing block:**

```rust
    }
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    let copy = scene.copy_row(b, false).unwrap();
    scene
        .translate(copy, [0.3, 0.1, 0.])
        .unwrap();
    let source = scene
        .objects()
        .iter()
        .find(|o| o.id == b)
        .unwrap()
        .geometry()
        .unwrap()
        .to_proto();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.instances.ordinary.len(), 3);
    let uploads = gpu.stats()[0];
    for opacity in [0., 0.5, 0.95] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact transparent placements at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
        gpu.select_objects(&scene, &[copy]).unwrap();
        if opacity > 0. {
            assert_ne!(pixels(&gpu), stored);
        }
        gpu.select_objects(&scene, &[]).unwrap();
        assert_eq!(pixels(&gpu), stored);
        assert_eq!(gpu.stats()[0], uploads);
    }
    assert_eq!(
        scene
            .objects()
            .iter()
            .find(|o| o.id == b)
            .unwrap()
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    gpu.set_opacity(1.);
    gpu.set_scene(&scene, None);
    pixels(&gpu);
    scene.clear();
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.4
npm --prefix ../session_tests run course -- save 20.4
```

Continue with [20.5 · Original picks, ambient metadata and cap consumers](../../consolidation/96zzzs-allocate-stable-object-record-rows.md).
