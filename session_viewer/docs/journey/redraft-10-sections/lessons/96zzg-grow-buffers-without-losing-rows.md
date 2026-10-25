# 20.3 · Shared arenas, object records and source preparation

**Steps:** 14 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Contiguous GPU Arena: [Slice 0 (v0..vn)] [Slice 1] [Slice 2]</text>
  <rect x="48" y="68" width="244" height="56" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <rect x="52" y="72" width="84" height="48" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="94" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 0</text>
  <rect x="140" y="72" width="72" height="48" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="176" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 1</text>
  <rect x="216" y="72" width="72" height="48" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 2</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Vertex Pulling: idx = indices[base + vid]; v = arena[idx]</text>
  <rect x="424" y="54" width="272" height="40" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices: array&lt;GpuVertex&gt;</text>
  <rect x="454" y="110" width="212" height="36" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Dirty Sub-Range -> write_buffer()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Single-Bind-Group Multi-Slice Scene Draw</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="828,138 864,72 900,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="904,138 940,72 976,138" fill="#bbf7d0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="980,138 1016,72 1052,138" fill="#fef08a" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-96zzg-grow-buffers-without-losing-rows"></a>1. Grow buffers without losing rows

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Offscreen Rg32Uint Texture (owner_id, sub_id)</text>
  <rect x="74" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="146" y="76" width="80" height="50" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="105" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">5×5 px Window</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">COPY_SRC Attachment</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">copy_texture_to_buffer (256B Row Pitch) -&gt; Staging</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer (MAP_READ | COPY_DST)</text>
  <rect x="446" y="90" width="228" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">bytes_per_row = 256 (WebGPU alignment)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">slice.map_async(MapMode::Read, ...)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Callback Reads Mapped [u32; 2] Winner</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_mapped_range() -&gt; (id, sub)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="94" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">staging.unmap()</text>
  <rect x="846" y="114" width="176" height="32" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Selected Owner Updated</text>
</svg>
</div>

### 1. `src/text_gpu_support.rs`

**Locate this existing block:**

```rust
    ) -> Self {
        Self {
            buf: ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size: stride,
                    usage,
                    mapped_at_creation: false,
                }),
            stride,
            len: 0,
            label,
            usage,
        }
    }
    pub fn append<const N: usize>(&mut self, ctx: &GpuCtx, vertices: &[[f32; N]]) {
        assert_eq!(self.stride, N as u64 * 4);
        if vertices.is_empty() {
            return;
        }
        let count = u32::try_from(vertices.len()).expect("Bounded text vertices");
        let size = (self.len as u64 + count as u64) * self.stride;
        if size > self.buf.size() {
            assert_eq!(
                self.len, 0,
                "Text upload starts at an empty buffer"
            );
            self.buf = ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(self.label),
                    size: size.next_power_of_two(),
                    usage: self.usage,
                    mapped_at_creation: false,
                });
        }
        let bytes: Vec<u8> = vertices
            .iter()
            .flatten()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        ctx.queue.write_buffer(
            &self.buf,
            self.len as u64 * self.stride,
            &bytes,
        );
        self.len += count;
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        assert!(stride > 0 && stride % 4 == 0);
        let usage = usage | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
        Self {
            buf: ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size: stride,
                    usage,
                    mapped_at_creation: false,
                }),
            stride,
            len: 0,
            label,
            usage,
        }
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
        let grew = u64::from(end) * self.stride > self.buf.size();
        if grew {
            let cap = self.buf.size() / self.stride;
            let cap = (cap + cap / 2).max(u64::from(end)).min(most);
            let next = ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(self.label),
                    size: cap * self.stride,
                    usage: self.usage,
                    mapped_at_creation: false,
                });
            if self.len > 0 {
                let mut encoder = ctx
                    .device
                    .create_command_encoder(&Default::default());
                encoder.copy_buffer_to_buffer(
                    &self.buf,
                    0,
                    &next,
                    0,
                    u64::from(self.len) * self.stride,
                );
                ctx.queue.submit([encoder.finish()]);
            }
            self.buf = next;
        }
        ctx.queue.write_buffer(
            &self.buf,
            u64::from(self.len) * self.stride,
            bytes,
        );
        self.len = end;
        grew
    }

    pub fn append<const N: usize>(&mut self, ctx: &GpuCtx, vertices: &[[f32; N]]) {
        assert_eq!(self.stride, N as u64 * 4);
        let bytes: Vec<u8> = vertices
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        self.append_bytes(ctx, &bytes);
    }
```

### 2. `src/growing_buffer_tests.rs`

