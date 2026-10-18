    pub fn replace(&mut self,next:Self){
        self.control().cancel();*self.0.borrow_mut()=next.0.borrow_mut().take();
    }
    pub fn has_route(&self)->bool