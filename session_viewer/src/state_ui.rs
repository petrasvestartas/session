use std::collections::HashMap;
use std::sync::Arc;
use crate::State;
use crate::gpu_session::InstanceData;
use crate::gumball::Gumball;
use crate::tree_ui::{self, populate_leaf_cache, render_tree_node};

impl State {
    pub(crate) fn build_ui(&mut self) -> egui::FullOutput {
        let egui_ctx = self.shell.egui_ctx.clone();
        let window = Arc::clone(&self.window);
        let raw_input = self.shell.egui_state.take_egui_input(&window);

        let tree_root = self.scene.session.tree.root();
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
        let mut vmap: HashMap<String, String> = self.scene.session.lookup
            .iter()
            .map(|(guid, geom)| {
                let name = geom_name(geom);
                let label = if name.is_empty() { guid.clone() } else { name.to_string() };
                (guid.clone(), label)
            })
            .collect();
        for ns in &self.scene.session.objects.nurbssurfaces {
            let g = ns.guid().to_string();
            let label = if ns.name.is_empty() { g.clone() } else { ns.name.clone() };
            vmap.entry(g).or_insert(label);
        }
        for nc in &self.scene.session.objects.nurbscurves {
            let g = nc.guid().to_string();
            let label = if nc.name.is_empty() { g.clone() } else { nc.name.clone() };
            vmap.entry(g).or_insert(label);
        }
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
        let box_select_snap = self.scene.box_select;
        let mut new_sel: Option<(Vec<String>, bool)> = None;
        let mut vis_chg: Vec<(String, bool)> = Vec::new();
        let mut lock_chg: Vec<(String, bool)> = Vec::new();
        let mut transform_lock_chg: Vec<(String, bool)> = Vec::new();
        let mut face_color_chg: Vec<(String, Option<[f32; 4]>)> = Vec::new();
        let mut pt_color_chg:   Vec<(String, Option<[f32; 4]>)> = Vec::new();
        let mut tree_search_buf = self.shell.tree_search.clone();
        let line_thickness = self.scene.line_thickness;
        let mut new_line_thickness: Option<f32> = None;
        let plane_scale = self.scene.gpu_session.plane_scale;
        let mut new_plane_scale: Option<f32> = None;
        let can_undo = !self.hist.undo_stack.is_empty();
        let can_redo = !self.hist.redo_stack.is_empty();
        let mut do_undo = false;
        let mut do_redo = false;

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
            ("del",      ""),
            ("clear",    ""),
            ("fit",      ""),
            ("help",     ""),
        ];

        let full_output = egui_ctx.run_ui(raw_input, |ui| {
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
                        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let s = cmd_input_buf.trim().to_string();
                            if !s.is_empty() {
                                cmd_submitted = Some(s);
                                cmd_input_buf = String::new();
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
            egui::Panel::right("panel")
                .default_size(400.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let arrow = |s: &str| egui::Button::new(egui::RichText::new(s).size(18.0)).min_size(egui::vec2(28.0, 24.0));
                        if ui.add_enabled(can_undo, arrow("↶")).on_hover_text("Undo (Ctrl+Z)").clicked() { do_undo = true; }
                        if ui.add_enabled(can_redo, arrow("↷")).on_hover_text("Redo (Ctrl+U)").clicked() { do_redo = true; }
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
                                        hdr_img(egui::include_image!("../assets/group.svg")))
                                        .on_hover_text("Group lock");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/lightbulb.svg")))
                                        .on_hover_text("Visibility");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/pointlinecolor.svg")))
                                        .on_hover_text("Point/line color");
                                    ui.separator();
                                    ui.add_sized([tree_ui::ICON_W, tree_ui::ROW_H],
                                        hdr_img(egui::include_image!("../assets/facecolor.svg")))
                                        .on_hover_text("Face color");
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
                                    render_tree_node(ui, child, &vmap, &selected, &hidden, &group_locked, &transform_locked, &face_colors, &pt_colors, &mut new_sel, &mut vis_chg, &mut lock_chg, &mut transform_lock_chg, &mut face_color_chg, &mut pt_color_chg, &leaf_cache, &search_lc, 0);
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
                                    ("Ctrl+Z",       "undo"),
                                    ("Ctrl+U",       "redo"),
                                ] {
                                    ui.monospace(*key);
                                    ui.label(*action);
                                    ui.end_row();
                                }
                            });
                        });
                    }); // ScrollArea
                });

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
        });

        if do_undo { self.undo(); }
        if do_redo { self.redo(); }

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
            let result = self.execute_command(&cmd);
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
