//! Exact-position PDF outline text. The importer already applied the original font glyph
//! and text matrices; re-shaping those meshes with a replacement font would lose the PDF.
//! Legacy imports mix glyphs with page fills; both sheet runs use this lane.
//! This lane owns unlit color/ID pipelines and explicitly borrows the arena's existing
//! geometry buffers. It adds no glyph geometry copy or alternative scene model.

use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module,
    vertex_layout,
};

/// Exact outline buffers, still owned and released by the shared geometry arena.
pub struct OutlineBuffers<'a> {
    pub vertices: &'a GrowBuf,
    pub objects: &'a GrowBuf,
    pub indices: &'a GrowBuf,
}

/// Dedicated unlit coverage and identity pipelines over the imported lettering indices.
pub struct OutlineTextLane {
    shader: wgpu::ShaderModule,
    color: wgpu::RenderPipeline,
    id: wgpu::RenderPipeline,
    physical_id: wgpu::RenderPipeline,
}

impl OutlineTextLane {
    /// Compile the exact-outline shader against the viewer's shared layouts and device.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            &ctx.device,
            "text-outline.shader",
            include_str!("../../shaders/text_outline.wgsl"),
        );
        let (color, id, physical_id) = pipelines(ctx, layouts, &shader, target);
        Self {
            shader,
            color,
            id,
            physical_id,
        }
    }

    /// Follow the shared coverage sample count while retaining all source geometry.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id, self.physical_id) = pipelines(ctx, layouts, &self.shader, target);
    }

    /// Draw imported glyph coverage after page fills and linework; depth remains read-only.
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.color);
        draw(pass, binds, buffers)
    }

    /// Sheet IDs preserve the physical depth and gradient already written by their face.
    pub fn draw_physical_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.physical_id);
        draw(pass, binds, buffers)
    }

    /// Preserve the existing exact object IDs and sheet depth comparison in the picking pass.
    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        buffers: &OutlineBuffers<'_>,
    ) -> u32 {
        pass.set_pipeline(&self.id);
        draw(pass, binds, buffers)
    }
}

/// Submit the existing indexed outlines without modifying vertex positions or winding.
fn draw(pass: &mut wgpu::RenderPass<'_>, binds: &Binds, buffers: &OutlineBuffers<'_>) -> u32 {
    if buffers.indices.is_empty() {
        return 0;
    }
    binds.set(pass);
    pass.set_vertex_buffer(0, buffers.vertices.buf.slice(..));
    pass.set_vertex_buffer(1, buffers.objects.buf.slice(..));
    pass.set_index_buffer(buffers.indices.buf.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..buffers.indices.len(), 0, 0..1);
    1
}

/// Opaque glyph interiors use sample coverage and straight-alpha compositing. Imported
/// sheet lettering keeps the established Greater color / GreaterEqual ID depth convention.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> (
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
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
        &ctx.device,
        target,
        &base
            .with("text-outline", "fs_main")
            .color(ColorWrite::Blended)
            .depth(DepthMode::ReadOnly),
    );
    let id = build(
        &ctx.device,
        Target::ID,
        &base
            .with("text-outline.id", "fs_id")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let physical_id = build(
        &ctx.device,
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
    fn isolated_outline_keeps_coverage_selection_and_identity() {
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 128)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.msaa_forced = None;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(
            Xform::identity().to_f32().map(f64::from),
            Instance::FLAG_PRINT | Instance::FLAG_SHEET,
        ));
        // A thin, slanted outline at a fractional pixel position. At one sample its
        // boundary has only black/white pixels; four samples preserve partial coverage.
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
        // Older serialized PDFs do not label text meshes. Their mixed print run must
        // receive identical coverage without trying to infer glyphs from triangles.
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
        // This is the same invalidation called by Escape, parent changes and camera changes.
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

    /// Recognize the exact opaque selection color.
    fn is_yellow(pixel: &[u8]) -> bool {
        pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 0
    }
    /// Recognize untouched white background after hiding an outline.
    fn is_white(pixel: &[u8]) -> bool {
        pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255
    }
}
