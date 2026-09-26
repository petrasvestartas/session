use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build,
    instance_id_layout, scene_module, vertex_layout,
};

/// The three mesh buffers one sheet draw reads, borrowed from the arena.
pub struct OutlineBuffers<'a> {
    pub vertices: &'a GrowBuf, // vertex positions
    pub objects: &'a GrowBuf, // object row per vertex
    pub indices: &'a GrowBuf, // triangle indices
}

/// Draws sheet fills and lettering: flat color, no lighting.
pub struct OutlineTextLane {
    shader: Shader, // text outline shader
    color: Pipeline, // in color
    id: Pipeline, // object ids
    physical_id: Pipeline, // object ids with depth and gradient
}

impl OutlineTextLane {
    /// Compile the shader and build the three pipelines.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            ctx,
            "text-outline.shader",
            shader!("text_outline.wgsl"),
        );
        let (color, id, physical_id) = pipelines(ctx, layouts, &shader, target);
        Self {
            shader,
            color,
            id,
            physical_id,
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id, self.physical_id) = pipelines(ctx, layouts, &self.shader, target);
    }

    /// Draw in color.
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.color)
    }

    /// Draw object ids, writing depth and gradient too.
    pub fn draw_physical_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.physical_id)
    }

    /// Draw object ids.
    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        draw(pass, binds, buffers, &self.id)
    }
}

/// One indexed draw over the buffers with `pipeline`; returns the draw count.
fn draw(
    pass: &mut wgpu::RenderPass<'_>,
    binds: &Binds,
    buffers: &OutlineBuffers<'_>,
    pipeline: &Pipeline,
) -> u32 {
    if buffers.indices.is_empty() {
        return 0;
    }

    pass.set_pipeline(pipeline);
    binds.set(pass);
    pass.set_vertex_buffer(0, buffers.vertices.buf.slice(..));
    pass.set_vertex_buffer(1, buffers.objects.buf.slice(..));
    pass.set_index_buffer(buffers.indices.buf.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..buffers.indices.len(), 0, 0..1);
    1
}

/// Build the three pipelines: color, id, physical id.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &Shader,
    target: Target,
) -> (
    Pipeline,
    Pipeline,
    Pipeline,
) {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance];
    let vertices = [vertex_layout(), instance_id_layout()];
    let base = PipelineDesc::new(
        shader,
        &groups,
        &vertices,
        wgpu::PrimitiveTopology::TriangleList,
    );
    let color = build(
        ctx,
        target,
        &base
            .with("text-outline", "fs_main")
            .color(ColorWrite::Blended)
            .depth(DepthMode::ReadOnly),
    );
    let id = build(
        ctx,
        Target::ID,
        &base
            .with("text-outline.id", "fs_id")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let physical_id = build(
        ctx,
        Target::ID,
        &base
            .with("text-outline.physical_id", "fs_physical_id")
            .physical()
            .depth(DepthMode::ReadOnlyEqual),
    );
    (color, id, physical_id)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::engine::gpu::{FrameInput, Gpu, Instance, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A lone glyph gets MSAA, keeps its id, turns yellow, hides fully.
    fn isolated_outline_keeps_coverage_selection_and_identity() {
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 128)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.msaa_forced = None;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(
            Xform::identity(),
            Instance::FLAG_PRINT | Instance::FLAG_SHEET,
        ));

        // a thin slanted triangle; only MSAA gives grey edge pixels
        for position in [[-0.7, -0.12, 0.5], [0.6, 0.24, 0.5], [0.6, 0.07, 0.5]] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.0, 0.0, 0.0, 1.0],
            });
            upload.arena.vids.push(0);
        }

        upload.arena.idx_text = vec![0, 1, 2];
        gpu.set_scene(&upload);
        assert_eq!(
            gpu.targets.samples, 4,
            "outline-only scenes must request coverage below the budget"
        );
        let frame = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let pixels = gpu.render_offscreen(&frame);
        let mut interiors = 0;
        let mut coverage = 0;

        for pixel in pixels.chunks_exact(4) {
            interiors += usize::from(pixel[0] == 0);
            coverage += usize::from(pixel[0] > 0 && pixel[0] < 255);
        }

        assert!(interiors > 100, "opaque glyph interiors must survive");
        assert!(
            coverage > 50,
            "outline edges must have real partial coverage"
        );
        let ids = gpu.render_ids_offscreen(&frame);
        assert!(
            ids.contains(&[1, 0]),
            "the dedicated outline ID pipeline must preserve object identity"
        );
        // older files put lettering in the print run; same picture
        upload.arena.idx_print = std::mem::take(&mut upload.arena.idx_text);
        gpu.set_scene(&upload);
        assert_eq!(gpu.targets.samples, 4);
        assert_eq!(pixels, gpu.render_offscreen(&frame));
        assert_eq!(ids, gpu.render_ids_offscreen(&frame));
        gpu.set_selected(0, true);
        let pixels = gpu.render_offscreen(&frame);
        assert!(
            pixels.chunks_exact(4).any(is_yellow),
            "selection must remain opaque yellow"
        );
        gpu.set_hidden(0, true);
        let pixels = gpu.render_offscreen(&frame);
        assert!(
            pixels.chunks_exact(4).all(is_white),
            "hidden outlines must leave no color pixels"
        );
        assert!(!gpu.render_ids_offscreen(&frame).contains(&[1, 0]));
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A cancelled pick returns nothing; the next pick works.
    fn canceled_completion_is_discarded_and_next_pick_still_completes() {
        let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
        gpu.view.show_grid = false;
        let frame = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        gpu.pick.request(32, 32);
        let at = gpu.pick.take_pending().unwrap();
        gpu.pick_frame(&frame, at);
        assert!(gpu.pick.busy());
        // same cancel as Escape or a camera move
        gpu.pick.cancel();
        gpu.ctx
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .unwrap();
        assert_eq!(
            gpu.pick.poll(),
            None,
            "an already submitted result must stay invalidated"
        );
        assert!(!gpu.pick.busy());
        gpu.pick.request(32, 32);
        let at = gpu.pick.take_pending().unwrap();
        gpu.pick_frame(&frame, at);
        gpu.ctx
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .unwrap();
        assert_eq!(
            gpu.pick.poll(),
            Some(None),
            "the next valid background pick must complete normally"
        );
        assert!(!gpu.pick.busy());
    }

    /// True for the selection yellow.
    fn is_yellow(pixel: &[u8]) -> bool {
        pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 0
    }

    /// True for the white background.
    fn is_white(pixel: &[u8]) -> bool {
        pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255
    }
}
