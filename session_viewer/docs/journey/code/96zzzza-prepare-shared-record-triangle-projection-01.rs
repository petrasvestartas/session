    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
