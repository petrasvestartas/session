//! Records one frame: every pass in order into one encoder, then a single submit.
use super::Gpu;
use super::frame::Binds;
use super::pick::PickMode;
use super::splat::RecordCx;
use super::surface_outline;

impl Gpu {
    /// Compute which triangles cover which screen tiles.
    fn triangle_tile_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        // a moved tile buffer needs a new bind group
        if self.arena.tiles.prepare(
            &self.ctx,
            (self.config.width, self.config.height),
            self.arena.face_count() / 3,
        ) {
            self.rebind_ink();
        }

        let b = self.frame.binds(&self.objects.group);
        self.arena.prepare_visibility(
            &self.ctx,
            encoder,
            &b,
            self.frame.mvp_f32,
            self.objects.geometry_revision(),
        );
    }

    /// Encode one frame into `view`; returns (draws, objects).
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> (u32, u32) {
        self.triangle_tile_pass(encoder);
        self.point_pass(encoder);

        // pass 1: background, faces and clouds write depth
        let mut draws = {
            let b = self.frame.binds(&self.objects.group);
            let mut pass = self.targets.begin_faces(encoder, view, clear);
            self.face_list(&mut pass, &b)
        };
        let size = (self.config.width, self.config.height);
        // no outlines in x-ray
        let faces =
            self.view.show_outlines && self.view.opacity > 0.0 && self.arena.face_count() > 0;
        let selected = self.selection_outline.prepare(
            &self.ctx,
            size,
            self.targets.samples,
            self.logical_size[0],
            faces,
        );
        let solid = self.solid_outline.prepare(
            &self.ctx,
            size,
            self.targets.samples,
            self.logical_size[0],
            faces,
        );
        // redraw the outline masks only when something changed
        let key = surface_outline::MaskKey {
            mvp: self.frame.mvp_f32,
            geometry: self.objects.geometry_revision(),
            selection: self.selection_revision,
            faces: self.arena.source_faces.revision(),
            size,
            samples: self.targets.samples,
            edges: self.view.show_mesh_edges,
            pen: self.view.thickness_px.to_bits(),
        };
        let stale = (solid && !self.solid_outline.is_valid(&key))
            || (selected && !self.selection_outline.is_valid(&key));

        if stale {
            let b = self.frame.binds(&self.objects.group);
            // edges widen the mask
            let ink = self.frame.binds(&self.objects.ink_group);
            let edges = self.view.show_mesh_edges;

            if solid && selected {
                // one pass writes both masks
                let mut pass = surface_outline::SurfaceOutline::begin_masks(
                    &self.solid_outline,
                    &self.selection_outline,
                    encoder,
                    &self.targets,
                );
                draws += self.arena.draw_masks(&mut pass, &b);
                draws += self.arena.source_faces.draw_masks(&mut pass, &b);

                if edges {
                    draws += self.segments.draw_masks(&mut pass, &ink);
                }
            } else if solid {
                let mut pass = self.solid_outline.begin_mask(encoder, &self.targets);
                draws += self.arena.draw_solid_mask(&mut pass, &b);

                if edges {
                    draws += self.segments.draw_solid_mask(&mut pass, &ink);
                }
            } else if selected {
                let mut pass = self.selection_outline.begin_mask(encoder, &self.targets);
                draws += self.arena.draw_selection_mask(&mut pass, &b);
                draws += self.arena.source_faces.draw_mask(&mut pass, &b);

                if edges {
                    draws += self.segments.draw_selection_mask(&mut pass, &ink);
                }
            }

            if solid {
                self.solid_outline.encode_pool(encoder);
                self.solid_outline.mark_valid(key);
            }

            if selected {
                self.selection_outline.encode_pool(encoder);
                self.selection_outline.mark_valid(key);
            }
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
        let basic = self.frame.binds(&self.objects.group);
        let b = self.frame.binds(&self.objects.ink_group);
        let mut draws = self.arena.source_faces.draw_highlight(pass, &basic);
        draws += self.arena.draw_print(pass, &basic);
        draws += self
            .segments
            .draw_unselected(pass, &b, v.show_mesh_edges, v.show_lines);
        // selected mesh edges now, its curves after the outline
        draws += self
            .segments
            .draw_selected(pass, &b, v.show_mesh_edges, false);
        draws += self
            .solid_outline
            .draw_combined(&self.selection_outline, pass);
        // selected curves over the outline
        draws += self.segments.draw_selected(pass, &b, false, v.show_lines);

        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, &b);
        }

        draws += self.arena.draw_text(pass, &basic);

        if v.show_points {
            draws += self.glyphs.draw_dots(pass, &b);
        }

        draws += self.control_net.draw_ribbons(pass, &b);
        draws += self.controls.draw_dots(pass, &b);
        // --8<-- [start:step-12]
        // Last of the ink list, so the widget is drawn over the object it moves.
        draws += self.gizmo_arms.draw_ribbons(pass, &b);
        draws += self.gizmo_dots.draw_dots(pass, &b);
        // --8<-- [end:step-12]
        draws += self.text.draw(pass);
        draws
    }

    /// Draw object ids around the cursor for a pick, then copy them out.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        self.triangle_tile_pass(encoder);
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

                let source = self.frame.pick_binds(&self.objects.ink_group);
                self.controls.draw_source_ids(&mut pass, &source);
            }

            if let Some(at) = at {
                self.pick.copy_window(&self.ctx, encoder, at, size);
            }

            return;
        }

        {
            // faces and clouds over the whole area, halo included
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, view);

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
            &self.arena.tiles,
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

            // text ids in every mode
            self.text
                .draw_ids(&mut pass, &self.frame.pick_transform_group);
        }

        if let Some(at) = at {
            self.pick.copy_window(&self.ctx, encoder, at, size);
        }
    }
}
