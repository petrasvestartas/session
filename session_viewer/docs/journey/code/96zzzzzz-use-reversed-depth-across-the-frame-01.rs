pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR:f64=1.;
pub const FAR:f64=0.;
pub const CLOSER:wgpu::CompareFunction=wgpu::CompareFunction::Greater;
pub const CLOSER_EQUAL:wgpu::CompareFunction=wgpu::CompareFunction::GreaterEqual;

pub fn surface() -> wgpu::DepthStencilState { state(true, crate::depth::CLOSER) }
pub fn ink() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER_EQUAL) }
pub fn grid() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER) }

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState { format: FORMAT, depth_write_enabled: Some(write), depth_compare: Some(compare),
        stencil: Default::default(), bias: Default::default() }
}

pub fn state_for_ambient()->wgpu::DepthStencilState{state(false,wgpu::CompareFunction::Equal)}

#[cfg(test)]
mod tests {
 #[test]
 fn every_frame_depth_state_uses_the_same_endpoints(){
  use super::*;
  assert_eq!((NEAR,FAR),(0.,1.));
  assert_eq!(surface().depth_compare,Some(CLOSER));
  assert_eq!(grid().depth_compare,Some(CLOSER));
  assert_eq!(ink().depth_compare,Some(CLOSER_EQUAL));
  assert_eq!(surface().depth_write_enabled,Some(true));
  assert_eq!(ink().depth_write_enabled,Some(false));
  assert_eq!(grid().depth_write_enabled,Some(false));
  assert_eq!(state_for_ambient().depth_compare,Some(wgpu::CompareFunction::Equal));
 }
}
