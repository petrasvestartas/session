    pub fn queue(&mut self,request:Request)->Result<(),&'static str>{self.queue_anchored(request,[0.;3])}
    pub fn queue_anchored(&mut self,request:Request,origin:[f64;3])->Result<(),&'static str>{
        let view=crate::pick_view::View::anchored(request.transform,request.window.full,request.density,origin)?;