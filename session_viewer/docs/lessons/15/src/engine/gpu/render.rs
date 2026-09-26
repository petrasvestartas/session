// --8<-- [start:004-encode]
use super::Gpu;
use super::frame::Binds; // register:frame

/// What every pass of one frame shares.
pub struct Frame<'a> {
    pub view: &'a wgpu::TextureView, // the canvas
    pub clear: wgpu::Color,          // background color
    pub tier: u8,                    // drag tier; 0 is full quality; register:perf
}

// `impl Gpu` blocks may sit in any file of the crate: this one adds the frame encoding.
impl Gpu {
    /// Encode one frame into `view`; returns the draw count.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> u32 {
        self.each_pass(|pass, g| pass.prepare(g, encoder)); // register:pass
        let tier = self.performance.drag_tier(); // register:perf
        self.point_pass(encoder); // register:clouds

        let frame = Frame {
            view,
            clear,
            tier,  // register:perf
        };
        // pass 1: background, section caps, faces and clouds write depth
        let mut draws = self.face_passes(encoder, &frame);
        // pass 2: ambient occlusion and the outline masks, each pass in turn
        self.each_pass(|pass, g| draws += pass.after_faces(g, encoder, &frame)); // register:pass
        draws += self.ink_pass(encoder, view); // pass 3: lines, markers, outlines and text; register:ink
        self.pending_pick(encoder); // a click waiting: draw the id pass now; register:pick
        draws
    }

    /// The first pass: each pass's own face passes, then the one the faces draw in.
    fn face_passes(&mut self, encoder: &mut wgpu::CommandEncoder, f: &Frame) -> u32 {
        let mut draws = 0;
        let mut drew = false;
        self.each_pass(|pass, g| draws += pass.before_faces(g, encoder, f, &mut drew)); // register:pass

        let mut pass = self
            .targets
            .begin_faces(encoder, f.view, (!drew).then_some(f.clear));
        let b = self.frame.binds(&self.objects.group); // register:objects

        if !drew { // register:backdrop
            draws += self.backdrop_list(&mut pass, &b);
        }

        for other in &self.passes { // register:pass
            draws += other.in_faces(self, &mut pass, &b);
        }

        draws += self.face_list(&mut pass, &b); // register:meshes
        draws
    }
}
// --8<-- [end:004-encode]

// --8<-- [start:04a-tail]
impl Gpu {

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
        draws += self.splat.draw_resolve(pass, &self.frame.cloud_group); // register:clouds
        draws
    }
}

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

        draws += self.sphere_draws(pass, &b); // register:markers
        draws += self.arena.draw_text(pass, &basic);
        draws += self.dot_draws(pass, &b); // register:markers
        draws += self.control_net.draw_ribbons(pass, &b); // register:shell
        draws += self.controls.draw_dots(pass, &b); // register:shell
        draws += self.text.draw(pass); // register:text
        draws
    }
}

impl Gpu {
    /// The vertex markers, when mesh edges and markers are shown.
    fn sphere_draws(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let v = &self.view;

        if v.show_mesh_edges && v.markers {
            self.glyphs.draw_spheres(pass, b)
        } else {
            0
        }
    }

    /// The point dots, when points are shown.
    fn dot_draws(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.view.show_points {
            self.glyphs.draw_dots(pass, b)
        } else {
            0
        }
    }
}

use super::splat::RecordCx;

impl Gpu {
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
}

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

impl Gpu {
    /// Build the triangle tables the frame reads; true when a slow drag tests ink against the fitted planes alone.
    fn tile_passes(&mut self, encoder: &mut wgpu::CommandEncoder, tier: u8) -> bool {
        let (projection, lists) = self.tile_readers();

        // nothing reads the tables: free them, the ink bind group follows
        if !projection && self.arena.tiles.release_unread(&self.ctx) {
            self.rebind_ink();
        }

        // a slow drag tests ink against the fitted planes alone; the lists return when it ends
        let rough = lists && tier >= 1;
        self.triangle_tile_pass(encoder, projection, lists && !rough);

        if rough {
            self.arena.tiles.drop_lists(encoder);
        }

        rough
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

    /// True when this pick draws strokes, the only ids that read the triangle tables.
    fn pick_reads_tiles(&self) -> bool {
        let v = &self.view;
        let pipes = v.show_mesh_edges && self.live_pipes() > 0;
        let ribbons = v.show_lines && self.live_ribbons() > 0;
        let lanes = self.registered.iter().any(|lane| lane.reads_tiles(v));
        !self.pick.source_query() && pick_strokes(self.pick.mode, pipes, ribbons, lanes)
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
            self.arena.triangle_count(),
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
}

/// Does a pick in `mode` draw strokes that read the tables: mesh edges, lines, or a lane's strokes?
fn pick_strokes(mode: PickMode, pipes: bool, ribbons: bool, lanes: bool) -> bool {
    lanes
        || match mode {
            PickMode::Edge | PickMode::Component => pipes,
            PickMode::Object => pipes || ribbons,
            PickMode::Controls { .. } => false,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A face-only scene picks without the tables; every stroke the mode draws needs them.
    #[test]
    fn only_stroke_picks_read_the_tables() {
        let controls = PickMode::Controls {
            parent: 0,
            cloud: false,
        };

        for mode in [
            PickMode::Object,
            PickMode::Edge,
            PickMode::Component,
            controls,
        ] {
            assert!(
                !pick_strokes(mode, false, false, false),
                "{mode:?} on faces alone"
            );
            assert!(
                pick_strokes(mode, false, false, true),
                "{mode:?} with a lane's strokes"
            );
        }

        assert!(pick_strokes(PickMode::Object, true, false, false));
        assert!(pick_strokes(PickMode::Object, false, true, false));
        assert!(pick_strokes(PickMode::Edge, true, false, false));
        assert!(pick_strokes(PickMode::Component, true, false, false));
        assert!(
            !pick_strokes(PickMode::Edge, false, true, false),
            "edge picks skip lines"
        );
        assert!(
            !pick_strokes(controls, true, true, false),
            "control dots are discs"
        );
    }
}

impl Gpu {
    /// The egui panels on top.
    fn draw_panels(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        if let Some(ui) = self.ui.as_ref() {
            ui.draw(encoder, view);
        }
    }
}

impl Gpu {
    /// Timestamp the GPU here when a bench installed a pass timer.
    pub(super) fn mark(&mut self, encoder: &mut wgpu::CommandEncoder, label: &'static str) {
        if let Some(timer) = self.timer.as_mut() {
            timer.mark(encoder, label);
        }
    }
}
// --8<-- [end:04a-tail]
