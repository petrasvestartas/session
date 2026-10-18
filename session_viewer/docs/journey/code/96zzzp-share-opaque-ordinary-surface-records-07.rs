    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(&self.pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass: