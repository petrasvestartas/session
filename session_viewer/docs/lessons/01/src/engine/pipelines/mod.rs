pub mod layouts;

pub use layouts::Layouts;

use crate::engine::gpu::buffers::GpuCtx;
use std::cell::{Cell, LazyCell, RefCell};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

thread_local! {
    /// Render and compute pipelines compiled on this thread so far.
    static PIPELINES: Cell<u32> = const { Cell::new(0) };

    /// Shader modules compiled on this thread so far.
    static SHADERS: Cell<u32> = const { Cell::new(0) };
}

/// Count one compiled pipeline.
pub fn count_pipeline() {
    PIPELINES.set(PIPELINES.get() + 1);
}

/// Count one compiled shader module.
pub fn count_shader() {
    SHADERS.set(SHADERS.get() + 1);
}

/// (pipelines, shader modules) compiled on this thread so far.
pub fn created() -> (u32, u32) {
    (PIPELINES.get(), SHADERS.get())
}

/// A GPU object made on its first use; clones share it, and equal means the same one.
pub struct Lazy<T>(Rc<LazyCell<T, Box<dyn FnOnce() -> T>>>);

impl<T> Lazy<T> {
    /// Made by `make` when first used.
    pub fn new(make: impl FnOnce() -> T + 'static) -> Self {
        Self(Rc::new(LazyCell::new(Box::new(make))))
    }
}

impl<T> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> std::ops::Deref for Lazy<T> {
    type Target = T;

    /// The object, made now if this is its first use.
    fn deref(&self) -> &T {
        LazyCell::force(&self.0)
    }
}

impl<T> PartialEq for Lazy<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Lazy<T> {}

impl<T> Hash for Lazy<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

/// A render pipeline, compiled when a pass first sets it.
pub type Pipeline = Lazy<wgpu::RenderPipeline>;

/// A shader module, compiled when a pipeline first needs it.
pub type Shader = Lazy<wgpu::ShaderModule>;

/// Shaders, layouts and pipelines by description: asking again, or a second lane of the same
/// kind asking, returns the first one, so an MSAA flip back and forth compiles nothing twice.
#[derive(Default)]
pub struct Cache {
    shaders: RefCell<HashMap<ShaderKey, Shader>>, // by label and source digest
    layouts: RefCell<HashMap<LayoutKey, wgpu::BindGroupLayout>>, // by label and entries
    pipelines: RefCell<HashMap<PipelineKey, Pipeline>>, // by everything they compile from
    pub clipping: Cell<bool>, // a plane cuts: ink pipelines built now keep their clip tests
}

impl Cache {
    /// Compile every pipeline asked for so far; returns how many there are.
    pub fn compile_all(&self) -> usize {
        let pipelines = self.pipelines.borrow();

        for pipeline in pipelines.values() {
            let _: &wgpu::RenderPipeline = pipeline;
        }

        pipelines.len()
    }
}

/// A shader's label, source length and source hash: the text itself is not kept.
type ShaderKey = (String, usize, u64);

/// A bind group layout's label and entries.
type LayoutKey = (String, Vec<wgpu::BindGroupLayoutEntry>);

/// Everything a render pipeline compiles from, owned, so the compile can wait for its first use.
#[derive(Clone, PartialEq, Eq, Hash)]
struct PipelineKey {
    label: String,                      // name shown in GPU errors
    shader: Shader,                     // vertex and fragment code
    vs: String,                         // vertex entry point
    fs: String,                         // fragment entry point
    groups: Vec<wgpu::BindGroupLayout>, // bind group layouts, in slot order
    buffers: Vec<(u64, wgpu::VertexStepMode, Vec<wgpu::VertexAttribute>)>, // vertex buffer layouts
    topology: wgpu::PrimitiveTopology,  // triangles or lines
    color: ColorWrite,                  // how color is written
    depth: DepthMode,                   // how depth is used
    scene_samples: Option<u32>,         // sets SCENE_MSAA in the shader
    clipping: Option<bool>,             // sets CLIPPING in an ink shader
    physical: bool,                     // also writes the triangle id target
    masks: bool,                        // writes both outline masks
    target: Target,                     // color format and samples
}

