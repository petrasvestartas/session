// --8<-- [start:step-15a]
use super::State;
#[cfg(target_arch = "wasm32")]
use super::{
    ControlId, FACING_UNKNOWN, GlyphPoint, GlyphRows, Pick, PickMode, SelectionMode,
    render_position,
};

impl State {
    /// True while a cloud query waits for the GPU pick.
    pub(super) fn cloud_query_awaiting_gpu(&self) -> bool {
        match &self.cloud_query {
            Some(query) => query.awaiting_gpu,
            None => false,
        }
    }

    /// The streamed cloud slot of row `parent`.
    pub(super) fn streamed_slot(&self, parent: u32) -> Option<usize> {
        for (slot, cloud) in self.scene.streamed.iter().enumerate() {
            if cloud.row == parent {
                return Some(slot);
            }
        }

        None
    }

    /// Drop the cloud query in flight.
    pub(super) fn cancel_cloud_query(&mut self) {
        if self.cloud_query.take().is_some() {
            self.gpu.pick.cancel();
            self.upload_controls();
            self.status("Point query cancelled because the view or selection changed");
        }
    }

// --8<-- [end:step-15a]
    // --8<-- [start:step-15b]
    /// Start a click query on a streamed cloud's source points.
    #[cfg(target_arch = "wasm32")]
    pub(super) fn start_cloud_query(&mut self, x: u32, y: u32) -> bool {
        use crate::app::cloud_query::{Query, QueryView};
        // only in cloud control mode
        let SelectionMode::Controls {
            parent,
            cloud: true,
            ..
        } = self.selection
        else {
            return false;
        };
        let Some(slot) = self.streamed_slot(parent) else {
            return false;
        };
        self.gpu.pick.cancel();
        self.query_generation = self.query_generation.wrapping_add(1); // new query id
        let cloud = &self.scene.streamed[slot];
        // the matrix that puts a source point on screen
        let projection = self
            .camera
            .view_proj_anchored(self.aspect(), &session_rust::Point::new(0.0, 0.0, 0.0));
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0]; // device pixels per CSS pixel
        let view = QueryView {
            matrix: &projection * &cloud.place,
            size: [
                f64::from(self.gpu.config.width),
                f64::from(self.gpu.config.height),
            ],
            at: [x, y],                                                       // the click
            radius: (self.selection_radius_css * scale).ceil().clamp(1.0, 128.0) + 3.5 * scale, // click tolerance plus a dot
        };
        self.cloud_query = Some(Query::new(self.query_generation, cloud, view));
        self.gpu.pick.start_source_query();
        self.advance_cloud_query();
        true
    }

    /// Fetch the next page of source points, or finish with the best one.
    #[cfg(target_arch = "wasm32")]
    fn advance_cloud_query(&mut self) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };
        query.awaiting_gpu = false;
        query.candidates.clear();

        // more pages to check
        if let Some(page) = query.next_page() {
            let progress = format!(
                "Checking source points: {} / {} (display LOD remains bounded)",
                query.checked, query.total
            );
            crate::app::cloud_query::fetch_page(query, page);
            self.status(&progress);
        } else if let Some(best) = query.best {
            // all pages checked: ask for the winner's source id
            crate::app::cloud_query::resolve_id(query, best);
            self.status("All eligible source points checked; resolving original point ID…");
        } else {
            self.cloud_query = None;
            self.gpu.pick.cancel();
            self.status("No visible source point in the selection window");
        }

        self.upload_controls();
    }

// --8<-- [end:step-15b]
    // --8<-- [start:step-15c]
    /// One page of source points arrived: draw them for the GPU to pick.
    #[cfg(target_arch = "wasm32")]
    pub fn cloud_query_batch(&mut self, batch: crate::app::cloud_query::Batch) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };

        // an old query's answer
        if query.id != batch.query || query.cancelled.get() {
            return;
        }

        let (candidates, revision) = match batch.result {
            Ok(result) => result,
            Err(error) => {
                self.cloud_query = None;
                self.gpu.pick.cancel();
                self.upload_controls();
                self.status(&format!("Point query failed: {error}"));
                return;
            }
        };
        query.checked += batch.count;
        query.revision = revision;

        // nothing near the click on this page
        if candidates.is_empty() {
            self.advance_cloud_query();
            return;
        }

        query.candidates = candidates;
        query.awaiting_gpu = true;
        let parent = query.parent;
        let at = query.view.at;
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0];
        let query = self.cloud_query.as_ref().unwrap();
        let mut glyphs = GlyphRows::default();

        // one dot per candidate
        for candidate in &query.candidates {
            glyphs.dots.push(GlyphPoint {
                center: render_position(candidate.position),
                radius: -3.5 * scale as f32,           // pixel size
                color: [0.15, 0.35, 0.9, 1.0],         // blue
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [candidate.local, FACING_UNKNOWN], // carries the point's row
            });
        }

        self.gpu.controls.reset();
        self.gpu
            .controls
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
        // a pick frame over the dots, never shown
        self.requested = PickMode::Controls {
            parent,
            cloud: false,
        };
        self.gpu
            .pick
            .configure(self.requested, self.selection_radius_css, scale);
        self.gpu.pick.request(at[0], at[1]);
        self.needs_frame = true;
    }

// --8<-- [end:step-15c]
    // --8<-- [start:step-15d]
    /// The GPU picked one of this page's dots: keep it, go on.
    #[cfg(target_arch = "wasm32")]
    pub(super) fn apply_cloud_query_pick(&mut self, pick: Option<Pick>) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };
        // the nearest so far, across pages
        query.best = match pick {
            Some(pick) if pick.row == query.parent && pick.sub < query.fields.count => {
                Some(pick.sub)
            }
            _ => None,
        };
        self.advance_cloud_query();
    }

    /// The winner's source id and position arrived: select it.
    #[cfg(target_arch = "wasm32")]
    pub fn cloud_query_resolved(&mut self, resolved: crate::app::cloud_query::Resolved) {
        let Some(query) = self.cloud_query.as_ref() else {
            return;
        };

        // an old query's answer
        if query.id != resolved.query || query.cancelled.get() {
            return;
        }

        let query = self.cloud_query.take().unwrap();
        let (source, position) = match resolved.result {
            Ok(result) => result,
            Err(error) => {
                self.gpu.pick.cancel();
                self.upload_controls();
                self.status(&format!("Point query failed: {error}"));
                return;
            }
        };
        let Some(best) = query.best else { return };
        let id = ControlId::Point(source);
        self.selection = SelectionMode::Controls {
            parent: query.parent,
            selected: Some(id),
            cloud: true,
        };
        self.controls.points = vec![crate::app::selection::Control { id, position }]; // the one dot shown
        self.gpu.splat.set_point(None);
        self.upload_controls();
        self.status(&format!(
            "Selected source point {source} (row {}); all {} eligible points checked",
            best, query.total
        ));
        self.touch();
    }
}
// --8<-- [end:step-15d]
