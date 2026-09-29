
use super::lane::PickMode;

impl Gpu {
    /// A click waiting: draw the id pass now.
    fn pending_pick(&mut self, encoder: &mut wgpu::CommandEncoder) {
        if let Some(at) = self.pick.take_pending() {
            self.id_pass(encoder, Some(at));
        }
    }

    /// Draw object ids around the cursor for a pick, then copy them out.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        // stroke ids test against current lists; faces, discs and text need no tables
        let size = (self.config.width, self.config.height);
        let mode = self.pick.mode;
        // draw only the window around the cursor, plus its halo
        let window = at.map(|position| self.pick.window(position, size));
        let view = self.pick.view_for(at, size);
        self.frame.write_pick(&self.ctx, view, size);
        // the window inside the drawn area
        let inner = window.map(|window| {
            (
                window.x.saturating_sub(view.x),
                window.y.saturating_sub(view.y),
                window.w.min(view.w),
                window.h.min(view.h),
            )
        });
        let basic = self.frame.pick_binds(&self.objects.group);

        // source point query: faces and clouds, then the source dots
        if self.pick.source_query() {
            if !self.pick.source_initialized() {
                let mut pass = self.pick.begin_pass(&self.ctx, encoder, view);

                if let Some((x, y, w, h)) = inner {
                    pass.set_scissor_rect(x, y, w, h);
                }

                self.arena.draw_face_ids(&mut pass, &basic);
                self.splat.draw_ids(&mut pass, &self.frame.pick_cloud_group);
            }

            {
                let mut pass = self.pick.begin_source(encoder);

                if let Some((x, y, w, h)) = inner {
                    pass.set_scissor_rect(x, y, w, h);
                }

                let source = self.frame.pick_binds(self.objects.ink_group());
                self.controls.draw_source_ids(&mut pass, &source);
            }

            if let Some(at) = at {
                self.pick.copy_window(&self.ctx, encoder, at, size);
            }

            return;
        }

        // each pass first: which solid a section cap pixel belongs to
        self.each_pass(|pass, g| {
            let basic = g.frame.pick_binds(&g.objects.group);
            pass.before_ids(g, encoder, &basic, (view.w, view.h));
        });
        let basic = self.frame.pick_binds(&self.objects.group);

        {
            // caps, faces and clouds over the whole area, halo included
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, view);

            for other in &self.passes {
                other.in_ids(self, &mut pass, &basic);
            }

            if mode == PickMode::Component {
                self.arena.draw_component_ids(&mut pass, &basic);
            } else {
                self.arena.draw_face_ids(&mut pass, &basic);
            }

            self.splat.draw_ids(&mut pass, &self.frame.pick_cloud_group);
        }
        // ink ids test against the depth just drawn
        let depth = self.pick.depth().expect("physical ID pass creates depth");
        let group = self.objects.pick_group(
            &self.ctx,
            &self.layouts,
            [depth, &self.targets.depth_msaa],
            [self.pick.gradient(), &self.targets.gradient_msaa],
            &self.arena.tiles, // register:tile-binding
        );
        let ink = self.frame.pick_binds(&group);
        {
            let mut pass = self.pick.begin_ink(encoder);

            if let Some((x, y, w, h)) = inner {
                pass.set_scissor_rect(x, y, w, h);
            }

            // which ink may answer depends on the mode
            match mode {
                PickMode::Edge | PickMode::Component => {
                    if self.view.show_mesh_edges {
                        self.segments.draw_edge_ids(&mut pass, &ink);
                    }
                }
                PickMode::Controls { cloud: false, .. } => {
                    self.controls.draw_dot_ids(&mut pass, &ink);
                }
                PickMode::Controls { cloud: true, .. } => {}
                PickMode::Object => {
                    if self.view.show_mesh_edges {
                        self.segments.draw_pipe_ids(&mut pass, &ink);
                    }

                    if self.view.show_lines {
                        self.segments.draw_ribbon_ids(&mut pass, &ink);
                    }

                    if self.view.show_mesh_edges && self.view.markers {
                        self.glyphs.draw_sphere_ids(&mut pass, &ink);
                    }

                    self.arena.draw_text_ids(&mut pass, &basic);

                    if self.view.show_points {
                        self.glyphs.draw_dot_ids(&mut pass, &ink);
                    }
                }
            }

            for lane in &self.registered {
                lane.draw_ids(&mut pass, &ink, &self.view, mode);
            }

            // text ids in every mode
            self.text
                .draw_ids(&mut pass, &self.frame.pick_transform_group);
        }

        if let Some(at) = at {
            self.pick.copy_window(&self.ctx, encoder, at, size);
        }
    }
}
