use std::collections::HashMap;
use std::sync::Arc;
use crate::State;
use crate::gpu_session::InstanceData;
use crate::gumball::Gumball;
use crate::tree_ui::{self, populate_leaf_cache, render_tree_node};

/// guid -> label map of every selectable/hideable geometry leaf. The tree UI uses membership
/// here to decide if a node is a clickable leaf (vs a group). MUST include every object kind that
/// is a first-class scene object: lookup geometry, NurbsSurfaces, NurbsCurves, AND
/// NurbsSurfaceTrimmeds — omitting one makes that kind unselectable/unhideable in the tree.
pub(crate) fn build_vmap(session: &session_rust::session::Session) -> HashMap<String, String> {
    use session_rust::session::Geometry;
    fn geom_name(g: &Geometry) -> &str {
        match g {
            Geometry::Point(x)      => &x.name,
            Geometry::Line(x)       => &x.name,
            Geometry::Polyline(x)   => &x.name,
            Geometry::PointCloud(x) => &x.name,
            Geometry::Mesh(x)       => &x.name,
            Geometry::Plane(x)      => &x.name,
            Geometry::OBB(x)        => &x.name,
            Geometry::BRep(x)       => &x.name,
            Geometry::Element(x)    => &x.name,
        }
    }
    let mut vmap: HashMap<String, String> = session.lookup
        .iter()
        .map(|(guid, geom)| {
            let name = geom_name(geom);
            let label = if name.is_empty() { guid.clone() } else { name.to_string() };
            (guid.clone(), label)
        })
        .collect();
    for ns in &session.objects.nurbssurfaces {
        let g = ns.guid().to_string();
        let label = if ns.name.is_empty() { g.clone() } else { ns.name.clone() };
        vmap.entry(g).or_insert(label);
    }
    for ts in &session.objects.nurbssurfacetrimmeds {
        let g = ts.guid().to_string();
        let label = if ts.name.is_empty() { g.clone() } else { ts.name.clone() };
        vmap.entry(g).or_insert(label);
    }
    for nc in &session.objects.nurbscurves {
        let g = nc.guid().to_string();
        let label = if nc.name.is_empty() { g.clone() } else { nc.name.clone() };
        vmap.entry(g).or_insert(label);
    }
    vmap
}

/// Undo/redo toolbar button rendering an SVG icon (assets/undo.svg, redo.svg).
/// The font lacks ↶/↷ (they render as empty boxes), so we use real SVG curved
/// arrows like the rest of the toolbar. Greys out when `enabled` is false.
fn arrow_button(ui: &mut egui::Ui, src: egui::ImageSource<'static>, enabled: bool) -> egui::Response {
    // SVG is white-filled; tint it dark when actionable, light grey when disabled.
    let tint = if enabled { egui::Color32::from_gray(50) } else { egui::Color32::from_gray(190) };
    let img = egui::Image::new(src).fit_to_exact_size(egui::vec2(18.0, 18.0)).tint(tint);
    ui.add_enabled(enabled, egui::Button::image(img))
}

/// Draw a constant-pixel osnap marker glyph at a screen position (CAD convention:
/// square=end/vertex, triangle=mid, X=intersection, circle=center, diamond=knot,
/// plus=near).
fn draw_snap_marker(painter: &egui::Painter, pos: egui::Pos2, kind: crate::snap::SnapKind) {
    use crate::snap::SnapKind;
    let col = egui::Color32::from_rgb(255, 215, 0);
    let s = egui::Stroke::new(1.5, col);
    let r = 6.0_f32;
    match kind {
        SnapKind::Endpoint | SnapKind::Vertex => {
            let rect = egui::Rect::from_center_size(pos, egui::vec2(r * 2.0, r * 2.0));
            painter.rect_stroke(rect, 0.0, s, egui::StrokeKind::Outside);
        }
        SnapKind::Midpoint => {
            let p0 = egui::pos2(pos.x, pos.y - r);
            let p1 = egui::pos2(pos.x - r, pos.y + r);
            let p2 = egui::pos2(pos.x + r, pos.y + r);
            painter.line_segment([p0, p1], s);
            painter.line_segment([p1, p2], s);
            painter.line_segment([p2, p0], s);
        }
        SnapKind::Intersection => {
            painter.line_segment([egui::pos2(pos.x - r, pos.y - r), egui::pos2(pos.x + r, pos.y + r)], s);
            painter.line_segment([egui::pos2(pos.x - r, pos.y + r), egui::pos2(pos.x + r, pos.y - r)], s);
        }
        SnapKind::Center => {
            painter.circle_stroke(pos, r, s);
        }
        SnapKind::Knot => {
            let t = egui::pos2(pos.x, pos.y - r);
            let rt = egui::pos2(pos.x + r, pos.y);
            let b = egui::pos2(pos.x, pos.y + r);
            let lt = egui::pos2(pos.x - r, pos.y);
            painter.line_segment([t, rt], s);
            painter.line_segment([rt, b], s);
            painter.line_segment([b, lt], s);
            painter.line_segment([lt, t], s);
        }
        SnapKind::Near => {
            painter.line_segment([egui::pos2(pos.x - r, pos.y), egui::pos2(pos.x + r, pos.y)], s);
            painter.line_segment([egui::pos2(pos.x, pos.y - r), egui::pos2(pos.x, pos.y + r)], s);
        }
        _ => {}
    }
}

impl State {
    pub(crate) fn build_ui(&mut self) -> egui::FullOutput {
        let egui_ctx = self.shell.egui_ctx.clone();
        let window = Arc::clone(&self.window);
        let raw_input = self.shell.egui_state.take_egui_input(&window);

        let tree_root = self.scene.session.tree.root();
        // Single source of truth (includes NurbsSurfaceTrimmed) — see build_vmap.
        let vmap: HashMap<String, String> = build_vmap(&self.scene.session);
        if self.scene.leaf_cache_dirty {
            self.scene.leaf_guid_cache.clear();
            self.scene.leaf_cache_dirty = false;
        }
        if let Some(root) = &tree_root {
            populate_leaf_cache(root, &vmap, &mut self.scene.leaf_guid_cache);
        }
        let leaf_cache = self.scene.leaf_guid_cache.clone();

        let edges = self.scene.session.graph.get_edges();
        let selected = self.scene.selected_guids.clone();
        let hidden = self.scene.hidden_guids.clone();
        let group_locked = self.scene.group_locked.clone();
        let transform_locked = self.scene.transform_locked.clone();
        let face_colors = self.scene.face_color_overrides.clone();
        let pt_colors   = self.scene.point_color_overrides.clone();
        // When a viewport pick changed the selection, expand every ancestor group
        // of a selected leaf so the row is visible, then scroll it into view.
        let reveal_selection = self.scene.reveal_in_tree;
        self.scene.reveal_in_tree = false;
        if reveal_selection {
            for (node_guid, leaves) in &leaf_cache {
                if leaves.iter().any(|g| selected.contains(g)) {
                    egui_ctx.data_mut(|d| d.insert_persisted(egui::Id::new(("tree_open", node_guid)), true));
                }
            }
        }
        let box_select_snap = self.scene.box_select;
        let mut new_sel: Option<(Vec<String>, bool)> = None;
        let mut vis_chg: Vec<(String, bool)> = Vec::new();
        let mut lock_chg: Vec<(String, bool)> = Vec::new();
        let mut transform_lock_chg: Vec<(String, bool)> = Vec::new();
        let mut face_color_chg: Vec<(String, Option<[f32; 4]>)> = Vec::new();
        let mut pt_color_chg:   Vec<(String, Option<[f32; 4]>)> = Vec::new();
        let mut did_scroll = false;
        let mut tree_search_buf = self.shell.tree_search.clone();
        let line_thickness = self.scene.line_thickness;
        let mut new_line_thickness: Option<f32> = None;
        let plane_scale = self.scene.gpu_session.plane_scale;
        let mut new_plane_scale: Option<f32> = None;
        let tess_angle = self.scene.gpu_session.tess_angle_deg;
        let mut new_tess_angle: Option<f32> = None;
        let can_undo = !self.hist.undo_stack.is_empty();
        let can_redo = !self.hist.redo_stack.is_empty();
        let mut do_undo = false;
        let mut do_redo = false;

        // Control-point edit mode (F10) status, shown in the toolbar row.
        let edit_status: Option<String> = if self.edit.active {
            let kind = match self.edit.kind {
                Some(crate::edit_state::EditKind::Mesh) => "Mesh",
                Some(crate::edit_state::EditKind::Polyline) => "Polyline",
                Some(crate::edit_state::EditKind::NurbsCurve) => "NurbsCurve",
                Some(crate::edit_state::EditKind::NurbsSurface) => "NurbsSurface",
                Some(crate::edit_state::EditKind::BRep) => "BRep",
                None => "—",
            };
            let n = self.edit.move_node_set().len();
            Some(format!("Edit: {kind}  ({n} pts)"))
        } else {
            None
        };

        // Gumball numeric-entry popup (shown when a handle was clicked, not dragged).
        let mut gumball_input_snap = self.gb.gumball_input.clone();
        let mut gumball_apply: Option<f32> = None;
        let mut gumball_cancel = false;

        // Active draw tool: route typed input, detect Enter-to-finish / Esc-to-cancel,
        // and draw the current snap marker in screen space.
        let tool_active = self.tool.is_active();
        let mut tool_finish = false;
        let mut tool_cancel_req = false;
        let snap_marker: Option<(egui::Pos2, crate::snap::SnapKind)> = if tool_active {
            self.tool.cursor_snap.as_ref().and_then(|s| {
                if matches!(s.kind, crate::snap::SnapKind::Plane | crate::snap::SnapKind::Grid) {
                    return None;
                }
                let vp = self.vp_rect();
                let view_proj = self.scene.camera.view_proj();
                crate::state_tool::project_point(&view_proj, vp, &s.point).map(|(sx, sy)| {
                    let ppp = egui_ctx.pixels_per_point();
                    (egui::pos2(sx / ppp, sy / ppp), s.kind)
                })
            })
        } else {
            None
        };

        let cmd_log_snap = self.shell.cmd_log.clone();
        let mut cmd_input_buf = self.shell.cmd_input.clone();
        let mut cmd_submitted: Option<String> = None;
        let cmd_history_snap = self.shell.cmd_history.clone();
        let mut cmd_history_idx_buf = self.shell.cmd_history_idx;
        let mut cmd_history_saved_buf = self.shell.cmd_history_saved.clone();

        const CMDS: &[(&str, &str)] = &[
            ("box",      "sx sy sz"),
            ("sphere",   "r"),
            ("cyl",      "r h"),
            ("cone",     "r h"),
            ("torus",    "R r"),
            ("point",    "x y z"),
            ("line",     "x0 y0 z0 x1 y1 z1"),
            ("poly",     "n r"),
            ("curve",    "deg"),
            ("del",      ""),
            ("clear",    ""),
            ("fit",      ""),
            ("help",     ""),
        ];

        // Perf HUD snapshot (read before the UI closure borrows nothing of self).
        let stats = self.scene.draw_stats;
        let dt = egui_ctx.input(|i| i.stable_dt);
        let fps = if dt > 1e-6 { 1.0 / dt } else { 0.0 };
        let mut hud_cull = self.scene.frustum_cull;
        let hud_backend = self.gpu.backend.clone();
        let hud_ssao_ok = self.gpu.ssao_supported;
        let mut hud_arctic = self.scene.arctic;
        let mut hud_outline = self.scene.outline;
        let mut hud_outline_px = self.scene.outline_px;
        let mut hud_fxaa = self.scene.fxaa;
        let mut panel_visible = self.shell.panel_visible;
        let mut hud_gradient = self.scene.arctic_gradient;
        let mut hud_ao_mode = self.scene.ao_mode;
        let mut hud_intensity = self.scene.ssao_intensity;
        let mut hud_radius = self.scene.ssao_radius_pct;

        let full_output = egui_ctx.run_ui(raw_input, |ui| {
            egui::Area::new(egui::Id::new("perf_hud"))
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(8.0, 8.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style()).inner_margin(6.0).show(ui, |ui| {
                        ui.monospace(format!("{fps:5.1} fps"));
                        ui.monospace(format!(
                            "meshes {}/{}  culled {}",
                            stats.mesh_drawn, stats.mesh_objects, stats.mesh_culled,
                        ));
                        ui.monospace(format!("draw calls {}", stats.draw_calls));
                        ui.checkbox(&mut hud_cull, "frustum cull");
                        ui.checkbox(&mut hud_arctic, "arctic");
                        if hud_ssao_ok {
                            ui.checkbox(&mut hud_outline, "outline");
                            if hud_outline {
                                ui.add(egui::Slider::new(&mut hud_outline_px, 1.0..=8.0).text("outline px"));
                            }
                            ui.checkbox(&mut hud_fxaa, "fxaa");
                        }
                        if hud_arctic {
                            if hud_ssao_ok {
                                ui.monospace(format!("gpu {hud_backend}"));
                            } else {
                                ui.colored_label(
                                    egui::Color32::RED,
                                    format!("AO OFF — {hud_backend} backend (need WebGPU)"),
                                );
                            }
                            ui.checkbox(&mut hud_gradient, "gradient");
                            egui::ComboBox::from_id_salt("ao_mode")
                                .selected_text(match hud_ao_mode {
                                    0 => "SSAO",
                                    1 => "HBAO",
                                    _ => "GTAO",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut hud_ao_mode, 0, "SSAO");
                                    ui.selectable_value(&mut hud_ao_mode, 1, "HBAO");
                                    ui.selectable_value(&mut hud_ao_mode, 2, "GTAO");
                                });
                            ui.add(egui::Slider::new(&mut hud_intensity, 0.0..=1.5).text("ao"));
                            ui.add(egui::Slider::new(&mut hud_radius, 0.1..=6.0).text("radius %"));
                        }
                    });
                });

