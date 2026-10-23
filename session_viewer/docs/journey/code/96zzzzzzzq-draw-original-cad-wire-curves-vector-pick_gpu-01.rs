    }


    pub(crate) fn submit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::production_sheet_gpu::Lane>) {let clipping=self.legacy_clipping.get_or_insert_with(||crate::clipping_gpu::Clipping::new(device));let group=clipping.group.clone();let buffer=clipping.buffer.clone();self.submit_finite(device,queue,queued,meshes,ink,faces,text,clouds,sheets,None,&group,&buffer,None,None,None,None)}
    pub(crate) fn submit_finite(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, queued: Queued, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::production_sheet_gpu::Lane>,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,clipping_buffer:&wgpu::Buffer,caps:Option<&mut crate::caps_gpu::Caps>,records:Option<&crate::instance_gpu::Lane>,ordinary:Option<&crate::production_ribbon_gpu::Lane>,matrices:Option<&crate::production_matrix_gpu::Lane>) {
        assert!(self.active.is_none(), "Only one pick may be in flight");
        let request = queued.request; let window = request.window;
        if self.image.as_ref().is_none_or(|image| image.window.draw[2..] != window.draw[2..] || image.readback.size() != window.bytes()) {
