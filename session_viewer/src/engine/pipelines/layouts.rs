use crate::engine::pipelines::bindings::{buffer_entry, texture_entry};
use wgpu::{BufferBindingType, ShaderStages, TextureSampleType};
/// A read-only storage buffer at `binding`, for the vertex stage.
fn storage_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    buffer_entry(
        binding,
        ShaderStages::VERTEX,
        BufferBindingType::Storage { read_only: true },
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
        entries: &[buffer_entry(0, stages, BufferBindingType::Uniform)],
    })
}

/// Group 1: pen and view settings at binding 0, clipping planes at 1.
fn line_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let stages = ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE;
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("line.layout"),
        entries: &[
            buffer_entry(0, stages, BufferBindingType::Uniform),
            buffer_entry(1, stages, BufferBindingType::Uniform),
        ],
    })
}

/// Group 2: object rows at binding 0, translations at 1.
fn instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance.layout"),
        entries: &[
            buffer_entry(
                0,
                ShaderStages::VERTEX | ShaderStages::COMPUTE,
                BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                1,
                ShaderStages::VERTEX | ShaderStages::COMPUTE,
                BufferBindingType::Storage { read_only: true },
            ),
        ],
    })
}

/// A depth texture binding for the fragment stage.
fn scene_depth(binding: u32, multisampled: bool) -> wgpu::BindGroupLayoutEntry {
    texture_entry(
        binding,
        ShaderStages::FRAGMENT,
        TextureSampleType::Depth,
        multisampled,
    )
}

/// A triangle id texture binding for the fragment stage.
fn scene_gradient(binding: u32, multisampled: bool) -> wgpu::BindGroupLayoutEntry {
    texture_entry(
        binding,
        ShaderStages::FRAGMENT,
        TextureSampleType::Uint,
        multisampled,
    )
}

/// Bind group 2 for ink: rows, depths, triangle ids, triangles, tiles.
fn ink_instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.instance.layout"),
        entries: &[
            buffer_entry(
                0,
                ShaderStages::VERTEX_FRAGMENT,
                BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                1,
                ShaderStages::VERTEX_FRAGMENT,
                BufferBindingType::Storage { read_only: true },
            ),
            scene_depth(2, false),
            scene_depth(3, true),
            scene_gradient(4, false),
            scene_gradient(5, true),
            buffer_entry(
                6,
                ShaderStages::FRAGMENT,
                BufferBindingType::Storage { read_only: true },
            ),
            buffer_entry(
                7,
                ShaderStages::FRAGMENT,
                BufferBindingType::Storage { read_only: true },
            ),
        ],
    })
}

/// Group 3 for markers and dots: one row table.
fn ink_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ink.rows.layout"),
        entries: &[buffer_entry(
            0,
            ShaderStages::VERTEX_FRAGMENT,
            BufferBindingType::Storage { read_only: true },
        )],
    })
}

/// Group 3 for lines: rows, source edge ids, selected edge.
fn segment_rows_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("segment.rows.layout"),
        entries: &[
            storage_entry(0),
            storage_entry(1),
            buffer_entry(2, ShaderStages::VERTEX, BufferBindingType::Uniform),
        ],
    })
}

/// Group 1 for points: records, positions, colors, normals.
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

/// Group 1 for the point resolve: depth and color textures.
fn resolve_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.resolve.layout"),
        entries: &[
            texture_entry(0, ShaderStages::FRAGMENT, TextureSampleType::Depth, false),
            texture_entry(
                1,
                ShaderStages::FRAGMENT,
                TextureSampleType::Float { filterable: false },
                false,
            ),
        ],
    })
}

/// The bind group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,          // group 0: camera matrix
    pub line: wgpu::BindGroupLayout,         // group 1: pen and view settings, clipping planes
    pub instance: wgpu::BindGroupLayout,     // group 2: object rows
    pub ink_instance: wgpu::BindGroupLayout, // group 2 for ink, with depth textures
    pub ink_rows: wgpu::BindGroupLayout,     // group 3 for markers and dots
    pub segment_rows: wgpu::BindGroupLayout, // group 3 for lines
    pub points: wgpu::BindGroupLayout,       // group 1 for points
    pub resolve: wgpu::BindGroupLayout,      // group 1 for the point resolve
}

impl Layouts {
    /// Build every layout once.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(
                device,
                "mvp.layout",
                ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
            ),
            line: line_layout(device),
            instance: instance_layout(device),
            ink_instance: ink_instance_layout(device),
            ink_rows: ink_rows_layout(device),
            segment_rows: segment_rows_layout(device),
            points: points_layout(device),
            resolve: resolve_layout(device),
        }
    }
}