            egui::Panel::bottom("cli")
                .min_size(28.0)
                .max_size(120.0)
                .show_inside(ui, |ui| {
                    if !cmd_log_snap.is_empty() {
                        egui::ScrollArea::vertical()
                            .id_salt("cli_log")
                            .max_height(90.0)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                for line in &cmd_log_snap {
                                    ui.monospace(line);
                                }
                            });
                        ui.separator();
                    }
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(">").monospace()
                            .color(egui::Color32::from_rgb(80, 200, 120)));
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut cmd_input_buf)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace)
                                .hint_text("type a command or press ↑ for history"),
                        );
                        // Keep the command box focused while a draw tool is active, so the
                        // click-then-type flow works after a viewport click clears focus.
                        // Skip on the frame the field just lost focus (e.g. Enter), or we
                        // would re-grab focus and clobber the `lost_focus()` check below.
                        if tool_active && !resp.lost_focus() && ui.ctx().memory(|m| m.focused().is_none()) {
                            resp.request_focus();
                        }
                        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let s = cmd_input_buf.trim().to_string();
                            if !s.is_empty() {
                                cmd_submitted = Some(s);
                                cmd_input_buf = String::new();
                            } else if tool_active {
                                // Enter on an empty line finishes the active polyline.
                                tool_finish = true;
                            }
                            resp.request_focus();
                        }
                        if resp.has_focus() {
                            // History: ↑ / ↓
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                match cmd_history_idx_buf {
                                    None if !cmd_history_snap.is_empty() => {
                                        cmd_history_saved_buf = cmd_input_buf.clone();
                                        let i = cmd_history_snap.len() - 1;
                                        cmd_history_idx_buf = Some(i);
                                        cmd_input_buf = cmd_history_snap[i].clone();
                                    }
                                    Some(i) if i > 0 => {
                                        cmd_history_idx_buf = Some(i - 1);
                                        cmd_input_buf = cmd_history_snap[i - 1].clone();
                                    }
                                    _ => {}
                                }
                                resp.request_focus();
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                match cmd_history_idx_buf {
                                    Some(i) if i + 1 < cmd_history_snap.len() => {
                                        cmd_history_idx_buf = Some(i + 1);
                                        cmd_input_buf = cmd_history_snap[i + 1].clone();
                                    }
                                    Some(_) => {
                                        cmd_history_idx_buf = None;
                                        cmd_input_buf = cmd_history_saved_buf.clone();
                                    }
                                    _ => {}
                                }
                                resp.request_focus();
                            }
                            // Autocomplete: show popup when typing the first word
                            let q = cmd_input_buf.to_lowercase();
                            if !q.is_empty() && !q.contains(' ') {
                                let suggestions: Vec<(&str, &str)> = CMDS.iter()
                                    .filter(|(c, _)| c.starts_with(q.as_str()))
                                    .copied()
                                    .collect();
                                if !suggestions.is_empty() {
                                    // Tab accepts first suggestion
                                    let tabbed = ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
                                    let mut accepted: Option<String> = if tabbed {
                                        Some(format!("{} ", suggestions[0].0))
                                    } else {
                                        None
                                    };
                                    let row_h = 20.0_f32;
                                    let popup_h = suggestions.len() as f32 * row_h + 8.0;
                                    let rect = resp.rect;
                                    egui::Area::new(egui::Id::new("cmd_ac"))
                                        .order(egui::Order::Tooltip)
                                        .fixed_pos(egui::pos2(rect.left(), rect.top() - popup_h - 2.0))
                                        .show(ui.ctx(), |ui| {
                                            egui::Frame::popup(ui.style()).inner_margin(4.0).show(ui, |ui| {
                                                for (cmd, hint) in &suggestions {
                                                    let label = if hint.is_empty() {
                                                        egui::RichText::new(*cmd).monospace()
                                                    } else {
                                                        egui::RichText::new(format!("{cmd}  {hint}")).monospace()
                                                    };
                                                    if ui.selectable_label(false, label).clicked() {
                                                        accepted = Some(format!("{cmd} "));
                                                    }
                                                }
                                            });
                                        });
                                    if let Some(completion) = accepted {
                                        cmd_input_buf = completion;
                                        resp.request_focus();
                                    }
                                }
                            }
                        }
                    });
                });
            // Collapse toggle for the right panel — when hidden, the 3D viewport
            // expands to the full window (egui's available_rect drives it).
            egui::Area::new(egui::Id::new("panel_toggle"))
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-4.0, 4.0))
                .show(ui.ctx(), |ui| {
                    let label = if panel_visible { "▶" } else { "◀" };
                    if ui.button(label).on_hover_text("Show/hide panel").clicked() {
                        panel_visible = !panel_visible;
                    }
                });
            if panel_visible {
            egui::Panel::right("panel")
                .default_size(400.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if arrow_button(ui, egui::include_image!("../assets/undo.svg"), can_undo).on_hover_text("Undo").clicked() { do_undo = true; }
                        if arrow_button(ui, egui::include_image!("../assets/redo.svg"), can_redo).on_hover_text("Redo").clicked() { do_redo = true; }
                        if let Some(status) = &edit_status {
                            ui.label(egui::RichText::new(status).strong()
                                .color(egui::Color32::from_rgb(200, 110, 0)))
                                .on_hover_text("Control-point edit mode — F10 to exit, Ctrl+Shift+LMB picks points/edges");
                        }
                    });
                    ui.separator();
                    egui::CollapsingHeader::new("Tree")
                        .default_open(true)
                        .show(ui, |ui| {
                            // Search bar
                            ui.horizontal(|ui| {
                                ui.add(egui::TextEdit::singleline(&mut tree_search_buf)
                                    .hint_text("search…")
                                    .desired_width(f32::INFINITY));
                            });
                            // Column headers
                            let hdr_img = |src: egui::ImageSource<'static>| {
                                egui::Image::new(src).fit_to_exact_size(egui::vec2(14.0, 14.0))
                            };
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/pointlinecolor.svg")))
                                        .on_hover_text("Point/line color");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/facecolor.svg")))
                                        .on_hover_text("Face color");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/group.svg")))
                                        .on_hover_text("Group lock");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/lightbulb.svg")))
                                        .on_hover_text("Visibility");
                                    ui.separator();
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new("Name").color(egui::Color32::from_gray(130)));
                                    });
                                });
                            });
                            ui.separator();
                            let search_lc = tree_search_buf.to_lowercase();
                            if let Some(root) = &tree_root {
                                for child in &root.borrow().children() {
                                    render_tree_node(ui, child, &vmap, &selected, &hidden, &group_locked, &transform_locked, &face_colors, &pt_colors, &mut new_sel, &mut vis_chg, &mut lock_chg, &mut transform_lock_chg, &mut face_color_chg, &mut pt_color_chg, &leaf_cache, &search_lc, 0, reveal_selection, &mut did_scroll);
                                }
                            }
                        });
                    egui::CollapsingHeader::new("Graph")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(180);
                            for (v0, v1) in &edges {
                                let l0 = vmap.get(v0).map(|s| s.as_str()).unwrap_or(v0.as_str());
                                let l1 = vmap.get(v1).map(|s| s.as_str()).unwrap_or(v1.as_str());
                                let both_sel = selected.contains(v0) && selected.contains(v1);
                                let resp = ui.selectable_label(both_sel, format!("{l0} — {l1}"));
                                if resp.clicked() {
                                    let shift = ui.ctx().input(|i| i.modifiers.shift);
                                    new_sel = Some((vec![v0.clone(), v1.clone()], shift));
                                }
                            }
                        });
                    egui::CollapsingHeader::new("Settings")
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new("settings_grid").num_columns(2).show(ui, |ui| {
                                ui.label("Size");
                                let mut lt = line_thickness;
                                if ui.add(egui::Slider::new(&mut lt, 1.0..=120.0).suffix(" mm")).changed() {
                                    new_line_thickness = Some(lt);
                                }
                                ui.end_row();
                                ui.label("Plane Scale");
                                let mut ps = plane_scale;
                                if ui.add(egui::Slider::new(&mut ps, 10.0..=2000.0).suffix(" mm")).changed() {
                                    new_plane_scale = Some(ps);
                                }
                                ui.end_row();
                                ui.label("Tessellation");
                                // Lower angle = finer mesh (more triangles). Drives NURBS/BRep density.
                                let mut ta = tess_angle;
                                if ui.add(egui::Slider::new(&mut ta, 3.0..=30.0).suffix("°")
                                    .text("coarse→").logarithmic(true)).changed() {
                                    new_tess_angle = Some(ta);
                                }
                                ui.end_row();
                            });
                        });
                    egui::CollapsingHeader::new("Shortcuts")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("shortcuts_grid").num_columns(2).striped(true).show(ui, |ui| {
                                for (key, action) in &[
                                    ("RMB drag",     "orbit"),
                                    ("Shift+RMB",    "pan"),
                                    ("Scroll",       "zoom"),
                                    ("WASD / ↑↓←→", "pan"),
                                    ("C",            "reset camera"),
                                    ("P / O",        "perspective / ortho"),
                                    ("T/B/L/R",      "named views"),
                                    ("LMB",          "select"),
                                    ("LMB drag",     "box select"),
                                    ("Shift+LMB",    "add to selection"),
                                    ("Q",            "toggle shading"),
                                    ("E",            "toggle back-face color"),
                                    ("M",            "tessellation wireframe"),
                                    ("F10",          "edit control points"),
                                    ("Ctrl+Shift+LMB", "F10: pick point · else: move edge"),
                                ] {
                                    ui.monospace(*key);
                                    ui.label(*action);
                                    ui.end_row();
                                }
                            });
                        });
                    }); // ScrollArea
                });
            } // if panel_visible

            // Box select overlay
            if let Some(((x0, y0), (x1, y1))) = box_select_snap {
                let ppp = ui.ctx().pixels_per_point();
                let painter = ui.ctx().layer_painter(egui::LayerId::new(
                    egui::Order::Foreground, egui::Id::new("box_sel")));
                let is_crossing = x1 < x0;
                let min = egui::pos2(x0.min(x1) as f32 / ppp, y0.min(y1) as f32 / ppp);
                let max = egui::pos2(x0.max(x1) as f32 / ppp, y0.max(y1) as f32 / ppp);
                let rect = egui::Rect::from_min_max(min, max);
                let (fill, stroke_col) = if is_crossing {
                    (egui::Color32::from_rgba_unmultiplied(180, 180, 180, 25),
                     egui::Color32::from_rgba_unmultiplied(200, 200, 200, 220))
                } else {
                    (egui::Color32::from_rgba_unmultiplied(180, 180, 180, 25),
                     egui::Color32::from_rgba_unmultiplied(200, 200, 200, 220))
                };
                painter.rect_filled(rect, 0.0, fill);
                painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, stroke_col), egui::StrokeKind::Outside);
            }

            // Draw-tool snap marker (screen space, constant pixel size).
            if let Some((pos, kind)) = snap_marker {
                let painter = ui.ctx().layer_painter(egui::LayerId::new(
                    egui::Order::Foreground, egui::Id::new("snap_marker")));
                draw_snap_marker(&painter, pos, kind);
            }
            // Esc cancels the active draw tool (works whether or not the CLI field has focus).
            if tool_active && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                tool_cancel_req = true;
            }
            // Enter on an empty command line finishes the tool — backstop that works even
            // if the box isn't focused. Gated on `cmd_submitted.is_none()` so a coordinate
            // submit (which clears the buffer the same frame) can't be mistaken for finish.
            if tool_active && cmd_submitted.is_none() && cmd_input_buf.trim().is_empty()
                && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                tool_finish = true;
            }

            // Gumball numeric-entry popup
            if let Some(input) = &mut gumball_input_snap {
                let (action, axis, unit) = input.handle.labels();
                let title = if axis.is_empty() {
                    format!("{action} ({unit})")
                } else {
                    format!("{action} {axis} ({unit})")
                };
                let ppp = ui.ctx().pixels_per_point();
                let pos = egui::pos2(
                    input.screen_pos.0 / ppp + 14.0,
                    input.screen_pos.1 / ppp + 8.0,
                );
                egui::Area::new(egui::Id::new("gumball_input"))
                    .order(egui::Order::Foreground)
                    .fixed_pos(pos)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::popup(ui.style()).inner_margin(6.0).show(ui, |ui| {
                            // Single-line popup: "Move X (mm)" label + entry field.
                            // Enter applies (or closes if empty/invalid), Esc cancels.
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(title).strong());
                                let resp = ui.add(
                                    egui::TextEdit::singleline(&mut input.buffer)
                                        .desired_width(64.0)
                                        .font(egui::TextStyle::Monospace),
                                );
                                if input.focus {
                                    resp.request_focus();
                                    input.focus = false;
                                }
                                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    match input.buffer.trim().parse::<f32>() {
                                        Ok(v) => gumball_apply = Some(v),
                                        Err(_) => gumball_cancel = true,
                                    }
                                }
                            });
                        });
                    });
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    gumball_cancel = true;
                }
            }
        });

        // Visible area not covered by egui panels (in physical px) — every scene
        // pass is confined to this rect (set_viewport/scissor), so the 3D view
        // and the panel are truly separate regions.
        {
            let avail = self.shell.egui_ctx.available_rect();
            let ppp = self.shell.egui_ctx.pixels_per_point();
            self.scene.viewport_px = [
                avail.min.x * ppp,
                avail.min.y * ppp,
                avail.width() * ppp,
                avail.height() * ppp,
            ];
        }

        self.scene.frustum_cull = hud_cull;
        self.scene.arctic = hud_arctic;
        self.scene.outline = hud_outline;
        self.scene.outline_px = hud_outline_px;
        self.scene.fxaa = hud_fxaa;
        self.shell.panel_visible = panel_visible;
        self.scene.arctic_gradient = hud_gradient;
        self.scene.ao_mode = hud_ao_mode;
        self.scene.ssao_intensity = hud_intensity;
        self.scene.ssao_radius_pct = hud_radius;

        if do_undo { self.undo(); }
        if do_redo { self.redo(); }

        // Draw-tool: Esc cancels, empty Enter finishes the active polyline.
        if tool_cancel_req {
            self.tool_cancel();
        } else if tool_finish {
            self.tool_finish_open();
        }

        // Gumball popup: persist typed text/focus, then apply or cancel.
        if let (Some(orig), Some(snap)) = (self.gb.gumball_input.as_mut(), gumball_input_snap.as_ref()) {
            orig.buffer = snap.buffer.clone();
            orig.focus = snap.focus;
        }
        if gumball_cancel {
            self.cancel_gumball_input();
        } else if let Some(v) = gumball_apply {
            self.apply_gumball_input(v);
        }

        if let Some((guids, shift)) = new_sel {
            if shift {
                for guid in &guids {
                    if self.scene.selected_guids.contains(guid) {
                        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, false, &self.gpu.queue);
                        self.scene.selected_guids.remove(guid);
                    } else if self.scene.gpu_session.pick.instance_id(guid).is_some() {
                        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
                        self.scene.selected_guids.insert(guid.clone());
                    }
                }
            } else {
                let refs: Vec<&str> = guids.iter().map(|s| s.as_str()).collect();
                self.set_selection(&refs);
            }
            if !self.scene.selected_guids.is_empty() {
                let origin = self.selected_centroid();
                match &mut self.gb.gumball {
                    Some(gb) => gb.set_origin(origin),
                    None => self.gb.gumball = Some(Gumball::new(origin)),
                }
            } else {
                self.gb.gumball = None;
            }
        }

        for (guid, should_hide) in vis_chg {
            self.scene.gpu_session.set_flag(&guid, InstanceData::FLAG_HIDDEN, should_hide, &self.gpu.queue);
            if should_hide { self.scene.hidden_guids.insert(guid); } else { self.scene.hidden_guids.remove(&guid); }
        }
        for (name, should_lock) in lock_chg {
            if should_lock { self.scene.group_locked.insert(name); } else { self.scene.group_locked.remove(&name); }
        }
        for (name, should_lock) in transform_lock_chg {
            if should_lock { self.scene.transform_locked.insert(name); } else { self.scene.transform_locked.remove(&name); }
        }

        for (node_name, new_color) in face_color_chg {
            let leaves: Vec<String> = self.scene.leaf_guid_cache.get(&node_name)
                .cloned()
                .unwrap_or_else(|| vec![node_name.clone()]);
            if let Some(color) = new_color {
                self.scene.face_color_overrides.insert(node_name, color);
                for g in &leaves {
                    self.scene.face_color_overrides.insert(g.clone(), color);
                    self.scene.gpu_session.set_face_color(g, color, &self.gpu.queue);
                }
            } else {
                self.scene.face_color_overrides.remove(&node_name);
                for g in &leaves {
                    self.scene.face_color_overrides.remove(g);
                    self.scene.gpu_session.reset_color(g, &self.gpu.queue);
                }
            }
        }
        for (node_name, new_color) in pt_color_chg {
            let leaves: Vec<String> = self.scene.leaf_guid_cache.get(&node_name)
                .cloned()
                .unwrap_or_else(|| vec![node_name.clone()]);
            if let Some(color) = new_color {
                self.scene.point_color_overrides.insert(node_name, color);
                for g in &leaves {
                    self.scene.point_color_overrides.insert(g.clone(), color);
                    self.scene.gpu_session.set_color(g, color, &self.gpu.queue);
                }
            } else {
                self.scene.point_color_overrides.remove(&node_name);
                for g in &leaves {
                    self.scene.point_color_overrides.remove(g);
                    self.scene.gpu_session.reset_color(g, &self.gpu.queue);
                }
            }
        }

        if let Some(t) = new_line_thickness { self.scene.line_thickness = t; self.apply_thickness(); }

        if let Some(s) = new_plane_scale {
            self.scene.gpu_session.plane_scale = s;
            let plane_guids: Vec<String> = self.scene.session.lookup.iter()
                .filter_map(|(g, geom)| if matches!(geom, session_rust::Geometry::Plane(_)) { Some(g.clone()) } else { None })
                .collect();
            for guid in &plane_guids {
                self.scene.gpu_session.remove(guid);
                if let Some(geom) = self.scene.session.lookup.get(guid) {
                    self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                }
                self.reapply_visibility_flags(guid);
            }
        }

        if let Some(a) = new_tess_angle {
            // Set the runtime density (chord factor tracks the angle) and re-tessellate every
            // NURBS surface + BRep once. Coarse meshes make this rebuild cheap.
            self.scene.gpu_session.tess_angle_deg = a;
            self.scene.gpu_session.tess_chord_factor = a * 0.0003; // 10° → 0.003 (default ratio)
            let surfaces: Vec<session_rust::NurbsSurface> =
                self.scene.session.objects.nurbssurfaces.clone();
            for s in &surfaces {
                let guid = s.guid().to_string();
                self.scene.gpu_session.remove(&guid);
                self.scene.gpu_session.add_nurbssurface(s, &self.gpu.device, &self.gpu.queue);
                self.reapply_visibility_flags(&guid);
                self.reapply_color_overrides(&guid);
            }
            let brep_guids: Vec<String> = self.scene.session.lookup.iter()
                .filter_map(|(g, geom)| if matches!(geom, session_rust::Geometry::BRep(_)) { Some(g.clone()) } else { None })
                .collect();
            for guid in &brep_guids {
                self.scene.gpu_session.remove(guid);
                if let Some(geom) = self.scene.session.lookup.get(guid) {
                    self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                }
                self.reapply_visibility_flags(guid);
                self.reapply_color_overrides(guid);
            }
            self.apply_thickness();
        }

        self.shell.cmd_input = cmd_input_buf;
        self.shell.cmd_history_idx = cmd_history_idx_buf;
        self.shell.cmd_history_saved = cmd_history_saved_buf;
        self.shell.tree_search = tree_search_buf;
        if let Some(cmd) = cmd_submitted {
            if self.shell.cmd_history.last().map(|s| s.as_str()) != Some(cmd.as_str()) {
                self.shell.cmd_history.push(cmd.clone());
            }
            self.shell.cmd_history_idx = None;
            self.shell.cmd_history_saved = String::new();
            self.shell.cmd_log.push(format!("> {cmd}"));
            let result = if self.tool.is_active() {
                self.tool_on_text(&cmd)
            } else {
                self.execute_command(&cmd)
            };
            if !result.is_empty() {
                for line in result.lines() {
                    self.shell.cmd_log.push(line.to_string());
                }
            }
            if self.shell.cmd_log.len() > 200 {
                let drain = self.shell.cmd_log.len() - 200;
                self.shell.cmd_log.drain(0..drain);
            }
        }

        full_output
    }
}
