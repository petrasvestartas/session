impl State {
    #[allow(dead_code)]
    fn select_by_guid(&mut self, guid: &str) {
        let prev: Vec<String> = self.selected_guids.drain().collect();
        for p in &prev {
            self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
        }
        if self.gpu_session.pick.instance_id(guid).is_some() {
            self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
            self.selected_guids.insert(guid.to_string());
            let origin = self.selected_centroid();
            self.gumball = Some(Gumball::new(origin));
        }
    }

    fn set_selection(&mut self, guids: &[&str]) {
        let prev: Vec<String> = self.selected_guids.drain().collect();
        for p in &prev {
            self.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.queue);
        }
        for guid in guids {
            if self.gpu_session.pick.instance_id(guid).is_some() {
                self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                self.selected_guids.insert(guid.to_string());
            }
        }
        if !self.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            self.gumball = Some(Gumball::new(origin));
        } else {
            self.gumball = None;
        }
    }
    pub fn update(&mut self) {
        self.controller.update_camera(&mut self.camera);
        let v = self.camera.view_matrix();
        let norm3 = |x: f32, y: f32, z: f32| -> [f32; 3] {
            let l = (x*x + y*y + z*z).sqrt().max(1e-30);
            [x/l, y/l, z/l]
        };
        let right   = norm3(v[0][0], v[1][0], v[2][0]);
        let up      = norm3(v[0][1], v[1][1], v[2][1]);
        let forward = norm3(v[0][2], v[1][2], v[2][2]);
        let cam_to_ws = |r: f32, u: f32, f: f32| -> [f32; 4] {
            let x = r*right[0] + u*up[0] + f*forward[0];
            let y = r*right[1] + u*up[1] + f*forward[1];
            let z = r*right[2] + u*up[2] + f*forward[2];
            let l = (x*x + y*y + z*z).sqrt().max(1e-30);
            [x/l, y/l, z/l, 0.0]
        };
        let cam = CameraUniform {
            view_proj:    self.camera.view_proj(),
            key_light_ws: { let kl = cam_to_ws(-0.3, 0.8, 0.6); let oh = if self.camera.proj_mode == ProjMode::Ortho { self.camera.ortho_scale * 1000.0 } else { 0.0_f32 }; [kl[0], kl[1], kl[2], oh] },
            fill_light_ws:cam_to_ws( 0.8,-0.2, 0.5),
            screen_size:  [self.config.width as f32, self.config.height as f32],
            point_size:   self.line_thickness / 3.0,
            flags:        (!self.shading_enabled as u32) | (if self.backface_highlight { 2 } else { 0 }),
        };
        self.queue.write_buffer(&self.camera_buf, 0, bytemuck::bytes_of(&cam));

        if let Some((cx, cy)) = self.pending_pick.take() {
            self.process_pick(cx as f32, cy as f32);
        }

        // Compute gumball scale after pick so a newly created gumball gets the
        // correct size on its first frame.
        if let Some(gb) = &self.gumball {
            const VIEWER_TO_MM: f32 = 1000.0;
            let vp_h = self.config.height as f32;
            self.gumball_scale = match self.camera.proj_mode {
                ProjMode::Perspective => {
                    // Use view-space Z depth of the gumball origin for accuracy
                    // when the orbit target and gumball are not at the same position.
                    let vm = self.camera.view_matrix();
                    let [ox, oy, oz] = gb.origin; // mm; view_matrix includes MM_TO_UNIT
                    let vz = vm[0][2]*ox + vm[1][2]*oy + vm[2][2]*oz + vm[3][2];
                    // vz is in viewer units, negative for objects in front of camera
                    let depth_mm = (-vz).max(0.001) * VIEWER_TO_MM;
                    use session_rust::tolerance::Tolerance;
                    let mm_per_px = 2.0 * depth_mm * (Tolerance::PI / 6.0).tan() / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
                ProjMode::Ortho => {
                    // ortho_scale is the half-height of the frustum in viewer units
                    let ortho_h_mm = self.camera.ortho_scale * 2.0 * VIEWER_TO_MM;
                    let mm_per_px = ortho_h_mm / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
            };
        }

    }
}