Create the file and type:

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf};
fn read(ctx: &GpuCtx, buffer: &wgpu::Buffer, size: u64) -> Vec<u8> {
    let output = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("growth readback"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = ctx
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(buffer, 0, &output, 0, size);
    ctx.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    output
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
    let bytes = output.slice(..).get_mapped_range().to_vec();
    output.unmap();
    bytes
}
#[test]
fn actual_gpu_growth_preserves_packed_records_and_reset_reuses_capacity() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut buffer = GrowBuf::new(
        &ctx,
        "packed arena preparation",
        20,
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
    );
    let records = [vec![11; 20], vec![22; 40], vec![33; 60]];
    let mut wanted = Vec::new();
    for record in records {
        let before = buffer.len();
        let replaced = buffer.append_bytes(&ctx, &record);
        if before > 0 {
            assert!(replaced);
        }
        wanted.extend(record);
        assert_eq!(
            read(&ctx, &buffer.buf, wanted.len() as u64),
            wanted
        );
    }
    assert_eq!(buffer.len(), 6);
    let size = buffer.buf.size();
    assert!(!buffer.append_bytes(&ctx, &[]));
    assert_eq!(buffer.len(), 6);
    buffer.reset();
    assert!(!buffer.append_bytes(&ctx, &[44; 20]));
    assert_eq!(buffer.buf.size(), size);
    assert_eq!(read(&ctx, &buffer.buf, 20), [44; 20]);
    buffer.release(&ctx);
    assert!(buffer.is_empty());
    assert_eq!(buffer.buf.size(), 20);
    let mut floats = GrowBuf::new(
        &ctx,
        "existing float adapter",
        8,
        wgpu::BufferUsages::VERTEX,
    );
    floats.append(&ctx, &[[1., 2.]]);
    floats.append(&ctx, &[[3., 4.]]);
    assert_eq!(
        read(&ctx, &floats.buf, 16),
        [1f32, 2., 3., 4.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>()
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod scene_label_screen;

pub mod text_gpu_support;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_label_screen;

#[cfg(test)]
mod growing_buffer_tests;
pub mod text_gpu_support;
```

## <a id="checkpoint-96zzh-prepare-shared-compact-mesh-ranges"></a>2. Prepare shared compact mesh ranges

### 1. `src/arena_rows.rs`

Create the file and type:

```rust
use crate::mesh::Mesh;
use std::{
    collections::HashMap,
    ops::Range,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeshRange {
    pub vertices: Range<u32>,
    pub indices: Range<u32>,
}
#[derive(Default)]
pub struct Rows {
    pub vertices: Vec<u8>,
    pub indices: Vec<u32>,
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
        self.indices
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
    pub fn release(&mut self) {
        *self = Self::default();
    }
}
```

### 2. `src/arena_rows_tests.rs`

Create the file and type:

```rust
use std::rc::Rc;
fn source() -> Rc<crate::mesh::Mesh> {
    Rc::new(
        crate::mesh::Mesh::new(
            vec![
                [0., 0., 0., 0.9, 0.1, 0.1],
                [1., 0., 0., 0.9, 0.1, 0.1],
                [0., 1., 0., 0.9, 0.1, 0.1],
            ],
            vec![0, 1, 2],
        )
        .unwrap()
        .with_alphas(vec![1. / 255.; 3])
        .unwrap(),
    )
}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions() {
    let a = source();
    let b = source();
    let mut rows = crate::arena_rows::Rows::default();
    let first = rows.append(&a).unwrap();
    assert_eq!(first.vertices, 0..3);
    assert_eq!(first.indices, 0..3);
    assert_eq!(rows.append(&Rc::clone(&a)).unwrap(), first);
    assert_eq!(rows.source_count(), 1);
    let second = rows.append(&b).unwrap();
    assert_eq!(second.vertices, 3..6);
    assert_eq!(second.indices, 3..6);
    assert_eq!(rows.source_count(), 2);
    assert_eq!(rows.indices, [0, 1, 2, 3, 4, 5]);
    assert_eq!(
        rows.vertices,
        [
            crate::packed_vertex::bytes(&a),
            crate::packed_vertex::bytes(&b)
        ]
        .concat()
    );
    assert_eq!(rows.vertices.len(), 6 * 20);
    assert_eq!(rows.vertices[19], 1);
    assert_eq!(
        rows.index_bytes(),
        [0u32, 1, 2, 3, 4, 5]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(a.indices(), &[0, 1, 2]);
    assert_eq!(b.indices(), &[0, 1, 2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage() {
    let mesh = source();
    let weak = Rc::downgrade(&mesh);
    let mut rows = crate::arena_rows::Rows::default();
    rows.append(&mesh).unwrap();
    let cap = [
        rows.vertices.capacity(),
        rows.indices.capacity(),
    ];
    drop(mesh);
    assert!(weak.upgrade().is_none());
    rows.reset();
    assert_eq!(rows.source_count(), 0);
    assert_eq!(
        [
            rows.vertices.capacity(),
            rows.indices.capacity()
        ],
        cap
    );
    let next = source();
    assert_eq!(rows.append(&next).unwrap().vertices, 0..3);
    rows.release();
    assert_eq!(
        [
            rows.vertices.capacity(),
            rows.indices.capacity()
        ],
        [0, 0]
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod navigation;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod navigation;

pub mod arena_rows;
#[cfg(test)]
mod arena_rows_tests;
#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-96zzi-upload-compact-ranges-to-shared-gpu-storage"></a>3. Upload compact ranges to shared GPU storage

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Camera / View Uniform (16-Byte Aligned)</text>
  <rect x="52" y="48" width="268" height="106" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="66" y="58" width="240" height="38" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="81" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">view_proj: [[f32; 4]; 4] (64B)</text>
  <rect x="66" y="104" width="240" height="36" fill="#fef08a" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye_pos / viewport_px (16B)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_buffer -&gt; wgpu::BindGroup (@group(0))</text>
  <rect x="424" y="50" width="272" height="102" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="442" y="64" width="236" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">BufferUsages::UNIFORM | COPY_DST</text>
  <rect x="442" y="106" width="236" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">pass.set_bind_group(0, &amp;bg, &amp;[])</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Shader Uniform Read: var&lt;uniform&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@group(0) @binding(0)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;uniform&gt; u: Uniforms;</text>
  <rect x="828" y="114" width="212" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">u.view_proj * vec4(p, 1.0)</text>
</svg>
</div>

### 1. `src/arena_rows.rs`

**Locate this existing block:**

```rust
    pub fn index_bytes(&self) -> Vec<u8> {
        self.indices
            .iter()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn index_bytes(&self) -> Vec<u8> {
        self.index_bytes_from(0)
    }
    pub fn index_bytes_from(&self, start: usize) -> Vec<u8> {
        self.indices[start..]
            .iter()
```

### 2. `src/mesh_arena.rs`

Create the file and type:

```rust
use crate::{
    arena_rows::{MeshRange, Rows},
    mesh::Mesh,
    text_gpu_support::{GpuCtx, GrowBuf},
};
use std::rc::Rc;

pub struct Arena {
    pub rows: Rows,
    pub vertices: GrowBuf,
    pub indices: GrowBuf,
    pub layout: wgpu::BindGroupLayout,
    pub group: wgpu::BindGroup,
    pub revision: u64,
}
impl Arena {
    pub fn new(ctx: &GpuCtx) -> Self {
        let vertices = GrowBuf::new(
            ctx,
            "arena compact vertices",
            20,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
        );
        let indices = GrowBuf::new(
            ctx,
            "arena indices",
            4,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::INDEX,
        );
        let layout = crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group = Self::bind(ctx, &layout, &vertices, &indices);
        Self {
            rows: Rows::default(),
            vertices,
            indices,
            layout,
            group,
            revision: 0,
        }
    }
    fn bind(
        ctx: &GpuCtx,
        layout: &wgpu::BindGroupLayout,
        vertices: &GrowBuf,
        indices: &GrowBuf,
    ) -> wgpu::BindGroup {
        ctx.device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("shared arena reads"),
                layout,
                entries: &[(0, &vertices.buf), (2, &indices.buf)].map(
                    |(binding, buffer)| wgpu::BindGroupEntry {
                        binding,
                        resource: buffer.as_entire_binding(),
                    },
                ),
            })
    }
    pub fn upload(
        &mut self,
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
            self.group = Self::bind(
                ctx,
                &self.layout,
                &self.vertices,
                &self.indices,
            );
            self.revision += 1;
        }
        Ok(range)
    }
    pub fn reset(&mut self) {
        self.rows.reset();
        self.vertices.reset();
        self.indices.reset();
    }
    pub fn release(&mut self, ctx: &GpuCtx) {
        *self = Self::new(ctx);
    }
    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.buf.size() + self.indices.buf.size()
    }
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod imported_source_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod imported_source_tests;
pub mod mesh_arena;
#[cfg(test)]
```

### 4. `src/growing_buffer_tests.rs`

**Locate this existing block:**

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf};
fn read(ctx: &GpuCtx, buffer: &wgpu::Buffer, size: u64) -> Vec<u8> {
    let output = ctx
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf};
pub(crate) fn read(ctx: &GpuCtx, buffer: &wgpu::Buffer, size: u64) -> Vec<u8> {
    let output = ctx
```

### 5. `src/mesh_arena_tests.rs`

Create the file and type:

```rust
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center: f32, color: [f32; 3]) -> Rc<crate::mesh::Mesh> {
    Rc::new(
        crate::mesh::Mesh::new(
            vec![
                [
                    center - 0.4,
                    -0.6,
                    0.,
                    color[0],
                    color[1],
                    color[2],
                ],
                [
                    center + 0.4,
                    -0.6,
                    0.,
                    color[0],
                    color[1],
                    color[2],
                ],
                [center, 0.6, 0., color[0], color[1], color[2]],
            ],
            vec![0, 1, 2],
        )
        .unwrap(),
    )
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut arena = crate::mesh_arena::Arena::new(&ctx);
    let a = mesh(-0.45, [1., 0., 0.]);
    let b = mesh(0.45, [0., 1., 0.]);
    let first = arena.upload(&ctx, &a).unwrap();
    let revision = arena.revision;
    assert_eq!(arena.upload(&ctx, &a).unwrap(), first);
    assert_eq!(arena.revision, revision);
    let second = arena.upload(&ctx, &b).unwrap();
    assert_eq!(second.vertices, 3..6);
    assert_eq!(second.indices, 3..6);
    assert!(arena.revision > revision);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &arena.vertices.buf, 120),
        [
            crate::packed_vertex::bytes(&a),
            crate::packed_vertex::bytes(&b)
        ]
        .concat()
    );
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &arena.indices.buf, 24),
        arena.rows.index_bytes()
    );
    let allocated = arena.allocated_bytes();
    let revision = arena.revision;
    arena.reset();
    assert_eq!(arena.rows.source_count(), 0);
    assert!(arena.vertices.is_empty());
    assert!(arena.indices.is_empty());
    assert_eq!(
        arena.upload(&ctx, &b).unwrap().vertices,
        0..3
    );
    assert_eq!(arena.allocated_bytes(), allocated);
    assert_eq!(arena.revision, revision);
    arena.release(&ctx);
    assert_eq!(arena.allocated_bytes(), 24);
    assert!(arena.vertices.is_empty());
    assert!(arena.indices.is_empty());
}
fn render(
    ctx: &GpuCtx,
    arena: &crate::mesh_arena::Arena,
    group: &wgpu::BindGroup,
) -> Vec<u8> {
    let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
    let layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&arena.layout)],
            immediate_size: 0,
        });
    let pipeline = ctx
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
    let texture = ctx
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual arena colors"),
            size: wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    let view = texture.create_view(&Default::default());
    let mut encoder = ctx
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..arena.indices.len(), 0..1);
    }
    let output = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64 * 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &output,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(256),
                rows_per_image: Some(64),
            },
        },
        texture.size(),
    );
    ctx.queue.submit([encoder.finish()]);
    crate::growing_buffer_tests::read(ctx, &output, 64 * 256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = GpuCtx::new(&gpu.device, &gpu.queue);
    let mut arena = crate::mesh_arena::Arena::new(&ctx);
    let a = mesh(-0.45, [1., 0., 0.]);
    let b = mesh(0.45, [0., 1., 0.]);
    arena.upload(&ctx, &a).unwrap();
    let old = arena.group.clone();
    arena.upload(&ctx, &b).unwrap();
    let fresh = render(&ctx, &arena, &arena.group);
    let left = (32 * 64 + 16) * 4;
    let right = (32 * 64 + 48) * 4;
    assert_eq!(&fresh[left..left + 4], &[255, 0, 0, 255]);
    assert_eq!(&fresh[right..right + 4], &[0, 255, 0, 255]);
    let stale = render(&ctx, &arena, &old);
    assert_eq!(&stale[left..left + 4], &[255, 0, 0, 255]);
    assert_eq!(
        &stale[right..right + 4],
        &[255; 4],
        "An old binding cannot read the appended right-hand range"
    );
}
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod move_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod mesh_arena_tests;
#[cfg(test)]
mod move_tests;
```

## <a id="checkpoint-96zzj-share-the-object-record-layout"></a>4. Share the object record layout

### 1. `src/object_record.rs`

Create the file and type:

```rust
pub const BYTES: usize = 64 + 16 + 64 + 16;
pub type Record = [u8; BYTES];
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod navigation_tests;
pub mod packed_vertex;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod navigation_tests;
pub mod object_record;
pub mod packed_vertex;
```

### 3. `src/gpu_mesh.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.primitive_base,
        );
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144, 160)] {
            if next[start..end] != self.previous[start..end] {
```

Replace it with:

```rust
            self.primitive_base,
        );
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144, BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

**Locate this block:**

```rust
    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
    pub(crate) fn packed(&self) -> [u8; 160] {
        self.previous
    }
    pub fn selected(&self) -> bool {
```

Replace it with:

```rust
    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
    pub(crate) fn packed(&self) -> Record {
        self.previous
    }
    pub fn selected(&self) -> bool {
```

**Locate this block:**

```rust
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

Replace it with:

```rust
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

**Locate this block:**

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: [u8; 160],
    primitive_base: u32,
    xray: bool,
    cap_flags: u32,
```

Replace it with:

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,
    primitive_base: u32,
    xray: bool,
    cap_flags: u32,
```

**Locate this block:**

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;
```

Replace it with:

```rust
use crate::object_record::{BYTES, Record};
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;
```

### 4. `src/instance_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .max_storage_buffer_binding_size,
        )
        .min(device.limits().max_buffer_size)
            / 160) as usize;
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: HashMap<usize, (Rc<GpuGeometry>, Vec<[u8; 160]>)> =
            HashMap::new();
        for row in rows {
```

Replace it with:

```rust
                .max_storage_buffer_binding_size,
        )
        .min(device.limits().max_buffer_size)
            / BYTES as u64) as usize;
        if limit == 0 {
            return Err("Device cannot hold one instance record");
        }
        let mut groups: HashMap<usize, (Rc<GpuGeometry>, Vec<Record>)> = HashMap::new();
        for row in rows {
```

**Locate this block:**

```rust
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(160),
                    },
                    count: None,
                }],
```

Replace it with:

```rust
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: std::num::NonZeroU64::new(BYTES as u64),
                    },
                    count: None,
                }],
