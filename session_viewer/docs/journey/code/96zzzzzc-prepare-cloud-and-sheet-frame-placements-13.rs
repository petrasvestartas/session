    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        self.set_anchored(device,queue,scene,selected,bases,[0.;3])
    }
    pub fn set_anchored(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,scene:&Scene,selected:&[ObjectId],bases:&HashMap<(ObjectId,usize),u32>,anchor:[f64;3])->Result<(),&'static str>{
