    pub fn remove(&mut self, id: ObjectId) -> bool {
        Rc::make_mut(&mut self.control_owners).remove(&id);
