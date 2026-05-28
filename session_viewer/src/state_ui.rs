impl State {
    fn build_ui(&mut self) -> egui::FullOutput {
        let egui_ctx = self.egui_ctx.clone();
        let window = Arc::clone(&self.window);
        let raw_input = self.egui_state.take_egui_input(&window);

        let tree_root = self.session.tree.root();
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
        let mut vmap: HashMap<String, String> = self.session.lookup
            .iter()
            .map(|(guid, geom)| {
                let name = geom_name(geom);
                let label = if name.is_empty() { guid.clone() } else { name.to_string() };
                (guid.clone(), label)
            })
            .collect();
        for ns in &self.session.objects.nurbssurfaces {
            let g = ns.guid().to_string();
            let label = if ns.name.is_empty() { g.clone() } else { ns.name.clone() };
            vmap.entry(g).or_insert(label);
        }
        for nc in &self.session.objects.nurbscurves {
            let g = nc.guid().to_string();
            let label = if nc.name.is_empty() { g.clone() } else { nc.name.clone() };
            vmap.entry(g).or_insert(label);
        }
        if self.leaf_cache_dirty {
            self.leaf_guid_cache.clear();
            self.leaf_cache_dirty = false;
        }
        if let Some(root) = &tree_root {
            populate_leaf_cache(root, &vmap, &mut self.leaf_guid_cache);
        }
        let leaf_cache = self.leaf_guid_cache.clone();

        let edges = self.session.graph.get_edges();
        let selected = self.selected_guids.clone();
        let hidden = self.hidden_guids.clone();
        let locked = self.group_locked.clone();
        let mut new_sel: Option<(Vec<String>, bool)> = None;
        let mut vis_chg: Vec<(String, bool)> = Vec::new();
        let mut lock_chg: Vec<(String, bool)> = Vec::new();
        let line_thickness = self.line_thickness;
        let mut new_line_thickness: Option<f32> = None;
        let plane_scale = self.gpu_session.plane_scale;
        let mut new_plane_scale: Option<f32> = None;

        let cmd_log_snap = self.cmd_log.clone();
        let mut cmd_input_buf = self.cmd_input.clone();
        let mut cmd_submitted: Option<String> = None;
        let cmd_history_snap = self.cmd_history.clone();
        let mut cmd_history_idx_buf = self.cmd_history_idx;
        let mut cmd_history_saved_buf = self.cmd_history_saved.clone();

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
                .default_size(240.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::CollapsingHeader::new("Tree")
                        .default_open(true)
                        .show(ui, |ui| {
                            if let Some(root) = &tree_root {
                                for child in &root.borrow().children() {
                                    render_tree_node(ui, child, &vmap, &selected, &hidden, &locked, &mut new_sel, &mut vis_chg, &mut lock_chg, &leaf_cache);
                                }
                            }
                        });
                    egui::CollapsingHeader::new("Graph")
                        .default_open(false)
                        .show(ui, |ui| {
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
                                    ("Shift+LMB",    "add to selection"),
                                    ("Q",            "toggle shading"),
                                    ("E",            "toggle back-face color"),
                                ] {
                                    ui.monospace(*key);
                                    ui.label(*action);
                                    ui.end_row();
                                }
                            });
                        });
                    }); // ScrollArea
                });
        });

        if let Some((guids, shift)) = new_sel {
            if shift {
                for guid in &guids {
                    if self.selected_guids.contains(guid) {
                        self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, false, &self.queue);
                        self.selected_guids.remove(guid);
                    } else if self.gpu_session.pick.instance_id(guid).is_some() {
                        self.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.queue);
                        self.selected_guids.insert(guid.clone());
                    }
                }
            } else {
                let refs: Vec<&str> = guids.iter().map(|s| s.as_str()).collect();
                self.set_selection(&refs);
            }
            if !self.selected_guids.is_empty() {
                let origin = self.selected_centroid();
                match &mut self.gumball {
                    Some(gb) => gb.set_origin(origin),
                    None => self.gumball = Some(Gumball::new(origin)),
                }
            } else {
                self.gumball = None;
            }
        }

        for (guid, should_hide) in vis_chg {
            self.gpu_session.set_flag(&guid, InstanceData::FLAG_HIDDEN, should_hide, &self.queue);
            if should_hide { self.hidden_guids.insert(guid); } else { self.hidden_guids.remove(&guid); }
        }
        for (name, should_lock) in lock_chg {
            if should_lock { self.group_locked.insert(name); } else { self.group_locked.remove(&name); }
        }

        if let Some(t) = new_line_thickness { self.line_thickness = t; self.apply_thickness(); }

        if let Some(s) = new_plane_scale {
            self.gpu_session.plane_scale = s;
            let plane_guids: Vec<String> = self.session.lookup.iter()
                .filter_map(|(g, geom)| if matches!(geom, session_rust::Geometry::Plane(_)) { Some(g.clone()) } else { None })
                .collect();
            for guid in &plane_guids {
                self.gpu_session.remove(guid);
                if let Some(geom) = self.session.lookup.get(guid) {
                    self.gpu_session.add_geometry(guid, geom, &self.device, &self.queue);
                }
                self.reapply_visibility_flags(guid);
            }
        }

        self.cmd_input = cmd_input_buf;
        self.cmd_history_idx = cmd_history_idx_buf;
        self.cmd_history_saved = cmd_history_saved_buf;
        if let Some(cmd) = cmd_submitted {
            if self.cmd_history.last().map(|s| s.as_str()) != Some(cmd.as_str()) {
                self.cmd_history.push(cmd.clone());
            }
            self.cmd_history_idx = None;
            self.cmd_history_saved = String::new();
            self.cmd_log.push(format!("> {cmd}"));
            let result = self.execute_command(&cmd);
            if !result.is_empty() {
                for line in result.lines() {
                    self.cmd_log.push(line.to_string());
                }
            }
            if self.cmd_log.len() > 200 {
                let drain = self.cmd_log.len() - 200;
                self.cmd_log.drain(0..drain);
            }
        }

        full_output
    }
}
