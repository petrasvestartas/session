/// One buffer binding, visible to `stages`.
fn buffer_entry(
    binding: u32,
    stages: wgpu::ShaderStages,
    ty: wgpu::BufferBindingType,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

/// A read-only storage buffer at `binding`, for the vertex stage.
fn storage_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    buffer_entry(
        binding,
        wgpu::ShaderStages::VERTEX,
        wgpu::BufferBindingType::Storage { read_only: true },
    )
}

/// A layout with one uniform buffer at binding 0.
fn uniform_layout(
    device: &wgpu::Device,
    label: &str,
    stages: wgpu::ShaderStages,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}

/// Group 2: object rows at binding 0, translations at 1.
fn instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance.layout"),
        entries: &[storage_entry(0), storage_entry(1)],
    })
}

/// A depth texture binding for the fragment stage.
fn scene_depth(binding: u32, multisampled: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Depth,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

/// Group 2 for ink: rows, translations, depth at 1x and 4x, gradient at 1x and 4x, triangles, tiles.
fn ink_instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.instance.layout"),
        entries: &[
            buffer_entry(
                0,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                1,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            scene_depth(2, false),
            scene_depth(3, true),
        ],
    })
}

// --8<-- [start:step-5a]
/// Group 3 for lines: rows, source edge ids, selected edge.
fn segment_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("segment.rows.layout"),
        entries: &[
            storage_entry(0),
            storage_entry(1),
            buffer_entry(
                2,
                wgpu::ShaderStages::VERTEX,
                wgpu::BufferBindingType::Uniform,
            ),
        ],
    })
}

/// The bind group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout, // group 0: camera matrix
    pub line: wgpu::BindGroupLayout, // group 1: pen and view settings
    pub instance: wgpu::BindGroupLayout, // group 2: object rows
    pub ink_instance: wgpu::BindGroupLayout, // group 2 for ink, with depth textures
    pub segment_rows: wgpu::BindGroupLayout, // group 3 for lines
    // --8<-- [end:step-5a]
}

impl Layouts {
    /// Build every layout once.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(device, "mvp.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
            line: uniform_layout(device, "line.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
            instance: instance_layout(device),
            ink_instance: ink_instance_layout(device),
            // --8<-- [start:step-5b]
            segment_rows: segment_rows_layout(device),
            // --8<-- [end:step-5b]
        }
    }
}
