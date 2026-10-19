        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(2, crop, &[]);
