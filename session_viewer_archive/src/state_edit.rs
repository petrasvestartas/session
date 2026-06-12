//! Sub-object (control-point / vertex) editing — the F10 edit mode.
//!
//! Builds an overlay of draggable nodes (control points / mesh vertices) and edges
//! (control polygon / mesh edges) for the targeted object, lets the user pick them
//! with Ctrl+Shift+LMB, and reuses the gumball to translate/rotate/scale the picked
//! nodes. Commits write the moved positions back into the kernel geometry, rebuild
//! the GPU mirror, and push an `EditGeom` undo action.

use crate::State;
use crate::camera::ProjMode;
use crate::edit_state::{f32p, EditEdge, EditKind, EditNode, NodeAddr};
use crate::gpu_adapters::{
    color_to_rgba_f32, mesh_crease_edges_to_segments, mesh_edges_to_segments,
    mesh_naked_edges_to_segments, mesh_to_vertices, mesh_vertex_glyphs, polyline_endpoint_glyphs,
    polyline_to_segments,
};
use crate::gpu_session::{CylinderSegment, GlyphPoint, InstanceData};
use crate::gumball::Gumball;
use crate::pick::{self, Ray};
use crate::undo_state::{EditSnapshot, UndoAction};
use session_rust::session::Geometry;
use session_rust::Point;

impl State {
    // ── Mode entry / exit ─────────────────────────────────────────────────────

    /// F10: toggle control-point edit mode. On enter, the first editable object in
    /// the current selection becomes the target; with none selected we arm the mode
    /// so the next plain click chooses a target.
    pub(crate) fn toggle_edit_mode(&mut self) {
        if self.edit.active {
            self.exit_edit_mode();
            return;
        }
        let target = self.scene.selected_guids.iter()
            .find(|g| self.editable_kind(g).is_some())
            .cloned();
        if let Some(g) = target {
            self.enter_edit_on(&g);
        } else {
            self.edit.active = true;
            self.edit.target = None;
            self.edit.kind = None;
            self.edit.clear_overlay();
            self.gb.gumball = None;
        }
    }

    /// Start editing `guid`: highlight it alone, build its control-point overlay,
    /// and hide the whole-object gumball (it returns once sub-objects are picked).
    pub(crate) fn enter_edit_on(&mut self, guid: &str) {
        let kind = match self.editable_kind(guid) { Some(k) => k, None => return };
        log::info!("[edit] enter_edit_on guid={guid} kind={kind:?}");
        let prev: Vec<String> = self.scene.selected_guids.drain().collect();
        for p in &prev {
            self.scene.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
        }
        // In edit mode the control-point overlay IS the selection cue — keep the object
        // at its default (un-highlighted) look so it stays visible under the cage.
        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
        self.scene.selected_guids.insert(guid.to_string());
        self.edit.active = true;
        self.edit.edge_mode = false;
        self.edit.use_edit_points = false; // F10 edits raw control points; edit points are the Ctrl+Shift+LMB path
        self.edit.target = Some(guid.to_string());
        self.edit.kind = Some(kind);
        self.edit.selected_nodes.clear();
        self.edit.selected_edges.clear();
        self.edit.before_snapshot = None;
        self.edit.drag_start.clear();
        self.rebuild_edit_overlay();
        self.gb.gumball = None;
        self.gb.gumball_press = None;
        self.gb.gumball_input = None;
        self.scene.reveal_in_tree = true;
    }

