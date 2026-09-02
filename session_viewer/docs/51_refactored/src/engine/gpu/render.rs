//! The frame list - `Gpu::encode_frame`: the compute prelude (splat records for both lanes,
//! the static skip, the 2x2 dispatches), then ONE render pass whose `scene_list` is eleven
//! draws in a fixed order. The order is the contract; every draw is a call into the family
//! that owns the rows, and every family hands back its draw count.

use super::backdrop::{draw_background, draw_grid};
use super::frame::Binds;
use super::splat::RecordCx;
use super::Gpu;

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

impl Gpu {
    /// Splat the clouds by COMPUTE before the render pass. One thread per point, twice (depth
    /// race, then colour claim); the render pass composites the result with one fullscreen
    /// triangle. TWO record sets - the walked lane and the stream lane bind different point
    /// buffers - but one pixel buffer pair: atomics compose across dispatches, so both lanes
    /// contest the same per-pixel depth race.
    fn splat_prelude(&mut self, encoder: &mut wgpu::CommandEncoder) {
        // Static skip FIRST: camera still, same scale, nothing rebuilt - the buffers already
        // hold this exact frame's splats, so not even the records are built.
        let (mvp, cloud_size) = (self.frame.mvp_f32, self.view.cloud_size);
        if self.splat.is_current(&mvp, cloud_size) {
            return;
        }

        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
            cloud_size,
            lod_split_px: self.view.lod_split_px,
            objects: &self.objects,
        };
        self.splat.walked.build(&cx, &self.cloud.draws, &self.cloud.nodes);
        self.splat.streamed.build(&cx, &self.stream.draws, &[]);
        if self.splat.total() == 0 {
            return;
        }
        self.splat.walked.write(&self.ctx);
        self.splat.streamed.write(&self.ctx);
        encoder.clear_buffer(&self.splat.pixels.depth, 0, None); // 0 bits = reverse-Z far = empty
        encoder.clear_buffer(&self.splat.pixels.color, 0, None);

        // BOTH lanes' depth races must settle before EITHER lane claims colours -
        // dispatches in one pass are ordered, so lane order inside each phase is free.
        let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        cp.set_pipeline(&self.pipelines.splat_depth);
        self.splat.walked.dispatch(&mut cp);
        self.splat.streamed.dispatch(&mut cp);
        cp.set_pipeline(&self.pipelines.splat_color);
        self.splat.walked.dispatch(&mut cp);
        self.splat.streamed.dispatch(&mut cp);
        self.splat.mark_current(&mvp, cloud_size);
    }

    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> (u32, u32) {
        let mut draws = 0u32;
        self.splat_prelude(encoder);

        {
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let mut pass = self.targets.begin_pass(encoder, view, color);
            draws += self.scene_list(&mut pass, &b);
        }

        (draws, self.objects.len())
    }

    /// The scene list - eleven draws, and the ORDER is the contract:
    /// background -> grid -> faces -> print -> pipes -> CLOUD -> sphere markers -> ink
    /// prepass -> ribbons -> text -> dots. Everything that WRITES depth comes first (the cloud
    /// included, since it went opaque); the flat ink lanes read that depth and never
    /// write it. The markers go with the solids so the line ink tests against them -
    /// a vertex marker is the topmost ink at its own joint.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let (p, v) = (&self.pipelines, &self.view);
        let mut draws = 0u32;

        draws += draw_background(pass, p);
        draws += draw_grid(pass, p, b);

        draws += self.arena.draw_faces(pass, p, b);
        draws += self.arena.draw_print(pass, p, b);
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, p, b, v.line_style);
        }

        draws += self.splat.draw_resolve(pass, p, &self.frame.cloud_group);

        // Markers go LAST of the solid lane - see `GlyphLane::draw_spheres`.
        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, p, b);
        }

        draws += self.ink_depth_prepass(pass, b);

        if v.show_lines {
            draws += self.segments.draw_ribbons(pass, p, b);
        }

        draws += self.arena.draw_text(pass, p, b);

        if v.show_points {
            draws += self.glyphs.draw_dots(pass, p, b);
        }
        draws
    }

    /// FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
    /// write depth (its AA feather would leave halos), so without this nothing in the flat
    /// lane occludes anything else in it and pure draw order wins - a point dot then sits
    /// on top of a polyline it is behind, at every camera angle.
    /// COST: it draws the whole flat lane a SECOND time. On 2D sheets (600k segments, all
    /// ribbons) that doubles the frame - so it is off by default and only worth enabling
    /// for 3D scenes where ink-vs-ink order is actually visible.
    fn ink_depth_prepass(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = 0u32;

        if INK_DEPTH_PREPASS && self.view.show_lines {
            draws += self.segments.draw_ribbon_depth(pass, &self.pipelines, b);
        }
        if INK_DEPTH_PREPASS && self.view.show_points {
            draws += self.glyphs.draw_dot_depth(pass, &self.pipelines, b);
        }
        draws
    }
}
