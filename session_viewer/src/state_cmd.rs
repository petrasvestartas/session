use crate::State;
use crate::undo_state::UndoAction;
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
                let mut b = BRep::create_box(sx, sy, sz);
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
                let mut b = BRep::create_sphere(r);
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
                let mut b = BRep::create_cylinder(r, h);
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
                let mut ns = Primitives::cone_surface(0.0, 0.0, 0.0, r, h);
                ns.name = name.clone();
                ns.set_guid(name.clone());
                self.scene.geom_guid_set.insert(name.clone()); self.scene.leaf_cache_dirty = true;
                self.scene.gpu_session.add_nurbssurface(&ns, &self.gpu.device, &self.gpu.queue);
                let node = TreeNode::new(ns.guid());
                self.scene.session.tree.add(&node, None);
                self.hist.push(UndoAction::AddNurbs { ns: ns.clone() });
                self.scene.session.objects.nurbssurfaces.push(ns);
                format!("+ {name}  (r={r} h={h} mm)")
            }
            "torus" => {
                let big_r = p(&parts, 1, 50.0);
                let small_r = p(&parts, 2, 15.0);
                let name = format!("torus_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let mut ns = Primitives::torus_surface(0.0, 0.0, 0.0, big_r, small_r);
                ns.name = name.clone();
                ns.set_guid(name.clone());
                self.scene.geom_guid_set.insert(name.clone()); self.scene.leaf_cache_dirty = true;
                self.scene.gpu_session.add_nurbssurface(&ns, &self.gpu.device, &self.gpu.queue);
                let node = TreeNode::new(ns.guid());
                self.scene.session.tree.add(&node, None);
                self.hist.push(UndoAction::AddNurbs { ns: ns.clone() });
                self.scene.session.objects.nurbssurfaces.push(ns);
                format!("+ {name}  (R={big_r} r={small_r} mm)")
            }
            "point" | "pt" => {
                let x = p(&parts, 1, 0.0);
                let y = p(&parts, 2, 0.0);
                let z = p(&parts, 3, 0.0);
                let mut pt = Point::new(x, y, z);
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
                let x0 = p(&parts, 1, 0.0); let y0 = p(&parts, 2, 0.0); let z0 = p(&parts, 3, 0.0);
                let x1 = p(&parts, 4, 100.0); let y1 = p(&parts, 5, 0.0); let z1 = p(&parts, 6, 0.0);
                let name = format!("line_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let mut l = Line::from_points(&Point::new(x0, y0, z0), &Point::new(x1, y1, z1));
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
                let n = p(&parts, 1, 4.0).round() as usize;
                let r = p(&parts, 2, 50.0);
                let n = n.max(3);
                let name = format!("poly_{}", self.shell.cmd_counter);
                self.shell.cmd_counter += 1;
                let pts: Vec<Point> = (0..=n).map(|i| {
                    let a = std::f32::consts::TAU * i as f32 / n as f32;
                    Point::new(r * a.cos(), r * a.sin(), 0.0)
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
            "del" | "delete" | "rm" => {
                let guids: Vec<String> = self.scene.selected_guids.drain().collect();
                let to_remove: Vec<(String, Geometry)> = guids.iter()
                    .filter_map(|g| self.scene.session.lookup.get(g).map(|geom| (g.clone(), geom.clone())))
                    .collect();
                let n = guids.len();
                for guid in &guids {
                    self.scene.gpu_session.remove(guid);
                    self.scene.session.lookup.remove(guid);
                    self.scene.geom_guid_set.remove(guid);
                }
                self.scene.leaf_cache_dirty = true;
                self.gb.gumball = None;
                if !to_remove.is_empty() {
                    self.hist.push(UndoAction::RemoveObjects { objects: to_remove });
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
