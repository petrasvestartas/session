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

    fn apply_undo(&mut self, action: &UndoAction) {
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
            UndoAction::RemoveObjects { objects } => {
                for (guid, geom) in objects {
                    self.scene.session.lookup.insert(guid.clone(), geom.clone());
                    self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
                    self.scene.geom_guid_set.insert(guid.clone());
                }
                self.scene.leaf_cache_dirty = true;
            }
            UndoAction::Transform { objects } => {
                for (guid, before, _after) in objects {
                    self.commit_object_transform(guid, *before);
                }
            }
        }
    }

    fn apply_redo(&mut self, action: &UndoAction) {
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
                self.scene.session.objects.nurbssurfaces.push(ns_clone);
                self.scene.geom_guid_set.insert(guid);
                self.scene.leaf_cache_dirty = true;
            }
            UndoAction::RemoveObjects { objects } => {
                for (guid, _) in objects {
                    self.scene.gpu_session.remove(guid);
                    self.scene.session.lookup.remove(guid);
                    self.scene.geom_guid_set.remove(guid);
                    self.scene.selected_guids.remove(guid);
                }
                self.scene.leaf_cache_dirty = true;
                if self.scene.selected_guids.is_empty() { self.gb.gumball = None; }
            }
            UndoAction::Transform { objects } => {
                for (guid, _before, after) in objects {
                    self.commit_object_transform(guid, *after);
                }
            }
        }
    }
}
