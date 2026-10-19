    pub fn has_route(&self)->bool{self.0.borrow().is_some()}
    pub fn control(&self)->Control