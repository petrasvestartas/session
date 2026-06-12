use crate::State;
use crate::camera::ProjMode;
use crate::gpu_session::InstanceData;
use crate::gumball::{self, Gumball};
use crate::pipelines::CameraUniform;

impl State {
    /// Visible 3D viewport rect (x, y, w, h) in physical pixels — the window
    /// minus egui panels. All screen↔world math must use this, not config dims.
    pub(crate) fn vp_rect(&self) -> (f32, f32, f32, f32) {
        let v = self.scene.viewport_px;
        if v[2] > 1.0 && v[3] > 1.0 {
            (v[0], v[1], v[2], v[3])
        } else {
            (0.0, 0.0, self.gpu.config.width.max(1) as f32, self.gpu.config.height.max(1) as f32)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn select_by_guid(&mut self, guid: &str) {
        let prev: Vec<String> = self.scene.selected_guids.drain().collect();
        for p in &prev {
            self.scene.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
        }
        if self.scene.gpu_session.pick.instance_id(guid).is_some() {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            self.scene.selected_guids.insert(guid.to_string());
            let origin = self.selected_centroid();
            self.gb.gumball = Some(Gumball::new(origin));
        }
    }

    pub(crate) fn set_selection(&mut self, guids: &[&str]) {
        self.gb.gumball_input = None; // a selection change cancels any open gumball popup
        self.gb.gumball_press = None;
        let prev: Vec<String> = self.scene.selected_guids.drain().collect();
        for p in &prev {
            self.scene.gpu_session.set_flag(p, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
        }
        for guid in guids {
            if self.scene.gpu_session.pick.instance_id(guid).is_some() {
                self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
                self.scene.selected_guids.insert(guid.to_string());
            }
        }
        if !self.scene.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            self.gb.gumball = Some(Gumball::new(origin));
        } else {
            self.gb.gumball = None;
        }
    }

    /// Upload the Arctic uniform (projection + AO params, radius scaled to the scene
    /// bbox) and the white ground-plane vertices (mm). Called once per frame while
    /// arctic mode is on.
    fn update_arctic(&mut self) {
        let mut mn = [f32::MAX; 3];
        let mut mx = [f32::MIN; 3];
        let mut found = false;
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
        // cached_boxes is cleared by transform/edit commits and only repopulated on
        // the next pick — reuse the last known bounds in that window so the AO
        // radius and ground plane never pop to the fallback box mid-session.
        if found {
            self.scene.last_arctic_bounds = Some((mn, mx));
        } else if let Some((cmn, cmx)) = self.scene.last_arctic_bounds {
            mn = cmn;
            mx = cmx;
        } else {
            mn = [-5000.0, -5000.0, 0.0];
            mx = [5000.0, 5000.0, 0.0];
        }
        let diag_mm = ((mx[0] - mn[0]).powi(2) + (mx[1] - mn[1]).powi(2) + (mx[2] - mn[2]).powi(2))
            .sqrt()
            .max(100.0);

        // Analytic ground plane in VIEW space: a point near the view center (camera
        // target xy at the lowest geometry z) + the world up axis through the view
        // rotation. No quad vertices — the shader ray-intersects this plane exactly.
        let z_mm = mn[2].min(0.0);
        let tx = self.scene.camera.target[0] * 1000.0;
        let ty = self.scene.camera.target[1] * 1000.0;
        let vm = self.scene.camera.view_matrix();
        let p = [tx, ty, z_mm];
        #[allow(unused_mut)]
        let mut plane_p_vs = [0.0f32; 4];
        for i in 0..3 {
            plane_p_vs[i] = vm[0][i] * p[0] + vm[1][i] * p[1] + vm[2][i] * p[2] + vm[3][i];
        }
        // World +Z through the view rotation (column 2); normalize away the mm scale.
        let nl = (vm[2][0] * vm[2][0] + vm[2][1] * vm[2][1] + vm[2][2] * vm[2][2]).sqrt().max(1e-20);
        let mut plane_n_vs = [vm[2][0] / nl, vm[2][1] / nl, vm[2][2] / nl, 0.0];

        let (proj, inv_proj) = self.scene.camera.proj_and_inverse();
        // Clamp relative to the scene so neither tiny parts (washed-out AO) nor
        // large structures (proportionally vanishing AO) silently degrade.
        let diag_m = diag_mm * 0.001;
        let radius_ws = (self.scene.ssao_radius_pct / 100.0 * diag_m)
            .clamp((diag_m * 0.0005).max(0.001), diag_m * 0.15);
        // Horizon fade range (view-space distance, scene-scaled): the plane starts
        // dissolving a few scene-sizes out and is gone well before the far clip.
        plane_p_vs[3] = diag_m * 3.0;
        plane_n_vs[3] = diag_m * 14.0;
        let arctic = crate::pipelines::ArcticUniform {
            proj,
            inv_proj,
            kernel: crate::pipelines::ArcticUniform::default().kernel,
            radius_ws,
            bias_ws: radius_ws * 0.05, // learnopengl bias/radius ratio (0.025 @ 0.5)
            intensity: self.scene.ssao_intensity,
            flags: self.scene.arctic_gradient as u32
                | ((self.scene.outline as u32) << 1)
                | ((self.scene.fxaa as u32) << 2)
                | (((self.scene.outline && !self.scene.selected_guids.is_empty()) as u32) << 3),
            ao_mode: self.scene.ao_mode,
            outline_px: self.scene.outline_px,
            screen_w: self.gpu.config.width,
            screen_h: self.gpu.config.height,
            plane_p_vs,
            plane_n_vs,
            vp_rect: {
                let (vx, vy, vw, vh) = self.vp_rect();
                [vx, vy, vw, vh]
            },
        };
        self.gpu.queue.write_buffer(&self.gpu.arctic_buf, 0, bytemuck::bytes_of(&arctic));
    }

    pub fn update(&mut self) {
        // The 3D scene renders ONLY into the visible viewport (window minus egui
        // panels) via set_viewport/scissor — match the camera aspect to it BEFORE
        // the controller runs so pan/zoom use this frame's viewport scale. No NDC
        // offset needed: the viewport transform does the placement.
        let vp = self.scene.viewport_px;
        if vp[2] > 1.0 && vp[3] > 1.0 {
            self.scene.camera.aspect = vp[2] / vp[3];
            self.scene.camera.vp_h = vp[3];
        }

        self.scene.controller.update_camera(&mut self.scene.camera);

        let v = self.scene.camera.view_matrix();
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
            view_proj:    self.scene.camera.view_proj(),
            key_light_ws: { let kl = cam_to_ws(-0.3, 0.8, 0.6); let oh = if self.scene.camera.proj_mode == ProjMode::Ortho { self.scene.camera.ortho_half_h() * 1000.0 } else { 0.0_f32 }; [kl[0], kl[1], kl[2], oh] },
            fill_light_ws:{ let fl = cam_to_ws( 0.8,-0.2, 0.5); [fl[0], fl[1], fl[2], self.scene.camera.tan_half_fov_y() * 1000.0] },
            screen_size:  [self.vp_rect().2, self.vp_rect().3],
            point_size:   self.scene.line_thickness / 3.0,
            flags:        (!self.scene.shading_enabled as u32)
                | (if self.scene.backface_highlight { 2 } else { 0 })
                | (if self.scene.arctic { 4 } else { 0 }),
        };
        self.gpu.queue.write_buffer(&self.gpu.camera_buf, 0, bytemuck::bytes_of(&cam));

        if self.scene.arctic || self.scene.outline {
            self.update_arctic();
        }

        if let Some((cx, cy)) = self.scene.pending_pick.take() {
            if self.tool.is_active() {
                self.tool_on_click(cx as f32, cy as f32);
            } else {
                self.process_pick(cx as f32, cy as f32);
            }
        }

        // Rebuild the draw-tool rubber-band/preview once per frame when flagged.
        if self.tool.preview_dirty {
            self.rebuild_draw_preview();
            self.tool.preview_dirty = false;
        }

        // Keep control-point handles a constant screen size while edit mode is on.
        self.update_edit_scale();

        // Compute gumball scale after pick so a newly created gumball gets the
        // correct size on its first frame.
        if let Some(gb) = &self.gb.gumball {
            const VIEWER_TO_MM: f32 = 1000.0;
            let vp_h = self.vp_rect().3;
            self.gb.gumball_scale = match self.scene.camera.proj_mode {
                ProjMode::Perspective => {
                    // Use view-space Z depth of the gumball origin for accuracy
                    // when the orbit target and gumball are not at the same position.
                    let vm = self.scene.camera.view_matrix();
                    let [ox, oy, oz] = gb.origin; // mm; view_matrix includes MM_TO_UNIT
                    let vz = vm[0][2]*ox + vm[1][2]*oy + vm[2][2]*oz + vm[3][2];
                    // vz is in viewer units, negative for objects in front of camera
                    let depth_mm = (-vz).max(0.001) * VIEWER_TO_MM;
                    let mm_per_px = 2.0 * depth_mm * self.scene.camera.tan_half_fov_y() / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
                ProjMode::Ortho => {
                    let ortho_h_mm = self.scene.camera.ortho_half_h() * 2.0 * VIEWER_TO_MM;
                    let mm_per_px = ortho_h_mm / vp_h;
                    gumball::SCREEN_PX * mm_per_px / gumball::ARC_RADIUS
                }
            };
        }

    }
}
