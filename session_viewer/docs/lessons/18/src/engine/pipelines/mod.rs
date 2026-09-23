pub mod layouts;

pub use layouts::Layouts;

use session_rust::RenderVertex;

/// Where a pipeline draws: color format and MSAA sample count.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

impl Target {
    /// The id pass target: two u32 per pixel, no MSAA.
    pub const ID: Target = Target {
        format: wgpu::TextureFormat::Rg32Uint,
        samples: 1,
    };
}

/// How a pipeline uses depth; reverse-Z, so nearer is greater.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    Opaque, // write, nearer wins
    OpaqueEqual, // write, nearer or equal wins
    ReadOnly, // test only, nearer wins
    ReadOnlyEqual, // test only, nearer or equal wins
    Always, // no test, no write
    Detached, // no depth attachment at all
}

impl DepthMode {
    /// The (write, compare) pair wgpu wants.
    fn state(self) -> (bool, wgpu::CompareFunction) {
        match self {
            DepthMode::Opaque => (true, wgpu::CompareFunction::Greater),
            DepthMode::OpaqueEqual => (true, wgpu::CompareFunction::GreaterEqual),
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnlyEqual => (false, wgpu::CompareFunction::GreaterEqual),
            DepthMode::Always | DepthMode::Detached => (false, wgpu::CompareFunction::Always),
        }
    }
}

/// How a pipeline writes color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorWrite {
    Opaque, // overwrite
    Blended,
    Max, // keep the larger value; for masks
    Nothing, // write nothing; the fragment shader has side effects
}

impl ColorWrite {
    /// The (blend, write mask) pair wgpu wants.
    fn state(self) -> (Option<wgpu::BlendState>, wgpu::ColorWrites) {
        match self {
            ColorWrite::Opaque => (None, wgpu::ColorWrites::ALL),
            ColorWrite::Blended => (
                Some(wgpu::BlendState::ALPHA_BLENDING),
                wgpu::ColorWrites::ALL,
            ),
            ColorWrite::Nothing => (None, wgpu::ColorWrites::empty()),
            ColorWrite::Max => {
                let max = wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Max,
                };
                (
                    Some(wgpu::BlendState {
                        color: max,
                        alpha: max,
                    }),
                    wgpu::ColorWrites::ALL,
                )
            }
        }
    }
}

/// Everything `build` needs for one render pipeline.
#[derive(Clone)]
pub struct PipelineDesc<'a> {
    pub label: &'a str, // name shown in GPU errors
    pub shader: &'a wgpu::ShaderModule,
    pub vs: &'a str, // vertex entry point
    pub fs: &'a str, // fragment entry point
    pub groups: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub topology: wgpu::PrimitiveTopology, // triangles or lines
    pub color: ColorWrite,
    pub depth: DepthMode,
    pub scene_samples: Option<u32>, // sets SCENE_MSAA in the shader
    pub physical: bool, // also writes the depth slope target
    // --8<-- [start:step-17a]
    pub masks: bool, // writes both outline masks with MAX blending
    // --8<-- [end:step-17a]
}

impl<'a> PipelineDesc<'a> {
    /// A base: `vs_main`, `fs_main`, opaque color, opaque depth.
    pub fn new(
        shader: &'a wgpu::ShaderModule,
        groups: &'a [&'a wgpu::BindGroupLayout],
        vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
        topology: wgpu::PrimitiveTopology,
    ) -> Self {
        Self {
            label: "",
            shader,
            vs: "vs_main",
            fs: "fs_main",
            groups,
            vertex_buffers,
            topology,
            color: ColorWrite::Opaque,
            depth: DepthMode::Opaque,
            scene_samples: None,
            physical: false,
            // --8<-- [start:step-17b]
            masks: false,
            // --8<-- [end:step-17b]
        }
    }

    /// A copy with another label and fragment entry point.
    pub fn with(&self, label: &'a str, fs: &'a str) -> Self {
        let mut d = self.clone();
        d.label = label;
        d.fs = fs;
        d
    }

    /// A copy with another vertex entry point.
    pub fn vertex(mut self, vs: &'a str) -> Self {
        self.vs = vs;
        self
    }

    /// A copy with another color mode.
    pub fn color(mut self, color: ColorWrite) -> Self {
        self.color = color;
        self
    }

    /// A copy that reads the scene depth at `samples`.
    pub fn scene_samples(mut self, samples: u32) -> Self {
        self.scene_samples = Some(samples);
        self
    }

    /// A copy that also writes the depth slope target.
    pub fn physical(mut self) -> Self {
        self.physical = true;
        self
    }

    // --8<-- [start:step-17c]
    /// A copy that writes both outline masks.
    pub fn masks(mut self) -> Self {
        self.masks = true;
        self
    }

    /// A copy with another depth mode.