/// Where a pipeline draws: color format and MSAA sample count.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Target {
    pub format: wgpu::TextureFormat, // color format
    pub samples: u32,                // MSAA samples
}

impl Target {
    /// The id pass target: two u32 per pixel, no MSAA.
    pub const ID: Target = Target {
        format: wgpu::TextureFormat::Rg32Uint,
        samples: 1,
    };
}

/// How a pipeline uses depth; reverse-Z, so nearer is greater.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DepthMode {
    Opaque,        // write, nearer wins
    OpaqueEqual,   // write, nearer or equal wins
    ReadOnly,      // test only, nearer wins
    ReadOnlyEqual, // test only, nearer or equal wins
    Always,        // no test, no write
    Detached,      // no depth attachment at all
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ColorWrite {
    Opaque,  // overwrite
    Blended, // alpha blend
    Max,     // keep the larger value; for masks
    Add,     // add to the value; for counts
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
            ColorWrite::Add => {
                let add = wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                };
                (
                    Some(wgpu::BlendState {
                        color: add,
                        alpha: add,
                    }),
                    wgpu::ColorWrites::ALL,
                )
            }
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
    pub label: &'a str,                                     // name shown in GPU errors
    pub shader: &'a Shader,                                 // vertex and fragment code
    pub vs: &'a str,                                        // vertex entry point
    pub fs: &'a str,                                        // fragment entry point
    pub groups: &'a [&'a wgpu::BindGroupLayout],            // bind group layouts, in slot order
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>], // vertex buffer layouts
    pub topology: wgpu::PrimitiveTopology,                  // triangles or lines
    pub color: ColorWrite,                                  // how color is written
    pub depth: DepthMode,                                   // how depth is used
    pub scene_samples: Option<u32>,                         // sets SCENE_MSAA in the shader
    pub physical: bool,                                     // also writes the triangle id target
    pub masks: bool, // writes both outline masks with MAX blending
}

impl<'a> PipelineDesc<'a> {
    /// A base: `vs_main`, `fs_main`, opaque color, opaque depth.
    pub fn new(
        shader: &'a Shader,
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

    /// A copy that also writes the triangle id target.
    pub fn physical(mut self) -> Self {
        self.physical = true;
        self
    }

    /// A copy that writes both outline masks.
    pub fn masks(mut self) -> Self {
        self.masks = true;
        self
    }

    /// A copy with another depth mode.
    pub fn depth(mut self, depth: DepthMode) -> Self {
        self.depth = depth;
        self
    }
}

/// Shared WGSL: groups 0-2, Instance, LineUniform, flags, `place`.
pub const SCENE: &str = shader!("scene.wgsl");

/// WGSL every scene shader ends with. A shared snippet is one file and one line here.
pub const PRELUDE: &[&str] = &[
    SCENE, // register:scene
];

/// A scene shader's full text: its own code, then the prelude.
pub fn scene_source(source: &str) -> String {
    PRELUDE
        .iter()
        .fold(source.to_owned(), |text, part| format!("{text}\n{part}"))
}

/// A shader with the shared scene and clipping code appended.
pub fn scene_module(ctx: &GpuCtx, label: &str, source: &str) -> Shader {
    module(ctx, label, &scene_source(source))
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

/// Append the WGSL every shader ends with: normals and physical output.
pub fn shared(source: &str) -> String {
    format!(
        "{source}\n{}\n{}",
        shader!("normals.wgsl"),
        shader!("physical.wgsl")
    )
}

/// A shader that declares its own bindings.
pub fn module(ctx: &GpuCtx, label: &str, source: &str) -> Shader {
    wgsl(ctx, label, shared(source))
}

/// The shader for exactly `source`: one per label and source, compiled on first use.
pub fn wgsl(ctx: &GpuCtx, label: &str, source: String) -> Shader {
    let mut hasher = std::hash::DefaultHasher::new();
    source.hash(&mut hasher);
    let key = (label.to_string(), source.len(), hasher.finish());
    let mut shaders = ctx.cache.shaders.borrow_mut();

    if let Some(shader) = shaders.get(&key) {
        return shader.clone();
    }

    let device = ctx.device.clone();
    let (name, text) = (label.to_string(), source);
    let shader = Shader::new(move || {
        count_shader();
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&name),
            source: wgpu::ShaderSource::Wgsl(text.into()),
        })
    });
    shaders.insert(key, shader.clone());
    shader
}

