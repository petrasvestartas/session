use crate::{labels_from_session, mat4_mul_cm, State};
use crate::camera::ProjMode;
use crate::gpu_session::InstanceData;
use crate::gumball::{self, HandleKind};
use crate::gumball_state::GumballInput;
use crate::pick::screen_to_world_ray;
use crate::undo_state::UndoAction;
use session_rust::session::Geometry;
use session_rust::Xform;
use winit::event::{MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

/// Squared cursor-travel (physical px) past which a gumball-handle press becomes
/// a drag. Below it, a press+release is treated as a click (opens the popup).
const GUMBALL_DRAG_THRESHOLD_SQ: f64 = 16.0; // 4 px

impl State {

    pub(crate) fn reapply_visibility_flags(&mut self, guid: &str) {
        if self.scene.hidden_guids.contains(guid) {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_HIDDEN, true, &self.gpu.queue);
        }

        if self.scene.glyphs_hidden_guids.contains(guid) {
            self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_GLYPHS_HIDDEN, true, &self.gpu.queue);
        }
    }

    pub(crate) fn reapply_color_overrides(&mut self, guid: &str) {
        if let Some(&color) = self.scene.face_color_overrides.get(guid) {
            self.scene.gpu_session.set_face_color(guid, color, &self.gpu.queue);
        }
        if let Some(&color) = self.scene.point_color_overrides.get(guid) {
            self.scene.gpu_session.set_color(guid, color, &self.gpu.queue);
        }
    }

    pub(crate) fn commit_object_transform(&mut self, guid: &str, model: [[f32; 4]; 4]) {
        let flat = [
            model[0][0] as f64, model[0][1] as f64, model[0][2] as f64, model[0][3] as f64,
            model[1][0] as f64, model[1][1] as f64, model[1][2] as f64, model[1][3] as f64,
            model[2][0] as f64, model[2][1] as f64, model[2][2] as f64, model[2][3] as f64,
            model[3][0] as f64, model[3][1] as f64, model[3][2] as f64, model[3][3] as f64,
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
        // NurbsSurface: matrix-only move like BRep — update the GPU model matrix and the
        // stored pick xform; NO control-point bake and NO re-tessellation. The pose lives
        // in the instance model (baked into CVs lazily, e.g. on delete — see state_cmd `del`).
        if self.scene.gpu_session.nurbs_pick_meshes.contains_key(guid) {
            let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
            self.scene.gpu_session.update_transform(guid, model, &self.gpu.queue);
            if let Some((_, xf)) = self.scene.gpu_session.nurbs_pick_meshes.get_mut(guid) {
                *xf = model;
            }
            if was_selected {
                self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            }
            self.reapply_color_overrides(guid);
            self.apply_thickness();
            return;
        }
        // NurbsSurfaceTrimmed: matrix-only move like NurbsSurface/BRep — update the GPU model
        // matrix + stored pick xform, no re-tessellation. Pose lives in the instance model.
        if self.scene.gpu_session.nurbs_trimmed_pick_meshes.contains_key(guid) {
            let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
            self.scene.gpu_session.update_transform(guid, model, &self.gpu.queue);
            if let Some((_, xf2)) = self.scene.gpu_session.nurbs_trimmed_pick_meshes.get_mut(guid) {
                *xf2 = model;
            }
            if was_selected {
                self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
            }
            self.reapply_color_overrides(guid);
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
            self.reapply_color_overrides(guid);
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
        self.reapply_color_overrides(guid);
        self.apply_thickness();
        self.scene.text_labels = labels_from_session(&self.scene.session);
    }

    /// Bake the current gumball transform (live in the GPU instance models) into
    /// CPU geometry and push an undo entry. Used by both drag-release and the
    /// numeric-entry popup. Reads `drag_origins` (set up at press) for the
    /// before-state and the instance models for the after-state, then clears them.
    pub(crate) fn commit_gumball_transform(&mut self) {
        let to_commit: Vec<(String, [[f32; 4]; 4])> = self.gb.drag_origins.keys()
            .filter_map(|guid| {
                self.scene.gpu_session.pick.instance_id(guid).map(|iid| {
                    (guid.clone(), self.scene.gpu_session.instances_cpu[iid as usize].model)
                })
            })
            .collect();
        // Collect before/after for undo (before committing, while CPU geom is pre-drag)
        let undo_objects: Vec<(String, [[f32; 4]; 4], [[f32; 4]; 4])> = to_commit.iter()
            .filter_map(|(guid, after)| {
                self.gb.drag_origins.get(guid).map(|before| (guid.clone(), *before, *after))
            })
            .collect();
        let mut snapshots = std::collections::HashMap::new();
        for guid in undo_objects.iter().map(|(g, ..)| g) {
            if let Some(geom) = self.gb.drag_geom_snapshots.get(guid) {
                snapshots.insert(guid.clone(), UndoAction::snap_geom(geom.clone()));
            } else if let Some(ns) = self.gb.drag_nurbs_snapshots.get(guid) {
                snapshots.insert(guid.clone(), UndoAction::snap_nurbs(ns.clone()));
            }
        }
        for (guid, model) in to_commit {
            self.commit_object_transform(&guid, model);
        }
        // After commit the CPU geometry holds the post-transform state; snapshot it
        // so redo is an absolute restore symmetric with undo (not a delta re-bake).
        let mut snapshots_after = std::collections::HashMap::new();
        for guid in undo_objects.iter().map(|(g, ..)| g) {
            if self.gb.drag_geom_snapshots.contains_key(guid) {
                if let Some(geom) = self.scene.session.lookup.get(guid) {
                    snapshots_after.insert(guid.clone(), UndoAction::snap_geom(geom.clone()));
                }
            } else if self.gb.drag_nurbs_snapshots.contains_key(guid) {
                if let Some(ns) = self.scene.session.objects.nurbssurfaces.iter().find(|n| n.guid() == *guid) {
                    snapshots_after.insert(guid.clone(), UndoAction::snap_nurbs(ns.clone()));
                }
            }
        }
        if !undo_objects.is_empty() {
            self.hist.push(UndoAction::Transform { objects: undo_objects, snapshots, snapshots_after });
        }
        self.gb.drag_origins.clear();
        self.gb.drag_geom_snapshots.clear();
        self.gb.drag_nurbs_snapshots.clear();
    }

    /// Open the numeric-entry popup for a clicked handle, anchored at the cursor.
    pub(crate) fn open_gumball_input(&mut self, handle: HandleKind, screen_x: f64, screen_y: f64) {
        self.gb.gumball_input = Some(GumballInput {
            handle,
            buffer: String::new(),
            screen_pos: (screen_x as f32, screen_y as f32),
            focus: true,
        });
    }

    /// Apply a typed numeric value to the objects captured at press time, commit
    /// it (with undo), and close the popup. Mirrors the drag path: build the delta,
    /// push it into the GPU instance models, then `commit_gumball_transform`.
    pub(crate) fn apply_gumball_input(&mut self, value: f32) {
        // In edit mode the popup drives a sub-object move instead of an object transform.
        if self.edit.active && !self.edit.drag_start.is_empty() {
            self.apply_edit_input(value);
            return;
        }
        let handle = match &self.gb.gumball_input {
            Some(input) => input.handle,
            None => return,
        };
        let origin = match &self.gb.gumball {
            Some(gb) => gb.origin,
            None => { self.gb.gumball_input = None; return; }
        };
        let delta = gumball::manual_delta(handle, value, origin);
        let updates: Vec<(String, [[f32; 4]; 4])> = self.gb.drag_origins.iter()
            .map(|(guid, orig)| (guid.clone(), mat4_mul_cm(&delta, orig)))
            .collect();
        for (guid, model) in &updates {
            self.scene.gpu_session.update_transform(guid, *model, &self.gpu.queue);
        }
        self.commit_gumball_transform();
        // Recompute the gumball origin from the post-transform geometry so it
        // follows a translation (centroid moves) and stays put for rotate/scale.
        if !self.scene.selected_guids.is_empty() {
            let origin = self.selected_centroid();
            if let Some(gb) = &mut self.gb.gumball { gb.set_origin(origin); }
        }
        self.gb.gumball_input = None;
    }

    /// Close the numeric-entry popup without applying and drop the pending drag state.
    pub(crate) fn cancel_gumball_input(&mut self) {
        self.gb.gumball_input = None;
        self.gb.gumball_press = None;
        self.gb.drag_origins.clear();
        self.gb.drag_geom_snapshots.clear();
        self.gb.drag_nurbs_snapshots.clear();
        self.edit.drag_start.clear();
        self.edit.before_snapshot = None;
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            if !pressed {
                let was_dragging = self.gb.gumball.as_ref().map_or(false, |gb| gb.drag.is_some());
                if was_dragging {
                    if let Some(gb) = &mut self.gb.gumball {
                        gb.drag = None;
                    }
                    if self.edit.active && !self.edit.drag_start.is_empty() {
                        self.commit_edit_transform();
                    } else {
                        self.commit_gumball_transform();
                    }
                    self.gb.gumball_press = None;
                    self.scene.box_select_start = None;
                    self.scene.box_select = None;
                    return;
                }

                // Handle pressed but never dragged past the threshold → it was a
                // click: open the numeric-entry popup for that handle instead.
                if let Some((handle, sx, sy)) = self.gb.gumball_press.take() {
                    self.open_gumball_input(handle, sx, sy);
                    self.scene.box_select_start = None;
                    self.scene.box_select = None;
                    return;
                }

                // Box select: if drag exceeded threshold, do box selection
                if let Some(((sx, sy), (ex, ey))) = self.scene.box_select.take() {
                    self.scene.box_select_start = None;
                    self.scene.pending_pick = None;
                    self.process_box_select(sx as f32, sy as f32, ex as f32, ey as f32);
                    return;
                }

                self.scene.box_select_start = None;
            }
            if pressed {
                self.scene.box_select_start = Some(self.scene.mouse_position);
                self.scene.box_select = None;
                self.scene.pending_pick = Some(self.scene.mouse_position);
            }
        }
        self.scene.controller.process_mouse_button(button, pressed);
    }

    pub fn handle_mouse_moved(&mut self, x: f64, y: f64) {
        let (px, py) = self.scene.mouse_position;
        self.scene.mouse_position = (x, y);

        // An active draw tool owns hover: refresh the snap/preview and allow RMB orbit,
        // but skip box-select and gumball hit-testing entirely.
        if self.tool.is_active() {
            self.tool_on_move(x as f32, y as f32);
            self.scene.controller.process_mouse_move((x - px) as f32, (y - py) as f32);
            return;
        }

        // Update box select while LMB is held and no gumball handle is engaged
        // (either actively dragging, or pressed and awaiting the click/drag decision).
        // The lmb_down gate also prevents a sticky no-button box-select if a release
        // was consumed by egui (e.g. over a panel) and never cleared box_select_start.
        if let Some((sx, sy)) = self.scene.box_select_start {
            let gumball_engaged = self.gb.gumball.as_ref().map_or(false, |gb| gb.drag.is_some())
                || self.gb.gumball_press.is_some();
            // Edit mode owns LMB-drag for the gumball only; suppress object box-select.
            if self.scene.lmb_down && !gumball_engaged && !self.edit.active {
                let dx = x - sx;
                let dy = y - sy;
                if dx*dx + dy*dy > 25.0 {
                    self.scene.box_select = Some(((sx, sy), (x, y)));
                    self.scene.pending_pick = None;
                } else if self.scene.box_select.is_some() {
                    self.scene.box_select = Some(((sx, sy), (x, y)));
                }
                if self.scene.box_select.is_some() {
                    self.scene.controller.process_mouse_move(0.0, 0.0);
                    let drag_info = self.gb.gumball.as_ref().and_then(|gb| {
                        gb.drag.as_ref().map(|ds| (ds.clone(), gb.origin))
                    });
                    if drag_info.is_none() { return; }
                }
            }
        }

        // A handle is pressed but the drag hasn't started yet: begin it once the
        // cursor moves past the threshold. If it never does, the release is a click.
        // Requires the LMB to still be held so a leaked press can't start a
        // no-button "sticky" drag on hover.
        if self.scene.lmb_down && self.gb.gumball.as_ref().map_or(false, |gb| gb.drag.is_none()) {
            if let Some((handle, px0, py0)) = self.gb.gumball_press {
                let dx = x - px0;
                let dy = y - py0;
                if dx*dx + dy*dy >= GUMBALL_DRAG_THRESHOLD_SQ {
                    let view = self.scene.camera.view_matrix();
                    let proj = self.scene.camera.proj_matrix();
                    let vp = self.vp_rect();
                    let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
                    let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
                    let origin = self.gb.gumball.as_ref().unwrap().origin;
                    let ds = gumball::begin_drag(handle, ray, origin, self.gb.gumball_scale);
                    self.gb.gumball.as_mut().unwrap().drag = Some(ds);
                }
            }
        }

        let drag_info = self.gb.gumball.as_ref().and_then(|gb| {
            gb.drag.as_ref().map(|ds| (ds.clone(), gb.origin))
        });
        if let Some((ds, origin)) = drag_info {
            let view = self.scene.camera.view_matrix();
            let proj = self.scene.camera.proj_matrix();
            let vp = self.vp_rect();
            let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gb.gumball_scale;
            if let Some(delta) = gumball::update_drag(&ds, ray, origin, scale) {
                if self.edit.active && !self.edit.drag_start.is_empty() {
                    // Sub-object drag: move the picked control points (gumball origin
                    // is updated to the moving centroid inside apply_edit_delta).
                    self.apply_edit_delta(&delta);
                } else {
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
            }
            return;
        }

        if self.gb.gumball.is_some() {
            let view = self.scene.camera.view_matrix();
            let proj = self.scene.camera.proj_matrix();
            let vp = self.vp_rect();
            let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
            let ray = screen_to_world_ray(&view, &proj, vp, (x as f32, y as f32), is_ortho);
            let scale = self.gb.gumball_scale;
            let show_gumball = self.gb.show_gumball;
            if let Some(gb) = &mut self.gb.gumball {
                gb.hovered = if show_gumball { gb.hit_test(ray, scale) } else { None };
            }
        }

        self.scene.controller.process_mouse_move((x - px) as f32, (y - py) as f32);
    }

    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scene.controller.process_scroll(delta);
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        // Track Ctrl/Shift directly from key events — ModifiersChanged is unreliable
        // on the web backend. Read at pick time for Ctrl+Shift sub-object selection.
        match code {
            KeyCode::ControlLeft | KeyCode::ControlRight => self.scene.ctrl_down = is_pressed,
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.scene.shift_down = is_pressed,
            _ => {}
        }
        if code == KeyCode::F10 && is_pressed {
            // Toggle control-point / sub-object edit mode for the selected object.
            self.toggle_edit_mode();
        } else if code == KeyCode::Escape && is_pressed {
            // With the popup open, Escape cancels it instead of quitting the app.
            // (egui only consumes Escape while the field has focus; if focus was lost
            // the key falls through to here, where it would otherwise exit.)
            if self.tool.is_active() {
                self.tool_cancel();
            } else if self.gb.gumball_input.is_some() {
                self.cancel_gumball_input();
            } else if self.edit.active {
                self.exit_edit_mode();
            } else {
                event_loop.exit();
            }
        } else if code == KeyCode::KeyF && is_pressed {
            self.fit_view();
        } else if code == KeyCode::KeyQ && is_pressed {
            self.scene.shading_enabled = !self.scene.shading_enabled;
        } else if code == KeyCode::KeyE && is_pressed {
            self.scene.backface_highlight = !self.scene.backface_highlight;
        } else if code == KeyCode::KeyM && is_pressed {
            self.scene.show_tess = !self.scene.show_tess;
            self.rebuild_tess_wireframe();
        } else if code == KeyCode::KeyG && is_pressed {
            self.scene.show_grid = !self.scene.show_grid;
        } else if code == KeyCode::KeyB && is_pressed && !self.scene.shift_down {
            // B toggles arctic mode (white + SSAO); Shift+B keeps the bottom view.
            self.scene.arctic = !self.scene.arctic;
        } else {
            self.scene.controller.process_key(code, is_pressed);
        }
    }

    /// Build/clear the NURBS + trimmed tessellation wireframe (triangle edges as LineList
    /// lines in the `line` arena, drawn by the existing `draw_lines` pass — no render-pass
    /// change). Toggled by `m`. Rebuilt as a static snapshot; uses each object's instance so
    /// it sits with the object at its committed position.
    pub(crate) fn rebuild_tess_wireframe(&mut self) {
        // Always clear the previous snapshot first.
        let prev: Vec<String> = self.scene.gpu_session.line.iter_slots()
            .map(|(g, _)| g.clone())
            .filter(|g| g.starts_with("__tess__"))
            .collect();
        for g in prev { self.scene.gpu_session.line.free(&g); }
        if !self.scene.show_tess { return; }

        let edge_color: [u8; 4] = [0, 0, 0, 255]; // black, like default object edges
        // Dedicated instance: identity model, white tint, no FLAG_SELECTED — so the
        // wireframe renders pure black regardless of the host object's color/selection
        // (the line shader does `vertex.color * inst.tint` and turns yellow on FLAG_SELECTED).
        let wire_iid = self.scene.gpu_session.pick.allocate("__tess_wire_inst__");
        self.scene.gpu_session.write_instance(wire_iid, [1.0, 1.0, 1.0, 1.0], &self.gpu.device, &self.gpu.queue);
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // Wireframe of one mesh as world-space LineList pairs (the source meshes are local,
        // so bake the host instance model in since we draw under the identity wire instance).
        let wire = |mesh: &session_rust::Mesh, m: &[[f32; 4]; 4]| -> Vec<crate::gpu_session::LineVertex> {
            let xp = |p: session_rust::Point| -> [f32; 3] {
                [
                    m[0][0]*p[0] as f32 + m[1][0]*p[1] as f32 + m[2][0]*p[2] as f32 + m[3][0],
                    m[0][1]*p[0] as f32 + m[1][1]*p[1] as f32 + m[2][1]*p[2] as f32 + m[3][1],
                    m[0][2]*p[0] as f32 + m[1][2]*p[1] as f32 + m[2][2]*p[2] as f32 + m[3][2],
                ]
            };
            let mut vs = Vec::new();
            for verts in mesh.face.values() {
                let n = verts.len();
                if n < 2 { continue; }
                for i in 0..n {
                    let a = mesh.vertex.get(&verts[i]).map(|v| v.position());
                    let b = mesh.vertex.get(&verts[(i + 1) % n]).map(|v| v.position());
                    if let (Some(a), Some(b)) = (a, b) {
                        vs.push(crate::gpu_session::LineVertex { position: xp(a), color: edge_color });
                        vs.push(crate::gpu_session::LineVertex { position: xp(b), color: edge_color });
                    }
                }
            }
            vs
        };

        // Transform a kernel (f64) point by an f32 model into world f32.
        let xp = |p: &session_rust::Point, m: &[[f32; 4]; 4]| -> [f32; 3] {
            [
                m[0][0]*p[0] as f32 + m[1][0]*p[1] as f32 + m[2][0]*p[2] as f32 + m[3][0],
                m[0][1]*p[0] as f32 + m[1][1]*p[1] as f32 + m[2][1]*p[2] as f32 + m[3][1],
                m[0][2]*p[0] as f32 + m[1][2]*p[1] as f32 + m[2][2]*p[2] as f32 + m[3][2],
            ]
        };
        // Collect (guid, verts), then allocate (can't borrow gpu_session twice at once).
        let mut jobs: Vec<(String, Vec<crate::gpu_session::LineVertex>)> = Vec::new();
        // NURBS surfaces: K iso-curves per direction (a clean quad-grid of "simple lines"),
        // NOT every dense triangle edge — ~25× fewer line vertices.
        const ISO_K: usize = 8;
        for (guid, surface) in &self.scene.gpu_session.nurbs_surfaces {
            if self.scene.hidden_guids.contains(guid) { continue; }
            let m = self.scene.gpu_session.pick.instance_id(guid)
                .map(|iid| self.scene.gpu_session.instances_cpu[iid as usize].model)
                .unwrap_or(identity);
            let mut vs = Vec::new();
            // iso_curve(iso_dir, t): t runs over the OTHER direction's domain.
            for iso_dir in 0..2 {
                let param_dir = 1 - iso_dir;
                if let Some((t0, t1)) = surface.domain(param_dir) {
                    for s in 0..=ISO_K {
                        let t = t0 + (t1 - t0) * (s as f64 / ISO_K as f64);
                        if let Some(crv) = surface.iso_curve(iso_dir, t) {
                            let (pts, _) = crv.to_polyline_adaptive(
                                session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
                            for w in pts.windows(2) {
                                vs.push(crate::gpu_session::LineVertex { position: xp(&w[0], &m), color: edge_color });
                                vs.push(crate::gpu_session::LineVertex { position: xp(&w[1], &m), color: edge_color });
                            }
                        }
                    }
                }
            }
            jobs.push((format!("__tess__{guid}"), vs));
        }
        for (guid, geom) in &self.scene.session.lookup {
            if self.scene.hidden_guids.contains(guid) { continue; }
            if let session_rust::session::Geometry::Mesh(mm) = geom {
                let m = self.scene.gpu_session.pick.instance_id(guid)
                    .map(|iid| self.scene.gpu_session.instances_cpu[iid as usize].model)
                    .unwrap_or(identity);
                jobs.push((format!("__tess__{guid}"), wire(mm, &m)));
            }
        }
        for (key, vs) in jobs {
            if vs.is_empty() { continue; }
            self.scene.gpu_session.line.allocate(&key, &vs, None, wire_iid, &self.gpu.device, &self.gpu.queue);
        }
    }

    pub(crate) fn fit_view(&mut self) {
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
        // The bbox cache is cleared by every transform/edit commit and only
        // rebuilt by a viewport pick — without this, F after a drag (or after a
        // tree selection) finds nothing and RESETS the camera instead of
        // zooming to the selection. A throwaway ray cast forces the rebuild.
        if self.scene.session.cached_boxes.len() != self.scene.session.lookup.len() {
            let ray = crate::pick::Ray { origin: [0.0, 0.0, 1.0e9], direction: [0.0, 0.0, -1.0] };
            let _ = crate::pick::pick_by_ray(&mut self.scene.session, ray, 0.0);
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
            } else if let Some((mesh, xf)) = self.scene.gpu_session.nurbs_pick_meshes.get(guid) {
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    let wx = xf[0][0]*v.x as f32 + xf[1][0]*v.y as f32 + xf[2][0]*v.z as f32 + xf[3][0];
                    let wy = xf[0][1]*v.x as f32 + xf[1][1]*v.y as f32 + xf[2][1]*v.z as f32 + xf[3][1];
                    let wz = xf[0][2]*v.x as f32 + xf[1][2]*v.y as f32 + xf[2][2]*v.z as f32 + xf[3][2];
                    for (i, c) in [wx, wy, wz].iter().enumerate() {
                        if *c < mn[i] { mn[i] = *c; }
                        if *c > mx[i] { mx[i] = *c; }
                    }
                    found = true;
                }
            } else if let Some((mesh, xf)) = self.scene.gpu_session.brep_pick_meshes.get(guid) {
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    let wx = xf[0][0]*v.x as f32 + xf[1][0]*v.y as f32 + xf[2][0]*v.z as f32 + xf[3][0];
                    let wy = xf[0][1]*v.x as f32 + xf[1][1]*v.y as f32 + xf[2][1]*v.z as f32 + xf[3][1];
                    let wz = xf[0][2]*v.x as f32 + xf[1][2]*v.y as f32 + xf[2][2]*v.z as f32 + xf[3][2];
                    for (i, c) in [wx, wy, wz].iter().enumerate() {
                        if *c < mn[i] { mn[i] = *c; }
                        if *c > mx[i] { mx[i] = *c; }
                    }
                    found = true;
                }
            } else if let Some((mesh, xf)) = self.scene.gpu_session.nurbs_trimmed_pick_meshes.get(guid) {
                for key in mesh.vertex.keys() {
                    let v = &mesh.vertex[key];
                    let wx = xf[0][0]*v.x as f32 + xf[1][0]*v.y as f32 + xf[2][0]*v.z as f32 + xf[3][0];
                    let wy = xf[0][1]*v.x as f32 + xf[1][1]*v.y as f32 + xf[2][1]*v.z as f32 + xf[3][1];
                    let wz = xf[0][2]*v.x as f32 + xf[1][2]*v.y as f32 + xf[2][2]*v.z as f32 + xf[3][2];
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
