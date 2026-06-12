use crate::State;
use crate::camera::ProjMode;
use crate::pick::{self, screen_to_world_ray};
use crate::tree_ui::{collect_group_leaves, locked_group_for_guid};

impl State {

    pub(crate) fn process_box_select(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        use session_rust::session::Geometry;
        let vp = self.scene.camera.view_proj();
        let (rx, ry, rw, rh) = self.vp_rect();
        let min_x = x0.min(x1);
        let max_x = x0.max(x1);
        let min_y = y0.min(y1);
        let max_y = y0.max(y1);

        let project = |pt: [f32; 3]| -> Option<(f32, f32)> {
            let cx = vp[0][0]*pt[0] + vp[1][0]*pt[1] + vp[2][0]*pt[2] + vp[3][0];
            let cy = vp[0][1]*pt[0] + vp[1][1]*pt[1] + vp[2][1]*pt[2] + vp[3][1];
            let cw = vp[0][3]*pt[0] + vp[1][3]*pt[1] + vp[2][3]*pt[2] + vp[3][3];
            if cw <= 0.0 { return None; }
            Some((rx + (cx/cw + 1.0) * 0.5 * rw, ry + (1.0 - cy/cw) * 0.5 * rh))
        };
        let apply_model = |m: [[f32; 4]; 4], lc: [f32; 3]| -> [f32; 3] {
            [
                m[0][0]*lc[0] + m[1][0]*lc[1] + m[2][0]*lc[2] + m[3][0],
                m[0][1]*lc[0] + m[1][1]*lc[1] + m[2][1]*lc[2] + m[3][1],
                m[0][2]*lc[0] + m[1][2]*lc[1] + m[2][2]*lc[2] + m[3][2],
            ]
        };
        let in_rect = |(sx, sy): (f32, f32)| sx >= min_x && sx <= max_x && sy >= min_y && sy <= max_y;

        let mut hits: Vec<String> = Vec::new();

        for (guid, geom) in &self.scene.session.lookup {
            if self.scene.hidden_guids.contains(guid) { continue; }
            let local: Option<[f32; 3]> = match geom {
                Geometry::Point(p) => Some([p[0] as f32, p[1] as f32, p[2] as f32]),
                Geometry::Line(l) => {
                    let s = l.start(); let e = l.end();
                    Some([(s[0]+e[0]) as f32*0.5, (s[1]+e[1]) as f32*0.5, (s[2]+e[2]) as f32*0.5])
                }
                Geometry::Polyline(pl) => {
                    let n = pl.coords.len() / 3;
                    if n == 0 { None } else {
                        let nf = n as f32;
                        let (mut sx, mut sy, mut sz) = (0.0f32, 0.0, 0.0);
                        for c in pl.coords.chunks(3) { sx += c[0] as f32; sy += c[1] as f32; sz += c[2] as f32; }
                        Some([sx/nf, sy/nf, sz/nf])
                    }
                }
                Geometry::Mesh(m) => {
                    if m.vertex.is_empty() { None } else {
                        let n = m.vertex.len() as f32;
                        let cx = m.vertex.values().map(|v| v.x as f32).sum::<f32>() / n;
                        let cy = m.vertex.values().map(|v| v.y as f32).sum::<f32>() / n;
                        let cz = m.vertex.values().map(|v| v.z as f32).sum::<f32>() / n;
                        Some([cx, cy, cz])
                    }
                }
                Geometry::Plane(pl) => { let o = pl.origin(); Some([o[0] as f32, o[1] as f32, o[2] as f32]) }
                Geometry::PointCloud(pc) => {
                    let pts = pc.get_points();
                    if pts.is_empty() { None } else {
                        let n = pts.len() as f32;
                        let (mut sx, mut sy, mut sz) = (0.0f32, 0.0, 0.0);
                        for p in &pts { sx += p[0] as f32; sy += p[1] as f32; sz += p[2] as f32; }
                        Some([sx/n, sy/n, sz/n])
                    }
                }
                Geometry::OBB(bb) => {
                    let corners = bb.corners();
                    if corners.is_empty() { None } else {
                        let n = corners.len() as f32;
                        let (mut sx, mut sy, mut sz) = (0.0f32, 0.0, 0.0);
                        for p in &corners { sx += p[0] as f32; sy += p[1] as f32; sz += p[2] as f32; }
                        Some([sx/n, sy/n, sz/n])
                    }
                }
                _ => None,
            };
            let world = if let Some(lc) = local {
                if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                    apply_model(self.scene.gpu_session.instances_cpu[iid as usize].model, lc)
                } else { lc }
            } else if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                let m = self.scene.gpu_session.instances_cpu[iid as usize].model;
                [m[3][0], m[3][1], m[3][2]]
            } else { continue };
            if project(world).map(in_rect).unwrap_or(false) { hits.push(guid.clone()); }
        }

        for ns in &self.scene.session.objects.nurbssurfaces {
            let guid = ns.guid().to_string();
            if self.scene.hidden_guids.contains(&guid) { continue; }
            let local = self.scene.gpu_session.nurbs_pick_meshes.get(&guid).and_then(|(m, _xf)| {
                if m.vertex.is_empty() { return None; }
                let n = m.vertex.len() as f32;
                // Local mesh centroid; the instance model is applied below to get world.
                Some([
                    m.vertex.values().map(|v| v.x as f32).sum::<f32>() / n,
                    m.vertex.values().map(|v| v.y as f32).sum::<f32>() / n,
                    m.vertex.values().map(|v| v.z as f32).sum::<f32>() / n,
                ])
            });
            if let Some(lc) = local {
                let world = if let Some(iid) = self.scene.gpu_session.pick.instance_id(&guid) {
                    apply_model(self.scene.gpu_session.instances_cpu[iid as usize].model, lc)
                } else { lc };
                if project(world).map(in_rect).unwrap_or(false) { hits.push(guid); }
            }
        }

        for nc in &self.scene.session.objects.nurbscurves {
            let guid = nc.guid().to_string();
            if self.scene.hidden_guids.contains(&guid) { continue; }
            let local = self.scene.gpu_session.nc_pick_pts.get(&guid).and_then(|pts| {
                if pts.is_empty() { return None; }
                let n = pts.len() as f32;
                let (mut sx, mut sy, mut sz) = (0.0f32, 0.0, 0.0);
                for p in pts { sx += p[0]; sy += p[1]; sz += p[2]; }
                Some([sx/n, sy/n, sz/n])
            });
            if let Some(lc) = local {
                let world = if let Some(iid) = self.scene.gpu_session.pick.instance_id(&guid) {
                    apply_model(self.scene.gpu_session.instances_cpu[iid as usize].model, lc)
                } else { lc };
                if project(world).map(in_rect).unwrap_or(false) { hits.push(guid); }
            }
        }

        let refs: Vec<&str> = hits.iter().map(|s| s.as_str()).collect();
        self.set_selection(&refs);
        if !hits.is_empty() { self.scene.reveal_in_tree = true; }
    }

    pub(crate) fn process_pick(&mut self, cx: f32, cy: f32) {
        let view = self.scene.camera.view_matrix();
        let proj = self.scene.camera.proj_matrix();
        let viewport = self.vp_rect();
        let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
        let ray = screen_to_world_ray(&view, &proj, viewport, (cx, cy), is_ortho);

        // Edit mode reroutes picking to sub-objects (control points / edges).
        if self.edit.active {
            self.process_pick_edit(ray, cx, cy);
            return;
        }

        // Ctrl+Shift+Left-click (no F10) → select & move a boundary edge.
        if self.scene.ctrl_down && self.scene.shift_down {
            self.process_pick_edge(ray);
            return;
        }

        let gumball_hit = self.gb.gumball.as_ref()
            .and_then(|gb| gb.hit_test(ray, self.gb.gumball_scale));
        if let Some(handle) = gumball_hit {
            self.gb.gumball_input = None; // a fresh handle press cancels any open popup
            self.gb.gumball_press = None;
            self.gb.drag_origins.clear();
            self.gb.drag_geom_snapshots.clear();
            self.gb.drag_nurbs_snapshots.clear();
            for guid in &self.scene.selected_guids {
                if self.scene.transform_locked.contains(guid) { continue; }
                if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                    let model = self.scene.gpu_session.instances_cpu[iid as usize].model;
                    self.gb.drag_origins.insert(guid.clone(), model);
                }
                // Snapshot pre-drag state for non-BRep lookup types (geometry gets baked on
                // commit so undo restores exact coords). BRep and NurbsSurface move
                // matrix-only — they carry NO snapshot, so undo/redo replays the model
                // matrix instead (see apply_undo/apply_redo `None` arm).
                if let Some(geom) = self.scene.session.lookup.get(guid) {
                    if !matches!(geom, session_rust::session::Geometry::BRep(_)) {
                        self.gb.drag_geom_snapshots.insert(guid.clone(), geom.clone());
                    }
                }
            }
            // Nothing transformable (whole selection transform-locked): don't engage
            // the gumball — fall through to a normal ray pick below.
            if !self.gb.drag_origins.is_empty() {
                if self.scene.lmb_down {
                    // Button still held: defer the click-vs-drag decision. A press that
                    // never moves past the threshold opens the popup on release; movement
                    // starts a drag (see handle_mouse_moved / handle_mouse_button).
                    self.gb.gumball_press = Some((handle, cx as f64, cy as f64));
                } else {
                    // The button was already released before this deferred pick ran
                    // (fast tap with no movement) — that is a click: open the popup now.
                    self.open_gumball_input(handle, cx as f64, cy as f64);
                }
                return;
            }
        }

        let pick_radius = self.scene.camera.pick_radius_mm(self.vp_rect().3, 8.0)
            .max(crate::gpu_adapters::SPHERE_RADIUS);
        let hits = pick::pick_by_ray(&mut self.scene.session, ray, pick_radius);
        let origin_pt = session_rust::Point::new(ray.origin[0] as f64, ray.origin[1] as f64, ray.origin[2] as f64);
        let dir_vec   = session_rust::Vector::new(ray.direction[0] as f64, ray.direction[1] as f64, ray.direction[2] as f64);
        let nurbs_hits = self.scene.gpu_session.pick_nurbssurfaces(&origin_pt, &dir_vec);
        let brep_hits  = self.scene.gpu_session.pick_breps(&origin_pt, &dir_vec);
        let trimmed_hits = self.scene.gpu_session.pick_nurbssurfacetrimmeds(&origin_pt, &dir_vec);
        // Also allow grabbing a trimmed surface by clicking near its edge curves (cut/seam).
        let trimmed_edge_hits = self.scene.gpu_session.pick_trimmed_edges(&origin_pt, &dir_vec, pick_radius);
        let nc_hits    = self.scene.gpu_session.pick_nurbscurves(&origin_pt, &dir_vec, pick_radius);

        // Combine all hits, filter hidden, pick closest
        let mut best_guid: Option<String> = None;
        let mut best_dist = f32::MAX;

        for hit in &hits {
            if self.scene.hidden_guids.contains(hit.guid()) { continue; }
            if (hit.distance as f32) < best_dist {
                best_dist = hit.distance as f32;
                best_guid = Some(hit.guid().to_string());
            }
        }

        let pt_dist = |p: &session_rust::Point| -> f32 {
            let dx = p[0] - origin_pt[0];
            let dy = p[1] - origin_pt[1];
            let dz = p[2] - origin_pt[2];
            (dx*dx + dy*dy + dz*dz).sqrt() as f32
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
        if let Some((guid, pt)) = trimmed_hits.into_iter().next() {
            if !self.scene.hidden_guids.contains(&guid) {
                let d = pt_dist(&pt);
                if d < best_dist { best_dist = d; best_guid = Some(guid); }
            }
        }
        if let Some((guid, pt)) = trimmed_edge_hits.into_iter().next() {
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

        let Some(raw_guid) = best_guid else {
            self.set_selection(&[]);
            return;
        };

        // Expand to locked group if any
        let final_guids: Vec<String> = if let Some(root) = self.scene.session.tree.root() {
            if let Some(locked_node) = locked_group_for_guid(
                &root, &raw_guid, &self.scene.group_locked, &self.scene.geom_guid_set,
            ) {
                collect_group_leaves(&locked_node, &self.scene.geom_guid_set)
            } else {
                vec![raw_guid]
            }
        } else {
            vec![raw_guid]
        };

        let refs: Vec<&str> = final_guids.iter().map(|s| s.as_str()).collect();
        self.set_selection(&refs);
        self.scene.reveal_in_tree = true;
    }

    pub fn selected_centroid(&self) -> [f32; 3] {
        use session_rust::session::Geometry;
        let mut sum = [0.0f32; 3];
        let mut count = 0usize;
        for guid in &self.scene.selected_guids {
            let local = if let Some(geom) = self.scene.session.lookup.get(guid) {
                match geom {
                    Geometry::Point(p) => Some([p[0] as f32, p[1] as f32, p[2] as f32]),
                    Geometry::Line(l) => {
                        let s = l.start(); let e = l.end();
                        Some([(s[0]+e[0]) as f32 * 0.5, (s[1]+e[1]) as f32 * 0.5, (s[2]+e[2]) as f32 * 0.5])
                    }
                    Geometry::Polyline(pl) => {
                        let pts = pl.get_points();
                        if pts.is_empty() { None } else {
                            let n = pts.len() as f32;
                            let (mut sx, mut sy, mut sz) = (0.0f32, 0.0f32, 0.0f32);
                            for p in &pts { sx += p[0] as f32; sy += p[1] as f32; sz += p[2] as f32; }
                            Some([sx/n, sy/n, sz/n])
                        }
                    }
                    Geometry::Mesh(m) => {
                        if m.vertex.is_empty() { None } else {
                            let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                            for v in m.vertex.values() {
                                let (x, y, z) = (v.x as f32, v.y as f32, v.z as f32);
                                mn[0] = mn[0].min(x); mn[1] = mn[1].min(y); mn[2] = mn[2].min(z);
                                mx[0] = mx[0].max(x); mx[1] = mx[1].max(y); mx[2] = mx[2].max(z);
                            }
                            Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                        }
                    }
                    // BRep (cyl/box/sphere via BRep::create_*) lives in `lookup` but its
                    // local origin is the base/anchor, not the centre. Use the bbox centre
                    // of its pick mesh (local coords; the outer block applies the instance
                    // model, which equals the pick-mesh xform) so the gumball sits centred.
                    Geometry::BRep(_) => {
                        self.scene.gpu_session.brep_pick_meshes.get(guid).and_then(|(mesh, _xf)| {
                            if mesh.vertex.is_empty() { return None; }
                            let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                            for v in mesh.vertex.values() {
                                let (x, y, z) = (v.x as f32, v.y as f32, v.z as f32);
                                mn[0] = mn[0].min(x); mn[1] = mn[1].min(y); mn[2] = mn[2].min(z);
                                mx[0] = mx[0].max(x); mx[1] = mx[1].max(y); mx[2] = mx[2].max(z);
                            }
                            Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                        })
                    }
                    Geometry::PointCloud(pc) => {
                        let pts = pc.get_points();
                        if pts.is_empty() { None } else {
                            let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                            for p in &pts {
                                for k in 0..3 { let c = p[k] as f32; mn[k] = mn[k].min(c); mx[k] = mx[k].max(c); }
                            }
                            Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                        }
                    }
                    Geometry::OBB(o) => {
                        let corners = o.corners();
                        if corners.is_empty() { None } else {
                            let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                            for p in &corners {
                                for k in 0..3 { let c = p[k] as f32; mn[k] = mn[k].min(c); mx[k] = mx[k].max(c); }
                            }
                            Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                        }
                    }
                    Geometry::Plane(pl) => { let o = pl.origin(); Some([o[0] as f32, o[1] as f32, o[2] as f32]) }
                    _ => None,
                }
            } else if let Some(ns) = self.scene.session.objects.nurbssurfaces.iter().find(|n| n.guid() == *guid) {
                // NurbsSurface lives outside `lookup`; centre the gumball on its
                // control-point AABB (matches the "bbox centre" rule used for meshes)
                // instead of falling through to the identity model translation (XY origin).
                let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                for i in 0..ns.cv_count_dir(Some(0)) {
                    for j in 0..ns.cv_count_dir(Some(1)) {
                        if let Some(p) = ns.get_cv(i, j) {
                            for k in 0..3 { mn[k] = mn[k].min(p[k] as f32); mx[k] = mx[k].max(p[k] as f32); }
                        }
                    }
                }
                if mn[0] <= mx[0] {
                    Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                } else { None }
            } else if let Some((mesh, _xf)) = self.scene.gpu_session.nurbs_trimmed_pick_meshes.get(guid) {
                // NurbsSurfaceTrimmed also lives outside `lookup`; centre the gumball on its
                // tessellation bbox (local coords; the outer block applies the instance model).
                if mesh.vertex.is_empty() { None } else {
                    let (mut mn, mut mx) = ([f32::MAX; 3], [f32::MIN; 3]);
                    for v in mesh.vertex.values() {
                        let (x, y, z) = (v.x as f32, v.y as f32, v.z as f32);
                        mn[0] = mn[0].min(x); mn[1] = mn[1].min(y); mn[2] = mn[2].min(z);
                        mx[0] = mx[0].max(x); mx[1] = mx[1].max(y); mx[2] = mx[2].max(z);
                    }
                    Some([(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5])
                }
            } else { None };

            if let Some(lc) = local {
                if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                    let m = self.scene.gpu_session.instances_cpu[iid as usize].model;
                    sum[0] += m[0][0]*lc[0]+m[1][0]*lc[1]+m[2][0]*lc[2]+m[3][0];
                    sum[1] += m[0][1]*lc[0]+m[1][1]*lc[1]+m[2][1]*lc[2]+m[3][1];
                    sum[2] += m[0][2]*lc[0]+m[1][2]*lc[1]+m[2][2]*lc[2]+m[3][2];
                } else {
                    sum[0] += lc[0]; sum[1] += lc[1]; sum[2] += lc[2];
                }
                count += 1;
            } else if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                let m = self.scene.gpu_session.instances_cpu[iid as usize].model;
                sum[0] += m[3][0]; sum[1] += m[3][1]; sum[2] += m[3][2];
                count += 1;
            }
        }
        if count > 0 { [sum[0]/count as f32, sum[1]/count as f32, sum[2]/count as f32] } else { [0.0; 3] }
    }
}