```

**Locate this block:**

```rust
use crate::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh};
use std::{
    collections::{BTreeMap, HashMap},
```

Replace it with:

```rust
use crate::object_record::{BYTES, Record};
use crate::{gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh};
use std::{
    collections::{BTreeMap, HashMap},
```

### 5. `src/renderer.rs`

**Locate this existing block:**

```rust
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

### 6. `src/face_pick.rs`

**Locate this existing block:**

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let crop = crate::gpu_layout::uniform(device, 80);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let crop = crate::gpu_layout::uniform(device, 80);
```

### 7. `src/cap_pipeline.rs`

**Locate this existing block:**

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let clip = crate::clipping_gpu::layout(device);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let clip = crate::clipping_gpu::layout(device);
```

### 8. `src/cap_pick_pipeline.rs`

**Locate this existing block:**

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let records =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let records =
```

### 9. `src/ambient_source_pipeline.rs`

**Locate this existing block:**

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let clip = crate::clipping_gpu::layout(device);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let clip = crate::clipping_gpu::layout(device);
```

### 10. `src/outline_gpu.rs`

**Locate this existing block:**

```rust
    let view = crate::gpu_layout::uniform(device, 80);
    let model = crate::gpu_layout::uniform(device, 160);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let view = crate::gpu_layout::uniform(device, 80);
    let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

### 11. `src/renderer.rs`

**Locate this existing block:**

```rust
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 160
            + self.settings_bytes as u64
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            + self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * crate::object_record::BYTES as u64
            + self.settings_bytes as u64
```

### 12. `src/object_record_tests.rs`

Create the file and type:

```rust
#[test]
fn assembled_object_uniform_and_instance_array_match_host_layout() {
    use wgpu::naga;
    let uniform = crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0);
    let instances = uniform
        .replace(
            "var<uniform> object_settings: ObjectSettings;",
            "var<storage, read> objects: array<ObjectSettings>;",
        )
        .replace(
            "return object_settings;",
            "return objects[index];",
        );
    for (source, offsets) in [
        (uniform, vec![0, 64, 80, 144]),
        (instances, vec![0, 64, 80, 144]),
        (
            crate::finite_project_source::source(),
            vec![0, 64, 68, 72, 80, 144],
        ),
    ] {
        let module = naga::front::wgsl::parse_str(&source).unwrap();
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();
        let (_, ty) = module
            .types
            .iter()
            .find(|(_, ty)| ty.name.as_deref() == Some("ObjectSettings"))
            .unwrap();
        let naga::TypeInner::Struct { members, span } = &ty.inner else {
            panic!("ObjectSettings must be a struct")
        };
        assert_eq!(*span as usize, crate::object_record::BYTES);
        assert_eq!(
            members
                .iter()
                .map(|member| member.offset)
                .collect::<Vec<_>>(),
            offsets
        );
    }
}
```

### 13. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod object_record;
pub mod packed_vertex;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod object_record;
#[cfg(test)]
mod object_record_tests;
pub mod packed_vertex;
```

## <a id="checkpoint-96zzk-borrow-the-current-arena-buffers"></a>5. Borrow the current arena buffers

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU RAM: #[repr(C)] Structs -&gt; bytemuck::cast_slice</text>
  <rect x="46" y="54" width="280" height="44" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="52" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="84" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0: 24B</text>
  <rect x="122" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="154" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1: 24B</text>
  <rect x="192" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2: 24B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="290" y="80" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">...</text>
  <rect x="66" y="112" width="240" height="34" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="133" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">&amp;[u8] Contiguous Byte Slice</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU DMA: queue.write_buffer(buf, 0, bytes)</text>
  <rect x="424" y="54" width="272" height="92" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="84" x2="672" y2="84" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">create_buffer_init / write_buffer</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="112" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">VERTEX | INDEX | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">array_stride = size_of::&lt;Vertex&gt;()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU VRAM Buffer -&gt; @vertex Input Locations</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="56" width="224" height="40" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
  <rect x="822" y="106" width="224" height="40" rx="4" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(1) color: vec3&lt;f32&gt;</text>
</svg>
</div>

### 1. `src/gpu_geometry.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(
            self.indices.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        pass.draw_indexed(0..self.index_count, 0, 0..count);
    }
```

Replace it with:

```rust
    }

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        self.draw_range(pass, 0..count);
    }
    pub(crate) fn draw_range(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        instances: std::ops::Range<u32>,
    ) {
        if self.index_count == 0 || instances.is_empty() {
            return;
        }
        let (vertices, indices) = self.buffers();
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(
            self.index_start..self.index_start + self.index_count,
            0,
            instances,
        );
    }
```

**Locate this block:**

```rust
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            indices,
            index_count,
            pull_group,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.indices.size()
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }
```

Replace it with:

```rust
        let index_count = source.indices().len() as u32;
        Self {
            source,
            index_count,
            index_start: 0,
            storage: Storage::Owned {
                vertices,
                indices,
                group: pull_group,
            },
        }
    }

    pub fn shared(
        ctx: &crate::text_gpu_support::GpuCtx,
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
        match &self.storage {
            Storage::Owned {
                vertices,
                indices,
                ..
            } => (vertices.clone(), indices.clone()),
            Storage::Shared(arena) => {
                let arena = arena.borrow();
                (
                    arena.vertices.buf.clone(),
                    arena.indices.buf.clone(),
                )
            }
        }
    }
    fn group(&self) -> wgpu::BindGroup {
        match &self.storage {
            Storage::Owned { group, .. } => group.clone(),
            Storage::Shared(arena) => arena.borrow().group.clone(),
        }
    }
    pub fn allocated_bytes(&self) -> u64 {
        match &self.storage {
            Storage::Owned {
                vertices,
                indices,
                ..
            } => vertices.size() + indices.size(),
            Storage::Shared(_) => 0,
        }
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_bind_group(3, &self.group(), &[]);
        pass.draw(0..self.index_count, 0..count);
    }
```

**Locate this block:**

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
```

Replace it with:

```rust
use crate::mesh::Mesh;
use std::{cell::RefCell, rc::Rc};
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) index_count: u32,
    pub(crate) index_start: u32,
    storage: Storage,
}

enum Storage {
    Owned {
        vertices: wgpu::Buffer,
        indices: wgpu::Buffer,
        group: wgpu::BindGroup,
    },
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
```

### 2. `src/face_pick.rs`

**Locate this existing block:**

```rust
                });
            let buffers = [
                &geometry.vertices,
                &geometry.indices,
                &faces,
                &settings,
                clipping,
            ];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                });
            let (vertices, indices) = geometry.buffers();
            let buffers = [
                &vertices, &indices, &faces, &settings, clipping,
            ];
```

### 3. `src/tile_gpu.rs`

**Locate this existing block:**

```rust
            .map(|row| {
                (
                    row.geometry.vertices.clone(),
                    row.geometry.indices.clone(),
                    row.model_buffer().clone(),
                    row.packed()[..64].try_into().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        if witness == self.witness {
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
            let count = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("finite original triangle interval"),
                contents: &bytes,
                usage: wgpu::BufferUsages::UNIFORM,
            });
            self.rows.push(Row {
                vertices: row.geometry.vertices.clone(),
                indices: row.geometry.indices.clone(),
                model: row.model_buffer().clone(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .map(|row| {
                let (vertices, indices) = row.geometry.buffers();
                (
                    vertices,
                    indices,
                    row.model_buffer().clone(),
                    row.packed()[..64].try_into().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        if witness == self.witness {
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
            let count = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("finite original triangle interval"),
                contents: &bytes,
                usage: wgpu::BufferUsages::UNIFORM,
            });
            self.rows.push(Row {
                vertices: row.geometry.buffers().0,
                indices: row.geometry.buffers().1,
                model: row.model_buffer().clone(),
```

### 4. `src/caps_gpu.rs`

**Locate this existing block:**

```rust
                    mesh.bind_model(&mut pass);
                    pass.set_vertex_buffer(0, mesh.geometry.vertices.slice(..));
                    pass.set_index_buffer(
                        mesh.geometry.indices.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    pass.draw_indexed(
                        0..mesh.geometry.index_count,
                        0,
                        plane..plane + 1,
                    );
                    self.draws += 1;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    mesh.bind_model(&mut pass);
                    mesh.geometry
                        .draw_range(&mut pass, plane..plane + 1);
                    self.draws += 1;
```

### 5. `src/cap_pick_gpu.rs`

**Locate this existing block:**

```rust
                    mesh.bind_model(&mut pass);
                    pass.set_vertex_buffer(0, mesh.geometry.vertices.slice(..));
                    pass.set_index_buffer(
                        mesh.geometry.indices.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    pass.draw_indexed(
                        0..mesh.geometry.index_count,
                        0,
                        plane..plane + 1,
                    );
                }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    mesh.bind_model(&mut pass);
                    mesh.geometry
                        .draw_range(&mut pass, plane..plane + 1);
                }
```

### 6. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(
        gpu.meshes[0].geometry.vertices.size(),
        source.vertices().len() as u64 * 20
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(
        gpu.meshes[0].geometry.buffers().0.size(),
        source.vertices().len() as u64 * 20
```

### 7. `src/geometry_storage_tests.rs`

Create the file and type:

```rust
#[test]
fn earlier_geometry_reads_current_shared_buffers_and_does_not_pin_originals() {
    use std::{cell::RefCell, rc::Rc};
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let arena = Rc::new(RefCell::new(crate::mesh_arena::Arena::new(
        &ctx,
    )));
    let a = Rc::new(
        crate::mesh::Mesh::from_kernel(&crate::cap_gpu_tests::source(false)).unwrap(),
    );
    let weak = Rc::downgrade(&a);
    let first =
        crate::gpu_geometry::GpuGeometry::shared(&ctx, a, arena.clone()).unwrap();
    let previous = first.buffers();
    let b = Rc::new(
        crate::mesh::Mesh::from_kernel(&crate::cap_gpu_tests::source(false)).unwrap(),
    );
    let second =
        crate::gpu_geometry::GpuGeometry::shared(&ctx, b, arena.clone()).unwrap();
    let current = first.buffers();
    assert_ne!(previous, current);
    assert_eq!(current, second.buffers());
    assert_eq!(first.index_start, 0);
    assert_eq!(second.index_start, first.index_count);
    let expected = [
        crate::packed_vertex::bytes(&first.source),
        crate::packed_vertex::bytes(&second.source),
    ]
    .concat();
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &current.0, expected.len() as u64),
        expected
    );
    assert_eq!(
        first.allocated_bytes() + second.allocated_bytes(),
        0
    );
    drop(first);
    assert!(weak.upgrade().is_none());
    assert_eq!(current, second.buffers());
}
```

### 8. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod gesture_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod geometry_storage_tests;
#[cfg(test)]
mod gesture_tests;
```

## <a id="checkpoint-96zzl-read-shared-ranges-through-object-records"></a>6. Read shared ranges through object records

### 1. `src/object_record.rs`

Replace the entire file with:

```rust
pub const RANGE: usize = 64 + 16 + 64 + 16;
pub const BYTES: usize = RANGE + 16;
pub type Record = [u8; BYTES];
```

### 2. `src/gpu_mesh.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                },
            self.cap_token,
            self.primitive_base,
        );
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144, BYTES)] {
            if next[start..end] != self.previous[start..end] {
```

Replace it with:

```rust
                },
            self.cap_token,
            self.primitive_base,
            self.geometry.index_start,
        );
        let mut writes = [0, 0];
        for (start, end) in [
            (0, 64),
            (64, 80),
            (80, 144),
            (144, RANGE),
            (RANGE, BYTES),
        ] {
            if next[start..end] != self.previous[start..end] {
```

**Locate this block:**

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style, 0, 0, 0);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

Replace it with:

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style, 0, 0, 0, geometry.index_start);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

**Locate this block:**

```rust
                .copy_from_slice(&value.to_ne_bytes());
        }
    }
    bytes
}
```

Replace it with:

```rust
                .copy_from_slice(&value.to_ne_bytes());
        }
    }
    bytes[RANGE..RANGE + 4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}
```

**Locate this block:**

```rust
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
```

Replace it with:

```rust
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
    index_start: u32,
) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
```

**Locate this block:**

```rust
use crate::object_record::{BYTES, Record};
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;
```

Replace it with:

```rust
use crate::object_record::{BYTES, RANGE, Record};
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;
```

### 3. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
    face_colour:vec4<f32>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
```

### 4. `src/cap_count.wgsl`

**Locate this existing block:**

```wgsl
    face_colour:vec4<f32>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
```

### 5. `src/outline_mask.wgsl`

**Locate this existing block:**

