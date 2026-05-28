impl State {

    fn process_pick(&mut self, cx: f32, cy: f32) {
        let view = self.scene.camera.view_matrix();
        let proj = self.scene.camera.proj_matrix();
        let viewport = (self.gpu.config.width as f32, self.gpu.config.height as f32);
        let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
        let ray = screen_to_world_ray(&view, &proj, viewport, (cx, cy), is_ortho);

        let gumball_hit = self.gb.gumball.as_ref()
            .and_then(|gb| gb.hit_test(ray, self.gb.gumball_scale));
        if let Some(handle) = gumball_hit {
            self.gb.drag_origins.clear();
            for guid in &self.scene.selected_guids {
                if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                    let model = self.scene.gpu_session.instances_cpu[iid as usize].model;
                    self.gb.drag_origins.insert(guid.clone(), model);
                }
            }
            let origin = self.gb.gumball.as_ref().unwrap().origin;
            let ds = gumball::begin_drag(handle, ray, origin, self.gb.gumball_scale);
            self.gb.gumball.as_mut().unwrap().drag = Some(ds);
            return;
        }

        let pick_radius = self.scene.camera.pick_radius_mm(self.gpu.config.height as f32, 8.0)
            .max(crate::gpu_adapters::SPHERE_RADIUS);
        let hits = pick::pick_by_ray(&mut self.scene.session, ray, pick_radius);
        let origin_pt = session_rust::Point::new(ray.origin[0], ray.origin[1], ray.origin[2]);
        let dir_vec   = session_rust::Vector::new(ray.direction[0], ray.direction[1], ray.direction[2]);
        let nurbs_hits = self.scene.gpu_session.pick_nurbssurfaces(&origin_pt, &dir_vec);
        let brep_hits  = self.scene.gpu_session.pick_breps(&origin_pt, &dir_vec);
        let nc_hits    = self.scene.gpu_session.pick_nurbscurves(&origin_pt, &dir_vec, pick_radius);

        // Combine all hits, filter hidden, pick closest
        let mut best_guid: Option<String> = None;
        let mut best_dist = f32::MAX;

        for hit in &hits {
            if self.scene.hidden_guids.contains(hit.guid()) { continue; }
            if hit.distance < best_dist {
                best_dist = hit.distance;
                best_guid = Some(hit.guid().to_string());
            }
        }

        let pt_dist = |p: &session_rust::Point| -> f32 {
            let dx = p[0] - origin_pt[0];
            let dy = p[1] - origin_pt[1];
            let dz = p[2] - origin_pt[2];
            (dx*dx + dy*dy + dz*dz).sqrt()
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

        let Some(raw_guid) = best_guid else { return; };

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
    }

    pub fn selected_centroid(&self) -> [f32; 3] {
        let mut sum = [0.0f32; 3];
        let mut count = 0usize;
        for guid in &self.scene.selected_guids {
            if let Some(iid) = self.scene.gpu_session.pick.instance_id(guid) {
                let m = self.scene.gpu_session.instances_cpu[iid as usize].model;
                sum[0] += m[3][0]; sum[1] += m[3][1]; sum[2] += m[3][2];
                count += 1;
            }
        }
        if count > 0 {
            [sum[0]/count as f32, sum[1]/count as f32, sum[2]/count as f32]
        } else {
            [0.0; 3]
        }
    }
}
