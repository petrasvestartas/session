use super::*;
impl Ambient {
 pub(crate) fn actual_sample_inputs(&self)->([wgpu::TextureView;4],[wgpu::Buffer;4],[wgpu::Texture;3]) {
  let state=self.state.as_ref().unwrap();([state.images.source_radius.view.clone(),state.images.source_normals[0].view.clone(),state.images.source_normals[1].view.clone(),state.images.source_normals[2].view.clone()],state.groups.buffers.clone(),[state.images.linear.texture.clone(),state.images.normals.texture.clone(),state.images.ao[2].texture.clone()])
 }
}
