# 20.1.1 · Bind the shared geometry as storage

**Chapter 20 · Shared GPU rendering · Section 20.1 · Shared geometry and primitive addresses** · [Contents](contents.md)

**Typing: 24–47 minutes.** [Estimate](typing-load.md).

A triangle address requires knowing which index occurrence is being rasterized, not just which source vertex was fetched by indexed drawing.

## Type

Continue from [Rebuild an edited Element without replacing its owner](96zg-rebuild-an-edited-element-without-replacing-its-owner.md). [Save or recover your work](recovery.md).

### 1. `src/gpu_geometry.rs`

Keep the vertex-pulling bind group beside the shared buffers. Share explicit storage bindings and one buffer allocation helper; empty storage arrays retain a four-byte safe binding. Prepare one nonindexed invocation per index occurrence without changing existing drawing.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source.vertices().iter().flatten()
            .flat_map(|value| value.to_ne_bytes()).collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"), contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        let bytes: Vec<u8> = source.normals().iter().flatten().flat_map(|value| value.to_ne_bytes()).collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh normals"), contents: &bytes, usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source.indices().iter()
            .flat_map(|index| index.to_ne_bytes()).collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"), contents: &bytes,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
        });
        let index_count = source.indices().len() as u32;
        Self { source, vertices, normals, indices, index_count }
    }

    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.normals.size() + self.indices.size() }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>,count:u32) {
        if self.index_count==0||count==0{return;}
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.normals.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..count);
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(4),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let bytes = source.vertices().iter().flatten().flat_map(|v| v.to_ne_bytes()).collect();
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.normals().iter().flatten().flat_map(|v| v.to_ne_bytes()).collect();
        let normals = buffer(device, "shared mesh normals", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let buffers = [&vertices, &normals, &indices];
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupEntry {
            binding, resource: buffers[binding as usize].as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self { source, vertices, normals, indices, index_count, pull_group }
    }

    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.normals.size() + self.indices.size() }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>,count:u32) {
        if self.index_count==0||count==0{return;}
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.normals.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..count);
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zh-bind-the-shared-geometry-as-storage-1.rs"
```

```text
--8<-- "journey/code/96zh-bind-the-shared-geometry-as-storage-2.rs"
```

```text
--8<-- "journey/code/96zh-bind-the-shared-geometry-as-storage-3.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.1.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The original CAD view, colors, normal diagnostic, selection and integer pick owners remain stable while this GPU boundary connects. The supplied GPU fixtures check its buffer or primitive-address behavior.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for bind the shared geometry as storage.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Prepare vertex pulling through the three existing buffers. The position/color buffer and index buffer already have STORAGE usage; add it to the normal buffer. A shared explicit layout binds all three as read-only vertex-stage storage. Keep the original indexed draw path for the existing picking and mask pipelines. This preparatory step does not activate the new color draw path yet.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why retain one geometry buffer owner?

Changing how a shader reads vertices must not upload a second copy of positions, normals or indices. The same original geometry still serves all placed instances.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Rebuild and compare the existing scene and GPU geometry byte counts. The ordinary nonempty meshes should retain their buffer sizes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.1.1
npm --prefix ../session_tests run course -- save 20.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare shared vertex pulling for current primitive addressing without a second geometry upload. Primitive MRT, packed vertices, MSAA and current finite visibility remain subsequent bridges.

Chrome loads this checkpoint and passes the startup, command focus and navigation checks.

[Full validation scope](release.md).

</details>