// --8<-- [end:step-17c]
    pub fn depth(mut self, depth: DepthMode) -> Self {
        self.depth = depth;
        self
    }
}

/// Shared WGSL: groups 0-2, Instance, LineUniform, flags, `place`.
pub const SCENE: &str = include_str!("../../shaders/scene.wgsl");

/// Compile a shader with the shared scene code appended.
pub fn scene_module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    module(device, label, &format!("{source}\n{SCENE}"))
}

/// One u32 at location 3: the object row.
const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];

/// One vec3 at location 0: a template position.
const TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];

/// Vertex slot 0: the kernel's RenderVertex (position, normal, color).
pub fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    RenderVertex::layout()
}

/// Vertex slot 1: one object row per vertex.
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &INSTANCE_ID_ATTRIBS,
    }
}

/// Vertex slot 0 for the marker quad: positions only.
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &TEMPLATE_ATTRIBS,
    }
}

// --8<-- [start:step-17d]
/// Append the WGSL every shader ends with: normals and physical output.
pub fn shared(source: &str) -> String {
    format!(
        "{source}\n{}\n{}",
        include_str!("../../shaders/normals.wgsl"),
        include_str!("../../shaders/physical.wgsl")
    )
}

/// Compile a shader that declares its own bindings.
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(shared(source).into()),
    })
}

/// Shared WGSL for ink: the visibility test and projected triangles.
pub const INK: &str = concat!(
    include_str!("../../shaders/ink_visibility.wgsl"),
    "\n",
    include_str!("../../shaders/projected_triangle.wgsl")
);

/// Compile an ink shader: scene code plus the ink code.
pub fn ink_module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    scene_module(device, label, &format!("{source}\n{INK}"))
}

/// A pipeline layout over `groups`, in slot order.
pub fn pipeline_layout(
// --8<-- [end:step-17d]
    device: &wgpu::Device,
    label: &str,
    groups: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    let mut slots: Vec<Option<&wgpu::BindGroupLayout>> = Vec::with_capacity(groups.len());

    for g in groups {
        slots.push(Some(*g));
    }

    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &slots,
        immediate_size: 0,
    })
}

/// Build one render pipeline; Depth32Float, no culling, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let (blend, write_mask) = desc.color.state();
    let mut targets = vec![Some(wgpu::ColorTargetState {
        format: target.format,
        blend,
        write_mask,
    })];

    // --8<-- [start:step-17e]
    // two mask targets with MAX blending
    if desc.masks {
        let max = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Max,
        };
        let coverage = Some(wgpu::ColorTargetState {
            format: target.format,
            blend: Some(wgpu::BlendState {
                color: max,
                alpha: max,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        });
        targets = vec![coverage.clone(), coverage];
    }

    // the depth slope target
    if desc.physical {
        targets.push(Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rgba16Float,
            blend: None,
            // a ReadOnlyEqual pass already wrote it: declare, do not write
            // --8<-- [end:step-17e]
            write_mask: if desc.depth == DepthMode::ReadOnlyEqual {
                wgpu::ColorWrites::empty()
            } else {
                wgpu::ColorWrites::ALL
            },
        }));
    }

    let mut constants = Vec::new();

    // shader constant: which depth texture is live
    if let Some(samples) = desc.scene_samples {
        constants.push(("SCENE_MSAA", f64::from(samples > 1)));
    }

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &targets,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
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
        depth_stencil: (desc.depth != DepthMode::Detached).then_some(wgpu::DepthStencilState {
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
