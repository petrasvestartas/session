// --8<-- [start:encode]
use super::Gpu;
use super::frame::Binds;
use super::pass::Frame;

// `impl Gpu` blocks may sit in any file of the crate: this one adds the frame encoding.
impl Gpu {
    /// Encode one frame into `view`; returns (draws, objects).
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> (u32, u32) {
        self.each_pass(|pass, g| pass.prepare(g, encoder));

        let tier = if self.view.ssao {
            0
        } else {
            self.performance.drag_tier()
        };

        let frame = Frame {
            view,
            clear,
            tier,
        };
        // pass 1: background, section caps, faces and clouds write depth
        let mut draws = self.face_passes(encoder, &frame);
        // pass 2: ambient occlusion and the outline masks, each pass in turn
        self.each_pass(|pass, g| draws += pass.after_faces(g, encoder, &frame));
        draws += self.ink_pass(encoder, view); // pass 3: lines, markers, outlines and text; register:ink
        (draws, self.objects.len())
    }
// --8<-- [end:encode]

// --8<-- [start:face-passes]
    /// The first pass: each pass's own face passes, then the one the faces draw in.
    fn face_passes(&mut self, encoder: &mut wgpu::CommandEncoder, f: &Frame) -> u32 {
        let mut draws = 0;
        let mut drew = false;
        self.each_pass(|pass, g| draws += pass.before_faces(g, encoder, f, &mut drew));

        let b = self.frame.binds(&self.objects.group);
        let mut pass = self
            .targets
            .begin_faces(encoder, f.view, (!drew).then_some(f.clear));

        if !drew {
            draws += self.backdrop_list(&mut pass, &b);
        }

        for other in &self.passes {
            draws += other.in_faces(self, &mut pass, &b);
        }

        draws + self.face_list(&mut pass, &b)
    }

    /// Draws of the backdrop: background and grid.
    pub(super) fn backdrop_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = self.backdrop.draw_background(pass, b);

        draws += self.grid_list(pass, b); // register:camera
        draws
    }

    /// Draws of the first pass after the backdrop and caps: faces, clouds.
    fn face_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let clipped = self.passes.iter().any(|pass| pass.clips());
        let opaque = self.view.opacity >= 1.0; // at full opacity the blend returns the face color itself
        let mut draws = 0;
        draws += self.arena.draw_faces(pass, b, opaque, clipped); // register:meshes
        draws
    }
}
// --8<-- [end:face-passes]

// --8<-- [start:02-grid]
// --8<-- [start:grid-list]
impl Gpu {
    /// The grid, when shown.
    fn grid_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.view.show_grid {
            self.backdrop.draw_grid(pass, b)
        } else {
            0
        }
    }
}
// --8<-- [end:grid-list]
// --8<-- [end:02-grid]

// --8<-- [start:04b-ink-pass]
// --8<-- [start:ink-pass]
impl Gpu {
    /// Pass 3: lines, markers, outlines and text over the faces.
    fn ink_pass(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) -> u32 {
        let mut pass = self.targets.begin_ink(encoder, view);
        self.scene_list(&mut pass)
    }

    /// Draws of the ink pass, back to front.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let v = &self.view;
        let basic = self.frame.binds(&self.objects.group);
        let b = self.frame.binds(self.objects.ink_group());
        let mut draws = 0;
        draws += self.arena.draw_print(pass, &basic);
        draws += self
            .segments
            .draw_unselected(pass, &b, v.show_mesh_edges, v.show_lines);
        // selected mesh edges now, its curves after the outline
        draws += self
            .segments
            .draw_selected(pass, &b, v.show_mesh_edges, false);
        for other in &self.passes {
            draws += other.over_ink(self, pass, &b);
        }
        // selected curves over the outline
        draws += self.segments.draw_selected(pass, &b, false, v.show_lines);
        for lane in &self.registered {
            draws += lane.draw_ink(pass, &b, v);
        }

        draws += self.arena.draw_text(pass, &basic);
        draws
    }
}
// --8<-- [end:ink-pass]
// --8<-- [end:04b-ink-pass]
