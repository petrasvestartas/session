        self.group=group(device,&self.layouts[3],vec![(0,rows.as_entire_binding()),(1,owner.as_entire_binding())]);self.bytes=bytes.to_vec();self.owners=owners.to_vec();self.count=(bytes.len()/48)as u32;
    }
    fn shader(&self,device:&wgpu::Device)->wgpu::ShaderModule {
        self.shader.borrow_mut().get_or_insert_with(||device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("actual production vector"),source:wgpu::ShaderSource::Wgsl(self.source.into())})).clone()
    }
    pub fn prepare(&self,device:&wgpu::Device,samples:u32,picking:bool) {
        if self.count==0{return;}let shader=self.shader(device);
        if picking {self.pick.borrow_mut().get_or_insert_with(||pipeline(device,&shader,&self.layouts,self.vertex,wgpu::TextureFormat::Rg32Uint,1,true));}
        else {let mut color=self.color.borrow_mut();if let Some(color)=color.as_mut(){color.select(samples,||pipeline(device,&shader,&self.layouts,self.vertex,self.format,samples,false));}else{*color=Some(crate::sample_pipelines::Variants::new(pipeline(device,&shader,&self.layouts,self.vertex,self.format,samples,false),samples));}}
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,shared:&[wgpu::BindGroup;4],picking:bool) {
        if self.count==0{return;}
        let color=self.color.borrow();let pick=self.pick.borrow();let pipeline=if picking{pick.as_ref()}else{color.as_ref().map(|p|&p.active)};
        if let Some(pipeline)=pipeline {pass.set_pipeline(pipeline);for(i,group)in shared[..3].iter().enumerate(){pass.set_bind_group(i as u32,group,&[]);}pass.set_bind_group(3,&self.group,&[]);pass.draw(0..self.vertices,0..self.count);}
    }
}
fn pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layouts: &[wgpu::BindGroupLayout; 4],
    vertex: &str,
    format: wgpu::TextureFormat,
    samples: u32,
    picking: bool,