```wgsl
struct View {transform:mat4x4<f32>,display:vec4<f32>}
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View {transform:mat4x4<f32>,display:vec4<f32>}
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

### 6. `src/ambient_sources.wgsl`

**Locate this existing block:**

```wgsl
struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

### 7. `src/finite_project.wgsl`

**Locate this existing block:**

```wgsl
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

### 8. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let input = mesh_vertex(occurrence);
    let object=object_at(instance);
    var output=geometry(input.position,input.colour,input.normal,object);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let object=object_at(instance);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
```

### 9. `src/triangle_pick.wgsl`

**Locate this existing block:**

```wgsl
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput{
    let input=mesh_vertex(occurrence);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput{
    let input=mesh_vertex(occurrence,object_settings.range.x);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
```

### 10. `src/face_pick.wgsl`

**Locate this existing block:**

```wgsl
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
```

### 11. `src/finite_project.wgsl`

**Locate this existing block:**

```wgsl
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
```

### 12. `src/object_record_tests.rs`

**Locate this existing block:**

```rust
    for (source, offsets) in [
        (uniform, vec![0, 64, 80, 144]),
        (instances, vec![0, 64, 80, 144]),
        (
            crate::finite_project_source::source(),
            vec![0, 64, 68, 72, 80, 144],
        ),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    for (source, offsets) in [
        (uniform, vec![0, 64, 80, 144, 160]),
        (instances, vec![0, 64, 80, 144, 160]),
        (
            crate::finite_project_source::source(),
            vec![0, 64, 68, 72, 80, 144, 160],
        ),
```

### 13. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(gpu.usage(), [0; 4]);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use crate::{
        gpu_geometry::GpuGeometry, gpu_mesh::GpuMesh, text_gpu_support::GpuCtx,
    };
    use std::cell::RefCell;
    for clipped in [false, true] {
        let scene = if clipped {
            crate::cap_gpu_tests::scene(false).0
        } else {
            let mut scene = Scene::empty();
            scene
                .insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap())
                .unwrap();
            scene
        };
        let mut r = renderer(&scene);
        r.set_outlines(true);
        let expected = pixels(&r);
        let mut hits = Vec::new();
        for mode in [
            crate::pick_sources::Mode::Object,
            crate::pick_sources::Mode::Face,
        ] {
            r.set_pick_mode(mode);
            r.request_pick([0., 0.], matrix()).unwrap();
            wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {
            assert_ne!(
                hits[1].unwrap()[1],
                0,
                "The uncut source returns an original face address"
            );
        }
        let ctx = GpuCtx::new(&r.device, &r.queue);
        let arena = Rc::new(RefCell::new(crate::mesh_arena::Arena::new(
            &ctx,
        )));
        let decoy = Rc::new(
            crate::mesh::Mesh::new(
                vec![
                    [0.7, 0.7, -0.6, 0., 1., 0.],
                    [0.9, 0.7, -0.6, 0., 1., 0.],
                    [0.8, 0.9, -0.6, 0., 1., 0.],
                ],
                vec![0, 1, 2],
            )
            .unwrap(),
        );
        arena
            .borrow_mut()
            .upload(&ctx, &decoy)
            .unwrap();
        let layout = r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object = scene
                .objects()
                .iter()
                .find(|object| object.id == mesh.id)
                .unwrap();
            let geometry = Rc::new(
                GpuGeometry::shared(&ctx, object.mesh.clone(), arena.clone()).unwrap(),
            );
            assert!(geometry.index_start >= 3);
            *mesh = GpuMesh::with_geometry(
                &r.device,
                &layout,
                object,
                super::style(&scene, object.id, false),
                geometry,
            );
        }
        r.caps.borrow_mut().set_scene(
            &r.queue,
            &scene,
            &mut r.meshes,
            &r.clipping.borrow().rows,
        );
        let mut base = 0;
        for mesh in &mut r.meshes {
            mesh.set_primitive_base(&r.queue, base)
                .unwrap();
            base += mesh.geometry.index_count / 3;
        }
        r.tiles
            .borrow_mut()
            .set(&r.device, &r.meshes);
        r.faces.clear();
        if let Some(outline) = r.global_outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        assert_eq!(
            pixels(&r),
            expected,
            "Nonzero shared ranges preserve actual color, clipping and outline pixels"
        );
        for (mode, expected) in [
            crate::pick_sources::Mode::Object,
            crate::pick_sources::Mode::Face,
        ]
        .into_iter()
        .zip(hits)
        {
            r.set_pick_mode(mode);
            r.request_pick([0., 0.], matrix()).unwrap();
            wait(&r);
            assert_eq!(
                r.take_pick(matrix()).unwrap().unwrap(),
                expected,
                "Shared source addressing preserves original picks"
            );
        }
        assert!(r.tiles.borrow().usage()[3] > 0);
        if clipped {
            assert_eq!(r.cap_usage()[1], 1);
        }
    }
}
```

### 14. `src/pick_gpu_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 320]);
    for row in scene.objects() {
```

Replace it with:

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 352]);
    for row in scene.objects() {
```

**Locate this block:**

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(&renderer.instance_usage()[..3], &[1, 2, 320]);
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

Replace it with:

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(&renderer.instance_usage()[..3], &[1, 2, 352]);
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

## <a id="checkpoint-96zzm-connect-the-renderer-to-one-mesh-arena"></a>7. Connect the renderer to one mesh arena

### 1. `src/geometry_cache.rs`

**Locate this existing block:**

```rust
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self
            .entries
            .get(&key)
            .and_then(Weak::upgrade)
        {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(source),
        ));
        self.entries
            .insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        self.uploaded_bytes +=
            (source.vertices().len() * 20 + source.indices().len() * 4) as u64;
        geometry
    }

    pub fn prune(&mut self) {
        self.entries
            .retain(|_, geometry| geometry.strong_count() > 0);
    }

    pub fn entries(&self) -> usize {
        self.entries.len()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena: Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(
        &mut self,
        ctx: &crate::text_gpu_support::GpuCtx,
        source: &Rc<Mesh>,
    ) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self
            .entries
            .get(&key)
            .and_then(Weak::upgrade)
        {
            return geometry;
        }
        let arena = self
            .arena
            .get_or_insert_with(|| {
                Rc::new(RefCell::new(crate::mesh_arena::Arena::new(
                    ctx,
                )))
            })
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
            self.arena = None;
        }
    }

    pub fn entries(&self) -> usize {
        self.entries.len()
    }
    pub fn allocated_bytes(&self) -> u64 {
        self.arena
            .as_ref()
            .map_or(0, |arena| arena.borrow().allocated_bytes())
    }
    pub fn cpu_bytes(&self) -> usize {
        self.arena.as_ref().map_or(0, |arena| {
            let arena = arena.borrow();
            arena.rows.vertices.capacity() + arena.rows.indices.capacity() * 4
        })
    }
```

### 2. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn usage(&self) -> [u64; 4] {
        crate::memory::gpu(&self.meshes)
    }
```

Replace it with:

```rust
    }

    pub fn usage(&self) -> [u64; 4] {
        let mut usage = crate::memory::gpu(&self.meshes);
        usage[1] += self.geometry.allocated_bytes();
        usage
    }
    pub fn arena_cpu_bytes(&self) -> usize {
        self.geometry.cpu_bytes()
    }
```

**Locate this block:**

```rust
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
```

Replace it with:

```rust
            } else {
                let geometry = self.geometry.get(
                    &crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue),
                    &object.mesh,
                );
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    )?;
    let cpu = crate::memory::cpu(
        editor.presentation_scenes(),
        renderer
            .geometry_owners()
            .map(|owner| &owner.source),
    );
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    )?;
    let mut cpu = crate::memory::cpu(
        editor.presentation_scenes(),
        renderer
            .geometry_owners()
            .map(|owner| &owner.source),
    );
    cpu[4] += renderer.arena_cpu_bytes();
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
```

### 4. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
    let original = source.vertices().to_vec();
    assert_eq!(
        gpu.meshes[0].geometry.buffers().0.size(),
        source.vertices().len() as u64 * 20
    );
    assert_eq!(
        gpu.meshes[0].geometry.allocated_bytes(),
        source.vertices().len() as u64 * 20 + source.indices().len() as u64 * 4
    );
    let mut cache = crate::geometry_cache::GeometryCache::default();
    let uploaded = cache.get(&gpu.device, &source);
    assert_eq!(
        cache.uploaded_bytes,
        uploaded.allocated_bytes(),
        "Nonempty compact upload statistics match actual GPU buffers"
    );
    let written = cache.uploaded_bytes;
    cache.get(&gpu.device, &source);
    assert_eq!(cache.uploads, 1);
    assert_eq!(cache.uploaded_bytes, written);
    let empty = crate::gpu_geometry::GpuGeometry::upload(
        &gpu.device,
        std::rc::Rc::new(crate::mesh::Mesh::new(vec![], vec![]).unwrap()),
    );
    assert_eq!(empty.allocated_bytes(), 24);
    pixels(&gpu);
    assert_eq!(source.vertices(), original);
    let uploads = gpu.stats();
    pixels(&gpu);
    assert_eq!(gpu.stats(), uploads);
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let original = source.vertices().to_vec();
    let buffers = gpu.meshes[0].geometry.buffers();
    assert_eq!(gpu.meshes[0].geometry.allocated_bytes(), 0);
    assert_eq!(
        gpu.usage()[1],
        buffers.0.size() + buffers.1.size(),
        "One arena owner accounts for the actual shared buffers"
    );
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let mut cache = crate::geometry_cache::GeometryCache::default();
    let uploaded = cache.get(&ctx, &source);
    assert_eq!(
        uploaded.buffers().0.size(),
        source.vertices().len() as u64 * 20
    );
    assert_eq!(
        cache.uploaded_bytes,
        cache.allocated_bytes(),
        "The first nonempty compact upload matches actual GPU allocation"
    );
    let written = cache.uploaded_bytes;
    cache.get(&ctx, &source);
    assert_eq!(cache.uploads, 1);
    assert_eq!(cache.uploaded_bytes, written);
    let empty = crate::gpu_geometry::GpuGeometry::upload(
        &gpu.device,
        std::rc::Rc::new(crate::mesh::Mesh::new(vec![], vec![]).unwrap()),
    );
    assert_eq!(empty.allocated_bytes(), 24);
    pixels(&gpu);
    assert_eq!(source.vertices(), original);
    let uploads = gpu.stats();
    pixels(&gpu);
    assert_eq!(gpu.stats(), uploads);
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
    assert_eq!(gpu.arena_cpu_bytes(), 0);
}
```

