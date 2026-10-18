
    pub fn remove(&mut self, id: ObjectId) -> bool {
        crate::graph_edit::remove(self,id);
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
