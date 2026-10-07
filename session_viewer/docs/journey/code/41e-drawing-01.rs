pub fn ink() -> wgpu::DepthStencilState { state(false, wgpu::CompareFunction::LessEqual) }
pub fn grid() -> wgpu::DepthStencilState { state(false, wgpu::CompareFunction::Less) }