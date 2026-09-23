use super::Gpu;
use super::frame::Binds;
use super::pick::PickMode;
use super::splat::RecordCx;
use super::surface_outline;

impl Gpu {
    /// Timestamp the GPU here when a bench installed a pass timer.
    fn mark(&mut self, encoder: &mut wgpu::CommandEncoder, label: &'static str) {
        if let Some(timer) = self.timer.as_mut() {
            timer.mark(encoder, label);
        }
    }

    /// What reads the triangle tables this frame: (the projection, the tile lists).
    fn tile_readers(&self) -> (bool, bool) {
        let v = &self.view;
        // strokes test visibility against the lists; discs read depth only
        let lists = (v.show_mesh_edges && self.live_pipes() > 0)
            || (v.show_lines && self.live_ribbons() > 0)
            || self.control_net.ribbon_count() > 0
            || self.registered.iter().any(|lane| lane.reads_tiles(v));
        (lists, lists)
    }

    /// Project the triangles, and bin them into screen tiles when `lists`; nothing when
    /// `projection` is false, and the stale tables rebuild on the next frame that reads them.
    fn triangle_tile_pass(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        projection: bool,
        lists: bool,
    ) {
        if !projection {
            return;
        }

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
            lists,
        );
    }

    /// Encode one frame into `view`; returns (draws, objects).
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> (u32, u32) {
        self.mark(encoder, "start");
        let tier = self.performance.drag_tier();
        let (projection, lists) = self.tile_readers();
        // a slow drag tests ink against the fitted planes alone; the lists return when it ends
        let rough = lists && tier >= 1;
        self.triangle_tile_pass(encoder, projection, lists && !rough);

        if rough {
            self.arena.tiles.drop_lists(encoder);
        }

        self.mark(encoder, "tiles");
        self.point_pass(encoder);
        self.mark(encoder, "points");

        // pass 1: background, section caps, faces and clouds write depth
        let caps = self.cap_planes().1 > 0;
        let mut draws = self.face_passes(encoder, view, clear);
        self.mark(encoder, "faces");
        // pass 2: ambient occlusion keeps the same quality throughout navigation
        let ambient = self.view.ssao && self.view.opacity > 0.0 && self.live_faces() > 0;

        if ambient {
            let full = (self.config.width, self.config.height);
            let dpr = f64::from(full.0) / self.logical_size[0].max(1.0);
            let target = self.target();
            if let Some(pipes) = super::ssao::cached(&mut self.ssao_pipes, &self.ctx, target) {
                // textures follow the canvas; pipelines stay
                if self.ssao.as_ref().is_some_and(|ssao| !ssao.fits(pipes, full, dpr)) {
                    self.ssao = None;
                }
                let ssao = self
                    .ssao
                    .get_or_insert_with(|| super::ssao::Ssao::new(&self.ctx, pipes, full, dpr));
                let receiver = ssao.receiver(&self.objects);
                let [vertices, owners, indices] = self.arena.geometry_buffers();
                draws += ssao.draw(
                    &self.ctx,
                    pipes,
                    &self.targets,
                    [vertices, owners, indices, self.objects.instance_buffer()],
                    encoder,
                    view,
                    self.frame.mvp_f32,
                    receiver,
                    self.objects.geometry_revision(),
                    self.timer.as_mut(),
                );
            }
        } else {
            self.ssao = None;
        }
        self.mark(encoder, "ssao");
        let size = (self.config.width, self.config.height);
        // no outlines in x-ray
        let faces = self.view.show_outlines && self.view.opacity > 0.0 && self.live_faces() > 0;
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
        // the compositing outline holds the taps and alpha for both masks
        let radius = surface_outline::radius(size, self.logical_size[0]);
        self.solid_outline
            .prepare_alpha(&self.ctx, (solid || selected).then_some(radius), size);
        // redraw the outline masks only when something changed
        let key = surface_outline::MaskKey {
            mvp: self.frame.mvp_f32,
            geometry: self.objects.geometry_revision(),
            selection: self.selection_revision,
            faces: self.arena.source_faces.revision(),
            size,
            samples: self.targets.samples,
            edges: self.view.show_mesh_edges && tier < 2,
            rough,
            pen: self.view.thickness_px.to_bits(),
        };
        let stale = (solid && !self.solid_outline.is_valid(&key))
            || (selected && !self.selection_outline.is_valid(&key));

        if stale {
            self.solid_outline.bind_faces(&self.ctx, &self.targets);

            if caps {
                self.clip.bind_primitives(&self.ctx, &self.targets);
            }

            let b = self.frame.binds(&self.objects.group);
            // edges widen the mask, except in a slow drag
            let ink = self.frame.binds(&self.objects.ink_group);
            let edges = key.edges && self.live_pipes() > 0;

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

                if caps {
                    draws += self.clip.draw_cap_masks(&mut pass, &b);
                }

                if edges {
                    draws += self.segments.draw_masks(&mut pass, &ink);
                }
            } else if solid && edges {
                // faces from the face pass's triangle ids, then the edges over them
                let mut pass = self.solid_outline.begin_mask(encoder, &self.targets);
                draws += self.solid_outline.draw_faces(&mut pass);
                draws += self.segments.draw_solid_mask(&mut pass, &ink);
            } else if solid {
                draws += self.solid_outline.encode_faces(encoder);
            } else if selected {
                let mut pass = self.selection_outline.begin_mask(encoder, &self.targets);
                draws += self.arena.draw_selection_mask(&mut pass, &b);
                draws += self.arena.source_faces.draw_mask(&mut pass, &b);

                if caps {
                    draws += self.clip.draw_cap_selection(&mut pass, &b);
                }

                if edges {
                    draws += self.segments.draw_selection_mask(&mut pass, &ink);
                }
            }

            self.mark(encoder, "masks");

            if solid {
                self.solid_outline.encode_pool(encoder);
                self.solid_outline.mark_valid(key);
            }

            if selected {
                self.selection_outline.encode_pool(encoder);
                self.selection_outline.mark_valid(key);
            }

            self.mark(encoder, "pool");
        }

        self.solid_outline
            .encode_alpha(&self.selection_outline, encoder, stale);
        self.mark(encoder, "alpha");

        {
            // pass 3: lines, markers, outlines and text over the faces
            let mut pass = self.targets.begin_ink(encoder, view);
            draws += self.scene_list(&mut pass);
        }

        self.mark(encoder, "ink");

        // a click waiting: draw the id pass now
        if let Some(at) = self.pick.take_pending() {
            self.id_pass(encoder, Some(at));
        }

        // gumball on top, with its own depth
        draws += self.widget.draw(encoder, view, &self.targets);

        if let Some(ui) = self.ui.as_ref() {
            ui.draw(encoder, view);
        }

        self.mark(encoder, "end");
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

    /// The first pass; for each plane that crosses a closed solid, the crossings are counted
    /// first and its section caps drawn before the faces, every plane but the last in a pass of its own.
    fn face_passes(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> u32 {
        let (planes, count) = self.cap_planes();
        let size = (self.config.width, self.config.height);
        let mut draws = 0;

        if count > 0 {
            self.clip.prepare_counts(&self.ctx, &self.layouts, size);
        }

        for (k, &plane) in planes[..count.saturating_sub(1)].iter().enumerate() {
            let b = self.frame.binds(&self.objects.group);
            draws += self.clip.encode_count(encoder, &self.arena, &b, plane);
            let mut pass = self
                .targets
                .begin_faces(encoder, view, (k == 0).then_some(clear));

            if k == 0 {
                draws += self.backdrop_list(&mut pass, &b);
            }

            draws += self.clip.draw_cap(&mut pass, &b, plane);
        }

        if count > 0 {
            let b = self.frame.binds(&self.objects.group);
            draws += self
                .clip
                .encode_count(encoder, &self.arena, &b, planes[count - 1]);
            self.mark(encoder, "counts");
        }

        let b = self.frame.binds(&self.objects.group);
        let mut pass = self
            .targets
            .begin_faces(encoder, view, (count <= 1).then_some(clear));

        if count <= 1 {
            draws += self.backdrop_list(&mut pass, &b);
        }

        if count > 0 {
            draws += self.clip.draw_cap(&mut pass, &b, planes[count - 1]);
        }

        draws + self.face_list(&mut pass, &b)
    }

    /// Draws of the backdrop: background and grid.
    fn backdrop_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = self.backdrop.draw_background(pass, b);

        if self.view.show_grid {
            draws += self.backdrop.draw_grid(pass, b);
        }

        draws
    }

    /// Draws of the first pass after the backdrop and caps: faces, clouds.
    fn face_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        // at full opacity the blend returns the face color itself
        let mut draws =
            self.arena
                .draw_faces(pass, b, self.view.opacity >= 1.0, self.clip.count() > 0);
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
        draws += self.solid_outline.draw_combined(pass);
        // selected curves over the outline
        draws += self.segments.draw_selected(pass, &b, false, v.show_lines);
        for lane in &self.registered {
            draws += lane.draw_ink(pass, &b, v);
        }

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

    /// Draw object ids around the cursor for a pick, then copy them out.
    pub(super) fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: Option<(u32, u32)>) {
        // pick ink always tests against current lists
        self.triangle_tile_pass(encoder, true, true);
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

        // which solid each section cap pixel belongs to
        let caps = self.cap_planes().1 > 0;

        if caps {
            self.clip.encode_pick(
                &self.ctx,
                &self.layouts,
                encoder,
                &self.arena,
                &basic,
                (view.w, view.h),
            );
        }

        {
            // caps, faces and clouds over the whole area, halo included
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, view);

            if caps {
                self.clip.draw_cap_ids(&mut pass, &basic);
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