## <a id="checkpoint-96zzn-describe-eight-byte-draw-slots"></a>8. Describe eight-byte draw slots

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Change Flags: { sync_scene: bool, redraw: bool }</text>
  <rect x="52" y="50" width="268" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">Orbit: sync_scene=false (64B only)</text>
  <rect x="52" y="104" width="268" height="44" rx="4" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Edit: dirty_slots [k..k+1] only</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Sub-Range Upload: write_buffer(arena, byte_off, slice)</text>
  <rect x="424" y="64" width="272" height="56" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="430" y="70" width="82" height="44" fill="#e2e8f0" stroke="#94a3b8" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="471" y="96" text-anchor="middle" font-family="monospace" font-size="9" fill="#64748b">Clean</text>
  <rect x="518" y="70" width="84" height="44" fill="#bbf7d0" stroke="#16a34a" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Dirty Write</text>
  <rect x="608" y="70" width="82" height="44" fill="#e2e8f0" stroke="#94a3b8" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="96" text-anchor="middle" font-family="monospace" font-size="9" fill="#64748b">Clean</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Zero Full-Scene Reallocation</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Retained GPU Storage Buffers Drawn Directly</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="836,136 872,72 908,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="916,136 952,64 988,136" fill="#bbf7d0" stroke="#16a34a" stroke-width="2.2"/>
  <polygon points="992,136 1024,78 1052,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

### 1. `src/draw_slots.rs`

Create the file and type:

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf};
pub const OWN_ROW: u32 = u32::MAX;
pub type Slot = [u32; 2];
const ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Uint32,
    },
    wgpu::VertexAttribute {
        offset: 4,
        shader_location: 1,
        format: wgpu::VertexFormat::Uint32,
    },
];
pub fn layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &ATTRIBUTES,
    }
}
pub struct Slots {
    pub(crate) buffer: GrowBuf,
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
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) {
        let bytes: Vec<u8> = std::iter::once([OWN_ROW, 0])
            .chain(rows.iter().copied())
            .flatten()
            .flat_map(u32::to_ne_bytes)
            .collect();
        self.buffer.reset();
        self.buffer.append_bytes(ctx, &bytes);
    }
    pub fn clear(&mut self, ctx: &GpuCtx, free: bool) {
        if free {
            self.buffer.release(ctx);
        }
        self.set(ctx, &[]);
    }
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>, slot: u32, first: u32) {
        assert!(first < self.buffer.len());
        pass.set_vertex_buffer(
            slot,
            self.buffer.buf.slice(u64::from(first) * 8..),
        );
    }
    pub fn instances(&self) -> u32 {
        self.buffer.len() - 1
    }
    pub fn allocated_bytes(&self) -> u64 {
        self.buffer.buf.size()
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod cloud_shader_contract;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cloud_shader_contract;
mod shader_check;

mod draw_slots;
```

### 3. `src/draw_slots_tests.rs`

Create the file and type:

```rust
#[test]
fn actual_slots_keep_integer_ids_sentinel_capacity_and_release() {
    let gpu = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx = crate::text_gpu_support::GpuCtx::new(&gpu.device, &gpu.queue);
    let mut slots = crate::draw_slots::Slots::new(&ctx);
    let bytes = |rows: &[[u32; 2]]| {
        rows.iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect::<Vec<_>>()
    };
    assert_eq!(slots.instances(), 0);
    assert_eq!(slots.allocated_bytes(), 8);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 8),
        bytes(&[[u32::MAX, 0]])
    );
    slots.set(&ctx, &[[7, 16_777_217], [9, u32::MAX - 1]]);
    let capacity = slots.allocated_bytes();
    assert_eq!(slots.instances(), 2);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 24),
        bytes(&[
            [u32::MAX, 0],
            [7, 16_777_217],
            [9, u32::MAX - 1]
        ])
    );
    slots.clear(&ctx, false);
    assert_eq!(slots.instances(), 0);
    assert_eq!(slots.allocated_bytes(), capacity);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 8),
        bytes(&[[u32::MAX, 0]])
    );
    slots.set(&ctx, &[[13, 21]]);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 16),
        bytes(&[[u32::MAX, 0], [13, 21]])
    );
    slots.clear(&ctx, true);
    assert_eq!(slots.instances(), 0);
    assert_eq!(slots.allocated_bytes(), 8);
    assert_eq!(
        crate::growing_buffer_tests::read(&ctx, &slots.buffer.buf, 8),
        bytes(&[[u32::MAX, 0]])
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
mod shader_check;

mod draw_slots;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shader_check;

mod draw_slots;
#[cfg(test)]
mod draw_slots_tests;
```

## <a id="checkpoint-96zzo-read-instance-properties-through-draw-slots"></a>9. Read instance properties through draw slots

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU CommandEncoder Records GPU Commands</text>
  <rect x="54" y="50" width="264" height="102" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::CommandEncoder</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">1. begin_render_pass (3D Scene)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">2. begin_render_pass (Overlay/UI)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">3. encoder.finish()</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pass 1 (LoadOp::Clear) -&gt; Pass 2 (LoadOp::Load)</text>
  <rect x="416" y="52" width="128" height="96" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="480" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Clear</text>
  <polygon points="452,132 480,88 508,132" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="52" width="128" height="96" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Load</text>
  <polygon points="612,126 640,86 668,126" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <rect x="590" y="130" width="100" height="12" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.submit([cmd_buf]) -&gt; GPU Execution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,132 934,58 984,132" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="136" width="224" height="18" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="149" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Composited Frame Ready</text>
</svg>
</div>

### 1. `src/draw_slots.rs`

**Locate this existing block:**

```rust
    pub(crate) buffer: GrowBuf,
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
        };
        slots.set(ctx, &[]);
        slots
    }
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot]) {
        let bytes: Vec<u8> = std::iter::once([OWN_ROW, 0])
            .chain(rows.iter().copied())
            .flatten()
            .flat_map(u32::to_ne_bytes)
            .collect();
        self.buffer.reset();
        self.buffer.append_bytes(ctx, &bytes);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) buffer: GrowBuf,
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

### 2. `src/instance_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .sum(),
            self.batches
                .values()
                .map(|b| b.records.size())
                .sum(),
```

Replace it with:

```rust
                .sum(),
            self.batches
                .values()
                .map(|b| b.records.size() + b.slots.allocated_bytes())
                .sum(),
```

**Locate this block:**

```rust
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
```

Replace it with:

```rust
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            batch.slots.bind(pass, 0, 1);
            batch.geometry.draw_pulled(pass, batch.count);
        }
    }
```

**Locate this block:**

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
                    }
                };
                batches.insert(key, batch);
```

