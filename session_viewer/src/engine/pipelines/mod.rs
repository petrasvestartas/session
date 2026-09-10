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

impl Target {
    /// The id pass: (object row + 1, sub-object id + 1) per pixel, never multisampled.
    pub const ID: Target = Target {
        format: wgpu::TextureFormat::Rg32Uint,
        samples: 1,
    };
}

/// How a pipeline treats depth. Every compare is reverse-Z: nearer is GREATER.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    /// Write, strict `Greater`: solids and depth-only prepasses.
    Opaque,
    /// Source point queries write depth and accept exact ties with resident source points.
    OpaqueEqual,
    /// Test only, strict `Greater`: sheet fills and the grid.
    ReadOnly,
    /// Test only, `GreaterEqual`: blended ink that must tie with its prepass and with faces.
    ReadOnlyEqual,
    /// No test, no write: the background.
    Always,
    /// No depth attachment at all: a full-screen pass over a texture.
    Detached,
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

/// What a pipeline writes to the colour target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorWrite {
    /// Overwrite: solids, ids, the backdrop.
    Opaque,
    /// Alpha-blend: ink with an AA feather.
    Blended,
    /// Keep the larger value: coverage masks, where a stroke's feather must not dent a face.
    Max,
    /// No colour at all: a rasterization run for its fragment side effects.
    Nothing,
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
    pub scene_samples: Option<u32>,
    pub physical: bool,
    /// Two `R8Unorm` coverage targets (solid, selected) written in one pass with MAX
    /// blending, so writing 0 is the same as discarding.
    pub masks: bool,
}

impl<'a> PipelineDesc<'a> {
    /// A base over `shader` with `vs_main`, opaque colour and opaque depth; the variants
    /// change the label, the fragment entry, the colour mode and the depth mode.
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
            masks: false,
        }
    }

    /// The variant `label`, drawn with fragment entry `fs`.
    pub fn with(&self, label: &'a str, fs: &'a str) -> Self {
        let mut d = self.clone();
        d.label = label;
        d.fs = fs;
        d
    }

    /// The same desc with another vertex entry.
    pub fn vertex(mut self, vs: &'a str) -> Self {
        self.vs = vs;
        self
    }

    /// The same desc with another colour mode.
    pub fn color(mut self, color: ColorWrite) -> Self {
        self.color = color;
        self
    }

    /// Specialize scene sampling independently of the output target (picking stays 1x).
    pub fn scene_samples(mut self, samples: u32) -> Self {
        self.scene_samples = Some(samples);
        self
    }

    /// Add immutable physical-gradient output beside the primary color target.
    pub fn physical(mut self) -> Self {
        self.physical = true;
        self
    }

    /// The same desc writing the solid and selected coverage masks together.
    pub fn masks(mut self) -> Self {
        self.masks = true;
        self
    }

    /// The same desc with another depth mode.
    pub fn depth(mut self, depth: DepthMode) -> Self {
        self.depth = depth;
        self
    }
}

/// The scene contract (`scene.wgsl`: groups 0-2, `Instance`, `LineUniform`, the flags,
/// `place`) that every lane on it is compiled with.
pub const SCENE: &str = include_str!("../../shaders/scene.wgsl");

/// A lane on the scene contract: faces, lettering, the grid.
pub fn scene_module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    module(device, label, &format!("{source}\n{SCENE}"))
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
    wgpu::VertexBufferLayout {
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &INSTANCE_ID_ATTRIBS,
    }
}

/// A unit template's positions at `@location(0)` (the marker quad).
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &TEMPLATE_ATTRIBS,
    }
}

/// The WGSL every module ends with: the normal transform and the physical output struct.
/// The layout test compiles exactly this text, so what naga validates is what the GPU runs.
pub fn shared(source: &str) -> String {
    format!(
        "{source}\n{}\n{}",
        include_str!("../../shaders/normals.wgsl"),
        include_str!("../../shaders/physical.wgsl")
    )
}

/// A shader with its own bindings (the backdrop, the point splats, the silhouettes).
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(shared(source).into()),
    })
}

/// The shared visibility rule and the projected-triangle records every ink fragment decides
/// with, so all the ink lanes hide against one another's surfaces.
pub const INK: &str = concat!(
    include_str!("../../shaders/ink_visibility.wgsl"),
    "\n",
    include_str!("../../shaders/projected_triangle.wgsl")
);

/// An ink lane: the scene contract plus the ink rule.
pub fn ink_module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    scene_module(device, label, &format!("{source}\n{INK}"))
}

/// The pipeline layout for `groups`, in slot order.
pub fn pipeline_layout(
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

/// One render pipeline from its description. Everything not in the desc is the same for all
/// of them: `Depth32Float`, no cull, no hardware bias, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let (blend, write_mask) = desc.color.state();
    let mut targets = vec![Some(wgpu::ColorTargetState {
        format: target.format,
        blend,
        write_mask,
    })];
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
    if desc.physical {
        targets.push(Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rgba16Float,
            blend: None,
            write_mask: if desc.depth == DepthMode::ReadOnlyEqual {
                wgpu::ColorWrites::empty()
            } else {
                wgpu::ColorWrites::ALL
            },
        }));
    }
    let mut constants = Vec::new();
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
