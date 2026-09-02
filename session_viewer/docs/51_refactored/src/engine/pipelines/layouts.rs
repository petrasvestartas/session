//! `Layouts` — every bind-group layout the viewer binds, built once per device.
//! A layout is the SHAPE of a bind group (binding index, stages, buffer kind); the buffers
//! themselves live in `gpu/`. Pipelines and bind groups both reference these, never their own.

/// One buffer binding, visible to `stages`.
fn buffer_entry(binding: u32, stages: wgpu::ShaderStages, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

/// One uniform buffer at binding 0.
fn uniform_layout(device: &wgpu::Device, label: &str, stages: wgpu::ShaderStages) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}

/// One read-only storage buffer at binding 0, vertex-visible: the row table every ink lane reads.
fn storage_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Storage { read_only: true })],
    })
}

/// The instance group: the 96 B rows at binding 0 and the 16 B anchored translations at
/// binding 1 - split so a re-anchor rewrites 16 B per object instead of 96.
fn instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let read = wgpu::BufferBindingType::Storage { read_only: true };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX, read),
            buffer_entry(1, wgpu::ShaderStages::VERTEX, read),
        ],
    })
}

/// A compute-visible buffer binding for the splat groups.
fn splat_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    buffer_entry(binding, wgpu::ShaderStages::COMPUTE, wgpu::BufferBindingType::Storage { read_only })
}

/// Splat group 0: the frame (mvp, cloud uniform) and the record table.
fn splat_group0_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.group0.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::COMPUTE, wgpu::BufferBindingType::Uniform),
            buffer_entry(1, wgpu::ShaderStages::COMPUTE, wgpu::BufferBindingType::Uniform),
            splat_entry(2, true),
        ],
    })
}

/// Splat group 1: a lane's points (pos, col, nrm) and the shared per-pixel depth/colour buffers.
fn splat_group1_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.group1.layout"),
        entries: &[
            splat_entry(0, true),
            splat_entry(1, true),
            splat_entry(2, false),
            splat_entry(3, false),
            splat_entry(4, true),
        ],
    })
}

/// The resolve pass reads the two per-pixel splat buffers from its fragment stage.
fn splat_resolve_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let read = wgpu::BufferBindingType::Storage { read_only: true };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.resolve.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::FRAGMENT, read),
            buffer_entry(1, wgpu::ShaderStages::FRAGMENT, read),
        ],
    })
}

/// The eight bind-group layouts. Group scheme for every draw: 0 = mvp, 1 = line/cloud uniform,
/// 2 = instances (rows + translations), 3 = the family's row table.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub segment: wgpu::BindGroupLayout,
    pub glyph: wgpu::BindGroupLayout,
    pub splat_group0: wgpu::BindGroupLayout,
    pub splat_group1: wgpu::BindGroupLayout,
    pub splat_resolve: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build every layout once; they outlive any pipeline or bind group made from them.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(device, "mvp.layout", wgpu::ShaderStages::VERTEX),
            // FRAGMENT too: the splat resolve reads the cloud uniform (bound with this layout)
            // from its fragment stage.
            line: uniform_layout(device, "line.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
            instance: instance_layout(device),
            segment: storage_layout(device, "segments.layout"),
            glyph: storage_layout(device, "glyphs.layout"),
            splat_group0: splat_group0_layout(device),
            splat_group1: splat_group1_layout(device),
            splat_resolve: splat_resolve_layout(device),
        }
    }
}