Replace it with:

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
```

**Locate this block:**

```rust
                    Rc::ptr_eq(&b.geometry, &geometry)
                        && b.records.size() >= bytes.len() as u64
                }) {
                    if batch.bytes != bytes {
                        queue.write_buffer(&batch.records, 0, &bytes);
                        self.writes += 1;
```

Replace it with:

```rust
                    Rc::ptr_eq(&b.geometry, &geometry)
                        && b.records.size() >= bytes.len() as u64
                }) {
                    if batch.slots.set(&ctx, &slots) {
                        self.writes += 1;
                        self.written_bytes += (slots.len() as u64 + 1) * 8;
                    }
                    if batch.bytes != bytes {
                        queue.write_buffer(&batch.records, 0, &bytes);
                        self.writes += 1;
```

**Locate this block:**

```rust
        let mut batches = BTreeMap::new();
        for (source, (geometry, records)) in groups {
            for (part, records) in records.chunks(limit).enumerate() {
                let key = (source, part);
                let bytes = records
                    .iter()
```

Replace it with:

```rust
        let mut batches = BTreeMap::new();
        for (source, (geometry, records)) in groups {
            for (part, records) in records.chunks(limit).enumerate() {
                let ctx = crate::text_gpu_support::GpuCtx::new(device, queue);
                let slots: Vec<crate::draw_slots::Slot> = records
                    .iter()
                    .enumerate()
                    .map(|(row, record)| {
                        [
                            row as u32,
                            u32::from_ne_bytes(record[64..68].try_into().unwrap()) >> 1,
                        ]
                    })
                    .collect();
                let key = (source, part);
                let bytes = records
                    .iter()
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
                    count: None,
                }],
            });
        let source = include_str!("triangle.wgsl")
            .replace(
                "var<uniform> object_settings: ObjectSettings;",
                "var<storage, read> objects: array<ObjectSettings>;",
            )
            .replace(
                "return object_settings;",
                "return objects[index];",
            );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shared definition placements"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(&source, 2, 0).into(),
            ),
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

Replace it with:

```rust
                    count: None,
                }],
            });
        let source = source();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shared definition placements"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

**Locate this block:**

```rust
    pub writes: u64,
    pub written_bytes: u64,
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
```

Replace it with:

```rust
    pub writes: u64,
    pub written_bytes: u64,
}
pub(crate) fn source() -> String {
    let source = include_str!("triangle.wgsl")
        .replace(
            "var<uniform> object_settings: ObjectSettings;",
            "var<storage, read> objects: array<ObjectSettings>;",
        )
        .replace(
            "return object_settings;",
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
            "(object.selection.x>>1u)+occurrence/3u+1u",
            "primitive_base+occurrence/3u+1u",
        );
    crate::clipping_gpu::source(&source, 2, 0)
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
```

**Locate this block:**

```rust
    count: u32,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
    count: u32,
    slots: crate::draw_slots::Slots,
}
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
```

### 3. `src/object_record_tests.rs`

**Locate this existing block:**

```rust
    let uniform = crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0);
    let instances = uniform
        .replace(
            "var<uniform> object_settings: ObjectSettings;",
            "var<storage, read> objects: array<ObjectSettings>;",
        )
        .replace(
            "return object_settings;",
            "return objects[index];",
        );
    for (source, offsets) in [
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let uniform = crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0);
    let instances = crate::instance_gpu::source();
    for (source, offsets) in [
```

### 4. `src/pick_gpu_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 352]);
    for row in scene.objects() {
```

Replace it with:

```rust
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 376]);
    for row in scene.objects() {
```

**Locate this block:**

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(&renderer.instance_usage()[..3], &[1, 2, 352]);
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

Replace it with:

```rust
    let mut renderer = renderer(&scene);
    assert_eq!(&renderer.instance_usage()[..3], &[1, 2, 376]);
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
```

## <a id="checkpoint-96zzp-classify-contact-fill-shading"></a>10. Classify contact fill shading

### 1. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let contact=(input.clip_flags&CONTACT_FLAG)!=0u;
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    if(contact){return vec4<f32>(input.colour,1.);}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color),oct32_decode(vertex.normal));
}
```

### 2. `src/face_pick.wgsl`

**Locate this existing block:**

```wgsl
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
```

### 3. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership() {
    let mut scene = Scene::empty();
    let mut prepared = PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();
    let original = prepared.geometry.to_proto();
    let vertices = prepared.display.vertices().len();
    prepared.display = prepared
        .display
        .with_alphas(vec![1. / 255.; vertices])
        .unwrap();
    let owner = scene.insert(prepared).unwrap();
    let mut gpu = renderer(&scene);
    let source = scene.objects()[0].mesh.clone();
    let color = source.vertices()[0][3..6].to_vec();
    let expected = color
        .iter()
        .map(|v| (v.clamp(0., 1.) * 255.).round() as u8)
        .collect::<Vec<_>>();
    let uploads = gpu.stats()[0];
    for opacity in [1., 0.5, 0.] {
        gpu.set_opacity(opacity);
        let image = pixels(&gpu);
        assert_eq!(
            &crate::cap_gpu_tests::at(&image, 160, 90)[..3],
            expected.as_slice()
        );
        gpu.request_pick([0., 0.25], matrix())
            .unwrap();
        wait(&gpu);
        assert_eq!(
            gpu.take_pick(matrix())
                .unwrap()
                .unwrap()
                .unwrap()[0],
            owner.value()
        );
    }
    gpu.select_objects(&scene, &[owner]).unwrap();
    let image = pixels(&gpu);
    assert_eq!(
        &crate::cap_gpu_tests::at(&image, 160, 90)[..3],
        &[255, 191, 13]
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(source.alphas(), vec![1. / 255.; vertices]);
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

## <a id="checkpoint-96zzq-open-files-through-one-measured-session-reader"></a>11. Open files through one measured Session reader

### 1. `src/load_measure.rs`

**Locate this existing block:**

```rust
) -> Result<T, &'static str> {
    let started = clock();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Result<T, &'static str> {
    run_result(name, bytes, clock, observe, operation)
}

pub fn run_result<T, E>(
    name: &str,
    bytes: u64,
    clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(Completed<'_>),
    operation: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    let started = clock();
```

### 2. `src/document_source.rs`

**Locate this existing block:**

```rust
}
pub(crate) fn prepare(
    bytes: &[u8],
    place: &Xform,
    point_px: f32,
) -> Result<Source, String> {
    if bytes.len() > crate::archive::LIMIT {
        return Err("Session exceeds the 512 MiB file limit".into());
    }
    let message = proto::Session::decode(bytes).map_err(|e| e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m) || !point_px.is_finite() {
        return Err("Invalid document placement or point size".into());
    }
    let tree = message
        .tree
        .as_ref()
        .and_then(|tree| tree.root.clone());
    let mut guids = HashSet::new();
    let objects = message
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    macro_rules! identity {
        ($field:ident) => {
            for object in &objects.$field {
                if object.guid.is_empty() || !guids.insert(object.guid.as_str()) {
                    return Err(
                        "Source GUIDs must be distinct within a document".into(),
                    );
                }
            }
        };
    }
    identity!(components);
    identity!(elements);
    identity!(planes);
    identity!(bboxes);
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    identity!(instances);
    identity!(sheets);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let sheets = objects
        .sheets
        .iter()
        .cloned()
        .map(|sheet| {
            crate::sheet_source::Prepared::new(Rc::new(sheet))
                .map(Rc::new)
                .map_err(str::to_owned)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    let world =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub(crate) fn observe(phase: crate::load_measure::Completed<'_>) {
    #[cfg(target_arch = "wasm32")]
    crate::browser_phase::completed(phase, "document");
    #[cfg(not(target_arch = "wasm32"))]
    let _ = phase;
}
pub(crate) fn prepare(
    bytes: &[u8],
    place: &Xform,
    point_px: f32,
) -> Result<Source, String> {
    prepare_at(bytes, place, point_px, None)
}
pub(crate) fn prepare_at(
    bytes: &[u8],
    place: &Xform,
    point_px: f32,
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
) -> Result<Source, String> {
    prepare_observed(
        bytes,
        place,
        point_px,
        location,
        &mut crate::load_measure::now,
        &mut observe,
    )
}
pub(crate) fn prepare_observed(
    bytes: &[u8],
    place: &Xform,
    point_px: f32,
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
    clock: &mut impl FnMut() -> f64,
    observe: &mut impl FnMut(crate::load_measure::Completed<'_>),
) -> Result<Source, String> {
    use crate::load_measure::run_result;
    let size = bytes.len() as u64;
    if bytes.len() > crate::archive::LIMIT {
        return Err("Session exceeds the 512 MiB file limit".into());
    }
    let message = run_result("decode", size, clock, observe, || {
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf".to_owned())
    })?;
    let (tree, sheets) = run_result(
        "validation",
        size,
        clock,
        observe,
        || -> Result<_, String> {
            crate::source_validate::session(&message)?;
            supported(&message)?;
            if !crate::placement::valid(&place.m) || !point_px.is_finite() {
                return Err("Invalid document placement or point size".into());
            }
            let tree = message
                .tree
                .as_ref()
                .and_then(|tree| tree.root.clone());
            let mut guids = HashSet::new();
            let objects = message
                .objects
                .as_ref()
                .ok_or("Session has no objects")?;
            macro_rules! identity {
                ($field:ident) => {
                    for object in &objects.$field {
                        if object.guid.is_empty() || !guids.insert(object.guid.as_str())
                        {
                            return Err(
                                "Source GUIDs must be distinct within a document"
                                    .into(),
                            );
                        }
                    }
                };
            }
            identity!(components);
            identity!(elements);
            identity!(planes);
            identity!(bboxes);
            identity!(meshes);
            identity!(points);
            identity!(lines);
            identity!(polylines);
            identity!(nurbscurves);
            identity!(nurbssurfaces);
            identity!(breps);
            identity!(pointclouds);
            identity!(instances);
            identity!(sheets);
            if let Some(root) = tree.as_ref() {
                crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
            }
            let mut placed = HashSet::new();
            for entry in &message.xforms {
                if !placed.insert(entry.guid.as_str())
                    || !crate::placement::valid(
                        &entry
                            .xform
                            .as_ref()
                            .ok_or("Missing placement matrix")?
                            .matrix,
                    )
                {
                    return Err("Invalid source placement".into());
                }
            }
            let sheets = objects
                .sheets
                .iter()
                .cloned()
                .map(|sheet| {
                    crate::sheet_source::Prepared::new(Rc::new(sheet))
                        .map(Rc::new)
                        .map_err(str::to_owned)
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok((tree, sheets))
        },
    )?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document = run_result("kernel", size, clock, observe, || {
        Session::from_proto(message)
            .map(Rc::new)
            .map_err(|e| e.to_string())
    })?;
    let world =
```

### 3. `src/source_phase_tests.rs`

Create the file and type:

```rust
use std::rc::Rc;
#[test]
fn full_element_reader_measures_actual_phases_and_preserves_location() {
    let bytes = crate::interaction_example::bytes();
    let location = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:full-phases".into(),
    ));
    let mut time = 0.;
    let mut phases = Vec::new();
    let source = crate::document_source::prepare_observed(
        &bytes,
        &session_rust::Xform::identity(),
        0.,
        Some(location.clone()),
        &mut || {
            time += 1.;
            time
        },
        &mut |p| {
            phases.push((
                p.name.to_owned(),
                p.duration_ms,
                p.elapsed_ms,
                p.bytes,
                p.succeeded,
            ))
        },
    )
    .unwrap();
    assert_eq!(
        phases
            .iter()
            .map(|p| p.0.as_str())
            .collect::<Vec<_>>(),
        ["decode", "validation", "kernel"]
    );
    for (i, p) in phases.iter().enumerate() {
        assert_eq!(p.1, 1.);
        assert_eq!(p.2, (i * 2 + 2) as f64);
        assert_eq!(p.3, bytes.len() as u64);
        assert!(p.4);
    }
    assert_eq!(source.document.objects.elements.len(), 3);
    assert!(!source.document.interactions.is_empty());
    assert!(Rc::ptr_eq(
        source.origin.location.as_ref().unwrap(),
        &location
    ));
}
#[test]
fn malformed_full_source_records_only_failed_decode() {
    let mut time = 0.;
    let mut phases = Vec::new();
    let result = crate::document_source::prepare_observed(
        &[255],
        &session_rust::Xform::identity(),
        0.,
        None,
        &mut || {
            time += 1.;
            time
        },
        &mut |p| phases.push((p.name.to_owned(), p.bytes, p.succeeded)),
    );
    assert_eq!(
        result.err().unwrap(),
        "Invalid session protobuf"
    );
    assert_eq!(phases, vec![("decode".into(), 1, false)]);
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod draw_slots_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod draw_slots_tests;

#[cfg(test)]
mod source_phase_tests;
```

### 5. `src/scene_document.rs`

**Locate this existing block:**

```rust
) -> Result<u32, String> {
    let source = crate::document_source::prepare(bytes, place, point_px)?;
    let mut candidate = scene.clone();
    let forest = candidate.forest.clone();
    let created = candidate.created;
    let current = candidate.view.current.clone();
    candidate.forest = Default::default();
    candidate.created = None;
    Rc::make_mut(&mut candidate.view).current = None;
    let key = candidate
        .reserve_document_key()
        .map_err(str::to_owned)?;
    let owners =
        crate::document_geometry::append(&mut candidate, &source, place, point_px)?;
    candidate.forest = forest;
    candidate.created = created;
    Rc::make_mut(&mut candidate.view).current = current;
    candidate.forest.add(
        key,
        if name.is_empty() {
            source.document.name.clone()
        } else {
            name.into()
        },
        source.tree,
        &owners,
    );
    candidate
        .forest
        .original_placements(key, &source.document, place, &source.world);
    Rc::make_mut(
        candidate
            .forest
            .documents
            .last_mut()
            .unwrap(),
    )
    .header = Some(source.origin);
    let elements = owners
        .iter()
        .filter_map(|(guid, id, _)| {
            (matches!(
                source.document.lookup.get(guid),
                Some(session_rust::Geometry::Element(_))
            ) || candidate.instances.contains_key(id))
            .then_some(*id)
        })
        .collect::<std::collections::HashSet<_>>();
    candidate
        .forest
        .classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) =
            source.document.lookup.get(guid)
        {
            crate::feature_display::append(
                &mut candidate,
                *id,
                &source.document,
                element,
            )
            .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner)
                .map_err(str::to_owned)?;
        }
    }
    let colours = candidate.forest.colours(key);
    Rc::make_mut(&mut candidate.view)
        .faces
        .extend(colours);
    scene.restore(candidate);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Result<u32, String> {
    import_at(scene, bytes, name, place, point_px, None)
}
pub fn import_at(
    scene: &mut Scene,
    bytes: &[u8],
    name: &str,
    place: &Xform,
    point_px: f32,
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
) -> Result<u32, String> {
    let source = crate::document_source::prepare_at(bytes, place, point_px, location)?;
    let mut candidate = scene.clone();
    let key = crate::load_measure::run_result(
        "display walk",
        bytes.len() as u64,
        &mut crate::load_measure::now,
        &mut crate::document_source::observe,
        || -> Result<u32, String> {
            let forest = candidate.forest.clone();
            let created = candidate.created;
            let current = candidate.view.current.clone();
            candidate.forest = Default::default();
            candidate.created = None;
            Rc::make_mut(&mut candidate.view).current = None;
            let key = candidate
                .reserve_document_key()
                .map_err(str::to_owned)?;
            let owners = crate::document_geometry::append(
                &mut candidate,
                &source,
                place,
                point_px,
            )?;
            candidate.forest = forest;
            candidate.created = created;
            Rc::make_mut(&mut candidate.view).current = current;
            candidate.forest.add(
                key,
                if name.is_empty() {
                    source.document.name.clone()
                } else {
                    name.into()
                },
                source.tree,
                &owners,
            );
            candidate.forest.original_placements(
                key,
                &source.document,
                place,
                &source.world,
            );
            Rc::make_mut(
                candidate
                    .forest
                    .documents
                    .last_mut()
                    .unwrap(),
            )
            .header = Some(source.origin);
            let elements = owners
                .iter()
                .filter_map(|(guid, id, _)| {
                    (matches!(
                        source.document.lookup.get(guid),
                        Some(session_rust::Geometry::Element(_))
                    ) || candidate.instances.contains_key(id))
                    .then_some(*id)
                })
                .collect::<std::collections::HashSet<_>>();
            candidate
                .forest
                .classify_elements(key, &elements);
            for (guid, id, _) in &owners {
                if let Some(session_rust::Geometry::Element(element)) =
                    source.document.lookup.get(guid)
                {
                    crate::feature_display::append(
                        &mut candidate,
                        *id,
                        &source.document,
                        element,
                    )
                    .map_err(str::to_owned)?;
                }
            }
            for (_, id, _) in &owners {
                if let Some(owner) = candidate.instances.get(id).cloned() {
                    crate::instance_features::append(&mut candidate, *id, &owner)
                        .map_err(str::to_owned)?;
                }
            }
            let colours = candidate.forest.colours(key);
            Rc::make_mut(&mut candidate.view)
                .faces
                .extend(colours);
            Ok(key)
        },
    )?;
    scene.restore(candidate);
```

### 6. `src/publication.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        assert_eq!(editor.scene.clouds().len(), 1);
    }
}
```

Replace it with:

```rust
        assert_eq!(editor.scene.clouds().len(), 1);
    }
}

