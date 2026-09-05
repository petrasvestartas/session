//! The frame list. `encode_frame` runs the point pass, then ONE scene pass whose `scene_list`
//! is the ordered lane draws. The order is the contract: everything that writes depth first,
//! the blended ink after, lettering last.

use super::frame::Binds;
use super::splat::RecordCx;
use super::Gpu;

impl Gpu {
    /// Encode the whole frame into `view`. Returns (draws, objects).
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> (u32, u32) {
        self.point_pass(encoder);

        let draws = {
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let mut pass = self.targets.begin_pass(encoder, view, clear);
            self.scene_list(&mut pass, &b)
        };
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

    /// The scene list, in order:
    /// 1 background · 2 grid · 3 faces · 4 sheet fills · 5 mesh edges · 6 clouds · 7 vertex
    /// markers · 8 lines · 9 lettering · 10 point dots. Lines write no depth: two lines on one
    /// pixel resolve by draw order (a depth prepass costs a second ribbon draw - measured +5 ms
    /// on view_mixed - for a case only coincident lines of different colours can show).
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let v = &self.view;
        let mut draws = 0u32;

        draws += self.backdrop.draw_background(pass);
        draws += self.backdrop.draw_grid(pass, b);
        draws += self.arena.draw_faces(pass, b);
        draws += self.arena.draw_print(pass, b);
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, b, v.line_style);
        }
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group);
        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, b);
        }
        if v.show_lines {
            draws += self.segments.draw_ribbons(pass, b);
        }
        draws += self.arena.draw_text(pass, b);
        if v.show_points {
            draws += self.glyphs.draw_dots(pass, b);
        }
        draws
    }
}
