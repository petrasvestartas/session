//! Pipelines are data. `PipelineDesc` names the ten things that differ between the viewer's
//! render pipelines; `build` turns one into a `wgpu::RenderPipeline` and is the only place
//! wgpu is asked for one. Shader modules are made by the caller, once per source.

use std::sync::OnceLock;

/// Where a pipeline draws: the surface format and the MSAA sample count of the pass.
///
/// MSAA cannot be mixed WITHIN a frame - sample count is a property of the render PASS - so
/// the viewer picks one per SCENE (`Gpu::msaa_now`) and rebuilds every pipeline on a flip.
#[derive(Clone, Copy)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

/// How a pipeline treats depth. Every compare is reverse-Z: nearer is GREATER.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    /// Write, strict `Greater`: solids, and the depth-only prepasses.
    Opaque,
    /// Test only, strict `Greater`: sheet fills and the grid.
    ReadOnly,
    /// Test only, `GreaterEqual`: blended ink that must tie with its prepass and with faces.
    ReadOnlyEqual,
    /// No test, no write: the background, and `VIEWER_NO_DEPTH`.
    Always,
}

impl DepthMode {
    /// The (write, compare) pair wgpu wants.
    fn state(self) -> (bool, wgpu::CompareFunction) {
        match self {
            DepthMode::Opaque => (true, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnlyEqual => (false, wgpu::CompareFunction::GreaterEqual),
            DepthMode::Always => (false, wgpu::CompareFunction::Always),
        }
    }
}

/// Everything `build` needs to make one render pipeline. A pipeline is data, not a function.
pub struct PipelineDesc<'a> {
    pub label: &'a str,
    pub shader: &'a wgpu::ShaderModule,
    pub vs: &'a str,
    pub fs: &'a str,
    pub groups: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub topology: wgpu::PrimitiveTopology,
    pub blend: Option<wgpu::BlendState>,
    /// False = depth-only prepass: every colour channel masked, only depth lands.
    pub write_color: bool,
    pub depth: DepthMode,
}

/// Everything `build_compute` needs to make one compute pipeline.
pub struct ComputeDesc<'a> {
    pub label: &'a str,
    pub shader: &'a wgpu::ShaderModule,
    pub entry: &'a str,
    pub groups: &'a [&'a wgpu::BindGroupLayout],
}

const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];

const TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];

/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &INSTANCE_ID_ATTRIBS,
    }
}

/// Vertex-buffer layout for the unit-cylinder / quad template positions (`@location(0)`, one `vec3<f32>`).
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &TEMPLATE_ATTRIBS,
    }
}

/// `mode`, or `Always` when `VIEWER_NO_DEPTH` is set. Read once; env vars never exist on wasm.
pub fn depth_or_always(mode: DepthMode) -> DepthMode {
    static NO_DEPTH: OnceLock<bool> = OnceLock::new();
    if *NO_DEPTH.get_or_init(|| std::env::var("VIEWER_NO_DEPTH").is_ok()) { DepthMode::Always } else { mode }
}

/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

/// The pipeline layout for `groups`, in slot order.
fn pipeline_layout(device: &wgpu::Device, label: &str, groups: &[&wgpu::BindGroupLayout]) -> wgpu::PipelineLayout {
    let groups: Vec<Option<&wgpu::BindGroupLayout>> = groups.iter().map(|g| Some(*g)).collect();
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &groups,
        immediate_size: 0,
    })
}

/// One render pipeline from its description. Everything not in the desc is the same for all
/// of them: one colour target, `Depth32Float`, no cull, no hardware bias, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let write_mask = if desc.write_color { wgpu::ColorWrites::ALL } else { wgpu::ColorWrites::empty() };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: Default::default(),
        },
        // The pass HAS a colour attachment, so a depth-only pipeline still declares one and
        // masks every channel - Dawn rejects an empty target list against a colour pass.
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: desc.blend, write_mask })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: desc.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        // No hardware bias anywhere: the units of `constant` on a float depth format are
        // implementation-defined, so faces recede in triangle.wgsl instead (FACE_PUSH).
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(depth_compare),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: target.samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}

/// One compute pipeline from its description.
pub fn build_compute(device: &wgpu::Device, desc: &ComputeDesc) -> wgpu::ComputePipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        module: desc.shader,
        entry_point: Some(desc.entry),
        compilation_options: Default::default(),
        cache: None,
    })
}
