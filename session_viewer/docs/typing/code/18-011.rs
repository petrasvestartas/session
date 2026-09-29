
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
