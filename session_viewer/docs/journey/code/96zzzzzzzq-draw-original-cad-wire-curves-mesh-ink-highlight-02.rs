    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {self.objects_at(device,scene,ids,[0.;3])}
    pub fn objects_at(&mut self,device:&wgpu::Device,scene:&Scene,ids:&[crate::scene::ObjectId],anchor:[f64;3])->Result<(),&'static str>{
        self.edge=false;self.target=None;self.mesh=None;let yellow = crate::colour::selection();
        self.strokes.set(device, scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()&&!r.mesh.parts.ink.active).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|crate::render_space::placed(&r.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}})))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()&&!r.mesh.parts.ink.active).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len())).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device,std::iter::empty())?;
        self.markers.set_points(device,std::iter::empty())?;
        Ok(())
    }


    fn clear_ink(&mut self,device:&wgpu::Device)->Result<(),&'static str>{self.strokes.set(device,std::iter::empty())?;self.paths.set_chains(device,std::iter::empty())?;self.markers.set_points(device,std::iter::empty())?;Ok(())}
    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,visible:crate::ink_ranges::Visibility) {
        if self.edge&&visible.edges||!self.edge&&visible.lines {self.strokes.draw(pass,view);}if visible.lines{self.paths.draw(pass,view);}
