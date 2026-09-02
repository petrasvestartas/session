//! `Pipelines` — every render and compute pipeline the viewer draws with, as data: one
//! `PipelineDesc` literal each, built by `build::build` from shader modules made once per
//! source. Rebuilt whole when the MSAA sample count flips (the count belongs to the pass).

pub mod build;
pub mod layouts;

pub use build::Target;
pub use layouts::Layouts;

use build::{build, build_compute, depth_or_always, instance_id_layout, module, template_layout};
use build::{ComputeDesc, DepthMode, PipelineDesc};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::{LineList, TriangleList, TriangleStrip};

/// Smooth AA feather + hairline fade on every blended lane.
const ALPHA: Option<wgpu::BlendState> = Some(wgpu::BlendState::ALPHA_BLENDING);

/// Every pipeline the viewer draws with, built once at startup and again on an MSAA flip.
pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
    /// Same program, depth WRITE off: a sheet's fills are exactly coplanar, so they composite
    /// in draw order (a painter's document) instead of flickering over one shared depth value.
    pub triangle_sheet: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,
    pub sphere: wgpu::RenderPipeline,
    pub sphere_depth: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
    pub ribbon_depth: wgpu::RenderPipeline,
    /// The flat lane's shader over the SOLID table; `GreaterEqual` is load-bearing (a mesh
    /// edge sits EXACTLY on its faces' depth, and strict `Greater` shreds it).
    pub ribbon_solid: wgpu::RenderPipeline,
    /// Depth-only prepasses for the solid ink: binary at half coverage, so the blended colour
    /// passes never write depth and their AA feather cannot depth-reject a later stroke.
    pub ribbon_solid_depth: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
    pub glyph_depth: wgpu::RenderPipeline,
    /// Fullscreen composite of the splat buffers.
    pub splat_resolve: wgpu::RenderPipeline,
    pub splat_depth: wgpu::ComputePipeline,
    pub splat_color: wgpu::ComputePipeline,
}

impl Pipelines {
    /// Build every pipeline for `target` from the shared layouts. One shader module per source.
    pub fn new(device: &wgpu::Device, target: Target, l: &Layouts) -> Self {
        let triangle = module(device, "triangle.shader", include_str!("../../shaders/triangle.wgsl"));
        let grid = module(device, "grid.shader", include_str!("../../shaders/grid.wgsl"));
        let background = module(device, "background.shader", include_str!("../../shaders/background.wgsl"));
        let cylinder = module(device, "cylinder.shader", include_str!("../../shaders/cylinder.wgsl"));
        let sphere = module(device, "sphere.shader", include_str!("../../shaders/sphere.wgsl"));
        let ribbon = module(device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl"));
        let glyph = module(device, "glyph.shader", include_str!("../../shaders/glyph.wgsl"));
        let resolve = module(device, "splat.resolve.shader", include_str!("../../shaders/splat_resolve.wgsl"));
        let splat = module(device, "splat.shader", include_str!("../../shaders/splat.wgsl"));

        // Group scheme: 0 = mvp, 1 = line/cloud uniform, 2 = instances, 3 = the family's rows.
        let solid = [&l.mvp, &l.line, &l.instance];
        let seg = [&l.mvp, &l.line, &l.instance, &l.segment];
        let gly = [&l.mvp, &l.line, &l.instance, &l.glyph];
        let splat_groups = [&l.splat_group0, &l.splat_group1];

        Self {
            triangle: build(device, target, &PipelineDesc {
                label: "triangle", shader: &triangle, vs: "vs_main", fs: "fs_main",
                groups: &solid, vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::Opaque,
            }),
            triangle_sheet: build(device, target, &PipelineDesc {
                label: "triangle.sheet", shader: &triangle, vs: "vs_main", fs: "fs_main",
                groups: &solid, vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnly,
            }),
            grid: build(device, target, &PipelineDesc {
                label: "grid", shader: &grid, vs: "vs_main", fs: "fs_main",
                groups: &[&l.mvp, &l.line], vertex_buffers: &[],
                topology: LineList, blend: None, write_color: true, depth: DepthMode::ReadOnly,
            }),
            background: build(device, target, &PipelineDesc {
                label: "background", shader: &background, vs: "vs_main", fs: "fs_main",
                groups: &[], vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Always,
            }),
            cylinder: build(device, target, &PipelineDesc {
                label: "cylinder", shader: &cylinder, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Opaque,
            }),
            sphere: build(device, target, &PipelineDesc {
                label: "sphere", shader: &sphere, vs: "vs_main", fs: "fs_main",
                groups: &gly, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true,
                depth: depth_or_always(DepthMode::ReadOnlyEqual), // VIEWER_NO_DEPTH
            }),
            sphere_depth: build(device, target, &PipelineDesc {
                label: "sphere.depth", shader: &sphere, vs: "vs_main", fs: "fs_depth",
                groups: &gly, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            ribbon: build(device, target, &PipelineDesc {
                label: "ribbon", shader: &ribbon, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleStrip, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnlyEqual,
            }),
            ribbon_depth: build(device, target, &PipelineDesc {
                label: "ribbon.depth", shader: &ribbon, vs: "vs_main", fs: "fs_depth",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleStrip, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            ribbon_solid: build(device, target, &PipelineDesc {
                label: "ribbon.solid", shader: &ribbon, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleStrip, blend: ALPHA, write_color: true,
                depth: depth_or_always(DepthMode::ReadOnlyEqual), // VIEWER_NO_DEPTH
            }),
            ribbon_solid_depth: build(device, target, &PipelineDesc {
                label: "ribbon.solid.depth", shader: &ribbon, vs: "vs_main", fs: "fs_depth",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleStrip, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            glyph: build(device, target, &PipelineDesc {
                label: "glyph", shader: &glyph, vs: "vs_main", fs: "fs_main",
                groups: &gly, vertex_buffers: &[],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnlyEqual,
            }),
            glyph_depth: build(device, target, &PipelineDesc {
                label: "glyph.depth", shader: &glyph, vs: "vs_main", fs: "fs_depth",
                groups: &gly, vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            splat_resolve: build(device, target, &PipelineDesc {
                label: "splat.resolve", shader: &resolve, vs: "vs_main", fs: "fs_main",
                groups: &[&l.line, &l.splat_resolve], vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Opaque,
            }),
            splat_depth: build_compute(device, &ComputeDesc {
                label: "splat.depth", shader: &splat, entry: "cs_depth", groups: &splat_groups,
            }),
            splat_color: build_compute(device, &ComputeDesc {
                label: "splat.color", shader: &splat, entry: "cs_color", groups: &splat_groups,
            }),
        }
    }
}
