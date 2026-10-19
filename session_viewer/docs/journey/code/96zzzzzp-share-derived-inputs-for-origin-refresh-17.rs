impl Highlight {
    pub fn rebase_face(&mut self,queue:&wgpu::Queue,anchor:[f64;3],target:Option<Target>)->bool {
        if self.target!=target||!target.is_some_and(|t|matches!(t.part,Part::Face(_))){return false;}
        let Some(mesh)=self.mesh.as_mut() else{return false;};mesh.rebase(queue,anchor);true
    }
