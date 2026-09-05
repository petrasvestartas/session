//! The frame list: physical surfaces first, then ink against their immutable depth and exact
//! face identities. The optional picking pass follows the same visibility rule and toggles.

use super::frame::Binds;
use super::splat::RecordCx;
use super::Gpu;

impl Gpu {
    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> (u32, u32) {
        self.arena.prepare_faces(&self.ctx, encoder, &self.frame, &self.objects);
        self.point_pass(encoder);

        let mut draws = {
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let mut pass = self.targets.begin_faces(encoder, view, clear);
            self.face_list(&mut pass, &b)
        };
        {
            let mut pass = self.targets.begin_ink(encoder, view);
            draws += self.scene_list(&mut pass);
        }

        if let Some(at) = self.pick.take_pending() {
            self.id_pass(encoder, Some(at));
        }
        (draws, self.objects.len())
    }

    /// The point lane's own pass, skipped while the camera, the knobs and the tables are what
    /// they were - a still cloud costs one fullscreen resolve.
    fn point_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
            cloud_size: self.view.cloud_size,
            lod_px: self.view.lod_px,
            objects: &self.objects,
            clouds: &self.cloud.clouds,
            nodes: &self.cloud.nodes,
        };
        self.splat.prelude(&self.ctx, &self.layouts, encoder, &cx, &self.frame.cloud_group);
    }

    /// Physical faces, backdrop and cloud resolve establish immutable occlusion before ink.
    fn face_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = self.backdrop.draw_background(pass);
        if self.view.show_grid { draws += self.backdrop.draw_grid(pass, b); }
        draws += self.arena.draw_faces(pass, b);
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group);
        draws
    }

    /// Markers follow all strokes so their complete footprints remain on top.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let v = &self.view;
        let basic = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
        let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.ink_group };
        let mut draws = self.arena.draw_print(pass, &basic);
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, &b, v.line_style);
        }
        if v.show_lines {
            draws += self.segments.draw_ribbons(pass, &b);
        }
        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, &b);
        }
        draws += self.arena.draw_text(pass, &basic);
        if v.show_points {
            draws += self.glyphs.draw_dots(pass, &b);
        }
        draws
    }

    /// The id pass: the scene list again, opaque, at 1x, under the same toggles and in the
    /// same order (what a lane hides it cannot pick), then one texel copied out for `Picker`.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        let size = (self.config.width, self.config.height);
        {
            let v = &self.view;
            let basic = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.ink_group };
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, size);
            self.arena.draw_face_ids(&mut pass, &basic);
            self.splat.draw_ids(&mut pass, &self.frame.cloud_group);
            if v.show_mesh_edges {
                self.segments.draw_pipe_ids(&mut pass, &b, v.line_style);
            }
            if v.show_lines {
                self.segments.draw_ribbon_ids(&mut pass, &b);
            }
            if v.show_mesh_edges && v.markers {
                self.glyphs.draw_sphere_ids(&mut pass, &b);
            }
            self.arena.draw_text_ids(&mut pass, &basic);
            if v.show_points {
                self.glyphs.draw_dot_ids(&mut pass, &b);
            }
        }
        if let Some(at) = at {
            self.pick.copy_texel(&self.ctx, encoder, at);
        }
    }
}