#[test]
fn full_element_file_preparation_retains_originals_interactions_and_shared_reload_origin()
 {
    use prost::Message;
    use session_rust::Geometry;
    let scene = Scene::demo();
    let before = scene.all_ids();
    let bytes = crate::interaction_example::bytes();
    let location = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:full-file".into(),
    ));
    let ready = prepare(&scene, &bytes, true, Some(location.clone())).unwrap();
    let originals = ready
        .objects()
        .iter()
        .filter_map(|row| match row.original.as_ref() {
            Some(Geometry::Element(element)) => Some(element.guid().to_owned()),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    let message = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    for element in &message.objects.as_ref().unwrap().elements {
        assert!(originals.contains(&element.guid));
    }
    let origin = ready.forest.documents[0]
        .header
        .as_ref()
        .unwrap();
    assert!(Rc::ptr_eq(
        origin.location.as_ref().unwrap(),
        &location
    ));
    assert!(!origin.header.interactions.is_empty());
    assert_eq!(scene.all_ids(), before);
    assert_eq!(
        prepare(&scene, &[255], true, None)
            .err()
            .unwrap(),
        "Invalid session protobuf"
    );
    assert_eq!(scene.all_ids(), before);
}
```

**Locate this block:**

```rust
    }
    let url = location
        .as_ref()
        .map(|location| location.value().to_owned());
    match crate::document::load_at(bytes, location) {
        Ok(loaded) => candidate
            .import(loaded)
            .map_err(str::to_owned)?,
        Err(_) => {
            let previous = candidate.all_ids();
            crate::scene_document::import(
                &mut candidate,
                bytes,
                "",
                &session_rust::Xform::identity(),
                0.,
            )?;
            if let Some(url) = url {
                candidate.sheet_import_url(&url, &previous);
            }
        }
    }
    Ok(candidate)
}
```

Replace it with:

```rust
    }
    let previous = candidate.all_ids();
    let url = location
        .as_ref()
        .map(|location| location.value().to_owned());
    crate::scene_document::import_at(
        &mut candidate,
        bytes,
        "",
        &session_rust::Xform::identity(),
        0.,
        location,
    )?;
    if let Some(url) = url {
        candidate.sheet_import_url(&url, &previous);
    }
    Ok(candidate)
}
```

## <a id="checkpoint-96zzr-route-current-close-and-opacity-commands"></a>12. Route current Close and Opacity commands

### 1. `src/commands.rs`

**Locate this existing block:**

```rust
    }),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }),
    ("Close", || Action::CloseTool),
    ("Unload Sources", || Action::UnloadSources),
```

### 2. `src/editor.rs`

**Locate this existing block:**

```rust
                    .ok_or("No point tool is running")?;
                if draft.gather.is_some()
                    || draft.project.is_some()
                    || draft.extend.is_some()
                {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    .ok_or("No point tool is running")?;
                if self.plane_only
                    || draft.plane_points.is_some()
                    || !matches!(draft.tool.name(), "Polyline" | "Curve")
                {
```

### 3. `src/commands.rs`

**Locate this existing block:**

```rust
    if name == "Opacity" {
        let [word] = &words[count..] else {
            return Err("Opacity 0..1 · Example: Opacity 0.5".into());
        };
        let value = word
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
            .ok_or("Opacity takes a value from 0 to 1")?;
        return Ok(Command::Editor(Action::Opacity(value)));
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if name == "Opacity" {
        let word = words
            .get(count)
            .ok_or("Opacity 0..1 · Example: Opacity 0.5")?;
        let value = word
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
            .ok_or("Opacity takes a value from 0 to 1")?;
        return Ok(Command::Editor(Action::Opacity(
            value as f32,
        )));
    }
```

### 4. `src/opacity_command_tests.rs`

**Locate this existing block:**

```rust
        "Opacity NaN",
        "Opacity 0.5 extra",
    ] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "Opacity NaN",
        "Opacity 1.000000001 extra",
    ] {
```

### 5. `src/current_command_tests.rs`

Create the file and type:

```rust
use crate::{
    commands::{Command, parse},
    editor::{Action, Editor},
};
fn command(e: &mut Editor, line: &str) -> Result<crate::editor::Change, &'static str> {
    let Command::Editor(a) = parse(line).unwrap() else {
        panic!("editor command")
    };
    e.apply(a)
}
#[test]
fn close_is_one_curve_transaction_and_never_releases_the_document() {
    let mut e = Editor::default();
    let original = e
        .scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let ids = e.scene.all_ids();
    assert!(command(&mut e, "Close").is_err());
    assert_eq!(e.scene.all_ids(), ids);
    for shape in [
        crate::tool::Shape::Polyline,
        crate::tool::Shape::Curve,
    ] {
        e.apply(Action::BeginTool(shape)).unwrap();
        for p in [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.]] {
            e.apply(Action::ToolPoint(p)).unwrap();
        }
        command(&mut e, "Close").unwrap();
        assert!(e.draft.is_none());
        assert_eq!(e.scene.all_ids().len(), ids.len() + 1);
        let added = e
            .scene
            .all_ids()
            .difference(&ids)
            .copied()
            .collect::<Vec<_>>();
        let geometry = crate::source_edit::original(&e.scene, added[0]).unwrap();
        assert!(match geometry {
            session_rust::Geometry::Polyline(p) => p.is_closed(),
            session_rust::Geometry::NurbsCurve(c) => c.is_closed(),
            _ => false,
        });
        e.apply(Action::Undo).unwrap();
        assert_eq!(e.scene.all_ids(), ids);
        assert_eq!(
            e.scene
                .objects()
                .iter()
                .map(|o| o.geometry().unwrap().to_proto())
                .collect::<Vec<_>>(),
            original
        );
    }
    e.apply(Action::BeginTool(
        crate::tool::Shape::Polyline,
    ))
    .unwrap();
    e.apply(Action::ToolPoint([0., 0., 0.]))
        .unwrap();
    let points = e.draft.as_ref().unwrap().points.clone();
    assert!(command(&mut e, "Close").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points, points);
    assert_eq!(e.scene.all_ids(), ids);
    e.apply(Action::Plane3).unwrap();
    assert!(command(&mut e, "Close").is_err());
    assert!(
        e.draft
            .as_ref()
            .unwrap()
            .plane_points
            .is_some()
    );
    assert_eq!(e.scene.all_ids(), ids);
}
#[test]
fn opacity_reads_only_the_first_finite_bounded_double() {
    let mut e = Editor::default();
    e.apply(Action::BeginTool(
        crate::tool::Shape::Polyline,
    ))
    .unwrap();
    e.apply(Action::ToolPoint([0., 0., 0.]))
        .unwrap();
    let ids = e.scene.all_ids();
    let epoch = e.interaction_epoch;
    for text in [
        "Opacity 0.5 extra",
        "Opacity 0.5 NaN",
        "Opacity 0.5 10",
    ] {
        command(&mut e, text).unwrap();
        assert_eq!(e.opacity, 0.5);
        assert_eq!(e.scene.all_ids(), ids);
        assert_eq!(
            e.draft.as_ref().unwrap().points,
            vec![[0., 0., 0.]]
        );
        assert_eq!(e.interaction_epoch, epoch);
    }
    for text in [
        "Opacity",
        "Opacity NaN extra",
        "Opacity -0.000000001 extra",
        "Opacity 1.000000001 extra",
    ] {
        assert!(parse(text).is_err());
        assert_eq!(e.opacity, 0.5);
    }
}
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod source_phase_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod source_phase_tests;