/// A bind group layout: one per label and entries.
pub fn layout(
    ctx: &GpuCtx,
    label: &str,
    entries: &[wgpu::BindGroupLayoutEntry],
) -> wgpu::BindGroupLayout {
    ctx.cache
        .layouts
        .borrow_mut()
        .entry((label.to_string(), entries.to_vec()))
        .or_insert_with(|| {
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(label),
                    entries,
                })
        })
        .clone()
}

/// A pipeline layout over `groups`, in slot order.
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

/// One render pipeline; Depth32Float, no culling, fill mode. It compiles on first use, and a
/// second request for the same description gets the same pipeline.
pub fn build(ctx: &GpuCtx, target: Target, desc: &PipelineDesc) -> Pipeline {
    let key = PipelineKey {
        label: desc.label.to_string(),
        shader: desc.shader.clone(),
        vs: desc.vs.to_string(),
        fs: desc.fs.to_string(),
        groups: desc.groups.iter().map(|group| (*group).clone()).collect(),
        buffers: desc
            .vertex_buffers
            .iter()
            .map(|b| (b.array_stride, b.step_mode, b.attributes.to_vec()))
            .collect(),
        topology: desc.topology,
        color: desc.color,
        depth: desc.depth,
        scene_samples: desc.scene_samples,
        // ink pipelines are rebuilt on every retarget, so they alone may drop their clip tests
        clipping: desc.scene_samples.map(|_| ctx.cache.clipping.get()),
        physical: desc.physical,
        masks: desc.masks,
        target,
    };
    let mut pipelines = ctx.cache.pipelines.borrow_mut();

    if let Some(pipeline) = pipelines.get(&key) {
        return pipeline.clone();
    }

    let device = ctx.device.clone();
    let desc = key.clone();
    let pipeline = Pipeline::new(move || compile(&device, &desc));
    pipelines.insert(key, pipeline.clone());
    pipeline
}

/// Compile the pipeline `desc` describes.
fn compile(device: &wgpu::Device, desc: &PipelineKey) -> wgpu::RenderPipeline {
    let groups: Vec<&wgpu::BindGroupLayout> = desc.groups.iter().collect();
    let layout = pipeline_layout(device, &desc.label, &groups);
    let buffers: Vec<wgpu::VertexBufferLayout> = desc
        .buffers
        .iter()
        .map(|(stride, step, attributes)| wgpu::VertexBufferLayout {
            array_stride: *stride,
            step_mode: *step,
            attributes,
        })
        .collect();
    let target = desc.target;
    let (depth_write, depth_compare) = desc.depth.state();
    let (blend, write_mask) = desc.color.state();
    let mut targets = vec![Some(wgpu::ColorTargetState {
        format: target.format,
        blend,
        write_mask,
    })];

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

    // the triangle id target
    if desc.physical {
        targets.push(Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rg16Uint,
            blend: None,
            // a ReadOnlyEqual pass already wrote it: declare, do not write
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

    // shader constant: no plane cuts, so the clip tests compile away
    if let Some(clipping) = desc.clipping {
        constants.push(("CLIPPING", f64::from(clipping)));
    }

    count_pipeline();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &desc.shader,
            entry_point: Some(&desc.vs),
            buffers: &buffers,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &desc.shader,
            entry_point: Some(&desc.fs),
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    /// Clones share one object, made once, on its first use.
    fn lazy_objects_are_made_once_on_first_use() {
        let made = Rc::new(Cell::new(0));
        let counter = made.clone();
        let lazy = Lazy::new(move || {
            counter.set(counter.get() + 1);
            7
        });
        let copy = lazy.clone();
        assert_eq!(made.get(), 0, "nothing is made before its first use");
        assert_eq!(*copy, 7);
        assert_eq!(*lazy, 7);
        assert_eq!(made.get(), 1, "clones share the one made");
        assert!(lazy == copy);
        assert!(lazy != Lazy::new(|| 7), "equal means the same object");
    }
}
