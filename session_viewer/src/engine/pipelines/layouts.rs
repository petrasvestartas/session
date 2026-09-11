//! `Layouts` — every bind-group layout the viewer binds, built once per device. A layout is
//! the SHAPE of a bind group; the buffers live in `gpu/`. Group scheme for every draw:
//! 0 = mvp, 1 = line/pen uniform, 2 = instances (rows + anchored translations), 3 = the lane's rows.

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

/// A read-only storage buffer at `binding`, vertex-visible.
fn storage_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    buffer_entry(
        binding,
        wgpu::ShaderStages::VERTEX,
        wgpu::BufferBindingType::Storage { read_only: true },
    )
}

/// One uniform buffer at binding 0.
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

/// The instance group: 96 B rows at binding 0 and 16 B anchored translations at binding 1,
/// split so a re-anchor rewrites 16 B per object instead of 96.
fn instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance.layout"),
        entries: &[
            buffer_entry(
                0,
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                1,
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
        ],
    })
}

/// The physical depth, sampled by the ink after the face pass finishes.
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

/// The immutable raster gradient, sampled without filtering.
fn scene_gradient(binding: u32, multisampled: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

/// Ink keeps the instance rows (0 rows, 1 anchored translations) and adds everything a
/// fragment needs to decide it is visible: physical depth at 2 single-sampled and 3
/// multisampled, the gradient the face pass wrote at 4 and 5, and the finite-triangle pool at
/// 6 (the projected triangles) and 7 (the per-tile lists). Only one of each sampled pair is
/// real; the other is the 1x1 placeholder `Targets::new` makes at the unused sample count.
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
            scene_gradient(4, false),
            scene_gradient(5, true),
            buffer_entry(
                6,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                7,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
        ],
    })
}

/// The ink lanes' row table, read by the vertex stage (and the fragment stage of the id pass).
fn ink_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.rows.layout"),
        entries: &[buffer_entry(
            0,
            wgpu::ShaderStages::VERTEX_FRAGMENT,
            wgpu::BufferBindingType::Storage { read_only: true },
        )],
    })
}

/// Segment rows retain source-edge IDs and one shared selection uniform.
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

/// The point lane's group: the record table at 0, then positions, colours, normals.
fn points_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("points.layout"),
        entries: &[
            storage_entry(0),
            storage_entry(1),
            storage_entry(2),
            storage_entry(3),
        ],
    })
}

/// The resolve pass reads the point lane's depth and colour targets from its fragment stage.
fn resolve_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.resolve.layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    })
}

/// The bind-group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub ink_instance: wgpu::BindGroupLayout,
    pub ink_rows: wgpu::BindGroupLayout,
    pub segment_rows: wgpu::BindGroupLayout,
    pub points: wgpu::BindGroupLayout,
    pub resolve: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build every layout once; they outlive any pipeline or bind group made from them.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(
                device,
                "mvp.layout",
                wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ),
            line: uniform_layout(
                device,
                "line.layout",
                wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ),
            instance: instance_layout(device),
            ink_instance: ink_instance_layout(device),
            ink_rows: ink_rows_layout(device),
            segment_rows: segment_rows_layout(device),
            points: points_layout(device),
            resolve: resolve_layout(device),
        }
    }
}
