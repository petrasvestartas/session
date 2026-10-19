    pub fn control_nets(&self)->Vec<(ObjectId,crate::stroke::Stroke)> {self.control_nets_at([0.;3])}
    pub fn control_nets_at(&self,anchor:[f64;3])->Vec<(ObjectId,crate::stroke::Stroke)> {