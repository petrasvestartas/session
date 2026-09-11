//! The frame list: physical surfaces first, then ink against their immutable depth. The
//! optional picking pass follows the same visibility rule and toggles.

use super::Gpu;
use super::frame::Binds;
use super::pick::PickMode;
use super::splat::RecordCx;
use super::surface_outline;

impl Gpu {
    /// Reconstruct finite triangle visibility before color or ID ink samples it.
    fn triangle_tile_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
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

    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> (u32, u32) {
        self.triangle_tile_pass(encoder);
        self.point_pass(encoder);

        let mut draws = {
            let b = self.frame.binds(&self.objects.group);
            let mut pass = self.targets.begin_faces(encoder, view, clear);
            self.face_list(&mut pass, &b)
        };
        let size = (self.config.width, self.config.height);
        // No silhouettes in x-ray: with the faces gone they would only paint over the edges
        // and vertices that are the picture.
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
        // The masks depend on the camera, the geometry, the selection and the highlighted
        // source face; while those stand still the previous masks are composited again.
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
            // The edges extend the coverage by their own footprint, tested against the same
            // physical depth the ink pass reads, so the ring wraps them.
            let ink = self.frame.binds(&self.objects.ink_group);
            let edges = self.view.show_mesh_edges;
            if solid && selected {
                // One rasterization of the faces writes both masks.
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
    pub(super) fn point_pass(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
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

    /// Backdrop, physical faces and the cloud resolve write the depth every ink fragment reads.
    fn face_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = self.backdrop.draw_background(pass);
        if self.view.show_grid {
            draws += self.backdrop.draw_grid(pass, b);
        }
        draws += self.arena.draw_faces(pass, b);
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group);
        draws
    }

    /// Markers follow all strokes so their complete footprints remain on top.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let v = &self.view;
        let basic = self.frame.binds(&self.objects.group);
        let b = self.frame.binds(&self.objects.ink_group);
        let mut draws = self.arena.source_faces.draw_highlight(pass, &basic);
        draws += self.arena.draw_print(pass, &basic);
        draws += self
            .segments
            .draw_unselected(pass, &b, v.show_mesh_edges, v.show_lines);
        // (pipes, ribbons): the selected object's mesh and BRep edges now; its standalone
        // curves come after the silhouette below, so they are not cut by it.
        draws += self
            .segments
            .draw_selected(pass, &b, v.show_mesh_edges, false);
        draws += self
            .solid_outline
            .draw_combined(&self.selection_outline, pass);
        // Standalone selected curves cover coincident mesh ink. Solid boundary strokes
        // stay below the silhouette so their yellow fringe cannot narrow its black border.
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
        draws += self.text.draw(pass);
        draws
    }

    /// The id pass: the scene list again, opaque, at 1x, under the same toggles and in the
    /// same order (what a lane hides it cannot pick), scissored to the pick window when there
    /// is one (the vertex work stays; the fill is what a full frame would cost), which is then
    /// copied out for `Picker`.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        self.triangle_tile_pass(encoder);
        let size = (self.config.width, self.config.height);
        let mode = self.pick.mode;
        // The pass draws the window about the cursor (plus the halo the plane fit reads) into
        // an attachment of that size; the uniforms see the scene through that window.
        let window = at.map(|position| self.pick.window(position, size));
        let view = self.pick.view_for(at, size);
        self.frame.write_pick(&self.ctx, view, size);
        let inner = window.map(|window| {
            (
                window.x.saturating_sub(view.x),
                window.y.saturating_sub(view.y),
                window.w.min(view.w),
                window.h.min(view.h),
            )
        });
        let basic = self.frame.pick_binds(&self.objects.group);
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
            // The whole attachment, halo included: the plane reconstruction reads neighbouring
            // texels, which must be occlusion samples rather than cleared ones.
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, view);
            if mode == PickMode::Component {
                self.arena.draw_component_ids(&mut pass, &basic);
            } else {
                self.arena.draw_face_ids(&mut pass, &basic);
            }
            self.splat.draw_ids(&mut pass, &self.frame.pick_cloud_group);
        }
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
            // Authored text covers geometry in every pick mode, just as its visible plane does.
            self.text
                .draw_ids(&mut pass, &self.frame.pick_transform_group);
        }
        if let Some(at) = at {
            self.pick.copy_window(&self.ctx, encoder, at, size);
        }
    }
}
