impl State {

    fn reapply_visibility_flags(&mut self, guid: &str) {
        if self.scene.hidden_guids.contains(guid) {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_HIDDEN, true, &self.gpu.queue);
        }

        if self.scene.glyphs_hidden_guids.contains(guid) {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &self.gpu.queue);
        }
    }

    fn commit_object_transform(&mut self, guid: &str, model: [[f32; 4]; 4]) {
        let flat = [
            model[0][0], model[0][1], model[0][2], model[0][3],
            model[1][0], model[1][1], model[1][2], model[1][3],
            model[2][0], model[2][1], model[2][2], model[2][3],
            model[3][0], model[3][1], model[3][2], model[3][3],
        ];
        let xf = Xform::from_matrix(flat);
        if let Some(geom) = self.scene.session.lookup.get_mut(guid) {
            match geom {
                Geometry::Mesh(m)        => { m.transform(Some(&xf)); }
                Geometry::Point(p)       => { p.xform = xf.clone(); p.transform(); }
                Geometry::Line(l)        => { l.xform = xf.clone(); l.transform(); }
                Geometry::Polyline(pl)   => { pl.xform = xf.clone(); pl.transform(); }
                Geometry::Plane(pl)      => { pl.xform = xf.clone(); pl.transform(); }
                Geometry::PointCloud(pc) => { pc.xform = xf.clone(); pc.transform(); }
                Geometry::OBB(o)         => { o.xform = xf.clone(); o.transform(); }
                Geometry::BRep(b)        => { b.xform = xf.clone(); }
                _ => {}
            }
        }
        self.scene.session.cached_boxes.clear();
        self.scene.session.cached_guids.clear();
        self.scene.session.invalidate_bvh_cache();
        // NurbsSurface objects live in session.objects.nurbssurfaces, not lookup.
        // Bake the model matrix into the surface control points and re-upload.
        if self.scene.gpu_session.nurbs_pick_meshes.contains_key(guid) {
            let flat = [
                model[0][0], model[0][1], model[0][2], model[0][3],
                model[1][0], model[1][1], model[1][2], model[1][3],
                model[2][0], model[2][1], model[2][2], model[2][3],
                model[3][0], model[3][1], model[3][2], model[3][3],
            ];
            let xf = Xform::from_matrix(flat);
            let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
            if let Some(ns) = self.scene.session.objects.nurbssurfaces.iter_mut().find(|n| n.guid() == guid) {
                ns.transform(&xf);
                let ns_clone = ns.clone();
                self.scene.gpu_session.remove(guid);
                self.scene.gpu_session.add_nurbssurface(&ns_clone, &self.gpu.device, &self.gpu.queue);
            }
            if was_selected {
                self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            }
            self.apply_thickness();
            return;
        }
        // BRep: xform was updated in the match above; only the GPU model matrix needs
        // updating — no re-tessellation required.
        if matches!(self.scene.session.lookup.get(guid), Some(Geometry::BRep(_))) {
            let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
            self.scene.gpu_session.update_transform(guid, model, &self.gpu.queue);
            if let Some((_, xf)) = self.scene.gpu_session.brep_pick_meshes.get_mut(guid) {
                *xf = model;
            }
            if was_selected {
                self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            }
            self.apply_thickness();
            self.scene.text_labels = labels_from_session(&self.scene.session);
            return;
        }
        let was_selected = self.scene.gpu_session.pick.instance_id(guid)
            .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
            .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
        self.scene.gpu_session.remove(guid);
        if let Some(geom) = self.scene.session.lookup.remove(guid) {
            self.scene.gpu_session.add_geometry(guid, &geom, &self.gpu.device, &self.gpu.queue);
            self.scene.session.lookup.insert(guid.to_string(), geom);
        }
        if was_selected {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
        }
        self.reapply_visibility_flags(guid);
        self.apply_thickness();
        self.scene.text_labels = labels_from_session(&self.scene.session);
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if !pressed {
                let was_dragging = self.gb.gumball.as_ref().map_or(false, |gb| gb.drag.is_some());
                if was_dragging {
                    if let Some(gb) = &mut self.gb.gumball {
                        gb.drag = None;
                    }
                    let to_commit: Vec<(String, [[f32; 4]; 4])> = self.gb.drag_origins.keys()
                        .filter_map(|guid| {
                            self.scene.gpu_session.pick.instance_id(guid).map(|iid| {
                                (guid.clone(), self.scene.gpu_session.instances_cpu[iid as usize].model)
                            })
                        })
                        .collect();
                    // Collect before/after for undo
                    let undo_objects: Vec<(String, [[f32; 4]; 4], [[f32; 4]; 4])> = to_commit.iter()
                        .filter_map(|(guid, after)| {
                            self.gb.drag_origins.get(guid).map(|before| (guid.clone(), *before, *after))
                        })
                        .collect();
                    for (guid, model) in to_commit {
                        self.commit_object_transform(&guid, model);
                    }
                    if !undo_objects.is_empty() {
                        self.hist.push(UndoAction::Transform { objects: undo_objects });
                    }
                    self.gb.drag_origins.clear();
                    return;
                }
            }
            if pressed {
                self.scene.pending_pick = Some(self.scene.mouse_position);
            }
        }
        self.scene.controller.process_mouse_button(button, pressed);
    }

    pub fn handle_mouse_moved(&mut self, x: f64, y: f64) {
        let (px, py) = self.scene.mouse_position;
        self.scene.mouse_position = (x, y);

        let drag_info = self.gb.gumball.as_ref().and_then(|gb| {
            gb.drag.as_ref().map(|ds| (ds.clone(), gb.origin))
        });
        if let Some((ds, origin)) = drag_info {
            let view = self.scene.camera.view_matrix();
            let proj = self.scene.camera.proj_matrix();
            let vp = (self.gpu.config.width as f32, self.gpu.config.height as f32);
            let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gb.gumball_scale;
            if let Some(delta) = gumball::update_drag(&ds, ray, origin, scale) {
                for (guid, orig) in &self.gb.drag_origins {
                    let new_model = mat4_mul_cm(&delta, orig);
                    self.scene.gpu_session.update_transform(guid, new_model, &self.gpu.queue);
                }
                if matches!(ds.handle, HandleKind::TranslateX | HandleKind::TranslateY | HandleKind::TranslateZ) {
                    if let Some(gb) = &mut self.gb.gumball {
                        gb.origin = [
                            ds.drag_start_origin[0] + delta[3][0],
                            ds.drag_start_origin[1] + delta[3][1],
                            ds.drag_start_origin[2] + delta[3][2],
                        ];
                    }
                }
            }
            return;
        }

        if self.gb.gumball.is_some() {
            let view = self.scene.camera.view_matrix();
            let proj = self.scene.camera.proj_matrix();
            let vp = (self.gpu.config.width as f32, self.gpu.config.height as f32);
            let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gb.gumball_scale;
            if let Some(gb) = &mut self.gb.gumball {
                gb.hovered = gb.hit_test(ray, scale);
            }
        }

        self.scene.controller.process_mouse_move((x - px) as f32, (y - py) as f32);
    }

    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scene.controller.process_scroll(delta);
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        } else if code == KeyCode::KeyZ && is_pressed && self.scene.key_mods.control_key() {
            self.undo();
        } else if code == KeyCode::KeyU && is_pressed && self.scene.key_mods.control_key() {
            self.redo();
        } else if code == KeyCode::KeyF && is_pressed {
            self.fit_view();
        } else if code == KeyCode::KeyQ && is_pressed {
            self.scene.shading_enabled = !self.scene.shading_enabled;
        } else if code == KeyCode::KeyE && is_pressed {
            self.scene.backface_highlight = !self.scene.backface_highlight;
        } else {
            self.scene.controller.process_key(code, is_pressed);
        }
    }

    fn fit_view(&mut self) {
        let mut mn = [f32::MAX; 3];
        let mut mx = [f32::MIN; 3];
        let mut found = false;
        if self.scene.selected_guids.is_empty() {
            for bbox in &self.scene.session.cached_boxes {
                for corner in &bbox.corners() {
                    for i in 0..3 {
                        let v = corner[i] as f32;
                        if v < mn[i] { mn[i] = v; }
                        if v > mx[i] { mx[i] = v; }
                    }
                    found = true;
                }
            }
            if !found { self.scene.camera.reset(); return; }
            let center = [(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5];
            let half_diag = (
                (mx[0]-mn[0]).powi(2) +
                (mx[1]-mn[1]).powi(2) +
                (mx[2]-mn[2]).powi(2)
            ).sqrt() * 0.5;
            self.scene.camera.fit_to_box(center, half_diag.max(50.0));
            return;
        }
        for guid in &self.scene.selected_guids {
            if let Some(idx) = self.scene.session.cached_guids.iter().position(|g| g == guid) {
                if idx < self.scene.session.cached_boxes.len() {
                    for corner in &self.scene.session.cached_boxes[idx].corners() {
                        for i in 0..3 {
                            let v = corner[i] as f32;
                            if v < mn[i] { mn[i] = v; }
                            if v > mx[i] { mx[i] = v; }
                        }
                    }
                    found = true;
                }
            } else if let Some(mesh) = self.scene.gpu_session.nurbs_pick_meshes.get(guid) {
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    for (i, c) in [v.x, v.y, v.z].iter().enumerate() {
                        if *c < mn[i] { mn[i] = *c; }
                        if *c > mx[i] { mx[i] = *c; }
                    }
                    found = true;
                }
            } else if let Some((mesh, xf)) = self.scene.gpu_session.brep_pick_meshes.get(guid) {
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
            } else if let Some(pts) = self.scene.gpu_session.nc_pick_pts.get(guid) {
                for p in pts {
                    for i in 0..3 {
                        if p[i] < mn[i] { mn[i] = p[i]; }
                        if p[i] > mx[i] { mx[i] = p[i]; }
                    }
                    found = true;
                }
            }
        }
        if !found { self.scene.camera.reset(); return; }
        let center = [(mn[0]+mx[0])*0.5, (mn[1]+mx[1])*0.5, (mn[2]+mx[2])*0.5];
        let half_diag = (
            (mx[0]-mn[0]).powi(2) +
            (mx[1]-mn[1]).powi(2) +
            (mx[2]-mn[2]).powi(2)
        ).sqrt() * 0.5;
        self.scene.camera.fit_to_box(center, half_diag.max(50.0));
    }
}
