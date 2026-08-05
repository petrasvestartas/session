use crate::State;
use crate::tool_state::DrawTool;
use crate::undo_state::{UndoAction, replace_or_push_nurbs};
use session_rust::session::Geometry;
use session_rust::{BRep, Color, Line, Point, Polyline, Primitives, Session, TreeNode};

impl State {

    pub(crate) fn apply_thickness(&mut self) {
        // Thickness is driven by camera.point_size uploaded every frame — no CPU work needed.
    }

    pub(crate) fn execute_command(&mut self, cmd: &str) -> String {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { return String::new(); }

        fn p(parts: &[&str], i: usize, default: f32) -> f32 {
            parts.get(i).and_then(|s| s.parse().ok()).unwrap_or(default)
        }

        match parts[0].to_lowercase().as_str() {
            "box" => {
                let sx = p(&parts, 1, 100.0);
                let sy = p(&parts, 2, sx);
                let sz = p(&parts, 3, sx);
                let mut b = BRep::create_box(sx as f64, sy as f64, sz as f64);
                let name = format!("box_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                b.name = name.clone();
                let guid = b.guid().to_string();
                self.scene.session.add_brep(b, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  ({sx}×{sy}×{sz} mm)")
            }
            "sphere" => {
                let r = p(&parts, 1, 50.0);
                let mut b = BRep::create_sphere(r as f64);
                let name = format!("sphere_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                b.name = name.clone();
                let guid = b.guid().to_string();
                self.scene.session.add_brep(b, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  (r={r} mm)")
            }
            "cylinder" | "cyl" => {
                let r = p(&parts, 1, 30.0);
                let h = p(&parts, 2, 80.0);
                let mut b = BRep::create_cylinder(r as f64, h as f64);
                let name = format!("cyl_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                b.name = name.clone();
                let guid = b.guid().to_string();
                self.scene.session.add_brep(b, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  (r={r} h={h} mm)")
            }
            "cone" => {
                let r = p(&parts, 1, 30.0);
                let h = p(&parts, 2, 80.0);
                let name = format!("cone_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let mut ns = Primitives::cone_surface(0.0, 0.0, 0.0, r as f64, h as f64);
                ns.name = name.clone();
                ns.set_guid(name.clone());
                self.scene.geom_guid_set.insert(name.clone()); self.scene.leaf_cache_dirty = true;
                self.scene.gpu_session.add_nurbssurface(&ns, &self.gpu.device, &self.gpu.queue);
                let node = TreeNode::new(ns.guid());
                self.scene.session.tree.add(&node, None);
                self.hist.push(UndoAction::AddNurbs { ns: ns.clone() });
                replace_or_push_nurbs(&mut self.scene.session.objects.nurbssurfaces, ns);
                format!("+ {name}  (r={r} h={h} mm)")
            }
            "torus" => {
                let big_r = p(&parts, 1, 50.0);
                let small_r = p(&parts, 2, 15.0);
                let name = format!("torus_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let mut ns = Primitives::torus_surface(0.0, 0.0, 0.0, big_r as f64, small_r as f64);
                ns.name = name.clone();
                ns.set_guid(name.clone());
                self.scene.geom_guid_set.insert(name.clone()); self.scene.leaf_cache_dirty = true;
                self.scene.gpu_session.add_nurbssurface(&ns, &self.gpu.device, &self.gpu.queue);
                let node = TreeNode::new(ns.guid());
                self.scene.session.tree.add(&node, None);
                self.hist.push(UndoAction::AddNurbs { ns: ns.clone() });
                replace_or_push_nurbs(&mut self.scene.session.objects.nurbssurfaces, ns);
                format!("+ {name}  (R={big_r} r={small_r} mm)")
            }
            "point" | "pt" => {
                if parts.len() == 1 {
                    self.start_draw_tool(DrawTool::Point);
                    return "Point: click in the view or type x,y,z  (Esc to stop)".to_string();
                }
                let x = p(&parts, 1, 0.0);
                let y = p(&parts, 2, 0.0);
                let z = p(&parts, 3, 0.0);
                let mut pt = Point::new(x as f64, y as f64, z as f64);
                let name = format!("pt_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                pt.name = name.clone();
                pt.pointcolor = Color::new(1.0, 0.8, 0.2, 1.0);
                let guid = pt.guid().to_string();
                self.scene.session.add_point(pt, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  ({x}, {y}, {z})")
            }
            "line" | "ln" => {
                if parts.len() == 1 {
                    self.start_draw_tool(DrawTool::Line);
                    return "Line: start point — click or type x,y,z  (Esc to cancel)".to_string();
                }
                let x0 = p(&parts, 1, 0.0); let y0 = p(&parts, 2, 0.0); let z0 = p(&parts, 3, 0.0);
                let x1 = p(&parts, 4, 100.0); let y1 = p(&parts, 5, 0.0); let z1 = p(&parts, 6, 0.0);
                let name = format!("line_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let mut l = Line::from_points(&Point::new(x0 as f64, y0 as f64, z0 as f64), &Point::new(x1 as f64, y1 as f64, z1 as f64));
                l.name = name.clone();
                let guid = l.guid().to_string();
                self.scene.session.add_line(l, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  ({x0},{y0},{z0})→({x1},{y1},{z1})")
            }
            "polyline" | "poly" => {
                if parts.len() == 1 {
                    self.start_draw_tool(DrawTool::Polyline);
                    return "Polyline: first point — click or type x,y,z  (Enter=finish, c=close, Esc=cancel)".to_string();
                }
                let n = p(&parts, 1, 4.0).round() as usize;
                let r = p(&parts, 2, 50.0);
                let n = n.max(3);
                let name = format!("poly_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let pts: Vec<Point> = (0..=n).map(|i| {
                    let a = std::f32::consts::TAU * i as f32 / n as f32;
                    Point::new((r * a.cos()) as f64, (r * a.sin()) as f64, 0.0)
                }).collect();
                let mut pl = Polyline::new(pts);
                pl.name = name.clone();
                pl.linecolor = Color::new(0.4, 0.9, 1.0, 1.0);
                let guid = pl.guid().to_string();
                self.scene.session.add_polyline(pl, None);
                if let Some(geom) = self.scene.session.lookup.get(&guid) {
                    self.scene.gpu_session.add_geometry(&guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.hist.push(UndoAction::AddLookup { guid: guid.clone(), geom: geom.clone() });
                }
                self.scene.geom_guid_set.insert(guid); self.scene.leaf_cache_dirty = true;
                format!("+ {name}  ({n}-gon, r={r} mm)")
            }
            "curve" | "nurbscurve" | "crv" => {
                let degree = if parts.len() >= 2 {
                    parts[1].parse::<usize>().unwrap_or(3).max(1)
                } else {
                    3
                };
                self.start_draw_tool(DrawTool::NurbsCurve { degree });
                return format!("Curve (deg {degree}): click control points or type x,y,z  (Enter=finish, u=undo, Esc=cancel)");
            }
            "move" | "m" => {
                // Edit mode: move the selected edge / CVs (osnap point-to-point) via the edit deform.
                if self.edit.active {
                    if self.begin_edit_move() {
                        self.start_draw_tool(DrawTool::Move);
                        self.tool.move_edit = true;
                        return "Move edge: base point — click/snap, type distance / x,y,z  (Esc)".to_string();
                    }
                    return "Move: pick an edge or CV first (Ctrl+Shift+LMB)".to_string();
                }
                if self.scene.selected_guids.is_empty() {
                    return "Move: select object(s) first, then run move".to_string();
                }
                let origins: Vec<(String, [[f32; 4]; 4])> = self.scene.selected_guids.iter()
                    .filter_map(|g| {
                        self.scene.gpu_session.pick.instance_id(g)
                            .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                            .map(|inst| (g.clone(), inst.model))
                    })
                    .collect();
                if origins.is_empty() {
                    return "Move: selection has no movable instance".to_string();
                }
                self.start_draw_tool(DrawTool::Move);
                self.tool.move_origins = origins;
                "Move: base point — click/snap, or type x,y,z  (Esc cancel)".to_string()
            }
            "del" | "delete" | "rm" => {
                let guids: Vec<String> = self.scene.selected_guids.drain().collect();
                let to_remove: Vec<(String, Geometry)> = guids.iter()
                    .filter_map(|g| self.scene.session.lookup.get(g).map(|geom| (g.clone(), geom.clone())))
                    .collect();
                // NurbsSurfaces (cone/torus) live in objects.nurbssurfaces, not lookup.
                // They move matrix-only, so the live pose is in the GPU instance model, not
                // the CVs — bake it into the stored clone so undo-delete restores the move.
                let nurbs_removed: Vec<session_rust::NurbsSurface> = guids.iter()
                    .filter_map(|g| {
                        self.scene.session.objects.nurbssurfaces.iter().find(|n| n.guid() == *g).map(|n| {
                            let mut clone = (**n).clone();
                            if let Some(iid) = self.scene.gpu_session.pick.instance_id(g) {
                                if let Some(inst) = self.scene.gpu_session.instances_cpu.get(iid as usize) {
                                    let m = inst.model;
                                    let flat = [
                                        m[0][0] as f64, m[0][1] as f64, m[0][2] as f64, m[0][3] as f64,
                                        m[1][0] as f64, m[1][1] as f64, m[1][2] as f64, m[1][3] as f64,
                                        m[2][0] as f64, m[2][1] as f64, m[2][2] as f64, m[2][3] as f64,
                                        m[3][0] as f64, m[3][1] as f64, m[3][2] as f64, m[3][3] as f64,
                                    ];
                                    clone.transform(&session_rust::Xform::from_matrix(flat));
                                }
                            }
                            clone
                        })
                    })
                    .collect();
                // NurbsCurves (the interactive `curve` tool) live in objects.nurbscurves.
                let curves_removed: Vec<session_rust::NurbsCurve> = guids.iter()
                    .filter_map(|g| self.scene.session.objects.nurbscurves.iter().find(|n| n.guid() == *g).map(|n| (**n).clone()))
                    .collect();
                let n = guids.len();
                for guid in &guids {
                    self.scene.gpu_session.remove(guid);
                    self.scene.session.lookup.remove(guid);
                    self.scene.session.objects.nurbssurfaces.retain(|n| n.guid() != *guid);
                    self.scene.session.objects.nurbscurves.retain(|n| n.guid() != *guid);
                    self.scene.geom_guid_set.remove(guid);
                }
                self.scene.leaf_cache_dirty = true;
                self.gb.gumball = None;
                if !to_remove.is_empty() || !nurbs_removed.is_empty() || !curves_removed.is_empty() {
                    self.hist.push(UndoAction::RemoveObjects { objects: to_remove, nurbs: nurbs_removed, curves: curves_removed });
                }
                format!("deleted {n} object(s)")
            }
            "clear" => {
                self.scene.session = Session::new("viewer");
                self.scene.gpu_session.rebuild_from(&self.scene.session, &self.gpu.device, &self.gpu.queue);
                self.scene.selected_guids.clear();
                self.scene.hidden_guids.clear();
                self.scene.geom_guid_set.clear();
                self.scene.leaf_cache_dirty = true;
                self.scene.glyphs_hidden_guids.clear();
                self.gb.gumball = None;
                self.scene.text_labels.clear();
                "scene cleared".to_string()
            }
            "fit" | "f" => {
                self.fit_view();
                "fit".to_string()
            }
            "help" | "?" => {
                "box [sx sy sz]  sphere [r]  cyl [r h]  cone [r h]  torus [R r]\npoint [x y z]  line [x0 y0 z0 x1 y1 z1]  poly [n r]\ndel  clear  fit".to_string()
            }
            other => format!("unknown: '{other}'  (type 'help')"),
        }
    }
}
