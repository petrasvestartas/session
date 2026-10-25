pub fn ink() -> wgpu::DepthStencilState {
    let mut depth=state(false,wgpu::CompareFunction::LessEqual);
    depth.bias=wgpu::DepthBiasState{constant:-2,slope_scale:-1.0,clamp:0.0};
    depth
}
pub fn grid() -> wgpu::DepthStencilState { state(false, wgpu::CompareFunction::Less) }