#[cfg(test)]
mod current_command_tests;
```

### 7. `src/inventory_tests.rs`

**Locate this existing block:**

```rust
    assert!(command::parse("Opacity 0.4 extra").is_ok());
    assert!(crate::commands::parse("Opacity 0.4 extra").is_err());
    for line in [
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(command::parse("Opacity 0.4 extra").is_ok());
    assert!(
        matches!(crate::commands::parse("Opacity 0.4 extra").unwrap(),crate::commands::Command::Editor(crate::editor::Action::Opacity(value)) if value==0.4)
    );
    for line in [
```

## <a id="checkpoint-96zzs-reuse-full-source-preparation-for-editor-and-reload"></a>13. Reuse full source preparation for editor and reload

### 1. `src/editor.rs`

**Locate this existing block:**

```rust
            Action::AddNested => {
                let loaded =
                    crate::document::load(&crate::tree_specimen::nested(true))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::Live(ready) => {
                self.history.clear();
                self.scene.restore(*ready);
            }
            Action::Adopt { scene: ready, .. } => {
                self.history.edit(&mut self.scene, |scene| {
                    scene.restore(*ready)
                });
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Action::AddNested => {
                let ready = crate::publication::prepare(
                    &self.scene,
                    &crate::tree_specimen::nested(true),
                    false,
                    None,
                )
                .map_err(|_| "Cannot prepare nested originals")?;
                self.history
                    .edit(&mut self.scene, |scene| scene.restore(ready));
            }
            Action::Live(ready) => {
                self.history.clear();
                self.scene.restore(*ready);
            }
            Action::Adopt { scene: ready, .. } => {
                self.history.edit(&mut self.scene, |scene| {
                    scene.restore(*ready)
                });
            }
            file @ (Action::Replace(_)
            | Action::Import(_)
            | Action::ImportAt(_, _)
            | Action::ReplaceAt(_, _)) => {
                let (bytes, replace, location) = match file {
                    Action::Replace(bytes) => (bytes, true, None),
                    Action::Import(bytes) => (bytes, false, None),
                    Action::ImportAt(bytes, location) => (bytes, false, Some(location)),
                    Action::ReplaceAt(bytes, location) => (bytes, true, Some(location)),
                    _ => unreachable!(),
                };
                let ready =
                    crate::publication::prepare(&self.scene, &bytes, replace, location)
                        .map_err(|_| "Cannot prepare original document")?;
                self.history
                    .edit(&mut self.scene, |scene| scene.restore(ready));
            }
```

### 2. `src/document.rs`

**Locate this existing block:**

```rust
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    let message = decode_raw(bytes)?;
    validate(&message)?;
    Ok(message)
}

pub(crate) fn restore(
    bytes: &[u8],
    origin: &crate::origin::Origin,
) -> Result<Rc<Session>, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    if crate::origin::FileVersion::of(bytes) != origin.version {
        return Err("Reloaded file version changed");
    }
    Session::from_proto(decode(bytes)?)
        .map(Rc::new)
        .map_err(|_| "Cannot restore source session")
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn restore(
    bytes: &[u8],
    origin: &crate::origin::Origin,
) -> Result<Rc<Session>, &'static str> {
    if crate::origin::FileVersion::of(bytes) != origin.version {
        return Err("Reloaded file version changed");
    }
    crate::document_source::prepare_at(
        bytes,
        &session_rust::Xform::identity(),
        0.,
        origin.location.clone(),
    )
    .map(|source| source.document)
    .map_err(|_| "Cannot restore source session")
}
```

### 3. `src/full_source_action_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    origin::Origin,
    reload_url::ReloadUrl,
};
use prost::Message;
use session_rust::{Geometry, proto};
use std::rc::Rc;
#[test]
fn all_editor_byte_actions_accept_full_elements_and_keep_one_undo_and_source_location()
{
    let bytes = crate::interaction_example::bytes();
    let message = proto::Session::decode(bytes.as_slice()).unwrap();
    for (replace, located) in [
        (false, false),
        (true, false),
        (false, true),
        (true, true),
    ] {
        let mut e = Editor::default();
        let before = e.scene.all_ids();
        let url = Rc::new(ReloadUrl::new("test:editor-full".into()));
        let action = match (replace, located) {
            (false, false) => Action::Import(bytes.clone()),
            (true, false) => Action::Replace(bytes.clone()),
            (false, true) => Action::ImportAt(bytes.clone(), url.clone()),
            (true, true) => Action::ReplaceAt(bytes.clone(), url.clone()),
        };
        e.apply(action).unwrap();
        let after = e.scene.all_ids();
        assert_ne!(after, before);
        assert_eq!(before.is_subset(&after), !replace);
        let guids = e
            .scene
            .objects()
            .iter()
            .filter_map(|row| match row.original.as_ref() {
                Some(Geometry::Element(element)) => Some(element.guid().to_owned()),
                _ => None,
            })
            .collect::<std::collections::HashSet<_>>();
        for element in &message.objects.as_ref().unwrap().elements {
            assert!(guids.contains(&element.guid));
        }
        let origin = e
            .scene
            .forest
            .documents
            .last()
            .unwrap()
            .header
            .as_ref()
            .unwrap();
        assert_eq!(
            origin.header.interactions,
            message.interactions
        );
        assert_eq!(origin.location.is_some(), located);
        if located {
            assert!(Rc::ptr_eq(
                origin.location.as_ref().unwrap(),
                &url
            ));
        }
        e.apply(Action::Undo).unwrap();
        assert_eq!(e.scene.all_ids(), before);
        e.apply(Action::Redo).unwrap();
        assert_eq!(e.scene.all_ids(), after);
        assert!(e.apply(Action::Replace(vec![255])).is_err());
        assert_eq!(e.scene.all_ids(), after);
        e.apply(Action::Undo).unwrap();
        assert_eq!(e.scene.all_ids(), before);
        e.apply(Action::Redo).unwrap();
        assert_eq!(e.scene.all_ids(), after);
    }
}
#[test]
fn released_mesh_reload_accepts_its_full_mixed_original_file_and_rejects_changed_bytes()
{
    let mut message =
        proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();
    let point = session_rust::Point::new(2.123456789, 3., 4.);
    let _ = point.guid();
    let original = point.to_proto();
    let guid = original.guid.clone();
    message
        .objects
        .as_mut()
        .unwrap()
        .points
        .push(original.clone());
    message
        .tree
        .as_mut()
        .unwrap()
        .root
        .as_mut()
        .unwrap()
        .children
        .push(proto::TreeNode {
            name: guid,
            ..Default::default()
        });
    let bytes = message.encode_to_vec();
    let mut origin = Origin::new(&message, &bytes);
    origin.location = Some(Rc::new(ReloadUrl::new(
        "test:mixed-reload".into(),
    )));
    let source = crate::document::restore(&bytes, &origin).unwrap();
    assert_eq!(source.objects.points.len(), 1);
    assert_eq!(source.objects.points[0].to_proto(), original);
    assert_eq!(
        source.objects.meshes.len(),
        message
            .objects
            .as_ref()
            .unwrap()
            .meshes
            .len()
    );
    for mesh in &source.objects.meshes {
        assert!(
            message
                .objects
                .as_ref()
                .unwrap()
                .meshes
                .contains(&mesh.to_proto())
        );
    }
    assert_eq!(
        crate::document::restore(&[255], &origin)
            .err()
            .unwrap(),
        "Reloaded file version changed"
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod current_command_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod current_command_tests;

#[cfg(test)]
mod full_source_action_tests;
```

## <a id="checkpoint-96zzt-share-the-current-selection-yellow"></a>14. Share the current selection yellow

### 1. `src/colour.rs`

**Locate this existing block:**

```rust
    mesh
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    mesh
}

pub fn selection() -> [f32; 4] {
    Color::yellow().to_unified_array()
}
pub fn selection_shader() -> String {
    let [r, g, b, _] = selection();
    format!("const SELECTION_COLOR: vec3<f32> = vec3<f32>({r:?}, {g:?}, {b:?});")
}
```

### 2. `src/highlight.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let yellow = [1., 0.75, 0.05, 1.];
        self.strokes.set(
```

Replace it with:

```rust
        ids: &[crate::scene::ObjectId],
    ) -> Result<(), &'static str> {
        self.mesh = None;
        let yellow = crate::colour::selection();
        self.strokes.set(
```

**Locate this block:**

```rust
        let mut markers = Vec::new();
        let yellow = [1., 0.75, 0.05, 1.];
        if let Some(target) = target.filter(|t| t.valid(scene)) {
            match target.part {
                Part::Face(key) => {
```

Replace it with:

```rust
        let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) {
            match target.part {
                Part::Face(key) => {
```

### 3. `src/tool_preview.rs`

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        let yellow = [1., 0.75, 0.05, 1.];
        let mut strokes = Vec::new();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        let yellow = crate::colour::selection();
        let mut strokes = Vec::new();
```

### 4. `src/clipping_gpu.rs`

**Locate this existing block:**

```rust
    format!(
        "@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        concat!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    format!(
        "{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        crate::colour::selection_shader(),
        concat!(
```

### 5. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
```

### 6. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
        &crate::cap_gpu_tests::at(&image, 160, 90)[..3],
        &[255, 191, 13]
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(source.alphas(), vec![1. / 255.; vertices]);
    scene.clear();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &crate::cap_gpu_tests::at(&image, 160, 90)[..3],
        &[255, 255, 0]
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(source.alphas(), vec![1. / 255.; vertices]);
    scene.clear();
    gpu.set_scene(&scene, None);
    assert_eq!(gpu.usage(), [0; 4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color()
{
    use wgpu::naga;
    let rgba = crate::colour::selection();
    assert_eq!(
        rgba,
        session_rust::Color::yellow().to_unified_array()
    );
    assert_eq!(rgba, [1., 1., 0., 1.]);
    let shader = crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0);
    assert!(shader.contains(&crate::colour::selection_shader()));
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&naga::front::wgsl::parse_str(&shader).unwrap())
    .unwrap();
    let mut scene = Scene::empty();
    let prepared = PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();
    let original = prepared.geometry.to_proto();
    let owner = scene.insert(prepared).unwrap();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let plain = pixels(&gpu);
    let uploads = gpu.stats()[0];
    gpu.select_objects(&scene, &[owner]).unwrap();
    let selected = pixels(&gpu);
    assert!(
        selected
            .chunks_exact(4)
            .any(|p| p[0] > 10 && p[0] == p[1] && p[2] == 0)
    );
    assert_ne!(plain, selected);
    gpu.request_pick([0., 0.25], matrix())
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(matrix())
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    gpu.select_objects(&scene, &[]).unwrap();
    assert_eq!(pixels(&gpu), plain);
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    scene.clear();
```

### 7. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
        let p = pixel(&selected, x);
        assert!(p[0] > p[1] && p[1] > p[2]);
    }
    renderer
        .select_objects(&scene, &[b])
        .unwrap();
    let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96));
    assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(
        &renderer.stats()[..2],
        &geometry[..2],
        "Only selected settings change, not primary geometry"
    );
    for (x, id) in [(-0.4, a), (0.4, b)] {
        renderer
            .request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap();
    draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap();
    assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "The actual unfinished rubber band and cursor render"
    );
    let p = &after[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 3];
    assert!(
        p[0] > p[1] && p[1] > p[2],
        "The preview is yellow: {p:?}"
    );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let p = pixel(&selected, x);
        assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0);
    }
    renderer
        .select_objects(&scene, &[b])
        .unwrap();
    let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96));
    assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(
        &renderer.stats()[..2],
        &geometry[..2],
        "Only selected settings change, not primary geometry"
    );
    for (x, id) in [(-0.4, a), (0.4, b)] {
        renderer
            .request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap();
    draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap();
    assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "The actual unfinished rubber band and cursor render"
    );
    let p = &after[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 3];
    assert_eq!(
        p,
        &[255, 255, 0],
        "The preview uses current selection yellow: {p:?}"
    );
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.3
npm --prefix ../session_tests run course -- save 20.3
```

Continue with [20.4 · Slots, independent instance owners and stable record updates](../../consolidation/96zzu-separate-primitive-addresses-from-selection.md).
