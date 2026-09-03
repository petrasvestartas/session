use crate::State;
use crate::gpu_session::InstanceData;
use crate::undo_state::{GeomSnapshot, UndoAction, replace_or_push_nurbs, replace_or_push_nurbscurve};

impl State {
    pub fn undo(&mut self) {
        if let Some(action) = self.hist.undo_stack.pop() {
            self.apply_undo(&action);
            self.hist.redo_stack.push(action);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.hist.redo_stack.pop() {
            self.apply_redo(&action);
            self.hist.undo_stack.push(action);
        }
    }

    pub(crate) fn apply_undo(&mut self, action: &UndoAction) {
        match action {
            UndoAction::AddLookup { guid, .. } => {
                self.scene.gpu_session.remove(guid);
                self.scene.session.lookup.remove(guid);
                self.scene.geom_guid_set.remove(guid);
                self.scene.leaf_cache_dirty = true;
                self.scene.selected_guids.remove(guid);
                if self.scene.selected_guids.is_empty() { self.gb.gumball = None; }
            }
            UndoAction::AddNurbs { ns } => {
                let guid = ns.guid().to_string();
                self.scene.gpu_session.remove(&guid);
                self.scene.session.objects.nurbssurfaces.retain(|n| n.guid() != guid);
                self.scene.geom_guid_set.remove(&guid);
                self.scene.leaf_cache_dirty = true;
                self.scene.selected_guids.remove(&guid);
                if self.scene.selected_guids.is_empty() { self.gb.gumball = None; }
            }
            UndoAction::AddNurbsCurve { nc } => {
                let guid = nc.guid().to_string();
                self.scene.gpu_session.remove(&guid);
                self.scene.session.objects.nurbscurves.retain(|n| n.guid() != guid);
                self.scene.geom_guid_set.remove(&guid);
                self.scene.leaf_cache_dirty = true;
                self.scene.selected_guids.remove(&guid);
                if self.scene.selected_guids.is_empty() { self.gb.gumball = None; }
            }
            UndoAction::RemoveObjects { objects, nurbs, curves } => {
                for (guid, geom) in objects {
                    self.scene.session.lookup.insert(guid.clone(), geom.clone());
                    self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.scene.geom_guid_set.insert(guid.clone());
                }
                for ns in nurbs {
                    let guid = ns.guid().to_string();
                    self.scene.gpu_session.add_nurbssurface(ns, &self.gpu.device, &self.gpu.queue);
                    replace_or_push_nurbs(&mut self.scene.session.objects.nurbssurfaces, ns.clone());
                    self.scene.geom_guid_set.insert(guid);
                }
                for nc in curves {
                    let guid = nc.guid().to_string();
                    self.scene.gpu_session.add_nurbscurve(nc, &self.gpu.device, &self.gpu.queue);
                    replace_or_push_nurbscurve(&mut self.scene.session.objects.nurbscurves, nc.clone());
                    self.scene.geom_guid_set.insert(guid);
                }
                self.scene.leaf_cache_dirty = true;
                // `del` cleared the selection; undo restores it so you land back where you were.
                let restored: Vec<String> = objects.iter().map(|(g, _)| g.clone())
                    .chain(nurbs.iter().map(|n| n.guid().to_string()))
                    .chain(curves.iter().map(|n| n.guid().to_string()))
                    .collect();
                let refs: Vec<&str> = restored.iter().map(|s| s.as_str()).collect();
                self.set_selection(&refs);
            }
            UndoAction::Transform { objects, snapshots, .. } => {
                for (guid, before, _after) in objects {
                    let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                        .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                        .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
                    match snapshots.get(guid) {
                        Some(GeomSnapshot::Geom(geom)) => {
                            self.scene.gpu_session.remove(guid);
                            self.scene.session.lookup.insert(guid.clone(), geom.clone());
                            self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                        }
                        Some(GeomSnapshot::Nurbs(ns)) => {
                            self.scene.gpu_session.remove(guid);
                            if let Some(slot) = self.scene.session.objects.nurbssurfaces.iter_mut().find(|n| n.guid() == guid) {
                                *slot = std::rc::Rc::new(ns.clone());
                            }
                            self.scene.gpu_session.add_nurbssurface(ns, &self.gpu.device, &self.gpu.queue);
                        }
                        None => {
                            self.commit_object_transform(guid, *before);
                        }
                    }
                    if was_selected {
                        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
                    }
                }
                if !self.scene.selected_guids.is_empty() {
                    let origin = self.selected_centroid();
                    if let Some(gb) = &mut self.gb.gumball { gb.set_origin(origin); }
                }
            }
            UndoAction::EditGeom { guid, before, .. } => {
                self.restore_edit_snapshot(guid, before);
            }
        }
    }

    pub(crate) fn apply_redo(&mut self, action: &UndoAction) {
        match action {
            UndoAction::AddLookup { guid, geom } => {
                self.scene.session.lookup.insert(guid.clone(), geom.clone());
                self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                self.scene.geom_guid_set.insert(guid.clone());
                self.scene.leaf_cache_dirty = true;
            }
            UndoAction::AddNurbs { ns } => {
                let ns_clone = ns.clone();
                let guid = ns_clone.guid().to_string();
                self.scene.gpu_session.add_nurbssurface(&ns_clone, &self.gpu.device, &self.gpu.queue);
                replace_or_push_nurbs(&mut self.scene.session.objects.nurbssurfaces, ns_clone);
                self.scene.geom_guid_set.insert(guid);
                self.scene.leaf_cache_dirty = true;
            }
            UndoAction::AddNurbsCurve { nc } => {
                let nc_clone = nc.clone();
                let guid = nc_clone.guid().to_string();
                self.scene.gpu_session.add_nurbscurve(&nc_clone, &self.gpu.device, &self.gpu.queue);
                replace_or_push_nurbscurve(&mut self.scene.session.objects.nurbscurves, nc_clone);
                self.scene.geom_guid_set.insert(guid);
                self.scene.leaf_cache_dirty = true;
            }
            UndoAction::RemoveObjects { objects, nurbs, curves } => {
                for (guid, _) in objects {
                    self.scene.gpu_session.remove(guid);
                    self.scene.session.lookup.remove(guid);
                    self.scene.geom_guid_set.remove(guid);
                    self.scene.selected_guids.remove(guid);
                }
                for ns in nurbs {
                    let guid = ns.guid().to_string();
                    self.scene.gpu_session.remove(&guid);
                    self.scene.session.objects.nurbssurfaces.retain(|n| n.guid() != guid);
                    self.scene.geom_guid_set.remove(&guid);
                    self.scene.selected_guids.remove(&guid);
                }
                for nc in curves {
                    let guid = nc.guid().to_string();
                    self.scene.gpu_session.remove(&guid);
                    self.scene.session.objects.nurbscurves.retain(|n| n.guid() != guid);
                    self.scene.geom_guid_set.remove(&guid);
                    self.scene.selected_guids.remove(&guid);
                }
                self.scene.leaf_cache_dirty = true;
                if self.scene.selected_guids.is_empty() { self.gb.gumball = None; }
            }
            UndoAction::Transform { objects, snapshots_after, .. } => {
                for (guid, _before, after) in objects {
                    let was_selected = self.scene.gpu_session.pick.instance_id(guid)
                        .and_then(|iid| self.scene.gpu_session.instances_cpu.get(iid as usize))
                        .map_or(false, |inst| inst.flags & InstanceData::FLAG_SELECTED != 0);
                    match snapshots_after.get(guid) {
                        Some(GeomSnapshot::Geom(geom)) => {
                            self.scene.gpu_session.remove(guid);
                            self.scene.session.lookup.insert(guid.clone(), geom.clone());
                            self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                        }
                        Some(GeomSnapshot::Nurbs(ns)) => {
                            self.scene.gpu_session.remove(guid);
                            if let Some(slot) = self.scene.session.objects.nurbssurfaces.iter_mut().find(|n| n.guid() == guid) {
                                *slot = std::rc::Rc::new(ns.clone());
                            }
                            self.scene.gpu_session.add_nurbssurface(ns, &self.gpu.device, &self.gpu.queue);
                        }
                        None => {
                            self.commit_object_transform(guid, *after);
                        }
                    }
                    if was_selected {
                        self.scene.gpu_session.set_flag(guid, InstanceData::FLAG_SELECTED, true, &self.gpu.queue);
                    }
                }
                if !self.scene.selected_guids.is_empty() {
                    let origin = self.selected_centroid();
                    if let Some(gb) = &mut self.gb.gumball { gb.set_origin(origin); }
                }
            }
            UndoAction::EditGeom { guid, after, .. } => {
                self.restore_edit_snapshot(guid, after);
            }
        }
    }
}