    /// Leave edit mode. F10 control-point editing restores the whole-object selection +
    /// gumball; a transient edge-move session (Ctrl+Shift+LMB) leaves NOTHING selected —
    /// it was just "move this edge", not "select this object".
    pub(crate) fn exit_edit_mode(&mut self) {
        // Tear down any in-flight gumball interaction so a mid-drag exit can't leak a
        // press/popup into the restored whole-object gumball.
        self.gb.gumball_press = None;
        self.gb.gumball_input = None;
        let was_edge = self.edit.edge_mode;
        self.edit.active = false;
        self.edit.edge_mode = false;
        self.edit.target = None;
        self.edit.kind = None;
        self.edit.clear_overlay();
        let sel: Vec<String> = self.scene.selected_guids.iter().cloned().collect();
        if was_edge {
            for g in &sel {
                self.scene.gpu_session.set_flag(g, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
            }
            self.scene.selected_guids.clear();
            self.gb.gumball = None;
        } else {
            // Restore the normal selection highlight that F10 edit mode suppressed.
            for g in &sel {
                self.scene.gpu_session.set_flag(g, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            }
            if !self.scene.selected_guids.is_empty() {
                let origin = self.selected_centroid();
                self.gb.gumball = Some(Gumball::new(origin));
            } else {
                self.gb.gumball = None;
            }
        }
    }

    /// Which edit kind a guid maps to, or None if it has no editable sub-objects.
    pub(crate) fn editable_kind(&self, guid: &str) -> Option<EditKind> {
        if let Some(g) = self.scene.session.lookup.get(guid) {
            return match g {
                Geometry::Mesh(_) => Some(EditKind::Mesh),
                Geometry::Polyline(_) => Some(EditKind::Polyline),
                Geometry::BRep(_) => Some(EditKind::BRep),
                _ => None,
            };
        }
        if self.scene.session.objects.nurbssurfaces.iter().any(|n| n.guid() == guid) {
            return Some(EditKind::NurbsSurface);
        }
        // Trimmed surfaces edit their underlying base surface the same way (seam/boundary
        // edit-points), then re-tessellate the trim — so they report as NurbsSurface here.
        if self.scene.session.objects.nurbssurfacetrimmeds.iter().any(|n| n.guid() == guid) {
            return Some(EditKind::NurbsSurface);
        }
        if self.scene.session.objects.nurbscurves.iter().any(|n| n.guid() == guid) {
            return Some(EditKind::NurbsCurve);
        }
        None
    }

    /// The editable base `NurbsSurface` for a guid — a standalone surface OR the `m_surface` of a
    /// `NurbsSurfaceTrimmed`. Lets the edit-point system reshape trimmed seams/boundaries too.
    pub(crate) fn edit_surface(&self, guid: &str) -> Option<&session_rust::NurbsSurface> {
        if let Some(s) = self.scene.session.objects.nurbssurfaces.iter().find(|x| x.guid() == guid) {
            return Some(s);
        }
        self.scene.session.objects.nurbssurfacetrimmeds.iter()
            .find(|t| t.guid() == guid).map(|t| &t.m_surface)
    }

    pub(crate) fn edit_surface_mut(&mut self, guid: &str) -> Option<&mut session_rust::NurbsSurface> {
        if self.scene.session.objects.nurbssurfaces.iter().any(|x| x.guid() == guid) {
            return self.scene.session.objects.nurbssurfaces.iter_mut().find(|x| x.guid() == guid);
        }
        self.scene.session.objects.nurbssurfacetrimmeds.iter_mut()
            .find(|t| t.guid() == guid).map(|t| &mut t.m_surface)
    }

    /// Re-upload a surface guid to the GPU after an edit — using the trimmed path if it is a
    /// trimmed surface (so the trim re-tessellates against the reshaped base surface).
    pub(crate) fn reupload_edit_surface(&mut self, guid: &str) {
        if let Some(ts) = self.scene.session.objects.nurbssurfacetrimmeds.iter()
            .find(|t| t.guid() == guid).cloned()
        {
            self.scene.gpu_session.add_nurbssurfacetrimmed(&ts, &self.gpu.device, &self.gpu.queue);
        } else if let Some(s) = self.scene.session.objects.nurbssurfaces.iter()
            .find(|n| n.guid() == guid).cloned()
        {
            self.scene.gpu_session.add_nurbssurface(&s, &self.gpu.device, &self.gpu.queue);
        }
    }

    // ── Overlay construction ──────────────────────────────────────────────────

    /// Rebuild the node/edge overlay from the target geometry, preserving the
    /// current sub-selection by index (topology is stable across an edit).
    pub(crate) fn rebuild_edit_overlay(&mut self) {
        let keep_nodes = self.edit.selected_nodes.clone();
        let keep_edges = self.edit.selected_edges.clone();
        match self.build_overlay_data() {
            Some((kind, nodes, edges)) => {
                let nn = nodes.len();
                let ne = edges.len();
                self.edit.kind = Some(kind);
                self.edit.nodes = nodes;
                self.edit.edges = edges;
                self.edit.selected_nodes = keep_nodes.into_iter().filter(|&i| i < nn).collect();
                self.edit.selected_edges = keep_edges.into_iter().filter(|&i| i < ne).collect();
                self.edit.recompute_centroid();
            }
            None => {
                self.edit.clear_overlay();
                self.edit.kind = None;
            }
        }
    }

    /// Read the target geometry and produce its nodes + edges in world space.
    fn build_overlay_data(&self) -> Option<(EditKind, Vec<EditNode>, Vec<EditEdge>)> {
        let guid = self.edit.target.as_ref()?;

        if let Some(geom) = self.scene.session.lookup.get(guid) {
            match geom {
                Geometry::Mesh(m) => {
                    let mut nodes = Vec::new();
                    let mut keys: Vec<usize> = m.vertex.keys().copied().collect();
                    keys.sort_unstable();
                    for k in keys {
                        let v = &m.vertex[&k];
                        nodes.push(EditNode { world: [v.x, v.y, v.z], addr: NodeAddr::MeshVertex(k) });
                    }
                    if nodes.is_empty() { return None; }
                    // F10 shows control points only — no cage lines (Part 2).
                    return Some((EditKind::Mesh, nodes, Vec::new()));
                }
                Geometry::Polyline(pl) => {
                    let pts = pl.get_points();
                    let mut nodes = Vec::with_capacity(pts.len());
                    for (i, p) in pts.iter().enumerate() {
                        nodes.push(EditNode { world: [p[0], p[1], p[2]], addr: NodeAddr::PolyPoint(i) });
                    }
                    if nodes.is_empty() { return None; }
                    return Some((EditKind::Polyline, nodes, Vec::new()));
                }
                Geometry::BRep(b) => {
                    let cols = b.xform.to_cols();
                    let mut nodes = Vec::new();
                    for (si, srf) in b.m_surfaces.iter().enumerate() {
                        let ni = srf.cv_count_dir(Some(0));
                        let nj = srf.cv_count_dir(Some(1));
                        for i in 0..ni {
                            for j in 0..nj {
                                let p = srf.get_cv(i, j).unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                                let w = mat_apply_f64(&cols, [p[0], p[1], p[2]]);
                                nodes.push(EditNode { world: w, addr: NodeAddr::BRepCv(si, i, j) });
                            }
                        }
                    }
                    if nodes.is_empty() { return None; }
                    // Control points only — no control-polygon cage lines (you edit the CVs,
                    // not the cage). The BRep's own edges still render via the normal pipeline.
                    return Some((EditKind::BRep, nodes, Vec::new()));
                }
                _ => return None,
            }
        }

        if let Some(srf) = self.edit_surface(guid) {
            // NurbsSurface moves matrix-only: its CVs are LOCAL and the pose lives in the
            // GPU instance model. Apply that model so the overlay nodes are in world space
            // (like the BRep arm above applies b.xform). flush_edit_to_geometry then writes
            // world back into the CVs, baking the pose.
            let model = self.scene.gpu_session.pick.instance_id(guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map(|inst| inst.model);
            // Edit-point boundary session: handles are the picked boundary iso-curve's edit
            // points (on the surface). Dragging one refits the boundary CV row and the surface
            // follows (Phase 2). Falls through to the full CV grid when not in edit-point mode.
            if self.edit.use_edit_points {
                if let Some(b) = self.edit.editpt_boundary {
                    if let Some(iso) = boundary_iso_curve(srf, b) {
                        let eps = crate::edit_points::edit_points(&iso);
                        let mut nodes = Vec::with_capacity(eps.len());
                        for (k, p) in eps.iter().enumerate() {
                            let lp = [p[0], p[1], p[2]];
                            let w = match &model { Some(m) => mat_apply_mixed(m, lp), None => lp };
                            nodes.push(EditNode { world: w, addr: NodeAddr::SurfaceEditPt(b, k) });
                        }
                        if nodes.is_empty() { return None; }
                        let edges = (0..nodes.len().saturating_sub(1))
                            .map(|i| EditEdge { a: i, b: i + 1 })
                            .collect();
                        return Some((EditKind::NurbsSurface, nodes, edges));
                    }
                }
                return None;
            }
            let ni = srf.cv_count_dir(Some(0));
            let nj = srf.cv_count_dir(Some(1));
            let mut nodes = Vec::with_capacity(ni * nj);
            for i in 0..ni {
                for j in 0..nj {
                    let p = srf.get_cv(i, j).unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                    let lp = [p[0], p[1], p[2]];
                    let w = match &model {
                        Some(m) => mat_apply_mixed(m, lp),
                        None => lp,
                    };
                    nodes.push(EditNode { world: w, addr: NodeAddr::SurfaceCv(i, j) });
                }
            }
            if nodes.is_empty() { return None; }
            log::info!("[edit] build_overlay NurbsSurface guid={guid} cv_grid={ni}x{nj} model_some={}", model.is_some());
            return Some((EditKind::NurbsSurface, nodes, Vec::new()));
        }

        if let Some(nc) = self.scene.session.objects.nurbscurves.iter().find(|n| n.guid() == guid) {
            if self.edit.use_edit_points {
                // Edit points lie ON the curve (one per CV); the overlay polyline through
                // consecutive edit points hugs the curve, so the cage doubles as the handle row.
                let eps = crate::edit_points::edit_points(nc);
                let mut nodes = Vec::with_capacity(eps.len());
                for (k, p) in eps.iter().enumerate() {
                    nodes.push(EditNode { world: [p[0], p[1], p[2]], addr: NodeAddr::CurveEditPt(k) });
                }
                if nodes.is_empty() { return None; }
                let edges = (0..nodes.len().saturating_sub(1))
                    .map(|i| EditEdge { a: i, b: i + 1 })
                    .collect();
                return Some((EditKind::NurbsCurve, nodes, edges));
            }
            let n = nc.cv_count();
            let mut nodes = Vec::with_capacity(n);
            for i in 0..n {
                let p = nc.get_cv(i).unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                nodes.push(EditNode { world: [p[0], p[1], p[2]], addr: NodeAddr::CurveCv(i) });
            }
            if nodes.is_empty() { return None; }
            return Some((EditKind::NurbsCurve, nodes, Vec::new()));
        }

        None
    }

    /// Set the gumball at the sub-selection centroid, or hide it when nothing is picked.
    pub(crate) fn update_edit_gumball(&mut self) {
        match self.edit.selection_centroid() {
            Some(c) => match &mut self.gb.gumball {
                Some(gb) => gb.set_origin(c),
                None => self.gb.gumball = Some(Gumball::new(c)),
            },
            None => self.gb.gumball = None,
        }
    }

    // ── Picking ───────────────────────────────────────────────────────────────

    /// Edit-mode pick dispatch (called from `process_pick` when edit mode is on).
    pub(crate) fn process_pick_edit(&mut self, ray: Ray, cx: f32, cy: f32) {
        // 1. Grab the gumball to move the current sub-selection.
        let gumball_hit = self.gb.gumball.as_ref()
            .and_then(|gb| gb.hit_test(ray, self.gb.gumball_scale));
        if let Some(handle) = gumball_hit {
            let set = self.edit.move_node_set();
            if !set.is_empty() {
                self.gb.gumball_input = None;
                self.gb.gumball_press = None;
                self.gb.drag_origins.clear();
                self.gb.drag_geom_snapshots.clear();
                self.gb.drag_nurbs_snapshots.clear();
                self.edit.before_snapshot = self.snapshot_target();
                self.edit.drag_start = set.iter().map(|&i| (i, self.edit.nodes[i].world)).collect();
                self.edit.clear_live(); // re-capture frozen topology for this drag
                if self.edit.use_edit_points {
                    self.capture_editpt_refit();
                }
                // Freeze the edge curve; during the drag it's just this base mapped by the
                // gumball delta — no per-move geometry re-evaluation (keeps the drag fast).
                self.edit.edge_polyline_base = self.edit.edge_polyline.clone();
                if self.scene.lmb_down {
                    self.gb.gumball_press = Some((handle, cx as f64, cy as f64));
                } else {
                    self.open_gumball_input(handle, cx as f64, cy as f64);
                }
                return;
            }
        }

        // 2. Ctrl+Shift+LMB → in an edge session re-pick a boundary edge; in F10
        // control-point mode toggle the nearest node/edge of the current target.
        if self.scene.ctrl_down && self.scene.shift_down {
            if self.edit.edge_mode {
                self.process_pick_edge(ray);
            } else {
                self.pick_subobject(ray);
            }
            return;
        }

        // A plain LMB in a transient edge session (entered without F10) ends it.
        if self.edit.edge_mode {
            self.exit_edit_mode();
            return;
        }

        // 3. Plain LMB on the object being edited: select/toggle the control point
        // (or edge) under the cursor directly — no modifier needed. Only fall through
        // to switching the target / clearing when no sub-object is under the cursor.
        if self.edit.target.is_some() && self.pick_subobject(ray) {
            return;
        }

        // 4. Plain LMB → switch the edit target to the clicked editable object.
        if let Some(g) = self.closest_object_under_ray(ray) {
            // Clicking the body of the object already being edited keeps the
            // current sub-selection (use Ctrl+Shift to change it, or the gumball).
            if self.edit.target.as_deref() == Some(g.as_str()) {
                return;
            }
            if self.editable_kind(&g).is_some() {
                self.enter_edit_on(&g);
                return;
            }
        }
        // Clicked empty space: drop the sub-selection but stay in edit mode.
        self.edit.selected_nodes.clear();
        self.edit.selected_edges.clear();
        self.gb.gumball = None;
    }

    /// Toggle the nearest control point (preferred) or edge under the ray.
    /// Returns true if a node or edge was hit (and its selection toggled).
    pub(crate) fn pick_subobject(&mut self, ray: Ray) -> bool {
        if self.edit.nodes.is_empty() { return false; }
        let r = (5.0 * self.edit.handle_scale).max(1.0);
        let node_tol = r * 2.0;
        let mut best_t = f32::MAX;
        let mut best_node: Option<usize> = None;
        for (i, n) in self.edit.nodes.iter().enumerate() {
            if let Some(t) = ray_sphere_t(ray, f32p(n.world), node_tol) {
                if t < best_t { best_t = t; best_node = Some(i); }
            }
        }
        if let Some(i) = best_node {
            if !self.edit.selected_nodes.remove(&i) { self.edit.selected_nodes.insert(i); }
            self.update_edit_gumball();
            return true;
        }
        let edge_tol = r * 1.5;
        let mut best_d = f32::MAX;
        let mut best_edge: Option<usize> = None;
        for (ei, e) in self.edit.edges.iter().enumerate() {
            let a = f32p(self.edit.nodes[e.a].world);
            let b = f32p(self.edit.nodes[e.b].world);
            if let Some(d) = ray_seg_dist(ray, a, b) {
                if d < edge_tol && d < best_d { best_d = d; best_edge = Some(ei); }
            }
        }
        if let Some(ei) = best_edge {
            if !self.edit.selected_edges.remove(&ei) { self.edit.selected_edges.insert(ei); }
            self.update_edit_gumball();
            return true;
        }
        false
    }

    /// Begin a Move of the active edit selection (edge / CVs): freeze the drag baseline like a
    /// gumball grab, so the Move tool can translate it via `apply_edit_delta`. Returns false when
    /// nothing is selected.
    pub(crate) fn begin_edit_move(&mut self) -> bool {
        let set = self.edit.move_node_set();
        if set.is_empty() { return false; }
        self.edit.before_snapshot = self.snapshot_target();
        self.edit.drag_start = set.iter().map(|&i| (i, self.edit.nodes[i].world)).collect();
        self.edit.clear_live();
        if self.edit.use_edit_points { self.capture_editpt_refit(); }
        self.edit.edge_polyline_base = self.edit.edge_polyline.clone();
        true
    }

    /// Cancel an in-progress edit Move: restore the selection to its start and drop the drag.
    pub(crate) fn abort_edit_move(&mut self) {
        const IDENT: [[f32; 4]; 4] = [[1.0,0.0,0.0,0.0],[0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0],[0.0,0.0,0.0,1.0]];
        if !self.edit.drag_start.is_empty() { self.apply_edit_delta(&IDENT); }
        self.edit.drag_start.clear();
        self.edit.clear_live();
        self.rebuild_edit_overlay();
        self.update_edit_gumball();
    }

    /// Select the WHOLE edit-point set (every handle + edge) of the current overlay — the full
    /// boundary edge / curve the user clicked — and put the gumball at its centroid so a drag
    /// transforms the entire edge together. Returns false when there are no handles (caller
    /// can fall back to the raw-CV path).
    fn select_all_editpoints(&mut self) -> bool {
        if self.edit.nodes.is_empty() { return false; }
        self.edit.selected_nodes = (0..self.edit.nodes.len()).collect();
        self.edit.selected_edges = (0..self.edit.edges.len()).collect();
        self.update_edit_gumball();
        true
    }

    /// Resample the SELECTED edge densely along the actual geometry into `edge_polyline`, so the
    /// highlighted edge renders as the true curve (a circle), not the chorded control polygon /
    /// edit-point polygon. Cheap (one curve, 64 samples); call after select and after each deform.
    fn refresh_edit_edge_curve(&mut self) {
        self.edit.edge_polyline.clear();
        let guid = match &self.edit.target { Some(g) => g.clone(), None => return };
        const N: usize = 160;
        // Surface boundary edit points → sample the boundary iso-curve (world via instance model).
        if self.edit.use_edit_points {
            if let Some(b) = self.edit.editpt_boundary {
                let model = self.scene.gpu_session.pick.instance_id(&guid)
                    .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                    .map(|inst| inst.model);
                if let Some(srf) = self.edit_surface(&guid) {
                    if let Some(iso) = boundary_iso_curve(srf, b) {
                        let (t0, t1) = iso.domain();
                        for k in 0..=N {
                            let t = t0 + (t1 - t0) * (k as f64 / N as f64);
                            let p = iso.point_at(t);
                            let lp = [p[0], p[1], p[2]];
                            let w = match &model { Some(m) => mat_apply_mixed(m, lp), None => lp };
                            self.edit.edge_polyline.push([w[0] as f32, w[1] as f32, w[2] as f32]);
                        }
                    }
                }
            }
            return;
        }
        // BRep edge → sample the smooth trim-on-surface (the kernel's 3D edge curve is only a
        // coarse degree-1 polyline, so re-evaluating the rational 2D trim on the surface gives a
        // much rounder circle). World via the BRep xform.
        if let Some(ei) = self.edit.brep_edge {
            if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get(&guid) {
                if ei < b.m_topology_edges.len() {
                    if let Some(&ti) = b.m_topology_edges[ei].trim_indices.first() {
                        let trim = &b.m_trims[ti as usize];
                        if trim.curve_2d_index >= 0 {
                            let c2d = &b.m_curves_2d[trim.curve_2d_index as usize];
                            let si = b.m_faces[b.m_loops[trim.loop_index as usize].face_index as usize].surface_index as usize;
                            let srf = &b.m_surfaces[si];
                            let cols = brep_xform_f32(&b.xform);
                            let (t0, t1) = c2d.domain();
                            for k in 0..=N {
                                let t = t0 + (t1 - t0) * (k as f64 / N as f64);
                                let uv = c2d.point_at(t);
                                if let Some(p) = srf.point_at(uv[0], uv[1]) {
                                    self.edit.edge_polyline.push(xf_pt(&cols, &p));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Capture the ΔE → ΔP refit context at drag start for an edit-point drag: the affected
    /// control points (Euclidean start position + weight) and, per moved edit point, the
    /// matching column of `R⁻¹`. `R` is frozen here and reused for the whole drag.
    fn capture_editpt_refit(&mut self) {
        self.edit.editpt_cv.clear();
        self.edit.editpt_cols.clear();
        let guid = match &self.edit.target { Some(g) => g.clone(), None => return };

        // ── Standalone curve: all CVs, columns over the whole curve. ──
        if let Some(nc) = self.scene.session.objects.nurbscurves.iter().find(|n| n.guid() == guid).cloned() {
            let ncv = nc.cv_count();
            for i in 0..ncv {
                let (x, y, z, w) = nc.get_cv_4d(i).unwrap_or((0.0, 0.0, 0.0, 1.0));
                let w = if w == 0.0 { 1.0 } else { w };
                self.edit.editpt_cv.push((NodeAddr::CurveCv(i), [x / w, y / w, z / w], w));
            }
            for &(idx, _) in &self.edit.drag_start {
                let k = match self.edit.nodes[idx].addr { NodeAddr::CurveEditPt(k) => k, _ => usize::MAX };
                let col = if k == usize::MAX { vec![0.0; ncv] }
                    else { crate::edit_points::inverse_collocation_column(&nc, k).unwrap_or_else(|| vec![0.0; ncv]) };
                self.edit.editpt_cols.push(col);
            }
            return;
        }

        // ── Surface boundary: the boundary CV row, columns over the iso-curve. ──
        let boundary = match self.edit.editpt_boundary { Some(b) => b, None => return };
        let srf = match self.edit_surface(&guid).cloned() {
            Some(s) => s, None => return,
        };
        let iso = match boundary_iso_curve(&srf, boundary) { Some(c) => c, None => return };
        let model = self.scene.gpu_session.pick.instance_id(&guid)
            .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
            .map(|inst| inst.model);
        // Boundary CV row, in the iso-curve's CV order, with world base position + weight.
        let row: Vec<(usize, usize)> = surface_boundary_addrs(&srf, boundary, None).iter()
            .filter_map(|a| if let NodeAddr::SurfaceCv(i, j) = a { Some((*i, *j)) } else { None })
            .collect();
        for &(i, j) in &row {
            let (x, y, z, w) = srf.get_cv_4d(i, j).unwrap_or((0.0, 0.0, 0.0, 1.0));
            let w = if w == 0.0 { 1.0 } else { w };
            let lp = [x / w, y / w, z / w];
            let base = match &model { Some(m) => mat_apply_mixed(m, lp), None => lp };
            self.edit.editpt_cv.push((NodeAddr::SurfaceCv(i, j), base, w));
        }
        // Opposite seam column: surface CVs coincident with a row CV but not in the row, welded to
        // the matching row CV's delta so a closed surface stays closed when its seam is moved.
        self.edit.editpt_mirror.clear();
        let local_of = |i: usize, j: usize| {
            let (x, y, z, w) = srf.get_cv_4d(i, j).unwrap_or((0.0, 0.0, 0.0, 1.0));
            let w = if w == 0.0 { 1.0 } else { w };
            [x / w, y / w, z / w]
        };
        let row_local: Vec<[f64; 3]> = row.iter().map(|&(i, j)| local_of(i, j)).collect();
        let (sni, snj) = (srf.cv_count_dir(Some(0)), srf.cv_count_dir(Some(1)));
        for i in 0..sni {
            for j in 0..snj {
                if row.contains(&(i, j)) { continue; }
                let p = local_of(i, j);
                for (a, rp) in row_local.iter().enumerate() {
                    if (p[0]-rp[0]).powi(2)+(p[1]-rp[1]).powi(2)+(p[2]-rp[2]).powi(2) < 1e-6 {
                        self.edit.editpt_mirror.push(((i, j), a));
                        break;
                    }
                }
            }
        }
        let ncv = iso.cv_count();
        for &(idx, _) in &self.edit.drag_start {
            let k = match self.edit.nodes[idx].addr { NodeAddr::SurfaceEditPt(_, k) => k, _ => usize::MAX };
            let col = if k == usize::MAX { vec![0.0; ncv] }
                else { crate::edit_points::inverse_collocation_column(&iso, k).unwrap_or_else(|| vec![0.0; ncv]) };
            self.edit.editpt_cols.push(col);
        }
        // Basis-weight FMA capture for the row CVs (the "moved set"), so the frozen
        // tessellation deforms with multiply-adds — same machinery as the CV-drag path.
        let mut mesh = match self.scene.gpu_session.nurbs_pick_meshes.get(&guid) {
            Some((m, _xf)) => m.clone(), None => return,
        };
        for vd in mesh.vertex.values_mut() {
            vd.attributes.remove("nx");
            vd.attributes.remove("ny");
            vd.attributes.remove("nz");
        }
        let mut keys: Vec<usize> = mesh.vertex.keys().copied().collect();
        keys.sort_unstable();
        let mut base = Vec::with_capacity(keys.len());
        let mut weights = Vec::with_capacity(keys.len());
        for kk in keys {
            let vd = &mesh.vertex[&kk];
            base.push((kk, [vd.x, vd.y, vd.z]));
            let uv = (vd.attributes.get("u").copied(), vd.attributes.get("v").copied());
            weights.push(match uv {
                (Some(u), Some(v)) => surface_basis_weights(&srf, &row, u, v),
                _ => vec![0.0; row.len()],
            });
        }
        self.edit.surf_base = base;
        self.edit.surf_weights = weights;
        self.edit.live_surf_mesh = Some(mesh);
    }

    /// Per-move deform for a surface boundary edit-point drag: edit-point deltas → boundary
    /// CV-row deltas via `R⁻¹`, written into the surface (weight-preserving) and FMA-applied
    /// to the frozen tessellation through the row's basis weights (same machinery as the
    /// CV-drag path, just with CV deltas sourced from the refit instead of the gumball).
    fn live_update_surface_editpoints(&mut self, guid: &str, iid: u32) {
        let nrow = self.edit.editpt_cv.len();
        if nrow == 0 || self.edit.surf_base.is_empty() { return; }
        let edit_deltas: Vec<[f64; 3]> = self.edit.drag_start.iter().map(|&(idx, start)| {
            let now = self.edit.nodes[idx].world;
            [now[0] - start[0], now[1] - start[1], now[2] - start[2]]
        }).collect();
        let mut cvdelta = vec![[0.0f64; 3]; nrow];
        for a in 0..nrow {
            let mut d = [0.0f64; 3];
            for k in 0..edit_deltas.len() {
                let c = self.edit.editpt_cols[k][a];
                if c != 0.0 {
                    d[0] += c * edit_deltas[k][0];
                    d[1] += c * edit_deltas[k][1];
                    d[2] += c * edit_deltas[k][2];
                }
            }
            cvdelta[a] = d;
        }
        // Weld coincident row CVs (a closed surface's first≡last CV) to a shared delta, else the
        // seam opens when the boundary is transformed.
        for a in 0..nrow {
            for c in (a + 1)..nrow {
                let (pa, pc) = (self.edit.editpt_cv[a].1, self.edit.editpt_cv[c].1);
                if (pa[0]-pc[0]).powi(2)+(pa[1]-pc[1]).powi(2)+(pa[2]-pc[2]).powi(2) < 1e-6 {
                    let avg = [(cvdelta[a][0]+cvdelta[c][0])*0.5, (cvdelta[a][1]+cvdelta[c][1])*0.5, (cvdelta[a][2]+cvdelta[c][2])*0.5];
                    cvdelta[a] = avg; cvdelta[c] = avg;
                }
            }
        }
        // Write the refit CV row into the surface (kernel truth used by commit). set_cv is
        // weight-aware on surfaces, so Euclidean positions keep the rational structure.
        // Snapshot edit state into locals first: edit_surface_mut borrows all of `self`
        // (it may reach into objects.nurbssurfacetrimmeds), which would otherwise clash with
        // reading self.edit.* inside the &mut-surface scope.
        let editpt_cv = self.edit.editpt_cv.clone();
        let editpt_mirror = self.edit.editpt_mirror.clone();
        if let Some(srf) = self.edit_surface_mut(guid) {
            for (a, (addr, base, _w)) in editpt_cv.iter().enumerate() {
                if let NodeAddr::SurfaceCv(i, j) = addr {
                    let p = [base[0] + cvdelta[a][0], base[1] + cvdelta[a][1], base[2] + cvdelta[a][2]];
                    srf.set_cv(*i, *j, &Point::new(p[0], p[1], p[2]));
                }
            }
            // Weld the opposite seam column to the matching row CV's delta.
            for &((i, j), a) in &editpt_mirror {
                let base = editpt_cv[a].1;
                let p = [base[0] + cvdelta[a][0], base[1] + cvdelta[a][1], base[2] + cvdelta[a][2]];
                srf.set_cv(i, j, &Point::new(p[0], p[1], p[2]));
            }
        }
        // Deform the frozen tessellation: pos = base + Σₐ wₐ·cvΔₐ (compute into a local first
        // so the mesh mutable borrow doesn't overlap the surf_base/weights reads).
        let mut new_pos: Vec<(usize, [f64; 3])> = Vec::with_capacity(self.edit.surf_base.len());
        for (vi, (key, base)) in self.edit.surf_base.iter().enumerate() {
            let w = &self.edit.surf_weights[vi];
            let mut p = *base;
            for a in 0..nrow {
                let wa = w[a];
                if wa != 0.0 {
                    p[0] += wa * cvdelta[a][0];
                    p[1] += wa * cvdelta[a][1];
                    p[2] += wa * cvdelta[a][2];
                }
            }
            new_pos.push((*key, p));
        }
        if let Some(mesh) = &mut self.edit.live_surf_mesh {
            for (key, p) in &new_pos {
                if let Some(vd) = mesh.vertex.get_mut(key) {
                    vd.x = p[0];
                    vd.y = p[1];
                    vd.z = p[2];
                }
            }
            let (mut vs, _is) = mesh_to_vertices(mesh);
            for v in &mut vs { v.instance_id = iid; }
            self.scene.gpu_session.tri.update_vertices(guid, &vs, &self.gpu.queue);
        }
    }

    /// Closest selectable scene object under the ray (mirrors `process_pick`'s
    /// hit-combining, minus the gumball and selection side-effects).
    pub(crate) fn closest_object_under_ray(&mut self, ray: Ray) -> Option<String> {
        let pick_radius = self.scene.camera
            .pick_radius_mm(self.vp_rect().3, 8.0)
            .max(crate::gpu_adapters::SPHERE_RADIUS);
        let hits = pick::pick_by_ray(&mut self.scene.session, ray, pick_radius);
        let origin_pt = session_rust::Point::new(ray.origin[0] as f64, ray.origin[1] as f64, ray.origin[2] as f64);
        let dir_vec = session_rust::Vector::new(ray.direction[0] as f64, ray.direction[1] as f64, ray.direction[2] as f64);
        let nurbs_hits = self.scene.gpu_session.pick_nurbssurfaces(&origin_pt, &dir_vec);
        let brep_hits = self.scene.gpu_session.pick_breps(&origin_pt, &dir_vec);
        let nc_hits = self.scene.gpu_session.pick_nurbscurves(&origin_pt, &dir_vec, pick_radius);

        let mut best_guid: Option<String> = None;
        let mut best_dist = f64::MAX;
        for hit in &hits {
            if self.scene.hidden_guids.contains(hit.guid()) { continue; }
            if hit.distance < best_dist { best_dist = hit.distance; best_guid = Some(hit.guid().to_string()); }
        }
        let pt_dist = |p: &session_rust::Point| -> f64 {
            let dx = p[0] - origin_pt[0];
            let dy = p[1] - origin_pt[1];
            let dz = p[2] - origin_pt[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        };
        if let Some((guid, pt)) = nurbs_hits.into_iter().next() {
            if !self.scene.hidden_guids.contains(&guid) {
                let d = pt_dist(&pt);
                if d < best_dist { best_dist = d; best_guid = Some(guid); }
            }
        }
        if let Some((guid, pt)) = brep_hits.into_iter().next() {
            if !self.scene.hidden_guids.contains(&guid) {
                let d = pt_dist(&pt);
                if d < best_dist { best_dist = d; best_guid = Some(guid); }
            }
        }
        if let Some((guid, pt)) = nc_hits.into_iter().next() {
            if !self.scene.hidden_guids.contains(&guid) {
                let d = pt_dist(&pt);
                if d < best_dist { best_guid = Some(guid); }
            }
        }
        best_guid
    }

    // ── Boundary-edge editing (Ctrl+Shift+RMB, no F10) ────────────────────────

    /// Pick the boundary edge of the clicked object and enter a transient edge-move
    /// session. The picked edge is highlighted as its control polygon (orange line) — the
    /// control-point spheres are hidden in this mode; a drag moves the whole edge's CVs.
    pub(crate) fn process_pick_edge(&mut self, ray: Ray) {
        let guid = match self.closest_object_under_ray(ray) { Some(g) => g, None => return };
        let kind = match self.editable_kind(&guid) { Some(k) => k, None => return };

        // ── Edit-point (Greville) boundary editing — handles sit ON the geometry. ──
        // A standalone NurbsCurve edits as its on-curve edit points; a NurbsSurface edits the
        // picked boundary's on-surface edit points (refit reshapes the surface). If setup
        // fails for any reason, fall through to the raw control-polygon path below so the
        // interaction is never left empty/stuck.
        if kind == EditKind::NurbsCurve {
            let same = self.edit.active && self.edit.use_edit_points
                && self.edit.target.as_deref() == Some(guid.as_str());
            if !same {
                self.enter_edit_on(&guid);
                self.edit.use_edit_points = true;
                self.edit.edge_mode = true;
                self.edit.editpt_boundary = None;
                self.rebuild_edit_overlay();
            }
            if self.select_all_editpoints() {
                self.scene.reveal_in_tree = true;
                return;
            }
            self.exit_edit_mode(); // setup produced no handles — fall back to raw path
        } else if kind == EditKind::NurbsSurface {
            let model = self.scene.gpu_session.pick.instance_id(&guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map(|inst| inst.model)
                .unwrap_or(IDENTITY4);
            let tol = self.scene.camera.pick_radius_mm(self.vp_rect().3, 16.0).max(1.0);
            // Validate the boundary + its iso-curve BEFORE mutating any edit state, so a miss
            // leaves the door open to the raw path with no side effects.
            let boundary = self.scene.session.objects.nurbssurfaces.iter()
                .find(|n| n.guid() == guid)
                .and_then(|srf| nearest_surface_boundary(srf, &model, ray)
                    .and_then(|(b, d)| if d < tol && boundary_iso_curve(srf, b).is_some() { Some(b) } else { None }));
            if let Some(b) = boundary {
                self.enter_edit_on(&guid);
                self.edit.use_edit_points = true;
                self.edit.edge_mode = true;
                self.edit.editpt_boundary = Some(b);
                self.rebuild_edit_overlay();
                if self.select_all_editpoints() {
                    self.refresh_edit_edge_curve();
                    self.scene.reveal_in_tree = true;
                    return;
                }
                self.exit_edit_mode();
            }
        }

        // BRep: pick the nearest actual 3D edge loop (works for trim-circle rims too, e.g. caps),
        // select its watertight CV set (hidden — handles/cage off), and draw the loop curve. The
        // gumball transforms the whole loop; re-tess is deferred to release (fast).
        if kind == EditKind::BRep {
            let tol = self.scene.camera.pick_radius_mm(self.vp_rect().3, 16.0).max(1.0);
            let picked = if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get(&guid) {
                let cols = brep_xform_f32(&b.xform);
                brep_nearest_edge(b, &cols, ray, tol).map(|e| {
                    let (addrs, refits) = brep_edge_watertight(b, e);
                    (e, addrs, refits)
                })
            } else {
                None
            };
            if let Some((edge, addrs, refits)) = picked {
                if !addrs.is_empty() || !refits.is_empty() {
                    // Refine each hole face (degree elevation, exact) and capture its CVs + the
                    // hole geometry, so a drag can bend it into a collar.
                    let mut deforms: Vec<(usize, [f64; 3], f64, f64, Vec<((usize, usize), [f64; 3])>)> = Vec::new();
                    if !refits.is_empty() {
                        if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get_mut(&guid) {
                            for (c2i, fs) in &refits {
                                b.m_surfaces[*fs].increase_degree(0, 4);
                                b.m_surfaces[*fs].increase_degree(1, 4);
                                let c2d = b.m_curves_2d[*c2i].clone();
                                let (t0, t1) = c2d.domain();
                                let rim: Vec<Point> = (0..=24).filter_map(|k| {
                                    let t = t0 + (t1 - t0) * (k as f64 / 24.0);
                                    let uv = c2d.point_at(t);
                                    b.m_surfaces[*fs].point_at(uv[0], uv[1])
                                }).collect();
                                if rim.is_empty() { continue; }
                                let mut c = [0.0f64; 3];
                                for p in &rim { c[0]+=p[0]; c[1]+=p[1]; c[2]+=p[2]; }
                                let n = rim.len() as f64; let center = [c[0]/n, c[1]/n, c[2]/n];
                                let dist = |p: &[f64;3]| ((p[0]-center[0]).powi(2)+(p[1]-center[1]).powi(2)+(p[2]-center[2]).powi(2)).sqrt();
                                // Slightly beyond the hole radius so the whole ring of CVs that
                                // controls the trim gets g=1 → the rim follows the wall exactly.
                                let r_in = rim.iter().map(|p| dist(&[p[0],p[1],p[2]])).fold(0.0, f64::max) * 1.25;
                                let (ni, nj) = (b.m_surfaces[*fs].cv_count_dir(Some(0)), b.m_surfaces[*fs].cv_count_dir(Some(1)));
                                // r_out = nearest OUTER-boundary CV (edge midpoint), so every
                                // boundary CV gets g≈0 (outline stays intact); inner CVs near the
                                // hole get g=1 (rim follows the wall, stays connected).
                                let mut cvs = Vec::new(); let mut r_out = f64::MAX;
                                for i in 0..ni { for j in 0..nj {
                                    if let Some(p) = b.m_surfaces[*fs].get_cv(i, j) {
                                        let pp = [p[0], p[1], p[2]];
                                        if i == 0 || i == ni-1 || j == 0 || j == nj-1 { r_out = r_out.min(dist(&pp)); }
                                        cvs.push(((i, j), pp));
                                    }
                                }}
                                let r_out = if r_out <= r_in { r_in * 1.5 } else { r_out };
                                deforms.push((*fs, center, r_in, r_out, cvs));
                            }
                        }
                    }
                    self.enter_edit_on(&guid);
                    self.edit.edge_mode = true;
                    self.edit.brep_edge = Some(edge);
                    self.edit.editpt_face_deform = deforms;
                    self.edit.selected_nodes = addrs.iter()
                        .filter_map(|a| self.edit.nodes.iter().position(|n| &n.addr == a))
                        .collect();
                    self.edit.selected_edges.clear();
                    self.edit.edges.clear();
                    self.update_edit_gumball();
                    self.refresh_edit_edge_curve();
                    self.scene.reveal_in_tree = true;
                    return;
                }
            }
        }

        let addrs = self.boundary_edge_addrs(&guid, kind, ray);
        if addrs.len() < 2 { return; }
        self.enter_edit_on(&guid);
        self.edit.edge_mode = true;
        // Map the ORDERED edge CV addresses to overlay node indices, then represent the
        // edge as its control polygon (consecutive segments) and select it. The gumball's
        // move set = these edges' endpoints = the edge's CVs.
        let idx: Vec<usize> = addrs.iter()
            .filter_map(|a| self.edit.nodes.iter().position(|n| &n.addr == a))
            .collect();
        self.edit.edges = idx.windows(2).map(|w| EditEdge { a: w[0], b: w[1] }).collect();
        self.edit.selected_edges = (0..self.edit.edges.len()).collect();
        self.edit.selected_nodes.clear();
        if self.edit.edges.is_empty() { self.exit_edit_mode(); return; }
        self.update_edit_gumball();
        self.scene.reveal_in_tree = true;
    }

    /// The CV addresses defining the boundary edge nearest the ray, per object kind.
    fn boundary_edge_addrs(&self, guid: &str, kind: EditKind, ray: Ray) -> Vec<NodeAddr> {
        let model = self.scene.gpu_session.pick.instance_id(guid)
            .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
            .map(|inst| inst.model)
            .unwrap_or(IDENTITY4);
        let tol = self.scene.camera.pick_radius_mm(self.vp_rect().3, 16.0).max(1.0);
        match kind {
            EditKind::NurbsSurface => {
                let srf = match self.edit_surface(guid) {
                    Some(s) => s, None => return vec![],
                };
                match nearest_surface_boundary(srf, &model, ray) {
                    Some((b, d)) if d < tol => surface_boundary_addrs(srf, b, None),
                    _ => vec![],
                }
            }
            EditKind::Mesh => {
                let m = match self.scene.session.lookup.get(guid) { Some(Geometry::Mesh(m)) => m, _ => return vec![] };
                let mut best = f32::MAX;
                let mut best_e: Option<(usize, usize)> = None;
                // Boundary (naked) edges only — never interior triangulation edges.
                for (a, b) in m.naked_edges(true) {
                    let (pa, pb) = match (m.vertex.get(&a), m.vertex.get(&b)) {
                        (Some(va), Some(vb)) => (va.position(), vb.position()), _ => continue,
                    };
                    if let Some(d) = ray_seg_dist(ray, xf_pt(&model, &pa), xf_pt(&model, &pb)) {
                        if d < tol && d < best { best = d; best_e = Some((a, b)); }
                    }
                }
                match best_e { Some((a, b)) => vec![NodeAddr::MeshVertex(a), NodeAddr::MeshVertex(b)], None => vec![] }
            }
            EditKind::Polyline => {
                let pl = match self.scene.session.lookup.get(guid) { Some(Geometry::Polyline(pl)) => pl, _ => return vec![] };
                let pts = pl.get_points();
                let mut best = f32::MAX;
                let mut bi = None;
                for i in 0..pts.len().saturating_sub(1) {
                    if let Some(d) = ray_seg_dist(ray, xf_pt(&model, &pts[i]), xf_pt(&model, &pts[i + 1])) {
                        if d < tol && d < best { best = d; bi = Some(i); }
                    }
                }
                match bi { Some(i) => vec![NodeAddr::PolyPoint(i), NodeAddr::PolyPoint(i + 1)], None => vec![] }
            }
            EditKind::NurbsCurve => {
                let nc = match self.scene.session.objects.nurbscurves.iter().find(|n| n.guid() == guid) {
                    Some(c) => c, None => return vec![],
                };
                let n = nc.cv_count();
                let mut best = f32::MAX;
                let mut bi = None;
                // Nearest control-polygon span (the control polygon may be off-curve, so no
                // tolerance gate — the object was already hit).
                for i in 0..n.saturating_sub(1) {
                    let pa = nc.get_cv(i).unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                    let pb = nc.get_cv(i + 1).unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                    if let Some(d) = ray_seg_dist(ray, xf_pt(&model, &pa), xf_pt(&model, &pb)) {
                        if d < best { best = d; bi = Some(i); }
                    }
                }
                match bi { Some(i) => vec![NodeAddr::CurveCv(i), NodeAddr::CurveCv(i + 1)], None => vec![] }
            }
            EditKind::BRep => {
                let b = match self.scene.session.lookup.get(guid) { Some(Geometry::BRep(b)) => b, _ => return vec![] };
                let cols = brep_xform_f32(&b.xform);
                let mut best = f32::MAX;
                let mut best_sb: Option<(usize, usize)> = None;
                for (si, srf) in b.m_surfaces.iter().enumerate() {
                    if let Some((bidx, d)) = nearest_surface_boundary(srf, &cols, ray) {
                        if d < tol && d < best { best = d; best_sb = Some((si, bidx)); }
                    }
                }
                match best_sb {
                    Some((si, bidx)) => surface_boundary_addrs(&b.m_surfaces[si], bidx, Some(si)),
                    None => vec![],
                }
            }
        }
    }

    // ── Dragging / committing ─────────────────────────────────────────────────

    /// Apply a gumball delta matrix to the moving node set (live, during a drag).
    pub(crate) fn apply_edit_delta(&mut self, delta: &[[f32; 4]; 4]) {
        for k in 0..self.edit.drag_start.len() {
            let (idx, start) = self.edit.drag_start[k];
            // f32 gumball delta applied onto the f64 base, in f64, so the base position
            // keeps full precision (only the increment is f32-grade screen math).
            let w = mat_apply_mixed(delta, start);
            if let Some(n) = self.edit.nodes.get_mut(idx) { n.world = w; }
        }
        if let Some(c) = self.edit.selection_centroid() {
            if let Some(gb) = &mut self.gb.gumball { gb.origin = c; }
        }
        // Live, in-place deform — touches only THIS object's GPU ranges (no whole-scene
        // re-upload, no remove+add).
        if self.edit.kind.is_some() {
            self.live_update_geometry();
        }
        // BRep hole: bend the refined hole face into a collar (inner ring follows, outer fixed).
        if !self.edit.editpt_face_deform.is_empty() {
            self.apply_brep_face_deform(delta);
        }
        // Keep the highlighted edge curve following the deformed geometry — cheaply: a full-edge
        // transform is affine, so map the frozen base polyline by the same gumball delta instead
        // of re-sampling the geometry each move.
        if !self.edit.edge_polyline_base.is_empty() {
            self.edit.edge_polyline = self.edit.edge_polyline_base.iter().map(|p| {
                let w = mat_apply_mixed(delta, [p[0] as f64, p[1] as f64, p[2] as f64]);
                [w[0] as f32, w[1] as f32, w[2] as f32]
            }).collect();
        }
        self.edit.recompute_centroid();
    }

    /// Bend each refined hole face into a collar under the gumball delta: each CV is pushed by
    /// `g·(delta·P − P)` where `g` smoothly falls from 1 within `r_in` of the hole to 0 beyond
    /// `r_out` (the outer boundary). The trim is unchanged, so the bent surface lifts the hole rim
    /// while the outline stays fixed. Re-tessellation happens on release.
    fn apply_brep_face_deform(&mut self, delta: &[[f32; 4]; 4]) {
        let guid = match &self.edit.target { Some(g) => g.clone(), None => return };
        if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get_mut(&guid) {
            for (fs, center, r_in, r_out, cvs) in &self.edit.editpt_face_deform {
                let span = (r_out - r_in).max(1e-9);
                for &((i, j), p) in cvs {
                    let d = ((p[0]-center[0]).powi(2)+(p[1]-center[1]).powi(2)+(p[2]-center[2]).powi(2)).sqrt();
                    let g = if d <= *r_in { 1.0 } else if d >= *r_out { 0.0 } else {
                        let t = (r_out - d) / span; t * t * (3.0 - 2.0 * t) // smoothstep
                    };
                    if g == 0.0 { continue; }
                    let pm = mat_apply_mixed(delta, p);
                    let np = [p[0]+g*(pm[0]-p[0]), p[1]+g*(pm[1]-p[1]), p[2]+g*(pm[2]-p[2])];
                    b.m_surfaces[*fs].set_cv(i, j, &Point::new(np[0], np[1], np[2]));
                }
            }
        }
    }

    /// In-place GPU update of the edited object during a drag. Writes only this
    /// object's vertex/segment/glyph ranges, with topology frozen, so per-move cost
    /// scales with the edited object — not the whole scene. The authoritative adaptive
    /// rebuild (re-tessellation, edges, BVH, undo) happens once on release in
    /// `commit_edit_transform`.
    fn live_update_geometry(&mut self) {
        let guid = match &self.edit.target { Some(g) => g.clone(), None => return };
        let kind = match self.edit.kind { Some(k) => k, None => return };
        let iid = match self.scene.gpu_session.pick.instance_id(&guid) { Some(i) => i, None => return };
        match kind {
            EditKind::Mesh => {
                if let Some(Geometry::Mesh(m)) = self.scene.session.lookup.get_mut(&guid) {
                    for n in &self.edit.nodes {
                        if let NodeAddr::MeshVertex(k) = n.addr {
                            if let Some(v) = m.vertex.get_mut(&k) {
                                v.set_position(Point::new(n.world[0], n.world[1], n.world[2]));
                            }
                        }
                    }
                    m.invalidate_triangle_bvh();
                }
                let (mut vs, segs, glyphs) = match self.scene.session.lookup.get(&guid) {
                    Some(Geometry::Mesh(m)) => {
                        let (vs, _is) = mesh_to_vertices(m);
                        let segs = if m.crease_angle_deg > 0.0 {
                            mesh_crease_edges_to_segments(m, iid, m.crease_angle_deg as f32)
                        } else {
                            mesh_edges_to_segments(m, iid)
                        };
                        (vs, segs, mesh_vertex_glyphs(m, iid))
                    }
                    _ => return,
                };
                for v in &mut vs { v.instance_id = iid; }
                self.scene.gpu_session.tri.update_vertices(&guid, &vs, &self.gpu.queue);
                self.scene.gpu_session.update_object_segments(&guid, &segs, &self.gpu.queue);
                self.scene.gpu_session.update_object_glyphs(&guid, &glyphs, &self.gpu.queue);
            }
            EditKind::Polyline => {
                if let Some(Geometry::Polyline(pl)) = self.scene.session.lookup.get_mut(&guid) {
                    for n in &self.edit.nodes {
                        if let NodeAddr::PolyPoint(i) = n.addr {
                            pl.set_point(i, &Point::new(n.world[0], n.world[1], n.world[2]));
                        }
                    }
                }
                let (segs, glyphs) = match self.scene.session.lookup.get(&guid) {
                    Some(Geometry::Polyline(pl)) => {
                        (polyline_to_segments(pl, iid), polyline_endpoint_glyphs(pl, iid))
                    }
                    _ => return,
                };
                self.scene.gpu_session.update_object_segments(&guid, &segs, &self.gpu.queue);
                self.scene.gpu_session.update_object_glyphs(&guid, &glyphs, &self.gpu.queue);
            }
            EditKind::NurbsCurve => {
                if self.edit.live_curve_segs == 0 {
                    self.edit.live_curve_segs = self.scene.gpu_session.guid_to_seg
                        .get(&guid).map(|r| r.end - r.start).unwrap_or(0);
                }
                let n_seg = self.edit.live_curve_segs;
                if n_seg == 0 { return; }
                // Edit-point drag: ΔPᵢ = Σₖ (R⁻¹)ᵢₖ·Δₖ over the frozen columns, applied to the
                // Euclidean CV bases captured at drag start; weights kept (circles stay rational).
                let editpt_deltas: Option<Vec<[f64; 3]>> = if self.edit.use_edit_points {
                    Some(self.edit.drag_start.iter().map(|&(idx, start)| {
                        let now = self.edit.nodes[idx].world;
                        [now[0] - start[0], now[1] - start[1], now[2] - start[2]]
                    }).collect())
                } else {
                    None
                };
                let curve = {
                    let nc = match self.scene.session.objects.nurbscurves.iter_mut().find(|n| n.guid() == guid) {
                        Some(n) => n, None => return,
                    };
                    if let Some(deltas) = &editpt_deltas {
                        for (a, (addr, base, w)) in self.edit.editpt_cv.iter().enumerate() {
                            let i = match addr { NodeAddr::CurveCv(i) => *i, _ => continue };
                            let mut p = *base;
                            for k in 0..deltas.len() {
                                let c = self.edit.editpt_cols[k][a];
                                if c != 0.0 {
                                    p[0] += c * deltas[k][0];
                                    p[1] += c * deltas[k][1];
                                    p[2] += c * deltas[k][2];
                                }
                            }
                            nc.set_cv_4d(i, p[0] * w, p[1] * w, p[2] * w, *w);
                        }
                    } else {
                        for n in &self.edit.nodes {
                            if let NodeAddr::CurveCv(i) = n.addr {
                                // Weight-preserving (NurbsCurve::set_cv isn't weight-aware in Rust).
                                let w = nc.get_cv_4d(i).map_or(1.0, |(_, _, _, w)| w);
                                nc.set_cv_4d(i, n.world[0] * w, n.world[1] * w, n.world[2] * w, w);
                            }
                        }
                    }
                    nc.clone()
                };
                // Fixed-count uniform resample so the segment range stays stable.
                let (t0, t1) = curve.domain();
                let mut segs = Vec::with_capacity(n_seg);
                let mut prev = curve.point_at(t0);
                for k in 1..=n_seg {
                    let t = t0 + (t1 - t0) * (k as f64 / n_seg as f64);
                    let p = curve.point_at(t);
                    segs.push(CylinderSegment {
                        p0: [prev[0] as f32, prev[1] as f32, prev[2] as f32], radius: 0.0,
                        p1: [p[0] as f32, p[1] as f32, p[2] as f32], instance_id: iid, color: [0.0; 4],
                    });
                    prev = p;
                }
                let first = curve.point_at(t0);
                let last = curve.point_at(t1);
                let glyphs = vec![
                    GlyphPoint { center: [first[0] as f32, first[1] as f32, first[2] as f32], radius: 0.0, color: [1.0; 4], instance_id: iid, _pad: [0; 3] },
                    GlyphPoint { center: [last[0] as f32, last[1] as f32, last[2] as f32], radius: 0.0, color: [1.0; 4], instance_id: iid, _pad: [0; 3] },
                ];
                self.scene.gpu_session.update_object_segments(&guid, &segs, &self.gpu.queue);
                self.scene.gpu_session.update_object_glyphs(&guid, &glyphs, &self.gpu.queue);
            }
            EditKind::NurbsSurface => {
                // Edit-point boundary drag: refit the boundary CV row via R⁻¹ (captured at
                // drag start) and FMA-deform the tessellation. Distinct from the CV-drag path
                // below whose "moved set" is the gumball-picked CVs themselves.
                if self.edit.use_edit_points {
                    self.live_update_surface_editpoints(&guid, iid);
                    return;
                }
                // Lazy capture at drag start: freeze the tessellation and precompute, per
                // vertex, the basis-function influence of each moved CV. A NURBS point is
                // linear in its CVs, so the drag then just does FMA — no point_at/normal_at.
                if self.edit.surf_base.is_empty() {
                    let mut mesh = match self.scene.gpu_session.nurbs_pick_meshes.get(&guid) {
                        Some((m, _xf)) => m.clone(),
                        None => {
                            log::info!("[edit] live_update surface guid={guid}: no nurbs_pick_mesh (trimmed) -> skip live deform");
                            return;
                        }
                    };
                    // Drop stored normals so mesh_to_vertices recomputes smooth normals from
                    // the deformed positions each move (live shading, no analytic normal_at).
                    for vd in mesh.vertex.values_mut() {
                        vd.attributes.remove("nx");
                        vd.attributes.remove("ny");
                        vd.attributes.remove("nz");
                    }
                    let moved: Vec<(usize, usize)> = self.edit.drag_start.iter().map(|&(idx, _)| {
                        match self.edit.nodes[idx].addr {
                            NodeAddr::SurfaceCv(i, j) => (i, j),
                            _ => (usize::MAX, usize::MAX),
                        }
                    }).collect();
                    if let Some(srf) = self.edit_surface(&guid) {
                        let mut keys: Vec<usize> = mesh.vertex.keys().copied().collect();
                        keys.sort_unstable();
                        let mut base = Vec::with_capacity(keys.len());
                        let mut weights = Vec::with_capacity(keys.len());
                        for k in keys {
                            let vd = &mesh.vertex[&k];
                            base.push((k, [vd.x, vd.y, vd.z]));
                            let uv = (vd.attributes.get("u").copied(), vd.attributes.get("v").copied());
                            weights.push(match uv {
                                (Some(u), Some(v)) => surface_basis_weights(srf, &moved, u, v),
                                _ => vec![0.0; moved.len()],
                            });
                        }
                        self.edit.surf_base = base;
                        self.edit.surf_weights = weights;
                    } else {
                        return;
                    }
                    self.edit.live_surf_mesh = Some(mesh);
                }
                // Per-move deform: pos = base + Σₖ weightₖ·(CV_nowₖ − CV_startₖ).
                let deltas: Vec<[f64; 3]> = self.edit.drag_start.iter().map(|&(idx, start)| {
                    let now = self.edit.nodes[idx].world;
                    [now[0] - start[0], now[1] - start[1], now[2] - start[2]]
                }).collect();
                let mut new_pos: Vec<(usize, [f64; 3])> = Vec::with_capacity(self.edit.surf_base.len());
                for (vi, (key, base)) in self.edit.surf_base.iter().enumerate() {
                    let w = &self.edit.surf_weights[vi];
                    let mut p = *base;
                    for k in 0..deltas.len() {
                        let wk = w[k];
                        if wk != 0.0 {
                            p[0] += wk * deltas[k][0];
                            p[1] += wk * deltas[k][1];
                            p[2] += wk * deltas[k][2];
                        }
                    }
                    new_pos.push((*key, p));
                }
                if let Some(mesh) = &mut self.edit.live_surf_mesh {
                    for (key, p) in &new_pos {
                        if let Some(vd) = mesh.vertex.get_mut(key) {
                            vd.x = p[0];
                            vd.y = p[1];
                            vd.z = p[2];
                        }
                    }
                    let (mut vs, _is) = mesh_to_vertices(mesh);
                    for v in &mut vs { v.instance_id = iid; }
                    self.scene.gpu_session.tri.update_vertices(&guid, &vs, &self.gpu.queue);
                }
            }
            EditKind::BRep => {
                // Edge-session drag (Ctrl+Shift+LMB): re-meshing the trimmed faces (CDT) every
                // mouse-move is the bottleneck, so skip it here — the moved CV handles and the
                // highlighted edge curve update live (cheap), and the solid re-tessellates once on
                // release in `flush_edit_to_geometry`. F10 CV editing (edge_mode=false) keeps the
                // in-place live re-tessellation below.
                if self.edit.edge_mode {
                    return;
                }
                // Write the moved CVs back into the underlying surfaces (xform inverse, as in
                // commit), then re-tessellate the faces in place — no remove+add, no
                // whole-scene re-upload. Coarse tessellation keeps this cheap for typical BReps.
                let (ta, tc) = (
                    self.scene.gpu_session.tess_angle_deg as f64,
                    self.scene.gpu_session.tess_chord_factor as f64,
                );
                if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get_mut(&guid) {
                    let inv = b.xform.inverse().map(|x| x.to_cols());
                    for n in &self.edit.nodes {
                        if let NodeAddr::BRepCv(si, i, j) = n.addr {
                            let local = match &inv { Some(m) => mat_apply_f64(m, n.world), None => n.world };
                            if let Some(srf) = b.m_surfaces.get_mut(si) {
                                srf.set_cv(i, j, &Point::new(local[0], local[1], local[2]));
                            }
                        }
                    }
                }
                let vs = match self.scene.session.lookup.get(&guid) {
                    Some(Geometry::BRep(b)) => {
                        let face_ms = b.face_meshes_q(Some((ta, tc)));
                        let mut vs = Vec::new();
                        for fm in &face_ms {
                            let (fvs, _fis) = mesh_to_vertices(fm);
                            vs.extend(fvs);
                        }
                        vs
                    }
                    _ => return,
                };
                if !vs.is_empty() {
                    let mut vs = vs;
                    for v in &mut vs { v.instance_id = iid; }
                    // No-op unless the tessellation count is unchanged (planar faces are
                    // stable; curved faces refresh on commit).
                    self.scene.gpu_session.tri.update_vertices(&guid, &vs, &self.gpu.queue);
                }
            }
        }
    }

    /// Numeric-entry equivalent of a drag (popup "apply"): build the delta, apply it,
    /// and commit.
    pub(crate) fn apply_edit_input(&mut self, value: f32) {
        let handle = match &self.gb.gumball_input { Some(i) => i.handle, None => return };
        let origin = match &self.gb.gumball { Some(gb) => gb.origin, None => { self.gb.gumball_input = None; return; } };
        let delta = crate::gumball::manual_delta(handle, value, origin);
        self.apply_edit_delta(&delta);
        self.commit_edit_transform();
        self.gb.gumball_input = None;
    }

    /// Finalize an edit: write positions into geometry, rebuild, and push undo.
    pub(crate) fn commit_edit_transform(&mut self) {
        self.flush_edit_to_geometry();
        if let (Some(guid), Some(before)) = (self.edit.target.clone(), self.edit.before_snapshot.take()) {
            if let Some(after) = self.snapshot_target() {
                self.hist.push(UndoAction::EditGeom { guid, before, after });
            }
        }
        self.edit.drag_start.clear();
        self.edit.clear_live();
        self.rebuild_edit_overlay();
        if self.edit.use_edit_points || self.edit.brep_edge.is_some() {
            self.refresh_edit_edge_curve();
        }
        self.update_edit_gumball();
    }

    /// Write the current node world positions back into the source geometry and
    /// rebuild its GPU representation.
    pub(crate) fn flush_edit_to_geometry(&mut self) {
        let guid = match &self.edit.target { Some(g) => g.clone(), None => return };
        let kind = match self.edit.kind { Some(k) => k, None => return };
        match kind {
            EditKind::Mesh => {
                if let Some(Geometry::Mesh(m)) = self.scene.session.lookup.get_mut(&guid) {
                    for n in &self.edit.nodes {
                        if let NodeAddr::MeshVertex(k) = n.addr {
                            if let Some(v) = m.vertex.get_mut(&k) {
                                v.set_position(Point::new(n.world[0], n.world[1], n.world[2]));
                            }
                        }
                    }
                    // Writing through VertexData::set_position bypasses the mesh mutators
                    // that normally drop the per-mesh triangle BVH, so invalidate it here —
                    // otherwise ray-casts (picking) keep hitting the pre-edit shape.
                    m.invalidate_triangle_bvh();
                }
                self.rebuild_lookup_target(&guid);
            }
            EditKind::Polyline => {
                if let Some(Geometry::Polyline(pl)) = self.scene.session.lookup.get_mut(&guid) {
                    for n in &self.edit.nodes {
                        if let NodeAddr::PolyPoint(i) = n.addr {
                            pl.set_point(i, &Point::new(n.world[0], n.world[1], n.world[2]));
                        }
                    }
                }
                self.rebuild_lookup_target(&guid);
            }
            EditKind::BRep => {
                // Don't overwrite hole faces deformed into a collar — their CVs were written
                // directly during the drag; the (stale, flat) overlay nodes would undo it.
                let skip: std::collections::HashSet<usize> =
                    self.edit.editpt_face_deform.iter().map(|(fs, ..)| *fs).collect();
                if let Some(Geometry::BRep(b)) = self.scene.session.lookup.get_mut(&guid) {
                    let inv = b.xform.inverse().map(|x| x.to_cols());
                    for n in &self.edit.nodes {
                        if let NodeAddr::BRepCv(si, i, j) = n.addr {
                            if skip.contains(&si) { continue; }
                            let local = match &inv { Some(m) => mat_apply_f64(m, n.world), None => n.world };
                            if let Some(srf) = b.m_surfaces.get_mut(si) {
                                srf.set_cv(i, j, &Point::new(local[0], local[1], local[2]));
                            }
                        }
                    }
                    // Re-derive the 3D edge curves from the 2D trims on the reshaped
                    // surfaces so the rendered edges / boundary loops follow the move
                    // (otherwise they stay at the pre-edit position and look detached).
                    recompute_brep_edges(b);
                }
                self.rebuild_lookup_target(&guid);
            }
            EditKind::NurbsSurface => {
                // Snapshot nodes first: edit_surface_mut borrows all of `self`.
                let nodes = self.edit.nodes.clone();
                let clone = {
                    let srf = self.edit_surface_mut(&guid);
                    match srf {
                        Some(srf) => {
                            for n in &nodes {
                                if let NodeAddr::SurfaceCv(i, j) = n.addr {
                                    srf.set_cv(i, j, &Point::new(n.world[0], n.world[1], n.world[2]));
                                }
                            }
                            srf.clone()
                        }
                        None => return,
                    }
                };
                // The drag already deformed the tri arena in place (basis-weight). Finalize
                // IN PLACE (refresh edges + pick mesh only) so commit is instant — no
                // re-tessellation, no whole-scene re-upload. Fall back to a full rebuild if
                // there's no live capture or the edge sample count changed.
                let live = self.edit.live_surf_mesh.take();
                // The in-place mesh only reflects the moved row; if a seam (mirror column) also
                // moved, force a full re-tessellation so the far side closes visually. Trimmed
                // surfaces always take the full path (their pick mesh lives in the trimmed maps,
                // and the trim must re-tessellate against the reshaped base surface).
                let is_trimmed = self.scene.session.objects.nurbssurfacetrimmeds.iter().any(|t| t.guid() == guid);
                let force_full = is_trimmed || !self.edit.editpt_mirror.is_empty();
                log::info!("[edit] flush NurbsSurface guid={guid} is_trimmed={is_trimmed} force_full={force_full}");
                if force_full || !self.try_finalize_surface_in_place(&guid, &clone, live) {
                    log::info!("[edit] flush: remove+reupload start");
                    self.scene.gpu_session.remove(&guid);
                    self.reupload_edit_surface(&guid);
                    log::info!("[edit] flush: remove+reupload done");
                }
                self.post_rebuild_reapply(&guid);
                log::info!("[edit] flush NurbsSurface done");
            }
            EditKind::NurbsCurve => {
                let clone = {
                    let nc = self.scene.session.objects.nurbscurves.iter_mut().find(|n| n.guid() == guid);
                    match nc {
                        Some(nc) => {
                            for n in &self.edit.nodes {
                                if let NodeAddr::CurveCv(i) = n.addr {
                                    // Write through homogeneous coords preserving the existing
                                    // weight. NurbsCurve::set_cv (unlike the surface setter) is
                                    // NOT weight-aware in Rust, so a plain set_cv_point on a
                                    // rational curve (arc/circle/conic) would divide the CV by
                                    // its weight and corrupt the shape. get_cv returns the
                                    // de-homogenized point, so to round-trip we store world*w.
                                    let w = nc.get_cv_4d(i).map_or(1.0, |(_, _, _, w)| w);
                                    nc.set_cv_4d(i, n.world[0] * w, n.world[1] * w, n.world[2] * w, w);
                                }
                            }
                            nc.clone()
                        }
                        None => return,
                    }
                };
                self.scene.gpu_session.remove(&guid);
                self.scene.gpu_session.add_nurbscurve(&clone, &self.gpu.device, &self.gpu.queue);
                self.post_rebuild_reapply(&guid);
            }
        }
    }

    /// Finalize a NurbsSurface edit IN PLACE after a basis-weight drag: the tri arena is
    /// already deformed, so we only refresh the boundary iso-curve edges (in-place segment
    /// write, no whole-scene re-upload) and the cached pick mesh + surface. Returns false
    /// (→ caller does a full rebuild) when there's no live capture or the iso-curve sample
    /// count changed (so the in-place segment write can't match the existing range).
    fn try_finalize_surface_in_place(
        &mut self,
        guid: &str,
        surface: &session_rust::NurbsSurface,
        live: Option<session_rust::Mesh>,
    ) -> bool {
        let mesh = match live { Some(m) => m, None => return false };
        let iid = match self.scene.gpu_session.pick.instance_id(guid) { Some(i) => i, None => return false };
        // Regenerate the boundary edges the SAME way add_nurbssurface does — the deformed
        // mesh's own naked edges (same topology ⇒ same count ⇒ an in-place write).
        let mut edge_segs = mesh_naked_edges_to_segments(&mesh, iid);
        if let Some(lc) = surface.linecolors.get(0) {
            let c = color_to_rgba_f32(lc);
            for s in &mut edge_segs { s.color = c; }
        }
        // In-place segment write requires an unchanged count; else bail to a full rebuild.
        let cur_len = self.scene.gpu_session.guid_to_seg.get(guid).map(|r| r.end - r.start).unwrap_or(0);
        if edge_segs.len() != cur_len { return false; }
        self.scene.gpu_session.update_object_segments(guid, &edge_segs, &self.gpu.queue);
        // Refresh picking: store the deformed mesh and drop its (stale) triangle BVH.
        let xf = self.scene.gpu_session.nurbs_pick_meshes.get(guid)
            .map(|(_, x)| *x)
            .unwrap_or([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);
        let mut pm = mesh;
        pm.invalidate_triangle_bvh();
        self.scene.gpu_session.nurbs_pick_meshes.insert(guid.to_string(), (pm, xf));
        self.scene.gpu_session.nurbs_surfaces.insert(guid.to_string(), surface.clone());
        true
    }

    /// Rebuild a lookup-resident object (mesh/polyline/brep) on the GPU after its
    /// coordinates changed, and invalidate the session caches.
    fn rebuild_lookup_target(&mut self, guid: &str) {
        self.scene.session.cached_boxes.clear();
        self.scene.session.cached_guids.clear();
        self.scene.session.invalidate_bvh_cache();
        self.scene.gpu_session.remove(guid);
        if let Some(geom) = self.scene.session.lookup.remove(guid) {
            self.scene.gpu_session.add_geometry(guid, &geom, &self.gpu.device, &self.gpu.queue);
            self.scene.session.lookup.insert(guid.to_string(), geom);
        }
        self.post_rebuild_reapply(guid);
    }

    /// Re-apply highlight / visibility / color / thickness after a GPU rebuild.
    fn post_rebuild_reapply(&mut self, guid: &str) {
        // The edit target stays un-highlighted while editing (the cage is the cue),
        // so a rebuild mid-edit must not re-apply the FLAG_SELECTED yellow tint.
        let highlight = self.scene.selected_guids.contains(guid)
            && !(self.edit.active && self.edit.target.as_deref() == Some(guid));
        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, highlight, &self.gpu.queue);
        self.reapply_visibility_flags(guid);
        self.reapply_color_overrides(guid);
        self.apply_thickness();
    }

    // ── Snapshots (undo) ──────────────────────────────────────────────────────

    /// Clone the current target geometry for an undo snapshot.
    pub(crate) fn snapshot_target(&self) -> Option<EditSnapshot> {
        let guid = self.edit.target.as_ref()?;
        if let Some(g) = self.scene.session.lookup.get(guid) {
            return Some(EditSnapshot::Geom(g.clone()));
        }
        if let Some(s) = self.edit_surface(guid) {
            return Some(EditSnapshot::Nurbs(s.clone()));
        }
        if let Some(c) = self.scene.session.objects.nurbscurves.iter().find(|n| n.guid() == guid) {
            return Some(EditSnapshot::Curve(c.clone()));
        }
        None
    }

    /// Restore a snapshot into its store and rebuild the GPU mirror (undo/redo).
    pub(crate) fn restore_edit_snapshot(&mut self, guid: &str, snap: &EditSnapshot) {
        self.scene.gpu_session.remove(guid);
        match snap {
            EditSnapshot::Geom(g) => {
                self.scene.session.lookup.insert(guid.to_string(), g.clone());
                self.scene.gpu_session.add_geometry(guid, g, &self.gpu.device, &self.gpu.queue);
            }
            EditSnapshot::Nurbs(s) => {
                if let Some(slot) = self.edit_surface_mut(guid) {
                    *slot = s.clone();
                }
                self.reupload_edit_surface(guid);
            }
            EditSnapshot::Curve(c) => {
                if let Some(slot) = self.scene.session.objects.nurbscurves.iter_mut().find(|n| n.guid() == guid) {
                    *slot = c.clone();
                }
                self.scene.gpu_session.add_nurbscurve(c, &self.gpu.device, &self.gpu.queue);
            }
        }
        self.scene.session.cached_boxes.clear();
        self.scene.session.cached_guids.clear();
        self.scene.session.invalidate_bvh_cache();
        let highlight = self.scene.selected_guids.contains(guid)
            && !(self.edit.active && self.edit.target.as_deref() == Some(guid));
        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, highlight, &self.gpu.queue);
        self.reapply_visibility_flags(guid);
        self.reapply_color_overrides(guid);
        self.apply_thickness();
        if self.edit.active && self.edit.target.as_deref() == Some(guid) {
            self.rebuild_edit_overlay();
            self.update_edit_gumball();
        }
    }

    // ── Per-frame handle scale ────────────────────────────────────────────────

    /// Recompute the screen-constant handle scale from the node centroid. Mirrors
    /// the gumball scale formula so control points stay a fixed pixel size.
    pub(crate) fn update_edit_scale(&mut self) {
        if !self.edit.active || self.edit.nodes.is_empty() { return; }
        const VIEWER_TO_MM: f32 = 1000.0;
        let vp_h = self.vp_rect().3;
        let c = self.edit.centroid;
        self.edit.handle_scale = match self.scene.camera.proj_mode {
            ProjMode::Perspective => {
                let vm = self.scene.camera.view_matrix();
                let vz = vm[0][2] * c[0] + vm[1][2] * c[1] + vm[2][2] * c[2] + vm[3][2];
                let depth_mm = (-vz).max(0.001) * VIEWER_TO_MM;
                let mm_per_px = 2.0 * depth_mm * self.scene.camera.tan_half_fov_y() / vp_h;
                crate::gumball::SCREEN_PX * mm_per_px / crate::gumball::ARC_RADIUS
            }
            ProjMode::Ortho => {
                let ortho_h_mm = self.scene.camera.ortho_half_h() * 2.0 * VIEWER_TO_MM;
                let mm_per_px = ortho_h_mm / vp_h;
                crate::gumball::SCREEN_PX * mm_per_px / crate::gumball::ARC_RADIUS
            }
        };
    }
}

/// Re-derive a BRep's 3D edge curves from its 2D trim curves evaluated on the
/// (possibly just-edited) surfaces, so rendered edges / boundary loops follow a
/// control-point move. Each topology edge is rebuilt from its first trim: sample the
/// 2D pcurve over its domain, map each (u,v) through the owning face's surface, and
/// store a degree-1 NurbsCurve through those points (what the edge renderer samples).
/// Shared edges may crack if the two adjoining faces' CVs were moved independently —
/// inherent to per-surface CV editing, same caveat as Rhino's SolidPtOn.
fn recompute_brep_edges(b: &mut session_rust::BRep) {
    use session_rust::NurbsCurve;
    const SAMPLES: usize = 32;
    let n_edges = b.m_topology_edges.len();
    for ei in 0..n_edges {
        let (c3i, trim_idx) = {
            let edge = &b.m_topology_edges[ei];
            (edge.curve_3d_index, edge.trim_indices.first().copied().unwrap_or(-1))
        };
        if c3i < 0 || (c3i as usize) >= b.m_curves_3d.len() { continue; }
        if trim_idx < 0 || (trim_idx as usize) >= b.m_trims.len() { continue; }
        let (c2i, loop_idx) = {
            let trim = &b.m_trims[trim_idx as usize];
            (trim.curve_2d_index, trim.loop_index)
        };
        if c2i < 0 || (c2i as usize) >= b.m_curves_2d.len() { continue; }
        if loop_idx < 0 || (loop_idx as usize) >= b.m_loops.len() { continue; }
        let face_idx = b.m_loops[loop_idx as usize].face_index;
        if face_idx < 0 || (face_idx as usize) >= b.m_faces.len() { continue; }
        let srf_idx = b.m_faces[face_idx as usize].surface_index;
        if srf_idx < 0 || (srf_idx as usize) >= b.m_surfaces.len() { continue; }

        let c2d = b.m_curves_2d[c2i as usize].clone();
        let (t0, t1) = c2d.domain();
        let mut pts = Vec::with_capacity(SAMPLES + 1);
        {
            let srf = &b.m_surfaces[srf_idx as usize];
            for k in 0..=SAMPLES {
                let t = t0 + (t1 - t0) * (k as f64 / SAMPLES as f64);
                let uv = c2d.point_at(t);
                if let Some(p) = srf.point_at(uv[0], uv[1]) {
                    pts.push(p);
                }
            }
        }
        if pts.len() >= 2 {
            b.m_curves_3d[c3i as usize] = NurbsCurve::create(false, 1, &pts);
        }
    }
}

const IDENTITY4: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

/// f32 GPU model applied to a kernel f64 point → world f32.
fn xf_pt(m: &[[f32; 4]; 4], p: &session_rust::Point) -> [f32; 3] {
    [
        m[0][0] * p[0] as f32 + m[1][0] * p[1] as f32 + m[2][0] * p[2] as f32 + m[3][0],
        m[0][1] * p[0] as f32 + m[1][1] * p[1] as f32 + m[2][1] * p[2] as f32 + m[3][1],
        m[0][2] * p[0] as f32 + m[1][2] * p[1] as f32 + m[2][2] * p[2] as f32 + m[3][2],
    ]
}

/// Kernel `Xform` (f64) → GPU f32 column-major matrix.
fn brep_xform_f32(xf: &session_rust::Xform) -> [[f32; 4]; 4] {
    let c = xf.to_cols();
    let mut o = [[0.0f32; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            o[a][b] = c[a][b] as f32;
        }
    }
    o
}

/// Nearest of a surface's 4 boundary iso-curves to the ray: `(boundary, distance)` where
/// boundary 0=v_min, 1=v_max, 2=u_min, 3=u_max.
fn nearest_surface_boundary(
    srf: &session_rust::NurbsSurface,
    model: &[[f32; 4]; 4],
    ray: Ray,
) -> Option<(usize, f32)> {
    let du = srf.domain(0)?;
    let dv = srf.domain(1)?;
    // (boundary, iso_dir, t): iso_curve(0,v) is the u-curve at fixed v, etc.
    let specs = [(0usize, 0usize, dv.0), (1, 0, dv.1), (2, 1, du.0), (3, 1, du.1)];
    let mut best = f32::MAX;
    let mut bi = None;
    for &(b, iso_dir, t) in &specs {
        if let Some(crv) = srf.iso_curve(iso_dir, t) {
            let (pts, _) = crv.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
            for w in pts.windows(2) {
                if let Some(d) = ray_seg_dist(ray, xf_pt(model, &w[0]), xf_pt(model, &w[1])) {
                    if d < best { best = d; bi = Some(b); }
                }
            }
        }
    }
    bi.map(|b| (b, best))
}

/// The iso-curve for a surface boundary (0=v_min, 1=v_max, 2=u_min, 3=u_max). Its CVs are
/// the boundary CV row in the same order as `surface_boundary_addrs`, so its Greville edit
/// points / `R⁻¹` map straight onto that row.
fn boundary_iso_curve(srf: &session_rust::NurbsSurface, boundary: usize) -> Option<session_rust::NurbsCurve> {
    let du = srf.domain(0)?;
    let dv = srf.domain(1)?;
    let (iso_dir, t) = match boundary {
        0 => (0, dv.0),
        1 => (0, dv.1),
        2 => (1, du.0),
        3 => (1, du.1),
        _ => return None,
    };
    srf.iso_curve(iso_dir, t)
}

/// CV addresses along a surface boundary → `SurfaceCv` (standalone) or `BRepCv(si,..)`.
fn surface_boundary_addrs(
    srf: &session_rust::NurbsSurface,
    boundary: usize,
    brep_si: Option<usize>,
) -> Vec<NodeAddr> {
    let ni = srf.cv_count_dir(Some(0));
    let nj = srf.cv_count_dir(Some(1));
    let ijs: Vec<(usize, usize)> = match boundary {
        0 => (0..ni).map(|i| (i, 0)).collect(),
        1 => (0..ni).map(|i| (i, nj.saturating_sub(1))).collect(),
        2 => (0..nj).map(|j| (0, j)).collect(),
        3 => (0..nj).map(|j| (ni.saturating_sub(1), j)).collect(),
        _ => vec![],
    };
    ijs.into_iter().map(|(i, j)| match brep_si {
        Some(si) => NodeAddr::BRepCv(si, i, j),
        None => NodeAddr::SurfaceCv(i, j),
    }).collect()
}

/// Which CVs of `srf` a 2D trim curve controls: a boundary row/col when the trim is iso-aligned
/// (u or v ≈ const at a domain extreme), else `None` (an interior trim — caller moves the whole
/// surface). This is what lets a planar cap (circular interior trim) follow a moved rim while a
/// lateral surface only moves its boundary row.
fn brep_iso_aligned_cvs(srf: &session_rust::NurbsSurface, c2d: &session_rust::NurbsCurve) -> Option<Vec<(usize, usize)>> {
    let du = srf.domain(0)?;
    let dv = srf.domain(1)?;
    let ni = srf.cv_count_dir(Some(0));
    let nj = srf.cv_count_dir(Some(1));
    let (t0, t1) = c2d.domain();
    let mut us = Vec::new();
    let mut vs = Vec::new();
    for k in 0..=8 {
        let t = t0 + (t1 - t0) * (k as f64 / 8.0);
        let uv = c2d.point_at(t);
        us.push(uv[0]);
        vs.push(uv[1]);
    }
    let span = |v: &[f64]| v.iter().cloned().fold(f64::MIN, f64::max) - v.iter().cloned().fold(f64::MAX, f64::min);
    let (su, sv) = (span(&us), span(&vs));
    let umid = us.iter().sum::<f64>() / us.len() as f64;
    let vmid = vs.iter().sum::<f64>() / vs.len() as f64;
    let tol = 1e-6_f64;
    let near = |a: f64, b: f64, scale: f64| (a - b).abs() <= tol.max(scale.abs() * 1e-4);
    if sv <= su * 0.01 + tol {
        if near(vmid, dv.0, dv.1 - dv.0) { return Some((0..ni).map(|i| (i, 0)).collect()); }
        if near(vmid, dv.1, dv.1 - dv.0) { return Some((0..ni).map(|i| (i, nj - 1)).collect()); }
    }
    if su <= sv * 0.01 + tol {
        if near(umid, du.0, du.1 - du.0) { return Some((0..nj).map(|j| (0, j)).collect()); }
        if near(umid, du.1, du.1 - du.0) { return Some((0..nj).map(|j| (ni - 1, j)).collect()); }
    }
    None
}

/// The topology edge whose 3D curve is nearest the ray (within `tol`), or None. Picks the actual
/// edge loop — works for trim-circle rims (caps) that aren't surface domain boundaries.
fn brep_nearest_edge(b: &session_rust::BRep, cols: &[[f32; 4]; 4], ray: Ray, tol: f32) -> Option<usize> {
    let mut best = f32::MAX;
    let mut bi = None;
    for (ei, e) in b.m_topology_edges.iter().enumerate() {
        let c3 = e.curve_3d_index;
        if c3 < 0 || (c3 as usize) >= b.m_curves_3d.len() { continue; }
        let crv = &b.m_curves_3d[c3 as usize];
        let (t0, t1) = crv.domain();
        let mut prev = xf_pt(cols, &crv.point_at(t0));
        for k in 1..=48 {
            let t = t0 + (t1 - t0) * (k as f64 / 48.0);
            let p = xf_pt(cols, &crv.point_at(t));
            if let Some(d) = ray_seg_dist(ray, prev, p) {
                if d < tol && d < best { best = d; bi = Some(ei); }
            }
            prev = p;
        }
    }
    bi
}

/// Watertight contributions for a topology `edge`: returns (CV moves, trim refits).
/// - iso boundary row (lateral surface) → move that CV row;
/// - interior trim on an OUTER loop (a cap fully bounded by the edge) → move the whole patch;
/// - interior trim on an INNER loop (a hole) → refit the 2D trim (keep the flat face), returned as
///   `(curve_2d_index, face_surface_index)` so the face doesn't lift.
fn brep_edge_watertight(b: &session_rust::BRep, edge: usize) -> (Vec<NodeAddr>, Vec<(usize, usize)>) {
    use session_rust::brep::BRepLoopType;
    let mut out: Vec<(usize, usize, usize)> = Vec::new();
    let mut refits: Vec<(usize, usize)> = Vec::new();
    for &ti in &b.m_topology_edges[edge].trim_indices {
        let trim = &b.m_trims[ti as usize];
        if trim.curve_2d_index < 0 { continue; }
        let lp = trim.loop_index as usize;
        let s = b.m_faces[b.m_loops[lp].face_index as usize].surface_index as usize;
        let srf = &b.m_surfaces[s];
        let c2d = &b.m_curves_2d[trim.curve_2d_index as usize];
        if let Some(cvs) = brep_iso_aligned_cvs(srf, c2d) {
            for (i, j) in cvs { if !out.contains(&(s, i, j)) { out.push((s, i, j)); } }
        } else if matches!(b.m_loops[lp].loop_type, BRepLoopType::Inner) {
            // Hole: refit the 2D trim, don't move the face's CVs (else the flat face lifts/curves).
            let r = (trim.curve_2d_index as usize, s);
            if !refits.contains(&r) { refits.push(r); }
        } else {
            let ni = srf.cv_count_dir(Some(0));
            let nj = srf.cv_count_dir(Some(1));
            for i in 0..ni { for j in 0..nj { if !out.contains(&(s, i, j)) { out.push((s, i, j)); } } }
        }
    }
    // Coincidence-closure: also move any CV sharing a 3D position with a moved CV. This keeps the
    // solid closed at box CORNERS (3 faces meet) and at closed-surface SEAMS (cone/cylinder/torus
    // first≡last CV column) — otherwise the unmoved coincident CV stays and the object opens.
    let seeds: Vec<session_rust::Point> = out.iter()
        .filter_map(|&(s, i, j)| b.m_surfaces[s].get_cv(i, j))
        .collect();
    for (s, srf) in b.m_surfaces.iter().enumerate() {
        let ni = srf.cv_count_dir(Some(0));
        let nj = srf.cv_count_dir(Some(1));
        for i in 0..ni {
            for j in 0..nj {
                if out.contains(&(s, i, j)) { continue; }
                if let Some(cv) = srf.get_cv(i, j) {
                    let hit = seeds.iter().any(|p| {
                        (cv[0] - p[0]).powi(2) + (cv[1] - p[1]).powi(2) + (cv[2] - p[2]).powi(2) < 1e-6
                    });
                    if hit { out.push((s, i, j)); }
                }
            }
        }
    }
    let cv_addrs = out.into_iter().map(|(s, i, j)| NodeAddr::BRepCv(s, i, j)).collect();
    (cv_addrs, refits)
}

/// Per-moved-CV basis-function influence at surface parameter (u,v):
/// `weightₖ = Nᵢ(u)·Nⱼ(v)·Wᵢⱼ / Σₐᵦ Nₐ(u)Nᵦ(v)Wₐᵦ` (the rational denominator; `=1` for
/// non-rational since the basis is a partition of unity). Lets a control-point drag deform
/// the frozen tessellation with cheap multiply-adds instead of re-evaluating `point_at`.
/// `moved` lists the moved CV indices (i,j) parallel to the returned weights.
fn surface_basis_weights(
    srf: &session_rust::NurbsSurface,
    moved: &[(usize, usize)],
    u: f64,
    v: f64,
) -> Vec<f64> {
    use session_rust::nurbsknot::{eval_basis, find_span};
    let (ou, ov) = (srf.order(0), srf.order(1));
    let (cu, cv) = (srf.cv_count_dir(Some(0)), srf.cv_count_dir(Some(1)));
    let ku = &srf.m_nurbsknot[0];
    let kv = &srf.m_nurbsknot[1];
    if ou < 2 || ov < 2 || cu < ou || cv < ov {
        return vec![0.0; moved.len()];
    }
    let su = find_span(ou, cu, ku, u);
    let sv = find_span(ov, cv, kv, v);
    let bu = eval_basis(ou, ku, su, u); // bu[a] → CV (su+a)
    let bv = eval_basis(ov, kv, sv, v); // bv[b] → CV (sv+b)
    let rational = srf.is_rational();
    let wij = |i: usize, j: usize| -> f64 {
        if rational { srf.get_cv_4d(i, j).map_or(1.0, |(_, _, _, w)| w) } else { 1.0 }
    };
    let mut denom = 0.0;
    for a in 0..ou {
        for b in 0..ov {
            denom += bu[a] * bv[b] * wij(su + a, sv + b);
        }
    }
    if denom.abs() < 1e-12 {
        return vec![0.0; moved.len()];
    }
    moved.iter().map(|&(ci, cj)| {
        if ci < su || ci >= su + ou || cj < sv || cj >= sv + ov {
            return 0.0;
        }
        bu[ci - su] * bv[cj - sv] * wij(ci, cj) / denom
    }).collect()
}

/// Column-major affine 4×4 (kernel `Xform`, f64) applied to a point, ignoring the
/// perspective row.
fn mat_apply_f64(m: &[[f64; 4]; 4], p: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * p[0] + m[1][0] * p[1] + m[2][0] * p[2] + m[3][0],
        m[0][1] * p[0] + m[1][1] * p[1] + m[2][1] * p[2] + m[3][1],
        m[0][2] * p[0] + m[1][2] * p[1] + m[2][2] * p[2] + m[3][2],
    ]
}

/// Apply an f32 matrix (GPU instance model / gumball delta) to an f64 point in f64,
/// so the f64 base keeps its precision and only the f32-grade increment is added.
fn mat_apply_mixed(m: &[[f32; 4]; 4], p: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] as f64 * p[0] + m[1][0] as f64 * p[1] + m[2][0] as f64 * p[2] + m[3][0] as f64,
        m[0][1] as f64 * p[0] + m[1][1] as f64 * p[1] + m[2][1] as f64 * p[2] + m[3][1] as f64,
        m[0][2] as f64 * p[0] + m[1][2] as f64 * p[1] + m[2][2] as f64 * p[2] + m[3][2] as f64,
    ]
}

/// Nearest forward ray/sphere intersection parameter, or None when it misses.
fn ray_sphere_t(ray: Ray, c: [f32; 3], r: f32) -> Option<f32> {
    let oc = [ray.origin[0] - c[0], ray.origin[1] - c[1], ray.origin[2] - c[2]];
    let b = 2.0 * (oc[0] * ray.direction[0] + oc[1] * ray.direction[1] + oc[2] * ray.direction[2]);
    let cc = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - r * r;
    let disc = b * b - 4.0 * cc;
    if disc < 0.0 { return None; }
    let s = disc.sqrt();
    let t0 = (-b - s) * 0.5;
    if t0 >= 0.0 { return Some(t0); }
    let t1 = (-b + s) * 0.5;
    if t1 >= 0.0 { Some(t1) } else { None }
}

/// Minimum distance between a ray P(s)=o+s·d (s ≥ 0) and a finite segment
/// Q(t)=a+t·(b−a) (t ∈ [0,1]). Robust constrained closest-point (Ericson): solve
/// the unconstrained system, clamp s to the ray, clamp t to the segment, and
/// re-solve s for a clamped t so corner cases stay correct.
fn ray_seg_dist(ray: Ray, a: [f32; 3], b: [f32; 3]) -> Option<f32> {
    let d = ray.direction;
    let e = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let r = [ray.origin[0] - a[0], ray.origin[1] - a[1], ray.origin[2] - a[2]];
    let dot = |u: [f32; 3], v: [f32; 3]| u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
    let aa = dot(d, d);            // ray dir length² (≈1, normalized)
    let ee = dot(e, e);            // segment length²
    if ee < 1e-10 || aa < 1e-12 { return None; }
    let b_ = dot(d, e);
    let c_ = dot(d, r);
    let f_ = dot(e, r);
    let denom = aa * ee - b_ * b_;
    // s minimizes along the ray (parallel → s = 0); clamp to the forward half.
    let mut s = if denom.abs() > 1e-12 { (b_ * f_ - c_ * ee) / denom } else { 0.0 };
    s = s.max(0.0);
    // t from s, clamped to the segment; if clamped, re-solve s for that fixed t.
    let mut t = (s * b_ + f_) / ee;
    if t < 0.0 {
        t = 0.0;
        s = (-c_ / aa).max(0.0);
    } else if t > 1.0 {
        t = 1.0;
        s = ((b_ - c_) / aa).max(0.0);
    }
    let px = ray.origin[0] + d[0] * s - (a[0] + e[0] * t);
    let py = ray.origin[1] + d[1] * s - (a[1] + e[1] * t);
    let pz = ray.origin[2] + d[2] * s - (a[2] + e[2] * t);
    Some((px * px + py * py + pz * pz).sqrt())
}
