pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub fn surface() -> wgpu::DepthStencilState { state(true, wgpu::CompareFunction::Less) }
pub fn ink() -> wgpu::DepthStencilState {
    let mut depth=state(false,wgpu::CompareFunction::LessEqual);
    depth.bias=wgpu::DepthBiasState{constant:-2,slope_scale:-1.0,clamp:0.0};
    depth
}

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState { format: FORMAT, depth_write_enabled: Some(write), depth_compare: Some(compare),
        stencil: Default::default(), bias: Default::default() }
}
