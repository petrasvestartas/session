impl State {

    fn process_pick(&mut self, cx: f32, cy: f32) {
        let view = self.camera.view_matrix();
        let proj = self.camera.proj_matrix();
        let viewport = (self.config.width as f32, self.config.height as f32);
        let is_ortho = self.camera.proj_mode == ProjMode::Ortho;
        let ray = screen_to_world_ray(&view, &proj, viewport, (cx, cy), is_ortho);

        let gumball_hit = self.gumball.as_ref()
            .and_then(|gb| gb.hit_test(ray, self.gumball_scale));
        if let Some(handle) = gumball_hit {
            self.drag_origins.clear();
            for guid in &self.selected_guids {
                if let Some(iid) = self.gpu_session.pick.instance_id(guid) {
                    let model = self.gpu_session.instances_cpu[iid as usize].model;
                    self.drag_origins.insert(guid.clone(), model);
                }
            }
            let origin = self.gumball.as_ref().unwrap().origin;
            let ds = gumball::begin_drag(handle, ray, origin, self.gumball_scale);
            self.gumball.as_mut().unwrap().drag = Some(ds);
            return;
        }

        let pick_radius = self.camera.pick_radius_mm(self.config.height as f32, 8.0)
            .max(crate::gpu_adapters::SPHERE_RADIUS);
        let hits     = pick::pick_by_ray(&mut self.session, ray, pick_radius);
        let origin_pt  = session_rust::Point::new(ray.origin[0], ray.origin[1], ray.origin[2]);
        let dir_vec    = session_rust::Vector::new(ray.direction[0], ray.direction[1], ray.direction[2]);
        let nurbs_hits = self.gpu_session.pick_nurbssurfaces(&origin_pt, &dir_vec);
        let brep_hits  = self.gpu_session.pick_breps(&origin_pt, &dir_vec);
        let nc_hits    = self.gpu_session.pick_nurbscurves(&origin_pt, &dir_vec, pick_radius);
        log::info!("PICK hits={} nurbs={} brep={} nc={}", hits.len(), nurbs_hits.len(), brep_hits.len(), nc_hits.len());

        // Prefer solid geometry (Mesh/OBB) over thin geometry (Line/Polyline/Point)
        // when both are hit at similar depth. Thin geometry only wins if it is
        // clearly closer (by more than pick_radius) than any solid hit.
        let first_solid = hits.iter().find(|h| {
            !self.hidden_guids.contains(h.guid()) && matches!(
                self.session.lookup.get(h.guid()),
                Some(Geometry::Mesh(_)) | Some(Geometry::OBB(_))
            )
        });
        let first_any = hits.iter().find(|h| !self.hidden_guids.contains(h.guid()));
        let from_session = match (first_solid, first_any) {
            (Some(solid), Some(any)) if solid.guid() != any.guid() => {
                if any.distance + pick_radius < solid.distance {
                    Some(any.guid().to_string())
                } else {
                    Some(solid.guid().to_string())
                }
            }
            (_, Some(h)) => Some(h.guid().to_string()),
            _ => None,
        };

        let new_guid = from_session
            .or_else(|| nurbs_hits.iter()
                .find(|(g,_)| !self.hidden_guids.contains(g.as_str()))
                .map(|(g,_)| g.clone()))
            .or_else(|| brep_hits.iter()
                .find(|(g,_)| !self.hidden_guids.contains(g.as_str()))
                .map(|(g,_)| g.clone()))
            .or_else(|| nc_hits.iter()
                .find(|(g,_)| !self.hidden_guids.contains(g.as_str()))
                .map(|(g,_)| g.clone()));
        // Expand single picked guid to its locked group (if any)
        let pick_guids: Vec<String> = if let (Some(guid), Some(root)) = (&new_guid, self.session.tree.root()) {
            if let Some(locked_node) = locked_group_for_guid(&root, guid, &self.group_locked, &self.geom_guid_set) {
                collect_group_leaves(&locked_node, &self.geom_guid_set)
            } else {
                vec![guid.clone()]
            }
        } else if let Some(guid) = &new_guid {
            vec![guid.clone()]
        } else {
            vec![]
        };

        let shift = self.controller.select_add();
        if shift {
            let all_sel = !pick_guids.is_empty() && pick_guids.iter().all(|g| self.selected_guids.contains(g));
            for guid in &pick_guids {
                if all_sel {
                    self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, false, &self.queue);
                    self.selected_guids.remove(guid);
                } else {
                    self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                    self.selected_guids.insert(guid.clone());
                }
            }
        } else {
            let prev: Vec<String> = self.selected_guids.drain().collect();
            for p in &prev {
                self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
            }
            if !pick_guids.is_empty() {
                let reclick = pick_guids.len() == prev.len()
                    && pick_guids.iter().all(|g| prev.contains(g));
                if !reclick {
                    for guid in &pick_guids {
                        self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                        self.selected_guids.insert(guid.clone());
                    }
                }
            }
        }

        if !self.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            match &mut self.gumball {
                Some(gb) => gb.set_origin(origin),
                None     => self.gumball = Some(Gumball::new(origin)),
            }
        } else {
            self.gumball = None;
        }

    }


    /// Center of the AABB union over all selected objects.
    fn selected_centroid(&self) -> [f32; 3] {
        let mut mn = [f32::MAX;  3];
        let mut mx = [f32::MIN; 3];
        let mut found = false;
        for guid in &self.selected_guids {
            if let Some(idx) = self.session.cached_guids.iter().position(|g| g == guid) {
                if idx < self.session.cached_boxes.len() {
                    for corner in &self.session.cached_boxes[idx].corners() {
                        for i in 0..3 {
                            let v = corner[i] as f32;
                            if v < mn[i] { mn[i] = v; }
                            if v > mx[i] { mx[i] = v; }
                        }
                    }
                    found = true;
                }
            } else if let Some(mesh) = self.gpu_session.nurbs_pick_meshes.get(guid) {
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    for (i, c) in [v.x, v.y, v.z].iter().enumerate() {
                        if *c < mn[i] { mn[i] = *c; }
                        if *c > mx[i] { mx[i] = *c; }
                    }
                    found = true;
                }
            } else if let Some((mesh, xf)) = self.gpu_session.brep_pick_meshes.get(guid) {
                // BRep local-space mesh transformed by xf to world space.
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    let wx = xf[0][0]*v.x + xf[1][0]*v.y + xf[2][0]*v.z + xf[3][0];
                    let wy = xf[0][1]*v.x + xf[1][1]*v.y + xf[2][1]*v.z + xf[3][1];
                    let wz = xf[0][2]*v.x + xf[1][2]*v.y + xf[2][2]*v.z + xf[3][2];
                    for (i, c) in [wx, wy, wz].iter().enumerate() {
                        if *c < mn[i] { mn[i] = *c; }
                        if *c > mx[i] { mx[i] = *c; }
                    }
                    found = true;
                }
            } else if let Some(pts) = self.gpu_session.nc_pick_pts.get(guid) {
                // NurbsCurve — use polyline AABB.
                for p in pts {
                    for i in 0..3 {
                        if p[i] < mn[i] { mn[i] = p[i]; }
                        if p[i] > mx[i] { mx[i] = p[i]; }
                    }
                    found = true;
                }
            }
        }
        if found { [(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5] } else { [0.0; 3] }
    }
}
