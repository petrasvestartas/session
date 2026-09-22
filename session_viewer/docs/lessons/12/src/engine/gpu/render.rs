// --8<-- [start:step-3a]
use super::Gpu;
use super::frame::Binds;
use super::pick::PickMode;
use super::splat::RecordCx;

impl Gpu {
    /// Encode one frame into `view`; returns (draws, objects).
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> (u32, u32) {
        self.point_pass(encoder);

        // pass 1: background, faces and clouds write depth
        let mut draws = {
            let b = Binds {
                mvp: &self.frame.mvp_group, // the camera matrix
                line: &self.frame.line_group, // pen settings
                instances: &self.objects.group, // per-object rows
            };
            let mut pass = self.targets.begin_faces(encoder, view, clear);
            self.face_list(&mut pass, &b)
        };

        if self.selection_outline.prepare(
            &self.ctx,
            (self.config.width, self.config.height),
            self.targets.samples,
            self.logical_size[0],
            self.arena.face_count() > 0,
        ) {
            let b = Binds {
                mvp: &self.frame.mvp_group, // the camera matrix
                line: &self.frame.line_group, // pen settings
                instances: &self.objects.group, // per-object rows
            };
            let mut pass = self.selection_outline.begin_mask(encoder, &self.targets);
            draws += self.arena.draw_selection_mask(&mut pass, &b);
        }

        {
            // pass 3: lines, markers, outlines and text over the faces
            let mut pass = self.targets.begin_ink(encoder, view);
            draws += self.scene_list(&mut pass);
        }

        // a click waiting: draw the id pass now
        if let Some(at) = self.pick.take_pending() {
            self.id_pass(encoder, Some(at));
        }

        (draws, self.objects.len())
    }

// --8<-- [end:step-3a]
    // --8<-- [start:step-3b]
    /// Draw the point clouds; skipped while nothing changed.
    pub(super) fn point_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
            // point size in framebuffer pixels
            cloud_size: self.view.cloud_size * self.config.width as f32
                / self.logical_size[0].max(1.0) as f32,
            lod_px: self.view.lod_px,
            objects: &self.objects,
            clouds: &self.cloud.clouds,
            nodes: &self.cloud.nodes,
        };
        self.splat.prelude(
            &self.ctx,
            &self.layouts,
            encoder,
            &cx,
            &self.frame.cloud_group,
        );
    }

    /// Draws of the first pass: background, grid, faces, clouds.
    fn face_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = self.backdrop.draw_background(pass);

        if self.view.show_grid {
            draws += self.backdrop.draw_grid(pass, b);
        }

        draws += self.arena.draw_faces(pass, b);
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group);
        draws
    }

    /// Draws of the ink pass, back to front.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let v = &self.view;
        let basic = Binds {
            mvp: &self.frame.mvp_group, // the camera matrix
            line: &self.frame.line_group, // pen settings
            instances: &self.objects.group, // per-object rows
        };
        let b = Binds {
            mvp: &self.frame.mvp_group, // the camera matrix
            line: &self.frame.line_group, // pen settings
            instances: &self.objects.ink_group, // per-object rows plus depth
        };
        let mut draws = self.arena.draw_print(pass, &basic);

        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, &b);
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

        draws += self.selection_outline.draw(pass);
        draws += self.text.draw(pass);
        draws
    }

    // --8<-- [end:step-3b]
    // --8<-- [start:step-16]
    /// Draw object ids around the cursor for a pick, then copy them out.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        let size = (self.config.width, self.config.height);
        let mode = self.pick.mode;
        let window = match at {
            Some(position) => Some(self.pick.window(position, size)),
            None => None,
        };
        let basic = Binds {
            mvp: &self.frame.mvp_group, // the camera matrix
            line: &self.frame.line_group, // pen settings
            instances: &self.objects.group, // per-object rows
        };
        {
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, size);

            // Plane reconstruction also reads neighboring texels: render a small halo around
            if let Some(window) = window {
                let left = window.x.saturating_sub(3);
                let top = window.y.saturating_sub(3);
                let right = (window.x + window.w + 3).min(size.0);
                let bottom = (window.y + window.h + 3).min(size.1);
                pass.set_scissor_rect(left, top, right - left, bottom - top);
            }

            self.arena.draw_face_ids(&mut pass, &basic);
            self.splat.draw_ids(&mut pass, &self.frame.cloud_group);
        }
        // ink ids test against the depth just drawn
        let depth = self.pick.depth().expect("physical ID pass creates depth");
        let group = self.objects.pick_group(
            &self.ctx,
            &self.layouts,
            [depth, &self.targets.depth_msaa],
            [self.pick.gradient(), &self.targets.gradient_msaa],
        );
        let ink = Binds {
            mvp: &self.frame.mvp_group, // the camera matrix
            line: &self.frame.line_group, // pen settings
            instances: &group, // per-object rows
        };
        {
            let mut pass = self.pick.begin_ink(encoder);

            if let Some(window) = window {
                pass.set_scissor_rect(window.x, window.y, window.w, window.h);
            }

            // which ink may answer depends on the mode
            match mode {
                PickMode::Edge => {
                    if self.view.show_mesh_edges {
                        self.segments.draw_edge_ids(&mut pass, &ink);
                    }
                }
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
        }

        if let Some(at) = at {
            self.pick.copy_window(&self.ctx, encoder, at);
        }
    }
}
// --8<-- [end:step-16]
