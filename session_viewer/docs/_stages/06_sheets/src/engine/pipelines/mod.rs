//! Pipelines are data. `PipelineDesc` names what differs between the viewer's render
//! pipelines and `build` is the only place wgpu is asked for one. Every lane owns its own
//! descs and rebuilds them through `retarget` when the MSAA sample count flips.

pub mod layouts;

pub use layouts::Layouts;

use session_rust::RenderVertex;

/// Where a pipeline draws: the colour format and the sample count of the pass.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

/// How a pipeline treats depth. Every compare is reverse-Z: nearer is GREATER.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    /// Write, strict `Greater`: solids and depth-only prepasses.
    Opaque,
    /// Test only, strict `Greater`: sheet fills and the grid.
    ReadOnly,
    /// Test only, `GreaterEqual`: blended ink that must tie with its prepass and with faces.
    ReadOnlyEqual,
    /// No test, no write: the background.
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

/// What a pipeline writes to the colour target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorWrite {
    /// Overwrite: solids, ids, the backdrop.
    Opaque,
    /// Alpha-blend: ink with an AA feather.
    Blended,
    /// Nothing: a depth-only prepass (the pass still has a colour attachment to declare).
    Masked,
}

impl ColorWrite {
    /// The (blend, write mask) pair wgpu wants.
    fn state(self) -> (Option<wgpu::BlendState>, wgpu::ColorWrites) {
        match self {
            ColorWrite::Opaque => (None, wgpu::ColorWrites::ALL),
            ColorWrite::Blended => (Some(wgpu::BlendState::ALPHA_BLENDING), wgpu::ColorWrites::ALL),
            ColorWrite::Masked => (None, wgpu::ColorWrites::empty()),
        }
    }
}

/// Everything `build` needs for one render pipeline. A lane makes one base per shader and
/// derives its variants with `with`, `color` and `depth`.
#[derive(Clone)]
pub struct PipelineDesc<'a> {
    pub label: &'a str,
    pub shader: &'a wgpu::ShaderModule,
    pub vs: &'a str,
    pub fs: &'a str,
    pub groups: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub topology: wgpu::PrimitiveTopology,
    pub color: ColorWrite,
    pub depth: DepthMode,
}

impl<'a> PipelineDesc<'a> {
    /// A base over `shader` with `vs_main`, opaque colour and opaque depth; the variants
    /// change the label, the fragment entry, the colour mode and the depth mode.
    pub fn new(shader: &'a wgpu::ShaderModule, groups: &'a [&'a wgpu::BindGroupLayout], vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>], topology: wgpu::PrimitiveTopology) -> Self {
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque }
    }

    /// The variant `label`, drawn with fragment entry `fs`.
    pub fn with(&self, label: &'a str, fs: &'a str) -> Self {
        let mut d = self.clone();
        d.label = label;
        d.fs = fs;
        d
    }

    /// The same desc with another colour mode.
    pub fn color(mut self, color: ColorWrite) -> Self {
        self.color = color;
        self
    }

    /// The same desc with another depth mode.
    pub fn depth(mut self, depth: DepthMode) -> Self {
        self.depth = depth;
        self
    }
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

/// The mesh vertex slot: the kernel's interleaved `RenderVertex` (pos, normal, colour).
pub fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    RenderVertex::layout()
}

/// One `u32` object row per vertex at `@location(3)`.
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout { array_stride: 4, step_mode: wgpu::VertexStepMode::Vertex, attributes: &INSTANCE_ID_ATTRIBS }
}

/// A unit template's positions at `@location(0)` (the cylinder, the marker quad).
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout { array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex, attributes: &TEMPLATE_ATTRIBS }
}

/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some(label), source: wgpu::ShaderSource::Wgsl(source.into()) })
}

/// The pipeline layout for `groups`, in slot order.
fn pipeline_layout(device: &wgpu::Device, label: &str, groups: &[&wgpu::BindGroupLayout]) -> wgpu::PipelineLayout {
    let mut slots: Vec<Option<&wgpu::BindGroupLayout>> = Vec::with_capacity(groups.len());
    for g in groups {
        slots.push(Some(*g));
    }
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some(label), bind_group_layouts: &slots, immediate_size: 0 })
}

/// One render pipeline from its description. Everything not in the desc is the same for all
/// of them: one colour target, `Depth32Float`, no cull, no hardware bias, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let (blend, write_mask) = desc.color.state();

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &[Some(wgpu::ColorTargetState { format: target.format, blend, write_mask })],
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(depth_compare),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: target.samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}
