use super::*;

/// The cap shader for a face pass at `samples`: its textures and their loaders first.
pub(super) fn cap_source(samples: u32) -> String {
    let (counts, primitives, count, primitive) = if samples > 1 {
        (
            "texture_multisampled_2d<f32>",
            "texture_multisampled_2d<u32>",
            "textureLoad(counts, at, i32(k))",
            "textureLoad(primitives, at, i32(k))",
        )
    } else {
        (
            "texture_2d<f32>",
            "texture_2d<u32>",
            "textureLoad(counts, at, 0)",
            "textureLoad(primitives, at, 0)",
        )
    };
    format!(
        "@group(3) @binding(0) var counts: {counts}; // crossings behind the plane per sample
@group(3) @binding(1) var primitives: {primitives}; // the face pass's triangle ids
const SAMPLES: u32 = {samples}u; // samples per pixel
// Crossings behind the plane at sample `k`, a selected solid's weighing SELECTED_CROSSING.
fn count_at(at: vec2<i32>, k: u32) -> f32 {{ return {count}.x; }}
// Triangle id at sample `k`.
fn primitive_at(at: vec2<i32>, k: u32) -> u32 {{ let h = {primitive}.xy; return h.x | (h.y << 16u); }}
{CAP}"
    )
}

/// One texture binding of the fragment stage.
fn cap_texture_entry(
    binding: u32,
    sample_type: wgpu::TextureSampleType,
    samples: u32,
) -> wgpu::BindGroupLayoutEntry {
    texture_entry(binding, ShaderStages::FRAGMENT, sample_type, samples > 1)
}

/// The pipelines of the pick through the caps.
pub(super) fn pick_pipelines(ctx: &GpuCtx, l: &Layouts) -> PickPipelines {
    let records = layout(
        ctx,
        "clip.pick",
        &[buffer_entry(
            5,
            ShaderStages::FRAGMENT,
            BufferBindingType::Storage { read_only: false },
        )],
    );
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance, &records];
    let buffers = [vertex_layout(), instance_id_layout()];
    let window = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel, never written
        samples: 1,
    };
    let solids = PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
        .vertex("vs_count")
        .color(ColorWrite::Nothing)
        .depth(DepthMode::Detached);
    let count = build(
        ctx,
        window,
        &solids.with("section pick count", "fs_pick_count"),
    );
    let owner = build(
        ctx,
        window,
        &solids.with("section pick owner", "fs_pick_owner"),
    );
    let shader = scene_module(ctx, "clip.cap", &cap_source(1));
    let ids = build(
        ctx,
        Target::ID,
        &PipelineDesc::new(&shader, &groups, &[], TriangleList)
            .vertex("vs_cap")
            .with("section cap ids", "fs_cap_id")
            .physical(),
    );
    PickPipelines {
        layout: records,
        count,
        owner,
        placed: None,
        ids,
    }
}

/// The pick's count and owner pipelines for instanced solids, `records` at group 3.
pub(super) fn pick_placed_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    records: &wgpu::BindGroupLayout,
) -> [Pipeline; 2] {
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance, records];
    let buffers = [vertex_layout(), placed_layout()];
    let window = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel, never written
        samples: 1,
    };
    let placed = PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
        .vertex("vs_count_placed")
        .color(ColorWrite::Nothing)
        .depth(DepthMode::Detached);
    [
        build(
            ctx,
            window,
            &placed.with("section pick count placed", "fs_pick_count"),
        ),
        build(
            ctx,
            window,
            &placed.with("section pick owner placed", "fs_pick_owner"),
        ),
    ]
}

/// The crossing count of instanced closed solids at the scene's sample count.
pub(super) fn count_placed_pipeline(ctx: &GpuCtx, l: &Layouts, target: Target) -> Pipeline {
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), placed_layout()];
    let counted = Target {
        format: COUNT_FORMAT,
        samples: target.samples,
    };
    build(
        ctx,
        counted,
        &PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
            .with("section count placed", "fs_count")
            .vertex("vs_count_placed")
            .color(ColorWrite::Add)
            .depth(DepthMode::Detached),
    )
}

/// The count, cap and mask pipelines at the scene's sample count.
pub(super) fn cap_pipelines(ctx: &GpuCtx, l: &Layouts, target: Target) -> CapPipelines {
    let counts = layout(
        ctx,
        "clip.counts",
        &[cap_texture_entry(
            0,
            TextureSampleType::Float { filterable: false },
            target.samples,
        )],
    );
    let primitives = layout(
        ctx,
        "clip.primitives",
        &[cap_texture_entry(
            1,
            TextureSampleType::Uint,
            target.samples,
        )],
    );
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let count = build(
        ctx,
        Target {
            format: COUNT_FORMAT,
            samples: target.samples,
        },
        &PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
            .with("section count", "fs_count")
            .vertex("vs_count")
            .color(ColorWrite::Add)
            .depth(DepthMode::Detached),
    );
    let shader = scene_module(ctx, "clip.cap", &cap_source(target.samples));
    let cap_groups = [&l.mvp, &l.line, &l.instance, &counts];
    let cap = build(
        ctx,
        target,
        &PipelineDesc::new(&shader, &cap_groups, &[], TriangleList)
            .with("section caps", "fs_cap")
            .vertex("vs_cap")
            .physical(),
    );
    let mask_groups = [&l.mvp, &l.line, &l.instance, &primitives];
    let base = PipelineDesc::new(&shader, &mask_groups, &[], TriangleList)
        .vertex("vs_cap")
        .depth(DepthMode::Always);
    let mask = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
        samples: target.samples,
    };
    let masks = build(
        ctx,
        mask,
        &base.with("section cap masks", "fs_cap_masks").masks(),
    );
    let selection = build(
        ctx,
        mask,
        &base.with("section cap selection", "fs_cap_selection"),
    );
    CapPipelines {
        counts,
        primitives,
        count,
        count_placed: None,
        cap,
        masks,
        selection,
    }
}
