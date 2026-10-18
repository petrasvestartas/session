model_layout: &wgpu::BindGroupLayout, crop_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    mesh_with(device,view_layout,model_layout,crop_layout,false)
}
pub fn mesh_with(device:&wgpu::Device,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,crop_layout:&wgpu::BindGroupLayout,shared:bool)->wgpu::RenderPipeline {